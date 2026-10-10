//! M1a records: drafts, public tapes, signals (SPEC §20b), plus the shared paid-page
//! lifecycle used by embryo, draft and public media (original §55).

use crate::agent::Bank;
use crate::effects::Res;
use crate::genome::{self, Program};
use std::sync::Arc;
use crate::genome::{MAX_PAGES, PAGE_CELLS};
use crate::intent::{dom, unpack, Intent, Outcome};
use crate::media::Media;
use crate::sim::{signal_key, Sim};
use crate::species::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MediaRef {
    Embryo,
    Draft,
    Public { tile: u32, uid: u64 },
}

impl Sim {
    pub(crate) fn media_ref(&self, i: usize, r: MediaRef) -> Option<&Media> {
        match r {
            MediaRef::Embryo => self.st.agents[i].embryo.as_ref().map(|e| &e.raw),
            MediaRef::Draft => self.st.agents[i].draft.as_ref(),
            MediaRef::Public { tile, uid } => self.st.dropped.get(&tile)?.iter().find(|m| m.uid == uid),
        }
    }

    pub(crate) fn media_mut(&mut self, i: usize, r: MediaRef) -> Option<&mut Media> {
        match r {
            MediaRef::Embryo => self.st.agents[i].embryo.as_mut().map(|e| &mut e.raw),
            MediaRef::Draft => self.st.agents[i].draft.as_mut(),
            MediaRef::Public { tile, uid } => self.st.dropped.get_mut(&tile)?.iter_mut().find(|m| m.uid == uid),
        }
    }

    fn media_tile(&self, i: usize, r: MediaRef) -> u32 {
        match r {
            MediaRef::Public { tile, .. } => tile,
            _ => self.tile_of(i),
        }
    }

    /// Public record addressed by (packed offset, selector) at resolve time.
    pub(crate) fn public_at(&self, i: usize, off: u8, selector: u16) -> Result<MediaRef, Outcome> {
        let t = self.rel_tile(i, off).ok_or(Outcome::Boundary)?;
        if selector == 0 {
            return Err(Outcome::InvalidTarget); // parcel media: none in M1a
        }
        let m = self
            .st
            .dropped
            .get(&t)
            .and_then(|l| l.get(selector as usize - 1))
            .ok_or(Outcome::InvalidTarget)?;
        Ok(MediaRef::Public { tile: t, uid: m.uid })
    }

    /// Public record addressed by a WRITE (r1/r2 relative position within contact, r6 selector).
    fn public_by_vec(&self, i: usize, r: &[u16; 8]) -> Result<MediaRef, Outcome> {
        let t = self.rel_vec_tile(i, r[1] as i16, r[2] as i16, 1)?;
        if r[6] == 0 {
            return Err(Outcome::InvalidTarget);
        }
        let m = self.st.dropped.get(&t).and_then(|l| l.get(r[6] as usize - 1)).ok_or(Outcome::InvalidTarget)?;
        Ok(MediaRef::Public { tile: t, uid: m.uid })
    }

    /// Remove a public record that has no slots left.
    fn prune_public(&mut self, r: MediaRef) {
        if let MediaRef::Public { tile, uid } = r {
            if let Some(list) = self.st.dropped.get_mut(&tile) {
                list.retain(|m| m.uid != uid || m.slots() > 0);
                if list.is_empty() {
                    self.st.dropped.remove(&tile);
                }
            }
        }
    }

