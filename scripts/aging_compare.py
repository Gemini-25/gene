#!/usr/bin/env python3
"""Side by side: population and deepest generation by day for several runs, then totals.

usage: aging_compare.py RUN_DIR[=LABEL] ... [--days 32,96,160,224]
"""
import csv, json, sys
from pathlib import Path

DAY = 4096


def main(args):
    days = [32, 96, 160, 224]
    runs = []
    for a in args:
        if a.startswith("--days="):
            days = [int(d) for d in a.split("=", 1)[1].split(",")]
            continue
        path, _, label = a.partition("=")
        runs.append((Path(path), label or Path(path).name))
    cols = []
    for path, label in runs:
        rows = {int(r["tick"]): r for r in csv.DictReader(open(path / "stats.csv")) if r.get("tick") and r.get("max_gen")}
        summary = json.load(open(path / "summary.json")) if (path / "summary.json").exists() else {}
        cols.append((label, rows, summary))
    print("| day | " + " | ".join(label for label, _, _ in cols) + " |")
    print("| --- |" + " --- |" * len(cols))
    for d in days:
        cells = []
        for _, rows, _ in cols:
            r = rows.get(d * DAY)
            cells.append(f"{r['pop']} / {r['max_gen']}" if r else "—")
        print(f"| {d} | " + " | ".join(cells) + " |")
    cells = []
    for _, rows, s in cols:
        last = rows[max(rows)]
        end = f"day {max(rows) / DAY:.0f}: {last['pop']}, births {last['births']}"
        if s.get("extinct_at"):
            end += f", extinct day {s['extinct_at'] / DAY:.0f}"
        cells.append(end)
    print("| end | " + " | ".join(cells) + " |")
    print("(cells: population / deepest generation)")


if __name__ == "__main__":
    main(sys.argv[1:])
