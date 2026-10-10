//! M1e contact between agents: taking from a bare inventory, abrasion, PushActor (SPEC §20f).
use gene_core::intent::Outcome;
use gene_core::isa::assemble;
use gene_core::sim::{Config, Sim, RULESET_M1D, RULESET_M1E};
use gene_core::species::*;
use gene_core::world::Parcel;

/// Agents act only while their energy is at least 50000; the fixture gives that to agent 0 alone.
const GATE: &str = "
.module 0
    CONST r0, 0
    CONST r3, 0
    READ r9
    CONST r11, 50000
    LTU r9, r11
    BRNZ r9, idle
";

/// Agent 0 at (20, 20) facing north with 60000 energy; agent 1 in front at (20, 19) facing east,
/// with the default 32768. Flat ground, friction 256.
fn pair(body: &str, ruleset: &str) -> Sim {
    let src = format!("{GATE}{body}\nspin:\n    YIELD\n    JUMP spin\nidle:\n    YIELD\n    JUMP idle\n");
    let prog = assemble(&src).unwrap();
    let mut s = Sim::with_program_ruleset(Config { seed: 5, width: 64, height: 64, agents: 2 }, &prog, ruleset).unwrap();
    s.fixture_place(0, 20, 20, 0);
    s.fixture_place(1, 20, 19, 1);
    s.st.agents[0].energy = 60000;
    let w = &mut s.st.world;
    w.elev.iter_mut().for_each(|e| *e = 0);
    w.mu.iter_mut().for_each(|m| *m = 256);
    w.surface_c.iter_mut().for_each(|c| *c = 0);
    s.fixture_rebase();
    s
}

fn step(s: &mut Sim) {
    s.step();
    let a = s.audit();
    assert!(a.ok(&s.st.atoms0), "tick {}: energy diff {}, heat_cap_ok {}", s.st.tick, a.energy - a.expected, a.heat_cap_ok);
}

fn by_uid(s: &Sim, uid: u64) -> Option<&gene_core::agent::Agent> {
    s.st.agents.iter().find(|a| a.uid == uid)
}

fn receipt(s: &Sim, uid: u64) -> Option<(u8, u16)> {
    by_uid(s, uid).and_then(|a| a.receipt)
}

const OK: u8 = Outcome::Ok as u8;

/// Take 5 G from the agent in front into the own inventory, twice.
const TAKE: &str = "
    CONST r0, 0x13
    CONST r1, 0
    CONST r3, 3
    CONST r5, 5
    TRANSFER 0
    TRANSFER 0
    JUMP spin
";

#[test]
fn a_shell_closes_the_inventory() {
    let mut s = pair(TAKE, RULESET_M1E);
    let (me, other) = (s.st.agents[0].uid, s.st.agents[1].uid);
    let g0 = by_uid(&s, other).unwrap().inv[G];
    step(&mut s);
    assert_eq!(receipt(&s, me), Some((Outcome::Blocked as u8, 0)), "shell 2: closed");
    assert_eq!(by_uid(&s, other).unwrap().inv[G], g0);
    // the shell is gone: the inventory is open to contact
    let i = s.st.agents.iter().position(|a| a.uid == other).unwrap();
    s.st.agents[i].shell_s = 0;
    s.fixture_rebase();
    let mine = by_uid(&s, me).unwrap().inv[G];
    step(&mut s);
    assert_eq!(receipt(&s, me), Some((OK, 5)));
    assert_eq!(by_uid(&s, other).unwrap().inv[G], g0 - 5);
    assert_eq!(by_uid(&s, me).unwrap().inv[G], mine + 5);
    assert_eq!(s.obs.m1.taken, 5);
}

#[test]
fn taking_is_off_before_m1e() {
    let mut s = pair(TAKE, RULESET_M1D);
    s.st.agents[1].shell_s = 0;
    s.fixture_rebase();
    let me = s.st.agents[0].uid;
    step(&mut s);
    assert_eq!(receipt(&s, me), Some((Outcome::InvalidTarget as u8, 0)));
}

#[test]
fn energy_is_never_taken() {
    let src = "
    CONST r0, 0x13
    CONST r1, 0
    CONST r3, 15
    CONST r5, 100
    TRANSFER 0
    JUMP spin
";
    let mut s = pair(src, RULESET_M1E);
    s.st.agents[1].shell_s = 0;
    s.fixture_rebase();
    let me = s.st.agents[0].uid;
    step(&mut s);
    assert_eq!(receipt(&s, me), Some((Outcome::InvalidTarget as u8, 0)));
}

