"""Stereo against mono: the guards in docs/CORE.md (promise 9), measured.

Usage: python3 -I stereo_guard.py INDEX.json WAVDIR OUT.json
For every loop rendered as 'mono' and as one or more space recipes:
  fold_dLU      loudness of (L+R)/2 minus the mono render's, in LU
  fold_resid    energy of (fold - mono) against mono, dB (how different the fold is)
  fold_bands    per-octave level of the fold minus mono, dB (63 Hz .. 8 kHz)
  st_dLU        loudness of the stereo render minus the mono render (as played on two speakers)
  side_mid      side energy against mid energy, dB (records: -12 to -5)
  low_side      side against mid below 120 Hz, dB (bass and kick centered)
  corr_med/p05  L-R correlation over 400 ms windows: median and 5th percentile
  balance       L minus R level over the whole render, dB
  bal_p95       95th percentile of |L - R| level over 2 s windows, dB
  cost          render wall time against mono (a rough proxy)
  bounce        how fast the image jumps: 90th percentile of the change in
                L-R level from one 100 ms window to the next, above 200 Hz, dB
                (round 1's ping-pong and per-hit placement read high here)
  drift_bal     how far the image travels slowly: the 5th-95th percentile
                range of L-R level over 4 s windows, dB
  drift_width   the same for side/mid above 200 Hz over 4 s windows, dB
"""
import json, os, sys
import numpy as np
import soundfile as sf
import pyloudnorm as pyln
from scipy.signal import butter, sosfilt, sosfiltfilt

idx = json.load(open(sys.argv[1]))
wavdir = sys.argv[2]
RATE = 44100
meter = pyln.Meter(RATE)
OCT = [63, 125, 250, 500, 1000, 2000, 4000, 8000]

def lp(x, f):
    # Steep and zero-phase, so side just above the cutoff doesn't leak in.
    return sosfiltfilt(butter(10, f, btype='low', fs=RATE, output='sos'), x)

def hp200(x):
    return sosfilt(butter(4, 200, btype='high', fs=RATE, output='sos'), x)

def octave(x, fc):
    lo, hi = fc / np.sqrt(2), min(fc * np.sqrt(2), RATE / 2 - 100)
    return sosfilt(butter(3, [lo, hi], btype='band', fs=RATE, output='sos'), x)

def db(a, b):
    return 10 * np.log10((a + 1e-20) / (b + 1e-20))

def load(f):
    x, _ = sf.read(os.path.join(wavdir, f), dtype='float64')
    return x

groups = {}
for r in idx:
    groups.setdefault(r['pick'], {})[r['tag']] = r

