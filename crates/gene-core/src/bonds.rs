//! M1d: face bonds, anchors, grip, and assembly motion checked by a load graph (SPEC §20e).

use crate::effects::{priority, Res};
use crate::intent::{dom, rotate, sub, Intent, Outcome};
use crate::isa::op;
use crate::sim::Sim;
use crate::species::{heat_cap, mass};
use std::collections::VecDeque;

/// Largest assembly that moves as one (original §11).
pub const MAX_ASSEMBLY: usize = 16;
/// LINK cost per strength unit; UNLINK cost per action (original §11).
pub const LINK_Q: u32 = 64;
pub const UNLINK_Q: u32 = 128;
/// Load capacity per strength unit (original §64).
pub const LOAD_PER_STRENGTH: u64 = 16;
/// DRIVE sub-op 2: push the parcel in front one tile forward.
pub const DRIVE_PUSH: u16 = 2;

const R4_ANCHOR: u16 = 4;
const R4_GRIP: u16 = 5;

/// Can `source_cap` units entering at `root` meet every parcel's demand through the undirected
/// bond edges (a, b, capacity)? Integer Edmonds–Karp on at most 16 parcels plus source and sink.
pub fn load_ok(n: usize, edges: &[(usize, usize, u64)], root: usize, source_cap: u64, demand: &[u64]) -> bool {
    let total: u128 = demand.iter().map(|&d| d as u128).sum();
    if total == 0 {
        return true;
    }
    if total > source_cap as u128 {
        return false;
    }
    let (s, t, m) = (n, n + 1, n + 2);
    let mut cap = vec![vec![0u128; m]; m];
    cap[s][root] += source_cap as u128;
    for &(a, b, c) in edges {
        cap[a][b] += c as u128;
        cap[b][a] += c as u128;
    }
    for (v, &d) in demand.iter().enumerate() {
        cap[v][t] += d as u128;
    }
    let mut flow = 0u128;
    loop {
        let mut prev = vec![usize::MAX; m];
        prev[s] = s;
        let mut q = VecDeque::from([s]);
        while let Some(u) = q.pop_front() {
            if u == t {
                break;
            }
            for v in 0..m {
                if prev[v] == usize::MAX && cap[u][v] > 0 {
                    prev[v] = u;
                    q.push_back(v);
                }
            }
        }
        if prev[t] == usize::MAX {
            break;
        }
        let mut aug = u128::MAX;
        let mut v = t;
        while v != s {
            aug = aug.min(cap[prev[v]][v]);
            v = prev[v];
        }
        let mut v = t;
        while v != s {
            let u = prev[v];
            cap[u][v] -= aug;
            cap[v][u] += aug;
            v = u;
        }
        flow += aug;
    }
    flow == total
}

/// One rigid motion of a bonded assembly, optionally carrying the acting agent.
struct Plan {
    /// Component tiles (BFS order) and their destinations.
    parts: Vec<u32>,
    dst: Vec<u32>,
    /// The acting agent's (src, dst) when it moves with the assembly.
    actor: Option<(u32, u32)>,
    /// Quarter turns applied to parcels and the actor.
    turn: u8,
    /// Index in `parts` where the load enters, and the entry capacity.
    root: usize,
    source_cap: u64,
    /// Base action cost (256 translate and push, 64 rotate).
    base: u32,
    /// Push: any grip on the assembly holds it, the pusher's own included.
    push: bool,
}

impl Sim {
    /// The parcel tile this agent grips. A grip keeps its parcel within reach (own tile or a
    /// neighbour), so five lookups find it.
    pub(crate) fn grip_of(&self, i: usize) -> Option<u32> {
        let uid = self.st.agents[i].uid;
        let me = self.tile_of(i);
        let w = &self.st.world;
        if w.parcels.is_empty() {
            return None;
        }
        std::iter::once(Some(me))
            .chain((0..4).map(|d| w.neighbor(me, d)))
            .flatten()
            .find(|t| w.parcels.get(t).is_some_and(|p| p.grip_uid == uid))
    }

    pub(crate) fn release_grip(&mut self, i: usize) {
        if let Some(t) = self.grip_of(i) {
            let p = self.st.world.parcels.get_mut(&t).unwrap();
            p.grip = 0;
            p.grip_uid = 0;
        }
    }

