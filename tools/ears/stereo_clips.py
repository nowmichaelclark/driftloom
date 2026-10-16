"""Stereo against mono clips: per loop, mono, room, wide (30 s each), then
wide + drifting (60 s), every segment loudness-matched to -18 LUFS.

Usage: python3 -I stereo_clips.py WAVDIR OUTDIR PICK[,PICK...] [TAG:SECONDS,...] [SKIP]
The optional order replaces round 1's (Ears 4: mono:30,wide2-C:30,drift2-C:60);
SKIP seconds are dropped from the start of every segment (a loop's intro).
"""
import glob, os, subprocess, sys
import numpy as np
import soundfile as sf
import pyloudnorm as pyln

wavdir, out, picks = sys.argv[1], sys.argv[2], [int(p) for p in sys.argv[3].split(',')]
os.makedirs(out, exist_ok=True)
RATE = 44100
meter = pyln.Meter(RATE)
order = [('mono', 30), ('room', 30), ('wide-C', 30), ('widedrift-C', 60)]
if len(sys.argv) > 4:
    order = [(t.split(':')[0], float(t.split(':')[1])) for t in sys.argv[4].split(',')]
skip = float(sys.argv[5]) if len(sys.argv) > 5 else 0.0

for pick in picks:
    parts, marks, t = [], [], 0.0
    lead = None
    for tag, sec in order:
        f = glob.glob(os.path.join(wavdir, f'{pick:03d}-*-{tag}.wav'))[0]
        lead = os.path.basename(f).split('-')[1]
        x, _ = sf.read(f, dtype='float64')
        x = x[int(skip * RATE): int((skip + sec) * RATE)]
        x = x * 10 ** ((-18 - meter.integrated_loudness(x)) / 20)
        fr = int(0.3 * RATE)
        ramp = np.linspace(0, 1, fr)[:, None]
        x[:fr] *= ramp
        x[-fr:] *= ramp[::-1]
        marks.append((tag, t))
        parts += [x, np.zeros((int(1.0 * RATE), 2))]
        t += sec + 1.0
    clip = np.concatenate(parts[:-1])
    clip *= min(1.0, 0.98 / np.max(np.abs(clip)))
    name = f'stereo-{pick:02d}-{lead}'
    wav = os.path.join(out, name + '.wav')
    sf.write(wav, clip.astype(np.float32), RATE, subtype='FLOAT')
    subprocess.run(['ffmpeg', '-loglevel', 'error', '-y', '-i', wav, '-c:a', 'libmp3lame', '-b:a', '320k', '-joint_stereo', '0',
                    os.path.join(out, name + '.mp3')], check=True)
    os.remove(wav)
    print(name, ' '.join(f'{tag} {int(s // 60)}:{int(s % 60):02d}' for tag, s in marks))
