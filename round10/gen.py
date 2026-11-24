# Round 10: the scene redrawn from procedural, natural shapes.
# python3 gen.py out.svg (needs ../round9/gen.py); node render.mjs out.svg out.png 1024; python3 sheet.py round10.png before.png "a:out.svg:" "b:out.svg:56 72 400 400"
# Only what Android's vector drawables can draw: paths, circles, linear and
# radial gradients (radial on circles), per-shape opacity, group transforms,
# clip paths. No filters, no blur, no group opacity.
import math, random, sys, os
OUT = sys.argv[1] if len(sys.argv) > 1 else 'r10.svg'
VAR = sys.argv[2] if len(sys.argv) > 2 else 'a'
HERE = os.path.dirname(os.path.abspath(__file__))

def f(x): return f"{x:.1f}".rstrip('0').rstrip('.')
def P(pts, close=True):
    return "M" + " L".join(f"{f(x)},{f(y)}" for x, y in pts) + (" Z" if close else "")
def smooth(pts, close=True):
    n = len(pts); d = f"M{f(pts[0][0])},{f(pts[0][1])}"
    rng = range(n) if close else range(n - 1)
    for i in rng:
        p0 = pts[i-1] if (close or i > 0) else pts[i]; p1 = pts[i]; p2 = pts[(i+1) % n]
        p3 = pts[(i+2) % n] if (close or i + 2 < n) else p2
        c1 = (p1[0]+(p2[0]-p0[0])/6, p1[1]+(p2[1]-p0[1])/6); c2 = (p2[0]-(p3[0]-p1[0])/6, p2[1]-(p3[1]-p1[1])/6)
        d += f" C{f(c1[0])},{f(c1[1])} {f(c2[0])},{f(c2[1])} {f(p2[0])},{f(p2[1])}"
    return d + (" Z" if close else "")

class Noise:
    """1-D fractal value noise."""
    def __init__(self, seed, oct=4):
        r = random.Random(seed); self.v = [r.uniform(-1, 1) for _ in range(4096)]; self.oct = oct
    def base(self, x):
        i = math.floor(x); t = x - i; t = t*t*(3-2*t)
        return self.v[i % 4096]*(1-t) + self.v[(i+1) % 4096]*t
    def __call__(self, x):
        s, a, fq, norm = 0, 1, 1, 0
        for _ in range(self.oct): s += a*self.base(x*fq + 37.1*fq); norm += a; a *= 0.5; fq *= 2.03
        return s/norm

# ------------------------------------------------------------------ layout
Y0 = 352            # the waterline
MX, MY, MR = 146, 142, 58
FX, LIPY = 372, 176  # the drop: its center and the lip
if VAR == 'c':       # the drop closer, larger
    FX, LIPY = 366, 150

defs = []; body = []
WR = random.Random(101); wn = Noise(55, 3)
def hw(s): return 23 + 17*s**0.85            # half-width, widening as it drops
def cx_(s): return FX + 3*wn(s*2.2) + 2*s   # its center line, wandering a little
def fy(s): return LIPY + (Y0 - LIPY)*s
def grad_v(id_, y1, y2, stops):
    s = ''.join(f'<stop offset="{o}" stop-color="{c}"' + (f' stop-opacity="{a}"' if a != 1 else '') + '/>' for o, c, a in stops)
    defs.append(f'<linearGradient id="{id_}" gradientUnits="userSpaceOnUse" x1="0" y1="{y1}" x2="0" y2="{y2}">{s}</linearGradient>')
def grad_r(id_, stops):
    s = ''.join(f'<stop offset="{o}" stop-color="{c}" stop-opacity="{a}"/>' for o, c, a in stops)
    defs.append(f'<radialGradient id="{id_}">{s}</radialGradient>')
def path(d, fill, o=1):
    body.append(f'<path d="{d}" fill="{fill}"' + (f' opacity="{o:.3g}"' if o < 1 else '') + '/>')
def soft(cx, cy, r, gid, sx=1, sy=1, o=1):
    c = f'<circle cx="{f(cx)}" cy="{f(cy)}" r="{f(r)}" fill="url(#{gid})"' + (f' opacity="{o:.3g}"' if o < 1 else '') + '/>'
    if sx != 1 or sy != 1:
        c = f'<g transform="translate({f(cx)} {f(cy)}) scale({sx:.3g} {sy:.3g}) translate({f(-cx)} {f(-cy)})">{c}</g>'
    body.append(c)

def mix(a, b, t):
    a = [int(a[i:i+2], 16) for i in (1, 3, 5)]; b = [int(b[i:i+2], 16) for i in (1, 3, 5)]
    return '#' + ''.join(f'{round(a[i]+(b[i]-a[i])*t):02x}' for i in range(3))

# ------------------------------------------------------------------ sky
grad_v('sky', -40, Y0, [(0, '#24163e', 1), (0.42, '#4e2a5a', 1), (0.78, '#a4566a', 1), (1, '#ec9a66', 1)])
body.append(f'<rect x="-128" y="-128" width="768" height="{Y0+128}" fill="url(#sky)"/>')
SR = random.Random(3); stars = []; bigstars = []
for _ in range(70):
    x, y = SR.uniform(-128, 640), SR.uniform(-128, 250)
    if math.hypot(x-MX, y-MY) < MR*1.9: continue
    if y > 200 and SR.random() < 0.6: continue
    r = SR.choice([0.6, 0.7, 0.8, 0.9, 1.1, 1.3]); o = SR.uniform(0.3, 0.85)*(1 - max(0, y-120)/200)
    stars.append((x, y, r, o))
