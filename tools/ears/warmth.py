"""How warm a stab is, measured the same way on a record and on a sketch.

Usage: python3 -I warmth.py REF.wav [NONE.wav SKETCH.wav LABEL]...
For the record, the whole track above 150 Hz; for a sketch, the difference
between a render with it and the same loop without it (the stab and its
echo alone). Reported over 150 Hz-4 kHz, where a chord stab lives (hats
and air above 4 kHz are left out, so a record's cymbals don't count):
  centroid   spectral centroid, Hz (lower is darker)
  hi/mid     energy at 2-4 kHz against 250 Hz-1 kHz, dB (the "piercing" band
             against the body)
  1-2k/mid   energy at 1-2 kHz against 250 Hz-1 kHz, dB
  rise       median 10-90% rise time of the loudest onsets, ms (a softer
             attack is a longer rise)
"""
import sys
import numpy as np
import soundfile as sf
from scipy.signal import butter, sosfilt, welch

RATE = 44100

def load(f):
    x, sr = sf.read(f, dtype='float64')
    if x.ndim > 1:
        x = x.mean(axis=1)
    assert sr == RATE, sr
    return x

def measure(x, label):
    x = sosfilt(butter(4, [150, 4000], btype='band', fs=RATE, output='sos'), x)
    f, p = welch(x, fs=RATE, nperseg=8192)
    sel = (f >= 150) & (f <= 4000)
    cen = np.sum(f[sel] * p[sel]) / np.sum(p[sel])
    e = lambda lo, hi: np.sum(p[(f >= lo) & (f < hi)])
    hm = 10 * np.log10(e(2000, 4000) / e(250, 1000))
    km = 10 * np.log10(e(1000, 2000) / e(250, 1000))
    # Rise times: envelope at 1 ms, the 30 loudest peaks at least 150 ms apart.
    hop = 44
    env = np.sqrt(np.convolve(x ** 2, np.ones(hop) / hop, mode='same'))[::hop]
    order = np.argsort(env)[::-1]
    taken, rises = [], []
    for i in order:
        if len(taken) >= 30:
            break
        if any(abs(i - j) < 150 for j in taken):
            continue
        taken.append(i)
        pk = env[i]
        k = i
        while k > 0 and env[k] > 0.9 * pk:
            k -= 1
        hi = k
        while k > 0 and env[k] > 0.1 * pk and i - k < 200:
            k -= 1
        rises.append(hi - k)
    print(f"{label:28} centroid {cen:6.0f} Hz   hi/mid {hm:6.1f} dB   1-2k/mid {km:6.1f} dB   rise {np.median(rises):4.0f} ms")
    return {'label': label, 'centroid': round(cen), 'hi_mid': round(hm, 1), 'k_mid': round(km, 1), 'rise_ms': float(np.median(rises))}

if __name__ == '__main__':
    a = sys.argv[1:]
    measure(load(a[0]), 'reference')
    for i in range(1, len(a), 3):
        nb, ns = load(a[i]), load(a[i + 1])
        n = min(len(nb), len(ns))
        measure(ns[:n] - nb[:n], a[i + 2])
