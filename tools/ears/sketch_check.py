"""What a sketch adds, and what it takes: every render of a loop against
the same loop's 'none' render (no sketch), from a render2.mjs index.

Usage: python3 -I sketch_check.py INDEX.json WAVDIR [OUT.json]

Per render, against its loop's 'none':
  added      energy of (render - none) against none, dB: the sketch with its
             echo and reverb (render2.mjs seeds Math.random, so the two
             renders differ only by the sketch)
  peak2s     the loudest 2-second window of the difference against the same
             window of 'none', dB (how far under the loop the sketch peaks),
             over windows within 20 dB of the loop's median level (an intro
             or a full stop would otherwise read as the sketch towering)
  med2s      the median of that over the same windows
  bands      share of energy in 40-120, 120-250, 250-1k, 1-5k, 5k+ (none, render)
  refused    refused notes per voice, none -> render, and the voices that
             lost notes or asked fewer times (any is a failure)
  layers     with stems on both renders: each layer's own level change, dB
             (its channel's stem less the sketches in that channel; 0.0 when
             nothing was refused and nothing else moved)
  cost       render wall time against none
"""
import json, os, sys
import numpy as np
import soundfile as sf
from scipy.signal import butter, sosfilt

idx = json.load(open(sys.argv[1]))
wavdir = sys.argv[2]
RATE = 44100


def band(x, lo, hi):
    return sosfilt(butter(4, [lo, hi], btype='band', fs=RATE, output='sos'), x)


def bands(m):
    tot = np.sum(m ** 2) + 1e-20
    return {n: round(100 * np.sum(band(m, lo, hi) ** 2) / tot, 1)
            for n, lo, hi in [('40-120', 40, 120), ('120-250', 120, 250), ('250-1k', 250, 1000), ('1-5k', 1000, 5000), ('5k+', 5000, 16000)]}


def windows(x, w=2.0):
    n = int(w * RATE)
    return np.array([np.sum(x[i:i + n] ** 2) for i in range(0, len(x) - n + 1, n // 2)])


def db(a, b):
    return 10 * np.log10((a + 1e-20) / (b + 1e-20))


groups = {}
for r in idx:
    groups.setdefault((r['pick'], r.get('seconds', 60)), {})[r['tag']] = r

rows = []
for (pick, sec), g in sorted(groups.items()):
    if 'none' not in g:
        continue
    b0 = g['none']
    xb, _ = sf.read(os.path.join(wavdir, b0['file']), dtype='float64')
    mb = xb.mean(axis=1)
    sb = None
    if b0.get('stemNames'):
        sb, _ = sf.read(os.path.join(wavdir, b0['file'].replace('.wav', '-stems.wav')), dtype='float64')
    for tag, r in g.items():
        if tag == 'none':
            continue
        xa, _ = sf.read(os.path.join(wavdir, r['file']), dtype='float64')
        n = min(len(xa), len(xb))
        ma = xa[:n].mean(axis=1)
        d = ma - mb[:n]
        wd, wb = windows(d), windows(mb[:n])
        live = wb > np.median(wb) * 0.01  # within 20 dB of the loop's usual level
        rel = db(wd[live], wb[live]) if live.any() else np.array([-99.0])
        lost = []
        for v, k in b0['askedWho'].items():
            if r['askedWho'].get(v, 0) < k:
                lost.append(f'{v} asked {k}->{r["askedWho"].get(v, 0)}')
        for v in set(b0['who']) | set(r['who']):
            if v.split('.')[-1] in ('dubStab', 'dubStabR1', 'organR1'):
                continue
            if r['who'].get(v, 0) > b0['who'].get(v, 0):
                lost.append(f'{v} refused {b0["who"].get(v, 0)}->{r["who"][v]}')
        sketch_refused = sum(v for k, v in r['who'].items() if k.split('.')[-1] in ('dubStab', 'dubStabR1', 'organR1'))
        layers = None
        if sb is not None and r.get('stemNames'):
            sa, _ = sf.read(os.path.join(wavdir, r['file'].replace('.wav', '-stems.wav')), dtype='float64')
            layers = {}
            names = r['stemNames']
            # A layer's stem holds the sketches that play into its channel;
            # take them out, so what is left is the layer's own notes.
            HOME = {'dubStab': 'chords', 'organ': 'chords'}  # the tuned wash has a channel of its own
            own = {}
            for i, name in enumerate(names):
                if name in ('drums', 'bass', 'chords', 'melody', 'texture'):
                    own[name] = sa[:n, i].copy()
            for i, name in enumerate(names):
                if name in HOME:
                    own[HOME[name]] -= sa[:n, i]
            for i, name in enumerate(b0['stemNames']):
                if name in own:
                    eb = np.sum(sb[:n, i] ** 2)
                    layers[name] = None if eb < 1e-9 else round(db(np.sum(own[name] ** 2), eb), 2)
        rows.append({
            'pick': pick, 'lead': r['lead'], 'tag': tag, 'seconds': sec, 'sketch': r.get('sketch'),
            'added': round(db(np.sum(d ** 2), np.sum(mb[:n] ** 2)), 1),
            'peak2s': round(float(np.max(rel)), 1), 'med2s': round(float(np.median(rel)), 1),
            'bands': [bands(mb[:n]), bands(ma)],
            'refused': [b0['refused'], r['refused']], 'sketch_refused': sketch_refused,
            'others_lost': lost, 'layers': layers,
            'cost': round(r['ms'] / b0['ms'], 2), 'chords_per_min': [round(b0['changesPerMin'] or 0, 1), round(r['changesPerMin'] or 0, 1)],
        })

if len(sys.argv) > 3:
    json.dump(rows, open(sys.argv[3], 'w'), indent=1)
for r in rows:
    lay = '' if r['layers'] is None else ' layers ' + ' '.join(f'{k}:{v}' for k, v in r['layers'].items() if v is not None)
    print(f"{r['pick']:3d} {r['lead']:9} {r['tag']:12} added {r['added']:6.1f}  peak2s {r['peak2s']:6.1f}  med2s {r['med2s']:6.1f}"
          f"  refused {r['refused'][0]}->{r['refused'][1]} (sketch {r['sketch_refused']})  others lost {len(r['others_lost'])}"
          f"  cost {r['cost']}{lay}")
    for x in r['others_lost']:
        print('      ', x)
