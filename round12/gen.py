# Round 12: three simpler directions, drawn to the app-icon guidelines.
# python3 gen.py   (writes r12-1.svg, r12-2.svg, r12-3.svg; needs ../round9/gen.py for the moon's map)
#
# The guidelines held (Android adaptive icons, Material product icons, Apple's):
# one focal point, a few bold shapes, no text, no fine detail; a full-bleed
# background (drawn out to the 108 dp layer, -128..640); the key shapes inside
# Android's 66 dp safe circle (radius 235 here, centered) and the maskable
# icon's 40% circle; one light, from the upper left; readable at 48 and 16 px
# and as one color.
# Only what Android's vector drawables draw: paths, circles, linear and radial
# gradients (radial on circles), per-shape opacity, group transforms.
import math, random, sys, os
HERE = os.path.dirname(os.path.abspath(__file__))
def f(x): return f"{x:.1f}".rstrip('0').rstrip('.')
def P(pts, close=True): return "M" + " L".join(f"{f(x)},{f(y)}" for x, y in pts) + (" Z" if close else "")
def smooth(pts, close=True):
    n = len(pts); d = f"M{f(pts[0][0])},{f(pts[0][1])}"
    for i in (range(n) if close else range(n - 1)):
        p0 = pts[i-1] if (close or i > 0) else pts[i]; p1 = pts[i]; p2 = pts[(i+1) % n]
        p3 = pts[(i+2) % n] if (close or i + 2 < n) else p2
        c1 = (p1[0]+(p2[0]-p0[0])/6, p1[1]+(p2[1]-p0[1])/6); c2 = (p2[0]-(p3[0]-p1[0])/6, p2[1]-(p3[1]-p1[1])/6)
        d += f" C{f(c1[0])},{f(c1[1])} {f(c2[0])},{f(c2[1])} {f(p2[0])},{f(p2[1])}"
    return d + (" Z" if close else "")

class Icon:
    def __init__(self): self.defs = []; self.body = []; self.n = 0
    def gid(self, p): self.n += 1; return f"{p}{self.n}"
    def lin(self, x1, y1, x2, y2, stops):
        i = self.gid('g'); s = ''.join(f'<stop offset="{o}" stop-color="{c}"' + (f' stop-opacity="{a}"' if a != 1 else '') + '/>' for o, c, a in stops)
        self.defs.append(f'<linearGradient id="{i}" gradientUnits="userSpaceOnUse" x1="{f(x1)}" y1="{f(y1)}" x2="{f(x2)}" y2="{f(y2)}">{s}</linearGradient>'); return f'url(#{i})'
    def rad(self, stops):
        i = self.gid('r'); s = ''.join(f'<stop offset="{o}" stop-color="{c}" stop-opacity="{a}"/>' for o, c, a in stops)
        self.defs.append(f'<radialGradient id="{i}">{s}</radialGradient>'); return f'url(#{i})'
    def path(self, d, fill, o=1): self.body.append(f'<path d="{d}" fill="{fill}"' + (f' opacity="{o:.3g}"' if o < 1 else '') + '/>')
    def circle(self, cx, cy, r, fill, o=1, sx=1, sy=1):
        c = f'<circle cx="{f(cx)}" cy="{f(cy)}" r="{f(r)}" fill="{fill}"' + (f' opacity="{o:.3g}"' if o < 1 else '') + '/>'
        if sx != 1 or sy != 1: c = f'<g transform="translate({f(cx)} {f(cy)}) scale({sx:.4g} {sy:.4g}) translate({f(-cx)} {f(-cy)})">{c}</g>'
        self.body.append(c)
    def raw(self, s): self.body.append(s)
    def svg(self): return f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">\n<defs>{"".join(self.defs)}</defs>\n' + '\n'.join(self.body) + '\n</svg>\n'

