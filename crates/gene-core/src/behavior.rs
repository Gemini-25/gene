//! Behaviour families from resolved effects (original §58, analysis version A1.4).
//!
//! Observer only: the analysis reads `EffectFact`s and never feeds anything back into the world.
//! Each agent's effects in a 256-tick window become 8-byte tokens; consecutive token triples are
//! hashed (BLAKE3 over "behavior-a1.4\0" + 24 bytes), and the 128 smallest distinct hashes form
//! the window's bottom-k sketch. Two sketches are compared with the union bottom-k estimator Jhat.
//! A window joins the best family with Jhat >= 3/4, founds a new family when Jhat < 1/4 against
//! every candidate, and otherwise stays "intermediate" (similar but not the same). A family seen
//! in 3 windows from at least 2 agents is a RepeatedBehavior.

use crate::observer::EffectFact;
use std::collections::{BTreeMap, BTreeSet, HashMap};

pub const WINDOW: u64 = 256;
pub const SKETCH: usize = 128;
/// Hashes of a sketch that go into the inverted index.
const INDEX_K: usize = 8;
const MAX_CANDIDATES: usize = 1024;
/// Archive size up to which every family is compared.
const FULL_SCAN: usize = 1536;

pub type Token = [u8; 8];

/// floor(log2(1 + actual)), at most 16.
pub fn magnitude(actual: u32) -> u8 {
    (63 - (actual as u64 + 1).leading_zeros()).min(16) as u8
}

/// Bins of the gap to the agent's previous effect: 0, 1, 2..3, 4..7, 8..15, 16+.
pub fn delay_bin(d: u64) -> u8 {
    match d {
        0 => 0,
        1 => 1,
        2..=3 => 2,
        4..=7 => 3,
        8..=15 => 4,
        _ => 5,
    }
}

pub fn token(f: &EffectFact, delay: u8, first: bool) -> Token {
    [f.kind, f.outcome, f.dir, f.channel, magnitude(f.actual), delay, first as u8, 0]
}

/// Bottom-k sketch of a window's token 3-grams (sorted, distinct). Empty below 3 tokens.
pub fn sketch(tokens: &[Token]) -> Vec<u64> {
    let mut hs: Vec<u64> = tokens
        .windows(3)
        .map(|w| {
            let mut h = blake3::Hasher::new();
            h.update(b"behavior-a1.4\0");
            for t in w {
                h.update(t);
            }
            u64::from_le_bytes(h.finalize().as_bytes()[..8].try_into().unwrap())
        })
        .collect();
    hs.sort_unstable();
    hs.dedup();
    hs.truncate(SKETCH);
    hs
}

/// Union bottom-k comparison: (I, |U|) with U the smallest min(k, |A ∪ B|) hashes of the union
/// and I how many of them are in both. Jhat = I / |U|.
pub fn jhat(a: &[u64], b: &[u64]) -> (usize, usize) {
    let (mut i, mut j, mut u, mut both) = (0, 0, 0, 0);
    while u < SKETCH && (i < a.len() || j < b.len()) {
        match (a.get(i), b.get(j)) {
            (Some(x), Some(y)) if x == y => {
                both += 1;
                i += 1;
                j += 1;
            }
            (Some(x), Some(y)) if x < y => i += 1,
            (Some(_), Some(_)) => j += 1,
            (Some(_), None) => i += 1,
            (None, _) => j += 1,
        }
        u += 1;
    }
    (both, u)
}

#[derive(Clone, Debug)]
pub struct Family {
    pub id: u32,
    /// Sketch of the founding window (the family's representative; never replaced).
    pub sketch: Vec<u64>,
    pub first_tick: u64,
    pub first_uid: u64,
    pub last_tick: u64,
    pub windows: u64,
    /// Windows that were similar (1/4 <= Jhat < 3/4) with this as the best match.
    pub near: u64,
    pub agents: BTreeSet<u64>,
    pub repeated_at: Option<u64>,
    /// First tokens of the founding window, for a readable description.
    pub example: Vec<Token>,
}

/// Use state of a family (original §58 기술 상태), from when it was last seen. Only behaviour
/// evidence is used here; records and artifacts are reported separately.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UseState {
    /// Seen within the last 4096 ticks.
    LiveUsers,
    /// Last seen 4096..3 * 4096 ticks ago.
    Fading,
    /// Not seen for three 4096-tick windows.
    NotObserved,
}

pub const USE_WINDOW: u64 = 4096;

