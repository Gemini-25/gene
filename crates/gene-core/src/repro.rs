//! Embryo state machine: REPRO sub-ops, embryo WRITE/Resize. SPEC §11.

use crate::agent::{Embryo, EmbryoState, MutationRecord, BODY_G, BODY_S, INV_VOLUME, MIN_CHILD_ENERGY};
use crate::effects::Res;
use crate::genome::{self, pages_for};
use crate::intent::{sub, Intent, Outcome};
use crate::media::Media;
use crate::rng::{stream_key, Domain, Stream};
use crate::sim::Sim;
use crate::species::*;

impl Sim {
    pub(crate) fn eff_repro(&mut self, i: usize, it: &Intent) -> Option<Res> {
        Some(match it.sub {
            sub::REPRO_ALLOCATE => self.repro_allocate(i),
            sub::REPRO_RESIZE => self.embryo_resize(i, it.r[5] as u32),
            sub::REPRO_FINISH => self.repro_finish(i, it.r[5] as u32),
            sub::REPRO_STEP => self.repro_step(i),
            sub::REPRO_REOPEN => self.repro_reopen(i),
            sub::REPRO_RELEASE => return self.repro_release(i, it.r[5]),
            sub::REPRO_CANCEL => self.repro_cancel(i),
            _ => (Outcome::InvalidTarget, 0),
        })
    }

    fn embryo_state(&self, i: usize) -> Option<EmbryoState> {
        self.st.agents[i].embryo.as_ref().map(|e| e.state)
    }

    fn repro_allocate(&mut self, i: usize) -> Res {
        if self.st.agents[i].embryo.is_some() {
            return (Outcome::InvalidTarget, 0);
        }
        let need_g = BODY_G + 1;
        let a = &self.st.agents[i];
        if a.inv[G] < need_g || a.inv[S] < BODY_S {
            return (Outcome::Matter, 0);
        }
        let uid = self.alloc_media_uid();
        let a = &mut self.st.agents[i];
        a.inv[G] -= need_g;
        a.inv[S] -= BODY_S;
        a.embryo_ordinal += 1;
        a.embryo = Some(Embryo {
            state: EmbryoState::Writing,
            body_g: BODY_G,
            shell_s: BODY_S,
            raw: Media::empty(uid),
            result: None,
            plan: None,
            step_cursor: 0,
            used_len: 0,
            mutation_applied: false,
            energy: 0,
            ret: [0; NSPEC],
            ordinal: a.embryo_ordinal,
            mutation: MutationRecord::default(),
        });
        (Outcome::Ok, 1)
    }

    pub(crate) fn embryo_write_cell(&mut self, i: usize, index: u32, value: u16) -> Res {
        if self.embryo_state(i) != Some(EmbryoState::Writing) {
            return (Outcome::InvalidTarget, 0);
        }
        if self.st.agents[i].embryo.as_ref().unwrap().raw.read(index).is_none() {
            return (Outcome::Capacity, 0);
        }
        if self.st.agents[i].s.budget < 16 {
            return (Outcome::Energy, 0);
        }
        let me = self.tile_of(i);
        self.pay(i, 16, me);
        let raw = &mut self.st.agents[i].embryo.as_mut().unwrap().raw;
        raw.write(index, value);
        raw.revision += 1;
        (Outcome::Ok, 1)
    }

    pub(crate) fn embryo_resize(&mut self, i: usize, target: u32) -> Res {
        if self.embryo_state(i) != Some(EmbryoState::Writing) {
            return (Outcome::InvalidTarget, 0);
        }
        self.media_resize(i, crate::records::MediaRef::Embryo, target)
    }

