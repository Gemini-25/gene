//! gene-m0-3 (SPEC §20i): soil keeps up to SOIL_RETAIN units of water from running downhill.
use gene_core::env::SOIL_RETAIN;
use gene_core::isa::assemble;
use gene_core::sim::{Config, Sim, RULESET_M0B, RULESET_M0C, RULESET_M2B};
use gene_core::species::*;

/// A world with no water (and no G to decompose or respire into water) except `w` units on a
/// tile standing above all its neighbours.
fn slope(ruleset: &str, w: u64) -> (Sim, u32) {
    let prog = assemble(".module 0\nspin:\n    YIELD\n    JUMP spin\n").unwrap();
    let mut s = Sim::with_program_ruleset(Config { seed: 3, width: 64, height: 64, agents: 1 }, &prog, ruleset).unwrap();
    for t in 0..s.st.world.n() as u32 {
        for sp in [W, I, V, G] {
            let k = s.st.world.loose_total(t, sp);
            s.st.world.take_loose(t, sp, k);
        }
    }
    // raise one inland tile above all four neighbours: water can only leave it by running downhill
    let t = s.st.world.idx(32, 32);
    let top = (0..4).filter_map(|d| s.st.world.neighbor(t, d)).map(|u| s.st.world.elev[u as usize]).max().unwrap();
    s.st.world.elev[t as usize] = top + 3;
    s.st.world.put_loose(t, W, w);
    s.st.agents[0].inv = [0; NSPEC]; // no respiration water either
    s.fixture_rebase();
    (s, t)
}

fn water(s: &Sim, t: u32) -> u64 {
    s.st.world.loose_total(t, W) + s.st.world.loose_total(t, I)
}

#[test]
fn soil_keeps_a_little_water_on_a_slope() {
    let (mut old, t) = slope(RULESET_M0B, 5);
    let (mut new, t2) = slope(RULESET_M0C, 5);
    assert_eq!(t, t2);
    for _ in 0..256 {
        old.step();
        new.step();
    }
    assert!(old.audit_ok() && new.audit_ok());
    assert_eq!(water(&old, t), 0, "gene-m0-2: every unit runs downhill");
    assert_eq!(water(&new, t), 5, "gene-m0-3: the soil keeps it");
}

#[test]
fn only_water_above_the_retained_amount_runs_off() {
    let (mut s, t) = slope(RULESET_M2B, 40);
    for _ in 0..512 {
        s.step();
    }
    assert!(s.audit_ok());
    assert_eq!(water(&s, t), SOIL_RETAIN);
    let total: u64 = (0..s.st.world.n() as u32).map(|u| water(&s, u)).sum();
    assert_eq!(total, 40, "water is conserved");
}
