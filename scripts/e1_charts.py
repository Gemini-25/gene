#!/usr/bin/env python3
"""Static SVG charts for docs/results/E1.md (no third-party packages).

usage: e1_charts.py RUNS_DIR OUT_DIR
RUNS_DIR holds the batch directories E1a_founder4/, E1b_founder5/ and, when present, E1c_founder6/, E1d_founder7/.
Seeds still running are drawn up to where they are.
"""
import csv, math, statistics as st, sys
from pathlib import Path

LIGHT = dict(surface="#fcfcfb", ink="#0b0b0b", ink2="#52514e", muted="#898781", grid="#e1e0d9",
             axis="#c3c2b7", s1="#2a78d6", s2="#eb6834", s3="#1baf7a", s4="#eda100", faint="#c3c2b7")
DARK = dict(surface="#1a1a19", ink="#f0efec", ink2="#c3c2b7", muted="#898781", grid="#2c2c2a",
            axis="#383835", s1="#3987e5", s2="#d95926", s3="#199e70", s4="#c98500", faint="#4a4a46")
FONT = 'system-ui, -apple-system, "Segoe UI", sans-serif'


def style():
    def block(p):
        return "".join(f"--{k}:{v};" for k, v in p.items())
    return (f"<style>svg{{{block(LIGHT)}}}"
            f"@media (prefers-color-scheme: dark){{svg{{{block(DARK)}}}}}"
            f"text{{font-family:{FONT};fill:var(--ink2);font-size:12px;paint-order:stroke;stroke:var(--surface);stroke-width:3px;stroke-linejoin:round}}"
            f".t{{fill:var(--ink);font-size:15px;font-weight:600}}.m{{fill:var(--muted);font-size:11px}}"
            f".num{{font-variant-numeric:tabular-nums}}</style>")


def svg(w, h, body, title):
    return (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" width="{w}" height="{h}" '
            f'role="img" aria-label="{title}">{style()}'
            f'<rect width="{w}" height="{h}" fill="var(--surface)"/>{body}</svg>\n')


def load(p):
    """Rows of a CSV, dropping a truncated last line (files of runs still in progress)."""
    with open(p) as f:
        rows = list(csv.DictReader(f))
    return [r for r in rows if None not in r.values() and all(v != "" for k, v in r.items() if k in ("tick", "uid", "age", "pop"))]


def seeds(batch):
    return sorted(p for p in Path(batch).iterdir() if p.name.startswith("seed_") and (p / "stats.csv").exists())


# experiment directory, label, colour slot (colour follows the founder in every chart)
EXPERIMENTS = [("E1a_founder4", "Founder-4", "s2"), ("E1b_founder5", "Founder-5", "s1"), ("E1c_founder6", "Founder-6", "s3"),
               ("E1d_founder7", "Founder-7", "s4")]


def present(runs):
    return [e for e in EXPERIMENTS if (Path(runs) / e[0]).is_dir() and seeds(Path(runs) / e[0])]


