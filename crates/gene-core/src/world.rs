//! Tiles, terrain generation and spatial helpers. SPEC §3.

use crate::rng::{self, Domain, Stream};
use crate::species::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const BASE_HEAT_CAP: u64 = 1024;
pub const P_OPEN: u64 = 256;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct World {
    pub w: u32,
    pub h: u32,
    pub elev: Vec<i16>,
    pub mu: Vec<u16>,
    pub lpeak: Vec<u16>,
    pub substrate: Vec<u8>,
    pub surface_c: Vec<u8>,
    pub loose: Vec<[u16; NSPEC]>,
    pub residue: BTreeMap<u32, [u64; NSPEC]>,
    pub heat_u: Vec<u64>,
    /// Reaction credits for Rx2..Rx6 at the loose site.
    pub rx_credit: Vec<[u16; 5]>,
    /// Derived: heat capacity C per tile.
    #[serde(skip)]
    pub heat_c: Vec<u64>,
    /// Derived: exposure X per tile.
    #[serde(skip)]
    pub expo: Vec<u16>,
    /// Derived: occupant agent index + 1 (0 = none).
    #[serde(skip)]
    pub occ: Vec<u32>,
    /// Derived: edges grouped by (bucket * 4 + color).
    #[serde(skip)]
    pub edges: Vec<Vec<(u32, u32)>>,
}

/// Relative direction offsets in absolute frame for orientation 0..3 (N, E, S, W).
pub const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

impl World {
    pub fn n(&self) -> usize {
        (self.w * self.h) as usize
    }

    #[inline]
    pub fn idx(&self, x: u32, y: u32) -> u32 {
        y * self.w + x
    }

    #[inline]
    pub fn xy(&self, t: u32) -> (u32, u32) {
        (t % self.w, t / self.w)
    }

    /// Absolute offset from (x, y).
    pub fn offset(&self, t: u32, dx: i32, dy: i32) -> Option<u32> {
        let (x, y) = self.xy(t);
        let nx = x as i64 + dx as i64;
        let ny = y as i64 + dy as i64;
        if nx < 0 || ny < 0 || nx >= self.w as i64 || ny >= self.h as i64 {
            None
        } else {
            Some(self.idx(nx as u32, ny as u32))
        }
    }

    pub fn neighbor(&self, t: u32, abs_dir: u8) -> Option<u32> {
        let (dx, dy) = DIRS[(abs_dir & 3) as usize];
        self.offset(t, dx, dy)
    }

    #[inline]
    pub fn theta(&self, t: u32) -> u64 {
        self.heat_u[t as usize] / self.heat_c[t as usize]
    }

    #[inline]
    pub fn loose_total(&self, t: u32, s: usize) -> u64 {
        let base = self.loose[t as usize][s] as u64;
        if self.residue.is_empty() {
            base
        } else {
            base + self.residue.get(&t).map_or(0, |r| r[s])
        }
    }

    /// Visible stock as seen by READ (saturating u16).
    pub fn loose_visible(&self, t: u32, s: usize) -> u16 {
        self.loose_total(t, s).min(u16::MAX as u64) as u16
    }

    /// Space for normal deposit into the loose channel.
    pub fn loose_space(&self, t: u32, s: usize) -> u64 {
        (u16::MAX as u64).saturating_sub(self.loose_total(t, s))
    }

    /// Normal deposit (caller checked space).
    pub fn put_loose(&mut self, t: u32, s: usize, n: u64) {
        let cur = self.loose[t as usize][s] as u64;
        assert!(cur + n <= u16::MAX as u64, "loose overflow");
        self.loose[t as usize][s] = (cur + n) as u16;
    }

    /// Forced return (death, destruction): overflow goes to residue.
    pub fn force_loose(&mut self, t: u32, s: usize, n: u64) {
        let cur = self.loose[t as usize][s] as u64;
        let room = u16::MAX as u64 - cur;
        let put = room.min(n);
        self.loose[t as usize][s] = (cur + put) as u16;
        if n > put {
            self.residue.entry(t).or_insert([0; NSPEC])[s] += n - put;
        }
    }

