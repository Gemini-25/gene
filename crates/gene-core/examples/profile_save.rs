//! Instruction-count profiling of a saved state (valgrind --tool=callgrind).
//! usage: profile_save SAVE.bin [ticks]
fn main() {
    let mut sim = gene_core::Sim::load(&std::fs::read(std::env::args().nth(1).unwrap()).unwrap()).unwrap();
    let ticks: u64 = std::env::args().nth(2).map_or(32, |v| v.parse().unwrap());
    for _ in 0..ticks {
        sim.step();
    }
    println!("tick {} pop {}", sim.st.tick, sim.st.agents.len());
}