    fn repro_finish(&mut self, i: usize, len: u32) -> Res {
        if self.embryo_state(i) != Some(EmbryoState::Writing) {
            return (Outcome::InvalidTarget, 0);
        }
        let cap = self.st.agents[i].embryo.as_ref().unwrap().raw.capacity();
        if len == 0 || len > cap {
            return (Outcome::InvalidTarget, 0);
        }
        if self.st.agents[i].s.budget < 2 * len {
            return (Outcome::Energy, 0);
        }
        let me = self.tile_of(i);
        self.pay(i, 2 * len, me);
        let seed = self.st.config.seed;
        let tick = self.st.tick;
        let parsed = {
            let e = self.st.agents[i].embryo.as_ref().unwrap();
            e.raw.prefix(len).map(|p| (p.to_vec(), genome::parse(p).is_ok()))
        };
        let Some((prefix, true)) = parsed else {
            self.st.agents[i].embryo.as_mut().unwrap().state = EmbryoState::Invalid;
            self.obs.mutation.finish_invalid += 1;
            return (Outcome::Format, 1);
        };
        self.obs.mutation.finished += 1;
        let applied = self.st.agents[i].embryo.as_ref().unwrap().mutation_applied;
        if applied {
            let e = self.st.agents[i].embryo.as_mut().unwrap();
            e.used_len = len;
            e.state = EmbryoState::Ready;
            return (Outcome::Ok, 1);
        }
        let a = &self.st.agents[i];
        let ordinal = a.embryo.as_ref().unwrap().ordinal;
        let mut st = Stream::new(stream_key(seed, Domain::Mutation, a.uid, ordinal), tick);
        let learned = if self.caps.commit {
            a.learned.as_ref().and_then(|b| a.bank_progs.iter().find(|p| p.0 == b.media.uid)).map(|p| p.1.clone())
        } else {
            None
        };
        // mating (SPEC §20n): the first neighbour north, east, south, west, 1 time in MATE_ODDS
        let mut prefix_m = None;
        if let Some(mode) = self.caps.mate {
            let t = self.tile_of(i);
            let partner = (0..4u8).filter_map(|d| self.st.world.neighbor(t, d)).find_map(|u| self.agent_at(u));
            if let Some(j) = partner {
                if st.below(crate::mutation::MATE_ODDS) == 0 {
                    let other = self.st.agents[j].program();
                    prefix_m = Some(match mode {
                        crate::sim::Mating::Any => crate::mutation::mate(&prefix, other, &mut st),
                        crate::sim::Mating::Homologous => crate::mutation::mate_homologous(&prefix, other, &mut st),
                    });
                }
            }
        }
        let a = &self.st.agents[i];
        let mated = prefix_m.as_ref().is_some_and(|m| *m != prefix);
        let (result, rec) = crate::mutation::mutate_with(prefix_m.as_deref().unwrap_or(&prefix), a.program(), learned.as_deref(), &mut st);
        if mated {
            self.obs.mutation.mated += 1;
        }
        if let Some(op) = rec.structural_op {
            if rec.structural_applied {
                self.obs.mutation.structural[op as usize] += 1;
            } else {
                self.obs.mutation.structural_noop += 1;
            }
        }
        self.obs.mutation.point_flips += rec.point_flips as u64;
        let same = result == prefix;
        let new_uid = if same { 0 } else { self.alloc_media_uid() };
        let e = self.st.agents[i].embryo.as_mut().unwrap();
        e.used_len = len;
        e.mutation = rec;
        if same {
            e.mutation_applied = true;
            e.state = EmbryoState::Ready;
            self.obs.mutation.unchanged += 1;
        } else {
            e.plan = Some(result);
            e.result = Some(Media::empty(new_uid));
            e.step_cursor = 0;
            e.state = EmbryoState::SealedWork;
            self.obs.mutation.sealed += 1;
        }
        (Outcome::Ok, 1)
    }

