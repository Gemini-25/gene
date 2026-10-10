//! Stage 6: simultaneous single-actor Translate via a dependency graph. SPEC §13.

use crate::effects::priority;
use crate::intent::{sub, Intent, Outcome};
use crate::isa::op;
use crate::sim::Sim;

struct Cand {
    i: usize,
    src: u32,
    dst: u32,
    fric: u32,
    dug: i64,
    prio: u64,
    uid: u64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum St {
    Unknown,
    OnPath,
    Ok,
    Fail,
}

impl Sim {
    pub(crate) fn stage_single_motion(&mut self) {
        let tick = self.st.tick;
        let mut cands: Vec<Cand> = Vec::new();
        let mut early: Vec<(usize, Intent, Outcome)> = Vec::new();
        for i in 0..self.st.agents.len() {
            let a = &self.st.agents[i];
            let Some(it) = &a.s.intent else { continue };
            if it.opc != op::DRIVE || it.sub != sub::DRIVE_TRANSLATE {
                continue;
            }
            let dir = it.r[5];
            if dir > 3 {
                early.push((i, it.clone(), Outcome::InvalidTarget));
                continue;
            }
            let src = self.tile_of(i);
            let abs = (a.orient + dir as u8) & 3;
            let Some(dst) = self.st.world.neighbor(src, abs) else {
                early.push((i, it.clone(), Outcome::Boundary));
                continue;
            };
            let w = &self.st.world;
            let mass = a.mass();
            let fric = 256 + (mass * w.mu_eff(dst)).div_ceil(960);
            let dz = w.elev[dst as usize] as i64 - w.elev[src as usize] as i64;
            let dug = mass as i64 * dz;
            let need = fric + dug.max(0) as u64;
            if need > a.s.budget as u64 {
                early.push((i, it.clone(), Outcome::Energy));
                continue;
            }
            cands.push(Cand {
                i,
                src,
                dst,
                fric: fric as u32,
                dug,
                prio: priority(a.keys.prio6, tick),
                uid: a.uid,
            });
        }
        for (i, it, o) in early {
            self.finish_effect(i, &it, (o, 0));
        }
        if cands.is_empty() {
            return;
        }
        // One winner per destination: smallest (priority, uid).
        cands.sort_unstable_by_key(|c| (c.dst, c.prio, c.uid));
        let mut winners: Vec<Cand> = Vec::new();
        for c in cands {
            if winners.last().is_some_and(|w| w.dst == c.dst) {
                let it = self.st.agents[c.i].s.intent.clone().unwrap();
                self.finish_effect(c.i, &it, (Outcome::Blocked, 0));
            } else {
                winners.push(c);
            }
        }
        // Map agent index -> winner index.
        let mut mover = vec![usize::MAX; self.st.agents.len()];
        for (k, c) in winners.iter().enumerate() {
            mover[c.i] = k;
        }
        // next[k]: Ok(Some(j)) depends on winner j, Ok(None) empty target, Err(()) blocked target.
        let next: Vec<Result<Option<usize>, ()>> = winners
            .iter()
            .map(|c| match self.agent_at(c.dst) {
                None => Ok(None),
                Some(j) if mover[j] != usize::MAX => Ok(Some(mover[j])),
                Some(_) => Err(()),
            })
            .collect();
        let mut state = vec![St::Unknown; winners.len()];
        for start in 0..winners.len() {
            if state[start] != St::Unknown {
                continue;
            }
            let mut path = vec![];
            let mut k = start;
            let result = loop {
                match state[k] {
                    St::Ok => break St::Ok,
                    St::Fail => break St::Fail,
                    St::OnPath => break St::Ok, // closed cycle moves as a whole
                    St::Unknown => {}
                }
                state[k] = St::OnPath;
                path.push(k);
                match next[k] {
                    Ok(None) => break St::Ok,
                    Err(()) => break St::Fail,
                    Ok(Some(j)) => k = j,
                }
            };
            for p in path {
                state[p] = result;
            }
        }
        // Heat shares from the starting state, then commit all moves together.
        let ok: Vec<usize> = (0..winners.len()).filter(|k| state[*k] == St::Ok).collect();
        let shares: Vec<(u64, u64)> = ok
            .iter()
            .map(|&k| {
                let c = &winners[k];
                let cap = self.st.agents[c.i].heat_cap();
                let w = &self.st.world;
                let q = (w.heat_u[c.src as usize] as u128 * cap as u128 / w.heat_c[c.src as usize] as u128) as u64;
                (q, cap)
            })
            .collect();
        for (n, &k) in ok.iter().enumerate() {
            let c = &winners[k];
            let (q, cap) = shares[n];
            let w = &mut self.st.world;
            w.heat_u[c.src as usize] -= q;
            w.heat_c[c.src as usize] -= cap;
            w.occ[c.src as usize] = 0;
        }
        for (n, &k) in ok.iter().enumerate() {
            let c = &winners[k];
            let (q, cap) = shares[n];
            let w = &mut self.st.world;
            w.heat_u[c.dst as usize] += q;
            w.heat_c[c.dst as usize] += cap;
            w.occ[c.dst as usize] = c.i as u32 + 1;
            let (x, y) = w.xy(c.dst);
            let a = &mut self.st.agents[c.i];
            a.x = x as u16;
            a.y = y as u16;
        }
        for &k in &ok {
            let c = &winners[k];
            self.pay(c.i, c.fric, c.src);
            if c.dug > 0 {
                self.pay_ug(c.i, c.dug as u32);
            } else if c.dug < 0 {
                self.st.world.heat_u[c.dst as usize] += (-c.dug) as u64;
            }
            let it = self.st.agents[c.i].s.intent.clone().unwrap();
            self.finish_effect(c.i, &it, (Outcome::Ok, 1));
            self.obs.tick.moves += 1;
        }
        for (k, c) in winners.iter().enumerate() {
            if state[k] == St::Fail {
                let it = self.st.agents[c.i].s.intent.clone().unwrap();
                self.finish_effect(c.i, &it, (Outcome::Blocked, 0));
            }
        }
    }
}
