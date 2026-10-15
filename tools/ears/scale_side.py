"""A recipe's width turned up or down, offline: every stereo move in the
space sketch is linear in its width settings (reverb and echo width, spread,
placement and its sway all feed the side signal and only the side), so
scaling a render's side by K is the same as scaling every width setting by
K. The mid, and so the fold to mono, is untouched.

Usage: python3 -I scale_side.py INDEX.json WAVDIR K TAG[,TAG...]
Writes <file>-x<K>.wav for each matching job (tag <tag>x<K>) and
INDEX-x<K>.json with the new entries added. Run it before the width
ceiling, which then applies to the scaled render.
"""
import json, os, sys
import numpy as np
import soundfile as sf

idx = json.load(open(sys.argv[1]))
wavdir, k, tags = sys.argv[2], float(sys.argv[3]), sys.argv[4].split(',')
suffix = f'x{k:g}'
out = []
for r in idx:
    if r['tag'] not in tags:
        continue
    x, sr = sf.read(os.path.join(wavdir, r['file']), dtype='float64')
    m, s = (x[:, 0] + x[:, 1]) / 2, (x[:, 0] - x[:, 1]) / 2 * k
    name = r['file'].replace('.wav', f'-{suffix}.wav')
    sf.write(os.path.join(wavdir, name), np.stack([m + s, m - s], axis=1).astype(np.float32), sr, subtype='FLOAT')
    out.append({**r, 'file': name, 'tag': r['tag'] + suffix})
    print(name)
json.dump(idx + out, open(sys.argv[1].replace('.json', f'-{suffix}.json'), 'w'))
