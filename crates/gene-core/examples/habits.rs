//! Action profile per group: run a saved state for TICKS and count, per agent-day, every effect the
//! group's agents resolve, keyed by the judge's verb (op, channel, source>destination; SPEC §17f),
//! ok and failed, plus births, deaths and mean energy.
//! Groups are founder slots ((root - 1) mod N for `slotN`; slot2 is scripts/compete.py's split, slot3
//! the E2 founder types) or, with `hash`, the first 8 hex digits of the agent's genome hash at the
//! start (children inherit the parent's group).
//!
//! usage: habits SAVE.bin TICKS [slotN|hash] [TOP]
use gene_core::behavior::{describe_verb, verb};
use gene_core::observer::EffectFact;
use gene_core::sim::Sim;
use std::collections::{BTreeMap, HashMap};

#[derive(Default)]
struct Group {
    agent_ticks: u64,
    energy: u64,
    births: u64,
    deaths: u64,
    death_age: u64,
    /// verb (outcome forced to Ok so failures share the key) -> (ok, failed, amount when ok)
    acts: HashMap<u32, (u64, u64, u64)>,
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let mut sim = Sim::load(&std::fs::read(&a[0]).unwrap()).unwrap();
    let ticks: u64 = a[1].parse().unwrap();
    let mode = a.get(2).map_or("slot2", |m| m.as_str());
    let slots: u64 = mode.strip_prefix("slot").map_or(2, |n| n.parse().unwrap());
    let top: usize = a.get(3).map_or(24, |v| v.parse().unwrap());
    let hex = |b: &[u8]| b.iter().take(4).map(|x| format!("{x:02x}")).collect::<String>();
    let mut group: HashMap<u64, String> = HashMap::new();
    for ag in &sim.st.agents {
        let g = if mode == "hash" {
            hex(&gene_core::genome::hash_cells(ag.code.prefix(ag.used_len).unwrap_or(&[])))
        } else {
            format!("slot{}", (ag.root - 1) % slots)
        };
        group.insert(ag.uid, g);
    }
    let of = |group: &HashMap<u64, String>, uid: u64| group.get(&uid).cloned().unwrap_or_else(|| "?".into());
    let mut groups: BTreeMap<String, Group> = BTreeMap::new();
    sim.obs.facts = Some(Vec::new());
    let start = sim.st.tick;
    while sim.st.tick < start + ticks {
        sim.step();
        for b in sim.obs.births.drain(..) {
            let g = of(&group, b.parent);
            groups.entry(g.clone()).or_default().births += 1;
            group.insert(b.uid, g);
        }
        for d in sim.obs.deaths.drain(..) {
            let e = groups.entry(of(&group, d.uid)).or_default();
            e.deaths += 1;
            e.death_age += d.age;
        }
        for f in sim.obs.facts.as_mut().unwrap().drain(..) {
            let ok = f.outcome <= 1;
            let Some(v) = verb(&EffectFact { outcome: 0, ..f }) else { continue };
            let e = groups.entry(of(&group, f.uid)).or_default().acts.entry(v).or_default();
            if ok {
                e.0 += 1;
                e.2 += f.actual as u64;
            } else {
                e.1 += 1;
            }
        }
        for ag in &sim.st.agents {
            let e = groups.entry(of(&group, ag.uid)).or_default();
            e.agent_ticks += 1;
            e.energy += ag.energy as u64;
        }
    }
    let names: Vec<&String> = groups.keys().filter(|k| groups[*k].agent_ticks > 0).collect();
    println!("{} from tick {start} for {ticks} ticks (per agent-day: ok / failed, mean amount when ok)", a[0]);
    for n in &names {
        let g = &groups[*n];
        println!(
            "{n}: mean living {:.1}, mean energy {}, births/agent-day {:.3}, deaths {} (mean age {:.1} d)",
            g.agent_ticks as f64 / ticks as f64,
            g.energy / g.agent_ticks.max(1),
            g.births as f64 * 4096.0 / g.agent_ticks.max(1) as f64,
            g.deaths,
            g.death_age as f64 / g.deaths.max(1) as f64 / 4096.0
        );
    }
    let rate = |g: &Group, v: u32| {
        let (ok, bad, amt) = g.acts.get(&v).copied().unwrap_or_default();
        let per = 4096.0 / g.agent_ticks.max(1) as f64;
        (ok as f64 * per, bad as f64 * per, if ok > 0 { amt as f64 / ok as f64 } else { 0.0 })
    };
    let mut keys: Vec<u32> = groups.values().flat_map(|g| g.acts.keys().copied()).collect();
    keys.sort();
    keys.dedup();
    let weight = |v: u32| names.iter().map(|n| { let (o, b, _) = rate(&groups[*n], v); o + b }).sum::<f64>();
    keys.sort_by(|x, y| weight(*y).partial_cmp(&weight(*x)).unwrap());
    print!("{:<28}", "verb");
    for n in &names {
        print!(" | {:>26}", n);
    }
    println!();
    for v in keys.iter().take(top) {
        print!("{:<28}", describe_verb(*v));
        for n in &names {
            let (o, b, amt) = rate(&groups[*n], *v);
            print!(" | {:>8.2} / {:>7.2} ×{:>6.1}", o, b, amt);
        }
        println!();
    }
}