    /// After the parcel on t changed or vanished: bonds never exceed the current cohesion of
    /// either side, and a vanished parcel leaves no bond behind.
    pub(crate) fn clamp_bonds(&mut self, t: u32) {
        let w = &mut self.st.world;
        let Some(p) = w.parcels.get(&t) else {
            for f in 0..4u8 {
                if let Some(q) = w.neighbor(t, f).and_then(|nb| w.parcels.get_mut(&nb)) {
                    q.bond[(f as usize + 2) & 3] = 0;
                }
            }
            return;
        };
        let b = p.cohesion();
        let mut faces = p.bond;
        for f in 0..4u8 {
            if faces[f as usize] == 0 {
                continue;
            }
            let nb = w.neighbor(t, f).expect("bond across the map edge");
            let q = w.parcels.get_mut(&nb).expect("bond without a partner");
            let v = faces[f as usize].min(b).min(q.cohesion());
            q.bond[(f as usize + 2) & 3] = v;
            faces[f as usize] = v;
        }
        let p = w.parcels.get_mut(&t).unwrap();
        p.bond = faces;
        p.anchor = p.anchor.min(b);
        p.grip = p.grip.min(b);
        if p.grip == 0 {
            p.grip_uid = 0;
        }
    }

    /// Bonded component around `root` in BFS order (faces N, E, S, W). More than 16 is Limits.
    fn component(&self, root: u32) -> Result<Vec<u32>, Outcome> {
        let w = &self.st.world;
        let mut out = vec![root];
        let mut k = 0;
        while k < out.len() {
            let t = out[k];
            k += 1;
            let p = &w.parcels[&t];
            for f in 0..4u8 {
                if p.bond[f as usize] == 0 {
                    continue;
                }
                let nb = w.neighbor(t, f).expect("bond across the map edge");
                if !out.contains(&nb) {
                    if out.len() == MAX_ASSEMBLY {
                        return Err(Outcome::Limits);
                    }
                    out.push(nb);
                }
            }
        }
        Ok(out)
    }

    /// LINK / UNLINK: face bond (r4 0..3), anchor (4) or grip (5) on the parcel at (r1, r2).
    pub(crate) fn eff_link(&mut self, i: usize, it: &Intent) -> Res {
        let r = it.r;
        let link = it.opc == op::LINK;
        if r[0] != dom::PARCEL || r[4] > R4_GRIP {
            return (Outcome::InvalidTarget, 0);
        }
        let ta = match self.rel_vec_tile(i, r[1] as i16, r[2] as i16, 1) {
            Ok(t) => t,
            Err(e) => return (e, 0),
        };
        let w = &self.st.world;
        let Some(pa) = w.parcels.get(&ta) else { return (Outcome::InvalidTarget, 0) };
        if it.parcel_uid != Some(pa.uid) {
            return (Outcome::Stale, 0);
        }
        let me = self.tile_of(i);
        let uid = self.st.agents[i].uid;
        let mut tb = None;
        let (cur, limit) = match r[4] {
            R4_ANCHOR => (pa.anchor, pa.cohesion()),
            R4_GRIP => {
                if pa.grip_uid != 0 && pa.grip_uid != uid {
                    return (Outcome::Busy, 0);
                }
                if link && pa.grip_uid != uid && self.grip_of(i).is_some() {
                    return (Outcome::Busy, 0); // one grip per agent
                }
                (if pa.grip_uid == uid { pa.grip } else { 0 }, pa.cohesion())
            }
            f => {
                // both endpoints within reach: one of them is the agent's own tile
                let Some(t) = w.neighbor(ta, f as u8) else { return (Outcome::Boundary, 0) };
                if w.manhattan(me, t) > 1 {
                    return (Outcome::InvalidTarget, 0);
                }
                let Some(pb) = w.parcels.get(&t) else { return (Outcome::InvalidTarget, 0) };
                if it.target_uid != Some(pb.uid) {
                    return (Outcome::Stale, 0);
                }
                tb = Some(t);
                (pa.bond[f as usize], pa.cohesion().min(pb.cohesion()))
            }
        };
        let req = r[5] as u32;
        if req == 0 {
            return (Outcome::Noop, 0);
        }
        let budget = self.st.agents[i].s.budget;
        let (n, next) = if link {
            if budget < LINK_Q {
                return (Outcome::Energy, 0);
            }
            let room = limit.saturating_sub(cur) as u32;
            if room == 0 {
                return (Outcome::Capacity, 0);
            }
            let n = req.min(room).min(budget / LINK_Q);
            self.pay(i, n * LINK_Q, ta);
            self.obs.m1.links += n as u64;
            (n, cur + n as u8)
        } else {
            if cur == 0 {
                return (Outcome::Noop, 0);
            }
            if budget < UNLINK_Q {
                return (Outcome::Energy, 0);
            }
            let n = req.min(cur as u32);
            self.pay(i, UNLINK_Q, ta);
            self.obs.m1.unlinks += 1;
            (n, cur - n as u8)
        };
        let w = &mut self.st.world;
        match r[4] {
            R4_ANCHOR => w.parcels.get_mut(&ta).unwrap().anchor = next,
            R4_GRIP => {
                let p = w.parcels.get_mut(&ta).unwrap();
                p.grip = next;
                p.grip_uid = if next > 0 { uid } else { 0 };
            }
            f => {
                w.parcels.get_mut(&ta).unwrap().bond[f as usize] = next;
                w.parcels.get_mut(&tb.unwrap()).unwrap().bond[(f as usize + 2) & 3] = next;
            }
        }
        (if n == req { Outcome::Ok } else { Outcome::Partial }, n)
    }

