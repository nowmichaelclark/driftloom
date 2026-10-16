# Ears tools: measuring records against Driftloom

> Kept as Ears 1 wrote it. The maintained scripts are in `tools/ears/` (see its README); where they differ, `tools/ears/` wins. The Drive-download paths below are from a session that no longer exists.

The scripts behind `docs/ears/batch-1.md`, kept here so a later session can rebuild them. Python needs `pip install --break-system-packages librosa soundfile`; the renderer needs Playwright's Chromium (global npm install) and a clone of the repo.

1. Pull audio from the Drive connector one file per call (files under about 6 MB), then `python3 -I sweep2.py` decodes the saved results into `audio/<album>/`. Album names come from the RULES list in `sweep2.py`; add rules for a new batch.
2. `node render-driftloom.mjs REPO OUTDIR N SECONDS SEED` renders Driftloom loops as the app plays them.
3. `python3 -I analyze.py OUT.jsonl FILES...` and `python3 -I extra.py OUT.jsonl FILES...` measure; run two in parallel on a two-core box (about 10 s of CPU per minute of audio).
4. `python3 -I summarize.py` prints per-album medians.

Mikey's side: `driftloom-ears.sh` does it all in one pass (every file under 5.5 MB), then the output folder goes into the Driftloom folder in Drive.

## analyze.py

