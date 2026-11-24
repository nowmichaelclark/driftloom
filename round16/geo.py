# Round 16: round 15's scene as plain shapes, so each style can redraw it.
import math, os, random
HERE = os.path.dirname(os.path.abspath(__file__))
full = open(os.path.join(HERE, '..', 'round12', 'gen.py')).read()
src = full[:full.index('# the moon')] + full[full.index('def fir('):full.index('def gel_stream(')] \
    + full[full.index('def ripples('):full.index('# ======')] + full[full.index('def crescent('):full.index('def design1(')]
G = {'__name__': 'r16', '__file__': os.path.join(HERE, '..', 'round12', 'gen.py')}
exec(compile(src, 'round12/gen.py', 'exec'), G)
Icon, P, smooth, f, fir, ripples, crescent = G['Icon'], G['P'], G['smooth'], G['f'], G['fir'], G['ripples'], G['crescent']

class Noise:
    def __init__(self, seed, oct=4):
        r = random.Random(seed); self.v = [r.uniform(-1, 1) for _ in range(4096)]; self.oct = oct
    def base(self, x):
        i = math.floor(x); t = x - i; t = t*t*(3-2*t)
        return self.v[i % 4096]*(1-t) + self.v[(i+1) % 4096]*t
    def __call__(self, x):
        s, a, fq, norm = 0, 1, 1, 0
        for _ in range(self.oct): s += a*self.base(x*fq + 37.1*fq); norm += a; a *= 0.5; fq *= 2.03
        return s/norm

H = 392; LIP = 196; LX0, LX1 = 226, 286; FX0, FX1 = 206, 306
MOON = (352, 100, 33)
TREES = [(-100, 54), (-66, 72), (-32, 58), (4, 76), (40, 60), (72, 46), (470, 58), (506, 80), (544, 66), (584, 88), (622, 70)]
FAR_TREES = [92, 104, 118, 150, 166, 184, 200, 312, 330, 346, 372, 392, 410, 432]

def edges(y):
    t = max(0, min(1, (y - LIP)/(H - LIP))); k = t**1.2
    return LX0 + (FX0 - LX0)*k, LX1 + (FX1 - LX1)*k

def geo():
    g = {}
    n1 = Noise(3, 5); n2 = Noise(4, 5)
    g['ridge1'] = [(x, 176 - 22*n1(x/110) - 6*n1(x/23)) for x in range(-130, 660, 10)]
    g['ridge2'] = [(x, 188 - 12*n2(x/80) - 5*n2(x/19)) for x in range(-130, 660, 10)]
    tn = Noise(11, 4)
    g['ftop'] = [(x, LIP - 2 + 3*tn(x/20) - (0 if LX0 - 8 < x < LX1 + 8 else 4 + 3*abs(tn(x/9)))) for x in range(60, 460, 6)]
    g['far'] = g['ftop'] + [(460, H + 4), (60, H + 4)]
    g['far_trees'] = [(x, LIP + 1 + 3*tn(x/20) - 4, 14 + (i*7 % 11)) for i, x in enumerate(FAR_TREES)]
    wn = Noise(61, 3); lft = []; rgt = []
    for i in range(25):
        y = LIP + (H - LIP)*i/24; a, b = edges(y); rag = (i/24)**2*4
        lft.append((a + rag*wn(i*0.9), y)); rgt.append((b + rag*wn(i*0.9 + 20), y))
    g['water'] = rgt + lft[::-1]
    ln = Noise(23, 5); rn = Noise(29, 5)
    def lx(y):
        t = (y + 40)/(H + 44)
        return 96 + 64*math.sin(math.pi*min(1, t*1.6))**1.5 - 30*t + 22*ln(t*4) + 6*ln(t*17)
    lt = [(x, 56 + 14*ln(x/60 + 3) + 6*ln(x/13) + 0.004*(x - 40)**2*(x > 40)) for x in range(-140, 100, 8)]
    g['ledge'] = [(lx(y), y) for y in range(int(lt[-1][1]) + 8, H + 8, 8)]
    g['left'] = lt + g['ledge'] + [(lx(H + 6) + 20, H + 6), (-140, H + 6)]
    def rx(y):
        t = (y - 120)/(H - 116)
        return 414 - 46*math.sin(math.pi*min(1, t*1.4))**1.3 + 24*t + 16*rn(t*4) + 5*rn(t*15)
    rt = [(x, 112 + 12*rn(x/50 + 7) + 5*rn(x/11)) for x in range(660, 420, -8)]
    g['redge'] = [(rx(y), y) for y in range(int(rt[-1][1]) + 8, H + 8, 8)]
    g['right'] = rt + g['redge'] + [(rx(H + 6) - 20, H + 6), (660, H + 6)]
    g['lt'], g['rt'] = lt, rt
    g['trees'] = []
    for x, h in TREES:
        top = lt if x < 256 else rt
        g['trees'].append((x, min(yy for xx, yy in top if abs(xx - x) < 9) + 5, h))
    return g

def sky_rect(): return [(-128, -128), (640, -128), (640, H + 2), (-128, H + 2)]
def pool_rect(): return [(-128, H), (640, H), (640, 640), (-128, 640)]
def under(pts, y=330): return smooth(pts, close=False) + f" L640,{y} L-128,{y} Z"