    /// Stage 5: DRIVE intents that move structures, one agent at a time in (priority, uid) order.
    /// Translate without a grip is left to the simultaneous stage 6.
    pub(crate) fn stage_assembly(&mut self) {
        if !self.caps.bonds {
            return;
        }
        let tick = self.st.tick;
        let mut order: Vec<(u64, u64, usize)> = Vec::new();
        for (i, a) in self.st.agents.iter().enumerate() {
            if a.s.intent.as_ref().is_some_and(|it| it.opc == op::DRIVE) {
                order.push((priority(a.keys.prio6, tick), a.uid, i));
            }
        }
        order.sort_unstable();
        for (_, _, i) in order {
            // an agent pushed earlier in this stage has no intent left
            let Some(it) = self.st.agents[i].s.intent.clone() else { continue };
            let res = match it.sub {
                sub::DRIVE_TRANSLATE => match self.grip_of(i) {
                    Some(g) => self.grip_translate(i, &it, g),
                    None => continue,
                },
                sub::DRIVE_ROTATE => match self.grip_of(i) {
                    Some(g) if it.r[5] <= 3 && it.r[5] != 0 => self.grip_rotate(i, &it, g),
                    _ => self.eff_rotate(i, &it),
                },
                DRIVE_PUSH => self.push_parcel(i),
                crate::contact::DRIVE_PUSH_ACTOR if self.caps.contact => self.push_actor(i, &it),
                _ => (Outcome::InvalidTarget, 0),
            };
            self.finish_effect(i, &it, res);
            self.st.agents[i].s.intent = None;
        }
    }

    fn grip_translate(&mut self, i: usize, it: &Intent, g: u32) -> Res {
        let dir = it.r[5];
        if dir > 3 {
            return (Outcome::InvalidTarget, 0);
        }
        let abs = (self.st.agents[i].orient + dir as u8) & 3;
        let src = self.tile_of(i);
        let parts = match self.component(g) {
            Ok(p) => p,
            Err(e) => return (e, 0),
        };
        let w = &self.st.world;
        let Some(adst) = w.neighbor(src, abs) else { return (Outcome::Boundary, 0) };
        let mut dst = Vec::with_capacity(parts.len());
        for &t in &parts {
            match w.neighbor(t, abs) {
                Some(d) => dst.push(d),
                None => return (Outcome::Boundary, 0),
            }
        }
        let source_cap = LOAD_PER_STRENGTH * w.parcels[&g].grip as u64;
        self.execute(i, Plan { parts, dst, actor: Some((src, adst)), turn: 0, root: 0, source_cap, base: 256, push: false })
    }

    fn grip_rotate(&mut self, i: usize, it: &Intent, g: u32) -> Res {
        let turn = it.r[5] as u8;
        let parts = match self.component(g) {
            Ok(p) => p,
            Err(e) => return (e, 0),
        };
        let w = &self.st.world;
        let (cx, cy) = w.xy(g);
        let spin = |t: u32| -> Option<u32> {
            let (x, y) = w.xy(t);
            let (dx, dy) = rotate(turn, x as i32 - cx as i32, y as i32 - cy as i32);
            w.offset(g, dx, dy)
        };
        let src = self.tile_of(i);
        let Some(adst) = spin(src) else { return (Outcome::Boundary, 0) };
        let mut dst = Vec::with_capacity(parts.len());
        for &t in &parts {
            match spin(t) {
                Some(d) => dst.push(d),
                None => return (Outcome::Boundary, 0),
            }
        }
        let source_cap = LOAD_PER_STRENGTH * w.parcels[&g].grip as u64;
        self.execute(i, Plan { parts, dst, actor: Some((src, adst)), turn, root: 0, source_cap, base: 64, push: false })
    }

