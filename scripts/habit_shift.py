#!/usr/bin/env python3
"""Behaviour shift of evolved genomes against their founder (a detector beside the judge, §17f):
each genome shares a fresh world with its founder (founder i runs program i mod 2, as in
scripts/compete.py), and the verbs whose rate per agent-day changed most are listed - verbs the
lineage dropped, verbs it took up, and verbs whose rate moved by at least RATIO.

A change of use of known actions (E2 seed 1: aborting mutated embryos with the founder's
cancel routine) is invisible to novelty by action order but shows here.

usage: habit_shift.py SAVE.bin HASH[,HASH...] --founder NAME [--ruleset m2d] [--seeds 1,2]
                      [--ticks 32768] [--min-rate 0.2] [--ratio 3] [--bin ./target/release/gene]
"""
import argparse, csv, io, subprocess, tempfile
from collections import defaultdict
from pathlib import Path


def profile(a, spec, seed, tmp):
    out = Path(tmp) / f"w{seed}"
    subprocess.run([a.bin, "run", "--seed", str(seed), "--size", "128", "--agents", "64", "--ticks", "8192", "--founder", spec,
                    "--ruleset", a.ruleset, "--map-every", "0", "--stats-every", "4096", "--keep-going", "--quiet",
                    "--save-final", "--out", str(out)], check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    hab = Path(a.bin).parent / "examples" / "habits"
    txt = subprocess.run([str(hab), str(out / "final.bin"), str(a.ticks), "slot2", "0", "csv"], check=True, capture_output=True, text=True).stdout
    rates = defaultdict(lambda: [0.0, 0.0])  # verb -> [founder slot, genome slot] successful per agent-day
    for r in csv.DictReader(io.StringIO(txt)):
        slot = 0 if r["group"] == "slot0" else 1 if r["group"] == "slot1" else None
        if slot is not None:
            rates[r["verb"]][slot] += float(r["ok_per_day"])
    return rates


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("save")
    ap.add_argument("hashes")
    ap.add_argument("--founder", required=True)
    ap.add_argument("--ruleset", default="m2d")
    ap.add_argument("--seeds", default="1,2")
    ap.add_argument("--ticks", type=int, default=32768)
    ap.add_argument("--min-rate", type=float, default=0.2)
    ap.add_argument("--ratio", type=float, default=3.0)
    ap.add_argument("--bin", default="./target/release/gene")
    a = ap.parse_args()
    seeds = [int(s) for s in a.seeds.split(",")]
    for h in a.hashes.split(","):
        total = defaultdict(lambda: [0.0, 0.0])
        with tempfile.TemporaryDirectory() as tmp:
            for s in seeds:
                for v, (f, g) in profile(a, f"{a.founder}+{a.save}@{h}", s, tmp).items():
                    total[v][0] += f / len(seeds)
                    total[v][1] += g / len(seeds)
        print(f"## {h} against {a.founder} ({a.ruleset}, seeds {a.seeds}; successful effects per agent-day)")
        print("| verb | founder | genome | change |")
        print("| --- | --- | --- | --- |")
        rows = []
        for v, (f, g) in total.items():
            if max(f, g) < a.min_rate:
                continue
            if f < a.min_rate / 4:
                kind = "taken up"
            elif g < a.min_rate / 4:
                kind = "dropped"
            elif g / f >= a.ratio or f / g >= a.ratio:
                kind = f"×{g / f:.2f}"
            else:
                continue
            rows.append((abs(g - f), v, f, g, kind))
        for _, v, f, g, kind in sorted(rows, reverse=True):
            print(f"| {v} | {f:.2f} | {g:.2f} | {kind} |")
        print()


if __name__ == "__main__":
    main()
