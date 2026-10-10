//! `gene serve`: local web viewer. The simulation thread owns the Sim; HTTP handlers only
//! read published snapshots or ask the simulation thread through a channel, so viewing never
//! changes the world (original I07, SPEC §15).

use crate::{die, founder_source, genome_color, hex, program_diff};
use gene_core::genome::hash_cells;
use gene_core::isa;
use gene_core::sim::Sim;
use gene_core::species::*;
use std::collections::{BTreeMap, BTreeSet};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const PAGE: &str = include_str!("../../../assets/viewer.html");

#[derive(Clone, Copy)]
struct Control {
    paused: bool,
    /// Target ticks per second, 0 = as fast as possible.
    tps: u32,
    step: u64,
}

struct Shared {
    control: Control,
    snapshot: Arc<String>,
    history: Arc<String>,
}

enum Req {
    Agent(u64, mpsc::Sender<String>),
    Genomes(mpsc::Sender<String>),
    Water(mpsc::Sender<Vec<u8>>),
    Terrain(mpsc::Sender<Vec<u8>>),
    Save(mpsc::Sender<String>),
}

pub struct ServeOpts {
    pub port: u16,
    pub save_dir: PathBuf,
    pub founder: String,
}

pub fn serve(sim: Sim, o: ServeOpts) {
    let shared = Arc::new(Mutex::new(Shared {
        control: Control { paused: true, tps: 10, step: 0 },
        snapshot: Arc::new(String::from("{}")),
        history: Arc::new(String::from("{}")),
    }));
    let (tx, rx) = mpsc::channel::<Req>();
    let listener = TcpListener::bind(("127.0.0.1", o.port)).unwrap_or_else(|e| die(format!("port {}: {e}", o.port)));
    eprintln!("gene viewer: http://127.0.0.1:{}/  (paused; press play in the page)", o.port);
    {
        let shared = shared.clone();
        std::thread::spawn(move || sim_thread(sim, shared, rx, o.save_dir, o.founder));
    }
    for stream in listener.incoming().flatten() {
        let shared = shared.clone();
        let tx = tx.clone();
        std::thread::spawn(move || {
            let _ = handle(stream, &shared, &tx);
        });
    }
}

// ---------------------------------------------------------------- simulation side

struct Hist {
    tick: Vec<u64>,
    pop: Vec<usize>,
    births: Vec<u64>,
    deaths: Vec<u64>,
    genomes: Vec<usize>,
    energy: Vec<u32>,
}

impl Hist {
    fn push(&mut self, sim: &Sim, genomes: usize) {
        let ag = &sim.st.agents;
        self.tick.push(sim.st.tick);
        self.pop.push(ag.len());
        self.births.push(sim.obs.total_births);
        self.deaths.push(sim.obs.total_deaths_energy + sim.obs.total_deaths_body);
        self.genomes.push(genomes);
        self.energy.push((ag.iter().map(|a| a.energy as u64).sum::<u64>() / ag.len().max(1) as u64) as u32);
        if self.tick.len() > 4000 {
            // keep every other point: bounded memory, full time range
            fn half<T: Copy>(v: &mut Vec<T>) {
                *v = v.iter().step_by(2).copied().collect();
            }
            half(&mut self.tick);
            half(&mut self.pop);
            half(&mut self.births);
            half(&mut self.deaths);
            half(&mut self.genomes);
            half(&mut self.energy);
        }
    }

    fn json(&self) -> String {
        fn arr<T: ToString>(v: &[T]) -> String {
            v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")
        }
        format!(
            "{{\"tick\":[{}],\"pop\":[{}],\"births\":[{}],\"deaths\":[{}],\"genomes\":[{}],\"energy\":[{}]}}",
            arr(&self.tick),
            arr(&self.pop),
            arr(&self.births),
            arr(&self.deaths),
            arr(&self.genomes),
            arr(&self.energy)
        )
    }
}

/// Genome hash per agent uid; inherited code never changes during a life.
struct HashCache(BTreeMap<u64, [u8; 32]>);

