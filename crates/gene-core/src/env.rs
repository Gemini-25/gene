//! Stage 7 transport and stage 8 environment. SPEC §5.

use crate::agent::INV_VOLUME;
use crate::rng::{permutation, sr, stream_key, Domain, Stream};
use crate::sim::Sim;
use crate::species::*;
use crate::world::P_OPEN;

const GASES: [usize; 4] = [C, O, H, V];

impl Sim {
    pub(crate) fn stage_transport(&mut self) {
        let tick = self.st.tick;
        let bucket = (tick % 8) as usize;
        let mut st = Stream::new(stream_key(self.st.config.seed, Domain::Resolve, 0, 7), tick);
        let perm = permutation(&mut st, 4);
        for color in perm {
            let slot = bucket * 4 + color;
            let list = std::mem::take(&mut self.st.world.edges[slot]);
            for &(a, b) in &list {
                self.edge_flux(a, b, tick);
            }
            self.st.world.edges[slot] = list;
        }
    }

    fn edge_flux(&mut self, a: u32, b: u32, tick: u64) {
        let object = (a as u64) << 32 | b as u64;
        let min_p = P_OPEN;
        for s in GASES {
            let (na, nb) = (self.st.world.loose_total(a, s), self.st.world.loose_total(b, s));
            if na == nb {
                continue;
            }
            let (src, dst, diff) = if na > nb { (a, b, na - nb) } else { (b, a, nb - na) };
            let cand = sr(diff * min_p, 2048, self.keys.edge[s], object, tick).min(32);
            self.flow(src, dst, s, cand);
        }
        let w = &self.st.world;
        let (ea, eb) = (w.elev[a as usize], w.elev[b as usize]);
        if ea != eb {
            let (src, dst) = if ea > eb { (a, b) } else { (b, a) };
            let cand = sr(32 * min_p, 256, self.keys.edge[W], object, tick).min(32);
            self.flow(src, dst, W, cand);
        } else {
            let (na, nb) = (w.loose_total(a, W), w.loose_total(b, W));
            if na != nb {
                let (src, dst, diff) = if na > nb { (a, b, na - nb) } else { (b, a, nb - na) };
                let cand = sr(diff * min_p, 8192, self.keys.edge[W], object, tick).min(32);
                self.flow(src, dst, W, cand);
            }
        }
        self.conduct(a, b);
    }

    /// Move up to `cand` units of species s from src to dst with heat and potential energy.
    fn flow(&mut self, src: u32, dst: u32, s: usize, cand: u64) {
        let w = &mut self.st.world;
        let cand = cand.min(w.loose_total(src, s)).min(w.loose_space(dst, s));
        if cand == 0 {
            return;
        }
        let dz = w.elev[dst as usize] as i64 - w.elev[src as usize] as i64;
        let (u, c) = (w.heat_u[src as usize], w.heat_c[src as usize]);
        let mut n = cand;
        while n > 0 {
            let c_move = n * CAP[s];
            let q = match u.checked_mul(c_move) {
                Some(p) => p / c,
                None => (u as u128 * c_move as u128 / c as u128) as u64,
            };
            let dug = n as i64 * MASS[s] as i64 * dz;
            if dug <= 0 || q as i64 >= dug {
                w.take_loose(src, s, n);
                w.put_loose(dst, s, n);
                w.heat_u[src as usize] -= q;
                w.heat_c[src as usize] -= c_move;
                w.heat_c[dst as usize] += c_move;
                w.heat_u[dst as usize] += (q as i64 - dug) as u64;
                return;
            }
            n -= 1;
        }
    }

    fn conduct(&mut self, a: u32, b: u32) {
        let w = &mut self.st.world;
        let (ua, ub) = (w.heat_u[a as usize], w.heat_u[b as usize]);
        let (ca, cb) = (w.heat_c[a as usize], w.heat_c[b as usize]);
        let (ta, tb) = (ua / ca, ub / cb);
        if ta == tb {
            return;
        }
        let (hot, cold, dt, ch, cc) = if ta > tb { (a, b, ta - tb, ca, cb) } else { (b, a, tb - ta, cb, ca) };
        let kappa = (ch.min(cc) * P_OPEN / 16384).max(1);
        let mut q = kappa * dt;
        // cap = floor(dt*ch*cc/(ch+cc)); divide only when the cap binds
        if q as u128 * (ch as u128 + cc as u128) > dt as u128 * ch as u128 * cc as u128 {
            q = (dt as u128 * ch as u128 * cc as u128 / (ch as u128 + cc as u128)) as u64;
        }
        let q = q.min(w.heat_u[hot as usize]);
        w.heat_u[hot as usize] -= q;
        w.heat_u[cold as usize] += q;
    }

