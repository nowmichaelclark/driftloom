# Round 18: paint the textured parts as separate layers (far cliff; near cliffs with talus and junipers; the bank in front),
# each on transparency, for tracing.  python3 layers.py <variation> outdir
import sys, os, math, random
import numpy as np
from ren import *
def paint(name, outdir):
    v = V[name]; lay = LAYOUT[name]; g = geo(); os.makedirs(outdir, exist_ok=True)
    ldx = 1 if v['L'][0] > 0 else -1
    # the far cliff
    far = Layer(); fm = mask([g['far']]); rock(far, fm, LIP - 4, v, v['hz'], 100, 0.8)
    rim = np.clip(fm - np.roll(fm, 5, axis=0), 0, 1); comp(far, hexrgb(v['lightc']), rim*0.5*(1 - v['hz']))
    rr = random.Random(5)
    juniper_layer(far, [(g['far_trees'][i][0], g['far_trees'][i][1] + 2, rr.uniform(3.5, 5)) for i in (1, 2, 4, 9, 11, 12)], v, ldx)
    far.save(f'{outdir}/far.png')
    # the near cliffs, their talus, their junipers
    near = Layer()
    tal = []
    for pts, dsg in ((g['ledge'], 1), (g['redge'], -1)):
        a = [p for p in pts if p[1] > H - 90]
        sk = [(x + dsg*(6 + 60*((y - (H - 90))/90)**1.5), y) for x, y in a]
        tal.append(a + sk[::-1] + [(a[0][0], H + 2)])
    lm = mask([g['left']]); rm = mask([g['right']])
    rock(near, lm, min(y for x, y in g['lt']), v, 0, 300, 1.2)
    rock(near, rm, min(y for x, y in g['rt']), v, 0, 400, 1.2)
    for mm_ in (lm, rm):
        rim = np.clip(mm_ - np.roll(mm_, 4, axis=0), 0, 1); comp(near, hexrgb(v['lightc']), rim*0.55)
    tm = mask(tal); rub = noise(10, 8, 3, 500)
    comp(near, ramp([(0, v['bands'][-3][1]), (1, v['bands'][-1][1])], np.clip((Y - (H - 90))/90 + rub*0.6, 0, 1))*(0.8 + rub[..., None]*0.6), tm)
    spots = []
    for x, base, h in g['trees']:
        for k in range(rr.randint(1, 3)): spots.append((x + rr.uniform(-12, 12), base - 3 + rr.uniform(-2, 2), h*rr.uniform(0.13, 0.22)))
    for poly in tal:
        for k in range(3):
            x, y = poly[rr.randrange(1, len(poly)//2)]; spots.append((x + rr.uniform(4, 22)*(1 if x < 256 else -1), y, rr.uniform(4.5, 7)))
    juniper_layer(near, spots, v, ldx)
    near.save(f'{outdir}/near.png')
    # the bank in front, lower left: a sloping shore of rock and sand, rounded boulders at the water, a big juniper
    if lay.get('bank'):
        bank = Layer(); bn = Noise(91, 4)
        edge = [(x, H + 4 + 8*bn(x/40) + max(0, x - 20)**1.5*0.06) for x in range(-140, 175, 6)]
        bp = [(-140, H - 4)] + [(x, y - 14*max(0, 1 - (x + 140)/180)) for x, y in edge] + [(176, 660), (-140, 660)]
        bm_ = mask([bp]); rub = noise(14, 10, 3, 800)
        bc = ramp([(0, v['bands'][2][1]), (0.5, v['bands'][4][1]), (1, v['bands'][-1][1])], np.clip((Y - H)/150 + rub*0.5, 0, 1))
        gyb, gxb = np.gradient(blur(bm_, 10)*40 + rub*6); lb = np.clip(0.75 - gyb*0.5 - gxb*ldx*0.3, 0.4, 1.2)
        comp(bank, bc*lb[..., None], bm_)
        for bx, by, br in v.get('boulders', []):
            ang = np.arctan2(Y - by, X - bx); wob = 1 + 0.18*np.sin(ang*3 + bx) + 0.1*np.sin(ang*5 + by)
            d = np.sqrt(((X - bx)/(br*1.3))**2 + ((Y - by)/(br*0.8))**2)/wob; bmask = np.clip((1 - d)*br*S*0.5, 0, 1)*(Y < by + br*0.5)
            bh = blur(bmask, 6)*24 + noise(10, 10, 2, int(bx))*6; gyb, gxb = np.gradient(bh)
            lb = np.clip(0.62 - gxb*ldx*0.2 - gyb*0.3, 0.3, 1)
            comp(bank, ramp([(0, v['bands'][-1][1]), (0.6, v['bands'][4][1]), (1, v['bands'][2][1])], lb), bmask)
        juniper_layer(bank, [(30, H + 22, 34), (86, H + 34, 18), (-40, H + 40, 30)], v, ldx)
        bank.save(f'{outdir}/bank.png')
if __name__ == '__main__':
    paint(sys.argv[1], sys.argv[2])