rows = []
for pick, g in groups.items():
    if 'mono' not in g:
        continue
    mono = load(g['mono']['file'])
    m = mono[:, 0]
    lm1 = meter.integrated_loudness(m)
    lm2 = meter.integrated_loudness(mono)
    for tag, r in g.items():
        if tag == 'mono':
            continue
        x = load(r['file'])
        n = min(len(x), len(m))
        L, R = x[:n, 0], x[:n, 1]
        mm = m[:n]
        fold = (L + R) / 2
        mid, side = fold, (L - R) / 2
        w = int(0.4 * RATE)
        cors = []
        for i in range(0, n - w, w):
            a, b = L[i:i + w], R[i:i + w]
            if np.sum(a * a) > 1e-9 and np.sum(b * b) > 1e-9:
                cors.append(np.corrcoef(a, b)[0, 1])
        w2 = 2 * RATE
        bals = []
        for i in range(0, n - w2, w2):
            a, b = np.sum(L[i:i + w2] ** 2), np.sum(R[i:i + w2] ** 2)
            if a > 1e-7 and b > 1e-7:
                bals.append(abs(db(a, b)))
        Lh, Rh = hp200(L), hp200(R)
        def lvl(x, w):
            k = int(w * RATE)
            m = len(x) // k
            return np.sum(x[: m * k].reshape(m, k) ** 2, axis=1)
        l1, r1 = lvl(Lh, 0.1), lvl(Rh, 0.1)
        ok1 = (l1 + r1) > np.median(l1 + r1) * 0.03
        b1 = db(l1, r1)
        jumps = np.abs(np.diff(b1))[(ok1[1:] & ok1[:-1])]
        l4, r4 = lvl(Lh, 4.0), lvl(Rh, 4.0)
        s4, m4 = lvl((Lh - Rh) / 2, 4.0), lvl((Lh + Rh) / 2, 4.0)
        ok4 = (l4 + r4) > np.median(l4 + r4) * 0.03
        b4, w4 = db(l4, r4)[ok4], db(s4, m4)[ok4]
        rows.append({
            'bounce': round(float(np.percentile(jumps, 90)), 2) if len(jumps) else None,
            'drift_bal': round(float(np.percentile(b4, 95) - np.percentile(b4, 5)), 2) if len(b4) > 2 else None,
            'drift_width': round(float(np.percentile(w4, 95) - np.percentile(w4, 5)), 2) if len(w4) > 2 else None,
            'pick': pick, 'lead': r['lead'], 'recipe': tag, 'space': r['space'],
            'fold_dLU': round(meter.integrated_loudness(fold) - lm1, 2),
            'fold_resid': round(db(np.sum((fold - mm) ** 2), np.sum(mm ** 2)), 1),
            'fold_bands': {fc: round(db(np.sum(octave(fold, fc) ** 2), np.sum(octave(mm, fc) ** 2)), 2) for fc in OCT},
            'st_dLU': round(meter.integrated_loudness(x[:n]) - lm2, 2),
            'side_mid': round(db(np.sum(side ** 2), np.sum(mid ** 2)), 1),
            'side_mid_hi': round(db(np.sum(hp200(side) ** 2), np.sum(hp200(mid) ** 2)), 1),
            'low_side': round(db(np.sum(lp(side, 120) ** 2), np.sum(lp(mid, 120) ** 2)), 1),
            'corr_med': round(float(np.median(cors)), 2) if cors else None,
            'corr_p05': round(float(np.percentile(cors, 5)), 2) if cors else None,
            'balance': round(db(np.sum(L ** 2), np.sum(R ** 2)), 2),
            'bal_p95': round(float(np.percentile(bals, 95)), 1) if bals else None,
            'cost': round(r['ms'] / g['mono']['ms'], 2),
            'refused': [g['mono']['refused'], r['refused']],
        })

json.dump(rows, open(sys.argv[3], 'w'), indent=1)
hdr = f"{'loop':14} {'recipe':11} {'fold dLU':>8} {'resid':>6} {'worst oct':>9} {'st dLU':>7} {'S/M':>6} {'S/M>200':>7} {'lowS':>6} {'corr':>5} {'c05':>5} {'bal':>5} {'b95':>5} {'cost':>5} {'bnc':>5} {'dBal':>5} {'dW':>5}"
print(hdr)
for r in sorted(rows, key=lambda r: (r['pick'], r['recipe'])):
    worst = max(r['fold_bands'].values(), key=abs)
    f = lambda v: f'{v:5.2f}' if v is not None else '    -'
    print(f"{str(r['pick'])+' '+r['lead']:14} {r['recipe']:11} {r['fold_dLU']:8.2f} {r['fold_resid']:6.1f} {worst:9.2f} {r['st_dLU']:7.2f} {r['side_mid']:6.1f} {r['side_mid_hi']:7.1f} {r['low_side']:6.1f} {r['corr_med']:5.2f} {r['corr_p05']:5.2f} {r['balance']:5.2f} {r['bal_p95']:5.1f} {r['cost']:5.2f} {f(r['bounce'])} {f(r['drift_bal'])} {f(r['drift_width'])}")
for rec in sorted({r['recipe'] for r in rows}):
    sel = [r for r in rows if r['recipe'] == rec]
    if not sel:
        continue
    med = lambda k: float(np.median([r[k] for r in sel if r[k] is not None]))
    print(f"median {rec:11} fold dLU {med('fold_dLU'):.2f}  resid {med('fold_resid'):.1f}  S/M {med('side_mid'):.1f}  S/M>200 {med('side_mid_hi'):.1f}  lowS {med('low_side'):.1f}  corr {med('corr_med'):.2f}  c05 {med('corr_p05'):.2f}  cost {med('cost'):.2f}  bounce {med('bounce'):.2f}  dBal {med('drift_bal'):.2f}  dW {med('drift_width'):.2f}")
