"""Step 5 of the album card: read the notes.

Usage: python3 -I theory.py STEM_DIR MIX_FILE OUT.json

  meter       tempo, beat grid, beats per bar, swing, triplet share, cross-rhythm
  key         tonic, mode (diatonic modes, pentatonic) from pitch classes, with margin
  chords      a Viterbi-smoothed chord sequence (10 chord types x 12 roots),
              chord changes a minute, how long chords last, vocabulary
  bass        pitch notes, pedal / ostinato / walking / slides
  lead        top-line notes, interval sizes and phrase contours (reliable only
              when the line sits clearly above its accompaniment)
  form        bar-synchronous repetition: phrase and section lengths in bars

Each has a calibration test in cal_theory.py; docs/EARS.md quotes where each
stops being reliable. The chord counter replaces analyze.py's 2-second
counter, which undercounts fast changes (it read 29 true changes a minute
as 17).
"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
import scipy.signal as sps

from cardlib import (SR, load_stems, mono, rms_env, smooth, runs, save_json, mmss, NOTE_NAMES,
                     hz_to_midi, midi_to_hz, note_name)

HOP = 256                          # onset-strength hop (11.6 ms)


# --- meter ---------------------------------------------------------------------------
def onset_env(y, sr=SR, hop=HOP):
    import librosa
    return librosa.onset.onset_strength(y=y, sr=sr, hop_length=hop)


def tempo_of(env, sr=SR, hop=HOP, lo=50.0, hi=190.0):
    """Beat period (s) from the autocorrelation of the onset strength, with a mild
    prior toward 70-140 BPM to settle octave doubts. Returns (period_s, strength)."""
    x = env - env.mean()
    ac = np.correlate(x, x, "full")[len(x) - 1:]
    ac = ac / max(ac[0], 1e-12)
    fps = sr / hop
    lag_lo, lag_hi = int(60 / hi * fps), int(60 / lo * fps)
    seg = ac[lag_lo:lag_hi + 1]
    if len(seg) < 3:
        return None, 0.0
    # reward a lag whose multiples are also strong; prior favors ~100 BPM
    lags = np.arange(lag_lo, lag_hi + 1)
    score = np.zeros(len(lags))
    for i, l in enumerate(lags):
        s = ac[l]
        for m in (2, 3, 4):
            if m * l < len(ac):
                s += 0.5 * ac[m * l]
        bpm = 60 * fps / l
        s *= np.exp(-0.5 * (np.log2(bpm / 100) / 0.9) ** 2)
        score[i] = s
    pk, _ = sps.find_peaks(score)
    if len(pk) == 0:
        return None, 0.0
    i = pk[np.argmax(score[pk])]
    l = lags[i]
    # refine to a fraction of a frame by parabolic interpolation of the raw autocorrelation
    a, b, c = ac[l - 1], ac[l], ac[l + 1]
    den = a - 2 * b + c
    off = 0.5 * (a - c) / den if den != 0 else 0
    return (l + off) / fps, float(ac[l])


def beat_phase(env, period, sr=SR, hop=HOP):
    fps = sr / hop
    n = len(env)
    best, bp = -1, 0.0
    for ph in np.arange(0, period, 0.005):
        t = np.arange(ph, n / fps, period)
        v = np.interp(t * fps, np.arange(n), env).sum()
        if v > best:
            best, bp = v, ph
    return bp


def refine_beats(env, period, phase, sr=SR, hop=HOP):
    """Nudge period (+-0.8%) and phase together so the beat grid sits on the onset peaks
    over the whole track (removes the drift a frame-rounded period leaves)."""
    fps = sr / hop
    n = len(env)
    best = (-1.0, period, phase)
    for dp in np.linspace(-0.008, 0.008, 33):
        p = period * (1 + dp)
        for ph in np.arange(max(0, phase - 0.03), phase + 0.03, 0.004):
            t = np.arange(ph, n / fps, p)
            v = np.interp(t * fps, np.arange(n), env).sum() / len(t)
            if v > best[0]:
                best = (v, p, ph)
    return best[1], best[2]


def beat_strengths(y, period, phase, sr=SR, hop=HOP):
    """Linear loudness jump at each beat: the loudest 20-ms frame in [-30, +80] ms
    minus the quietest in [-120, 0] ms. (The onset envelope is logarithmic and squeezes
    accents, so the bar is read from this instead.)"""
    import librosa
    r = librosa.feature.rms(y=y.astype(np.float32), frame_length=512, hop_length=hop)[0]
    fps = sr / hop
    t = np.arange(phase, len(y) / sr - period, period)
    out = []
    for x in t:
        i = int(x * fps)
        hi = r[max(0, i - int(0.03 * fps)): i + int(0.08 * fps) + 1].max()
        lo = r[max(0, i - int(0.12 * fps)): i + 1].min()
        out.append(hi - lo)
    return np.array(out)


def beats_per_bar(b, cands=(2, 3, 4, 5, 6, 7, 8, 9, 12)):
    """Pick the bar length (in beats) from the autocorrelation of beat-level strengths b."""
    if len(b) < 24:
        return None, 0.0, {}
    x = b - b.mean()
    ac = np.correlate(x, x, "full")[len(x) - 1:] / max((x ** 2).sum(), 1e-12)
    L = min(len(ac) - 1, 33)
    # normalize each lag by the number of terms to remove the taper
    ac = ac[:L + 1] * len(x) / (len(x) - np.arange(L + 1))
    sc = {}
    for m in cands:
        if m + 1 > L:
            continue
        nb = 0.5 * (ac[m - 1] + ac[m + 1])
        sc[m] = float(ac[m] - nb)
    if not sc:
        return None, 0.0, {}
    best = max(sc, key=sc.get)
    # a multiple of a shorter strong candidate is the shorter one (8 -> 4)
    for m in sorted(sc):
        if m < best and best % m == 0 and sc[m] >= 0.7 * sc[best]:
            best = m
            break
    return int(best), float(sc[best]), {int(k): round(v, 3) for k, v in sc.items()}


def _circ_peaks(ph, w, bins=60):
    h, _ = np.histogram(ph, bins=bins, range=(0, 1), weights=w)
    k = np.array([1, 2, 3, 2, 1], float); k /= k.sum()
    hs = np.convolve(np.tile(h, 3), k, mode="same")[bins:2 * bins]
    pk, _ = sps.find_peaks(np.tile(hs, 3), height=0.0)
    out = []
    for p in pk:
        if bins <= p < 2 * bins:
            c = (p - bins + 0.5) / bins
            near = (np.abs(((ph - c + 0.5) % 1.0) - 0.5) < 0.06)
            if w[near].sum() > 0:
                # circular weighted mean around the peak
                off = (((ph[near] - c + 0.5) % 1.0) - 0.5)
                c = (c + (off * w[near]).sum() / w[near].sum()) % 1.0
            out.append((float(hs[p - bins]), float(c)))
    return sorted(out, reverse=True)


def swing_phases(on_t, on_w, period, phase):
    """Where the off-beat onsets sit between two beats (0.5 straight eighths, 0.667
    triplet swing, 0.75 dotted), read from the two strongest clusters of onset phase
    so it does not depend on which one is called the beat; and the same inside a half
    beat for sixteenths. Reports the later of the two possible readings (d, 1-d),
    which is the usual long-short swing."""
    ph = ((on_t - phase) / period) % 1.0
    w = on_w
    out = {}

    def pair(ph_, w_):
        pk = _circ_peaks(ph_, w_)
        if not pk:
            return None
        A = pk[0]
        for hB, cB in pk[1:]:
            d = ((cB - A[1]) % 1.0)
            circ = min(d, 1 - d)
            if circ >= 0.2 and hB >= 0.25 * A[0]:
                return max(d, 1 - d), circ, hB / A[0]
        return None

    if len(ph) >= 16:
        r = pair(ph, w)
        if r:
            out["eighth_off_phase"] = float(r[0])
            out["eighth_off_cluster_ratio"] = float(r[2])
        r2 = pair((ph * 2) % 1.0, w)
        if r2:
            out["sixteenth_off_phase"] = float(r2[0])
    pk0 = _circ_peaks(ph, w)
    pa = ((ph - pk0[0][1]) % 1.0) if pk0 else ph          # phase counted from the strongest cluster
    tri = (np.abs(pa - 1 / 3) < 0.04) | (np.abs(pa - 2 / 3) < 0.04)
    dup = (np.abs(pa - 0.25) < 0.04) | (np.abs(pa - 0.75) < 0.04)
    s_ = w[tri].sum() + w[dup].sum()
    out["triplet_share"] = float(w[tri].sum() / s_) if s_ > 0 else None
    return out


def cross_rhythm(env, period, sr=SR, hop=HOP, swing=None):
    """Strong onset periodicities in beats, and any pair in a non-integer ratio
    (3:2, 4:3, 5:4, 5:3, 7:4 ...) as evidence of cross-rhythm."""
    fps = sr / hop
    x = env - env.mean()
    ac = np.correlate(x, x, "full")[len(x) - 1:]
    ac = ac / max(ac[0], 1e-12)
    lo, hi = int(0.3 * period * fps), int(8 * period * fps)
    seg = ac[lo:hi]
    pk, pr = sps.find_peaks(seg, prominence=0.04)
    allp = sorted([((lo + p) / fps / period, float(seg[p])) for p in pk], key=lambda r: r[0])
    pers = sorted(allp, key=lambda r: -r[1])[:8]
    cand = [r for r in allp if r[0] < 4.05 and r[1] >= 0.05]
    ratios = [(1.5, "3:2"), (4 / 3, "4:3"), (1.25, "5:4"), (5 / 3, "5:3"), (7 / 4, "7:4"), (7 / 5, "7:5"), (2.5, "5:2")]
    found = []
    for i in range(len(cand)):
        for j in range(len(cand)):
            if cand[j][0] > cand[i][0]:
                r = cand[j][0] / cand[i][0]
                def gdist(p):
                    # distance to the swing grid: whole and half beats, plus the swung gaps s and 1-s
                    f = p % 1.0
                    pts = [0.0, 0.5, 1.0] + ([swing, 1 - swing, 1 / 3, 2 / 3] if swing else [])
                    return min(abs(f - c) for c in pts)
                off_grid = max(gdist(cand[i][0]), gdist(cand[j][0])) > 0.07
                for v, name in ratios:
                    if abs(r - v) < 0.03 * v and off_grid:
                        found.append({"periods_beats": [round(cand[i][0], 2), round(cand[j][0], 2)], "ratio": name})
    return {"periods_beats": [{"beats": round(p, 2), "strength": round(s, 2)} for p, s in pers[:8]], "ratios_found": found[:4]}


def meter(drums, mix, sr=SR):
    """Tempo, beats per bar, swing, triplets, cross-rhythm. Uses the drums stem when it is clearly active."""
    y = drums if np.sqrt((drums ** 2).mean()) > 0.2 * np.sqrt((mix ** 2).mean()) else mix
    env = onset_env(y, sr)
    period, strength = tempo_of(env, sr)
    res = {"source": "drums" if y is drums else "mix"}
    if period is None:
        res["note"] = "no tempo found"
        return res
    phase = beat_phase(env, period, sr)
    period, phase = refine_beats(env, period, phase, sr)
    res.update({"bpm": 60 / period, "period_s": period, "pulse_strength": strength, "phase_s": phase,
                "bpm_if_half_or_double": [30 / period, 120 / period]})
    bpb, conf, scores = beats_per_bar(beat_strengths(y, period, phase, sr))
    res.update({"beats_per_bar": bpb, "bar_score": conf, "bar_scores": scores})
    import librosa
    on_f = librosa.onset.onset_detect(onset_envelope=env, sr=sr, hop_length=HOP, units="frames", backtrack=False)
    on_t = on_f * HOP / sr
    on_w = env[on_f]
    res.update(swing_phases(on_t, on_w, period, phase))
    # a shuffle is a triplet grid, not a cross-rhythm: its third-beat periods are on the grid
    res["cross_rhythm"] = cross_rhythm(env, period, sr, swing=res.get("eighth_off_phase"))
    return res


# --- key and mode ------------------------------------------------------------------------
MAJOR = np.array([1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1], float)
MODES = ["ionian (major)", "dorian", "phrygian", "lydian", "mixolydian", "aeolian (minor)", "locrian"]


def chroma_of(y, sr=SR, hop=2048, fmin=65.41, octaves=6, bins_per_oct=36):
    import librosa
    C = np.abs(librosa.cqt(y.astype(np.float32), sr=sr, hop_length=hop, fmin=fmin, n_bins=octaves * bins_per_oct,
                           bins_per_octave=bins_per_oct, tuning=0.0))
    # fold 36 bins/octave into 12 pitch classes, summing the 3 bins around each semitone
    n = C.shape[1]
    ch = np.zeros((12, n))
    for b in range(C.shape[0]):
        semi = int(round(b / (bins_per_oct / 12))) % 12
        pc = (semi + int(round(12 * np.log2(fmin / 16.3516)))) % 12          # C0 = 16.35 Hz
        ch[pc] += C[b] ** 2
    return np.sqrt(ch), hop / sr


def tuning_offset(y, sr=SR):
    import librosa
    return float(librosa.estimate_tuning(y=y.astype(np.float32), sr=sr))


def key_mode(chroma, bass_pc=None):
    """Tonic and mode from the pitch-class weights (and the bass, if given)."""
    w = chroma.sum(axis=1)
    w = w / max(w.sum(), 1e-12)
    # 1) the diatonic collection (key signature) that holds the most weight
    cov = np.array([sum(w[(k + d) % 12] for d in np.where(MAJOR)[0]) for k in range(12)])
    k = int(np.argmax(cov))
    order = np.argsort(-cov)
    # 2) tonic within it: weight of the tonic and its fifth, plus the bass
    scale = [(k + d) % 12 for d in np.where(MAJOR)[0]]
    sc = {}
    for t in scale:
        s = w[t] + 0.5 * w[(t + 7) % 12] + 0.3 * w[(t + 4) % 12 if (t + 4) % 12 in scale else (t + 3) % 12]
        if bass_pc is not None:
            s += 1.2 * bass_pc[t]
        sc[t] = s
    rank = sorted(sc, key=sc.get, reverse=True)
    tonic = rank[0]
    deg = scale.index(tonic)
    mode = MODES[deg]
    # pentatonic: do 5 of the 7 notes hold >= 95% of the weight?
    in_scale = sorted(((w[p], p) for p in scale), reverse=True)
    top5 = sum(v for v, _ in in_scale[:5])
    pent = top5 >= 0.88 * sum(v for v, _ in in_scale)
    res = {"tonic": NOTE_NAMES[tonic], "mode": mode, "collection_of": NOTE_NAMES[k] + " major notes",
           "collection_coverage": float(cov[k]), "collection_margin": float(cov[k] - cov[order[1]]),
           "tonic_margin": float((sc[rank[0]] - sc[rank[1]]) / max(sc[rank[0]], 1e-9)),
           "pitch_class_weights": {NOTE_NAMES[i]: round(float(w[i]), 3) for i in range(12)},
           "pentatonic_fits": bool(pent)}
    if pent:
        missing = [NOTE_NAMES[p] for v, p in in_scale[5:]]
        res["pentatonic_missing"] = missing
        have = {p for v, p in in_scale[:5]}
        res["pentatonic_reading"] = NOTE_NAMES[tonic] + (" major pentatonic" if (tonic + 4) % 12 in have else
                                                         " minor pentatonic" if (tonic + 3) % 12 in have else " pentatonic")
    return res


# --- chords -----------------------------------------------------------------------------------
CHORD_TYPES = {"maj": [0, 4, 7], "min": [0, 3, 7], "7": [0, 4, 7, 10], "maj7": [0, 4, 7, 11], "min7": [0, 3, 7, 10],
               "sus2": [0, 2, 7], "sus4": [0, 5, 7], "dim": [0, 3, 6], "aug": [0, 4, 8], "5": [0, 7]}


def chord_templates():
    names, T = [], []
    for ty, iv in CHORD_TYPES.items():
        for r in range(12):
            v = np.zeros(12)
            for i, d in enumerate(iv):
                v[(r + d) % 12] = 1.0 if i != 3 else 0.8
            names.append((NOTE_NAMES[r] + ("" if ty == "maj" else ty)))
            T.append(v / np.linalg.norm(v))
    return names, np.array(T)


def chords(y, sr=SR, bass=None, change_cost=0.6, hop=2048):
    """Viterbi chord path. change_cost is in cosine-similarity units: a change is
    taken only when the new chord fits better by that much over the frames it covers."""
    ch, hop_s = chroma_of(y, sr, hop)
    ch = ch / np.maximum(np.linalg.norm(ch, axis=0, keepdims=True), 1e-9)
    names, T = chord_templates()
    sim = T @ ch                                              # (templates, frames)
    quiet = np.sqrt((ch ** 2).sum(axis=0)) < 1e-9
    n = sim.shape[1]
    K = len(names)
    # Viterbi: stay free, change costs change_cost
    dp = np.zeros((K, n))
    bp = np.zeros((K, n), int)
    dp[:, 0] = sim[:, 0]
    for t in range(1, n):
        prev = dp[:, t - 1]
        stay = prev
        best_prev = int(np.argmax(prev))
        change = prev[best_prev] - change_cost
        use_change = change > stay
        dp[:, t] = np.where(use_change, change, stay) + sim[:, t]
        bp[:, t] = np.where(use_change, best_prev, np.arange(K))
    path = np.zeros(n, int)
    path[-1] = int(np.argmax(dp[:, -1]))
    for t in range(n - 1, 0, -1):
        path[t - 1] = bp[path[t], t]
    segs = []
    s = 0
    for t in range(1, n + 1):
        if t == n or path[t] != path[s]:
            segs.append({"t0": s * hop_s, "t1": t * hop_s, "chord": names[path[s]],
                         "fit": float(sim[path[s], s:t].mean())})
            s = t
    dur = n * hop_s
    res = {"segments": segs, "changes": max(0, len(segs) - 1), "changes_per_min": max(0, len(segs) - 1) / (dur / 60)}
    d = np.array([g["t1"] - g["t0"] for g in segs])
    res["settled_chords_per_min"] = float((d >= 1.5).sum() / (dur / 60))      # chords that last 1.5 s or more
    if len(d):
        res["chord_len_s"] = {"p10": float(np.percentile(d, 10)), "median": float(np.median(d)), "p90": float(np.percentile(d, 90))}
    share = {}
    for g in segs:
        share[g["chord"]] = share.get(g["chord"], 0) + g["t1"] - g["t0"]
    top = sorted(share.items(), key=lambda kv: -kv[1])[:8]
    res["vocabulary"] = [{"chord": c, "share": round(v / dur, 3)} for c, v in top]
    res["distinct"] = len(share)
    return res


def chord_changes_old(y, sr=SR):
    """analyze.py's counter (2-s averaged chroma, similarity < 0.85), kept for comparison."""
    import librosa
    yh = librosa.effects.harmonic(y.astype(np.float32), margin=2.0)
    chroma = librosa.feature.chroma_cqt(y=yh, sr=sr, hop_length=2048)
    hop_c = 2048 / sr
    k2 = max(1, int(2 / hop_c))
    nseg = chroma.shape[1] // k2
    seg = chroma[:, : nseg * k2].reshape(12, nseg, k2).mean(axis=2)
    seg = seg / np.maximum(np.linalg.norm(seg, axis=0, keepdims=True), 1e-9)
    sim = (seg[:, 1:] * seg[:, :-1]).sum(axis=0)
    return float((sim < 0.85).sum() / (len(y) / sr / 60))


