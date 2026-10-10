#!/usr/bin/env python3
"""Competition assay: is an evolved genome fitter than its founder in a fresh world?

For each seed, two runs share the world and the founder positions:
  test     founder + SAVE@HASH   (founder i runs program i mod 2: the odd uids carry the genome)
  neutral  founder + founder     (the same split with identical programs)
The genome's fitness signal is its share of the living at the end minus the share of the same
slots in the neutral run (paired by seed, so where the founders happened to start cancels out).

usage: compete.py SAVE.bin HASH[,HASH...] [--founder founder7] [--ruleset m2c] [--seeds 1-4]
                  [--size 128] [--agents 64] [--ticks 100000] [--jobs 4] [--out DIR] [--bin PATH]
"""
import argparse, csv, os, subprocess, sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path


def seeds(spec):
    out = []
    for part in spec.split(","):
        a, _, b = part.partition("-")
        out += list(range(int(a), int(b or a) + 1))
    return out


def living_by_slot(run, agents):
    """Living agents at the end by founder slot ((root - 1) mod 2), and births by slot."""
    alive = {u: (u - 1) % 2 for u in range(1, agents + 1)}
    births = [0, 0]
    for b in csv.DictReader(open(run / "births.csv")):
        if b.get("root") and b.get("uid"):
            slot = (int(b["root"]) - 1) % 2
            alive[int(b["uid"])] = slot
            births[slot] += 1
    for d in csv.DictReader(open(run / "deaths.csv")):
        if d.get("uid"):
            alive.pop(int(d["uid"]), None)
    living = [0, 0]
    for slot in alive.values():
        living[slot] += 1
    return living, births


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("save")
    ap.add_argument("hashes")
    ap.add_argument("--founder", default="founder7")
    ap.add_argument("--ruleset", default="m2c")
    ap.add_argument("--seeds", default="1-4")
    ap.add_argument("--size", type=int, default=128)
    ap.add_argument("--agents", type=int, default=64)
    ap.add_argument("--ticks", type=int, default=100000)
    ap.add_argument("--jobs", type=int, default=4)
    ap.add_argument("--out", default="runs/compete")
    ap.add_argument("--bin", default="./target/release/gene")
    a = ap.parse_args()
    out = Path(a.out)
    ss = seeds(a.seeds)
    hashes = a.hashes.split(",")
    jobs = []  # (label, seed, founder spec, dir)
    for s in ss:
        jobs.append(("neutral", s, f"{a.founder}+{a.founder}", out / f"neutral_{a.founder}_{a.ruleset}_s{s}"))
        for h in hashes:
            jobs.append((h, s, f"{a.founder}+{a.save}@{h}", out / f"{h}_{a.ruleset}_s{s}"))

    def run(job):
        _, s, spec, d = job
        if (d / "summary.json").exists():
            return
        cmd = [a.bin, "run", "--seed", str(s), "--size", str(a.size), "--agents", str(a.agents), "--ticks", str(a.ticks),
               "--founder", spec, "--ruleset", a.ruleset, "--map-every", "0", "--stats-every", "4096", "--keep-going",
               "--quiet", "--out", str(d)]
        subprocess.run(["nice", "-n", "10"] + cmd, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

    with ThreadPoolExecutor(a.jobs) as ex:
        list(ex.map(run, jobs))

    neutral = {s: living_by_slot(out / f"neutral_{a.founder}_{a.ruleset}_s{s}", a.agents) for s in ss}
    print(f"{a.founder} vs evolved genomes from {a.save}, {a.ruleset}, {a.size}², {a.agents} agents, {a.ticks} ticks")
    print("genome            | " + " | ".join(f"seed {s}" for s in ss) + " | mean   | wins")
    for h in hashes:
        diffs = []
        for s in ss:
            (lt, _), (ln, _) = living_by_slot(out / f"{h}_{a.ruleset}_s{s}", a.agents), neutral[s]
            share = lt[1] / max(1, sum(lt))
            base = ln[1] / max(1, sum(ln))
            diffs.append(100 * (share - base))
        mean = sum(diffs) / len(diffs)
        wins = sum(d > 0 for d in diffs)
        print(f"{h:17} | " + " | ".join(f"{d:+6.1f}" for d in diffs) + f" | {mean:+6.1f} | {wins}/{len(diffs)}")
    print("(percentage points of the living held by the genome's slots, minus the same slots with the founder)")


if __name__ == "__main__":
    main()