grad_r('star', [(0, '#fff6e4', 0.55), (1, '#fff6e4', 0)])
for x, y, r, o in stars:
    if r > 1.0: soft(x, y, r*4, 'star', o=o*0.8)
    body.append(f'<circle cx="{f(x)}" cy="{f(y)}" r="{r}" fill="#fff6e4" opacity="{o:.2f}"/>')

# ------------------------------------------------------------------ the moon (round 9's map of seas, placed anew)
src = open(os.path.join(HERE, '..', 'round9', 'gen.py')).read()
g9 = {'__name__': 'g9'}; argv = sys.argv; sys.argv = ['gen.py', 'harvest', os.devnull, 'v5']
exec(compile(src, 'gen.py', 'exec'), g9); sys.argv = argv
grad_r('halo', [(0.36, '#ffe2a8', 0.34), (0.6, '#ffd49a', 0.12), (1, '#ffd49a', 0)])
grad_r('halo2', [(0, '#ffdcb0', 0.16), (1, '#ffdcb0', 0)])
soft(MX, MY, 300, 'halo2')
soft(MX, MY, MR*2.5, 'halo')
k = MR/70
defs.append(g9['MOONGRAD'])
defs.append('<radialGradient id="limb"><stop offset="0.68" stop-color="#b8804c" stop-opacity="0"/><stop offset="1" stop-color="#b8804c" stop-opacity="0.32"/></radialGradient>')
defs.append('<clipPath id="moonclip"><circle cx="160" cy="272" r="70"/></clipPath>')
body.append(f'<g transform="translate({MX} {MY}) scale({k:.4f}) translate(-160 -272)">' + ''.join(g9['moon'][1:]) + '</g>')

# a few long thin clouds, lit from below by the moon
def lens(x1, x2, y, t, bend=0):
    xm = (x1+x2)/2
    return f"M{f(x1)},{f(y)} Q{f(xm)},{f(y-t+bend)} {f(x2)},{f(y)} Q{f(xm)},{f(y+t+bend)} {f(x1)},{f(y)} Z"
def wisp(x1, x2, y, t, seed, n=60):
    # a stratus band: a lumpy top, a flatter underside, thinning to nothing at the ends
    nz = Noise(seed, 4); nz2 = Noise(seed + 1, 3); top = []; bot = []
    for i in range(n+1):
        u = i/n; x = x1+(x2-x1)*u; env = math.sin(math.pi*u)**0.8
        w = t*env*(0.55 + 0.9*max(0, 0.5 + nz(u*9)))
        yy = y + 3*nz2(u*2.5)
        top.append((x, yy - w*0.8)); bot.append((x, yy + t*env*0.25 + 0.6*nz2(u*11)))
    return smooth(top + bot[::-1])
grad_r('cloud', [(0, '#f4b08c', 0.5), (0.6, '#f0a888', 0.2), (1, '#f0a888', 0)])
grad_r('cloud2', [(0, '#c87a8c', 0.5), (1, '#c87a8c', 0)])
CL = random.Random(8)
for (cx0, y, n, spread, gid, o) in [(60, 226, 7, 90, 'cloud', 0.55), (240, 246, 5, 60, 'cloud', 0.4), (470, 214, 6, 90, 'cloud2', 0.5), (-60, 252, 4, 60, 'cloud2', 0.4)]:
    for i in range(n):
        x = cx0 + CL.gauss(0, spread*0.5); r = CL.uniform(14, 30)
        soft(x, y + CL.gauss(0, 4) - 6*math.exp(-((x - cx0)/spread)**2), r, gid, sx=CL.uniform(2.0, 3.6), sy=CL.uniform(0.18, 0.32), o=o*CL.uniform(0.5, 1))
# ------------------------------------------------------------------ mountains: fractal ridges, rim-lit on the moon's side
def ridge(x0, x1, y0, y1, rough, seed, depth=8, peaks=()):
    r = random.Random(seed); pts = [(x0, y0), (x1, y1)]; amp = (x1-x0)*0.18
    for _ in range(depth):
        out = [pts[0]]
        for a, b in zip(pts, pts[1:]):
            mx = (a[0]+b[0])/2 + r.uniform(-0.15, 0.15)*(b[0]-a[0]); my = (a[1]+b[1])/2 + r.uniform(-amp, amp)
            out += [(mx, my), b]
        pts = out; amp *= rough
    if peaks:
        pts = [(x, y - sum(h*math.exp(-((x-px)/w)**2) for px, h, w in peaks)) for x, y in pts]
    return pts
