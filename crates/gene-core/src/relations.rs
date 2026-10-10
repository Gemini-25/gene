//! Relation graph and groups (original §58 "관계 graph와 집단", §29). Observer only: nothing here
//! feeds back into the world.
//!
//! Facts are real interactions: resource moved between agents (mass, per species or energy),
//! record cells copied between agents and public media, and successful contacts with parcels
//! (artifacts). Every 4096 ticks the raw sums of each directed (from, to, kind) become integer
//! weights min(255, floor(log2(1 + sum))); the last four windows feed a deterministic weighted
//! label propagation, and the clusters keep their IDs across windows by agent Jaccard.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// Observer node key (type, uid): Agent 0, Parcel 1, Media 2.
pub type Node = (u8, u64);
pub const AGENT: u8 = 0;
pub const PARCEL: u8 = 1;
pub const MEDIA: u8 = 2;

/// Relation kinds.
pub const RESOURCE: u8 = 0;
pub const RECORD: u8 = 1;
pub const USE: u8 = 2;

pub const WINDOW: u64 = 4096;

/// Directed edge key (from, to, kind) and one window of integer weights.
type EdgeKey = (Node, Node, u8);
type Weights = BTreeMap<EdgeKey, u8>;
/// Windows of weights kept per edge.
pub const KEEP: usize = 4;
const MAX_PASSES: usize = 20;
/// UI thresholds of a civilization-like candidate (original §29).
pub const CIV_AGENTS: usize = 20;
pub const CIV_WINDOWS: u32 = 3;

/// One resolved interaction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RelFact {
    pub tick: u64,
    pub from: Node,
    pub to: Node,
    pub kind: u8,
    /// Species 0..9, energy 15, or 255.
    pub channel: u8,
    /// Mass (resource), cells (record) or 1 per contact (use).
    pub amount: u64,
}

/// A cluster of the last closed window.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Group {
    pub id: u64,
    /// Living agents (UID order) and the other nodes (parcels, media) of the cluster.
    pub agents: Vec<u64>,
    pub others: Vec<Node>,
    /// Bit k set when an internal edge of kind k exists.
    pub kinds: u8,
    /// Sum of internal edge weights over the kept windows.
    pub weight: u64,
    /// Consecutive windows this ID has existed.
    pub windows: u32,
    /// Living agents whose own-label share of neighbour weight is below 0.6.
    pub border: usize,
    /// Every internal edge touches the same non-agent node.
    pub hub: bool,
    pub civ: bool,
    /// The earlier group this one split from.
    pub parent: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GroupEvent {
    New { window: u64, id: u64 },
    Split { window: u64, id: u64, from: u64 },
    Merge { window: u64, into: u64, from: u64 },
    Ended { window: u64, id: u64 },
    Civ { window: u64, id: u64, agents: usize },
}

#[derive(Default)]
pub struct Graph {
    /// Window being filled.
    window: u64,
    cur: BTreeMap<EdgeKey, u64>,
    /// Closed windows: (window id, weight per directed edge and kind).
    hist: VecDeque<(u64, Weights)>,
    pub groups: Vec<Group>,
    pub events: Vec<GroupEvent>,
    next_id: u64,
    /// Raw totals since the start, per kind (for summaries).
    pub totals: [u64; 3],
}

impl Graph {
    pub fn push(&mut self, f: &RelFact) {
        if f.from == f.to || f.amount == 0 {
            return;
        }
        *self.cur.entry((f.from, f.to, f.kind)).or_default() += f.amount;
        self.totals[f.kind as usize] += f.amount;
    }

    /// Close every window that ends at or before `tick`. `alive` tells living agents apart.
    /// Returns true when the groups were recomputed.
    pub fn advance(&mut self, tick: u64, alive: &dyn Fn(u64) -> bool) -> bool {
        let mut closed = false;
        while (self.window + 1) * WINDOW <= tick {
            let raw = std::mem::take(&mut self.cur);
            let weights: Weights = raw.into_iter().map(|(k, v)| (k, weight(v))).collect();
            self.hist.push_back((self.window, weights));
            while self.hist.len() > KEEP {
                self.hist.pop_front();
            }
            self.regroup(self.window, alive);
            self.window += 1;
            closed = true;
        }
        closed
    }

