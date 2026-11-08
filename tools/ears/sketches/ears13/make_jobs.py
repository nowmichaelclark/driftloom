"""Ears 13: the clip jobs for every candidate, for tools/clips.mjs.

Usage: python3 -P make_jobs.py OUTDIR [SECONDS]
Writes OUTDIR/<candidate>.json. Job names are <candidate>-<profile><index>-<tag>;
the tag "none" is the loop without the candidate (sketch_check.py's and
low_guard.py's baseline, through ab.py). Corpus seed 20261009 (the one Ears 4
and 8 used); loop numbers are its indices.

The chord-following sketches (lowBell, colorTone) hear the chords through the
chapel organ's retunes, which the core sends only while the organ plays: every
job of theirs, "none" included, forces the organ on with its level, dry and
room pinned to 0. That silent organ renders a loop bit for bit as without it
(checked on haven #38: the difference is all zeros).
"""
import json
import os
import sys

out = sys.argv[1]
SECONDS = float(sys.argv[2]) if len(sys.argv) > 2 else 45
os.makedirs(out, exist_ok=True)
SEED = 20261009
PROF = {0: 'shrine', 4: 'vapor', 12: 'cinder', 21: 'glade', 38: 'haven', 45: 'undertow', 48: 'halcyon', 62: 'grove',
        69: 'dust', 71: 'thaw', 23: 'wayfare', 73: 'hollow', 39: 'tide'}
SILENT_ORGAN = {'force': {'organ': 'on'}, 'pin': {'organ.level': 0, 'organ.verb': 0, 'organ.dry': 0}}


def job(cand, i, tag, **kw):
    j = {'name': f'{cand}-{PROF[i]}{i}-{tag}', 'corpus': [SEED, i], 'seconds': SECONDS}
    for k, v in kw.items():
        if k in j and isinstance(v, dict):
            j[k] = {**j[k], **v}
        else:
            j[k] = v
    return j


def merge(*ds):
    o = {}
    for d in ds:
        for k, v in d.items():
            o[k] = {**o.get(k, {}), **v} if isinstance(v, dict) else v
    return o


C = {}
# lowBell and colorTone: without, with (both through the silent organ).
for cand, loops in [('lowBell', [38, 62, 71, 69]), ('colorTone', [0, 4, 39, 62])]:
    C[cand] = [job(cand, i, t, **merge(SILENT_ORGAN, {'sketch': True} if t == 'with' else {}))
               for i in loops for t in ('none', 'with')]
# lowBell's swelling kind: the same jobs, rendered with lowBellSwell's build.
C['lowBellSwell'] = [dict(j, name=j['name'].replace('-with', '-swell')) for j in C['lowBell'] if j['name'].endswith('-with')]
# padWow and widePad: without, with (widePad is a stereo sketch: the stereo
# render is the clip, and a mono render of each checks the fold).
C['padWow'] = [job('padWow', i, t, **({'sketch': True} if t == 'with' else {})) for i in [0, 71, 45, 62] for t in ('none', 'with')]
# widePad renders nothing in stereo yet (the core calls render_side only on a
# channel's bus blocks, not on whole-channel ones): kept for the Ears 13b.
C['widePad'] = [job('widePad', i, t, **({'sketch': True} if t != 'none' and t != 'mono' else {}),
                    **({'layout': 1} if t.startswith('mono') else {}))
                for i in [0, 71, 45, 62] for t in ('none', 'with', 'mono', 'monowith')]
for j in C['widePad']:
    if j['name'].endswith('-monowith'):
        j['sketch'] = True
# deepDuck: today, the approved subDuck (as each loop draws it), and the deep one.
C['deepDuck'] = []
for i in [62, 45, 48, 69]:
    C['deepDuck'] += [job('deepDuck', i, 'none', force={'subDuck': 'today'}),
                      job('deepDuck', i, 'approved', force={'subDuck': 'on'}),
                      job('deepDuck', i, 'with', force={'subDuck': 'on'}, pin={'sub_duck.depth': 14, 'sub_duck.recover': 0.6})]
# wideSweep: today, the approved padSweep (as drawn), the wide ride, the fast swing.
C['wideSweep'] = []
for i in [62, 45, 0, 71]:
    C['wideSweep'] += [job('wideSweep', i, 'none', force={'padSweep': 'today'}),
                       job('wideSweep', i, 'approved', force={'padSweep': 'on'}),
                       job('wideSweep', i, 'with', force={'padSweep': 'on'},
                           pin={'pad_sweep.lo': 380, 'pad_sweep.oct': 4.2, 'pad_sweep.period': 16, 'pad_sweep.q': 1.0}),
                       job('wideSweep', i, 'fast', force={'padSweep': 'on'},
                           pin={'pad_sweep.lo': 550, 'pad_sweep.oct': 2.6, 'pad_sweep.period': 6.5, 'pad_sweep.q': 0.9})]
for cand, jobs in C.items():
    json.dump(jobs, open(os.path.join(out, f'{cand}.json'), 'w'), indent=1)
    print(cand, len(jobs))
