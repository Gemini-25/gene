//! Mutation plan drawn once per embryo at Finish. SPEC §12.

use crate::agent::MutationRecord;
use crate::genome::{self, Program};
use crate::rng::Stream;

/// Module-wise editable view.
struct Edit {
    entry_tag: u16,
    point: u16,
    structural: u16,
    weights: [u16; 8],
    mods: Vec<(u16, Vec<u32>)>,
}

impl Edit {
    fn from(p: &Program) -> Self {
        Edit {
            entry_tag: p.entry_tag,
            point: p.point_threshold,
            structural: p.structural_threshold,
            weights: p.weights,
            mods: p
                .modules
                .iter()
                .map(|m| (m.tag, p.words[m.start as usize..(m.start + m.len) as usize].to_vec()))
                .collect(),
        }
    }

    fn to_program(&self) -> Program {
        let mut modules = Vec::new();
        let mut words = Vec::new();
        for (tag, ws) in &self.mods {
            modules.push(genome::Module { tag: *tag, start: words.len() as u16, len: ws.len() as u16 });
            words.extend_from_slice(ws);
        }
        Program {
            entry_tag: self.entry_tag,
            point_threshold: self.point,
            structural_threshold: self.structural,
            weights: self.weights,
            modules,
            words,
        }
    }

    fn free_tag(&self) -> u16 {
        (0..=u16::MAX).find(|t| !self.mods.iter().any(|m| m.0 == *t)).unwrap_or(u16::MAX)
    }

    fn total_words(&self) -> usize {
        self.mods.iter().map(|m| m.1.len()).sum()
    }
}

pub const OP_NAMES: [&str; 8] =
    ["Replace", "Insert", "Delete", "DuplicateBlock", "MoveBlock", "DuplicateModule", "SplitMerge", "Recombine"];

/// Apply one structural operator. Returns false (Noop) when not applicable.
fn structural(e: &mut Edit, opn: u8, st: &mut Stream, donor: &Program, extra: Option<&Program>) -> bool {
    let nm = e.mods.len() as u32;
    match opn {
        0 => {
            let total = e.total_words() as u32;
            let mut k = st.below(total) as usize;
            let w = st.next_u32();
            for m in &mut e.mods {
                if k < m.1.len() {
                    m.1[k] = w;
                    return true;
                }
                k -= m.1.len();
            }
            false
        }
        1 => {
            let mi = st.below(nm) as usize;
            let pos = st.below(e.mods[mi].1.len() as u32 + 1) as usize;
            let k = st.range(1, 8);
            let ws: Vec<u32> = (0..k).map(|_| st.next_u32()).collect();
            e.mods[mi].1.splice(pos..pos, ws);
            true
        }
        2 => {
            let mi = st.below(nm) as usize;
            let len = e.mods[mi].1.len() as u32;
            if len < 2 {
                return false;
            }
            let k = st.range(1, 8.min(len - 1));
            let start = st.below(len - k + 1) as usize;
            e.mods[mi].1.drain(start..start + k as usize);
            true
        }
        3 => {
            let mi = st.below(nm) as usize;
            let len = e.mods[mi].1.len() as u32;
            let k = st.range(1, 16.min(len));
            let start = st.below(len - k + 1) as usize;
            let block: Vec<u32> = e.mods[mi].1[start..start + k as usize].to_vec();
            let tj = st.below(nm) as usize;
            let pos = st.below(e.mods[tj].1.len() as u32 + 1) as usize;
            e.mods[tj].1.splice(pos..pos, block);
            true
        }
        4 => {
            let mi = st.below(nm) as usize;
            let len = e.mods[mi].1.len() as u32;
            let k = st.range(1, 16.min(len));
            if k == len {
                return false;
            }
            let start = st.below(len - k + 1) as usize;
            let block: Vec<u32> = e.mods[mi].1.drain(start..start + k as usize).collect();
            let mut pos = st.below(len - k) as usize;
            if pos >= start {
                pos += 1;
            }
            e.mods[mi].1.splice(pos..pos, block);
            true
        }
        5 => {
            if e.mods.len() >= genome::MAX_MODULES {
                return false;
            }
            let mi = st.below(nm) as usize;
            let tag = e.free_tag();
            let ws = e.mods[mi].1.clone();
            e.mods.insert(mi + 1, (tag, ws));
            true
        }
        6 => {
            if st.next_u32() & 1 == 0 {
                if e.mods.len() >= genome::MAX_MODULES {
                    return false;
                }
                let mi = st.below(nm) as usize;
                let len = e.mods[mi].1.len() as u32;
                if len < 2 {
                    return false;
                }
                let b = st.range(1, len - 1) as usize;
                let tag = e.free_tag();
                let back = e.mods[mi].1.split_off(b);
                e.mods.insert(mi + 1, (tag, back));
                true
            } else {
                if nm < 2 {
                    return false;
                }
                let i = st.below(nm - 1) as usize;
                let (_, back) = e.mods.remove(i + 1);
                e.mods[i].1.extend(back);
                true
            }
        }
        7 => {
            // donors: inherited modules, then learned modules (M1b)
            let n_extra = extra.map_or(0, |x| x.modules.len());
            let src = st.below((donor.modules.len() + n_extra) as u32) as usize;
            let (p, k) = if src < donor.modules.len() { (donor, src) } else { (extra.unwrap(), src - donor.modules.len()) };
            let m = p.modules[k];
            let ws = p.words[m.start as usize..(m.start + m.len) as usize].to_vec();
            let tj = st.below(nm) as usize;
            e.mods[tj].1 = ws;
            true
        }
        _ => false,
    }
}