    /// Take from loose first, then residue. Caller checked availability.
    pub fn take_loose(&mut self, t: u32, s: usize, n: u64) {
        let cur = self.loose[t as usize][s] as u64;
        let from_loose = cur.min(n);
        self.loose[t as usize][s] = (cur - from_loose) as u16;
        let rest = n - from_loose;
        if rest > 0 {
            let r = self.residue.get_mut(&t).expect("residue available");
            assert!(r[s] >= rest);
            r[s] -= rest;
            if r.iter().all(|v| *v == 0) {
                self.residue.remove(&t);
            }
        }
    }

    pub fn loose_stock_u64(&self, t: u32) -> [u64; NSPEC] {
        let mut s = [0u64; NSPEC];
        for i in 0..NSPEC {
            s[i] = self.loose_total(t, i);
        }
        s
    }

    /// Light at tile t on the given tick.
    pub fn light(&self, t: u32, tick: u64) -> u64 {
        let p = ((tick + 1024) % 4096) as i64;
        let f = (1024 - (p - 1024).abs()).max(0) as u64;
        self.lpeak[t as usize] as u64 * f / 1024
    }

    pub fn mu_eff(&self, t: u32) -> u64 {
        let mu = self.mu[t as usize] as u64;
        let surf = mu.saturating_sub(self.surface_c[t as usize] as u64 / 2).max(64);
        mu.min(surf)
    }

    pub fn rebuild_static_caches(&mut self) {
        let n = self.n();
        // exposure: P=256 inside, 0 outside the map.
        self.expo = (0..n as u32)
            .map(|t| {
                let mut sum = P_OPEN;
                for d in 0..4 {
                    if self.neighbor(t, d).is_some() {
                        sum += P_OPEN;
                    }
                }
                (sum / 5).max(1) as u16
            })
            .collect();
        // edges by bucket and color
        let mut edges = vec![Vec::new(); 32];
        for y in 0..self.h {
            for x in 0..self.w {
                let a = self.idx(x, y);
                if x + 1 < self.w {
                    let b = self.idx(x + 1, y);
                    edges[edge_bucket(a, b) as usize * 4 + (x & 1) as usize].push((a, b));
                }
                if y + 1 < self.h {
                    let b = self.idx(x, y + 1);
                    edges[edge_bucket(a, b) as usize * 4 + 2 + (y & 1) as usize].push((a, b));
                }
            }
        }
        self.edges = edges;
    }
}

pub fn edge_bucket(a: u32, b: u32) -> u8 {
    let (lo, hi) = if a < b { (a, b) } else { (b, a) };
    let mut h = blake3::Hasher::new();
    h.update(&lo.to_le_bytes());
    h.update(&hi.to_le_bytes());
    h.finalize().as_bytes()[0] & 7
}

/// One value-noise layer at lattice spacing l.
fn noise_layer(seed: u64, w: u32, h: u32, l: u32, field: u64, res: u64) -> Vec<u32> {
    let vx = w / l + 1;
    let vy = h / l + 1;
    let mut verts = vec![0u32; (vx * vy) as usize];
    for y in 0..vy {
        for x in 0..vx {
            let obj = (x * l) as u64 | ((y * l) as u64) << 32;
            let key = rng::stream_key(seed, Domain::WorldGen, obj, field * 2 + res);
            verts[(y * vx + x) as usize] = Stream::new(key, 0).next_u32() & 0xFFFF;
        }
    }
    let mut out = vec![0u32; (w * h) as usize];
    let l2 = (l * l) as u64;
    for y in 0..h {
        for x in 0..w {
            let (cx, cy) = (x / l, y / l);
            let (u, v) = ((x % l) as u64, (y % l) as u64);
            let a = verts[(cy * vx + cx) as usize] as u64;
            let b = verts[(cy * vx + cx + 1) as usize] as u64;
            let c = verts[((cy + 1) * vx + cx) as usize] as u64;
            let d = verts[((cy + 1) * vx + cx + 1) as usize] as u64;
            let ll = l as u64;
            let val = ((ll - u) * (ll - v) * a + u * (ll - v) * b + (ll - u) * v * c + u * v * d) / l2;
            out[(y * w + x) as usize] = val as u32;
        }
    }
    out
}