    /// Paid page lifecycle: add / restore / cancel cursor / drop absent slot / recycle 4 cells.
    pub(crate) fn media_resize(&mut self, i: usize, r: MediaRef, target: u32) -> Res {
        if target as usize > MAX_PAGES {
            return (Outcome::Limits, 0);
        }
        let me = self.tile_of(i);
        let mt = self.media_tile(i, r);
        let (n, has_hole, last_present, recycle) = match self.media_ref(i, r) {
            Some(m) => (m.slots() as u32, m.has_hole(), m.present.last().copied(), m.recycle),
            None if r == MediaRef::Draft => (0, false, None, None),
            None => return (Outcome::InvalidTarget, 0),
        };
        if target > n || (target == n && has_hole) {
            let w = &self.st.world;
            let dug = MASS[G] as i64 * (w.elev[mt as usize] as i64 - w.elev[me as usize] as i64);
            if (self.st.agents[i].s.budget as i64) < 128 + dug.max(0) {
                return (Outcome::Energy, 0);
            }
            if self.st.agents[i].inv[G] < 1 {
                return (Outcome::Matter, 0);
            }
            if r == MediaRef::Draft && self.st.agents[i].draft.is_none() {
                let uid = self.alloc_media_uid();
                self.st.agents[i].draft = Some(Media::empty(uid));
                self.obs.m1.drafts_created += 1;
            }
            self.st.agents[i].inv[G] -= 1;
            self.move_heat(me, mt, CAP[G]);
            if dug > 0 {
                self.pay_ug(i, dug as u32);
            } else if dug < 0 {
                self.st.world.heat_u[mt as usize] += (-dug) as u64;
            }
            self.pay(i, 128, mt);
            let m = self.media_mut(i, r).expect("media");
            if target > n {
                m.push_page();
            } else {
                m.restore_hole();
            }
            return (Outcome::Ok, 1);
        }
        if target == n {
            if recycle.is_some() {
                self.media_mut(i, r).unwrap().recycle = None;
                return (Outcome::Ok, 0);
            }
            return (Outcome::Noop, 0);
        }
        let last = n - 1;
        if last_present == Some(false) {
            self.media_mut(i, r).unwrap().pop_slot();
            self.after_shrink(i, r);
            return (Outcome::Ok, 1);
        }
        let next = match recycle {
            Some((page, next)) if page as u32 == last => next as u32,
            _ => 0,
        };
        let k = 4u32.min(PAGE_CELLS as u32 - next).min(self.st.agents[i].s.budget / 32);
        if k == 0 {
            return (Outcome::Energy, 0);
        }
        self.pay(i, 32 * k, mt);
        let m = self.media_mut(i, r).unwrap();
        let base = last as usize * PAGE_CELLS + next as usize;
        m.cells[base..base + k as usize].fill(0);
        m.revision += 1;
        let next = next + k;
        if next as usize == PAGE_CELLS {
            m.pop_slot();
            self.st.world.force_loose(mt, G, 1);
            self.after_shrink(i, r);
            return (Outcome::Ok, 1);
        }
        m.recycle = Some((last as u16, next as u16));
        (Outcome::Partial, 0)
    }

    fn after_shrink(&mut self, i: usize, r: MediaRef) {
        match r {
            MediaRef::Draft => {
                if self.st.agents[i].draft.as_ref().is_some_and(|d| d.slots() == 0) {
                    self.st.agents[i].draft = None;
                }
            }
            MediaRef::Public { .. } => self.prune_public(r),
            MediaRef::Embryo => {}
        }
    }

    /// WRITE Cell into an embryo (Writing), draft or public record.
    pub(crate) fn media_write_cell(&mut self, i: usize, r: MediaRef, index: u32, value: u16) -> Res {
        let mt = self.media_tile(i, r);
        match self.media_ref(i, r) {
            None => return (Outcome::InvalidTarget, 0),
            Some(m) if m.read(index).is_none() => return (Outcome::Capacity, 0),
            _ => {}
        }
        if self.st.agents[i].s.budget < 16 {
            return (Outcome::Energy, 0);
        }
        self.pay(i, 16, mt);
        let m = self.media_mut(i, r).unwrap();
        m.write(index, value);
        m.revision += 1;
        if matches!(r, MediaRef::Public { .. }) {
            self.obs.m1.public_writes += 1;
        }
        (Outcome::Ok, 1)
    }