    /// Undirected adjacency over the kept windows: weight sum and kinds per pair.
    fn adjacency(&self) -> BTreeMap<Node, BTreeMap<Node, (u64, u8)>> {
        let mut adj: BTreeMap<Node, BTreeMap<Node, (u64, u8)>> = BTreeMap::new();
        for (_, w) in &self.hist {
            for (&(a, b, k), &v) in w {
                if v == 0 {
                    continue;
                }
                for (x, y) in [(a, b), (b, a)] {
                    let e = adj.entry(x).or_default().entry(y).or_default();
                    e.0 += v as u64;
                    e.1 |= 1 << k;
                }
            }
        }
        adj
    }

    fn regroup(&mut self, window: u64, alive: &dyn Fn(u64) -> bool) {
        let adj = self.adjacency();
        let labels = propagate(window, &adj);
        // clusters by label, in label order
        let mut members: BTreeMap<Node, Vec<Node>> = BTreeMap::new();
        for (&n, &l) in &labels {
            members.entry(l).or_default().push(n);
        }
        let mut found: Vec<Group> = Vec::new();
        for nodes in members.values() {
            let set: BTreeSet<Node> = nodes.iter().copied().collect();
            let agents: Vec<u64> = nodes.iter().filter(|n| n.0 == AGENT && alive(n.1)).map(|n| n.1).collect();
            if agents.len() < 2 {
                continue;
            }
            let others: Vec<Node> = nodes.iter().filter(|n| n.0 != AGENT).copied().collect();
            let (mut kinds, mut weight, mut edges) = (0u8, 0u64, 0usize);
            let mut touch: BTreeMap<Node, usize> = BTreeMap::new();
            for &u in nodes {
                for (&v, &(w, k)) in &adj[&u] {
                    if u < v && set.contains(&v) {
                        kinds |= k;
                        weight += w;
                        edges += 1;
                        for n in [u, v] {
                            if n.0 != AGENT {
                                *touch.entry(n).or_default() += 1;
                            }
                        }
                    }
                }
            }
            let hub = edges >= 2 && touch.values().any(|&c| c == edges);
            let label = labels[&nodes[0]];
            let border = agents
                .iter()
                .filter(|&&a| {
                    let (own, all) = adj[&(AGENT, a)].iter().fold((0u64, 0u64), |(o, t), (v, &(w, _))| {
                        (o + if labels[v] == label { w } else { 0 }, t + w)
                    });
                    own * 10 < all * 6
                })
                .count();
            found.push(Group { agents, others, kinds, weight, hub, border, ..Group::default() });
        }
        self.match_ids(window, found);
    }

