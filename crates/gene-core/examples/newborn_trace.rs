//! Follow the newborns of the next day from a saved state: energy, inventory and VM state
//! every 64 ticks until death or one day. Prints a summary and a few traces.
//!
//! usage: newborn_trace SAVE.bin
use gene_core::sim::Sim;
use gene_core::species::*;
use std::collections::BTreeMap;

fn main() {
    let bytes = std::fs::read(std::env::args().nth(1).unwrap()).unwrap();
    let mut sim = Sim::load(&bytes).unwrap();
    sim.threads = 2;
    let start = sim.st.tick;
    // uid -> (birth tick, samples of (age, energy, inv G, inv S, pc, module, embryo state, photo credit))
    let mut tr: BTreeMap<u64, (u64, Vec<(u64, u32, u16, u16, u16, u8, u16, u32)>)> = BTreeMap::new();
    let mut died: BTreeMap<u64, u64> = BTreeMap::new();
    while sim.st.tick < start + 2 * 4096 {
        sim.step();
        for b in sim.obs.births.drain(..) {
            if b.tick < start + 4096 {
                tr.insert(b.uid, (b.tick, Vec::new()));
            }
        }
        for d in sim.obs.deaths.drain(..) {
            died.insert(d.uid, d.tick);
        }
        if sim.st.tick % 64 == 0 {
            for a in &sim.st.agents {
                if let Some((t0, v)) = tr.get_mut(&a.uid) {
                    if sim.st.tick - *t0 <= 4096 {
                        v.push((sim.st.tick - *t0, a.energy, a.inv[G], a.inv[S], a.vm.pc, a.vm.module, a.embryo_state_code(), a.photo_credit));
                        if v.len() == 1 {
                            let w = &sim.st.world;
                            let t = w.idx(a.x as u32, a.y as u32);
                            if *t0 % 7 == 0 {
                                println!("uid {} inv {:?} vol {} / {} theta {} tile C {} W {} O-space {}", a.uid, a.inv, a.inv_volume(), gene_core::agent::INV_VOLUME, w.theta(t), w.loose_total(t, C), w.loose_total(t, W), w.loose_space(t, O));
                            }
                        }
                    }
                }
            }
        }
    }
    let mut early = 0;
    let mut first_e = Vec::new();
    for (uid, (t0, v)) in &tr {
        let age = died.get(uid).map(|d| d - t0);
        if age.is_some_and(|a| a < 1024) {
            early += 1;
        }
        if let Some(s) = v.first() {
            first_e.push(s.1);
        }
    }
    first_e.sort();
    println!("newborns {}  died within 1024 ticks {}  energy at first sample: median {}", tr.len(), early, first_e.get(first_e.len() / 2).copied().unwrap_or(0));
    for (uid, (t0, v)) in tr.iter().filter(|(u, (t0, _))| died.get(u).is_some_and(|d| d - t0 < 1024)).take(4) {
        println!("uid {uid} born {t0} (phase {}) died at age {}", (t0 + 1024) % 4096, died[uid] - t0);
        for s in v {
            println!("  age {:>5} energy {:>6} G {:>3} S {:>3} pc {:>4} module {} embryo {} photo {}", s.0, s.1, s.2, s.3, s.4, s.5, s.6, s.7);
        }
    }
}
