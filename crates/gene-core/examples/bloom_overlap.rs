//! Blooms of the previous and the current epoch (SPEC §20m) at a saved state: their areas, their
//! overlap, and the share of agents on each, to tell finding new blooms from staying on old ones.
//! usage: bloom_overlap SAVE.bin
fn main() {
    let sim = gene_core::Sim::load(&std::fs::read(std::env::args().nth(1).unwrap()).unwrap()).unwrap();
    let w = &sim.st.world;
    let e = gene_core::env::BLOOM_EPOCH;
    let now = gene_core::env::moss_ground(w, sim.st.config.seed, true, sim.st.tick);
    let before = gene_core::env::moss_ground(w, sim.st.config.seed, true, sim.st.tick.saturating_sub(e));
    let count = |m: &[u8]| m.iter().filter(|&&v| v != 0).count();
    let both = now.iter().zip(&before).filter(|(a, b)| **a != 0 && **b != 0).count();
    let n = sim.st.agents.len().max(1) as f64;
    let on = |m: &[u8]| sim.st.agents.iter().filter(|a| m[w.idx(a.x as u32, a.y as u32) as usize] != 0).count() as f64 / n;
    println!(
        "tick {} (epoch phase {:.2}): blooms now {:.1}% of the map, previous {:.1}%, overlap {:.1}%; agents on current {:.1}%, on previous {:.1}%",
        sim.st.tick,
        (sim.st.tick % e) as f64 / e as f64,
        100.0 * count(&now) as f64 / w.n() as f64,
        100.0 * count(&before) as f64 / w.n() as f64,
        100.0 * both as f64 / w.n() as f64,
        100.0 * on(&now),
        100.0 * on(&before)
    );
}
