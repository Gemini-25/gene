use gene_core::sim::{Config, Sim};
use gene_core::species::*;

fn main() {
    let args: Vec<u64> = std::env::args().skip(1).map(|a| a.parse().unwrap()).collect();
    let (seed, size, n, ticks) = (args[0], args[1] as u32, args[2] as u32, args[3]);
    let mut sim = Sim::new(Config { seed, width: size, height: size, agents: n }).unwrap();
    println!("founder raw cells {} words {}", sim.st.agents[0].used_len, sim.st.agents[0].program().words.len());
    let t0 = std::time::Instant::now();
    for k in 0..ticks {
        sim.step();
        if (k + 1) % (ticks / 20).max(1) == 0 {
            let acc = sim.obs.take_acc();
            let ag = &sim.st.agents;
            let pop = ag.len();
            let mean_e = ag.iter().map(|a| a.energy as u64).sum::<u64>() / pop.max(1) as u64;
            let mean_g = ag.iter().map(|a| a.inv[G] as u64).sum::<u64>() as f64 / pop.max(1) as f64;
            let emb: Vec<usize> = (0..5).map(|s| ag.iter().filter(|a| a.embryo_state_code() == s).count()).collect();
            let maxgen = ag.iter().map(|a| a.generation).max().unwrap_or(0);
            let at = acc.agent_ticks.max(1) as f64;
            println!("        per agent-tick: upkeep {:.1} vm {:.1} attempt {:.1} effect {:.1} | photoG/agent-tick {:.4}",
                acc.upkeep_q as f64 / at, acc.vm_q as f64 / at, acc.attempt_q as f64 / at, acc.effect_q as f64 / at,
                acc.photosynthesis as f64 / at);
            println!(
                "t={:6} pop={:5} births={:5} dE={:4} dB={:3} E={:5} G={:5.1} emb={:?} gen={} photo={} resp={} moves={} inert={}",
                sim.st.tick, pop, sim.obs.total_births, sim.obs.total_deaths_energy, sim.obs.total_deaths_body,
                mean_e, mean_g, emb, maxgen, acc.photosynthesis, acc.respiration, acc.moves, acc.inert
            );
        }
    }
    let dt = t0.elapsed().as_secs_f64();
    println!("TPS {:.1}  audit_ok {}", ticks as f64 / dt, sim.audit_ok());
    let names = ["READ","WRITE","DRIVE","LINK","UNLINK","TRANSFER","WORK","EMIT","REPRO","COPY"];
    for (k, row) in sim.obs.outcomes.iter().enumerate() {
        if row.iter().sum::<u64>() > 0 { println!("{:9} {:?}", names[k], row); }
    }
    for (k, row) in sim.obs.repro.iter().enumerate() {
        if row.iter().sum::<u64>() > 0 { println!("REPRO.{} {:?}", k, row); }
    }
    println!("{:?}", sim.obs.mutation);
}