# the moon's map from round 9, to place anew
src = open(os.path.join(HERE, '..', 'round9', 'gen.py')).read()
g9 = {'__name__': 'g9'}; argv = sys.argv; sys.argv = ['gen.py', 'harvest', os.devnull, 'v5']
exec(compile(src, 'gen.py', 'exec'), g9); sys.argv = argv
def full_moon(ic, cx, cy, r, seas=True, halo=True):
    if halo: ic.circle(cx, cy, r*2.6, ic.rad([(0.38, '#ffe2a8', 0.32), (1, '#ffd49a', 0)]))
    k = r/70
    ic.defs.append(g9['MOONGRAD']); ic.defs.append('<radialGradient id="limb"><stop offset="0.68" stop-color="#b8804c" stop-opacity="0"/><stop offset="1" stop-color="#b8804c" stop-opacity="0.32"/></radialGradient>')
    ic.defs.append('<clipPath id="moonclip"><circle cx="160" cy="272" r="70"/></clipPath>')
    parts = g9['moon'][1:] if seas else g9['moon'][1:2]
    ic.raw(f'<g transform="translate({f(cx)} {f(cy)}) scale({k:.4f}) translate(-160 -272)">' + ''.join(parts) + '</g>')

def sky(ic, top, mid, low, horizon):
    ic.path(P([(-128, -128), (640, -128), (640, horizon + 1), (-128, horizon + 1)]), ic.lin(0, -128, 0, horizon, [(0, top, 1), (0.55, mid, 1), (1, low, 1)]))
def stars(ic, pts):
    for x, y, r, o in pts:
        ic.circle(x, y, r, '#fff6e4', o)

def fir(x, base, h, n=4, slim=0.5):
    """A clean fir: smooth drooping tiers. Returns the silhouette and its lit left half."""
    top = base - h; W = h*slim; R = []; L = []
    for i in range(n):
        t0 = i/n; t1 = (i + 1)/n; y0 = top + h*0.86*t0; y1 = top + h*0.86*t1
        w0 = (W/2)*(0.08 + 0.92*t0)*0.5; w1 = (W/2)*(0.2 + 0.8*t1)
        for side, out in ((1, R), (-1, L)):
            out += [(x + side*w0, y0 + (h*0.03 if i else 0)), (x + side*(w0 + (w1-w0)*0.6), y0 + (y1-y0)*0.6), (x + side*w1, y1 + h*0.03)]
    tw = h*0.035
    sil = [(x, top)] + R + [(x + tw, R[-1][1]), (x + tw, base), (x - tw, base), (x - tw, L[-1][1])] + L[::-1]
    lit = [(x, top)] + L + [(x, L[-1][1] - h*0.03)]
    return P(sil), P(lit)

