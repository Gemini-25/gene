//! M1f: record media attached to parcels, public selector 0 (SPEC §20g).
use gene_core::intent::Outcome;
use gene_core::isa::assemble;
use gene_core::sim::{Config, Sim, RULESET_M1E, RULESET_M1F};
use gene_core::species::*;
use gene_core::world::Parcel;

/// One agent at (20, 20) facing north, a non-blocking parcel of 4 S in front at (20, 19).
fn sim(body: &str, ruleset: &str) -> Sim {
    let src = format!(".module 0\n{body}\nspin:\n    YIELD\n    JUMP spin\n");
    let prog = assemble(&src).unwrap();
    let mut s = Sim::with_program_ruleset(Config { seed: 5, width: 64, height: 64, agents: 1 }, &prog, ruleset).unwrap();
    s.fixture_place(0, 20, 20, 0);
    s.st.agents[0].energy = 60000;
    let w = &mut s.st.world;
    w.elev.iter_mut().for_each(|e| *e = 0);
    w.mu.iter_mut().for_each(|m| *m = 256);
    let t = s.st.world.idx(20, 19);
    let mut matrix = [0; NSPEC];
    matrix[S] = 4;
    s.st.world.parcels.insert(t, Parcel { matrix, ..Parcel::new(1) });
    s.st.next_parcel_uid = 2;
    s.st.world.parcel_changed(t);
    s.fixture_rebase();
    s
}

fn step(s: &mut Sim) -> Option<(u8, u16)> {
    s.step();
    let a = s.audit();
    assert!(a.ok(&s.st.atoms0), "tick {}: energy diff {}, heat_cap_ok {}", s.st.tick, a.energy - a.expected, a.heat_cap_ok);
    s.st.agents[0].receipt
}

const OK: u8 = Outcome::Ok as u8;

/// Resize the medium of the parcel in front to r5 pages.
const RESIZE: &str = "
    CONST r0, 5
    CONST r1, 0
    CONST r2, -1
    CONST r6, 0
    CONST r5, 1
    WRITE 1
";

fn front(s: &Sim) -> Option<&Parcel> {
    s.st.world.parcels.get(&s.st.world.idx(20, 19))
}

#[test]
fn resize_puts_a_medium_on_the_parcel() {
    let mut s = sim(RESIZE, RULESET_M1F);
    let g0 = s.st.agents[0].inv[G];
    assert_eq!(step(&mut s), Some((OK, 1)));
    let m = front(&s).unwrap().media.as_ref().expect("medium");
    assert_eq!((m.slots(), m.present_pages()), (1, 1));
    assert_eq!(s.st.agents[0].inv[G], g0 - 1, "the page is the actor's G");
    assert_eq!(front(&s).unwrap().matter()[G], 1, "and now part of the parcel");
    assert_eq!(s.obs.m1.parcel_media_created, 1);
}

#[test]
fn parcel_media_are_off_before_m1f() {
    let mut s = sim(RESIZE, RULESET_M1E);
    assert_eq!(step(&mut s), Some((Outcome::InvalidTarget as u8, 0)));
    assert!(front(&s).unwrap().media.is_none());
}

#[test]
fn no_parcel_no_medium() {
    let mut s = sim(RESIZE, RULESET_M1F);
    let t = s.st.world.idx(20, 19);
    s.st.world.parcels.remove(&t);
    s.st.world.parcel_changed(t);
    s.fixture_rebase();
    assert_eq!(step(&mut s), Some((Outcome::InvalidTarget as u8, 0)));
}

#[test]
fn write_then_read_back() {
    let body = format!(
        "{RESIZE}
    CONST r4, 7
    CONST r5, 4242
    WRITE 0
    CONST r3, 0
    READ r9
    CONST r3, 1
    READ r10
    CONST r3, 2
    READ r11
"
    );
    let mut s = sim(&body, RULESET_M1F);
    step(&mut s);
    assert_eq!(step(&mut s), Some((OK, 1)));
    step(&mut s);
    let r = s.st.agents[0].vm.regs;
    assert_eq!((r[9], r[10], r[11]), (4242, 8, 128), "cell 7, logical length 8, capacity 128");
}

#[test]
fn copy_own_code_onto_the_parcel() {
    let body = format!(
        "{RESIZE}
    CONST r0, 6
    CONST r1, 0x15
    CONST r3, 0
    CONST r4, 0
    CONST r5, 0
    CONST r6, 4
    CONST r7, 0
    COPY 0
"
    );
    let mut s = sim(&body, RULESET_M1F);
    step(&mut s);
    assert_eq!(step(&mut s), Some((OK, 4)));
    let code: Vec<u16> = (0..4).map(|k| s.st.agents[0].code.read(k).unwrap()).collect();
    let m = front(&s).unwrap().media.as_ref().unwrap();
    let on: Vec<u16> = (0..4).map(|k| m.read(k).unwrap()).collect();
    assert_eq!(on, code);
    assert_eq!(s.obs.m1.copies_to_public, 1);
}

