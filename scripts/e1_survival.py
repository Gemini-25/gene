#!/usr/bin/env python3
"""Join births/deaths with tile properties: survival of newborns by birth time of day and light."""
import csv, subprocess, sys, statistics as st
from pathlib import Path

def main(seed_dir, size=512):
    seed_dir = Path(seed_dir)
    seed = int(seed_dir.name.split("_")[1])
    out = subprocess.run(["./target/release/examples/tiles", str(seed), str(size)], capture_output=True, text=True, check=True).stdout
    tiles = {}
    for r in csv.DictReader(out.splitlines()):
        tiles[(int(r["x"]), int(r["y"]))] = (int(r["lpeak"]), int(r["elev"]), int(r["water0"]))
    deaths = {int(d["uid"]): int(d["age"]) for d in csv.DictReader(open(seed_dir / "deaths.csv"))}
    stats = list(csv.DictReader(open(seed_dir / "stats.csv")))
    end = int(stats[-1]["tick"])
    rows = []
    for b in csv.DictReader(open(seed_dir / "births.csv")):
        t = int(b["tick"])
        if end - t < 4096:
            continue  # censored: not yet a full day old
        lp, el, w0 = tiles[(int(b["x"]), int(b["y"]))]
        age = deaths.get(int(b["uid"]))
        survived_day = age is None or age >= 4096
        phase = ((t + 1024) % 4096)  # 0..2048 daylight, 1024 = noon
        rows.append((phase, lp, el, w0, survived_day))
    def rate(sel):
        sel = list(sel)
        return (sum(1 for r in sel if r[4]) / len(sel), len(sel)) if sel else (float("nan"), 0)
    print(f"newborns with a full day observed: {len(rows)}; overall 1-day survival {rate(rows)[0]:.1%}")
    print("\nby birth phase (p = (tick+1024) mod 4096; daylight 0..2048, noon 1024):")
    for lo in range(0, 4096, 512):
        r, n = rate(x for x in rows if lo <= x[0] < lo + 512)
        print(f"  p {lo:4}-{lo+511:4}: survival {r:6.1%}  n={n}")
    print("\nby tile Lpeak:")
    for lo in range(32768, 65536, 8192):
        r, n = rate(x for x in rows if lo <= x[1] < lo + 8192)
        print(f"  Lpeak {lo}-{lo+8191}: survival {r:6.1%}  n={n}")

if __name__ == "__main__":
    main(sys.argv[1], int(sys.argv[2]) if len(sys.argv) > 2 else 512)