def gel_stream(ic, xl0, xr0, y0, xl1, xr1, y1, warm=True, brink=False):
    """The stream as a smooth gel band: body, shaded right side, gloss down the left, white toward the foot."""
    def edge(a, b, t): return a + (b - a)*t
    def band(u0, u1, t0, t1, n=24):
        Lp = []; Rp = []
        for i in range(n + 1):
            t = t0 + (t1 - t0)*i/n; tap = math.sin(math.pi*i/n)**0.3 if (t0 > 0 or t1 < 1) else 1
            k = t**2.2; bulge = 3*math.sin(math.pi*min(1, t*4))*(1 - t)   # it leans out a little at the brink, then flares at the foot
            xl = edge(xl0, xl1, k) - bulge; xr = edge(xr0, xr1, k) + bulge; y = edge(y0, y1, t)
            um = (u0 + u1)/2; uh = (u1 - u0)/2*tap
            Lp.append((xl + (xr - xl)*(um - uh), y)); Rp.append((xl + (xr - xl)*(um + uh), y))
        return smooth(Lp + Rp[::-1])
    body = ic.lin(0, y0, 0, y1, [(0, '#b86c60', 1), (0.07, '#eca470', 1), (0.18, '#f8c896', 1), (0.55, '#fde2c2', 1), (1, '#fff4e6', 1)])
    ic.path(band(0, 1, 0, 1), body)
    ic.path(band(0.68, 1, 0, 1), '#c87a62', 0.42)
    ic.path(band(0.86, 1, 0, 1), '#a05a58', 0.3)
    ic.path(band(0.06, 0.34, 0.01, 0.66), ic.lin(0, y0, 0, y0 + (y1 - y0)*0.66, [(0, '#ffffff', 0.9), (0.7, '#ffffff', 0.35), (1, '#ffffff', 0.1)]))
    ic.path(band(0.44, 0.56, 0.04, 0.5), '#ffffff', 0.3)
    ic.path(band(-0.01, 1.01, 0.5, 1), ic.lin(0, y0 + (y1 - y0)*0.5, 0, y1, [(0, '#ffffff', 0), (1, '#ffffff', 0.8)]))
    # spray at the foot, soft, over the bottom edge
    sp = ic.rad([(0, '#ffffff', 0.75), (0.55, '#fff4e8', 0.3), (1, '#fff4e8', 0)])
    w1 = xr1 - xl1
    for dx, dy, rr in [(-0.42, -6, 0.32), (0.42, -6, 0.32), (-0.15, -10, 0.4), (0.18, -8, 0.38), (0, -2, 0.55)]:
        ic.circle((xl1 + xr1)/2 + dx*w1, y1 + dy, rr*w1, sp, sx=1.3, sy=0.6)
    # the brink: one bright line
    if not brink: return
    hl = [(xl0 + (xr0 - xl0)*u, y0 + 0.3 + 3*(2*u - 1)**4) for u in [i/16 for i in range(17)]]
    ic.path(P(hl + [(x, y + 2.6*math.sin(math.pi*i/16)**0.4) for i, (x, y) in enumerate(hl)][::-1]), '#fffaf0', 0.95)

def lips(ic, xl0, xr0, y0, col):
    """The notch's two rock lips over the stream's top corners, so it comes out between them."""
    ic.path(smooth([(xl0 - 14, y0 - 4), (xl0 + 9, y0 - 3), (xl0 + 5, y0 + 10), (xl0 + 1, y0 + 34), (xl0 - 10, y0 + 40)]), col)
    ic.path(smooth([(xr0 + 14, y0 - 4), (xr0 - 9, y0 - 3), (xr0 - 5, y0 + 10), (xr0 - 1, y0 + 34), (xr0 + 10, y0 + 40)]), col)

def ripples(ic, cx, cy, rings, col='#fff0d4'):
    for rx, w, o in rings:
        ry = rx*0.2
        outer = [(cx + rx*math.cos(a), cy + ry*math.sin(a)) for a in [math.pi*i/24 for i in range(25)]]
        inner = [(cx + (rx - w*0.3)*math.cos(a), cy + (ry - w)*math.sin(a)) for a in [math.pi*i/24 for i in range(24, -1, -1)]]
        ic.path(P(outer + inner), col, o)

# ============================================================ 1. the gorge: a tall stream between two cliffs, a crescent above
def crescent(ic, cx, cy, r, ox=24, oy=-16, r2=None):
    r2 = r2 or r*0.88
    d = math.hypot(ox, oy); a = (r*r - r2*r2 + d*d)/(2*d); h = math.sqrt(r*r - a*a); ux, uy = ox/d, oy/d
    px, py = cx + a*ux, cy + a*uy; p1 = (px - h*uy, py + h*ux); p2 = (px + h*uy, py - h*ux)
    ic.circle(cx, cy, r*2.6, ic.rad([(0.3, '#ffe2a8', 0.3), (1, '#ffd49a', 0)]))
    ic.circle(cx, cy, r, '#7a4a7a', 0.3)
    ic.path(f"M{f(p1[0])},{f(p1[1])} A{f(r)},{f(r)} 0 1 1 {f(p2[0])},{f(p2[1])} A{f(r2)},{f(r2)} 0 0 0 {f(p1[0])},{f(p1[1])} Z",
            ic.lin(cx - r, cy - r, cx + r*0.4, cy + r, [(0, '#fff6dc', 1), (0.5, '#ffdc98', 1), (1, '#f0b468', 1)]))