```python
"""Measure audio files the same way, whether a record or a Driftloom loop.

Usage: python3 -I analyze.py OUT.jsonl FILE [FILE ...]
Appends one JSON object per file; skips files already in OUT.jsonl.

Every figure is about something a listener can hear:
  loudness      lufs, lra (EBU R128, from ffmpeg), crest
  tone          centroid (Hz), shares of energy by band, share 2-5 kHz
  space         width (side/mid, dB), tail (median decay after notes, dB/s)
  motion        onsets per second, tempo, pulse (how clear the beat is)
  pitch         tuning (cents off A440), wobble (cents, slow pitch drift
                on held notes), bends
  harmony       pitch classes active, chord changes per minute, bass hold
                (share of time the bass note stays put), major/minor lean
  form          repeat (how alike the music is to itself a few seconds
                later), drift10 / drift60 (how far the sound has moved
                after 10 s and 60 s), arc_db (slow loudness swell),
                sweep_oct (slow brightness movement, octaves)
"""
import json, os, subprocess, sys, re
import numpy as np
import librosa
import scipy.signal as sps

SR = 22050


def decode(path, sr, channels):
    cmd = ["ffmpeg", "-nostdin", "-v", "error", "-i", path, "-f", "f32le",
           "-ac", str(channels), "-ar", str(sr), "-"]
    raw = subprocess.run(cmd, capture_output=True, check=True).stdout
    x = np.frombuffer(raw, dtype=np.float32)
    return x.reshape(-1, channels).T if channels > 1 else x


def ebur128(path):
    r = subprocess.run(["ffmpeg", "-nostdin", "-hide_banner", "-i", path,
                        "-af", "ebur128=framelog=quiet", "-f", "null", "-"],
                       capture_output=True, text=True).stderr
    tail = r[r.rfind("Summary:"):]
    i = re.search(r"I:\s+(-?[\d.]+) LUFS", tail)
    l = re.search(r"LRA:\s+(-?[\d.]+) LU", tail)
    return (float(i.group(1)) if i else None, float(l.group(1)) if l else None)


def db(x):
    return 10 * np.log10(np.maximum(x, 1e-12))


def analyze(path):
    out = {"file": path, "album": os.path.basename(os.path.dirname(path)),
           "track": os.path.basename(path)}
    stereo = decode(path, SR, 2)
    y = stereo.mean(axis=0).astype(np.float32)
    dur = len(y) / SR
    out["seconds"] = round(dur, 1)
    if dur < 8 or np.max(np.abs(y)) < 1e-4:
        out["skip"] = "too short or silent"
        return out

    out["lufs"], out["lra"] = ebur128(path)
    rms_all = np.sqrt(np.mean(y ** 2))
    out["crest_db"] = round(float(20 * np.log10(np.max(np.abs(y)) / max(rms_all, 1e-9))), 1)

    # --- tone -------------------------------------------------------------
    S = np.abs(librosa.stft(y, n_fft=4096, hop_length=1024)) ** 2
    freqs = librosa.fft_frequencies(sr=SR, n_fft=4096)
    frame_e = S.sum(axis=0)
    loud = frame_e > np.percentile(frame_e, 20)          # ignore near-silence
    spec = S[:, loud].sum(axis=1)
    total = spec.sum()
    bands = {"sub_lt120": (0, 120), "low_120_500": (120, 500),
             "mid_500_2k": (500, 2000), "pres_2k_5k": (2000, 5000),
             "air_gt5k": (5000, SR / 2)}
    for name, (lo, hi) in bands.items():
        out[name] = round(float(spec[(freqs >= lo) & (freqs < hi)].sum() / total), 4)
    cent = (freqs[:, None] * S).sum(axis=0) / np.maximum(S.sum(axis=0), 1e-12)
    out["centroid_hz"] = round(float(np.median(cent[loud])), 0)

    # slow brightness movement: centroid in octaves over sounding frames,
    # smoothed over ~3 s, intro and outro (first and last 10%) left out
    hop_s = 1024 / SR
    nfr = len(cent)
    core = np.zeros(nfr, bool)
    core[int(nfr * 0.1): int(nfr * 0.9)] = True
    oc = np.log2(np.maximum(cent[loud & core], 20))
    k = max(1, int(3 / hop_s))
    if len(oc) > k * 3:
        oc_s = np.convolve(oc, np.ones(k) / k, mode="valid")
        out["sweep_oct"] = round(float(np.percentile(oc_s, 90) - np.percentile(oc_s, 10)), 2)

    # --- space ------------------------------------------------------------
    mid = (stereo[0] + stereo[1]) / 2
    side = (stereo[0] - stereo[1]) / 2
    out["width_db"] = round(float(db(np.mean(side ** 2)) - db(np.mean(mid ** 2))), 1)

    # --- loudness over time -------------------------------------------------
    rms = librosa.feature.rms(y=y, frame_length=2048, hop_length=512)[0]
    rdb = 20 * np.log10(np.maximum(rms, 1e-6))
    hop_r = 512 / SR
    k30 = int(30 / hop_r)
    mid_r = rdb[int(len(rdb) * 0.1): int(len(rdb) * 0.9)]
    if len(mid_r) > k30 * 2:
        smooth = np.convolve(mid_r, np.ones(k30) / k30, mode="valid")
        out["arc_db"] = round(float(np.percentile(smooth, 90) - np.percentile(smooth, 10)), 1)

    # --- onsets, tempo, pulse --------------------------------------------
    env = librosa.onset.onset_strength(y=y, sr=SR, hop_length=512)
    on = librosa.onset.onset_detect(onset_envelope=env, sr=SR, hop_length=512,
                                    backtrack=False, units="frames")
    out["onsets_per_s"] = round(len(on) / dur, 2)
    tempo = librosa.feature.tempo(onset_envelope=env, sr=SR, hop_length=512)
    out["tempo"] = round(float(np.atleast_1d(tempo)[0]), 1)
    ac = librosa.autocorrelate(env - env.mean(), max_size=int(4 / hop_r))
    ac = ac / max(ac[0], 1e-9)
    lo = int(0.25 / hop_r)
    out["pulse"] = round(float(np.max(ac[lo:])), 3)

    # tail: how fast sound dies after an isolated note (dB per second)
    slopes = []
    for f in on:
        nxt = on[on > f]
        gap = (nxt[0] - f) * hop_r if len(nxt) else dur - f * hop_r
        if gap < 0.8:
            continue
        seg = rdb[f: f + int(min(gap, 3.0) / hop_r)]
        if len(seg) < 10:
            continue
        p = int(np.argmax(seg[: max(3, len(seg) // 4)]))
        tail = seg[p:]
        if len(tail) < 8:
            continue
        t = np.arange(len(tail)) * hop_r
        slopes.append(np.polyfit(t, tail, 1)[0])
    out["tail_db_per_s"] = round(float(np.median(slopes)), 1) if len(slopes) >= 3 else None
    out["isolated_notes"] = len(slopes)

    # --- harmony -----------------------------------------------------------
    yh = librosa.effects.harmonic(y, margin=2.0)
    chroma = librosa.feature.chroma_cqt(y=yh, sr=SR, hop_length=2048)
    hop_c = 2048 / SR
    cn = chroma / np.maximum(chroma.max(axis=0, keepdims=True), 1e-9)
    act = (cn > 0.5).sum(axis=0)
    energetic = librosa.feature.rms(y=yh, frame_length=4096, hop_length=2048)[0]
    m = energetic[: chroma.shape[1]] > np.percentile(energetic, 25)
    out["pitch_classes"] = round(float(np.median(act[m[: len(act)]])) if m.any() else 0, 1)
    # chord changes: big jumps in 2-s averaged chroma
    k2 = max(1, int(2 / hop_c))
    nseg = chroma.shape[1] // k2
    if nseg > 3:
        seg = chroma[:, : nseg * k2].reshape(12, nseg, k2).mean(axis=2)
        seg = seg / np.maximum(np.linalg.norm(seg, axis=0, keepdims=True), 1e-9)
        sim = (seg[:, 1:] * seg[:, :-1]).sum(axis=0)
        out["chord_changes_per_min"] = round(float((sim < 0.85).sum() / (dur / 60)), 1)
    # key lean: Krumhansl profiles over the whole track
    maj = np.array([6.35, 2.23, 3.48, 2.33, 4.38, 4.09, 2.52, 5.19, 2.39, 3.66, 2.29, 2.88])
    mnr = np.array([6.33, 2.68, 3.52, 5.38, 2.60, 3.53, 2.54, 4.75, 3.98, 2.69, 3.34, 3.17])
    cm = chroma.mean(axis=1)
    cmaj = max(np.corrcoef(np.roll(maj, i), cm)[0, 1] for i in range(12))
    cmin = max(np.corrcoef(np.roll(mnr, i), cm)[0, 1] for i in range(12))
    out["key_major_r"] = round(float(cmaj), 3)
    out["key_minor_r"] = round(float(cmin), 3)
    # bass hold: strongest pitch class below ~260 Hz, how long it stays put
    Cb = np.abs(librosa.cqt(yh, sr=SR, hop_length=2048, fmin=librosa.note_to_hz("C1"),
                            n_bins=36, bins_per_octave=12))
    bass_pc = Cb.argmax(axis=0) % 12
    bmask = Cb.max(axis=0) > np.percentile(Cb.max(axis=0), 30)
    if bmask.sum() > 20:
        b = bass_pc[bmask]
        vals, counts = np.unique(b, return_counts=True)
        out["bass_hold"] = round(float(counts.max() / len(b)), 3)
        # bass note per ~1 s (mode of each window), so flicker isn't counted
        kb = max(1, int(1 / (2048 / SR)))
        nb = len(b) // kb
        if nb > 3:
            per = [np.bincount(b[i * kb:(i + 1) * kb], minlength=12).argmax() for i in range(nb)]
            changes = int((np.diff(per) != 0).sum())
            out["bass_changes_per_min"] = round(float(changes / (len(b) * 2048 / SR / 60)), 1)

    # tuning offset
    out["tuning_cents"] = round(float(librosa.estimate_tuning(y=yh, sr=SR) * 100), 1)

    # --- pitch wobble and bends on held partials (works with chords) --------
    # Track spectral peaks 150-2500 Hz frame to frame; a partial held for at
    # least 0.5 s gives one wobble figure (std of its pitch around its own
    # straight-line trend, in cents) and counts as a bend if it glides more
    # than 50 cents end to end.
    hop_p = 1024
    P = np.abs(librosa.stft(yh, n_fft=8192, hop_length=hop_p))
    fr = librosa.fft_frequencies(sr=SR, n_fft=8192)
    lo_b, hi_b = np.searchsorted(fr, 150), np.searchsorted(fr, 2500)
    L = np.log(np.maximum(P[lo_b - 1: hi_b + 1], 1e-9))
    tracks, live = [], []
    min_len = int(0.5 * SR / hop_p)
    for t in range(P.shape[1]):
        col = L[:, t]
        pk = np.where((col[1:-1] > col[:-2]) & (col[1:-1] > col[2:]))[0] + 1
        if len(pk) == 0:
            cents_now = np.array([])
        else:
            pk = pk[col[pk] > col.max() - 3.5]          # within ~30 dB of the top
            pk = pk[np.argsort(col[pk])[::-1][:8]]
            a_, b_, c_ = col[pk - 1], col[pk], col[pk + 1]
            off = 0.5 * (a_ - c_) / np.where(a_ - 2 * b_ + c_ == 0, -1e-9, a_ - 2 * b_ + c_)
            f = (lo_b - 1 + pk + off) * SR / 8192
            cents_now = 1200 * np.log2(f / 440.0)
        used = set()
        nxt = []
        for tr in live:
            if len(cents_now):
                d = np.abs(cents_now - tr[-1])
                j = int(np.argmin(d))
                if d[j] < 30 and j not in used:
                    used.add(j)
                    tr.append(cents_now[j])
                    nxt.append(tr)
                    continue
            if len(tr) >= min_len:
                tracks.append(tr)
        for j, c in enumerate(cents_now):
            if j not in used:
                nxt.append([c])
        live = nxt
    tracks += [tr for tr in live if len(tr) >= min_len]
    # Long partials are judged in 3-s pieces, so a slow wow isn't mistaken
    # for a trend and one long drone doesn't count as one sample.
    wob, bends, pieces = [], 0, 0
    chunk = int(3 * SR / hop_p)
    for tr in tracks:
        r_all = np.array(tr)
        for s in range(0, len(r_all), chunk):
            r = r_all[s: s + chunk]
            if len(r) < min_len:
                continue
            pieces += 1
            x = np.arange(len(r))
            fit = np.polyfit(x, r, 1)
            wob.append(np.std(r - np.polyval(fit, x)))
            secs = len(r) * hop_p / SR
            glide = abs(fit[0] * (len(r) - 1))
            if glide > 40 and glide / secs > 30:
                bends += 1
    out["held_partials"] = len(tracks)
    out["wobble_cents"] = round(float(np.median(wob)), 2) if len(wob) >= 3 else None
    out["bends_per_min"] = round(bends / (dur / 60), 2)
    out["bend_share"] = round(bends / pieces, 3) if pieces else None

    # --- form: repetition and drift -----------------------------------------
    mf = librosa.feature.mfcc(y=y, sr=SR, n_mfcc=13, hop_length=2048)
    feat = np.vstack([mf[1:], chroma[:, : mf.shape[1]] * 10])
    feat = (feat - feat.mean(axis=1, keepdims=True)) / np.maximum(feat.std(axis=1, keepdims=True), 1e-9)
    hop_f = 2048 / SR
    # repeat: best correlation of the feature sequence with itself 1-16 s later
    best = 0.0
    n = feat.shape[1]
    for lag in range(int(1 / hop_f), min(int(16 / hop_f), n // 2)):
        a1, b1 = feat[:, :-lag].ravel(), feat[:, lag:].ravel()
        r = np.corrcoef(a1, b1)[0, 1]
        best = max(best, r)
    out["repeat"] = round(float(best), 3)
    # drift: distance between 5-s means at 10 s and 60 s apart
    w = int(5 / hop_f)
    nw = n // w
    if nw >= 14:
        M = feat[:, : nw * w].reshape(feat.shape[0], nw, w).mean(axis=2)
        def dist(lag):
            return float(np.mean(np.linalg.norm(M[:, lag:] - M[:, :-lag], axis=0)))
        out["drift10"] = round(dist(2), 2)
        out["drift60"] = round(dist(12), 2)
    return out


def main():
    outp, files = sys.argv[1], sys.argv[2:]
    done = set()
    if os.path.exists(outp):
        for line in open(outp):
            try:
                done.add(json.loads(line)["file"])
            except Exception:
                pass
    for p in files:
        if p in done:
            continue
        try:
            row = analyze(p)
        except Exception as e:
            row = {"file": p, "error": repr(e)}
        with open(outp, "a") as f:
            f.write(json.dumps(row) + "\n")
        print(row.get("track"), "ok" if "error" not in row else row["error"], flush=True)


if __name__ == "__main__":
    main()
```

