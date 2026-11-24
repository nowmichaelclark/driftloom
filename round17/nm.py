# Round 17: round 16's New Mexico, refined, in five variations.
# Ghost Ranch's order of rock, top to bottom: a pale cap, a cream cliff, orange and red ledges, purple-red slopes at the foot;
# each cliff shaded as a form, streaked with desert varnish, a talus skirt at its foot, junipers dotted over it.
# python3 nm.py <variation> out.svg
import math, os, sys, random
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'round16'))
from geo import *
mx, my, mr = MOON

def mix(a, b, t):
    a = [int(a[i:i+2], 16) for i in (1, 3, 5)]; b = [int(b[i:i+2], 16) for i in (1, 3, 5)]
    return '#%02x%02x%02x' % tuple(int(a[i] + (b[i] - a[i])*t) for i in range(3))
def moon_d(cx=mx, cy=my, r=mr):
    t = Icon(); crescent(t, cx, cy, r, ox=-r*0.55, oy=-r*0.36, r2=r*0.86); return t.body[-1].split('d="')[1].split('"')[0]
def clip_open(ic, d):
    cid = ic.gid('c'); ic.defs.append(f'<clipPath id="{cid}"><path d="{d}"/></clipPath>'); ic.raw(f'<g clip-path="url(#{cid})">')

V = {
 'dusk': dict(sky=[(0, '#16305a'), (0.42, '#2f7c9c'), (0.72, '#9ab8b0'), (0.88, '#f0b894'), (1, '#f8d8b4')],
              bands=['#f6e4c4', '#f0d0a0', '#e09a6a', '#c8643e', '#a84836', '#8a4a5a'], haze='#d8b8b8', hz=0.56,
              light='#fff4e0', shade='#3a1420', lightdir=1, mesa='#7a78aa', pool=['#6ac8c4', '#2a8a9a', '#0f3a4a'],
              sky_obj='moon', jun='#3c5a46', junlit='#7a9a72', stars=0),
 'glow':  dict(sky=[(0, '#1e1a4a'), (0.4, '#5a4a8a'), (0.68, '#c87a8a'), (0.86, '#f6a86a'), (1, '#ffd090')],
              bands=['#ffe8b0', '#ffc878', '#f08848', '#d8502e', '#b03a2a', '#7a3446'], haze='#e8a890', hz=0.5,
              light='#ffd080', shade='#3a0c18', lightdir=-1, mesa='#5a4a80', pool=['#f0a878', '#7a5a8a', '#1e1a40'],
              sky_obj='star', jun='#2e4630', junlit='#8a9a48', stars=0),
 'night': dict(sky=[(0, '#0c0a24'), (0.45, '#24184a'), (0.8, '#4a2a6a'), (1, '#7a4a80')],
              bands=['#d8c4d8', '#c8a8c0', '#a87890', '#8a5468', '#6a3a52', '#4a2844'], haze='#5a3a70', hz=0.55,
              light='#e8dcff', shade='#0c0418', lightdir=1, mesa='#3a2a5a', pool=['#5a3a7a', '#2a1a4a', '#0e0822'],
              sky_obj='moon', jun='#1a2a2a', junlit='#4a6a62', stars=1),
 'okeeffe': dict(sky=[(0, '#7ab0d0'), (0.6, '#b8d4dc'), (1, '#f8e4d0')],
              bands=['#fbeedc', '#f6d6bc', '#eeb49c', '#e0907e', '#cc7a7c', '#b87888'], haze='#f0d4c8', hz=0.6,
              light='#ffffff', shade='#7a3a48', lightdir=1, mesa='#9aa0c8', pool=['#a8dcd8', '#6ab0b8', '#3a7a8a'],
              sky_obj='moon_white', jun='#6a8a76', junlit='#a8c0a0', stars=0, soft=1),
 'monsoon': dict(sky=[(0, '#2a2a4a'), (0.45, '#5a5a7a'), (0.75, '#a8a0a8'), (1, '#f0d0a0')],
              bands=['#fbe6bc', '#f6cc8a', '#ec9858', '#d26838', '#b04a32', '#8a4250'], haze='#b8a8b0', hz=0.52,
              light='#fff0c8', shade='#2a1220', lightdir=-1, mesa='#5a5a80', pool=['#7ab0b0', '#3a6a80', '#1a2a44'],
              sky_obj='storm', jun='#34503c', junlit='#88a070', stars=0),
}

