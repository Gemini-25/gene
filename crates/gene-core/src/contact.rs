//! M1e: contact between agents. Abrasion of another agent's shell and body, and PushActor.
//! Taking from a bare inventory is a TRANSFER source (effects.rs). SPEC §20f, original §13, §10.

use crate::effects::Res;
use crate::intent::{dom, Intent, Outcome};
use crate::sim::Sim;
use crate::species::{G, S};

/// DRIVE sub-op 3: step forward and push the agent in front one tile further.
pub const DRIVE_PUSH_ACTOR: u16 = 3;
/// Contact work that removes one shell unit, and one body G unit once the shell is gone.
pub const SHELL_UNIT_Q: u32 = 2048;
pub const BODY_UNIT_Q: u32 = 8192;
/// Dissipated once per push, as for any translate.
const PUSH_BASE_Q: u64 = 256;

impl Sim {
    /// The agent on tile t, if it is the one captured at Think.
    fn captured_at(&self, t: u32, it: &Intent) -> Result<usize, Outcome> {
        match (self.agent_at(t), it.target_uid) {
            (Some(j), Some(uid)) if self.st.agents[j].uid == uid => Ok(j),
            (_, None) => Err(Outcome::InvalidTarget),
            _ => Err(Outcome::Stale),
        }
    }

    /// WORK Abrade on ActorSurface: r5 q of contact work on the agent at (r1, r2), radius 1.
    /// Every q is heat on the target tile at once; the target keeps the progress on its current
    /// layer, and each whole unit returns one S (shell) or G (body) to the target tile as loose.
    /// Work beyond what the current layer can still take is not done and not paid.
    pub(crate) fn eff_abrade_actor(&mut self, i: usize, it: &Intent) -> Res {
        let r = it.r;
        if (r[1], r[2]) == (0, 0) {
            return (Outcome::InvalidTarget, 0);
        }
        let t = match self.rel_vec_tile(i, r[1] as i16, r[2] as i16, 1) {
            Ok(t) => t,
            Err(e) => return (e, 0),
        };
        let j = match self.captured_at(t, it) {
            Ok(j) => j,
            Err(e) => return (e, 0),
        };
        let req = r[5] as u32;
        if req == 0 {
            return (Outcome::Noop, 0);
        }
        let v = &self.st.agents[j];
        let (unit, layer) = if v.shell_s > 0 { (SHELL_UNIT_Q, v.shell_s) } else { (BODY_UNIT_Q, v.body_g) };
        let room = (layer as u64 * unit as u64).saturating_sub(v.abrasion as u64);
        if room == 0 {
            return (Outcome::Capacity, 0);
        }
        let budget = self.st.agents[i].s.budget;
        if budget == 0 {
            return (Outcome::Energy, 0);
        }
        let n = (req as u64).min(room).min(budget as u64) as u32;
        self.pay(i, n, t);
        self.obs.m1.abrasion_q += n as u64;
        let v = &mut self.st.agents[j];
        let progress = v.abrasion as u64 + n as u64;
        let units = (progress / unit as u64) as u16;
        let shell = unit == SHELL_UNIT_Q;
        if shell {
            v.shell_s -= units;
        } else {
            v.body_g -= units;
        }
        // a finished layer leaves no credit for the next one
        v.abrasion = if shell && v.shell_s == 0 { 0 } else { (progress % unit as u64) as u32 };
        if units > 0 {
            // the same tile holds the body and the loose units, so its heat capacity is unchanged
            self.st.world.force_loose(t, if shell { S } else { G }, units as u64);
            if shell {
                self.obs.m1.shell_removed += units as u64;
            } else {
                self.obs.m1.body_removed += units as u64;
            }
        }
        (if n == req { Outcome::Ok } else { Outcome::Partial }, n)
    }