## extra.py

```python
"""Second pass: finer energy bands, and chord density above the bass.

Usage: python3 -I extra.py OUT.jsonl FILE [FILE ...]
"""
import json, os, subprocess, sys
import numpy as np
import librosa

SR = 22050
EDGES = [(0, 40), (40, 60), (60, 120), (120, 250), (250, 1000), (1000, 2000), (2000, 5000), (5000, 11025)]


def decode(path):
    raw = subprocess.run(["ffmpeg", "-nostdin", "-v", "error", "-i", path, "-f", "f32le",
                          "-ac", "1", "-ar", str(SR), "-"], capture_output=True, check=True).stdout
    return np.frombuffer(raw, np.float32)


def run(path):
    y = decode(path)
    out = {"file": path, "album": os.path.basename(os.path.dirname(path)), "track": os.path.basename(path),
           "seconds": round(len(y) / SR, 1)}
    if len(y) < SR * 8:
        out["skip"] = 1
        return out
    S = np.abs(librosa.stft(y, n_fft=8192, hop_length=2048)) ** 2
    f = librosa.fft_frequencies(sr=SR, n_fft=8192)
    e = S.sum(axis=0)
    loud = e > np.percentile(e, 20)
    s = S[:, loud].sum(axis=1)
    s = s / s.sum()
    for lo, hi in EDGES:
        out[f"b{lo}_{hi}"] = round(float(s[(f >= lo) & (f < hi)].sum()), 4)
    # Chord density above the bass: chroma from 200-2000 Hz only, pitch
    # classes within 6 dB of the strongest, in sounding frames.
    yh = librosa.effects.harmonic(y, margin=2.0)
    C = np.abs(librosa.cqt(yh, sr=SR, hop_length=2048, fmin=librosa.note_to_hz("G3"),
                           n_bins=36, bins_per_octave=12)) ** 2
    chroma = np.zeros((12, C.shape[1]))
    for b in range(36):
        chroma[(b + 7) % 12] += C[b]            # G3 is pitch class 7
    ce = chroma.sum(axis=0)
    m = ce > np.percentile(ce, 30)
    cn = chroma[:, m] / np.maximum(chroma[:, m].max(axis=0, keepdims=True), 1e-12)
    out["upper_pcs"] = round(float(np.median((cn > 0.25).sum(axis=0))), 1) if m.any() else None
    return out


if __name__ == "__main__":
    outp, files = sys.argv[1], sys.argv[2:]
    done = set()
    if os.path.exists(outp):
        done = {json.loads(l)["file"] for l in open(outp) if l.strip()}
    with open(outp, "a") as fh:
        for p in files:
            if p in done:
                continue
            try:
                row = run(p)
            except Exception as ex:
                row = {"file": p, "error": repr(ex)}
            fh.write(json.dumps(row) + "\n")
            fh.flush()
```

## summarize.py

```python
"""Per-album medians of the measures, with Driftloom loops as the last rows.

Usage: python3 -I summarize.py [--csv out.csv]
Reads records*.jsonl (records) and driftloom*.jsonl (loops).
Split pieces of one track are pooled by album like any other file, and
seconds-weighted where that matters (onsets, changes per minute).
"""
import glob, json, sys
from collections import defaultdict
import numpy as np

COLS = [
    ("lufs", "LUFS", 1), ("lra", "LRA", 1), ("crest_db", "crest", 1),
    ("centroid_hz", "bright Hz", 0), ("pres_2k_5k", "2-5k %", 100), ("air_gt5k", ">5k %", 100),
    ("sub_lt120", "<120 %", 100), ("width_db", "width", 1),
    ("onsets_per_s", "notes/s", 2), ("tempo", "tempo", 0), ("pulse", "pulse", 2),
    ("tail_db_per_s", "tail dB/s", 1),
    ("pitch_classes", "pcs", 1), ("chord_changes_per_min", "chords/min", 1),
    ("bass_hold", "bass hold", 2), ("bass_changes_per_min", "bass/min", 1),
    ("tuning_cents", "tuning", 0), ("wobble_cents", "wobble", 2), ("bend_share", "bends", 3),
    ("repeat", "repeat", 2), ("drift10", "drift10", 2), ("drift60", "drift60", 2),
    ("arc_db", "arc dB", 1), ("sweep_oct", "sweep oct", 2),
]


def load(patterns, group_key):
    rows = []
    for pat in patterns:
        for p in glob.glob(pat):
            for line in open(p):
                d = json.loads(line)
                if "error" in d or "skip" in d:
                    continue
                rows.append(d)
    groups = defaultdict(list)
    for d in rows:
        groups[group_key(d)].append(d)
    return groups


def med(rows, key, weight=False):
    v = [(r[key], r.get("seconds", 1)) for r in rows if r.get(key) is not None]
    if not v:
        return None
    vals = np.array([a for a, _ in v], float)
    if weight:
        w = np.array([b for _, b in v], float)
        order = np.argsort(vals)
        cw = np.cumsum(w[order]) / w.sum()
        return float(vals[order][np.searchsorted(cw, 0.5)])
    return float(np.median(vals))


def table(groups, label_width=34):
    head = f"{'':{label_width}}" + "".join(f"{c[1]:>10}" for c in COLS)
    print(head)
    for name in sorted(groups):
        rows = groups[name]
        cells = []
        for key, _, scale in COLS:
            m = med(rows, key, weight=True)
            if m is None:
                cells.append(f"{'-':>10}")
            else:
                val = m * (scale if scale > 2 else 1)
                dec = 1 if scale > 2 else scale
                cells.append(f"{val:>10.{dec}f}")
        print(f"{name[:label_width]:{label_width}}" + "".join(cells) + f"   n={len(rows)}")


if __name__ == "__main__":
    rec = load(["records*.jsonl"], lambda d: d["album"])
    table(rec)
    print()
    loops = load(["driftloom*.jsonl"], lambda d: "Driftloom: " + d["track"].split("-", 1)[1].replace(".wav", ""))
    if loops:
        table(loops)
        allrows = [r for v in loops.values() for r in v]
        print()
        table({"Driftloom: ALL LOOPS": allrows})
```

## sweep2.py

