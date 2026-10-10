//! M1d bonds, anchors, grip and assembly motion (SPEC §20e).
use gene_core::intent::Outcome;
use gene_core::isa::assemble;
use gene_core::sim::{Config, Sim, RULESET_M1C, RULESET_M1D};
use gene_core::species::*;
use gene_core::world::Parcel;

/// One agent at (20, 20) facing north on a flat world (elevation 0, friction 256), so parcel
/// demand is ceil(mass * distance * 256 / 960): 256 per tile for a 16-unit S matrix.
fn sim(src: &str, ruleset: &str) -> Sim {
    let prog = assemble(src).unwrap();
    let mut s = Sim::with_program_ruleset(Config { seed: 5, width: 64, height: 64, agents: 1 }, &prog, ruleset).unwrap();
    s.fixture_place(0, 20, 20, 0);
    s.st.agents[0].inv[S] = 24;
    s.st.agents[0].energy = 60000;
    let w = &mut s.st.world;
    w.elev.iter_mut().for_each(|e| *e = 0);
    w.mu.iter_mut().for_each(|m| *m = 256);
    w.surface_c.iter_mut().for_each(|c| *c = 0);
    s.fixture_rebase();
    s
}

/// Put a parcel with `n` units of S at (x, y).
fn put(s: &mut Sim, x: u32, y: u32, n: u16, c: u8) -> u32 {
    let t = s.st.world.idx(x, y);
    let uid = s.st.next_parcel_uid;
    s.st.next_parcel_uid += 1;
    let mut matrix = [0; NSPEC];
    matrix[S] = n;
    s.st.world.parcels.insert(t, Parcel { matrix, c, ..Parcel::new(uid) });
    s.st.world.parcel_changed(t);
    s.fixture_rebase();
    t
}

/// Bond the parcel at t to its neighbour across absolute face f.
fn bond(s: &mut Sim, t: u32, f: u8, v: u8) {
    let nb = s.st.world.neighbor(t, f).unwrap();
    s.st.world.parcels.get_mut(&t).unwrap().bond[f as usize] = v;
    s.st.world.parcels.get_mut(&nb).unwrap().bond[(f as usize + 2) & 3] = v;
}

fn run(s: &mut Sim, n: usize) -> Vec<Option<(u8, u16)>> {
    let mut receipts = vec![];
    for _ in 0..n {
        s.step();
        let a = s.audit();
        assert!(a.ok(&s.st.atoms0), "tick {}: energy diff {}, heat_cap_ok {}", s.st.tick, a.energy - a.expected, a.heat_cap_ok);
        receipts.push(s.st.agents[0].receipt);
    }
    receipts
}

const OK: u8 = Outcome::Ok as u8;

fn at(s: &Sim, x: u32, y: u32) -> Option<&Parcel> {
    s.st.world.parcels.get(&s.st.world.idx(x, y))
}

#[test]
fn face_bond_costs_64_per_unit_and_stops_at_cohesion() {
    let src = "
.module 0
    CONST r0, 2
    CONST r1, 0
    CONST r2, 0
    CONST r4, 0
    CONST r5, 100
    LINK 0
    CONST r5, 6
    UNLINK 0
spin:
    YIELD
    JUMP spin
";
    let mut s = sim(src, RULESET_M1D);
    put(&mut s, 20, 20, 4, 0); // under the agent: cohesion 4 * 4 = 16
    put(&mut s, 20, 19, 8, 0); // in front: cohesion 32
    let e0 = s.st.agents[0].energy;
    let r = run(&mut s, 1);
    assert_eq!(r[0], Some((Outcome::Partial as u8, 16)), "limited by the weaker side");
    assert_eq!(at(&s, 20, 20).unwrap().bond[0], 16);
    assert_eq!(at(&s, 20, 19).unwrap().bond[2], 16, "bonds are symmetric");
    let spent = e0 - s.st.agents[0].energy;
    assert!(spent >= 64 + 16 * 64, "attempt + 16 units, spent {spent}");
    let r = run(&mut s, 1);
    assert_eq!(r[0], Some((OK, 6)));
    assert_eq!(at(&s, 20, 19).unwrap().bond[2], 10);
}

#[test]
fn links_are_off_before_m1d() {
    let src = "
.module 0
    CONST r0, 2
    CONST r4, 4
    CONST r5, 1
    LINK 0
spin:
    YIELD
    JUMP spin
";
    let mut s = sim(src, RULESET_M1C);
    put(&mut s, 20, 20, 4, 0);
    let r = run(&mut s, 1);
    assert_eq!(r[0], Some((Outcome::InvalidTarget as u8, 0)));
}