# --- bass -----------------------------------------------------------------------------------------
def pitch_track(y, sr=SR, fmin=30.0, fmax=400.0, hop_s=0.02):
    """f0 track by pYIN at 8 kHz: (times, midi float with nan where unvoiced)."""
    import librosa
    y8 = librosa.resample(y.astype(np.float32), orig_sr=sr, target_sr=8000)
    hop = int(8000 * hop_s)
    f0, voiced, prob = librosa.pyin(y8, fmin=fmin, fmax=fmax, sr=8000, frame_length=1536, hop_length=hop,
                                    fill_na=np.nan, resolution=0.1)
    t = np.arange(len(f0)) * hop_s
    midi = np.where(np.isfinite(f0) & (prob > 0.5), hz_to_midi(np.where(np.isfinite(f0), f0, 1.0)), np.nan)
    return t, midi


def notes_from_track(t, midi, hop_s=0.02, min_len=0.08, jump=0.7):
    """Segment a pitch track into notes: a note ends at a gap or a jump over `jump` semitones."""
    notes = []
    i, n = 0, len(midi)
    while i < n:
        if not np.isfinite(midi[i]):
            i += 1
            continue
        j = i + 1
        while j < n and np.isfinite(midi[j]) and abs(midi[j] - np.median(midi[max(i, j - 4):j])) < jump:
            j += 1
        if (j - i) * hop_s >= min_len:
            seg = midi[i:j]
            slope = float(np.polyfit(np.arange(len(seg)), seg, 1)[0] / hop_s) if len(seg) > 3 else 0.0
            notes.append({"t": float(t[i]), "dur": float((j - i) * hop_s), "midi": float(np.median(seg)),
                          "slope_semi_s": slope, "start": float(seg[:3].mean()), "end": float(seg[-3:].mean())})
        i = j
    return notes


