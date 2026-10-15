"""A width ceiling, sketched offline: the side signal (L - R) / 2 is turned
down wherever it rises above CEIL dB against the mid, measured above 200 Hz
over 2-second windows; the mid is untouched, so the fold to mono is exactly
the input's. A core block could do the same, deterministically.

Usage: python3 -I width_ceiling.py INDEX.json WAVDIR CEIL_DB TAG[,TAG...]
Writes <file>-C.wav next to each input and INDEX-ceiling.json.
"""
import json, os, sys
import numpy as np
import soundfile as sf
from scipy.signal import butter, sosfilt, lfilter

idx = json.load(open(sys.argv[1]))
wavdir, ceil = sys.argv[2], float(sys.argv[3])
tags = sys.argv[4].split(',')
RATE = 44100
HOP = 441  # 10 ms
hp = butter(4, 200, btype='high', fs=RATE, output='sos')

def smooth_energy(x, win_s):
    e = np.add.reduceat(x ** 2, np.arange(0, len(x), HOP)) / HOP
    k = max(1, int(win_s * RATE / HOP))
    return np.convolve(e, np.ones(k) / k, mode='same')

out = []
for r in idx:
    if r['tag'] not in tags:
        continue
    x, _ = sf.read(os.path.join(wavdir, r['file']), dtype='float64')
    L, R = x[:, 0], x[:, 1]
    M, S = (L + R) / 2, (L - R) / 2
    em, es = smooth_energy(sosfilt(hp, M), 2.0), smooth_energy(sosfilt(hp, S), 2.0)
    target = np.minimum(1.0, np.sqrt(10 ** (ceil / 10) * em / np.maximum(es, 1e-20)))
    # Down in about half a second, back up over about three.
    g = np.empty_like(target)
    prev = 1.0
    for i, t in enumerate(target):
        a = 0.98 if t < prev else 0.9967
        prev = a * prev + (1 - a) * t
        g[i] = prev
    gs = np.interp(np.arange(len(S)), np.arange(len(g)) * HOP, g)
    S2 = S * gs
    y = np.stack([M + S2, M - S2], axis=1)
    name = r['file'].replace('.wav', '-C.wav')
    sf.write(os.path.join(wavdir, name), y.astype(np.float32), RATE, subtype='FLOAT')
    out.append({**r, 'file': name, 'tag': r['tag'] + 'C', 'ceiling_min_gain_dB': round(20 * np.log10(max(g.min(), 1e-6)), 1)})
    print(name, 'deepest cut %.1f dB' % out[-1]['ceiling_min_gain_dB'])
json.dump(idx + out, open(sys.argv[1].replace('.json', '-ceiling.json'), 'w'))
