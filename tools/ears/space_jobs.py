"""Space recipes for the stereo sketch: per-loop draws from each recipe's ranges.

Round 1 (Ears 2): room, placed, drifting, wide, widedrift, from the seed.
Round 2 (Ears 4): wide2 and drift2 (round 3: room3 and ens3), from the seed and what the loop is
(a scan.mjs row): heavy parts centered and widened symmetrically, only
light parts placed and by less, no ping-pong to speak of, sparse and wet
loops near mono, and drifting that is slow (a breath of 60-140 s, a light
part crossing over 30-60 s).

Usage: python3 space_jobs.py SEEDS.json OUT.json PICKS SECONDS [RECIPES] [nomono]
SEEDS.json is {pick: seed} or a scan.mjs list (needed for round 2).
"""
import json, random, sys

def draw(seed):
    r = random.Random(seed ^ 0x5BACE)
    u = r.uniform
    room = {'verbWidth': round(u(0.5, 0.9), 2), 'echoWidth': round(u(0.5, 0.9), 2)}
    s = r.choice([-1, 1]); h = r.choice([-1, 1]); t = r.choice([-1, 1])
    pan = {'chords': round(s * u(0.15, 0.4), 2), 'melody': round(-s * u(0.0, 0.2), 2), 'texture': round(t * u(0.2, 0.5), 2)}
    hat = round(h * u(0.15, 0.35), 2); shk = round(-h * u(0.2, 0.4), 2)
    drum = {'hat': hat, 'ohat': hat, 'shaker': shk, 'jingle': shk, 'ojingle': shk,
            'tap': round(u(-0.3, 0.3), 2), 'rim': round(u(-0.3, 0.3), 2)}
    placed = {**room, 'pan': pan, 'drumPan': drum}
    td = min(u(0.2, 0.45), 0.75 - abs(pan['texture']))
    move = {'texture': {'depth': round(td, 2), 'period': round(u(16, 48), 1)}}
    if r.random() < 0.5:
        cd = min(u(0.1, 0.25), 0.55 - abs(pan['chords']))
        move['chords'] = {'depth': round(cd, 2), 'period': round(u(24, 64), 1)}
    drifting = {**placed, 'move': move}
    # Wide: drawn after the others so their values don't move.
    s2 = r.choice([-1, 1]); t2 = r.choice([-1, 1]); h2 = r.choice([-1, 1])
    wpan = {'chords': round(s2 * u(0.3, 0.55), 2), 'melody': round(-s2 * u(0.0, 0.12), 2), 'texture': round(-s2 * u(0.25, 0.45), 2)}  # opposite the chords
    what = round(h2 * u(0.25, 0.45), 2); wshk = round(-h2 * u(0.3, 0.5), 2)
    wdrum = {'hat': what, 'ohat': what, 'shaker': wshk, 'jingle': wshk, 'ojingle': wshk,
             'tap': round(u(-0.4, 0.4), 2), 'rim': round(u(-0.4, 0.4), 2)}
    wide = {'verbWidth': round(u(0.8, 1.0), 2), 'echoWidth': round(u(0.8, 1.0), 2), 'pan': wpan, 'drumPan': wdrum,
            'spread': {'chords': round(u(0.3, 0.5), 2), 'texture': round(u(0.4, 0.7), 2), 'melody': round(u(0.0, 0.2), 2)}}
    wmove = {'texture': {'depth': round(min(u(0.2, 0.4), 0.85 - abs(wpan['texture'])), 2), 'period': round(u(16, 48), 1)}}
    if r.random() < 0.5:
        wmove['chords'] = {'depth': round(min(u(0.1, 0.25), 0.7 - abs(wpan['chords'])), 2), 'period': round(u(24, 64), 1)}
    widedrift = {**wide, 'move': wmove}
    return {'room': room, 'placed': placed, 'drifting': drifting, 'wide': wide, 'widedrift': widedrift}

LIGHT_TEXTURE = {'chime', 'bells', 'drops'}