/// Mating (SPEC §20n): 1 finished embryo in MATE_ODDS that has a neighbour takes one module.
pub const MATE_ODDS: u32 = 8;

/// Mating (SPEC §20n): one module of `partner` (uniform) overwrites one module of the embryo's
/// validated prefix (uniform, tag kept), as Recombine does with the parent's own modules. Returns
/// the prefix unchanged when the result would break the limits.
pub fn mate(prefix: &[u16], partner: &Program, st: &mut Stream) -> Vec<u16> {
    let Ok(p) = genome::parse(prefix) else { return prefix.to_vec() };
    if partner.modules.is_empty() || p.modules.is_empty() {
        return prefix.to_vec();
    }
    let mut e = Edit::from(&p);
    let m = partner.modules[st.below(partner.modules.len() as u32) as usize];
    let tj = st.below(e.mods.len() as u32) as usize;
    e.mods[tj].1 = partner.words[m.start as usize..(m.start + m.len) as usize].to_vec();
    let np = e.to_program();
    if np.within_limits() {
        np.to_cells()
    } else {
        prefix.to_vec()
    }
}

/// Draw the mutation plan for a validated prefix.
/// `parent` supplies the rates (its inherited header) and the Recombine donor modules.
pub fn mutate(prefix: &[u16], parent: &Program, st: &mut Stream) -> (Vec<u16>, MutationRecord) {
    mutate_with(prefix, parent, None, st)
}