def bass_line(y, sr=SR):
    t, midi = pitch_track(y, sr)
    notes = notes_from_track(t, midi)
    dur = len(y) / sr
    res = {"notes": len(notes), "notes_per_s": len(notes) / dur}
    if len(notes) < 8:
        res["note"] = "too few bass notes"
        return res
    m = np.array([n["midi"] for n in notes])
    pcs = np.round(m).astype(int) % 12
    wts = np.array([n["dur"] for n in notes])
    hist = np.bincount(pcs, weights=wts, minlength=12)
    hist = hist / hist.sum()
    res["pc_hist"] = {NOTE_NAMES[i]: round(float(hist[i]), 3) for i in range(12) if hist[i] > 0.01}
    res["top_pc"] = NOTE_NAMES[int(hist.argmax())]
    res["top_pc_share"] = float(hist.max())
    res["distinct_pcs"] = int((hist > 0.03).sum())
    d = np.diff(np.round(m)).astype(int)
    res["repeat_share"] = float((d == 0).mean())
    res["step_share"] = float(((np.abs(d) >= 1) & (np.abs(d) <= 2)).mean())
    res["third_share"] = float(((np.abs(d) >= 3) & (np.abs(d) <= 4)).mean())
    res["fourth_fifth_share"] = float(((np.abs(d) >= 5) & (np.abs(d) <= 7)).mean())
    res["octave_share"] = float((np.abs(d) == 12).mean())
    # glides: notes whose pitch changes over the note by >= 0.6 semitone with a steady slope
    gl = [n for n in notes if abs(n["end"] - n["start"]) >= 0.6 and n["dur"] >= 0.1]
    res["slide_share"] = float(len(gl) / len(notes))
    # repeating figure: best lag L (2..16 notes) where the pitch-class sequence repeats
    seq = pcs
    best = (0.0, None)
    for L in range(2, 17):
        if len(seq) > 3 * L:
            match = float((seq[L:] == seq[:-L]).mean())
            if match > best[0] + 0.02 or (L > 2 and best[1] and L % best[1] != 0 and match > best[0]):
                best = (match, L)
    res["repeat_match"] = best[0]
    res["figure_len_notes"] = best[1]
    if best[1]:
        res["figure_s"] = float(np.median([notes[i + best[1]]["t"] - notes[i]["t"] for i in range(len(notes) - best[1])]))
    # one-word reading
    if hist.max() >= 0.7 and res["repeat_share"] >= 0.5:
        kind = "pedal (one note held or repeated)"
    elif res["slide_share"] >= 0.3:
        kind = "slides between notes"
    elif best[0] >= 0.8 and best[1] and res["distinct_pcs"] >= 2:
        kind = f"ostinato (a {best[1]}-note figure repeats)"
    elif res["notes_per_s"] >= 1.2 and (res["step_share"] + res["third_share"]) >= 0.6 and res["distinct_pcs"] >= 4:
        kind = "walking (steady stepwise motion)"
    else:
        kind = "root-note movement / mixed"
    res["reading"] = kind
    return res


