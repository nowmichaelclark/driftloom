"""Calibration of techniques.py: sections, stem activity, sweeps, echoes, pumping, tape, stereo motion."""
import numpy as np
import scipy.signal as sps

from calsig import SR, RNG, partials, env_ad, string_note, place, fade, t_, pan_expected
import techniques as T


def pad(freqs, dur, level=0.1, harm=3, seed=0):
    r = np.random.default_rng(seed)
    t = t_(dur)
    y = np.zeros_like(t)
    for f in freqs:
        for k in range(1, harm + 1):
            y += (level / k) * np.sin(2 * np.pi * f * k * t + r.uniform(0, 6.28))
    return y


def noise_band(lo, hi, dur, level=0.05, seed=0):
    r = np.random.default_rng(seed)
    sos = sps.butter(4, [lo, hi], "bandpass", fs=SR, output="sos")
    return sps.sosfilt(sos, r.standard_normal(int(dur * SR))) * level


# ---------------------------------------------------------------------------------------------
def cal_sections(row):
    # each section: a different chord, brightness and noise colour; hard edges at known times
    chords = [[220, 277, 330], [196, 247, 294], [175, 220, 262], [262, 330, 392], [233, 294, 349], [247, 311, 370]]
    bands = [(200, 800), (1500, 4000), (400, 1500), (3000, 6000), (100, 500), (800, 2500)]
    for label, lengths in (("long sections (20-40 s)", [30, 28, 42, 28, 22]), ("short sections (10-14 s)", [14, 10, 12, 10, 14, 10])):
        edges = np.cumsum([0] + lengths)
        y = np.zeros(int(edges[-1] * SR))
        for i, L in enumerate(lengths):
            seg = pad(chords[i % len(chords)], L, 0.08, harm=2 + i % 3, seed=i) + noise_band(*bands[i % len(bands)], L, 0.04, seed=i)
            a, b = int(edges[i] * SR), int(edges[i + 1] * SR)
            y[a:b] += seg
        sec = T.sections(y)
        det = np.array([b["t"] for b in sec["boundaries"]])
        truth = edges[1:-1]
        hits, errs = 0, []
        for tb in truth:
            if len(det):
                j = np.argmin(np.abs(det - tb))
                if abs(det[j] - tb) <= 4:
                    hits += 1
                    errs.append(det[j] - tb)
        row("techniques", f"sections, {label}: boundaries found of {len(truth)}", len(truth), hits, (len(truth) - 0 if label.startswith("long") else 0) if False else (len(truth) - (0 if label.startswith("long") else 2), len(truth)), "",
            f"{len(det)} detected (false alarms {len(det) - hits}); detected at {np.round(det, 1).tolist()}")
        if errs:
            row("techniques", f"sections, {label}: timing error (median abs)", 0.0, float(np.median(np.abs(errs))), 2.0, "s")


def cal_sections_hard(row):
    """Harder: only the chord changes (same timbre), and the change is a 4-s crossfade."""
    lengths = [24, 20, 28, 22, 26]
    chords = [[220, 277, 330], [196, 247, 294], [175, 220, 262], [233, 294, 349], [262, 330, 392]]
    edges = np.cumsum([0] + lengths)
    total = edges[-1]
    y = np.zeros(int(total * SR))
    t = t_(total)
    for i, ch in enumerate(chords):
        g = np.interp(t, [edges[i] - 2, edges[i] + 2, edges[i + 1] - 2, edges[i + 1] + 2], [0, 1, 1, 0]) if i else np.interp(t, [0, edges[1] - 2, edges[1] + 2], [1, 1, 0])
        if i == len(chords) - 1:
            g = np.interp(t, [edges[i] - 2, edges[i] + 2, total], [0, 1, 1])
        y += g * pad(ch, total, 0.08, harm=3, seed=i)
    sec = T.sections(y)
    det = np.array([b["t"] for b in sec["boundaries"]])
    truth = edges[1:-1]
    hits = sum(1 for tb in truth if len(det) and np.abs(det - tb).min() <= 4)
    row("techniques", "sections, chord-only changes with 4-s crossfades: found of 4", 4, hits, (2, 4), "",
        f"{len(det)} detected at {np.round(det, 1).tolist()}; harder than real best cases, so read as a lower bound")