fn noise_field(seed: u64, w: u32, h: u32, field: u64) -> Vec<u16> {
    let v64 = noise_layer(seed, w, h, 64, field, 0);
    let v16 = noise_layer(seed, w, h, 16, field, 1);
    v64.iter().zip(&v16).map(|(a, b)| ((3 * a + b) / 4) as u16).collect()
}

/// WorldGen-3 terrain and initial loose stocks (agents and U are placed by the caller).
pub fn generate(seed: u64, w: u32, h: u32) -> World {
    assert!(w % 64 == 0 && h % 64 == 0 && w > 0 && h > 0, "world size must be a multiple of 64");
    let height = noise_field(seed, w, h, 0);
    let moisture = noise_field(seed, w, h, 1);
    let light = noise_field(seed, w, h, 2);
    let ore = noise_field(seed, w, h, 3);
    let n = (w * h) as usize;
    let mut loose = vec![[0u16; NSPEC]; n];
    for t in 0..n {
        let mut st = Stream::new(rng::stream_key(seed, Domain::WorldGen, t as u64, 8), 0);
        let d0 = st.next_u32();
        let d1 = st.next_u32();
        let l = &mut loose[t];
        l[W] = 8 + moisture[t] / 2048;
        l[C] = 16;
        l[O] = 16;
        l[G] = (d0 % 4) as u16;
        l[S] = 2;
        l[X] = if ore[t] >= 49152 { 32 + (d1 % 32) as u16 } else { 0 };
    }
    let mut world = World {
        w,
        h,
        elev: height.iter().map(|v| (v / 4096) as i16).collect(),
        mu: moisture.iter().map(|v| 256 + v / 128).collect(),
        lpeak: light.iter().map(|v| 32768 + v / 2).collect(),
        substrate: moisture.iter().map(|v| (v / 16384) as u8).collect(),
        surface_c: vec![0; n],
        loose,
        residue: BTreeMap::new(),
        heat_u: vec![0; n],
        rx_credit: vec![[0; 5]; n],
        heat_c: vec![0; n],
        expo: vec![],
        occ: vec![0; n],
        edges: vec![],
    };
    world.rebuild_static_caches();
    world
}

/// Spawn candidates sorted by spawn hash: own G >= 1, four in-map neighbours with |dz| <= 4.
pub fn spawn_tiles(world: &World, seed: u64, count: usize) -> Option<Vec<u32>> {
    let mut cand: Vec<(u32, u32)> = Vec::new();
    for t in 0..world.n() as u32 {
        if world.loose[t as usize][G] < 1 {
            continue;
        }
        let e = world.elev[t as usize];
        let ok = (0..4).all(|d| match world.neighbor(t, d) {
            Some(nb) => (world.elev[nb as usize] - e).abs() <= 4,
            None => false,
        });
        if ok {
            let key = rng::stream_key(seed, Domain::WorldGen, t as u64, 10);
            cand.push((Stream::new(key, 0).next_u32(), t));
        }
    }
    if cand.len() < count {
        return None;
    }
    cand.sort_unstable();
    Some(cand[..count].iter().map(|c| c.1).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generation_is_deterministic_and_in_range() {
        let a = generate(3, 128, 64);
        let b = generate(3, 128, 64);
        assert_eq!(a.elev, b.elev);
        assert_eq!(a.loose, b.loose);
        assert!(a.elev.iter().all(|e| (0..16).contains(e)));
        assert!(a.lpeak.iter().all(|l| *l >= 32768));
        let c = generate(4, 128, 64);
        assert_ne!(a.elev, c.elev);
    }

    #[test]
    fn exposure_edges() {
        let w = generate(1, 64, 64);
        assert_eq!(w.expo[w.idx(10, 10) as usize], 256);
        assert_eq!(w.expo[w.idx(0, 10) as usize], 204);
        assert_eq!(w.expo[0], 153);
        let total: usize = w.edges.iter().map(|e| e.len()).sum();
        assert_eq!(total, 2 * 64 * 63);
    }
}
