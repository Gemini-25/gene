//! Artifact identity and repetition (original §58 "artifact의 동일성과 반복"). Observer only.
//!
//! A pattern is the content of the 13 tiles within Manhattan distance 2 of a parcel, serialized
//! in (dy, dx) order without coordinates, UIDs or makers, and hashed. An occurrence is (pattern,
//! set of parcel UIDs in the footprint): seeing the same parcels again is not a new making.
//! Occurrences that share no parcel are independent. A pattern with three independent
//! occurrences built by at least two agents (matrix deposits, compression, bonds) is a repeated
//! artifact.

use crate::relations::{act, RelFact, PARCEL, USE};
use crate::world::World;
use std::collections::{BTreeMap, BTreeSet};

/// Footprint offsets, dy ascending then dx ascending.
pub const FOOTPRINT: [(i32, i32); 13] = [
    (0, -2),
    (-1, -1),
    (0, -1),
    (1, -1),
    (-2, 0),
    (-1, 0),
    (0, 0),
    (1, 0),
    (2, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (0, 2),
];
/// Bytes per footprint tile: outside, present, matrix bins 10, pore bins 10, c >> 4, orient,
/// bond bins 4, anchor bin, public record present.
pub const TILE_BYTES: usize = 30;
pub const REPEAT_OCCURRENCES: u32 = 3;
pub const REPEAT_AGENTS: usize = 2;

#[derive(Clone, Debug, Default)]
pub struct Pattern {
    pub hash: u64,
    pub first_tick: u64,
    /// Parcels in the footprint, and bonded faces between them (each bond once).
    pub parcels: u8,
    pub bonds: u8,
    /// Only the centre parcel: no bonds, no compression, no record.
    pub trivial: bool,
    /// Distinct occurrences seen, and how many of them share no parcel with each other.
    pub occurrences: u64,
    pub independent: u32,
    /// Agents whose building acts made parcels of the independent occurrences.
    pub contributors: BTreeSet<u64>,
    /// No building act is known for any parcel of the independent occurrences.
    pub natural: bool,
    pub repeated_at: Option<u64>,
    /// Serialized footprint of the first occurrence (13 × TILE_BYTES).
    pub example: Vec<u8>,
    used: BTreeSet<u64>,
}

#[derive(Default)]
pub struct Artifacts {
    builders: BTreeMap<u64, BTreeSet<u64>>,
    pub patterns: BTreeMap<u64, Pattern>,
    seen: BTreeSet<(u64, Vec<u64>)>,
    /// Patterns in the order they became repeated.
    pub repeated: Vec<u64>,
}

/// min(16, floor(log2(1 + n))).
fn bin(n: u64) -> u8 {
    (63 - n.saturating_add(1).leading_zeros()).min(16) as u8
}

/// The footprint around tile t: bytes and the parcel UIDs in it (sorted).
pub fn footprint(w: &World, has_record: &dyn Fn(u32) -> bool, t: u32) -> (Vec<u8>, Vec<u64>) {
    let mut out = Vec::with_capacity(13 * TILE_BYTES);
    let mut uids = Vec::new();
    for &(dx, dy) in &FOOTPRINT {
        let mut b = [0u8; TILE_BYTES];
        match w.offset(t, dx, dy) {
            None => b[0] = 1,
            Some(u) => {
                if let Some(p) = w.parcels.get(&u) {
                    uids.push(p.uid);
                    b[1] = 1;
                    for s in 0..10 {
                        b[2 + s] = bin(p.matrix[s] as u64);
                        b[12 + s] = bin(p.pore[s] as u64);
                    }
                    b[22] = p.c >> 4;
                    b[23] = p.orient;
                    for f in 0..4 {
                        b[24 + f] = bin(p.bond[f] as u64);
                    }
                    b[28] = bin(p.anchor as u64);
                }
                b[29] = has_record(u) as u8;
            }
        }
        out.extend_from_slice(&b);
    }
    uids.sort_unstable();
    (out, uids)
}

pub fn pattern_hash(bytes: &[u8]) -> u64 {
    let mut h = blake3::Hasher::new();
    h.update(b"artifact-a1.4\0");
    h.update(bytes);
    u64::from_le_bytes(h.finalize().as_bytes()[..8].try_into().unwrap())
}

impl Artifacts {
    /// Building acts tell who made which parcel.
    pub fn push(&mut self, f: &RelFact) {
        if f.kind == USE && f.to.0 == PARCEL && act::builds(f.channel) {
            self.builders.entry(f.to.1).or_default().insert(f.from.1);
        }
    }

    /// Look at every parcel's footprint. Returns the patterns that became repeated now.
    pub fn scan(&mut self, w: &World, has_record: &dyn Fn(u32) -> bool, tick: u64) -> Vec<u64> {
        let mut now = Vec::new();
        for &t in w.parcels.keys() {
            let (bytes, uids) = footprint(w, has_record, t);
            let hash = pattern_hash(&bytes);
            if !self.seen.insert((hash, uids.clone())) {
                continue;
            }
            let p = self.patterns.entry(hash).or_insert_with(|| {
                let parcels = bytes.chunks(TILE_BYTES).filter(|c| c[1] == 1).count() as u8;
                // count each bond once: east (face 1) and south (face 2) of every tile
                let bonds = bytes.chunks(TILE_BYTES).filter(|c| c[1] == 1).map(|c| (c[25] > 0) as u8 + (c[26] > 0) as u8).sum();
                let centre = &bytes[6 * TILE_BYTES..7 * TILE_BYTES];
                let trivial = parcels == 1 && centre[22] == 0 && centre[24..29].iter().all(|&x| x == 0) && centre[29] == 0;
                Pattern { hash, first_tick: tick, parcels, bonds, trivial, natural: true, example: bytes.clone(), ..Pattern::default() }
            });
            p.occurrences += 1;
            if uids.iter().any(|u| p.used.contains(u)) {
                continue;
            }
            p.independent += 1;
            p.used.extend(uids.iter().copied());
            for u in &uids {
                if let Some(b) = self.builders.get(u) {
                    p.contributors.extend(b.iter().copied());
                    p.natural = false;
                }
            }
            if p.repeated_at.is_none() && p.independent >= REPEAT_OCCURRENCES && p.contributors.len() >= REPEAT_AGENTS {
                p.repeated_at = Some(tick);
                self.repeated.push(hash);
                now.push(hash);
            }
        }
        now
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::Parcel;

    fn world() -> World {
        crate::world::generate(1, 64, 64)
    }

    fn put(w: &mut World, x: u32, y: u32, uid: u64, s: u16) {
        let t = w.idx(x, y);
        let mut p = Parcel::new(uid);
        p.matrix[crate::species::S] = s;
        w.parcels.insert(t, p);
    }

    fn built(a: &mut Artifacts, parcel: u64, agent: u64) {
        a.push(&RelFact { tick: 0, from: (0, agent), to: (PARCEL, parcel), kind: USE, channel: act::MATRIX_IN, amount: 1, tile: 0 });
    }

    #[test]
    fn same_shape_elsewhere_is_the_same_pattern() {
        let mut w = world();
        put(&mut w, 10, 10, 1, 8);
        put(&mut w, 11, 10, 2, 8);
        put(&mut w, 40, 40, 3, 8);
        put(&mut w, 41, 40, 4, 8);
        let none = |_| false;
        let (a, ua) = footprint(&w, &none, w.idx(10, 10));
        let (b, ub) = footprint(&w, &none, w.idx(40, 40));
        assert_eq!(pattern_hash(&a), pattern_hash(&b), "no coordinates or UIDs in the content");
        assert_eq!((ua, ub), (vec![1, 2], vec![3, 4]));
        let (c, _) = footprint(&w, &none, w.idx(11, 10));
        assert_ne!(pattern_hash(&a), pattern_hash(&c), "the pair seen from its other end differs");
    }

    #[test]
    fn three_independent_builds_by_two_agents_repeat() {
        let mut w = world();
        let mut arts = Artifacts::default();
        for (k, x) in [10u32, 20, 30].iter().enumerate() {
            let uid = 100 + k as u64;
            put(&mut w, *x, 10, uid, 4);
            built(&mut arts, uid, if k == 0 { 7 } else { 8 });
        }
        let none = |_| false;
        let now = arts.scan(&w, &none, 4096);
        assert_eq!(now.len(), 1);
        let p = &arts.patterns[&now[0]];
        assert_eq!((p.independent, p.contributors.len(), p.parcels, p.trivial), (3, 2, 1, true));
        // looking again is not a new making
        assert!(arts.scan(&w, &none, 8192).is_empty());
        assert_eq!(arts.patterns[&now[0]].occurrences, 3);
    }

    #[test]
    fn one_maker_is_not_enough_and_unbuilt_is_natural() {
        let mut w = world();
        let mut arts = Artifacts::default();
        for (k, x) in [10u32, 20, 30].iter().enumerate() {
            put(&mut w, *x, 10, 100 + k as u64, 4);
            built(&mut arts, 100 + k as u64, 7);
        }
        put(&mut w, 50, 50, 200, 32);
        let none = |_| false;
        assert!(arts.scan(&w, &none, 4096).is_empty());
        let lone = arts.patterns.values().find(|p| p.contributors.is_empty()).unwrap();
        assert!(lone.natural);
    }
}