def cal_activity(row):
    dur = 160
    on = [(10, 40), (60, 95), (110, 150)]
    y = np.zeros(int(dur * SR))
    for a, b in on:
        i, j = int(a * SR), int(b * SR)
        y[i:j] = pad([196, 247, 294], b - a, 0.1, seed=a)
    bleed = RNG.standard_normal(len(y)) * 0.0006                    # about -50 dB under the pad
    st = {"other": np.stack([y + bleed, y + bleed])}
    mix = y + bleed
    res = T.stem_activity(st, mix)["other"]
    act = res["active"]
    row("techniques", "stem activity: intervals found of 3", 3, len(act), 0)
    if len(act) == 3:
        e = [abs(a[0] - o[0]) for a, o in zip(act, on)] + [abs(a[1] - o[1]) for a, o in zip(act, on)]
        row("techniques", "stem activity: edge error (max)", 0.0, float(max(e)), 1.5, "s")


def cal_floor_drops(row):
    dur = 140
    t = t_(dur)
    kicks = np.zeros(int(dur * SR))
    k = np.sin(2 * np.pi * np.cumsum(55 + 80 * np.exp(-t[:SR // 3] / 0.03)) / SR) * np.exp(-t[:SR // 3] / 0.08)
    for b in range(int(dur / 0.5)):
        i = int(b * 0.5 * SR)
        kicks[i:i + len(k)] += k[: len(kicks) - i]
    bass = np.sin(2 * np.pi * 55 * t) * 0.3
    drops = [(30.0, 33.0), (60.0, 64.0), (95.0, 97.5)]
    low_gate = np.ones_like(t)
    for a, b in drops:
        low_gate[int(a * SR):int(b * SR)] = 0.0
    low = (kicks + bass) * low_gate
    rest = pad([262, 330, 392], dur, 0.1, harm=3) + noise_band(1500, 4000, dur, 0.02, seed=5)
    st = {"drums": np.stack([kicks * low_gate] * 2), "bass": np.stack([bass * low_gate] * 2),
          "other": np.stack([rest, rest]), "vocals": np.zeros((2, len(t)))}
    res = T.floor_drops(st)
    found = res.get("drops", [])
    row("techniques", "floor drops found (3 true, 2.5 to 4 s long)", 3, len(found), 0, "drops")
    if len(found) == 3:
        e = [abs(f["t0"] - a) for f, (a, b) in zip(found, drops)] + [abs(f["t1"] - b) for f, (a, b) in zip(found, drops)]
        row("techniques", "floor drop edge error (max)", 0.0, float(max(e)), 0.5, "s")
    # a full stop (everything gone) is not a floor drop
    gate = np.ones_like(t); gate[int(50 * SR):int(54 * SR)] = 0
    st2 = {k_: v * gate for k_, v in st.items()}
    for k_ in ("drums", "bass"):
        st2[k_] = st[k_] * gate
    res2 = T.floor_drops(st2)
    row("techniques", "a full stop counted as a floor drop (false alarm)", 0, sum(1 for d in res2.get("drops", []) if 48 < d["t0"] < 56), 0, "drops")
    # a drop of 1.5 s is under the 2 s limit
    gate = np.ones_like(t); gate[int(50 * SR):int(51.5 * SR)] = 0
    st3 = dict(st); st3["drums"] = np.stack([kicks] * 2) * gate; st3["bass"] = np.stack([bass] * 2) * gate
    res3 = T.floor_drops(st3)
    row("techniques", "a 1.5-s drop (under the 2-s limit) reported", 0, sum(1 for d in res3.get("drops", []) if 48 < d["t0"] < 54), "limit", "drops",
        "drops between 2 and about 2.5 s are borderline")
    # no low end at all: the tool must say so rather than count
    st4 = dict(st); st4["drums"] = np.zeros_like(st["drums"]); st4["bass"] = np.stack([bass * (np.arange(len(t)) % (8 * SR) < 2 * SR)] * 2)
    res4 = T.floor_drops(st4)
    row("techniques", "sparse low end: tool declines to count", 1, int("note" in res4), 0, "")


def cal_silence(row):
    # digital silence inside a track (real stems have it): no crash, and no sweep invented from it
    y = pad([220, 277, 330], 60, 0.1, harm=3)
    y[int(20 * SR):int(30 * SR)] = 0.0
    try:
        sw = T.sweeps(y)
        ok = 1
    except Exception as e:                      # noqa: BLE001
        sw, ok = [], 0
    row("techniques", "sweeps on a track with 10 s of digital silence: runs", 1, ok, 0, "")
    row("techniques", "sweeps on that track: sweeps invented", 0, len(sw), 1, "sweeps")


def cal_sweeps(row):
    for oct_, fa, fb in ((4.3, 300, 6000), (2.0, 500, 2000), (1.0, 800, 1600), (0.5, 1000, 1414)):
        dur = 70
        raw = RNG.standard_normal(int(dur * SR)) * 0.1
        out = np.zeros_like(raw)
        zi = None
        blk = 1024
        for i in range(0, len(raw), blk):
            tt = i / SR
            frac = np.clip((tt - 20) / 25, 0, 1)                 # sweep from 20 s to 45 s
            fc = fa * (fb / fa) ** frac
            sos = sps.butter(4, fc, "low", fs=SR, output="sos")
            seg, zi = sps.sosfilt(sos, raw[i:i + blk], zi=np.zeros((sos.shape[0], 2)) if zi is None else zi)
            out[i:i + blk] = seg
        sw = T.sweeps(out)
        up = [s for s in sw if s["dir"] == "up"]
        if oct_ >= 1.0:
            m = max(up, key=lambda s: s["octaves"]) if up else None
            row("techniques", f"sweep up {oct_} octaves of cutoff over 25 s", oct_, m["octaves"] if m else 0.0, max(0.7, oct_ * 0.4), "oct",
                (f"found {m['t0']:.0f}-{m['t1']:.0f} s" if m else "not found"))
        else:
            row("techniques", f"sweep up {oct_} octave (below the 0.7 limit): reported or not", 0, len(up), 1, "", "a miss is the right answer here")


def cal_echoes(row):
    for delay, fb in ((0.375, 0.5), (0.75, 0.45), (0.21, 0.55)):
        dur = 45
        pl = [string_note(f, 1.0, 0.004, 40) for f in (262, 330, 392, 294, 349, 440, 220, 311, 370, 247, 277, 415, 330, 262)]
        y = place(pl, [1 + 3.0 * i for i in range(len(pl))], dur) * 0.4
        d = int(delay * SR)
        out = y.copy()
        g = 1.0
        for k in range(1, 8):
            g *= fb
            out[k * d:] += g * y[: len(y) - k * d]
        r = T.echoes(out)
        got = r["delays"][0]["delay_s"] if r["delays"] else None
        row("techniques", f"echo {int(delay * 1000)} ms, feedback {fb}", delay, got if got is not None else 0.0, 0.015, "s",
            f"{r['isolated_onsets']} isolated onsets" + ("" if got else "; " + r.get("note", "")))
    # no echo: should report none
    pl = [string_note(f, 1.0, 0.004, 40) for f in (262, 330, 392, 294, 349, 440, 220, 311, 370, 247, 277, 415, 330, 262)]
    y = place(pl, [1 + 3.0 * i for i in range(len(pl))], 45) * 0.4
    r = T.echoes(y)
    row("techniques", "dry plucks: no echo reported", 0, len(r["delays"]), 0)
    # plucks over a sustained pad: repeats are hidden; the tool must say "not reliable"
    r2 = T.echoes(y + pad([196, 247, 294], 45, 0.05))
    d2 = r2["delays"][0] if r2["delays"] else None
    row("techniques", "plucks over a pad: reading flagged not reliable (or none)", 1, 1 if (d2 is None or not d2["reliable"]) else 0, 0, "",
        "limit: sustained material hides repeats" + (f"; read {d2['delay_s']:.3f} s with bump {d2['bump_db']:.1f} dB" if d2 else ""))
    for delay in (0.375, 0.75):
        pass


def cal_pumping(row):
    sr = SR
    dur = 40
    period = 0.5
    kicks = np.arange(1.0, dur - 1, period)
    drums = np.zeros(int(dur * sr))
    for k in kicks:
        i = int(k * sr)
        tt = t_(0.3)
        drums[i: i + len(tt)] += np.sin(2 * np.pi * (55 + 90 * np.exp(-tt * 30)) * tt) * np.exp(-tt * 12) * 0.8
    base = pad([196, 247, 294, 392], dur, 0.08, seed=4)
    for depth in (0, 3, 6, 10):
        g = np.ones(len(base))
        for k in kicks:
            i = int(k * sr)
            tt = t_(0.45)
            dip = 10 ** (-depth / 20)
            env = 1 - (1 - dip) * np.exp(-tt / 0.12)
            g[i: i + len(tt)] *= env
        r = T.pumping(drums, base * g)
        if depth == 0:
            row("techniques", "pumping 0 dB (no duck): reads near zero", 0, r.get("pump_db", -1), 0.5, "dB")
        else:
            row("techniques", f"pumping {depth} dB duck, 120 ms recovery: reads 55-110% of the true depth", depth, r.get("pump_db", -1),
                (0.55 * depth, 1.1 * depth), "dB", f"lag {r.get('lag_ms', 0):.0f} ms; under-reads fast ducks (20 ms window), so treat it as a floor")


def cal_tape(row):
    dur = 40
    t = t_(dur)
    fund = [220, 277.2, 330, 440]
    for depth, rate in ((5, 0.8), (12, 0.8), (25, 2.5), (12, 5.0)):
        y = np.zeros_like(t)
        for f in fund:
            ph = 2 * np.pi * np.cumsum(f * 2 ** (depth * np.sin(2 * np.pi * rate * t + f) / 1200)) / SR
            y += 0.1 * (np.sin(ph) + 0.4 * np.sin(2 * ph))
        r = T.tape(y)
        row("techniques", f"tape wobble {depth} cents peak at {rate} Hz: depth", depth, r.get("wobble_peak_cents", 0.0), max(2.5, depth * 0.3), "cents")
        row("techniques", f"tape wobble {depth} cents peak at {rate} Hz: rate", rate, r.get("wobble_hz", 0.0), 0.4, "Hz")
    # speed drift: a chord whose pitch rises 50 cents between 10 s and 25 s
    cents = np.interp(t, [0, 10, 25, dur], [0, 0, 50, 50])
    y = np.zeros_like(t)
    for f in fund:
        ph = 2 * np.pi * np.cumsum(f * 2 ** (cents / 1200)) / SR
        y += 0.1 * (np.sin(ph) + 0.4 * np.sin(2 * ph))
    r = T.tape(y)
    row("techniques", "speed drift: pitch rises 50 cents over 15 s (range)", 50, r.get("speed_drift_range_cents", 0.0), 15, "cents")
    r0 = T.tape(sum(0.1 * (np.sin(2 * np.pi * f * t) + 0.4 * np.sin(4 * np.pi * f * t)) for f in fund))
    row("techniques", "steady chord: wobble reads near zero", 0, r0.get("wobble_peak_cents", 0.0), 2.0, "cents")
    row("techniques", "steady chord: speed drift reads near zero", 0, r0.get("speed_drift_range_cents", 0.0), 5, "cents")


def cal_stereo_motion(row):
    dur = 90
    base = RNG.standard_normal(int(dur * SR)) * 0.1
    base = sps.sosfilt(sps.butter(2, 2000, "low", fs=SR, output="sos"), base)
    t = t_(dur)
    for p_amp, rate in ((0.6, 0.1), (0.9, 0.05), (0.3, 0.2)):
        p = p_amp * np.sin(2 * np.pi * rate * t)
        th = (p + 1) * np.pi / 4
        st = np.stack([base * np.cos(th), base * np.sin(th)])
        r = T.stereo_motion(st)
        exp = (np.sin(th) - np.cos(th)) / (np.sin(th) + np.cos(th))
        truth_range = float(np.percentile(exp, 95) - np.percentile(exp, 5))
        row("techniques", f"stereo movement +-{p_amp} at {rate} Hz: pan range", truth_range, r["pan_range_p5_p95"], 0.1)
        row("techniques", f"stereo movement +-{p_amp} at {rate} Hz: cycle rate", rate, r.get("pan_cycle_hz", 0.0), 0.02, "Hz")
    st = np.stack([base, base])
    r = T.stereo_motion(st)
    row("techniques", "centered stem: pan range near zero", 0, r["pan_range_p5_p95"], 0.05)


TESTS = {"sections": cal_sections, "sections_hard": cal_sections_hard, "activity": cal_activity, "silence": cal_silence, "floor_drops": cal_floor_drops, "sweeps": cal_sweeps, "echoes": cal_echoes,
         "pumping": cal_pumping, "tape": cal_tape, "stereo_motion": cal_stereo_motion}