impl Family {
    pub fn use_state(&self, now: u64) -> UseState {
        let gap = now.saturating_sub(self.last_tick);
        if gap < USE_WINDOW {
            UseState::LiveUsers
        } else if gap < 3 * USE_WINDOW {
            UseState::Fading
        } else {
            UseState::NotObserved
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Counts {
    pub windows: u64,
    pub short: u64,
    pub matched: u64,
    pub novel: u64,
    pub intermediate: u64,
}

struct Open {
    window: u64,
    last_tick: Option<u64>,
    tokens: Vec<Token>,
}

/// One closed window, for the high-level judge (SPEC §17f). Kept only when `Analyzer::log`
/// is Some.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowRecord {
    pub uid: u64,
    pub window: u64,
    /// Matched or founded family; None for short or intermediate windows.
    pub family: Option<u32>,
    /// Tokens whose effect succeeded (Ok or Partial), and all tokens.
    pub ok: u16,
    pub total: u16,
    /// A successful effect changed the world outside the agent (see `is_output`).
    pub output: bool,
    /// The agent's situation when the window closed (`situation`), or UNKNOWN_SITUATION; filled
    /// in by the caller, which can see the world.
    pub ctx: u8,
    /// Up to 8 distinct action 3-grams of the window (`action_grams`), 0 for unused slots.
    pub grams: [u32; 8],
}

/// Ordered triples of successful actions, ignoring direction, amount and timing: each gram is
/// kind0 << 16 | kind1 << 8 | kind2 (kind = (opcode - 0x20) << 3 | sub-op), plus 1 so 0 means
/// none. The judge uses them to tell a new combination of actions from a founder routine run
/// with other timings or directions (SPEC §17f).
pub fn action_grams(tokens: &[Token]) -> Vec<u32> {
    let mut g: Vec<u32> = tokens
        .windows(3)
        .filter(|w| w.iter().all(|t| t[1] <= 1 && t[0] != 255))
        .map(|w| ((w[0][0] as u32) << 16 | (w[1][0] as u32) << 8 | w[2][0] as u32) + 1)
        .collect();
    g.sort_unstable();
    g.dedup();
    g
}

pub const UNKNOWN_SITUATION: u8 = 0x80;

/// Situation bits for the judge (SPEC §17f): energy band (0 below 20000, 1 below 50000, 2 at
/// least 50000) and whether the agent's tile holds water or ice (bit 2).
pub fn situation(energy: u32, wet: bool) -> u8 {
    let band = if energy < 20000 { 0 } else if energy < 50000 { 1 } else { 2 };
    band | (wet as u8) << 2
}

/// A successful token that changed the world outside the agent: matter moved, external work, a
/// bond made or broken, a push, an emission, or a child released. Reading, walking, turning and
/// private writes are not outputs.
pub fn is_output(t: &Token) -> bool {
    use crate::intent::sub;
    if t[1] > 1 || t[0] == 255 {
        return false;
    }
    let (opc, s) = ((t[0] >> 3) + crate::isa::op::READ, (t[0] & 7) as u16);
    match opc {
        crate::isa::op::TRANSFER => t[4] > 0,
        crate::isa::op::WORK => t[2] != 255,
        crate::isa::op::LINK | crate::isa::op::UNLINK | crate::isa::op::EMIT => true,
        crate::isa::op::DRIVE => s == crate::bonds::DRIVE_PUSH || s == crate::contact::DRIVE_PUSH_ACTOR,
        crate::isa::op::REPRO => s == sub::REPRO_RELEASE,
        _ => false,
    }
}

#[derive(Default)]
pub struct Analyzer {
    open: BTreeMap<u64, Open>,
    pub families: Vec<Family>,
    index: HashMap<u64, Vec<u32>>,
    pub counts: Counts,
    pub log: Option<Vec<WindowRecord>>,
}

pub enum Verdict {
    Short,
    Matched(u32),
    Novel(u32),
    Intermediate(u32),
}

impl Analyzer {
    /// Feed facts in (tick, stage, sequence) order, as the simulation emits them.
    pub fn push(&mut self, f: &EffectFact) {
        let w = f.tick / WINDOW;
        if self.open.get(&f.uid).is_some_and(|o| o.window != w) {
            let o = self.open.remove(&f.uid).unwrap();
            self.close(f.uid, o);
        }
        let o = self.open.entry(f.uid).or_insert(Open { window: w, last_tick: None, tokens: Vec::new() });
        let first = o.tokens.is_empty();
        let delay = if first { 0 } else { delay_bin(f.tick - o.last_tick.unwrap()) };
        o.tokens.push(token(f, delay, first));
        o.last_tick = Some(f.tick);
    }

    /// Close every window that ended before `tick` (call at window boundaries and at the end).
    pub fn flush(&mut self, tick: u64) {
        let w = tick / WINDOW;
        let done: Vec<u64> = self.open.iter().filter(|(_, o)| o.window < w).map(|(u, _)| *u).collect();
        for uid in done {
            let o = self.open.remove(&uid).unwrap();
            self.close(uid, o);
        }
    }

    /// Close everything still open (end of a run).
    pub fn finish(&mut self) {
        let all: Vec<u64> = self.open.keys().copied().collect();
        for uid in all {
            let o = self.open.remove(&uid).unwrap();
            self.close(uid, o);
        }
    }

    fn close(&mut self, uid: u64, o: Open) {
        let tick = o.last_tick.unwrap_or(o.window * WINDOW);
        let v = self.classify(uid, tick, &o.tokens);
        if let Some(log) = &mut self.log {
            let family = match v {
                Verdict::Matched(id) | Verdict::Novel(id) => Some(id),
                Verdict::Short | Verdict::Intermediate(_) => None,
            };
            let ok = o.tokens.iter().filter(|t| t[1] <= 1).count().min(65535) as u16;
            let output = o.tokens.iter().any(is_output);
            let mut grams = [0u32; 8];
            for (k, g) in action_grams(&o.tokens).into_iter().take(8).enumerate() {
                grams[k] = g;
            }
            log.push(WindowRecord {
                uid,
                window: o.window,
                family,
                ok,
                total: o.tokens.len().min(65535) as u16,
                output,
                ctx: UNKNOWN_SITUATION,
                grams,
            });
        }
    }

    pub fn classify(&mut self, uid: u64, tick: u64, tokens: &[Token]) -> Verdict {
        self.counts.windows += 1;
        if tokens.len() < 3 {
            self.counts.short += 1;
            return Verdict::Short;
        }
        let sk = sketch(tokens);
        // best family by Jhat over the candidates; ties go to the oldest family
        let mut best: Option<(u32, usize, usize)> = None;
        let mut all_novel = true;
        for id in self.candidates(&sk) {
            let (i, u) = jhat(&sk, &self.families[id as usize].sketch);
            if 4 * i >= u {
                all_novel = false;
            }
            let better = match best {
                None => true,
                Some((bid, bi, bu)) => i * bu > bi * u || (i * bu == bi * u && id < bid),
            };
            if better {
                best = Some((id, i, u));
            }
        }
        match best {
            Some((id, i, u)) if 4 * i >= 3 * u => {
                self.counts.matched += 1;
                let f = &mut self.families[id as usize];
                f.windows += 1;
                f.last_tick = f.last_tick.max(tick);
                f.agents.insert(uid);
                if f.repeated_at.is_none() && f.windows >= 3 && f.agents.len() >= 2 {
                    f.repeated_at = Some(tick);
                }
                Verdict::Matched(id)
            }
            Some((id, _, _)) if !all_novel => {
                self.counts.intermediate += 1;
                self.families[id as usize].near += 1;
                Verdict::Intermediate(id)
            }
            _ => {
                self.counts.novel += 1;
                let id = self.families.len() as u32;
                for h in sk.iter().take(INDEX_K) {
                    self.index.entry(*h).or_default().push(id);
                }
                self.families.push(Family {
                    id,
                    sketch: sk,
                    first_tick: tick,
                    first_uid: uid,
                    last_tick: tick,
                    windows: 1,
                    near: 0,
                    agents: BTreeSet::from([uid]),
                    repeated_at: None,
                    example: tokens.iter().take(6).copied().collect(),
                });
                Verdict::Novel(id)
            }
        }
    }

    /// Families to compare: all of them while the archive is small; otherwise the families
    /// sharing one of the window's 8 smallest hashes (most shared first, at most 1024), plus the
    /// 256 oldest and the 256 newest as a fixed sample.
    fn candidates(&self, sk: &[u64]) -> Vec<u32> {
        let n = self.families.len();
        if n <= FULL_SCAN {
            return (0..n as u32).collect();
        }
        let mut shared: BTreeMap<u32, u32> = BTreeMap::new();
        for h in sk.iter().take(INDEX_K) {
            for id in self.index.get(h).into_iter().flatten() {
                *shared.entry(*id).or_default() += 1;
            }
        }
        let mut ranked: Vec<(u32, u32)> = shared.into_iter().collect();
        ranked.sort_unstable_by_key(|(id, c)| (std::cmp::Reverse(*c), *id));
        let mut out: BTreeSet<u32> = ranked.into_iter().take(MAX_CANDIDATES).map(|(id, _)| id).collect();
        out.extend(0..256u32);
        out.extend((n as u32 - 256)..n as u32);
        out.into_iter().collect()
    }
}

/// Readable form of a token: "TRANSFER.0 Ok d1 c7 m2 +3".
pub fn describe(t: &Token) -> String {
    const OPS: [&str; 10] = ["READ", "WRITE", "DRIVE", "LINK", "UNLINK", "TRANSFER", "WORK", "EMIT", "REPRO", "COPY"];
    const OUT: [&str; 15] = [
        "Ok", "Partial", "Noop", "Boundary", "Blocked", "Energy", "Matter", "Capacity", "Invalid", "Stale", "Limits",
        "Format", "Occluded", "QueryLimit", "Busy",
    ];
    let op = OPS.get((t[0] >> 3) as usize).copied().unwrap_or("?");
    let out = OUT.get(t[1] as usize).copied().unwrap_or("?");
    let mut s = format!("{op}.{} {out}", t[0] & 7);
    if t[2] != 255 {
        s += &format!(" d{}", t[2]);
    }
    if t[3] != 255 {
        s += &format!(" c{}", t[3]);
    }
    s += &format!(" m{} +{}", t[4], t[5]);
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fact(tick: u64, uid: u64, kind: u8, outcome: u8) -> EffectFact {
        EffectFact { tick, uid, kind, outcome, dir: 255, channel: 255, actual: 1 }
    }

    #[test]
    fn use_state_follows_the_last_sighting() {
        let f = Family {
            id: 0,
            sketch: vec![],
            first_tick: 0,
            first_uid: 0,
            last_tick: 10_000,
            windows: 3,
            near: 0,
            agents: Default::default(),
            repeated_at: None,
            example: vec![],
        };
        assert_eq!(f.use_state(10_000 + 4095), UseState::LiveUsers);
        assert_eq!(f.use_state(10_000 + 4096), UseState::Fading);
        assert_eq!(f.use_state(10_000 + 3 * 4096), UseState::NotObserved);
    }

    #[test]
    fn magnitude_and_delay_bins() {
        assert_eq!((magnitude(0), magnitude(1), magnitude(3), magnitude(u32::MAX)), (0, 1, 2, 16));
        assert_eq!([0, 1, 3, 7, 15, 16, 999].map(delay_bin), [0, 1, 2, 3, 4, 5, 5]);
    }

    #[test]
    fn jhat_counts_the_union_bottom_k() {
        assert_eq!(jhat(&[1, 2, 3], &[1, 2, 3]), (3, 3));
        assert_eq!(jhat(&[1, 2], &[3, 4]), (0, 4));
        assert_eq!(jhat(&[1, 3], &[1, 2, 3]), (2, 3));
        let a: Vec<u64> = (0..128).collect();
        let b: Vec<u64> = (64..192).collect();
        assert_eq!(jhat(&a, &b), (64, 128), "only the 128 smallest of the union count");
    }

    #[test]
    fn repeated_behaviour_needs_three_windows_and_two_agents() {
        let mut an = Analyzer::default();
        // the same 4-effect routine, by agent 1 in windows 0 and 1, then by agent 2 in window 2
        for (w, uid) in [(0u64, 1u64), (1, 1), (2, 2)] {
            for k in 0..4 {
                an.push(&fact(w * WINDOW + k * 8, uid, 5 << 3, 0));
            }
        }
        an.finish();
        assert_eq!(an.families.len(), 1);
        let f = &an.families[0];
        assert_eq!((f.windows, f.agents.len()), (3, 2));
        assert!(f.repeated_at.is_some());
        // a different routine is novel
        for k in 0..4 {
            an.push(&fact(4 * WINDOW + k, 3, 2 << 3, 4));
        }
        an.finish();
        assert_eq!(an.families.len(), 2);
        assert_eq!(an.counts.novel, 2);
    }

    #[test]
    fn one_agent_alone_is_not_repeated() {
        let mut an = Analyzer::default();
        for w in 0..5u64 {
            for k in 0..4 {
                an.push(&fact(w * WINDOW + k * 8, 9, 5 << 3, 0));
            }
        }
        an.finish();
        assert_eq!(an.families[0].windows, 5);
        assert!(an.families[0].repeated_at.is_none());
    }

    #[test]
    fn short_windows_found_nothing() {
        let mut an = Analyzer::default();
        an.push(&fact(0, 1, 5 << 3, 0));
        an.push(&fact(1, 1, 5 << 3, 0));
        an.finish();
        assert!(an.families.is_empty());
        assert_eq!(an.counts.short, 1);
    }
}
