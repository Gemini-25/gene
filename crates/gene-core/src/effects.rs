//! Stage 4 LocalResolve: TRANSFER, DRIVE Rotate, WRITE, WORK, COPY, LINK dispatch. SPEC §10, §13.

use crate::agent::{ENERGY_MAX, INV_VOLUME};
use crate::intent::{dom, rotate, sub, unpack, Intent, Outcome, OFFSETS};
use crate::isa::op;
use crate::rng::{permutation, stream_key, Domain, Stream};
use crate::sim::Sim;
use crate::species::*;

pub type Res = (Outcome, u32);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Store {
    Inv,
    Loose(u8),
    Actor(u8),
    EmbryoRet,
    Matrix(u8),
    Pore(u8),
}

pub(crate) fn priority(key: [u32; 2], tick: u64) -> u64 {
    let mut s = Stream::new(key, tick);
    let lo = s.next_u32() as u64;
    let hi = s.next_u32() as u64;
    lo | hi << 32
}

impl Sim {
    pub(crate) fn rel_tile(&self, i: usize, off: u8) -> Option<u32> {
        let a = &self.st.agents[i];
        let (dx, dy) = OFFSETS[off as usize];
        let (ax, ay) = rotate(a.orient, dx, dy);
        self.st.world.offset(self.tile_of(i), ax, ay)
    }

    pub(crate) fn rel_vec_tile(&self, i: usize, dx: i16, dy: i16, radius: i32) -> Result<u32, Outcome> {
        let (dx, dy) = (dx as i32, dy as i32);
        if dx.abs() + dy.abs() > radius {
            return Err(Outcome::InvalidTarget);
        }
        let a = &self.st.agents[i];
        let (ax, ay) = rotate(a.orient, dx, dy);
        self.st.world.offset(self.tile_of(i), ax, ay).ok_or(Outcome::Boundary)
    }

    pub(crate) fn agent_at(&self, t: u32) -> Option<usize> {
        match self.st.world.occ[t as usize] {
            0 => None,
            v => Some((v - 1) as usize),
        }
    }

    pub(crate) fn stage_local_resolve(&mut self) {
        let tick = self.st.tick;
        let seed = self.st.config.seed;
        let mut st = Stream::new(stream_key(seed, Domain::Resolve, 0, 4), tick);
        let ox = st.below(64) as i64;
        let oy = st.below(64) as i64;
        let perm = permutation(&mut st, 9);
        let mut rank = [0usize; 9];
        for (pos, &c) in perm.iter().enumerate() {
            rank[c] = pos;
        }
        let mut order: Vec<(usize, u64, u64, usize)> = Vec::new();
        for (i, a) in self.st.agents.iter().enumerate() {
            let Some(it) = &a.s.intent else { continue };
            // motion is stage 6; with bonds every DRIVE is stage 5 (it may carry an assembly)
            if it.opc == op::DRIVE && (it.sub == sub::DRIVE_TRANSLATE || self.caps.bonds) {
                continue;
            }
            let bx = (a.x as i64 - ox).div_euclid(64);
            let by = (a.y as i64 - oy).div_euclid(64);
            let color = (bx.rem_euclid(3) + 3 * by.rem_euclid(3)) as usize;
            order.push((rank[color], priority(a.keys.prio4, tick), a.uid, i));
        }
        order.sort_unstable();
        for (_, _, _, i) in order {
            let it = self.st.agents[i].s.intent.clone().expect("intent");
            let res = match it.opc {
                op::TRANSFER => self.eff_transfer(i, &it),
                op::DRIVE => self.eff_rotate(i, &it),
                op::WRITE => self.eff_write(i, &it),
                op::WORK => self.eff_work(i, &it),
                op::COPY => self.eff_copy(i, &it),
                op::REPRO => match self.eff_repro(i, &it) {
                    Some(r) => r,
                    None => continue, // Release reserved; receipt at stage 10
                },
                op::EMIT => self.eff_emit(i, &it),
                op::LINK | op::UNLINK if self.caps.bonds => self.eff_link(i, &it),
                _ => (Outcome::InvalidTarget, 0), // LINK, UNLINK before M1d
            };
            self.finish_effect(i, &it, res);
        }
    }

