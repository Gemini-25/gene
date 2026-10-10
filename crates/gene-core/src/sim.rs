//! Simulation state and the canonical tick. SPEC §13, §15.

use crate::agent::*;
use crate::genome::{self, Program};
use crate::media::Media;
use crate::observer::Observer;
use crate::rng::{self, FluxDomain};
use crate::species::*;
use crate::vm::{self, ActorFace, Sense};
use crate::world::{self, World, BASE_HEAT_CAP};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

pub const RULESET: &str = "gene-m0-1";
/// M1a: signals, public tape, drafts (SPEC §20b).
pub const RULESET_M1: &str = "gene-m1-1";
/// M1b: M1a + DraftCommit / learned binding (SPEC §20c).
pub const RULESET_M1B: &str = "gene-m1-2";
/// M1c: M1b + parcels (SPEC §20d).
pub const RULESET_M1C: &str = "gene-m1-3";

/// Capabilities switched on by the ruleset.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Caps {
    pub m1: bool,
    pub commit: bool,
    pub parcels: bool,
}

impl Caps {
    pub fn of(ruleset: &str) -> Option<Caps> {
        match ruleset {
            RULESET => Some(Caps { m1: false, commit: false, parcels: false }),
            RULESET_M1 => Some(Caps { m1: true, commit: false, parcels: false }),
            RULESET_M1B => Some(Caps { m1: true, commit: true, parcels: false }),
            RULESET_M1C => Some(Caps { m1: true, commit: true, parcels: true }),
            _ => None,
        }
    }
}