def mountain(pts, base, fill, lit, shade, seed, rim=1.6, rimo=0.55):
    poly = pts + [(pts[-1][0], base), (pts[0][0], base)]
    path(P(poly), fill)
    # rim light on slopes that face the moon (rising to the right, moon on the left)... and the shade on the others
    rl = []; sh = []
    r = random.Random(seed)
    for i, (a, b) in enumerate(zip(pts, pts[1:])):
        dy = b[1]-a[1]; dx = b[0]-a[0] or 0.01; s = dy/dx
        side = a[0] > MX
        if (s > 0.25 and side) or (s < -0.25 and not side):
            th = rim*min(1, abs(s))
            rl.append([a, b, (b[0], b[1]+th), (a[0], a[1]+th)])
    if rl: path(' '.join(P(q) for q in rl), lit, rimo)
    # the side of each peak away from the moon in shadow, its edge a fractal spur line
    shadows = []
    for i in range(3, len(pts) - 3):
        x, y = pts[i]
        lo = min(p[1] for p in pts[max(0, i-12):i+13])
        if y != lo or base - y < 14: continue
        d = 1 if x > MX else -1
        foot_x = x + d*(base - y)*r.uniform(0.25, 0.6)
        spur = ridge(0, 1, 0, 0, 0.55, int(x*17), depth=5)
        line = [(x + (foot_x - x)*u + d*0.18*(base - y)*v*4, y + (base - y)*u) for (u, v) in [(p[0], p[1]/max(1, (base-y))) for p in spur]]
        line = [(x + (foot_x - x)*t + 7*Noise(int(x*5), 3)(t*4)*t, y + (base - y)*t) for t in [k/24 for k in range(25)]]
        # the ridge from the peak, on the far side, back to the foot
        j = i; side = []
        while 0 < j < len(pts) - 1 and (pts[j][0] - foot_x)*d < 0:
            side.append(pts[j]); j += d
        if len(side) < 2: continue
        poly = line + [(foot_x + d*2, base), (side[-1][0], base)] + side[::-1]
        shadows.append(P(poly))
    if shadows: path(' '.join(shadows), shade, 0.38)
    # gullies: tapered dark lines dropping from the high points
    gl = []
    for i in range(2, len(pts)-2, 1):
        x, y = pts[i]
        if y < pts[i-2][1] and y < pts[i+2][1] and r.random() < 0.45 and base - y > 18:
            L = (base - y)*r.uniform(0.4, 0.8); lean = r.uniform(-0.35, 0.35)
            nz = Noise(int(x*13), 3); line = []; line2 = []
            for j in range(13):
                u = j/12; yy = y + 2 + L*u; xx = x + lean*L*u + 3*nz(u*3)
                w = 0.8*math.sin(math.pi*min(1, u*1.3))**0.7 + 0.1
                line.append((xx-w, yy)); line2.append((xx+w, yy))
            gl.append(P(line + line2[::-1]))
    if gl: path(' '.join(gl), shade, 0.16)

# farthest range, hazy
far = ridge(-128, 640, 300, 306, 0.56, 11, peaks=((210, 70, 60), (300, 40, 50), (-40, 30, 70), (520, 20, 80)))
mountain(far, Y0, '#98546e', '#e8a28c', '#7a3e62', 11, rim=1.4, rimo=0.5)
grad_v('haze1', 280, Y0, [(0, '#c87878', 0), (1, '#d88a7a', 0.55)])
body.append(f'<rect x="-128" y="280" width="768" height="{Y0-280}" fill="url(#haze1)"/>')
mid = ridge(-128, 640, 326, 322, 0.52, 23, peaks=((40, 46, 50), (110, 30, 40), (250, 26, 45)))
mountain(mid, Y0, '#6e3658', '#c8807e', '#4e2448', 23, rim=1.2, rimo=0.55)
grad_v('haze2', 314, Y0, [(0, '#b86c74', 0), (1, '#c87a78', 0.5)])
body.append(f'<rect x="-128" y="314" width="768" height="{Y0-314}" fill="url(#haze2)"/>')

# ------------------------------------------------------------------ trees
def spruce(x, base, h, seed, slim=0.36, detail=1.0):
    """A spruce's silhouette: a thin leader, then tiers of drooping branches with ragged, uneven tips."""
    r = random.Random(seed); top = base - h; W = h*slim*r.uniform(0.85, 1.15)
    n = max(3, int(h/7*detail) + r.randint(2, 4))
    left = []; right = []
    for side, out in ((-1, left), (1, right)):
        ph = r.uniform(0, 0.5)
        for i in range(n):
            t0 = (i + ph*0.5)/n; t1 = (i + 1)/n
            if t0 > 1: break
            env0 = (W/2)*(0.08 + 0.92*t0**0.95); env1 = (W/2)*(0.08 + 0.92*t1**0.95)
            tip = env1*r.uniform(0.82, 1.18)
            y0 = top + h*0.9*t0; y1 = top + h*0.9*t1
            # inner start of the tier, then needles out to the drooping tip
            out.append((x + side*env0*0.35, y0))
            m = 2 if detail < 1 else 3
            for j in range(1, m+1):
                u = j/(m+1)
                out.append((x + side*(env0*0.35 + (tip - env0*0.35)*u + r.uniform(-1, 1)*0.08*tip), y0 + (y1-y0)*u**1.4 - r.uniform(0, 1.2)))
            out.append((x + side*tip, y1 + r.uniform(0.3, 1.6)*h/60))
            out.append((x + side*tip*0.7, y1 - (y1-y0)*0.12))
    tw = max(0.6, h*0.025)
    pts = [(x + r.uniform(-0.3, 0.3), top - h*0.04)] + right + [(x+tw, base), (x-tw, base)] + left[::-1]
    return P(pts)

