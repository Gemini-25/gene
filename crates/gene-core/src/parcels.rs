//! M1c parcels: WORK on parcels, natural pore exchange, matrix/pore/boundary chemistry (SPEC §20d).

use crate::effects::Res;
use crate::intent::{Intent, Outcome};
use crate::rng::sr;
use crate::sim::Sim;
use crate::species::*;
use crate::world::{Parcel, GAS, SOLID};

const POREX: [usize; 5] = [C, O, H, V, W];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Site {
    Matrix = 0,
    Pore = 1,
    Boundary = 2,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Comp {
    Loose,
    Matrix,
    Pore,
}

fn source(site: Site, s: usize) -> Comp {
    match site {
        Site::Matrix => {
            if GAS[s] {
                Comp::Loose
            } else {
                Comp::Matrix
            }
        }
        Site::Pore => Comp::Pore,
        Site::Boundary => {
            if GAS[s] {
                Comp::Loose
            } else {
                Comp::Pore
            }
        }
    }
}

fn dest(site: Site, s: usize) -> Comp {
    match site {
        Site::Matrix => {
            if SOLID[s] {
                Comp::Matrix
            } else {
                Comp::Loose
            }
        }
        Site::Pore => Comp::Pore,
        Site::Boundary => {
            if GAS[s] {
                Comp::Loose
            } else {
                Comp::Pore
            }
        }
    }
}

/// Does the reaction take place at this site at all?
fn applies(site: Site, rx: &Reaction) -> bool {
    match site {
        Site::Matrix => rx.inputs.iter().any(|&(s, _)| source(site, s) == Comp::Matrix),
        Site::Pore => true,
        Site::Boundary => {
            matches!(rx.id, 2 | 5 | 6)
                && rx.inputs.iter().any(|&(s, _)| source(site, s) == Comp::Pore)
                && rx.inputs.iter().any(|&(s, _)| source(site, s) == Comp::Loose)
        }
    }
}

/// Local copy of everything one site reaction touches on a tile.
#[derive(Clone)]
struct Scene {
    loose: [u64; NSPEC],
    parcel: Parcel,
    u: u64,
    occupied: bool,
}

impl Scene {
    fn get(&self, c: Comp, s: usize) -> u64 {
        match c {
            Comp::Loose => self.loose[s],
            Comp::Matrix => self.parcel.matrix[s] as u64,
            Comp::Pore => self.parcel.pore[s] as u64,
        }
    }

    /// Apply one extent if every bound holds afterwards.
    fn step(&mut self, site: Site, rx: &Reaction, theta: u64) -> bool {
        if theta < rx.min_theta {
            return false;
        }
        let o_src = if site == Site::Pore { Comp::Pore } else { Comp::Loose };
        if rx.reducing && self.get(o_src, O) > 1 {
            return false;
        }
        if rx.heat < 0 && self.u < (-rx.heat) as u64 {
            return false;
        }
        let mut next = self.clone();
        for &(s, k) in rx.inputs {
            let c = source(site, s);
            if next.get(c, s) < k as u64 {
                return false;
            }
            match c {
                Comp::Loose => next.loose[s] -= k as u64,
                Comp::Matrix => next.parcel.matrix[s] -= k,
                Comp::Pore => next.parcel.pore[s] -= k,
            }
        }
        for &(s, k) in rx.outputs {
            match dest(site, s) {
                Comp::Loose => next.loose[s] += k as u64,
                Comp::Matrix => next.parcel.matrix[s] += k,
                Comp::Pore => next.parcel.pore[s] += k,
            }
        }
        if next.loose.iter().any(|v| *v > u16::MAX as u64) {
            return false;
        }
        let p = &next.parcel;
        if p.volume() > 64 || p.pore_volume() > p.capacity() || (p.blocks() && next.occupied) {
            return false;
        }
        next.u = if rx.heat >= 0 { next.u + rx.heat as u64 } else { next.u - (-rx.heat) as u64 };
        *self = next;
        true
    }
}

impl Sim {
    /// WORK Compress/Abrade/Heat on a parcel (domain 2).
    pub(crate) fn eff_work_parcel(&mut self, i: usize, it: &Intent) -> Res {
        let r = it.r;
        let t = match self.rel_vec_tile(i, r[1] as i16, r[2] as i16, 1) {
            Ok(t) => t,
            Err(e) => return (e, 0),
        };
        let Some(p) = self.st.world.parcels.get(&t) else { return (Outcome::InvalidTarget, 0) };
        if it.parcel_uid != Some(p.uid) {
            return (Outcome::Stale, 0);
        }
        let req = r[5] as u32;
        if it.sub == crate::intent::sub::WORK_HEAT {
            if req == 0 {
                return (Outcome::Noop, 0);
            }
            let n = req.min(self.st.agents[i].s.budget);
            if n == 0 {
                return (Outcome::Energy, 0);
            }
            self.pay(i, n, t);
            self.rel_use(i, t);
            return (if n == req { Outcome::Ok } else { Outcome::Partial }, n);
        }
        let compress = it.sub == crate::intent::sub::WORK_COMPRESS;
        let unit = if compress { 64 } else { 256 };
        if req < unit {
            return (Outcome::Noop, 0);
        }
        let by_energy = self.st.agents[i].s.budget / unit;
        let room = if compress { 255 - p.c as u32 } else { p.c as u32 };
        let mut n = (req / unit).min(room).min(by_energy);
        let occupied = self.st.world.occ[t as usize] != 0;
        let mut why = Outcome::Capacity;
        if compress {
            while n > 0 {
                let c = p.c + n as u8;
                let k = Parcel::capacity_for(p.volume(), c);
                let blocks = p.volume() >= 16 && c >= 128;
                if p.pore_volume() <= k && !(blocks && occupied) {
                    break;
                }
                if blocks && occupied {
                    why = Outcome::Blocked;
                }
                n -= 1;
            }
        }
        if n == 0 {
            return if by_energy == 0 { (Outcome::Energy, 0) } else { (why, 0) };
        }
        self.pay(i, n * unit, t);
        let p = self.st.world.parcels.get_mut(&t).unwrap();
        p.c = if compress { p.c + n as u8 } else { p.c - n as u8 };
        self.st.world.parcel_changed(t);
        self.rel_use(i, t);
        (if n == req / unit { Outcome::Ok } else { Outcome::Partial }, n * unit)
    }

    /// Stage 7 (before edges): natural exchange between due pores and the tile's loose stock.
    pub(crate) fn pore_exchange(&mut self) {
        let tick = self.st.tick;
        let due: Vec<u32> = self.st.world.parcels.keys().copied().filter(|t| (*t as u64) % 16 == tick % 16).collect();
        for t in due {
            let w = &mut self.st.world;
            let p = w.parcels.get(&t).unwrap();
            let (uid, perm) = (p.uid, p.perm() as u64);
            for s in POREX {
                let p = &w.parcels[&t];
                let k = p.capacity() as i64;
                let d = w.loose_total(t, s) as i64 * k - p.pore[s] as i64 * 64;
                if d == 0 {
                    continue;
                }
                let denom = if s == W { 64 * 8192 } else { 64 * 2048 };
                let mut n = sr(d.unsigned_abs() * perm, denom, self.keys.pore[s], uid, tick).min(32);
                if d > 0 {
                    let free = (p.capacity().saturating_sub(p.pore_volume()) / VOL[s]) as u64;
                    n = n.min(w.loose_total(t, s)).min(free);
                    if n > 0 {
                        w.take_loose(t, s, n);
                        w.parcels.get_mut(&t).unwrap().pore[s] += n as u16;
                    }
                } else {
                    n = n.min(p.pore[s] as u64).min(w.loose_space(t, s));
                    if n > 0 {
                        w.parcels.get_mut(&t).unwrap().pore[s] -= n as u16;
                        w.put_loose(t, s, n);
                    }
                }
            }
        }
    }

    /// Stage 8: Matrix, Pore and PoreBoundary sites of the parcel on tile t.
    pub(crate) fn parcel_chemistry(&mut self, t: u32, x: u64, tick: u64) {
        if !self.st.world.parcels.contains_key(&t) {
            return;
        }
        for site in [Site::Matrix, Site::Pore, Site::Boundary] {
            for (k, rx) in CREDIT_REACTIONS.iter().enumerate() {
                if !applies(site, rx) {
                    continue;
                }
                let theta = self.st.world.theta(t);
                let rate = rx.rates[temp_bin(theta)] as u64;
                if rate == 0 {
                    continue;
                }
                let Some(p) = self.st.world.parcels.get(&t) else { return };
                let mut scene = Scene {
                    loose: self.st.world.loose_stock_u64(t),
                    parcel: p.clone(),
                    u: self.st.world.heat_u[t as usize],
                    occupied: self.st.world.occ[t as usize] != 0,
                };
                let mut probe = scene.clone();
                let mut possible = 0;
                while possible < 16 && probe.step(site, rx, theta) {
                    possible += 1;
                }
                if possible == 0 {
                    continue;
                }
                let key = self.keys.parcel_rx[site as usize][k];
                let inc = if site == Site::Boundary {
                    sr(rate * x * p.perm() as u64 * possible, 65536, key, p.uid, tick)
                } else {
                    sr(rate * x * possible, 256, key, p.uid, tick)
                };
                let mut credit = p.credit[site as usize][k] as u64 + inc;
                let mut ext = 0;
                while credit >= 65536 && ext < 4 && scene.step(site, rx, theta) {
                    credit -= 65536;
                    ext += 1;
                }
                // write the scene back (same tile: heat capacity and Ug are unchanged)
                let w = &mut self.st.world;
                for s in 0..NSPEC {
                    let now = w.loose_total(t, s);
                    if scene.loose[s] > now {
                        w.put_loose(t, s, scene.loose[s] - now);
                    } else if scene.loose[s] < now {
                        w.take_loose(t, s, now - scene.loose[s]);
                    }
                }
                w.heat_u[t as usize] = scene.u;
                scene.parcel.credit[site as usize][k] = credit.min(65535) as u16;
                w.parcels.insert(t, scene.parcel);
                if ext > 0 {
                    self.obs.m1.parcel_reactions += ext;
                    self.parcel_settle(t);
                    if !self.st.world.parcels.contains_key(&t) {
                        return;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::{Config, RULESET_M1C};

    fn world_with_parcel(matrix: [u16; NSPEC]) -> (Sim, u32) {
        let prog = crate::isa::assemble(".module 0\nspin: YIELD\n JUMP spin\n").unwrap();
        let mut s = Sim::with_program_ruleset(Config { seed: 2, width: 64, height: 64, agents: 1 }, &prog, RULESET_M1C).unwrap();
        let t = s.st.world.idx(40, 40);
        s.st.world.parcels.insert(t, Parcel { matrix, ..Parcel::new(77) });
        s.st.world.parcel_changed(t);
        s.fixture_rebase();
        (s, t)
    }

    #[test]
    fn matrix_oxidation_sends_gases_to_the_loose_stock() {
        let mut m = [0; NSPEC];
        m[G] = 8;
        m[S] = 8;
        let (mut s, t) = world_with_parcel(m);
        // a full credit: the next due tick at 270..329 K runs at least one extent
        s.st.world.parcels.get_mut(&t).unwrap().credit[Site::Matrix as usize][0] = 65535;
        let (c0, w0) = (s.st.world.loose_total(t, C), s.st.world.loose_total(t, W));
        let x = s.st.world.expo[t as usize] as u64;
        let tick = s.st.tick;
        s.parcel_chemistry(t, x, tick);
        let p = &s.st.world.parcels[&t];
        assert_eq!(p.matrix[G], 7, "one G burned inside the matrix");
        assert_eq!(s.st.world.loose_total(t, C), c0 + 1);
        assert_eq!(s.st.world.loose_total(t, W), w0 + 1, "liquid product leaves the matrix");
        assert!(s.audit_ok());
    }

    #[test]
    fn burning_the_last_solid_removes_the_parcel() {
        let mut m = [0; NSPEC];
        m[G] = 1;
        let (mut s, t) = world_with_parcel(m);
        s.st.world.parcels.get_mut(&t).unwrap().credit[Site::Matrix as usize][0] = 65535;
        let x = s.st.world.expo[t as usize] as u64;
        let tick = s.st.tick;
        s.parcel_chemistry(t, x, tick);
        assert!(!s.st.world.parcels.contains_key(&t));
        assert_eq!(s.st.world.perm[t as usize], 256);
        assert!(s.audit_ok());
    }
}
