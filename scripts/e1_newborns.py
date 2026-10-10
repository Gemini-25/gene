#!/usr/bin/env python3
"""Newborn fate over a batch: one-day survival, morning share and births per agent-day by period,
founder genome vs mutants, and when newborns that die young die (docs/results/E1.md).

usage: e1_newborns.py BATCH_DIR
"""
import csv, statistics as st, sys
from collections import Counter
from pathlib import Path

DAY = 4096
PERIODS = [(0, 7), (7, 14), (14, 21), (21, 28), (28, 35), (35, 42), (42, 48)]


def rows(path):
    with open(path) as f:
        return [r for r in csv.DictReader(f) if None not in r.values() and r.get("tick")]


def main(batch):
    surv = {p: [0, 0] for p in PERIODS}
    morning = {p: [0, 0] for p in PERIODS}
    per_agent = {p: [] for p in PERIODS}
    by_genome = {}
    young = {"early": [], "late": []}
    for sd in sorted(Path(batch).glob("seed_*")):
        death = {}
        for r in rows(sd / "deaths.csv"):
            death[int(r["uid"])] = int(r["tick"])
            age, t = int(r["age"]), int(r["tick"])
            if age < DAY:
                key = "early" if t < 7 * DAY else "late" if t >= 35 * DAY else None
                if key:
                    young[key].append((age, (t + 1024) % DAY >= 2048))
        pop = {int(r["tick"]): int(r["pop"]) for r in rows(sd / "stats.csv")}
        births = rows(sd / "births.csv")
        founder = Counter(r["genome"] for r in births[:300]).most_common(1)[0][0] if births else None
        end = max(pop) if pop else 0
        n = Counter()
        for r in births:
            t, uid = int(r["tick"]), int(r["uid"])
            if t > end - DAY:
                continue
            ok = death.get(uid) is None or death[uid] - t >= DAY
            for p in PERIODS:
                if p[0] * DAY <= t < p[1] * DAY:
                    surv[p][0] += ok
                    surv[p][1] += 1
                    morning[p][0] += (t + 1024) % DAY < 1024
                    morning[p][1] += 1
                    n[p] += 1
            per = "early" if t < 7 * DAY else "late" if t >= 35 * DAY else None
            if per:
                g = by_genome.setdefault((per, r["genome"] == founder), [0, 0])
                g[0] += ok
                g[1] += 1
        for p in PERIODS:
            ts = [t for t in pop if p[0] * DAY <= t < p[1] * DAY]
            if ts and n[p]:
                per_agent[p].append(n[p] / st.mean(pop[t] for t in ts) / (p[1] - p[0]))
    print("| 출생 날 | 하루 생존 | 오전 출생 비율 | 개체당 하루 출생 |")
    print("| --- | --- | --- | --- |")
    for p in PERIODS:
        if surv[p][1]:
            print(f"| {p[0]}..{p[1]} | {100 * surv[p][0] / surv[p][1]:.1f}% | {100 * morning[p][0] / morning[p][1]:.0f}% | {st.median(per_agent[p]):.2f} |")
    print()
    for (per, is_f), (ok, all_) in sorted(by_genome.items()):
        print(f"{per:5} {'founder genome' if is_f else 'mutant':14} one-day survival {100 * ok / all_:.1f}% (n={all_})")
    for k, v in young.items():
        if v:
            print(f"newborns dying within a day, {k}: median age {st.median(a for a, _ in v):.0f}, at night {100 * sum(n for _, n in v) / len(v):.0f}% (n={len(v)})")


if __name__ == "__main__":
    main(sys.argv[1])
