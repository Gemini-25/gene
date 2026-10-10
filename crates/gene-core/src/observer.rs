//! Read-only observation: counters and event logs. Never feeds back into the world. SPEC §17.

use crate::agent::{Agent, MutationRecord};
use crate::sim::SimState;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeathCause {
    Energy,
    Body,
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
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
#[derive(Clone, Debug, Default)]
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
}

#[derive(Clone, Debug, Default)]
pub struct Observer {
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
