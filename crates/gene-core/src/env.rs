//! Stage 7 transport and stage 8 environment. SPEC §5.

use crate::agent::INV_VOLUME;
use crate::rng::{permutation, sr, stream_key, Domain, Stream};
use crate::sim::Sim;
use crate::species::*;

const GASES: [usize; 4] = [C, O, H, V];
/// Water a tile keeps against running downhill when the ruleset has `retain` (SPEC §20i).
pub const SOIL_RETAIN: u64 = 8;

impl Sim {
    pub(crate) fn stage_transport(&mut self) {
        let tick = self.st.tick;
        let bucket = (tick % 8) as usize;
        let mut st = Stream::new(stream_key(self.st.config.seed, Domain::Resolve, 0, 7), tick);
        let perm = permutation(&mut st, 4);
        if self.caps.parcels && !self.st.world.parcels.is_empty() {
            self.pore_exchange();
        }
        let keys = self.keys.edge;
        let retain = if self.caps.retain { SOIL_RETAIN } else { 0 };
        let threads = self.threads.max(1);
        if threads > 1 && self.pool.is_none() {
            self.pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().ok();
        }
        let w = &mut self.st.world;
        // tiles holding residue, as a dense mask so that the others skip the map lookup
        // (transport only takes residue, never adds it, so the mask stays a superset)
        let marked: Vec<u32> = w.residue.keys().copied().collect();
        let rmask = &mut self.env_mask;
        if !marked.is_empty() {
            rmask.resize(w.n(), 0);
            for t in &marked {
                rmask[*t as usize] = 1;
            }
        }
        for color in perm {
            let slot = bucket * 4 + color;
            // Edges of one colour never share an endpoint (SPEC §5.2), so any order — and any
            // split into row bands that keeps each edge inside one band — gives the same result.
            if let (true, Some(pool)) = (threads > 1 && w.residue.is_empty() && w.h as usize >= 8 * threads, &self.pool) {
                transport_parallel(pool, w, slot, color, threads, &keys, retain, tick);
            } else {
                let list = std::mem::take(&mut w.edges[slot]);
                let plain = w.residue.is_empty();
                let mut band = Band {
                    base: 0,
                    loose: &mut w.loose,
                    heat_u: &mut w.heat_u,
                    heat_c: &mut w.heat_c,
                    residue: Some(&mut w.residue),
                    plain,
                    rmask: if plain { &[] } else { rmask.as_slice() },
                };
                for &(a, b) in &list {
                    edge_flux(&mut band, &w.elev, &w.perm, &w.metal, &keys, retain, a, b, tick);
                }
                w.edges[slot] = list;
            }
        }
        for t in &marked {
            rmask[*t as usize] = 0;
        }
    }

    pub(crate) fn stage_environment(&mut self) {
        let tick = self.st.tick;
        let threads = self.threads.max(1);
        let parallel = threads > 1 && self.st.world.h as usize >= 8 * threads;
        if parallel && self.pool.is_none() {
            self.pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().ok();
        }
        // Due tiles are independent (SPEC §5.1). Tiles with an actor, records, a parcel or
        // overflow residue go through the full path; the rest take the simple path with the same
        // arithmetic (in parallel row bands when there are threads), which skips the map lookups.
        let bands = if parallel && self.pool.is_some() { threads } else { 1 };
        let complex = self.env_simple_bands(tick, bands);
        for t in complex {
            self.env_tile(t, tick);
        }
    }