def population_chart(runs, out):
    W, H = 760, 400
    L, R, T, B = 64, 170, 64, 48
    pw, ph = W - L - R, H - T - B
    max_day = 49
    ymin, ymax = 10, 10000
    x = lambda d: L + pw * d / max_day
    y = lambda v: T + ph * (1 - (math.log10(v) - math.log10(ymin)) / (math.log10(ymax) - math.log10(ymin)))
    body = [f'<text x="{L}" y="26" class="t">Population per seed (log scale)</text>',
            f'<text x="{L}" y="44" class="m">population at dawn each day (night survivors); 512×512 world, 1000 founders; thin lines single seeds, thick lines median</text>']
    for v in (10, 30, 100, 300, 1000, 3000, 10000):
        body.append(f'<line x1="{L}" x2="{L+pw}" y1="{y(v):.1f}" y2="{y(v):.1f}" stroke="var(--grid)" stroke-width="1"/>')
        body.append(f'<text x="{L-8}" y="{y(v)+4:.1f}" text-anchor="end" class="m num">{v if v < 1000 else str(v//1000)+"K"}</text>')
    for d in range(0, max_day + 1, 7):
        body.append(f'<text x="{x(d):.1f}" y="{T+ph+18}" text-anchor="middle" class="m num">{d}</text>')
    body.append(f'<text x="{L+pw/2}" y="{H-8}" text-anchor="middle" class="m">day (4096 ticks)</text>')
    body.append(f'<line x1="{L}" x2="{L+pw}" y1="{T+ph}" y2="{T+ph}" stroke="var(--axis)" stroke-width="1"/>')

    def series(batch):
        per = []
        for sd in seeds(batch):
            rows = load(sd / "stats.csv")
            if not rows:
                continue
            # tick 0 (start) and each dawn (tick = 3072 mod 4096): the population that survived the night
            pts = [(0.0, 1000)] + [(int(r["tick"]) / 4096, int(r["pop"])) for r in rows if int(r["tick"]) % 4096 == 3072]
            last = rows[-1]
            extinct = int(last["pop"]) == 0
            if extinct and pts[-1][1] != 0:
                pts.append((int(last["tick"]) / 4096, 0))
            per.append((pts, extinct))
        return per

    def median_line(per):
        ticks = sorted({t for pts, _ in per for t, _ in pts})
        med = []
        for t in ticks:
            vals = []
            for pts, extinct in per:
                lk = dict(pts)
                if t in lk:
                    vals.append(lk[t])
                elif extinct and t > pts[-1][0]:
                    vals.append(0)
            # a median needs every seed: stop where a seed still running has no value yet
            if len(vals) < len(per):
                break
            med.append((t, st.median(vals)))
        return med

    def path(pts):
        # an extinction (0) is drawn down to the axis floor; other sub-floor points are skipped
        pts = [(d, max(v, ymin)) for d, v in pts if v >= ymin or v == 0]
        return " ".join(f"{'M' if i == 0 else 'L'}{x(d):.1f},{y(v):.1f}" for i, (d, v) in enumerate(pts))

    exps = present(runs)
    data = {name: series(Path(runs) / d) for d, name, _ in exps}
    for _, name, c in exps:
        for pts, _ in data[name]:
            body.append(f'<path d="{path(pts)}" fill="none" stroke="var(--{c})" stroke-opacity="0.22" stroke-width="1"/>')
    meds = {}
    labels = []
    for _, name, c in exps:
        med = median_line(data[name])
        meds[name] = med
        body.append(f'<path d="{path(med)}" fill="none" stroke="var(--{c})" stroke-width="2" stroke-linejoin="round" stroke-linecap="round"/>')
        alive = [p for p in med if p[1] >= ymin]
        if all(p[-1][1] == 0 for p, _ in data[name]):
            ext_day = max(p[-1][0] for p, _ in data[name])
            n = len(data[name])
            body.append(f'<circle cx="{x(ext_day):.1f}" cy="{y(ymin):.1f}" r="4" fill="var(--{c})" stroke="var(--surface)" stroke-width="2"/>')
            body.append(f'<text x="{x(ext_day)+10:.1f}" y="{y(ymin)-8:.1f}">{name}: all {n} seeds extinct by day {ext_day:.1f}</text>')
        elif alive:
            e = alive[-1]
            body.append(f'<circle cx="{x(e[0]):.1f}" cy="{y(e[1]):.1f}" r="4" fill="var(--{c})" stroke="var(--surface)" stroke-width="2"/>')
            labels.append([y(e[1]) + 4, x(e[0]) + 10, f"{name} median {e[1]:.0f}"])
    # keep end labels at least 14 px apart
    labels.sort()
    for k in range(1, len(labels)):
        labels[k][0] = max(labels[k][0], labels[k - 1][0] + 14)
    for ly_, lx_, text in labels:
        body.append(f'<text x="{lx_:.1f}" y="{ly_:.1f}" class="num">{text}</text>')
    # legend
    lx, ly = L + pw + 16, T + 8
    order = sorted(exps, key=lambda e: e[1], reverse=True)
    for k, (_, name, c) in enumerate(order):
        body.append(f'<line x1="{lx}" x2="{lx+18}" y1="{ly+k*20}" y2="{ly+k*20}" stroke="var(--{c})" stroke-width="2"/>')
        body.append(f'<text x="{lx+24}" y="{ly+k*20+4}">{name}</text>')
    Path(out, "population.svg").write_text(svg(W, H, "".join(body), "Population per seed by founder"))
    return meds


