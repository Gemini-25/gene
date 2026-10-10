use gene_core::agent::BASE_UPKEEP;
use gene_core::intent::Fault;
use gene_core::isa::assemble;
use gene_core::sim::{Config, Sim};

fn sim_with(src: &str, agents: u32) -> Sim {
    let prog = assemble(src).unwrap();
    Sim::with_program(Config { seed: 1, width: 64, height: 64, agents }, &prog).unwrap()
}

fn upkeep(sim: &Sim, i: usize) -> u32 {
    let _ = BASE_UPKEEP;
    sim.st.agents[i].upkeep()
}

#[test]
fn arithmetic_divzero_and_costs() {
    let mut sim = sim_with(
        ".module 0\n CONST r1, 7\n CONST r2, 5\n SUB r1, r2\n MUL r1, r2\n CONST r3, 0\n DIV r1, r3\n YIELD\nspin: JUMP spin\n",
        1,
    );
    let e0 = sim.st.agents[0].energy;
    let m = upkeep(&sim, 0);
    sim.step();
    let a = &sim.st.agents[0];
    assert_eq!(a.vm.regs[2], 5);
    assert_eq!(a.vm.regs[1], 0, "DIV by zero yields 0");
    assert_eq!(a.vm.fault, Fault::DivZero as u8);
    // 7 instructions x 4q + upkeep; respiration may add 1666
    let spent = e0 as i64 - a.energy as i64;
    assert!(spent == (7 * 4 + m) as i64 || spent == (7 * 4 + m) as i64 - 1666, "spent {spent}");
}

#[test]
fn fuel_limits_instructions_per_tick() {
    let mut sim = sim_with(".module 0\nloop: NOP\n JUMP loop\n", 1);
    sim.st.agents[0].inv = [0; 10]; // no respiration
    sim.fixture_rebase();
    let e0 = sim.st.agents[0].energy;
    let m = upkeep(&sim, 0);
    sim.step();
    assert_eq!(e0 - sim.st.agents[0].energy, 32 * 4 + m);
    assert!(sim.audit_ok());
}

#[test]
fn fifth_external_query_hits_the_limit() {
    let src = ".module 0\n CONST r0, 1\n READ r5\n READ r5\n READ r5\n READ r5\n READ r6\n YIELD\nspin: JUMP spin\n";
    let mut sim = sim_with(src, 1);
    sim.st.agents[0].inv = [0; 10];
    sim.fixture_rebase();
    let e0 = sim.st.agents[0].energy;
    let m = upkeep(&sim, 0);
    sim.step();
    let a = &sim.st.agents[0];
    assert_eq!(a.vm.fault, Fault::QueryLimit as u8);
    // 7 instructions x 4q + 4 queries x 8q
    assert_eq!(e0 - a.energy, 7 * 4 + 4 * 8 + m);
}

#[test]
fn call_ret_and_out_of_range_branch() {
    let src = ".entry 0\n.module 0\n CALL 1\n CONST r2, 9\n JUMP 0\n.module 1\n CONST r1, 3\n BRNZ r1, far\n CONST r3, 4\n RET\n.module 2\nfar: NOP\n";
    // `far` is in module 2: assembling a cross-module branch must fail
    assert!(assemble(src).is_err());
    let src = ".entry 0\n.module 0\n CALL 1\n CONST r2, 9\n YIELD\nhere: JUMP here\n.module 1\n CONST r1, 3\n .word 0x7fff0110\n CONST r3, 4\n RET\n";
    let mut sim = sim_with(src, 1);
    sim.step();
    let r = sim.st.agents[0].vm.regs;
    assert_eq!((r[1], r[2], r[3]), (3, 9, 4), "out-of-range BRNZ falls through");
    assert!(sim.st.agents[0].vm.stack.is_empty());
}

#[test]
fn ret_on_empty_stack_restarts_entry_and_yields() {
    let mut sim = sim_with(".module 0\n CONST r1, 1\n ADD r2, r1\n RET\n", 1);
    sim.step();
    sim.step();
    assert_eq!(sim.st.agents[0].vm.regs[2], 2);
    assert_eq!(sim.st.agents[0].vm.pc, 0);
}

const MOVER: &str = ".module 0\n CONST r5, 0\n DRIVE 0\nspin: YIELD\n JUMP spin\n";
const STILL: &str = ".module 0\nspin: YIELD\n JUMP spin\n";

#[test]
fn head_on_pair_swaps() {
    let mut sim = sim_with(MOVER, 2);
    sim.fixture_place(0, 10, 10, 1); // facing east
    sim.fixture_place(1, 11, 10, 3); // facing west
    sim.fixture_rebase();
    sim.step();
    let (a, b) = (&sim.st.agents[0], &sim.st.agents[1]);
    assert_eq!((a.x, b.x), (11, 10));
    assert_eq!(a.receipt.map(|r| r.0), Some(0));
    assert!(sim.audit_ok());
}

#[test]
fn same_destination_has_one_winner() {
    let mut sim = sim_with(MOVER, 2);
    sim.fixture_place(0, 10, 10, 1);
    sim.fixture_place(1, 12, 10, 3);
    sim.fixture_rebase();
    sim.step();
    let xs = [sim.st.agents[0].x, sim.st.agents[1].x];
    assert!(xs == [11, 12] || xs == [10, 11], "{xs:?}");
    assert!(sim.audit_ok());
}

#[test]
fn chain_into_stationary_actor_fails() {
    let prog = assemble(MOVER).unwrap();
    let still = assemble(STILL).unwrap();
    let mut sim = Sim::with_program(Config { seed: 1, width: 64, height: 64, agents: 3 }, &prog).unwrap();
    // agent 2 stays still: swap its program
    sim.st.agents[2].prog = Some(std::sync::Arc::new(still));
    sim.fixture_place(0, 10, 10, 1);
    sim.fixture_place(1, 11, 10, 1);
    sim.fixture_place(2, 12, 10, 1);
    sim.fixture_rebase();
    sim.step();
    let xs: Vec<u16> = sim.st.agents.iter().map(|a| a.x).collect();
    assert_eq!(xs, vec![10, 11, 12]);
    assert_eq!(sim.st.agents[0].receipt.map(|r| r.0), Some(4), "Blocked");
}

#[test]
fn cycle_of_four_rotates() {
    let mut sim = sim_with(MOVER, 4);
    // square: (10,10)->E, (11,10)->S, (11,11)->W, (10,11)->N
    sim.fixture_place(0, 10, 10, 1);
    sim.fixture_place(1, 11, 10, 2);
    sim.fixture_place(2, 11, 11, 3);
    sim.fixture_place(3, 10, 11, 0);
    sim.fixture_rebase();
    sim.step();
    let p: Vec<(u16, u16)> = sim.st.agents.iter().map(|a| (a.x, a.y)).collect();
    assert_eq!(p, vec![(11, 10), (11, 11), (10, 11), (10, 10)]);
    assert!(sim.audit_ok());
}

#[test]
fn founder5_reproduces_alone() {
    let prog = assemble(gene_core::FOUNDER5_ASM).unwrap();
    let mut sim = Sim::with_program(Config { seed: 2, width: 64, height: 64, agents: 1 }, &prog).unwrap();
    for _ in 0..8192 {
        sim.step();
        if sim.obs.total_births > 0 {
            break;
        }
    }
    assert!(sim.obs.total_births >= 1, "no birth within two days");
    let child = sim.st.agents.iter().find(|a| a.generation == 1).expect("child alive at birth");
    assert!(gene_core::genome::parse(child.code.prefix(child.used_len).unwrap()).is_ok());
    assert!(sim.audit_ok());
}
