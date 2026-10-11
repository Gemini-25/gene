#!/usr/bin/env python3
"""Share of births that carry a mutation (point flips or an applied structural change), by period.
Founder code builds every mutated embryo (REPRO Step); a lineage that aborts mutated embryos shows
up as a falling share.

usage: mutant_births.py RUN_DIR [RUN_DIR ...] [--days 32]
"""
import csv, sys
from pathlib import Path

DAY = 4096


def main(args):
    span = 32
    runs = []
    for a in args:
        if a.startswith("--days="):
            span = int(a.split("=", 1)[1])
        else:
            runs.append(Path(a))
    cols = []
    for r in runs:
        bins = {}
        for b in csv.DictReader(open(r / "births.csv")):
            if not b.get("point_flips"):
                continue
            k = int(b["tick"]) // (span * DAY)
            m = int(b["point_flips"]) > 0 or b["structural_applied"] == "1"
            n, x = bins.get(k, (0, 0))
            bins[k] = (n + 1, x + m)
        cols.append((r.name if r.name.startswith("seed") else str(r), bins))
    keys = sorted({k for _, b in cols for k in b})
    print("| days | " + " | ".join(n for n, _ in cols) + " |")
    print("| --- |" + " --- |" * len(cols))
    for k in keys:
        cells = []
        for _, b in cols:
            n, x = b.get(k, (0, 0))
            cells.append(f"{100 * x / n:.1f}% of {n}" if n else "—")
        print(f"| {k * span}–{(k + 1) * span} | " + " | ".join(cells) + " |")
    print("(share of births with a point flip or an applied structural change)")


if __name__ == "__main__":
    main(sys.argv[1:])
