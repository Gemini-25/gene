//! Headless runner for Gene M0. SPEC §17, §18.

use gene_core::genome::hash_cells;
use gene_core::isa;
use gene_core::observer::DeathCause;
use gene_core::sim::{Config, Sim};
use gene_core::species::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

mod serve;

const USAGE: &str = "gene — Gene M0 headless runner

USAGE:
  gene run    [--seed N] [--size S] [--agents A] [--ticks T] [--founder NAME|FILE[+NAME|FILE...]]
              [--out DIR] [--stats-every K] [--map-every M] [--checkpoint-every C]
              [--audit-every K] [--keep-going] [--save-final] [--quiet] [--ruleset m0|m0b|m1a..m1f|m2] [--sim-threads N]
  gene batch  --seeds A-B|A,B,C-D [--threads N] [same options as run]  (one directory per seed)
  gene resume --from CHECKPOINT [--ticks T] [--out DIR] [...]
  gene serve  [--port 8080] [--load SAVE.bin] [same world options as run]
              local web viewer: map, play/pause/speed, agent inspection, genome census
  gene asm    FILE.asm                   assemble, print size, hash and disassembly
  gene inspect SAVE.bin [--founder NAME]  genome census of a saved state with diffs against the founder
  gene founders                          list built-in founders

Defaults follow SPEC §3: size 512, agents 1000, founder founder4.
Several founders joined with + share one world (E2): founder i runs program i mod n.
Runs stop early at extinction unless --keep-going is given.";

#[derive(Clone, Debug)]
struct Opts {
    seed: u64,
    size: u32,
    agents: u32,
    ticks: u64,
    founder: String,
    out: PathBuf,
    stats_every: u64,
    map_every: u64,
    checkpoint_every: u64,
    audit_every: u64,
    keep_going: bool,
    quiet: bool,
    save_final: bool,
    ruleset: String,
    /// Threads inside one simulation (results are identical for any value).
    sim_threads: usize,
}

impl Default for Opts {
    fn default() -> Self {
        Opts {
            seed: 1,
            size: 512,
            agents: 1000,
            ticks: 200_000,
            founder: "founder4".into(),
            out: PathBuf::from("runs/out"),
            stats_every: 256,
            map_every: 16_384,
            checkpoint_every: 0,
            audit_every: 4096,
            keep_going: false,
            quiet: false,
            save_final: false,
            ruleset: gene_core::sim::RULESET.into(),
            sim_threads: 0,
        }
    }
}

fn die(msg: impl AsRef<str>) -> ! {
    eprintln!("error: {}", msg.as_ref());
    std::process::exit(2)
}

fn parse_opts(args: &[String], o: &mut Opts) -> BTreeMap<String, String> {
    let mut extra = BTreeMap::new();
    let mut i = 0;
    while i < args.len() {
        let k = args[i].as_str();
        let mut val = || {
            i += 1;
            args.get(i).cloned().unwrap_or_else(|| die(format!("{k} needs a value")))
        };
        let num = |v: String| v.replace('_', "").parse::<u64>().unwrap_or_else(|_| die(format!("bad number {v}")));
        match k {
            "--seed" => o.seed = num(val()),
            "--size" => o.size = num(val()) as u32,
            "--agents" => o.agents = num(val()) as u32,
            "--ticks" => o.ticks = num(val()),
            "--founder" => o.founder = val(),
            "--out" => o.out = PathBuf::from(val()),
            "--stats-every" => o.stats_every = num(val()).max(1),
            "--map-every" => o.map_every = num(val()),
            "--checkpoint-every" => o.checkpoint_every = num(val()),
            "--audit-every" => o.audit_every = num(val()),
            "--keep-going" => o.keep_going = true,
            "--quiet" => o.quiet = true,
            "--save-final" => o.save_final = true,
            "--sim-threads" => o.sim_threads = num(val()) as usize,
            "--ruleset" => {
                o.ruleset = match val().as_str() {
                    "m0" | "gene-m0-1" => gene_core::sim::RULESET.into(),
                    "m1" | "m1a" | "gene-m1-1" => gene_core::sim::RULESET_M1.into(),
                    "m1b" | "gene-m1-2" => gene_core::sim::RULESET_M1B.into(),
                    "m1c" | "gene-m1-3" => gene_core::sim::RULESET_M1C.into(),
                    "m1d" | "gene-m1-4" => gene_core::sim::RULESET_M1D.into(),
                    "m1e" | "gene-m1-5" => gene_core::sim::RULESET_M1E.into(),
                    "m1f" | "gene-m1-6" => gene_core::sim::RULESET_M1F.into(),
                    "m0b" | "gene-m0-2" => gene_core::sim::RULESET_M0B.into(),
                    "m2" | "gene-m2-1" => gene_core::sim::RULESET_M2.into(),
                    "m0c" | "gene-m0-3" => gene_core::sim::RULESET_M0C.into(),
                    "m2b" | "gene-m2-2" => gene_core::sim::RULESET_M2B.into(),
                    other => die(format!("unknown ruleset {other} (m0, m0b, m0c, m1a..m1f, m2, m2b)")),
                }
            }
            "--seeds" | "--threads" | "--from" | "--port" | "--load" => {
                let v = val();
                extra.insert(k.to_string(), v);
            }
            _ => die(format!("unknown option {k}\n\n{USAGE}")),
        }
        i += 1;
    }
    extra
}

