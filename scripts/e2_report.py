#!/usr/bin/env python3
"""E2 main experiment summary (SPEC §18b): per seed, population by day, deepest generation, the
living by founder type at the end, the share of mutated births at the end, the commonest genome,
matter put into embryos, code transfers, and the judge's new families and chains with the verbs
founder code never used.

usage: e2_report.py [RUN_ROOT=runs/E2_main] [--days 64,128,256,384,488]
"""
import csv, json, sys
from pathlib import Path

DAY = 4096
NAMES = ["founder8m", "founder9m", "founder10m"]


def rows(path):
    with open(path) as f:
        return [r for r in csv.DictReader(f) if None not in r.values()]


def living_by_founder(sd, agents=249, kinds=3):
    """Living agents at the end by founder type of their root ((root - 1) mod kinds)."""
    alive = {u: u for u in range(1, agents + 1)}
    for b in rows(sd / "births.csv"):
        alive[int(b["uid"])] = int(b["root"])
    for d in rows(sd / "deaths.csv"):
        alive.pop(int(d["uid"]), None)
    out = [0] * kinds
    for root in alive.values():
        out[(root - 1) % kinds] += 1
    return out


def mutated_share(sd, since):
    """Share of births from tick `since` on that carry a point flip or an applied structural change."""
    n = m = 0
    for b in rows(sd / "births.csv"):
        if int(b["tick"]) >= since and b.get("point_flips"):
            n += 1
            m += int(b["point_flips"]) > 0 or b["structural_applied"] == "1"
    return m / n if n else 0.0


def main(argv):
    root = Path("runs/E2_main")
    days = [64, 128, 256, 384, 488]
    for a in argv:
        if a.startswith("--days="):
            days = [int(d) for d in a.split("=", 1)[1].split(",")]
        else:
            root = Path(a)
    seeds = sorted(p for p in root.iterdir() if p.name.startswith("seed_"))
    print(f"# {root} ({len(seeds)} seeds)\n")
    print("| seed | " + " | ".join(f"day {d}" for d in days) + " | end | births | max gen | done |")
    print("| --- |" + " --- |" * (len(days) + 4))
    for sd in seeds:
        st = {int(r["tick"]): r for r in rows(sd / "stats.csv") if r.get("tick")}
        if not st:
            continue
        cells = [f"{st[d * DAY]['pop']} / g{st[d * DAY]['max_gen']}" if d * DAY in st else "—" for d in days]
        last = st[max(st)]
        summary = json.load(open(sd / "summary.json")) if (sd / "summary.json").exists() else {}
        end = f"day {max(st) / DAY:.0f}: {last['pop']}"
        if summary.get("extinct_at"):
            end += " (extinct)"
        print(f"| {sd.name} | " + " | ".join(cells) + f" | {end} | {last['births']} | {last['max_gen']} | {'yes' if summary else 'no'} |")
    print("\n(cells: population / deepest generation)\n")
    print("| seed | living by founder at the end (" + ", ".join(NAMES) + ") | mutated births, last 64 days | commonest genome |")
    print("| --- | --- | --- | --- |")
    for sd in seeds:
        if not (sd / "summary.json").exists():
            continue
        st = rows(sd / "stats.csv")
        end = int(st[-1]["tick"])
        top = ""
        if (sd / "genomes.txt").exists():
            g = [l.split() for l in open(sd / "genomes.txt") if l.startswith("genome ")]
            if g:
                top = f"{g[0][1][:8]}: {g[0][3]} carriers, nearest {g[0][-1]}"
        print(f"| {sd.name} | {' / '.join(map(str, living_by_founder(sd)))} | {100 * mutated_share(sd, end - 64 * DAY):.1f}% | {top} |")
    print()
    for sd in seeds:
        if not (sd / "summary.json").exists():
            continue
        s = json.load(open(sd / "summary.json"))
        print(f"## {sd.name}\n")
        last = rows(sd / "stats.csv")[-1]
        print(f"- provisioned (matter into embryos): {last.get('provisioned')}, adoptions {last.get('adoptions')}, foreign births {last.get('foreign_births')}")
        births = rows(sd / "births.csv")
        fed = sum(1 for b in births if b.get("inv_g") and int(b["inv_g"]) > 0)
        print(f"- births with G from the parent: {fed} of {len(births)}")
        if (sd / "lineage.csv").exists():
            lin = rows(sd / "lineage.csv")
            nonkin = [r for r in lin if r.get("kind") == "non-kin"]
            print(f"- code transfers (lineage.csv): {len(lin)}, non-kin {len(nonkin)}")
        h = s.get("highlevel", {})
        print(f"- judge: families judged {h.get('families_judged')}, pass {h.get('families_pass')}, new {h.get('new_families_pass')}; chains seen {h.get('chains_seen')}, pass {h.get('chains_pass')}, new {h.get('new_chains_pass')}")
        judge = rows(sd / "judge.csv")
        new_fams = [r for r in judge if r["pass"] == "1" and r["founder_repertoire"] == "0"]
        beh = {r["family"]: r for r in rows(sd / "behaviors.csv")}
        for r in new_fams:
            b = beh.get(r["family"], {})
            verbs = r.get("new_verbs", "")
            print(f"  - family {r['family']}: carriers {r['carriers']}, fitness ×{int(r['fitness_x1000']) / 1000:.2f}, windows {r['windows']}, "
                  f"first day {int(b.get('first_tick', 0)) / DAY:.0f}, first agent {b.get('first_agent')}, new verbs [{verbs}] | {b.get('example', '')[:160]}")
        hl = rows(sd / "highlevel.csv")
        for r in hl:
            if r["verdict"] == "pass" and r["new"] == "1":
                print(f"  - NEW CHAIN {r['chain']}: agents {r['agents']}, occurrences {r['occurrences']}, situations {r['situations']}, "
                      f"new verbs [{r.get('new_verbs', '')}], agents {r['example_agents']} | {r['steps'][:200]}")
        print()


if __name__ == "__main__":
    main(sys.argv[1:])
