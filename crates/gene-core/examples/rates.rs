//! Heritable mutation rates in a saved state: how many agents carry each (point, structural)
//! threshold pair of their genome header.
//! usage: rates SAVE.bin [SAVE.bin ...]
use std::collections::BTreeMap;
fn main() {
    for path in std::env::args().skip(1) {
        let sim = gene_core::Sim::load(&std::fs::read(&path).unwrap()).unwrap();
        let mut n: BTreeMap<(u16, u16), usize> = BTreeMap::new();
        for a in &sim.st.agents {
            let p = a.program();
            *n.entry((p.point_threshold, p.structural_threshold)).or_default() += 1;
        }
        let mut v: Vec<_> = n.into_iter().collect();
        v.sort_by_key(|x| std::cmp::Reverse(x.1));
        let s: Vec<String> = v.iter().take(6).map(|((p, st), c)| format!("point {p} structural {st}: {c}")).collect();
        println!("{path} (day {:.0}, {} agents): {}", sim.st.tick as f64 / 4096.0, sim.st.agents.len(), s.join("; "));
    }
}
