//! gene-m0-2 / gene-m2-1 (SPEC §20h): LocalTrace, passive signal conduction, faster
//! decomposition at living temperatures, and the locality order.
use gene_core::intent::Fault;
use gene_core::isa::assemble;
use gene_core::sim::{signal_key, Config, Sim, RULESET, RULESET_M0B, RULESET_M1F, RULESET_M2};
use gene_core::species::*;
use gene_core::world::Parcel;

/// Agents with at least 50000 energy run the writer part, the others the reader part.
fn two(writer: &str, reader: &str, ruleset: &str) -> Sim {
    let src = format!(
        "
.module 0
    CONST r12, 1
    CONST r0, 0
    CONST r3, 0
    READ r9
    CONST r11, 50000
    LTU r9, r11
    BRNZ r9, reader
{writer}
    JUMP spin
reader:
{reader}
spin:
    YIELD
    JUMP spin
"
    );
    let prog = assemble(&src).unwrap();
    let mut s = Sim::with_program_ruleset(Config { seed: 5, width: 64, height: 64, agents: 2 }, &prog, ruleset).unwrap();
    s.fixture_place(0, 20, 20, 0); // writer, facing north
    s.fixture_place(1, 21, 19, 0); // reader, facing north; the writer's front tile is on its left
    s.st.agents[0].energy = 60000;
    s.st.agents[0].inv[S] = 8;
    s.fixture_rebase();
    s
}

const DROP_S: &str = "
    CONST r0, 0
    CONST r1, 0x11
    CONST r3, 7
    CONST r5, 2
    TRANSFER 0
";

const READ_TRACE: &str = "
    YIELD
    CONST r0, 9
    CONST r1, -1
    CONST r2, 0
    CONST r4, 0
    CONST r3, 0
    READ r9
    CONST r3, 1
    READ r10
    CONST r3, 2
    READ r11
    CONST r3, 3
    READ r13
";

fn reader(s: &Sim) -> &gene_core::agent::Agent {
    s.st.agents.iter().find(|a| (a.x, a.y) == (21, 19)).unwrap()
}

#[test]
fn a_neighbour_reads_the_trace_of_a_transfer() {
    let mut s = two(DROP_S, READ_TRACE, RULESET_M2);
    for _ in 0..2 {
        s.step();
        assert!(s.audit_ok());
    }
    let r = reader(&s).vm.regs;
    // matter moved, the actor is south of the trace tile = behind the reader (relative 2),
    // 2 units, one tick ago
    assert_eq!((r[9], r[10], r[11], r[13]), (1, 2, 2, 1));
}

#[test]
fn traces_expire_after_256_ticks() {
    let late = READ_TRACE.replace("    YIELD\n", "    CONST r13, 300\nw:\n    YIELD\n    SUB r13, r12\n    BRNZ r13, w\n");
    let mut s = two(DROP_S, &late, RULESET_M2);
    for _ in 0..305 {
        s.step();
    }
    let r = reader(&s).vm.regs;
    assert_eq!((r[9], r[10], r[11]), (0, 0, 0), "an expired slot reads as empty");
    assert!(s.st.traces.is_empty(), "and is pruned");
}

#[test]
fn local_trace_is_off_before_m2() {
    let mut s = two(DROP_S, READ_TRACE, RULESET_M1F);
    for _ in 0..2 {
        s.step();
    }
    assert_eq!(reader(&s).vm.fault, Fault::BadAddress as u8);
    assert!(s.st.traces.is_empty());
}

fn metal(s: &mut Sim, x: u32, y: u32, uid: u64) -> u32 {
    let t = s.st.world.idx(x, y);
    let mut matrix = [0; NSPEC];
    matrix[M] = 4;
    s.st.world.parcels.insert(t, Parcel { matrix, ..Parcel::new(uid) });
    s.st.world.parcel_changed(t);
    t
}

#[test]
fn bonded_metal_conducts_signals_as_in_original_t33() {
    let prog = assemble(".module 0\nspin:\n    YIELD\n    JUMP spin\n").unwrap();
    let mut s = Sim::with_program_ruleset(Config { seed: 5, width: 64, height: 64, agents: 1 }, &prog, RULESET_M2).unwrap();
    s.fixture_place(0, 40, 40, 0);
    let a = metal(&mut s, 20, 20, 1);
    let b = metal(&mut s, 21, 20, 2);
    let lone = metal(&mut s, 25, 20, 3);
    s.st.next_parcel_uid = 4;
    s.st.world.parcels.get_mut(&a).unwrap().bond[1] = 8;
    s.st.world.parcels.get_mut(&b).unwrap().bond[3] = 8;
    s.fixture_rebase();
    s.st.signal.insert(signal_key(a, 0), 8);
    s.st.signal.insert(signal_key(lone, 0), 8);
    s.step();
    let get = |t: u32| s.st.signal.get(&signal_key(t, 0)).copied().unwrap_or(0);
    assert_eq!((get(a), get(b), get(lone)), (4, 1, 0), "A 8, B 0 -> A 4, B 1; an unbonded tile keeps nothing");
}

