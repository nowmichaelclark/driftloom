"""Before/after clips and the numbers behind them.

Usage: python3 -I clips.py INDEX.json WAVDIR OUTDIR
Writes, per sound, one MP3: loop A before, loop A after, loop B before,
loop B after, each loudness-matched to -18 LUFS, 0.8 s apart. Prints a
table of what moved.
"""
import json, os, subprocess, sys
import numpy as np
import soundfile as sf
import pyloudnorm as pyln
from scipy.signal import butter, sosfilt

idx = json.load(open(sys.argv[1]))
wavdir, out = sys.argv[2], sys.argv[3]
os.makedirs(out, exist_ok=True)
RATE = 44100
meter = pyln.Meter(RATE)

def band(x, lo, hi):
    sos = butter(4, [lo, hi], btype='band', fs=RATE, output='sos')
    return sosfilt(sos, x)

def bands(m):
    tot = np.sum(m ** 2) + 1e-20
    res = {}
    for name, lo, hi in [('40-120', 40, 120), ('120-250', 120, 250), ('250-1k', 250, 1000), ('1-5k', 1000, 5000), ('5k+', 5000, 16000)]:
        res[name] = 100 * np.sum(band(m, lo, hi) ** 2) / tot
    return res

by = {}
for r in idx:
    sound, when = r['tag'].rsplit('-', 1)
    by.setdefault(sound, {}).setdefault(r['pick'], {})[when] = r

rows = []
for sound, picks in by.items():
    parts = []
    for pick, pair in picks.items():
        segs = {}
        for when in ('before', 'after'):
            x, sr = sf.read(os.path.join(wavdir, pair[when]['file']), dtype='float64')
            segs[when] = x
        b, a = segs['before'], segs['after']
        n = min(len(a), len(b))
        lb, la = meter.integrated_loudness(b), meter.integrated_loudness(a)
        mb, ma = b[:n].mean(axis=1), a[:n].mean(axis=1)
        added = 10 * np.log10(np.sum((ma - mb) ** 2) / (np.sum(mb ** 2) + 1e-20) + 1e-20)
        bb, ba = bands(mb), bands(ma)
        rows.append({
            'sound': sound, 'pick': pick, 'lead': pair['before']['lead'],
            'lufs_before': round(lb, 1), 'lufs_after': round(la, 1),
            'added_dB_vs_loop': round(added, 1),
            'bands_before': {k: round(v, 1) for k, v in bb.items()},
            'bands_after': {k: round(v, 1) for k, v in ba.items()},
            'refused': [pair['before']['refused'], pair['after']['refused']],
            'asked': [pair['before']['asked'], pair['after']['asked']],
            'chords_per_min': [round(pair['before']['changesPerMin'] or 0, 1), round(pair['after']['changesPerMin'] or 0, 1)],
            'render_ms': [round(pair['before']['ms']), round(pair['after']['ms'])],
        })
        for when, x in (('before', b), ('after', a)):
            l = meter.integrated_loudness(x)
            y = x * 10 ** ((-18 - l) / 20)
            f = int(0.3 * RATE)
            ramp = np.linspace(0, 1, f)[:, None]
            y[:f] *= ramp
            y[-f:] *= ramp[::-1]
            parts.append(y)
            parts.append(np.zeros((int(0.8 * RATE), 2)))
    clip = np.concatenate(parts[:-1])
    peak = np.max(np.abs(clip))
    if peak > 0.98:
        clip *= 0.98 / peak
    wav = os.path.join(out, f'{sound}.wav')
    sf.write(wav, clip.astype(np.float32), RATE, subtype='FLOAT')
    subprocess.run(['ffmpeg', '-loglevel', 'error', '-y', '-i', wav, '-c:a', 'libmp3lame', '-b:a', '256k', os.path.join(out, f'{sound}.mp3')], check=True)
    os.remove(wav)

json.dump(rows, open(os.path.join(out, 'numbers.json'), 'w'), indent=1)
for r in rows:
    print(r['sound'], r['pick'], r['lead'], 'LUFS', r['lufs_before'], '->', r['lufs_after'], '| added', r['added_dB_vs_loop'], 'dB',
          '| refused', r['refused'], 'of', r['asked'], '| chords/min', r['chords_per_min'])
    print('   bands before', r['bands_before'])
    print('   bands after ', r['bands_after'])
