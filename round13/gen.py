# Round 13: the gorge (round 12's 1), refined.
# python3 gen.py out.svg   (needs ../round12/gen.py for the shared helpers, ../round9/gen.py for the moon's map)
import math, os, sys, random
HERE = os.path.dirname(os.path.abspath(__file__))
src = open(os.path.join(HERE, '..', 'round12', 'gen.py')).read()
src = src[:src.index('# ============================================================ 1.')]
G = {'__name__': 'r12', '__file__': os.path.join(HERE, '..', 'round12', 'gen.py')}
full = open(os.path.join(HERE, '..', 'round12', 'gen.py')).read()
src += full[full.index('def crescent('):full.index('def design1(')]
exec(compile(src, 'round12/gen.py', 'exec'), G)
Icon, P, smooth, f, fir, ripples = G['Icon'], G['P'], G['smooth'], G['f'], G['fir'], G['ripples']

OUT = sys.argv[1] if len(sys.argv) > 1 else 'r13.svg'
H = 392                      # the pool's surface
BX0, BX1, BY = 222, 290, 200 # the brink: its two corners, its height
FX0, FX1 = 198, 314          # the stream's foot

def interp(pts, x):
    for (x0, y0), (x1, y1) in zip(pts, pts[1:]):
        if min(x0, x1) <= x <= max(x0, x1): return y0 + (y1 - y0)*(x - x0)/((x1 - x0) or 1)
    return pts[-1][1]

def stream(ic):
    """The stream as a gel, like the app's Play button: smooth body, a shaded side, a gloss down the lit side, white at the foot."""
    def at(t):
        k = t**2.0; xl = BX0 + (FX0 - BX0)*k; xr = BX1 + (FX1 - BX1)*k
        return xl, xr, BY + (H - BY)*t
    def band(u0, u1, t0, t1, n=30, taper=True):
        L = []; R = []
        for i in range(n + 1):
            t = t0 + (t1 - t0)*i/n; tap = math.sin(math.pi*i/n)**0.35 if taper else 1
            xl, xr, y = at(t); um = (u0 + u1)/2; uh = (u1 - u0)/2*tap
            L.append((xl + (xr - xl)*(um - uh), y)); R.append((xl + (xr - xl)*(um + uh), y))
        return smooth(L + R[::-1])
    ic.path(band(0, 1, 0, 1, taper=False), ic.lin(0, BY, 0, H, [(0, '#e09468', 1), (0.1, '#f6c08c', 1), (0.5, '#fde0be', 1), (1, '#fff4e6', 1)]))
    ic.path(band(0.7, 1.0, 0, 1, taper=False), '#c8785e', 0.4)          # the side away from the light
    ic.path(band(0.88, 1.0, 0, 1, taper=False), '#9a5458', 0.3)
    ic.path(band(0.33, 0.39, 0.03, 0.9), '#e8a07a', 0.35)                # two soft folds
    ic.path(band(0.6, 0.65, 0.06, 0.85), '#e8a07a', 0.3)
    ic.path(band(0.07, 0.3, 0.0, 0.62), ic.lin(0, BY, 0, BY + (H - BY)*0.62, [(0, '#ffffff', 0.92), (0.65, '#ffffff', 0.4), (1, '#ffffff', 0.08)]))
    ic.path(band(0.44, 0.54, 0.02, 0.45), '#ffffff', 0.32)
    ic.path(band(-0.01, 1.01, 0.48, 1, taper=False), ic.lin(0, BY + (H - BY)*0.48, 0, H, [(0, '#ffffff', 0), (1, '#ffffff', 0.85)]))
    # the brink: the water bending over, a bright line across it
    hl = [(BX0 + (BX1 - BX0)*u, BY + 1.5 + 1.2*math.sin(math.pi*u)) for u in [i/16 for i in range(17)]]
    ic.path(P(hl + [(x, y + 2.2*math.sin(math.pi*i/16)**0.5) for i, (x, y) in enumerate(hl)][::-1]), '#fff8ee', 0.9)