    fn repro_step(&mut self, i: usize) -> Res {
        if self.embryo_state(i) != Some(EmbryoState::SealedWork) {
            return (Outcome::InvalidTarget, 0);
        }
        let me = self.tile_of(i);
        let (len, cursor, slots) = {
            let e = self.st.agents[i].embryo.as_ref().unwrap();
            (e.plan.as_ref().unwrap().len() as u32, e.step_cursor, e.result.as_ref().unwrap().slots())
        };
        let mut wrote = 0u32;
        if cursor < len {
            let budget = self.st.agents[i].s.budget;
            let g = self.st.agents[i].inv[G] as usize;
            let max_k = 4.min(len - cursor);
            let mut k = 0;
            let mut first_fail = Outcome::Energy;
            for cand in (1..=max_k).rev() {
                let pages = pages_for((cursor + cand) as usize).saturating_sub(slots);
                let cost = 32 * cand + 128 * pages as u32;
                if cost <= budget && pages <= g {
                    k = cand;
                    break;
                }
                if cand == 1 {
                    first_fail = if cost > budget { Outcome::Energy } else { Outcome::Matter };
                }
            }
            if k == 0 {
                return (first_fail, 0);
            }
            let pages = pages_for((cursor + k) as usize).saturating_sub(slots);
            self.pay(i, 32 * k + 128 * pages as u32, me);
            let a = &mut self.st.agents[i];
            a.inv[G] -= pages as u16;
            let e = a.embryo.as_mut().unwrap();
            let plan = e.plan.as_ref().unwrap();
            let res = e.result.as_mut().unwrap();
            for _ in 0..pages {
                res.push_page();
            }
            for c in cursor..cursor + k {
                res.write(c, plan[c as usize]);
            }
            res.revision += 1;
            e.step_cursor += k;
            wrote = k;
        }
        let done = self.st.agents[i].embryo.as_ref().unwrap().step_cursor == len;
        if !done {
            return (Outcome::Ok, wrote);
        }
        if self.st.agents[i].s.budget < 2 * len {
            return if wrote > 0 { (Outcome::Partial, wrote) } else { (Outcome::Energy, 0) };
        }
        self.pay(i, 2 * len, me);
        let e = self.st.agents[i].embryo.as_mut().unwrap();
        let plan = e.plan.take().unwrap();
        let mut result = e.result.take().unwrap();
        result.foreign = e.raw.foreign;
        let old = std::mem::replace(&mut e.raw, result);
        e.used_len = len;
        e.mutation_applied = true;
        let valid = genome::parse(&plan).is_ok();
        e.state = if valid { EmbryoState::Ready } else { EmbryoState::Invalid };
        if valid {
            self.obs.mutation.step_valid += 1;
        } else {
            self.obs.mutation.step_invalid += 1;
        }
        self.drop_media(me, old);
        (Outcome::Ok, wrote.max(1))
    }

    fn repro_reopen(&mut self, i: usize) -> Res {
        match self.embryo_state(i) {
            Some(EmbryoState::Invalid | EmbryoState::Ready) => {
                self.st.agents[i].embryo.as_mut().unwrap().state = EmbryoState::Writing;
                (Outcome::Ok, 1)
            }
            _ => (Outcome::InvalidTarget, 0),
        }
    }

    fn repro_release(&mut self, i: usize, dir: u16) -> Option<Res> {
        if self.embryo_state(i) != Some(EmbryoState::Ready) || dir > 3 {
            return Some((Outcome::InvalidTarget, 0));
        }
        if self.st.agents[i].embryo.as_ref().unwrap().energy < MIN_CHILD_ENERGY {
            return Some((Outcome::Energy, 0));
        }
        let abs = (self.st.agents[i].orient + dir as u8) & 3;
        let me = self.tile_of(i);
        let Some(t) = self.st.world.neighbor(me, abs) else { return Some((Outcome::Boundary, 0)) };
        if self.st.world.occ[t as usize] != 0 || self.reserved.contains(&t) || self.st.world.blocked(t) {
            return Some((Outcome::Blocked, 0));
        }
        self.reserved.insert(t);
        self.st.agents[i].s.birth_tile = Some(t);
        None
    }

    fn repro_cancel(&mut self, i: usize) -> Res {
        let me = self.tile_of(i);
        let Some(e) = self.st.agents[i].embryo.take() else { return (Outcome::InvalidTarget, 0) };
        let mut back = e.ret;
        back[G] += e.body_g + 1;
        back[S] += e.shell_s;
        let a = &mut self.st.agents[i];
        let mut vol = a.inv_volume();
        for s in 0..NSPEC {
            let fit = ((INV_VOLUME.saturating_sub(vol)) / VOL[s]).min(back[s] as u32);
            a.inv[s] += fit as u16;
            vol += fit * VOL[s];
            let rest = back[s] as u64 - fit as u64;
            if rest > 0 {
                self.st.world.force_loose(me, s, rest);
            }
        }
        self.st.world.heat_u[me as usize] += e.energy as u64;
        self.drop_media(me, e.raw);
        if let Some(r) = e.result {
            self.drop_media(me, r);
        }
        (Outcome::Ok, 1)
    }
}
