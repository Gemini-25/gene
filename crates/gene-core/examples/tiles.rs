//! Dump static tile properties of a generated world as CSV: x,y,lpeak,elev,mu,water0
use gene_core::species::W;
fn main() {
    let a: Vec<u64> = std::env::args().skip(1).map(|s| s.parse().unwrap()).collect();
    let (seed, size) = (a[0], a[1] as u32);
    let w = gene_core::world::generate(seed, size, size);
    println!("x,y,lpeak,elev,mu,water0");
    for t in 0..w.n() as u32 {
        let (x, y) = w.xy(t);
        println!("{x},{y},{},{},{},{}", w.lpeak[t as usize], w.elev[t as usize], w.mu[t as usize], w.loose[t as usize][W]);
    }
}
