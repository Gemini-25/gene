//! High-level behaviour judge (SPEC §17f; user decision 3 of 2026-10-10). Observer only.
//!
//! B (filter): a repeated behaviour family counts only when its effects mostly succeed and it is
//! tied to success: its carriers have more children per day of life than all agents with a
//! classified window (so that short-lived newborns do not flatter every family), or it spread to
//! another founder type (at least SPREAD_CARRIERS carriers in each of two types) without its
//! carriers reproducing less than the population. Founders that share code can each fall into
//! the same failure (a broken reproduction loop in carriers with no children); that is not
//! spread.
//! C (hierarchy): a high-level behaviour is an ordered chain of 2 or 3 families that follow each
//! other in one agent's consecutive windows, all passing B, used by several agents, in several
//! situations (time of day, the agent's energy band, a wet or dry tile), and leaving an output
//! (matter moved, a structure or a record touched).
//! New: a family or chain is the founders' own repertoire when at least a tenth of its windows
//! come from agents whose genome is still exactly a founder genome; what founder code does in
//! any situation is not counted as new (user decision 2). A family is also the founders' own
//! when every ordered triple of successful actions in its example (ignoring direction, amount
//! and timing) was seen in some window of founder code: a founder routine with other timings
//! is not a new behaviour. D (a person checking on screen) happens in the viewer.

use crate::behavior::{action_grams, action_set, Family, WindowRecord, UNKNOWN_SITUATION, WINDOW};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashSet};

/// Share of windows from unchanged founder genomes at which a behaviour is the founders' own.
pub const FOUNDER_SHARE: (u64, u64) = (1, 10);
pub const MIN_SUCCESS: (u64, u64) = (1, 2);
/// Carriers must reproduce at least 1.1 times the population rate (or spread to another type).
pub const FITNESS: (u64, u64) = (11, 10);
pub const CHAIN_AGENTS: usize = 5;
/// Carriers a founder type needs for a family to count as spread to it.
pub const SPREAD_CARRIERS: usize = 3;
/// Carriers a family needs before its reproduction rate means anything.
pub const FITNESS_CARRIERS: usize = 5;
/// Distinct situations (time of day × energy band × wet tile) a chain must start in.
pub const CHAIN_CONTEXTS: u32 = 2;
/// Agent ids kept per chain (enough to count CHAIN_AGENTS and to show a few on screen).
const KEEP_AGENTS: usize = 32;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Chain {
    pub families: Vec<u32>,
    pub first_window: u64,
    pub occurrences: u64,
    pub agents: Vec<u64>,
    pub agent_count: usize,
    /// Bit k: seen in day quarter k (dawn-morning, afternoon, evening, night).
    pub phases: u8,
    /// Bit 8·phase + situation: the situations the chain started in (known ones only).
    pub contexts: u32,
    /// Occurrences with an output in one of their windows.
    pub outputs: u64,
    /// Occurrences by agents with an unchanged founder genome.
    pub founder_occurrences: u64,
}

#[derive(Clone, Debug)]
pub struct FamilyJudgement {
    pub id: u32,
    pub ok: u64,
    pub total: u64,
    pub carriers: usize,
    /// Children per carrier-day over children per agent-day, ×1000.
    pub fitness_milli: u64,
    pub types: usize,
    /// Windows by agents with an unchanged founder genome.
    pub founder_windows: u64,
    pub windows: u64,
    pub founder: bool,
    pub pass: bool,
}

#[derive(Clone, Debug)]
pub struct Candidate {
    pub chain: Chain,
    /// The chain itself or one of its families is not the founders' own.
    pub new: bool,
    /// None when the chain passes C; otherwise the first rule it fails.
    pub reject: Option<&'static str>,
}

