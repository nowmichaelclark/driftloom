"""Ears 13: listening clips from clips.mjs renders, one file per candidate and tag.

Usage: python3 -P make_clips.py PLAN.json RENDERDIR CLIPDIR
PLAN.json: [{"file": "02-lowBell", "candidate": "lowBell", "loops": ["haven38", ...],
             "tags": ["none", "with"], "names": {"none": "without", ...}}, ...]
Each file CLIPDIR/<file>-<name>.opus holds the candidate's loops in the plan's
order, each segment matched to -18 LUFS (integrated, K-weighted, stereo) with
a second of silence between, so every tag of a candidate puts the same loop
at the same time. The renders are read from RENDERDIR/<candidate>/ as
<candidate>-<loop>-<tag>.wav (clips.mjs's names). Opus at 96 kbps through
ffmpeg, as clips.mjs --opus makes it. Writes CLIPDIR/timestamps.json.
"""
import json
import os
import subprocess
import sys

import numpy as np
import pyloudnorm as pyln
import soundfile as sf

plan = json.load(open(sys.argv[1]))
src, out = sys.argv[2], sys.argv[3]
os.makedirs(out, exist_ok=True)
RATE = 44100
meter = pyln.Meter(RATE)
GAP = np.zeros((RATE, 2))
stamps = {}
for p in plan:
    for tag in p['tags']:
        name = p['names'].get(tag, tag)
        parts, t, marks = [], 0.0, []
        for loop in p['loops']:
            x, sr = sf.read(os.path.join(src, p.get('dir', p['candidate']), f"{p['candidate']}-{loop}-{tag}.wav"), always_2d=True)
            assert sr == RATE
            g = 10 ** ((-18.0 - meter.integrated_loudness(x)) / 20)
            y = x * g
            peak = np.max(np.abs(y))
            if peak > 0.98:
                y *= 0.98 / peak
            marks.append({'loop': loop, 'at': round(t, 1), 'seconds': round(len(y) / RATE, 1), 'gain_dB': round(20 * np.log10(g), 2)})
            parts += [y, GAP]
            t += len(y) / RATE + 1.0
        wav = os.path.join(out, f"{p['file']}-{name}.wav")
        sf.write(wav, np.concatenate(parts), RATE, subtype='FLOAT')
        opus = wav[:-4] + '.opus'
        subprocess.run(['ffmpeg', '-y', '-loglevel', 'error', '-i', wav, '-c:a', 'libopus', '-b:a', '96k', opus], check=True)
        os.remove(wav)
        stamps[os.path.basename(opus)] = marks
        print(os.path.basename(opus), [m['at'] for m in marks])
json.dump(stamps, open(os.path.join(out, 'timestamps.json'), 'w'), indent=1)