fn founder_source(name: &str) -> String {
    match name {
        "founder4" => gene_core::FOUNDER_ASM.to_string(),
        "founder5" => gene_core::FOUNDER5_ASM.to_string(),
        "founder6" => gene_core::FOUNDER6_ASM.to_string(),
        "founder7" => gene_core::FOUNDER7_ASM.to_string(),
        "founder8" => gene_core::FOUNDER8_ASM.to_string(),
        "founder9" => gene_core::FOUNDER9_ASM.to_string(),
        "founder10" => gene_core::FOUNDER10_ASM.to_string(),
        path => fs::read_to_string(path).unwrap_or_else(|e| die(format!("cannot read founder {path}: {e}"))),
    }
}

/// Founder programs of `--founder a+b+c` (E2: several public ancestors in one world; founder i
/// runs program i mod n).
fn founder_programs(spec: &str) -> Vec<gene_core::genome::Program> {
    spec.split('+').map(|n| isa::assemble(&founder_source(n)).unwrap_or_else(|e| die(format!("{n}: {e}")))).collect()
}

/// The founder program a program is nearest to (shortest listing of differences).
fn nearest<'a>(founders: &'a [gene_core::genome::Program], p: &gene_core::genome::Program) -> Option<&'a gene_core::genome::Program> {
    if founders.len() <= 1 {
        return founders.first();
    }
    founders.iter().min_by_key(|f| program_diff(f, p).len())
}

/// Founder type of a genome: the index of the founder program with exactly this genome.
fn founder_kind(founders: &[gene_core::genome::Program], genome: &[u8; 32]) -> u16 {
    founders.iter().position(|f| hash_cells(&f.to_cells()) == *genome).unwrap_or(0) as u16
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = args.first() else { die(USAGE) };
    let mut o = Opts::default();
    match cmd.as_str() {
        "run" => {
            parse_opts(&args[1..], &mut o);
            let sim = new_sim(&o);
            run(sim, &o);
        }
        "batch" => {
            let extra = parse_opts(&args[1..], &mut o);
            let seeds = extra.get("--seeds").unwrap_or_else(|| die("--seeds required, e.g. 1-20 or 1,3,5-7"));
            let seeds = parse_seeds(seeds).unwrap_or_else(|| die(format!("bad --seeds {seeds} (e.g. 1-20 or 1,3,5-7)")));
            let threads: usize = extra.get("--threads").map_or(4, |t| t.parse().unwrap());
            batch(&o, seeds, threads);
        }
        "resume" => {
            let extra = parse_opts(&args[1..], &mut o);
            let from = extra.get("--from").unwrap_or_else(|| die("--from CHECKPOINT required"));
            let bytes = fs::read(from).unwrap_or_else(|e| die(format!("{from}: {e}")));
            let mut sim = Sim::load(&bytes).unwrap_or_else(|e| die(format!("{from}: {e:?}")));
            sim.threads = o.sim_threads.max(1);
            o.seed = sim.st.config.seed;
            run(sim, &o);
        }
        "asm" => {
            let path = args.get(1).unwrap_or_else(|| die("asm FILE"));
            let src = founder_source(path);
            let p = isa::assemble(&src).unwrap_or_else(|e| die(e.to_string()));
            let cells = p.to_cells();
            println!(
                "modules {}  words {}  raw cells {}  pages {}  blake3 {}",
                p.modules.len(),
                p.words.len(),
                cells.len(),
                cells.len().div_ceil(128),
                hex(&hash_cells(&cells))
            );
            print!("{}", isa::disassemble(&p));
        }
        "serve" => {
            let extra = parse_opts(&args[1..], &mut o);
            let port = extra.get("--port").map_or(8080, |p| p.parse().unwrap_or_else(|_| die("bad --port")));
            if o.sim_threads == 0 {
                // interactive use: spare one core for the browser
                o.sim_threads = std::thread::available_parallelism().map_or(1, |n| n.get().saturating_sub(1).max(1));
            }
            let mut sim = match extra.get("--load") {
                Some(path) => {
                    let bytes = fs::read(path).unwrap_or_else(|e| die(format!("{path}: {e}")));
                    Sim::load(&bytes).unwrap_or_else(|e| die(format!("{path}: {e:?}")))
                }
                None => new_sim(&o),
            };
            sim.threads = o.sim_threads;
            serve::serve(sim, serve::ServeOpts { port, save_dir: o.out.clone(), founder: o.founder.clone() });
        }
        "inspect" => {
            let path = args.get(1).unwrap_or_else(|| die("inspect SAVE.bin"));
            parse_opts(&args[2..], &mut o);
            let bytes = fs::read(path).unwrap_or_else(|e| die(format!("{path}: {e}")));
            let sim = Sim::load(&bytes).unwrap_or_else(|e| die(format!("{path}: {e:?}")));
            print!("{}", genome_report(&sim, &o.founder, 5));
        }
        "founders" => {
            for name in ["founder4", "founder5", "founder6", "founder7", "founder8", "founder9", "founder10"] {
                let p = isa::assemble(&founder_source(name)).unwrap();
                let cells = p.to_cells();
                println!("{name}: {} words, {} cells, {}", p.words.len(), cells.len(), &hex(&hash_cells(&cells))[..16]);
            }
        }
        "-h" | "--help" | "help" => println!("{USAGE}"),
        _ => die(USAGE),
    }
}

