//! Uids of agents in a saved state whose genome hash starts with HASH (hex).
//! usage: agents_with SAVE.bin HASH [MAX]
fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let sim = gene_core::Sim::load(&std::fs::read(&a[0]).unwrap()).unwrap();
    let max: usize = a.get(2).map_or(5, |v| v.parse().unwrap());
    let hex = |b: &[u8]| b.iter().map(|x| format!("{x:02x}")).collect::<String>();
    let mut n = 0;
    for ag in &sim.st.agents {
        let h = gene_core::genome::hash_cells(ag.code.prefix(ag.used_len).unwrap_or(&[]));
        if hex(&h).starts_with(&a[1]) {
            println!("{} age {:.1}d energy {} gen {}", ag.uid, ag.age as f64 / 4096.0, ag.energy, ag.generation);
            n += 1;
            if n >= max {
                break;
            }
        }
    }
}
