//! Moss ground in a saved state (SPEC §20k): tiles per substrate class, and loose G on fertile
//! and other tiles.
//! usage: fertile SAVE.bin
fn main() {
    let sim = gene_core::Sim::load(&std::fs::read(std::env::args().nth(1).unwrap()).unwrap()).unwrap();
    let w = &sim.st.world;
    let mut n = [0u64; 4];
    let (mut g_f, mut g_o) = (0u64, 0u64);
    for t in 0..(w.w * w.h) {
        let s = w.substrate[t as usize].min(3) as usize;
        n[s] += 1;
        let g = w.loose_total(t, gene_core::species::G);
        if s >= gene_core::env::MOSS_SUBSTRATE as usize {
            g_f += g;
        } else {
            g_o += g;
        }
    }
    let fert = n[3];
    let other = n.iter().sum::<u64>() - fert;
    println!("tiles by substrate 0..3: {:?}; fertile {:.1}%", n, 100.0 * fert as f64 / (fert + other) as f64);
    println!("loose G per tile: fertile {:.2}, other {:.2}", g_f as f64 / fert.max(1) as f64, g_o as f64 / other.max(1) as f64);
}
