# Round 16, the painterly and raster styles: Monet, Van Gogh, stained glass, pixel art.
# They take their colors from round 15 drawn wide (base768.png: the view -128..640), then remap them.
# python3 paint.py <style> out.svg
import math, os, sys, random, colorsys
import numpy as np
from PIL import Image, ImageDraw
from geo import *
HERE = os.path.dirname(os.path.abspath(__file__))
B = np.asarray(Image.open(os.path.join(HERE, 'base768.png')).convert('RGB')).astype(float)/255
def sample(x, y):
    i = int(min(767, max(0, y + 128))); j = int(min(767, max(0, x + 128))); return B[i, j]
def lum(c): return 0.3*c[0] + 0.59*c[1] + 0.11*c[2]
def hexc(c): return '#%02x%02x%02x' % tuple(int(max(0, min(1, v))*255) for v in c)
def rgb(h): h = h.lstrip('#'); return np.array([int(h[i:i+2], 16)/255 for i in (0, 2, 4)])
def ramp(stops, t):
    t = max(0, min(1, t))
    for (a, ca), (b, cb) in zip(stops, stops[1:]):
        if a <= t <= b:
            u = (t - a)/((b - a) or 1); return rgb(ca) + (rgb(cb) - rgb(ca))*u
    return rgb(stops[-1][1])
g = geo()
# the regions as a map: 0 sky, 1 far cliff, 2 water, 3 near cliffs, 4 pool, 5 moon
R = Image.new('L', (768, 768), 0); d = ImageDraw.Draw(R)
sh = lambda pts: [(x + 128, y + 128) for x, y in pts]
d.polygon(sh(g['far']), fill=1); d.polygon(sh(g['water']), fill=2)
d.polygon(sh(g['left']), fill=3); d.polygon(sh(g['right']), fill=3)
d.rectangle((0, H + 128, 768, 768), fill=4)
mx, my, mr = MOON; d.ellipse((mx - mr + 128, my - mr + 128, mx + mr + 128, my + mr + 128), fill=5)
R = np.asarray(R)
def region(x, y): return int(R[int(min(767, max(0, y + 128))), int(min(767, max(0, x + 128)))])
rnd = random.Random(16)

def ellipse_pts(x, y, L, W, a, n=8):
    ca, sa = math.cos(a), math.sin(a)
    return [(x + L/2*math.cos(t)*ca - W/2*math.sin(t)*sa, y + L/2*math.cos(t)*sa + W/2*math.sin(t)*ca) for t in [2*math.pi*i/n for i in range(n)]]

def underpaint(ic, tf, cell=16):
    by = {}
    for gy in range(-128, 640, cell):
        for gx in range(-128, 640, cell):
            c = tf(sample(gx + cell/2, gy + cell/2), region(gx + cell/2, gy + cell/2), gx, gy)
            by.setdefault(hexc(c), []).append(f"M{gx},{gy}h{cell}v{cell}h-{cell}Z")
    for col, ds in by.items(): ic.path(''.join(ds), col)

def underpaint_geo(ic, tf):
    """The scene's own shapes, each with a gradient sampled from round 15 and remapped: no blocks under the strokes."""
    def grad(x, y0, y1, reg, n=5):
        stops = [(i/(n - 1), hexc(tf(sample(x, y0 + (y1 - y0)*i/(n - 1)), reg, x, 0)), 1) for i in range(n)]
        return ic.lin(0, y0, 0, y1, stops)
    ic.path(P(sky_rect()), grad(40, -120, H, 0))
    ic.path(P(g['far']), grad(150, LIP, H, 1)); ic.path(smooth(g['water']), grad(256, LIP + 4, H - 4, 2))
    ic.path(smooth(g['left']), grad(-60, 60, H, 3)); ic.path(smooth(g['right']), grad(560, 120, H, 3))
    ic.path(P(pool_rect()), grad(60, H + 2, 640, 4))

def group_paths(ic, items, o=1):
    by = {}
    for col, d_ in items: by.setdefault(col, []).append(d_)
    for col, ds in by.items(): ic.path(' '.join(ds), col, o)

