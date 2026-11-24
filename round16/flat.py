# Round 16, the drawn styles: a flat cartoon (after Samurai Jack), 90s 3D gloss (after Kai's Power Tools and Bryce),
# New Mexico (Ghost Ranch cliffs, O'Keeffe's soft forms), Oklahoma (its flat-style painting and red earth),
# a woodblock print, a national-park poster.  python3 flat.py <style> out.svg
import math, os, sys, random
from geo import *
rnd = random.Random(16)
mx, my, mr = MOON

def deci(pts, k): return pts[::k] + ([pts[-1]] if (len(pts) - 1) % k else [])
def deci_poly(pts, k, keep=2): return deci(pts[:-keep], k) + pts[-keep:]
def outline(ic, d, fill, stroke, w, o=1):
    ic.raw(f'<path d="{d}" fill="{fill}" stroke="{stroke}" stroke-width="{w}" stroke-linejoin="round"' + (f' opacity="{o}"' if o < 1 else '') + '/>')
def moon_d(cx=mx, cy=my, r=mr):
    t = Icon(); crescent(t, cx, cy, r, ox=-r*0.55, oy=-r*0.36, r2=r*0.86); return t.body[-1].split('d="')[1].split('"')[0]
def bars(ic, col, ys, o=1, x=256):
    for i, (y, w) in enumerate(ys):
        ic.path(P([(x - w/2, y - 1.6), (x + w/2, y - 1.6), (x + w/2, y + 1.6), (x - w/2, y + 1.6)]), col, o)
def trees_path(fn, items): return ' '.join(fn(*t) for t in items)

# ------------------------------------------------------------------ flat cartoon: stepped sky, black angular rock, no lines, no gradients
def cartoon():
    ic = Icon(); g = geo()
    bands = ['#1c0f33', '#3a1a55', '#6a2a6a', '#a8406c', '#e06a5a', '#f6a060']
    for i, c in enumerate(bands):
        y0 = -128 + i*((H - 60 + 128)/len(bands)); ic.path(P([(-128, y0), (640, y0), (640, H + 2), (-128, H + 2)]), c)
    ic.raw(f'<path d="{moon_d()}" fill="#fff4dc"/>')
    ic.path(P(deci(g['ridge1'], 3) + [(640, H), (-128, H)]), '#5a2058')
    far = deci(g['ftop'], 4) + [(460, H + 4), (60, H + 4)]
    ic.path(P(far), '#2a1236')
    for x0 in range(70, 450, 34):     # flat angular shadow planes on the far rock
        if LX0 - 30 < x0 < LX1 + 4: continue
        w = rnd.uniform(10, 20); y0 = LIP + rnd.uniform(10, 40)
        ic.path(P([(x0, y0), (x0 + w, y0 + 18), (x0 + w*0.7, H + 4), (x0 - 6, H + 4)]), '#1a0a24')
        ic.path(P([(x0 + w + 2, y0 + 20), (x0 + w + 5, y0 + 24), (x0 + w + 2, H + 4), (x0 + w - 1, H + 4)]), '#c8506c', 0.9)
    wt = [(LX0, LIP), (LX1, LIP), (FX1, H), (FX0, H)]
    ic.path(P(wt), '#f6eeff')
    for u, w in ((0.18, 4), (0.42, 6), (0.7, 3), (0.86, 5)):
        ic.path(P([(LX0 + (LX1 - LX0)*u, LIP + 14), (LX0 + (LX1 - LX0)*u + w, LIP + 14), (FX0 + (FX1 - FX0)*u + w*1.4, H), (FX0 + (FX1 - FX0)*u, H)]), '#9ab0e8')
    for dx, dy, r in [(-50, -2, 22), (-28, -14, 26), (0, -20, 30), (28, -14, 26), (52, -2, 22)]: ic.circle(256 + dx, H + dy, r, '#ffffff')
    for poly in (g['left'], g['right']): ic.path(P(deci_poly(poly, 3)), '#0c0614')
    for pts in (g['ledge'],):          # a single flat edge of light on the left rock
        e = deci(pts, 3); ic.path(P(e + [(x - 5, y) for x, y in e[::-1]]), '#e06a5a')
    def tri(x, base, h):
        return P([(x, base - h*1.15), (x + h*0.16, base - h*0.35), (x + h*0.06, base - h*0.35), (x + h*0.2, base), (x - h*0.2, base), (x - h*0.06, base - h*0.35), (x - h*0.16, base - h*0.35)])
    ic.path(trees_path(tri, g['trees']), '#0c0614')
    ic.path(P(pool_rect()), '#160a24')
    bars(ic, '#f6eeff', [(H + 16, 110), (H + 34, 80), (H + 54, 56), (H + 78, 36), (H + 106, 20)], 0.9)
    bars(ic, '#fff4dc', [(H + 24, 30), (H + 44, 22), (H + 70, 14)], 0.8, x=mx)
    return ic