impl Candidate {
    pub fn pass(&self) -> bool {
        self.reject.is_none()
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct Life {
    kind: u16,
    born: u64,
    died: Option<u64>,
    children: u64,
    /// The genome is exactly one of the founder genomes.
    founder_code: bool,
    /// Has at least one window in a family.
    classified: bool,
}

#[derive(Default, Serialize, Deserialize)]
struct FamilyStats {
    ok: u64,
    total: u64,
    windows: u64,
    founder_windows: u64,
    carriers: BTreeSet<u64>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Judge {
    stats: BTreeMap<u32, FamilyStats>,
    /// Per agent: the last two classified windows (window, family, output, situation).
    last: BTreeMap<u64, [(u64, u32, bool, u8); 2]>,
    chains: BTreeMap<Vec<u32>, Chain>,
    outputs: HashSet<(u64, u64)>,
    newest_window: u64,
    life: BTreeMap<u64, Life>,
    founder_genomes: BTreeSet<[u8; 32]>,
    /// Action 3-grams seen in windows of founder code.
    founder_grams: HashSet<u32>,
}

fn phase(window: u64) -> u8 {
    (((window * WINDOW + 1024) % 4096) / 1024) as u8
}

impl Judge {
    /// A founder at tick 0 with its founder type (which public ancestor it is) and genome.
    pub fn founder(&mut self, uid: u64, kind: u16, genome: [u8; 32]) {
        self.founder_genomes.insert(genome);
        self.life.insert(uid, Life { kind, born: 0, died: None, children: 0, founder_code: true, classified: false });
    }

    /// A founder genome with no founder agent alive (a loaded state).
    pub fn founder_genome(&mut self, genome: [u8; 32]) {
        self.founder_genomes.insert(genome);
    }

    /// An agent already alive when observation starts (a loaded state). Register founder
    /// genomes first.
    pub fn known(&mut self, uid: u64, kind: u16, born: u64, genome: [u8; 32]) {
        let founder_code = self.founder_genomes.contains(&genome);
        self.life.insert(uid, Life { kind, born, died: None, children: 0, founder_code, classified: false });
    }

    pub fn birth(&mut self, child: u64, parent: u64, tick: u64, genome: [u8; 32]) {
        let kind = self.life.get(&parent).map_or(0, |l| l.kind);
        if let Some(l) = self.life.get_mut(&parent) {
            l.children += 1;
        }
        let founder_code = self.founder_genomes.contains(&genome);
        self.life.insert(child, Life { kind, born: tick, died: None, children: 0, founder_code, classified: false });
    }

    pub fn death(&mut self, uid: u64, tick: u64) {
        if let Some(l) = self.life.get_mut(&uid) {
            l.died = Some(tick);
        }
    }

    /// The agent changed the world outside itself in this tick (relation fact).
    pub fn output(&mut self, uid: u64, tick: u64) {
        self.outputs.insert((uid, tick / WINDOW));
    }

    pub fn window(&mut self, r: &WindowRecord) {
        if r.window > self.newest_window {
            self.newest_window = r.window;
            // outputs of long-closed windows are no longer needed
            if self.newest_window % 64 == 0 {
                let keep = self.newest_window.saturating_sub(8);
                self.outputs.retain(|&(_, w)| w >= keep);
                // agents silent for this long (mostly dead) can no longer extend a chain
                self.last.retain(|_, s| s[1].0 >= keep);
            }
        }
        let Some(fam) = r.family else { return };
        let founder_code = match self.life.get_mut(&r.uid) {
            Some(l) => {
                l.classified = true;
                l.founder_code
            }
            None => false,
        };
        if founder_code {
            self.founder_grams.extend(r.grams.iter().copied().filter(|&g| g != 0));
        }
        let s = self.stats.entry(fam).or_default();
        s.ok += r.ok as u64;
        s.total += r.total as u64;
        s.windows += 1;
        s.founder_windows += founder_code as u64;
        s.carriers.insert(r.uid);
        let prev = self.last.get(&r.uid).copied();
        let here = r.output || self.outputs.contains(&(r.uid, r.window));
        let cur = (r.window, fam, here, r.ctx);
        let out = |w: (u64, u32, bool, u8)| w.2 || self.outputs.contains(&(r.uid, w.0));
        let mut found: Vec<(Vec<u32>, u64, bool, u8)> = Vec::new();
        if let Some([a, b]) = prev {
            if b.0 != u64::MAX && b.0 + 1 == r.window && b.1 != fam {
                found.push((vec![b.1, fam], b.0, out(b) || here, b.3));
                if a.0 != u64::MAX && a.0 + 1 == b.0 && a.1 != b.1 {
                    found.push((vec![a.1, b.1, fam], a.0, out(a) || out(b) || here, a.3));
                }
            }
        }
        for (key, first, output, ctx) in found {
            let c = self.chains.entry(key.clone()).or_insert_with(|| Chain { families: key, first_window: first, ..Chain::default() });
            c.occurrences += 1;
            c.phases |= 1 << phase(first);
            if ctx != UNKNOWN_SITUATION {
                c.contexts |= 1 << (8 * phase(first) as u32 + (ctx & 7) as u32);
            }
            c.outputs += output as u64;
            c.founder_occurrences += founder_code as u64;
            if !c.agents.contains(&r.uid) {
                c.agent_count += 1;
                if c.agents.len() < KEEP_AGENTS {
                    c.agents.push(r.uid);
                }
            }
        }
        let slots = self.last.entry(r.uid).or_insert([(u64::MAX, 0, false, UNKNOWN_SITUATION); 2]);
        *slots = [slots[1], cur];
    }

    /// Judge every repeated family (B) and every chain (C) at `end`. Chains used by at least two
    /// agents are returned, passing ones first, each with the rule it fails if any.
    pub fn finish(&self, families: &[Family], end: u64) -> (Vec<FamilyJudgement>, Vec<Candidate>) {
        // children per agent-day for everyone, and per carrier set
        let days = |uids: &mut dyn Iterator<Item = &u64>| -> (u64, u64) {
            let mut kids = 0;
            let mut ticks = 0;
            for u in uids {
                if let Some(l) = self.life.get(u) {
                    kids += l.children;
                    ticks += l.died.unwrap_or(end).saturating_sub(l.born).max(1);
                }
            }
            (kids, ticks)
        };
        let founders_own = |part: u64, all: u64| part * FOUNDER_SHARE.1 >= all * FOUNDER_SHARE.0;
        let (all_k, all_t) = days(&mut self.life.iter().filter(|(_, l)| l.classified).map(|(u, _)| u));
        let mut out = Vec::new();
        let mut pass: BTreeMap<u32, bool> = BTreeMap::new();
        let mut founder: BTreeMap<u32, bool> = BTreeMap::new();
        for f in families.iter().filter(|f| f.repeated_at.is_some()) {
            let Some(st) = self.stats.get(&f.id) else { continue };
            let (ok, total, carriers) = (&st.ok, &st.total, &st.carriers);
            let (k, t) = days(&mut carriers.iter());
            // (k / t) / (all_k / all_t), ×1000
            let fitness_milli = if t == 0 || all_k == 0 { 0 } else { (k as u128 * all_t as u128 * 1000 / (t as u128 * all_k as u128)) as u64 };
            let mut per_type: BTreeMap<u16, usize> = BTreeMap::new();
            for u in carriers {
                if let Some(l) = self.life.get(u) {
                    *per_type.entry(l.kind).or_default() += 1;
                }
            }
            let types: BTreeSet<u16> = per_type.iter().filter(|(_, n)| **n >= SPREAD_CARRIERS).map(|(k, _)| *k).collect();
            let success = ok * MIN_SUCCESS.1 >= total * MIN_SUCCESS.0;
            let fit = carriers.len() >= FITNESS_CARRIERS && fitness_milli * FITNESS.1 >= 1000 * FITNESS.0;
            // spread counts only when the carriers do not do worse than everyone else
            let spread = types.len() >= 2 && fitness_milli >= 1000;
            let linked = fit || spread;
            let grams = action_grams(&f.example);
            let known_actions = !self.founder_grams.is_empty() && grams.iter().all(|g| self.founder_grams.contains(g));
            let is_founder = founders_own(st.founder_windows, st.windows) || known_actions;
            pass.insert(f.id, success && linked);
            founder.insert(f.id, is_founder);
            out.push(FamilyJudgement {
                id: f.id,
                ok: *ok,
                total: *total,
                carriers: carriers.len(),
                fitness_milli,
                types: types.len(),
                founder_windows: st.founder_windows,
                windows: st.windows,
                founder: is_founder,
                pass: success && linked,
            });
        }
        // what each step of a chain does (successful action kinds), and the step sequences of
        // chains founder code runs: a chain whose steps do the same is a founder chain with
        // other timings or a failing action slipped in, not a new combination
        let by_id: BTreeMap<u32, &Family> = families.iter().map(|f| (f.id, f)).collect();
        let steps = |c: &Chain| -> Vec<Vec<u8>> { c.families.iter().map(|f| by_id.get(f).map_or(vec![], |f| action_set(&f.example))).collect() };
        let founder_steps: BTreeSet<Vec<Vec<u8>>> =
            self.chains.values().filter(|c| c.founder_occurrences > 0 && founders_own(c.founder_occurrences, c.occurrences)).map(steps).collect();
        let mut cands = Vec::new();
        for c in self.chains.values().filter(|c| c.agent_count >= 2) {
            let reject = if !c.families.iter().all(|f| pass.contains_key(f)) {
                Some("family not repeated")
            } else if !c.families.iter().all(|f| pass.get(f) == Some(&true)) {
                Some("family filtered")
            } else if c.agent_count < CHAIN_AGENTS {
                Some("few agents")
            } else if c.contexts.count_ones() < CHAIN_CONTEXTS {
                Some("one situation")
            } else if c.outputs * 2 < c.occurrences {
                Some("no output")
            } else {
                None
            };
            let new_family = c.families.iter().any(|f| founder.get(f) == Some(&false));
            let new = new_family || (!founders_own(c.founder_occurrences, c.occurrences) && !founder_steps.contains(&steps(c)));
            cands.push(Candidate { chain: c.clone(), new, reject });
        }
        cands.sort_by_key(|c| (c.reject.is_some(), std::cmp::Reverse(c.new), std::cmp::Reverse(c.chain.agent_count), c.chain.families.clone()));
        (out, cands)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec(uid: u64, window: u64, family: u32, ok: u16) -> WindowRecord {
        WindowRecord { uid, window, family: Some(family), ok, total: 10, output: false, ctx: 2, grams: [0; 8] }
    }

    fn fam(id: u32, first_tick: u64) -> Family {
        Family {
            id,
            sketch: vec![],
            first_tick,
            first_uid: 0,
            last_tick: 0,
            windows: 10,
            near: 0,
            agents: Default::default(),
            repeated_at: Some(first_tick),
            example: vec![],
            contexts: Default::default(),
        }
    }

    /// Agents 1..=n run families 7 then 8 in consecutive windows, at two times of day, with
    /// output; family 9 is a failing loop. With `mutants`, agents 1..=n are children with a
    /// changed genome; otherwise they are the founders themselves.
    fn world(n: u64, start_window: u64, mutants: bool) -> Judge {
        let mut j = Judge::default();
        for u in 1..=20 {
            j.founder(if mutants { 100 + u } else { u }, (u % 2) as u16, [0; 32]);
        }
        for u in 1..=n {
            if mutants {
                j.birth(u, 100 + u, 0, [1; 32]);
            }
            for (k, w) in [start_window, start_window + 8].into_iter().enumerate() {
                j.output(u, w * WINDOW + 3);
                j.window(&rec(u, w, 7, 9));
                j.window(&rec(u, w + 1, 8, 8));
                j.window(&rec(u, w + 2, 9, 1));
                if k == 0 {
                    j.birth(1000 + u, u, w * WINDOW, [2; 32]); // carriers reproduce
                }
            }
        }
        j
    }

    #[test]
    fn a_new_chain_of_successful_families_is_a_candidate() {
        let j = world(6, 40, true);
        let fams = [fam(7, 40 * WINDOW), fam(8, 41 * WINDOW), fam(9, 42 * WINDOW)];
        let (judged, cands) = j.finish(&fams, 100 * WINDOW);
        let by = |id| judged.iter().find(|f| f.id == id).unwrap();
        assert!(by(7).pass && by(8).pass);
        assert!(!by(9).pass, "a failing loop is filtered out");
        let cands: Vec<_> = cands.into_iter().filter(|c| c.pass()).collect();
        assert_eq!(cands.len(), 1, "{cands:?}");
        assert_eq!(cands[0].chain.families, vec![7, 8]);
        assert!(cands[0].new);
        assert_eq!(cands[0].chain.agent_count, 6);
    }

    #[test]
    fn spread_without_children_is_not_linked_to_success() {
        // founder types that share code can each fall into the same failure; carriers with no
        // children in two types are not a behaviour spreading
        let mut j = world(6, 40, true);
        for u in 201..=206u64 {
            j.birth(u, u - 100, 0, [3; 32]);
            for w in 60..63 {
                j.window(&rec(u, w, 5, 9));
            }
        }
        let (judged, _) = j.finish(&[fam(5, 60 * WINDOW), fam(7, 40 * WINDOW)], 100 * WINDOW);
        let f5 = judged.iter().find(|f| f.id == 5).unwrap();
        assert_eq!((f5.types, f5.fitness_milli), (2, 0));
        assert!(!f5.pass);
    }

    #[test]
    fn too_few_agents_is_not_a_candidate() {
        let j = world(4, 40, true);
        let fams = [fam(7, 40 * WINDOW), fam(8, 41 * WINDOW), fam(9, 42 * WINDOW)];
        assert!(j.finish(&fams, 100 * WINDOW).1.iter().all(|c| c.reject == Some("few agents") || c.reject == Some("family filtered")));
    }

    #[test]
    fn a_founder_routine_with_other_timings_is_not_a_new_family() {
        let mut j = world(6, 40, true);
        // founder code (agent 101) once ran TRANSFER, TRANSFER, TRANSFER in a row
        let take: crate::behavior::Token = [40, 0, 255, 3, 2, 0, 0, 0];
        let gram = action_grams(&[take, take, take])[0];
        j.window(&WindowRecord { uid: 101, window: 5, family: Some(99), ok: 3, total: 3, output: true, ctx: 2, grams: [gram, 0, 0, 0, 0, 0, 0, 0] });
        let mut f7 = fam(7, 40 * WINDOW);
        f7.example = vec![take, [40, 0, 1, 3, 2, 3, 0, 0], [40, 1, 2, 3, 1, 5, 0, 0]]; // other directions and timings
        let mut f8 = fam(8, 41 * WINDOW);
        f8.example = vec![[32, 0, 255, 255, 1, 0, 0, 0]; 3]; // LINK x3: never done by founder code
        let (judged, _) = j.finish(&[f7, f8, fam(9, 42 * WINDOW)], 100 * WINDOW);
        let by = |id| judged.iter().find(|f| f.id == id).unwrap();
        assert!(by(7).founder, "same actions in the same order as founder code");
        assert!(!by(8).founder, "a new order of successful actions");
    }

    #[test]
    fn a_founder_chain_with_a_failing_action_slipped_in_is_not_new() {
        let mut j = Judge::default();
        for u in 1..=20 {
            j.founder(100 + u, 0, [0; 32]);
        }
        let alloc: crate::behavior::Token = [64, 0, 255, 255, 1, 0, 0, 0];
        let grow: crate::behavior::Token = [65, 0, 255, 255, 1, 2, 0, 0];
        let bad_step: crate::behavior::Token = [67, 8, 255, 255, 0, 1, 0, 0];
        let copy: crate::behavior::Token = [72, 0, 255, 255, 2, 1, 0, 0];
        // founder code runs 7 > 8 (allocate and grow, then copy); mutants run 17 > 8, where 17
        // is 7 with a failing step in the middle
        for u in 1..=6 {
            let f = 100 + u;
            for (k, w) in [10u64, 18].into_iter().enumerate() {
                j.output(f, w * WINDOW + 1);
                j.window(&rec(f, w, 7, 9));
                j.window(&rec(f, w + 1, 8, 9));
                let m = u;
                if k == 0 {
                    j.birth(m, f, 0, [1; 32]);
                }
                j.output(m, (w + 40) * WINDOW + 1);
                j.window(&rec(m, w + 40, 17, 9));
                j.window(&rec(m, w + 41, 8, 9));
                j.birth(1000 + 10 * u + k as u64, m, (w + 40) * WINDOW, [1; 32]);
            }
        }
        // founder windows have shown these action triples
        let g = action_grams(&[alloc, grow, grow])[0];
        j.window(&WindowRecord { uid: 101, window: 90, family: Some(7), ok: 4, total: 4, output: true, ctx: 2, grams: [g, 0, 0, 0, 0, 0, 0, 0] });
        let mut f7 = fam(7, 10 * WINDOW);
        f7.example = vec![alloc, grow, grow, grow];
        let mut f17 = fam(17, 50 * WINDOW);
        f17.example = vec![alloc, grow, bad_step, grow];
        let mut f8 = fam(8, 11 * WINDOW);
        f8.example = vec![copy; 4];
        let (_, cands) = j.finish(&[f7, f8, f17], 100 * WINDOW);
        let c = cands.iter().find(|c| c.chain.families == vec![17, 8]).expect("chain 17 > 8");
        assert_eq!(c.chain.founder_occurrences, 0);
        assert!(!c.new, "same steps as the founder chain 7 > 8");
    }

    #[test]
    fn the_founders_repertoire_is_not_new() {
        let j = world(6, 40, false);
        let fams = [fam(7, 40 * WINDOW), fam(8, 41 * WINDOW), fam(9, 42 * WINDOW)];
        let (_, cands) = j.finish(&fams, 100 * WINDOW);
        let cands: Vec<_> = cands.into_iter().filter(|c| c.pass()).collect();
        assert_eq!(cands.len(), 1);
        assert!(!cands[0].new, "what unchanged founder code does is the founders' own, at any time");
    }
}