    /// Keep IDs across windows: greedy one-to-one pairs with agent Jaccard >= 1/2, best score
    /// first, then the earlier ID.
    fn match_ids(&mut self, window: u64, mut found: Vec<Group>) {
        let prev = std::mem::take(&mut self.groups);
        let mut pairs: Vec<(u64, u64, usize, usize)> = Vec::new(); // (inter, union, prev k, new k)
        for (pk, p) in prev.iter().enumerate() {
            for (nk, n) in found.iter().enumerate() {
                let inter = overlap(&p.agents, &n.agents) as u64;
                let union = (p.agents.len() + n.agents.len()) as u64 - inter;
                if inter > 0 && 2 * inter >= union {
                    pairs.push((inter, union, pk, nk));
                }
            }
        }
        pairs.sort_by(|a, b| (b.0 * a.1).cmp(&(a.0 * b.1)).then(prev[a.2].id.cmp(&prev[b.2].id)).then(a.3.cmp(&b.3)));
        let mut used_prev = vec![false; prev.len()];
        let mut used_new = vec![false; found.len()];
        for &(_, _, pk, nk) in &pairs {
            if used_prev[pk] || used_new[nk] {
                continue;
            }
            used_prev[pk] = true;
            used_new[nk] = true;
            found[nk].id = prev[pk].id;
            found[nk].windows = prev[pk].windows + 1;
            found[nk].parent = prev[pk].parent;
        }
        // the earlier group each new cluster overlaps most (ties: smaller ID)
        let best_prev = |n: &Group| {
            prev.iter()
                .map(|p| (overlap(&p.agents, &n.agents), std::cmp::Reverse(p.id), p.id))
                .filter(|x| x.0 > 0)
                .max()
                .map(|x| x.2)
        };
        for nk in 0..found.len() {
            if used_new[nk] {
                continue;
            }
            self.next_id += 1;
            let id = self.next_id;
            found[nk].id = id;
            found[nk].windows = 1;
            match best_prev(&found[nk]) {
                Some(from) => {
                    found[nk].parent = Some(from);
                    self.events.push(GroupEvent::Split { window, id, from });
                }
                None => self.events.push(GroupEvent::New { window, id }),
            }
        }
        for (pk, p) in prev.iter().enumerate() {
            if used_prev[pk] {
                continue;
            }
            // most of its agents went into a group that kept another ID: a merge
            let into = found
                .iter()
                .map(|n| (overlap(&p.agents, &n.agents), std::cmp::Reverse(n.id), n.id))
                .filter(|x| 2 * x.0 > p.agents.len())
                .max()
                .map(|x| x.2);
            match into {
                Some(into) => self.events.push(GroupEvent::Merge { window, into, from: p.id }),
                None => self.events.push(GroupEvent::Ended { window, id: p.id }),
            }
        }
        for g in &mut found {
            g.civ = g.agents.len() >= CIV_AGENTS && g.windows >= CIV_WINDOWS && g.kinds.count_ones() >= 2;
            let was = prev.iter().any(|p| p.id == g.id && p.civ);
            if g.civ && !was {
                self.events.push(GroupEvent::Civ { window, id: g.id, agents: g.agents.len() });
            }
        }
        found.sort_by_key(|g| g.id);
        self.groups = found;
    }

    /// Group ID of each living agent in the current groups.
    pub fn membership(&self) -> BTreeMap<u64, u64> {
        let mut m = BTreeMap::new();
        for g in &self.groups {
            for &a in &g.agents {
                m.insert(a, g.id);
            }
        }
        m
    }
}

/// min(255, floor(log2(1 + sum))).
pub fn weight(sum: u64) -> u8 {
    (63 - (sum.saturating_add(1)).leading_zeros()).min(255) as u8
}

fn overlap(a: &[u64], b: &[u64]) -> usize {
    let (mut i, mut j, mut n) = (0, 0, 0);
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => {
                n += 1;
                i += 1;
                j += 1;
            }
        }
    }
    n
}