    pub(crate) fn finish_effect(&mut self, i: usize, it: &Intent, res: Res) {
        let a = &mut self.st.agents[i];
        a.receipt = Some((res.0 as u8, res.1.min(65535) as u16));
        self.obs.record_outcome(it.opc, it.sub, res.0 as u8);
        if let Some(f) = &mut self.obs.facts {
            let mut fact = crate::observer::EffectFact::new(self.st.tick, a.uid, it, res.0 as u8, res.1);
            if self.caps.contact {
                crate::contact::mark_fact(&mut fact, it);
            }
            f.push(fact);
        }
    }

    fn store_tile(&self, i: usize, s: Store) -> Result<u32, Outcome> {
        match s {
            Store::Loose(o) | Store::Actor(o) | Store::Matrix(o) | Store::Pore(o) => {
                self.rel_tile(i, o).ok_or(Outcome::Boundary)
            }
            _ => Ok(self.tile_of(i)),
        }
    }

    fn eff_transfer(&mut self, i: usize, it: &Intent) -> Res {
        let r = it.r;
        let (Some((sd, so)), Some((dd, doff))) = (unpack(r[0]), unpack(r[1])) else {
            return (Outcome::InvalidTarget, 0);
        };
        let ch = r[3];
        let amount = r[5] as u32;
        if ch == 15 {
            return self.transfer_energy(i, it, (sd, so), (dd, doff), amount);
        }
        if ch >= 10 {
            return (Outcome::InvalidTarget, 0);
        }
        let s = ch as usize;
        let parcels = self.caps.parcels;
        let src = match (sd, so) {
            (dom::SELF, 0) => Store::Inv,
            (dom::LOOSE, o) => Store::Loose(o),
            (dom::ACTOR, o) if o > 0 && self.caps.contact => Store::Actor(o),
            (dom::EMBRYO, 0) => Store::EmbryoRet,
            (dom::PARCEL, o) if parcels => Store::Matrix(o),
            (dom::PORE, o) if parcels => Store::Pore(o),
            _ => return (Outcome::InvalidTarget, 0),
        };
        let dst = match (dd, doff) {
            (dom::SELF, 0) => Store::Inv,
            (dom::LOOSE, o) => Store::Loose(o),
            (dom::ACTOR, o) if o > 0 => Store::Actor(o),
            (dom::EMBRYO, 0) => Store::EmbryoRet,
            (dom::PARCEL, o) if parcels => Store::Matrix(o),
            (dom::PORE, o) if parcels => Store::Pore(o),
            _ => return (Outcome::InvalidTarget, 0),
        };
        if (matches!(src, Store::Matrix(_)) || matches!(dst, Store::Matrix(_))) && !crate::world::SOLID[s] {
            return (Outcome::InvalidTarget, 0);
        }
        let src_tile = match self.store_tile(i, src) {
            Ok(t) => t,
            Err(e) => return (e, 0),
        };
        let dst_tile = match self.store_tile(i, dst) {
            Ok(t) => t,
            Err(e) => return (e, 0),
        };
        let has_embryo = self.st.agents[i].embryo.is_some();
        if (src == Store::EmbryoRet || dst == Store::EmbryoRet) && !has_embryo {
            return (Outcome::InvalidTarget, 0);
        }
        let mut target = None;
        if let Store::Actor(_) = dst {
            if let Store::Actor(_) = src {
                return (Outcome::InvalidTarget, 0); // one captured agent per effect
            }
            match (self.agent_at(dst_tile), it.target_uid) {
                (Some(j), Some(uid)) if self.st.agents[j].uid == uid => target = Some(j),
                _ => return (Outcome::Stale, 0),
            }
            if self.st.agents[target.unwrap()].port_mask & (1 << s) == 0 {
                return (Outcome::InvalidTarget, 0);
            }
        }
        // M1e: another agent's inventory is open to contact withdrawal only while its shell is 0
        if let Store::Actor(_) = src {
            match (self.agent_at(src_tile), it.target_uid) {
                (Some(j), Some(uid)) if self.st.agents[j].uid == uid => target = Some(j),
                _ => return (Outcome::Stale, 0),
            }
            if self.st.agents[target.unwrap()].shell_s > 0 {
                return (Outcome::Blocked, 0);
            }
        }
        // parcel endpoints: object identity (Stale) and access (Blocked).
        // The captured uid belongs to the source if it is a parcel store, else to the destination.
        let cap_src = matches!(src, Store::Matrix(_) | Store::Pore(_));
        for (store, t) in [(src, src_tile), (dst, dst_tile)] {
            let cur = self.st.world.parcels.get(&t);
            let captured = (store == src) == cap_src;
            match store {
                Store::Pore(_) => {
                    let Some(p) = cur else { return (Outcome::InvalidTarget, 0) };
                    if captured && it.parcel_uid != Some(p.uid) {
                        return (Outcome::Stale, 0);
                    }
                    if p.perm() < 32 {
                        return (Outcome::Blocked, 0);
                    }
                }
                Store::Matrix(_) if store == src => {
                    let Some(p) = cur else { return (Outcome::Matter, 0) };
                    if it.parcel_uid != Some(p.uid) {
                        return (Outcome::Stale, 0);
                    }
                    if p.c >= 64 {
                        return (Outcome::Blocked, 0);
                    }
                }
                Store::Matrix(_) => {
                    if captured && it.parcel_uid.is_some() && cur.map(|p| p.uid) != it.parcel_uid {
                        return (Outcome::Stale, 0);
                    }
                }
                _ => {}
            }
        }
        let same = match (src, dst) {
            (Store::Inv, Store::Inv) | (Store::EmbryoRet, Store::EmbryoRet) => true,
            (Store::Loose(_), Store::Loose(_))
            | (Store::Matrix(_), Store::Matrix(_))
            | (Store::Pore(_), Store::Pore(_)) => src_tile == dst_tile,
            _ => false,
        };
        if same || amount == 0 {
            return (Outcome::Noop, 0);
        }
        let a = &self.st.agents[i];
        let w = &self.st.world;
        let avail: u64 = match src {
            Store::Inv => a.inv[s] as u64,
            Store::Loose(_) => w.loose_total(src_tile, s),
            Store::EmbryoRet => a.embryo.as_ref().unwrap().ret[s] as u64,
            Store::Matrix(_) => w.parcels.get(&src_tile).map_or(0, |p| p.matrix[s] as u64),
            Store::Pore(_) => w.parcels[&src_tile].pore[s] as u64,
            Store::Actor(_) => self.st.agents[target.unwrap()].inv[s] as u64,
        };
        let inv_space = |stock: &[u16; NSPEC]| (INV_VOLUME.saturating_sub(volume(stock)) / VOL[s]) as u64;
        let space: u64 = match dst {
            Store::Inv => inv_space(&a.inv),
            Store::Loose(_) => w.loose_space(dst_tile, s),
            Store::Actor(_) => inv_space(&self.st.agents[target.unwrap()].inv),
            Store::EmbryoRet => inv_space(&a.embryo.as_ref().unwrap().ret),
            Store::Matrix(_) => 64 - w.parcels.get(&dst_tile).map_or(0, |p| p.volume()) as u64,
            Store::Pore(_) => {
                let p = &w.parcels[&dst_tile];
                (p.capacity().saturating_sub(p.pore_volume()) / VOL[s]) as u64
            }
        };
        let dz = w.elev[dst_tile as usize] as i64 - w.elev[src_tile as usize] as i64;
        let per_unit = 16 + (MASS[s] as i64 * dz).max(0) as u64;
        let by_energy = a.s.budget as u64 / per_unit;
        let mut n = (amount as u64).min(avail).min(space).min(by_energy);
        // parcel geometry: pore capacity after the change, and no actor under a blocking parcel
        let mut occupancy = false;
        if parcels && (matches!(src, Store::Matrix(_) | Store::Pore(_)) || matches!(dst, Store::Matrix(_) | Store::Pore(_))) {
            while n > 0 {
                match self.parcel_check(src, src_tile, dst, dst_tile, s, n) {
                    Ok(()) => break,
                    Err(Outcome::Blocked) => occupancy = true,
                    Err(_) => {}
                }
                n -= 1;
            }
        }
        if n == 0 {
            return if by_energy == 0 {
                (Outcome::Energy, 0)
            } else if avail == 0 {
                (Outcome::Matter, 0)
            } else if occupancy && space > 0 {
                (Outcome::Blocked, 0)
            } else {
                (Outcome::Capacity, 0)
            };
        }
        let src_parcel = self.st.world.parcels.get(&src_tile).map(|p| p.uid);
        // move matter
        match src {
            Store::Inv => self.st.agents[i].inv[s] -= n as u16,
            Store::Loose(_) => self.st.world.take_loose(src_tile, s, n),
            Store::EmbryoRet => self.st.agents[i].embryo.as_mut().unwrap().ret[s] -= n as u16,
            Store::Matrix(_) => self.st.world.parcels.get_mut(&src_tile).unwrap().matrix[s] -= n as u16,
            Store::Pore(_) => self.st.world.parcels.get_mut(&src_tile).unwrap().pore[s] -= n as u16,
            Store::Actor(_) => {
                self.st.agents[target.unwrap()].inv[s] -= n as u16;
                self.obs.m1.taken += n;
            }
        }
        match dst {
            Store::Inv => self.st.agents[i].inv[s] += n as u16,
            Store::Loose(_) => self.st.world.put_loose(dst_tile, s, n),
            Store::Actor(_) => self.st.agents[target.unwrap()].inv[s] += n as u16,
            Store::EmbryoRet => self.st.agents[i].embryo.as_mut().unwrap().ret[s] += n as u16,
            Store::Matrix(_) => {
                if !self.st.world.parcels.contains_key(&dst_tile) {
                    let uid = self.st.next_parcel_uid;
                    self.st.next_parcel_uid += 1;
                    self.st.world.parcels.insert(
                        dst_tile,
                        crate::world::Parcel::new(uid),
                    );
                    self.obs.m1.parcels_created += 1;
                }
                self.st.world.parcels.get_mut(&dst_tile).unwrap().matrix[s] += n as u16;
            }
            Store::Pore(_) => self.st.world.parcels.get_mut(&dst_tile).unwrap().pore[s] += n as u16,
        }
        for (store, t) in [(src, src_tile), (dst, dst_tile)] {
            if matches!(store, Store::Matrix(_) | Store::Pore(_)) {
                self.parcel_settle(t);
            }
        }
        self.move_heat(src_tile, dst_tile, n * CAP[s]);
        if self.obs.rel.is_some() {
            use crate::relations::{AGENT, PARCEL, RESOURCE, USE};
            let me = (AGENT, self.st.agents[i].uid);
            let mass = n * MASS[s] as u64;
            if let Some(j) = target {
                let other = (AGENT, self.st.agents[j].uid);
                let (from, to) = if let Store::Actor(_) = dst { (me, other) } else { (other, me) };
                self.rel(from, to, RESOURCE, s as u8, mass);
            }
            use crate::relations::act;
            if let (Store::Matrix(_) | Store::Pore(_), Some(p)) = (src, src_parcel) {
                let a = if let Store::Matrix(_) = src { act::MATRIX_OUT } else { act::PORE };
                self.rel(me, (PARCEL, p), USE, a, 1);
            }
            if let Store::Matrix(_) | Store::Pore(_) = dst {
                self.rel_use(i, dst_tile, if let Store::Matrix(_) = dst { act::MATRIX_IN } else { act::PORE });
            }
        }
        let me = self.tile_of(i);
        self.pay(i, (n * 16) as u32, me);
        let dug = MASS[s] as i64 * dz * n as i64;
        if dug > 0 {
            self.pay_ug(i, dug as u32);
        } else if dug < 0 {
            self.st.world.heat_u[dst_tile as usize] += (-dug) as u64;
        }
        (if n == amount as u64 { Outcome::Ok } else { Outcome::Partial }, n as u32)
    }