def budget(batch):
    rows = []
    for sd in seeds(batch):
        day = [r for r in load(sd / "stats.csv") if int(r["tick"]) <= 1024]
        at = sum(int(r["pop"]) for r in day) * 256
        if not at:
            continue
        rows.append({k: sum(int(r[k]) for r in day) / at for k in ("upkeep_q", "vm_q", "attempt_q", "effect_q")}
                    | {"income": sum(int(r["photosynthesis"]) for r in day) * 1666 / at})
    return {k: st.mean(r[k] for r in rows) for k in rows[0]}


def budget_chart(runs, out):
    bud = [(name, budget(Path(runs) / d)) for d, name, _ in present(runs)]
    W, H = 760, 194 + 56 * len(bud)
    L, R, T = 110, 40, 64
    pw = W - L - R
    vmax = 300
    x = lambda v: L + pw * v / vmax
    parts = (("upkeep_q", "upkeep", "s1"), ("vm_q", "VM", "s2"), ("attempt_q", "effect attempts", "s3"), ("effect_q", "effect amounts", "s4"))
    body = [f'<text x="{24}" y="26" class="t">First afternoon energy budget (q per agent-tick)</text>',
            f'<text x="{24}" y="44" class="m">mean over seeds, ticks 0–1024; the black tick is photosynthesis income</text>']
    for v in range(0, vmax + 1, 50):
        body.append(f'<line x1="{x(v):.1f}" x2="{x(v):.1f}" y1="{T}" y2="{T+8+56*len(bud)}" stroke="var(--grid)" stroke-width="1"/>')
        body.append(f'<text x="{x(v):.1f}" y="{T+24+56*len(bud)}" text-anchor="middle" class="m num">{v}</text>')
    for i, (name, bd) in enumerate(bud):
        yy = T + 18 + i * 56
        body.append(f'<text x="{L-12}" y="{yy+16}" text-anchor="end" style="fill:var(--ink)">{name}</text>')
        acc = 0.0
        for key, _, c in parts:
            w = bd[key]
            gap = 2 if acc > 0 else 0
            body.append(f'<rect x="{x(acc)+gap:.1f}" y="{yy}" width="{max(0, x(acc+w)-x(acc)-gap):.1f}" height="24" fill="var(--{c})"/>')
            acc += w
        body.append(f'<text x="{x(acc)+8:.1f}" y="{yy+16}" class="num">spend {acc:.0f}</text>')
        inc = bd["income"]
        body.append(f'<line x1="{x(inc):.1f}" x2="{x(inc):.1f}" y1="{yy-6}" y2="{yy+30}" stroke="var(--ink)" stroke-width="2"/>')
        body.append(f'<text x="{x(inc):.1f}" y="{yy-10}" text-anchor="middle" class="m num">income {inc:.0f}</text>')
    lx, ly = L, T + 46 + 56 * len(bud)
    for k, (_, label, c) in enumerate(parts):
        body.append(f'<rect x="{lx+k*150}" y="{ly-9}" width="12" height="12" rx="2" fill="var(--{c})"/>')
        body.append(f'<text x="{lx+k*150+18}" y="{ly+2}">{label}</text>')
    Path(out, "budget.svg").write_text(svg(W, H, "".join(body), "First afternoon energy budget by founder"))
    return bud