# --- lead ------------------------------------------------------------------------------------------
def lead_line(y, sr=SR, fmin=196.0, octaves=4, hop=512):
    """Top line from the harmonic-summed CQT salience in 196 Hz - 3 kHz. Returns notes and summaries."""
    import librosa
    bpo = 12
    C = np.abs(librosa.cqt(y.astype(np.float32), sr=sr, hop_length=hop, fmin=fmin, n_bins=octaves * bpo + 19,
                           bins_per_octave=bpo))
    nb = octaves * bpo
    sal = np.zeros((nb, C.shape[1]))
    for h, wt in ((1, 1.0), (2, 0.7), (3, 0.5)):
        off = int(round(bpo * np.log2(h)))
        sal += wt * C[off: off + nb]
    hop_s = hop / sr
    peak = sal.max(axis=0)
    floor = np.percentile(peak, 40)
    midi = np.where(peak > floor, hz_to_midi(fmin) + sal.argmax(axis=0), np.nan)
    midi = np.round(midi)
    # median smoothing over 5 frames
    sm = midi.copy()
    for i in range(2, len(midi) - 2):
        w = midi[i - 2:i + 3]
        w = w[np.isfinite(w)]
        sm[i] = np.median(w) if len(w) >= 3 else np.nan
    notes = notes_from_track(np.arange(len(sm)) * hop_s, sm, hop_s, min_len=0.09, jump=0.6)
    return notes, hop_s, sm


