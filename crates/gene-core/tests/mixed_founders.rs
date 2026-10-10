//! Several public ancestors in one world (E2, user decision 2).
use gene_core::genome::hash_cells;
use gene_core::isa::assemble;
use gene_core::sim::{Config, Sim, RULESET_M2};

fn cfg() -> Config {
    Config { seed: 11, width: 64, height: 64, agents: 30 }
}

#[test]
fn one_program_is_the_single_founder_world() {
    let p = assemble(gene_core::FOUNDER7_ASM).unwrap();
    let mut a = Sim::with_program_ruleset(cfg(), &p, RULESET_M2).unwrap();
    let mut b = Sim::with_programs_ruleset(cfg(), std::slice::from_ref(&p), RULESET_M2).unwrap();
    for _ in 0..64 {
        a.step();
        b.step();
    }
    assert_eq!(a.state_hash(), b.state_hash());
}

#[test]
fn founders_alternate_and_keep_the_ledger() {
    let progs = [assemble(gene_core::FOUNDER6_ASM).unwrap(), assemble(gene_core::FOUNDER7_ASM).unwrap()];
    let hashes: Vec<[u8; 32]> = progs.iter().map(|p| hash_cells(&p.to_cells())).collect();
    assert_ne!(hashes[0], hashes[1]);
    let mut s = Sim::with_programs_ruleset(cfg(), &progs, RULESET_M2).unwrap();
    for (i, a) in s.st.agents.iter().enumerate() {
        assert_eq!(a.uid, i as u64 + 1);
        assert_eq!(hash_cells(a.code.prefix(a.used_len).unwrap()), hashes[i % 2], "founder {i}");
    }
    for _ in 0..512 {
        s.step();
    }
    assert!(s.audit_ok());
    assert!(Sim::with_programs_ruleset(cfg(), &[], RULESET_M2).is_err());
}
