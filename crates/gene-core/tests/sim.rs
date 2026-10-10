use gene_core::sim::{Config, Sim};

fn small(seed: u64) -> Config {
    Config { seed, width: 64, height: 64, agents: 24 }
}

#[test]
fn ledger_is_exact_every_tick() {
    let mut sim = Sim::new(small(7)).unwrap();
    assert!(sim.audit_ok(), "initial audit");
    for _ in 0..3000 {
        sim.step();
        let a = sim.audit();
        assert!(
            a.ok(&sim.st.atoms0),
            "tick {}: energy {} expected {} (diff {}), atoms {:?} vs {:?}, heat_cap_ok {}",
            sim.st.tick,
            a.energy,
            a.expected,
            a.energy - a.expected,
            a.atoms,
            sim.st.atoms0,
            a.heat_cap_ok
        );
    }
}

#[test]
fn same_seed_same_hash_different_seed_differs() {
    let mut a = Sim::new(small(3)).unwrap();
    let mut b = Sim::new(small(3)).unwrap();
    let mut c = Sim::new(small(4)).unwrap();
    for _ in 0..800 {
        a.step();
        b.step();
        c.step();
    }
    assert_eq!(a.state_hash(), b.state_hash());
    assert_ne!(a.state_hash(), c.state_hash());
}

#[test]
fn save_resume_equals_uninterrupted() {
    let mut straight = Sim::new(small(11)).unwrap();
    let mut first = Sim::new(small(11)).unwrap();
    for _ in 0..700 {
        straight.step();
    }
    for _ in 0..350 {
        first.step();
    }
    let bytes = first.save();
    let mut resumed = Sim::load(&bytes).unwrap();
    assert_eq!(resumed.state_hash(), first.state_hash());
    for _ in 0..350 {
        resumed.step();
    }
    assert_eq!(resumed.state_hash(), straight.state_hash());
    assert!(resumed.audit_ok());
}

#[test]
fn corrupted_save_is_rejected() {
    let sim = Sim::new(small(1)).unwrap();
    let mut bytes = sim.save();
    let k = bytes.len() / 2;
    bytes[k] ^= 1;
    assert!(Sim::load(&bytes).is_err());
}

#[test]
fn parallel_transport_matches_serial_bit_for_bit() {
    let prog = gene_core::isa::assemble(gene_core::FOUNDER6_ASM).unwrap();
    let cfg = Config { seed: 9, width: 128, height: 128, agents: 64 };
    let mut serial = Sim::with_program(cfg.clone(), &prog).unwrap();
    let mut par = Sim::with_program(cfg, &prog).unwrap();
    par.threads = 4;
    par.par_think_min = 1;
    for _ in 0..3000 {
        serial.step();
        par.step();
    }
    assert_eq!(serial.state_hash(), par.state_hash());
    assert!(par.audit_ok());
}
