"""Step 4 of the album card: follow a track through time.

Usage: python3 -I techniques.py STEM_DIR MIX_FILE OUT.json

Measures (each has a calibration test in cal_techniques.py):
  sections       boundaries from a novelty curve over a self-similarity matrix
                 of timbre, chroma and band levels (1-s frames, 16-s kernel)
  stem_activity  when each stem enters and leaves (1-s level, thresholds
                 relative to the stem's own loud parts and to the mix)
  floor_drops    kick and bass (drums + bass stems) gone for 2 s or more while the
                 rest keeps playing; the Ears 1 "floor drop", now with timestamps
  sweeps         filter / brightness sweeps: monotone moves of the spectral
                 rolloff (90% energy) of 0.7 octave or more over 5 s or more
  echoes         delay times: onset-triggered, stacked decay envelopes with
                 the smooth decay taken out; the bumps are the repeats
  pumping        level dip of the non-drum stems locked to the kick, over and
                 above what random times give
  tape           pitch wobble (peak cents and rate) of held partials, and
                 common-mode speed drift across all of them
  stereo_motion  per stem: pan range, how fast it moves, and its rate

Reliability limits are measured, not guessed: docs/EARS.md quotes the
calibration table. In short: sections of 10 s or longer; sweeps of 0.7
octave or more; echoes only where 6 or more notes ring alone for 1.5 s;
pumping of 1.5 dB or more; wobble of 4 cents or more.
"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
import scipy.signal as sps

from cardlib import SR, load_stems, mono, db, mmss, rms_env, smooth, runs, save_json, decode


# --- sections ----------------------------------------------------------------------
def sections(y, sr=SR, hop_s=1.0, half_s=8, min_gap_s=10):
    import librosa
    hop = int(hop_s * sr)
    mf = librosa.feature.mfcc(y=y, sr=sr, n_mfcc=13, hop_length=hop, n_fft=4096)[1:]
    ch = librosa.feature.chroma_stft(y=y, sr=sr, hop_length=hop, n_fft=4096)
    S = np.abs(librosa.stft(y, n_fft=4096, hop_length=hop)) ** 2
    fr = librosa.fft_frequencies(sr=sr, n_fft=4096)
    edges = [0, 120, 500, 2000, 5000, sr / 2]
    bl = np.vstack([10 * np.log10(S[(fr >= lo) & (fr < hi)].sum(axis=0) + 1e-9) for lo, hi in zip(edges[:-1], edges[1:])])
    n = min(mf.shape[1], ch.shape[1], bl.shape[1])
    F = np.vstack([mf[:, :n], ch[:, :n] * 6, bl[:, :n] * 0.5])
    F = (F - F.mean(axis=1, keepdims=True)) / np.maximum(F.std(axis=1, keepdims=True), 1e-9)
    F = np.apply_along_axis(lambda r: smooth(r, 3), 1, F)
    Fn = F / np.maximum(np.linalg.norm(F, axis=0, keepdims=True), 1e-9)
    SM = Fn.T @ Fn
    L = int(half_s / hop_s)
    g = np.exp(-0.5 * (np.arange(-L, L)[:, None] ** 2 + np.arange(-L, L)[None, :] ** 2) / (L / 2) ** 2)
    sign = np.sign(np.arange(-L, L) + 0.5)
    K = g * (sign[:, None] * sign[None, :])
    nov = np.zeros(n)
    P = np.pad(SM, L, mode="edge")
    for i in range(n):
        nov[i] = (P[i:i + 2 * L, i:i + 2 * L] * K).sum()
    nov = np.maximum(nov, 0)
    if nov.max() <= 0:
        return {"boundaries": [], "novelty": nov}
    thr = nov.mean() + 0.75 * nov.std()
    pk, pr = sps.find_peaks(nov, height=thr, distance=max(1, int(min_gap_s / hop_s)))
    b = [{"t": float(p * hop_s), "strength": float(nov[p] / nov.max())} for p in pk]
    return {"boundaries": b, "novelty": nov}


def section_lengths(bounds, total_s):
    ts = [0.0] + [b["t"] for b in bounds] + [total_s]
    return [float(b - a) for a, b in zip(ts[:-1], ts[1:])]


# --- stem activity -------------------------------------------------------------------
def stem_activity(stems, mix, sr=SR):
    """{stem: {active: [[t0,t1]], active_share, enters: [t], leaves: [t]}}"""
    out = {}
    mix_env = rms_env(mix, sr, sr // 2)
    mix_ref = np.percentile(mix_env, 90)
    for name, st in stems.items():
        y = mono(st)
        env = rms_env(y, sr, sr // 2)                      # 1-s window, 0.5-s hop
        if np.sqrt((y ** 2).mean()) < 1e-4:
            out[name] = {"active": [], "active_share": 0.0, "enters": [], "leaves": []}
            continue
        ref = np.percentile(env, 90)
        thr = max(ref - 25, mix_ref - 40)
        on = np.zeros(len(env), bool)
        state = False
        for i, v in enumerate(env):                       # hysteresis of 3 dB
            state = (v > thr - 3) if state else (v > thr)
            on[i] = state
        iv = runs(on, 4)                                   # 2 s minimum
        iv = [(a, b) for a, b in iv]
        # join gaps shorter than 3 s
        merged = []
        for a, b in iv:
            if merged and (a - merged[-1][1]) * 0.5 < 3.0:
                merged[-1] = (merged[-1][0], b)
            else:
                merged.append((a, b))
        act = [[a * 0.5, b * 0.5 + 0.5] for a, b in merged]
        total = len(y) / sr
        out[name] = {"active": act, "active_share": float(sum(b - a for a, b in act) / total),
                     "enters": [a for a, b in act if a > 1.0], "leaves": [b for a, b in act if b < total - 1.5]}
    return out


def floor_drops(stems, sr=SR, min_s=2.0):
    """Floor drops: the kick and bass (drums + bass stems) fall away for 2 s or more while
    the rest (other + vocals) keeps playing. Same idea as Ears 1's sections.py, on stems
    instead of a 120 Hz split. Counted only where the low end is normally there (on in
    75% or more of the frames); otherwise the reading is 'low end too sparse to say'.
    A full stop (the rest falls away too) is not a floor drop."""
    low = sum(mono(stems[k]) for k in ("drums", "bass") if k in stems)
    rest = sum(mono(stems[k]) for k in ("other", "vocals") if k in stems)
    if not isinstance(low, np.ndarray) or not isinstance(rest, np.ndarray):
        return {"note": "needs a low stem and a rest stem"}
    win, hop = sr, sr // 4
    el, er = rms_env(low, win, hop), rms_env(rest, win, hop)
    n = min(len(el), len(er))
    el, er = el[:n], er[:n]
    a, b = int(n * 0.05), int(n * 0.95)                       # skip the fades at both ends
    low_off = el < np.percentile(el[a:b], 90) - 18
    rest_on = er > np.percentile(er[a:b], 50) - 12
    mask = np.zeros(n, bool)
    mask[a:b] = (low_off & rest_on)[a:b]
    minutes = (b - a) * 0.25 / 60
    low_normally_on = 1 - float(low_off[a:b].mean())
    res = {"low_on_share": low_normally_on}
    if low_normally_on < 0.75:
        res["note"] = "low end too sparse to say (on in under 75% of the track)"
        return res
    need = int(round((min_s - 1.0) / 0.25)) + 1           # a drop of min_s fills this many whole 1-s windows
    # a run of windows whose centres lie in [i, j) means the drop spans from the first window's start
    # to the last window's end, less a quarter second of slack (calibrated: edges within 0.25 s)
    iv = [(i * 0.25, (j - 1) * 0.25 + 1.0) for i, j in runs(mask, need)]
    res["drops"] = [{"t0": float(x), "t1": float(y_), "len_s": float(y_ - x)} for x, y_ in iv]
    res["drops_per_min"] = len(iv) / minutes
    res["drop_share"] = float(mask[a:b].mean())
    return res


# --- sweeps ---------------------------------------------------------------------------
def spectral_track(y, sr=SR, win_s=1.0, hop_s=0.5, nfft=4096):
    """Per frame: rolloff (90%) and centroid in Hz, plus frame level (dB)."""
    w, h = int(win_s * sr), int(hop_s * sr)
    n = max(0, (len(y) - w) // h + 1)
    win = np.hanning(w)
    fr = np.fft.rfftfreq(nfft, 1 / sr)
    roll, cen, lev = np.zeros(n), np.zeros(n), np.zeros(n)
    for i in range(n):
        seg = y[i * h: i * h + w] * win
        P = np.abs(np.fft.rfft(seg, nfft)) ** 2
        P[fr < 40] = 0
        tot = P.sum() + 1e-18
        c = np.cumsum(P) / tot
        roll[i] = fr[min(np.searchsorted(c, 0.9), len(fr) - 1)]
        cen[i] = (fr * P).sum() / tot
        lev[i] = 10 * np.log10(tot / nfft + 1e-18)
    return roll, cen, lev, hop_s


def _regress_slope(x, half, hop):
    """Least-squares slope over +-half frames (units of x per second)."""
    n = len(x)
    out = np.zeros(n)
    t = np.arange(-half, half + 1) * hop
    den = (t ** 2).sum()
    xp = np.pad(x, half, mode="edge")
    for i in range(n):
        out[i] = (t * xp[i: i + 2 * half + 1]).sum() / den
    return out


def sweeps(y, sr=SR, min_oct=0.7, min_s=5.0):
    """Monotone moves of the 90% rolloff: [{t0, t1, octaves, dir}]."""
    roll, cen, lev, hop = spectral_track(y, sr)
    if len(roll) < 20:
        return []
    ok = lev > lev.max() - 40
    lr = np.log2(np.maximum(roll, 40))
    if ok.any() and not ok.all():
        lr = np.where(ok, lr, np.interp(np.arange(len(lr)), np.where(ok)[0], lr[ok]))
    sm = smooth(lr, 5)
    slope = _regress_slope(sm, 6, hop)                      # 6 s each side
    out = []
    for sign in (1, -1):
        m = (sign * slope) > 0.015                          # 0.015 octave a second
        for a, b in runs(m, int(min_s / hop)):
            span = sm[b - 1] - sm[a]
            if sign * span >= min_oct:
                out.append({"t0": a * hop, "t1": b * hop, "octaves": float(span), "dir": "up" if sign > 0 else "down"})
    out.sort(key=lambda r: r["t0"])
    return out


# --- echoes ------------------------------------------------------------------------------
def echoes(y, sr=SR, max_lag=1.6, min_isolated=6):
    """Echo (repeat) times from stacked decay envelopes after isolated onsets."""
    import events as E
    on, _ = E.detect_onsets(y, sr)
    hop = 110                                              # 5 ms
    env = rms_env(y, 220, hop)
    ehz = sr / hop
    # level of each onset (peak of the next 40 ms); a "source" note is an onset
    # whose later onsets in the next max_lag seconds are all at least 3 dB
    # weaker (repeats are weaker than what they repeat) and which is not itself
    # the tail of something louder just before
    lv = np.array([env[int(t * ehz): int(t * ehz) + 9].max() if int(t * ehz) + 9 <= len(env) else -120 for t in on])
    iso = []
    for i, t in enumerate(on):
        if t < 0.1 or t + max_lag + 0.2 > len(y) / sr:
            continue
        later = [lv[j] for j in range(i + 1, len(on)) if on[j] - t < max_lag]
        before = [lv[j] for j in range(i) if t - on[j] < 0.4]
        if all(l < lv[i] - 3 for l in later) and all(l < lv[i] - 3 for l in before) and lv[i] > -60:
            iso.append(t)
    res = {"isolated_onsets": len(iso), "delays": []}
    if len(iso) < min_isolated:
        res["note"] = f"only {len(iso)} isolated onsets (need {min_isolated}); no echo reading"
        return res
    stack = []
    for t in iso:
        i0 = int(t * ehz)
        seg = env[i0 - 4: i0 + int(max_lag * ehz)]
        if len(seg) < int(max_lag * ehz):
            continue
        p = int(np.argmax(seg[:20]))
        seg = seg[p:p + int((max_lag - 0.05) * ehz)] - seg[p]
        stack.append(seg)
    n = min(len(s) for s in stack)
    A = np.array([s[:n] for s in stack])
    med = np.median(A, axis=0)                             # dB below the peak, typical note
    trend = sps.medfilt(med, 2 * int(0.12 * ehz) + 1)
    # edge-safe trend: median filter pads with zeros, so use a line fit where it misbehaves
    resid = med - trend
    resid[: int(0.04 * ehz)] = 0
    resid[-int(0.06 * ehz):] = 0
    pk, pr = sps.find_peaks(resid, height=1.5, distance=int(0.03 * ehz))
    cand = [(float(p / ehz), float(resid[p])) for p in pk if p / ehz >= 0.06]
    if not cand:
        res["note"] = "no repeats found"
        return res
    # the fundamental delay explains the later bumps (2x, 3x): pick the one with most support
    best = None
    for d, h in cand:
        support = sum(1 for d2, h2 in cand if d2 > d * 1.5 and abs(d2 / d - round(d2 / d)) < 0.06)
        score = h + 1.5 * support
        if best is None or score > best[0]:
            best = (score, d, h, support)
    res["delays"] = [{"delay_s": best[1], "bump_db": best[2], "later_repeats_seen": best[3],
                      "reliable": bool(best[2] >= 3.0 and best[3] >= 1)}]
    res["all_bumps"] = [{"delay_s": d, "db": h} for d, h in cand[:6]]
    return res


# --- pumping -------------------------------------------------------------------------------
def kick_times(drums, sr=SR):
    """Kick hits: peaks of the 30-150 Hz level (prominence 10 dB, 0.15 s apart),
    moved back to where the level first rises within 10 dB of the peak."""
    sos = sps.butter(4, [30, 150], "bandpass", fs=sr, output="sos")
    k = sps.sosfiltfilt(sos, drums)
    hop = 110
    env = smooth(rms_env(k, 441, hop), 3)
    ehz = sr / hop
    pk, _ = sps.find_peaks(env, prominence=10, distance=int(0.15 * ehz), height=env.max() - 35)
    out = []
    for p in pk:
        j = p
        while j > 0 and env[j] > env[p] - 10 and p - j < int(0.08 * ehz):
            j -= 1
        out.append(j / ehz)
    return np.array(out)


def pumping(drums, rest, sr=SR, rng=None):
    """Level dip of `rest` locked to kick onsets, minus the same from random times."""
    import events as E
    rng = rng or np.random.default_rng(3)
    on = kick_times(drums, sr)
    if len(on) < 16:
        return {"kicks": int(len(on)), "note": "too few kicks"}
    ioi = np.diff(on)
    period = float(np.median(ioi[(ioi > 0.2) & (ioi < 1.5)])) if ((ioi > 0.2) & (ioi < 1.5)).any() else 0.5
    hop = 110
    env = rms_env(rest, 440, hop)                          # 20 ms window, 5 ms hop
    ehz = sr / hop
    n = int(min(period, 1.0) * ehz)

    def stack(times):
        rows = []
        for t in times:
            i = int(t * ehz)
            if i + n < len(env):
                rows.append(env[i: i + n])
        return np.mean(np.array(rows), axis=0) if rows else np.zeros(n)

    real = smooth(stack(on), 3)
    ctrl_depths = []
    for _ in range(5):
        rt = rng.uniform(on[0], on[-1], len(on))
        c = smooth(stack(rt), 3)
        ctrl_depths.append(c.max() - c.min())
    depth = float(real.max() - real.min())
    dip_i = int(np.argmin(real))
    return {"kicks": int(len(on)), "period_s": period, "depth_db": depth, "control_depth_db": float(np.mean(ctrl_depths)),
            "pump_db": max(0.0, depth - float(np.mean(ctrl_depths))), "lag_ms": float(dip_i / ehz * 1000)}


# --- tape ------------------------------------------------------------------------------------
def partial_tracks(y, sr=SR, nfft=16384, win=4096, hop=512, fmin=100, fmax=3000, min_s=3.0, max_peaks=6):
    """Cents tracks of held partials: [(t_start, cents array, f_hz)]."""
    n = (len(y) - win) // hop + 1
    w = np.hanning(win)
    fr = np.fft.rfftfreq(nfft, 1 / sr)
    lo, hi = np.searchsorted(fr, fmin), np.searchsorted(fr, fmax)
    tracks, live = [], []                                  # live: dict(t0, c list, f)
    minlen = int(min_s * sr / hop)
    for i in range(n):
        X = np.abs(np.fft.rfft(y[i * hop: i * hop + win] * w, nfft))[lo - 1: hi + 1]
        L = np.log(np.maximum(X, 1e-9))
        pk = np.where((L[1:-1] > L[:-2]) & (L[1:-1] >= L[2:]))[0] + 1
        if len(pk):
            pk = pk[L[pk] > L.max() - 3.5]
            pk = pk[np.argsort(L[pk])[::-1][:max_peaks]]
            a, b, c = L[pk - 1], L[pk], L[pk + 1]
            den = a - 2 * b + c
            off = np.where(den == 0, 0, 0.5 * (a - c) / np.where(den == 0, 1, den))
            f = (lo - 1 + pk + off) * sr / nfft
            cents = 1200 * np.log2(f / 440.0)
        else:
            f, cents = np.array([]), np.array([])
        used, nxt = set(), []
        for tr in live:
            if len(cents):
                d = np.abs(cents - tr["c"][-1])
                j = int(np.argmin(d))
                if d[j] < 35 and j not in used:
                    used.add(j)
                    tr["c"].append(cents[j]); tr["f"] = f[j]
                    nxt.append(tr)
                    continue
            if len(tr["c"]) >= minlen:
                tracks.append(tr)
        for j in range(len(cents)):
            if j not in used:
                nxt.append({"t0": i * hop / sr, "c": [cents[j]], "f": f[j]})
        live = nxt
    tracks += [t for t in live if len(t["c"]) >= minlen]
    return tracks, hop / sr


def window_response(f, T):
    """How much a Hann window of length T seconds attenuates a frequency
    modulation at f Hz (1 at 0 Hz, 0.5 at 1/T). Used to correct wobble and
    vibrato depths read from windowed spectra; capped at 0.3 so a very fast
    wobble is not blown up."""
    x = np.asarray(f, float) * T
    x = np.where(np.abs(np.abs(x) - 1) < 1e-6, 1 + 1e-6, x)
    h = np.abs(np.sinc(x) / (1 - x ** 2))
    return np.maximum(np.where(np.abs(x) < 1e-9, 1.0, h), 0.3)


def tape(y, sr=SR):
    """Wobble (peak cents, Hz) and speed drift from held partials."""
    tracks, h = partial_tracks(y, sr)
    res = {"partials": len(tracks)}
    if len(tracks) < 3:
        res["note"] = "fewer than 3 held partials; no reading"
        return res
    wob, rates = [], []
    for tr in tracks:
        c = np.array(tr["c"])
        k = max(3, int(4.0 / h) | 1)
        trend = sps.medfilt(c, k) if len(c) > k else np.full_like(c, np.median(c))
        r = (c - trend)[k // 2: -(k // 2) or None]
        if len(r) < int(2.0 / h):
            continue
        wob.append(float(np.sqrt(2) * np.std(r)))
        spec = np.abs(np.fft.rfft(r * np.hanning(len(r)), 2048))
        fr = np.fft.rfftfreq(2048, h)
        band = (fr >= 0.3) & (fr <= 8)
        rates.append(float(fr[np.argmax(np.where(band, spec, 0))]))
    if wob:
        rate = float(np.median(rates))
        res["wobble_hz"] = rate
        res["wobble_peak_cents_raw"] = float(np.median(wob))
        res["wobble_peak_cents"] = float(np.median(wob) / window_response(rate, 4096 / sr))
        res["wobble_tracks"] = len(wob)
    # common-mode speed: median over concurrent partials of the cents change per frame
    nfr = int(max(t["t0"] / h + len(t["c"]) for t in tracks)) + 1
    d = [[] for _ in range(nfr)]
    for t in tracks:
        c = np.array(t["c"])
        t0 = int(round(t["t0"] / h))
        for j in range(1, len(c)):
            d[t0 + j].append(c[j] - c[j - 1])
    cm = np.array([np.median(v) if len(v) >= 3 else 0.0 for v in d])
    cum = np.cumsum(cm)
    # slow speed move: change over 4-s windows
    k = int(4.0 / h)
    if len(cum) > 2 * k:
        mov = cum[k:] - cum[:-k]
        res["speed_move_p95_cents_per_4s"] = float(np.percentile(np.abs(mov), 95))
        res["speed_drift_range_cents"] = float(np.percentile(smooth(cum, int(2 / h)), 95) - np.percentile(smooth(cum, int(2 / h)), 5))
    return res


# --- stereo movement -----------------------------------------------------------------------------
def stereo_motion(st, sr=SR, win_s=1.0):
    """Pan over time of one stereo stem: range, rate and the dominant slow rate."""
    w = int(win_s * sr)
    n = st.shape[1] // w
    if n < 20:
        return {"note": "too short"}
    L = np.sqrt((st[0, : n * w].reshape(n, w) ** 2).mean(axis=1))
    R = np.sqrt((st[1, : n * w].reshape(n, w) ** 2).mean(axis=1))
    lev = 20 * np.log10(np.maximum(np.maximum(L, R), 1e-9))
    ok = lev > lev.max() - 35
    pan = (R - L) / np.maximum(R + L, 1e-9)
    if ok.sum() < 20:
        return {"note": "too quiet"}
    p = pan[ok]
    out = {"pan_mean": float(p.mean()), "pan_range_p5_p95": float(np.percentile(p, 95) - np.percentile(p, 5)),
           "pan_rate_per_s": float(np.mean(np.abs(np.diff(pan[ok]))) / win_s)}
    # dominant slow cycle (needs a steady stretch)
    if ok.mean() > 0.8:
        x = pan - pan.mean()
        spec = np.abs(np.fft.rfft(x * np.hanning(len(x)), 4 * len(x)))
        fr = np.fft.rfftfreq(4 * len(x), win_s)
        band = (fr >= 0.02) & (fr <= 0.45)
        i = np.argmax(np.where(band, spec, 0))
        out["pan_cycle_hz"] = float(fr[i])
        out["pan_cycle_strength"] = float(spec[i] / (spec[band].mean() + 1e-12))
    return out


# --- loudness builds -------------------------------------------------------------------------------
def builds(mix, sr=SR, min_db=5.0, min_s=20.0):
    env = smooth(rms_env(mix, sr * 2, sr), 5)               # 2-s window, 1-s hop
    out = []
    slope = np.gradient(env)
    for a, b in runs(smooth(slope, 7) > 0.05, int(min_s)):
        if env[b - 1] - env[a] >= min_db:
            out.append({"t0": float(a), "t1": float(b), "rise_db": float(env[b - 1] - env[a])})
    return out


def run(stem_dir, mix_file, out_json):
    stems = load_stems(stem_dir)
    mix = sum(mono(s) for s in stems.values())
    total = len(mix) / SR
    res = {"seconds": total}
    sec = sections(mix)
    res["sections"] = {"boundaries": sec["boundaries"], "lengths_s": section_lengths(sec["boundaries"], total)}
    res["stem_activity"] = stem_activity(stems, mix)
    res["floor_drops"] = floor_drops(stems)
    res["sweeps"] = {"mix": sweeps(mix)}
    for k in ("other", "bass"):
        if k in stems:
            res["sweeps"][k] = sweeps(mono(stems[k]))
    res["echoes"] = {k: echoes(mono(stems[k])) for k in ("other", "drums") if k in stems}
    rest = sum(mono(stems[k]) for k in ("other", "vocals") if k in stems)
    res["pumping"] = {"other": pumping(mono(stems["drums"]), rest)}
    if "bass" in stems:
        res["pumping"]["bass"] = pumping(mono(stems["drums"]), mono(stems["bass"]))
    res["tape"] = {k: tape(mono(stems[k])) for k in ("other", "bass") if k in stems}
    res["stereo_motion"] = {k: stereo_motion(v) for k, v in stems.items()}
    res["stereo_motion"]["mix"] = stereo_motion(sum(stems.values()))
    res["builds"] = builds(mix)
    save_json(out_json, res)
    return res


if __name__ == "__main__":
    run(sys.argv[1], sys.argv[2], sys.argv[3])