#[test]
fn two_pushers_on_one_target_one_is_stale() {
    let src = format!("{GATE}{PUSH}\nspin:\n    YIELD\n    JUMP spin\nidle:\n    YIELD\n    JUMP idle\n");
    let prog = assemble(&src).unwrap();
    let mut s = Sim::with_program_ruleset(Config { seed: 5, width: 64, height: 64, agents: 3 }, &prog, RULESET_M1E).unwrap();
    s.fixture_place(0, 20, 20, 0); // pushes north
    s.fixture_place(1, 20, 19, 0); // the target
    s.fixture_place(2, 19, 19, 1); // pushes east
    s.st.agents[0].energy = 60000;
    s.st.agents[2].energy = 60000;
    s.fixture_rebase();
    let (a, c) = (s.st.agents[0].uid, s.st.agents[2].uid);
    step(&mut s);
    let mut r = [receipt(&s, a).unwrap().0, receipt(&s, c).unwrap().0];
    r.sort();
    assert_eq!(r, [OK, Outcome::Stale as u8], "the second pusher finds someone else in front");
    assert_eq!(s.obs.m1.actor_pushes, 1);
}

/// Abrade the agent in front with r5 q, twice.
fn abrade(q: u32) -> String {
    format!(
        "
    CONST r0, 3
    CONST r1, 0
    CONST r2, -1
    CONST r5, {q}
    WORK 1
    WORK 1
    JUMP spin
"
    )
}

#[test]
fn abrasion_wears_the_shell_then_the_body() {
    let mut s = pair(&abrade(3000), RULESET_M1E);
    let (me, other) = (s.st.agents[0].uid, s.st.agents[1].uid);
    let t = s.st.world.idx(20, 19);
    let (s0, g0) = (s.st.world.loose_total(t, S), s.st.world.loose_total(t, G));
    step(&mut s);
    assert_eq!(receipt(&s, me), Some((OK, 3000)));
    let v = by_uid(&s, other).unwrap();
    assert_eq!((v.shell_s, v.abrasion), (1, 3000 - 2048));
    assert_eq!(s.st.world.loose_total(t, S), s0 + 1, "the worn unit is loose on the target tile");
    // the shell's last unit needs only 1096 more; the rest is not done and not paid
    step(&mut s);
    assert_eq!(receipt(&s, me), Some((Outcome::Partial as u8, 2 * 2048 - 3000)));
    let v = by_uid(&s, other).unwrap();
    assert_eq!((v.shell_s, v.abrasion), (0, 0), "a finished layer leaves no credit");
    assert_eq!(s.st.world.loose_total(t, S), s0 + 2);
    assert_eq!(s.st.world.loose_total(t, G), g0);
    assert_eq!(s.obs.m1.shell_removed, 2);
    assert_eq!(s.obs.m1.abrasion_q, 3000 + 1096);
}

#[test]
fn abrasion_through_the_body_kills() {
    let mut s = pair(&abrade(16384), RULESET_M1E);
    let other = s.st.agents[1].uid;
    s.st.agents[1].shell_s = 0;
    s.st.agents[1].body_g = 9;
    s.fixture_rebase();
    step(&mut s);
    assert_eq!(s.obs.m1.body_removed, 2);
    assert!(by_uid(&s, other).is_none(), "body 7 < 8: dead at stage 10");
    assert_eq!(s.obs.total_deaths_body, 1);
}

#[test]
fn abrading_oneself_or_nobody_is_invalid() {
    let src = "
    CONST r0, 3
    CONST r1, 0
    CONST r2, 0
    CONST r5, 4096
    WORK 1
    CONST r2, 1
    WORK 1
    JUMP spin
";
    let mut s = pair(src, RULESET_M1E);
    let me = s.st.agents[0].uid;
    step(&mut s);
    assert_eq!(receipt(&s, me), Some((Outcome::InvalidTarget as u8, 0)), "own tile");
    step(&mut s);
    assert_eq!(receipt(&s, me), Some((Outcome::InvalidTarget as u8, 0)), "nobody behind");
}

#[test]
fn abrading_an_agent_is_off_before_m1e() {
    let mut s = pair(&abrade(4096), RULESET_M1D);
    let me = s.st.agents[0].uid;
    step(&mut s);
    assert_eq!(receipt(&s, me), Some((Outcome::InvalidTarget as u8, 0)));
    assert_eq!(s.st.agents[1].shell_s, 2);
}

const PUSH: &str = "
    DRIVE 3
    JUMP spin
";

#[test]
fn push_moves_both_agents() {
    let mut s = pair(PUSH, RULESET_M1E);
    let (me, other) = (s.st.agents[0].uid, s.st.agents[1].uid);
    let e0 = by_uid(&s, me).unwrap().energy;
    step(&mut s);
    assert_eq!(receipt(&s, me), Some((OK, 1)));
    let (a, b) = (by_uid(&s, me).unwrap(), by_uid(&s, other).unwrap());
    assert_eq!(((a.x, a.y), (b.x, b.y)), ((20, 19), (20, 18)));
    assert_eq!((a.orient, b.orient), (0, 1), "nobody turns");
    let fric = |m: u64| (m * 256).div_ceil(960);
    let spent = e0 - a.energy;
    assert!(spent as u64 >= 64 + 256 + fric(a.mass()) + fric(b.mass()), "spent {spent}");
    assert_eq!(s.obs.m1.actor_pushes, 1);
}

