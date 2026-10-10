//! Copy lineage and transfer evidence (original §58 "복사 계보·덮어쓰기·기술 상태"). Observer
//! only.
//!
//! Every record medium carries, per cell interval [start, end), the latest fact that wrote it:
//! a literal WRITE, a COPY (with the author the copied cells came from, resolved when copied),
//! bit wear, recycling to zero, or a destroyed page. A transfer is proven only by the chain
//! copy -> adoption (or birth) -> execution of the code that holds the copied cells by an agent
//! other than their author.

use std::collections::{BTreeMap, BTreeSet};

/// Who first wrote the cells of a span.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Author {
    /// Written cell by cell by this agent.
    Literal(u64),
    /// Copied out of this agent's own code (inherited or learned).
    Code(u64),
}

impl Author {
    pub fn agent(self) -> u64 {
        match self {
            Author::Literal(a) | Author::Code(a) => a,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Origin {
    Literal { agent: u64, tick: u64 },
    /// Copied by `agent` from `src` (media UID); `author` is where the cells first came from.
    Copied { src: u64, agent: u64, tick: u64, author: Option<Author> },
    Damaged { tick: u64 },
    Erased { tick: u64 },
    Destroyed { tick: u64 },
}

impl Origin {
    pub fn author(&self) -> Option<Author> {
        match *self {
            Origin::Literal { agent, .. } => Some(Author::Literal(agent)),
            Origin::Copied { author, .. } => author,
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
    pub origin: Origin,
}

/// Facts the simulation emits when lineage collection is on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MediaFact {
    Write { tick: u64, agent: u64, media: u64, cell: u32 },
    /// `own_code`: the source is the copying agent's own code bank.
    Copy { tick: u64, agent: u64, src: u64, own_code: bool, src_start: u32, dst: u64, dst_start: u32, len: u32 },
    Damage { tick: u64, media: u64, cell: u32 },
    Erase { tick: u64, media: u64, start: u32, end: u32 },
    Destroy { tick: u64, media: u64, start: u32, end: u32 },
    /// A committed draft became the agent's learned code (same media UID).
    Adopt { tick: u64, agent: u64, media: u64, len: u32 },
    /// The child was born with this code medium (its embryo record).
    Born { tick: u64, agent: u64, parent: u64, media: u64 },
    /// Sampled: the agent is running code from this medium.
    Exec { tick: u64, agent: u64, media: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferKind {
    /// The author is the agent's parent (copied into the embryo or learned from a parent's record).
    Kin,
    NonKin,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transfer {
    pub tick: u64,
    pub agent: u64,
    pub media: u64,
    pub author: u64,
    pub kind: TransferKind,
    /// Cells of the running code that came from this author.
    pub cells: u32,
    /// Adopted (learned) or inherited at birth.
    pub learned: bool,
}

#[derive(Default)]
pub struct Lineage {
    spans: BTreeMap<u64, Vec<Span>>,
    parent: BTreeMap<u64, u64>,
    /// Media that run as code, with (agent, learned?) and the used prefix length.
    running: BTreeMap<u64, (u64, bool, u32)>,
    /// (agent, media, author) already reported.
    reported: BTreeSet<(u64, u64, u64)>,
    pub transfers: Vec<Transfer>,
    pub writes: u64,
    pub copies: u64,
    pub copied_cells: u64,
    pub damaged: u64,
}

impl Lineage {
    /// The spans of a medium, sorted and disjoint.
    pub fn spans(&self, media: u64) -> &[Span] {
        self.spans.get(&media).map_or(&[], |v| v.as_slice())
    }

    fn set(&mut self, media: u64, start: u32, end: u32, origin: Origin) {
        if start >= end {
            return;
        }
        let v = self.spans.entry(media).or_default();
        let mut out = Vec::with_capacity(v.len() + 2);
        for s in v.drain(..) {
            if s.end <= start || s.start >= end {
                out.push(s);
                continue;
            }
            if s.start < start {
                out.push(Span { end: start, ..s });
            }
            if s.end > end {
                out.push(Span { start: end, ..s });
            }
        }
        out.push(Span { start, end, origin });
        out.sort_by_key(|s| s.start);
        *v = out;
    }

    /// Source spans over [start, start + len), with gaps (no recorded origin) filled by `fill`.
    fn slice(&self, media: u64, start: u32, len: u32, fill: Option<Author>) -> Vec<(u32, u32, Option<Author>)> {
        let end = start + len;
        let mut out = Vec::new();
        let mut at = start;
        for s in self.spans(media) {
            if s.end <= start || s.start >= end {
                continue;
            }
            let (a, b) = (s.start.max(start), s.end.min(end));
            if a > at {
                out.push((at, a, fill));
            }
            let author = match s.origin {
                Origin::Damaged { .. } | Origin::Erased { .. } | Origin::Destroyed { .. } => None,
                o => o.author().or(fill),
            };
            out.push((a, b, author));
            at = b;
        }
        if at < end {
            out.push((at, end, fill));
        }
        out
    }

    pub fn push(&mut self, f: &MediaFact) {
        match *f {
            MediaFact::Write { tick, agent, media, cell } => {
                self.writes += 1;
                self.set(media, cell, cell + 1, Origin::Literal { agent, tick });
            }
            MediaFact::Copy { tick, agent, src, own_code, src_start, dst, dst_start, len } => {
                self.copies += 1;
                self.copied_cells += len as u64;
                // cells of one's own code without a recorded origin are that agent's
                let fill = own_code.then_some(Author::Code(agent));
                for (a, b, author) in self.slice(src, src_start, len, fill) {
                    let off = a - src_start;
                    self.set(dst, dst_start + off, dst_start + off + (b - a), Origin::Copied { src, agent, tick, author });
                }
            }
            MediaFact::Damage { tick, media, cell } => {
                self.damaged += 1;
                self.set(media, cell, cell + 1, Origin::Damaged { tick });
            }
            MediaFact::Erase { tick, media, start, end } => self.set(media, start, end, Origin::Erased { tick }),
            MediaFact::Destroy { tick, media, start, end } => self.set(media, start, end, Origin::Destroyed { tick }),
            MediaFact::Adopt { agent, media, len, .. } => {
                self.running.insert(media, (agent, true, len));
            }
            MediaFact::Born { agent, parent, media, .. } => {
                self.parent.insert(agent, parent);
                if self.spans.contains_key(&media) {
                    self.running.insert(media, (agent, false, u32::MAX));
                }
            }
            MediaFact::Exec { tick, agent, media } => self.exec(tick, agent, media),
        }
    }

    fn exec(&mut self, tick: u64, agent: u64, media: u64) {
        let Some(&(owner, learned, len)) = self.running.get(&media) else { return };
        if owner != agent {
            return;
        }
        let mut by_author: BTreeMap<u64, u32> = BTreeMap::new();
        for s in self.spans(media) {
            if s.start >= len {
                continue;
            }
            if let Some(a) = s.origin.author() {
                if a.agent() != agent {
                    *by_author.entry(a.agent()).or_default() += s.end.min(len) - s.start;
                }
            }
        }
        for (author, cells) in by_author {
            if !self.reported.insert((agent, media, author)) {
                continue;
            }
            let kind = if self.parent.get(&agent) == Some(&author) { TransferKind::Kin } else { TransferKind::NonKin };
            self.transfers.push(Transfer { tick, agent, media, author, kind, cells, learned });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn later_writes_split_earlier_spans() {
        let mut l = Lineage::default();
        l.push(&MediaFact::Copy { tick: 1, agent: 5, src: 100, own_code: true, src_start: 0, dst: 200, dst_start: 0, len: 10 });
        l.push(&MediaFact::Write { tick: 2, agent: 6, media: 200, cell: 4 });
        let s = l.spans(200);
        assert_eq!(s.iter().map(|s| (s.start, s.end)).collect::<Vec<_>>(), vec![(0, 4), (4, 5), (5, 10)]);
        assert_eq!(s[0].origin.author(), Some(Author::Code(5)));
        assert_eq!(s[1].origin.author(), Some(Author::Literal(6)));
    }

    #[test]
    fn copies_keep_the_first_author() {
        let mut l = Lineage::default();
        // agent 5 publishes its code on record 300; agent 9 copies it into its draft 400
        l.push(&MediaFact::Copy { tick: 1, agent: 5, src: 100, own_code: true, src_start: 0, dst: 300, dst_start: 0, len: 8 });
        l.push(&MediaFact::Copy { tick: 2, agent: 9, src: 300, own_code: false, src_start: 2, dst: 400, dst_start: 0, len: 4 });
        l.push(&MediaFact::Damage { tick: 3, media: 300, cell: 3 });
        let s = l.spans(400);
        assert!(s.iter().all(|s| s.origin.author() == Some(Author::Code(5))), "{s:?}");
        // the damage came after the copy and does not reach the draft
        assert_eq!(s.iter().map(|s| s.end - s.start).sum::<u32>(), 4);
    }

    #[test]
    fn a_transfer_needs_adoption_and_execution() {
        let mut l = Lineage::default();
        l.push(&MediaFact::Copy { tick: 1, agent: 5, src: 100, own_code: true, src_start: 0, dst: 300, dst_start: 0, len: 8 });
        l.push(&MediaFact::Copy { tick: 2, agent: 9, src: 300, own_code: false, src_start: 0, dst: 400, dst_start: 0, len: 8 });
        l.push(&MediaFact::Exec { tick: 3, agent: 9, media: 400 });
        assert!(l.transfers.is_empty(), "a draft is not running code");
        l.push(&MediaFact::Adopt { tick: 4, agent: 9, media: 400, len: 6 });
        l.push(&MediaFact::Exec { tick: 5, agent: 9, media: 400 });
        l.push(&MediaFact::Exec { tick: 6, agent: 9, media: 400 });
        assert_eq!(l.transfers.len(), 1, "reported once");
        let t = &l.transfers[0];
        assert_eq!((t.agent, t.author, t.cells, t.kind, t.learned), (9, 5, 6, TransferKind::NonKin, true));
    }

    #[test]
    fn own_code_round_trip_is_not_a_transfer() {
        let mut l = Lineage::default();
        l.push(&MediaFact::Copy { tick: 1, agent: 9, src: 100, own_code: true, src_start: 0, dst: 400, dst_start: 0, len: 8 });
        l.push(&MediaFact::Adopt { tick: 2, agent: 9, media: 400, len: 8 });
        l.push(&MediaFact::Exec { tick: 3, agent: 9, media: 400 });
        assert!(l.transfers.is_empty());
    }

    #[test]
    fn code_written_into_an_embryo_shows_at_birth() {
        let mut l = Lineage::default();
        // parent 5 copies a record by agent 7 into its embryo 500; child 11 is born with it
        l.push(&MediaFact::Write { tick: 1, agent: 7, media: 300, cell: 0 });
        l.push(&MediaFact::Copy { tick: 2, agent: 5, src: 300, own_code: false, src_start: 0, dst: 500, dst_start: 0, len: 1 });
        l.push(&MediaFact::Born { tick: 3, agent: 11, parent: 5, media: 500 });
        l.push(&MediaFact::Exec { tick: 4, agent: 11, media: 500 });
        assert_eq!(l.transfers.len(), 1);
        assert_eq!((l.transfers[0].author, l.transfers[0].kind, l.transfers[0].learned), (7, TransferKind::NonKin, false));
    }
}
