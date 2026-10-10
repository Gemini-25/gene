#!/usr/bin/env python3
"""E2 mixed-founder runs: living agents per founder type by day, the use of the M1 habits, and
births whose genome is exactly another type's founder genome (a learned founder module put into
a child by Recombine: the founders are one module each and share their header).

Founder i (uid i + 1) runs program i mod n, so an agent's type is (root - 1) mod n.

usage: e2_mix.py RUN_DIR N_FOUNDERS [NAMES...] [AGENTS]
NAMES are built-in founder names (founder8 ...) so their genome hashes can be looked up.
"""
import csv, sys
from pathlib import Path

DAY = 4096


def main(run, n, names):
    run = Path(run)
    complete = lambda r: None not in r.values() and all(v != "" for k, v in r.items() if k in ("tick", "uid", "root"))
    births = [r for r in csv.DictReader(open(run / "births.csv")) if complete(r)]
    deaths = {int(r["uid"]): int(r["tick"]) for r in csv.DictReader(open(run / "deaths.csv")) if r.get("tick") and r.get("uid")}
    stats = [r for r in csv.DictReader(open(run / "stats.csv")) if None not in r.values()]
    end = int(stats[-1]["tick"])
    life = {}  # uid -> (type, born, died)
    founders = int(stats[0]["pop"]) if stats else 0
    # founders: uids 1..agents (all alive at tick 0); agents from summary
    import json
    agents = json.load(open(run / "summary.json"))["agents"] if (run / "summary.json").exists() else int(sys.argv[4 + n]) if len(sys.argv) > 4 + n else founders
    for u in range(1, agents + 1):
        life[u] = ((u - 1) % n, 0, deaths.get(u))
    for b in births:
        u, root = int(b["uid"]), int(b["root"])
        life[u] = ((root - 1) % n, int(b["tick"]), deaths.get(u))
    days = list(range(0, end + 1, DAY))
    print("day | " + " | ".join(names[k] if k < len(names) else f"type {k}" for k in range(n)))
    for d in days:
        t = d + DAY // 2  # midday
        if t > end:
            t = end
        alive = [0] * n
        for k, b, x in life.values():
            if b <= t and (x is None or x > t):
                alive[k] += 1
        print(f"{d // DAY:3d} | " + " | ".join(f"{a:5d}" for a in alive))
    hashes, modules = {}, {}
    try:
        import subprocess
        out = subprocess.run(["./target/release/gene", "founders"], capture_output=True, text=True).stdout
        for line in out.splitlines():
            name, rest = line.split(":", 1)
            fields = [f.strip() for f in rest.split(",")]
            hashes[name.strip()] = fields[2]
            if len(fields) > 3 and fields[3].startswith("modules "):
                modules[name.strip()] = dict(m.split(":") for m in fields[3][8:].split(";"))
    except Exception:
        pass
    kind_of_hash = {hashes[nm]: k for k, nm in enumerate(names) if nm in hashes}
    cross = {}
    for b in births:
        k = kind_of_hash.get(b["genome"])
        root_k = (int(b["root"]) - 1) % n
        if k is not None and k != root_k:
            key = (names[root_k], names[k])
            cross.setdefault(key, []).append(int(b["tick"]))
    for (a, b), ts in sorted(cross.items()):
        print(f"{a} parent line, {b} founder genome: {len(ts)} births, first at day {min(ts) / DAY:.1f}")
    # habit combinations (modular founders): which founder each non-empty hook module came from
    if modules and births and "modules" in births[0]:
        owners_of = {}  # module content hash -> founders having it at any tag
        for nm, mods in modules.items():
            for t, h in mods.items():
                owners_of.setdefault(h, set()).add(nm)
        # a habit is content only one founder has (shared content: the main module and empty hooks)
        origin = {h: next(iter(o)) for h, o in owners_of.items() if len(o) == 1}
        combos = {}
        for b in births:
            if not b.get("modules"):
                continue
            hab = set()
            for m in b["modules"].split(";"):
                t, h = m.split(":")
                if h in origin:
                    hab.add((t, origin[h]))
            owners = {o for _, o in hab}
            root_kind = names[(int(b["root"]) - 1) % n] if names else None
            own = {(t, origin[h]) for t, h in modules.get(root_kind, {}).items() if h in origin}
            if hab != own and owners - {root_kind}:
                key = tuple(sorted(hab))
                combos.setdefault(key, []).append(int(b["tick"]))
        for key, ts in sorted(combos.items(), key=lambda kv: -len(kv[1])):
            print(f"habit mix {key}: {len(ts)} births, first at day {min(ts) / DAY:.1f}")
    cols = ["emits", "signal_reads", "public_reads", "copies_from_public", "drafts_created", "commits", "adoptions", "parcels", "parcels_created", "provisioned"]
    last = stats[-1]
    print("end:", ", ".join(f"{c} {last[c]}" for c in cols if c in last))


if __name__ == "__main__":
    main(sys.argv[1], int(sys.argv[2]), sys.argv[3:])