    pub(crate) fn stage_environment(&mut self) {
        let tick = self.st.tick;
        let n = self.st.world.n() as u64;
        let mut t = tick % 16;
        while t < n {
            self.env_tile(t as u32, tick);
            t += 16;
        }
    }

    fn env_tile(&mut self, t: u32, tick: u64) {
        let ti = t as usize;
        let x = self.st.world.expo[ti] as u64;
        let occupant = self.agent_at(t);
        // 1. light
        let l = self.st.world.light(t, tick);
        let refl = l / 4;
        let absorbed = l - refl;
        self.st.flows.solar_in += l as u128;
        self.st.flows.radiation_out += refl as u128;
        let th = self.st.world.theta(t);
        match occupant {
            Some(i) if (260..=330).contains(&th) => {
                let a = &mut self.st.agents[i];
                let to_credit = absorbed * x * (a.body_g.min(16) as u64) / 4096;
                a.photo_credit += to_credit as u32;
                self.st.world.heat_u[ti] += absorbed - to_credit;
            }
            _ => self.st.world.heat_u[ti] += absorbed,
        }
        if let Some(i) = occupant {
            let a = &mut self.st.agents[i];
            if a.photo_credit > PHOTO_CREDIT_CAP {
                self.st.world.heat_u[ti] += (a.photo_credit - PHOTO_CREDIT_CAP) as u64;
                a.photo_credit = PHOTO_CREDIT_CAP;
            }
            let leak = a.photo_credit / 256;
            a.photo_credit -= leak;
            self.st.world.heat_u[ti] += leak as u64;
            // 2. Rx0 photosynthesis, up to 4
            for _ in 0..4 {
                let w = &self.st.world;
                let a = &self.st.agents[i];
                if !(260..=330).contains(&w.theta(t))
                    || a.photo_credit < PHOTO_COST
                    || w.loose_total(t, C) == 0
                    || w.loose_total(t, W) == 0
                    || w.loose_space(t, O) == 0
                    || a.inv_volume() + VOL[G] > INV_VOLUME
                {
                    break;
                }
                let w = &mut self.st.world;
                w.take_loose(t, C, 1);
                w.take_loose(t, W, 1);
                w.put_loose(t, O, 1);
                w.heat_u[ti] += PHOTO_HEAT;
                let a = &mut self.st.agents[i];
                a.photo_credit -= PHOTO_COST;
                a.inv[G] += 1;
                self.obs.tick.photosynthesis += 1;
            }
        }
        // 3. sky exchange toward 280
        {
            let w = &mut self.st.world;
            let k = (2048 * x).div_ceil(256);
            let (u, c) = (w.heat_u[ti], w.heat_c[ti]);
            let th = u / c;
            if th > 280 {
                let q = (u - 280 * c).min(k * (th - 280));
                w.heat_u[ti] -= q;
                self.st.flows.radiation_out += q as u128;
            } else if th < 280 {
                let q = (280 * c - u).min(k * (280 - th));
                w.heat_u[ti] += q;
                self.st.flows.sky_in += q as u128;
            }
        }
        // 4. external chemistry at the loose site
        self.loose_chemistry(t, x, tick);
        self.phase_changes(t);
        self.media_chemistry(t, x, tick);
        // 5. heat damage
        if let Some(i) = occupant {
            if self.st.world.theta(t) > 340 && self.st.agents[i].body_g > 0 {
                self.st.agents[i].body_g -= 1;
                self.st.world.force_loose(t, G, 1);
                self.obs.tick.heat_damage += 1;
            }
        }
    }

    fn rx_possible(&self, t: u32, rx: &Reaction) -> u64 {
        let w = &self.st.world;
        if rx.reducing && w.loose_total(t, O) > 1 {
            return 0;
        }
        if w.theta(t) < rx.min_theta {
            return 0;
        }
        let mut p = u64::MAX;
        for &(s, k) in rx.inputs {
            p = p.min(w.loose_total(t, s) / k as u64);
        }
        for &(s, k) in rx.outputs {
            p = p.min(w.loose_space(t, s) / k as u64);
        }
        if rx.heat < 0 {
            p = p.min(w.heat_u[t as usize] / (-rx.heat) as u64);
        }
        p
    }

