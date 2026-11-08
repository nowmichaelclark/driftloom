"""Ears 13b: write one version of twang for one loop's tempo into core/sketch/.

Usage: python3 -P variant.py VERSION BPM [OUT_DIR]   (VERSION: soft, slow, bright)
  soft    a 4.5 Hz tremolo, 35% deep; the pick's cutoff 18 times the note, the tail's 2.5
  slow    a tempo-synced one, two cycles a beat (an eighth), 55% deep; rounder (12, 2)
  bright  a 5.5 Hz tremolo, 45% deep; the brightest pick (30, 3.5)
The block reads the tempo for the spacing of its picks and the synced tremolo, so
each loop is its own build (a clip's --pin does not reach a sketch's own settings).
"""
import os, re, sys
V = {'soft': (18.0, 2.5, 0.35, 4.5, 0.0), 'slow': (12.0, 2.0, 0.55, 0.0, 2.0), 'bright': (30.0, 3.5, 0.45, 5.5, 0.0)}
version, bpm = sys.argv[1], float(sys.argv[2])
here = os.path.dirname(os.path.abspath(__file__))
out = sys.argv[3] if len(sys.argv) > 3 else os.path.join(here, '..', '..', '..', '..', '..', 'core', 'sketch')
b, r, d, hz, pb = V[version]
src = open(os.path.join(here, 'block.rs')).read()
rest = (f'pub const REST: Settings = Settings {{ level: 0.06, rel: 2.2, bright: {b!r}, round: {r!r}, depth: {d!r}, '
        f'rate: {hz!r}, per_beat: {pb!r}, bpm: {bpm!r}, start: f64::NAN }};')
src, n = re.subn(r'pub const REST: Settings = Settings \{[^\n]*\};', rest.replace('\\', '\\\\'), src)
assert n == 1
for name in ('block.rs', 'where.txt'):
    text = src if name == 'block.rs' else open(os.path.join(here, name)).read()
    open(os.path.join(out, name), 'w').write(text)
print(version, bpm)
