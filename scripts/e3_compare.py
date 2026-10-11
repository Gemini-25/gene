#!/usr/bin/env python3
"""E3 pilot against E2 main at the same days (decision 8): population, living by founder type,
births, mutated births, food calls, records copied, parcels built, moss made, and the judge's new
families and chains so far. Founder types and the number of founders come from each run's
summary.json (E5 has four types and 248 founders); G taken by contact is shown when present.

usage: e3_compare.py [E3_ROOT=runs/E3_pilot] [E2_ROOT=runs/E2_main] [--days 25,50,75,100]
"""
import csv, json, sys
from pathlib import Path

DAY = 4096
NAMES = ["F8m", "F9m", "F10m"]


def rows(path):
    with open(path) as f:
        return [r for r in csv.DictReader(f) if None not in r.values()]


def living_by_founder(sd, until, agents=249, kinds=3):
    alive = {u: u for u in range(1, agents + 1)}
    for b in rows(sd / "births.csv"):
        if int(b["tick"]) <= until:
            alive[int(b["uid"])] = int(b["root"])
    for d in rows(sd / "deaths.csv"):
        if int(d["tick"]) <= until:
            alive.pop(int(d["uid"]), None)
    out = [0] * kinds
    for root in alive.values():
        out[(root - 1) % kinds] += 1
    return out


def setup(sd):
    """(founders, kinds) of a run from its summary.json; the E2 setup when there is none."""
    try:
        j = json.load(open(sd / "summary.json"))
        return j["agents"], len(j["founder"].split("+"))
    except (OSError, KeyError, ValueError):
        return 249, 3


def summary(sd, day):
    st = {int(r["tick"]): r for r in rows(sd / "stats.csv")}
    t = day * DAY
    if t not in st:
        return None
    upto = [r for k, r in st.items() if k <= t]
    total = lambda c: sum(int(r.get(c) or 0) for r in upto)
    births = [b for b in rows(sd / "births.csv") if int(b["tick"]) <= t and int(b["tick"]) > t - 16 * DAY]
    mut = sum(1 for b in births if int(b["point_flips"]) > 0 or b["structural_applied"] == "1")
    return {
        "pop": st[t]["pop"],
        "living": living_by_founder(sd, t, *setup(sd)),
        "gen": st[t]["max_gen"],
        "births": st[t]["births"],
        "mutated": f"{100 * mut / max(1, len(births)):.0f}%",
        "emits": st[t]["emits"],  # M1 counts are running totals
        "copies": st[t]["copies_from_public"],
        "parcels": st[t]["parcels"],
        "moss": total("moss") if "moss" in st[t] else 0,  # per stats interval
        "taken": st[t].get("taken", "0"),  # running total
    }


def main(argv):
    e3, e2, days = Path("runs/E3_pilot"), Path("runs/E2_main"), [25, 50, 75, 100]
    pos = [a for a in argv if not a.startswith("--")]
    for a in argv:
        if a.startswith("--days="):
            days = [int(d) for d in a.split("=", 1)[1].split(",")]
    if pos:
        e3 = Path(pos[0])
    if len(pos) > 1:
        e2 = Path(pos[1])
    print("| run | day | pop | living by founder type | max gen | births | mutated births (16 d) | food calls | records copied | parcels | moss G made | G taken by contact |")
    print("| --- |" + " --- |" * 11)
    for sd in sorted(p for p in e3.iterdir() if p.name.startswith("seed_")):
        for root in (e2, e3):
            run = root / sd.name
            if not (run / "stats.csv").exists():
                continue
            for d in days:
                s = summary(run, d)
                if s:
                    print(f"| {root.name}/{sd.name} | {d} | {s['pop']} | {' / '.join(map(str, s['living']))} | {s['gen']} | {s['births']} | {s['mutated']} | {s['emits']} | {s['copies']} | {s['parcels']} | {s['moss']} | {s['taken']} |")


if __name__ == "__main__":
    main(sys.argv[1:])