impl HashCache {
    fn refresh(&mut self, sim: &Sim) {
        let alive: BTreeSet<u64> = sim.st.agents.iter().map(|a| a.uid).collect();
        self.0.retain(|k, _| alive.contains(k));
        for a in &sim.st.agents {
            self.0.entry(a.uid).or_insert_with(|| hash_cells(a.code.prefix(a.used_len).unwrap_or(&[])));
        }
    }

    fn distinct(&self) -> usize {
        self.0.values().collect::<BTreeSet<_>>().len()
    }
}

fn sim_thread(mut sim: Sim, shared: Arc<Mutex<Shared>>, rx: mpsc::Receiver<Req>, save_dir: PathBuf, founder: String) {
    let founder_prog = isa::assemble(&founder_source(&founder)).ok();
    let mut hist = Hist { tick: vec![], pop: vec![], births: vec![], deaths: vec![], genomes: vec![], energy: vec![] };
    let mut cache = HashCache(BTreeMap::new());
    cache.refresh(&sim);
    hist.push(&sim, cache.distinct());
    let mut epoch = Instant::now();
    let mut epoch_ticks = 0u64;
    let mut last_ctl = (true, 10u32);
    let mut tps_window: Vec<(Instant, u64)> = vec![(Instant::now(), sim.st.tick)];
    let mut dirty = true;
    loop {
        while let Ok(req) = rx.try_recv() {
            answer(req, &sim, &cache, founder_prog.as_ref(), &save_dir);
        }
        let ctl = shared.lock().unwrap().control;
        if (ctl.paused, ctl.tps) != last_ctl {
            last_ctl = (ctl.paused, ctl.tps);
            epoch = Instant::now();
            epoch_ticks = 0;
        }
        let slice = Instant::now();
        let mut ran = 0u64;
        loop {
            if ctl.paused {
                let mut sh = shared.lock().unwrap();
                if sh.control.step == 0 {
                    break;
                }
                sh.control.step -= 1;
            } else if ctl.tps > 0 {
                let allowed = (epoch.elapsed().as_secs_f64() * ctl.tps as f64) as u64;
                if epoch_ticks >= allowed {
                    break;
                }
            }
            if sim.st.agents.is_empty() && !ctl.paused {
                break;
            }
            sim.step();
            ran += 1;
            epoch_ticks += 1;
            if sim.st.tick % 256 == 0 {
                cache.refresh(&sim);
                hist.push(&sim, cache.distinct());
            }
            if slice.elapsed() > Duration::from_millis(40) {
                break;
            }
        }
        if ran > 0 {
            dirty = true;
            sim.obs.births.clear();
            sim.obs.deaths.clear();
        }
        let now = Instant::now();
        tps_window.push((now, sim.st.tick));
        while tps_window.len() > 2 && now.duration_since(tps_window[0].0) > Duration::from_secs(2) {
            tps_window.remove(0);
        }
        if dirty {
            cache.refresh(&sim);
            let (t0, k0) = tps_window[0];
            let dt = now.duration_since(t0).as_secs_f64();
            let tps = if dt > 0.0 { (sim.st.tick - k0) as f64 / dt } else { 0.0 };
            let snap = snapshot(&sim, &cache, &ctl, tps);
            let mut sh = shared.lock().unwrap();
            sh.snapshot = Arc::new(snap);
            sh.history = Arc::new(hist.json());
            dirty = ran > 0;
        }
        if ran == 0 {
            std::thread::sleep(Duration::from_millis(8));
        }
    }
}

fn color_u32(h: &[u8; 32]) -> u32 {
    let c = genome_color(h);
    (c[0] as u32) << 16 | (c[1] as u32) << 8 | c[2] as u32
}