# ------------------------------------------------------------------ Monet: soft broken color, short dabs, light everywhere
def monet():
    ic = Icon()
    def tf(c, reg, x, y, jit=0.0):
        h, l, s = colorsys.rgb_to_hls(*c)
        l = 0.32 + 0.62*l; s = 0.3 + 0.35*min(1, s)
        if reg == 3: h = (h + 0.0) % 1; l *= 0.82
        if reg == 4: h = (h - 0.06) % 1
        h = (h + jit) % 1
        return np.array(colorsys.hls_to_rgb(h, min(0.97, l), min(1, s)))
    underpaint_geo(ic, tf)
    items = []
    for i in range(14000):
        x = rnd.uniform(-128, 640); y = rnd.uniform(-128, 640); reg = region(x, y)
        a = {0: rnd.gauss(0, 0.5), 1: math.pi/2 + rnd.gauss(0, 0.35), 2: math.pi/2 + rnd.gauss(0, 0.12), 3: math.pi/2 + rnd.gauss(0, 0.5), 4: rnd.gauss(0, 0.12), 5: rnd.uniform(0, 3)}[reg]
        L = rnd.uniform(8, 13); W = rnd.uniform(3, 4.5)
        c = tf(sample(x, y), reg, x, y, rnd.gauss(0, 0.05))
        if rnd.random() < 0.06:   # a stroke of the complement, as Monet broke his color
            h, l, s = colorsys.rgb_to_hls(*c); c = np.array(colorsys.hls_to_rgb((h + rnd.choice([0.4, 0.5, 0.6])) % 1, l, s*0.5))
        c = c + np.array([rnd.gauss(0, 0.03) for _ in range(3)])
        items.append((hexc(np.round(c*12)/12), P(ellipse_pts(x, y, L, W, a))))
    group_paths(ic, items, 0.8)
    # the moon's light on the pool, and lily pads
    for i in range(70):
        y = rnd.uniform(H + 6, 600); w = 6 + (y - H)*0.12
        x = MOON[0] - 96 + rnd.gauss(0, 6 + (y - H)*0.08)
        ic.path(P(ellipse_pts(x, y, rnd.uniform(8, 16), 3.5, rnd.gauss(0, 0.1))), rnd.choice(['#fff0b8', '#ffe2a0', '#f8d8e8']), 0.85)
    for x, y, r in [(80, 470, 26), (126, 500, 18), (400, 452, 22), (436, 488, 30), (40, 540, 30), (470, 560, 24)]:
        ic.circle(x, y, r, '#5e8a6a', 0.85, sx=1, sy=0.38)
        for k in range(6): ic.path(P(ellipse_pts(x + rnd.uniform(-r*0.6, r*0.6), y + rnd.uniform(-r*0.2, r*0.2), 8, 3, rnd.gauss(0, 0.2))), rnd.choice(['#7aa880', '#9cc490', '#4a7460']), 0.9)
        if r > 22: ic.circle(x + r*0.2, y - 4, 5, '#f4a0c0'); ic.circle(x + r*0.2 - 1.5, y - 5.5, 2.2, '#ffe0ec')
    crescent(ic, *MOON[:2], MOON[2], ox=-MOON[2]*0.55, oy=-MOON[2]*0.36, r2=MOON[2]*0.86)
    return ic