    fn rx_apply(&mut self, t: u32, rx: &Reaction) {
        let w = &mut self.st.world;
        for &(s, k) in rx.inputs {
            w.take_loose(t, s, k as u64);
        }
        for &(s, k) in rx.outputs {
            w.put_loose(t, s, k as u64);
        }
        if rx.heat >= 0 {
            w.heat_u[t as usize] += rx.heat as u64;
        } else {
            w.heat_u[t as usize] -= (-rx.heat) as u64;
        }
    }

    fn loose_chemistry(&mut self, t: u32, x: u64, tick: u64) {
        for (k, rx) in CREDIT_REACTIONS.iter().enumerate() {
            let rate = rx.rates[temp_bin(self.st.world.theta(t))] as u64;
            if rate == 0 {
                continue;
            }
            let possible = self.rx_possible(t, rx);
            if possible == 0 {
                continue;
            }
            let inc = sr(rate * x * possible.min(16), 256, self.keys.loose_rx[k], t as u64, tick);
            let mut credit = self.st.world.rx_credit[t as usize][k] as u64 + inc;
            let mut ext = 0;
            while credit >= 65536 && ext < 4 && self.rx_possible(t, rx) > 0 {
                self.rx_apply(t, rx);
                credit -= 65536;
                ext += 1;
            }
            self.st.world.rx_credit[t as usize][k] = credit.min(65535) as u16;
        }
    }

    fn phase_changes(&mut self, t: u32) {
        let ti = t as usize;
        let w = &mut self.st.world;
        for _ in 0..4 {
            let th = w.theta(t);
            if th >= 374 && w.loose_total(t, W) > 0 && w.loose_space(t, V) > 0 && w.heat_u[ti] >= 440 {
                w.take_loose(t, W, 1);
                w.put_loose(t, V, 1);
                w.heat_u[ti] -= 440;
            } else if th <= 372 && w.loose_total(t, V) > 0 && w.loose_space(t, W) > 0 {
                w.take_loose(t, V, 1);
                w.put_loose(t, W, 1);
                w.heat_u[ti] += 440;
            } else {
                break;
            }
        }
        for _ in 0..4 {
            let th = w.theta(t);
            if th <= 272 && w.loose_total(t, W) > 0 && w.loose_space(t, I) > 0 {
                w.take_loose(t, W, 1);
                w.put_loose(t, I, 1);
                w.heat_u[ti] += 60;
            } else if th >= 274 && w.loose_total(t, I) > 0 && w.loose_space(t, W) > 0 && w.heat_u[ti] >= 60 {
                w.take_loose(t, I, 1);
                w.put_loose(t, W, 1);
                w.heat_u[ti] -= 60;
            } else {
                break;
            }
        }
    }

    fn media_chemistry(&mut self, t: u32, x: u64, tick: u64) {
        if self.st.dropped.is_empty() {
            return;
        }
        let Some(mut list) = self.st.dropped.remove(&t) else { return };
        let ti = t as usize;
        for m in &mut list {
            let w = &self.st.world;
            let rate = RX2.rates[temp_bin(w.theta(t))] as u64;
            let possible = |w: &crate::world::World, m: &crate::media::Media| {
                (m.present_pages() as u64)
                    .min(w.loose_total(t, O))
                    .min(w.loose_space(t, C))
                    .min(w.loose_space(t, W))
            };
            let p = possible(w, m);
            if rate == 0 || p == 0 {
                continue;
            }
            let inc = sr(rate * x * p.min(16), 256, self.keys.media_rx2, m.uid, tick);
            let mut credit = m.credit as u64 + inc;
            let mut ext = 0;
            while credit >= 65536 && ext < 4 && possible(&self.st.world, m) > 0 {
                m.oxidize_top_page();
                let w = &mut self.st.world;
                w.take_loose(t, O, 1);
                w.put_loose(t, C, 1);
                w.put_loose(t, W, 1);
                w.heat_u[ti] += 5000;
                credit -= 65536;
                ext += 1;
                self.obs.tick.media_oxidized += 1;
            }
            m.credit = credit.min(65535) as u16;
        }
        list.retain(|m| m.present_pages() > 0);
        if !list.is_empty() {
            self.st.dropped.insert(t, list);
        }
    }
}
