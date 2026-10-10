//! Tiles, terrain generation and spatial helpers. SPEC §3.

use crate::rng::{self, Domain, Stream};
use crate::species::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const BASE_HEAT_CAP: u64 = 1024;
pub const P_OPEN: u64 = 256;

/// A structure on a tile (M1c, SPEC §20d).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Parcel {
    pub uid: u64,
    pub matrix: [u16; NSPEC],
    pub pore: [u16; NSPEC],
    pub c: u8,
    /// Reaction credits per site (matrix, pore, boundary) for Rx2..Rx6.
    pub credit: [[u16; 5]; 3],
    /// M1d: symmetric face bond strength per absolute face (N, E, S, W).
    pub bond: [u8; 4],
    /// M1d: bond to the fixed ground.
    pub anchor: u8,
    /// M1d: quarter turns accumulated by assembly rotation.
    pub orient: u8,
    /// M1d: grip strength and the gripping agent's uid (0 = none).
    pub grip: u8,
    pub grip_uid: u64,
}

/// Species allowed in a matrix (solids).
pub const SOLID: [bool; NSPEC] = [false, false, false, true, false, true, true, true, false, true];
/// Gases (exchange through pores, leave matrices to the loose stock).
pub const GAS: [bool; NSPEC] = [false, true, true, false, true, false, false, false, true, false];

/// Cohesion per matrix unit (G, X, M, S, I; others 0). Original §11.
pub const COHESION: [u32; NSPEC] = [0, 0, 0, 3, 0, 4, 6, 4, 0, 1];

impl Parcel {
    pub fn new(uid: u64) -> Parcel {
        Parcel {
            uid,
            matrix: [0; NSPEC],
            pore: [0; NSPEC],
            c: 0,
            credit: [[0; 5]; 3],
            bond: [0; 4],
            anchor: 0,
            orient: 0,
            grip: 0,
            grip_uid: 0,
        }
    }

    /// Bond limit of this parcel: min(255, Σ cohesion·matrix).
    pub fn cohesion(&self) -> u8 {
        let b: u32 = (0..NSPEC).map(|s| COHESION[s] * self.matrix[s] as u32).sum();
        b.min(255) as u8
    }

    pub fn volume(&self) -> u32 {
        volume(&self.matrix)
    }

    pub fn pore_volume(&self) -> u32 {
        volume(&self.pore)
    }

    /// Permeability P = max(1, 256 - floor(s*c/64)).
    pub fn perm(&self) -> u16 {
        256u32.saturating_sub(self.volume() * self.c as u32 / 64).max(1) as u16
    }

    /// Pore capacity K = 64 - ceil(s*c/256) for a given matrix volume and compression.
    pub fn capacity_for(s: u32, c: u8) -> u32 {
        64u32.saturating_sub((s * c as u32).div_ceil(256))
    }

    pub fn capacity(&self) -> u32 {
        Self::capacity_for(self.volume(), self.c)
    }

    pub fn blocks(&self) -> bool {
        self.volume() >= 16 && self.c >= 128
    }

    pub fn matter(&self) -> [u64; NSPEC] {
        let mut m = [0u64; NSPEC];
        for i in 0..NSPEC {
            m[i] = self.matrix[i] as u64 + self.pore[i] as u64;
        }
        m
    }
}

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
    /// Structures by tile (M1c).
    pub parcels: BTreeMap<u32, Parcel>,
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
    /// Derived: permeability P per tile (256 without a parcel).
    #[serde(skip)]
    pub perm: Vec<u16>,
    /// Derived: min(8, metal M in the tile's matrix) for conduction.
    #[serde(skip)]
    pub metal: Vec<u8>,
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

    pub fn manhattan(&self, a: u32, b: u32) -> u32 {
        let (ax, ay) = self.xy(a);
        let (bx, by) = self.xy(b);
        ax.abs_diff(bx) + ay.abs_diff(by)
    }

    pub fn neighbor(&self, t: u32, abs_dir: u8) -> Option<u32> {
        let (dx, dy) = DIRS[(abs_dir & 3) as usize];
        self.offset(t, dx, dy)
    }

    pub fn heat_u_at(&self, t: u32) -> u64 {
        self.heat_u[t as usize]
    }

    pub fn heat_c_at(&self, t: u32) -> u64 {
        self.heat_c[t as usize]
    }

    pub fn surface_c_at(&self, t: u32) -> u8 {
        self.surface_c[t as usize]
    }

    pub fn rx_credit_at(&self, t: u32) -> [u16; 5] {
        self.rx_credit[t as usize]
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

    /// Ground friction without the parcel term (what a parcel slides on, M1d).
    pub fn mu_ground(&self, t: u32) -> u64 {
        let mu = self.mu[t as usize] as u64;
        let surf = mu.saturating_sub(self.surface_c[t as usize] as u64 / 2).max(64);
        mu.min(surf)
    }

    pub fn mu_eff(&self, t: u32) -> u64 {
        let mu = self.mu[t as usize] as u64;
        let mut m = self.mu_ground(t);
        if let Some(p) = self.parcels.get(&t) {
            let s = p.volume() as u64;
            if !p.blocks() && s >= 4 && p.c >= 64 {
                m = m.min(mu.saturating_sub(p.c as u64 * s / 16).max(64));
            }
        }
        m
    }

    pub fn blocked(&self, t: u32) -> bool {
        self.parcels.get(&t).is_some_and(|p| p.blocks())
    }

    /// Recompute P, metal and exposure X around tile t after its parcel changed.
    pub fn parcel_changed(&mut self, t: u32) {
        let p = self.parcels.get(&t);
        self.perm[t as usize] = p.map_or(P_OPEN as u16, |p| p.perm());
        self.metal[t as usize] = p.map_or(0, |p| p.matrix[M].min(8) as u8);
        self.update_expo(t);
        for d in 0..4 {
            if let Some(nb) = self.neighbor(t, d) {
                self.update_expo(nb);
            }
        }
    }

    fn update_expo(&mut self, t: u32) {
        let ps = self.perm[t as usize] as u64;
        let mut sum = ps;
        for d in 0..4 {
            if let Some(nb) = self.neighbor(t, d) {
                sum += ps.min(self.perm[nb as usize] as u64);
            }
        }
        self.expo[t as usize] = (sum / 5).max(1) as u16;
    }

    pub fn rebuild_static_caches(&mut self) {
        let n = self.n();
        self.perm = vec![P_OPEN as u16; n];
        self.metal = vec![0; n];
        for (t, p) in &self.parcels {
            self.perm[*t as usize] = p.perm();
            self.metal[*t as usize] = p.matrix[M].min(8) as u8;
        }
        // exposure X from P (0 outside the map)
        self.expo = vec![0; n];
        for t in 0..n as u32 {
            self.update_expo(t);
        }
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
        parcels: BTreeMap::new(),
        perm: vec![],
        metal: vec![],
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
