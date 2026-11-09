"""Ears 13b: the clip jobs for batch 6's seven levered candidates and twang.

Usage: python3 -P make_jobs_13b.py OUTDIR
Writes OUTDIR/<candidate>.json for tools/clips.mjs. Job names are
<candidate>-<profile><index>-<tag>; the tag "none" is the loop without the
candidate (ab.py's, sketch_check.py's and low_guard.py's baseline). Corpus
seed 20261009, as Ears 4, 8 and 13. Every lever is item 99's (the header of
tools/clips.mjs); dropSpotlight and twang are sketch blocks, rendered with
their own builds (README.md). twang's builds carry the loop's tempo, so its
jobs are one file per loop (twang-<loop>.json).

The rhythm levers' loops are in 16 steps a bar: in a 6/8 loop tripletKit's
thirds of the dotted beat are its eighths (it only thins the hats), and
kick332's 0, 6 and 12 of a 16-step bar fall between steps of a 12-step one.
tripletKit moves only the hits on a beat's second and fourth sixteenths (an
eighth-note kit loses its off-beats and gains no thirds), so its loops are
ones whose kits play sixteenths (tide #52's bars are 12 steps of 4-step beats).
"""
import json
import os
import sys

out = sys.argv[1]
os.makedirs(out, exist_ok=True)
SEED = 20261009
PROF = {0: 'shrine', 1: 'undertow', 4: 'vapor', 74: 'bloom', 75: 'undertow', 9: 'dust', 10: 'thaw', 13: 'shrine', 21: 'glade', 38: 'haven', 39: 'tide', 45: 'undertow',
        47: 'grove', 49: 'dust', 52: 'tide', 62: 'grove', 69: 'dust', 70: 'vapor', 71: 'thaw'}
RETUNE = {'set': {'chords.RETUNE': 1}}
# widePad: the chords' spread past its range, and the width ceiling raised with it
# (spread-chords alone stops at the ceiling: -7 to -9 dB on the chord stem).
WIDE = {'spread-chords': 1.4, 'spread-max': 2, 'spread-cutoff': 250, 'ceiling-busy': -4, 'ceiling-sparse': -5, 'ceiling-very-sparse': -7}
STEM = 'layer.melody=off,layer.bass=off,layer.drums=off,layer.texture=off'
CENTS = {4: -32, 69: 18, 0: -24, 38: 27}


def job(cand, i, tag, seconds=45, **kw):
    j = {'name': f'{cand}-{PROF[i]}{i}-{tag}', 'corpus': [SEED, i], 'seconds': seconds}
    j.update(kw)
    return j


def merge(*ds):
    o = {}
    for d in ds:
        for k, v in d.items():
            o[k] = {**o.get(k, {}), **v} if isinstance(v, dict) else v
    return o


C = {}
C['tripletKit'] = [job('tripletKit', i, t, **({'force': {'tripletKit': 'on'}} if t == 'with' else {}))
                   for i in [69, 74, 52, 75] for t in ('none', 'with')]
C['widePad'] = [job('widePad', i, t, **({'space': WIDE} if t == 'with' else {}), **({'layout': 1} if t == 'mono' else {}))
                for i in [0, 71, 45, 62] for t in ('none', 'with', 'mono')]
# The chord layer alone (the other layers asked off: a close stand-in for its stem;
# the budget steals a little less, so a few more chord notes sound).
C['widePadStem'] = [job('widePadStem', i, t, request=STEM, **({'space': WIDE} if t == 'with' else {}))
                    for i in [0, 71, 45, 62] for t in ('none', 'with')]
C['dottedPulse'] = [job('dottedPulse', i, t, **({'force': {'dottedPulse': t}} if t != 'none' else {}))
                    for i in [45, 69, 47, 1] for t in ('none', 'pulse', 'kick332')]