```python
"""Decode downloaded split pieces into audio/<album>/, choosing the album
from the track name. Pieces whose whole track is already here are dropped.

Usage: python3 -I sweep2.py
"""
import base64, glob, json, os, re, shutil

RESULTS = "/root/.claude/projects/-home-claude/6077a4f8-5c39-5129-8e52-06e657912264/tool-results"
HERE = os.path.dirname(os.path.abspath(__file__))
AUDIO = os.path.join(HERE, "audio")

RULES = [
    (r"^Narkopop", "Gas - Narkopop"),
    (r"20 Seconds Of Affection|No\. No|A Guided Tour|Black Sea|They Won't See Me|Cupid's Head", "The Field - Cupid's Head"),
    (r"Pauline Anna Strom", "Pauline Anna Strom - Trans-Millenia Music"),
    (r"[A-Z]{3}-[A-Z]{3}", "Pendant - Make Me Know You Sweet"),
    (r"^0\d - (Largo|Rondo|Finale|Opening)", "Konrad Sprenger - Stack Music"),
    (r"^0\d (joy|grief & loss|come here go there|lust)", "Rei Harakami - [lust]"),
    (r"angel play|rainbow canyon", "Iasos - Inter-Dimensional Music"),
    (r"Yialmelic", "Yialmelic Frequencies - Yililok"),
    (r"Radiance|Q-Loop|Quadrant|Mutism|Lyot|Presence|e2e4|Q1\.", "Basic Channel - BCD"),
    (r"Aphex Twin", "Aphex Twin - Drukqs (piano)"),
    (r"Lost Day|Lizard Point|Tal Coat|Dunwich", "Brian Eno - Ambient 4 On Land"),
    (r"The Orb", "The Orb - Orbus Terrarum"),
    (r"The Park|The Backyard", "Robert Ashley - Private Parts"),
    (r"Letting Go|^02 - Together|With Faith|Apart|Still Right Here|Hope We Can", "Nine Inch Nails - Ghosts V Together"),
    (r"Ein Wort|Dubwise|Wirklichkeit|Batacuda|Leben im", "Monoton - Blau"),
    (r"neu!", "Neu! - Neu! 2"),
    (r"sehr kosmisch|watussi", "Harmonia - Musik von Harmonia"),
    (r"Kelly Lee Owens", "Kelly Lee Owens - Inner Song"),
    (r"Sowiesoso|Zum Wohl|In Ewigkeit", "Cluster - Sowiesoso"),
]

have = {os.path.splitext(f)[0] for d in os.listdir(AUDIO) for f in os.listdir(os.path.join(AUDIO, d))}
swept = os.path.join(HERE, "swept")
os.makedirs(swept, exist_ok=True)
for path in sorted(glob.glob(os.path.join(RESULTS, "mcp-Google_Drive-download_file_content-*.txt"))):
    d = json.load(open(path))
    title = d["title"]
    whole = re.sub(r" - piece\d\d\.opus$", "", title)
    album = next((a for pat, a in RULES if re.search(pat, title)), "_unsorted")
    if whole in have:
        print(f"skip (whole track already here)  {title}")
    else:
        os.makedirs(os.path.join(AUDIO, album), exist_ok=True)
        data = base64.b64decode(d["content"])
        open(os.path.join(AUDIO, album, title.replace("/", "_")), "wb").write(data)
        print(f"{len(data):>9}  {album} / {title}")
    shutil.move(path, os.path.join(swept, os.path.basename(path)))
```

## render-driftloom.mjs

```js
// Render Driftloom loops to WAV files, the way the app plays them:
// the real Engine and Synth (JavaScript engine, full quality) through an
// OfflineAudioContext in headless Chromium, with "Let the loop wander" on
// at its default amount. Lives outside the repo; reads the repo's modules.
//
// Usage: node render-driftloom.mjs REPO OUTDIR N SECONDS SEED
import fs from 'node:fs';
import http from 'node:http';
import path from 'node:path';
import { createRequire } from 'node:module';
import { execSync } from 'node:child_process';

const [REPO, OUT, N = '60', SECONDS = '60', SEED = '20261009'] = process.argv.slice(2);
const ROOT = path.resolve(REPO);
fs.mkdirSync(OUT, { recursive: true });
const RATE = 44100;

const PAGE = `<!doctype html><meta charset="utf-8"><script type="module">
import { Engine } from '/js/engine.js';
import { Synth } from '/js/synth.js';
import { newSpec, render } from '/js/generator.js';
import { Rng, mulberry32 } from '/js/rng.js';

function leadOf(spec) {
  return Object.entries(spec.mix || {}).sort((a, b) => b[1] - a[1])[0]?.[0] || '?';
}
window.specsFor = (seed, n) => {
  const master = new Rng(seed);
  return Array.from({ length: n }, () => newSpec(master.seed32()));
};
window.renderOne = async (spec, seconds, rate) => {
  const ctx = new OfflineAudioContext(2, Math.ceil(seconds * rate), rate);
  Math.random = mulberry32(spec.seed >>> 0 || 1);
  const synth = new Synth(ctx, 'full');
  const engine = new Engine(ctx, synth);
  if (engine.clock.worker) engine.clock.worker.terminate();
  engine.driftOn = true;
  engine.driftAmount = 1;
  engine.driftRng = new Rng((spec.seed ^ 0x5eed1e55) >>> 0);
  engine.load(spec);
  engine.playing = true;
  engine.nextStepTime = 0.05;
  let guard = 0;
  while (engine.nextStepTime < seconds && guard++ < 400000) {
    engine._scheduleStep(engine.step, engine.nextStepTime);
    engine._advance();
  }
  const buf = await ctx.startRendering();
  const L = buf.getChannelData(0), R = buf.getChannelData(1);
  const pcm = new Int16Array(L.length * 2);
  for (let i = 0; i < L.length; i++) {
    pcm[2 * i] = Math.max(-32768, Math.min(32767, Math.round(L[i] * 32767)));
    pcm[2 * i + 1] = Math.max(-32768, Math.min(32767, Math.round(R[i] * 32767)));
  }
  let s = '';
  const bytes = new Uint8Array(pcm.buffer);
  for (let i = 0; i < bytes.length; i += 0x8000) s += String.fromCharCode.apply(null, bytes.subarray(i, i + 0x8000));
  const r = render(spec);
  const meta = r.meta || {};
  return {
    b64: btoa(s), lead: leadOf(spec), mix: spec.mix, seed: spec.seed,
    bpm: spec.bpm ?? null,
    mood: spec.mood ?? null, meta, bars: r.totalSteps ? r.totalSteps / 16 : null,
  };
};
window.ready = true;
</script>`;

const TYPES = { '.js': 'text/javascript', '.mjs': 'text/javascript', '.wasm': 'application/wasm', '.json': 'application/json', '.html': 'text/html' };
const server = http.createServer((req, res) => {
  const url = decodeURIComponent(req.url.split('?')[0]);
  if (url === '/__render.html') { res.writeHead(200, { 'content-type': 'text/html' }); return res.end(PAGE); }
  const file = path.join(ROOT, path.normalize(url));
  if (!file.startsWith(ROOT) || !fs.existsSync(file) || fs.statSync(file).isDirectory()) { res.writeHead(404); return res.end(); }
  res.writeHead(200, { 'content-type': TYPES[path.extname(file)] || 'application/octet-stream' });
  fs.createReadStream(file).pipe(res);
});
await new Promise((r) => server.listen(0, '127.0.0.1', r));
const port = server.address().port;

const require = createRequire(import.meta.url);
const globalRoot = execSync('npm root -g').toString().trim();
const { chromium } = require(path.join(globalRoot, 'playwright'));
const browser = await chromium.launch({ args: ['--no-sandbox'] });
const page = await browser.newPage();
page.on('pageerror', (e) => console.error('page error:', e.message));
await page.goto(`http://127.0.0.1:${port}/__render.html`);
await page.waitForFunction(() => window.ready === true, null, { timeout: 20000 });

