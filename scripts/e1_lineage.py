#!/usr/bin/env python3
"""Evolution signals from births.csv: founder lineages, share of mutant genomes, spreading mutants.

usage: e1_lineage.py BATCH_DIR FOUNDER_HASH16
"""
import csv, sys, statistics as st
from collections import Counter
from pathlib import Path

def main(batch, founder):
    rows = []
    for sd in sorted(p for p in Path(batch).iterdir() if p.name.startswith("seed_")):
        ok = lambda r: None not in r.values() and all(v != "" for k, v in r.items() if k in ("tick", "generation", "genome"))
        births = [b for b in csv.DictReader(open(sd / "births.csv")) if ok(b)]
        stats = [r for r in csv.DictReader(open(sd / "stats.csv")) if None not in r.values()]
        if not births or not stats:
            continue
        end = int(stats[-1]["tick"])
        gens = max(int(b["generation"]) for b in births)
        recent = [b for b in births if int(b["tick"]) > end - 2 * 4096]
        if not recent:
            rows.append((sd.name, 0, 0, 0.0, "", 0, gens))
            continue
        c = Counter(b["genome"] for b in recent)
        mutant_share = sum(n for g, n in c.items() if g != founder) / len(recent)
        top, topn = c.most_common(1)[0]
        roots = len({b["root"] for b in recent})
        rows.append((sd.name, len(recent), roots, mutant_share, top, topn, gens))
    print("| seed | births in last 2 days | founder lineages among them | mutant share | most common genome (share) | max generation |")
    print("|---|---|---|---|---|---|")
    for name, n, roots, ms, top, topn, gens in rows:
        if n == 0:
            print(f"| {name[5:]} | 0 | — | — | — | {gens} |")
            continue
        tag = "founder" if top == founder else top
        print(f"| {name[5:]} | {n} | {roots} | {ms:.0%} | {tag} ({topn / n:.0%}) | {gens} |")
    alive = [r for r in rows if r[1]]
    if alive:
        print()
        print(f"median mutant share {st.median(r[3] for r in alive):.0%}; "
              f"seeds where a mutant genome is the most common: {sum(1 for r in alive if r[4] != founder)}/{len(alive)}; "
              f"median founder lineages left {st.median(r[2] for r in alive):.0f}")

if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