def treeline(x0, x1, ground, hmin, hmax, density, seed, detail=0.6, crown_p=0.25, floor=None):
    floor = Y0 + 1 if floor is None else floor
    r = random.Random(seed); nz = Noise(seed, 3); shapes = []; x = x0
    while x < x1:
        g = ground(x); hh = hmin + (hmax-hmin)*max(0, 0.5+0.9*nz(x/30))*r.uniform(0.6, 1.2)
        if r.random() < 0.05: hh *= 1.5
        if r.random() < crown_p:   # a broadleaf crown: a lumpy, uneven mass
            rr = hh*0.32*r.uniform(0.8, 1.3); cy = g - rr*0.9; pts = []
            ph1, ph2 = r.uniform(0, 6.3), r.uniform(0, 6.3)
            for i in range(24):
                a = 2*math.pi*i/24; k = 1 + 0.14*math.sin(3*a + ph1) + 0.1*math.sin(7*a + ph2) + r.uniform(-0.1, 0.1)
                pts.append((x + rr*1.1*k*math.cos(a), cy + rr*0.85*k*math.sin(a)))
            shapes.append(smooth(pts)); x += rr*r.uniform(1.0, 1.6)
        else:
            shapes.append(spruce(x, g+1, hh, int(x*97) ^ seed, slim=r.uniform(0.3, 0.42), detail=detail)); x += hh*r.uniform(0.12, 0.3)*density
    gpts = [(x0, ground(x0)+1)] + [(xx, ground(xx)+1) for xx in range(int(x0), int(x1), 4)] + [(x1, ground(x1)+1), (x1, floor), (x0, floor)]
    return ' '.join(shapes) + ' ' + P(gpts)

if VAR == 'b':
    CLIFF_L = 300
else:
    CLIFF_L = 264
far_ground = lambda x: 338 - 4*math.sin(x/70) - 2*math.sin(x/23)
path(treeline(-128, CLIFF_L+30, far_ground, 9, 16, 1.0, 5, detail=0.5, crown_p=0.2), '#5c2c52')
grad_v('haze3', 326, Y0, [(0, '#a8607a', 0), (1, '#b46c78', 0.45)])
body.append(f'<rect x="-128" y="326" width="{CLIFF_L+158}" height="{Y0-326}" fill="url(#haze3)"/>')
near_ground = lambda x: 348 - 3*math.sin(x/50+1) - 1.5*math.sin(x/17)
path(treeline(-128, CLIFF_L+20, near_ground, 13, 26, 1.1, 9, detail=0.7, crown_p=0.08), '#3e1c40')
# a thin band of mist lying on the far shore
grad_v('shoremist', 338, Y0+2, [(0, '#e8a890', 0), (0.7, '#f0b498', 0.32), (1, '#f0b498', 0)])
body.append(f'<rect x="-128" y="338" width="{CLIFF_L+150}" height="{Y0+2-338}" fill="url(#shoremist)"/>')

# ------------------------------------------------------------------ the lake
grad_v('lake', Y0, 520, [(0, '#4a2856', 1), (0.2, '#2e1840', 1), (1, '#10071c', 1)])
body.append(f'<rect x="-128" y="{Y0}" width="768" height="{640-Y0+128}" fill="url(#lake)"/>')
# the far shore and the mountains, mirrored faintly
grad_v('reflfade', Y0, Y0+90, [(0, '#2a1238', 0.75), (1, '#2a1238', 0)])
mir = lambda pts, sq=0.8: [(x, Y0 + (Y0-y)*sq) for x, y in pts]
path(P(mir(mid, 0.7) + [(640, Y0), (-128, Y0)]), 'url(#reflfade)', 0.6)
grad_v('reflfade2', Y0, Y0+34, [(0, '#1c0a2a', 0.85), (1, '#1c0a2a', 0)])
tl = [(x, near_ground(x) - 14 - 8*(0.5+0.5*Noise(9)(x/40))) for x in range(-128, CLIFF_L+20, 6)]
path(smooth(mir(tl, 0.9) + [(CLIFF_L+20, Y0), (-128, Y0)]), 'url(#reflfade2)')
# the bright line where the far shore meets the water
grad_v('hline', Y0-1, Y0+2, [(0, '#f8b088', 0), (0.5, '#f8b088', 0.5), (1, '#f8b088', 0)])
body.append(f'<rect x="-128" y="{Y0-1}" width="{CLIFF_L+150}" height="3" fill="url(#hline)"/>')

# the moon's light on the water: a soft column, then glints (round 8's 5, finer)
grad_r('mglow', [(0, '#ffd890', 0.32), (1, '#ffd890', 0)])
soft(MX, Y0+56, 70, 'mglow', sx=0.42, sy=1.1)
GR = random.Random(5); gl_gold = []; gl_lilac = []; sparks = []
def sparkle(x, y, rr):
    q = 0.16
    return (f"M{f(x)},{f(y-rr)} Q{f(x+rr*q)},{f(y-rr*q)} {f(x+rr)},{f(y)} Q{f(x+rr*q)},{f(y+rr*q)} {f(x)},{f(y+rr)} "
            f"Q{f(x-rr*q)},{f(y+rr*q)} {f(x-rr)},{f(y)} Q{f(x-rr*q)},{f(y-rr*q)} {f(x)},{f(y-rr)} Z")
