# Round 18: the icon as one SVG. Sky, mesa, the stream, the pool and the mist are drawn as vectors with gradients;
# the textured layers (far cliff, near cliffs with talus and junipers, the bank in front) come in as traced shapes.
# python3 build18.py <variation> out.svg   (after layers.py and trace.py for that variation)
import sys, os, math, random
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'round16'))
from geo import *
from ren import V, hexrgb
mx, my, mr = MOON
def moon_d(cx=mx, cy=my, r=mr):
    t = Icon(); crescent(t, cx, cy, r, ox=-r*0.55, oy=-r*0.36, r2=r*0.86); return t.body[-1].split('d="')[1].split('"')[0]
def traced(ic, path):
    s = open(path).read(); ic.raw(s[s.index('<g '):s.rindex('</g>') + 4])
def stops(st): return [(o, c, 1) for o, c in st]
def build(name, d):
    v = V[name]; ic = Icon(); g = geo(); rnd = random.Random(18)
    ic.path(P(sky_rect()), ic.lin(0, -128, 0, H, stops(v['sky'])))
    if v.get('wisps'):
        for x, y, r in [(60, 62, 90), (230, 48, 70), (420, 70, 110)]:
            ic.circle(x, y, r, ic.rad([(0, v['wispc'], 0.55), (1, v['wispc'], 0)]), sx=2.6, sy=0.16)
    if v['obj'] in ('moon', 'moonw'):
        ic.circle(mx, my, mr*2.8, ic.rad([(0, '#fff6e6', 0.35), (1, '#fff6e6', 0)]))
        ic.raw(f'<path d="{moon_d()}" fill="{"#ffffff" if v["obj"] == "moonw" else "#fff4dc"}"/>')
    if v['obj'] == 'star':
        ic.circle(mx, my, 40, ic.rad([(0, '#fff4d8', 0.55), (1, '#fff4d8', 0)]))
        ic.path(P([(mx - 10, my), (mx, my - 2.2), (mx + 10, my), (mx, my + 2.2)]), '#ffffff'); ic.path(P([(mx, my - 10), (mx + 2.2, my), (mx, my + 10), (mx - 2.2, my)]), '#ffffff')
    mesa = [(-130, 200), (-40, 196), (20, 172), (60, 152), (90, 145), (170, 144), (196, 152), (250, 174), (330, 188), (420, 182), (520, 194), (660, 198)]
    ic.path(smooth(mesa, close=False) + f" L640,{H} L-128,{H} Z", ic.lin(0, 140, 0, 240, [(0, v['mesa'], 1), (1, v['haze'], 1)]))
    traced(ic, f'{d}/far.svg')
    ic.path(P([(-128, H - 110), (640, H - 110), (640, H + 2), (-128, H + 2)]), ic.lin(0, H - 110, 0, H, [(0, v['haze'], 0), (1, v['haze'], 0.5)]))
    # ---- the stream: one drop, or two with a ledge pool between; strands, edges thinning, foam lower down, a bright lip
    T = v.get('tier')
    if T:
        segs = [(LIP, T, (LX0, LX1), (LX0 - 4, LX1 + 4)), (T + 6, H, (LX0 - 30, LX1 + 30), (FX0 - 14, FX1 + 14))]
    else:
        segs = [(LIP, H, (LX0, LX1), (FX0, FX1))]
    for si, (y0, y1, (a0, b0), (a1, b1)) in enumerate(segs):
        def gx(y, u, y0=y0, y1=y1, a0=a0, b0=b0, a1=a1, b1=b1):
            t = max(0, min(1, (y - y0)/(y1 - y0)))**1.2; a = a0 + (a1 - a0)*t; bb = b0 + (b1 - b0)*t; return a + (bb - a)*u
        wn = Noise(61 + si, 3)
        body = [(gx(y, 1) + (y - y0)/(y1 - y0)*4*wn(y/30), y) for y in range(y0, y1 + 1, 8)] + [(gx(y, 0) + (y - y0)/(y1 - y0)*4*wn(y/30 + 9), y) for y in range(y1, y0 - 1, -8)]
        bd = smooth(body)
        ic.path(bd, ic.lin(0, y0, 0, y1, [(0, v['water0'], 1), (0.55, v['water1'], 1), (1, '#ffffff', 1)]))
        cid = ic.gid('w'); ic.defs.append(f'<clipPath id="{cid}"><path d="{bd}"/></clipPath>'); ic.raw(f'<g clip-path="url(#{cid})">')
        for i in range(46 if si == len(segs) - 1 else 26):
            u = rnd.uniform(-0.02, 1.02); ya = y0 - 4; yb = y1 - rnd.uniform(0, (y1 - y0)*0.35); w = rnd.uniform(0.8, 2.8)
            pts = [(gx(y, u) + 1.2*math.sin(y/17 + i), y) for y in range(int(ya), int(yb), 10)] + [(gx(yb, u), yb)]
            dark = i % 3 == 0
            ic.path(P(pts + [(x + w, y) for x, y in pts[::-1]]), v['water_dark'] if dark else '#ffffff', rnd.uniform(0.15, 0.35) if dark else rnd.uniform(0.3, 0.7))
        for side in (0, 1):
            u0, u1 = (0, 0.12) if side == 0 else (0.88, 1)
            pts = [(gx(y, u0), y) for y in range(y0, y1 + 1, 14)] + [(gx(y, u1), y) for y in range(y1, y0 - 1, -14)]
            ic.path(P(pts), ic.lin(gx((y0 + y1)/2, u0), 0, gx((y0 + y1)/2, u1), 0, [(0, v['shade'], 0.35 if side == 0 else 0), (1, v['shade'], 0 if side == 0 else 0.35)]))
        gu = (v['gloss'] - 0.1, v['gloss'] + 0.1); ym = y1 - (y1 - y0)*0.3
        gl = [(gx(y, gu[0]), y) for y in range(y0 + 4, int(ym), 10)] + [(gx(y, gu[1]), y) for y in range(int(ym), y0 + 3, -10)]
        ic.path(smooth(gl), ic.lin(0, y0, 0, ym, [(0, '#ffffff', 0.8), (1, '#ffffff', 0)]))
        fh = min(110, (y1 - y0)*0.5)
        ic.path(P([(150, y1 - fh), (360, y1 - fh), (360, y1 + 2), (150, y1 + 2)]), ic.lin(0, y1 - fh, 0, y1, [(0, '#ffffff', 0), (1, '#ffffff', 0.92)]))
        ic.raw('</g>')
        a_, b_ = gx(y0, 0), gx(y0, 1)
        ic.path(smooth([(a_ - 2, y0 + 1), ((a_ + b_)/2, y0 - 4), (b_ + 2, y0 + 1), (b_, y0 + 9), ((a_ + b_)/2, y0 + 10), (a_, y0 + 9)]), ic.lin(0, y0 - 4, 0, y0 + 10, [(0, '#ffffff', 1), (1, '#ffffff', 0.3)]))
    if T:   # the ledge pool between the drops: a lip of rock, foam boiling over it
        ic.path(smooth([(LX0 - 40, T + 4), (256, T - 2), (LX1 + 40, T + 4), (LX1 + 36, T + 12), (256, T + 9), (LX0 - 36, T + 12)]), v['bands'][3][1])
        for dx, rr in ((-30, 16), (-8, 20), (14, 20), (34, 15)):
            ic.circle(256 + dx, T + 3, rr, ic.rad([(0, '#ffffff', 0.95), (0.6, '#ffffff', 0.45), (1, '#ffffff', 0)]), sx=1.4, sy=0.55)
    traced(ic, f'{d}/near.svg')
    # ---- the pool: its color, the cliffs' warm light mirrored softly, the stream's reflection, rings
    pc = v['pool']
    ic.path(P(pool_rect()), ic.lin(0, H, 0, 600, stops(pc)))
    ic.path(P([(-128, H), (640, H), (640, H + 50), (-128, H + 50)]), ic.lin(0, H, 0, H + 50, [(0, v['bands'][-2][1], 0.3), (1, v['bands'][-2][1], 0)]))
    ic.path(P([(FX0 + 14, H), (FX1 - 14, H), (FX1 - 4, H + 110), (FX0 + 4, H + 110)]), ic.lin(0, H, 0, H + 110, [(0, '#ffffff', 0.4), (1, '#ffffff', 0)]))
    ic.path(P([(-128, H - 0.5), (640, H - 0.5), (640, H + 2.5), (-128, H + 2.5)]), ic.lin(-128, 0, 640, 0, [(0, '#ffffff', 0), (0.5, '#ffffff', 0.6), (1, '#ffffff', 0)]))
    for i, (y, w) in enumerate([(H + 16, 120), (H + 36, 96), (H + 60, 70), (H + 90, 48), (H + 124, 30)]):
        ic.path(f"M{f(256 - w/2)},{y} Q256,{f(y - 2)} {f(256 + w/2)},{y} Q256,{f(y + 2)} {f(256 - w/2)},{y} Z", '#ffffff', 0.5 - i*0.08)
    if v['obj'] in ('moon', 'moonw', 'star'):
        for i, y in enumerate([H + 22, H + 44, H + 72]):
            w = 22 - i*5; ic.path(f"M{f(mx - w/2)},{y} Q{mx},{y - 2} {f(mx + w/2)},{y} Q{mx},{y + 2} {f(mx - w/2)},{y} Z", '#fff6e0', 0.5 - i*0.12)
    ripples(ic, 256, H + 6, [(70, 2.4, 0.45), (104, 1.8, 0.28)], col='#ffffff')
    for dx, dy, rr, sx, sy in [(-50, -8, 30, 1.5, 0.7), (48, -8, 30, 1.5, 0.7), (-20, -26, 38, 1.2, 0.8), (22, -24, 36, 1.2, 0.8), (0, -2, 56, 1.7, 0.6), (0, -46, 30, 1.2, 0.9)]:
        ic.circle(256 + dx, H + dy, rr, ic.rad([(0, '#ffffff', 0.85), (0.6, '#ffffff', 0.3), (1, '#ffffff', 0)]), sx=sx, sy=sy)
    if os.path.exists(f'{d}/bank.svg'): traced(ic, f'{d}/bank.svg')
    ic.circle(256, 250, 660, ic.rad([(0.33, v['vig'], 0), (0.75, v['vig'], 0.3), (1, v['vig'], 0.4)]))
    return ic
if __name__ == '__main__':
    name, out = sys.argv[1], sys.argv[2]
    open(out, 'w').write(build(name, f'L_{name}').svg()); print(out, len(open(out).read())//1024, 'KB')