    fn transfer_energy(&mut self, i: usize, it: &Intent, src: (u16, u8), dst: (u16, u8), amount: u32) -> Res {
        if src != (dom::SELF, 0) {
            return (Outcome::InvalidTarget, 0);
        }
        let target = match dst {
            (dom::EMBRYO, 0) => {
                if self.st.agents[i].embryo.is_none() {
                    return (Outcome::InvalidTarget, 0);
                }
                None
            }
            (dom::ACTOR, o) if o > 0 => {
                let Some(t) = self.rel_tile(i, o) else { return (Outcome::Boundary, 0) };
                match (self.agent_at(t), it.target_uid) {
                    (Some(j), Some(uid)) if self.st.agents[j].uid == uid => {
                        if self.st.agents[j].port_mask & (1 << 15) == 0 {
                            return (Outcome::InvalidTarget, 0);
                        }
                        Some(j)
                    }
                    _ => return (Outcome::Stale, 0),
                }
            }
            _ => return (Outcome::InvalidTarget, 0),
        };
        if amount == 0 {
            return (Outcome::Noop, 0);
        }
        let held = match target {
            None => self.st.agents[i].embryo.as_ref().unwrap().energy,
            Some(j) => self.st.agents[j].energy,
        };
        let room = ENERGY_MAX - held;
        let budget = self.st.agents[i].s.budget;
        let cost = |q: u32| q + q.div_ceil(64);
        let mut by_energy = (budget as u64 * 64 / 65) as u32;
        while cost(by_energy + 1) <= budget {
            by_energy += 1;
        }
        while by_energy > 0 && cost(by_energy) > budget {
            by_energy -= 1;
        }
        let n = amount.min(room).min(by_energy);
        if n == 0 {
            return if by_energy == 0 { (Outcome::Energy, 0) } else { (Outcome::Capacity, 0) };
        }
        let me = self.tile_of(i);
        self.pay(i, n.div_ceil(64), me);
        self.pay_ug(i, n); // leaves the sender as available energy, not heat
        match target {
            None => self.st.agents[i].embryo.as_mut().unwrap().energy += n,
            Some(j) => {
                self.st.agents[j].energy += n;
                let (a, b) = (self.st.agents[i].uid, self.st.agents[j].uid);
                self.rel((0, a), (0, b), crate::relations::RESOURCE, 15, n as u64);
            }
        }
        (if n == amount { Outcome::Ok } else { Outcome::Partial }, n)
    }

