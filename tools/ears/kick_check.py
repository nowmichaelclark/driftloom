"""Kick pitch and decay, and how the bass moves around the kick, from one track's stems.
Ears 8 (2026-10-18): written to check the card numbers the first synthesis built
on, where the card tools flag them (pitch under 60 Hz, pumping that reads about
70% of the true depth or locks onto the wrong period, attacks under 60 Hz).

Usage: python3 -I kick_check.py kick  STEMDIR [T0 T1]
       python3 -I kick_check.py duck  STEMDIR
       python3 -I kick_check.py swell STEMDIR T0 T1
STEMDIR holds drums.wav and bass.wav (split.py's output for one track).

kick   every kick (an onset of 6 dB or more in the 30-150 Hz band of the drums
       stem, within 12 dB of its loudest): the strongest 30-130 Hz line 40-160 ms
       after the onset (the kick's note, after its pitch sweep), the notes it
       lands on, and how fast the low band falls from its peak to 30 dB down (dB/s)
duck   the bass stem's 30-150 Hz level against time since each kick, averaged
       over every kick: a dip right after the kick that recovers is a duck; the
       bass lowest at the kick and highest between kicks is a bass that plays
       off the beat (the two take turns); the depth here is the stem's own, not
       the 70% floor techniques.py reports
swell  bass-note rise times (10% to 90%) in a window; a duck's recovery reads
       as a slow rise, so read it next to `duck` (Ears 8: a plucked sine bass
       read 30 ms; Joyrex J4 20 to 640 ms with a duck under it)
"""
import sys
import numpy as np
import soundfile as sf
from scipy.signal import butter, sosfiltfilt, find_peaks

RATE = 44100


def load(path):
    x, sr = sf.read(path, always_2d=True)
    x = x.mean(axis=1)
    assert sr == RATE, sr
    return x


def band(x, lo, hi):
    sos = butter(4, [lo, hi], btype='band', fs=RATE, output='sos')
    return sosfiltfilt(sos, x)


def env_db(x, hop=0.005, win=0.02):
    h, w = int(hop * RATE), int(win * RATE)
    n = (len(x) - w) // h
    e = np.array([np.sqrt(np.mean(x[i * h:i * h + w] ** 2)) for i in range(n)])
    return 20 * np.log10(e + 1e-9), hop


def kick_onsets(drums, min_gap=0.2):
    low = band(drums, 30, 150)
    e, hop = env_db(low)
    d = np.diff(e, prepend=e[0])
    top = np.percentile(e, 95)
    pk, _ = find_peaks(d, height=6, distance=int(min_gap / hop))
    pk = [p for p in pk if e[min(len(e) - 1, p + 4)] > top - 12]
    return np.array(pk) * hop, low, e, hop


def kick(stemdir, t0=None, t1=None):
    drums = load(f'{stemdir}/drums.wav')
    on, low, e, hop = kick_onsets(drums)
    if t0 is not None:
        on = on[(on >= t0) & (on <= t1)]
    pitches, decays = [], []
    for t in on:
        a, b = int((t + 0.04) * RATE), int((t + 0.16) * RATE)
        if b > len(drums):
            continue
        seg = drums[a:b] * np.hanning(b - a)
        spec = np.abs(np.fft.rfft(seg, 1 << 16))
        f = np.fft.rfftfreq(1 << 16, 1 / RATE)
        m = (f >= 30) & (f <= 130)
        pitches.append(f[m][np.argmax(spec[m])])
        i = int(t / hop)
        seg_e = e[i:i + int(1.5 / hop)]
        if len(seg_e) < 10:
            continue
        p = int(np.argmax(seg_e[:int(0.08 / hop)]))
        below = np.where(seg_e[p:] < seg_e[p] - 30)[0]
        if len(below):
            decays.append(30 / (below[0] * hop))
    pitches, decays = np.array(pitches), np.array(decays)
    midi = 69 + 12 * np.log2(pitches / 440)
    names = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B']
    vals, counts = np.unique(np.round(midi).astype(int), return_counts=True)
    top = sorted(zip(counts, vals), reverse=True)[:4]
    print(f'{stemdir.split("/")[-1]}: {len(on)} kicks, pitch median {np.median(pitches):.1f} Hz '
          f'(10-90% {np.percentile(pitches, 10):.1f}-{np.percentile(pitches, 90):.1f}); notes '
          + ', '.join(f'{names[v % 12]}{v // 12 - 1} {c / len(midi):.0%}' for c, v in top)
          + f'; decay median {np.median(decays):.0f} dB/s (10-90% {np.percentile(decays, 10):.0f}-{np.percentile(decays, 90):.0f}); '
          f'kick spacing median {np.median(np.diff(on)):.3f} s')


def duck(stemdir):
    drums = load(f'{stemdir}/drums.wav')
    bass = load(f'{stemdir}/bass.wav')
    on, *_ = kick_onsets(drums)
    gap = np.median(np.diff(on))
    be, hop = env_db(band(bass, 30, 150))
    span = int(gap / hop)
    rows = []
    for t in on:
        i = int(t / hop)
        if i + span < len(be) and be[i:i + span].max() > np.percentile(be, 60):
            rows.append(be[i:i + span])
    m = np.mean(rows, axis=0)
    lo = int(np.argmin(m))
    print(f'{stemdir.split("/")[-1]}: {len(rows)} kicks every {gap:.3f} s; bass level after the kick: '
          f'at 0 ms {m[0] - m.max():+.1f} dB, lowest {m[lo] - m.max():+.1f} dB at {lo * hop * 1000:.0f} ms, '
          f'highest at {int(np.argmax(m)) * hop * 1000:.0f} ms; '
          + ' '.join(f'{k * 50}ms:{m[min(len(m) - 1, int(k * 0.05 / hop))] - m.max():+.0f}' for k in range(0, int(gap / 0.05) + 1)))


def swell(stemdir, t0, t1):
    bass = load(f'{stemdir}/bass.wav')
    x = band(bass, 30, 300)[int(t0 * RATE):int(t1 * RATE)]
    e, hop = env_db(x, hop=0.002, win=0.01)
    d = np.diff(e, prepend=e[0])
    pk, _ = find_peaks(d, height=1.5, distance=int(0.15 / hop))
    rises = []
    for p in pk:
        a = max(0, p - int(0.02 / hop))
        seg = e[a:a + int(0.8 / hop)]
        if len(seg) < 20:
            continue
        top = int(np.argmax(seg))
        lin = 10 ** (seg[:top + 1] / 20)
        if lin.max() <= 0 or seg[top] < np.percentile(e, 50):
            continue
        r10 = np.argmax(lin >= 0.1 * lin.max())
        r90 = np.argmax(lin >= 0.9 * lin.max())
        rises.append((t0 + a * hop, (r90 - r10) * hop * 1000))
    print(f'{stemdir.split("/")[-1]} {t0}-{t1} s: {len(rises)} notes; rise 10-90% ms: '
          + ', '.join(f'{t:.1f}s {r:.0f}' for t, r in rises[:14])
          + (f'; median {np.median([r for _, r in rises]):.0f} ms' if rises else ''))


if __name__ == '__main__':
    cmd, stemdir, *rest = sys.argv[1:]
    args = [float(v) for v in rest]
    {'kick': kick, 'duck': duck, 'swell': swell}[cmd](stemdir, *args)