fn new_sim(o: &Opts) -> Sim {
    let progs = founder_programs(&o.founder);
    let cfg = Config { seed: o.seed, width: o.size, height: o.size, agents: o.agents };
    let mut sim = Sim::with_programs_ruleset(cfg, &progs, &o.ruleset).unwrap_or_else(|e| die(format!("{e:?}")));
    sim.threads = o.sim_threads.max(1);
    sim
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

#[derive(Clone, Debug, Default)]
struct Summary {
    seed: u64,
    ticks_run: u64,
    extinct_at: Option<u64>,
    final_pop: usize,
    peak_pop: usize,
    births: u64,
    deaths_energy: u64,
    deaths_body: u64,
    max_generation: u32,
    distinct_genomes: usize,
    tps: f64,
    audit_ok: bool,
    hash: String,
    /// JSON object: effect outcome counts per opcode and the M1 counters.
    effects: String,
    /// JSON object: relation totals and group counts (original §58, §29).
    relations: String,
    /// JSON object: high-level judge counts (SPEC §17f).
    highlevel: String,
}

/// Effect outcomes per opcode (nonzero only) and the M1 counters, as a JSON object.
/// Behaviour families seen in at least 3 windows (original §58), oldest first.
fn behaviors_csv(an: &gene_core::behavior::Analyzer, now: u64) -> String {
    use gene_core::behavior::UseState;
    let mut out = String::from("family,first_tick,first_agent,last_tick,windows,near,agents,repeated_at,state_at_end,example\n");
    for f in an.families.iter().filter(|f| f.windows >= 3) {
        let ex: Vec<String> = f.example.iter().map(gene_core::behavior::describe).collect();
        out += &format!(
            "{},{},{},{},{},{},{},{},{},{}\n",
            f.id,
            f.first_tick,
            f.first_uid,
            f.last_tick,
            f.windows,
            f.near,
            f.agents.len(),
            f.repeated_at.map_or(String::new(), |t| t.to_string()),
            match f.use_state(now) {
                UseState::LiveUsers => "live",
                UseState::Fading => "fading",
                UseState::NotObserved => "not_observed",
            },
            ex.join(" | ")
        );
    }
    out
}

fn write_groups(out: &mut impl Write, g: &gene_core::relations::Graph, tick: u64) {
    use gene_core::relations::{MEDIA, PARCEL, WINDOW};
    for x in &g.groups {
        let kinds: String = ["R", "C", "U"].iter().enumerate().filter(|(k, _)| x.kinds & (1 << k) != 0).map(|(_, s)| *s).collect();
        writeln!(
            out,
            "{},{},{},{},{},{},{},{},{},{},{},{},{}",
            tick / WINDOW - 1,
            tick,
            x.id,
            x.agents.len(),
            x.others.iter().filter(|n| n.0 == PARCEL).count(),
            x.others.iter().filter(|n| n.0 == MEDIA).count(),
            kinds,
            x.weight,
            x.windows,
            x.border,
            x.hub as u8,
            x.civ as u8,
            x.parent.map_or(String::new(), |p| p.to_string())
        )
        .unwrap();
    }
}

/// Repeated families with the judge's filter (SPEC §17f B).
fn judge_csv(j: &[gene_core::judge::FamilyJudgement]) -> String {
    let mut out = String::from("family,ok_tokens,tokens,carriers,fitness_x1000,founder_types,windows,founder_code_windows,founder_repertoire,pass\n");
    for f in j {
        out += &format!(
            "{},{},{},{},{},{},{},{},{},{}\n",
            f.id, f.ok, f.total, f.carriers, f.fitness_milli, f.types, f.windows, f.founder_windows, f.founder as u8, f.pass as u8
        );
    }
    out
}

/// Chains of families used by two or more agents (SPEC §17f C), passing ones first, with what
/// each step does.
fn highlevel_csv(c: &[gene_core::judge::Candidate], an: &gene_core::behavior::Analyzer) -> String {
    let mut out = String::from("chain,verdict,new,first_tick,occurrences,founder_code_occurrences,agents,phases,outputs,example_agents,steps\n");
    for k in c {
        let ch = &k.chain;
        let steps: Vec<String> = ch
            .families
            .iter()
            .map(|f| an.families[*f as usize].example.iter().take(4).map(gene_core::behavior::describe).collect::<Vec<_>>().join(" · "))
            .collect();
        out += &format!(
            "{},{},{},{},{},{},{},{},{},{},\"{}\"\n",
            ch.families.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(">"),
            k.reject.unwrap_or("pass"),
            k.new as u8,
            ch.first_window * gene_core::behavior::WINDOW,
            ch.occurrences,
            ch.founder_occurrences,
            ch.agent_count,
            ch.phases.count_ones(),
            ch.outputs,
            ch.agents.iter().take(5).map(|a| a.to_string()).collect::<Vec<_>>().join(" "),
            steps.join(" => ")
        );
    }
    out
}

/// A public record lies on tile t (dropped, or on the parcel there).
fn has_record(sim: &Sim, t: u32) -> bool {
    sim.st.dropped.contains_key(&t) || sim.st.world.parcels.get(&t).is_some_and(|p| p.media.is_some())
}

/// Structure patterns seen at two or more independent places (original §58), oldest first.
fn artifacts_csv(a: &gene_core::artifacts::Artifacts) -> String {
    let mut out = String::from("pattern,first_tick,parcels,bonds,trivial,occurrences,independent,contributors,natural,repeated_at\n");
    let mut rows: Vec<_> = a.patterns.values().filter(|p| p.independent >= 2).collect();
    rows.sort_by_key(|p| (p.first_tick, p.hash));
    for p in rows {
        out += &format!(
            "{:016x},{},{},{},{},{},{},{},{},{}\n",
            p.hash,
            p.first_tick,
            p.parcels,
            p.bonds,
            p.trivial as u8,
            p.occurrences,
            p.independent,
            p.contributors.len(),
            p.natural as u8,
            p.repeated_at.map_or(String::new(), |t| t.to_string())
        );
    }
    out
}

/// Code that came from another agent through records and is running (original §58 transfers).
fn lineage_csv(l: &gene_core::lineage::Lineage) -> String {
    use gene_core::lineage::TransferKind;
    let mut out = String::from("tick,agent,media,author,kind,cells,via\n");
    for t in &l.transfers {
        let kind = if t.kind == TransferKind::Kin { "kin" } else { "non-kin" };
        let via = if t.learned { "learned" } else { "birth" };
        out += &format!("{},{},{},{},{},{},{}\n", t.tick, t.agent, t.media, t.author, kind, t.cells, via);
    }
    out
}

fn relations_json(g: &gene_core::relations::Graph, a: &gene_core::artifacts::Artifacts, l: &gene_core::lineage::Lineage) -> String {
    use gene_core::relations::GroupEvent;
    let (mut new, mut splits, mut merges, mut civ) = (0, 0, 0, 0);
    for e in &g.events {
        match e {
            GroupEvent::New { .. } => new += 1,
            GroupEvent::Split { .. } => splits += 1,
            GroupEvent::Merge { .. } => merges += 1,
            GroupEvent::Civ { .. } => civ += 1,
            GroupEvent::Ended { .. } => {}
        }
    }
    format!(
        "{{\"resource_mass\": {}, \"record_cells\": {}, \"use_contacts\": {}, \"groups_now\": {}, \"largest_now\": {}, \"groups_formed\": {}, \"splits\": {}, \"merges\": {}, \"civ_candidates\": {}, \"artifact_patterns\": {}, \"repeated_artifacts\": {}, \"repeated_trivial\": {}, \"record_writes\": {}, \"record_copies\": {}, \"copied_cells\": {}, \"bit_damage\": {}, \"transfers_kin\": {}, \"transfers_nonkin\": {}}}",
        g.totals[0],
        g.totals[1],
        g.totals[2],
        g.groups.len(),
        g.groups.iter().map(|x| x.agents.len()).max().unwrap_or(0),
        new + splits,
        splits,
        merges,
        civ,
        a.patterns.len(),
        a.repeated.iter().filter(|h| !a.patterns[h].trivial).count(),
        a.repeated.iter().filter(|h| a.patterns[h].trivial).count(),
        l.writes,
        l.copies,
        l.copied_cells,
        l.damaged,
        l.transfers.iter().filter(|t| t.kind == gene_core::lineage::TransferKind::Kin).count(),
        l.transfers.iter().filter(|t| t.kind == gene_core::lineage::TransferKind::NonKin).count()
    )
}

fn effects_json(sim: &Sim, an: &gene_core::behavior::Analyzer) -> String {
    const OPS: [&str; 10] = ["READ", "WRITE", "DRIVE", "LINK", "UNLINK", "TRANSFER", "WORK", "EMIT", "REPRO", "COPY"];
    const OUT: [&str; 15] = [
        "Ok", "Partial", "Noop", "Boundary", "Blocked", "Energy", "Matter", "Capacity", "InvalidTarget", "Stale", "Limits",
        "Format", "Occluded", "QueryLimit", "Busy",
    ];
    let mut ops = Vec::new();
    for (k, row) in sim.obs.outcomes.iter().enumerate() {
        let cells: Vec<String> =
            row.iter().enumerate().filter(|(_, n)| **n > 0).map(|(o, n)| format!("\"{}\": {n}", OUT[o])).collect();
        if !cells.is_empty() {
            ops.push(format!("\"{}\": {{{}}}", OPS[k], cells.join(", ")));
        }
    }
    let m = &sim.obs.m1;
    let m1 = [
        ("emits", m.emits),
        ("signal_reads", m.signal_reads),
        ("public_reads", m.public_reads),
        ("copies_from_public", m.copies_from_public),
        ("copies_to_public", m.copies_to_public),
        ("draft_to_embryo", m.draft_to_embryo),
        ("public_writes", m.public_writes),
        ("drafts_created", m.drafts_created),
        ("foreign_births", m.foreign_births),
        ("commits", m.commits),
        ("adoptions", m.adoptions),
        ("parcels_created", m.parcels_created),
        ("parcel_reactions", m.parcel_reactions),
        ("links", m.links),
        ("unlinks", m.unlinks),
        ("assembly_moves", m.assembly_moves),
        ("pushes", m.pushes),
        ("load_failures", m.load_failures),
        ("taken", m.taken),
        ("abrasion_q", m.abrasion_q),
        ("shell_removed", m.shell_removed),
        ("body_removed", m.body_removed),
        ("actor_pushes", m.actor_pushes),
        ("parcel_media_created", m.parcel_media_created),
        ("parcel_media_dropped", m.parcel_media_dropped),
    ];
    let m1: Vec<String> = m1.iter().map(|(k, v)| format!("\"{k}\": {v}")).collect();
    let c = &an.counts;
    let behavior = format!(
        "\"windows\": {}, \"short\": {}, \"matched\": {}, \"novel\": {}, \"intermediate\": {}, \"families\": {}, \"repeated\": {}",
        c.windows,
        c.short,
        c.matched,
        c.novel,
        c.intermediate,
        an.families.len(),
        an.families.iter().filter(|f| f.repeated_at.is_some()).count()
    );
    format!("{{\"outcomes\": {{{}}}, \"m1\": {{{}}}, \"behavior\": {{{}}}}}", ops.join(", "), m1.join(", "), behavior)
}

fn summary_json(s: &Summary, o: &Opts) -> String {
    format!(
        "{{\n  \"ruleset\": \"{}\",\n  \"founder\": \"{}\",\n  \"seed\": {},\n  \"size\": {},\n  \"agents\": {},\n  \"ticks_requested\": {},\n  \"ticks_run\": {},\n  \"extinct_at\": {},\n  \"final_pop\": {},\n  \"peak_pop\": {},\n  \"births\": {},\n  \"deaths_energy\": {},\n  \"deaths_body\": {},\n  \"max_generation\": {},\n  \"distinct_genomes\": {},\n  \"tps\": {:.1},\n  \"audit_ok\": {},\n  \"core_hash\": \"{}\",\n  \"effects\": {},\n  \"relations\": {},\n  \"highlevel\": {}\n}}\n",
        o.ruleset,
        o.founder,
        s.seed,
        o.size,
        o.agents,
        o.ticks,
        s.ticks_run,
        s.extinct_at.map_or("null".into(), |t| t.to_string()),
        s.final_pop,
        s.peak_pop,
        s.births,
        s.deaths_energy,
        s.deaths_body,
        s.max_generation,
        s.distinct_genomes,
        s.tps,
        s.audit_ok,
        s.hash,
        if s.effects.is_empty() { "null" } else { &s.effects },
        if s.relations.is_empty() { "null" } else { &s.relations },
        if s.highlevel.is_empty() { "null" } else { &s.highlevel }
    )
}

fn run(mut sim: Sim, o: &Opts) -> Summary {
    fs::create_dir_all(&o.out).unwrap_or_else(|e| die(format!("{}: {e}", o.out.display())));
    let open = |name: &str| BufWriter::new(File::create(o.out.join(name)).unwrap());
    let mut stats = open("stats.csv");
    let mut births = open("births.csv");
    let mut deaths = open("deaths.csv");
    writeln!(
        stats,
        "tick,pop,births,deaths_energy,deaths_body,mean_energy,mean_inv_g,embryos,max_gen,mean_gen,distinct_genomes,mean_genome_len,\
upkeep_q,vm_q,attempt_q,effect_q,photosynthesis,respiration,moves,inert,loose_G,loose_C,loose_O,loose_W,dropped_pages,mean_theta,ledger_residual,\
emits,signal_reads,public_reads,copies_from_public,copies_to_public,draft_to_embryo,public_writes,drafts_created,bit_flips,foreign_births,commits,adoptions,parcels,parcels_created,parcel_reactions,links,unlinks,assembly_moves,parcels_moved,pushes,load_failures,\
taken,abrasion_q,shell_removed,body_removed,actor_pushes,parcel_media,parcel_media_created,parcel_media_dropped,dry_tiles"
    )
    .unwrap();
    writeln!(births, "tick,uid,parent,root,generation,genome,len,structural_op,structural_applied,point_flips,foreign,x,y").unwrap();
    writeln!(deaths, "tick,uid,cause,age,children,generation,inv_g,embryo_state").unwrap();
    let start = Instant::now();
    let mut s = Summary { seed: sim.st.config.seed, audit_ok: true, ..Default::default() };
    let t_begin = sim.st.tick;
    let t_end = t_begin + o.ticks;
    if o.map_every > 0 {
        write_map(&sim, &o.out.join(format!("map_{:07}.png", sim.st.tick)));
    }
    let mut behavior = gene_core::behavior::Analyzer::default();
    behavior.log = Some(Vec::new());
    sim.obs.facts = Some(Vec::new());
    let mut judge = gene_core::judge::Judge::default();
    let founders = founder_programs(&o.founder);
    for a in &sim.st.agents {
        if a.generation == 0 {
            let g = hash_cells(a.code.prefix(a.used_len).unwrap_or(&[]));
            judge.founder(a.uid, founder_kind(&founders, &g), g);
        }
    }
    let mut graph = gene_core::relations::Graph::default();
    let mut artifacts = gene_core::artifacts::Artifacts::default();
    let mut lineage = gene_core::lineage::Lineage::default();
    sim.obs.lineage = Some(Vec::new());
    let mut groups = open("groups.csv");
    writeln!(groups, "window,tick,id,agents,parcels,media,kinds,weight,windows,border,hub,civ,parent").unwrap();
    sim.obs.rel = Some(Vec::new());
    while sim.st.tick < t_end {
        sim.step();
        let t = sim.st.tick;
        for f in sim.obs.facts.as_mut().unwrap().drain(..) {
            behavior.push(&f);
        }
        if t % gene_core::behavior::WINDOW == 0 {
            behavior.flush(t);
            for r in behavior.log.as_mut().unwrap().drain(..) {
                judge.window(&r);
            }
        }
        for f in sim.obs.rel.as_mut().unwrap().drain(..) {
            graph.push(&f);
            artifacts.push(&f);
            if f.from.0 == gene_core::relations::AGENT {
                judge.output(f.from.1, f.tick);
            }
        }
        for f in sim.obs.lineage.as_mut().unwrap().drain(..) {
            lineage.push(&f);
        }
        if t % gene_core::relations::WINDOW == 0 {
            let ag = &sim.st.agents;
            if graph.advance(t, &|uid| ag.binary_search_by_key(&uid, |a| a.uid).is_ok()) {
                write_groups(&mut groups, &graph, t);
            }
            let w = &sim.st.world;
            artifacts.scan(w, &|u| has_record(&sim, u), t);
        }
        s.peak_pop = s.peak_pop.max(sim.st.agents.len());
        for b in sim.obs.births.drain(..) {
            judge.birth(b.uid, b.parent, b.tick, b.genome_hash);
            writeln!(
                births,
                "{},{},{},{},{},{},{},{},{},{},{},{},{}",
                b.tick,
                b.uid,
                b.parent,
                b.root,
                b.generation,
                &hex(&b.genome_hash)[..16],
                b.len,
                b.mutation.structural_op.map_or("".into(), |x| gene_core::mutation::OP_NAMES[x as usize].to_string()),
                b.mutation.structural_applied as u8,
                b.mutation.point_flips,
                b.foreign as u8,
                b.x,
                b.y
            )
            .unwrap();
        }
        for d in sim.obs.deaths.drain(..) {
            judge.death(d.uid, d.tick);
            let cause = match d.cause {
                DeathCause::Energy => "energy",
                DeathCause::Body => "body",
            };
            writeln!(
                deaths,
                "{},{},{},{},{},{},{},{}",
                d.tick, d.uid, cause, d.age, d.children, d.generation, d.inv_g, d.embryo_state
            )
            .unwrap();
        }
        let extinct = sim.st.agents.is_empty();
        // with --keep-going an empty world runs on: record the extinction once, then the usual
        // cadence (rows every tick would make an abiotic run crawl)
        let newly_extinct = extinct && s.extinct_at.is_none();
        let audit_now = o.audit_every > 0 && t % o.audit_every == 0;
        if audit_now || newly_extinct || t == t_end {
            let a = sim.audit();
            if !a.ok(&sim.st.atoms0) {
                s.audit_ok = false;
                eprintln!("LEDGER FAILURE at tick {t}: energy {} expected {}", a.energy, a.expected);
                break;
            }
        }
        if t % o.stats_every == 0 || newly_extinct || t == t_end {
            write_stats(&mut stats, &mut sim);
        }
        if o.map_every > 0 && (t % o.map_every == 0 || newly_extinct || t == t_end) {
            write_map(&sim, &o.out.join(format!("map_{t:07}.png")));
        }
        if o.checkpoint_every > 0 && t % o.checkpoint_every == 0 {
            fs::write(o.out.join(format!("ckpt_{t:07}.bin")), sim.save()).unwrap();
        }
        if !o.quiet && t % 16_384 == 0 {
            let el = start.elapsed().as_secs_f64();
            eprintln!(
                "[seed {}] t={t} pop={} births={} gen≤{} {:.0} TPS",
                s.seed,
                sim.st.agents.len(),
                sim.obs.total_births,
                sim.st.agents.iter().map(|a| a.generation).max().unwrap_or(0),
                (t - t_begin) as f64 / el
            );
        }
        if extinct {
            s.extinct_at.get_or_insert(t);
            if !o.keep_going {
                break;
            }
        }
    }
    let el = start.elapsed().as_secs_f64();
    s.ticks_run = sim.st.tick - t_begin;
    s.tps = s.ticks_run as f64 / el.max(1e-9);
    s.final_pop = sim.st.agents.len();
    s.births = sim.obs.total_births;
    s.deaths_energy = sim.obs.total_deaths_energy;
    s.deaths_body = sim.obs.total_deaths_body;
    s.max_generation = sim.st.agents.iter().map(|a| a.generation).max().unwrap_or(0);
    s.distinct_genomes = distinct_genomes(&sim);
    s.hash = hex(&sim.state_hash());
    behavior.finish();
    for r in behavior.log.as_mut().unwrap().drain(..) {
        judge.window(&r);
    }
    let (judged, candidates) = judge.finish(&behavior.families, sim.st.tick);
    fs::write(o.out.join("judge.csv"), judge_csv(&judged)).unwrap();
    fs::write(o.out.join("highlevel.csv"), highlevel_csv(&candidates, &behavior)).unwrap();
    s.highlevel = format!(
        "{{\"families_judged\": {}, \"families_pass\": {}, \"new_families_pass\": {}, \"chains_seen\": {}, \"chains_pass\": {}, \"new_chains_pass\": {}}}",
        judged.len(),
        judged.iter().filter(|f| f.pass).count(),
        judged.iter().filter(|f| f.pass && !f.founder).count(),
        candidates.len(),
        candidates.iter().filter(|c| c.pass()).count(),
        candidates.iter().filter(|c| c.pass() && c.new).count()
    );
    s.effects = effects_json(&sim, &behavior);
    if sim.st.tick % gene_core::relations::WINDOW != 0 {
        artifacts.scan(&sim.st.world, &|u| has_record(&sim, u), sim.st.tick); // the structures left at the end
    }
    s.relations = relations_json(&graph, &artifacts, &lineage);
    fs::write(o.out.join("lineage.csv"), lineage_csv(&lineage)).unwrap();
    fs::write(o.out.join("artifacts.csv"), artifacts_csv(&artifacts)).unwrap();
    fs::write(o.out.join("behaviors.csv"), behaviors_csv(&behavior, sim.st.tick)).unwrap();
    fs::write(o.out.join("genomes.txt"), genome_report(&sim, &o.founder, 5)).unwrap();
    fs::write(o.out.join("summary.json"), summary_json(&s, o)).unwrap();
    if o.save_final {
        fs::write(o.out.join("final.bin"), sim.save()).unwrap();
    }
    s
}

fn distinct_genomes(sim: &Sim) -> usize {
    let set: BTreeSet<[u8; 32]> = sim
        .st
        .agents
        .iter()
        .map(|a| hash_cells(a.code.prefix(a.used_len).unwrap_or(&[])))
        .collect();
    set.len()
}

fn write_stats(out: &mut impl Write, sim: &mut Sim) {
    let acc = sim.obs.take_acc();
    let ag = &sim.st.agents;
    let n = ag.len().max(1) as f64;
    let w = &sim.st.world;
    let mut loose = [0u64; NSPEC];
    let mut dry = 0u64;
    for t in 0..w.n() as u32 {
        for s in 0..NSPEC {
            loose[s] += w.loose_total(t, s);
        }
        dry += (w.loose_total(t, W) + w.loose_total(t, I) == 0) as u64;
    }
    let theta: f64 = if ag.is_empty() {
        0.0
    } else {
        ag.iter().map(|a| w.theta(w.idx(a.x as u32, a.y as u32)) as f64).sum::<f64>() / n
    };
    let dropped: u64 = sim.st.dropped.values().flatten().map(|m| m.present_pages() as u64).sum();
    let a = sim.audit();
    writeln!(
        out,
        "{},{},{},{},{},{:.0},{:.2},{},{},{:.2},{},{:.1},{},{},{},{},{},{},{},{},{},{},{},{},{},{:.1},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
        sim.st.tick,
        ag.len(),
        sim.obs.total_births,
        sim.obs.total_deaths_energy,
        sim.obs.total_deaths_body,
        ag.iter().map(|a| a.energy as f64).sum::<f64>() / n,
        ag.iter().map(|a| a.inv[G] as f64).sum::<f64>() / n,
        ag.iter().filter(|a| a.embryo.is_some()).count(),
        ag.iter().map(|a| a.generation).max().unwrap_or(0),
        ag.iter().map(|a| a.generation as f64).sum::<f64>() / n,
        distinct_genomes(sim),
        ag.iter().map(|a| a.used_len as f64).sum::<f64>() / n,
        acc.upkeep_q,
        acc.vm_q,
        acc.attempt_q,
        acc.effect_q,
        acc.photosynthesis,
        acc.respiration,
        acc.moves,
        acc.inert,
        loose[G],
        loose[C],
        loose[O],
        loose[W],
        dropped,
        theta,
        a.energy - a.expected,
        sim.obs.m1.emits,
        sim.obs.m1.signal_reads,
        sim.obs.m1.public_reads,
        sim.obs.m1.copies_from_public,
        sim.obs.m1.copies_to_public,
        sim.obs.m1.draft_to_embryo,
        sim.obs.m1.public_writes,
        sim.obs.m1.drafts_created,
        sim.obs.m1.bit_flips,
        sim.obs.m1.foreign_births,
        sim.obs.m1.commits,
        sim.obs.m1.adoptions,
        sim.st.world.parcels.len(),
        sim.obs.m1.parcels_created,
        sim.obs.m1.parcel_reactions,
        sim.obs.m1.links,
        sim.obs.m1.unlinks,
        sim.obs.m1.assembly_moves,
        sim.obs.m1.parcels_moved,
        sim.obs.m1.pushes,
        sim.obs.m1.load_failures,
        sim.obs.m1.taken,
        sim.obs.m1.abrasion_q,
        sim.obs.m1.shell_removed,
        sim.obs.m1.body_removed,
        sim.obs.m1.actor_pushes,
        sim.st.world.parcels.values().filter(|p| p.media.is_some()).count(),
        sim.obs.m1.parcel_media_created,
        sim.obs.m1.parcel_media_dropped,
        dry
    )
    .unwrap();
}

fn genome_color(h: &[u8; 32]) -> [u8; 3] {
    // bright, distinguishable colours from the hash
    let hue = (h[0] as f32 / 255.0) * 6.0;
    let (r, g, b) = match hue as u32 {
        0 => (1.0, hue.fract(), 0.0),
        1 => (1.0 - hue.fract(), 1.0, 0.0),
        2 => (0.0, 1.0, hue.fract()),
        3 => (0.0, 1.0 - hue.fract(), 1.0),
        4 => (hue.fract(), 0.0, 1.0),
        _ => (1.0, 0.0, 1.0 - hue.fract()),
    };
    let v = 0.75 + 0.25 * (h[1] as f32 / 255.0);
    [(r * v * 255.0) as u8, (g * v * 255.0) as u8, (b * v * 255.0) as u8]
}

fn write_map(sim: &Sim, path: &Path) {
    let w = &sim.st.world;
    let scale: u32 = (512 / w.w.max(w.h)).clamp(1, 8);
    let (iw, ih) = (w.w * scale, w.h * scale);
    let mut px = vec![0u8; (iw * ih * 3) as usize];
    let mut base = vec![[0u8; 3]; w.n()];
    for t in 0..w.n() as u32 {
        let e = w.elev[t as usize] as f32 / 15.0;
        let water = (w.loose_total(t, W) as f32 / 64.0).min(1.0);
        let g = 40.0 + 120.0 * e;
        let mut c = [g * 0.85, g * 0.8, g * 0.7];
        c[0] *= 1.0 - 0.6 * water;
        c[1] *= 1.0 - 0.3 * water;
        c[2] = c[2] * (1.0 - water) + 200.0 * water;
        base[t as usize] = [c[0] as u8, c[1] as u8, c[2] as u8];
    }
    for (t, _) in sim.st.dropped.iter() {
        base[*t as usize] = [90, 60, 40];
    }
    for a in &sim.st.agents {
        let t = w.idx(a.x as u32, a.y as u32) as usize;
        base[t] = genome_color(&hash_cells(a.code.prefix(a.used_len).unwrap_or(&[])));
    }
    for y in 0..ih {
        for x in 0..iw {
            let t = ((y / scale) * w.w + x / scale) as usize;
            let k = ((y * iw + x) * 3) as usize;
            px[k..k + 3].copy_from_slice(&base[t]);
        }
    }
    let file = File::create(path).unwrap();
    let mut enc = png::Encoder::new(BufWriter::new(file), iw, ih);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header().unwrap().write_image_data(&px).unwrap();
}

/// Seed list: comma-separated numbers and inclusive ranges, e.g. "1-20" or "1,3,5-7".
fn parse_seeds(spec: &str) -> Option<Vec<u64>> {
    let mut out = Vec::new();
    for part in spec.split(',') {
        let (a, b) = part.split_once('-').unwrap_or((part, part));
        let (a, b): (u64, u64) = (a.trim().parse().ok()?, b.trim().parse().ok()?);
        if a > b {
            return None;
        }
        out.extend(a..=b);
    }
    out.sort_unstable();
    out.dedup();
    Some(out)
}

fn batch(o: &Opts, seeds: Vec<u64>, threads: usize) {
    let queue = std::sync::Mutex::new(seeds.clone());
    let results = std::sync::Mutex::new(Vec::new());
    std::thread::scope(|sc| {
        for _ in 0..threads.max(1) {
            sc.spawn(|| loop {
                let Some(seed) = queue.lock().unwrap().pop() else { break };
                let mut so = o.clone();
                so.seed = seed;
                so.out = o.out.join(format!("seed_{seed:03}"));
                let sim = new_sim(&so);
                let s = run(sim, &so);
                eprintln!(
                    "[seed {seed}] done: ticks {} extinct {:?} births {} gen≤{} final pop {} ({:.0} TPS)",
                    s.ticks_run, s.extinct_at, s.births, s.max_generation, s.final_pop, s.tps
                );
                results.lock().unwrap().push(s);
            });
        }
    });
    let mut rs = results.into_inner().unwrap();
    rs.sort_by_key(|s| s.seed);
    let mut f = BufWriter::new(File::create(o.out.join("batch.csv")).unwrap());
    writeln!(
        f,
        "seed,ticks_run,extinct_at,final_pop,peak_pop,births,deaths_energy,deaths_body,max_generation,distinct_genomes,tps,audit_ok,core_hash"
    )
    .unwrap();
    for s in &rs {
        writeln!(
            f,
            "{},{},{},{},{},{},{},{},{},{},{:.1},{},{}",
            s.seed,
            s.ticks_run,
            s.extinct_at.map_or(String::new(), |t| t.to_string()),
            s.final_pop,
            s.peak_pop,
            s.births,
            s.deaths_energy,
            s.deaths_body,
            s.max_generation,
            s.distinct_genomes,
            s.tps,
            s.audit_ok,
            s.hash
        )
        .unwrap();
    }
}

/// Instruction-level diff (LCS) between two programs, module by module in order.
fn program_diff(a: &gene_core::genome::Program, b: &gene_core::genome::Program) -> String {
    let lines = |p: &gene_core::genome::Program| -> Vec<String> {
        let mut v = vec![format!(
            ".header entry={} point={} structural={} weights={:?}",
            p.entry_tag, p.point_threshold, p.structural_threshold, p.weights
        )];
        for m in &p.modules {
            v.push(format!(".module {}", m.tag));
            for pc in 0..m.len {
                v.push(format!("  {}", isa::disassemble_word(p.words[(m.start + pc) as usize])));
            }
        }
        v
    };
    let (x, y) = (lines(a), lines(b));
    let (n, m) = (x.len(), y.len());
    let mut dp = vec![vec![0u16; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            dp[i][j] = if x[i] == y[j] { dp[i + 1][j + 1] + 1 } else { dp[i + 1][j].max(dp[i][j + 1]) };
        }
    }
    let (mut i, mut j, mut out, mut pos) = (0, 0, String::new(), 0usize);
    while i < n || j < m {
        if i < n && j < m && x[i] == y[j] {
            i += 1;
            j += 1;
            pos += 1;
        } else if j < m && (i == n || dp[i][j + 1] >= dp[i + 1][j]) {
            out.push_str(&format!("  + [{pos:4}] {}\n", y[j].trim()));
            j += 1;
            pos += 1;
        } else {
            out.push_str(&format!("  - [{pos:4}] {}\n", x[i].trim()));
            i += 1;
        }
    }
    if out.is_empty() {
        if a == b {
            out.push_str("  (identical to the founder)\n");
        } else {
            out.push_str("  (same decoded instructions; differs only in operand bits the VM ignores)\n");
        }
    }
    out
}

fn genome_report(sim: &Sim, founder: &str, top: usize) -> String {
    let founders = founder_programs(founder);
    let mut census: BTreeMap<[u8; 32], (usize, u32, BTreeSet<u64>, u32, usize)> = BTreeMap::new();
    for (i, a) in sim.st.agents.iter().enumerate() {
        let h = hash_cells(a.code.prefix(a.used_len).unwrap_or(&[]));
        let e = census.entry(h).or_insert((0, a.used_len, BTreeSet::new(), 0, i));
        e.0 += 1;
        e.2.insert(a.root);
        e.3 = e.3.max(a.generation);
    }
    let mut v: Vec<_> = census.into_iter().collect();
    v.sort_by(|a, b| b.1 .0.cmp(&a.1 .0).then(a.0.cmp(&b.0)));
    let mut out = format!(
        "tick {}  population {}  distinct genomes {}  founder {}\n\n",
        sim.st.tick,
        sim.st.agents.len(),
        v.len(),
        founder
    );
    for (h, (count, len, roots, gen, idx)) in v.iter().take(top) {
        out.push_str(&format!(
            "genome {}  carriers {}  cells {}  founder lineages {}  max generation {}\n",
            &hex(h)[..16],
            count,
            len,
            roots.len(),
            gen
        ));
        let p = sim.st.agents[*idx].program();
        if let Some(fp) = nearest(&founders, p) {
            out.push_str(&program_diff(fp, p));
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::parse_seeds;

    #[test]
    fn seed_lists() {
        assert_eq!(parse_seeds("1-3"), Some(vec![1, 2, 3]));
        assert_eq!(parse_seeds("7"), Some(vec![7]));
        assert_eq!(parse_seeds("5,1-2,2"), Some(vec![1, 2, 5]));
        assert_eq!(parse_seeds("3-1"), None);
        assert_eq!(parse_seeds("x"), None);
    }
}
