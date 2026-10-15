"""Calibration: test signals with known answers, run through each card tool.

Usage: python3 -I calibrate.py [events techniques theory space ...] [--out calibration.json]

Every test builds a signal whose answer is known by construction, runs the
tool, and records truth, measured value, error and a pass mark. The table it
writes is what docs/EARS.md quotes as "where each measure is reliable". The
signals are made here, with numpy, from nothing; no audio is read.

Add a test by writing a function that appends rows with row(...).
"""
import json, os, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np

from cardlib import SR, save_json
from calsig import *  # noqa: F401,F403  (signal makers)

ROWS = []


def row(tool, test, truth, measured, tol, unit="", note=""):
    """Record one check. tol is the allowed absolute error (or a (lo, hi) pair for the measured value)."""
    if tol == "limit":                       # reported, not graded: shows where a measure stops being reliable
        ok = None
        err = None if measured is None or truth is None else float(measured - truth)
    elif isinstance(tol, tuple):
        ok = bool(tol[0] <= measured <= tol[1])
        err = None
    else:
        err = None if measured is None or truth is None else float(measured - truth)
        ok = bool(err is not None and abs(err) <= tol)
    ROWS.append({"tool": tool, "test": test, "truth": truth,
                 "measured": None if measured is None else round(float(measured), 3),
                 "error": None if err is None else round(err, 3), "tolerance": tol, "unit": unit,
                 "ok": ok, "note": note})
    print(("limit" if ok is None else "ok   " if ok else "MISS ") + f"{tool:12s} {test:46s} truth={truth} measured={ROWS[-1]['measured']} {unit}", flush=True)