const specs = await page.evaluate(([s, n]) => window.specsFor(s, n), [Number(SEED), Number(N)]);
const index = [];
for (let i = 0; i < specs.length; i++) {
  const r = await page.evaluate(([spec, sec, rate]) => window.renderOne(spec, sec, rate), [specs[i], Number(SECONDS), RATE]);
  const pcm = Buffer.from(r.b64, 'base64');
  const header = Buffer.alloc(44);
  header.write('RIFF', 0); header.writeUInt32LE(36 + pcm.length, 4); header.write('WAVE', 8);
  header.write('fmt ', 12); header.writeUInt32LE(16, 16); header.writeUInt16LE(1, 20); header.writeUInt16LE(2, 22);
  header.writeUInt32LE(RATE, 24); header.writeUInt32LE(RATE * 4, 28); header.writeUInt16LE(4, 32); header.writeUInt16LE(16, 34);
  header.write('data', 36); header.writeUInt32LE(pcm.length, 40);
  const name = `${String(i).padStart(3, '0')}-${r.lead}.wav`;
  fs.writeFileSync(path.join(OUT, name), Buffer.concat([header, pcm]));
  delete r.b64;
  index.push({ file: name, ...r });
  console.log(name, r.bpm ?? '', r.mood ?? '');
}
fs.writeFileSync(path.join(OUT, 'index.json'), JSON.stringify(index, null, 1));
await browser.close();
server.close();
```

## driftloom-ears.sh

One pass: unzip (nested too), 96 kbps Opus, tracks over 5 minutes cut into 4-minute pieces, safety re-cut of anything over 5.5 MB. Replaces the old two-step convert-then-split.

```bash
#!/usr/bin/env bash
# driftloom-ears.sh -- get albums ready for Claude to pull from Google Drive.
#
# One pass does everything: unzips (zips inside zips too), turns every
# track into a 96 kbps stereo Opus file, and cuts anything longer than
# 5 minutes into 4-minute pieces, so every file stays under the 5.5 MB
# that Claude's Drive link carries. Any format works: flac, mp3, m4a,
# ogg, opus, wav, aiff, wv, ape, wma.
#
# Usage:
#   ./driftloom-ears.sh  ZIP_OR_FOLDER_OR_FILE  [more ...]
#
# Example:
#   ./driftloom-ears.sh ~/Downloads/"Music Pile 2.zip"
#
# Results land in a folder named after the first thing you give it, like
# ./for-claude - Music Pile 2/<Album>/... Drag that whole folder into the
# Driftloom folder on Google Drive. Running it again skips finished tracks.

set -euo pipefail

BITRATE="${BITRATE:-96k}"
PART_SECONDS="${PART_SECONDS:-240}"
CUT_OVER="${CUT_OVER:-300}"
LIMIT_BYTES=$((5500 * 1024))
AUDIO_RE='\.(flac|mp3|m4a|m4b|aac|alac|ogg|oga|opus|wav|aif|aiff|wv|ape|wma)$'

need() { command -v "$1" >/dev/null 2>&1 || { echo "Missing $1. On Fedora: sudo dnf install $2"; exit 1; }; }
need ffmpeg ffmpeg-free
need ffprobe ffmpeg-free
need unzip unzip

[ "$#" -ge 1 ] || { sed -n '2,20p' "$0"; exit 1; }
first="$(basename "${1%/}")"
OUT="${OUT:-$PWD/for-claude - ${first%.zip}}"

mkdir -p "$OUT"
# Work space next to the output, not /tmp: on Fedora /tmp lives in RAM,
# and a 3 GB zip would fill it.
WORK="$(mktemp -d "$PWD/.ears-work.XXXXXX")"
trap 'rm -rf "$WORK"' EXIT

done_count=0; skip_count=0; split_count=0; fail_count=0

clean() { printf '%s' "$1" | sed 's#[/\\:*?"<>|]#_#g; s/^ *//; s/ *$//'; }

convert_one() {
  local src="$1" album="$2"
  local base dir dur
  base="$(clean "$(basename "${src%.*}")")"
  dir="$OUT/$(clean "$album")"
  mkdir -p "$dir"

  if compgen -G "$dir/$base*.opus" >/dev/null; then
    skip_count=$((skip_count + 1)); return
  fi

  dur="$(ffprobe -v error -show_entries format=duration -of default=nw=1:nk=1 "$src" 2>/dev/null | cut -d. -f1)"
  dur="${dur:-0}"

  if [ "$dur" -gt "$CUT_OVER" ]; then
    echo "  $album / $base  ($((dur / 60)) min, cutting into parts)"
    if ffmpeg -nostdin -hide_banner -v error -i "$src" -map 0:a:0 -vn \
         -c:a libopus -b:a "$BITRATE" \
         -f segment -segment_time "$PART_SECONDS" -reset_timestamps 1 \
         "$dir/$base - part%02d.opus"; then
      split_count=$((split_count + 1)); done_count=$((done_count + 1))
    else
      echo "    FAILED"; fail_count=$((fail_count + 1))
    fi
  else
    echo "  $album / $base"
    if ffmpeg -nostdin -hide_banner -v error -i "$src" -map 0:a:0 -vn \
         -c:a libopus -b:a "$BITRATE" "$dir/$base.opus"; then
      done_count=$((done_count + 1))
    else
      echo "    FAILED"; fail_count=$((fail_count + 1))
    fi
  fi
}

# Every audio file under ROOT. The album name comes from the folders it
# sits in ("Harmonia/Musik von Harmonia" -> "Harmonia - Musik von Harmonia"),
# or from ROOT's own name when the tracks sit at the top.
convert_tree() {
  local root="$1" rootname="$2" f parent rel album
  while IFS= read -r -d '' f; do
    parent="$(dirname "$f")"
    if [ "$parent" = "$root" ]; then
      album="$rootname"
    else
      rel="${parent#"$root"/}"
      album="${rel//\// - }"
    fi
    convert_one "$f" "$album"
  done < <(find "$root" -type f -regextype posix-extended -iregex ".*$AUDIO_RE" -print0 | sort -z)
}

