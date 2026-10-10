//! M1a: signals, public records, drafts (SPEC §20b).
use gene_core::intent::{Fault, Outcome};
use gene_core::isa::assemble;
use gene_core::media::Media;
use gene_core::sim::{Config, Sim, RULESET, RULESET_M1};

fn sim(src: &str, agents: u32, ruleset: &str) -> Sim {
    let prog = assemble(src).unwrap();
    Sim::with_program_ruleset(Config { seed: 3, width: 64, height: 64, agents }, &prog, ruleset).unwrap()
}

fn step_audited(s: &mut Sim, n: usize) {
    for _ in 0..n {
        s.step();
        let a = s.audit();
        assert!(a.ok(&s.st.atoms0), "tick {}: energy diff {}, heat_cap_ok {}", s.st.tick, a.energy - a.expected, a.heat_cap_ok);
    }
}

const EMIT_AND_LISTEN: &str = "
.module 0
    CONST r3, 1
    CONST r5, 100
    EMIT 0
    CONST r0, 4
    CONST r1, 0
    CONST r2, 0
    CONST r3, 0
    CONST r4, 1
    READ r8
    CONST r1, 1
    READ r9
spin:
    YIELD
    JUMP spin
";

#[test]
fn emit_reaches_own_tile_and_neighbours_next_tick() {
    let mut s = sim(EMIT_AND_LISTEN, 1, RULESET_M1);
    s.st.agents[0].inv = [0; 10];
    s.fixture_place(0, 20, 20, 0);
    s.fixture_rebase();
    let e0 = s.st.agents[0].energy;
    let m = s.st.agents[0].upkeep();
    step_audited(&mut s, 1);
    // 3 instructions + attempt 128 + 2*|100|
    assert_eq!(e0 - s.st.agents[0].energy, 3 * 4 + 128 + 200 + m);
    step_audited(&mut s, 1);
    let r = s.st.agents[0].vm.regs;
    assert_eq!((r[8], r[9]), (100, 50));
    step_audited(&mut s, 1);
    assert!(s.st.signal.is_empty(), "no conduction: the field decays when nobody emits");
}

#[test]
fn emit_is_off_in_m0() {
    let mut s = sim(EMIT_AND_LISTEN, 1, RULESET);
    s.step();
    assert_eq!(s.st.agents[0].receipt.map(|r| r.0), Some(Outcome::InvalidTarget as u8));
    s.step();
    assert_eq!(s.st.agents[0].vm.regs[8], 0);
    assert!(s.st.signal.is_empty());
}

/// Draft creation, own code -> draft, cancel (becomes a public record), public resize and write,
/// public -> new draft, embryo, draft -> embryo, public shrink.
const RECORDS: &str = "
.module 0
    CONST r0, 7
    CONST r5, 1
    WRITE 1             ; draft: first page
    CONST r0, 6
    CONST r1, 7
    CONST r4, 0
    CONST r5, 0
    CONST r6, 4
    CONST r7, 0
    COPY 0              ; own code -> draft
    WRITE 3             ; cancel draft -> public record on own tile
    CONST r0, 5
    CONST r1, 0
    CONST r2, 0
    CONST r6, 1
    CONST r5, 2
    WRITE 1             ; public record: second page
    CONST r4, 5
    CONST r5, 777
    WRITE 0             ; public cell 5 = 777
    CONST r0, 7
    CONST r5, 1
    WRITE 1             ; new draft
    CONST r0, 5
    CONST r1, 7
    CONST r4, 4
    CONST r5, 0
    CONST r6, 4
    CONST r7, 1
    COPY 0              ; public[4..8) -> draft[0..4)
    REPRO 0             ; allocate embryo
    CONST r5, 1
    REPRO 1             ; embryo page
    CONST r0, 7
    CONST r1, 8
    CONST r4, 0
    CONST r5, 0
    CONST r6, 4
    COPY 0              ; draft -> embryo
    CONST r0, 5
    CONST r1, 0
    CONST r2, 0
    CONST r6, 1
    CONST r5, 1
    WRITE 1             ; public shrink: recycle 4 cells of the last page
spin:
    YIELD
    JUMP spin
";

