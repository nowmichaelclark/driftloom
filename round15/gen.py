# Round 15: a new direction, in layers: near cliffs framing the view, a far cliff the water drops from, hazy ridges;
# a recess behind the drop) in the app's own colors and feel (deep plum, soft glows, the gel of its Play button).
# python3 gen.py out.svg   (shares round12's helpers)
import math, os, sys, random
HERE = os.path.dirname(os.path.abspath(__file__))
full = open(os.path.join(HERE, '..', 'round12', 'gen.py')).read()
src = full[:full.index('# the moon')] + full[full.index('def fir('):full.index('def gel_stream(')] \
    + full[full.index('def ripples('):full.index('# ======')] + full[full.index('def crescent('):full.index('def design1(')]
G = {'__name__': 'r15', '__file__': os.path.join(HERE, '..', 'round12', 'gen.py')}
exec(compile(src, 'round12/gen.py', 'exec'), G)
Icon, P, smooth, f, fir, ripples = G['Icon'], G['P'], G['smooth'], G['f'], G['fir'], G['ripples']
OUT = sys.argv[1] if len(sys.argv) > 1 else 'round15.svg'

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

def strip(ic, xf, y0, y1, w, fill, o, k=0.5, n=14):
    """A strip following x = xf(y), pointed at both ends."""
    L = []; R = []
    for i in range(n + 1):
        t = i/n; y = y0 + (y1 - y0)*t; hw = w/2*math.sin(math.pi*t)**k; x = xf(y)
        L.append((x - hw, y)); R.append((x + hw, y))
    ic.path(smooth(L + R[::-1]), fill, o)


H = 392                         # the pool
LIP = 196                       # the brink, on the far cliff
LX0, LX1 = 226, 286             # the water at the brink
FX0, FX1 = 206, 306             # the water at the foot

def edge(pts_fn, n, t0, t1):
    return [pts_fn(t0 + (t1 - t0)*i/n) for i in range(n + 1)]

def facets(ic, rnd, poly_clip, x0, x1, y0, y1, lit, dark, n, moon_side=1, o=1.0):
    """Rock planes: tall, narrow, irregular, side by side, broken at different heights; each catches the moon on its right."""
    cid = ic.gid('c'); ic.defs.append(f'<clipPath id="{cid}"><path d="{poly_clip}"/></clipPath>'); ic.raw(f'<g clip-path="url(#{cid})">')
    x = x0; lits = []; darks = []
    while x < x1:
        w = rnd.uniform(16, 40); ya = y0 + rnd.uniform(-20, 30)
        while ya < y1:
            yb = ya + rnd.uniform(70, 190); sk = rnd.uniform(-7, 7)
            top = [(x, ya + rnd.uniform(0, 16)), (x + w*rnd.uniform(0.3, 0.7), ya), (x + w, ya + rnd.uniform(4, 22))]
            bot = [(x + w + sk, yb + rnd.uniform(-12, 0)), (x + sk*0.5 + w*0.4, yb + rnd.uniform(0, 10)), (x + sk, yb - rnd.uniform(0, 14))]
            q = top + bot
            # the face toward the moon: a lit sliver down its right side; the shaded side down its left
            sw = w*rnd.uniform(0.2, 0.38)
            lits.append(P([top[2], (top[2][0] - sw, top[2][1] + 3), (bot[0][0] - sw, bot[0][1] - 3), bot[0]]))
            if rnd.random() < 0.6: darks.append(P([top[0], (top[0][0] + w*0.25, top[0][1] + 6), (bot[2][0] + w*0.25, bot[2][1] - 4), bot[2]]))
            ya = yb + rnd.uniform(-4, 8)
        x += w
    ic.path(' '.join(lits), lit, 0.2*o); ic.path(' '.join(darks), dark, 0.32*o)
    ic.raw('</g>')