# ------------------------------------------------------------------ 90s 3D gloss: chrome rock, a checkered mirror, a glossy sphere, a lens flare
def gloss():
    ic = Icon(); g = geo()
    ic.path(P(sky_rect()), ic.lin(0, -128, 0, H, [(0, '#060a30', 1), (0.45, '#1a3a8a', 1), (0.8, '#5a7ac8', 1), (0.93, '#f0a070', 1), (1, '#ffd8a0', 1)]))
    for x, y, r in [(80, 120, 60), (180, 150, 40), (470, 170, 70)]:
        ic.circle(x, y, r, ic.rad([(0, '#ffffff', 0.35), (1, '#ffffff', 0)]), sx=2.4, sy=0.35)
    ic.path(under(g['ridge1'], H + 2), ic.lin(0, 150, 0, 230, [(0, '#8a7ac0', 1), (0.5, '#3a3070', 1), (1, '#6a5aa0', 1)]))
    chrome = lambda y0, y1, a, b: ic.lin(0, y0, 0, y1, [(0, a, 1), (0.18, '#ffffff', 1), (0.32, b, 1), (0.6, a, 1), (0.8, '#1a1030', 1), (1, b, 1)])
    ic.path(P(g['far']), chrome(LIP, H, '#8a5ab0', '#4a2a78'))
    ic.path(smooth(g['water']), ic.lin(LX0, 0, LX1, 0, [(0, '#80c8ff', 1), (0.25, '#ffffff', 1), (0.45, '#a8e0ff', 1), (0.75, '#4a90e0', 1), (1, '#c8ecff', 1)]))
    for poly, x0, x1 in ((g['left'], -128, 180), (g['right'], 340, 640)):
        ic.path(smooth(poly), ic.lin(x0, 0, x1, 0, [(0, '#2a1a48', 1), (0.55, '#6a4a98', 1), (0.8, '#e8d0ff', 1), (0.9, '#5a3a88', 1), (1, '#20103a', 1)]))
        ic.path(smooth(poly), ic.lin(0, 40, 0, H, [(0, '#ffffff', 0.25), (0.3, '#ffffff', 0), (1, '#000000', 0.5)]))
    # the checkered mirror: Bryce's ground plane
    ic.path(P(pool_rect()), ic.lin(0, H, 0, 640, [(0, '#3a4a9a', 1), (1, '#0a0a24', 1)]))
    rows = [H + 2]
    while rows[-1] < 640: rows.append(rows[-1] + (rows[-1] - H + 4)*0.35 + 2)
    for i in range(len(rows) - 1):
        y0, y1 = rows[i], rows[i + 1]
        for j in range(-30, 31):
            if (i + j) % 2: continue
            def X(y, j): return 256 + j*36*(y - H + 6)/60
            ic.path(P([(X(y0, j), y0), (X(y0, j + 1), y0), (X(y1, j + 1), y1), (X(y1, j), y1)]), '#c8d0ff', 0.16)
    ic.path(P([(FX0, H + 2), (FX1, H + 2), (FX1 + 20, 600), (FX0 - 20, 600)]), ic.lin(0, H, 0, 600, [(0, '#c8ecff', 0.6), (1, '#c8ecff', 0)]))
    # the glossy sphere moon, and its flare
    ic.circle(mx, my, mr*2.4, ic.rad([(0, '#ffe8b0', 0.5), (1, '#ffe8b0', 0)]))
    ic.circle(mx, my, mr, ic.rad([(0, '#fff8e0', 1), (0.6, '#f8c860', 1), (1, '#b06820', 1)]))
    ic.circle(mx - mr*0.35, my - mr*0.4, mr*0.32, ic.rad([(0, '#ffffff', 0.95), (1, '#ffffff', 0)]))
    ic.path(P([(mx - 140, my - 1.2), (mx + 140, my - 1.2), (mx + 140, my + 1.2), (mx - 140, my + 1.2)]), ic.lin(mx - 140, 0, mx + 140, 0, [(0, '#ffffff', 0), (0.5, '#ffffff', 0.8), (1, '#ffffff', 0)]))
    for k, (t, r, c) in enumerate([(0.35, 10, '#a0ffd0'), (0.6, 18, '#ffa0e0'), (0.85, 7, '#ffffff'), (1.2, 26, '#80c0ff')]):
        ic.circle(mx + (256 - mx)*t*1.6, my + (300 - my)*t*1.6, r, c, 0.18)
    return ic

