#!/usr/bin/env python3
"""E2 mixed-founder runs: living agents per founder type by day, and the use of the M1 habits.

Founder i (uid i + 1) runs program i mod n, so an agent's type is (root - 1) mod n.

usage: e2_mix.py RUN_DIR N_FOUNDERS [NAMES...]
"""
import csv, sys
from pathlib import Path

DAY = 4096


def main(run, n, names):
    run = Path(run)
    births = list(csv.DictReader(open(run / "births.csv")))
    deaths = {int(r["uid"]): int(r["tick"]) for r in csv.DictReader(open(run / "deaths.csv")) if r.get("tick")}
    stats = list(csv.DictReader(open(run / "stats.csv")))
    end = int(stats[-1]["tick"])
    life = {}  # uid -> (type, born, died)
    founders = int(stats[0]["pop"]) if stats else 0
    # founders: uids 1..agents (all alive at tick 0); agents from summary
    import json
    agents = json.load(open(run / "summary.json"))["agents"] if (run / "summary.json").exists() else founders
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
    cols = ["emits", "signal_reads", "public_reads", "copies_from_public", "drafts_created", "commits", "adoptions", "parcels", "parcels_created"]
    last = stats[-1]
    print("end:", ", ".join(f"{c} {last[c]}" for c in cols if c in last))


if __name__ == "__main__":
    main(sys.argv[1], int(sys.argv[2]), sys.argv[3:])
