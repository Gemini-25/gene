//! Physical record media made of paid 128-cell pages. SPEC §6, §11, original §55.

use crate::genome::{pages_for, PAGE_CELLS};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Media {
    pub uid: u64,
    /// slot_count * 128 cells. Cells of holes are kept zero and never read.
    pub cells: Vec<u16>,
    pub present: Vec<bool>,
    pub logical_len: u32,
    pub revision: u64,
    /// Rx2 reaction credit while exposed (dropped media only).
    pub credit: u16,
    /// Paid shrink cursor: (page index, next cell).
    pub recycle: Option<(u16, u16)>,
}

impl Media {
    pub fn empty(uid: u64) -> Self {
        Media {
            uid,
            cells: vec![],
            present: vec![],
            logical_len: 0,
            revision: 0,
            credit: 0,
            recycle: None,
        }
    }

    pub fn from_cells(uid: u64, data: &[u16]) -> Self {
        let pages = pages_for(data.len());
        let mut cells = vec![0u16; pages * PAGE_CELLS];
        cells[..data.len()].copy_from_slice(data);
        Media {
            uid,
            cells,
            present: vec![true; pages],
            logical_len: data.len() as u32,
            revision: 0,
            credit: 0,
            recycle: None,
        }
    }

    pub fn slots(&self) -> usize {
        self.present.len()
    }

    pub fn capacity(&self) -> u32 {
        (self.slots() * PAGE_CELLS) as u32
    }

    pub fn present_pages(&self) -> u32 {
        self.present.iter().filter(|p| **p).count() as u32
    }

    pub fn has_hole(&self) -> bool {
        self.present.iter().any(|p| !*p)
    }

    pub fn read(&self, i: u32) -> Option<u16> {
        let i = i as usize;
        if i >= self.cells.len() || !self.present[i / PAGE_CELLS] {
            return None;
        }
        Some(self.cells[i])
    }

    /// Write a cell inside present capacity. Does not bump revision (callers do per transaction).
    pub fn write(&mut self, i: u32, v: u16) -> bool {
        let iu = i as usize;
        if iu >= self.cells.len() || !self.present[iu / PAGE_CELLS] {
            return false;
        }
        self.cells[iu] = v;
        if i + 1 > self.logical_len {
            self.logical_len = i + 1;
        }
        if let Some((page, _)) = self.recycle {
            if page as usize == iu / PAGE_CELLS {
                self.recycle = Some((page, 0));
            }
        }
        true
    }

    /// Readable prefix [0, n) with no holes and within logical length.
    pub fn prefix(&self, n: u32) -> Option<&[u16]> {
        if n > self.logical_len || n as usize > self.cells.len() {
            return None;
        }
        let pages = pages_for(n as usize);
        if self.present[..pages].iter().any(|p| !*p) {
            return None;
        }
        Some(&self.cells[..n as usize])
    }

    pub fn push_page(&mut self) {
        self.cells.extend(std::iter::repeat_n(0, PAGE_CELLS));
        self.present.push(true);
        self.recycle = None;
        self.revision += 1;
    }

    /// Restore the smallest hole index as a zeroed present page.
    pub fn restore_hole(&mut self) -> bool {
        if let Some(i) = self.present.iter().position(|p| !*p) {
            self.present[i] = true;
            self.cells[i * PAGE_CELLS..(i + 1) * PAGE_CELLS].fill(0);
            self.recycle = None;
            self.revision += 1;
            true
        } else {
            false
        }
    }

    /// Remove the last slot (must be absent or fully recycled by the caller).
    pub fn pop_slot(&mut self) {
        self.present.pop();
        self.cells.truncate(self.present.len() * PAGE_CELLS);
        self.logical_len = self.logical_len.min(self.capacity());
        self.recycle = None;
        self.revision += 1;
    }

    /// Turn the highest present page into a hole (oxidation). Returns false if none.
    pub fn oxidize_top_page(&mut self) -> bool {
        if let Some(i) = self.present.iter().rposition(|p| *p) {
            self.present[i] = false;
            self.cells[i * PAGE_CELLS..(i + 1) * PAGE_CELLS].fill(0);
            if let Some((page, _)) = self.recycle {
                if page as usize == i {
                    self.recycle = None;
                }
            }
            self.revision += 1;
            true
        } else {
            false
        }
    }
}