#[test]
fn face_bond_needs_both_ends_within_reach() {
    // the target is in front, its partner two tiles ahead: out of reach
    let src = "
.module 0
    CONST r0, 2
    CONST r1, 0
    CONST r2, -1
    CONST r4, 0
    CONST r5, 4
    LINK 0
spin:
    YIELD
    JUMP spin
";
    let mut s = sim(src, RULESET_M1D);
    put(&mut s, 20, 19, 4, 0);
    put(&mut s, 20, 18, 4, 0);
    let r = run(&mut s, 1);
    assert_eq!(r[0], Some((Outcome::InvalidTarget as u8, 0)));
}

const GRIP_AND_MOVE: &str = "
.module 0
    CONST r0, 2
    CONST r1, 0
    CONST r2, -1
    CONST r4, 5
    CONST r5, 255
    LINK 0
    CONST r5, 1
    DRIVE 0
spin:
    YIELD
    JUMP spin
";

/// Agent at (20, 20); A in front at (20, 19); B north of A at (20, 18), bonded with `b`.
fn two_parcels(b: u8) -> Sim {
    let mut s = sim(GRIP_AND_MOVE, RULESET_M1D);
    let a = put(&mut s, 20, 19, 16, 0);
    put(&mut s, 20, 18, 16, 0);
    bond(&mut s, a, 0, b);
    s
}

#[test]
fn grip_carries_the_bonded_assembly() {
    let mut s = two_parcels(16);
    let (ua, ub) = (at(&s, 20, 19).unwrap().uid, at(&s, 20, 18).unwrap().uid);
    let r = run(&mut s, 2);
    assert_eq!(r[0], Some((Outcome::Partial as u8, 64)), "grip limited by cohesion 64");
    assert_eq!(r[1], Some((OK, 2)), "two parcels moved");
    let a = &s.st.agents[0];
    assert_eq!((a.x, a.y, a.orient), (21, 20, 0));
    assert_eq!(at(&s, 21, 19).unwrap().uid, ua);
    assert_eq!(at(&s, 21, 18).unwrap().uid, ub);
    assert!(at(&s, 20, 19).is_none() && at(&s, 20, 18).is_none());
    assert_eq!(at(&s, 21, 19).unwrap().grip_uid, a.uid, "the grip moves along");
    assert_eq!(at(&s, 21, 19).unwrap().bond[0], 16);
    assert_eq!(s.obs.m1.assembly_moves, 1);
}

#[test]
fn a_weak_bond_cannot_carry_its_load() {
    // B needs 256; a bond of 15 carries 240
    let mut s = two_parcels(15);
    let r = run(&mut s, 2);
    assert_eq!(r[1], Some((Outcome::Blocked as u8, 0)));
    assert_eq!((s.st.agents[0].x, s.st.agents[0].y), (20, 20));
    assert!(at(&s, 20, 18).is_some());
    assert_eq!(s.obs.m1.load_failures, 1);
}

#[test]
fn an_anchor_holds_the_whole_assembly() {
    let mut s = two_parcels(16);
    s.st.world.parcels.get_mut(&s.st.world.idx(20, 18)).unwrap().anchor = 1;
    let r = run(&mut s, 2);
    assert_eq!(r[1], Some((Outcome::Blocked as u8, 0)));
    assert_eq!(s.obs.m1.load_failures, 0, "refused before the load graph");
}

#[test]
fn rotation_turns_positions_bonds_and_orientation() {
    let src = "
.module 0
    CONST r0, 2
    CONST r1, 0
    CONST r2, -1
    CONST r4, 5
    CONST r5, 255
    LINK 0
    CONST r5, 1
    DRIVE 1
spin:
    YIELD
    JUMP spin
";
    let mut s = sim(src, RULESET_M1D);
    let a = put(&mut s, 20, 19, 16, 0);
    put(&mut s, 20, 18, 16, 0);
    bond(&mut s, a, 0, 32); // B moves two tiles: demand 512
    let ub = at(&s, 20, 18).unwrap().uid;
    let r = run(&mut s, 2);
    assert_eq!(r[1], Some((OK, 2)));
    // clockwise about A: the agent goes from south to west of A, B from north to east
    let ag = &s.st.agents[0];
    assert_eq!((ag.x, ag.y, ag.orient), (19, 19, 1));
    let pa = at(&s, 20, 19).unwrap();
    let pb = at(&s, 21, 19).unwrap();
    assert_eq!(pb.uid, ub);
    assert_eq!((pa.bond, pb.bond), ([0, 32, 0, 0], [0, 0, 0, 32]));
    assert_eq!((pa.orient, pb.orient), (1, 1));
    assert_eq!(pa.grip_uid, ag.uid, "still holding A, now in front again");
}

