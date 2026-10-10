//! Where do newborns start? Load a saved state, describe the ground under newborns and adults,
//! then follow the births of the next day and relate one-day survival to the air C at the
//! birth tile. Used for docs/results/E1.md (E1-c, late decline).
//!
//! usage: newborn_env SAVE.bin [DAYS_TO_FOLLOW]

use gene_core::sim::Sim;
use gene_core::species::*;
use std::collections::BTreeMap;

const DAY: u64 = 4096;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let bytes = std::fs::read(&args[1]).expect("save file");
    let follow: u64 = args.get(2).map_or(1, |d| d.parse().unwrap());
    let mut sim = Sim::load(&bytes).expect("load");
    sim.threads = 2;
    let w = &sim.st.world;
    let n = w.n() as u32;
    let all_c: u64 = (0..n).map(|t| w.loose_total(t, C)).sum();
    println!("tick {}  pop {}  mean air C per tile {:.2}", sim.st.tick, sim.st.agents.len(), all_c as f64 / n as f64);
    // ground under agents by age class
    let mut by: BTreeMap<&str, Vec<(u64, u64, u64, u64)>> = BTreeMap::new();
    let mut zero_w: BTreeMap<&str, u64> = BTreeMap::new();
    for a in &sim.st.agents {
        let t = w.idx(a.x as u32, a.y as u32);
        let class = if a.age < 512 {
            "a: newborn <512"
        } else if a.age < DAY {
            "b: young <1 day"
        } else {
            "c: adult"
        };
        by.entry(class).or_default().push((w.loose_total(t, C), w.loose_total(t, W), a.inv[G] as u64, a.energy as u64));
        *zero_w.entry(class).or_default() += (w.loose_total(t, W) == 0) as u64;
    }
    let all_w0 = (0..n).filter(|&t| w.loose_total(t, W) == 0).count() as f64 / n as f64;
    let all_dry = (0..n).filter(|&t| w.loose_total(t, W) + w.loose_total(t, I) == 0).count() as f64 / n as f64;
    println!("tiles with no water at all (W + I == 0): {:.0}% of the map", 100.0 * all_dry);
    println!("tiles with W == 0: {:.0}% of the map", 100.0 * all_w0);
    println!("{:<18} {:>6} {:>8} {:>8} {:>9} {:>8} {:>8} {:>8}", "class", "n", "tile C", "C == 0", "tile W", "W == 0", "inv G", "energy");
    for (k, v) in &by {
        let m = |f: &dyn Fn(&(u64, u64, u64, u64)) -> u64| v.iter().map(f).sum::<u64>() as f64 / v.len() as f64;
        let zero = v.iter().filter(|x| x.0 == 0).count() as f64 / v.len() as f64;
        println!(
            "{:<18} {:>6} {:>8.2} {:>7.0}% {:>9.2} {:>7.0}% {:>8.2} {:>8.0}",
            k,
            v.len(),
            m(&|x| x.0),
            100.0 * zero,
            m(&|x| x.1),
            100.0 * zero_w[k] as f64 / v.len() as f64,
            m(&|x| x.2),
            m(&|x| x.3)
        );
    }
    // follow the births of the next days
    let start = sim.st.tick;
    let mut born: BTreeMap<u64, (u64, u64, u64)> = BTreeMap::new(); // uid -> (tick, tile C, tile W)
    let mut died: BTreeMap<u64, u64> = BTreeMap::new();
    while sim.st.tick < start + (follow + 1) * DAY {
        sim.step();
        for b in sim.obs.births.drain(..) {
            if b.tick < start + follow * DAY {
                let t = sim.st.world.idx(b.x as u32, b.y as u32);
                born.insert(b.uid, (b.tick, sim.st.world.loose_total(t, C), sim.st.world.loose_total(t, W)));
            }
        }
        for d in sim.obs.deaths.drain(..) {
            died.insert(d.uid, d.tick);
        }
    }
    let bin = |c: u64| match c {
        0 => 0,
        1..=3 => 1,
        4..=7 => 4,
        8..=15 => 8,
        _ => 16,
    };
    let label = |b: u64| match b {
        0 => "0".to_string(),
        1 => "1..3".into(),
        4 => "4..7".into(),
        8 => "8..15".into(),
        _ => "16+".into(),
    };
    let mut by_c: BTreeMap<u64, (u64, u64)> = BTreeMap::new();
    let mut by_w: BTreeMap<u64, (u64, u64)> = BTreeMap::new();
    let mut by_both: BTreeMap<bool, (u64, u64)> = BTreeMap::new();
    for (uid, (t, c, w)) in &born {
        let ok = died.get(uid).is_none_or(|d| d - t >= DAY) as u64;
        for (m, k) in [(&mut by_c, bin(*c)), (&mut by_w, bin(*w))] {
            let e = m.entry(k).or_default();
            e.0 += ok;
            e.1 += 1;
        }
        let e = by_both.entry(*c > 0 && *w > 0).or_default();
        e.0 += ok;
        e.1 += 1;
    }
    println!("\nbirths in the next {follow} day(s): one-day survival by the birth tile");
    for (name, m) in [("air C", &by_c), ("water W", &by_w)] {
        for (b, (ok, all)) in m {
            println!("  {name} {:<6} survival {:>5.1}%  n={}", label(*b), 100.0 * *ok as f64 / *all as f64, all);
        }
    }
    for (both, (ok, all)) in &by_both {
        println!("  photosynthesis possible (C > 0 and W > 0): {both:<5} survival {:>5.1}%  n={}", 100.0 * *ok as f64 / *all as f64, all);
    }
}
