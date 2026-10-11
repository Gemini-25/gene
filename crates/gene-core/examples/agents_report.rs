//! Who is alive in a saved state: age, generation, energy, inventory G, embryo state, children,
//! where the VM is (module, pc), the most common (module, pc) spots, and agents with short code
//! (under 400 cells: a main module overwritten by a hook, e.g. by mating, SPEC §20n).
//! usage: agents_report SAVE.bin
use std::collections::BTreeMap;
fn main() {
    let sim = gene_core::Sim::load(&std::fs::read(std::env::args().nth(1).unwrap()).unwrap()).unwrap();
    let st = &sim.st;
    let day = 4096u64;
    let mut ages: Vec<u64> = st.agents.iter().map(|a| a.age).collect();
    ages.sort_unstable();
    let q = |v: &[u64], f: f64| v[((v.len() as f64 - 1.0) * f) as usize];
    println!("tick {} (day {:.1}) agents {}", st.tick, st.tick as f64 / day as f64, st.agents.len());
    if ages.is_empty() {
        return;
    }
    println!("age days: min {:.1} q1 {:.1} median {:.1} q3 {:.1} max {:.1}", q(&ages, 0.0) as f64 / 4096.0, q(&ages, 0.25) as f64 / 4096.0, q(&ages, 0.5) as f64 / 4096.0, q(&ages, 0.75) as f64 / 4096.0, q(&ages, 1.0) as f64 / 4096.0);
    let mut gens: Vec<u64> = st.agents.iter().map(|a| a.generation as u64).collect();
    gens.sort_unstable();
    println!("generation: min {} median {} max {}", gens[0], q(&gens, 0.5), gens[gens.len() - 1]);
    let mut emb: BTreeMap<u16, usize> = BTreeMap::new();
    let mut spots: BTreeMap<(u8, u16), usize> = BTreeMap::new();
    let (mut e_sum, mut g_sum, mut kids, mut rich) = (0u64, 0u64, 0u64, 0usize);
    for a in &st.agents {
        *emb.entry(a.embryo_state_code()).or_default() += 1;
        *spots.entry((a.vm.module, a.vm.pc)).or_default() += 1;
        e_sum += a.energy as u64;
        g_sum += a.inv[gene_core::species::G] as u64;
        kids += a.children as u64;
        rich += (a.energy >= 50000) as usize;
    }
    let n = st.agents.len() as u64;
    println!("mean energy {} (>= 50000: {}), mean inventory G {}, mean children so far {:.2}", e_sum / n, rich, g_sum / n, kids as f64 / n as f64);
    println!("embryo state counts (0 none, 1 writing, 2 sealed, 3 invalid, 4 ready): {emb:?}");
    let mut top: Vec<_> = spots.into_iter().collect();
    top.sort_by_key(|x| std::cmp::Reverse(x.1));
    println!("most common VM spots (module, pc): {:?}", &top[..top.len().min(8)]);
    let short: Vec<_> = st.agents.iter().filter(|a| a.used_len < 400).collect();
    let kids_short: u64 = short.iter().map(|a| a.children as u64).sum();
    let mut sa: Vec<u64> = short.iter().map(|a| a.age).collect();
    sa.sort_unstable();
    let med = if sa.is_empty() { 0.0 } else { q(&sa, 0.5) as f64 / 4096.0 };
    println!("code under 400 cells: {} agents, children so far {}, median age {:.1} days", short.len(), kids_short, med);
}
