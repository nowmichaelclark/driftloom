"""How a long piece moves: the whole mix in 10 s windows, over 10 to 30+ minutes.

Usage: python3 -P longform.py AUDIO OUT.json [--win 10]
       python3 -P longform.py --selftest

Ears 10 (2026-11-02), for the long-form lines on beatless ambient cards (queue
item 88's long pieces and Endless, the glacial vibe, the arc). The deep pass
(album.py) measures sections and sweeps at the scale of seconds; this looks
at minutes. Per window: level (RMS of the mid, dB), brightness (centroid),
band shares, width above 200 Hz (side/mid dB), a change rate (positive spectral
flux against the window's mean magnitude) and a chroma.
From those series:

  * swells: the level and the brightness smoothed over 60 s; peaks that rise
    3 dB (level) or 0.3 octave (brightness) above the troughs on both sides,
    their count, spacing, depth, and the autocorrelation period if one stands
    out (0.3 or more);
  * sections: a novelty curve (a checkerboard kernel 60 s each side) over
    the z-scored windows; boundaries are its peaks, at least 60 s apart;
  * returns: a window "returns" when the nearest window at least 120 s
    earlier is as close as the median distance between neighboring windows;
    the share of windows that return, and whether the end comes back to the
    opening;
  * travel: distance from the opening window over time, where it peaks;
  * harmony at long scale: how often the strongest pitch class changes (a
    minute's windows smoothed), and how many of the windows keep the
    track's three strongest pitch classes on top.

The selftest renders three signals with known answers (a 120 s level swell,
a 240 s filter swing, an A-B-A form) and prints what the tool reads.
Not calibrated beyond that: a guide, read it against a listen.
"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np

SR = 22050
BANDS = [(0, 120), (120, 500), (500, 2000), (2000, 5000), (5000, 11025)]
BAND_NAMES = ["sub_lt120", "low_120_500", "mid_500_2k", "pres_2k_5k", "air_gt5k"]


def db(x):
    return 10 * np.log10(np.maximum(x, 1e-12))


def windows(x, win):
    """x: (2, n) float32. Per-window rows."""
    import librosa
    hop = int(win * SR)
    n = x.shape[1] // hop
    rows = []
    freqs = np.fft.rfftfreq(4096, 1 / SR)
    for i in range(n):
        seg = x[:, i * hop:(i + 1) * hop]
        mid = (seg[0] + seg[1]) / 2
        side = (seg[0] - seg[1]) / 2
        S = np.abs(librosa.stft(mid, n_fft=4096, hop_length=2048)) ** 2
        Ss = np.abs(librosa.stft(side, n_fft=4096, hop_length=2048)) ** 2
        p = S.mean(axis=1)
        tot = p.sum() + 1e-12
        hi = freqs >= 200
        row = {"t": i * win, "level_db": float(db(np.mean(mid ** 2))),
               "centroid_hz": float((freqs * p).sum() / tot),
               "width_db": float(db(Ss.mean(axis=1)[hi].sum()) - db(p[hi].sum())),
               "bands": [float(p[(freqs >= a) & (freqs < b)].sum() / tot) for a, b in BANDS]}
        M = np.sqrt(S)
        # change rate: positive spectral flux a frame against the window's mean magnitude (0 = static;
        # onset counts were noise on drones, Ears 10)
        row["flux"] = float(np.maximum(np.diff(M, axis=1), 0).sum(axis=0).mean() / (M.sum(axis=0).mean() + 1e-12))
        ch = librosa.feature.chroma_stft(S=S, sr=SR, n_fft=4096)
        row["chroma"] = (ch.mean(axis=1) / (ch.mean(axis=1).max() + 1e-12)).tolist()
        rows.append(row)
    return rows


def smooth(v, k):
    if len(v) < k or k < 2:
        return np.asarray(v, float)
    pad = np.pad(np.asarray(v, float), (k // 2, k - 1 - k // 2), mode="edge")
    return np.convolve(pad, np.ones(k) / k, mode="valid")


def swells(v, win, rise):
    """Peaks of a smoothed series that stand `rise` above the lowest point on each side
    before a higher peak. Returns (peak times, depths)."""
    v = np.asarray(v, float)
    peaks = []
    for i in range(1, len(v) - 1):
        if not (v[i] >= v[i - 1] and v[i] > v[i + 1]):
            continue
        left = v[:i][::-1]
        lmin = v[i]
        for u in left:
            if u > v[i]:
                break
            lmin = min(lmin, u)
        rmin = v[i]
        for u in v[i + 1:]:
            if u > v[i]:
                break
            rmin = min(rmin, u)
        depth = v[i] - max(lmin, rmin)
        if depth >= rise:
            peaks.append((i * win, round(float(depth), 2)))
    return [p[0] for p in peaks], [p[1] for p in peaks]


def period(v, win, min_s=30):
    v = np.asarray(v, float) - np.mean(v)
    if len(v) < 8 or np.std(v) < 1e-9:
        return None, None
    ac = np.correlate(v, v, "full")[len(v) - 1:]
    ac = ac / ac[0]
    lo = max(1, int(min_s / win))
    hi = len(v) // 2
    if hi <= lo + 1:
        return None, None
    seg = ac[lo:hi]
    cand = [j for j in range(1, len(seg) - 1) if seg[j] >= seg[j - 1] and seg[j] >= seg[j + 1]]
    if not cand:
        return None, None
    j = max(cand, key=lambda k: seg[k])
    return ((j + lo) * win, round(float(seg[j]), 2)) if seg[j] >= 0.3 else (None, round(float(seg[j]), 2))


def summarize(rows, win):
    t = np.array([r["t"] for r in rows], float)
    lev = np.array([r["level_db"] for r in rows])
    loud = lev > lev.max() - 40
    oct_ = np.log2(np.maximum([r["centroid_hz"] for r in rows], 20.0))
    wid = np.array([r["width_db"] for r in rows])
    ons = np.array([r["flux"] for r in rows])
    B = np.array([r["bands"] for r in rows])
    C = np.array([r["chroma"] for r in rows])
    k = max(2, int(round(60 / win)))
    ls, cs = smooth(lev, k), smooth(oct_, k)
    out = {"window_s": win, "minutes": round(len(rows) * win / 60, 1)}
    pk, dp = swells(ls, win, 3.0)
    lp = period(ls, win)
    out["level"] = {"p10_db": round(float(np.percentile(lev[loud], 10)), 1),
                    "p90_db": round(float(np.percentile(lev[loud], 90)), 1),
                    "range_db": round(float(np.percentile(lev[loud], 90) - np.percentile(lev[loud], 10)), 1),
                    "swells": len(pk), "swell_peaks_s": pk, "swell_depths_db": dp,
                    "swell_spacing_s": float(np.median(np.diff(pk))) if len(pk) > 1 else None,
                    "period_s": lp[0], "period_ac": lp[1]}
    pk, dp = swells(cs, win, 0.3)
    cp = period(cs, win)
    out["brightness"] = {"median_hz": round(float(np.median(2 ** oct_[loud])), 0),
                         "range_oct": round(float(np.percentile(oct_[loud], 90) - np.percentile(oct_[loud], 10)), 2),
                         "swells": len(pk), "swell_peaks_s": pk, "swell_depths_oct": dp,
                         "swell_spacing_s": float(np.median(np.diff(pk))) if len(pk) > 1 else None,
                         "period_s": cp[0], "period_ac": cp[1]}
    out["width"] = {"median_db": round(float(np.median(wid[loud])), 1),
                    "p10_db": round(float(np.percentile(wid[loud], 10)), 1),
                    "p90_db": round(float(np.percentile(wid[loud], 90)), 1)}
    out["change_rate"] = {"median_flux": round(float(np.median(ons)), 3),
                          "p90_flux": round(float(np.percentile(ons, 90)), 3)}
    # sections and returns over z-scored features
    F = np.column_stack([lev, oct_, wid, ons, B, C * 0.5])
    sd = F.std(axis=0)
    Z = (F - F.mean(axis=0)) / np.where(sd < 1e-9, 1, sd)
    n = len(Z)
    D = np.linalg.norm(Z[:, None, :] - Z[None, :, :], axis=2)
    h = max(2, int(round(60 / win)))
    nov = np.zeros(n)
    for i in range(h, n - h):
        a, b = slice(i - h, i), slice(i, i + h)
        nov[i] = D[a, b].mean() - 0.5 * (D[a, a].mean() + D[b, b].mean())
    bounds = []
    thr = np.median(nov[h:n - h]) + np.std(nov[h:n - h]) if n > 2 * h else np.inf
    for i in np.argsort(-nov):
        if nov[i] < thr or nov[i] <= 0:
            break
        if all(abs(i - j) * win >= 60 for j in bounds):
            bounds.append(int(i))
    bounds = sorted(bounds)
    edges = [0] + [b * win for b in bounds] + [n * win]
    out["sections"] = {"boundaries_s": [b * win for b in bounds],
                       "median_section_s": float(np.median(np.diff(edges)))}
    neigh = float(np.median(np.linalg.norm(np.diff(Z, axis=0), axis=1)))
    gap = int(round(120 / win))
    ret = []
    for i in range(gap, n):
        d = D[i, :i - gap + 1].min()
        ret.append(d <= neigh)
    out["returns"] = {"share": round(float(np.mean(ret)), 2) if ret else None,
                      "neighbor_distance": round(neigh, 2)}
    trav = D[0]
    tail = max(1, int(round(60 / win)))
    out["travel"] = {"max_from_opening": round(float(trav.max()), 2), "max_at_s": int(np.argmax(trav)) * win,
                     "end_from_opening": round(float(trav[-tail:].mean()), 2),
                     "end_returns_to_opening": bool(trav[-tail:].mean() <= max(neigh * 1.5, 0.35 * trav.max()))}
    # long-scale harmony
    Cs = np.array([smooth(C[:, j], k) for j in range(12)]).T
    top = Cs.argmax(axis=1)
    ch = int((np.diff(top) != 0).sum())
    top3 = set(np.argsort(-C.mean(axis=0))[:3].tolist())
    keep = np.mean([set(np.argsort(-c)[:3].tolist()) == top3 for c in C])
    names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"]
    out["harmony"] = {"top_pc_changes_per_10min": round(ch / (n * win / 600), 1),
                      "track_top3": [names[j] for j in sorted(top3)],
                      "windows_with_track_top3": round(float(keep), 2)}
    # a coarse timeline, one line a minute
    per = max(1, int(round(60 / win)))
    tl = []
    for m in range(0, n, per):
        s = slice(m, m + per)
        tl.append([int(m * win), round(float(lev[s].mean()), 1), int(2 ** oct_[s].mean()),
                   round(float(wid[s].mean()), 1), round(float(ons[s].mean()), 3), names[int(Cs[m].argmax())]])
    out["timeline_per_min"] = {"columns": ["t_s", "level_db", "centroid_hz", "width_db", "flux", "top_pc"],
                               "rows": tl}
    return out


def run(path, out_path, win=10):
    from cardlib import decode
    x = decode(path, SR, 2)
    rows = windows(x, win)
    res = summarize(rows, win)
    res["source"] = os.path.basename(path)
    res["series"] = {"t": [r["t"] for r in rows],
                     "level_db": [round(r["level_db"], 1) for r in rows],
                     "centroid_hz": [round(r["centroid_hz"]) for r in rows],
                     "width_db": [round(r["width_db"], 1) for r in rows],
                     "flux": [round(r["flux"], 3) for r in rows],
                     "bands": [[round(b, 3) for b in r["bands"]] for r in rows]}
    with open(out_path, "w") as f:
        json.dump(res, f, indent=1)
    return res


def selftest():
    from scipy.signal import butter, sosfilt
    rng = np.random.default_rng(1)
    t = np.arange(int(600 * SR)) / SR
    chord = sum(np.sin(2 * np.pi * f * t) for f in (220, 277.2, 329.6)) / 3
    # 1: a 6 dB (peak to trough 12 dB) level swell every 120 s
    a = chord * 10 ** (6 * np.sin(2 * np.pi * t / 120) / 20) * 0.2
    r1 = summarize(windows(np.stack([a, a]).astype(np.float32), 10), 10)
    print("swell 120 s, 12 dB p-p: swells", r1["level"]["swells"], "spacing", r1["level"]["swell_spacing_s"],
          "period", r1["level"]["period_s"], "range", r1["level"]["range_db"])
    # 2: noise through a lowpass swinging 2 octaves (500 Hz to 2 kHz) every 240 s, in 2 s blocks
    noise = rng.standard_normal(len(t)) * 0.1
    y = np.zeros_like(noise)
    blk = 2 * SR
    for i in range(0, len(t), blk):
        fc = 1000 * 2 ** np.sin(2 * np.pi * (i / SR) / 240)
        y[i:i + blk] = sosfilt(butter(2, fc, "lowpass", fs=SR, output="sos"), noise[i:i + blk])
    r2 = summarize(windows(np.stack([y, y]).astype(np.float32), 10), 10)
    print("filter 240 s, 2 oct: bright swells", r2["brightness"]["swells"], "spacing", r2["brightness"]["swell_spacing_s"],
          "period", r2["brightness"]["period_s"], "range", r2["brightness"]["range_oct"])
    # 3: A (180 s) B (180 s) A (180 s); B another chord, brighter and wider
    tt = np.arange(int(180 * SR)) / SR
    A = sum(np.sin(2 * np.pi * f * tt) for f in (220, 277.2, 329.6)) / 3 * 0.2
    Bl = sum(np.sin(2 * np.pi * f * tt) for f in (392, 493.9, 587.3, 1174.7)) / 4 * 0.2
    Br = sum(np.sin(2 * np.pi * f * tt + 1.3) for f in (392, 493.9, 587.3, 1174.7)) / 4 * 0.2
    L = np.concatenate([A, Bl, A]) + rng.standard_normal(3 * len(tt)) * 0.002
    R = np.concatenate([A, Br, A]) + rng.standard_normal(3 * len(tt)) * 0.002
    r3 = summarize(windows(np.stack([L, R]).astype(np.float32), 10), 10)
    print("A-B-A at 180/360 s: boundaries", r3["sections"]["boundaries_s"], "returns share", r3["returns"]["share"],
          "end returns to opening", r3["travel"]["end_returns_to_opening"])


if __name__ == "__main__":
    a = sys.argv[1:]
    if a and a[0] == "--selftest":
        selftest()
        sys.exit()
    win = 10
    if "--win" in a:
        i = a.index("--win"); win = float(a[i + 1]); del a[i:i + 2]
    r = run(a[0], a[1], win)
    print(json.dumps({k: v for k, v in r.items() if k not in ("series", "timeline_per_min")}, indent=1))
