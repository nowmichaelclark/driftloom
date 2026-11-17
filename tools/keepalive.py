"""Write audio/keepalive.flac and audio/keepalive.wav: the media session's
keepalive (js/media.js). Fifteen seconds of a 20 Hz sine at 8 kHz, mono,
16-bit, its power -60 dB (peak about -57 dBFS): above the level a browser
judges silent, far under hearing at that pitch, and exactly 300 cycles, so
the loop wraps without a click.

Usage: python3 -P tools/keepalive.py [OUT_DIR]   (default: audio)
"""

import os
import sys

import numpy as np
import soundfile as sf

RATE = 8000
SECONDS = 15
HZ = 20
POWER_DB = -60.0

out = sys.argv[1] if len(sys.argv) > 1 else 'audio'
t = np.arange(RATE * SECONDS) / RATE
amp = np.sqrt(2.0 * 10 ** (POWER_DB / 10))
x = amp * np.sin(2 * np.pi * HZ * t)
for name, fmt in (('keepalive.flac', 'FLAC'), ('keepalive.wav', 'WAV')):
    sf.write(os.path.join(out, name), x, RATE, subtype='PCM_16', format=fmt)
print(f'{HZ} Hz, {SECONDS} s at {RATE} Hz, power {POWER_DB} dB, peak {20 * np.log10(amp):.1f} dBFS')
