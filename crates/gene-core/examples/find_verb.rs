//! Agents in a saved state that do a given verb within the next ticks: steps the state and prints
//! the first agents whose successful effects match (opcode index 0..9 and sub-op, e.g. "1.3" for
//! WRITE CancelDraft), with the effect detail.
//! usage: find_verb SAVE.bin OP.SUB[:CHANNEL:SRC:DST] [TICKS] [MAX]  (e.g. 5.0:3:1:2 = TRANSFER G loose>parcel)
use gene_core::sim::Sim;
fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let mut sim = Sim::load(&std::fs::read(&a[0]).unwrap()).unwrap();
    let mut parts = a[1].split(':');
    let (op, sub) = parts.next().unwrap().split_once('.').map(|(o, s)| (o.parse::<u8>().unwrap(), s.parse::<u8>().unwrap())).unwrap();
    let want: Vec<u8> = parts.map(|v| v.parse().unwrap()).collect();
    let ticks: u64 = a.get(2).map_or(4096, |v| v.parse().unwrap());
    let max: usize = a.get(3).map_or(5, |v| v.parse().unwrap());
    sim.obs.facts = Some(Vec::new());
    let mut seen = std::collections::BTreeSet::new();
    for _ in 0..ticks {
        sim.step();
        for f in sim.obs.facts.as_mut().unwrap().drain(..) {
            let detail_ok = want.is_empty() || (f.channel == want[0] && f.src == want[1] && f.dst == want[2]);
            if f.kind == (op << 3 | sub) && f.outcome <= 1 && detail_ok && seen.insert(f.uid) {
                println!("tick {} uid {} outcome {} channel {} actual {} src {} dst {}", f.tick, f.uid, f.outcome, f.channel, f.actual, f.src, f.dst);
            }
        }
        if seen.len() >= max {
            break;
        }
    }
}
