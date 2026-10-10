//! Copy lineage end to end (SPEC §17e): a writer puts its code on a parcel's record, a reader
//! copies it into its draft, commits, adopts and runs it: one non-kin transfer.
use gene_core::isa::assemble;
use gene_core::lineage::{Lineage, TransferKind};
use gene_core::sim::{Config, Sim, RULESET_M1F};
use gene_core::species::*;
use gene_core::world::Parcel;

const SRC: &str = "
.module 0
    CONST r12, 1
    CONST r0, 0
    CONST r3, 0
    READ r9
    CONST r11, 50000
    LTU r9, r11
    BRNZ r9, reader
    ; writer: a one-page record on the parcel in front, then its own code onto it, 4 cells a time
    CONST r5, 0
wpages:
    ADD r5, r12
    CONST r0, 5
    CONST r1, 0
    CONST r2, -1
    CONST r6, 0
    WRITE 1
    CONST r10, PAGES
    MOV r11, r5
    LTU r11, r10
    BRNZ r11, wpages
    CONST r8, 0
wloop:
    CONST r0, 6
    CONST r1, 0x15
    CONST r3, 0
    MOV r4, r8
    MOV r5, r8
    CONST r6, 4
    CONST r7, 0
    COPY 0
    CONST r10, 4
    ADD r8, r10
    CONST r10, LEN
    MOV r11, r8
    LTU r11, r10
    BRNZ r11, wloop
    JUMP spin
reader:
    CONST r13, 40
rwait:
    YIELD
    SUB r13, r12
    BRNZ r13, rwait
    CONST r5, 0
rpages:
    ADD r5, r12
    CONST r0, 7
    WRITE 1
    CONST r10, PAGES
    MOV r11, r5
    LTU r11, r10
    BRNZ r11, rpages
    CONST r8, 0
rloop:
    CONST r0, 0x15
    CONST r1, 7
    CONST r7, 0
    MOV r4, r8
    MOV r5, r8
    CONST r6, 4
    COPY 0
    CONST r10, 4
    ADD r8, r10
    CONST r10, LEN
    MOV r11, r8
    LTU r11, r10
    BRNZ r11, rloop
    CONST r0, 7
    CONST r5, LEN
    WRITE 2
    YIELD
    CALL 0
spin:
    YIELD
    JUMP spin
";

#[test]
fn code_copied_from_a_record_runs_in_another_agent() {
    let len = assemble(&SRC.replace("LEN", "0").replace("PAGES", "0")).unwrap().raw_len();
    let src = SRC.replace("LEN", &len.to_string()).replace("PAGES", &len.div_ceil(128).to_string());
    let prog = assemble(&src).unwrap();
    assert_eq!(prog.raw_len(), len);
    let mut s = Sim::with_program_ruleset(Config { seed: 5, width: 64, height: 64, agents: 2 }, &prog, RULESET_M1F).unwrap();
    s.fixture_place(0, 20, 20, 0); // writer, facing north
    s.fixture_place(1, 20, 18, 2); // reader, facing south: the same parcel is in front of both
    s.st.agents[0].energy = 60000;
    s.st.agents[0].inv[G] = 40; // pages for the record
    s.st.agents[1].inv[G] = 40; // and for the draft
    let t = s.st.world.idx(20, 19);
    let mut matrix = [0; NSPEC];
    matrix[S] = 4;
    s.st.world.parcels.insert(t, Parcel { matrix, ..Parcel::new(1) });
    s.st.next_parcel_uid = 2;
    s.st.world.parcel_changed(t);
    s.fixture_rebase();
    let (writer, reader) = (s.st.agents[0].uid, s.st.agents[1].uid);
    s.obs.lineage = Some(Vec::new());
    let mut l = Lineage::default();
    for _ in 0..600 {
        s.step();
        assert!(s.audit_ok(), "tick {}", s.st.tick);
        for f in s.obs.lineage.as_mut().unwrap().drain(..) {
            l.push(&f);
        }
    }
    assert!(s.obs.m1.adoptions >= 1, "{:?}", s.obs.m1);
    assert_eq!(l.transfers.len(), 1, "{:?}", l.transfers);
    let tr = &l.transfers[0];
    assert_eq!((tr.agent, tr.author, tr.kind, tr.learned), (reader, writer, TransferKind::NonKin, true));
    assert_eq!(tr.cells as usize, len, "every cell of the adopted code came from the writer");
}
