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

// ---------------------------------------------------------------- SPEC §20j aging (candidate)

#[test]
fn aging_raises_upkeep_by_one_per_day_of_age_and_keeps_the_ledger() {
    use gene_core::sim::{RULESET_M2C, RULESET_M2D};
    let prog = assemble(".module 0\nspin:\n    YIELD\n    JUMP spin\n").unwrap();
    let drain = |ruleset: &str, age: u64| {
        let mut s = Sim::with_program_ruleset(Config { seed: 3, width: 64, height: 64, agents: 1 }, &prog, ruleset).unwrap();
        s.st.agents[0].age = age;
        s.st.agents[0].inv = [0; NSPEC]; // no respiration
        s.fixture_rebase();
        let e0 = s.st.agents[0].energy;
        s.step();
        assert!(s.audit_ok());
        e0 - s.st.agents[0].energy
    };
    let young = drain(RULESET_M2C, 0);
    assert_eq!(drain(RULESET_M2B, 40 * 4096), young, "no aging without the capability");
    assert_eq!(drain(RULESET_M2C, 40 * 4096) - young, 40, "40 days old: 40q more per tick");
    assert_eq!(drain(RULESET_M2D, 40 * 4096) - young, 20, "the milder candidate: 1q per two days");
}

// ---------------------------------------------------------------- SPEC §20k moss (candidate)

fn idle_world(ruleset: &str) -> Sim {
    let prog = assemble(".module 0\nspin:\n    YIELD\n    JUMP spin\n").unwrap();
    Sim::with_program_ruleset(Config { seed: 3, width: 64, height: 64, agents: 1 }, &prog, ruleset).unwrap()
}

fn loose_g(s: &Sim, fertile: bool) -> u64 {
    (0..s.st.world.n() as u32)
        .filter(|&t| (s.st.world.substrate[t as usize] >= gene_core::env::MOSS_SUBSTRATE) == fertile)
        .map(|t| s.st.world.loose_total(t, G))
        .sum()
}

#[test]
fn moss_grows_on_fertile_ground_in_daylight_and_the_ledger_balances() {
    use gene_core::sim::{RULESET_M2D, RULESET_M2E};
    let mut old = idle_world(RULESET_M2D);
    let mut new = idle_world(RULESET_M2E);
    assert!((0..new.st.world.n()).any(|t| new.st.world.substrate[t] >= gene_core::env::MOSS_SUBSTRATE), "the test world has fertile ground");
    let (g0_old, g0_new) = (loose_g(&old, true), loose_g(&new, true));
    for _ in 0..2048 {
        old.step();
        new.step();
    }
    assert!(old.audit_ok() && new.audit_ok(), "moss keeps energy and atoms");
    assert!(loose_g(&new, true) > g0_new + 50, "moss adds G on fertile ground");
    assert!(loose_g(&old, true) <= g0_old, "no moss in gene-m2-4");
    // off the fertile ground both worlds hold the same G, tile for tile
    for t in 0..new.st.world.n() as u32 {
        if new.st.world.substrate[t as usize] < gene_core::env::MOSS_SUBSTRATE && new.st.world.occ[t as usize] == 0 {
            assert_eq!(new.st.world.loose_total(t, G), old.st.world.loose_total(t, G));
        }
    }
}

// ---------------------------------------------------------------- SPEC §20l air signal (candidate)

#[test]
fn signals_spread_through_the_air_only_in_gene_m2_6() {
    use gene_core::sim::{signal_key, RULESET_M2E, RULESET_M2F};
    for (ruleset, spreads) in [(RULESET_M2E, false), (RULESET_M2F, true)] {
        let mut s = idle_world(ruleset);
        let t = s.st.world.idx(10, 10);
        s.st.signal.insert(signal_key(t, 1), 64);
        s.step();
        let at = |s: &Sim, u: u32| s.st.signal.get(&signal_key(u, 1)).copied().unwrap_or(0);
        let east = s.st.world.idx(11, 10);
        if spreads {
            assert_eq!(at(&s, t), 32, "keeps half");
            assert_eq!(at(&s, east), 8, "each neighbour gets an eighth");
            s.step();
            assert_eq!(at(&s, t), 16 + 4 * 1, "half of 32 plus an eighth of each neighbour's 8");
        } else {
            assert_eq!(at(&s, t), 0, "without conduction the field holds only new emissions");
            assert_eq!(at(&s, east), 0);
        }
        assert!(s.audit_ok());
    }
}

// ---------------------------------------------------------------- SPEC §20m moving moss blooms

#[test]
fn moss_blooms_move_every_epoch_and_cover_about_one_block_in_sixteen() {
    use gene_core::env::{moss_ground, BLOOM_EPOCH};
    let s = idle_world(gene_core::sim::RULESET_M2G);
    let w = &s.st.world;
    let a = moss_ground(w, 3, true, 0);
    let b = moss_ground(w, 3, true, BLOOM_EPOCH - 1);
    let c = moss_ground(w, 3, true, BLOOM_EPOCH);
    assert_eq!(a, b, "the same epoch keeps its blooms");
    assert_ne!(a, c, "the next epoch draws new ones");
    // over many epochs about 1/16 of the ground is moss
    let total: usize = (0..64).map(|e| moss_ground(w, 3, true, e * BLOOM_EPOCH).iter().filter(|&&m| m != 0).count()).sum();
    let share = total as f64 / (64.0 * w.n() as f64);
    assert!((0.03..0.10).contains(&share), "share {share}");
    // whole 16x16 blocks
    let t = (0..w.n()).find(|&t| c[t] != 0).unwrap() as u32;
    let (x, y) = (t % w.w, t / w.w);
    assert_eq!(c[((y | 15) * w.w + (x | 15)) as usize], 1, "the block's far corner is moss too");
}

#[test]
fn moss_blooms_grow_g_and_keep_the_ledger() {
    let mut s = idle_world(gene_core::sim::RULESET_M2G);
    let mask = gene_core::env::moss_ground(&s.st.world, 3, true, 0);
    let g = |s: &Sim| (0..s.st.world.n() as u32).filter(|&t| mask[t as usize] != 0).map(|t| s.st.world.loose_total(t, G)).sum::<u64>();
    let g0 = g(&s);
    for _ in 0..2048 {
        s.step();
    }
    assert!(s.audit_ok());
    assert!(g(&s) > g0 + 50, "blooms add G");
}