def survival_chart(groups, out):
    """groups: [(name, colour, [(phase label, rate, n), ...]), ...] drawn as grouped bars."""
    W, H = 760, 340
    L, R, T, B = 64, 40, 92, 56
    pw, ph = W - L - R, H - T - B
    phases = [lbl for lbl, _, _ in groups[0][2]]
    band = pw / len(phases)
    bw = 24
    y = lambda v: T + ph * (1 - v)
    names = " vs ".join(g[0] for g in groups)
    body = [f'<text x="{L}" y="26" class="t">Newborns alive one day after birth, by birth time</text>',
            f'<text x="{L}" y="44" class="m">{names}; seeds pooled; births in the last day of a run are excluded</text>']
    for k, (name, c, _) in enumerate(groups):
        body.append(f'<rect x="{L+k*130}" y="{56}" width="12" height="12" rx="2" fill="var(--{c})"/>')
        body.append(f'<text x="{L+k*130+18}" y="{67}">{name}</text>')
    for v in (0, 0.25, 0.5, 0.75, 1.0):
        body.append(f'<line x1="{L}" x2="{L+pw}" y1="{y(v):.1f}" y2="{y(v):.1f}" stroke="var(--grid)" stroke-width="1"/>')
        body.append(f'<text x="{L-8}" y="{y(v)+4:.1f}" text-anchor="end" class="m num">{int(v*100)}%</text>')
    for i, label in enumerate(phases):
        cx0 = L + band * i + band / 2
        span = len(groups) * bw + (len(groups) - 1) * 6
        for k, (name, c, rows) in enumerate(groups):
            _, rate, cnt = rows[i]
            cx = cx0 - span / 2 + bw / 2 + k * (bw + 6)
            if cnt:
                top = y(rate)
                hgt = T + ph - top
                if hgt > 0:
                    r = min(4, hgt)
                    body.append(f'<path d="M{cx-bw/2:.1f},{T+ph} V{top+r:.1f} Q{cx-bw/2:.1f},{top:.1f} {cx-bw/2+r:.1f},{top:.1f} '
                                f'H{cx+bw/2-r:.1f} Q{cx+bw/2:.1f},{top:.1f} {cx+bw/2:.1f},{top+r:.1f} V{T+ph} Z" fill="var(--{c})">'
                                f'<title>{name}, {label}: {rate:.0%} of {cnt}</title></path>')
                body.append(f'<text x="{cx:.1f}" y="{top-6:.1f}" text-anchor="middle" class="num" style="fill:var(--ink);font-size:11px">{rate:.0%}</text>')
        ns = " / ".join(f"{rows[i][2]:,}" for _, _, rows in groups)
        body.append(f'<text x="{cx0:.1f}" y="{T+ph+16}" text-anchor="middle" class="m">{label}</text>')
        body.append(f'<text x="{cx0:.1f}" y="{T+ph+32}" text-anchor="middle" class="m num">n={ns}</text>')
    body.append(f'<line x1="{L}" x2="{L+pw}" y1="{T+ph}" y2="{T+ph}" stroke="var(--axis)" stroke-width="1"/>')
    Path(out, "newborn_survival.svg").write_text(svg(W, H, "".join(body), "Newborn one-day survival by birth time"))


def newborn_survival(batch):
    """Pool births across seeds: survival >= 1 day by birth phase."""
    bins = [(0, 512, "dawn"), (512, 1024, "morning"), (1024, 1536, "early afternoon"), (1536, 2048, "late afternoon")]
    acc = {b[2]: [0, 0] for b in bins}
    for sd in seeds(batch):
        deaths = {int(d["uid"]): int(d["age"]) for d in load(sd / "deaths.csv")}
        stats = load(sd / "stats.csv")
        if not stats:
            continue
        end = int(stats[-1]["tick"])
        for b in load(sd / "births.csv"):
            t = int(b["tick"])
            if end - t < 4096:
                continue
            p = (t + 1024) % 4096
            age = deaths.get(int(b["uid"]))
            ok = age is None or age >= 4096
            for lo, hi, name in bins:
                if lo <= p < hi:
                    acc[name][0] += ok
                    acc[name][1] += 1
    return [(name, (acc[name][0] / acc[name][1]) if acc[name][1] else 0.0, acc[name][1]) for _, _, name in bins]