# ------------------------------------------------------------------ New Mexico: banded cliffs, a flat-topped mesa, turquoise, junipers
def newmexico():
    ic = Icon(); g = geo()
    ic.path(P(sky_rect()), ic.lin(0, -128, 0, H, [(0, '#1a3a6a', 1), (0.45, '#3a8aa8', 1), (0.8, '#e8b098', 1), (1, '#f6d4b0', 1)]))
    ic.circle(mx, my, mr*2.6, ic.rad([(0, '#fff8ec', 0.35), (1, '#fff8ec', 0)]))
    ic.raw(f'<path d="{moon_d()}" fill="#fffaf0"/>')
    # the mesa: flat-topped, like the one O'Keeffe painted over and over
    mesa = [(-130, 200), (-40, 196), (20, 170), (60, 150), (90, 142), (170, 141), (200, 150), (250, 172), (330, 186), (420, 180), (520, 192), (660, 198)]
    ic.path(smooth(mesa, close=False) + f" L640,{H} L-128,{H} Z", ic.lin(0, 140, 0, 230, [(0, '#6a6aa8', 1), (1, '#9a88b8', 1)]))
    bands = ['#f4d8b0', '#e8a07a', '#c8604a', '#f0c088', '#d47860', '#a84a3e']
    def banded(poly, x0, x1, y0, y1, k=1.0, dk=0.0):
        cid = ic.gid('c'); ic.defs.append(f'<clipPath id="{cid}"><path d="{smooth(poly)}"/></clipPath>'); ic.raw(f'<g clip-path="url(#{cid})">')
        n = Noise(rnd.randint(1, 999), 3); y = y0; i = 0
        while y < y1 + 20:
            h = rnd.uniform(14, 34); top = [(x, y + 6*n(x/80 + i)) for x in range(int(x0), int(x1) + 12, 12)]
            ic.path(smooth(top, close=False) + f" L{x1},{y1 + 40} L{x0},{y1 + 40} Z", ic.lin(0, y, 0, y + h, [(0, bands[i % len(bands)], 1), (1, bands[(i + 1) % len(bands)], 1)]))
            y += h; i += 1
        ic.path(P([(x0, y0 - 40), (x1, y0 - 40), (x1, y1 + 40), (x0, y1 + 40)]), ic.lin(x0, 0, x1, 0, [(0, '#3a1418', dk), (1, '#3a1418', dk)]))
        ic.path(P([(x0, y0 - 40), (x1, y0 - 40), (x1, y1 + 40), (x0, y1 + 40)]), ic.lin(0, y0, 0, y1, [(0, '#ffffff', 0.1*k), (1, '#2a0c10', 0.45)]))
        ic.raw('</g>')
    banded(g['far'], 60, 460, LIP - 8, H)
    ic.path(smooth(g['water']), ic.lin(0, LIP, 0, H, [(0, '#e0f4f4', 1), (0.5, '#c0e8ea', 1), (1, '#ffffff', 1)]))
    ic.path(P([(LX0 + 8, LIP + 4), (LX0 + 22, LIP + 4), (FX0 + 26, H - 60), (FX0 + 10, H - 60)]), ic.lin(0, LIP, 0, H - 60, [(0, '#ffffff', 0.9), (1, '#ffffff', 0)]))
    banded(g['left'], -140, 180, 40, H, dk=0.35); banded(g['right'], 340, 660, 100, H, dk=0.3)
    # junipers: rounded, dark sage
    jun = []
    for x, base, h in g['trees']:
        for k in range(3):
            r = h*0.18*(1 - k*0.2); jun.append((x + rnd.uniform(-6, 6), base - h*0.15 - k*r*1.1, r))
    for x, y, r in jun: ic.circle(x, y, r, '#3a5a48')
    for x, y, r in jun: ic.circle(x - r*0.25, y - r*0.3, r*0.6, '#6a8a6a', 0.7)
    ic.path(P(pool_rect()), ic.lin(0, H, 0, 600, [(0, '#5ac0c0', 1), (0.4, '#2a8a9a', 1), (1, '#123a4a', 1)]))
    bars(ic, '#ffffff', [(H + 14, 110), (H + 32, 84), (H + 54, 60), (H + 80, 40), (H + 110, 24)], 0.55)
    for dx, dy, rr, sx, sy in [(-40, -6, 26, 1.5, 0.7), (40, -6, 26, 1.5, 0.7), (0, -20, 36, 1.2, 0.85), (0, -2, 52, 1.7, 0.6)]:
        ic.circle(256 + dx, H + dy, rr, ic.rad([(0, '#ffffff', 0.9), (1, '#ffffff', 0)]), sx=sx, sy=sy)
    return ic