def design1():
    ic = Icon(); H = 392
    sky(ic, '#1e1236', '#4e2a5c', '#e08a6a', H)
    stars(ic, [(150, 60, 1.6, .8), (300, 40, 1.2, .6), (360, 110, 1.4, .7), (250, 150, 1.0, .5), (200, 110, 1.1, .5), (330, 190, 1.0, .4)])
    crescent(ic, 300, 112, 44)
    # far ridge in the gap, low and warm
    ic.path(smooth([(206, 214), (226, 199), (256, 197), (286, 199), (306, 214)], close=False) + f" L312,{H+2} L200,{H+2} Z", '#1c0e2c')
    # the stream: tall, from a brink high in the gorge
    gel_stream(ic, 228, 282, 196, 212, 300, H)
    # two cliffs, tall, framing it; lit faces toward the gap's light
    Lc = [(-128, 40), (60, 60), (120, 90), (170, 140), (206, 186), (226, 198), (222, 240), (214, 300), (206, H + 4), (-128, H + 4)]
    Rc = [(640, 20), (470, 44), (400, 84), (340, 140), (300, 184), (284, 198), (290, 240), (300, 300), (310, H + 4), (640, H + 4)]
    ic.path(P(Lc), ic.lin(0, 40, 0, H, [(0, '#2e1a46', 1), (1, '#170a24', 1)]))
    ic.path(P(Rc), ic.lin(0, 20, 0, H, [(0, '#28163e', 1), (1, '#12081e', 1)]))
    ic.path(P([(170, 140), (206, 186), (226, 198), (222, 240), (214, 300), (206, H + 2), (180, H + 2), (178, 300), (150, 200)]), '#4a2c62', 0.45)
    ic.path(P([(300, 184), (284, 198), (290, 240), (300, 300), (310, H + 2), (330, H + 2), (326, 280), (312, 210)]), '#0c0616', 0.35)
    for edge in (Lc[:6], Rc[:6]):
        ic.path(P(edge + [(x + (3 if edge is Lc[:6] else -3), y + 2) for x, y in edge[::-1]]), '#e8a078', 0.4)
    # firs on the cliffs' tops
    for x, h, base in [(30, 66, 58), (70, 50, 64), (112, 58, 92), (146, 40, 122), (430, 66, 60), (470, 50, 48), (388, 52, 90), (356, 40, 124)]:
        s_, l_ = fir(x, base + 4, h); ic.path(s_, '#140a22'); ic.path(l_, '#3a2050', 0.6)
    # the pool
    ic.path(P([(-128, H), (640, H), (640, 640), (-128, 640)]), ic.lin(0, H, 0, 512, [(0, '#4a2856', 1), (1, '#140a22', 1)]))
    ic.path(P([(-128, H - 1), (640, H - 1), (640, H + 2), (-128, H + 2)]), '#f0a070', 0.3)
    for i, (y, w) in enumerate([(410, 80), (428, 62), (450, 46), (476, 32)]):
        ic.path(f"M{f(256-w/2)},{y} Q256,{f(y-2.2)} {f(256+w/2)},{y} Q256,{f(y+2.2)} {f(256-w/2)},{y} Z", '#ffecd0', 0.5 - i*0.1)
    ripples(ic, 256, H + 4, [(60, 3, 0.85), (94, 2.4, 0.55), (130, 1.8, 0.3)])
    return ic