# slowOpening's loops draw no lateFloor of their own: a job's "today" does not
# force one off (dlrender reads "today" as not forced; batch 6, Ears 13b).
C['slowOpening'] = []
for i in [69, 39, 49]:
    C['slowOpening'] += [job('slowOpening', i, 'none', 100, force={'lateFloor': 'today'}),
                         job('slowOpening', i, 'approved', 100, force={'lateFloor': 'on'}),
                         job('slowOpening', i, 'with', 100, force={'lateFloor': 'on'},
                             set={'latefloor.SECONDS_MAX': 90, 'latefloor.FADE': 12})]
C['planingSevenths'] = [job('planingSevenths', i, t, **({'force': {'planingSevenths': 'on'}} if t == 'with' else {}))
                        for i in [0, 39, 71, 47] for t in ('none', 'with')]
C['tuningOffset'] = [job('tuningOffset', i, t, **({'set': {'master.CENTS': CENTS[i]}} if t == 'with' else {}))
                     for i in [4, 69, 0, 38] for t in ('none', 'with')]
# dropSpotlight: every job has the floor drop forced and the chords' retunes on;
# "with" and "swell" turn the sketch on with the drums (mod 0) and the bass (mod 1)
# routed into it. "swell" is rendered with the dropSpotlightSwell build.
DROP = merge(RETUNE, {'force': {'floorDrop': 'shapes'}})
C['dropSpotlight'] = [job('dropSpotlight', i, t, 50, **merge(DROP, {'sketch': True, 'route': {'sketch': 'drums', 'sketch.1': 'bass'}} if t != 'none' else {}))
                      for i in [9, 21, 39, 47] for t in ('none', 'with')]
C['dropSpotlightSwell'] = [dict(j, name=j['name'].replace('-with', '-swell')) for j in C['dropSpotlight'] if j['name'].endswith('-with')]
# twang: the chords' retunes on in every job; each version is a build per loop.
for i in [70, 10, 13, 71]:
    C[f'twang-{PROF[i]}{i}'] = [job('twang', i, 'none', **RETUNE)] + [job('twang', i, v, **merge(RETUNE, {'sketch': True}))
                                                                    for v in ('soft', 'slow', 'bright')]
# The sampler (00-everything-13b): five loops, each with one or two of the eight,
# chosen by hand to cover them all; 50 s each (tide #39's from 30 s, so the slow
# floor's arrival at 54.6 s falls inside it). "mononone" and "monowith" are the
# same as one channel, for stereo_guard.py. grove #47 is rendered with the
# dropSpotlight build, thaw #71 with twang's soft build at 58 BPM.
EVERY = {
    69: merge({'force': {'tripletKit': 'on'}, 'set': {'master.CENTS': 18}}),
    0: merge({'force': {'planingSevenths': 'on'}, 'space': WIDE}),
    47: merge(DROP, {'force': {'dottedPulse': 'pulse'}, 'sketch': True, 'route': {'sketch': 'drums', 'sketch.1': 'bass'}}),
    71: merge(RETUNE, {'sketch': True, 'set': {'master.CENTS': -20}}),
    39: {'force': {'lateFloor': 'on'}, 'set': {'latefloor.SECONDS_MAX': 90, 'latefloor.FADE': 12}},
}
BASE = {47: DROP, 71: RETUNE, 39: {'force': {'lateFloor': 'today'}}}
for i, w in EVERY.items():
    at = {'from': 30, 'seconds': 80} if i == 39 else {'seconds': 50}
    base = BASE.get(i, {})
    jobs = []
    for tag, kw in [('none', base), ('with', w)]:
        sec = at['seconds']
        extra = {'from': at['from']} if 'from' in at else {}
        jobs.append(job('everything13b', i, tag, sec, **kw, **extra))
        if 'space' in kw:
            kw = {k: v for k, v in kw.items() if k != 'space'}
        jobs.append(job('everything13b', i, 'mono' + tag, sec, layout=1, **kw, **extra))
    C[f'every13b-{PROF[i]}{i}'] = jobs
for cand, jobs in C.items():
    # Ears 13's widePad.json (its block) stays as it was.
    name = 'widePad-13b' if cand == 'widePad' else cand
    json.dump(jobs, open(os.path.join(out, f'{name}.json'), 'w'), indent=1)
    print(cand, len(jobs))