/// As `mutate`, with learned modules as additional Recombine donors (M1b).
pub fn mutate_with(
    prefix: &[u16],
    parent: &Program,
    learned: Option<&Program>,
    st: &mut Stream,
) -> (Vec<u16>, MutationRecord) {
    let mut rec = MutationRecord::default();
    let mut cells = prefix.to_vec();
    if let Ok(p) = genome::parse(prefix) {
        if st.next_u16() < parent.structural_threshold {
            let sum: u32 = parent.weights.iter().map(|w| *w as u32).sum();
            if sum > 0 {
                let mut pick = st.below(sum);
                let mut opn = 0u8;
                for (k, w) in parent.weights.iter().enumerate() {
                    if pick < *w as u32 {
                        opn = k as u8;
                        break;
                    }
                    pick -= *w as u32;
                }
                rec.structural_op = Some(opn);
                let mut e = Edit::from(&p);
                if structural(&mut e, opn, st, parent, learned) {
                    let np = e.to_program();
                    if np.within_limits() {
                        cells = np.to_cells();
                        rec.structural_applied = true;
                    }
                }
            }
        }
    }
    let pairs = cells.len().div_ceil(2);
    for k in 0..pairs {
        if st.next_u16() < parent.point_threshold {
            let single = 2 * k + 1 >= cells.len();
            let bit = st.below(if single { 16 } else { 32 }) as usize;
            let cell = 2 * k + bit / 16;
            cells[cell] ^= 1 << (bit % 16);
            rec.point_flips += 1;
        }
    }
    (cells, rec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::{stream_key, Domain};

    #[test]
    fn mating_puts_one_partner_module_in_place_of_one_module() {
        let me = crate::isa::assemble(crate::FOUNDER8M_ASM).unwrap();
        let partner = crate::isa::assemble(crate::FOUNDER10M_ASM).unwrap();
        let cells = me.to_cells();
        let partner_mods: Vec<Vec<u32>> =
            partner.modules.iter().map(|m| partner.words[m.start as usize..(m.start + m.len) as usize].to_vec()).collect();
        let mut changed = 0;
        for o in 0..40 {
            let mut st = Stream::new(stream_key(4, Domain::Mutation, 1, o), 1);
            let out = mate(&cells, &partner, &mut st);
            let p = genome::parse(&out).expect("valid");
            assert_eq!(p.modules.len(), me.modules.len(), "tags kept, no module added");
            let differs: Vec<usize> = (0..p.modules.len())
                .filter(|&k| {
                    let (a, b) = (p.modules[k], me.modules[k]);
                    p.words[a.start as usize..(a.start + a.len) as usize] != me.words[b.start as usize..(b.start + b.len) as usize]
                })
                .collect();
            assert!(differs.len() <= 1, "at most one module replaced");
            for &k in &differs {
                let m = p.modules[k];
                assert!(partner_mods.contains(&p.words[m.start as usize..(m.start + m.len) as usize].to_vec()), "it is a partner module");
                changed += 1;
            }
        }
        assert!(changed > 10, "mating usually changes the embryo ({changed} of 40)");
    }

    #[test]
    fn zero_rates_never_mutate() {
        let mut p = crate::sim::founder_program();
        p.point_threshold = 0;
        p.structural_threshold = 0;
        let cells = p.to_cells();
        for o in 0..50 {
            let mut st = Stream::new(stream_key(1, Domain::Mutation, 1, o), 5);
            let (out, rec) = mutate(&cells, &p, &mut st);
            assert_eq!(out, cells);
            assert_eq!(rec.point_flips, 0);
        }
    }

    #[test]
    fn every_operator_keeps_limits_or_noops() {
        let p = crate::sim::founder_program();
        for opn in 0..8u8 {
            for o in 0..40 {
                let mut st = Stream::new(stream_key(2, Domain::Mutation, opn as u64, o), 1);
                let mut e = Edit::from(&p);
                if structural(&mut e, opn, &mut st, &p, None) {
                    let np = e.to_program();
                    assert!(np.within_limits(), "op {opn}");
                    let cells = np.to_cells();
                    assert_eq!(genome::parse(&cells).map(|_| ()).is_ok(), np.module_index(np.entry_tag).is_some());
                }
            }
        }
    }

    #[test]
    fn always_structural_changes_something() {
        let mut p = crate::sim::founder_program();
        p.structural_threshold = u16::MAX;
        p.point_threshold = 0;
        p.weights = [1, 0, 0, 0, 0, 0, 0, 0];
        let cells = p.to_cells();
        let mut st = Stream::new(stream_key(3, Domain::Mutation, 1, 1), 1);
        let (out, rec) = mutate(&cells, &p, &mut st);
        assert_eq!(rec.structural_op, Some(0));
        assert!(rec.structural_applied);
        assert_eq!(out.len(), cells.len());
    }
}
