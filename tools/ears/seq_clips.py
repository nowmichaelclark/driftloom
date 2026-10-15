"""Clips from a list of segments, each loudness-matched, with timestamps.

Usage: python3 -I seq_clips.py PLAN.json WAVDIR OUTDIR
PLAN.json: {"clips": [{"name": "dubstab", "segments": [
    {"file": "038-haven-none.wav", "label": "haven, without", "start": 10, "seconds": 30}, ...]}]}
Every segment is matched to -18 LUFS (integrated, over the segment), faded
in and out over 0.3 s and followed by 1 s of silence; the clip is then
scaled down only if a peak would pass -0.2 dBFS. Prints each clip's
timestamps, and writes them to OUTDIR/timestamps.json.
"""
import json, os, subprocess, sys
import numpy as np
import soundfile as sf
import pyloudnorm as pyln

plan = json.load(open(sys.argv[1]))
wavdir, out = sys.argv[2], sys.argv[3]
os.makedirs(out, exist_ok=True)
RATE = 44100
meter = pyln.Meter(RATE)
stamps = {}
for clip in plan['clips']:
    parts, marks, t = [], [], 0.0
    for seg in clip['segments']:
        x, _ = sf.read(os.path.join(wavdir, seg['file']), dtype='float64')
        a = int(seg.get('start', 0) * RATE)
        x = x[a: a + int(seg['seconds'] * RATE)].copy()
        x *= 10 ** ((-18 - meter.integrated_loudness(x)) / 20)
        f = int(0.3 * RATE)
        ramp = np.linspace(0, 1, f)[:, None]
        x[:f] *= ramp
        x[-f:] *= ramp[::-1]
        marks.append((seg['label'], t))
        parts += [x, np.zeros((RATE, 2))]
        t += len(x) / RATE + 1.0
    y = np.concatenate(parts[:-1])
    y *= min(1.0, 0.977 / np.max(np.abs(y)))
    wav = os.path.join(out, clip['name'] + '.wav')
    sf.write(wav, y.astype(np.float32), RATE, subtype='FLOAT')
    subprocess.run(['ffmpeg', '-loglevel', 'error', '-y', '-i', wav, '-c:a', 'libmp3lame', '-b:a', '320k', '-joint_stereo', '0',
                    os.path.join(out, clip['name'] + '.mp3')], check=True)
    os.remove(wav)
    stamps[clip['name']] = [(lab, f'{int(s // 60)}:{int(s % 60):02d}') for lab, s in marks]
    print(clip['name'], ' | '.join(f'{lab} {ts}' for lab, ts in stamps[clip['name']]))
json.dump(stamps, open(os.path.join(out, 'timestamps.json'), 'w'), indent=1)