# ------------------------------------------------------------------ Oklahoma: the flat style's plain ground, fine lines, flat color; red earth, cedars
def oklahoma():
    ic = Icon(); g = geo(); ink = '#2a1810'
    ic.path(P([(-128, -128), (640, -128), (640, 640), (-128, 640)]), '#f0e2c4')
    outline(ic, moon_d(), '#e8a838', ink, 2.5)
    for k in range(10):   # a ring of small marks around the moon
        a = 2*math.pi*k/10; ic.circle(mx + mr*1.6*math.cos(a), my + mr*1.6*math.sin(a), 2.6, '#c84a30')
    ridge = deci(g['ridge2'], 4)
    outline(ic, P(ridge + [(640, H), (-128, H)]), '#d8c09a', ink, 2)
    outline(ic, P(deci_poly(g['far'], 3)), '#e8b070', ink, 2.5)
    # a stepped band across the far rock
    for x in range(70, 450, 18):
        if LX0 - 12 < x < LX1 + 4: continue
        outline(ic, P([(x, 300), (x + 6, 300), (x + 6, 294), (x + 12, 294), (x + 12, 300), (x + 18, 300), (x + 18, 306), (x, 306)]), '#2a7a78', ink, 1.2)
    outline(ic, P([(LX0, LIP), (LX1, LIP), (FX1, H), (FX0, H)]), '#fbf6ea', ink, 2.5)
    for u in (0.2, 0.4, 0.6, 0.8):
        pts = [(LX0 + (LX1 - LX0)*u + (FX0 + (FX1 - FX0)*u - LX0 - (LX1 - LX0)*u)*t + 3*math.sin(t*14), LIP + 8 + (H - LIP - 16)*t) for t in [i/20 for i in range(21)]]
        ic.raw(f'<path d="{smooth(pts, close=False)}" fill="none" stroke="#3a8ab0" stroke-width="2"/>')
    for poly, fill in ((g['left'], '#b5523a'), (g['right'], '#9a4230')):
        outline(ic, P(deci_poly(poly, 2)), fill, ink, 2.5)
    # dots along the left rock's edge, triangles along the right's
    for x, y in g['ledge'][2:-2:2]: ic.circle(x - 10, y, 2.4, '#f0e2c4')
    for x, y in g['redge'][1:-2:2]: ic.path(P([(x + 6, y - 5), (x + 14, y), (x + 6, y + 5)]), '#f0e2c4')
    def cedar(x, base, h):
        return smooth([(x, base - h), (x + h*0.16, base - h*0.6), (x + h*0.22, base - h*0.2), (x + h*0.04, base - h*0.1), (x + h*0.04, base), (x - h*0.04, base), (x - h*0.04, base - h*0.1), (x - h*0.22, base - h*0.2), (x - h*0.16, base - h*0.6)])
    for t in g['trees']: outline(ic, cedar(*t), '#2e5a3a', ink, 1.8)
    outline(ic, P([(-140, H), (652, H), (652, 652), (-140, 652)]), '#5aa8a0', ink, 2.5)
    for k, rx in enumerate([60, 100, 140, 180]):
        ic.raw(f'<path d="M{256 - rx},{H + 8 + k*14} Q256,{H + 26 + k*22} {256 + rx},{H + 8 + k*14}" fill="none" stroke="#f0e2c4" stroke-width="2.4"/>')
    for x in range(-128, 640, 24):
        outline(ic, P([(x, 600), (x + 12, 584), (x + 24, 600)]), '#c84a30', ink, 1.4)
    return ic