#[test]
fn a_parcel_medium_holds_eight_pages() {
    let body = RESIZE.replace("CONST r5, 1", "CONST r5, 9");
    let mut s = sim(&body, RULESET_M1F);
    assert_eq!(step(&mut s), Some((Outcome::Limits as u8, 0)));
}

#[test]
fn the_medium_outlives_its_parcel() {
    // take the parcel's whole matrix: it vanishes and its medium stays on the tile
    let body = format!(
        "{RESIZE}
    CONST r0, 0x12
    CONST r1, 0
    CONST r3, 7
    CONST r5, 4
    TRANSFER 0
"
    );
    let mut s = sim(&body, RULESET_M1F);
    step(&mut s);
    let uid = front(&s).unwrap().media.as_ref().unwrap().uid;
    assert_eq!(step(&mut s), Some((OK, 4)));
    assert!(front(&s).is_none(), "the parcel is gone");
    let t = s.st.world.idx(20, 19);
    let list = s.st.dropped.get(&t).expect("dropped record");
    assert_eq!(list.iter().map(|m| m.uid).collect::<Vec<_>>(), vec![uid]);
    assert_eq!(s.obs.m1.parcel_media_dropped, 1);
}

#[test]
fn the_medium_moves_with_the_parcel() {
    let body = format!(
        "{RESIZE}
    DRIVE 2
"
    );
    let mut s = sim(&body, RULESET_M1F);
    step(&mut s);
    let uid = front(&s).unwrap().media.as_ref().unwrap().uid;
    assert_eq!(step(&mut s), Some((OK, 1)));
    let moved = s.st.world.parcels.get(&s.st.world.idx(20, 18)).expect("pushed parcel");
    assert_eq!(moved.media.as_ref().map(|m| m.uid), Some(uid));
}

#[test]
fn records_on_parcels_keep_the_ledger_and_match_parallel() {
    // put S down as a parcel in front, give it a medium, write on it, carry on
    let src = "
.module 0
loop:
    CONST r0, 0
    CONST r1, 0x12
    CONST r3, 7
    CONST r5, 2
    TRANSFER 0
    CONST r0, 5
    CONST r1, 0
    CONST r2, -1
    CONST r6, 0
    CONST r5, 2
    WRITE 1
    RANDOM r4
    CONST r8, 255
    AND r4, r8
    RANDOM r5
    WRITE 0
    CONST r5, 1
    DRIVE 2
    DRIVE 1
    CONST r5, 0
    DRIVE 0
    JUMP loop
";
    let prog = assemble(src).unwrap();
    let cfg = Config { seed: 9, width: 64, height: 64, agents: 200 };
    let mut a = Sim::with_program_ruleset(cfg.clone(), &prog, RULESET_M1F).unwrap();
    let mut b = Sim::with_program_ruleset(cfg, &prog, RULESET_M1F).unwrap();
    b.threads = 4;
    b.par_think_min = 1;
    for k in 0..600 {
        a.step();
        b.step();
        assert!(a.audit_ok(), "tick {}", a.st.tick);
        if k % 100 == 0 {
            assert_eq!(a.state_hash(), b.state_hash(), "tick {}", a.st.tick);
        }
    }
    let m = &a.obs.m1;
    assert!(m.parcel_media_created > 0 && m.public_writes > 0, "{m:?}");
    assert_eq!(a.state_hash(), b.state_hash());
}

#[test]
fn a_parcel_medium_oxidizes_like_any_exposed_record() {
    let mut s = sim(RESIZE, RULESET_M1F);
    step(&mut s);
    let t = s.st.world.idx(20, 19);
    // one credit short of a reaction, with oxygen on the tile
    s.st.world.parcels.get_mut(&t).unwrap().media.as_mut().unwrap().credit = 65535;
    if s.st.world.loose_total(t, O) < 4 {
        s.st.world.put_loose(t, O, 4);
    }
    s.fixture_rebase();
    for _ in 0..16 {
        step(&mut s); // one environment due per 16 ticks
    }
    assert!(front(&s).unwrap().media.is_none(), "its only page became C and W");
    assert_eq!(s.obs.acc.media_oxidized, 1, "G1 + O1 -> C1 + W1 (C and W then spread as gases)");
    assert!(!s.st.dropped.contains_key(&t), "nothing left to drop");
}
