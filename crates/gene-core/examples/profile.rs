//! Fixed workload for instruction-count profiling (valgrind --tool=callgrind).
//! usage: profile [size] [warmup] [ticks]
use gene_core::sim::{Config, Sim};
fn main() {
    let arg = |k: usize, d: u64| std::env::args().nth(k).map_or(d, |v| v.parse().unwrap());
    let (size, warm, ticks) = (arg(1, 256) as u32, arg(2, 256), arg(3, 64));
    let prog = gene_core::isa::assemble(gene_core::FOUNDER5_ASM).unwrap();
    let mut sim = Sim::with_program(Config { seed: 1, width: size, height: size, agents: (size * size / 262) as u32 }, &prog).unwrap();
    for _ in 0..warm {
        sim.step();
    }
    for _ in 0..ticks {
        sim.step();
    }
    println!("tick {} pop {} hash {:02x?}", sim.st.tick, sim.st.agents.len(), &sim.state_hash()[..6]);
}