def build(v):
    ic = Icon()
    # ---- sky: deep at the top, a warm glow low in the gap behind the brink
    ic.path(P([(-128, -128), (640, -128), (640, H + 2), (-128, H + 2)]), ic.lin(0, -128, 0, H, [(0, '#22143e', 1), (0.4, '#4a2a66', 1), (0.72, '#9a4c70', 1), (1, '#e0846c', 1)]))
    ic.circle(256, 210, 170, ic.rad([(0, '#ffbc84', 0.8), (0.45, '#f4986e', 0.35), (1, '#e08070', 0)]), sx=1.05, sy=1.0)
    for x, y, r, o in [(150, 40, 1.6, .8), (196, 92, 1.1, .55), (232, 34, 1.3, .65), (360, 70, 1.0, .45), (330, 150, 1.0, .4), (180, 150, 1.1, .45), (275, 120, 0.9, .4)]:
        ic.circle(x, y, r, '#fff6e4', o)
    # the crescent
    cx, cy, r = v['moon']
    G['crescent'](ic, cx, cy, r, ox=r*0.55, oy=-r*0.36, r2=r*0.86)
    # ---- far ridges in the gap, behind the brink: where the river comes from
    ic.path(smooth([(150, 206), (196, 176), (226, 186), (258, 168), (296, 184), (330, 172), (372, 206)], close=False) + " L372,260 L150,260 Z", '#b06878')
    ic.path(smooth([(150, 214), (210, 192), (250, 200), (300, 188), (370, 214)], close=False) + " L370,260 L150,260 Z", '#7c3e62')
    # ---- the stream
    stream(ic)
    # ---- the cliffs, drawn over the stream's edges so it comes out between them
    Ltop = [(-128, 16), (-40, 24), (24, 46), (64, 52), (98, 86), (134, 94), (166, 136), (196, 146), (214, 180), (BX0 + 2.5, BY - 1)]
    def sedge(side, inset):
        out = []
        for i in range(13):
            t = i/12; k = t**2.0; y = BY + (H - BY)*t
            x = (BX0 + (FX0 - BX0)*k + inset) if side < 0 else (BX1 + (FX1 - BX1)*k - inset)
            out.append((x, y if i else BY - 1))
        out[-1] = (out[-1][0], H + 4); return out
    Lin = sedge(-1, 2.5)
    Rtop = [(640, 6), (560, 14), (494, 38), (456, 44), (420, 80), (384, 88), (352, 130), (322, 140), (302, 178), (BX1 - 2.5, BY - 1)]
    Rin = sedge(1, 2.5)
    left = Ltop + Lin[1:] + [(-128, H + 4)]
    right = Rtop + Rin[1:] + [(640, H + 4)]
    ic.path(P(left), ic.lin(0, 0, 0, H, [(0, '#2e1a48', 1), (1, '#140a22', 1)]))
    ic.path(P(right), ic.lin(0, 0, 0, H, [(0, '#28163e', 1), (1, '#10061a', 1)]))
    # planes: the inner faces, warm in the stream's glow; a ledge plane on each
    # facets: broad columns down from the ledges' corners, lighter toward the gap, darker outward
    def column(top, xa, xb, lean, fill, o):
        ya = interp(top, xa); yb = interp(top, xb)
        ic.path(P([(xa, ya + 2), (xb, yb + 2), (xb + lean, H + 4), (xa + lean, H + 4)]), ic.lin(0, min(ya, yb), 0, H, [(0, fill, o), (1, fill, o*0.25)]))
    column(Ltop, 98, 166, -12, '#3a2256', 0.45)
    column(Ltop, 166, 214, -8, '#4a2c64', 0.4)
    column(Ltop, -128, 24, 0, '#0c0616', 0.35)
    column(Rtop, 352, 420, 12, '#140a22', 0.3)
    column(Rtop, 494, 640, 0, '#08030e', 0.4)
    # rim light along the cliffs' tops
    for top, d in ((Ltop, 1), (Rtop, -1)):
        ic.path(P(top + [(x + d*0.6, y + 3.6) for x, y in top[::-1]]), '#f8b088', 0.7)
    # the stream's glow on the rock around it
    ic.circle(256, 300, 120, ic.rad([(0, '#ffb890', 0.28), (0.6, '#f09880', 0.1), (1, '#f09880', 0)]), sx=0.95, sy=1.35)
    # firs on the rims and ledges, standing on the edge
    trees = v['trees']
    sil = []; lit = []
    for x, h in trees:
        top = Ltop if x < 256 else Rtop
        base = interp(top, x) + h*0.06 + 3
        s_, l_ = fir(x, base, h, n=v.get('tiers', 4)); sil.append(s_); lit.append(l_)
    ic.path(' '.join(sil), '#120820'); ic.path(' '.join(lit), '#3c2254', 0.6)
    # ---- the pool
    ic.path(P([(-128, H), (640, H), (640, 640), (-128, 640)]), ic.lin(0, H, 0, 512, [(0, '#5c3062', 1), (0.3, '#34194a', 1), (1, '#140a22', 1)]))
    ic.path(P([(-128, H - 0.5), (640, H - 0.5), (640, H + 2), (-128, H + 2)]), ic.lin(0, 0, 512, 0, [(0, '#f0a080', 0.1), (0.5, '#ffd0a8', 0.6), (1, '#f0a080', 0.1)]))
    # the stream's light in the water
    ic.circle(256, H + 46, 60, ic.rad([(0, '#ffd8b0', 0.45), (1, '#ffd8b0', 0)]), sx=0.75, sy=1.1)
    for i, (y, w) in enumerate([(404, 92), (420, 72), (440, 56), (464, 40), (492, 28)]):
        ic.path(f"M{f(256-w/2)},{y} Q256,{f(y-2.2)} {f(256+w/2)},{y} Q256,{f(y+2.2)} {f(256-w/2)},{y} Z", '#ffecd0', 0.55 - i*0.1)
    # spray at the foot, over the bottom edge
    sp = ic.rad([(0, '#ffffff', 0.8), (0.5, '#fff4e8', 0.35), (1, '#fff4e8', 0)])
    for dx, dy, rr in [(-44, -8, 30), (44, -8, 30), (-18, -14, 38), (20, -12, 36), (0, -2, 52)]:
        ic.circle(256 + dx, H + dy, rr, sp, sx=1.4, sy=0.55)
    ripples(ic, 256, H + 4, [(66, 3, 0.85), (100, 2.4, 0.55), (138, 1.8, 0.32)])
    # a soft vignette, so the light in the middle carries
    ic.circle(256, 240, 660, ic.rad([(0.31, '#0a0414', 0), (0.7, '#0a0414', 0.55), (1, '#0a0414', 0.6)]))
    return ic

V = {'moon': (324, 88, 46),
     'trees': [(-70, 64), (-36, 84), (0, 58), (44, 80), (70, 56), (120, 50), (142, 38),
               (580, 60), (548, 82), (510, 58), (470, 78), (440, 52)]}
open(OUT, 'w').write(build(V).svg())
print(OUT)
