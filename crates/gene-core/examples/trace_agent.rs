//! Follow agents from a saved state: every effect they resolve (with outcome, direction, channel
//! and amount), relation facts they take part in, births and deaths, and their energy and
//! inventory every 256 ticks. Children born to a followed agent are followed too.
//!
//! usage: trace_agent SAVE.bin TICKS UID[,UID...]
use gene_core::behavior::describe;
use gene_core::sim::Sim;
use gene_core::species::*;
use std::collections::BTreeSet;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut sim = Sim::load(&std::fs::read(&args[0]).unwrap()).unwrap();
    let ticks: u64 = args[1].parse().unwrap();
    let mut follow: BTreeSet<u64> = args[2].split(',').map(|u| u.parse().unwrap()).collect();
    sim.obs.facts = Some(Vec::new());
    sim.obs.rel = Some(Vec::new());
    let end = sim.st.tick + ticks;
    while sim.st.tick < end {
        sim.step();
        let t = sim.st.tick;
        for b in sim.obs.births.drain(..) {
            if follow.contains(&b.parent) {
                println!("{t} BIRTH parent {} child {} at ({}, {}) inv_g {}", b.parent, b.uid, b.x, b.y, b.inv_g);
                follow.insert(b.uid);
            }
        }
        for d in sim.obs.deaths.drain(..) {
            if follow.contains(&d.uid) {
                println!("{t} DEATH {} age {} children {}", d.uid, d.age, d.children);
            }
        }
        for f in sim.obs.facts.as_mut().unwrap().drain(..) {
            if follow.contains(&f.uid) {
                let a = sim.st.agents.iter().find(|a| a.uid == f.uid);
                let (x, y, o) = a.map_or((0, 0, 0), |a| (a.x, a.y, a.orient));
                println!("{} {} {} actual {} at ({x}, {y}) facing {o}", f.tick, f.uid, describe(&gene_core::behavior::token(&f, 0, false)), f.actual);
            }
        }
        for r in sim.obs.rel.as_mut().unwrap().drain(..) {
            if follow.contains(&r.from.1) || follow.contains(&r.to.1) {
                println!("{} REL kind {} {:?} -> {:?} channel {} amount {}", r.tick, r.kind, r.from, r.to, r.channel, r.amount);
            }
        }
        if t % 256 == 0 {
            for a in sim.st.agents.iter().filter(|a| follow.contains(&a.uid)) {
                println!(
                    "{t} STATE {} energy {} inv G {} S {} age {:.2}d embryo {} at ({}, {}) pc {}:{}",
                    a.uid,
                    a.energy,
                    a.inv[G],
                    a.inv[S],
                    a.age as f64 / 4096.0,
                    a.embryo_state_code(),
                    a.x,
                    a.y,
                    a.vm.module,
                    a.vm.pc
                );
            }
        }
    }
}
