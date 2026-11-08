"""Ears 13: a clips.mjs index as a render2.mjs one, for sketch_check.py and
low_guard.py, and a short A/B table of its own.

Usage: python3 -P ab.py CLIPDIR [BASE]
CLIPDIR holds clips.mjs's index.json and WAVs named <cand>-<loop>-<tag>.
Writes CLIPDIR/r2index.json (pick = corpus index, tag, file, lead) and
CLIPDIR/ab.json, and prints per render against its loop's BASE tag
(default "none"): loudness change, the part alone (render minus base: exact,
the core renders bit for bit) by ear against the loop, its loudest 3 s, the
20-60 and 60-120 Hz shares, and the bits it left alone.

The core's player has no voice budget report per voice yet, so refused notes
are written as 0: a channel block (every Ears 13 sketch) plays outside the
budget, and the pin-only candidates change a channel block's setting, so
neither can move a note (checked: the generator's choices and every other
layer's notes are the same, since the index's "drew" lists and the hashes of
the renders without the block match the plain build).
"""
import json
import os
import sys

import numpy as np
import pyloudnorm as pyln
import soundfile as sf
from scipy.signal import butter, sosfilt

d = sys.argv[1]
base = sys.argv[2] if len(sys.argv) > 2 else 'none'
idx = json.load(open(os.path.join(d, 'index.json')))
RATE = 44100
meter = pyln.Meter(RATE)
rows = []
for r in idx:
    cand, loop, tag = r['name'].rsplit('-', 2)
    rows.append({'pick': r['corpus'][1], 'tag': tag, 'file': r['name'] + '.wav', 'lead': loop, 'seconds': r['seconds'],
                 'refused': 0, 'who': {}, 'askedWho': {}, 'ms': 1, 'changesPerMin': 0, 'sketch': r.get('sketch'),
                 'layout': r['layout'], 'hash': r['hash'], 'drew': r['choices']})
# render2's tag for the baseline is "none"
for x in rows:
    if x['tag'] == base:
        x['tag'] = 'none'
json.dump([x for x in rows if x['layout'] == 'stereo'], open(os.path.join(d, 'r2index.json'), 'w'), indent=1)


def band(x, lo, hi):
    return sosfilt(butter(4, [lo, hi], btype='band', fs=RATE, output='sos'), x)


def lufs(x):
    return meter.integrated_loudness(x) if np.any(x) else -99.0


out = []
by = {}
for x in rows:
    by.setdefault((x['pick'], x['layout']), {})[x['tag']] = x
for (pick, lay), g in sorted(by.items()):
    if 'none' not in g:
        continue
    b, _ = sf.read(os.path.join(d, g['none']['file']), always_2d=True)
    for tag, x in g.items():
        if tag == 'none':
            continue
        a, _ = sf.read(os.path.join(d, x['file']), always_2d=True)
        n = min(len(a), len(b))
        a, bb = a[:n], b[:n]
        part = a - bb
        lb, la = lufs(bb), lufs(a)
        lp = lufs(part) if np.max(np.abs(part)) > 0 else -99.0
        w = int(3 * RATE)
        pk = -99.0
        ref = []
        for i in range(0, n - w, w // 2):
            ref.append(np.sum(bb[i:i + w] ** 2))
        med = np.median(ref)
        for k, i in enumerate(range(0, n - w, w // 2)):
            if ref[k] < med * 0.01:
                continue
            e = 10 * np.log10((np.sum(part[i:i + w] ** 2) + 1e-20) / (ref[k] + 1e-20))
            pk = max(pk, e)
        mb, ma = bb.mean(axis=1), a.mean(axis=1)
        tot = lambda m: np.sum(m ** 2) + 1e-20
        sh = lambda m, lo, hi: round(100 * np.sum(band(m, lo, hi) ** 2) / tot(m), 1)
        row = {'pick': pick, 'lead': x['lead'], 'layout': lay, 'tag': tag, 'dLU': round(la - lb, 2),
               'partLU': round(lp - lb, 1), 'peak3s_dB': round(pk, 1),
               'sub': [sh(mb, 20, 60), sh(ma, 20, 60)], 'low': [sh(mb, 60, 120), sh(ma, 60, 120)],
               'p2k5k': [sh(mb, 2000, 5000), sh(ma, 2000, 5000)], 'drew': x['drew']}
        out.append(row)
        print(f"#{pick:<3}{x['lead']:12}{lay:7}{tag:10} dLU {row['dLU']:+5.2f}  part {row['partLU']:6.1f} LU  peak3s {row['peak3s_dB']:6.1f} dB"
              f"  20-60 {row['sub'][0]}->{row['sub'][1]}%  60-120 {row['low'][0]}->{row['low'][1]}%  2-5k {row['p2k5k'][0]}->{row['p2k5k'][1]}%")
json.dump(out, open(os.path.join(d, 'ab.json'), 'w'), indent=1)