# ============================================================ 2. the moon window: the scene in silhouette inside a great moon
def design2():
    ic = Icon(); C = (256, 256); R = 196
    ic.path(P([(-128, -128), (640, -128), (640, 640), (-128, 640)]), ic.lin(0, -128, 0, 640, [(0, '#1a0e30', 1), (0.55, '#3a1e52', 1), (1, '#24123a', 1)]))
    stars(ic, [(40, 50, 1.6, .7), (470, 40, 1.4, .6), (30, 470, 1.2, .5), (486, 480, 1.4, .55), (60, 300, 1.1, .4), (480, 250, 1.2, .45)])
    ic.circle(256, 256, R*1.3, ic.rad([(0.74, '#ffc890', 0.22), (1, '#ffc890', 0)]))
    # the moon: its map, large
    full_moon(ic, 256, 256, R, seas=True, halo=False)
    ic.defs.append(f'<clipPath id="win"><circle cx="256" cy="256" r="{R}"/></clipPath>')
    ic.raw('<g clip-path="url(#win)">')
    H = 360
    # far hills, a warm glow at the horizon line
    ic.path(smooth([(40, 330), (120, 300), (190, 318), (256, 300), (330, 316), (400, 298), (480, 320)], close=False) + f" L480,{H+2} L40,{H+2} Z", '#c0806a', 0.75)
    rock = smooth([(40, 236), (120, 226), (200, 232), (280, 222), (360, 230), (480, 226)], close=False) + f" L480,{H+4} L40,{H+4} Z"
    ic.path(rock, '#2a1640')
    for x, h in [(84, 50), (108, 70), (134, 56), (160, 82), (186, 60), (214, 46), (240, 64), (268, 50), (390, 58), (414, 78), (440, 54), (466, 66)]:
        s_, l_ = fir(x, 232, h); ic.path(s_, '#2a1640')
    ic.path(P([(40, H), (480, H), (480, 480), (40, 480)]), ic.lin(0, H, 0, 460, [(0, '#3a2050', 1), (1, '#1c0e2c', 1)]))
    for i, (y, w) in enumerate([(376, 70), (394, 56), (416, 42), (442, 30)]):
        ic.path(f"M{f(334-w/2)},{y} Q334,{f(y-2.4)} {f(334+w/2)},{y} Q334,{f(y+2.4)} {f(334-w/2)},{y} Z", '#ffecd0', 0.55 - i*0.1)
    gel_stream(ic, 306, 362, 226, 290, 378, H)
    for x, h in [(300, 40), (322, 30), (346, 44), (368, 34)]:
        s_, l_ = fir(x, 236, h); ic.path(s_, '#2a1640')
    ic.path(smooth([(280, 232), (300, 228), (334, 230), (370, 226), (392, 232)], close=False) + " L392,240 L280,240 Z", '#2a1640')
    ripples(ic, 334, H + 4, [(56, 3, 0.85), (88, 2.4, 0.5), (120, 1.8, 0.3)])
    ic.raw('</g>')
    # the moon's rim, a thin bright ring
    ic.circle(256, 256, R + 3, ic.rad([(0.97, '#ffe8b8', 0), (0.985, '#ffe8b8', 0.7), (1, '#ffe8b8', 0)]))
    return ic

# ============================================================ 3. paper layers
def design3():
    ic = Icon(); H = 364
    sky(ic, '#22143c', '#5e3064', '#f0966c', H)
    stars(ic, [(70, 70, 1.6, .75), (140, 120, 1.1, .5), (40, 160, 1.2, .5), (210, 50, 1.2, .55), (460, 60, 1.4, .6), (480, 140, 1.1, .45)])
    full_moon(ic, 330, 150, 64)
    def layer(pts, fill, shadow=0.35, dy=5):
        d = smooth(pts, close=False) + " L640,640 L-128,640 Z"
        ic.raw(f'<g transform="translate(0 {dy})"><path d="{d}" fill="#120818" opacity="{shadow}"/></g>')   # the paper's shadow on the layer behind
        ic.path(d, fill)
    layer([(-128, 280), (-20, 240), (60, 262), (130, 226), (200, 258), (300, 236), (380, 262), (470, 234), (560, 256), (640, 246)], '#b06a7c', 0.2)
    layer([(-128, 300), (0, 286), (100, 298), (170, 274), (240, 300), (340, 284), (420, 300), (520, 280), (640, 296)], '#844868', 0.3)
    # the rock on the left, the rock on the right, the stream between them
    gel_stream(ic, 228, 282, 252, 216, 296, H)
    layer([(-128, 250), (60, 244), (150, 236), (210, 244), (226, 252), (222, 300), (214, 340), (212, H + 2), (640, H + 2)], '#3e2050', 0.0)
    ic.path(P([(212, H + 2), (640, H + 2), (640, 640), (212, 640)]), '#3e2050')   # (the layer above is the left rock; cut it at the stream)
    return ic