    pub(crate) fn eff_rotate(&mut self, i: usize, it: &Intent) -> Res {
        match it.sub {
            sub::DRIVE_ROTATE => {
                let q = it.r[5];
                if q > 3 {
                    return (Outcome::InvalidTarget, 0);
                }
                if q == 0 {
                    return (Outcome::Noop, 0);
                }
                if self.st.agents[i].s.budget < 64 {
                    return (Outcome::Energy, 0);
                }
                let me = self.tile_of(i);
                self.pay(i, 64, me);
                let a = &mut self.st.agents[i];
                a.orient = (a.orient + q as u8) & 3;
                (Outcome::Ok, 1)
            }
            _ => (Outcome::InvalidTarget, 0), // Push: capability off in M0
        }
    }

    fn eff_write(&mut self, i: usize, it: &Intent) -> Res {
        let r = it.r;
        if self.caps.m1 {
            if matches!(it.sub, sub::WRITE_CELL | sub::WRITE_RESIZE) && matches!(r[0], dom::OWN_DRAFT | dom::PUBLIC_TAPE) {
                return self.eff_write_record(i, it);
            }
            if it.sub == 2 {
                return self.eff_write_record(i, it);
            }
            if it.sub == 3 {
                return self.cancel_draft(i);
            }
        }
        match it.sub {
            sub::WRITE_CELL => {
                if r[0] != dom::EMBRYO {
                    return (Outcome::InvalidTarget, 0);
                }
                self.embryo_write_cell(i, r[4] as u32, r[5])
            }
            sub::WRITE_RESIZE => {
                if r[0] != dom::EMBRYO {
                    return (Outcome::InvalidTarget, 0);
                }
                self.embryo_resize(i, r[5] as u32)
            }
            sub::WRITE_ORIENT => {
                let o = r[5];
                if o > 3 {
                    return (Outcome::InvalidTarget, 0);
                }
                if o as u8 == self.st.agents[i].orient {
                    return (Outcome::Noop, 0);
                }
                if self.st.agents[i].s.budget < 64 {
                    return (Outcome::Energy, 0);
                }
                let me = self.tile_of(i);
                self.pay(i, 64, me);
                self.st.agents[i].orient = o as u8;
                (Outcome::Ok, 1)
            }
            sub::WRITE_PORT => {
                if r[5] & 0x7C00 != 0 {
                    return (Outcome::InvalidTarget, 0);
                }
                self.st.agents[i].port_mask = r[5];
                (Outcome::Ok, 1)
            }
            _ => (Outcome::InvalidTarget, 0), // Commit/CancelDraft: capability off
        }
    }

