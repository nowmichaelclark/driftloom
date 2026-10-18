#!/usr/bin/env python3
"""Spot checks on one track's stems, for the numbers a card leans on.

    python3 -I check_stems.py WORKDIR/<album>/stems/<track> [--drop-db 15]

Run it on each deep-pass track after `album.py`, before writing the card.
It prints what the card tools cannot see from one stem at a time:

  stems     RMS of drums, bass, other, vocals and each one's share of the
            power: a stem under about 2% is empty, so its bass behavior,
            pumping and wobble numbers are void; drums and bass both empty
            means the track has no beat
  bands     share of energy under 120 Hz, 120-500, 500-2k, 2-5k, above 5k,
            for the sum of the stems and for each stem (shows where the
            separator put the sub)
  lines     the strongest spectral lines 20-200 Hz of bass and other: a
            note group whose register is 1/2 or 1/3 of the strongest line
            is a subharmonic misread
  grid      autocorrelation peaks of the drums-stem onsets (seconds), and
            the BPM that a 4-beat bar of the strongest peak implies: checks
            the tempo octave the meter tool reports
  floor     runs of 2 s or more where the low end (under 120 Hz, drums +
            bass + other summed) is DROP_DB below its 90th percentile:
            floor drops that the stems-only count misses when the separator
            parks the sub in another stem
"""
import argparse
import glob
import os
import sys

import numpy as np
import soundfile as sf
from scipy.signal import butter, find_peaks, sosfilt

STEMS = ("drums", "bass", "other", "vocals")
BANDS = {"<120": (0, 120), "120-500": (120, 500), "0.5-2k": (500, 2000),
         "2-5k": (2000, 5000), ">5k": (5000, 30000)}


def load(folder, name):
    paths = glob.glob(os.path.join(folder, name + ".wav"))
    if not paths:
        return None, None
    x, sr = sf.read(paths[0])
    return (x.mean(1) if x.ndim > 1 else x), sr