fn snapshot(sim: &Sim, cache: &HashCache, ctl: &Control, tps: f64) -> String {
    let ag = &sim.st.agents;
    let w = &sim.st.world;
    let tick = sim.st.tick;
    let p = (tick + 1024) % 4096;
    let light = (1024 - (p as i64 - 1024).abs()).max(0) as f64 / 1024.0;
    let list = |f: &dyn Fn(&gene_core::agent::Agent) -> String| ag.iter().map(f).collect::<Vec<_>>().join(",");
    format!(
        "{{\"tick\":{tick},\"day\":{},\"phase\":{p},\"light\":{light:.3},\"pop\":{},\"births\":{},\"deaths\":{},\
\"genomes\":{},\"tps\":{tps:.1},\"paused\":{},\"speed\":{},\"w\":{},\"h\":{},\"seed\":{},\"ruleset\":\"{}\",\
\"signal\":[{}],\"agents\":{{\"uid\":[{}],\"x\":[{}],\"y\":[{}],\"c\":[{}],\"e\":[{}],\"g\":[{}],\"emb\":[{}]}}}}",
        tick / 4096,
        ag.len(),
        sim.obs.total_births,
        sim.obs.total_deaths_energy + sim.obs.total_deaths_body,
        cache.distinct(),
        ctl.paused,
        ctl.tps,
        w.w,
        w.h,
        sim.st.config.seed,
        sim.st.ruleset,
        sim.st.signal.iter().take(20000).map(|(k, v)| format!("{},{},{}", k >> 2, k & 3, v)).collect::<Vec<_>>().join(","),
        list(&|a| a.uid.to_string()),
        list(&|a| a.x.to_string()),
        list(&|a| a.y.to_string()),
        list(&|a| cache.0.get(&a.uid).map_or(0, color_u32).to_string()),
        list(&|a| a.energy.to_string()),
        list(&|a| a.generation.to_string()),
        list(&|a| a.embryo_state_code().to_string()),
    )
}