y = Y0 + 3; rows = []
while y < 512:
    t = (y - Y0)/150
    for _ in range(1 if GR.random() < 0.55 else 2):
        half = GR.uniform(5, 26)*(1 - t*0.4)*(1 + t*0.6); cx = MX + GR.gauss(0, 13)*(1 + t*0.8)
        o = 0.92*max(0, 1-t)**1.5
        if o < 0.05: continue
        rows.append((cx, y, half, GR.uniform(0.7, 1.4)*(1 + t*0.7), o, GR.random() < 0.22))
        if o > 0.6 and GR.random() < 0.18: sparks.append((cx + GR.uniform(-half*0.4, half*0.4), y, GR.uniform(2.2, 3.2), o))
    y += GR.uniform(3, 5.5) + t*13
for cx, y, half, th, o, lil in rows:
    body.append(f'<path d="{lens(cx-half, cx+half, y, th)}" fill="{"#d8c0e8" if lil else "#ffe2a8"}" opacity="{o:.2f}"/>')
for x, y, rr, o in sparks:
    body.append(f'<path d="{sparkle(x, y, rr)}" fill="#fff8e8" opacity="{o*0.85:.2f}"/>')

# the calm lines: long, thin, tapered, slightly wavering
def stroke_line(x1, x2, y, thick, amp, seed):
    r = random.Random(seed); n = 22; ph = r.uniform(0, 6.28); wl = r.uniform(30, 70); top, bot = [], []
    for i in range(n+1):
        t = i/n; x = x1+(x2-x1)*t; yy = y + amp*math.sin(x/wl*6.28+ph); w = thick*math.sin(math.pi*t)**0.9/2
        top.append((x, yy-w)); bot.append((x, yy+w))
    return P(top + bot[::-1])
LR = random.Random(9); dim = []; lit = []
y = Y0 + 8
while y < 530:
    t = (y - Y0)/170; L = LR.uniform(30, 80)*(1 + t*1.6); x = LR.uniform(-60, 560)
    if FX - 80 < x < FX + 80 and y < Y0 + 40: x -= 170
    (lit if abs(x - MX) < 60 else dim).append(stroke_line(x-L/2, x+L/2, y, 0.8 + t*1.4, 0.2 + t*0.4, int(y*7)))
    y += 7 + t*20
path(' '.join(dim), '#9a6a98', 0.26); path(' '.join(lit), '#f8d098', 0.36)

# ------------------------------------------------------------------ the cliff (face-on), its rock, the trees on top
CR = random.Random(17)
cn = Noise(41, 5)
# the top edge, with a notch where the drop goes over
def cliff_top(x):
    y = (LIPY - 16) - 10*cn(x/60) - 4*cn(x/14 + 3) + 10*max(0, 1 - abs(x - FX)/160)*0
    notch = math.exp(-((x-FX)/30)**6)
    y = y*(1-notch) + LIPY*notch
    return y - 6*max(0, (x - 520)/60)
top_pts = [(x, cliff_top(x)) for x in range(CLIFF_L + 24, 660, 3)]
# the left edge: steps down to the water, rough
left = []; y = top_pts[0][1]; x = CLIFF_L + 24
ln = Noise(77, 4)
for i in range(1, 41):
    u = i/40; yy = top_pts[0][1] + (Y0 + 2 - top_pts[0][1])*u
    xx = CLIFF_L + 24 - 30*u**1.3 + 7*ln(u*5) + (5 if (i % 9) < 2 else 0)
    left.append((xx, yy))
cliff_poly = left[::-1] + top_pts + [(660, Y0 + 2)]
back_ground = lambda x: (LIPY - 3) - 12*(1 - math.exp(-((x-FX)/40)**2)) - 10*cn(x/60)*(1 - math.exp(-((x-FX)/40)**2)) - 4*cn(x/14 + 3)*(1 - math.exp(-((x-FX)/40)**2))
path(treeline(CLIFF_L + 34, 660, back_ground, 10, 24, 0.9, 77, detail=0.6, crown_p=0, floor=LIPY + 24), '#4a2652')
grad_v('backhaze', LIPY - 30, LIPY + 4, [(0, '#a05a72', 0), (1, '#b06a76', 0.55)])
body.append(f'<rect x="{CLIFF_L + 30}" y="{LIPY - 30}" width="{630 - CLIFF_L}" height="34" fill="url(#backhaze)"/>')
grad_v('rock', LIPY - 30, Y0, [(0, '#2e1a46', 1), (0.55, '#1e1030', 1), (1, '#120820', 1)])
path(P(cliff_poly), 'url(#rock)')
defs.append(f'<clipPath id="cliffclip"><path d="{P(cliff_poly)}"/></clipPath>')
# rock: strata dipping a little, broken by joints that don't line up; lit wedges toward the moon
facets_lit = []; facets_dark = []; cracks = []
def bed(y0, seed):
    nz = Noise(seed, 4); dip = CR.uniform(0.02, 0.12)
    return lambda x: y0 + (x - 300)*dip + 7*nz(x/45)
beds = []; yb = LIPY + CR.uniform(8, 18)
while yb < Y0 + 30:
    beds.append(bed(yb, int(yb*3))); yb += CR.uniform(22, 48)
