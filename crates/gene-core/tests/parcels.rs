//! M1c parcels (SPEC §20d).
use gene_core::intent::{Fault, Outcome};
use gene_core::isa::assemble;
use gene_core::sim::{Config, Sim, RULESET_M1B, RULESET_M1C};
use gene_core::species::*;

fn sim(src: &str, ruleset: &str) -> Sim {
    let prog = assemble(src).unwrap();
    let mut s = Sim::with_program_ruleset(Config { seed: 5, width: 64, height: 64, agents: 1 }, &prog, ruleset).unwrap();
    s.fixture_place(0, 20, 20, 0);
    s.st.agents[0].inv[S] = 24;
    s.fixture_rebase();
    s
}

fn run_audited(s: &mut Sim, n: usize) -> Vec<Option<u8>> {
    let mut receipts = vec![];
    for _ in 0..n {
        s.step();
        let a = s.audit();
        assert!(a.ok(&s.st.atoms0), "tick {}: energy diff {}, heat_cap_ok {}", s.st.tick, a.energy - a.expected, a.heat_cap_ok);
        receipts.push(s.st.agents[0].receipt.map(|r| r.0));
    }
    receipts
}

const BUILDER: &str = "
.module 0
    CONST r0, 0
    CONST r1, 0x12
    CONST r3, 7
    CONST r5, 4
    TRANSFER 0
    CONST r0, 2
    CONST r1, 0
    CONST r2, -1
    CONST r5, 6400
    WORK 0
    CONST r0, 0
    CONST r1, 0x1A
    CONST r3, 3
    CONST r5, 3
    TRANSFER 0
spin:
    YIELD
    JUMP spin
";

#[test]
fn build_compress_and_store() {
    let mut s = sim(BUILDER, RULESET_M1C);
    let receipts = run_audited(&mut s, 4);
    assert_eq!(&receipts[..3], &[Some(0), Some(0), Some(0)], "{receipts:?}");
    let front = s.st.world.idx(20, 19);
    let p = &s.st.world.parcels[&front];
    assert_eq!(p.matrix[S], 4);
    assert_eq!(p.c, 100);
    assert_eq!(p.pore[G], 3);
    assert_eq!(s.st.world.perm[front as usize], 256 - 4 * 100 / 64);
    assert!(s.st.world.expo[front as usize] < 256, "exposure follows permeability");
    // natural exchange brings oxygen into the pore over a few due ticks
    run_audited(&mut s, 64);
    assert!(s.st.world.parcels[&front].pore[O] > 0);
}

#[test]
fn parcels_are_off_before_m1c() {
    let mut s = sim(BUILDER, RULESET_M1B);
    let receipts = run_audited(&mut s, 3);
    assert_eq!(receipts[0], Some(Outcome::InvalidTarget as u8));
    assert!(s.st.world.parcels.is_empty());
}

const BLOCKER: &str = "
.module 0
    CONST r0, 0
    CONST r1, 0x12
    CONST r3, 7
    CONST r5, 16
    TRANSFER 0
    CONST r0, 2
    CONST r1, 0
    CONST r2, -1
    CONST r5, 8192
    WORK 0
    CONST r5, 0
    DRIVE 0
    CONST r0, 1
    CONST r1, 0
    CONST r2, -2
    CONST r3, 0
    CONST r4, 0
    READ r8
spin:
    YIELD
    JUMP spin
";

#[test]
fn a_dense_parcel_blocks_movement_and_sight() {
    let mut s = sim(BLOCKER, RULESET_M1C);
    let receipts = run_audited(&mut s, 4);
    let front = s.st.world.idx(20, 19);
    let p = &s.st.world.parcels[&front];
    assert_eq!((p.matrix[S], p.c), (16, 128));
    assert!(s.st.world.blocked(front));
    assert_eq!(receipts[2], Some(Outcome::Blocked as u8), "cannot walk into a wall");
    assert_eq!((s.st.agents[0].x, s.st.agents[0].y), (20, 20));
    assert_eq!(s.st.agents[0].vm.fault, Fault::Occluded as u8, "cannot see through a wall");
}

#[test]
fn emptying_the_matrix_removes_the_parcel_and_returns_the_pore() {
    let src = "
.module 0
    CONST r0, 0
    CONST r1, 0x12
    CONST r3, 7
    CONST r5, 4
    TRANSFER 0
    CONST r1, 0x1A
    CONST r3, 3
    CONST r5, 2
    TRANSFER 0
    CONST r0, 0x12
    CONST r1, 0
    CONST r3, 7
    CONST r5, 4
    TRANSFER 0
spin:
    YIELD
    JUMP spin
";
    let mut s = sim(src, RULESET_M1C);
    let front = s.st.world.idx(20, 19);
    let g0 = s.st.world.loose_total(front, G);
    let receipts = run_audited(&mut s, 4);
    assert_eq!(&receipts[..3], &[Some(0), Some(0), Some(0)], "{receipts:?}");
    assert!(!s.st.world.parcels.contains_key(&front));
    assert_eq!(s.st.world.loose_total(front, G), g0 + 2, "pore contents fall back to the loose stock");
    assert_eq!(s.st.world.perm[front as usize], 256);
}

#[test]
fn compressed_matrix_cannot_be_taken_apart() {
    let src = "
.module 0
    CONST r0, 0
    CONST r1, 0x12
    CONST r3, 7
    CONST r5, 4
    TRANSFER 0
    CONST r0, 2
    CONST r1, 0
    CONST r2, -1
    CONST r5, 4096
    WORK 0
    CONST r0, 0x12
    CONST r1, 0
    CONST r3, 7
    CONST r5, 4
    TRANSFER 0
spin:
    YIELD
    JUMP spin
";
    let mut s = sim(src, RULESET_M1C);
    let receipts = run_audited(&mut s, 4);
    assert_eq!(receipts[2], Some(Outcome::Blocked as u8), "c >= 64 locks the matrix");
}

#[test]
fn parallel_with_parcels_matches_serial() {
    let prog = assemble(BUILDER).unwrap();
    let cfg = Config { seed: 6, width: 128, height: 128, agents: 40 };
    let mut a = Sim::with_program_ruleset(cfg.clone(), &prog, RULESET_M1C).unwrap();
    let mut b = Sim::with_program_ruleset(cfg, &prog, RULESET_M1C).unwrap();
    b.threads = 4;
    b.par_think_min = 1;
    for _ in 0..600 {
        a.step();
        b.step();
    }
    assert!(!a.st.world.parcels.is_empty());
    assert_eq!(a.state_hash(), b.state_hash());
    assert!(b.audit_ok());
}