    /// Process due tiles without an actor, records, a parcel or residue in `threads` row bands
    /// (in parallel when more than one); return the others in due order per band.
    fn env_simple_bands(&mut self, tick: u64, threads: usize) -> Vec<u32> {
        let w = &mut self.st.world;
        let width = w.w;
        let rows = (w.h as usize).div_ceil(threads) as u32;
        let keys = self.keys.loose_rx;
        let dropped = &self.st.dropped;
        let parcels = &w.parcels;
        let occ = &w.occ;
        let (lpeak, expo, heat_c) = (&w.lpeak, &w.expo, &w.heat_c);
        let mut heat_u: &mut [u64] = &mut w.heat_u;
        let mut loose: &mut [[u16; NSPEC]] = &mut w.loose;
        let mut credit: &mut [[u16; 5]] = &mut w.rx_credit;
        let mut bands = Vec::new();
        let mut y0 = 0;
        while y0 < w.h {
            let y1 = (y0 + rows).min(w.h);
            let len = ((y1 - y0) * width) as usize;
            let (u, ur) = std::mem::take(&mut heat_u).split_at_mut(len);
            let (l, lr) = std::mem::take(&mut loose).split_at_mut(len);
            let (c, cr) = std::mem::take(&mut credit).split_at_mut(len);
            heat_u = ur;
            loose = lr;
            credit = cr;
            bands.push(EnvBand {
                base: y0 * width,
                heat_u: u,
                loose: l,
                credit: c,
                heat_c,
                flows: crate::sim::Flows::default(),
                complex: vec![],
            });
            y0 = y1;
        }
        // tiles with records, a parcel or residue, as a dense mask (map lookups per tile cost
        // more than the work). Under locality only whole 64-tile runs are due, so only their
        // ranges of the maps are visited, not every key.
        let residue = &w.residue;
        let (locality, decomp) = (self.caps.locality, self.caps.decomp);
        let mut marked: Vec<u32> = Vec::new();
        if locality {
            let seg = crate::world::SEGMENT;
            let phase = (tick % 16) as u32;
            for s in 0..(occ.len() as u32).div_ceil(seg) {
                if crate::world::seg_hash(s) % 16 == phase {
                    let r = s * seg..(s + 1) * seg;
                    marked.extend(dropped.range(r.clone()).map(|(t, _)| *t));
                    marked.extend(parcels.range(r.clone()).map(|(t, _)| *t));
                    marked.extend(residue.range(r).map(|(t, _)| *t));
                }
            }
        } else {
            marked.extend(dropped.keys().chain(parcels.keys()).chain(residue.keys()).copied());
        }
        let mask = &mut self.env_mask;
        mask.resize(occ.len(), 0);
        for t in &marked {
            mask[*t as usize] = 1;
        }
        let mask = &*mask;
        let run = |band: &mut EnvBand| {
            let end = band.base + band.heat_u.len() as u32;
            crate::world::for_due_tiles(band.base, end, tick, locality, |t| {
                if occ[t as usize] != 0 || mask[t as usize] != 0 {
                    band.complex.push(t);
                } else {
                    env_simple(band, t, tick, lpeak[t as usize] as u64, expo[t as usize] as u64, &keys, decomp);
                }
            });
        };
        if bands.len() == 1 {
            run(&mut bands[0]);
        } else {
            let pool = self.pool.as_ref().expect("pool");
            pool.scope(|sc| {
                for band in bands.iter_mut() {
                    sc.spawn(move |_| run(band));
                }
            });
        }
        for t in marked {
            self.env_mask[t as usize] = 0;
        }
        let mut complex = Vec::new();
        for b in bands {
            self.st.flows.solar_in += b.flows.solar_in;
            self.st.flows.sky_in += b.flows.sky_in;
            self.st.flows.radiation_out += b.flows.radiation_out;
            complex.extend(b.complex);
        }
        complex
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
        if self.caps.parcels {
            self.parcel_chemistry(t, x, tick);
        }
        self.phase_changes(t);
        self.media_chemistry(t, x, tick);
        if self.caps.m1 {
            self.media_wear(t, x);
        }
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
        rx_possible_at(&w.loose_stock_u64(t), w.heat_u[t as usize], w.theta(t), rx)
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
        // theta only changes when a reaction runs; recompute it then (same values as before)
        let mut theta = self.st.world.theta(t);
        for (k, rx) in CREDIT_REACTIONS.iter().enumerate() {
            let rate = rx_rate(rx, theta, self.caps.decomp);
            if rate == 0 {
                continue;
            }
            let possible = {
                let w = &self.st.world;
                rx_possible_at(&w.loose_stock_u64(t), w.heat_u[t as usize], theta, rx)
            };
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
                theta = self.st.world.theta(t);
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

    /// Exposure bit wear of records lying on tile t (SPEC §20b), then of the medium attached to
    /// the parcel there (M1f: it is just as exposed).
    fn media_wear(&mut self, t: u32, x: u64) {
        let seed = self.st.config.seed;
        let tick = self.st.tick;
        let mut hits: Vec<(u64, u32)> = Vec::new();
        if let Some(list) = self.st.dropped.get_mut(&t) {
            for m in list.iter_mut() {
                if let Some(c) = wear_one(m, x, seed, tick) {
                    hits.push((m.uid, c));
                }
            }
        }
        if let Some(m) = self.st.world.parcels.get_mut(&t).and_then(|p| p.media.as_mut()) {
            if let Some(c) = wear_one(m, x, seed, tick) {
                hits.push((m.uid, c));
            }
        }
        self.obs.m1.bit_flips += hits.len() as u64;
        if let Some(v) = &mut self.obs.lineage {
            for (media, cell) in hits {
                v.push(crate::lineage::MediaFact::Damage { tick, media, cell });
            }
        }
    }

    /// Rx2 oxidation of exposed records on tile t: dropped ones in UID order, then the parcel's.
    /// A record without present pages is gone (its G already left as C and W).
    fn media_chemistry(&mut self, t: u32, x: u64, tick: u64) {
        if let Some(mut list) = self.st.dropped.remove(&t) {
            for m in &mut list {
                self.oxidize_one(t, m, x, tick);
            }
            list.retain(|m| m.present_pages() > 0);
            if !list.is_empty() {
                self.st.dropped.insert(t, list);
            }
        }
        let Some(mut m) = self.st.world.parcels.get_mut(&t).and_then(|p| p.media.take()) else { return };
        self.oxidize_one(t, &mut m, x, tick);
        if m.present_pages() > 0 {
            self.st.world.parcels.get_mut(&t).unwrap().media = Some(m);
        }
    }

    fn oxidize_one(&mut self, t: u32, m: &mut crate::media::Media, x: u64, tick: u64) {
        let ti = t as usize;
        let w = &self.st.world;
        let rate = rx_rate(&RX2, w.theta(t), self.caps.decomp);
        let possible = |w: &crate::world::World, m: &crate::media::Media| {
            (m.present_pages() as u64)
                .min(w.loose_total(t, O))
                .min(w.loose_space(t, C))
                .min(w.loose_space(t, W))
        };
        let p = possible(w, m);
        if rate == 0 || p == 0 {
            return;
        }
        let inc = sr(rate * x * p.min(16), 256, self.keys.media_rx2, m.uid, tick);
        let mut credit = m.credit as u64 + inc;
        let mut ext = 0;
        while credit >= 65536 && ext < 4 && possible(&self.st.world, m) > 0 {
            if let (Some(v), Some(page)) = (&mut self.obs.lineage, m.present.iter().rposition(|p| *p)) {
                let start = (page * crate::genome::PAGE_CELLS) as u32;
                v.push(crate::lineage::MediaFact::Destroy { tick, media: m.uid, start, end: start + crate::genome::PAGE_CELLS as u32 });
            }
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
}

/// Mutable view of a contiguous run of tiles (a band of rows) for transport.
struct Band<'a> {
    base: u32,
    loose: &'a mut [[u16; NSPEC]],
    heat_u: &'a mut [u64],
    heat_c: &'a mut [u64],
    /// Overflow store; only the serial path may touch it (parallel requires it empty).
    residue: Option<&'a mut std::collections::BTreeMap<u32, [u64; NSPEC]>>,
    /// No overflow anywhere when the stage started: totals are the loose stock alone. Transport
    /// only moves loose stock within u16 bounds, so this cannot change during the stage.
    plain: bool,
    /// Otherwise, 1 for the tiles that held residue when the stage started (global index).
    rmask: &'a [u8],
}

impl Band<'_> {
    #[inline]
    fn total(&self, t: u32, s: usize) -> u64 {
        let base = self.loose[(t - self.base) as usize][s] as u64;
        if self.plain || self.rmask[t as usize] == 0 {
            return base;
        }
        match &self.residue {
            Some(r) if !r.is_empty() => base + r.get(&t).map_or(0, |x| x[s]),
            _ => base,
        }
    }

    #[inline]
    fn take(&mut self, t: u32, s: usize, n: u64) {
        let l = &mut self.loose[(t - self.base) as usize][s];
        let from_loose = (*l as u64).min(n);
        *l -= from_loose as u16;
        let rest = n - from_loose;
        if rest > 0 {
            let r = self.residue.as_mut().expect("residue only on the serial path");
            let e = r.get_mut(&t).expect("residue available");
            e[s] -= rest;
            if e.iter().all(|v| *v == 0) {
                r.remove(&t);
            }
        }
    }

    #[inline]
    fn put(&mut self, t: u32, s: usize, n: u64) {
        let l = &mut self.loose[(t - self.base) as usize][s];
        assert!(*l as u64 + n <= u16::MAX as u64, "loose overflow");
        *l += n as u16;
    }

    #[inline]
    fn u(&mut self, t: u32) -> &mut u64 {
        &mut self.heat_u[(t - self.base) as usize]
    }

    #[inline]
    fn c(&mut self, t: u32) -> &mut u64 {
        &mut self.heat_c[(t - self.base) as usize]
    }
}

/// How many extents of rx the stock, the free space and the heat allow (0 when blocked).
fn rx_possible_at(stock: &[u64; NSPEC], u: u64, theta: u64, rx: &Reaction) -> u64 {
    if rx.reducing && stock[O] > 1 {
        return 0;
    }
    if theta < rx.min_theta {
        return 0;
    }
    let mut p = u64::MAX;
    for &(s, k) in rx.inputs {
        p = p.min(stock[s] / k as u64);
        if p == 0 {
            return 0;
        }
    }
    for &(s, k) in rx.outputs {
        p = p.min((u16::MAX as u64).saturating_sub(stock[s]) / k as u64);
    }
    if rx.heat < 0 {
        p = p.min(u / (-rx.heat) as u64);
    }
    p
}

#[allow(clippy::too_many_arguments)]
fn edge_flux(
    band: &mut Band,
    elev: &[i16],
    perm: &[u16],
    metal: &[u8],
    keys: &[[u32; 2]; NSPEC],
    retain: u64,
    a: u32,
    b: u32,
    tick: u64,
) {
    let object = (a as u64) << 32 | b as u64;
    let min_p = perm[a as usize].min(perm[b as usize]) as u64;
    for s in GASES {
        let (na, nb) = (band.total(a, s), band.total(b, s));
        if na == nb {
            continue;
        }
        let (src, dst, diff) = if na > nb { (a, b, na - nb) } else { (b, a, nb - na) };
        let cand = sr(diff * min_p, 2048, keys[s], object, tick).min(32);
        flow(band, elev, src, dst, s, cand);
    }
    let (ea, eb) = (elev[a as usize], elev[b as usize]);
    if ea != eb {
        let (src, dst) = if ea > eb { (a, b) } else { (b, a) };
        // a dry upper tile (or one holding only what the soil keeps) moves nothing; the draw is
        // counter-based, so skipping it changes nothing
        let have = band.total(src, W);
        if have > retain {
            let cand = sr(32 * min_p, 256, keys[W], object, tick).min(32).min(have - retain);
            flow(band, elev, src, dst, W, cand);
        }
    } else {
        let (na, nb) = (band.total(a, W), band.total(b, W));
        if na != nb {
            let (src, dst, diff) = if na > nb { (a, b, na - nb) } else { (b, a, nb - na) };
            let cand = sr(diff * min_p, 8192, keys[W], object, tick).min(32);
            flow(band, elev, src, dst, W, cand);
        }
    }
    let m = (metal[a as usize] + metal[b as usize]).min(8) as u64;
    conduct(band, a, b, min_p, m);
}

/// Move up to `cand` units of species s from src to dst with heat and potential energy.
fn flow(band: &mut Band, elev: &[i16], src: u32, dst: u32, s: usize, cand: u64) {
    let space = (u16::MAX as u64).saturating_sub(band.total(dst, s));
    let cand = cand.min(band.total(src, s)).min(space);
    if cand == 0 {
        return;
    }
    let dz = elev[dst as usize] as i64 - elev[src as usize] as i64;
    let (u, c) = (*band.u(src), *band.c(src));
    let mut n = cand;
    while n > 0 {
        let c_move = n * CAP[s];
        let q = match u.checked_mul(c_move) {
            Some(p) => p / c,
            None => (u as u128 * c_move as u128 / c as u128) as u64,
        };
        let dug = n as i64 * MASS[s] as i64 * dz;
        if dug <= 0 || q as i64 >= dug {
            band.take(src, s, n);
            band.put(dst, s, n);
            *band.u(src) -= q;
            *band.c(src) -= c_move;
            *band.c(dst) += c_move;
            *band.u(dst) += (q as i64 - dug) as u64;
            return;
        }
        n -= 1;
    }
}

fn conduct(band: &mut Band, a: u32, b: u32, min_p: u64, metal: u64) {
    let (ua, ub) = (*band.u(a), *band.u(b));
    let (ca, cb) = (*band.c(a), *band.c(b));
    let (ta, tb) = (ua / ca, ub / cb);
    if ta == tb {
        return;
    }
    let (hot, cold, dt, ch, cc) = if ta > tb { (a, b, ta - tb, ca, cb) } else { (b, a, tb - ta, cb, ca) };
    let kappa = (ch.min(cc) * min_p / 16384).max(1) + metal;
    let mut q = kappa * dt;
    // cap = floor(dt*ch*cc/(ch+cc)); divide only when the cap binds
    if q as u128 * (ch as u128 + cc as u128) > dt as u128 * ch as u128 * cc as u128 {
        q = (dt as u128 * ch as u128 * cc as u128 / (ch as u128 + cc as u128)) as u64;
    }
    let q = q.min(*band.u(hot));
    *band.u(hot) -= q;
    *band.u(cold) += q;
}

/// Row bands whose boundaries keep every edge of `color` inside one band:
/// east edges stay in their row; south edges (y, y+1) start on even rows (colour 2) or odd rows (3).
fn band_rows(h: u32, color: usize, threads: usize) -> Vec<u32> {
    let mut rows = ((h as usize).div_ceil(threads) as u32 + 1) & !1; // even band height
    rows = rows.max(2);
    let offset = if color == 3 { 1 } else { 0 };
    let mut cuts = vec![0];
    let mut y = offset + rows;
    while y < h {
        cuts.push(y);
        y += rows;
    }
    cuts.push(h);
    cuts
}

fn transport_parallel(
    pool: &rayon::ThreadPool,
    w: &mut crate::world::World,
    slot: usize,
    color: usize,
    threads: usize,
    keys: &[[u32; 2]; NSPEC],
    retain: u64,
    tick: u64,
) {
    let width = w.w;
    let cuts = band_rows(w.h, color, threads);
    let list = std::mem::take(&mut w.edges[slot]);
    // edges are in row-major order of their first endpoint: split by band
    let mut parts: Vec<&[(u32, u32)]> = Vec::with_capacity(cuts.len() - 1);
    let mut rest: &[(u32, u32)] = &list;
    for k in 1..cuts.len() {
        let limit = cuts[k] * width;
        let n = rest.partition_point(|e| e.0 < limit);
        let (head, tail) = rest.split_at(n);
        parts.push(head);
        rest = tail;
    }
    let (elev, perm, metal) = (&w.elev, &w.perm, &w.metal);
    let mut loose: &mut [[u16; NSPEC]] = &mut w.loose;
    let mut heat_u: &mut [u64] = &mut w.heat_u;
    let mut heat_c: &mut [u64] = &mut w.heat_c;
    let mut bands = Vec::with_capacity(parts.len());
    for k in 1..cuts.len() {
        let len = ((cuts[k] - cuts[k - 1]) * width) as usize;
        let (l, lr) = std::mem::take(&mut loose).split_at_mut(len);
        let (u, ur) = std::mem::take(&mut heat_u).split_at_mut(len);
        let (c, cr) = std::mem::take(&mut heat_c).split_at_mut(len);
        loose = lr;
        heat_u = ur;
        heat_c = cr;
        bands.push(Band { base: cuts[k - 1] * width, loose: l, heat_u: u, heat_c: c, residue: None, plain: true, rmask: &[] });
    }
    pool.scope(|sc| {
        for (mut band, edges) in bands.into_iter().zip(parts) {
            sc.spawn(move |_| {
                for &(a, b) in edges {
                    edge_flux(&mut band, elev, perm, metal, keys, retain, a, b, tick);
                }
            });
        }
    });
    w.edges[slot] = list;
}

/// Mutable view of a band of tiles for the environment stage (actor-free, record-free tiles).
struct EnvBand<'a> {
    base: u32,
    heat_u: &'a mut [u64],
    loose: &'a mut [[u16; NSPEC]],
    credit: &'a mut [[u16; 5]],
    heat_c: &'a [u64],
    flows: crate::sim::Flows,
    complex: Vec<u32>,
}

impl EnvBand<'_> {
    #[inline]
    fn k(&self, t: u32) -> usize {
        (t - self.base) as usize
    }
    #[inline]
    fn theta(&self, t: u32) -> u64 {
        self.heat_u[self.k(t)] / self.heat_c[t as usize]
    }
    #[inline]
    fn total(&self, t: u32, s: usize) -> u64 {
        self.loose[self.k(t)][s] as u64
    }
    #[inline]
    fn space(&self, t: u32, s: usize) -> u64 {
        u16::MAX as u64 - self.total(t, s)
    }
    #[inline]
    fn add(&mut self, t: u32, s: usize, d: i64) {
        let k = self.k(t);
        self.loose[k][s] = (self.loose[k][s] as i64 + d) as u16;
    }
    #[inline]
    fn u(&mut self, t: u32) -> &mut u64 {
        let k = self.k(t);
        &mut self.heat_u[k]
    }
}

