#!/usr/bin/env python3
"""Aggregate an E1 batch directory into markdown tables (stdout)."""
import csv, json, statistics as st, sys
from pathlib import Path

def load(path):
    with open(path) as f:
        return list(csv.DictReader(f))

def med(xs):
    return st.median(xs) if xs else float("nan")

def main(root):
    root = Path(root)
    seeds = sorted(p for p in root.iterdir() if p.name.startswith("seed_"))
    rows = []
    for sd in seeds:
        s = json.loads((sd / "summary.json").read_text())
        stats = load(sd / "stats.csv")
        deaths = load(sd / "deaths.csv")
        births = load(sd / "births.csv")
        # first-day energy budget per agent-tick (ticks 1..1024 = afternoon of day 0)
        day = [r for r in stats if int(r["tick"]) <= 1024]
        at = sum(int(r["pop"]) for r in day) * 256 or 1
        budget = {k: sum(int(r[k]) for r in day) / at for k in ("upkeep_q", "vm_q", "attempt_q", "effect_q")}
        photo = sum(int(r["photosynthesis"]) for r in day) / at * 1666
        ages = [int(d["age"]) for d in deaths]
        childless = sum(1 for d in deaths if d["children"] == "0") / max(1, len(deaths))
        mutated = sum(1 for b in births if int(b["point_flips"]) > 0 or b["structural_applied"] == "1")
        last = stats[-1]
        rows.append(dict(seed=s["seed"], ticks=s["ticks_run"], extinct=s["extinct_at"], final=s["final_pop"],
                         peak=s["peak_pop"], births=s["births"], gen=max([int(b["generation"]) for b in births] or [0]),
                         genomes=s["distinct_genomes"], tps=s["tps"], audit=s["audit_ok"], budget=budget, photo=photo,
                         age=med(ages), childless=childless, mutated=mutated, mean_len=float(last["mean_genome_len"] or 0)))
    print(f"| seed | ticks | extinct at | final pop | peak | births | max gen | genomes | median death age | childless deaths | TPS |")
    print("|---|---|---|---|---|---|---|---|---|---|---|")
    for r in rows:
        print(f"| {r['seed']} | {r['ticks']} | {r['extinct'] if r['extinct'] is not None else '—'} | {r['final']} | {r['peak']} | "
              f"{r['births']} | {r['gen']} | {r['genomes']} | {r['age']:.0f} | {r['childless']:.0%} | {r['tps']:.0f} |")
    print()
    keys = ("upkeep_q", "vm_q", "attempt_q", "effect_q")
    print("First afternoon (ticks 0..1024), mean q per agent-tick across seeds:")
    print()
    print("| upkeep | VM | effect attempts | effect amounts | total spend | photosynthesis income |")
    print("|---|---|---|---|---|---|")
    m = {k: st.mean(r["budget"][k] for r in rows) for k in keys}
    print(f"| {m['upkeep_q']:.1f} | {m['vm_q']:.1f} | {m['attempt_q']:.1f} | {m['effect_q']:.1f} | "
          f"{sum(m.values()):.1f} | {st.mean(r['photo'] for r in rows):.1f} |")
    print()
    ext = [r["extinct"] for r in rows if r["extinct"] is not None]
    print(f"extinct: {len(ext)}/{len(rows)}; survived: {len(rows) - len(ext)}/{len(rows)}")
    if ext:
        print(f"extinction tick median {med(ext):.0f}, range {min(ext)}..{max(ext)}")
    alive = [r for r in rows if r["extinct"] is None]
    if alive:
        print(f"survivors: final pop median {med([r['final'] for r in alive]):.0f}, "
              f"max generation median {med([r['gen'] for r in alive]):.0f}, "
              f"births median {med([r['births'] for r in alive]):.0f}")
    print(f"all ledger audits ok: {all(r['audit'] for r in rows)}")

if __name__ == "__main__":
    main(sys.argv[1])