# ------------------------------------------------------------------ woodblock print: Prussian blue fading into paper, inked outlines, lines of falling water
def woodblock():
    ic = Icon(); g = geo(); ink = '#232836'
    ic.path(P([(-128, -128), (640, -128), (640, 640), (-128, 640)]), '#f2e6c8')
    ic.path(P(sky_rect()), ic.lin(0, -128, 0, 150, [(0, '#1c2c5a', 1), (0.6, '#3a5a8a', 0.7), (1, '#f2e6c8', 0)]))
    outline(ic, moon_d(), '#f8f0d8', ink, 2)
    ic.path(under(g['ridge1'], H), ic.lin(0, 150, 0, 220, [(0, '#5a6a8a', 1), (1, '#f2e6c8', 0)]))
    outline(ic, P(g['far']), '#c8b890', ink, 2.2)
    ic.path(P(g['far']), ic.lin(0, LIP, 0, H, [(0, '#4a7a6a', 0.9), (0.4, '#4a7a6a', 0), (1, '#6a5a48', 0.3)]))
    outline(ic, smooth(g['water']), '#fbf8ee', ink, 2)
    for i in range(13):
        u = (i + 0.5)/13
        pts = []
        for t in [k/24 for k in range(25)]:
            y = LIP + 4 + (H - LIP - 20)*t; a, b = edges(y); pts.append((a + (b - a)*u + 1.6*math.sin(t*20 + i), y))
        ic.raw(f'<path d="{smooth(pts, close=False)}" fill="none" stroke="#3a5a9a" stroke-width="1.4" opacity="0.8"/>')
    for poly in (g['left'], g['right']):
        outline(ic, smooth(poly), '#8a8a6a', ink, 2.4)
        ic.path(smooth(poly), ic.lin(0, 40, 0, H, [(0, '#2a5a4a', 0.95), (0.35, '#3a6a5a', 0.4), (1, '#2a2a30', 0.7)]))
    # moss dots in clusters along the rock edges, as the print-makers cut them
    dots = []
    for pts, d_ in ((g['ledge'], -1), (g['redge'], 1)):
        for x, y in pts[::3]:
            for k in range(3): dots.append((x + d_*rnd.uniform(6, 16), y + rnd.uniform(-5, 5), rnd.uniform(1.3, 2.4)))
    for x, y in g['lt'][::2] + g['rt'][::2]:
        dots.append((x + rnd.uniform(-4, 4), y + rnd.uniform(4, 10), rnd.uniform(1.3, 2.4)))
    ic.path(' '.join(f"M{f(x - r)},{f(y)} a{f(r)},{f(r)} 0 1 0 {f(2*r)},0 a{f(r)},{f(r)} 0 1 0 {f(-2*r)},0 Z" for x, y, r in dots), ink)
    # pines: flat-topped clouds of needles on crooked trunks
    for x, base, h in g['trees']:
        ic.path(P([(x - 2.5, base), (x + 2.5, base), (x + 6, base - h*0.45), (x + 1, base - h*0.5)]), ink)
        for k, (ox, oy, w) in enumerate([(-h*0.12, 0.42, 0.34), (h*0.14, 0.58, 0.3), (0, 0.74, 0.24)]):
            cx = x + 3 + ox; cy = base - h*oy
            outline(ic, f"M{f(cx - h*w)},{f(cy)} Q{f(cx - h*w*0.5)},{f(cy - h*0.12)} {f(cx)},{f(cy - h*0.13)} Q{f(cx + h*w*0.5)},{f(cy - h*0.12)} {f(cx + h*w)},{f(cy)} Z", '#2a4a3a', ink, 1.4)
    ic.path(P(pool_rect()), ic.lin(0, H, 0, 560, [(0, '#3a6aa0', 1), (1, '#1c2c5a', 1)]))
    for k in range(6):
        rx = 50 + k*26; y = H + 8 + k*12
        ic.raw(f'<path d="M{256 - rx},{y} Q{256 - rx/2},{y - 6} 256,{y} T{256 + rx},{y}" fill="none" stroke="#f2e6c8" stroke-width="1.8" opacity="{0.9 - k*0.1:.2f}"/>')
    for dx, dy, r in [(-34, -4, 16), (-14, -10, 20), (10, -10, 20), (32, -4, 16)]:
        outline(ic, f"M{256 + dx - r},{H + dy} a{r},{r*0.8} 0 0 1 {2*r},0 Z", '#fbf8ee', ink, 1.6)
    return ic

