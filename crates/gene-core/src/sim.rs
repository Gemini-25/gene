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
/// M1d: M1c + bonds, anchors, grip and assembly motion (SPEC §20e).
pub const RULESET_M1D: &str = "gene-m1-4";
/// M1e: M1d + contact between agents: theft from a bare inventory, abrasion, PushActor (SPEC §20f).
pub const RULESET_M1E: &str = "gene-m1-5";
/// M1f: M1e + record media attached to parcels (SPEC §20g). The whole of M1 (v1).
pub const RULESET_M1F: &str = "gene-m1-6";
/// M0 physics with faster decomposition at living temperatures and the locality order
/// (SPEC §20h). For survival experiments after E1-d.
pub const RULESET_M0B: &str = "gene-m0-2";
/// Everything: M1f + passive signal conduction + LocalTrace, on the gene-m0-2 physics (SPEC §20h).
pub const RULESET_M2: &str = "gene-m2-1";
/// gene-m0-2 + soil keeps up to SOIL_RETAIN units of water from running downhill (SPEC §20i).
pub const RULESET_M0C: &str = "gene-m0-3";
/// gene-m2-1 on the gene-m0-3 physics (SPEC §20i).
pub const RULESET_M2B: &str = "gene-m2-2";
/// gene-m0-3 + aging: upkeep grows by 1q/tick per day of age (SPEC §20j, candidate).
pub const RULESET_M0D: &str = "gene-m0-4";
/// gene-m2-2 + aging (SPEC §20j, candidate).
pub const RULESET_M2C: &str = "gene-m2-3";
/// gene-m2-2 + milder aging: 1q/tick more per two days of age (SPEC §20j, candidate).
pub const RULESET_M2D: &str = "gene-m2-4";
/// gene-m2-4 + moss: Rx0 at the loose site of the wettest ground, driven by light (SPEC §20k,
/// candidate for decision 8).
pub const RULESET_M2E: &str = "gene-m2-5";
/// gene-m2-5 + signals spread through the air as through bonded metal (SPEC §20l, candidate for
/// decision 8).
pub const RULESET_M2F: &str = "gene-m2-6";
/// gene-m2-4 + moss in blooms that move: every 4 days a fresh random set of 16×16 blocks is moss
/// ground (SPEC §20m, candidate for decision 8).
pub const RULESET_M2G: &str = "gene-m2-7";
/// gene-m2-7 + air conduction of signals (SPEC §20l, §20m, candidate for decision 8).
pub const RULESET_M2H: &str = "gene-m2-8";
/// gene-m2-4 + mating: a finished embryo may take a module from a neighbour's code (SPEC §20n,
/// candidate for decision 8).
pub const RULESET_M2I: &str = "gene-m2-9";
/// gene-m2-5 + food patches: small, one-day, rich moss patches drawn afresh every day (SPEC §20o,
/// candidate after E3).
pub const RULESET_M2J: &str = "gene-m2-10";

/// Capabilities switched on by the ruleset.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Caps {
    pub m1: bool,
    pub commit: bool,
    pub parcels: bool,
    pub bonds: bool,
    pub contact: bool,
    pub parcel_media: bool,
    /// Passive signal conduction along bonded metal parcels (original v1).
    pub conduction: bool,
    /// LocalTrace: tiles remember their last four external actions (original v2).
    pub trace: bool,
    /// Rx2 at least 16 at living temperatures (SPEC §20h).
    pub decomp: bool,
    /// Environment due and transport buckets by 64-tile runs (SPEC §20h).
    pub locality: bool,
    /// Soil keeps water from running downhill (SPEC §20i).
    pub retain: bool,
    /// Upkeep grows by 1q/tick for every this many ticks of age; 0 is no aging (SPEC §20j).
    pub aging: u64,
    /// Moss: fertile ground makes G from C and W with the light it absorbs (SPEC §20k).
    pub moss: bool,
    /// Every tile conducts signals to its four neighbours, as bonded metal does (SPEC §20l).
    pub air_signal: bool,
    /// Moss ground is a set of discs drawn afresh every epoch instead of the wettest substrate
    /// (SPEC §20m, §20o; needs `moss`).
    pub bloom: Option<crate::env::Bloom>,
    /// Mating: at Finish, 1 embryo in MATE_ODDS with a neighbour takes one of its modules
    /// (SPEC §20n).
    pub mate: bool,
}

