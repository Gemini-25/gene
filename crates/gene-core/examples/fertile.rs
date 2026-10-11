//! Moss ground in a saved state (SPEC §20k, §20m): tiles per substrate class, loose G on moss
//! ground and elsewhere, and the share of agents standing on or next to moss ground. Moss ground
//! is the wettest substrate, or the blooms of the current epoch when the ruleset has them.
//! usage: fertile SAVE.bin
fn main() {
    let sim = gene_core::Sim::load(&std::fs::read(std::env::args().nth(1).unwrap()).unwrap()).unwrap();
    let w = &sim.st.world;
    let mask = gene_core::env::moss_ground(w, sim.st.config.seed, sim.caps.bloom, sim.st.tick);
    let mut n = [0u64; 4];
    let (mut g_f, mut g_o) = (0u64, 0u64);
    for t in 0..(w.w * w.h) {
        let s = w.substrate[t as usize].min(3) as usize;
        n[s] += 1;
        let g = w.loose_total(t, gene_core::species::G);
        if mask[t as usize] != 0 {
            g_f += g;
        } else {
            g_o += g;
        }
    }
    let fert = mask.iter().filter(|&&m| m != 0).count() as u64;
    let other = n.iter().sum::<u64>() - fert;
    println!("tiles by substrate 0..3: {:?}; moss ground {:.1}%", n, 100.0 * fert as f64 / (fert + other) as f64);
    println!("loose G per tile: fertile {:.2}, other {:.2}", g_f as f64 / fert.max(1) as f64, g_o as f64 / other.max(1) as f64);
    let fertile = |t: u32| mask[t as usize] != 0;
    let (mut on, mut near) = (0usize, 0usize);
    for a in &sim.st.agents {
        let t = w.idx(a.x as u32, a.y as u32);
        if fertile(t) {
            on += 1;
        } else if (0..4).filter_map(|d| w.neighbor(t, d)).any(fertile) {
            near += 1;
        }
    }
    let border = (0..(w.w * w.h)).filter(|&t| !fertile(t) && (0..4).filter_map(|d| w.neighbor(t, d)).any(fertile)).count();
    println!("tiles bordering fertile ground: {border}");
    let (mut sx, mut sy, mut c) = (0u64, 0u64, 0u64);
    for t in (0..(w.w * w.h)).filter(|&t| fertile(t)) {
        let (x, y) = w.xy(t);
        sx += x as u64;
        sy += y as u64;
        c += 1;
    }
    if c > 0 {
        println!("centre of fertile ground: ({}, {})", sx / c, sy / c);
    }
    let n = sim.st.agents.len().max(1) as f64;
    println!("agents {}: on fertile ground {:.1}%, next to it {:.1}%", sim.st.agents.len(), 100.0 * on as f64 / n, 100.0 * near as f64 / n);
}
