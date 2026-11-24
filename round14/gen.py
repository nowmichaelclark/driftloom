# Round 14: the gorge, so it reads at once as falling water rather than a glowing peak: cool white water with strands,
# a flat cliff top it pours over, stepped rock faces, a plume of spray at the foot.
# python3 gen.py out.svg   (shares round12's helpers)
import math, os, sys, random
HERE = os.path.dirname(os.path.abspath(__file__))
full = open(os.path.join(HERE, '..', 'round12', 'gen.py')).read()
src = full[:full.index('# the moon')] + full[full.index('def fir('):full.index('def gel_stream(')] \
    + full[full.index('def ripples('):full.index('# ======')] + full[full.index('def crescent('):full.index('def design1(')]
G = {'__name__': 'r14', '__file__': os.path.join(HERE, '..', 'round12', 'gen.py')}
exec(compile(src, 'round12/gen.py', 'exec'), G)
Icon, P, smooth, f, fir, ripples = G['Icon'], G['P'], G['smooth'], G['f'], G['fir'], G['ripples']
OUT = sys.argv[1] if len(sys.argv) > 1 else 'round14.svg'

H = 392                       # the pool
TOP = 172                     # the brink
LX0, LX1 = 212, 300           # the water at the brink
FX0, FX1 = 204, 308           # the water at the foot

class Noise:
    """1-D fractal value noise (round 10's)."""
    def __init__(self, seed, oct=4):
        r = random.Random(seed); self.v = [r.uniform(-1, 1) for _ in range(4096)]; self.oct = oct
    def base(self, x):
        i = math.floor(x); t = x - i; t = t*t*(3-2*t)
        return self.v[i % 4096]*(1-t) + self.v[(i+1) % 4096]*t
    def __call__(self, x):
        s, a, fq, norm = 0, 1, 1, 0
        for _ in range(self.oct): s += a*self.base(x*fq + 37.1*fq); norm += a; a *= 0.5; fq *= 2.03
        return s/norm

def interp(pts, x):
    for (x0, y0), (x1, y1) in zip(pts, pts[1:]):
        if min(x0, x1) <= x <= max(x0, x1): return y0 + (y1 - y0)*(x - x0)/((x1 - x0) or 1)
    return pts[-1][1]

def wedge(ic, x, y0, y1, w, fill, o, lean=0, k=0.6):
    """A thin tapered strip (a crack, a strand): pointed at both ends; a small k keeps it even."""
    n = 10; L = []; R = []
    for i in range(n + 1):
        t = i/n; y = y0 + (y1 - y0)*t; hw = w/2*math.sin(math.pi*t)**k; xx = x + lean*t
        L.append((xx - hw, y)); R.append((xx + hw, y))
    ic.path(smooth(L + R[::-1]), fill, o)

