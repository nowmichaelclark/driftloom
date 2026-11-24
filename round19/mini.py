# Round 19: round 17's 1 (dusk), pared back: no junipers, no streaks, clean bands,
# a full moon, and the water redrawn three ways (1 to 3).
# python3 mini.py <1|2|3> out.svg
import math, os, sys, random
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, '..', 'round16')); sys.path.insert(0, os.path.join(HERE, '..', 'round17'))
from geo import *
from nm import V, mix, clip_open

v = V['dusk']
MX, MY, MR = 356, 98, 36

def lens(ic, x, y, w, t, col, o):
    """A tapered glint: thin at both ends, t thick in the middle."""
    ic.path(f"M{f(x - w/2)},{f(y)} Q{f(x)},{f(y - t)} {f(x + w/2)},{f(y)} Q{f(x)},{f(y + t)} {f(x - w/2)},{f(y)} Z", col, o)

def ring(ic, cx, cy, rx, ry, w, col, o, gap=None):
    """A flat ripple ring, thicker in front, thin at the sides; gap=(a0, a1) leaves an arc out."""
    out_ = []; in_ = []
    for i in range(49):
        a = math.pi*2*i/48
        if gap and gap[0] < a < gap[1]: continue
        out_.append((cx + rx*math.cos(a), cy + ry*math.sin(a)))
        th = w*(0.25 + 0.75*max(0, math.sin(a)))
        in_.append((cx + (rx - th*0.4)*math.cos(a), cy + (ry - th)*math.sin(a)))
    ic.path(P(out_ + in_[::-1]), col, o)

def moon(ic):
    ic.circle(MX, MY, MR*3.2, ic.rad([(0, '#fff8ec', 0.42), (0.45, '#fff8ec', 0.12), (1, '#fff8ec', 0)]))
    ic.circle(MX, MY, MR, ic.rad([(0, '#fffaf0', 1), (0.8, '#fff3dc', 1), (1, '#f6e2c4', 1)]))
    # a few soft seas, faint, roughly where the real ones sit
    for dx, dy, r, sx, sy in [(-8, -10, 15, 1.3, 0.8), (10, -6, 11, 1, 1.1), (-10, 10, 10, 1, 1.2)]:
        ic.circle(MX + dx, MY + dy, r, ic.rad([(0, '#e6d0bc', 0.5), (1, '#e6d0bc', 0)]), sx=sx, sy=sy)

def cliff(ic, poly, x0, x1, top, depth, seed):
    clip_open(ic, smooth(poly))
    n = Noise(seed, 3); B = v['bands']
    fr = [0, 0.05, 0.36, 0.5, 0.62, 0.78, 1.2]
    for i in range(len(B)):
        ya = top + (H - top)*fr[i] - 4
        edge = [(x, ya + 3*n(x/70 + i)) for x in range(int(x0) - 12, int(x1) + 14, 12)]
        c0 = mix(B[i], v['haze'], depth); c1 = mix(B[min(i + 1, len(B) - 1)], v['haze'], depth)
        ic.path(smooth(edge, close=False) + f" L{x1 + 14},{H + 40} L{x0 - 12},{H + 40} Z",
                ic.lin(0, ya, 0, top + (H - top)*fr[i + 1], [(0, c0, 1), (0.7, c0, 1), (1, c1, 1)]))
    ic.path(P([(x0 - 20, top - 20), (x1 + 20, top - 20), (x1 + 20, H + 20), (x0 - 20, H + 20)]),
            ic.lin(x0, 0, x1, 0, [(0, v['shade'], 0.34*(1 - depth)), (0.55, v['shade'], 0), (0.85, v['light'], 0.1), (1, v['light'], 0.26*(1 - depth*0.6))]))
    if depth == 0: ic.path(P([(x0 - 20, top - 20), (x1 + 20, top - 20), (x1 + 20, H + 20), (x0 - 20, H + 20)]), v['shade'], 0.12)
    ic.path(P([(x0 - 20, H - 70), (x1 + 20, H - 70), (x1 + 20, H + 20), (x0 - 20, H + 20)]),
            ic.lin(0, H - 70, 0, H, [(0, v['shade'], 0), (1, v['shade'], 0.3*(1 - depth))]))
    ic.raw('</g>')