    fn eff_work(&mut self, i: usize, it: &Intent) -> Res {
        let r = it.r;
        if self.caps.parcels && r[0] == dom::PARCEL {
            return self.eff_work_parcel(i, it);
        }
        if self.caps.contact && crate::contact::is_actor_abrade(it) {
            return self.eff_abrade_actor(i, it);
        }
        let req = r[5] as u32;
        match it.sub {
            sub::WORK_COMPRESS | sub::WORK_ABRADE => {
                if r[0] != dom::SURFACE {
                    return (Outcome::InvalidTarget, 0);
                }
                let t = match self.rel_vec_tile(i, r[1] as i16, r[2] as i16, 1) {
                    Ok(t) => t,
                    Err(e) => return (e, 0),
                };
                let compress = it.sub == sub::WORK_COMPRESS;
                let unit = if compress { 64 } else { 256 };
                if req < unit {
                    return (Outcome::Noop, 0);
                }
                let c = self.st.world.surface_c[t as usize] as u32;
                let room = if compress { 255 - c } else { c };
                let by_energy = self.st.agents[i].s.budget / unit;
                let n = (req / unit).min(room).min(by_energy);
                if n == 0 {
                    return if by_energy == 0 {
                        (Outcome::Energy, 0)
                    } else {
                        (Outcome::Capacity, 0)
                    };
                }
                self.pay(i, n * unit, t);
                let sc = &mut self.st.world.surface_c[t as usize];
                *sc = if compress { *sc + n as u8 } else { *sc - n as u8 };
                (if n == req / unit { Outcome::Ok } else { Outcome::Partial }, n * unit)
            }
            sub::WORK_HEAT => {
                let t = match r[0] {
                    dom::SELF => self.tile_of(i),
                    dom::LOOSE | dom::ACTOR | dom::SURFACE => {
                        match self.rel_vec_tile(i, r[1] as i16, r[2] as i16, 1) {
                            Ok(t) => t,
                            Err(e) => return (e, 0),
                        }
                    }
                    _ => return (Outcome::InvalidTarget, 0),
                };
                if req == 0 {
                    return (Outcome::Noop, 0);
                }
                let n = req.min(self.st.agents[i].s.budget);
                if n == 0 {
                    return (Outcome::Energy, 0);
                }
                self.pay(i, n, t);
                (if n == req { Outcome::Ok } else { Outcome::Partial }, n)
            }
            _ => (Outcome::InvalidTarget, 0),
        }
    }
}