#[test]
fn record_lifecycle_conserves_and_tracks_provenance() {
    let mut s = sim(RECORDS, 1, RULESET_M1);
    let mut receipts = vec![];
    for _ in 0..16 {
        step_audited(&mut s, 1);
        receipts.push(s.st.agents[0].receipt.map(|r| r.0));
    }
    let a = &s.st.agents[0];
    assert!(receipts.iter().all(|r| *r != Some(Outcome::InvalidTarget as u8)), "{receipts:?}");
    let t = s.st.world.idx(a.x as u32, a.y as u32);
    let public = &s.st.dropped[&t][0];
    assert_eq!(public.read(5), Some(777));
    assert_eq!(public.present_pages(), 2);
    assert_eq!(public.recycle, Some((1, 4)), "shrink cursor advanced by 4 cells");
    let draft = a.draft.as_ref().expect("second draft");
    assert!(draft.foreign, "draft holds public bytes");
    let raw = &a.embryo.as_ref().unwrap().raw;
    assert!(raw.foreign, "embryo got public bytes via the draft");
    assert_eq!(raw.read(1), public.read(5));
    assert_eq!(s.obs.m1.copies_from_public, 1);
    assert_eq!(s.obs.m1.draft_to_embryo, 1);
}

#[test]
fn growing_a_record_on_another_tile_moves_matter_heat_and_height() {
    let src = "
.module 0
    CONST r0, 5
    CONST r1, 0
    CONST r2, -1
    CONST r6, 1
    CONST r5, 2
    WRITE 1
spin:
    YIELD
    JUMP spin
";
    let mut s = sim(src, 1, RULESET_M1);
    // find a tile whose northern neighbour sits higher
    let w = &s.st.world;
    let (mut x0, mut y0) = (0, 0);
    'find: for y in 1..63 {
        for x in 0..64 {
            let t = w.idx(x, y);
            if w.elev[(t - 64) as usize] > w.elev[t as usize] {
                (x0, y0) = (x, y);
                break 'find;
            }
        }
    }
    assert!(y0 > 0, "terrain has a slope");
    s.fixture_place(0, x0 as u16, y0 as u16, 0);
    let front = s.st.world.idx(x0, y0 - 1);
    s.st.dropped.insert(front, vec![Media::from_cells(999, &[1, 2, 3])]);
    s.fixture_rebase();
    let g0 = s.st.agents[0].inv[3];
    step_audited(&mut s, 1);
    assert_eq!(s.st.agents[0].receipt.map(|r| r.0), Some(Outcome::Ok as u8));
    step_audited(&mut s, 1);
    assert_eq!(s.st.agents[0].inv[3], g0 - 1);
    assert_eq!(s.st.dropped[&front][0].present_pages(), 2);
}

#[test]
fn copy_from_a_public_record_uses_a_sensing_query() {
    let src = "
.module 0
    CONST r0, 1
    READ r8
    READ r8
    READ r8
    READ r8
    CONST r0, 5
    CONST r1, 7
    CONST r6, 4
    CONST r7, 1
    COPY 0
spin:
    YIELD
    JUMP spin
";
    let mut s = sim(src, 1, RULESET_M1);
    let t = s.tile_of(0);
    s.st.dropped.insert(t, vec![Media::from_cells(999, &[1, 2, 3, 4])]);
    s.fixture_rebase();
    s.step();
    assert_eq!(s.st.agents[0].vm.fault, Fault::QueryLimit as u8);
    assert!(s.st.agents[0].receipt.is_none(), "no effect submitted");
}

#[test]
fn exposed_records_wear_one_bit_per_clock_period() {
    let mut s = sim(".module 0\nspin: YIELD\n JUMP spin\n", 1, RULESET_M1);
    let t = s.tile_of(0) ^ 1; // a neighbouring tile in the same row
    let mut m = Media::from_cells(999, &[0; 128]);
    m.exposure_clock = 16383;
    s.st.dropped.insert(t, vec![m]);
    s.fixture_rebase();
    step_audited(&mut s, 16); // every tile is due once in 16 ticks
    let m = &s.st.dropped[&t][0];
    assert_eq!(m.wear_events, 1);
    assert_eq!(m.cells.iter().map(|c| c.count_ones()).sum::<u32>(), 1);
}