def build(v):
    rnd = random.Random(v.get('seed', 15)); ic = Icon()
    # ---- the sky: the app's plum night and its soft glow
    ic.path(P([(-128, -128), (640, -128), (640, 320), (-128, 320)]), ic.lin(0, -128, 0, 300, [(0, '#120826', 1), (0.45, '#2a1448', 1), (0.8, '#5a2a78', 1), (1, '#8a4a96', 1)]))
    ic.circle(400, 60, 240, ic.rad([(0, '#d070d0', 0.32), (0.5, '#9a48b0', 0.12), (1, '#9a48b0', 0)]), sx=1.2, sy=0.9)
    for x, y, r, o in v['stars']: ic.circle(x, y, r, '#f4ecff', o)
    cx, cy, r = v['moon']; G['crescent'](ic, cx, cy, r, ox=-r*0.55, oy=-r*0.36, r2=r*0.86)
    # ---- far ridges, hazy
    n1 = Noise(3, 5); n2 = Noise(4, 5)
    ic.path(smooth([(x, 176 - 22*n1(x/110) - 6*n1(x/23)) for x in range(-130, 660, 10)], close=False) + " L640,330 L-128,330 Z", '#7a4490')
    ic.path(smooth([(x, 188 - 12*n2(x/80) - 5*n2(x/19)) for x in range(-130, 660, 10)], close=False) + " L640,330 L-128,330 Z", '#62347e')
    # ---- the far cliff: a plateau the river crosses, dropping in one sheet from its lip
    tn = Noise(11, 4)
    ftop = [(x, LIP - 2 + 3*tn(x/20) - (0 if LX0 - 8 < x < LX1 + 8 else 4 + 3*abs(tn(x/9)))) for x in range(60, 460, 6)]
    fpoly = ftop + [(460, H + 4), (60, H + 4)]
    ic.path(P(fpoly), ic.lin(0, LIP, 0, H, [(0, '#54307a', 1), (1, '#2e1850', 1)]))
    facets(ic, rnd, P(fpoly), 60, 460, LIP, H, '#9a78c8', '#1a0c30', 0, o=0.6)
    # tiny firs along the plateau's edge, away from the river
    sil = []
    for x in [92, 104, 118, 150, 166, 184, 200, 312, 330, 346, 372, 392, 410, 432]:
        s_, l_ = fir(x, LIP + 1 + 3*tn(x/20) - 4, rnd.uniform(14, 24), n=3); sil.append(s_)
    ic.path(' '.join(sil), '#3a1e58')
    # ---- the drop: the app's gel, pale lilac to white
    def edges(y):
        t = (y - LIP)/(H - LIP); k = t**1.2
        return LX0 + (FX0 - LX0)*k, LX1 + (FX1 - LX1)*k
    wn = Noise(61, 3); lft = []; rgt = []
    for i in range(25):
        y = LIP + (H - LIP)*i/24; a, b = edges(y); rag = (i/24)**2*4
        lft.append((a + rag*wn(i*0.9), y)); rgt.append((b + rag*wn(i*0.9 + 20), y))
    body = rgt + lft[::-1]
    ic.path(smooth(body), ic.lin(0, LIP, 0, H, [(0, '#e8dcff', 1), (0.3, '#d2bef6', 1), (0.7, '#ece2ff', 1), (1, '#ffffff', 1)]))
    cid = ic.gid('w'); ic.defs.append(f'<clipPath id="{cid}"><path d="{smooth(body)}"/></clipPath>'); ic.raw(f'<g clip-path="url(#{cid})">')
    ic.path(P([(200, LIP - 4), (258, LIP - 4), (252, H), (190, H)]), ic.lin(0, 0, 0, H, [(0, '#7a58b8', 0.38), (1, '#7a58b8', 0.12)]))
    for i in range(18):
        u = rnd.uniform(0.03, 0.97)
        def xf(y, u=u):
            a, b = edges(y); return a + (b - a)*u
        if i % 2: strip(ic, xf, LIP - 10, H - rnd.uniform(0, 50), rnd.uniform(1.5, 3), '#8a6cc8', rnd.uniform(0.2, 0.35), 0.12)
        else: strip(ic, xf, LIP - 10, H - rnd.uniform(0, 50), rnd.uniform(2, 4.5), '#ffffff', rnd.uniform(0.35, 0.65), 0.12)
    def gx(y, u): a, b = edges(y); return a + (b - a)*u
    gl = [(gx(y, 0.66), y) for y in range(LIP + 4, H - 60, 12)] + [(gx(y, 0.9), y) for y in range(H - 60, LIP + 3, -12)]
    ic.path(smooth(gl), ic.lin(0, LIP, 0, H - 60, [(0, '#ffffff', 0.85), (0.6, '#ffffff', 0.3), (1, '#ffffff', 0)]))
    ic.path(P([(150, H - 110), (360, H - 110), (360, H + 2), (150, H + 2)]), ic.lin(0, H - 110, 0, H, [(0, '#ffffff', 0), (1, '#ffffff', 0.95)]))
    ic.raw('</g>')
    ic.path(smooth([(LX0 - 2, LIP + 1), (LX0 + 8, LIP - 4), (256, LIP - 5), (LX1 - 8, LIP - 4), (LX1 + 2, LIP + 1), (LX1, LIP + 12), (256, LIP + 14), (LX0, LIP + 12)]),
            ic.lin(0, LIP - 5, 0, LIP + 14, [(0, '#ffffff', 1), (0.45, '#f4eeff', 0.9), (1, '#e4d6ff', 0)]))
    # haze lying at the far cliff's foot
    ic.path(P([(-128, H - 70), (640, H - 70), (640, H + 2), (-128, H + 2)]), ic.lin(0, H - 70, 0, H, [(0, '#c8a8e8', 0), (1, '#c8a8e8', 0.4)]))
    # ---- the near cliffs, framing the view: dark, overhanging, the moon catching the edges that face it
    ln = Noise(23, 5); rn = Noise(29, 5)
    def lx(y):   # the left cliff's edge facing the drop: a bulge, an undercut, a foot
        t = (y + 40)/(H + 44)
        return 96 + 64*math.sin(math.pi*min(1, t*1.6))**1.5 - 30*t + 22*ln(t*4) + 6*ln(t*17)
    lt = [(x, 56 + 14*ln(x/60 + 3) + 6*ln(x/13) + 0.004*(x - 40)**2*(x > 40)) for x in range(-140, 100, 8)]
    lstart = lt[-1]
    left = lt + [(lx(y), y) for y in range(int(lstart[1]) + 8, H + 8, 8)] + [(lx(H + 6) + 20, H + 6), (-140, H + 6)]
    def rx(y):
        t = (y - 120)/(H - 116)
        return 414 - 46*math.sin(math.pi*min(1, t*1.4))**1.3 + 24*t + 16*rn(t*4) + 5*rn(t*15)
    rt = [(x, 112 + 12*rn(x/50 + 7) + 5*rn(x/11)) for x in range(660, 420, -8)]
    right = rt + [(rx(y), y) for y in range(int(rt[-1][1]) + 8, H + 8, 8)] + [(rx(H + 6) - 20, H + 6), (660, H + 6)]
    for poly, g0, g1 in ((left, '#2a1844', '#120a22'), (right, '#24143c', '#0e061c')):
        ic.path(smooth(poly), ic.lin(0, 0, 0, H, [(0, g0, 1), (1, g1, 1)]))
    facets(ic, rnd, smooth(left), -140, 170, 40, H, '#8a6ab8', '#06020e', 0)
    facets(ic, rnd, smooth(right), 360, 660, 120, H, '#6a5098', '#06020e', 0)
    # rim light: the left cliff's edge facing the moon
    rim = [(lx(y), y) for y in range(int(lstart[1]) + 8, H, 6)]
    ic.path(smooth(rim + [(x - 4 - 2*math.sin(y/9), y) for x, y in rim[::-1]]), '#d8b8f8', 0.45)
    ic.path(P(lt + [(x, y + 3) for x, y in lt[::-1]]), '#d8b8f8', 0.35)
    ic.path(P(rt + [(x, y + 3) for x, y in rt[::-1]]), '#d8b8f8', 0.3)
    # firs on the near cliffs
    sil = []; lit = []
    for x, h in v['trees']:
        top = lt if x < 256 else rt
        base = min(yy for xx, yy in top if abs(xx - x) < 9) + 5
        s_, l_ = fir(x, base, h, n=5); sil.append(s_); lit.append(l_)
    ic.path(' '.join(sil), '#0e0618'); ic.path(' '.join(lit), '#3a2660', 0.5)
    # ---- the pool
    ic.path(P([(-128, H), (640, H), (640, 640), (-128, 640)]), ic.lin(0, H, 0, 512, [(0, '#5a3482', 1), (0.35, '#2e1850', 1), (1, '#120826', 1)]))
    ic.path(P([(-128, H - 0.5), (640, H - 0.5), (640, H + 2), (-128, H + 2)]), ic.lin(-128, 0, 640, 0, [(0, '#d8c0ff', 0.05), (0.5, '#f0e6ff', 0.6), (1, '#d8c0ff', 0.05)]))
    for i, (y, w) in enumerate([(410, 90), (428, 74), (450, 58), (476, 42), (504, 28)]):
        ic.path(f"M{f(256-w/2)},{y} Q256,{f(y-2.4)} {f(256+w/2)},{y} Q256,{f(y+2.4)} {f(256-w/2)},{y} Z", '#eadcff', 0.5 - i*0.08)
    ripples(ic, 256, H + 6, [(76, 3, 0.65), (112, 2.4, 0.42), (150, 1.8, 0.24)], col='#efe4ff')
    sp = ic.rad([(0, '#ffffff', 0.95), (0.55, '#f6f0ff', 0.5), (1, '#f0e8ff', 0)])
    for dx, dy, rr, sx, sy in v['mist']: ic.circle(256 + dx, H + dy, rr, sp, sx=sx, sy=sy)
    ic.circle(256, 250, 660, ic.rad([(0.31, '#0a0418', 0), (0.7, '#0a0418', 0.5), (1, '#0a0418', 0.6)]))
    return ic

V = {'seed': 15, 'moon': (352, 100, 33),
     'stars': [(170, 40, 1.6, .8), (230, 18, 1.2, .6), (260, 84, 1.3, .7), (310, 30, 1.0, .5), (330, 128, 1.0, .45), (210, 140, 1.0, .5), (480, 160, 1.1, .5)],
     'trees': [(-100, 54), (-66, 72), (-32, 58), (4, 76), (40, 60), (72, 46), (470, 58), (506, 80), (544, 66), (584, 88), (622, 70)],
     'mist': [(-46, -6, 26, 1.5, 0.7), (46, -6, 26, 1.5, 0.7), (-22, -16, 34, 1.3, 0.8), (24, -14, 34, 1.3, 0.8), (4, -28, 36, 1.1, 0.9), (0, -2, 52, 1.7, 0.6)]}
open(OUT, 'w').write(build(V).svg())
print(OUT)