/// Stage 8 for a due tile with no actor and no records: light, sky exchange, loose chemistry,
/// phase changes. Same arithmetic as `Sim::env_tile` for such tiles.
#[allow(clippy::too_many_arguments)]
fn env_simple(b: &mut EnvBand, t: u32, tick: u64, lpeak: u64, x: u64, keys: &[[u32; 2]; 5], decomp: bool) {
    let p = ((tick + 1024) % 4096) as i64;
    let f = (1024 - (p - 1024).abs()).max(0) as u64;
    let l = lpeak * f / 1024;
    let refl = l / 4;
    b.flows.solar_in += l as u128;
    b.flows.radiation_out += refl as u128;
    *b.u(t) += l - refl;
    // sky exchange toward 280
    let k = (2048 * x).div_ceil(256);
    let (u, c) = (*b.u(t), b.heat_c[t as usize]);
    let th = u / c;
    if th > 280 {
        let q = (u - 280 * c).min(k * (th - 280));
        *b.u(t) -= q;
        b.flows.radiation_out += q as u128;
    } else if th < 280 {
        let q = (280 * c - u).min(k * (280 - th));
        *b.u(t) += q;
        b.flows.sky_in += q as u128;
    }
    // loose-site chemistry
    for (ki, rx) in CREDIT_REACTIONS.iter().enumerate() {
        let rate = rx_rate(rx, b.theta(t), decomp);
        if rate == 0 {
            continue;
        }
        let possible = band_rx_possible(b, t, rx);
        if possible == 0 {
            continue;
        }
        let inc = sr(rate * x * possible.min(16), 256, keys[ki], t as u64, tick);
        let kk = b.k(t);
        let mut credit = b.credit[kk][ki] as u64 + inc;
        let mut ext = 0;
        while credit >= 65536 && ext < 4 && band_rx_possible(b, t, rx) > 0 {
            for &(s, n) in rx.inputs {
                b.add(t, s, -(n as i64));
            }
            for &(s, n) in rx.outputs {
                b.add(t, s, n as i64);
            }
            if rx.heat >= 0 {
                *b.u(t) += rx.heat as u64;
            } else {
                *b.u(t) -= (-rx.heat) as u64;
            }
            credit -= 65536;
            ext += 1;
        }
        b.credit[kk][ki] = credit.min(65535) as u16;
    }
    // phase changes
    for _ in 0..4 {
        let th = b.theta(t);
        if th >= 374 && b.total(t, W) > 0 && b.space(t, V) > 0 && *b.u(t) >= 440 {
            b.add(t, W, -1);
            b.add(t, V, 1);
            *b.u(t) -= 440;
        } else if th <= 372 && b.total(t, V) > 0 && b.space(t, W) > 0 {
            b.add(t, V, -1);
            b.add(t, W, 1);
            *b.u(t) += 440;
        } else {
            break;
        }
    }
    for _ in 0..4 {
        let th = b.theta(t);
        if th <= 272 && b.total(t, W) > 0 && b.space(t, I) > 0 {
            b.add(t, W, -1);
            b.add(t, I, 1);
            *b.u(t) += 60;
        } else if th >= 274 && b.total(t, I) > 0 && b.space(t, W) > 0 && *b.u(t) >= 60 {
            b.add(t, I, -1);
            b.add(t, W, 1);
            *b.u(t) -= 60;
        } else {
            break;
        }
    }
}

