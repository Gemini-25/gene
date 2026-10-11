//! Where each species is held in a saved state: loose on tiles (with residue), in agents (body,
//! inventory, embryo), in dropped records and in parcels.
//! usage: matter_census SAVE.bin [SAVE.bin ...]
use gene_core::species::*;
fn main() {
    const NAMES: [&str; NSPEC] = ["W", "C", "O", "G", "H", "X", "M", "S", "V", "I"];
    for path in std::env::args().skip(1) {
        let sim = gene_core::Sim::load(&std::fs::read(&path).unwrap()).unwrap();
        let (st, w) = (&sim.st, &sim.st.world);
        let mut rows = [[0u64; NSPEC]; 4];
        for t in 0..w.n() as u32 {
            let s = w.loose_stock_u64(t);
            for k in 0..NSPEC {
                rows[0][k] += s[k];
            }
        }
        for a in &st.agents {
            let m = a.matter();
            for k in 0..NSPEC {
                rows[1][k] += m[k];
            }
        }
        for ms in st.dropped.values() {
            for md in ms {
                rows[2][G] += md.present_pages() as u64;
            }
        }
        for p in w.parcels.values() {
            let m = p.matter();
            for k in 0..NSPEC {
                rows[3][k] += m[k];
            }
        }
        println!("{path}: tick {} (day {:.0}), agents {}", st.tick, st.tick as f64 / 4096.0, st.agents.len());
        println!("{:8} {}", "", NAMES.map(|n| format!("{n:>10}")).join(""));
        for (name, r) in ["loose", "agents", "records", "parcels"].iter().zip(&rows) {
            println!("{name:8} {}", r.map(|v| format!("{v:>10}")).join(""));
        }
    }
}
