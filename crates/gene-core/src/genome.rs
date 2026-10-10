//! Genome binary format. SPEC §7.

use serde::{Deserialize, Serialize};

pub const MAGIC: u16 = 0xC17A;
pub const FORMAT: u16 = 2;
pub const HEADER_CELLS: usize = 16;
pub const DESC_CELLS: usize = 8;
pub const MAX_WORDS: usize = 4096;
pub const MAX_MODULES: usize = 64;
pub const MAX_RAW_CELLS: usize = 9216;
pub const PAGE_CELLS: usize = 128;
pub const MAX_PAGES: usize = MAX_RAW_CELLS / PAGE_CELLS;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Module {
    pub tag: u16,
    pub start: u16,
    pub len: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Program {
    pub entry_tag: u16,
    pub point_threshold: u16,
    pub structural_threshold: u16,
    pub weights: [u16; 8],
    pub modules: Vec<Module>,
    pub words: Vec<u32>,
}

impl Program {
    pub fn raw_len(&self) -> usize {
        HEADER_CELLS + DESC_CELLS * self.modules.len() + 2 * self.words.len()
    }

    pub fn module_index(&self, tag: u16) -> Option<usize> {
        self.modules.iter().position(|m| m.tag == tag)
    }

    pub fn entry_index(&self) -> usize {
        self.module_index(self.entry_tag).expect("validated program has entry")
    }

    #[inline]
    pub fn word(&self, module: usize, pc: u16) -> u32 {
        let m = self.modules[module];
        self.words[m.start as usize + pc as usize]
    }

    /// Serialize to raw cells. Recomputes starts from module order.
    pub fn to_cells(&self) -> Vec<u16> {
        let mut out = Vec::with_capacity(self.raw_len());
        out.push(MAGIC);
        out.push(FORMAT);
        out.push(self.entry_tag);
        out.push(self.modules.len() as u16);
        out.push(self.words.len() as u16);
        out.push(self.point_threshold);
        out.push(self.structural_threshold);
        out.extend_from_slice(&self.weights);
        out.push(0);
        let mut start = 0u16;
        for m in &self.modules {
            out.extend_from_slice(&[m.tag, start, m.len, 0, 0, 0, 0, 0]);
            start += m.len;
        }
        for w in &self.words {
            out.push(*w as u16);
            out.push((*w >> 16) as u16);
        }
        out
    }

    /// Fix module starts so that they follow descriptor order contiguously.
    pub fn renumber(&mut self) {
        let mut s = 0u16;
        for m in &mut self.modules {
            m.start = s;
            s += m.len;
        }
    }

    pub fn smallest_free_tag(&self) -> Option<u16> {
        (0..=u16::MAX).find(|t| !self.modules.iter().any(|m| m.tag == *t))
    }

    pub fn within_limits(&self) -> bool {
        !self.modules.is_empty()
            && self.modules.len() <= MAX_MODULES
            && !self.words.is_empty()
            && self.words.len() <= MAX_WORDS
            && self.raw_len() <= MAX_RAW_CELLS
            && self.modules.iter().all(|m| m.len >= 1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormatError {
    TooShort,
    Magic,
    Count,
    Reserved,
    Descriptor,
    Entry,
    Length,
}

/// Validate and parse a used prefix. The prefix length must equal the encoded length.
pub fn parse(cells: &[u16]) -> Result<Program, FormatError> {
    if cells.len() < HEADER_CELLS {
        return Err(FormatError::TooShort);
    }
    if cells[0] != MAGIC || cells[1] != FORMAT {
        return Err(FormatError::Magic);
    }
    let entry_tag = cells[2];
    let mc = cells[3] as usize;
    let wc = cells[4] as usize;
    if mc == 0 || mc > MAX_MODULES || wc == 0 || wc > MAX_WORDS {
        return Err(FormatError::Count);
    }
    if cells[15] != 0 {
        return Err(FormatError::Reserved);
    }
    let total = HEADER_CELLS + DESC_CELLS * mc + 2 * wc;
    if total > MAX_RAW_CELLS || cells.len() != total {
        return Err(FormatError::Length);
    }
    let mut weights = [0u16; 8];
    weights.copy_from_slice(&cells[7..15]);
    let mut modules = Vec::with_capacity(mc);
    let mut expect = 0usize;
    for i in 0..mc {
        let d = &cells[HEADER_CELLS + DESC_CELLS * i..HEADER_CELLS + DESC_CELLS * (i + 1)];
        let (tag, start, len) = (d[0], d[1] as usize, d[2] as usize);
        if d[3..].iter().any(|&c| c != 0) {
            return Err(FormatError::Reserved);
        }
        if len == 0 || start != expect || modules.iter().any(|m: &Module| m.tag == tag) {
            return Err(FormatError::Descriptor);
        }
        expect += len;
        modules.push(Module { tag, start: start as u16, len: len as u16 });
    }
    if expect != wc {
        return Err(FormatError::Descriptor);
    }
    if !modules.iter().any(|m| m.tag == entry_tag) {
        return Err(FormatError::Entry);
    }
    let base = HEADER_CELLS + DESC_CELLS * mc;
    let words = (0..wc)
        .map(|i| cells[base + 2 * i] as u32 | (cells[base + 2 * i + 1] as u32) << 16)
        .collect();
    Ok(Program {
        entry_tag,
        point_threshold: cells[5],
        structural_threshold: cells[6],
        weights,
        modules,
        words,
    })
}

pub fn pages_for(cells: usize) -> usize {
    cells.div_ceil(PAGE_CELLS)
}

pub fn hash_cells(cells: &[u16]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    for c in cells {
        h.update(&c.to_le_bytes());
    }
    *h.finalize().as_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Program {
        Program {
            entry_tag: 3,
            point_threshold: 64,
            structural_threshold: 2048,
            weights: [4, 2, 2, 2, 1, 1, 1, 1],
            modules: vec![Module { tag: 3, start: 0, len: 2 }, Module { tag: 0, start: 2, len: 1 }],
            words: vec![0x0001_0001, 0x0000_0017, 0xdead_beef],
        }
    }

    #[test]
    fn roundtrip() {
        let p = sample();
        let cells = p.to_cells();
        assert_eq!(cells.len(), p.raw_len());
        assert_eq!(parse(&cells).unwrap(), p);
    }

    #[test]
    fn rejects_bad_descriptor_and_length() {
        let mut cells = sample().to_cells();
        cells[HEADER_CELLS + 1] = 1; // first module must start at 0
        assert_eq!(parse(&cells), Err(FormatError::Descriptor));
        let cells = sample().to_cells();
        assert_eq!(parse(&cells[..cells.len() - 1]), Err(FormatError::Length));
        let mut cells = sample().to_cells();
        cells[2] = 9;
        assert_eq!(parse(&cells), Err(FormatError::Entry));
    }
}