#[test]
fn conduction_is_off_in_m1f() {
    let prog = assemble(".module 0\nspin:\n    YIELD\n    JUMP spin\n").unwrap();
    let mut s = Sim::with_program_ruleset(Config { seed: 5, width: 64, height: 64, agents: 1 }, &prog, RULESET_M1F).unwrap();
    s.fixture_place(0, 40, 40, 0);
    let a = metal(&mut s, 20, 20, 1);
    let b = metal(&mut s, 21, 20, 2);
    s.st.next_parcel_uid = 3;
    s.st.world.parcels.get_mut(&a).unwrap().bond[1] = 8;
    s.st.world.parcels.get_mut(&b).unwrap().bond[3] = 8;
    s.fixture_rebase();
    s.st.signal.insert(signal_key(a, 0), 8);
    s.step();
    assert!(s.st.signal.is_empty());
}

#[test]
fn decomposition_floor_at_living_temperatures() {
    assert_eq!((rx_rate(&RX2, 300, false), rx_rate(&RX2, 300, true)), (1, 16));
    assert_eq!(rx_rate(&RX2, 400, true), 16, "the next bin is not slower");
    assert_eq!(rx_rate(&RX2, 600, true), 256, "hot bins unchanged");
    assert_eq!(rx_rate(&RX2, 250, true), 0, "frozen ground stays");
    assert_eq!(rx_rate(&RX3, 300, true), RX3.rates[1] as u64, "only Rx2");
}

#[test]
fn dead_matter_decays_faster_in_gene_m0_2() {
    let prog = assemble(".module 0\nspin:\n    YIELD\n    JUMP spin\n").unwrap();
    let left = |ruleset: &str| {
        let mut s = Sim::with_program_ruleset(Config { seed: 9, width: 64, height: 64, agents: 1 }, &prog, ruleset).unwrap();
        for t in 0..s.st.world.n() as u32 {
            s.st.world.put_loose(t, G, 30);
        }
        s.fixture_rebase();
        for _ in 0..4096 {
            s.step();
        }
        assert!(s.audit_ok());
        (0..s.st.world.n() as u32).map(|t| s.st.world.loose_total(t, G)).sum::<u64>()
    };
    let (slow, fast) = (left(RULESET), left(RULESET_M0B));
    let start = 64 * 64 * 30;
    assert!(start - fast > 4 * (start - slow), "decayed {} vs {}", start - fast, start - slow);
}

#[test]
fn locality_order_keeps_the_ledger_and_matches_parallel() {
    let src = "
.module 0
loop:
    CONST r0, 0
    CONST r1, 0x12
    CONST r3, 7
    CONST r5, 2
    TRANSFER 0
    CONST r0, 2
    CONST r1, 0
    CONST r2, -1
    CONST r4, 5
    CONST r5, 255
    LINK 0
    CONST r3, 1
    CONST r5, 40
    EMIT 0
    CONST r0, 9
    CONST r1, 0
    CONST r2, -1
    CONST r3, 0
    CONST r4, 0
    READ r9
    CONST r5, 1
    DRIVE 1
    CONST r5, 0
    DRIVE 0
    JUMP loop
";
    let prog = assemble(src).unwrap();
    let cfg = Config { seed: 4, width: 64, height: 64, agents: 300 };
    let mut a = Sim::with_program_ruleset(cfg.clone(), &prog, RULESET_M2).unwrap();
    let mut b = Sim::with_program_ruleset(cfg, &prog, RULESET_M2).unwrap();
    b.threads = 4;
    b.par_think_min = 1;
    for k in 0..512 {
        a.step();
        b.step();
        assert!(a.audit_ok(), "tick {}", a.st.tick);
        if k % 128 == 0 {
            assert_eq!(a.state_hash(), b.state_hash(), "tick {}", a.st.tick);
        }
    }
    assert!(!a.st.traces.is_empty());
    assert_eq!(a.state_hash(), b.state_hash());
}