/// Deterministic weighted label propagation. Initial label = own node key; pass order by
/// BLAKE3("group-a1.4", window, node) then node key; the heaviest neighbour label wins, ties keep
/// the current label, then the smallest. At most 20 passes.
pub fn propagate(window: u64, adj: &BTreeMap<Node, BTreeMap<Node, (u64, u8)>>) -> BTreeMap<Node, Node> {
    let mut labels: BTreeMap<Node, Node> = adj.keys().map(|&n| (n, n)).collect();
    let mut order: Vec<(u64, Node)> = adj
        .keys()
        .map(|&n| {
            let mut h = blake3::Hasher::new();
            h.update(b"group-a1.4\0");
            h.update(&window.to_le_bytes());
            h.update(&[n.0]);
            h.update(&n.1.to_le_bytes());
            (u64::from_le_bytes(h.finalize().as_bytes()[..8].try_into().unwrap()), n)
        })
        .collect();
    order.sort_unstable();
    for _ in 0..MAX_PASSES {
        let mut changed = 0;
        for &(_, u) in &order {
            let mut tally: BTreeMap<Node, u64> = BTreeMap::new();
            for (v, &(w, _)) in &adj[&u] {
                *tally.entry(labels[v]).or_default() += w;
            }
            let Some(&best) = tally.values().max() else { continue };
            let cur = labels[&u];
            if tally.get(&cur) == Some(&best) {
                continue;
            }
            let pick = *tally.iter().find(|(_, &w)| w == best).unwrap().0;
            labels.insert(u, pick);
            changed += 1;
        }
        if changed == 0 {
            break;
        }
    }
    labels
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fact(tick: u64, a: Node, b: Node, kind: u8, amount: u64) -> RelFact {
        RelFact { tick, from: a, to: b, kind, channel: 255, amount }
    }

    #[test]
    fn weights_are_log2_of_the_window_sum() {
        assert_eq!([0, 1, 2, 3, 7, 8, u64::MAX].map(weight), [0, 1, 1, 2, 3, 3, 63]);
        // splitting a transfer into small ones does not raise the weight
        let mut g = Graph::default();
        for _ in 0..8 {
            g.push(&fact(1, (AGENT, 1), (AGENT, 2), RESOURCE, 1));
        }
        g.advance(WINDOW, &|_| true);
        assert_eq!(g.hist[0].1[&((AGENT, 1), (AGENT, 2), RESOURCE)], 3);
    }

    #[test]
    fn two_communities_and_a_stable_id() {
        let mut g = Graph::default();
        let alive = |_| true;
        for w in 0..4u64 {
            let t = w * WINDOW + 1;
            // agents 1..4 share parcel 100, agents 11..14 all trade with each other
            for a in 1..=4 {
                g.push(&fact(t, (AGENT, a), (PARCEL, 100), USE, 5));
            }
            for a in 11..=14 {
                for b in 11..=14 {
                    g.push(&fact(t, (AGENT, a), (AGENT, b), RESOURCE, 50));
                }
            }
            g.advance((w + 1) * WINDOW, &alive);
        }
        assert_eq!(g.groups.len(), 2, "{:?}", g.groups);
        let a = g.groups.iter().find(|x| x.agents.contains(&1)).unwrap();
        assert_eq!(a.agents, vec![1, 2, 3, 4]);
        assert_eq!(a.others, vec![(PARCEL, 100)]);
        assert!(a.hub, "everything goes through one parcel");
        assert_eq!(a.kinds, 1 << USE);
        assert_eq!(a.windows, 4, "the ID survives every window");
        let b = g.groups.iter().find(|x| x.agents.contains(&11)).unwrap();
        assert_eq!(b.agents, vec![11, 12, 13, 14]);
        assert!(!b.hub && !b.civ);
    }

    #[test]
    fn a_large_lasting_group_with_two_kinds_is_a_candidate() {
        let mut g = Graph::default();
        for w in 0..3u64 {
            let t = w * WINDOW + 1;
            for a in 1..=24u64 {
                g.push(&fact(t, (AGENT, a), (PARCEL, 500 + a % 3), USE, 3));
                g.push(&fact(t, (MEDIA, 900), (AGENT, a), RECORD, 40));
            }
            g.advance((w + 1) * WINDOW, &|_| true);
        }
        assert_eq!(g.groups.len(), 1);
        assert!(g.groups[0].civ);
        assert!(g.events.iter().any(|e| matches!(e, GroupEvent::Civ { agents: 24, .. })));
    }

    #[test]
    fn dead_agents_do_not_count() {
        let mut g = Graph::default();
        g.push(&fact(1, (AGENT, 1), (AGENT, 2), RESOURCE, 9));
        g.push(&fact(1, (AGENT, 2), (AGENT, 3), RESOURCE, 9));
        g.advance(WINDOW, &|uid| uid != 2);
        assert_eq!(g.groups.len(), 1);
        assert_eq!(g.groups[0].agents, vec![1, 3]);
    }
}