const PUSH: &str = "
.module 0
    CONST r5, 0
    DRIVE 2
    DRIVE 0
spin:
    YIELD
    JUMP spin
";

#[test]
fn push_moves_a_wall_and_frees_the_way() {
    let mut s = sim(PUSH, RULESET_M1D);
    put(&mut s, 20, 19, 16, 128);
    assert!(s.st.world.blocked(s.st.world.idx(20, 19)));
    let r = run(&mut s, 2);
    assert_eq!(r[0], Some((OK, 1)));
    assert!(s.st.world.blocked(s.st.world.idx(20, 18)));
    assert_eq!(r[1], Some((OK, 1)), "the agent steps into the freed tile");
    assert_eq!((s.st.agents[0].x, s.st.agents[0].y), (20, 19));
    assert_eq!(s.obs.m1.pushes, 1);
}

#[test]
fn push_stops_at_another_parcel_and_at_17() {
    let mut s = sim(PUSH, RULESET_M1D);
    put(&mut s, 20, 19, 16, 0);
    put(&mut s, 20, 18, 4, 0); // not bonded: in the way
    let r = run(&mut s, 1);
    assert_eq!(r[0], Some((Outcome::Blocked as u8, 0)));

    let mut s = sim(PUSH, RULESET_M1D);
    let mut prev = None;
    for y in (2..=19).rev() {
        let t = put(&mut s, 20, y, 1, 0);
        if let Some(p) = prev {
            bond(&mut s, p, 0, 1);
        }
        prev = Some(t);
    }
    let r = run(&mut s, 1);
    assert_eq!(r[0], Some((Outcome::Limits as u8, 0)), "18 bonded parcels exceed 16");
}

#[test]
fn losing_matrix_weakens_and_removing_breaks_bonds() {
    // take 2 of 4 S out of the front parcel: its cohesion falls from 16 to 8
    let src = "
.module 0
    CONST r0, 0x12
    CONST r1, 0
    CONST r3, 7
    CONST r5, 2
    TRANSFER 0
    CONST r0, 0x02
    CONST r5, 1
    TRANSFER 0
spin:
    YIELD
    JUMP spin
";
    let mut s = sim(src, RULESET_M1D);
    let under = put(&mut s, 20, 20, 1, 0); // cohesion 4
    let front = put(&mut s, 20, 19, 4, 0);
    let back = put(&mut s, 20, 21, 4, 0);
    s.st.world.parcels.get_mut(&front).unwrap().anchor = 16;
    bond(&mut s, under, 2, 4);
    let r = run(&mut s, 1);
    assert_eq!(r[0], Some((OK, 2)));
    assert_eq!(s.st.world.parcels[&front].anchor, 8, "anchor clamped to the new cohesion");
    let r = run(&mut s, 1);
    assert_eq!(r[0], Some((OK, 1)));
    assert!(!s.st.world.parcels.contains_key(&under));
    assert_eq!(s.st.world.parcels[&back].bond, [0; 4], "partner of a vanished parcel is free");
}

#[test]
fn parallel_with_assemblies_matches_serial() {
    let src = "
.module 0
loop:
    CONST r0, 0
    CONST r1, 0x12
    CONST r3, 7
    CONST r5, 4
    TRANSFER 0
    CONST r0, 2
    CONST r1, 0
    CONST r2, -1
    CONST r4, 5
    CONST r5, 255
    LINK 0
    CONST r5, 1
    DRIVE 0
    DRIVE 1
    CONST r5, 255
    UNLINK 0
    DRIVE 2
    CONST r0, 0x12
    CONST r1, 0
    CONST r3, 7
    CONST r5, 4
    TRANSFER 0
    JUMP loop
";
    let prog = assemble(src).unwrap();
    let cfg = Config { seed: 6, width: 128, height: 128, agents: 40 };
    let mut a = Sim::with_program_ruleset(cfg.clone(), &prog, RULESET_M1D).unwrap();
    let mut b = Sim::with_program_ruleset(cfg, &prog, RULESET_M1D).unwrap();
    b.threads = 4;
    for _ in 0..600 {
        a.step();
        b.step();
        assert!(a.audit_ok(), "tick {}", a.st.tick);
    }
    assert!(a.obs.m1.assembly_moves > 0 && a.obs.m1.pushes > 0, "{:?}", a.obs.m1);
    assert_eq!(a.state_hash(), b.state_hash());
}

