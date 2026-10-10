use gene_core::sim::{Config, Sim};
use std::time::Instant;
fn main() {
    let threads: usize = std::env::args().nth(1).map_or(1, |t| t.parse().unwrap());
    // optional second argument: a saved state to time instead of the Founder-5 start
    let mut sim = match std::env::args().nth(2) {
        Some(path) => Sim::load(&std::fs::read(path).unwrap()).unwrap(),
        None => {
            let prog = gene_core::isa::assemble(gene_core::FOUNDER5_ASM).unwrap();
            let mut sim = Sim::with_program(Config { seed: 1, width: 512, height: 512, agents: 1000 }, &prog).unwrap();
            for _ in 0..512 { sim.step(); }
            sim
        }
    };
    sim.threads = threads;
    let mut t = [0f64; 10];
    let n = 1024;
    for _ in 0..n {
        let mut c = Instant::now();
        let mut lap = |k: usize, c: &mut Instant| { t[k] += c.elapsed().as_secs_f64(); *c = Instant::now(); };
        sim.bench_stage(0); lap(0, &mut c);
        sim.bench_stage(1); lap(1, &mut c);
        sim.bench_stage(2); lap(2, &mut c);
        sim.bench_stage(3); lap(3, &mut c);
        sim.bench_stage(4); lap(4, &mut c);
        sim.bench_stage(5); lap(5, &mut c);
        sim.bench_stage(6); lap(6, &mut c);
        sim.bench_stage(7); lap(7, &mut c);
        sim.bench_stage(8); lap(8, &mut c);
        sim.bench_stage(9); lap(9, &mut c);
    }
    let names = ["maint","think","seal","local","motion","transport","env","metab","life","commit"];
    let tot: f64 = t.iter().sum();
    for k in 0..10 { println!("{:10} {:8.3} ms/tick {:5.1}%", names[k], t[k]*1000.0/n as f64, 100.0*t[k]/tot); }
    println!("total {:.3} ms/tick -> {:.0} TPS, pop {}", tot*1000.0/n as f64, n as f64/tot, sim.st.agents.len());
    let a = Instant::now(); let _ = sim.audit(); println!("audit {:.1} ms", a.elapsed().as_secs_f64()*1000.0);
    let a = Instant::now(); let _ = sim.state_hash(); println!("hash {:.1} ms", a.elapsed().as_secs_f64()*1000.0);
}
