//! Read-only observation: counters and event logs. Never feeds back into the world. SPEC §17.

use crate::agent::{Agent, MutationRecord};
use crate::sim::SimState;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeathCause {
    Energy,
    Body,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TickStats {
    pub inert: u64,
    pub upkeep_q: u64,
    pub vm_q: u64,
    pub respiration: u64,
    pub photosynthesis: u64,
    pub moves: u64,
    pub heat_damage: u64,
    pub media_oxidized: u64,
    pub attempt_q: u64,
    pub effect_q: u64,
    pub agent_ticks: u64,
}

impl TickStats {
    fn add(&mut self, o: &TickStats) {
        self.inert += o.inert;
        self.upkeep_q += o.upkeep_q;
        self.vm_q += o.vm_q;
        self.respiration += o.respiration;
        self.photosynthesis += o.photosynthesis;
        self.moves += o.moves;
        self.heat_damage += o.heat_damage;
        self.media_oxidized += o.media_oxidized;
        self.attempt_q += o.attempt_q;
        self.effect_q += o.effect_q;
        self.agent_ticks += o.agent_ticks;
    }
}

#[derive(Clone, Debug)]
pub struct BirthEvent {
    pub tick: u64,
    pub uid: u64,
    pub parent: u64,
    pub root: u64,
    pub generation: u32,
    pub genome_hash: [u8; 32],
    pub len: u32,
    pub mutation: MutationRecord,
    /// The child's genome holds bytes copied from a public record (non-kin transmission).
    pub foreign: bool,
    pub x: u16,
    pub y: u16,
    /// G the child was born carrying (provisioned by its parent through the embryo store).
    pub inv_g: u16,
    /// Per module: tag and the first 4 bytes of the BLAKE3 hash of its words, to see which
    /// founder's habits a genome carries (E2 modular founders).
    pub modules: Vec<(u16, u32)>,
}

/// (tag, short content hash) of every module of a program.
pub fn module_hashes(p: &crate::genome::Program) -> Vec<(u16, u32)> {
    p.modules
        .iter()
        .map(|m| {
            let words = &p.words[m.start as usize..(m.start + m.len) as usize];
            let mut h = blake3::Hasher::new();
            for w in words {
                h.update(&w.to_le_bytes());
            }
            (m.tag, u32::from_le_bytes(h.finalize().as_bytes()[..4].try_into().unwrap()))
        })
        .collect()
}

#[derive(Clone, Debug)]
pub struct DeathEvent {
    pub tick: u64,
    pub uid: u64,
    pub cause: DeathCause,
    pub age: u64,
    pub children: u32,
    pub generation: u32,
    pub inv_g: u16,
    pub embryo_state: u16,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MutationStats {
    pub finished: u64,
    pub unchanged: u64,
    pub sealed: u64,
    pub structural: [u64; 8],
    pub structural_noop: u64,
    pub point_flips: u64,
    pub step_valid: u64,
    pub step_invalid: u64,
    pub finish_invalid: u64,
}

/// M1a counters (SPEC §20b).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct M1Stats {
    pub emits: u64,
    pub signal_reads: u64,
    pub public_reads: u64,
    pub copies_from_public: u64,
    pub copies_to_public: u64,
    pub draft_to_embryo: u64,
    pub public_writes: u64,
    pub public_resizes: u64,
    pub drafts_created: u64,
    pub bit_flips: u64,
    pub foreign_births: u64,
    pub commits: u64,
    pub adoptions: u64,
    pub parcels_created: u64,
    pub parcel_reactions: u64,
    /// M1d: strength units added by LINK, UNLINK actions, assembly moves (and parcels carried),
    /// pushes, and moves refused by the load graph.
    pub links: u64,
    pub unlinks: u64,
    pub assembly_moves: u64,
    pub parcels_moved: u64,
    pub pushes: u64,
    pub load_failures: u64,
    /// M1e: units taken from bare inventories, contact work on agents, shell and body units it
    /// removed, and agents pushed.
    pub taken: u64,
    pub abrasion_q: u64,
    pub shell_removed: u64,
    pub body_removed: u64,
    pub actor_pushes: u64,
    /// M1f: media created on parcels, and parcel media left as dropped records when their
    /// parcel vanished.
    pub parcel_media_created: u64,
    pub parcel_media_dropped: u64,
    /// Matter units a parent put into its embryo's store (they go to the child at birth).
    pub provisioned: u64,
}

/// One resolved effect, as the behaviour analysis sees it (original §58). Observer only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EffectFact {
    pub tick: u64,
    pub uid: u64,
    /// (opcode - 0x20) << 3 | sub-op; 255 when the sub-op does not fit, except for effects that
    /// ignore their sub-op, which keep sub-op 0.
    pub kind: u8,
    pub outcome: u8,
    /// Actor-relative direction 0..3 (front, right, back, left), 255 when there is none.
    pub dir: u8,
    /// Species 0..9 / energy 15 / signal channel / bond target, 255 when there is none.
    /// CONTACT is set when the effect acts on another agent's inventory or body (M1e).
    pub channel: u8,
    pub actual: u32,
    /// Address domains (`intent::dom`) of the source (r0) and destination (r1) for TRANSFER and
    /// COPY, and of the target (r0) for READ, WRITE and WORK; 255 when there is none. Not part of
    /// the A1.4 token: the judge uses them to tell what an action was done to (SPEC §17f verbs).
    pub src: u8,
    pub dst: u8,
}

