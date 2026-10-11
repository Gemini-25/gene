//! Behaviour families from resolved effects (original §58, analysis version A1.4).
//!
//! Observer only: the analysis reads `EffectFact`s and never feeds anything back into the world.
//! Each agent's effects in a 256-tick window become 8-byte tokens; consecutive token triples are
//! hashed (BLAKE3 over "behavior-a1.4\0" + 24 bytes), and the 128 smallest distinct hashes form
//! the window's bottom-k sketch. Two sketches are compared with the union bottom-k estimator Jhat.
//! A window joins the best family with Jhat >= 3/4, founds a new family when Jhat < 1/4 against
//! every candidate, and otherwise stays "intermediate" (similar but not the same). A family seen
//! in 3 windows from at least 2 agents is a RepeatedBehavior.
//!
//! Each closed window also gets a context (original §58): the stock of G at the agent's tile,
//! the light there, how crowded it is and how much the agent walked. Once the archive is large,
//! the families compared are the ones sharing hashes with the window plus fixed hash samples of
//! the same context and of the whole archive.

use crate::observer::EffectFact;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};

pub const WINDOW: u64 = 256;
pub const SKETCH: usize = 128;
/// Hashes of a sketch that go into the inverted index.
const INDEX_K: usize = 8;
const MAX_CANDIDATES: usize = 1024;
/// Archive size up to which every family is compared.
const FULL_SCAN: usize = 1536;
/// Size of each fixed hash sample of families (per context, and of the whole archive).
const SAMPLE: usize = 256;
/// Number of contexts (5 G bins x 4 light bins x 3 crowding bins x 4 movement bins).
pub const CONTEXTS: u8 = 240;
/// Context of a window whose agent was gone when the window closed.
pub const UNKNOWN_CONTEXT: u8 = 255;

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

/// What the world showed at the agent's tile when a window closed (original §58): G loose and
/// residue, the light, and the other agents within 2 tiles (Chebyshev).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Place {
    pub g: u64,
    pub light: u64,
    pub crowd: u32,
}

/// Context id 0..240 from the place and the window's successful moves. Bins (original §58):
/// G 0 / 1..3 / 4..15 / 16..63 / 64+, light 0 / 1..8191 / 8192..32767 / 32768+, crowding
/// 0..2 / 3..6 / 7+, moves 0 / 1..7 / 8..31 / 32+.
pub fn context(p: &Place, moves: u32) -> u8 {
    let g = match p.g {
        0 => 0,
        1..=3 => 1,
        4..=15 => 2,
        16..=63 => 3,
        _ => 4,
    };
    let l = match p.light {
        0 => 0,
        1..=8191 => 1,
        8192..=32767 => 2,
        _ => 3,
    };
    let c = match p.crowd {
        0..=2 => 0,
        3..=6 => 1,
        _ => 2,
    };
    let m = match moves {
        0 => 0,
        1..=7 => 1,
        8..=31 => 2,
        _ => 3,
    };
    ((g * 4 + l) * 3 + c) * 4 + m
}

/// Successful translations in a window's tokens.
pub fn moves(tokens: &[Token]) -> u32 {
    let step = (crate::isa::op::DRIVE - crate::isa::op::READ) << 3 | crate::intent::sub::DRIVE_TRANSLATE as u8;
    tokens.iter().filter(|t| t[0] == step && t[1] <= 1).count() as u32
}

/// Fixed sample key of a family: the first 8 bytes of BLAKE3("behavior-sample\0" + id).
fn sample_key(id: u32) -> u64 {
    let mut h = blake3::Hasher::new();
    h.update(b"behavior-sample\0");
    h.update(&id.to_le_bytes());
    u64::from_le_bytes(h.finalize().as_bytes()[..8].try_into().unwrap())
}