def water(ic, rnd):
    def edges(y):
        t = (y - TOP)/(H - TOP); return LX0 + (FX0 - LX0)*t, LX1 + (FX1 - LX1)*t
    en = Noise(31, 3)
    lft = [(LX0 + (FX0 - LX0)*t + 1.6*en(t*6), TOP + (H - TOP)*t) for t in [i/14 for i in range(15)]]
    rgt = [(LX1 + (FX1 - LX1)*t + 1.6*en(t*6 + 9), TOP + (H - TOP)*t) for t in [i/14 for i in range(15)]]
    body = lft[:1] + rgt + lft[::-1]
    ic.path(P(body), ic.lin(0, TOP, 0, H, [(0, '#d6eefa', 1), (0.35, '#b4dcf0', 1), (0.75, '#cfeaf6', 1), (1, '#ffffff', 1)]))
    # the side away from the light
    ic.path(P([(LX0 + (LX1 - LX0)*0.72, TOP), (LX1, TOP), (FX1, H), (FX0 + (FX1 - FX0)*0.72, H)]), ic.lin(0, 0, 0, H, [(0, '#5e8ab8', 0.35), (1, '#5e8ab8', 0.12)]))
    cid = ic.gid('w'); ic.defs.append(f'<clipPath id="{cid}"><path d="{P(body)}"/></clipPath>'); ic.raw(f'<g clip-path="url(#{cid})">')
    # strands: darker grooves and bright ropes of water, long and falling
    for i in range(22):
        u = rnd.uniform(0.04, 0.96); y0 = TOP - 14; y1 = H - rnd.uniform(0, 40)
        xa, xb = edges(y0); x = xa + (xb - xa)*u; xa2, xb2 = edges(y1); lean = (xa2 + (xb2 - xa2)*u) - x
        if i % 2: wedge(ic, x, y0, y1, rnd.uniform(2.5, 5), '#7fb0d6', rnd.uniform(0.2, 0.4), lean, 0.15)
        else: wedge(ic, x, y0, y1, rnd.uniform(3, 7), '#ffffff', rnd.uniform(0.35, 0.7), lean, 0.15)
    ic.raw('</g>')
    # the gloss down the lit side, as on the app's buttons
    g = [(LX0 + (LX1 - LX0)*0.08, TOP + 6), (LX0 + (LX1 - LX0)*0.3, TOP + 6), (FX0 + (FX1 - FX0)*0.27, H - 70), (FX0 + (FX1 - FX0)*0.1, H - 70)]
    ic.path(P(g), ic.lin(0, TOP, 0, H - 70, [(0, '#ffffff', 0.85), (0.6, '#ffffff', 0.3), (1, '#ffffff', 0)]))
    # the river above the brink, coming toward us between its banks
    ic.path(smooth([(LX0 + 10, TOP - 9), (256, TOP - 10), (LX1 - 10, TOP - 9), (LX1 + 4, TOP + 2), (LX0 - 4, TOP + 2)]), ic.lin(0, TOP - 10, 0, TOP, [(0, '#34447e', 1), (1, '#9cc4e4', 1)]))
    # the brink: the water bending over the edge, bright, a little proud of the rock
    ic.path(smooth([(LX0 - 1, TOP), (LX0 + 14, TOP - 4), (256, TOP - 5), (LX1 - 14, TOP - 4), (LX1 + 1, TOP), (LX1 - 1, TOP + 16), (256, TOP + 18), (LX0 + 1, TOP + 16)]),
            ic.lin(0, TOP - 5, 0, TOP + 18, [(0, '#ffffff', 1), (0.4, '#f2faff', 0.9), (1, '#d4ecf8', 0)]))
    # white water at the foot, rising
    ic.path(P([(FX0 - 2, H - 90), (FX1 + 2, H - 90), (FX1 + 2, H), (FX0 - 2, H)]), ic.lin(0, H - 90, 0, H, [(0, '#ffffff', 0), (1, '#ffffff', 0.95)]))