# ------------------------------------------------------------------ Van Gogh: swirling strokes in ultramarine and chrome yellow, cypresses
def vangogh():
    ic = Icon()
    pal = {0: [(0, '#0b1a44'), (0.25, '#1d3f8a'), (0.45, '#3c6cc0'), (0.62, '#8ab8e0'), (0.78, '#f2d65a'), (1, '#fff6c8')],
           1: [(0, '#141c4a'), (0.35, '#2e4a96'), (0.6, '#6a8ac8'), (1, '#c8d8f0')],
           2: [(0, '#5a7ac0'), (0.6, '#c8e0f0'), (1, '#fffbe0')],
           3: [(0, '#0a1418'), (0.25, '#18303a'), (0.5, '#2e5a64'), (1, '#9cc8b8')],
           4: [(0, '#0a1430'), (0.35, '#20408a'), (0.7, '#7ab0d8'), (1, '#f2e080')],
           5: [(0, '#f2c840'), (1, '#fff4b0')]}
    def tf(c, reg, x, y): return ramp(pal[reg], lum(c)*1.15)
    underpaint_geo(ic, tf)
    centers = [(MOON[0], MOON[1], 1.0), (150, 70, 0.8), (40, 170, -0.7), (520, 190, 0.6)]
    def flow(x, y, reg):
        if reg == 0 or reg == 5:
            vx, vy = 1.0, 0.0
            for cx, cy, k in centers:
                dx, dy = x - cx, y - cy; r2 = dx*dx + dy*dy + 400; s = k*5000/r2
                vx += -dy/math.sqrt(r2)*s; vy += dx/math.sqrt(r2)*s
            return math.atan2(vy, vx)
        if reg in (1, 3): return math.pi/2 + 0.35*math.sin(x/20 + y/40)
        if reg == 2: return math.pi/2
        return 0.15*math.sin(x/30 + y/9)
    items = []
    for i in range(7000):
        x = rnd.uniform(-128, 640); y = rnd.uniform(-128, 640); reg = region(x, y)
        L = rnd.uniform(14, 24); W = rnd.uniform(3, 4.6); pts = [(x, y)]
        for k in range(4):
            a = flow(*pts[-1], reg); pts.append((pts[-1][0] + L/4*math.cos(a), pts[-1][1] + L/4*math.sin(a)))
        Lft = []; Rgt = []
        for k, (px, py) in enumerate(pts):
            a = flow(px, py, reg); hw = W/2*(0.55 + 0.45*math.sin(math.pi*k/4)); nx, ny = -math.sin(a), math.cos(a)
            Lft.append((px + nx*hw, py + ny*hw)); Rgt.append((px - nx*hw, py - ny*hw))
        c = tf(sample(x, y), reg, x, y) + np.array([rnd.gauss(0, 0.05) for _ in range(3)])
        items.append((hexc(np.round(c*10)/10), smooth(Lft + Rgt[::-1])))
    group_paths(ic, items, 0.95)
    # the moon in its rings
    for k, rr in enumerate([MOON[2]*1.55, MOON[2]*1.9, MOON[2]*2.3]):
        for j in range(14):
            a0 = 2*math.pi*j/14 + k*0.2; a1 = a0 + 0.4
            pts = [(MOON[0] + rr*math.cos(a0 + (a1 - a0)*t), MOON[1] + rr*math.sin(a0 + (a1 - a0)*t)) for t in (0, 0.5, 1)]
            ic.path(smooth([(x + 1.8*math.cos(a0), y + 1.8*math.sin(a0)) for x, y in pts] + [(x - 1.8*math.cos(a0), y - 1.8*math.sin(a0)) for x, y in pts[::-1]]), ['#f6e080', '#fff0b0', '#e8c850'][k], 0.85 - k*0.18)
    crescent(ic, *MOON[:2], MOON[2], ox=-MOON[2]*0.55, oy=-MOON[2]*0.36, r2=MOON[2]*0.86)
    # cypresses in place of the firs: dark flames of strokes
    for x, base, h in g['trees']:
        h *= 1.35; w = h*0.24
        for j in range(16):
            u = rnd.uniform(-1, 1); y0 = base - rnd.uniform(0, h*0.9); y1 = y0 + rnd.uniform(h*0.2, h*0.4)
            half = w*(1 - ((base - y0)/h)**1.4)
            xa = x + u*half
            ic.path(smooth([(xa - 2, y1), (xa - 1.5 + 3*math.sin(j), (y0 + y1)/2), (xa, y0 - 3), (xa + 1.8 + 3*math.sin(j), (y0 + y1)/2), (xa + 2, y1)]), rnd.choice(['#0a2018', '#12302a', '#1c4436', '#061210']), 0.95)
    return ic