def width_scale(info):
    """How much width a loop takes: sparse loops little, wet sparse loops less."""
    nps = sum(info['perSec'].values())
    dens = min(1.0, max(0.0, (nps - 0.5) / 5))
    w = 0.25 + 0.75 * dens
    if info['space'] > 0.7:
        w *= 0.75
    return round(w, 2), dens


def draw2(seed, info):
    r = random.Random(seed ^ 0x5BAC2)
    u = r.uniform
    w, dens = width_scale(info)
    light = info.get('texture') in LIGHT_TEXTURE
    side = r.choice([-1, 1])
    hat = round(side * w * u(0.08, 0.15), 2)
    drum = {'hat': hat, 'ohat': hat, 'shaker': round(hat * 0.7, 2), 'jingle': round(hat * 0.7, 2), 'ojingle': round(hat * 0.7, 2)}
    pan = {'chords': 0, 'melody': 0, 'texture': round(-side * w * u(0.1, 0.25), 2) if light else 0}
    wide2 = {
        'verbWidth': round(w * u(0.55, 0.8), 2), 'echoWidth': round(w * u(0.15, 0.3), 2),
        'pan': pan, 'drumPan': drum,
        'spread': {'chords': round(w * u(0.3, 0.5), 2), 'texture': round(w * u(0.35, 0.6), 2) if not light else round(w * u(0.15, 0.3), 2),
                   'melody': round(w * u(0.0, 0.12), 2)},
        'ceil': -7.0 if dens > 0.5 else (-9.0 if dens > 0.15 else -12.0),
        'w': w,
    }
    drift2 = json.loads(json.dumps(wide2))
    drift2['breath'] = {'period': round(u(60, 140), 1), 'depth': round(u(0.3, 0.45), 2)}
    cross = round(u(60, 120), 1)  # a full sway: across in 30-60 s
    if light:
        drift2['move'] = {'texture': {'depth': round(min(w * u(0.2, 0.3), 0.45 - abs(pan['texture'])), 2), 'period': cross}}
    if info.get('drums'):
        drift2['drumMove'] = {'depth': round(min(w * u(0.1, 0.18), 0.3 - abs(hat)), 2), 'period': cross}
    # Round 3 (Ears 4): where the width lives, heard apart. Steady is round 2
    # held still (wide2). Room: the dry parts all centered, width only in a
    # smoother reverb side (allpass-decorrelated, `verbSmooth`), no echo
    # width. Ensemble: no room width; chords and texture spread 1.6 times
    # round 2's, the light kit placed as in round 2.
    room3 = {'verbSmooth': round(w * u(0.7, 0.9), 2), 'ceil': wide2['ceil'], 'w': w}
    ens3 = {'pan': pan, 'drumPan': drum, 'ceil': wide2['ceil'], 'w': w,
            'spread': {k: round(min(0.8, v * 1.6), 2) for k, v in wide2['spread'].items()}}
    return {'wide2': wide2, 'drift2': drift2, 'room3': room3, 'ens3': ens3}


if __name__ == '__main__':
    raw = json.load(open(sys.argv[1]))
    info = {str(x['pick']): x for x in raw} if isinstance(raw, list) else None
    seeds = {k: v['seed'] for k, v in info.items()} if info else raw  # {pick: seed}
    picks = [int(p) for p in sys.argv[3].split(',')]
    secs = float(sys.argv[4])
    jobs = []
    for p in picks:
        rec = draw(seeds[str(p)])
        if info:
            rec.update(draw2(seeds[str(p)], info[str(p)]))
        if len(sys.argv) <= 6 or sys.argv[6] != 'nomono':
            jobs.append({'pick': p, 'seconds': secs, 'tag': 'mono'})
        for name in (sys.argv[5].split(',') if len(sys.argv) > 5 else ('room', 'placed', 'drifting')):
            jobs.append({'pick': p, 'seconds': secs, 'tag': name, 'space': rec[name]})
    json.dump({'seed': 20261009, 'n': 80, 'jobs': jobs}, open(sys.argv[2], 'w'), indent=1)
