//! Fixed organism, VM state and embryo. SPEC §6, §8, §11.

use crate::genome::Program;
use crate::media::Media;
use crate::species::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const BODY_G: u16 = 16;
pub const BODY_S: u16 = 2;
pub const MEM_CELLS: usize = 64;
pub const INV_VOLUME: u32 = 64;
pub const ENERGY_MAX: u32 = 65_535;
pub const BASE_UPKEEP: u32 = 32;
pub const MIN_CHILD_ENERGY: u32 = 16_384;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VmState {
    pub regs: [u16; 16],
    pub module: u8,
    pub pc: u16,
    pub stack: Vec<(u8, u16)>,
    pub fault: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum EmbryoState {
    Writing = 1,
    SealedWork = 2,
    Invalid = 3,
    Ready = 4,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationRecord {
    pub structural_op: Option<u8>,
    pub structural_applied: bool,
    pub point_flips: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Embryo {
    pub state: EmbryoState,
    pub body_g: u16,
    pub shell_s: u16,
    pub raw: Media,
    /// Result media being written by Step (SealedWork).
    pub result: Option<Media>,
    /// Planned result bytes (SealedWork).
    pub plan: Option<Vec<u16>>,
    pub step_cursor: u32,
    pub used_len: u32,
    pub mutation_applied: bool,
    pub energy: u32,
    pub ret: [u16; NSPEC],
    pub ordinal: u64,
    pub mutation: MutationRecord,
}

impl Embryo {
    pub fn matter(&self) -> [u64; NSPEC] {
        let mut m = [0u64; NSPEC];
        for i in 0..NSPEC {
            m[i] = self.ret[i] as u64;
        }
        m[G] += self.body_g as u64 + 1 + self.raw.present_pages() as u64;
        if let Some(r) = &self.result {
            m[G] += r.present_pages() as u64;
        }
        m[S] += self.shell_s as u64;
        m
    }

    pub fn pages(&self) -> u32 {
        1 + self.raw.present_pages() + self.result.as_ref().map_or(0, |r| r.present_pages())
    }
}

/// Per-tick scratch, never persisted (checkpoints happen between ticks).
#[derive(Clone, Debug, Default)]
pub struct Scratch {
    pub intent: Option<crate::intent::Intent>,
    pub budget: u32,
    pub heat_pending: u64,
    pub inert: bool,
    pub queries: u8,
    pub draws: u64,
    pub child_uid: u64,
    pub birth_tile: Option<u32>,
    pub signal_reads: u8,
    pub public_reads: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Agent {
    pub uid: u64,
    pub parent: u64,
    pub root: u64,
    pub generation: u32,
    pub birth_tick: u64,
    pub x: u16,
    pub y: u16,
    pub orient: u8,
    pub energy: u32,
    pub body_g: u16,
    pub shell_s: u16,
    pub inv: [u16; NSPEC],
    pub photo_credit: u32,
    pub port_mask: u16,
    pub age: u64,
    #[serde(with = "mem64")]
    pub mem: [u16; MEM_CELLS],
    pub code: Media,
    pub used_len: u32,
    pub vm: VmState,
    pub receipt: Option<(u8, u16)>,
    pub last_actual: u16,
    pub embryo: Option<Embryo>,
    pub embryo_ordinal: u64,
    pub children: u32,
    /// Private draft record (M1a).
    pub draft: Option<Media>,
    #[serde(skip)]
    pub prog: Option<Arc<Program>>,
    #[serde(skip)]
    pub keys: Keys,
    #[serde(skip)]
    pub s: Scratch,
}

/// Cached RNG keys derived from the agent uid (derived state, never persisted).
#[derive(Clone, Copy, Debug, Default)]
pub struct Keys {
    pub ctrl: [u32; 2],
    pub prio4: [u32; 2],
    pub prio6: [u32; 2],
}

impl Keys {
    pub fn derive(seed: u64, uid: u64) -> Self {
        use crate::rng::{stream_key, Domain};
        Keys {
            ctrl: stream_key(seed, Domain::Controller, uid, 0),
            prio4: stream_key(seed, Domain::Resolve, uid, 4),
            prio6: stream_key(seed, Domain::Resolve, uid, 6),
        }
    }
}

impl Agent {
    pub fn program(&self) -> &Program {
        self.prog.as_ref().expect("program parsed")
    }

    pub fn matter(&self) -> [u64; NSPEC] {
        let mut m = [0u64; NSPEC];
        for i in 0..NSPEC {
            m[i] = self.inv[i] as u64;
        }
        m[G] += self.body_g as u64 + 1 + self.code.present_pages() as u64;
        m[G] += self.draft.as_ref().map_or(0, |d| d.present_pages() as u64);
        m[S] += self.shell_s as u64;
        if let Some(e) = &self.embryo {
            let em = e.matter();
            for i in 0..NSPEC {
                m[i] += em[i];
            }
        }
        m
    }

    pub fn heat_cap(&self) -> u64 {
        heat_cap(&self.matter())
    }

    pub fn mass(&self) -> u64 {
        mass(&self.matter())
    }

    pub fn upkeep(&self) -> u32 {
        BASE_UPKEEP
            + 1
            + self.code.present_pages()
            + self.draft.as_ref().map_or(0, |d| d.present_pages())
            + self.embryo.as_ref().map_or(0, |e| e.pages())
    }

    pub fn inv_volume(&self) -> u32 {
        volume(&self.inv)
    }

    pub fn embryo_state_code(&self) -> u16 {
        self.embryo.as_ref().map_or(0, |e| e.state as u16)
    }
}

mod mem64 {
    use super::MEM_CELLS;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(m: &[u16; MEM_CELLS], s: S) -> Result<S::Ok, S::Error> {
        m.as_slice().serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<[u16; MEM_CELLS], D::Error> {
        let v: Vec<u16> = Vec::deserialize(d)?;
        v.try_into().map_err(|_| serde::de::Error::custom("work memory must have 64 cells"))
    }
}