def design3b():
    ic = Icon(); H = 364
    sky(ic, '#22143c', '#5e3064', '#f0966c', H)
    stars(ic, [(70, 70, 1.6, .75), (140, 120, 1.1, .5), (40, 160, 1.2, .5), (210, 50, 1.2, .55), (460, 60, 1.4, .6), (480, 140, 1.1, .45)])
    full_moon(ic, 340, 146, 62)
    def shadowed(d, fill, shadow=0.3, dy=6):
        ic.raw(f'<g transform="translate(0 {dy})"><path d="{d}" fill="#120818" opacity="{shadow}"/></g>')
        ic.path(d, fill)
    def ridge(pts): return smooth(pts, close=False) + f" L640,{H+2} L-128,{H+2} Z"
    shadowed(ridge([(-128, 282), (-20, 246), (60, 266), (140, 228), (210, 262), (300, 240), (380, 266), (470, 236), (560, 258), (640, 248)]), '#b06a7c', 0.18)
    shadowed(ridge([(-128, 306), (0, 290), (100, 302), (170, 280), (240, 304), (340, 288), (420, 304), (520, 284), (640, 300)]), '#844868', 0.25)
    # the stream comes down between two rocks
    gel_stream(ic, 226, 286, 244, 212, 302, H)
    leftrock = smooth([(-128, 330), (20, 318), (110, 296), (180, 262), (232, 242)], close=False) + f" L226,300 L220,{H+2} L-128,{H+2} Z"
    rightrock = smooth([(280, 242), (330, 262), (400, 296), (490, 318), (640, 330)], close=False) + f" L640,{H+2} L294,{H+2} L288,300 Z"
    shadowed(leftrock, ic.lin(0, 230, 0, H, [(0, '#4a2860', 1), (1, '#2a1640', 1)]), 0.3)
    shadowed(rightrock, ic.lin(0, 230, 0, H, [(0, '#3c2052', 1), (1, '#20102e', 1)]), 0.3)
    for pts in ([(-128, 254), (40, 246), (130, 236), (196, 234), (232, 242)], [(280, 242), (312, 232), (380, 236), (470, 248), (640, 256)]):
        ic.path(smooth(pts, close=False) + ' ' + smooth([(x, y + 3) for x, y in pts[::-1]], close=False)[0:0] + '', 'none')
    # a fir on each rock
    for x, h, base in [(150, 72, 282), (186, 50, 266), (112, 46, 300), (366, 66, 282), (402, 46, 298)]:
        s, l = fir(x, base, h); ic.path(s, '#1a0c28'); ic.path(l, '#4a2a62', 0.55)
    # the lake: a paper layer too, with the moon's path as three soft strokes
    shadowed(P([(-128, H), (640, H), (640, 640), (-128, 640)]), ic.lin(0, H, 0, 512, [(0, '#4a2856', 1), (1, '#140a22', 1)]), 0.0)
    for i, (y, w) in enumerate([(378, 70), (398, 54), (422, 40), (450, 28)]):
        cx = 340
        ic.path(f"M{f(cx-w/2)},{y} Q{f(cx)},{f(y-2.5)} {f(cx+w/2)},{y} Q{f(cx)},{f(y+2.5)} {f(cx-w/2)},{y} Z", '#ffd890', 0.6 - i*0.12)
    ic.circle(257, H + 2, 50, ic.rad([(0, '#ffffff', 0.8), (0.5, '#fff0dc', 0.3), (1, '#fff0dc', 0)]), sx=1.6, sy=0.3)
    ripples(ic, 257, H + 4, [(56, 3, 0.8), (88, 2.4, 0.5), (122, 1.8, 0.28)])
    return ic

for name, fn in (('r12-1.svg', design1), ('r12-2.svg', design2), ('r12-3.svg', design3b)):
    open(os.path.join(HERE, name), 'w').write(fn().svg())
print('ok')