    /// WRITE sub-ops 0 Cell, 1 Resize, 3 CancelDraft on draft (7) and public (5) targets.
    pub(crate) fn eff_write_record(&mut self, i: usize, it: &Intent) -> Res {
        let r = it.r;
        if it.sub == 2 {
            return if self.caps.commit { self.draft_commit(i, r[5] as u32) } else { (Outcome::InvalidTarget, 0) };
        }
        let target = match r[0] {
            dom::OWN_DRAFT => MediaRef::Draft,
            dom::PUBLIC_TAPE => match self.public_by_vec(i, &r) {
                Ok(t) => t,
                Err(e) => return (e, 0),
            },
            _ => return (Outcome::InvalidTarget, 0),
        };
        match it.sub {
            0 => self.media_write_cell(i, target, r[4] as u32, r[5]),
            1 => {
                let res = self.media_resize(i, target, r[5] as u32);
                if matches!(target, MediaRef::Public { .. }) && matches!(res.0, Outcome::Ok | Outcome::Partial) {
                    self.obs.m1.public_resizes += 1;
                }
                res
            }
            _ => (Outcome::InvalidTarget, 0),
        }
    }

    /// DraftCommit (WRITE 2): validate the draft prefix; adoption happens at stage 11 (SPEC §20c).
    pub(crate) fn draft_commit(&mut self, i: usize, len: u32) -> Res {
        let Some(d) = self.st.agents[i].draft.as_ref() else { return (Outcome::InvalidTarget, 0) };
        if len == 0 || len > d.logical_len {
            return (Outcome::InvalidTarget, 0);
        }
        if self.st.agents[i].s.budget < 2 * len {
            return (Outcome::Energy, 0);
        }
        let me = self.tile_of(i);
        self.pay(i, 2 * len, me);
        let a = &self.st.agents[i];
        let Some(p) = a.draft.as_ref().unwrap().prefix(len).and_then(|c| genome::parse(c).ok()) else {
            return (Outcome::Format, 0);
        };
        let (bound, words) = binding_size(a.program(), &p);
        if bound > genome::MAX_MODULES || words > genome::MAX_WORDS {
            return (Outcome::Limits, 0);
        }
        self.st.agents[i].s.pending_adopt = Some(len);
        self.obs.m1.commits += 1;
        (Outcome::Ok, 1)
    }

    /// Adopt the committed draft: learned binding, retained old code, slot remap.
    pub(crate) fn adopt(&mut self, i: usize, len: u32) {
        let me = self.tile_of(i);
        let a = &mut self.st.agents[i];
        let Some(draft) = a.draft.take() else { return };
        let p = Arc::new(genome::parse(draft.prefix(len).expect("validated")).expect("validated"));
        let new_uid = draft.uid;
        if let Some(old) = a.learned.take() {
            a.retained.push(old);
        }
        a.learned = Some(Bank { media: draft, used_len: len });
        a.bank_progs.push((new_uid, p.clone()));
        // binding: learned tags first, then inherited tags the learned code does not define
        let inh = a.prog.clone().expect("program");
        let mut targets: Vec<(u16, (u64, u16))> =
            p.modules.iter().enumerate().map(|(k, m)| (m.tag, (new_uid, k as u16))).collect();
        for (k, m) in inh.modules.iter().enumerate() {
            if !p.modules.iter().any(|x| x.tag == m.tag) {
                targets.push((m.tag, (a.code.uid, k as u16)));
            }
        }
        let entry_tag = p.entry_tag;
        let dropped = remap_slots(a, targets, entry_tag);
        self.obs.m1.adoptions += 1;
        for m in dropped {
            self.drop_media(me, m);
        }
    }

    /// Drop retained banks that no running frame references any more.
    pub(crate) fn gc_retained(&mut self, i: usize) {
        let me = self.tile_of(i);
        let a = &mut self.st.agents[i];
        let targets: Vec<(u16, (u64, u16))> = a.binding.iter().map(|(t, s)| (*t, a.slots[*s as usize])).collect();
        let entry_tag = a.binding.iter().find(|(_, s)| *s == a.entry_slot).map(|(t, _)| *t).unwrap_or(0);
        let dropped = remap_slots(a, targets, entry_tag);
        for m in dropped {
            self.drop_media(me, m);
        }
    }

    pub(crate) fn cancel_draft(&mut self, i: usize) -> Res {
        let me = self.tile_of(i);
        match self.st.agents[i].draft.take() {
            Some(d) => {
                self.drop_media(me, d);
                (Outcome::Ok, 1)
            }
            None => (Outcome::InvalidTarget, 0),
        }
    }