fn json_str(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    o.push('"');
    for ch in s.chars() {
        match ch {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

fn answer(req: Req, sim: &Sim, cache: &HashCache, founder: Option<&gene_core::genome::Program>, save_dir: &PathBuf) {
    match req {
        Req::Agent(uid, tx) => {
            let Some(a) = sim.st.agents.iter().find(|a| a.uid == uid) else {
                let _ = tx.send("{\"error\":\"not alive\"}".into());
                return;
            };
            let w = &sim.st.world;
            let t = w.idx(a.x as u32, a.y as u32);
            let inv: Vec<String> =
                (0..NSPEC).filter(|s| a.inv[*s] > 0).map(|s| format!("\"{}\":{}", SYMBOL[s], a.inv[s])).collect();
            let diff = founder.map_or(String::new(), |f| program_diff(f, a.program()));
            let emb = a.embryo.as_ref().map_or("null".to_string(), |e| {
                format!(
                    "{{\"state\":{},\"raw_len\":{},\"capacity\":{},\"energy\":{},\"mutated\":{}}}",
                    e.state as u8,
                    e.raw.logical_len,
                    e.raw.capacity(),
                    e.energy,
                    e.mutation_applied
                )
            });
            let regs = a.vm.regs.iter().map(|r| r.to_string()).collect::<Vec<_>>().join(",");
            let mem = a.mem[..32].iter().map(|r| r.to_string()).collect::<Vec<_>>().join(",");
            let s = format!(
                "{{\"uid\":{},\"parent\":{},\"root\":{},\"generation\":{},\"age\":{},\"x\":{},\"y\":{},\"orient\":{},\
\"energy\":{},\"photo_credit\":{},\"body_g\":{},\"shell_s\":{},\"inv\":{{{}}},\"children\":{},\"genome\":\"{}\",\
\"cells\":{},\"pages\":{},\"upkeep\":{},\"theta\":{},\"tile_light_peak\":{},\"tile_elev\":{},\"tile_water\":{},\
\"pc\":[{},{}],\"fault\":{},\"regs\":[{}],\"mem\":[{}],\"embryo\":{},\"draft\":{},\"learned\":{},\"retained\":{},\"diff\":{}}}",
                a.uid,
                a.parent,
                a.root,
                a.generation,
                a.age,
                a.x,
                a.y,
                a.orient,
                a.energy,
                a.photo_credit,
                a.body_g,
                a.shell_s,
                inv.join(","),
                a.children,
                cache.0.get(&a.uid).map_or(String::new(), |h| hex(h)[..16].to_string()),
                a.used_len,
                a.code.present_pages(),
                a.upkeep(),
                w.theta(t),
                w.lpeak[t as usize],
                w.elev[t as usize],
                w.loose_total(t, W),
                a.vm.module,
                a.vm.pc,
                a.vm.fault,
                regs,
                mem,
                emb,
                a.draft.as_ref().map_or("null".to_string(), |d| format!(
                    "{{\"len\":{},\"capacity\":{},\"foreign\":{}}}",
                    d.logical_len,
                    d.capacity(),
                    d.foreign
                )),
                a.learned.as_ref().map_or("null".to_string(), |b| format!(
                    "{{\"len\":{},\"foreign\":{},\"diff\":{}}}",
                    b.used_len,
                    b.media.foreign,
                    json_str(&a.bank_progs.iter().find(|p| p.0 == b.media.uid).map_or(String::new(), |p| {
                        founder.map_or(String::new(), |f| program_diff(f, &p.1))
                    }))
                )),
                a.retained.len(),
                json_str(&diff)
            );
            let _ = tx.send(s);
        }
        Req::Genomes(tx) => {
            let mut census: BTreeMap<[u8; 32], (usize, u32, BTreeSet<u64>, usize)> = BTreeMap::new();
            for (i, a) in sim.st.agents.iter().enumerate() {
                let Some(h) = cache.0.get(&a.uid) else { continue };
                let e = census.entry(*h).or_insert((0, 0, BTreeSet::new(), i));
                e.0 += 1;
                e.1 = e.1.max(a.generation);
                e.2.insert(a.root);
            }
            let mut v: Vec<_> = census.into_iter().collect();
            v.sort_by(|a, b| b.1 .0.cmp(&a.1 .0).then(a.0.cmp(&b.0)));
            let items: Vec<String> = v
                .iter()
                .take(12)
                .map(|(h, (n, g, roots, idx))| {
                    let a = &sim.st.agents[*idx];
                    let diff = founder.map_or(String::new(), |f| program_diff(f, a.program()));
                    format!(
                        "{{\"hash\":\"{}\",\"color\":{},\"count\":{},\"cells\":{},\"max_gen\":{},\"lineages\":{},\"diff\":{}}}",
                        &hex(h)[..16],
                        color_u32(h),
                        n,
                        a.used_len,
                        g,
                        roots.len(),
                        json_str(&diff)
                    )
                })
                .collect();
            let _ = tx.send(format!("{{\"distinct\":{},\"top\":[{}]}}", v.len(), items.join(",")));
        }
        Req::Water(tx) => {
            let w = &sim.st.world;
            let mut out = Vec::with_capacity(w.n());
            for t in 0..w.n() as u32 {
                let mut v = w.loose_total(t, W).min(254) as u8;
                if sim.st.dropped.contains_key(&t) {
                    v = 255; // marker: dropped records here
                }
                out.push(v);
            }
            let _ = tx.send(out);
        }
        Req::Terrain(tx) => {
            let w = &sim.st.world;
            let mut out = Vec::with_capacity(8 + 2 * w.n());
            out.extend_from_slice(&w.w.to_le_bytes());
            out.extend_from_slice(&w.h.to_le_bytes());
            for t in 0..w.n() {
                out.push(w.elev[t] as u8);
                out.push((w.lpeak[t] >> 8) as u8);
            }
            let _ = tx.send(out);
        }
        Req::Save(tx) => {
            let _ = std::fs::create_dir_all(save_dir);
            let path = save_dir.join(format!("serve_seed{}_t{:07}.bin", sim.st.config.seed, sim.st.tick));
            let msg = match std::fs::write(&path, sim.save()) {
                Ok(()) => format!("{{\"saved\":{}}}", json_str(&path.display().to_string())),
                Err(e) => format!("{{\"error\":{}}}", json_str(&e.to_string())),
            };
            let _ = tx.send(msg);
        }
    }
}

// ---------------------------------------------------------------- HTTP side

fn query(q: &str, key: &str) -> Option<String> {
    q.split('&').find_map(|kv| {
        let (k, v) = kv.split_once('=').unwrap_or((kv, ""));
        (k == key).then(|| v.to_string())
    })
}

fn respond(s: &mut TcpStream, status: &str, ctype: &str, body: &[u8]) -> std::io::Result<()> {
    write!(
        s,
        "HTTP/1.1 {status}\r\nContent-Type: {ctype}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    )?;
    s.write_all(body)
}

fn ask<T>(tx: &mpsc::Sender<Req>, make: impl FnOnce(mpsc::Sender<T>) -> Req) -> Option<T> {
    let (rtx, rrx) = mpsc::channel();
    tx.send(make(rtx)).ok()?;
    rrx.recv_timeout(Duration::from_secs(10)).ok()
}

fn handle(mut stream: TcpStream, shared: &Arc<Mutex<Shared>>, tx: &mpsc::Sender<Req>) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    // drain headers
    let mut content_len = 0usize;
    loop {
        let mut h = String::new();
        if reader.read_line(&mut h)? == 0 || h == "\r\n" {
            break;
        }
        if let Some(v) = h.to_ascii_lowercase().strip_prefix("content-length:") {
            content_len = v.trim().parse().unwrap_or(0);
        }
    }
    if content_len > 0 {
        let mut buf = vec![0u8; content_len.min(1 << 16)];
        let _ = reader.read_exact(&mut buf);
    }
    let target = line.split_whitespace().nth(1).unwrap_or("/");
    let (path, q) = target.split_once('?').unwrap_or((target, ""));
    let json = "application/json";
    match path {
        "/" | "/index.html" => respond(&mut stream, "200 OK", "text/html; charset=utf-8", PAGE.as_bytes()),
        "/api/state" => {
            let s = shared.lock().unwrap().snapshot.clone();
            respond(&mut stream, "200 OK", json, s.as_bytes())
        }
        "/api/history" => {
            let s = shared.lock().unwrap().history.clone();
            respond(&mut stream, "200 OK", json, s.as_bytes())
        }
        "/api/control" => {
            let mut sh = shared.lock().unwrap();
            if query(q, "pause").is_some() {
                sh.control.paused = true;
            }
            if query(q, "play").is_some() {
                sh.control.paused = false;
            }
            if let Some(n) = query(q, "step").and_then(|v| v.parse::<u64>().ok()) {
                sh.control.paused = true;
                sh.control.step += n;
            }
            if let Some(t) = query(q, "tps").and_then(|v| v.parse::<u32>().ok()) {
                sh.control.tps = t;
            }
            drop(sh);
            respond(&mut stream, "200 OK", json, b"{\"ok\":true}")
        }
        "/api/agent" => {
            let uid = query(q, "uid").and_then(|v| v.parse().ok()).unwrap_or(0);
            let s = ask(tx, |r| Req::Agent(uid, r)).unwrap_or_else(|| "{\"error\":\"timeout\"}".into());
            respond(&mut stream, "200 OK", json, s.as_bytes())
        }
        "/api/genomes" => {
            let s = ask(tx, Req::Genomes).unwrap_or_else(|| "{\"error\":\"timeout\"}".into());
            respond(&mut stream, "200 OK", json, s.as_bytes())
        }
        "/api/water" => {
            let b = ask(tx, Req::Water).unwrap_or_default();
            respond(&mut stream, "200 OK", "application/octet-stream", &b)
        }
        "/api/terrain" => {
            let b = ask(tx, Req::Terrain).unwrap_or_default();
            respond(&mut stream, "200 OK", "application/octet-stream", &b)
        }
        "/api/save" => {
            let s = ask(tx, Req::Save).unwrap_or_else(|| "{\"error\":\"timeout\"}".into());
            respond(&mut stream, "200 OK", json, s.as_bytes())
        }
        _ => respond(&mut stream, "404 Not Found", "text/plain", b"not found"),
    }
}