def build(water):
    ic = Icon(); g = geo()
    ic.path(P(sky_rect()), ic.lin(0, -128, 0, 210, [(o, c, 1) for o, c in v['sky']]))
    moon(ic)
    mesa = [(-130, 200), (-40, 196), (20, 172), (60, 152), (90, 145), (170, 144), (196, 152), (250, 174), (330, 188), (420, 182), (520, 194), (660, 198)]
    ic.path(smooth(mesa, close=False) + f" L640,{H} L-128,{H} Z", ic.lin(0, 140, 0, 230, [(0, v['mesa'], 1), (1, mix(v['mesa'], v['haze'], 0.5), 1)]))
    # the far cliff, hazier, its top lit
    cliff(ic, g['far'], 60, 460, LIP - 4, v['hz'], 31)
    ic.path(P(g['ftop'] + [(x, y + 2.5) for x, y in g['ftop'][::-1]]), v['light'], 0.4)
    ic.path(P([(-128, H - 110), (640, H - 110), (640, H + 2), (-128, H + 2)]), ic.lin(0, H - 110, 0, H, [(0, v['haze'], 0), (1, v['haze'], 0.5)]))
    # the stream: one clean sheet, a gloss down the lit side, white at the foot
    ic.path(smooth(g['water']), ic.lin(0, LIP, 0, H, [(0, mix('#ffffff', v['pool'][0], 0.25), 1), (0.5, mix('#ffffff', v['pool'][0], 0.38), 1), (1, '#ffffff', 1)]))
    clip_open(ic, smooth(g['water']))
    gx = (lambda y, u: edges(y)[0] + (edges(y)[1] - edges(y)[0])*u)
    gl = [(gx(y, 0.6), y) for y in range(LIP + 4, H - 60, 12)] + [(gx(y, 0.86), y) for y in range(H - 60, LIP + 3, -12)]
    ic.path(smooth(gl), ic.lin(0, LIP, 0, H - 60, [(0, '#ffffff', 0.85), (1, '#ffffff', 0)]))
    sh = [(gx(y, 0.0), y) for y in range(LIP + 4, H, 12)] + [(gx(y, 0.16), y) for y in range(H, LIP + 3, -12)]
    ic.path(smooth(sh), ic.lin(0, LIP, 0, H, [(0, v['pool'][1], 0.22), (1, v['pool'][1], 0)]))
    ic.path(P([(150, H - 90), (360, H - 90), (360, H + 2), (150, H + 2)]), ic.lin(0, H - 90, 0, H, [(0, '#ffffff', 0), (1, '#ffffff', 0.9)]))
    ic.raw('</g>')
    ic.path(smooth([(LX0 - 2, LIP + 1), (256, LIP - 5), (LX1 + 2, LIP + 1), (LX1, LIP + 10), (256, LIP + 12), (LX0, LIP + 10)]), '#ffffff', 0.9)
    # the near cliffs and their talus
    cliff(ic, g['left'], -140, 190, min(y for x, y in g['lt']), 0, 32)
    cliff(ic, g['right'], 330, 660, min(y for x, y in g['rt']), 0, 33)
    for top in (g['lt'], g['rt']):
        ic.path(P(top + [(x, y + 2) for x, y in top[::-1]]), v['light'], 0.3)
    for pts, d in ((g['ledge'], 1), (g['redge'], -1)):
        a = [p for p in pts if p[1] > H - 80]
        sk = [(x + d*(4 + 46*((y - (H - 80))/80)**1.6), y) for x, y in a]
        ic.path(smooth(a + sk[::-1]), ic.lin(0, H - 80, 0, H, [(0, mix(v['bands'][4], v['shade'], 0.15), 1), (1, mix(v['bands'][5], v['shade'], 0.35), 1)]))
    # ---- the pool
    pc = v['pool']
    ic.path(P(pool_rect()), ic.lin(0, H, 0, 600, [(0, pc[0], 1), (0.35, pc[1], 1), (1, pc[2], 1)]))
    ic.path(P([(-128, H - 0.5), (640, H - 0.5), (640, H + 2), (-128, H + 2)]), ic.lin(-128, 0, 640, 0, [(0, '#ffffff', 0), (0.5, '#ffffff', 0.55), (1, '#ffffff', 0)]))
    r = random.Random(19)
    if water == 1:
        # still water: the stream and the moon mirrored, the mirror broken into tapered streaks by a calm swell
        y = H + 5; k = 0
        while y < H + 170:
            t = (y - H)/170; w = (FX1 - FX0 - 14)*(1 - 0.25*t)*r.uniform(0.75, 1.05)
            lens(ic, 256 + r.uniform(-3, 3)*(1 + 4*t), y, w, 2.0 + 3.0*t, '#ffffff', 0.6*(1 - t)**1.2)
            y += 5 + 15*t + r.uniform(0, 3); k += 1
        y = H + 8
        while y < H + 120:
            t = (y - H)/120
            lens(ic, MX + r.uniform(-2, 2)*(1 + 4*t), y, (30 - 10*t)*r.uniform(0.7, 1.05), 1.2 + 1.8*t, '#fff6e2', 0.7*(1 - t)**1.2)
            y += 6 + 16*t + r.uniform(0, 3)
        ring(ic, 256, H + 6, 74, 11, 2.6, '#ffffff', 0.35, gap=(math.pi*1.15, math.pi*1.85))
        ring(ic, 256, H + 7, 112, 17, 2.2, '#ffffff', 0.2, gap=(math.pi*1.1, math.pi*1.9))
    elif water == 2:
        # rings: the stream's landing spreads in flat rings that fade; the moon lays one soft column of light
        ic.circle(MX, H + 70, 60, ic.rad([(0, '#fff6e2', 0.35), (1, '#fff6e2', 0)]), sx=0.45, sy=1.3)
        for k, (rx, w, o) in enumerate([(66, 3.0, 0.55), (96, 2.6, 0.4), (132, 2.2, 0.28), (176, 1.8, 0.18), (228, 1.5, 0.1)]):
            ring(ic, 256, H + 8 + k*2, rx, rx*0.17, w, '#ffffff', o, gap=(math.pi*1.05, math.pi*1.95))
        for k in range(4):
            lens(ic, MX + r.uniform(-2, 2), H + 14 + 18*k**1.2, 20 - k*3, 1.8, '#fff6e2', 0.6 - k*0.12)
    else:
        # glints: a path of tapered glints under the moon, widening toward you; the stream's light a soft glow
        ic.circle(256, H + 40, 70, ic.rad([(0, '#ffffff', 0.4), (1, '#ffffff', 0)]), sx=0.9, sy=0.9)
        for k in range(14):
            t = k/13; y = H + 6 + 150*t**1.3; spread = 6 + 50*t
            for j in range(1 + (k > 3) + (k > 8)):
                lens(ic, MX + r.uniform(-spread, spread), y + r.uniform(-2, 2), 8 + 26*t*r.uniform(0.6, 1.1), 1.2 + 2*t, '#fff6e2', 0.75 - 0.45*t)
        ring(ic, 256, H + 6, 72, 11, 2.4, '#ffffff', 0.35, gap=(math.pi*1.15, math.pi*1.85))
    # the mist at the foot, a few soft clouds
    for dx, dy, rr, sx, sy in [(-40, -6, 26, 1.5, 0.7), (40, -6, 26, 1.5, 0.7), (0, -18, 34, 1.2, 0.85), (0, -2, 50, 1.7, 0.6)]:
        ic.circle(256 + dx, H + dy, rr, ic.rad([(0, '#ffffff', 0.85), (0.6, '#ffffff', 0.3), (1, '#ffffff', 0)]), sx=sx, sy=sy)
    ic.circle(256, 250, 660, ic.rad([(0.33, '#1a0a14', 0), (0.75, '#1a0a14', 0.3), (1, '#1a0a14', 0.42)]))
    return ic

if __name__ == '__main__':
    open(sys.argv[2], 'w').write(build(int(sys.argv[1])).svg()); print(sys.argv[2])