fn band_rx_possible(b: &mut EnvBand, t: u32, rx: &Reaction) -> u64 {
    if rx.reducing && b.total(t, O) > 1 {
        return 0;
    }
    if b.theta(t) < rx.min_theta {
        return 0;
    }
    let mut p = u64::MAX;
    for &(s, k) in rx.inputs {
        p = p.min(b.total(t, s) / k as u64);
    }
    for &(s, k) in rx.outputs {
        p = p.min(b.space(t, s) / k as u64);
    }
    if rx.heat < 0 {
        p = p.min(*b.u(t) / (-rx.heat) as u64);
    }
    p
}

/// One exposure due of a record: the clock advances (4 on exposed ground, else 1) and every
/// 16384 flips one uniformly drawn bit of a present page. Returns the cell that flipped.
fn wear_one(m: &mut crate::media::Media, x: u64, seed: u64, tick: u64) -> Option<u32> {
    let clock = m.exposure_clock as u32 + if x >= 64 { 4 } else { 1 };
    if clock < 16384 {
        m.exposure_clock = clock as u16;
        return None;
    }
    m.exposure_clock = (clock - 16384) as u16;
    let present: Vec<usize> = (0..m.slots()).filter(|p| m.present[*p]).collect();
    if present.is_empty() {
        return None;
    }
    let key = crate::rng::stream_key(seed, Domain::Environment, m.uid, (2u64 << 56) | m.wear_events);
    let mut st = Stream::new(key, tick);
    let page = present[st.below(present.len() as u32) as usize];
    let cell = st.below(128) as usize;
    let bit = st.below(16);
    m.cells[page * 128 + cell] ^= 1 << bit;
    m.wear_events += 1;
    m.revision += 1;
    if m.recycle.is_some_and(|(p, _)| p as usize == page) {
        m.recycle = Some((page as u16, 0));
    }
    Some((page * 128 + cell) as u32)
}