for input in "$@"; do
  if [ -d "$input" ]; then
    echo "Folder: $input"
    convert_tree "$(cd "$input" && pwd)" "$(basename "$input")"
  elif [ -f "$input" ] && [[ "${input,,}" == *.zip ]]; then
    echo "Zip: $input (unpacking)"
    name="$(basename "${input%.*}")"
    mkdir -p "$WORK/$name"
    unzip -q -o "$input" -d "$WORK/$name"
    # Zips inside the zip: unpack each where it sits (a zip holding one
    # folder unpacks without an extra level), until none are left.
    while [ -n "$(find "$WORK/$name" -type f -iname '*.zip' -print -quit)" ]; do
      while IFS= read -r -d '' z; do
        d="${z%.*}"; unzip -q -o "$z" -d "$d" && rm -f "$z"
        inner=("$d"/*)
        if [ "${#inner[@]}" -eq 1 ] && [ -d "${inner[0]}" ]; then
          mv "${inner[0]}"/* "$d"/ 2>/dev/null; rmdir "${inner[0]}" 2>/dev/null || true
        fi
      done < <(find "$WORK/$name" -type f -iname '*.zip' -print0)
    done
    root="$WORK/$name"
    # A zip holding one folder: start inside it, so album names stay short.
    entries=("$root"/*)
    if [ "${#entries[@]}" -eq 1 ] && [ -d "${entries[0]}" ]; then
      root="${entries[0]}"; name="$(basename "$root")"
    fi
    convert_tree "$root" "$name"
    rm -rf "${WORK:?}/$name"
  elif [ -f "$input" ] && [[ "${input,,}" =~ $AUDIO_RE ]]; then
    echo "File: $input"
    convert_one "$input" "$(basename "${input%.*}")"
  else
    echo "Skipping (not a folder, zip or audio file): $input"
  fi
done

echo
echo "Done: $done_count tracks ($split_count cut into parts), $skip_count already done, $fail_count failed."

# Cutting can leave a tail of a second or less; drop those.
find "$OUT" -type f -name '*.opus' -size -20k -delete
# Safety net: anything still over the limit (a very dense track) is cut
# again, without re-encoding.
while IFS= read -r -d '' f; do
  echo "  re-cutting (still too big): $(basename "$f")"
  ffmpeg -nostdin -hide_banner -v error -i "$f" -map 0:a:0 -c copy \
    -f segment -segment_time 120 -reset_timestamps 1 "${f%.opus} - piece%02d.opus" && rm -f "$f"
done < <(find "$OUT" -type f -name '*.opus' -size +"${LIMIT_BYTES}"c -print0)
big="$(find "$OUT" -type f -name '*.opus' -size +"${LIMIT_BYTES}"c)"
[ -z "$big" ] || { echo "Still over the limit; tell Claude which ones:"; echo "$big"; }
echo "Total size: $(du -sh "$OUT" | cut -f1)  in  $OUT"
echo "Upload that whole folder to the Driftloom folder in Google Drive."
```

# Added for batch two

The route for pile 2: `driftloom-ears.sh` on Mikey's side; here, download by file id in parallel batches of about 20–40 (each result lands in the tool-results folder), then `python3 -I sweep3.py` files them by title. Measure with `run-pile2.sh`-style two-worker runs of `analyze.py` and `extra.py` (`extra.py` now skips files already done), then `sections.py`. `summarize2.py` prints the batch-two tables.

## sweep3.py

Files Drive downloads into `audio/<album>/` by title rules; add rules for a new pile.

```python
"""Pile 2: decode Drive downloads into audio/<album>/<title>, routing by title.

Usage: python3 -I sweep3.py
Unknown titles go to audio/_unsorted.
"""
import base64, glob, json, os, re, shutil

RESULTS = "/root/.claude/projects/-home-claude/6077a4f8-5c39-5129-8e52-06e657912264/tool-results"
HERE = os.path.dirname(os.path.abspath(__file__))

RULES = [
    (r"^Mike & Rich", "Mike & Rich - Expert Knob Twiddlers"),
    (r"^Death Grips - Fashion Week", "Death Grips - Fashion Week (Instrumentals)"),
    (r"^Blank Banshee", "Blank Banshee - Blank Banshee 0"),
    (r"^\d+\.The Field", "The Field - From Here We Go Sublime"),
    (r"^0\d - [A-Z ]+ - part", "Ryuichi Sakamoto - Thousand Knives"),
    (r"^0\d - (Port |Nautical|Biokinetics)", "Porter Ricks - Biokinetics"),
    (r"^\d\d (Intro|LFO|Simon From Sydney|Nurture|Freeze|We Are Back|Tan Ta Ra|You Have to Understand|El Ef Oh|Love Is the Message|Mentok|Think a Moment|Groovy Distortion|Track 14)", "LFO - Frequencies"),
    (r"^\d\d (Ljoss|Thor's Stone|Irby Tremor|Onward|The Weight of Gold|An Hour|Anneka|Gathering|The Plumes|Friend, You Will)", "Forest Swords - Engravings"),
    (r"^\d\d (New Colour|The Animal Pattern|As a Child|Lying in the Reeds|Dragon Blue Eyes|Crystal Caverns|Raindance|Dream Girl|Earth|Cthulhu|Stands Tidal|Spirals)", "Lone - Galaxy Garden"),
    (r"^\d\d (Kalpol|Bike|Autriche|Bronchus|Basscadet|Eggshell|Doctrine|Maetl|Windwind|Lowride|444)", "Autechre - Incunabula"),
    (r"^\d\d autechre - ", "Autechre - Amber"),
    (r"^\d\d (Boring Angel|Americans|He She|Inside World|Zebra|Along|Problem Areas|Cryo|Still Life|Chrome Country)", "Oneohtrix Point Never - R Plus Seven"),
]

swept = os.path.join(HERE, "swept")
os.makedirs(swept, exist_ok=True)
n = 0
for path in sorted(glob.glob(os.path.join(RESULTS, "mcp-Google_Drive-download_file_content-*.txt"))):
    with open(path) as f:
        d = json.load(f)
    title = d["title"].replace("/", "_")
    album = next((a for pat, a in RULES if re.search(pat, title)), "_unsorted")
    out = os.path.join(HERE, "audio", album)
    os.makedirs(out, exist_ok=True)
    data = base64.b64decode(d["content"])
    with open(os.path.join(out, title), "wb") as f:
        f.write(data)
    shutil.move(path, os.path.join(swept, os.path.basename(path)))
    n += 1
    print(f"{len(data):>9}  {album[:28]:28}  {title}")
print(f"{n} files")
```

## sections.py

Floor drops (low end out for 2 s or more while the rest plays) and full stops (whole mix 30 dB down for 1 s or more).

```python
"""Breakdowns: how often the low end drops out while the music keeps playing.

Usage: python3 -I sections.py OUT.jsonl FILE [FILE ...]   (skips files done)

Per 0.5-s frame: energy below 120 Hz (kick and bass) and 120 Hz-5 kHz (the
rest). A "low drop" is a run of at least 2 s where the low band sits 12 dB
or more under its own 75th percentile while the rest stays within 12 dB of
its median (so it isn't a pause or a fade). Reported per minute, with the
median drop length. Also "full rests": runs of 1 s or more where everything
is 30 dB under the median (a real gap in the music).
"""
import json, os, subprocess, sys
import numpy as np

SR = 22050
HOP = SR // 2


def decode(path):
    raw = subprocess.run(["ffmpeg", "-nostdin", "-v", "error", "-i", path, "-f", "f32le",
                          "-ac", "1", "-ar", str(SR), "-"], capture_output=True, check=True).stdout
    return np.frombuffer(raw, np.float32)


def runs(mask, min_len):
    out, n = [], 0
    for m in list(mask) + [False]:
        if m:
            n += 1
        else:
            if n >= min_len:
                out.append(n)
            n = 0
    return out


def run(path):
    y = decode(path)
    out = {"file": path, "album": os.path.basename(os.path.dirname(path)), "track": os.path.basename(path),
           "seconds": round(len(y) / SR, 1)}
    if len(y) < SR * 20:
        out["skip"] = 1
        return out
    n = len(y) // HOP
    frames = y[: n * HOP].reshape(n, HOP) * np.hanning(HOP)
    P = np.abs(np.fft.rfft(frames, axis=1)) ** 2
    f = np.fft.rfftfreq(HOP, 1 / SR)
    low = 10 * np.log10(P[:, (f >= 30) & (f < 120)].sum(axis=1) + 1e-12)
    rest = 10 * np.log10(P[:, (f >= 120) & (f < 5000)].sum(axis=1) + 1e-12)
    tot = 10 * np.log10(P[:, f >= 30].sum(axis=1) + 1e-12)
    # Skip fades: the first and last 10%.
    a, b = int(n * 0.1), int(n * 0.9)
    low, rest, tot = low[a:b], rest[a:b], tot[a:b]
    minutes = (b - a) * 0.5 / 60
    drop = (low < np.percentile(low, 75) - 12) & (rest > np.median(rest) - 12)
    # Only where the low end is normally there: kick or bass sounding in at
    # least 75% of frames. Sparse bass isn't a breakdown.
    out["low_on_share"] = round(float(1 - drop.mean()), 3)
    if drop.mean() > 0.25:
        out["low_drops_per_min"] = None
        out["rests_per_min"] = round(len(runs(tot < np.median(tot) - 30, 2)) / minutes, 2)
        return out
    d = runs(drop, 4)
    gap = tot < np.median(tot) - 30
    g = runs(gap, 2)
    out["low_drops_per_min"] = round(len(d) / minutes, 2)
    out["low_drop_len_s"] = round(float(np.median(d)) * 0.5, 1) if d else None
    out["low_drop_share"] = round(sum(d) / (b - a), 3)
    out["rests_per_min"] = round(len(g) / minutes, 2)
    return out


if __name__ == "__main__":
    outp, files = sys.argv[1], sys.argv[2:]
    done = set()
    if os.path.exists(outp):
        done = {json.loads(l)["file"] for l in open(outp) if l.strip()}
    with open(outp, "a") as fh:
        for p in files:
            if p in done:
                continue
            try:
                row = run(p)
            except Exception as ex:
                row = {"file": p, "error": repr(ex)}
            fh.write(json.dumps(row) + "\n")
            fh.flush()
```

## summarize2.py

Album table and pooled comparisons in batch one's definitions (set PILE2 to the new pile's album names).

```python
"""Batch-2 summary in the batch-1 report's terms.

Usage: python3 -I summarize2.py
Album table (medians per file; chords/min seconds-weighted), pooled figures
for the six differences for batch 1, pile 2 and Driftloom, side by side.
Definitions match docs/ears/batch-1.md: drift = median of per-file drift60/drift10;
"steady beat" = pulse >= 0.6; 1-5 kHz = b1000_2000 + b2000_5000.
"""
import json, glob
from collections import defaultdict
import numpy as np

PILE2 = {"Autechre - Amber", "Oneohtrix Point Never - R Plus Seven", "LFO - Frequencies",
         "The Field - From Here We Go Sublime", "Ryuichi Sakamoto - Thousand Knives", "Porter Ricks - Biokinetics",
         "Mike & Rich - Expert Knob Twiddlers", "Lone - Galaxy Garden", "Autechre - Incunabula",
         "Forest Swords - Engravings", "Death Grips - Fashion Week (Instrumentals)", "Blank Banshee - Blank Banshee 0"}


def rows(pats):
    out = []
    for pat in pats:
        for p in sorted(glob.glob(pat)):
            for l in open(p):
                d = json.loads(l)
                if "error" not in d and "skip" not in d:
                    out.append(d)
    return out


R = rows(["records*.jsonl"])
E = {d["file"]: d for d in rows(["extra*.jsonl"])}
D = rows(["driftloom.jsonl"])
DL = rows(["driftloom-long.jsonl"])
for d in R + D + DL:
    d.update({k: v for k, v in E.get(d["file"], {}).items() if k.startswith("b") or k == "upper_pcs"})


def med(rs, k, w=False):
    v = [(r[k], r.get("seconds", 1)) for r in rs if r.get(k) is not None]
    if not v:
        return float("nan")
    a = np.array([x for x, _ in v], float)
    if not w:
        return float(np.median(a))
    ws = np.array([s for _, s in v], float)
    o = np.argsort(a)
    return float(a[o][np.searchsorted(np.cumsum(ws[o]) / ws.sum(), 0.5)])


def ratio(rs):
    v = [r["drift60"] / r["drift10"] for r in rs if r.get("drift60") and r.get("drift10")]
    return float(np.median(v)) if v else float("nan")


def k15(r):
    return r["b1000_2000"] + r["b2000_5000"] if "b1000_2000" in r else None


for r in R + D + DL:
    r["k15"] = k15(r)
    r["above5k"] = r.get("b5000_11025")

b1 = [r for r in R if r["album"] not in PILE2]
p2 = [r for r in R if r["album"] in PILE2]

print(f"batch 1 files {len(b1)}, pile 2 files {len(p2)}, with extras {sum('b40_60' in r for r in p2)}")
print()
print("| Album | n | Width (dB) | 40–60 Hz % | 60–120 % | 120–250 % | 1–5 kHz % | >5 kHz % | Chords/min | Tail (dB/s) | Tuning (cents) | Drift 60/10 | Pulse | Notes/s | Upper pcs | Wobble |")
print("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
groups = defaultdict(list)
for r in p2:
    groups[r["album"]].append(r)
for a in sorted(groups):
    rs = groups[a]
    print(f"| {a} | {len(rs)} | {med(rs,'width_db'):.0f} | {100*med(rs,'b40_60'):.1f} | {100*med(rs,'b60_120'):.0f} | "
          f"{100*med(rs,'b120_250'):.0f} | {100*med(rs,'k15'):.1f} | {100*med(rs,'above5k'):.1f} | {med(rs,'chord_changes_per_min'):.1f} | "
          f"{med(rs,'tail_db_per_s'):.1f} | {med(rs,'tuning_cents'):+.0f} | {ratio(rs):.2f} | {med(rs,'pulse'):.2f} | "
          f"{med(rs,'onsets_per_s',True):.1f} | {med(rs,'upper_pcs'):.0f} | {med(rs,'wobble_cents'):.1f} |")

print()
print("Pooled (median per file)")
hdr = ["set", "n", "width", "lra", "tail", "chords(w)", "tuning", "|tun|>=8 share", "drift60/10", "repeat", "pulse", "notes/s", "upper_pcs", "wobble", "1-5k%", "40-60%", "arc_db", "sweep_oct"]
print(" | ".join(hdr))
def pooled(name, rs, drs=None):
    t = [r["tuning_cents"] for r in rs if r.get("tuning_cents") is not None]
    vals = [name, len(rs), med(rs, "width_db"), med(rs, "lra"), med(rs, "tail_db_per_s"), med(rs, "chord_changes_per_min"),
            med(rs, "tuning_cents"), np.mean([abs(x) >= 8 for x in t]) if t else float("nan"), ratio(drs or rs), med(rs, "repeat"),
            med(rs, "pulse"), med(rs, "onsets_per_s", True), med(rs, "upper_pcs"), med(rs, "wobble_cents"),
            100 * med(rs, "k15"), 100 * med(rs, "b40_60"), med(rs, "arc_db"), med(rs, "sweep_oct")]
    print(" | ".join(f"{v:.2f}" if isinstance(v, float) else str(v) for v in vals))
pooled("batch1", b1)
pooled("pile2", p2)
pooled("driftloom60", D, DL)
pooled("driftloom150", DL)

print()
print("Band shares by beat (median %), pulse >= 0.6 = steady beat")
bands = ["b0_40", "b40_60", "b60_120", "b120_250", "b250_1000", "b1000_2000", "b2000_5000", "b5000_11025"]
print("set | n | " + " | ".join(bands))
KIT = {x["file"]: x["meta"].get("kit") for x in json.load(open("driftloom/index.json"))}
for d in D:
    d["has_drums"] = KIT.get(d["track"], KIT.get(d["file"].split("/")[-1])) != "none"
for name, rs in [("batch1", b1), ("pile2", p2), ("driftloom", D)]:
    for beat in (False, True):
        if name == "driftloom":
            s = [r for r in rs if r["has_drums"] == beat and "b40_60" in r]
        else:
            s = [r for r in rs if r.get("pulse") is not None and (r["pulse"] >= 0.6) == beat and "b40_60" in r]
        print(f"{name} {'beat' if beat else 'nobeat'} | {len(s)} | " + " | ".join(f"{100*med(s,b):.1f}" for b in bands))

print()
print("Per-album album-level medians, both batches together: how many albums in each range")
allg = defaultdict(list)
for r in R:
    allg[r["album"]].append(r)
def count(fn, label):
    a1 = [a for a in allg if a not in PILE2 and fn(allg[a])]
    a2 = [a for a in allg if a in PILE2 and fn(allg[a])]
    print(f"{label}: batch1 {len(a1)}/{sum(a not in PILE2 for a in allg)}, pile2 {len(a2)}/{sum(a in PILE2 for a in allg)}  {a2}")
count(lambda rs: med(rs, "chord_changes_per_min") < 1, "chords <1/min")
count(lambda rs: med(rs, "chord_changes_per_min") < 2, "chords <2/min")
count(lambda rs: abs(med(rs, "tuning_cents")) >= 8, "|tuning| >= 8")
count(lambda rs: ratio(rs) > 1, "drift ratio > 1")
count(lambda rs: -2.4 <= med(rs, "tail_db_per_s") <= -0.3, "tail 0.3-2.4 dB/s")
count(lambda rs: med(rs, "width_db") > -15, "width wider than -15")
```

## spec.py

A 60-s log-frequency spectrogram for looking at by eye.

```python
"""Draw a 60-s spectrogram (log frequency, 30 Hz-8 kHz) for looking at by eye.

Usage: python3 -I spec.py START_SECONDS OUT.png FILE
"""
import subprocess, sys
import numpy as np
import librosa
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

start, outp, path = float(sys.argv[1]), sys.argv[2], sys.argv[3]
SR = 22050
raw = subprocess.run(["ffmpeg", "-nostdin", "-v", "error", "-ss", str(start), "-t", "60", "-i", path,
                      "-f", "f32le", "-ac", "1", "-ar", str(SR), "-"], capture_output=True, check=True).stdout
y = np.frombuffer(raw, np.float32)
S = librosa.amplitude_to_db(np.abs(librosa.stft(y, n_fft=4096, hop_length=512)), ref=np.max)
f = librosa.fft_frequencies(sr=SR, n_fft=4096)
t = np.arange(S.shape[1]) * 512 / SR
keep = (f >= 30) & (f <= 8000)
fig, ax = plt.subplots(figsize=(12, 5), dpi=90)
ax.pcolormesh(t, f[keep], S[keep], shading="auto", vmin=-80, vmax=0, cmap="magma")
ax.set_yscale("log")
ax.set_yticks([40, 60, 120, 250, 500, 1000, 2000, 4000, 8000])
ax.set_yticklabels(["40", "60", "120", "250", "500", "1k", "2k", "4k", "8k"])
ax.set_xlabel(f"seconds from {start:.0f}")
ax.set_title(path.split("/")[-1][:80])
fig.tight_layout()
fig.savefig(outp)
```

## render-proto.mjs

The Driftloom renderer with prototype switches: `PROTO` (JSON for globalThis.PROTO), `PICK` (corpus indices) and `TAG` (file suffix). Used for the before/after clips in `docs/ears/prototypes.md`.

```js
// Render Driftloom loops to WAV files, the way the app plays them:
// the real Engine and Synth (JavaScript engine, full quality) through an
// OfflineAudioContext in headless Chromium, with "Let the loop wander" on
// at its default amount. Lives outside the repo; reads the repo's modules.
//
// Usage: PROTO='{"stereo":true}' PICK=8,9 TAG=after node render-proto.mjs REPO OUTDIR N SECONDS SEED
import fs from 'node:fs';
import http from 'node:http';
import path from 'node:path';
import { createRequire } from 'node:module';
import { execSync } from 'node:child_process';

const [REPO, OUT, N = '60', SECONDS = '60', SEED = '20261009'] = process.argv.slice(2);
const ROOT = path.resolve(REPO);
fs.mkdirSync(OUT, { recursive: true });
const RATE = 44100;

const PAGE = `<!doctype html><meta charset="utf-8"><script type="module">
import { Engine } from '/js/engine.js';
import { Synth } from '/js/synth.js';
import { newSpec, render } from '/js/generator.js';
import { Rng, mulberry32 } from '/js/rng.js';

function leadOf(spec) {
  return Object.entries(spec.mix || {}).sort((a, b) => b[1] - a[1])[0]?.[0] || '?';
}
window.specsFor = (seed, n) => {
  const master = new Rng(seed);
  return Array.from({ length: n }, () => newSpec(master.seed32()));
};
window.renderOne = async (spec, seconds, rate) => {
  const ctx = new OfflineAudioContext(2, Math.ceil(seconds * rate), rate);
  Math.random = mulberry32(spec.seed >>> 0 || 1);
  const synth = new Synth(ctx, 'full');
  const engine = new Engine(ctx, synth);
  if (engine.clock.worker) engine.clock.worker.terminate();
  engine.driftOn = true;
  engine.driftAmount = 1;
  engine.driftRng = new Rng((spec.seed ^ 0x5eed1e55) >>> 0);
  engine.load(spec);
  engine.playing = true;
  engine.nextStepTime = 0.05;
  let guard = 0;
  while (engine.nextStepTime < seconds && guard++ < 400000) {
    engine._scheduleStep(engine.step, engine.nextStepTime);
    engine._advance();
  }
  const buf = await ctx.startRendering();
  const L = buf.getChannelData(0), R = buf.getChannelData(1);
  const pcm = new Int16Array(L.length * 2);
  for (let i = 0; i < L.length; i++) {
    pcm[2 * i] = Math.max(-32768, Math.min(32767, Math.round(L[i] * 32767)));
    pcm[2 * i + 1] = Math.max(-32768, Math.min(32767, Math.round(R[i] * 32767)));
  }
  let s = '';
  const bytes = new Uint8Array(pcm.buffer);
  for (let i = 0; i < bytes.length; i += 0x8000) s += String.fromCharCode.apply(null, bytes.subarray(i, i + 0x8000));
  const r = render(spec);
  const meta = r.meta || {};
  return {
    b64: btoa(s), lead: leadOf(spec), mix: spec.mix, seed: spec.seed,
    bpm: spec.bpm ?? null,
    mood: spec.mood ?? null, meta, bars: r.totalSteps ? r.totalSteps / 16 : null,
  };
};
window.setProto = (p) => { globalThis.PROTO = p; };
window.ready = true;
</script>`;

const TYPES = { '.js': 'text/javascript', '.mjs': 'text/javascript', '.wasm': 'application/wasm', '.json': 'application/json', '.html': 'text/html' };
const server = http.createServer((req, res) => {
  const url = decodeURIComponent(req.url.split('?')[0]);
  if (url === '/__render.html') { res.writeHead(200, { 'content-type': 'text/html' }); return res.end(PAGE); }
  const file = path.join(ROOT, path.normalize(url));
  if (!file.startsWith(ROOT) || !fs.existsSync(file) || fs.statSync(file).isDirectory()) { res.writeHead(404); return res.end(); }
  res.writeHead(200, { 'content-type': TYPES[path.extname(file)] || 'application/octet-stream' });
  fs.createReadStream(file).pipe(res);
});
await new Promise((r) => server.listen(0, '127.0.0.1', r));
const port = server.address().port;

const require = createRequire(import.meta.url);
const globalRoot = execSync('npm root -g').toString().trim();
const { chromium } = require(path.join(globalRoot, 'playwright'));
const browser = await chromium.launch({ args: ['--no-sandbox'] });
const page = await browser.newPage();
page.on('pageerror', (e) => console.error('page error:', e.message));
await page.goto(`http://127.0.0.1:${port}/__render.html`);
await page.waitForFunction(() => window.ready === true, null, { timeout: 20000 });

const specs = await page.evaluate(([s, n]) => window.specsFor(s, n), [Number(SEED), Number(N)]);
const index = [];
const PICK = (process.env.PICK || '').split(',').filter(Boolean).map(Number);
const TAG = process.env.TAG || 'x';
await page.evaluate((p) => window.setProto(p), JSON.parse(process.env.PROTO || '{}'));
for (let i = 0; i < specs.length; i++) {
  if (PICK.length && !PICK.includes(i)) continue;
  const r = await page.evaluate(([spec, sec, rate]) => window.renderOne(spec, sec, rate), [specs[i], Number(SECONDS), RATE]);
  const pcm = Buffer.from(r.b64, 'base64');
  const header = Buffer.alloc(44);
  header.write('RIFF', 0); header.writeUInt32LE(36 + pcm.length, 4); header.write('WAVE', 8);
  header.write('fmt ', 12); header.writeUInt32LE(16, 16); header.writeUInt16LE(1, 20); header.writeUInt16LE(2, 22);
  header.writeUInt32LE(RATE, 24); header.writeUInt32LE(RATE * 4, 28); header.writeUInt16LE(4, 32); header.writeUInt16LE(16, 34);
  header.write('data', 36); header.writeUInt32LE(pcm.length, 40);
  const name = `${String(i).padStart(3, '0')}-${r.lead}-${TAG}.wav`;
  fs.writeFileSync(path.join(OUT, name), Buffer.concat([header, pcm]));
  delete r.b64;
  index.push({ file: name, ...r });
  console.log(name, r.bpm ?? '', r.mood ?? '');
}
fs.writeFileSync(path.join(OUT, `index-${TAG}.json`), JSON.stringify(index, null, 1));
await browser.close();
server.close();
```