def build(v):
    rnd = random.Random(v.get('seed', 14))
    ic = Icon()
    # ---- sky: a clear night, blue above, a little warmth low down
    ic.path(P([(-128, -128), (640, -128), (640, 260), (-128, 260)]), ic.lin(0, -128, 0, 200, [(0, '#121a44', 1), (0.55, '#2c3470', 1), (0.85, '#5a4c88', 1), (1, '#8a6090', 1)]))
    for x, y, r, o in [(70, 44, 1.6, .8), (150, 22, 1.2, .6), (128, 92, 1.0, .5), (220, 54, 1.4, .7), (262, 112, 1.0, .45), (420, 136, 1.0, .5), (300, 26, 1.1, .55), (460, 40, 1.3, .6)]:
        ic.circle(x, y, r, '#f4f2ff', o)
    cx, cy, r = v['moon']
    G['crescent'](ic, cx, cy, r, ox=r*0.55, oy=-r*0.36, r2=r*0.86)
    # ---- the plateau's far edge: a soft forest band behind the brink
    fb = [(-128, 150)] + [(x, 146 - 10*math.sin(x/37) - 6*math.sin(x/13 + 1)) for x in range(-120, 650, 10)] + [(640, 150)]
    ic.path(smooth(fb, close=False) + f" L640,{TOP+4} L-128,{TOP+4} Z", '#241c48')
    # ---- the cliff wall: one face across the scene, the water pouring from a notch in its top
    tn = Noise(v.get('rim', 5), 4)
    def rugged(x0, x1, y0, y1, n=28):
        out = []
        for i in range(n + 1):
            u = i/n; x = x0 + (x1 - x0)*u
            out.append((x, y0 + (y1 - y0)*u + 2.4*tn(x/26)))
        return out
    Ltop = rugged(-128, 196, 150, 158) + [(LX0 - 5, TOP - 12)]
    Rtop = rugged(640, 316, 147, 157) + [(LX1 + 5, TOP - 12)]
    def edgex(y, side):
        t = (y - TOP)/(H - TOP)
        return (LX0 + (FX0 - LX0)*t + 2) if side < 0 else (LX1 + (FX1 - LX1)*t - 2)
    left = Ltop + [(edgex(TOP, -1), TOP - 2), (edgex(H + 4, -1), H + 4), (-128, H + 4)]
    right = Rtop + [(edgex(TOP, 1), TOP - 2), (edgex(H + 4, 1), H + 4), (640, H + 4)]
    rockL = ic.lin(0, 150, 0, H, [(0, '#4a3e6e', 1), (1, '#241a3c', 1)])
    rockR = ic.lin(0, 150, 0, H, [(0, '#3a3060', 1), (1, '#1c1432', 1)])
    # the water first, so the rock's edge sits over it
    water(ic, rnd)
    ic.path(P(left), rockL); ic.path(P(right), rockR)
    # the faces, as round 10's rock (Mikey liked it), drawn bolder for an icon: beds that dip and waver,
    # joints that don't line up, a lit wedge left of each joint, a shaded wedge under each bed's top
    cr = random.Random(v.get('rock', 17))
    for side, poly, litc in ((-1, left, '#8a78b8'), (1, right, '#6e60a0')):
        cid = ic.gid('c'); ic.defs.append(f'<clipPath id="{cid}"><path d="{P(poly)}"/></clipPath>'); ic.raw(f'<g clip-path="url(#{cid})">')
        # big forms first: buttresses, each with a face turned to the light on its left and a shadow on its right
        x = (-140 if side < 0 else 652); xe = edgex(TOP, side)
        while (x < xe - 20) if side < 0 else (x > xe + 20):
            w = cr.uniform(80, 150); xa, xb = (x, x + w) if side < 0 else (x - w, x); ln = cr.uniform(-10, 10)
            col = [(xa, 100), (xb, 100), (xb + ln, H + 10), (xa + ln, H + 10)]
            ic.path(P(col), ic.lin(xa, 0, xb, 0, [(0, '#06021a', 0.15), (0.05, '#b8a4e0', 0.15 if side < 0 else 0.1), (0.4, '#b8a4e0', 0), (0.75, '#06021a', 0), (1, '#06021a', 0.15)]))
            x = xb if side < 0 else xa
        def bed(y0, seed):
            nz = Noise(seed, 4); dip = cr.uniform(0.03, 0.1)*side
            return lambda x: y0 + (x - 256)*dip + 6*nz(x/50)
        beds = []; yb = TOP - 30 + cr.uniform(0, 10)
        while yb < H + 50: beds.append(bed(yb, int(yb*7) + side)); yb += cr.uniform(34, 60)
        lit = []; dark = []; cracks = []
        for bi in range(len(beds) - 1):
            top_, bot_ = beds[bi], beds[bi + 1]
            x = -150
            while x < 660:
                L = cr.uniform(50, 180)
                if cr.random() < 0.8:
                    pts = [(x + L*u, top_(x + L*u)) for u in [i/16 for i in range(17)]]; th = cr.uniform(1.8, 3.6)
                    cracks.append(P([(px, py - th*math.sin(math.pi*i/16)**0.7/2) for i, (px, py) in enumerate(pts)] +
                                    [(px, py + th*math.sin(math.pi*i/16)**0.7/2) for i, (px, py) in enumerate(pts)][::-1]))
                x += L + cr.uniform(6, 40)
            x = -150 + cr.uniform(0, 30)
            while x < 660:
                x2 = x + cr.uniform(-8, 8); y1 = top_(x); y2 = bot_(x2); nzc = Noise(int(x*13 + bi), 3); l1 = []; l2 = []
                for i in range(13):
                    u = i/12; yy = y1 + (y2 - y1)*u; xx = x + (x2 - x)*u + 2.5*nzc(u*3); w = cr.uniform(0.8, 1.8)*math.sin(math.pi*u)**0.5 + 0.3
                    l1.append((xx - w, yy)); l2.append((xx + w*0.6, yy))
                if cr.random() < 0.85: cracks.append(P(l1 + l2[::-1]))
                nx = x + cr.uniform(26, 70)
                if cr.random() < 0.65:
                    sw = (nx - x)*cr.uniform(0.18, 0.45)
                    lit.append(P([(x + 1.5, y1 + 1), (x + 1.5 + sw, y1 + 1 + cr.uniform(0, 4)), (x2 + 1.5 + sw*cr.uniform(0.2, 0.8), y2 - 2), (x2 + 1.5, y2 - 1)]))
                if cr.random() < 0.45:
                    dark.append(P([(x + 2, y1 + 1.5), (nx - 1, top_(nx) + 1.5), (nx - 1, top_(nx) + cr.uniform(6, 14)), (x + 2, y1 + cr.uniform(4, 10))]))
                x = nx
            # each bed a little darker toward its foot
            xs = [-150 + i*20 for i in range(42)]
            band = [(x, top_(x)) for x in xs] + [(x, bot_(x)) for x in xs[::-1]]
            ic.path(P(band), ic.lin(0, top_(256) - 10, 0, bot_(256) + 10, [(0, '#000000', 0), (1, '#06021a', 0.35)]))
        ic.path(' '.join(lit), litc, 0.38)
        ic.path(' '.join(dark), '#08041a', 0.45)
        ic.path(' '.join(cracks), '#06020f', 0.75)
        ic.raw('</g>')
    # the rock beside the water: in the water's shadow close in, its cool light a little further out
    for side in (-1, 1):
        pts = [(edgex(y, side), y) for y in (TOP, H + 4)]
        o = [(x + side*30, y) for x, y in pts]
        ic.path(P(pts + o[::-1]), ic.lin(pts[0][0], 0, pts[0][0] + side*30, 0, [(0, '#0a0618', 0.6), (1, '#0a0618', 0)]))
    ic.circle(256, 300, 150, ic.rad([(0, '#a8d8f0', 0.18), (0.7, '#a8d8f0', 0.05), (1, '#a8d8f0', 0)]), sx=1, sy=1.3)
    # the cliff's top edge in the moonlight
    for top, d in ((Ltop, 1), (Rtop, -1)):
        ic.path(P(top + [(x, y + 5) for x, y in top[::-1]]), '#e8d2e8', 0.5); ic.path(P(top + [(x, y + 1.8) for x, y in top[::-1]]), '#fff0f4', 0.6)
    # firs standing on the cliff top, away from the water
    sil = []; lit = []
    for x, h in v['trees']:
        base = interp(Ltop if x < 256 else Rtop, x) + 3
        s_, l_ = fir(x, base, h, n=4); sil.append(s_); lit.append(l_)
    ic.path(' '.join(sil), '#0e0a22'); ic.path(' '.join(lit), '#3a3466', 0.6)
    # boulders at the foot, either side of the plume
    for bx, by, bw, bh in v['boulders']:
        pts = [(bx + bw/2*math.cos(a)*(1 + 0.12*math.sin(3*a + bx)), by - bh*max(0, math.sin(a))**0.8 + 2) for a in [math.pi*i/12 for i in range(13)]]
        ic.path(smooth(pts), ic.lin(bx - bw/2, by - bh, bx + bw/3, by, [(0, '#8a7cb4', 1), (0.3, '#2a2246', 1), (1, '#0c0818', 1)]))
    # ---- the pool
    ic.path(P([(-128, H), (640, H), (640, 640), (-128, 640)]), ic.lin(0, H, 0, 512, [(0, '#3a4c86', 1), (0.35, '#1e2650', 1), (1, '#0c1028', 1)]))
    ic.path(P([(-128, H - 0.5), (640, H - 0.5), (640, H + 2), (-128, H + 2)]), ic.lin(-128, 0, 640, 0, [(0, '#c8e4ff', 0.05), (0.5, '#e8f6ff', 0.6), (1, '#c8e4ff', 0.05)]))
    # the falls' reflection, broken
    for i, (y, w) in enumerate([(410, 96), (428, 80), (450, 62), (476, 44), (504, 30)]):
        ic.path(f"M{f(256-w/2)},{y} Q256,{f(y-2.4)} {f(256+w/2)},{y} Q256,{f(y+2.4)} {f(256-w/2)},{y} Z", '#d8eeff', 0.55 - i*0.09)
    ripples(ic, 256, H + 6, [(80, 3, 0.7), (118, 2.4, 0.45), (158, 1.8, 0.25)], col='#e0f2ff')
    # the plume: spray boiling up where the water lands
    sp = ic.rad([(0, '#ffffff', 0.95), (0.55, '#f4faff', 0.5), (1, '#eef8ff', 0)])
    for dx, dy, rr, sx, sy in v['plume']:
        ic.circle(256 + dx, H + dy, rr, sp, sx=sx, sy=sy)
    # a soft vignette
    ic.circle(256, 250, 660, ic.rad([(0.31, '#06041a', 0), (0.7, '#06041a', 0.5), (1, '#06041a', 0.6)]))
    return ic

V = {'moon': (378, 82, 40), 'seed': 14,
     'boulders': [(148, H + 2, 64, 22), (188, H + 2, 36, 14), (334, H + 2, 40, 15), (378, H + 2, 74, 25), (-60, H + 2, 90, 16), (560, H + 2, 80, 14)],
     'ledges': [TOP - 26, 232, 300],
     'joints': ([-40, 30, 96, 150, 184], [334, 372, 420, 486, 556]),
     'trees': [(-80, 70), (-46, 92), (-10, 62), (30, 84), (66, 60), (104, 48), (134, 36),
               (390, 38), (420, 50), (458, 62), (494, 86), (532, 64), (566, 92), (604, 70)],
     'plume': [(-50, -6, 30, 1.5, 0.7), (50, -6, 30, 1.5, 0.7), (-24, -16, 38, 1.3, 0.8), (24, -14, 38, 1.3, 0.8), (0, -28, 40, 1.1, 0.9), (0, -2, 56, 1.6, 0.6)]}
open(OUT, 'w').write(build(V).svg())
print(OUT)