#[test]
fn push_needs_a_free_tile_beyond() {
    let mut s = pair(PUSH, RULESET_M1E);
    let me = s.st.agents[0].uid;
    let t = s.st.world.idx(20, 18);
    let mut matrix = [0; NSPEC];
    matrix[S] = 16;
    s.st.world.parcels.insert(t, Parcel { matrix, c: 128, ..Parcel::new(1) });
    s.st.next_parcel_uid = 2;
    s.st.world.parcel_changed(t);
    s.fixture_rebase();
    step(&mut s);
    assert_eq!(receipt(&s, me), Some((Outcome::Blocked as u8, 0)));
    assert_eq!((by_uid(&s, me).unwrap().x, by_uid(&s, me).unwrap().y), (20, 20));
}

#[test]
fn push_with_nobody_in_front_is_invalid() {
    let mut s = pair(PUSH, RULESET_M1E);
    s.fixture_place(1, 30, 30, 0);
    s.fixture_rebase();
    let me = s.st.agents[0].uid;
    step(&mut s);
    assert_eq!(receipt(&s, me), Some((Outcome::InvalidTarget as u8, 0)));
}

#[test]
fn pushed_agent_loses_its_pending_move() {
    // the target also acts: it tries to walk east this tick
    let src = "
    DRIVE 3
    JUMP spin
";
    let gate_body = format!(
        "{GATE}{src}\nspin:\n    YIELD\n    JUMP spin\nidle:\n    CONST r5, 0\n    DRIVE 0\nrest:\n    YIELD\n    JUMP rest\n"
    );
    let prog = assemble(&gate_body).unwrap();
    let mut s = Sim::with_program_ruleset(Config { seed: 5, width: 64, height: 64, agents: 2 }, &prog, RULESET_M1E).unwrap();
    s.fixture_place(0, 20, 20, 0);
    s.fixture_place(1, 20, 19, 1);
    s.st.agents[0].energy = 60000;
    s.fixture_rebase();
    let (me, other) = (s.st.agents[0].uid, s.st.agents[1].uid);
    step(&mut s);
    assert_eq!(receipt(&s, me), Some((OK, 1)));
    assert_eq!(receipt(&s, other), Some((Outcome::Stale as u8, 0)));
    let b = by_uid(&s, other).unwrap();
    assert_eq!((b.x, b.y), (20, 18), "pushed, and no step east after it");
}

#[test]
fn drive_3_is_invalid_before_m1e() {
    let mut s = pair(PUSH, RULESET_M1D);
    let me = s.st.agents[0].uid;
    step(&mut s);
    assert_eq!(receipt(&s, me), Some((Outcome::InvalidTarget as u8, 0)));
}

#[test]
fn push_parcel_carries_the_pusher_in_m1e() {
    let src = "
    DRIVE 2
    JUMP spin
";
    let mut s = pair(src, RULESET_M1E);
    s.fixture_place(1, 30, 30, 0);
    let t = s.st.world.idx(20, 19);
    let mut matrix = [0; NSPEC];
    matrix[S] = 16;
    s.st.world.parcels.insert(t, Parcel { matrix, c: 128, ..Parcel::new(1) });
    s.st.next_parcel_uid = 2;
    s.st.world.parcel_changed(t);
    s.fixture_rebase();
    let me = s.st.agents[0].uid;
    step(&mut s);
    assert_eq!(receipt(&s, me), Some((OK, 1)));
    let a = by_uid(&s, me).unwrap();
    assert_eq!((a.x, a.y), (20, 19), "the pusher follows the wall");
    assert!(s.st.world.blocked(s.st.world.idx(20, 18)));
}

#[test]
fn contact_in_a_crowd_keeps_the_ledger_and_matches_parallel() {
    let src = "
.module 0
loop:
    CONST r0, 3
    CONST r1, 0
    CONST r2, -1
    CONST r5, 2048
    WORK 1
    CONST r0, 0x13
    CONST r1, 0
    CONST r3, 3
    CONST r5, 3
    TRANSFER 0
    DRIVE 3
    CONST r5, 1
    DRIVE 1
    CONST r5, 0
    DRIVE 0
    JUMP loop
";
    let prog = assemble(src).unwrap();
    let cfg = Config { seed: 7, width: 64, height: 64, agents: 400 };
    let mut a = Sim::with_program_ruleset(cfg.clone(), &prog, RULESET_M1E).unwrap();
    let mut b = Sim::with_program_ruleset(cfg, &prog, RULESET_M1E).unwrap();
    b.threads = 4;
    b.par_think_min = 1;
    for _ in 0..400 {
        a.step();
        b.step();
        assert!(a.audit_ok(), "tick {}", a.st.tick);
    }
    let m = &a.obs.m1;
    assert!(m.actor_pushes > 0 && m.shell_removed > 0 && m.taken > 0, "{m:?}");
    assert_eq!(a.state_hash(), b.state_hash());
}
