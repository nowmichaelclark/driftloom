"""Space recipes for the stereo sketch: per-loop draws from each recipe's ranges."""
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

if __name__ == '__main__':
    seeds = json.load(open(sys.argv[1]))  # {pick: seed}
    picks = [int(p) for p in sys.argv[3].split(',')]
    secs = float(sys.argv[4])
    jobs = []
    for p in picks:
        rec = draw(seeds[str(p)])
        if len(sys.argv) <= 6 or sys.argv[6] != 'nomono':
            jobs.append({'pick': p, 'seconds': secs, 'tag': 'mono'})
        for name in (sys.argv[5].split(',') if len(sys.argv) > 5 else ('room', 'placed', 'drifting')):
            jobs.append({'pick': p, 'seconds': secs, 'tag': name, 'space': rec[name]})
    json.dump({'seed': 20261009, 'n': 80, 'jobs': jobs}, open(sys.argv[2], 'w'), indent=1)
