//! Finding moving blooms (SPEC §20m): run a saved state and, every 512 ticks, print the epoch
//! phase, the share of agents on moss ground (and the moss ground's share of the map), and the
//! loose G on moss ground per tile. A lineage that finds new blooms faster climbs sooner after
//! each epoch boundary.
//! usage: bloom_track SAVE.bin [TICKS=16384]
fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let mut sim = gene_core::Sim::load(&std::fs::read(&a[0]).unwrap()).unwrap();
    let ticks: u64 = a.get(1).map_or(16384, |v| v.parse().unwrap());
    let end = sim.st.tick + ticks;
    let epoch = gene_core::env::BLOOM_EPOCH;
    while sim.st.tick < end {
        sim.step();
        if sim.st.tick % 512 == 0 {
            let w = &sim.st.world;
            let mask = gene_core::env::moss_ground(w, sim.st.config.seed, sim.caps.bloom, sim.st.tick);
            let area = mask.iter().filter(|&&m| m != 0).count().max(1);
            let on = sim.st.agents.iter().filter(|ag| mask[w.idx(ag.x as u32, ag.y as u32) as usize] != 0).count();
            let g: u64 = (0..w.n() as u32).filter(|&t| mask[t as usize] != 0).map(|t| w.loose_total(t, gene_core::species::G)).sum();
            println!(
                "tick {} phase {:5.2} | agents {} on moss {:4.1}% (moss is {:.1}% of the map) | G per moss tile {:.1}",
                sim.st.tick,
                (sim.st.tick % epoch) as f64 / epoch as f64,
                sim.st.agents.len(),
                100.0 * on as f64 / sim.st.agents.len().max(1) as f64,
                100.0 * area as f64 / w.n() as f64,
                g as f64 / area as f64
            );
        }
    }
}
