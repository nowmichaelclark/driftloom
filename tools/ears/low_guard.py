"""The low-end guard and a sketch's level by ear, against the same loop without it.

Usage: python3 -I low_guard.py INDEX.json WAVDIR [OUT.json] [--from S]

For every render with a 'none' render of the same loop (render2.mjs seeds the
synth's randomness, so the two differ only by the sketch):
  sub, low      share of the energy at 20-60 Hz and 60-120 Hz, none -> render
                (Ears 1's yardsticks: drum loops under ~35% at 60-120 Hz,
                drumless loops under ~15% at 40-60 Hz)
  dLU           integrated loudness (K-weighted) of the render minus none's
  partLU        loudness of the sketch alone (render minus none) against none,
                LU: how loud the part reads by ear, not by energy (Ears 4: a
                darker part reads quieter at the same energy)
  peak3s        the loudest 3 s window of the part against the same window of
                the loop, LU, over windows where the loop is within 20 LU of
                its median (an intro or a stop would otherwise tower)
  sub_peak      the loudest 3 s window's 20-60 Hz level, render against none, dB
  phone         share of the part's energy above 150 Hz (what a phone speaker
                can play; a sub-only part reads as nothing there)
--from S skips the first S seconds (to leave out an intro both renders share).
"""
import json, os, sys
import numpy as np
import soundfile as sf
import pyloudnorm as pyln
from scipy.signal import butter, sosfilt

args = [a for a in sys.argv[1:] if not a.startswith('--')]
skip = 0.0
if '--from' in sys.argv:
    skip = float(sys.argv[sys.argv.index('--from') + 1])
    args = [a for a in args if a != sys.argv[sys.argv.index('--from') + 1]]
idx = json.load(open(args[0]))
wavdir = args[1]
RATE = 44100
meter = pyln.Meter(RATE)


def mono(path):
    x, sr = sf.read(path, dtype='float64', always_2d=True)
    return x.mean(axis=1)[int(skip * RATE):]


def bandshare(x, lo, hi):
    sos = butter(4, [lo, hi], btype='band', fs=RATE, output='sos')
    return float(np.sum(sosfilt(sos, x) ** 2) / (np.sum(x ** 2) + 1e-20))


def lufs(x):
    try:
        v = meter.integrated_loudness(np.stack([x, x], axis=1))
    except ValueError:
        return -99.0
    return float(v) if np.isfinite(v) else -99.0


def win_lufs(x, w=3.0, hop=1.0):
    n, h = int(w * RATE), int(hop * RATE)
    return [lufs(x[i:i + n]) for i in range(0, len(x) - n + 1, h)]


by = {}
for r in idx:
    by.setdefault(r['pick'], {})[r['tag']] = r
rows = []
for pick, g in sorted(by.items()):
    if 'none' not in g:
        continue
    a = mono(os.path.join(wavdir, g['none']['file']))
    for tag, r in sorted(g.items()):
        if tag == 'none' or (r.get('then') is not None) != (g['none'].get('then') is not None):
            continue
        b = mono(os.path.join(wavdir, r['file']))
        n = min(len(a), len(b))
        x, y = a[:n], b[:n]
        d = y - x
        lw, dw = win_lufs(x), win_lufs(d)
        med = np.median([v for v in lw if v > -90]) if lw else -99
        ok = [i for i, v in enumerate(lw) if v > med - 20 and dw[i] > -90]
        peak = max((dw[i] - lw[i] for i in ok), default=None)
        sos_sub = butter(4, [20, 60], btype='band', fs=RATE, output='sos')
        sx, sy = sosfilt(sos_sub, x), sosfilt(sos_sub, y)
        wn = 3 * RATE
        subp = max((10 * np.log10((np.sum(sy[i:i + wn] ** 2) + 1e-20) / (np.sum(sx[i:i + wn] ** 2) + 1e-20))
                    for i in range(0, n - wn + 1, RATE)), default=None)
        hp = butter(4, 150, btype='high', fs=RATE, output='sos')
        row = {
            'pick': pick, 'lead': r.get('lead'), 'tag': tag,
            'sub': [round(bandshare(x, 20, 60), 3), round(bandshare(y, 20, 60), 3)],
            'low': [round(bandshare(x, 60, 120), 3), round(bandshare(y, 60, 120), 3)],
            'dLU': round(lufs(y) - lufs(x), 2),
            'partLU': round(lufs(d) - lufs(x), 1),
            'peak3s': None if peak is None else round(peak, 1),
            'sub_peak': None if subp is None else round(float(subp), 1),
            'phone': round(float(np.sum(sosfilt(hp, d) ** 2) / (np.sum(d ** 2) + 1e-20)), 2),
        }
        rows.append(row)
        print(f"#{pick:<3} {str(r.get('lead')):9} {tag:12} sub {row['sub'][0]:.1%}->{row['sub'][1]:.1%}  "
              f"60-120 {row['low'][0]:.1%}->{row['low'][1]:.1%}  dLU {row['dLU']:+.2f}  part {row['partLU']:+.1f} LU  "
              f"peak3s {row['peak3s']}  subpeak {row['sub_peak']} dB  phone {row['phone']:.0%}")
if len(args) > 2:
    json.dump(rows, open(args[2], 'w'), indent=1)