for bi in range(len(beds) - 1):
    top_, bot_ = beds[bi], beds[bi+1]
    # the bedding crack: partial, tapered
    x = CLIFF_L - 20
    while x < 660:
        L = CR.uniform(30, 140)
        if CR.random() < 0.75:
            pts = [(x + L*u, top_(x + L*u)) for u in [i/16 for i in range(17)]]
            th = CR.uniform(1.0, 2.6)
            cracks.append(P([(px, py - th*math.sin(math.pi*i/16)**0.8/2) for i, (px, py) in enumerate(pts)] +
                            [(px, py + th*math.sin(math.pi*i/16)**0.8/2) for i, (px, py) in enumerate(pts)][::-1]))
        x += L + CR.uniform(4, 30)
    # joints in this stratum
    x = CLIFF_L - 10 + CR.uniform(0, 20)
    while x < 660:
        x2 = x + CR.uniform(-6, 6); y1 = top_(x); y2 = bot_(x2)
        nzc = Noise(int(x*11 + bi), 3); l1 = []; l2 = []
        for i in range(13):
            u = i/12; yy = y1 + (y2-y1)*u; xx = x + (x2-x)*u + 2*nzc(u*3); w = CR.uniform(0.5, 1.2)*math.sin(math.pi*u)**0.5 + 0.2
            l1.append((xx - w, yy)); l2.append((xx + w*0.5, yy))
        if CR.random() < 0.85: cracks.append(P(l1 + l2[::-1]))
        # the block to the right of the joint: a lit wedge on its left, a shaded wedge under its top
        nx = x + CR.uniform(14, 44)
        if CR.random() < 0.6:
            sw = (nx - x)*CR.uniform(0.15, 0.45)
            facets_lit.append(P([(x + 1, y1 + 1), (x + 1 + sw, y1 + 1 + CR.uniform(0, 3)), (x2 + 1 + sw*CR.uniform(0.2, 0.8), y2 - 2), (x2 + 1, y2 - 1)]))
        if CR.random() < 0.35:
            facets_dark.append(P([(x + 2, y1 + 1), (nx - 1, top_(nx) + 1), (nx - 1, top_(nx) + CR.uniform(4, 10)), (x + 2, y1 + CR.uniform(3, 8))]))
        x = nx
body.append('<g clip-path="url(#cliffclip)">')
path(' '.join(facets_lit), '#4a2c62', 0.42)
path(' '.join(facets_dark), '#0c0616', 0.4)
path(' '.join(cracks), '#0a0412', 0.7)
# the face behind the drop: wet, darker, with a sheen
grad_v('wet', LIPY, Y0, [(0, '#0e0618', 0.85), (1, '#0e0618', 0.5)])
wet = [(FX - hw(u)*1.15 - 3 - 4*math.sin(u*9)*u, LIPY + 4 + (Y0-LIPY)*u) for u in [i/20 for i in range(21)]] + \
      [(FX + hw(u)*1.15 + 4 + 5*math.sin(u*7)*u, LIPY + 4 + (Y0-LIPY)*u) for u in [i/20 for i in range(20, -1, -1)]]
path(smooth(wet), 'url(#wet)')
# the moonlit left edge of the cliff
edge = []; edge2 = []
for i, (x, y) in enumerate(left[::-1] + [top_pts[0]]):
    pass
body.append('</g>')
rim = [(x + 0.5, y) for x, y in left] ; rim2 = [(x + 3.2 + 1.5*math.sin(i), y) for i, (x, y) in enumerate(left)]
path(P(rim + rim2[::-1]), '#6a4078', 0.5)
# the lit rim of the top edge
tr1 = [(x, y) for x, y in top_pts if x < 600]; tr2 = [(x, y + 2.2) for x, y in tr1]
path(P(tr1 + tr2[::-1]), '#7a4a80', 0.5)

# the trees on top of the cliff, larger, two depths
TR = random.Random(31); back = []; front = []
x = CLIFF_L + 10
while x < 660:
    if abs(x - FX) < 40: x += 6; continue
    h = TR.uniform(26, 58)*(1.0 if x > FX else 0.85)
    back.append(spruce(x, cliff_top(x) + 4, h*TR.uniform(0.8, 1.1), int(x*13), slim=TR.uniform(0.3, 0.4)))
    x += TR.uniform(7, 14)
x = CLIFF_L + 18
while x < 660:
    if abs(x - FX) < 48: x += 8; continue
    if TR.random() < 0.55:
        h = TR.uniform(34, 72)
        front.append(spruce(x, cliff_top(x) + 8, h, int(x*29) + 5, slim=TR.uniform(0.32, 0.42)))
    x += TR.uniform(12, 24)
path(' '.join(back), '#2c1640')
path(' '.join(front), '#140a20')

# ------------------------------------------------------------------ the drop
grad_v('stream', LIPY, Y0, [(0, '#b8706a', 1), (0.06, '#e8a474', 1), (0.18, '#f8cc98', 1), (0.5, '#fde4c4', 1), (1, '#fff2e0', 1)])
# the body: three nested shapes so its edges fade
grad_v('streammist', LIPY, Y0, [(0, '#e8b8a0', 0), (0.3, '#f8dcc8', 0.5), (1, '#fff0e4', 0.8)])
for k_, o, fill_ in ((1.22, 0.35, 'url(#streammist)'), (1.1, 0.5, 'url(#streammist)'), (0.96, 0.9, 'url(#stream)')):
    L_ = []; R_ = []
    for i in range(41):
        s = i/40; e = 2.5*wn(s*7 + 11)*(0.4 + s)
        L_.append((cx_(s) - hw(s)*k_ + e, fy(s))); R_.append((cx_(s) + hw(s)*k_ + e*0.8, fy(s)))
    
    path(smooth(L_ + R_[::-1]), fill_, o)