    /// EMIT: own tile +a, four neighbours +trunc(a/2) on the next field.
    pub(crate) fn eff_emit(&mut self, i: usize, it: &Intent) -> Res {
        if !self.caps.m1 {
            return (Outcome::InvalidTarget, 0);
        }
        let ch = it.r[3];
        let amp = it.r[5] as i16 as i64;
        if ch > 3 || !(-255..=255).contains(&amp) {
            return (Outcome::InvalidTarget, 0);
        }
        if amp == 0 {
            return (Outcome::Noop, 0);
        }
        let cost = 2 * amp.unsigned_abs() as u32;
        if self.st.agents[i].s.budget < cost {
            return (Outcome::Energy, 0);
        }
        let me = self.tile_of(i);
        self.pay(i, cost, me);
        *self.signal_next.entry(signal_key(me, ch)).or_insert(0) += amp;
        for d in 0..4 {
            if let Some(nb) = self.st.world.neighbor(me, d) {
                *self.signal_next.entry(signal_key(nb, ch)).or_insert(0) += amp / 2;
            }
        }
        self.obs.m1.emits += 1;
        (Outcome::Ok, amp.unsigned_abs() as u32)
    }

    /// COPY with M1a sources/destinations. Sources: OwnCode 6, Embryo 8, Draft 7, Public 5.
    /// Destinations: Embryo 8 (own sources 6/7/8 only, Writing), Draft 7, Public 5.
    pub(crate) fn eff_copy(&mut self, i: usize, it: &Intent) -> Res {
        let r = it.r;
        let (Some((sd, so)), Some((dd, doff))) = (unpack(r[0]), unpack(r[1])) else {
            return (Outcome::InvalidTarget, 0);
        };
        let m1 = self.caps.m1;
        let src_ok = match (sd, so) {
            (dom::OWN_CODE, 0) => self.st.agents[i].bank_media(r[7]).is_some(),
            (dom::EMBRYO, 0) => true,
            (dom::OWN_DRAFT, 0) => m1,
            (dom::PUBLIC_TAPE, _) => m1,
            _ => false,
        };
        let dst = match (dd, doff) {
            (dom::EMBRYO, 0) if sd != dom::PUBLIC_TAPE => MediaRef::Embryo,
            (dom::OWN_DRAFT, 0) if m1 => MediaRef::Draft,
            (dom::PUBLIC_TAPE, off) if m1 => match self.public_at(i, off, r[3]) {
                Ok(t) => t,
                Err(e) => return (e, 0),
            },
            _ => return (Outcome::InvalidTarget, 0),
        };
        if !src_ok {
            return (Outcome::InvalidTarget, 0);
        }
        let count = r[6] as u32;
        if !(1..=4).contains(&count) {
            return (Outcome::InvalidTarget, 0);
        }
        let Some(cap) = it.copy.clone() else { return (Outcome::InvalidTarget, 0) };
        if dst == MediaRef::Embryo
            && self.st.agents[i].embryo.as_ref().map(|e| e.state) != Some(crate::agent::EmbryoState::Writing)
        {
            return (Outcome::InvalidTarget, 0);
        }
        // Source still the same record and revision?
        let a = &self.st.agents[i];
        let cur = match sd {
            dom::OWN_CODE => a.bank_media(r[7]).map(|m| (m.uid, m.revision)),
            dom::EMBRYO => a.embryo.as_ref().map(|e| (e.raw.uid, e.raw.revision)),
            dom::OWN_DRAFT => a.draft.as_ref().map(|d| (d.uid, d.revision)),
            _ => cap
                .src_tile
                .and_then(|t| self.st.dropped.get(&t))
                .and_then(|l| l.iter().find(|m| m.uid == cap.src_uid))
                .map(|m| (m.uid, m.revision)),
        };
        if cur != Some((cap.src_uid, cap.src_rev)) {
            return (Outcome::Stale, 0);
        }
        if cap.bytes.is_empty() {
            return (Outcome::InvalidTarget, 0);
        }
        let Some(dm) = self.media_ref(i, dst) else { return (Outcome::InvalidTarget, 0) };
        let dst_off = r[5] as u32;
        let mut room = 0u32;
        while room < cap.bytes.len() as u32 && dm.read(dst_off + room).is_some() {
            room += 1;
        }
        let by_energy = self.st.agents[i].s.budget / 32;
        let n = (cap.bytes.len() as u32).min(room).min(by_energy);
        if n == 0 {
            return if by_energy == 0 { (Outcome::Energy, 0) } else { (Outcome::Capacity, 0) };
        }
        let mt = self.media_tile(i, dst);
        self.pay(i, 32 * n, mt);
        let foreign_src = match sd {
            dom::PUBLIC_TAPE => true,
            dom::OWN_DRAFT => self.st.agents[i].draft.as_ref().is_some_and(|d| d.foreign),
            dom::EMBRYO => self.st.agents[i].embryo.as_ref().is_some_and(|e| e.raw.foreign),
            _ => false,
        };
        let m = self.media_mut(i, dst).unwrap();
        for k in 0..n {
            m.write(dst_off + k, cap.bytes[k as usize]);
        }
        m.revision += 1;
        if foreign_src && !matches!(dst, MediaRef::Public { .. }) {
            m.foreign = true;
        }
        match (sd, dst) {
            (dom::PUBLIC_TAPE, _) => self.obs.m1.copies_from_public += 1,
            (_, MediaRef::Public { .. }) => self.obs.m1.copies_to_public += 1,
            (dom::OWN_DRAFT, MediaRef::Embryo) => self.obs.m1.draft_to_embryo += 1,
            _ => {}
        }
        (if n == count { Outcome::Ok } else { Outcome::Partial }, n)
    }
}