impl Sim {
    /// Would moving n units of s between these stores leave every touched parcel valid?
    fn parcel_check(&self, src: Store, src_tile: u32, dst: Store, dst_tile: u32, s: usize, n: u64) -> Result<(), Outcome> {
        let mut touched: Vec<(u32, crate::world::Parcel)> = Vec::new();
        let get = |t: u32, touched: &mut Vec<(u32, crate::world::Parcel)>| -> usize {
            if let Some(k) = touched.iter().position(|x| x.0 == t) {
                return k;
            }
            let p = self.st.world.parcels.get(&t).cloned().unwrap_or(crate::world::Parcel::new(0));
            touched.push((t, p));
            touched.len() - 1
        };
        let n16 = n as u16;
        match src {
            Store::Matrix(_) => {
                let k = get(src_tile, &mut touched);
                touched[k].1.matrix[s] -= n16;
            }
            Store::Pore(_) => {
                let k = get(src_tile, &mut touched);
                touched[k].1.pore[s] -= n16;
            }
            _ => {}
        }
        match dst {
            Store::Matrix(_) => {
                let k = get(dst_tile, &mut touched);
                touched[k].1.matrix[s] += n16;
            }
            Store::Pore(_) => {
                let k = get(dst_tile, &mut touched);
                touched[k].1.pore[s] += n16;
            }
            _ => {}
        }
        for (t, p) in &touched {
            if p.volume() > 64 || p.pore_volume() > p.capacity() {
                return Err(Outcome::Capacity);
            }
            if p.blocks() && self.st.world.occ[*t as usize] != 0 {
                return Err(Outcome::Blocked);
            }
        }
        Ok(())
    }

    /// After a parcel changed: remove it when its matrix is empty, then refresh P and X.
    pub(crate) fn parcel_settle(&mut self, t: u32) {
        let w = &mut self.st.world;
        if w.parcels.get(&t).is_some_and(|p| p.volume() == 0) {
            let p = w.parcels.remove(&t).unwrap();
            for s in 0..NSPEC {
                w.force_loose(t, s, p.pore[s] as u64);
            }
            // its medium stays on the tile as a dropped record (same matter, same tile)
            if let Some(mut m) = p.media {
                m.revision += 1;
                self.obs.m1.parcel_media_dropped += 1;
                self.drop_media(t, m);
            }
        }
        self.clamp_bonds(t);
        self.st.world.parcel_changed(t);
    }
}
