#!/usr/bin/env python3
"""Rebuild BATCH_DIR/batch.csv from seed_*/summary.json (for batches split over several runs).

usage: batch_csv.py BATCH_DIR
"""
import json, sys
from pathlib import Path

COLS = "seed,ticks_run,extinct_at,final_pop,peak_pop,births,deaths_energy,deaths_body,max_generation,distinct_genomes,tps,audit_ok,core_hash"


def row(d):
    return ",".join([
        str(d["seed"]), str(d["ticks_run"]), "" if d["extinct_at"] is None else str(d["extinct_at"]),
        str(d["final_pop"]), str(d["peak_pop"]), str(d["births"]), str(d["deaths_energy"]), str(d["deaths_body"]),
        str(d["max_generation"]), str(d["distinct_genomes"]), f'{d["tps"]:.1f}', "true" if d["audit_ok"] else "false",
        d["core_hash"],
    ])


def main(batch):
    sums = [json.loads(p.read_text()) for p in sorted(Path(batch).glob("seed_*/summary.json"))]
    sums.sort(key=lambda d: d["seed"])
    Path(batch, "batch.csv").write_text(COLS + "\n" + "".join(row(d) + "\n" for d in sums))
    print(f"{len(sums)} seeds -> {Path(batch, 'batch.csv')}")


if __name__ == "__main__":
    main(sys.argv[1])