/// Number of bound modules and words if `learned` were adopted over `inherited`.
fn binding_size(inherited: &Program, learned: &Program) -> (usize, usize) {
    let mut bound = learned.modules.len();
    let mut words = learned.words.len();
    for m in &inherited.modules {
        if !learned.modules.iter().any(|x| x.tag == m.tag) {
            bound += 1;
            words += m.len as usize;
        }
    }
    (bound, words)
}

/// Rebuild the slot table from binding targets, keeping slots the VM still references,
/// remap the VM, and return banks that are no longer referenced (to be dropped).
fn remap_slots(a: &mut crate::agent::Agent, targets: Vec<(u16, (u64, u16))>, entry_tag: u16) -> Vec<Media> {
    let old_slot = |a: &crate::agent::Agent, s: u8| -> (u64, u16) {
        if a.slots.is_empty() {
            (a.code.uid, s as u16)
        } else {
            a.slots[s as usize]
        }
    };
    let mut table: Vec<(u64, u16)> = Vec::new();
    let mut binding = Vec::new();
    for (tag, tgt) in targets {
        binding.push((tag, table.len() as u8));
        table.push(tgt);
    }
    let map = |a: &crate::agent::Agent, s: u8, table: &mut Vec<(u64, u16)>| -> u8 {
        let tgt = old_slot(a, s);
        match table.iter().position(|x| *x == tgt) {
            Some(k) => k as u8,
            None => {
                table.push(tgt);
                (table.len() - 1) as u8
            }
        }
    };
    let module = map(a, a.vm.module, &mut table);
    let stack: Vec<(u8, u16)> = a.vm.stack.clone().into_iter().map(|(s, pc)| (map(a, s, &mut table), pc)).collect();
    a.vm.module = module;
    a.vm.stack = stack;
    a.entry_slot = binding.iter().find(|(t, _)| *t == entry_tag).map(|(_, s)| *s).unwrap_or(0);
    a.slots = table;
    a.binding = binding;
    // banks still referenced by some slot stay; others leave
    let learned_uid = a.learned.as_ref().map(|b| b.media.uid);
    let used: Vec<u64> = a.slots.iter().map(|s| s.0).collect();
    let mut dropped = Vec::new();
    let mut keep = Vec::new();
    for b in std::mem::take(&mut a.retained) {
        if used.contains(&b.media.uid) && Some(b.media.uid) != learned_uid {
            keep.push(b);
        } else {
            dropped.push(b.media);
        }
    }
    a.retained = keep;
    let live: Vec<u64> = a.learned.iter().chain(&a.retained).map(|b| b.media.uid).collect();
    a.bank_progs.retain(|(uid, _)| live.contains(uid));
    dropped
}