# the strands: many thin tapered threads, smooth and glassy at the top, breaking up lower down
def strand(u, s0, s1, w, seed, spread=1.0):
    nz = Noise(seed, 3); n = max(6, int((s1 - s0)*40)); a = []; b = []
    for i in range(n+1):
        s = s0 + (s1 - s0)*i/n; t = i/n
        x = cx_(s) + (u - 0.5)*2*hw(s)*spread + 3.0*nz(s*5)*s**1.2
        ww = w*(math.sin(math.pi*t)**0.6)*(0.5 + s)
        a.append((x - ww/2, fy(s))); b.append((x + ww/2, fy(s)))
    return P(a + b[::-1])
buckets = {'hi': [], 'cream': [], 'peach': [], 'shade': []}
for i in range(260):
    u = WR.random()
    s0 = 0.03 + WR.uniform(0.0, 0.85)**1.2; s1 = min(1.04, s0 + WR.uniform(0.1, 0.55)*(0.5 + s0))
    w = WR.uniform(0.35, 1.8)*(0.7 + s0)
    kind = WR.choices(['hi', 'cream', 'peach', 'shade'], [2, 5, 3, 3])[0]
    if s0 < 0.12 and kind == 'hi': s1 = min(s1, 0.4)
    buckets[kind].append(strand(u, s0, s1, w, i*7 + 3, spread=1.0 + (0.12 if u < 0.08 or u > 0.92 else 0)))
path(' '.join(buckets['shade']), '#c07868', 0.4)
path(' '.join(buckets['peach']), '#f0a878', 0.55)
path(' '.join(buckets['cream']), '#fff4e4', 0.7)
path(' '.join(buckets['hi']), '#ffffff', 0.85)
def plume(u, s_head, L, w, seed):
    r = random.Random(seed); nz = Noise(seed, 3); n = 16; a_ = []; b_ = []
    for i in range(n+1):
        t = i/n                                   # 0 = tail (top), 1 = head (bottom)
        s = s_head - L*(1 - t); x = cx_(s) + (u - 0.5)*2*hw(s) + 1.2*nz(t*2)
        ww = w*(t**1.6)*(1 - 0.6*max(0, t - 0.85)/0.15)
        a_.append((x - ww/2, fy(s))); b_.append((x + ww/2, fy(s)))
    hx, hy = a_[-1][0] + w*0.2, fy(s_head) + w*0.35
    return smooth(a_ + [(hx, hy)] + b_[::-1])
pl_w = []; pl_c = []
for i in range(70):
    u = 0.06 + 0.88*WR.random(); sh = WR.uniform(0.32, 1.0); L = WR.uniform(0.06, 0.16)*(0.6 + sh)
    w = WR.uniform(2.0, 5.5)*(0.5 + sh)
    (pl_w if WR.random() < 0.55 else pl_c).append(plume(u, sh, L, w, 500 + i))
path(' '.join(pl_c), '#ffe8d0', 0.5)
path(' '.join(pl_w), '#ffffff', 0.62)
# loose threads off the edges, lower down
loose = []
for i in range(16):
    side = WR.choice([-1, 1]); s0 = WR.uniform(0.35, 0.8); s1 = min(1.0, s0 + WR.uniform(0.1, 0.3))
    u = 0.5 + side*WR.uniform(0.52, 0.62)
    loose.append(strand(u, s0, s1, WR.uniform(0.4, 1.0), 900 + i))
path(' '.join(loose), '#fff0dc', 0.45)
# the river above the brink, glimpsed between the trees, catching the sky
lip_l = FX - hw(0) - 1; lip_r = FX + hw(0) + 1
grad_v('river', LIPY - 4, LIPY, [(0, '#5a3466', 1), (1, '#c89090', 1)])
path(P([(lip_l - 1, LIPY + 0.5), (lip_l + 3, LIPY - 3.5), (lip_r - 3, LIPY - 3.5), (lip_r + 1, LIPY + 0.5)]), 'url(#river)')
# the brink: one bright line where the water rolls over, a little lower at the sides
crest = [(lip_l + (lip_r - lip_l)*u, LIPY - 0.5 + 1.8*(2*u - 1)**4) for u in [i/16 for i in range(17)]]
crest2 = [(x, y + 1.7*math.sin(math.pi*i/16)**0.5) for i, (x, y) in enumerate(crest)]
path(P(crest + crest2[::-1]), '#fff8ec', 0.95)
# ------------------------------------------------------------------ the foot: mist, foam, ripples, the drop in the water
grad_r('mist', [(0, '#fff4e4', 0.62), (0.5, '#ffe8d4', 0.25), (1, '#ffe8d4', 0)])
grad_r('mist2', [(0, '#f8d8c8', 0.35), (1, '#f8d8c8', 0)])
MR_ = random.Random(61)
for i in range(18):
    x = FX + MR_.gauss(0, 30); yy = Y0 - MR_.uniform(-2, 34); rr = MR_.uniform(14, 34)
    soft(x, yy, rr, 'mist2' if i % 3 else 'mist', sx=1.4, sy=0.8, o=MR_.uniform(0.5, 0.9))