impl Caps {
    pub fn of(ruleset: &str) -> Option<Caps> {
        match ruleset {
            RULESET => Some(Caps::default()),
            RULESET_M1 => Some(Caps { m1: true, ..Caps::default() }),
            RULESET_M1B => Some(Caps { m1: true, commit: true, ..Caps::default() }),
            RULESET_M1C => Some(Caps { m1: true, commit: true, parcels: true, ..Caps::default() }),
            RULESET_M1D => Some(Caps { m1: true, commit: true, parcels: true, bonds: true, ..Caps::default() }),
            RULESET_M1E => Some(Caps { m1: true, commit: true, parcels: true, bonds: true, contact: true, ..Caps::default() }),
            RULESET_M1F => {
                Some(Caps { m1: true, commit: true, parcels: true, bonds: true, contact: true, parcel_media: true, ..Caps::default() })
            }
            RULESET_M0B => Some(Caps { decomp: true, locality: true, ..Caps::default() }),
            RULESET_M0C => Some(Caps { decomp: true, locality: true, retain: true, ..Caps::default() }),
            RULESET_M2B => Some(Caps { retain: true, ..Caps::of(RULESET_M2)? }),
            RULESET_M0D => Some(Caps { aging: AGING_PERIOD, ..Caps::of(RULESET_M0C)? }),
            RULESET_M2C => Some(Caps { aging: AGING_PERIOD, ..Caps::of(RULESET_M2B)? }),
            RULESET_M2D => Some(Caps { aging: 2 * AGING_PERIOD, ..Caps::of(RULESET_M2B)? }),
            RULESET_M2E => Some(Caps { moss: true, ..Caps::of(RULESET_M2D)? }),
            RULESET_M2F => Some(Caps { air_signal: true, ..Caps::of(RULESET_M2E)? }),
            RULESET_M2G => Some(Caps { bloom: Some(crate::env::BLOOMS), ..Caps::of(RULESET_M2E)? }),
            RULESET_M2H => Some(Caps { air_signal: true, ..Caps::of(RULESET_M2G)? }),
            RULESET_M2I => Some(Caps { mate: true, ..Caps::of(RULESET_M2D)? }),
            RULESET_M2J => Some(Caps { bloom: Some(crate::env::PATCHES), ..Caps::of(RULESET_M2E)? }),
            RULESET_M2 => Some(Caps {
                m1: true,
                commit: true,
                parcels: true,
                bonds: true,
                contact: true,
                parcel_media: true,
                conduction: true,
                trace: true,
                decomp: true,
                locality: true,
                retain: false,
                aging: 0,
                moss: false,
                air_signal: false,
                bloom: None,
                mate: false,
            }),
            _ => None,
        }
    }
}

impl Sim {
    /// Passive conduction (original §22, v1): a tile whose metal parcel is bonded to a metal
    /// neighbour keeps trunc(S/2) of its signal and receives trunc(S_n/8) from each such
    /// neighbour, from the old field (synchronous). Other tiles keep only new emissions.
    /// Air conduction (SPEC §20l): every tile keeps trunc(S/2) of its signal and receives
    /// trunc(S_n/8) from each of its four neighbours, from the old field (synchronous) - the metal
    /// rule on every tile. Written as each nonzero old value scattering to itself and its
    /// neighbours, which gives the same truncated terms.
    fn spread_signals(&self, next: &mut BTreeMap<u64, i64>) {
        let w = &self.st.world;
        for (&k, &v) in &self.st.signal {
            let (t, ch) = ((k >> 2) as u32, (k & 3) as u16);
            let v = v as i64;
            if v / 2 != 0 {
                *next.entry(signal_key(t, ch)).or_insert(0) += v / 2;
            }
            if v / 8 != 0 {
                for d in 0..4u8 {
                    if let Some(nb) = w.neighbor(t, d) {
                        *next.entry(signal_key(nb, ch)).or_insert(0) += v / 8;
                    }
                }
            }
        }
    }

