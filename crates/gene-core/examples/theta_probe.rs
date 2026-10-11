//! Temperature over a day: every 256 ticks, the mean and minimum tile temperature (K) over all
//! tiles and over tiles with an agent, and the share of agents on tiles outside the respiration
//! range (260..340 K).
//! usage: theta_probe SAVE.bin [TICKS]
fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let mut sim = gene_core::Sim::load(&std::fs::read(&a[0]).unwrap()).unwrap();
    let ticks: u64 = a.get(1).map_or(4096, |v| v.parse().unwrap());
    let end = sim.st.tick + ticks;
    while sim.st.tick < end {
        sim.step();
        if sim.st.tick % 256 == 0 {
            let w = &sim.st.world;
            let n = w.w as u64 * w.h as u64;
            let (mut sum, mut min) = (0u64, u64::MAX);
            for t in 0..n as u32 {
                let th = w.theta(t) as u64;
                sum += th;
                min = min.min(th);
            }
            let (mut asum, mut amin, mut out) = (0u64, u64::MAX, 0usize);
            for ag in &sim.st.agents {
                let th = w.theta(w.idx(ag.x as u32, ag.y as u32)) as u64;
                asum += th;
                amin = amin.min(th);
                out += !(260..=340).contains(&th) as usize;
            }
            let na = sim.st.agents.len().max(1) as u64;
            println!(
                "tick {} (day phase {:4}): tiles mean {} min {} | agent tiles mean {} min {} | outside 260..340: {} of {}",
                sim.st.tick, sim.st.tick % 4096, sum / n, min, asum / na, if amin == u64::MAX { 0 } else { amin }, out, sim.st.agents.len()
            );
        }
    }
}