def lead_summary(notes, gap_phrase=0.45):
    res = {"notes": len(notes)}
    if len(notes) < 8:
        res["note"] = "too few lead notes"
        return res
    m = np.array([round(n["midi"]) for n in notes])
    d = np.diff(m).astype(int)
    ad = np.abs(d)
    res["range_semitones_p10_p90"] = float(np.percentile(m, 90) - np.percentile(m, 10))
    res["repeat_share"] = float((ad == 0).mean())
    res["step_share"] = float(((ad >= 1) & (ad <= 2)).mean())
    res["third_share"] = float(((ad >= 3) & (ad <= 4)).mean())
    res["fourth_fifth_share"] = float(((ad >= 5) & (ad <= 7)).mean())
    res["leap_share"] = float((ad > 7).mean())
    res["median_abs_interval"] = float(np.median(ad))
    res["up_share"] = float((d > 0).sum() / max(1, (d != 0).sum()))
    # phrases split by gaps; contour of each
    ph, cur = [], [0]
    for i in range(1, len(notes)):
        gap = notes[i]["t"] - (notes[i - 1]["t"] + notes[i - 1]["dur"])
        if gap > gap_phrase:
            ph.append(cur); cur = [i]
        else:
            cur.append(i)
    ph.append(cur)
    kinds = {"arch": 0, "rising": 0, "falling": 0, "flat or wavy": 0}
    lens = []
    for p in ph:
        if len(p) < 4:
            continue
        v = m[p]
        lens.append(len(p))
        net = v[-1] - v[0]
        mid_peak = v.argmax() in range(len(v) // 4, len(v) - len(v) // 4) and v.max() - max(v[0], v[-1]) >= 2
        if mid_peak:
            kinds["arch"] += 1
        elif net >= 3:
            kinds["rising"] += 1
        elif net <= -3:
            kinds["falling"] += 1
        else:
            kinds["flat or wavy"] += 1
    tot = sum(kinds.values())
    res["phrases"] = tot
    if tot:
        res["contour_share"] = {k: round(v / tot, 2) for k, v in kinds.items()}
        res["phrase_len_notes_median"] = float(np.median(lens))
    return res


# --- form ------------------------------------------------------------------------------------------
def form(y, bar_s, sr=SR):
    """Bar-synchronous repetition: similarity of bar i to bar i+lag for lags 1..32,
    the lags that stand out, and the share of bars that near-duplicate an earlier bar."""
    import librosa
    if bar_s is None or bar_s <= 0.5:
        return {"note": "no bar length"}
    hop = 1024
    mf = librosa.feature.mfcc(y=y.astype(np.float32), sr=sr, n_mfcc=13, hop_length=hop)[1:]
    ch = librosa.feature.chroma_stft(y=y.astype(np.float32), sr=sr, hop_length=hop)
    F = np.vstack([mf, ch * 4])
    fps = sr / hop
    nb = int(F.shape[1] / (bar_s * fps))
    if nb < 12:
        return {"note": "fewer than 12 bars"}
    B = np.array([F[:, int(i * bar_s * fps): int((i + 1) * bar_s * fps)].mean(axis=1) for i in range(nb)]).T
    B = (B - B.mean(axis=1, keepdims=True)) / np.maximum(B.std(axis=1, keepdims=True), 1e-9)
    Bn = B / np.maximum(np.linalg.norm(B, axis=0, keepdims=True), 1e-9)
    S = Bn.T @ Bn
    maxlag = min(32, nb // 2)
    curve = np.array([np.mean(np.diag(S, k)) for k in range(1, maxlag + 1)])
    res = {"bars": nb, "bar_s": bar_s, "similarity_by_lag_bars": {str(i + 1): round(float(v), 3) for i, v in enumerate(curve)}}
    pk, _ = sps.find_peaks(np.concatenate([[curve[0] - 1], curve, [curve[-1] - 1]]), prominence=0.03)
    stand = sorted([(int(p), float(curve[p - 1])) for p in pk if 1 <= p <= maxlag], key=lambda r: -r[1])[:4]
    res["repeat_lags_bars"] = [{"bars": a, "similarity": round(b, 3)} for a, b in stand]
    dup = 0
    for i in range(1, nb):
        if S[i, :i].max() >= 0.97:
            dup += 1
    res["near_duplicate_bar_share"] = float(dup / (nb - 1))
    return res


def run(stem_dir, mix_file, out_json):
    stems = load_stems(stem_dir)
    d = mono(stems["drums"])
    mix = sum(mono(s) for s in stems.values())
    total = len(mix) / SR
    res = {"seconds": total}
    res["meter"] = meter(d, mix)
    harm = sum(mono(stems[k]) for k in ("other", "vocals", "bass") if k in stems)
    bass_pc = None
    res["bass"] = bass_line(mono(stems["bass"])) if "bass" in stems else {}
    if res["bass"].get("pc_hist"):
        bass_pc = np.zeros(12)
        for k, v in res["bass"]["pc_hist"].items():
            bass_pc[NOTE_NAMES.index(k)] = v
    ch, _ = chroma_of(sum(mono(stems[k]) for k in ("other", "vocals") if k in stems) + 0.5 * mono(stems.get("bass", np.zeros((2, 1)))) if "bass" in stems else harm)
    res["key"] = key_mode(ch, bass_pc)
    res["chords"] = chords(harm)
    res["chords"]["old_counter_per_min"] = chord_changes_old(harm)
    other = mono(stems["other"])
    notes, hop_s, _ = lead_line(other)
    res["lead"] = lead_summary(notes)
    res["lead"]["note_rows_first_40"] = [{"at": mmss(n["t"]), "midi": round(n["midi"]), "dur": round(n["dur"], 2)} for n in notes[:40]]
    # a check that works on real music, where no truth exists: if chord changes are real musical
    # events they land on the beat grid far more often than chance does
    per0, ph0 = res["meter"].get("period_s"), res["meter"].get("phase_s")
    if per0 and res["chords"]["segments"]:
        ts = np.array([g["t0"] for g in res["chords"]["segments"][1:]])
        if len(ts) >= 8:
            tol = 0.15
            off = np.abs(((ts - ph0 + per0 / 2) % per0) - per0 / 2)
            res["chords"]["change_on_beat_share"] = float((off <= tol).mean())
            res["chords"]["change_on_beat_chance"] = float(min(1.0, 2 * tol / per0))
    bpb, per = res["meter"].get("beats_per_bar"), res["meter"].get("period_s")
    res["form"] = form(mix, bpb * per if bpb and per else None)
    save_json(out_json, res)
    return res


if __name__ == "__main__":
    run(sys.argv[1], sys.argv[2], sys.argv[3])