soft(FX, Y0 - 6, 70, 'mist', sx=1.3, sy=0.42)
# foam: lumpy pale masses where the water lands
foam = []
for i in range(9):
    x = FX + MR_.uniform(-1, 1)*hw(1)*1.1; rr = MR_.uniform(4, 9); cy = Y0 - rr*0.4
    pts = []
    for j in range(12):
        a = 2*math.pi*j/12; kk = 1 + MR_.uniform(-0.2, 0.2)
        pts.append((x + rr*1.5*kk*math.cos(a), cy + rr*0.6*kk*math.sin(a)))
    foam.append(smooth(pts))
path(' '.join(foam), '#fff6ec', 0.55)
# rocks at the foot, half in the mist
rocks = []
for (x, w, h) in [(FX - 62, 24, 12), (FX + 58, 30, 14), (FX + 86, 18, 8), (FX - 84, 14, 6)]:
    pts = []
    for j in range(10):
        a = math.pi + math.pi*j/9; kk = 1 + MR_.uniform(-0.15, 0.1)
        pts.append((x + w/2*kk*math.cos(a), Y0 + 1 + h*kk*math.sin(a)))
    rocks.append(smooth(pts + [(x + w/2, Y0 + 2), (x - w/2, Y0 + 2)]))
path(' '.join(rocks), '#1a0e28')
soft(FX, Y0 + 2, 40, 'mist', sx=2.2, sy=0.25, o=0.7)

# ripples where the drop lands: broken arcs, thick in front, thin at the sides
def arc(cx, cy, rx, ry, a1, a2, wmax, seed):
    r = random.Random(seed); n = 30; wob = r.uniform(0, 6.28); o_, i_ = [], []
    for i in range(n+1):
        t = i/n; a = a1+(a2-a1)*t; kk = 1 + 0.025*math.sin(3*a+wob)
        w = wmax*max(0.15, math.sin(a))*math.sin(math.pi*t)**0.7
        x, yy = cx+rx*kk*math.cos(a), cy+ry*kk*math.sin(a)
        o_.append((x, yy+w/2)); i_.append((x, yy-w/2))
    return P(o_ + i_[::-1])
RR = random.Random(3); br = []; dm = []
for rx, wmax, o in [(52, 2.4, 0.8), (76, 2.2, 0.6), (104, 1.9, 0.42), (136, 1.5, 0.28)]:
    ry = rx*0.13; a = 0.1; segs = []
    while a < math.pi - 0.1:
        L = RR.uniform(0.45, 1.2); b = min(math.pi - 0.05, a + L); segs.append((a, b)); a = b + RR.uniform(0.1, 0.26)
    for a1, a2 in segs:
        mid_ = (a1+a2)/2; d = arc(FX, Y0 + 3, rx, ry, a1, a2, wmax, int(rx*100 + a1*10))
        (br if math.cos(mid_) > -0.1 else dm).append((d, o))
for d, o in br: body.append(f'<path d="{d}" fill="#fff0d4" opacity="{o:.2f}"/>')
for d, o in dm: body.append(f'<path d="{d}" fill="#e8b48c" opacity="{o*0.8:.2f}"/>')
# the drop' light in the water, broken by the ripples
y = Y0 + 8; fr = random.Random(12)
while y < Y0 + 110:
    t = (y - Y0 - 8)/100; half = fr.uniform(6, 20)*(1 - t*0.4); cx = FX + fr.uniform(-10, 10)
    body.append(f'<path d="{lens(cx-half, cx+half, y, fr.uniform(0.8, 1.5))}" fill="#ffecd0" opacity="{0.5*max(0, 1-t)**1.3:.2f}"/>')
    y += fr.uniform(4, 7) + t*10
# the cliff's dark reflection to the right of the drop' light
grad_v('cliffrefl', Y0, Y0 + 140, [(0, '#0c0414', 0.6), (1, '#0c0414', 0)])
crf = [(x, 2*Y0 - y) for x, y in cliff_poly]
path(P(crf), 'url(#cliffrefl)')

# ------------------------------------------------------------------ the near shore: a few reeds in the corner, in silhouette
if VAR == 'reeds':
    RE = random.Random(4); reeds = []
    for i in range(14):
        x0 = RE.uniform(-40, 40); base = 530; h = RE.uniform(40, 110); lean = RE.uniform(-0.25, 0.45)
        w = RE.uniform(1.6, 3.2); a = []; b = []
        for j in range(13):
            u = j/12; x = x0 + lean*h*u**1.8; yy = base - h*u; ww = w*(1-u)**0.8 + 0.2
            a.append((x - ww/2, yy)); b.append((x + ww/2, yy))
        reeds.append(P(a + b[::-1]))
    body.append(f'<path d="{" ".join(reeds)}" fill="#0c0614"/>')

svg = f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
<defs>{"".join(defs)}</defs>
{chr(10).join(body)}
</svg>
'''
open(OUT, 'w').write(svg)
print(OUT, len(svg)//1024, 'KB')