def fmt(t):
    return "%d:%04.1f" % (int(t // 60), t % 60)


def band_shares(x, sr):
    spec = np.abs(np.fft.rfft(x)) ** 2
    f = np.fft.rfftfreq(len(x), 1 / sr)
    tot = spec.sum() + 1e-30
    return {k: round(float(spec[(f >= a) & (f < b)].sum() / tot), 3)
            for k, (a, b) in BANDS.items()}


def lines(x, sr, lo=20, hi=200, n=6):
    spec = np.abs(np.fft.rfft(x * np.hanning(len(x))))
    f = np.fft.rfftfreq(len(x), 1 / sr)
    m = (f >= lo) & (f <= hi)
    fm, sm = f[m], spec[m]
    idx, _ = find_peaks(sm, distance=max(1, int(len(fm) / 60)))
    idx = idx[np.argsort(sm[idx])[::-1][:n]]
    return [(round(float(fm[j]), 1), round(float(20 * np.log10(sm[j] / sm.max())), 1))
            for j in sorted(idx)]


def drum_grid(x, sr):
    import librosa
    env = librosa.onset.onset_strength(y=x, sr=sr, hop_length=512)
    fps = sr / 512
    ac = librosa.autocorrelate(env - env.mean())
    lags = np.arange(len(ac)) / fps
    m = (lags > 0.12) & (lags < 2.5)
    idx, _ = find_peaks(ac[m])
    idx = idx[np.argsort(ac[m][idx])[::-1][:6]]
    return [(round(float(lags[m][j]), 3), round(float(ac[m][j] / ac[0]), 2))
            for j in sorted(idx)]


def low_runs(mix, sr, drop_db, min_s=2.0):
    low = sosfilt(butter(4, 120, "low", fs=sr, output="sos"), mix)
    n = sr // 2
    k = len(low) // n
    db = 20 * np.log10(np.sqrt((low[:k * n].reshape(k, n) ** 2).mean(1)) + 1e-9)
    ref = float(np.percentile(db, 90))
    off = db < ref - drop_db
    runs, s = [], None
    for i, v in enumerate(list(off) + [False]):
        if v and s is None:
            s = i
        if not v and s is not None:
            if (i - s) / 2 >= min_s:
                runs.append((fmt(s / 2), (i - s) / 2))
            s = None
    return ref, float(1 - off.mean()), runs


def report(stems_dir, drop_db=15.0):
    """Run every check on one track's stems. Returns {"text": printable report,
    "shares": {stem: share of power}, "flags": [plain-words warnings],
    "grid": [(lag s, strength)], "grid_bpm_4beat": float or None, "runs": [(time, s)]}."""
    audio, sr = {}, None
    for s in STEMS:
        x, r = load(stems_dir, s)
        if x is not None:
            audio[s], sr = x, r
    if not audio:
        raise FileNotFoundError("no stems found in " + stems_dir)

    out, flags = [], []
    power = {s: float((x ** 2).mean()) for s, x in audio.items()}
    tot = sum(power.values()) + 1e-30
    shares = {s: p / tot for s, p in power.items()}
    out.append("stems (rms, share of power):")
    for s, p in power.items():
        out.append("  %-7s %.4f  %5.1f%%" % (s, p ** 0.5, 100 * p / tot))
    if "drums" in power and "bass" in power and (power["drums"] + power["bass"]) / tot < 0.02:
        out.append("  -> drums and bass are empty: no beat; meter, swing and "
                   "chord-on-beat numbers are meaningless")
        flags.append("drums and bass are empty: no beat; meter, swing and on-beat numbers are void")
    elif "bass" in power and shares["bass"] < 0.02:
        out.append("  -> bass stem is near empty: bass behavior, bass pumping and "
                   "bass wobble are void")
        flags.append("bass stem is near empty (%.1f%%): bass notes, pumping and wobble are void" % (100 * shares["bass"]))
    elif "bass" in power and shares["bass"] < 0.05:
        flags.append("bass stem is thin (%.1f%%): bass notes, pumping and wobble are guesses" % (100 * shares["bass"]))
    if "other" in power and shares["other"] < 0.05:
        flags.append("other stem holds %.1f%% of the power: top line, chords, other-stem pumping and echoes are void or guesses" % (100 * shares["other"]))
    if "vocals" in power and shares["vocals"] < 0.06:
        flags.append("vocals stem holds %.1f%%: bleed on an instrumental, not a voice" % (100 * shares["vocals"]))

    mix = sum(audio[s] for s in ("drums", "bass", "other") if s in audio)
    out.append("bands (share of energy), sum of drums+bass+other: %s" % band_shares(mix, sr))
    for s in audio:
        out.append("  %-7s %s" % (s, band_shares(audio[s], sr)))

    for s in ("bass", "other"):
        if s in audio:
            out.append("lines 20-200 Hz, %s stem (Hz, dB from the strongest): %s" % (s, lines(audio[s], sr)))

    g, bpm4, beat_bpm = [], None, None
    if "drums" in audio and shares["drums"] > 0.02:
        g = drum_grid(audio["drums"], sr)
        out.append("drum-onset repeats (s, strength): %s" % (g,))
        best = max(g, key=lambda p: p[1])
        bpm4 = 4 * 60 / best[0]
        strong = [lag for lag, v in g if v >= 0.75 * best[1] and lag >= 0.25]
        beat_bpm = 60 / min(strong) if strong else None
        out.append("  strongest %.3f s -> a 4-beat bar means %.1f BPM, a 2-beat bar "
                   "%.1f BPM; compare with the meter tool's BPM (it may be half)"
                   % (best[0], bpm4, 2 * 60 / best[0]))
        if beat_bpm:
            out.append("  shortest strong repeat (at least 75%% of the strongest) %.3f s -> a beat of %.1f BPM "
                       "(this is the reading the tempo note on the card uses)" % (min(strong), beat_bpm))

    ref, on, runs = low_runs(mix, sr, drop_db)
    out.append("floor: low end under 120 Hz, all stems summed; 90th percentile "
               "%.1f dB, on %.0f%% of the time" % (ref, 100 * on))
    out.append("  runs of 2 s or more, %.0f dB under: %s" %
               (drop_db, ", ".join("%s for %.1f s" % r for r in runs) or "none"))
    out.append("  (compare with techniques.json; the stems-only count misses drops "
               "when the separator parks the sub in 'other'; the first and last "
               "seconds of a file are intro and tail, not drops)")
    return {"text": "\n".join(out), "shares": shares, "flags": flags, "grid": g,
            "grid_bpm_4beat": bpm4, "grid_bpm_beat": beat_bpm, "runs": runs}


def tempo_note(meter_bpm, grid_bpm):
    """Plain words on how the meter tool's BPM sits against the drum-grid BPM."""
    if not meter_bpm or not grid_bpm:
        return None
    r = meter_bpm / grid_bpm
    for target, word in ((1.0, "agrees with"), (0.5, "is half of"), (2.0, "is double"),
                         (2 / 3, "is two thirds of"), (1.5, "is one and a half times"),
                         (0.75, "is three quarters of"), (4 / 3, "is four thirds of")):
        if abs(r - target) < 0.06 * target:
            return "meter tool %.0f BPM %s the drum-grid beat of %.1f BPM (use the grid; if it is off by a simple ratio, check by ear)" % (
                meter_bpm, word, grid_bpm) if word != "agrees with" else \
                "meter tool %.0f BPM agrees with the drum-grid beat of %.1f BPM" % (meter_bpm, grid_bpm)
    return "meter tool %.0f BPM does not match the drum-grid beat of %.1f BPM by a simple ratio: listen" % (meter_bpm, grid_bpm)


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("stems_dir", help="WORKDIR/<album>/stems/<track>")
    ap.add_argument("--drop-db", type=float, default=15.0,
                    help="how far under the 90th percentile counts as out (default 15)")
    a = ap.parse_args()
    try:
        r = report(a.stems_dir, a.drop_db)
    except FileNotFoundError as e:
        sys.exit(str(e))
    print(r["text"])


if __name__ == "__main__":
    main()