def carbon_chart(runs, out):
    """Two panels sharing the day axis: CO2 in the air, and carbon held in dead organic matter."""
    exps = [e for e in present(runs) if e[0] != "E1a_founder4"]
    W, H = 760, 360
    L, T, B, gap = 64, 108, 44, 56
    pw = (W - L - 40 - gap) // 2
    ph = H - T - B
    max_day = 49
    panels = [("CO2 in the air (loose C), millions", lambda r: int(r["loose_C"]) / 1e6),
              ("dead organic carbon (loose G + dropped pages), millions", lambda r: (int(r["loose_G"]) + int(r["dropped_pages"])) / 1e6)]
    meds = {}
    for d, name, _ in exps:
        per = []
        for sd in seeds(Path(runs) / d):
            rows = [r for r in load(sd / "stats.csv") if int(r["tick"]) % 4096 == 3072]
            per.append({int(r["tick"]) // 4096: r for r in rows})
        days = sorted(set.intersection(*[set(p) for p in per])) if per else []
        meds[name] = [(day, [st.median(f(p[day]) for p in per) for _, f in panels]) for day in days]
    body = [f'<text x="{L}" y="26" class="t">Carbon: air vs dead matter</text>',
            f'<text x="{L}" y="44" class="m">median over seeds at dawn; 512×512 world</text>']
    for k, (_, name, c) in enumerate(exps):
        body.append(f'<line x1="{L+k*120}" x2="{L+k*120+18}" y1="62" y2="62" stroke="var(--{c})" stroke-width="2"/>')
        body.append(f'<text x="{L+k*120+24}" y="66">{name}</text>')
    for pi, (title, _) in enumerate(panels):
        x0 = L + pi * (pw + gap)
        vals = [v[pi] for m in meds.values() for _, v in m] or [1]
        vmax = max(vals) * 1.1
        vmin = 0
        x = lambda dd: x0 + pw * dd / max_day
        y = lambda v: T + ph * (1 - (v - vmin) / (vmax - vmin))
        body.append(f'<text x="{x0}" y="{T-14}" class="m">{title}</text>')
        step = 1 if vmax <= 6 else 2
        v = 0.0
        while v <= vmax:
            body.append(f'<line x1="{x0}" x2="{x0+pw}" y1="{y(v):.1f}" y2="{y(v):.1f}" stroke="var(--grid)" stroke-width="1"/>')
            body.append(f'<text x="{x0-6}" y="{y(v)+4:.1f}" text-anchor="end" class="m num">{v:g}</text>')
            v += step
        for dd in range(0, max_day + 1, 7):
            body.append(f'<text x="{x(dd):.1f}" y="{T+ph+16}" text-anchor="middle" class="m num">{dd}</text>')
        body.append(f'<line x1="{x0}" x2="{x0+pw}" y1="{T+ph}" y2="{T+ph}" stroke="var(--axis)" stroke-width="1"/>')
        for _, name, c in exps:
            pts = meds[name]
            if not pts:
                continue
            path = " ".join(f"{'M' if i == 0 else 'L'}{x(dd):.1f},{y(v[pi]):.1f}" for i, (dd, v) in enumerate(pts))
            body.append(f'<path d="{path}" fill="none" stroke="var(--{c})" stroke-width="2" stroke-linejoin="round"/>')
    body.append(f'<text x="{W/2}" y="{H-8}" text-anchor="middle" class="m">day (4096 ticks)</text>')
    Path(out, "carbon.svg").write_text(svg(W, H, "".join(body), "Carbon in the air and in dead matter by founder"))
    return meds


if __name__ == "__main__":
    runs, out = sys.argv[1], sys.argv[2]
    Path(out).mkdir(parents=True, exist_ok=True)
    meds = population_chart(runs, out)
    bud = budget_chart(runs, out)
    groups = [(name, c, newborn_survival(Path(runs) / d)) for d, name, c in present(runs) if d != "E1a_founder4"]
    survival_chart(groups, out)
    carbon = carbon_chart(runs, out)
    print("budget:", {n: {k: round(v, 1) for k, v in b.items()} for n, b in bud})
    for name, _, rows in groups:
        print(f"survival by phase, {name}:", [(n, f"{r:.1%}", c) for n, r, c in rows])
    for name, med in meds.items():
        print(f"{name} median dawn population by day:", [(round(d, 2), v) for d, v in med][::7])
    for name, med in carbon.items():
        print(f"{name} carbon (air C, dead G) millions by day:", [(d, [round(x, 2) for x in v]) for d, v in med][::7])
