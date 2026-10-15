"""Step 3 of the album card: cut stems into note events, measure each, group.

Usage: python3 -I events.py STEM_DIR OUT.json [--max-events 500]

Per stem: onsets -> events (from an onset to the next onset, or until the
sound is 35 dB down, at most 6 s). Per event we measure
  register      f0 (Hz, note name) by a harmonic sieve over spectral peaks
  attack        ms from 20 dB under the peak to the peak
  decay         dB a second after the peak (and whether the note was cut by
                the next onset before it fell 20 dB: "sustained")
  brightness    spectral centroid at the start and at the end of the note,
                and the change in octaves
  harmonicity   share of peak energy on whole multiples of f0, and how far
                (cents) the strongest peaks sit from multiples: string or
                reed ~ 0 to 10 cents; bell or metal, hundreds
  movement      vibrato depth (cents) and rate, and beating between close
                partials (Hz, which reads as detune in cents)
  level         event power against the whole mix at that moment (dB)
  position      pan, -1 left to +1 right, and left/right correlation
  clarity       how alone the event is (stem against the other stems, dB)
Then similar events are grouped (Ward clustering, number of groups chosen
by silhouette). Each group is an instrument candidate with its measured
fingerprint and the timestamps of its clearest examples.

Reliability (from calibrate.py): the measures are made for one note at a
time. Where an event is a chord or a stack of layers the f0 sieve picks one
of them, harmonicity reads low and beating reads high, so for pad and
chord stems treat harmonicity, vibrato and detune as "of the strongest
partial", not of the sound. See docs/EARS.md for the measured limits.
"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
import scipy.signal as sps
from scipy.cluster.hierarchy import linkage, fcluster
from scipy.spatial.distance import pdist, squareform

from cardlib import SR, STEMS, load_stems, mono, db, mmss, rms_env, smooth, save_json, hz_to_midi, note_name

HOP_ON = 256                      # onset detector hop (11.6 ms at 22.05 kHz)
FW, FH = 128, 32                  # fine envelope window and hop (1.45 ms hop)
MAX_LEN = 6.0
FEATS = ["f0_hz", "attack_ms", "decay_db_s", "bright_start_hz", "bright_slope_oct",
         "harm_share", "inharm_cents", "vib_cents", "beat_hz", "vs_mix_db", "pan",
         "lr_corr", "solo_db", "dur_s"]


# --- spectra and pitch --------------------------------------------------------
def peaks_of(seg, sr=SR, nfft=8192, n=14, floor_db=-45):
    """Spectral peaks of a segment: (freqs Hz, linear amps), strongest first."""
    seg = np.asarray(seg, np.float64)
    if len(seg) < 64:
        return np.array([]), np.array([])
    X = np.abs(np.fft.rfft(seg * np.hanning(len(seg)), nfft))
    L = 20 * np.log10(np.maximum(X, 1e-12))
    top = L.max()
    k = np.where((L[1:-1] > L[:-2]) & (L[1:-1] >= L[2:]) & (L[1:-1] > top + floor_db))[0] + 1
    if len(k) == 0:
        return np.array([]), np.array([])
    a, b, c = L[k - 1], L[k], L[k + 1]
    den = a - 2 * b + c
    off = np.where(den == 0, 0, 0.5 * (a - c) / np.where(den == 0, 1, den))
    f = (k + off) * sr / nfft
    amp = 10 ** ((b - 0.25 * (a - c) * off) / 20)
    o = np.argsort(-amp)[:n]
    return f[o], amp[o]


def sieve_f0(f, a, fmin=30.0, fmax=2200.0):
    """Harmonic sieve. f0 candidates are the lowest strong peak (within 15 dB of
    the strongest) and its half and third (a bass note can have a weak
    fundamental); partials count when within 2% of a whole multiple (k <= 12).
    The highest candidate that explains >= 90% of the best share of peak
    energy wins. Returns (f0, explained share), or (nan, 0).

    A free choice of candidates lets a very low f0 "explain" any set of
    partials (every partial lands near some multiple of a small enough
    spacing), so a bell reads as a string. Hence the restriction."""
    if len(f) == 0:
        return np.nan, 0.0
    strong = f[a >= a.max() * 10 ** (-15 / 20)]
    low = strong.min()
    cands = [c for c in (low, low / 2, low / 3) if fmin <= c <= fmax]
    if not cands:
        return np.nan, 0.0
    tot = (a ** 2).sum()
    scored = []
    for c in cands:
        k = np.maximum(np.round(f / c), 1)
        dev = np.abs(f / (k * c) - 1)
        m = (dev < 0.02) & (k <= 12)
        scored.append((float(c), float((a[m] ** 2).sum() / tot)))
    best = max(s for _, s in scored)
    for c, s in scored:
        if s >= 0.9 * best:
            return c, s
    return scored[0]


def harmonicity(f, a, f0):
    """(share of peak energy on whole multiples of f0, weighted mean cents off the
    nearest multiple for the peaks above the fundamental)."""
    if not np.isfinite(f0) or len(f) == 0:
        return np.nan, np.nan
    k = np.maximum(np.round(f / f0), 1)
    cents = 1200 * np.abs(np.log2(f / (k * f0)))
    share = float((a[cents < 35] ** 2).sum() / (a ** 2).sum())
    up = k >= 2
    if up.any():
        w = a[up] ** 2
        return share, float((cents[up] * w).sum() / w.sum())
    return share, 0.0


def pitch_track(seg, f0, sr=SR, hop=256, nfft=2048, pad=8):
    """Cents contour of the strongest partial near a harmonic of f0 (zero-padded STFT)."""
    if len(seg) < nfft + 4 * hop or not np.isfinite(f0):
        return None
    n = (len(seg) - nfft) // hop + 1
    win = np.hanning(nfft)
    X = np.empty((n, nfft * pad // 2 + 1))
    for i in range(n):
        X[i] = np.abs(np.fft.rfft(seg[i * hop: i * hop + nfft] * win, nfft * pad))
    freqs = np.fft.rfftfreq(nfft * pad, 1 / sr)
    mean = X.mean(axis=0)
    # strongest harmonic of f0 (1..6) by mean magnitude
    best, kbest = -1, 1
    for k in range(1, 7):
        fk = f0 * k
        if fk > sr / 2 - 200:
            break
        i = np.argmin(np.abs(freqs - fk))
        w = int(0.03 * fk / (freqs[1]))
        v = mean[max(0, i - w): i + w + 1].max()
        if v > best:
            best, kbest = v, k
    fk = f0 * kbest
    lo, hi = np.searchsorted(freqs, fk * 0.97), np.searchsorted(freqs, fk * 1.03)
    cents = []
    for i in range(n):
        j = lo + int(np.argmax(X[i, lo:hi]))
        a, b, c = np.log(X[i, j - 1] + 1e-12), np.log(X[i, j] + 1e-12), np.log(X[i, j + 1] + 1e-12)
        den = a - 2 * b + c
        off = float(np.clip(0.5 * (a - c) / den, -0.5, 0.5)) if den != 0 else 0
        cents.append(1200 * np.log2(freqs[1] * (j + off) / fk))
    return np.array(cents), hop / sr


def vibrato(contour, hop_s):
    """(peak depth in cents, rate in Hz) of the 3-9 Hz movement of a cents contour."""
    if contour is None or len(contour) < 24:
        return 0.0, 0.0
    x = np.arange(len(contour))
    r = contour - np.polyval(np.polyfit(x, contour, 1), x)
    spec = np.abs(np.fft.rfft(r * np.hanning(len(r)), 1024))
    fr = np.fft.rfftfreq(1024, hop_s)
    band = (fr >= 3.0) & (fr <= 9.0)
    if not band.any():
        return 0.0, 0.0
    i = np.argmax(np.where(band, spec, 0))
    depth = 2 * spec[i] / (np.hanning(len(r)).sum())          # sine amplitude from windowed FFT
    # the 2048-sample analysis window smooths fast vibrato: undo its response
    x = fr[i] * 2048 / SR
    x = 1 + 1e-6 if abs(abs(x) - 1) < 1e-6 else x
    resp = max(abs(np.sinc(x) / (1 - x ** 2)), 0.3)
    return float(depth / resp), float(fr[i])


def beating(seg, fc, sr=SR):
    """AM (beating) of the band around fc: (rate Hz, depth 0-1). Needs >= 1.5 s."""
    if len(seg) < 1.5 * sr or not np.isfinite(fc) or fc > sr / 2 - 300:
        return 0.0, 0.0
    sos = sps.butter(4, [fc * 0.94, fc * 1.06], "bandpass", fs=sr, output="sos")
    y = sps.sosfiltfilt(sos, seg)
    env = np.abs(sps.hilbert(y))
    env = env[int(0.05 * sr): -int(0.05 * sr)]
    # decimate the envelope to ~200 Hz
    env = sps.resample_poly(env, 1, int(sr / 200))
    m = env.mean()
    if m <= 1e-9:
        return 0.0, 0.0
    r = env / m - 1
    spec = np.abs(np.fft.rfft(r * np.hanning(len(r)), 8192))
    fr = np.fft.rfftfreq(8192, 1 / 200)
    band = (fr >= 0.3) & (fr <= 12)
    i = np.argmax(np.where(band, spec, 0))
    depth = 2 * spec[i] / np.hanning(len(r)).sum()
    return float(fr[i]), float(depth)


# --- events --------------------------------------------------------------------
def detect_onsets(y, sr=SR, delta=0.08):
    import librosa
    env = librosa.onset.onset_strength(y=y, sr=sr, hop_length=HOP_ON)
    fr = librosa.onset.onset_detect(onset_envelope=env, sr=sr, hop_length=HOP_ON, backtrack=True,
                                    pre_max=3, post_max=3, pre_avg=12, post_avg=12, delta=delta, wait=4,
                                    units="frames")
    return np.asarray(fr) * HOP_ON / sr, env


def measure_event(st, mixp, others_p, t0, t1, sr=SR, want_pitch=True):
    """One event: all measures. st: (2, n) stem; mixp / others_p: power arrays per sample."""
    i0, i1 = int(t0 * sr), int(t1 * sr)
    seg2 = st[:, i0:i1]
    y = seg2.mean(axis=0)
    if len(y) < 256:
        return None
    env = rms_env(y, FW, FH)
    p = int(np.argmax(env))
    pk = env[p]
    if pk < -75:
        return None
    # attack: last frame before the peak that is 20 dB under it -> the peak
    pre = np.where(env[:p + 1] < pk - 20)[0]
    a_start = pre[-1] if len(pre) else 0
    attack_ms = max(1.0, (p - a_start) * FH / sr * 1000)
    # decay: straight-line fit from the peak down to 25 dB under it
    post = env[p:]
    stop = np.where(post < pk - 25)[0]
    e = stop[0] if len(stop) else len(post)
    sustained = len(stop) == 0 or (post[:e] > pk - 20).all() and len(stop) == 0
    reached20 = bool((post < pk - 20).any())
    if e >= 8:
        tt = np.arange(e) * FH / sr
        decay = float(np.polyfit(tt, post[:e], 1)[0])
    else:
        decay = np.nan
    row = {"t": float(t0), "dur_s": float(t1 - t0), "attack_ms": float(attack_ms),
           "decay_db_s": decay, "reached_20db_down": reached20}
    # level and clarity
    mp = mixp[i0:i1].mean()
    op = others_p[i0:i1].mean()
    sp = (y ** 2).mean()
    row["vs_mix_db"] = float(10 * np.log10(max(sp, 1e-12) / max(mp, 1e-12)))
    row["solo_db"] = float(10 * np.log10(max(sp, 1e-12) / max(op, 1e-12)))
    l, r = np.sqrt((seg2[0] ** 2).mean()), np.sqrt((seg2[1] ** 2).mean())
    row["pan"] = float((r - l) / max(r + l, 1e-12))
    row["lr_corr"] = float(np.corrcoef(seg2[0], seg2[1])[0, 1]) if l > 1e-6 and r > 1e-6 else 1.0
    # steady part for spectra: 40 ms after the peak onward, up to 0.6 s
    ps = i0 + p * FH
    s0 = ps + int(0.04 * sr)
    steady = st.mean(axis=0)[s0: min(i1, s0 + int(0.6 * sr))]
    if len(steady) < 0.08 * sr:
        steady = y
    f, a = peaks_of(steady, sr)
    f0, expl = sieve_f0(f, a)
    row["f0_hz"] = float(f0)
    row["f0_note"] = note_name(hz_to_midi(f0)) if np.isfinite(f0) else None
    share, cents = harmonicity(f, a, f0)
    row["harm_share"], row["inharm_cents"] = share, cents
    if np.isfinite(f0) and share >= 0.7 and len(a) >= 3:
        k = np.maximum(np.round(f / f0), 1).astype(int)
        amp = {int(kk): float(aa) for kk, aa in zip(k, a)}
        a1 = amp.get(1, np.nan)
        row["h2_db"] = float(20 * np.log10(amp[2] / a1)) if 2 in amp and a1 > 0 else None
        odd = [amp[j] for j in (3, 5, 7) if j in amp]
        even = [amp[j] for j in (2, 4, 6) if j in amp]
        row["odd_even_db"] = float(20 * np.log10(np.mean(odd) / np.mean(even))) if odd and even else None
    # brightness over the note: centroid in 2048-sample frames
    ys = y[max(0, ps - i0 - 256):]
    if len(ys) >= 2048 + 512:
        nfr = (len(ys) - 2048) // 512 + 1
        win = np.hanning(2048)
        fr = np.fft.rfftfreq(2048, 1 / sr)
        cen = np.empty(nfr)
        lev = np.empty(nfr)
        for i in range(nfr):
            sp_ = np.abs(np.fft.rfft(ys[i * 512: i * 512 + 2048] * win)) ** 2
            band = (fr > 60) & (fr < 9000)
            cen[i] = (fr[band] * sp_[band]).sum() / max(sp_[band].sum(), 1e-12)
            lev[i] = 10 * np.log10(sp_.sum() + 1e-12)
        live = lev > lev.max() - 30
        idx = np.where(live)[0]
        if len(idx) >= 2:
            first = cen[idx[: max(1, min(2, len(idx) // 2))]]
            last = cen[idx[-max(1, min(2, len(idx) // 2)):]]
            row["bright_start_hz"] = float(np.median(first))
            row["bright_end_hz"] = float(np.median(last))
            row["bright_slope_oct"] = float(np.log2(max(row["bright_end_hz"], 1) / max(row["bright_start_hz"], 1)))
    if "bright_start_hz" not in row:
        fr = np.fft.rfftfreq(len(steady), 1 / sr)
        sp_ = np.abs(np.fft.rfft(steady)) ** 2
        band = (fr > 60) & (fr < 9000)
        row["bright_start_hz"] = float((fr[band] * sp_[band]).sum() / max(sp_[band].sum(), 1e-12))
        row["bright_slope_oct"] = 0.0
    # movement (only for notes that last)
    row["vib_cents"], row["vib_hz"], row["beat_hz"], row["beat_depth"] = 0.0, 0.0, 0.0, 0.0
    if want_pitch and np.isfinite(f0) and (t1 - t0) >= 0.6:
        seg = st.mean(axis=0)[ps + int(0.05 * sr): i1]
        tr = pitch_track(seg, f0)
        if tr is not None:
            row["vib_cents"], row["vib_hz"] = vibrato(*tr)
        if (t1 - t0) >= 1.8 and share >= 0.5:
            fk = f[0] if len(f) else f0
            row["beat_hz"], row["beat_depth"] = beating(seg, fk)
    row["detune_cents"] = (1200 * np.log2(1 + row["beat_hz"] / max(f[0] if len(f) else f0, 1))
                           if row["beat_depth"] >= 0.1 and row["beat_hz"] > 0 else 0.0)
    return row


def events_for_stem(name, st, mixp, others_p, max_events=500, sr=SR):
    y = st.mean(axis=0)
    if np.sqrt((y ** 2).mean()) < 1e-4:
        return []
    on, _ = detect_onsets(y, sr)
    if len(on) == 0:
        return []
    fine = rms_env(y, FW, FH)
    # a second onset within 0.25 s with no dip of 3 dB in between is the same
    # note (the kink at the end of a slow attack), not a new one
    keep = [0]
    for i in range(1, len(on)):
        j = keep[-1]
        if on[i] - on[j] < 0.25:
            a_, b_ = int(on[j] * sr / FH), int(on[i] * sr / FH)
            seg = fine[a_:b_ + 1]
            if len(seg) >= 2 and (np.maximum.accumulate(seg) - seg).max() < 3.0:
                continue
        keep.append(i)
    on = on[keep]
    env = rms_env(y, 1024, 512)        # 23 ms hop for the ends
    ends = []
    for i, t in enumerate(on):
        nxt = on[i + 1] if i + 1 < len(on) else len(y) / sr
        lim = min(nxt, t + MAX_LEN)
        f0_ = int(t * sr / 512)
        f1_ = int(lim * sr / 512)
        seg = env[f0_:f1_]
        if len(seg) > 3:
            p = int(np.argmax(seg))
            d = np.where(seg[p:] < seg[p] - 35)[0]
            if len(d):
                lim = min(lim, t + (p + d[0]) * 512 / sr + 0.05)
        ends.append(lim)
    # start each event at the quietest point shortly before its onset, so slow
    # attacks are measured from their real start (not from where the flux peaks)
    starts = []
    for i, t in enumerate(on):
        lo = max(on[i - 1] + 0.03 if i else 0.0, t - 0.8)
        a_, b_ = int(lo * sr / FH), int(min(t + 0.02, ends[i] - 0.05) * sr / FH)
        if b_ - a_ >= 2:
            seg = fine[a_:b_ + 1]
            starts.append(lo + (a_ - int(lo * sr / FH)) * FH / sr + float(np.argmin(seg)) * FH / sr)
        else:
            starts.append(float(t))
    on = np.array(starts)
    rows = []
    # keep the loudest events if there are too many
    lev = np.array([10 * np.log10((y[int(t * sr): int(e * sr)] ** 2).mean() + 1e-12) if e > t else -120
                    for t, e in zip(on, ends)])
    order = np.argsort(-lev)[:max_events]
    for i in sorted(order):
        if ends[i] - on[i] < 0.05:
            continue
        r = measure_event(st, mixp, others_p, on[i], ends[i], sr, want_pitch=(name != "drums"))
        if r is None:
            continue
        r["stem"] = name
        r["gap_next_s"] = float((on[i + 1] - on[i]) if i + 1 < len(on) else 1e9)
        rows.append(r)
    return rows


# --- grouping --------------------------------------------------------------------
def _matrix(rows, drums):
    cols = []
    def col(key, fn=lambda v: v, default=np.nan):
        return np.array([fn(r.get(key)) if r.get(key) is not None and np.isfinite(r.get(key, np.nan)) else default
                         for r in rows], float)
    if drums:
        cols = [(col("bright_start_hz", np.log2), 1.0), (col("attack_ms", np.log2), 0.6),
                (col("decay_db_s", lambda v: np.clip(v, -120, 0) / 30), 0.8), (col("pan"), 0.4),
                (col("dur_s", np.log2), 0.5)]
    else:
        cols = [(col("f0_hz", np.log2), 1.0), (col("attack_ms", np.log2), 0.7),
                (col("decay_db_s", lambda v: np.clip(v, -60, 0) / 15), 0.7),
                (col("bright_start_hz", np.log2), 0.9), (col("bright_slope_oct"), 0.4),
                (col("harm_share"), 0.6), (col("inharm_cents", lambda v: np.log2(1 + v / 10)), 0.5),
                (col("pan"), 0.4)]
    X = np.vstack([c for c, _ in cols]).T
    w = np.array([w for _, w in cols])
    med = np.nanmedian(X, axis=0)
    X = np.where(np.isnan(X), med, X)
    sd = np.maximum(X.std(axis=0), 1e-6)
    return ((X - np.median(X, axis=0)) / sd) * w


def silhouette(D, lab):
    n = len(lab)
    s = np.zeros(n)
    for i in range(n):
        same = lab == lab[i]
        if same.sum() < 2:
            continue
        a = D[i, same].sum() / (same.sum() - 1)
        b = min(D[i, lab == k].mean() for k in set(lab) if k != lab[i])
        s[i] = (b - a) / max(a, b, 1e-9)
    return float(s.mean())


def group_events(rows, drums=False, min_size=4, kmax=8):
    """Cluster events; returns a label per row (0..k-1) and the silhouette used."""
    n = len(rows)
    if n < 2 * min_size:
        return np.zeros(n, int), 0.0
    X = _matrix(rows, drums)
    Z = linkage(X, "ward")
    D = squareform(pdist(X))
    best = (0.0, 1, np.zeros(n, int))
    for k in range(2, min(kmax, n // min_size) + 1):
        lab = fcluster(Z, k, "maxclust") - 1
        if np.bincount(lab).min() < min_size:
            continue
        s = silhouette(D, lab)
        if s > best[0]:
            best = (s, k, lab)
    if best[0] < 0.18:
        return np.zeros(n, int), best[0]
    return best[2], best[0]


def _q(vals, q):
    v = np.array([x for x in vals if x is not None and np.isfinite(x)], float)
    return float(np.percentile(v, q)) if len(v) else None


def summarize_groups(rows, labels, name):
    groups = []
    for g in sorted(set(labels)):
        rs = [r for r, l in zip(rows, labels) if l == g]
        fing = {}
        for k in FEATS + ["h2_db", "odd_even_db", "bright_slope_oct", "bright_end_hz", "detune_cents", "vib_hz"]:
            vals = [r.get(k) for r in rs]
            fing[k] = {"p10": _q(vals, 10), "med": _q(vals, 50), "p90": _q(vals, 90)}
        med = fing["f0_hz"]["med"]
        # clearest examples: alone in the mix, long enough, close to the group median
        def score(r):
            z = 0.0
            if med and r.get("f0_hz") and np.isfinite(r["f0_hz"]):
                z = abs(np.log2(r["f0_hz"] / med))
            gap = min(r.get("gap_next_s", 0), 1.5)
            return r["solo_db"] + 4 * gap - 3 * z
        ex = sorted(rs, key=score, reverse=True)[:3]
        power = float(sum(10 ** (r["vs_mix_db"] / 10) * r["dur_s"] for r in rs))
        groups.append({"stem": name, "n": len(rs), "weight": power, "fingerprint": fing,
                       "examples": [{"t": e["t"], "at": mmss(e["t"]), "dur_s": round(e["dur_s"], 2),
                                     "note": e.get("f0_note"), "solo_db": round(e["solo_db"], 1)} for e in ex]})
    groups.sort(key=lambda g: -g["weight"])
    for i, g in enumerate(groups):
        g["id"] = f"{name}-{i + 1}"
    return groups


def describe(g):
    """Plain description of a group from its numbers (no judgment)."""
    f = g["fingerprint"]
    def m(k):
        return f[k]["med"]
    bits = []
    if g["stem"] == "drums":
        c = m("bright_start_hz")
        bits.append("low thump" if c and c < 400 else "mid snap" if c and c < 2500 else "high tick")
    else:
        h, ic = m("harm_share"), m("inharm_cents")
        if m("f0_hz"):
            bits.append(f"around {note_name(hz_to_midi(m('f0_hz')))}")
        if ic is not None and ic > 60:
            bits.append("bell- or metal-like (partials off whole multiples)")
        elif h is not None and h > 0.85:
            bits.append("harmonic (string/reed/pad-like)")
        elif h is not None:
            bits.append("mixed or noisy partials")
        oe = m("odd_even_db")
        if oe is not None and oe > 6:
            bits.append("odd harmonics stronger (square/clarinet-like)")
    a = m("attack_ms")
    bits.append("click attack" if a < 8 else "quick attack" if a < 40 else "soft attack" if a < 150 else "slow swell")
    d = m("decay_db_s")
    if d is not None:
        bits.append(f"falls {abs(d):.0f} dB/s")
    s = m("bright_slope_oct")
    if s is not None and abs(s) >= 0.3:
        bits.append("gets darker over the note" if s < 0 else "gets brighter over the note")
    if (m("vib_cents") or 0) > 8:
        bits.append(f"vibrato about {m('vib_cents'):.0f} cents at {m('vib_hz'):.1f} Hz")
    if (m("detune_cents") or 0) > 3:
        bits.append(f"beats like a detuned pair (about {m('detune_cents'):.0f} cents)")
    p = m("pan")
    if p is not None:
        bits.append("centered" if abs(p) < 0.15 else f"leans {'right' if p > 0 else 'left'} ({p:+.2f})")
    return ", ".join(bits)


def run(stem_dir, out_json, max_events=500):
    stems = load_stems(stem_dir)
    total = sum(s ** 2 for s in stems.values()).mean(axis=0)
    result = {"stem_dir": stem_dir, "stems": {}}
    for name, st in stems.items():
        own = (st ** 2).mean(axis=0)
        others = np.maximum(total - own, 1e-12)
        rows = events_for_stem(name, st, total, others, max_events)
        if len(rows) < 8:
            result["stems"][name] = {"events": len(rows), "groups": [], "note": "too few events"}
            continue
        lab, sil = group_events(rows, drums=(name == "drums"))
        groups = summarize_groups(rows, lab, name)
        for g in groups:
            g["description"] = describe(g)
        result["stems"][name] = {"events": len(rows), "silhouette": round(sil, 3), "groups": groups,
                                 "event_rows": rows}
        print(name, len(rows), "events,", len(groups), "groups, silhouette", round(sil, 2), flush=True)
    save_json(out_json, result)
    return result


if __name__ == "__main__":
    a = sys.argv[1:]
    me = 500
    if "--max-events" in a:
        i = a.index("--max-events"); me = int(a[i + 1]); del a[i:i + 2]
    run(a[0], a[1], me)