# ------------------------------------------------------------------ stained glass: panes of jewel color in lead, like the app's glass
def glass():
    ic = Icon()
    rng = random.Random(7)
    cand = [(rng.uniform(-140, 652), rng.uniform(-140, 652)) for _ in range(3000)]
    sites = []
    for x, y in cand:
        sp = 30 if region(x, y) in (0, 4) else 22
        if all((x - a)**2 + (y - b)**2 > sp**2 for a, b in sites): sites.append((x, y))
    def clip(poly, a, b, c):
        out = []
        for i in range(len(poly)):
            p, q = poly[i], poly[(i + 1) % len(poly)]
            pi_, qi = a*p[0] + b*p[1] - c, a*q[0] + b*q[1] - c
            if pi_ <= 0: out.append(p)
            if (pi_ < 0) != (qi < 0) and pi_ != qi:
                t = pi_/(pi_ - qi); out.append((p[0] + (q[0] - p[0])*t, p[1] + (q[1] - p[1])*t))
        return out
    cells = []
    for x, y in sites:
        cell = [(-150, -150), (662, -150), (662, 662), (-150, 662)]
        for (u, v_) in sorted(sites, key=lambda s_: (s_[0] - x)**2 + (s_[1] - y)**2)[1:24]:
            cell = clip(cell, u - x, v_ - y, ((u*u + v_*v_) - (x*x + y*y))/2)
        if len(cell) >= 3: cells.append(((x, y), cell))
    lead = '#1a1222'
    ic.path(P([(-128, -128), (640, -128), (640, 640), (-128, 640)]), lead)
    pal = {'sky': lambda y: ['#1e2a7a', '#2a2090', '#3a2a9a', '#4a2a8a', '#5a3aa0', '#6a3a90'][min(5, max(0, int((y + 128)/90)))],
           'far': lambda y: ['#7a4aa8', '#6a3a98', '#8a5ab8', '#5a2e88'],
           'water': lambda y: ['#f4f0ff', '#e4ecff', '#ffffff', '#d8e4fa', '#efe6ff'],
           'near': lambda y: ['#1c4a4a', '#245a52', '#16383e', '#2e6458', '#1a3a50'],
           'pool': lambda y: ['#1e3a8a', '#24489a', '#183070', '#2a2a84', '#3456a8'],
           'moon': lambda y: ['#f6c040', '#f8d468', '#e8a830'],
           'ridge': lambda y: ['#4a3a90', '#58429c', '#3e3282']}
    def panes(clipd, key, inset=0.84):
        cid = ic.gid('c'); ic.defs.append(f'<clipPath id="{cid}"><path d="{clipd}"/></clipPath>'); ic.raw(f'<g clip-path="url(#{cid})">')
        by = {}
        for (x, y), cell in cells:
            cx = sum(p[0] for p in cell)/len(cell); cy = sum(p[1] for p in cell)/len(cell)
            ch = pal[key](cy); col = ch if isinstance(ch, str) else rng.choice(ch)
            by.setdefault(col, []).append(P([(cx + (px - cx)*inset, cy + (py - cy)*inset) for px, py in cell]))
        for col, ds in by.items(): ic.path(' '.join(ds), col)
        # a sheen across the glass, light from the upper left
        ic.path(P([(-128, -128), (640, -128), (640, 640), (-128, 640)]), ic.lin(-128, -128, 640, 640, [(0, '#ffffff', 0.18), (0.5, '#ffffff', 0), (1, '#000000', 0.15)]))
        ic.raw('</g>')
        ic.raw(f'<path d="{clipd}" fill="none" stroke="{lead}" stroke-width="5" stroke-linejoin="round"/>')
    mx, my, mr = MOON
    panes(P(sky_rect()), 'sky')
    panes(under(g['ridge2'], H + 2), 'ridge')
    panes(P(g['far']), 'far')
    panes(smooth(g['water']), 'water', 0.8)
    panes(smooth(g['left']), 'near'); panes(smooth(g['right']), 'near')
    panes(P(pool_rect()), 'pool')
    cr = Icon(); crescent(cr, mx, my, mr, ox=-mr*0.55, oy=-mr*0.36, r2=mr*0.86); moon_d = cr.body[-1].split('d="')[1].split('"')[0]
    ic.raw(f'<path d="{moon_d}" fill="#f6c858" stroke="{lead}" stroke-width="5"/>')
    ic.raw(f'<path d="{moon_d}" fill="#fff0b0" opacity="0.5" transform="translate({mx} {my}) scale(0.7) translate({-mx} {-my})"/>')
    return ic

# ------------------------------------------------------------------ pixel art: the scene at 72 pixels across the full layer, sixteen colors
def pixel():
    ic = Icon()
    import subprocess
    subprocess.run(['node', os.path.join(HERE, 'render.mjs'), os.path.join(HERE, '..', 'round15', 'round15.svg'), '/tmp/_px.png', '64', '-128 -128 768 768'], check=True, cwd=HERE)
    from PIL import ImageEnhance
    im = ImageEnhance.Color(ImageEnhance.Contrast(Image.open('/tmp/_px.png').convert('RGB')).enhance(1.35)).enhance(1.4)
    q = im.quantize(colors=18, method=Image.Quantize.MEDIANCUT, dither=Image.Dither.NONE).convert('RGB')
    a = np.asarray(q); s = 768/64
    by = {}
    for yy in range(64):
        xx = 0
        while xx < 64:
            c = tuple(a[yy, xx]); x2 = xx
            while x2 + 1 < 64 and tuple(a[yy, x2 + 1]) == c: x2 += 1
            by.setdefault('#%02x%02x%02x' % c, []).append(f"M{f(-128 + xx*s)},{f(-128 + yy*s)}h{f((x2 - xx + 1)*s + 0.3)}v{f(s + 0.3)}h{f(-(x2 - xx + 1)*s - 0.3)}Z")
            xx = x2 + 1
    for col, ds in by.items(): ic.path(''.join(ds), col)
    return ic

if __name__ == '__main__':
    name, out = sys.argv[1], sys.argv[2]
    open(out, 'w').write(globals()[name]().svg()); print(out)