#[test]
fn sixteen_parcels_move_as_one() {
    let mut s = sim(PUSH, RULESET_M1D);
    // near-frictionless ground: each 1-unit parcel needs a load of 1
    s.st.world.mu.iter_mut().for_each(|m| *m = 1);
    let mut prev = None;
    for y in (4..=19).rev() {
        let t = put(&mut s, 20, y, 1, 0);
        if let Some(p) = prev {
            bond(&mut s, p, 0, 1);
        }
        prev = Some(t);
    }
    let r = run(&mut s, 1);
    assert_eq!(r[0], Some((OK, 16)));
    assert!(at(&s, 20, 3).is_some() && at(&s, 20, 19).is_none());
}

#[test]
fn a_wall_is_not_pushed_onto_another_agent() {
    let prog = assemble(PUSH).unwrap();
    let mut s = Sim::with_program_ruleset(Config { seed: 5, width: 64, height: 64, agents: 2 }, &prog, RULESET_M1D).unwrap();
    s.fixture_place(0, 20, 20, 0);
    s.fixture_place(1, 20, 18, 2); // faces south, toward the wall
    s.st.agents[0].energy = 60000;
    s.fixture_rebase();
    put(&mut s, 20, 19, 16, 128);
    s.step();
    assert!(s.audit_ok());
    let pusher = s.st.agents.iter().find(|a| (a.x, a.y) == (20, 20)).unwrap();
    assert_eq!(pusher.receipt.map(|r| r.0), Some(Outcome::Blocked as u8));
    assert!(s.st.world.blocked(s.st.world.idx(20, 19)), "the wall stays");
}

#[test]
fn rotation_does_not_swing_a_wall_into_an_agent() {
    let src = "
.module 0
    CONST r0, 2
    CONST r1, 0
    CONST r2, -1
    CONST r4, 5
    CONST r5, 255
    LINK 0
    CONST r5, 1
    DRIVE 1
spin:
    YIELD
    JUMP spin
";
    let prog = assemble(src).unwrap();
    let mut s = Sim::with_program_ruleset(Config { seed: 5, width: 64, height: 64, agents: 2 }, &prog, RULESET_M1D).unwrap();
    s.fixture_place(0, 20, 20, 0);
    s.fixture_place(1, 21, 19, 1); // where the wall would land
    s.st.agents[0].energy = 60000;
    let w = &mut s.st.world;
    w.elev.iter_mut().for_each(|e| *e = 0);
    w.mu.iter_mut().for_each(|m| *m = 256);
    s.fixture_rebase();
    let a = put(&mut s, 20, 19, 16, 0);
    put(&mut s, 20, 18, 16, 128);
    bond(&mut s, a, 0, 32);
    let uid0 = s.st.agents[0].uid;
    for _ in 0..2 {
        s.step();
        assert!(s.audit_ok());
    }
    let me = s.st.agents.iter().find(|a| a.uid == uid0).unwrap();
    assert_eq!(me.receipt.map(|r| r.0), Some(Outcome::Blocked as u8));
    assert!(at(&s, 20, 18).is_some(), "nothing moved");
}

#[test]
fn death_releases_the_grip() {
    let src = "
.module 0
    CONST r0, 2
    CONST r1, 0
    CONST r2, -1
    CONST r4, 5
    CONST r5, 10
    LINK 0
spin:
    YIELD
    JUMP spin
";
    let mut s = sim(src, RULESET_M1D);
    let t = put(&mut s, 20, 19, 16, 0);
    run(&mut s, 1);
    assert_eq!(s.st.world.parcels[&t].grip, 10);
    s.st.agents[0].energy = 0;
    s.fixture_rebase();
    s.step();
    assert!(s.st.agents.is_empty(), "starved");
    assert!(s.audit_ok());
    assert_eq!((s.st.world.parcels[&t].grip, s.st.world.parcels[&t].grip_uid), (0, 0));
}