/// Signal field key: tile << 2 | channel.
#[inline]
pub fn signal_key(t: u32, ch: u16) -> u64 {
    (t as u64) << 2 | (ch as u64 & 3)
}
pub const SAVE_MAGIC: &[u8; 8] = b"GENEM001";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub seed: u64,
    pub width: u32,
    pub height: u32,
    pub agents: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config { seed: 1, width: 512, height: 512, agents: 1000 }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Flows {
    pub solar_in: u128,
    pub sky_in: u128,
    pub radiation_out: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimState {
    pub ruleset: String,
    pub config: Config,
    pub tick: u64,
    pub world: World,
    pub agents: Vec<Agent>,
    pub dropped: BTreeMap<u32, Vec<Media>>,
    pub next_uid: u64,
    pub next_media_uid: u64,
    pub flows: Flows,
    pub e0: i128,
    pub atoms0: [u128; NELEM],
    /// Current signal field (nonzero entries only), key = tile << 2 | channel.
    pub signal: BTreeMap<u64, i16>,
    pub next_parcel_uid: u64,
}

/// Cached FluxRng keys.
#[derive(Clone, Debug, Default)]
pub struct FluxKeys {
    pub edge: [[u32; 2]; NSPEC],
    /// Loose site keys for Rx2..Rx6.
    pub loose_rx: [[u32; 2]; 5],
    pub media_rx2: [u32; 2],
    /// Parcel sites (matrix, pore, boundary) for Rx2..Rx6.
    pub parcel_rx: [[[u32; 2]; 5]; 3],
    /// Pore exchange per species.
    pub pore: [[u32; 2]; NSPEC],
}

pub struct Sim {
    pub st: SimState,
    pub obs: Observer,
    pub keys: FluxKeys,
    pub(crate) faces: Vec<ActorFace>,
    pub(crate) reserved: BTreeSet<u32>,
    pub caps: Caps,
    /// Emissions collected this tick for the next field.
    pub(crate) signal_next: BTreeMap<u64, i64>,
    /// Host threads for result-preserving parallel stages (not part of the state).
    pub threads: usize,
    pub(crate) pool: Option<rayon::ThreadPool>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Audit {
    pub energy: i128,
    pub expected: i128,
    pub atoms: [u128; NELEM],
    pub heat_cap_ok: bool,
}

impl Audit {
    pub fn ok(&self, atoms0: &[u128; NELEM]) -> bool {
        self.energy == self.expected && &self.atoms == atoms0 && self.heat_cap_ok
    }
}

#[derive(Debug)]
pub enum SimError {
    NotEnoughSpawnTiles,
    Founder(String),
    BadSave(&'static str),
}

pub fn founder_program() -> Program {
    crate::isa::assemble(crate::FOUNDER_ASM).expect("founder assembles")
}

impl Sim {
    pub fn new(config: Config) -> Result<Sim, SimError> {
        Self::with_program(config, &founder_program())
    }

    pub fn with_program(config: Config, prog: &Program) -> Result<Sim, SimError> {
        Self::with_program_ruleset(config, prog, RULESET)
    }

    pub fn with_program_ruleset(config: Config, prog: &Program, ruleset: &str) -> Result<Sim, SimError> {
        if Caps::of(ruleset).is_none() {
            return Err(SimError::BadSave("unknown ruleset"));
        }
        let cells = prog.to_cells();
        let parsed = genome::parse(&cells).map_err(|e| SimError::Founder(format!("{e:?}")))?;
        let parsed = Arc::new(parsed);
        let world = world::generate(config.seed, config.width, config.height);
        let tiles = world::spawn_tiles(&world, config.seed, config.agents as usize)
            .ok_or(SimError::NotEnoughSpawnTiles)?;
        let mut agents = Vec::with_capacity(tiles.len());
        let mut next_media_uid = 1;
        for (i, &t) in tiles.iter().enumerate() {
            let (x, y) = world.xy(t);
            let uid = i as u64 + 1;
            let mut inv = [0u16; NSPEC];
            inv[G] = 32;
            inv[S] = 4;
            let code = Media::from_cells(next_media_uid, &cells);
            next_media_uid += 1;
            let mut a = Agent {
                uid,
                parent: 0,
                root: uid,
                generation: 0,
                birth_tick: 0,
                x: x as u16,
                y: y as u16,
                orient: 0,
                energy: 32768,
                body_g: BODY_G,
                shell_s: BODY_S,
                inv,
                photo_credit: 0,
                port_mask: 0,
                age: 0,
                mem: [0; MEM_CELLS],
                code,
                used_len: cells.len() as u32,
                vm: VmState { module: parsed.entry_index() as u8, ..Default::default() },
                receipt: None,
                last_actual: 0,
                embryo: None,
                embryo_ordinal: 0,
                children: 0,
                draft: None,
                learned: None,
                retained: vec![],
                slots: vec![],
                binding: vec![],
                entry_slot: 0,
                bank_progs: vec![],
                prog: Some(parsed.clone()),
                keys: Keys::derive(config.seed, uid),
                s: Scratch::default(),
            };
            a.s = Scratch::default();
            agents.push(a);
        }
        let st = SimState {
            ruleset: ruleset.to_string(),
            config: config.clone(),
            tick: 0,
            world,
            agents,
            dropped: BTreeMap::new(),
            next_uid: tiles.len() as u64 + 1,
            next_media_uid,
            flows: Flows::default(),
            e0: 0,
            atoms0: [0; NELEM],
            signal: BTreeMap::new(),
            next_parcel_uid: 1,
        };
        let mut sim = Sim::from_state_unchecked(st);
        // Initial temperature 300 everywhere, after all matter is placed.
        for t in 0..sim.st.world.n() {
            sim.st.world.heat_u[t] = 300 * sim.st.world.heat_c[t];
        }
        let a = sim.audit();
        sim.st.e0 = a.energy;
        sim.st.atoms0 = a.atoms;
        Ok(sim)
    }

    fn from_state_unchecked(mut st: SimState) -> Sim {
        st.world.rebuild_static_caches();
        st.world.occ = vec![0; st.world.n()];
        for a in &mut st.agents {
            if a.prog.is_none() {
                let p = genome::parse(a.code.prefix(a.used_len).expect("inherited prefix"))
                    .expect("inherited code is valid");
                a.prog = Some(Arc::new(p));
            }
            a.keys = Keys::derive(st.config.seed, a.uid);
            a.rebuild_bank_progs();
        }
        let seed = st.config.seed;
        let mut keys = FluxKeys::default();
        for s in 0..NSPEC {
            keys.edge[s] = rng::flux_key(seed, FluxDomain::EdgeFlux, s as u32);
        }
        for (k, rx) in CREDIT_REACTIONS.iter().enumerate() {
            keys.loose_rx[k] = rng::flux_key(seed, FluxDomain::ReactionRound, rx.id as u32);
        }
        keys.media_rx2 = rng::flux_key(seed, FluxDomain::ReactionRound, (3 << 8) | 2);
        for (si, kind) in [1u32, 2, 4].iter().enumerate() {
            for (k, rx) in CREDIT_REACTIONS.iter().enumerate() {
                keys.parcel_rx[si][k] = rng::flux_key(seed, FluxDomain::ReactionRound, (kind << 8) | rx.id as u32);
            }
        }
        for s in 0..NSPEC {
            keys.pore[s] = rng::flux_key(seed, FluxDomain::PoreFlux, s as u32);
        }
        let caps = Caps::of(&st.ruleset).unwrap_or_default();
        let mut sim = Sim {
            st,
            obs: Observer::default(),
            keys,
            faces: vec![],
            reserved: BTreeSet::new(),
            caps,
            signal_next: BTreeMap::new(),
            threads: 1,
            pool: None,
        };
        sim.rebuild_occ();
        sim.recompute_heat_cap();
        sim
    }

    pub fn from_state(st: SimState) -> Result<Sim, SimError> {
        if Caps::of(&st.ruleset).is_none() {
            return Err(SimError::BadSave("ruleset mismatch"));
        }
        Ok(Self::from_state_unchecked(st))
    }

    pub fn rebuild_occ(&mut self) {
        let w = &mut self.st.world;
        w.occ.iter_mut().for_each(|o| *o = 0);
        for (i, a) in self.st.agents.iter().enumerate() {
            let t = w.idx(a.x as u32, a.y as u32) as usize;
            assert_eq!(w.occ[t], 0, "two actors on one tile");
            w.occ[t] = i as u32 + 1;
        }
    }

    pub fn tile_of(&self, i: usize) -> u32 {
        let a = &self.st.agents[i];
        self.st.world.idx(a.x as u32, a.y as u32)
    }

    pub(crate) fn computed_heat_cap(&self) -> Vec<u64> {
        let w = &self.st.world;
        let mut c: Vec<u64> = (0..w.n() as u32)
            .map(|t| BASE_HEAT_CAP + heat_cap(&w.loose_stock_u64(t)))
            .collect();
        for a in &self.st.agents {
            c[w.idx(a.x as u32, a.y as u32) as usize] += a.heat_cap();
        }
        for (t, ms) in &self.st.dropped {
            for m in ms {
                c[*t as usize] += m.present_pages() as u64 * CAP[G];
            }
        }
        for (t, p) in &w.parcels {
            c[*t as usize] += heat_cap(&p.matter());
        }
        c
    }

    pub fn recompute_heat_cap(&mut self) {
        self.st.world.heat_c = self.computed_heat_cap();
    }

    /// Full ledger recomputation. SPEC §4.
    pub fn audit(&self) -> Audit {
        let w = &self.st.world;
        let mut energy: i128 = 0;
        let mut atoms = [0u128; NELEM];
        for t in 0..w.n() as u32 {
            let s = w.loose_stock_u64(t);
            energy += enthalpy(&s) + w.heat_u[t as usize] as i128;
            energy += mass(&s) as i128 * w.elev[t as usize] as i128;
            add_atoms(&mut atoms, &s);
        }
        for a in &self.st.agents {
            let m = a.matter();
            let e = w.elev[w.idx(a.x as u32, a.y as u32) as usize] as i128;
            energy += a.energy as i128 + a.photo_credit as i128;
            energy += a.embryo.as_ref().map_or(0, |e| e.energy as i128);
            energy += enthalpy(&m) + mass(&m) as i128 * e;
            add_atoms(&mut atoms, &m);
        }
        for (t, ms) in &self.st.dropped {
            for md in ms {
                let mut m = [0u64; NSPEC];
                m[G] = md.present_pages() as u64;
                energy += enthalpy(&m) + mass(&m) as i128 * w.elev[*t as usize] as i128;
                add_atoms(&mut atoms, &m);
            }
        }
        for (t, p) in &w.parcels {
            let m = p.matter();
            energy += enthalpy(&m) + mass(&m) as i128 * w.elev[*t as usize] as i128;
            add_atoms(&mut atoms, &m);
        }
        let f = &self.st.flows;
        let expected = self.st.e0 + f.solar_in as i128 + f.sky_in as i128 - f.radiation_out as i128;
        let heat_cap_ok = self.computed_heat_cap() == w.heat_c;
        Audit { energy, expected, atoms, heat_cap_ok }
    }

    pub fn audit_ok(&self) -> bool {
        self.audit().ok(&self.st.atoms0)
    }

    /// CoreHash: BLAKE3 over the canonical serialization of the state. SPEC §15.
    pub fn state_hash(&self) -> [u8; 32] {
        let bytes = postcard::to_allocvec(&self.st).expect("serialize");
        *blake3::hash(&bytes).as_bytes()
    }

    pub fn save(&self) -> Vec<u8> {
        let body = postcard::to_allocvec(&self.st).expect("serialize");
        let mut out = Vec::with_capacity(body.len() + 40);
        out.extend_from_slice(SAVE_MAGIC);
        out.extend_from_slice(&body);
        out.extend_from_slice(blake3::hash(&body).as_bytes());
        out
    }

    pub fn load(bytes: &[u8]) -> Result<Sim, SimError> {
        if bytes.len() < 40 || &bytes[..8] != SAVE_MAGIC {
            return Err(SimError::BadSave("magic"));
        }
        let body = &bytes[8..bytes.len() - 32];
        if blake3::hash(body).as_bytes() != &bytes[bytes.len() - 32..] {
            return Err(SimError::BadSave("checksum"));
        }
        let st: SimState = postcard::from_bytes(body).map_err(|_| SimError::BadSave("decode"))?;
        Self::from_state(st)
    }

    /// Fixture helper: place agent i at (x, y) with an orientation. Call `fixture_rebase` after.
    pub fn fixture_place(&mut self, i: usize, x: u16, y: u16, orient: u8) {
        let a = &mut self.st.agents[i];
        a.x = x;
        a.y = y;
        a.orient = orient & 3;
    }

    /// Fixture helper: rebuild caches and reset the ledger baseline after manual edits.
    pub fn fixture_rebase(&mut self) {
        self.rebuild_occ();
        self.recompute_heat_cap();
        let a = self.audit();
        let f = &self.st.flows;
        self.st.e0 = a.energy - (f.solar_in as i128 + f.sky_in as i128 - f.radiation_out as i128);
        self.st.atoms0 = a.atoms;
    }

    pub fn alloc_media_uid(&mut self) -> u64 {
        let u = self.st.next_media_uid;
        self.st.next_media_uid += 1;
        u
    }

    pub fn drop_media(&mut self, t: u32, m: Media) {
        if m.present_pages() == 0 {
            return;
        }
        // records on a tile stay in UID order (SPEC §5.1 site order, §20b selectors)
        let list = self.st.dropped.entry(t).or_default();
        let pos = list.partition_point(|x| x.uid < m.uid);
        list.insert(pos, m);
    }

    // ---- energy helpers used by effect resolution ----

    /// Pay q from the agent's sealed budget; the energy becomes heat at `heat_tile`.
    pub(crate) fn pay(&mut self, i: usize, q: u32, heat_tile: u32) {
        let a = &mut self.st.agents[i];
        assert!(q <= a.s.budget && q <= a.energy, "overspend");
        a.energy -= q;
        a.s.budget -= q;
        self.st.world.heat_u[heat_tile as usize] += q as u64;
        self.obs.tick.effect_q += q as u64;
    }

    /// Pay q from the budget into potential energy (no heat).
    pub(crate) fn pay_ug(&mut self, i: usize, q: u32) {
        let a = &mut self.st.agents[i];
        assert!(q <= a.s.budget && q <= a.energy, "overspend");
        a.energy -= q;
        a.s.budget -= q;
        self.obs.tick.effect_q += q as u64;
    }

    /// Move heat capacity c_move from src to dst with its share of heat. Returns moved heat.
    pub(crate) fn move_heat(&mut self, src: u32, dst: u32, c_move: u64) -> u64 {
        if src == dst || c_move == 0 {
            return 0;
        }
        let w = &mut self.st.world;
        let (s, d) = (src as usize, dst as usize);
        let q = (w.heat_u[s] as u128 * c_move as u128 / w.heat_c[s] as u128) as u64;
        w.heat_u[s] -= q;
        w.heat_u[d] += q;
        w.heat_c[s] -= c_move;
        w.heat_c[d] += c_move;
        q
    }

    // ---- the tick ----

    pub fn step(&mut self) {
        self.stage_maintenance();
        self.stage_think();
        self.stage_seal();
        self.stage_local_resolve();
        self.stage_single_motion();
        self.stage_transport();
        self.stage_environment();
        self.stage_metabolism();
        self.stage_lifecycle();
        self.stage_commit();
    }

    /// Run one stage by index (benchmarking aid; calling all ten in order equals `step`).
    pub fn bench_stage(&mut self, k: usize) {
        match k {
            0 => self.stage_maintenance(),
            1 => self.stage_think(),
            2 => self.stage_seal(),
            3 => self.stage_local_resolve(),
            4 => self.stage_single_motion(),
            5 => self.stage_transport(),
            6 => self.stage_environment(),
            7 => self.stage_metabolism(),
            8 => self.stage_lifecycle(),
            _ => self.stage_commit(),
        }
    }

    fn stage_maintenance(&mut self) {
        let w = &mut self.st.world;
        for a in &mut self.st.agents {
            a.s = Scratch::default();
            let t = w.idx(a.x as u32, a.y as u32) as usize;
            let m = a.upkeep();
            let paid = m.min(a.energy);
            a.energy -= paid;
            w.heat_u[t] += paid as u64;
            if paid < m {
                a.s.inert = true;
                self.obs.tick.inert += 1;
            }
            self.obs.tick.upkeep_q += paid as u64;
            self.obs.tick.agent_ticks += 1;
        }
    }

    fn stage_think(&mut self) {
        self.faces.clear();
        self.faces.extend(self.st.agents.iter().map(|a| ActorFace {
            uid: a.uid,
            orient: a.orient,
            shell: a.shell_s > 0,
            port: a.port_mask,
        }));
        let sense = Sense {
            world: &self.st.world,
            faces: &self.faces,
            tick: self.st.tick,
            m1: self.caps.m1,
            parcels: self.caps.parcels,
            signal: &self.st.signal,
            dropped: &self.st.dropped,
        };
        for a in &mut self.st.agents {
            if a.s.inert {
                continue;
            }
            let before = a.energy;
            vm::think(a, &sense, a.keys.ctrl);
            let spent = (before - a.energy) as u64;
            let attempt = a
                .s
                .intent
                .as_ref()
                .map_or(0, |it| crate::intent::attempt_cost(it.opc, it.sub) as u64);
            self.obs.tick.vm_q += spent - attempt;
            self.obs.tick.attempt_q += attempt;
            self.obs.m1.signal_reads += a.s.signal_reads as u64;
            self.obs.m1.public_reads += a.s.public_reads as u64;
        }
    }

    fn stage_seal(&mut self) {
        let w = &mut self.st.world;
        for a in &mut self.st.agents {
            let t = w.idx(a.x as u32, a.y as u32) as usize;
            w.heat_u[t] += a.s.heat_pending;
            a.s.heat_pending = 0;
            a.s.budget = a.energy;
            if let Some(it) = &a.s.intent {
                if it.opc == crate::isa::op::REPRO && it.sub == crate::intent::sub::REPRO_RELEASE {
                    a.s.child_uid = self.st.next_uid;
                    self.st.next_uid += 1;
                }
            }
        }
    }

    fn stage_metabolism(&mut self) {
        let w = &mut self.st.world;
        for a in &mut self.st.agents {
            let t = w.idx(a.x as u32, a.y as u32);
            let th = w.theta(t);
            if !(260..=340).contains(&th)
                || a.inv[G] == 0
                || w.loose_total(t, O) == 0
                || w.loose_space(t, C) == 0
                || w.loose_space(t, W) == 0
                || a.energy + RESP_AVAIL > ENERGY_MAX
            {
                continue;
            }
            a.inv[G] -= 1;
            w.take_loose(t, O, 1);
            w.put_loose(t, C, 1);
            w.put_loose(t, W, 1);
            a.energy += RESP_AVAIL;
            w.heat_u[t as usize] += RESP_HEAT;
            self.obs.tick.respiration += 1;
        }
    }

    fn stage_lifecycle(&mut self) {
        // Deaths first.
        let mut dead = vec![false; self.st.agents.len()];
        for i in 0..self.st.agents.len() {
            let a = &self.st.agents[i];
            let cause = if a.energy == 0 {
                Some(crate::observer::DeathCause::Energy)
            } else if a.body_g < 8 {
                Some(crate::observer::DeathCause::Body)
            } else {
                None
            };
            if let Some(c) = cause {
                self.kill(i, c);
                dead[i] = true;
            }
        }
        // Births in child-uid order (= parent uid order).
        let mut births: Vec<(u64, usize)> = self
            .st
            .agents
            .iter()
            .enumerate()
            .filter(|(_, a)| a.s.birth_tile.is_some())
            .map(|(i, a)| (a.s.child_uid, i))
            .collect();
        births.sort_unstable();
        let mut newborn = Vec::new();
        for (child_uid, i) in births {
            if dead[i] {
                continue;
            }
            if let Some(child) = self.try_birth(i, child_uid) {
                newborn.push(child);
            }
            if let Some((o, _)) = self.st.agents[i].receipt {
                self.obs.record_outcome(crate::isa::op::REPRO, crate::intent::sub::REPRO_RELEASE, o);
            }
        }
        if dead.iter().any(|d| *d) {
            let mut k = 0;
            self.st.agents.retain(|_| {
                let keep = !dead[k];
                k += 1;
                keep
            });
        }
        self.st.agents.extend(newborn);
        self.rebuild_occ();
    }

    fn kill(&mut self, i: usize, cause: crate::observer::DeathCause) {
        let t = self.tile_of(i);
        let a = &mut self.st.agents[i];
        let w = &mut self.st.world;
        w.force_loose(t, G, a.body_g as u64 + 1);
        w.force_loose(t, S, a.shell_s as u64);
        for s in 0..NSPEC {
            w.force_loose(t, s, a.inv[s] as u64);
        }
        let mut heat = a.energy as u64 + a.photo_credit as u64;
        let code = std::mem::replace(&mut a.code, Media::empty(0));
        let embryo = a.embryo.take();
        let draft = a.draft.take();
        let banks: Vec<Media> = a.learned.take().into_iter().chain(a.retained.drain(..)).map(|b| b.media).collect();
        a.slots.clear();
        a.binding.clear();
        a.bank_progs.clear();
        self.obs.record_death(a, self.st.tick, cause);
        if let Some(e) = embryo {
            w.force_loose(t, G, e.body_g as u64 + 1);
            w.force_loose(t, S, e.shell_s as u64);
            for s in 0..NSPEC {
                w.force_loose(t, s, e.ret[s] as u64);
            }
            heat += e.energy as u64;
            self.drop_media(t, e.raw);
            if let Some(r) = e.result {
                self.drop_media(t, r);
            }
        }
        self.drop_media(t, code);
        if let Some(d) = draft {
            self.drop_media(t, d);
        }
        for m in banks {
            self.drop_media(t, m);
        }
        let a = &mut self.st.agents[i];
        a.energy = 0;
        a.photo_credit = 0;
        a.body_g = 0;
        a.shell_s = 0;
        a.inv = [0; NSPEC];
        self.st.world.heat_u[t as usize] += heat;
        self.st.world.occ[t as usize] = 0;
    }

    fn try_birth(&mut self, i: usize, child_uid: u64) -> Option<Agent> {
        let pt = self.tile_of(i);
        let bt = self.st.agents[i].s.birth_tile.take().expect("reserved");
        let w = &self.st.world;
        let adjacent = (0..4).any(|d| w.neighbor(pt, d) == Some(bt));
        if !adjacent {
            self.st.agents[i].receipt = Some((crate::intent::Outcome::Stale as u8, 0));
            return None;
        }
        if w.occ[bt as usize] != 0 || w.blocked(bt) {
            self.st.agents[i].receipt = Some((crate::intent::Outcome::Blocked as u8, 0));
            return None;
        }
        let e = self.st.agents[i].embryo.as_ref().expect("embryo");
        let m = e.matter();
        let c_child = heat_cap(&m);
        let dz = w.elev[bt as usize] as i64 - w.elev[pt as usize] as i64;
        let dug = mass(&m) as i64 * dz;
        if dug > 0 {
            if (self.st.agents[i].s.budget as i64) < dug {
                self.st.agents[i].receipt = Some((crate::intent::Outcome::Energy as u8, 0));
                return None;
            }
            self.pay_ug(i, dug as u32);
        } else if dug < 0 {
            self.st.world.heat_u[bt as usize] += (-dug) as u64;
        }
        self.move_heat(pt, bt, c_child);
        let seed = self.st.config.seed;
        let tick = self.st.tick;
        let parent = &mut self.st.agents[i];
        let e = parent.embryo.take().expect("embryo");
        parent.children += 1;
        parent.receipt = Some((crate::intent::Outcome::Ok as u8, 1));
        let abs_dir = (0..4u8).find(|d| self.st.world.neighbor(pt, *d) == Some(bt)).unwrap();
        let prog = Arc::new(
            genome::parse(e.raw.prefix(e.used_len).expect("ready prefix")).expect("ready is valid"),
        );
        let (x, y) = self.st.world.xy(bt);
        // Return stock enters the child inventory within capacity; the rest stays at the birth tile.
        let mut inv = [0u16; NSPEC];
        let mut vol = 0u32;
        for s in 0..NSPEC {
            let fit = ((INV_VOLUME - vol) / VOL[s]).min(e.ret[s] as u32);
            inv[s] = fit as u16;
            vol += fit * VOL[s];
            let rest = e.ret[s] as u64 - fit as u64;
            if rest > 0 {
                self.st.world.force_loose(bt, s, rest);
            }
        }
        let child = Agent {
            uid: child_uid,
            parent: parent.uid,
            root: parent.root,
            generation: parent.generation + 1,
            birth_tick: tick,
            x: x as u16,
            y: y as u16,
            orient: abs_dir,
            energy: e.energy,
            body_g: e.body_g,
            shell_s: e.shell_s,
            inv,
            photo_credit: 0,
            port_mask: 0,
            age: 0,
            mem: [0; MEM_CELLS],
            code: e.raw,
            used_len: e.used_len,
            vm: VmState { module: prog.entry_index() as u8, ..Default::default() },
            receipt: None,
            last_actual: 0,
            embryo: None,
            embryo_ordinal: 0,
            children: 0,
            draft: None,
            learned: None,
            retained: vec![],
            slots: vec![],
            binding: vec![],
            entry_slot: 0,
            bank_progs: vec![],
            prog: Some(prog),
            keys: Keys::derive(seed, child_uid),
            s: Scratch::default(),
        };
        self.st.world.occ[bt as usize] = u32::MAX; // occupied until rebuild
        self.obs.record_birth(&child, &e.mutation, tick);
        Some(child)
    }

    fn stage_commit(&mut self) {
        let tick = self.st.tick;
        if self.caps.commit {
            for i in 0..self.st.agents.len() {
                if let Some(len) = self.st.agents[i].s.pending_adopt.take() {
                    self.adopt(i, len);
                } else if !self.st.agents[i].retained.is_empty() {
                    self.gc_retained(i);
                }
            }
        }
        for a in &mut self.st.agents {
            if a.birth_tick != tick || a.generation == 0 {
                a.age += 1;
            }
            a.s.intent = None;
        }
        self.reserved.clear();
        // the field read next tick holds only this tick's emissions (no conduction in M1a)
        let next = std::mem::take(&mut self.signal_next);
        self.st.signal = next
            .into_iter()
            .filter(|(_, v)| *v != 0)
            .map(|(k, v)| (k, v.clamp(-32767, 32767) as i16))
            .collect();
        self.obs.end_tick(tick, &self.st);
        self.st.tick += 1;
    }
}