#[test]
fn founder6_runs_identically_under_both_rulesets_until_mutation_matters() {
    // Founders never use M1a primitives, so the first births are identical.
    let prog = assemble(gene_core::FOUNDER6_ASM).unwrap();
    let cfg = Config { seed: 4, width: 64, height: 64, agents: 8 };
    let mut a = Sim::with_program_ruleset(cfg.clone(), &prog, RULESET).unwrap();
    let mut b = Sim::with_program_ruleset(cfg, &prog, RULESET_M1).unwrap();
    for _ in 0..1500 {
        a.step();
        b.step();
    }
    let pos = |s: &Sim| s.st.agents.iter().map(|x| (x.uid, x.x, x.y, x.energy)).collect::<Vec<_>>();
    assert_eq!(pos(&a), pos(&b));
}

// ---------------------------------------------------------------- M1b: learned binding

/// Learner: copies public record #(phase+1) into a draft, commits it, then RETs into the new entry.
const LEARNER: &str = "
.module 0
    CONST r12, 1
    CONST r8, 0
    LOAD r10, r8
    MOV r7, r10
    ADD r7, r12
    CONST r0, 5
    CONST r1, 0
    CONST r2, 0
    CONST r3, 1
    MOV r6, r7
    READ r11
    MOV r5, r11
    CONST r9, 127
    ADD r5, r9
    CONST r9, 7
    SHR r5, r9
grow:
    CONST r0, 7
    CONST r3, 2
    READ r9
    LTU r9, r11
    BRNZ r9, do_grow
    JUMP copy_init
do_grow:
    CONST r0, 7
    WRITE 1
    JUMP grow
copy_init:
    CONST r9, 0
copy_loop:
    MOV r8, r9
    LTU r8, r11
    BRNZ r8, copy_go
    JUMP commit
copy_go:
    CONST r0, 5
    CONST r1, 7
    MOV r4, r9
    MOV r5, r9
    CONST r6, 4
    COPY 0
    ADD r9, r15
    JUMP copy_loop
commit:
    CONST r8, 0
    LOAD r10, r8
    ADD r10, r12
    STORE r10, r8
    MOV r5, r11
    WRITE 2
    RET
";

const LESSON: &str = ".module 0\n CONST r9, 42\nspin: YIELD\n JUMP spin\n";

fn learner_world(ruleset: &str) -> Sim {
    let mut s = sim(LEARNER, 1, ruleset);
    s.st.agents[0].inv[3] = 40;
    let t = s.tile_of(0);
    let own = assemble(LEARNER).unwrap().to_cells();
    let lesson = assemble(LESSON).unwrap().to_cells();
    s.st.dropped.insert(t, vec![Media::from_cells(900, &own), Media::from_cells(901, &lesson)]);
    s.fixture_rebase();
    s
}

#[test]
fn learned_code_is_adopted_retained_while_running_then_dropped() {
    let mut s = learner_world(gene_core::sim::RULESET_M1B);
    let t = s.tile_of(0);
    let mut saw_retained = false;
    for _ in 0..600 {
        step_audited(&mut s, 1);
        saw_retained |= !s.st.agents[0].retained.is_empty();
        if s.st.agents[0].vm.regs[9] == 42 {
            break;
        }
    }
    let a = &s.st.agents[0];
    assert_eq!(a.vm.regs[9], 42, "the second lesson runs");
    assert_eq!(s.obs.m1.adoptions, 2);
    assert!(saw_retained, "the first learned code stayed while it was still running");
    step_audited(&mut s, 2);
    let a = &s.st.agents[0];
    assert!(a.retained.is_empty(), "unreferenced retained code is dropped");
    assert!(a.draft.is_none());
    let lesson = assemble(LESSON).unwrap().to_cells();
    let learned = a.learned.as_ref().unwrap();
    assert_eq!(learned.media.prefix(learned.used_len).unwrap(), &lesson[..]);
    // records on the tile: the two lessons plus the dropped first learned code
    assert_eq!(s.st.dropped[&t].len(), 3);
    // save/resume keeps learned code executable
    let resumed = Sim::load(&s.save()).unwrap();
    assert_eq!(resumed.state_hash(), s.state_hash());
}

#[test]
fn commit_is_off_in_m1a() {
    let mut s = learner_world(RULESET_M1);
    for _ in 0..300 {
        s.step();
    }
    assert_eq!(s.obs.m1.adoptions, 0);
    assert!(s.st.agents[0].learned.is_none());
}
