use gene_core::rng::*;
use std::time::Instant;
fn main() {
    let key = flux_key(1, FluxDomain::EdgeFlux, 1);
    let n = 10_000_000u64;
    let t = Instant::now();
    let mut acc = 0u32;
    for i in 0..n { acc ^= philox([i as u32, 0, 7, 0], key)[0]; }
    let dt = t.elapsed().as_secs_f64();
    println!("philox {:.1} ns/call ({acc})", dt * 1e9 / n as f64);
    // count SR calls needing RNG in a real state
    let prog = gene_core::isa::assemble(gene_core::FOUNDER5_ASM).unwrap();
    let mut sim = gene_core::sim::Sim::with_program(gene_core::sim::Config { seed: 1, width: 512, height: 512, agents: 1000 }, &prog).unwrap();
    for _ in 0..512 { sim.step(); }
    let w = &sim.st.world;
    let tick = sim.st.tick;
    let bucket = (tick % 8) as usize;
    let (mut edges, mut need, mut nonzero) = (0u64, 0u64, 0u64);
    for c in 0..4 { for &(a, b) in &w.edges[bucket*4+c] {
        edges += 1;
        for s in [1usize, 2, 4, 8] { let d = (w.loose[a as usize][s] as i64 - w.loose[b as usize][s] as i64).unsigned_abs();
            if d > 0 { nonzero += 1; if d % 8 != 0 { need += 1; } } }
        let d = (w.loose[a as usize][0] as i64 - w.loose[b as usize][0] as i64).unsigned_abs();
        if w.elev[a as usize] == w.elev[b as usize] && d % 32 != 0 { need += 1; }
    }}
    println!("edges/tick {edges}, gas pairs with gradient {nonzero}, SR draws needed {need}");
}
