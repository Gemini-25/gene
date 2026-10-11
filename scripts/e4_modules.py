#!/usr/bin/env python3
"""Module layouts of births (E4, SPEC §18d): each birth's hook modules named by the founder module
they equal (call, listen, learn, build, empty, lost0 = the empty 'lost' hook, main) or '?' when
changed, counted per root founder type and layout, with the first and last day and the share of
births in the last 50 days. Layouts other than the root founder's own show where mating moved a
module; a habit in another hook means it now runs at another moment.

usage: e4_modules.py RUN_DIR [TOP]
"""
import csv, sys
from collections import Counter, defaultdict

NAMES = {"fbc27847": "main", "0999b262": "call", "4411eaf1": "listen", "e9978606": "empty",
         "73939fbf": "lost0", "9c4a7597": "learn", "7fd094cf": "build"}
FOUNDERS = ["F8m", "F9m", "F10m"]
OWN = {"F8m": "main call listen empty", "F9m": "main empty lost0 learn", "F10m": "main empty lost0 build"}
DAY = 4096


def main(run, top=20):
    rows = [r for r in csv.DictReader(open(f"{run}/births.csv")) if r.get("modules")]
    end = max(int(r["tick"]) for r in rows)
    count, first, last, recent = Counter(), {}, {}, Counter()
    for r in rows:
        kind = FOUNDERS[(int(r["root"]) - 1) % 3]
        tags = dict(m.split(":") for m in r["modules"].split(";"))
        layout = " ".join(NAMES.get(tags.get(str(t), ""), "?") for t in range(4))
        if layout == OWN[kind] or "?" in layout.split()[0]:
            continue
        key = (kind, layout)
        t = int(r["tick"])
        count[key] += 1
        first.setdefault(key, t)
        last[key] = t
        if t > end - 50 * DAY:
            recent[key] += 1
    print(f"{run}: {len(rows)} births; layouts other than the root founder's own (tags 0..3):")
    print("root | layout | births | first day | last day | births in the last 50 days")
    for key, n in count.most_common(top):
        if all(w == "?" or w in ("main",) for w in key[1].split()[1:]):
            pass
        print(f"{key[0]} | {key[1]} | {n} | {first[key] / DAY:.1f} | {last[key] / DAY:.1f} | {recent[key]}")


if __name__ == "__main__":
    main(sys.argv[1], int(sys.argv[2]) if len(sys.argv) > 2 else 20)