    fn conduct_signals(&self, next: &mut BTreeMap<u64, i64>) {
        let w = &self.st.world;
        let old = |t: u32, ch: u16| self.st.signal.get(&signal_key(t, ch)).copied().unwrap_or(0) as i64;
        for (&t, p) in &w.parcels {
            if p.matrix[crate::species::M] == 0 {
                continue;
            }
            let links: Vec<u32> = (0..4u8)
                .filter(|&f| p.bond[f as usize] > 0)
                .filter_map(|f| w.neighbor(t, f))
                .filter(|nb| w.parcels.get(nb).is_some_and(|q| q.matrix[crate::species::M] > 0))
                .collect();
            if links.is_empty() {
                continue;
            }
            for ch in 0..4u16 {
                let v = old(t, ch) / 2 + links.iter().map(|&nb| old(nb, ch) / 8).sum::<i64>();
                if v != 0 {
                    *next.entry(signal_key(t, ch)).or_insert(0) += v;
                }
            }
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
    /// LocalTrace: the last four external actions per tile (v2 capability).
    pub traces: BTreeMap<u32, [crate::traces::Trace; 4]>,
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
    /// Moss growth at the loose site (SPEC §20k).
    pub moss: [u32; 2],
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
    /// Scratch for the parallel environment stage (all zero between uses; not state).
    pub(crate) env_mask: Vec<u8>,
    /// Moss ground per tile (1) and the bloom epoch it was drawn for (derived, not state).
    pub(crate) moss_mask: Vec<u8>,
    pub(crate) moss_epoch: u64,
    /// Population from which Think runs on the pool (below it the split costs more than it saves).
    pub par_think_min: usize,
    /// LocalTrace event order within the tick (reset every tick).
    pub(crate) trace_seq: u32,
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
        Self::with_programs_ruleset(config, std::slice::from_ref(prog), ruleset)
    }

    /// A world with several public ancestors (E2): founder i runs `progs[i % progs.len()]`.
    /// With one program this is exactly `with_program_ruleset`.
    pub fn with_programs_ruleset(config: Config, progs: &[Program], ruleset: &str) -> Result<Sim, SimError> {
        if Caps::of(ruleset).is_none() {
            return Err(SimError::BadSave("unknown ruleset"));
        }
        if progs.is_empty() {
            return Err(SimError::Founder("no founder program".into()));
        }
        let mut founders = Vec::with_capacity(progs.len());
        for prog in progs {
            let cells = prog.to_cells();
            let parsed = genome::parse(&cells).map_err(|e| SimError::Founder(format!("{e:?}")))?;
            founders.push((cells, Arc::new(parsed)));
        }
        let world = world::generate(config.seed, config.width, config.height);
        let tiles = world::spawn_tiles(&world, config.seed, config.agents as usize)
            .ok_or(SimError::NotEnoughSpawnTiles)?;
        let mut agents = Vec::with_capacity(tiles.len());
        let mut next_media_uid = 1;
        for (i, &t) in tiles.iter().enumerate() {
            let (cells, parsed) = &founders[i % founders.len()];
            let (x, y) = world.xy(t);
            let uid = i as u64 + 1;
            let mut inv = [0u16; NSPEC];
            inv[G] = 32;
            inv[S] = 4;
            let code = Media::from_cells(next_media_uid, cells);
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
                abrasion: 0,
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
            traces: BTreeMap::new(),
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
        st.world.rebuild_static_caches_with(Caps::of(&st.ruleset).unwrap_or_default().locality);
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
        keys.moss = rng::flux_key(seed, FluxDomain::ReactionRound, 5 << 8);
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
            env_mask: Vec::new(),
            moss_mask: Vec::new(),
            moss_epoch: u64::MAX,
            par_think_min: 4096,
            trace_seq: 0,
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

    /// Observer: one interaction for the relation graph (no effect on the world).
    pub(crate) fn rel(&mut self, from: crate::relations::Node, to: crate::relations::Node, kind: u8, channel: u8, amount: u64, tile: u32) {
        if let Some(v) = &mut self.obs.rel {
            v.push(crate::relations::RelFact { tick: self.st.tick, from, to, kind, channel, amount, tile });
        }
    }

    /// Observer: one fact for the copy lineage (no effect on the world).
    pub(crate) fn lin(&mut self, f: crate::lineage::MediaFact) {
        if let Some(v) = &mut self.obs.lineage {
            v.push(f);
        }
    }

    /// Observer: every 256 ticks, agents running learned code or code that holds foreign cells.
    fn sample_code_use(&mut self) {
        let Some(v) = &mut self.obs.lineage else { return };
        if self.st.tick % 256 != 0 {
            return;
        }
        for a in &self.st.agents {
            let media = if a.slots.is_empty() { a.code.uid } else { a.slots[a.vm.module as usize].0 };
            if media != a.code.uid || a.code.foreign {
                v.push(crate::lineage::MediaFact::Exec { tick: self.st.tick, agent: a.uid, media });
            }
        }
    }

    /// Observer: agent i acted on the parcel on tile t (a Use relation; `act` from relations::act).
    pub(crate) fn rel_use(&mut self, i: usize, t: u32, act: u8) {
        if self.obs.rel.is_some() {
            if let Some(p) = self.st.world.parcels.get(&t) {
                let (a, p) = (self.st.agents[i].uid, p.uid);
                self.rel((0, a), (1, p), crate::relations::USE, act, 1, t);
            }
        }
    }

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
        self.sample_code_use();
        self.stage_seal();
        self.stage_local_resolve();
        self.stage_assembly();
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
            4 => {
                self.stage_assembly();
                self.stage_single_motion()
            }
            5 => self.stage_transport(),
            6 => self.stage_environment(),
            7 => self.stage_metabolism(),
            8 => self.stage_lifecycle(),
            _ => self.stage_commit(),
        }
    }

    fn stage_maintenance(&mut self) {
        self.trace_seq = 0;
        let w = &mut self.st.world;
        let aging = self.caps.aging;
        for a in &mut self.st.agents {
            a.s = Scratch::default();
            let t = w.idx(a.x as u32, a.y as u32) as usize;
            let m = a.upkeep() + if aging > 0 { a.aging_upkeep(aging) } else { 0 };
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
            bonds: self.caps.bonds,
            contact: self.caps.contact,
            signal: &self.st.signal,
            traces: &self.st.traces,
            trace: self.caps.trace,
            dropped: &self.st.dropped,
        };
        let before: Vec<u32> = self.st.agents.iter().map(|a| a.energy).collect();
        let threads = self.threads.max(1);
        let par = threads > 1 && self.st.agents.len() >= self.par_think_min.max(1);
        if par {
            if self.pool.is_none() {
                self.pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().ok();
            }
        }
        match &self.pool {
            // Each VM reads the shared world and writes only its own agent (SPEC §19), so the
            // split does not change the result.
            Some(pool) if par => {
                use rayon::prelude::*;
                let agents = &mut self.st.agents;
                let chunk = (agents.len() / (threads * 4)).max(64);
                pool.install(|| {
                    agents.par_iter_mut().with_min_len(chunk).for_each(|a| {
                        if !a.s.inert {
                            vm::think(a, &sense, a.keys.ctrl);
                        }
                    })
                });
            }
            _ => {
                for a in &mut self.st.agents {
                    if !a.s.inert {
                        vm::think(a, &sense, a.keys.ctrl);
                    }
                }
            }
        }
        for (a, before) in self.st.agents.iter().zip(before) {
            if a.s.inert {
                continue;
            }
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
            if let Some((o, act)) = self.st.agents[i].receipt {
                self.obs.record_outcome(crate::isa::op::REPRO, crate::intent::sub::REPRO_RELEASE, o);
                let a = &self.st.agents[i];
                if let (Some(f), Some(it)) = (&mut self.obs.facts, &a.s.intent) {
                    f.push(crate::observer::EffectFact::new(self.st.tick, a.uid, it, o, act as u32));
                }
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
        self.release_grip(i);
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
            abrasion: 0,
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
        if let Some(v) = &mut self.obs.lineage {
            v.push(crate::lineage::MediaFact::Born { tick, agent: child.uid, parent: parent.uid, media: child.code.uid });
        }
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
        // the field read next tick holds this tick's emissions, plus passive conduction along bonded
        // metal parcels where that capability is on (original §22)
        let mut next = std::mem::take(&mut self.signal_next);
        if self.caps.air_signal {
            self.spread_signals(&mut next);
        } else if self.caps.conduction && !self.st.signal.is_empty() {
            self.conduct_signals(&mut next);
        }
        self.prune_traces();
        self.st.signal = next
            .into_iter()
            .filter(|(_, v)| *v != 0)
            .map(|(k, v)| (k, v.clamp(-32767, 32767) as i16))
            .collect();
        self.obs.end_tick(tick, &self.st);
        self.st.tick += 1;
    }
}
