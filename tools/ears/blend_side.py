"""Two space recipes blended, offline: the mid of the first render and a
weighted sum of both renders' side signals. Exact when both renders share
their mid (every recipe in the space sketch leaves the mid as the mono
mix), because every stereo move is linear in its width.

Usage: python3 -I blend_side.py INDEX.json WAVDIR TAG_A:WEIGHT TAG_B:WEIGHT NEW_TAG
Writes <pick>-<lead>-<NEW_TAG>.wav for every loop that has both tags, and
INDEX-<NEW_TAG>.json with the new entries added (the first tag's job,
with 'space' holding both recipes and weights). Run it before the width
ceiling. Weights whose squares add to 1 keep the overall width when the
two sides are unrelated.
"""
import json, os, sys
import numpy as np
import soundfile as sf

idx = json.load(open(sys.argv[1]))
wavdir = sys.argv[2]
(ta, wa), (tb, wb) = [(t.split(':')[0], float(t.split(':')[1])) for t in sys.argv[3:5]]
new = sys.argv[5]
by = {}
for r in idx:
    by.setdefault(r['pick'], {})[r['tag']] = r
out = []
for pick, g in sorted(by.items()):
    if ta not in g or tb not in g:
        continue
    a, sr = sf.read(os.path.join(wavdir, g[ta]['file']), dtype='float64')
    b, _ = sf.read(os.path.join(wavdir, g[tb]['file']), dtype='float64')
    n = min(len(a), len(b))
    m = (a[:n, 0] + a[:n, 1]) / 2
    s = wa * (a[:n, 0] - a[:n, 1]) / 2 + wb * (b[:n, 0] - b[:n, 1]) / 2
    name = f"{pick:03d}-{g[ta]['lead']}-{new}.wav"
    sf.write(os.path.join(wavdir, name), np.stack([m + s, m - s], axis=1).astype(np.float32), sr, subtype='FLOAT')
    space = dict(g[ta].get('space') or {})
    space['blend'] = {ta: wa, tb: wb}
    out.append({**g[ta], 'file': name, 'tag': new, 'space': space})
    print(name)
json.dump(idx + out, open(sys.argv[1].replace('.json', f'-{new}.json'), 'w'))
