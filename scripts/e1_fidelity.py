#!/usr/bin/env python3
"""Find lineages that suppress mutation: parent genomes whose children are almost never mutated.

About a quarter of births normally carry a point or structural mutation. A genome whose embryos
cannot finish when a mutation is planned (e.g. Step replaced by another REPRO sub-op) only ever
produces exact copies. For each genome with at least MIN_KIDS children, compare its mutated share
with the batch-wide share; 0 of 30 children happens by chance with probability 0.75^30 ~ 2e-4.

usage: e1_fidelity.py BATCH_DIR [MIN_KIDS]
"""
import csv, sys
from collections import defaultdict
from pathlib import Path


def mutated(r):
    return r["structural_applied"] == "1" or r["point_flips"] not in ("", "0")


def main(batch, min_kids=30):
    print("| seed | births | mutated share | suppressing genomes (0 mutated of ≥%d children) | their share of the last 2 days' births |" % min_kids)
    print("|---|---|---|---|---|")
    total = [0, 0, 0]
    for sd in sorted(p for p in Path(batch).iterdir() if p.name.startswith("seed_")):
        rows = [r for r in csv.DictReader(open(sd / "births.csv")) if r.get("tick") and r.get("genome") and None not in r.values()]
        if not rows:
            continue
        genome = {r["uid"]: r["genome"] for r in rows}
        kids = defaultdict(lambda: [0, 0])
        for r in rows:
            g = genome.get(r["parent"])
            if g is None:
                continue
            kids[g][0] += 1
            kids[g][1] += mutated(r)
        share = sum(mutated(r) for r in rows) / len(rows)
        supp = {g for g, (n, m) in kids.items() if n >= min_kids and m == 0}
        end = int(rows[-1]["tick"])
        recent = [r for r in rows if int(r["tick"]) > end - 8192]
        rshare = sum(1 for r in recent if genome.get(r["parent"]) in supp) / max(1, len(recent))
        names = ", ".join(f"{g} ({kids[g][0]})" for g in sorted(supp, key=lambda g: -kids[g][0])[:3]) or "—"
        print(f"| {sd.name[5:]} | {len(rows)} | {share:.1%} | {len(supp)}: {names} | {rshare:.0%} |")
        total[0] += 1
        total[1] += bool(supp)
        total[2] += rshare
    if total[0]:
        print(f"\nseeds with a suppressing genome: {total[1]}/{total[0]}; mean share of recent births from them: {total[2]/total[0]:.0%}")


if __name__ == "__main__":
    main(sys.argv[1], int(sys.argv[2]) if len(sys.argv) > 2 else 30)