    /// DRIVE 2: push the assembly in front one tile forward. Contact support is limited by the
    /// front parcel's own cohesion. The agent stays (M1d) or follows into the front tile (M1e,
    /// original §10).
    fn push_parcel(&mut self, i: usize) -> Res {
        let abs = self.st.agents[i].orient;
        let w = &self.st.world;
        let me = self.tile_of(i);
        let Some(front) = w.neighbor(me, abs) else { return (Outcome::Boundary, 0) };
        let Some(p) = w.parcels.get(&front) else { return (Outcome::InvalidTarget, 0) };
        let source_cap = LOAD_PER_STRENGTH * p.cohesion() as u64;
        let parts = match self.component(front) {
            Ok(p) => p,
            Err(e) => return (e, 0),
        };
        let mut dst = Vec::with_capacity(parts.len());
        for &t in &parts {
            match w.neighbor(t, abs) {
                Some(d) => dst.push(d),
                None => return (Outcome::Boundary, 0),
            }
        }
        let actor = self.caps.contact.then_some((me, front));
        let r = self.execute(i, Plan { parts, dst, actor, turn: 0, root: 0, source_cap, base: 256, push: true });
        if r.0 == Outcome::Ok {
            self.obs.m1.pushes += 1;
        }
        r
    }

    fn execute(&mut self, i: usize, plan: Plan) -> Res {
        let uid = self.st.agents[i].uid;
        let w = &self.st.world;
        let n = plan.parts.len();
        if plan.parts.iter().any(|t| w.parcels[t].anchor > 0) {
            return (Outcome::Blocked, 0);
        }
        let held = |g: u64| g != 0 && (plan.push || g != uid);
        if plan.parts.iter().any(|t| held(w.parcels[t].grip_uid)) {
            return (Outcome::Busy, 0);
        }
        let actor_moves = plan.actor.filter(|(s, d)| s != d);
        let actor_dst = plan.actor.map(|a| a.1);
        // destinations: no outside parcel; a blocking parcel lands on no agent or reserved birth tile
        for (k, &d) in plan.dst.iter().enumerate() {
            if w.parcels.contains_key(&d) && !plan.parts.contains(&d) {
                return (Outcome::Blocked, 0);
            }
            if w.parcels[&plan.parts[k]].blocks() {
                let occ = w.occ[d as usize];
                let leaving = occ == i as u32 + 1 && actor_moves.is_some();
                if (occ != 0 && !leaving) || actor_dst == Some(d) || self.reserved.contains(&d) {
                    return (Outcome::Blocked, 0);
                }
            }
        }
        if let Some((_, d)) = actor_moves {
            let occ = w.occ[d as usize];
            let static_wall = w.parcels.get(&d).is_some_and(|p| p.blocks()) && !plan.parts.contains(&d);
            if (occ != 0 && occ != i as u32 + 1) || self.reserved.contains(&d) || static_wall {
                return (Outcome::Blocked, 0);
            }
        }
        // load graph (original §64): demand per parcel, bonds as undirected capacities
        let mut demand = Vec::with_capacity(n);
        let mut fric = Vec::with_capacity(n);
        let mut dug = Vec::with_capacity(n);
        for (k, &t) in plan.parts.iter().enumerate() {
            let d = plan.dst[k];
            let m = mass(&w.parcels[&t].matter());
            let dist = w.manhattan(t, d) as u64;
            let f = (m * dist * w.mu_ground(d)).div_ceil(960);
            let u = m as i64 * (w.elev[d as usize] as i64 - w.elev[t as usize] as i64);
            demand.push(f + u.max(0) as u64);
            fric.push(f);
            dug.push(u);
        }
        let mut edges = Vec::new();
        for (k, &t) in plan.parts.iter().enumerate() {
            let p = &w.parcels[&t];
            for f in 0..4u8 {
                if p.bond[f as usize] == 0 {
                    continue;
                }
                let nb = w.neighbor(t, f).unwrap();
                let j = plan.parts.iter().position(|x| *x == nb).unwrap();
                if j > k {
                    edges.push((k, j, LOAD_PER_STRENGTH * p.bond[f as usize] as u64));
                }
            }
        }
        if !load_ok(n, &edges, plan.root, plan.source_cap, &demand) {
            self.obs.m1.load_failures += 1;
            return (Outcome::Blocked, 0);
        }
        // energy: base, friction of every moving body, and every uphill lift (no downhill offset)
        let a = &self.st.agents[i];
        let (actor_fric, actor_dug) = match actor_moves {
            Some((s, d)) => {
                let m = a.mass();
                let dist = w.manhattan(s, d) as u64;
                let u = m as i64 * (w.elev[d as usize] as i64 - w.elev[s as usize] as i64);
                ((m * dist * w.mu_eff(d)).div_ceil(960), u)
            }
            None => (0, 0),
        };
        let need = plan.base as u64
            + actor_fric
            + actor_dug.max(0) as u64
            + fric.iter().sum::<u64>()
            + dug.iter().map(|u| u.max(&0).unsigned_abs()).sum::<u64>();
        if need > a.s.budget as u64 {
            return (Outcome::Energy, 0);
        }
        // heat shares from the starting state, then move everything together
        let mut shares: Vec<(u32, u32, u64, u64)> = Vec::with_capacity(n + 1);
        for (k, &t) in plan.parts.iter().enumerate() {
            let c = heat_cap(&w.parcels[&t].matter());
            let q = (w.heat_u[t as usize] as u128 * c as u128 / w.heat_c[t as usize] as u128) as u64;
            shares.push((t, plan.dst[k], q, c));
        }
        if let Some((s, d)) = actor_moves {
            let c = a.heat_cap();
            let q = (w.heat_u[s as usize] as u128 * c as u128 / w.heat_c[s as usize] as u128) as u64;
            shares.push((s, d, q, c));
        }
        let w = &mut self.st.world;
        for &(s, _, q, c) in &shares {
            w.heat_u[s as usize] -= q;
            w.heat_c[s as usize] -= c;
        }
        for &(_, d, q, c) in &shares {
            w.heat_u[d as usize] += q;
            w.heat_c[d as usize] += c;
        }
        let moved: Vec<_> = plan.parts.iter().map(|t| w.parcels.remove(t).unwrap()).collect();
        for (k, mut p) in moved.into_iter().enumerate() {
            if plan.turn != 0 {
                let b = p.bond;
                for f in 0..4 {
                    p.bond[(f + plan.turn as usize) & 3] = b[f];
                }
                p.orient = (p.orient + plan.turn) & 3;
            }
            w.parcels.insert(plan.dst[k], p);
        }
        if let Some((s, d)) = actor_moves {
            w.occ[s as usize] = 0;
            w.occ[d as usize] = i as u32 + 1;
            let (x, y) = w.xy(d);
            let a = &mut self.st.agents[i];
            a.x = x as u16;
            a.y = y as u16;
        }
        if plan.actor.is_some() {
            let a = &mut self.st.agents[i];
            a.orient = (a.orient + plan.turn) & 3;
        }
        // pay: base and the agent's friction at its start tile, each parcel's friction at its start
        let here = plan.actor.map_or(self.tile_of(i), |a| a.0);
        self.pay(i, plan.base + actor_fric as u32, here);
        for k in 0..n {
            self.pay(i, fric[k] as u32, plan.parts[k]);
        }
        let lifts = std::iter::once((actor_dug, actor_dst.unwrap_or(here))).chain(dug.iter().copied().zip(plan.dst.iter().copied()));
        for (u, d) in lifts.collect::<Vec<_>>() {
            if u > 0 {
                self.pay_ug(i, u as u32);
            } else if u < 0 {
                self.st.world.heat_u[d as usize] += u.unsigned_abs();
            }
        }
        let mut touched: Vec<u32> = plan.parts.iter().chain(plan.dst.iter()).copied().collect();
        touched.sort_unstable();
        touched.dedup();
        for t in touched {
            self.st.world.parcel_changed(t);
        }
        self.obs.m1.assembly_moves += 1;
        self.obs.m1.parcels_moved += n as u64;
        (Outcome::Ok, n as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::load_ok;

    #[test]
    fn original_section_64_examples() {
        // root demand 0, end demand 80, grip 10 (160), one bond of strength 4 (64): fails
        assert!(!load_ok(2, &[(0, 1, 64)], 0, 160, &[0, 80]));
        // bond strength 5 (80): passes
        assert!(load_ok(2, &[(0, 1, 80)], 0, 160, &[0, 80]));
        // a loop whose two paths carry 48 and 32 together delivers 80
        let edges = [(0, 1, 48), (1, 3, 48), (0, 2, 32), (2, 3, 32)];
        assert!(load_ok(4, &edges, 0, 160, &[0, 0, 0, 80]));
        assert!(!load_ok(4, &edges, 0, 160, &[0, 0, 0, 81]));
        // the source itself limits the total
        assert!(!load_ok(1, &[], 0, 79, &[80]));
    }

    #[test]
    fn opposite_flows_cancel_instead_of_adding() {
        // a chain 1 - 0 - 2 with the root in the middle: each side draws only its own edge
        assert!(load_ok(3, &[(0, 1, 16), (0, 2, 16)], 0, 64, &[10, 16, 16]));
        assert!(!load_ok(3, &[(0, 1, 16), (0, 2, 16)], 0, 64, &[0, 17, 15]));
    }
}
