//! LocalTrace (original §22, v2): every tile remembers its last four external actions, which
//! nearby agents can READ (domain 9). A short physical trace, not a history archive: no opcode,
//! module, intent or lineage; entries older than 256 ticks read as empty.

use crate::effects::Res;
use crate::intent::{dom, sub, unpack, Intent, Outcome};
use crate::isa::op;
use crate::sim::Sim;
use serde::{Deserialize, Serialize};

/// Trace kinds (0 = empty slot).
pub mod kind {
    pub const MATTER: u8 = 1;
    pub const FORCE: u8 = 2;
    pub const WORK: u8 = 3;
    pub const BOND: u8 = 4;
    pub const EMISSION: u8 = 5;
    pub const RECORD: u8 = 6;
}

/// Entries this old read as empty and are pruned.
pub const TRACE_LIFE: u64 = 256;
/// Direction value for "on this very tile".
pub const HERE: u8 = 4;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trace {
    pub kind: u8,
    /// Absolute direction from the trace tile to the actor after the action, or HERE.
    pub dir: u8,
    pub actual: u16,
    pub tick: u64,
    /// Order of resolution within the tick.
    pub seq: u32,
}

/// Absolute direction from tile a to tile b (HERE when equal).
fn dir_between(w: &crate::world::World, a: u32, b: u32) -> u8 {
    let ((ax, ay), (bx, by)) = (w.xy(a), w.xy(b));
    let (dx, dy) = (bx as i64 - ax as i64, by as i64 - ay as i64);
    if dx == 0 && dy == 0 {
        HERE
    } else if dx.abs() >= dy.abs() {
        if dx > 0 { 1 } else { 3 }
    } else if dy > 0 {
        2
    } else {
        0
    }
}

impl Sim {
    fn put_trace(&mut self, tile: u32, kind: u8, dir: u8, actual: u32) {
        let t = Trace { kind, dir, actual: actual.min(65535) as u16, tick: self.st.tick, seq: self.trace_seq };
        self.trace_seq += 1;
        let slots = self.st.traces.entry(tile).or_default();
        slots.copy_within(0..3, 1);
        slots[0] = t;
    }

    /// After a resolved effect: leave a trace where the world changed (LocalTrace capability).
    pub(crate) fn trace_effect(&mut self, i: usize, it: &Intent, res: Res) {
        if !self.caps.trace || !matches!(res.0, Outcome::Ok | Outcome::Partial) {
            return;
        }
        let me = self.tile_of(i);
        let w = &self.st.world;
        let orient = self.st.agents[i].orient;
        let r = it.r;
        let at = |t: Option<u32>, k: u8| t.map(|t| (t, k, dir_between(w, t, me)));
        let ext_tile = |addr: u16| -> Option<u32> {
            let (d, o) = unpack(addr)?;
            if matches!(d, dom::LOOSE | dom::PARCEL | dom::PORE | dom::ACTOR) {
                self.rel_tile(i, o)
            } else {
                None
            }
        };
        let vec_tile = || self.rel_vec_tile(i, r[1] as i16, r[2] as i16, 2).ok();
        // movers: the trace stays on the tile left, pointing where the actor went
        let left = |d: u8| w.neighbor(me, (d + 2) & 3).map(|t| (t, crate::traces::kind::FORCE, d));
        let found = match it.opc {
            op::TRANSFER => at(ext_tile(r[1]).or_else(|| ext_tile(r[0])), kind::MATTER),
            op::WORK => match r[0] {
                dom::SELF => Some((me, kind::WORK, HERE)),
                _ => at(vec_tile(), kind::WORK),
            },
            op::LINK | op::UNLINK => at(vec_tile(), kind::BOND),
            op::EMIT => Some((me, kind::EMISSION, HERE)),
            op::WRITE if r[0] == dom::PUBLIC_TAPE && it.sub <= 1 => at(vec_tile(), kind::RECORD),
            op::COPY => match unpack(r[1]) {
                Some((dom::PUBLIC_TAPE, o)) => at(self.rel_tile(i, o), kind::RECORD),
                _ => None,
            },
            op::DRIVE => match it.sub {
                sub::DRIVE_TRANSLATE => left((orient + r[5] as u8) & 3),
                crate::bonds::DRIVE_PUSH if !self.caps.contact => at(w.neighbor(me, orient), kind::FORCE),
                crate::bonds::DRIVE_PUSH | crate::contact::DRIVE_PUSH_ACTOR => left(orient),
                sub::DRIVE_ROTATE if self.grip_of(i).is_some() => Some((me, kind::FORCE, HERE)),
                _ => None,
            },
            _ => None,
        };
        if let Some((tile, k, dir)) = found {
            self.put_trace(tile, k, dir, res.1);
        }
    }

    /// Drop traces that can no longer be read (every TRACE_LIFE ticks).
    pub(crate) fn prune_traces(&mut self) {
        let now = self.st.tick;
        if !now.is_multiple_of(TRACE_LIFE) || self.st.traces.is_empty() {
            return;
        }
        self.st.traces.retain(|_, s| s.iter().any(|t| t.kind != 0 && t.tick + TRACE_LIFE > now));
    }
}

/// READ LocalTrace field (0 kind, 1 relative direction, 2 actual, 3 age) of slot `index`
/// (0 = most recent). Empty or expired slots read as 0.
pub fn read(slots: Option<&[Trace; 4]>, field: u16, index: u16, orient: u8, now: u64) -> Option<u16> {
    if index > 3 || field > 3 {
        return None;
    }
    let t = slots.map(|s| s[index as usize]).filter(|t| t.kind != 0 && now.saturating_sub(t.tick) < TRACE_LIFE);
    let Some(t) = t else { return Some(0) };
    Some(match field {
        0 => t.kind as u16,
        1 => {
            if t.dir == HERE {
                HERE as u16
            } else {
                ((t.dir + 4 - orient) & 3) as u16
            }
        }
        2 => t.actual,
        _ => (now - t.tick) as u16,
    })
}