impl EffectFact {
    pub const CONTACT: u8 = 0x40;

    pub fn new(tick: u64, uid: u64, it: &crate::intent::Intent, outcome: u8, actual: u32) -> EffectFact {
        use crate::intent::{dom, sub, unpack};
        use crate::isa::op;
        let r = it.r;
        // effects that ignore their sub-op (LINK, UNLINK, TRANSFER, EMIT, COPY) keep their kind
        // whatever the immediate: a mutated, unused immediate of 8 or more is the same action
        let ignores_sub = matches!(it.opc, op::LINK | op::UNLINK | op::TRANSFER | op::EMIT | op::COPY);
        let kind = if !(op::READ..=op::COPY).contains(&it.opc) {
            255
        } else if it.sub < 8 {
            (it.opc - op::READ) << 3 | it.sub as u8
        } else if ignores_sub {
            (it.opc - op::READ) << 3
        } else {
            255
        };
        let vec_dir = |dx: u16, dy: u16| match (dx as i16, dy as i16) {
            (0, -1) => 0,
            (1, 0) => 1,
            (0, 1) => 2,
            (-1, 0) => 3,
            _ => 255,
        };
        let (dir, channel) = match it.opc {
            op::DRIVE if it.sub == sub::DRIVE_TRANSLATE => (if r[5] <= 3 { r[5] as u8 } else { 255 }, 255),
            op::DRIVE => (255, (r[5] & 3) as u8),
            op::TRANSFER => {
                let ext = |a: u16| unpack(a).filter(|(d, o)| *o > 0 && *d != dom::SELF).map(|(_, o)| o - 1);
                (ext(r[0]).or(ext(r[1])).unwrap_or(255), if r[3] < 16 { r[3] as u8 } else { 255 })
            }
            op::WORK | op::LINK | op::UNLINK => (vec_dir(r[1], r[2]), if it.opc == op::WORK { 255 } else { r[4].min(255) as u8 }),
            op::EMIT => (255, (r[3] & 3) as u8),
            op::REPRO if it.sub == sub::REPRO_RELEASE => ((r[5] & 3) as u8, 255),
            _ => (255, 255),
        };
        let domain = |a: u16| unpack(a).map_or(255, |(d, _)| d as u8);
        let (src, dst) = match it.opc {
            op::TRANSFER | op::COPY => (domain(r[0]), domain(r[1])),
            op::READ | op::WRITE | op::WORK => (domain(r[0]), 255),
            _ => (255, 255),
        };
        EffectFact { tick, uid, kind, outcome, dir, channel, actual, src, dst }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Observer {
    /// Resolved effects for behaviour analysis; collected only when Some.
    pub facts: Option<Vec<EffectFact>>,
    /// Interactions for the relation graph (original §58); collected only when Some.
    pub rel: Option<Vec<crate::relations::RelFact>>,
    /// Record writes, copies, damage and code use for the copy lineage; collected only when Some.
    pub lineage: Option<Vec<crate::lineage::MediaFact>>,
    pub m1: M1Stats,
    pub tick: TickStats,
    pub acc: TickStats,
    pub births: Vec<BirthEvent>,
    pub deaths: Vec<DeathEvent>,
    pub total_births: u64,
    pub total_deaths_energy: u64,
    pub total_deaths_body: u64,
    /// Effect outcome counts, [opcode - 0x20][outcome].
    pub outcomes: [[u64; 15]; 10],
    /// REPRO outcome counts by sub-op.
    pub repro: [[u64; 15]; 7],
    pub mutation: MutationStats,
}

/// The cumulative counters of an Observer, saved next to a checkpoint so that a resumed run
/// continues its CSV columns (the event logs and analyses start afresh).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Counters {
    pub total_births: u64,
    pub total_deaths_energy: u64,
    pub total_deaths_body: u64,
    pub m1: M1Stats,
    pub outcomes: Vec<Vec<u64>>,
    pub repro: Vec<Vec<u64>>,
    pub mutation: MutationStats,
    pub acc: TickStats,
}

impl Observer {
    pub fn counters(&self) -> Counters {
        Counters {
            total_births: self.total_births,
            total_deaths_energy: self.total_deaths_energy,
            total_deaths_body: self.total_deaths_body,
            m1: self.m1.clone(),
            outcomes: self.outcomes.iter().map(|r| r.to_vec()).collect(),
            repro: self.repro.iter().map(|r| r.to_vec()).collect(),
            mutation: self.mutation.clone(),
            acc: self.acc.clone(),
        }
    }