/// Keep the `SAMPLE` smallest keys.
fn sample_insert(set: &mut BTreeSet<(u64, u32)>, id: u32) {
    set.insert((sample_key(id), id));
    if set.len() > SAMPLE {
        set.pop_last();
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    /// Contexts of its windows (`context`); unknown ones are left out.
    pub contexts: BTreeSet<u8>,
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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Counts {
    pub windows: u64,
    pub short: u64,
    pub matched: u64,
    pub novel: u64,
    pub intermediate: u64,
}

#[derive(Serialize, Deserialize)]
struct Open {
    window: u64,
    last_tick: Option<u64>,
    tokens: Vec<Token>,
    /// Distinct verbs of the window's successful effects (`verb`).
    verbs: Vec<u32>,
}

/// One closed window, for the high-level judge (SPEC §17f). Kept only when `Analyzer::log`
/// is Some.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    /// Up to 8 distinct verbs of the window's successful effects (`verb`), sorted, 0 unused.
    pub verbs: [u32; 8],
}

/// What a successful effect did and to what (SPEC §17f): the action kind (sub-op 0 where it is
/// unused), the channel (species, energy 15, signal channel, bond target) and the address
/// domains of its source and destination. TRANSFER of G from the agent's inventory into its
/// embryo is a different verb from TRANSFER of energy into the embryo or of G from the ground,
/// though all three are the same action kind. None for failures.
pub fn verb(f: &EffectFact) -> Option<u32> {
    if f.outcome > 1 || f.kind == 255 {
        return None;
    }
    Some((norm_kind(f.kind) as u32) << 24 | (f.channel as u32) << 16 | (f.src as u32) << 8 | f.dst as u32)
}

/// Readable form of a verb: "TRANSFER G self>embryo".
pub fn describe_verb(v: u32) -> String {
    const OPS: [&str; 10] = ["READ", "WRITE", "DRIVE", "LINK", "UNLINK", "TRANSFER", "WORK", "EMIT", "REPRO", "COPY"];
    const SPECIES: [&str; 10] = ["W", "C", "O", "G", "H", "X", "M", "S", "V", "I"];
    const DOMS: [&str; 12] = ["self", "loose", "parcel", "actor", "signal", "tape", "code", "draft", "embryo", "trace", "pore", "surface"];
    let (k, ch, src, dst) = ((v >> 24) as u8, (v >> 16) as u8, (v >> 8) as u8, v as u8);
    let op = OPS.get((k >> 3) as usize).copied().unwrap_or("?");
    let mut s = if matches!(op, "LINK" | "UNLINK" | "TRANSFER" | "EMIT" | "COPY") { op.to_string() } else { format!("{op}.{}", k & 7) };
    if ch != 255 {
        let c = ch & !EffectFact::CONTACT;
        s += &match (op, c) {
            ("TRANSFER", 15) => " energy".to_string(),
            ("TRANSFER", c) if (c as usize) < SPECIES.len() => format!(" {}", SPECIES[c as usize]),
            (_, c) => format!(" c{c}"),
        };
        if ch & EffectFact::CONTACT != 0 {
            s += " contact";
        }
    }
    let dom = |d: u8| DOMS.get(d as usize).copied().unwrap_or("?");
    match (src, dst) {
        (255, 255) => {}
        (a, 255) => s += &format!(" {}", dom(a)),
        (a, b) => s += &format!(" {}>{}", dom(a), dom(b)),
    }
    s
}

/// Ordered triples of successful actions, ignoring direction, amount and timing: each gram is
/// kind0 << 16 | kind1 << 8 | kind2 (kind = (opcode - 0x20) << 3 | sub-op), plus 1 so 0 means
/// none. The judge uses them to tell a new combination of actions from a founder routine run
/// with other timings or directions (SPEC §17f). LINK, UNLINK, TRANSFER, EMIT and COPY ignore
/// their sub-op, so it is set to 0 there: a mutated, unused immediate is not a new action.
pub fn action_grams(tokens: &[Token]) -> Vec<u32> {
    action_grams_impl(tokens)
}

/// The kinds of successful actions in a token sequence (sub-op 0 where it is unused), sorted:
/// what a window does, regardless of order, timing, failures and repeats.
pub fn action_set(tokens: &[Token]) -> Vec<u8> {
    let mut v: Vec<u8> = tokens.iter().filter(|t| t[1] <= 1 && t[0] != 255).map(|t| norm_kind(t[0])).collect();
    v.sort_unstable();
    v.dedup();
    v
}

fn norm_kind(k: u8) -> u8 {
    use crate::isa::op;
    let opc = (k >> 3) + op::READ;
    if matches!(opc, op::LINK | op::UNLINK | op::TRANSFER | op::EMIT | op::COPY) { k & !7 } else { k }
}

fn action_grams_impl(tokens: &[Token]) -> Vec<u32> {
    let kind = |k: u8| -> u32 {
        use crate::isa::op;
        let opc = (k >> 3) + op::READ;
        if matches!(opc, op::LINK | op::UNLINK | op::TRANSFER | op::EMIT | op::COPY) { (k & !7) as u32 } else { k as u32 }
    };
    let mut g: Vec<u32> = tokens
        .windows(3)
        .filter(|w| w.iter().all(|t| t[1] <= 1 && t[0] != 255))
        .map(|w| (kind(w[0][0]) << 16 | kind(w[1][0]) << 8 | kind(w[2][0])) + 1)
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

#[derive(Default, Serialize, Deserialize)]
pub struct Analyzer {
    open: BTreeMap<u64, Open>,
    pub families: Vec<Family>,
    index: HashMap<u64, Vec<u32>>,
    /// Fixed hash samples: of the whole archive, and of the families seen in each context.
    sample: BTreeSet<(u64, u32)>,
    ctx_sample: BTreeMap<u8, BTreeSet<(u64, u32)>>,
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
            self.close(f.uid, o, None);
        }
        let o = self.open.entry(f.uid).or_insert(Open { window: w, last_tick: None, tokens: Vec::new(), verbs: Vec::new() });
        let first = o.tokens.is_empty();
        let delay = if first { 0 } else { delay_bin(f.tick - o.last_tick.unwrap()) };
        o.tokens.push(token(f, delay, first));
        o.last_tick = Some(f.tick);
        if let Some(v) = verb(f) {
            if !o.verbs.contains(&v) {
                o.verbs.push(v);
            }
        }
    }

    /// Close every window that ended before `tick` (call at window boundaries and at the end).
    pub fn flush(&mut self, tick: u64) {
        self.flush_at(tick, |_| None);
    }

    /// As `flush`, with each agent's place now (None for agents that are gone).
    pub fn flush_at(&mut self, tick: u64, place: impl Fn(u64) -> Option<Place>) {
        let w = tick / WINDOW;
        let done: Vec<u64> = self.open.iter().filter(|(_, o)| o.window < w).map(|(u, _)| *u).collect();
        for uid in done {
            let o = self.open.remove(&uid).unwrap();
            self.close(uid, o, place(uid));
        }
    }

    /// Close everything still open (end of a run).
    pub fn finish(&mut self) {
        self.finish_at(|_| None);
    }

    pub fn finish_at(&mut self, place: impl Fn(u64) -> Option<Place>) {
        let all: Vec<u64> = self.open.keys().copied().collect();
        for uid in all {
            let o = self.open.remove(&uid).unwrap();
            self.close(uid, o, place(uid));
        }
    }

    fn close(&mut self, uid: u64, o: Open, place: Option<Place>) {
        let tick = o.last_tick.unwrap_or(o.window * WINDOW);
        let ctx = place.map_or(UNKNOWN_CONTEXT, |p| context(&p, moves(&o.tokens)));
        let v = self.classify_in(uid, tick, &o.tokens, ctx);
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
            let mut vs = o.verbs.clone();
            vs.sort_unstable();
            let mut verbs = [0u32; 8];
            for (k, v) in vs.into_iter().take(8).enumerate() {
                verbs[k] = v;
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
                verbs,
            });
        }
    }

    pub fn classify(&mut self, uid: u64, tick: u64, tokens: &[Token]) -> Verdict {
        self.classify_in(uid, tick, tokens, UNKNOWN_CONTEXT)
    }

    /// Classify a window seen in context `ctx` (`UNKNOWN_CONTEXT` if not known).
    pub fn classify_in(&mut self, uid: u64, tick: u64, tokens: &[Token], ctx: u8) -> Verdict {
        self.counts.windows += 1;
        if tokens.len() < 3 {
            self.counts.short += 1;
            return Verdict::Short;
        }
        let sk = sketch(tokens);
        // best family by Jhat over the candidates; ties go to the oldest family
        let mut best: Option<(u32, usize, usize)> = None;
        let mut all_novel = true;
        for id in self.candidates(&sk, ctx) {
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
                if ctx != UNKNOWN_CONTEXT && f.contexts.insert(ctx) {
                    sample_insert(self.ctx_sample.entry(ctx).or_default(), id);
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
                    contexts: BTreeSet::new(),
                });
                sample_insert(&mut self.sample, id);
                if ctx != UNKNOWN_CONTEXT {
                    self.families[id as usize].contexts.insert(ctx);
                    sample_insert(self.ctx_sample.entry(ctx).or_default(), id);
                }
                Verdict::Novel(id)
            }
        }
    }

    /// Families to compare: all of them while the archive is small; otherwise the families
    /// sharing one of the window's 8 smallest hashes (most shared first, at most 1024), plus
    /// the fixed hash samples of the window's context and of the whole archive (256 each).
    fn candidates(&self, sk: &[u64], ctx: u8) -> Vec<u32> {
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
        out.extend(self.sample.iter().map(|(_, id)| *id));
        if let Some(c) = self.ctx_sample.get(&ctx) {
            out.extend(c.iter().map(|(_, id)| *id));
        }
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
        EffectFact { tick, uid, kind, outcome, dir: 255, channel: 255, actual: 1, src: 255, dst: 255 }
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
            contexts: Default::default(),
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
    fn context_bins_follow_the_original_table() {
        let p = |g, light, crowd| Place { g, light, crowd };
        assert_eq!(context(&p(0, 0, 0), 0), 0);
        assert_eq!(context(&p(64, 32768, 7), 32), CONTEXTS - 1);
        assert_eq!(context(&p(3, 8191, 2), 7), ((1 * 4 + 1) * 3) * 4 + 1);
        assert_eq!(context(&p(4, 8192, 3), 8), ((2 * 4 + 2) * 3 + 1) * 4 + 2);
        let step = [(crate::isa::op::DRIVE - crate::isa::op::READ) << 3, 0, 0, 255, 1, 0, 0, 0];
        let turn = [step[0] | 1, 0, 0, 255, 1, 0, 0, 0];
        let blocked = [step[0], 4, 0, 255, 0, 0, 0, 0];
        assert_eq!(moves(&[step, turn, blocked, step]), 2);
    }

    #[test]
    fn a_large_archive_still_compares_the_same_context_sample() {
        // 2000 distinct one-off families, then a window that repeats family 1000 but shares
        // none of its 8 smallest hashes with the index: only a sample can find it.
        let mut an = Analyzer::default();
        let ctx = 17;
        for i in 0..2000u32 {
            let tokens: Vec<Token> = (0..4u8).map(|k| [5 << 3, 0, k, (i % 251) as u8, (i / 251) as u8, k, 0, 0]).collect();
            an.classify_in(i as u64, i as u64 * WINDOW, &tokens, if i == 1000 { ctx } else { 3 });
        }
        assert_eq!(an.families.len(), 2000);
        let in_ctx = an.ctx_sample.get(&ctx).map_or(0, |c| c.len());
        assert_eq!(in_ctx, 1);
        let cands = an.candidates(&[u64::MAX], ctx);
        assert!(cands.contains(&1000), "the context sample is compared");
        assert!(cands.len() <= 1 + SAMPLE);
        assert_eq!(an.families[1000].contexts, BTreeSet::from([ctx]));
    }

    #[test]
    fn windows_record_their_verbs() {
        let mut an = Analyzer { log: Some(Vec::new()), ..Analyzer::default() };
        let mut f = fact(10, 4, 5 << 3, 0); // TRANSFER, Ok
        (f.channel, f.src, f.dst) = (3, 1, 0); // G from the ground into the inventory
        an.push(&f);
        an.push(&f);
        let mut failed = f;
        (failed.outcome, failed.dst) = (6, 8); // failed: no verb
        an.push(&failed);
        an.finish();
        let r = an.log.unwrap()[0];
        assert_eq!(r.verbs.iter().filter(|&&v| v != 0).count(), 1);
        assert_eq!(describe_verb(r.verbs[0]), "TRANSFER G loose>self");
    }

    #[test]
    fn an_unused_immediate_does_not_hide_the_action() {
        use crate::intent::Intent;
        use crate::isa::op;
        let it = |opc, sub| Intent { opc, sub, r: [0; 8], copy: None, target_uid: None, parcel_uid: None };
        let f = |i: &Intent| EffectFact::new(0, 1, i, 0, 1).kind;
        assert_eq!(f(&it(op::TRANSFER, 16)), f(&it(op::TRANSFER, 0)), "TRANSFER ignores its sub-op");
        assert_eq!(f(&it(op::COPY, 9)), f(&it(op::COPY, 0)));
        assert_eq!(f(&it(op::REPRO, 9)), 255, "REPRO uses its sub-op: 9 is no action it knows");
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