# ------------------------------------------------------------------ national-park poster: six flat inks, bold planes, the moon's rays
def poster():
    ic = Icon(); g = geo()
    C = {'night': '#1d2a44', 'deep': '#0f1726', 'teal': '#2f5d6b', 'sand': '#e8d3a8', 'coral': '#d9714e', 'cream': '#f6ecd2'}
    ic.path(P(sky_rect()), C['night'])
    for k in range(12):
        a0 = 2*math.pi*k/12 + 0.1; a1 = a0 + 0.13
        ic.path(P([(mx, my), (mx + 600*math.cos(a0), my + 600*math.sin(a0)), (mx + 600*math.cos(a1), my + 600*math.sin(a1))]), C['teal'], 0.35)
    ic.raw(f'<path d="{moon_d(r=mr*1.1)}" fill="{C["cream"]}"/>')
    ic.path(P(deci(g['ridge1'], 2) + [(640, H), (-128, H)]), C['teal'])
    far = deci_poly(g['far'], 2); ic.path(P(far), C['coral'])
    for x0 in range(70, 450, 40):      # the far rock's shaded planes, one flat ink
        if LX0 - 40 < x0 < LX1 + 2: continue
        ic.path(P([(x0 + 16, LIP + 4), (x0 + 36, LIP + 2), (x0 + 30, H + 4), (x0 + 8, H + 4)]), '#a9503a')
    ic.path(P([(LX0, LIP), (LX1, LIP), (FX1, H), (FX0, H)]), C['cream'])
    ic.path(P([(LX0 + 30, LIP), (LX1, LIP), (FX1, H), (FX0 + 46, H)]), C['sand'])
    for poly in (g['left'], g['right']): ic.path(P(deci_poly(poly, 2)), C['deep'])
    e = deci(g['ledge'], 2); ic.path(P(e + [(x - 7, y) for x, y in e[::-1]]), C['coral'])
    def tri(x, base, h):
        return P([(x, base - h*1.1), (x + h*0.14, base - h*0.55), (x + h*0.07, base - h*0.55), (x + h*0.22, base), (x - h*0.22, base), (x - h*0.07, base - h*0.55), (x - h*0.14, base - h*0.55)])
    ic.path(trees_path(tri, g['trees']), C['deep'])
    ic.path(P(pool_rect()), C['night'])
    for dx, dy, r in [(-44, -2, 20), (-20, -12, 26), (8, -16, 28), (34, -10, 24), (54, -2, 18)]:
        ic.path(f"M{256 + dx - r},{H + 2} a{r},{r} 0 0 1 {2*r},0 Z", C['cream'])
    bars(ic, C['sand'], [(H + 20, 120), (H + 40, 90), (H + 62, 62), (H + 88, 40), (H + 118, 22)])
    ic.path(P([(-128, 600), (640, 600), (640, 640), (-128, 640)]), C['coral'])
    return ic

if __name__ == '__main__':
    name, out = sys.argv[1], sys.argv[2]
    open(out, 'w').write(globals()[name]().svg()); print(out)
