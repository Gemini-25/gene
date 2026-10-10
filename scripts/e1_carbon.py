#!/usr/bin/env python3
"""Carbon budget of a batch: air (loose C), loose G, dropped record pages, and photosynthesis per
agent, at the first and last dawn of each seed (medians across seeds).

usage: e1_carbon.py BATCH_DIR [BATCH_DIR ...]
"""
import csv, statistics as st, sys
from pathlib import Path


def dawns(stats):
    rows = [r for r in csv.DictReader(open(stats)) if None not in r.values() and r.get("tick")]
    return rows, [r for r in rows if int(r["tick"]) % 4096 == 3072]


def per_agent_photo(rows, day):
    sel = [r for r in rows if int(r["tick"]) // 4096 == day]
    pop = sum(int(r["pop"]) for r in sel) / max(1, len(sel))
    return sum(int(r["photosynthesis"]) for r in sel) / pop if pop else 0.0


def main(batches):
    print("| batch | seeds | day | air C | loose G | dropped pages | photosynthesis per agent per day |")
    print("|---|---|---|---|---|---|---|")
    for b in batches:
        first, last, photo0, photo1 = [], [], [], []
        for sd in sorted(p for p in Path(b).iterdir() if p.name.startswith("seed_") and (p / "summary.json").exists()):
            rows, d = dawns(sd / "stats.csv")
            if len(d) < 2:
                continue
            first.append(d[0])
            last.append(d[-1])
            photo0.append(per_agent_photo(rows, 1))
            photo1.append(per_agent_photo(rows, int(d[-1]["tick"]) // 4096 - 1))
        if not first:
            continue
        med = lambda rs, k: st.median(int(r[k]) for r in rs)
        for label, rs, ph in (("first dawn", first, photo0), ("last dawn", last, photo1)):
            day = st.median(int(r["tick"]) // 4096 for r in rs)
            print(f"| {Path(b).name} | {len(rs)} | {day:.0f} ({label}) | {med(rs, 'loose_C'):,.0f} | {med(rs, 'loose_G'):,.0f} | "
                  f"{med(rs, 'dropped_pages'):,.0f} | {st.median(ph):.0f} |")


if __name__ == "__main__":
    main(sys.argv[1:])
