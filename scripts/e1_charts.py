#!/usr/bin/env python3
"""Static SVG charts for docs/results/E1.md (no third-party packages).

usage: e1_charts.py RUNS_DIR OUT_DIR
RUNS_DIR must contain E1a_founder4/ and E1b_founder5/ batch directories.
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
    return sorted(p for p in Path(batch).iterdir() if p.name.startswith("seed_"))


def population_chart(runs, out):
    W, H = 760, 400
    L, R, T, B = 64, 150, 64, 48
    pw, ph = W - L - R, H - T - B
    max_day = 49
    ymin, ymax = 10, 3000
    x = lambda d: L + pw * d / max_day
    y = lambda v: T + ph * (1 - (math.log10(v) - math.log10(ymin)) / (math.log10(ymax) - math.log10(ymin)))
    body = [f'<text x="{L}" y="26" class="t">Population per seed (log scale)</text>',
            f'<text x="{L}" y="44" class="m">population at dawn each day (night survivors); 512×512 world, 1000 founders; thin lines single seeds, thick lines median of 20</text>']
    for v in (10, 30, 100, 300, 1000, 3000):
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
            if int(last["pop"]) == 0 and pts[-1][1] != 0:
                pts.append((int(last["tick"]) / 4096, 0))
            per.append(pts)
        return per

    def median_line(per):
        ticks = sorted({t for pts in per for t, _ in pts})
        lookup = [dict(pts) for pts in per]
        last = [pts[-1][0] for pts in per]
        med = []
        for t in ticks:
            vals = [lk.get(t, 0 if t > la else None) for lk, la in zip(lookup, last)]
            vals = [v for v in vals if v is not None]
            med.append((t, st.median(vals)))
        return med

    def path(pts):
        # an extinction (0) is drawn down to the axis floor; other sub-floor points are skipped
        pts = [(d, max(v, ymin)) for d, v in pts if v >= ymin or v == 0]
        return " ".join(f"{'M' if i == 0 else 'L'}{x(d):.1f},{y(v):.1f}" for i, (d, v) in enumerate(pts))

    b = series(Path(runs) / "E1b_founder5")
    a = series(Path(runs) / "E1a_founder4")
    for pts in b:
        body.append(f'<path d="{path(pts)}" fill="none" stroke="var(--s1)" stroke-opacity="0.22" stroke-width="1"/>')
    for pts in a:
        body.append(f'<path d="{path(pts)}" fill="none" stroke="var(--s2)" stroke-opacity="0.22" stroke-width="1"/>')
    mb, ma = median_line(b), median_line(a)
    body.append(f'<path d="{path(mb)}" fill="none" stroke="var(--s1)" stroke-width="2" stroke-linejoin="round" stroke-linecap="round"/>')
    body.append(f'<path d="{path(ma)}" fill="none" stroke="var(--s2)" stroke-width="2" stroke-linejoin="round" stroke-linecap="round"/>')
    # end labels
    eb = [p for p in mb if p[1] >= ymin][-1]
    body.append(f'<circle cx="{x(eb[0]):.1f}" cy="{y(eb[1]):.1f}" r="4" fill="var(--s1)" stroke="var(--surface)" stroke-width="2"/>')
    body.append(f'<text x="{x(eb[0])+10:.1f}" y="{y(eb[1])+4:.1f}" class="num">Founder-5 median {eb[1]:.0f}</text>')
    ext_day = max(pts[-1][0] for pts in a)
    body.append(f'<circle cx="{x(ext_day):.1f}" cy="{y(ymin):.1f}" r="4" fill="var(--s2)" stroke="var(--surface)" stroke-width="2"/>')
    body.append(f'<text x="{x(ext_day)+10:.1f}" y="{y(ymin)-8:.1f}">Founder-4: all 20 seeds extinct by day {ext_day:.1f}</text>')
    # legend
    lx, ly = L + pw + 16, T + 8
    for k, (c, name) in enumerate((("s1", "Founder-5"), ("s2", "Founder-4"))):
        body.append(f'<line x1="{lx}" x2="{lx+18}" y1="{ly+k*20}" y2="{ly+k*20}" stroke="var(--{c})" stroke-width="2"/>')
        body.append(f'<text x="{lx+24}" y="{ly+k*20+4}">{name}</text>')
    Path(out, "population.svg").write_text(svg(W, H, "".join(body), "Population per seed, Founder-4 vs Founder-5"))
    return mb


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
    bud = [("Founder-4", budget(Path(runs) / "E1a_founder4")), ("Founder-5", budget(Path(runs) / "E1b_founder5"))]
    W, H = 760, 250
    L, R, T = 110, 40, 64
    pw = W - L - R
    vmax = 300
    x = lambda v: L + pw * v / vmax
    parts = (("upkeep_q", "upkeep", "s1"), ("vm_q", "VM", "s2"), ("attempt_q", "effect attempts", "s3"), ("effect_q", "effect amounts", "s4"))
    body = [f'<text x="{24}" y="26" class="t">First afternoon energy budget (q per agent-tick)</text>',
            f'<text x="{24}" y="44" class="m">mean over 20 seeds, ticks 0–1024; the black tick is photosynthesis income</text>']
    for v in range(0, vmax + 1, 50):
        body.append(f'<line x1="{x(v):.1f}" x2="{x(v):.1f}" y1="{T}" y2="{T+120}" stroke="var(--grid)" stroke-width="1"/>')
        body.append(f'<text x="{x(v):.1f}" y="{T+136}" text-anchor="middle" class="m num">{v}</text>')
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
    lx, ly = L, T + 158
    for k, (_, label, c) in enumerate(parts):
        body.append(f'<rect x="{lx+k*150}" y="{ly-9}" width="12" height="12" rx="2" fill="var(--{c})"/>')
        body.append(f'<text x="{lx+k*150+18}" y="{ly+2}">{label}</text>')
    Path(out, "budget.svg").write_text(svg(W, H, "".join(body), "First afternoon energy budget, Founder-4 vs Founder-5"))
    return bud


def survival_chart(phase_rows, out):
    W, H = 760, 320
    L, R, T, B = 64, 40, 72, 56
    pw, ph = W - L - R, H - T - B
    n = len(phase_rows)
    band = pw / n
    y = lambda v: T + ph * (1 - v)
    body = [f'<text x="{L}" y="26" class="t">Newborns alive one day after birth, by birth time</text>',
            f'<text x="{L}" y="44" class="m">Founder-5, all 20 seeds pooled; births in the last day of a run are excluded</text>']
    for v in (0, 0.25, 0.5, 0.75, 1.0):
        body.append(f'<line x1="{L}" x2="{L+pw}" y1="{y(v):.1f}" y2="{y(v):.1f}" stroke="var(--grid)" stroke-width="1"/>')
        body.append(f'<text x="{L-8}" y="{y(v)+4:.1f}" text-anchor="end" class="m num">{int(v*100)}%</text>')
    yscale = y
    for i, (label, rate, cnt) in enumerate(phase_rows):
        cx = L + band * i + band / 2
        if cnt:
            top = yscale(rate)
            bw = 24
            hgt = T + ph - top
            if hgt > 0:
                r = min(4, hgt)
                body.append(f'<path d="M{cx-bw/2:.1f},{T+ph} V{top+r:.1f} Q{cx-bw/2:.1f},{top:.1f} {cx-bw/2+r:.1f},{top:.1f} '
                            f'H{cx+bw/2-r:.1f} Q{cx+bw/2:.1f},{top:.1f} {cx+bw/2:.1f},{top+r:.1f} V{T+ph} Z" fill="var(--s1)"/>')
            body.append(f'<text x="{cx:.1f}" y="{top-6:.1f}" text-anchor="middle" class="num" style="fill:var(--ink)">{rate:.0%}</text>')
            body.append(f'<text x="{cx:.1f}" y="{T+ph+32}" text-anchor="middle" class="m num">n={cnt}</text>')
        body.append(f'<text x="{cx:.1f}" y="{T+ph+16}" text-anchor="middle" class="m">{label}</text>')
    body.append(f'<line x1="{L}" x2="{L+pw}" y1="{T+ph}" y2="{T+ph}" stroke="var(--axis)" stroke-width="1"/>')
    Path(out, "newborn_survival.svg").write_text(svg(W, H, "".join(body), "Newborn one-day survival by birth time"))


def newborn_survival(runs):
    """Pool births across seeds: survival >= 1 day by birth phase."""
    bins = [(0, 512, "dawn"), (512, 1024, "morning"), (1024, 1536, "early afternoon"), (1536, 2048, "late afternoon")]
    acc = {b[2]: [0, 0] for b in bins}
    for sd in seeds(Path(runs) / "E1b_founder5"):
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


if __name__ == "__main__":
    runs, out = sys.argv[1], sys.argv[2]
    Path(out).mkdir(parents=True, exist_ok=True)
    mb = population_chart(runs, out)
    bud = budget_chart(runs, out)
    surv = newborn_survival(runs)
    survival_chart(surv, out)
    print("budget:", {n: {k: round(v, 1) for k, v in b.items()} for n, b in bud})
    print("survival by phase:", [(n, f"{r:.1%}", c) for n, r, c in surv])
    print("Founder-5 median dawn population by day:", [(round(d, 2), v) for d, v in mb][::7])