# --- events ----------------------------------------------------------------------
def cal_events():
    import events as E
    # one stem, 40 notes, bell and string alternating, several pitches, 2.5 s apart
    pitches = [196, 262, 330, 440, 523]
    notes, times, kind = [], [], []
    for i in range(40):
        f = pitches[(i // 2) % len(pitches)] * (1 + 0.0)
        if i % 2 == 0:
            notes.append(string_note(f, 2.4))
            kind.append(0)
        else:
            notes.append(bell_note(f, 2.4))
            kind.append(1)
        times.append(0.5 + 2.5 * i)
    y = place(notes, times, 0.5 + 2.5 * 40 + 1)
    y = y / np.abs(y).max() * 0.5
    st = np.stack([y, y])
    total = (st ** 2).mean(axis=0) * 1.0
    others = np.full_like(total, 1e-12)
    rows = E.events_for_stem("other", st, total, others, 200)
    row("events", "events found (40 notes)", 40, len(rows), 2, "events")
    lab, sil = E.group_events(rows)
    # match each event to the nearest note time to get its true kind
    tk = np.array(times)
    true = np.array([kind[int(np.argmin(np.abs(tk - r["t"])))] for r in rows])
    purity = 0.0
    for g in set(lab):
        m = lab == g
        purity += max((true[m] == 0).sum(), (true[m] == 1).sum())
    row("events", "bell vs string: group purity", 1.0, purity / len(rows), 0.08, "", f"{len(set(lab))} groups, silhouette {sil:.2f}")
    nanb = sum(1 for r, k in zip(rows, true) if k == 1 and not np.isfinite(r["inharm_cents"]))
    inh_s = np.nanmedian([r["inharm_cents"] for r, k in zip(rows, true) if k == 0])
    inh_b = np.nanmedian([r["inharm_cents"] for r, k in zip(rows, true) if k == 1])
    print("bells with no f0:", nanb)
    row("events", "string: cents off whole multiples (median)", 0.0, inh_s, 12, "cents")
    row("events", "bell: cents off whole multiples (median)", None, inh_b, (60, 700), "cents",
        "bell ratios 2.756 / 5.404 are far from whole multiples")
    hs = np.median([r["harm_share"] for r, k in zip(rows, true) if k == 0])
    hb = np.nanmedian([r["harm_share"] for r, k in zip(rows, true) if k == 1])
    row("events", "string harmonic share (median)", 1.0, hs, 0.12, "")
    row("events", "bell harmonic share (median, should be low)", None, hb, (0.0, 0.7), "")
    # f0 error on strings
    errs = []
    for r in rows:
        if r["f0_hz"] and np.isfinite(r["f0_hz"]):
            nearest = min(pitches, key=lambda p: abs(np.log2(r["f0_hz"] / p)))
            if kind[int(np.argmin(np.abs(tk - r["t"])))] == 0:
                errs.append(1200 * np.log2(r["f0_hz"] / nearest))
    row("events", "string f0 error (median abs)", 0.0, np.median(np.abs(errs)), 8, "cents")

    # known attack and decay
    for att, dec in [(0.005, 30), (0.02, 20), (0.08, 10), (0.2, 6)]:
        y1 = place([partials(330, [1, 2], [1, 0.3], 3.0) * env_ad(3.0, att, dec)], [0.5], 4.0) * 0.4
        st1 = np.stack([y1, y1]); tot = (st1 ** 2).mean(axis=0)
        rs = E.events_for_stem("other", st1, tot, np.full_like(tot, 1e-12), 50)
        r0 = max(rs, key=lambda r: r["dur_s"]) if rs else None
        if r0:
            # measured attack runs from -20 dB to the peak; truth for a linear ramp: 0.9 * attack (20 dB = 10% of amplitude)
            row("events", f"attack {int(att * 1000)} ms (linear ramp), 10%-to-peak", att * 0.9 * 1000, r0["attack_ms"],
                max(6, att * 1000 * 0.25), "ms")
            row("events", f"decay {dec} dB/s", -dec, r0["decay_db_s"], max(3, dec * 0.2), "dB/s")
        else:
            row("events", f"attack {int(att * 1000)} ms", att * 900, None, 5, "ms", "no event found")

    # brightness over the note: low-pass swept 4 kHz -> 500 Hz (-3 octaves) over the note
    from scipy.signal import butter, sosfilt
    d = 1.5
    ks = np.arange(1, 30)
    raw = partials(110, ks, 1.0 / ks, d)
    out = np.zeros_like(raw)
    blk = 512
    zi = None
    for i in range(0, len(raw), blk):
        frac = i / len(raw)
        fc = 4000 * (500 / 4000) ** frac
        sos = butter(2, fc, "low", fs=SR, output="sos")
        seg, zi_ = sosfilt(sos, raw[i:i + blk], zi=np.zeros((sos.shape[0], 2)) if zi is None else zi)
        zi = zi_
        out[i:i + blk] = seg
    sig = fade(out * env_ad(d, 0.01, 2))
    y1 = place([sig], [0.5], d + 1.0) * 0.4

    def cen(x):
        fr = np.fft.rfftfreq(len(x), 1 / SR); sp = np.abs(np.fft.rfft(x * np.hanning(len(x)))) ** 2
        b = (fr > 60) & (fr < 9000)
        return (fr[b] * sp[b]).sum() / sp[b].sum()
    truth = np.log2(cen(sig[int(0.9 * len(sig)):int(0.98 * len(sig))]) / cen(sig[int(0.02 * len(sig)):int(0.1 * len(sig))]))
    st1 = np.stack([y1, y1]); tot = (st1 ** 2).mean(axis=0)
    rs = E.events_for_stem("other", st1, tot, np.full_like(tot, 1e-12), 50)
    r0 = max(rs, key=lambda r: r["dur_s"])
    row("events", "brightness slope, filter swept 4 kHz to 500 Hz (centroid octaves)", truth, r0["bright_slope_oct"], 0.35, "oct",
        "truth = the clean signal's own centroid start vs end; the filter moves 3 octaves, the centroid less")

    # vibrato
    for depth, rate in [(25, 5.5), (12, 6.5), (40, 4.0)]:
        t = t_(3.0)
        ph = 2 * np.pi * np.cumsum(330 * 2 ** (depth * np.sin(2 * np.pi * rate * t) / 1200)) / SR
        y1 = place([(np.sin(ph) + 0.4 * np.sin(2 * ph)) * env_ad(3.0, 0.02, 1)], [0.3], 3.6) * 0.4
        st1 = np.stack([y1, y1]); tot = (st1 ** 2).mean(axis=0)
        rs = E.events_for_stem("other", st1, tot, np.full_like(tot, 1e-12), 50)
        r0 = max(rs, key=lambda r: r["dur_s"])
        row("events", f"vibrato depth {depth} cents at {rate} Hz", depth, r0["vib_cents"], max(6, depth * 0.3), "cents")
        row("events", f"vibrato rate {rate} Hz", rate, r0["vib_hz"], 0.8, "Hz")

    # detune beating: two saws 7 and 15 cents apart
    for cents in (7, 15):
        a = partials(220, np.arange(1, 8), 1.0 / np.arange(1, 8), 4.0)
        b = partials(220 * 2 ** (cents / 1200), np.arange(1, 8), 1.0 / np.arange(1, 8), 4.0)
        y1 = place([(a + b) * env_ad(4.0, 0.05, 0.5)], [0.3], 4.6) * 0.3
        st1 = np.stack([y1, y1]); tot = (st1 ** 2).mean(axis=0)
        rs = E.events_for_stem("other", st1, tot, np.full_like(tot, 1e-12), 50)
        r0 = max(rs, key=lambda r: r["dur_s"])
        row("events", f"detune {cents} cents (two saws at 220 Hz)", cents, r0["detune_cents"], max(3, cents * 0.35), "cents",
            f"beat {r0['beat_hz']:.2f} Hz, depth {r0['beat_depth']:.2f}")

    # pan and level
    base = place([string_note(262, 2.0)] * 6, [0.5 + 2.2 * i for i in range(6)], 15.0) * 0.4
    for p in (-1.0, -0.5, 0.0, 0.3, 1.0):
        st1 = pan_stereo(base, p)
        other = (RNG.standard_normal(len(base)) * 0.02)
        tot = (st1 ** 2).mean(axis=0) + other ** 2
        rs = E.events_for_stem("other", st1, tot, other ** 2 + 1e-12, 50)
        pans = [r["pan"] for r in rs]
        row("events", f"pan {p:+.1f} (constant power)", pan_expected(p), np.median(pans), 0.04, "")
    # level against the mix: stem 6 dB under a second stem playing the same time
    y1 = place([string_note(262, 2.0)] * 4, [0.5 + 2.4 * i for i in range(4)], 11.0) * 0.2
    loud = place([string_note(523, 2.0)] * 4, [0.5 + 2.4 * i for i in range(4)], 11.0) * 0.4
    st1 = np.stack([y1, y1]); tot = ((y1 + loud) ** 2) * 1.0
    # mix power = sum of independent powers (the two are at different pitches)
    tot = y1 ** 2 + loud ** 2
    rs = E.events_for_stem("other", st1, tot, loud ** 2 + 1e-12, 50)
    exp = 10 * np.log10(np.mean(y1 ** 2) / np.mean(tot))
    row("events", "level against the mix (second stem 6 dB louder)", 10 * np.log10(0.2 ** 2 / (0.2 ** 2 + 0.4 ** 2)),
        np.median([r["vs_mix_db"] for r in rs]), 1.5, "dB")
    return


# --- registry ----------------------------------------------------------------------
ALL = {"events": cal_events}


def main():
    a = sys.argv[1:]
    out = "calibration.json"
    if "--out" in a:
        i = a.index("--out"); out = a[i + 1]; del a[i:i + 2]
    names = a or list(ALL)
    # techniques, theory, space register themselves when their modules are present
    for mod in ("cal_techniques", "cal_theory", "cal_space"):
        try:
            m = __import__(mod)
            ALL.update({k: (lambda f=f: f(row)) for k, f in m.TESTS.items()})
        except ImportError:
            pass
    names = a or list(ALL)
    for n in names:
        t0 = time.time()
        ALL[n]()
        print(f"-- {n}: {time.time() - t0:.0f} s", flush=True)
    bad = [r for r in ROWS if r["ok"] is False]
    lim = [r for r in ROWS if r["ok"] is None]
    print(f"\n{len(ROWS) - len(bad) - len(lim)} of {len(ROWS) - len(lim)} graded checks within tolerance; {len(lim)} rows show known limits")
    save_json(out, ROWS)


if __name__ == "__main__":
    main()