def build(name):
    v = V[name]; ic = Icon(); g = geo(); rnd = random.Random(17); soft = v.get('soft', 0)
    # ---- sky
    ic.path(P(sky_rect()), ic.lin(0, -128, 0, 210, [(o, c, 1) for o, c in v['sky']]))
    if v['stars']:
        sr = random.Random(5)
        for i in range(90):
            x = sr.uniform(-128, 640); y = sr.uniform(-128, 180); ic.circle(x, y, sr.choice([0.7, 0.9, 1.2, 1.6]), '#fff4ec', sr.uniform(0.3, 0.9))
        ic.circle(120, 0, 260, ic.rad([(0, '#c8a8e0', 0.18), (1, '#c8a8e0', 0)]), sx=1.6, sy=0.35)
    so = v['sky_obj']
    if so in ('moon', 'moon_white'):
        ic.circle(mx, my, mr*2.8, ic.rad([(0, '#fff8ec', 0.35), (1, '#fff8ec', 0)]))
        ic.raw(f'<path d="{moon_d()}" fill="{"#ffffff" if so == "moon_white" else "#fff6e2"}"/>')
    if so == 'star':   # the evening star over the afterglow
        ic.circle(mx, my - 20, 40, ic.rad([(0, '#fff8e0', 0.5), (1, '#fff8e0', 0)]))
        ic.path(P([(mx - 9, my - 20), (mx, my - 23), (mx + 9, my - 20), (mx, my - 17)]), '#ffffff'); ic.path(P([(mx, my - 29), (mx + 3, my - 20), (mx, my - 11), (mx - 3, my - 20)]), '#ffffff')
    if so == 'storm':  # a monsoon cell: a towering cloud with a flat, dark base, rain falling from it, a rainbow in the light
        lobes = [(200, 96, 34), (240, 70, 46), (290, 40, 58), (345, 14, 66), (405, 30, 60), (460, 56, 50), (510, 84, 40), (560, 100, 34), (610, 104, 40), (150, 110, 26), (330, 74, 56), (420, 84, 52), (260, 96, 40), (500, 104, 36)]
        cl = ' '.join(f"M{f(x - r)},{f(y)} a{f(r)},{f(r)} 0 1 0 {f(2*r)},0 a{f(r)},{f(r)} 0 1 0 {f(-2*r)},0 Z" for x, y, r in lobes)
        cid = ic.gid('c'); ic.defs.append(f'<clipPath id="{cid}"><path d="M-128,-128 L640,-128 L640,116 L-128,116 Z"/></clipPath>'); ic.raw(f'<g clip-path="url(#{cid})">')
        ic.path(cl, ic.lin(0, -30, 0, 116, [(0, '#fffaf2', 1), (0.55, '#e8dce0', 1), (1, '#8a86a0', 1)]))
        for x, y, r in lobes[:9]: ic.circle(x - r*0.25, y - r*0.35, r*0.6, ic.rad([(0, '#ffffff', 0.7), (1, '#ffffff', 0)]))
        ic.raw('</g>')
        ic.path(P([(120, 96), (660, 96), (660, 120), (120, 120)]), ic.lin(0, 96, 0, 120, [(0, '#7a7690', 0), (1, '#7a7690', 0.5)]))
        for x0, w in ((290, 80), (400, 110), (520, 70)):
            ic.path(P([(x0, 112), (x0 + w, 112), (x0 + w - 36, 260), (x0 - 50, 260)]), ic.lin(0, 112, 0, 260, [(0, '#7a7894', 0.4), (1, '#7a7894', 0)]))
        for k, c in enumerate(['#e05050', '#f0a040', '#f0e060', '#60c060', '#5080e0', '#8050c0']):
            r1 = 190 - k*5
            ic.path(f"M{f(150 - r1)},250 A{r1},{r1} 0 0 1 {f(150 + r1)},250 L{f(150 + r1 - 5)},250 A{r1 - 5},{r1 - 5} 0 0 0 {f(150 - r1 + 5)},250 Z", c, 0.28)
    # ---- the mesa, flat-topped, hazy
    mesa = [(-130, 200), (-40, 196), (20, 172), (60, 152), (90, 145), (170, 144), (196, 152), (250, 174), (330, 188), (420, 182), (520, 194), (660, 198)]
    ic.path(smooth(mesa, close=False) + f" L640,{H} L-128,{H} Z", ic.lin(0, 140, 0, 230, [(0, v['mesa'], 1), (1, mix(v['mesa'], v['haze'], 0.5), 1)]))
    # ---- a cliff in its bands
    def cliff(poly, x0, x1, top, depth, side):
        clip_open(ic, smooth(poly))
        n = Noise(rnd.randint(1, 999), 3); B = v['bands']
        fr = [0, 0.05, 0.36, 0.5, 0.62, 0.78, 1.2]       # the cap, the cream cliff, the ledges, the slope
        for i in range(len(B)):
            ya = top + (H - top)*fr[i] - 4
            edge = [(x, ya + 3*n(x/70 + i) + (0 if soft else 1.2*n(x/9 + i))) for x in range(int(x0) - 12, int(x1) + 14, 12)]
            c0 = mix(B[i], v['haze'], depth); c1 = mix(B[min(i + 1, len(B) - 1)], v['haze'], depth)
            ic.path(smooth(edge, close=False) + f" L{x1 + 14},{H + 40} L{x0 - 12},{H + 40} Z", ic.lin(0, ya, 0, top + (H - top)*fr[i + 1], [(0, c0, 1), (0.75 if not soft else 0.3, c0, 1), (1, c1, 1)]))
        # the form: lit toward the light, shaded away from it
        lo, hi = (x0, x1) if v['lightdir'] > 0 else (x1, x0)
        ic.path(P([(x0 - 20, top - 20), (x1 + 20, top - 20), (x1 + 20, H + 20), (x0 - 20, H + 20)]),
                ic.lin(lo, 0, hi, 0, [(0, v['shade'], 0.38*(1 - depth)), (0.55, v['shade'], 0), (0.85, v['light'], 0.1), (1, v['light'], 0.28*(1 - depth*0.6))]))
        # desert varnish: dark streaks down the cream cliff
        if not soft:
            for k in range(int((x1 - x0)/16)):
                x = rnd.uniform(x0, x1); y0 = top + rnd.uniform(10, 50); y1 = y0 + rnd.uniform(24, 90); w = rnd.uniform(1.2, 3.5)
                ic.path(smooth([(x - w/2, y0), (x + w/2, y0), (x + w/3 + 2*math.sin(y1), y1 - 8), (x, y1), (x - w/3, y1 - 8)]), mix(v['shade'], v['haze'], depth), rnd.uniform(0.05, 0.12)*(1 - depth*0.5))
        if depth == 0: ic.path(P([(x0 - 20, top - 20), (x1 + 20, top - 20), (x1 + 20, H + 20), (x0 - 20, H + 20)]), v['shade'], 0.14)
        # shade at the foot
        ic.path(P([(x0 - 20, H - 70), (x1 + 20, H - 70), (x1 + 20, H + 20), (x0 - 20, H + 20)]), ic.lin(0, H - 70, 0, H, [(0, v['shade'], 0), (1, v['shade'], 0.35*(1 - depth))]))
        ic.raw('</g>')
        return n
    # the far cliff, hazier
    cliff(g['far'], 60, 460, LIP - 4, v['hz'], 0)
    ic.path(P(g['ftop'] + [(x, y + 2.5) for x, y in g['ftop'][::-1]]), v['light'], 0.4)
    ic.path(P([(-128, H - 110), (640, H - 110), (640, H + 2), (-128, H + 2)]), ic.lin(0, H - 110, 0, H, [(0, v['haze'], 0), (1, v['haze'], 0.5)]))
    # ---- the stream: cream-white, a gloss toward the light
    ic.path(smooth(g['water']), ic.lin(0, LIP, 0, H, [(0, mix('#ffffff', v['pool'][0], 0.25), 1), (0.5, mix('#ffffff', v['pool'][0], 0.4), 1), (1, '#ffffff', 1)]))
    clip_open(ic, smooth(g['water']))
    gx = (lambda y, u: edges(y)[0] + (edges(y)[1] - edges(y)[0])*u)
    gu = (0.62, 0.86) if v['lightdir'] > 0 else (0.14, 0.38)
    gl = [(gx(y, gu[0]), y) for y in range(LIP + 4, H - 60, 12)] + [(gx(y, gu[1]), y) for y in range(H - 60, LIP + 3, -12)]
    ic.path(smooth(gl), ic.lin(0, LIP, 0, H - 60, [(0, '#ffffff', 0.85), (1, '#ffffff', 0)]))
    for i in range(12):
        u = rnd.uniform(0.05, 0.95); pts = [(gx(y, u), y) for y in range(LIP + 4, H, 14)]
        ic.path(P(pts + [(x + 1.6, y) for x, y in pts[::-1]]), v['pool'][1], 0.18)
    ic.path(P([(150, H - 90), (360, H - 90), (360, H + 2), (150, H + 2)]), ic.lin(0, H - 90, 0, H, [(0, '#ffffff', 0), (1, '#ffffff', 0.9)]))
    ic.raw('</g>')
    ic.path(smooth([(LX0 - 2, LIP + 1), (256, LIP - 5), (LX1 + 2, LIP + 1), (LX1, LIP + 10), (256, LIP + 12), (LX0, LIP + 10)]), '#ffffff', 0.9)
    # ---- the near cliffs, then their talus skirts
    cliff(g['left'], -140, 190, min(y for x, y in g['lt']), 0, -1)
    cliff(g['right'], 330, 660, min(y for x, y in g['rt']), 0, 1)
    for top in (g['lt'], g['rt']):
        ic.path(P(top + [(x, y + 3) for x, y in top[::-1]]), v['light'], 0.5)
    tal = []
    for pts, d in ((g['ledge'], 1), (g['redge'], -1)):
        a = [p for p in pts if p[1] > H - 80]
        sk = [(x + d*(4 + 46*((y - (H - 80))/80)**1.6), y) for x, y in a]
        poly = a + sk[::-1]
        ic.path(smooth(poly), ic.lin(0, H - 80, 0, H, [(0, mix(v['bands'][4], v['shade'], 0.15), 1), (1, mix(v['bands'][5], v['shade'], 0.35), 1)]))
        tal.append(poly)
    # junipers: dotted over the tops, the plateau and the talus, each with a lit side and a shadow
    jun = []
    for x, base, h in g['trees']:
        for k in range(rnd.randint(1, 3)): jun.append((x + rnd.uniform(-10, 10), base - 4 - rnd.uniform(0, 3), h*rnd.uniform(0.12, 0.2)))
    for x, y, hh in [g['far_trees'][i] for i in (1, 2, 9, 12)]:
        for k in range(2): jun.append((x + k*6 - 3, y + 3 + k, 3.5 + k))
    for poly in tal:
        for k in range(2):
            x, y = poly[rnd.randrange(len(poly)//2)]; jun.append((x + rnd.uniform(-4, 18)*(1 if x < 256 else -1), y + rnd.uniform(-6, 2), rnd.uniform(4, 6)))
    if soft: jun = jun[::2]
    sd = 1 if v['lightdir'] > 0 else -1
    for x, y, r in jun: ic.circle(x - sd*r*0.5, y + r*0.7, r*0.9, v['shade'], 0.35, sx=1.4, sy=0.4)
    for x, y, r in jun: ic.circle(x, y, r, v['jun'])
    for x, y, r in jun: ic.circle(x + sd*r*0.3, y - r*0.3, r*0.55, v['junlit'], 0.75)
    # ---- the pool
    pc = v['pool']
    ic.path(P(pool_rect()), ic.lin(0, H, 0, 600, [(0, pc[0], 1), (0.35, pc[1], 1), (1, pc[2], 1)]))
    ic.path(P([(-128, H - 0.5), (640, H - 0.5), (640, H + 2.5), (-128, H + 2.5)]), ic.lin(-128, 0, 640, 0, [(0, '#ffffff', 0), (0.5, '#ffffff', 0.6), (1, '#ffffff', 0)]))
    for i, (y, w) in enumerate([(H + 14, 104), (H + 32, 80), (H + 54, 58), (H + 80, 40), (H + 110, 24)]):
        ic.path(f"M{f(256 - w/2)},{y} Q256,{f(y - 2.4)} {f(256 + w/2)},{y} Q256,{f(y + 2.4)} {f(256 - w/2)},{y} Z", '#ffffff', 0.6 - i*0.1)
    if so in ('moon', 'moon_white', 'star'):
        for i, y in enumerate([H + 20, H + 40, H + 66]):
            w = 22 - i*5; ic.path(f"M{f(mx - w/2)},{y} Q{mx},{y - 2} {f(mx + w/2)},{y} Q{mx},{y + 2} {f(mx - w/2)},{y} Z", '#fff6e0', 0.5 - i*0.12)
    ripples(ic, 256, H + 6, [(72, 2.6, 0.5), (106, 2, 0.3)], col='#ffffff')
    for dx, dy, rr, sx, sy in [(-42, -6, 26, 1.5, 0.7), (42, -6, 26, 1.5, 0.7), (0, -20, 36, 1.2, 0.85), (0, -2, 52, 1.7, 0.6)]:
        ic.circle(256 + dx, H + dy, rr, ic.rad([(0, '#ffffff', 0.9), (0.6, '#ffffff', 0.35), (1, '#ffffff', 0)]), sx=sx, sy=sy)
    if not soft: ic.circle(256, 250, 660, ic.rad([(0.33, '#1a0a14', 0), (0.75, '#1a0a14', 0.35), (1, '#1a0a14', 0.45)]))
    return ic

if __name__ == '__main__':
    open(sys.argv[2], 'w').write(build(sys.argv[1]).svg()); print(sys.argv[2])