    /// DRIVE 3: the pusher steps into the tile in front and the agent there moves one tile on, as
    /// one transaction. No chains; a target holding a grip stays put (Blocked); a pusher holding
    /// one must translate instead (Busy). The pushed agent's pending DRIVE becomes Stale.
    pub(crate) fn push_actor(&mut self, i: usize, it: &Intent) -> Res {
        let w = &self.st.world;
        let dir = self.st.agents[i].orient;
        let src = self.tile_of(i);
        let Some(front) = w.neighbor(src, dir) else { return (Outcome::Boundary, 0) };
        let j = match self.captured_at(front, it) {
            Ok(j) => j,
            Err(e) => return (e, 0),
        };
        let Some(beyond) = w.neighbor(front, dir) else { return (Outcome::Boundary, 0) };
        if self.grip_of(i).is_some() {
            return (Outcome::Busy, 0);
        }
        if self.grip_of(j).is_some() {
            return (Outcome::Blocked, 0);
        }
        if w.occ[beyond as usize] != 0 || w.blocked(beyond) || self.reserved.contains(&beyond) {
            return (Outcome::Blocked, 0);
        }
        // friction of each body at its destination, every uphill lift, one base cost
        let (a, b) = (&self.st.agents[i], &self.st.agents[j]);
        let (ma, mb) = (a.mass(), b.mass());
        let fa = (ma * w.mu_eff(front)).div_ceil(960);
        let fb = (mb * w.mu_eff(beyond)).div_ceil(960);
        let ua = ma as i64 * (w.elev[front as usize] as i64 - w.elev[src as usize] as i64);
        let ub = mb as i64 * (w.elev[beyond as usize] as i64 - w.elev[front as usize] as i64);
        let need = PUSH_BASE_Q + fa + fb + ua.max(0) as u64 + ub.max(0) as u64;
        if need > a.s.budget as u64 {
            return (Outcome::Energy, 0);
        }
        // heat shares from the starting state, then both move together
        let (ca, cb) = (a.heat_cap(), b.heat_cap());
        let qa = (w.heat_u[src as usize] as u128 * ca as u128 / w.heat_c[src as usize] as u128) as u64;
        let qb = (w.heat_u[front as usize] as u128 * cb as u128 / w.heat_c[front as usize] as u128) as u64;
        let w = &mut self.st.world;
        for (s, d, q, c) in [(src, front, qa, ca), (front, beyond, qb, cb)] {
            w.heat_u[s as usize] -= q;
            w.heat_c[s as usize] -= c;
            w.heat_u[d as usize] += q;
            w.heat_c[d as usize] += c;
        }
        w.occ[src as usize] = 0;
        w.occ[front as usize] = i as u32 + 1;
        w.occ[beyond as usize] = j as u32 + 1;
        for (k, t) in [(i, front), (j, beyond)] {
            let (x, y) = self.st.world.xy(t);
            let a = &mut self.st.agents[k];
            a.x = x as u16;
            a.y = y as u16;
        }
        // base and the pusher's friction at its start tile, the target's friction at its own
        self.pay(i, (PUSH_BASE_Q + fa) as u32, src);
        self.pay(i, fb as u32, front);
        for (u, d) in [(ua, front), (ub, beyond)] {
            if u > 0 {
                self.pay_ug(i, u as u32);
            } else if u < 0 {
                self.st.world.heat_u[d as usize] += u.unsigned_abs();
            }
        }
        if let Some(jt) = self.st.agents[j].s.intent.take_if(|t| t.opc == crate::isa::op::DRIVE) {
            self.finish_effect(j, &jt, (Outcome::Stale, 0));
        }
        self.obs.m1.actor_pushes += 1;
        (Outcome::Ok, 1)
    }
}

/// Behaviour facts tell taking and abrading another agent apart from the same effect on the
/// ground: TRANSFER from ActorInventory keeps its species with CONTACT set, WORK Abrade on
/// ActorSurface has channel CONTACT.
pub(crate) fn mark_fact(f: &mut crate::observer::EffectFact, it: &Intent) {
    use crate::observer::EffectFact;
    if is_actor_abrade(it) {
        f.channel = EffectFact::CONTACT;
    } else if it.opc == crate::isa::op::TRANSFER
        && f.channel != 255
        && crate::intent::unpack(it.r[0]).is_some_and(|(d, o)| d == dom::ACTOR && o > 0)
    {
        f.channel |= EffectFact::CONTACT;
    }
}

/// WORK on ActorSurface is Abrade only; Compress and Heat keep their M0 meaning.
pub(crate) fn is_actor_abrade(it: &Intent) -> bool {
    it.opc == crate::isa::op::WORK && it.sub == crate::intent::sub::WORK_ABRADE && it.r[0] == dom::ACTOR
}