    pub fn restore(&mut self, c: Counters) {
        self.total_births = c.total_births;
        self.total_deaths_energy = c.total_deaths_energy;
        self.total_deaths_body = c.total_deaths_body;
        self.m1 = c.m1;
        for (dst, src) in self.outcomes.iter_mut().zip(&c.outcomes) {
            for (d, v) in dst.iter_mut().zip(src) {
                *d = *v;
            }
        }
        for (dst, src) in self.repro.iter_mut().zip(&c.repro) {
            for (d, v) in dst.iter_mut().zip(src) {
                *d = *v;
            }
        }
        self.mutation = c.mutation;
        self.acc = c.acc;
    }
}

impl Observer {
    pub fn record_outcome(&mut self, opc: u8, sub: u16, outcome: u8) {
        let k = opc.wrapping_sub(0x20) as usize;
        if k < 10 && (outcome as usize) < 15 {
            self.outcomes[k][outcome as usize] += 1;
        }
        if opc == crate::isa::op::REPRO && (sub as usize) < 7 && (outcome as usize) < 15 {
            self.repro[sub as usize][outcome as usize] += 1;
        }
    }

    pub fn record_birth(&mut self, child: &Agent, m: &MutationRecord, tick: u64) {
        self.total_births += 1;
        if child.code.foreign {
            self.m1.foreign_births += 1;
        }
        let cells = child.code.prefix(child.used_len).unwrap_or(&[]);
        self.births.push(BirthEvent {
            tick,
            uid: child.uid,
            parent: child.parent,
            root: child.root,
            generation: child.generation,
            genome_hash: crate::genome::hash_cells(cells),
            len: child.used_len,
            mutation: m.clone(),
            foreign: child.code.foreign,
            x: child.x,
            y: child.y,
            inv_g: child.inv[crate::species::G],
            modules: child.prog.as_deref().map(module_hashes).unwrap_or_default(),
        });
    }

    pub fn record_death(&mut self, a: &Agent, tick: u64, cause: DeathCause) {
        match cause {
            DeathCause::Energy => self.total_deaths_energy += 1,
            DeathCause::Body => self.total_deaths_body += 1,
        }
        self.deaths.push(DeathEvent {
            tick,
            uid: a.uid,
            cause,
            age: a.age,
            children: a.children,
            generation: a.generation,
            inv_g: a.inv[crate::species::G],
            embryo_state: a.embryo_state_code(),
        });
    }

    pub fn end_tick(&mut self, _tick: u64, _st: &SimState) {
        let t = std::mem::take(&mut self.tick);
        self.acc.add(&t);
    }

    pub fn take_acc(&mut self) -> TickStats {
        std::mem::take(&mut self.acc)
    }
}
