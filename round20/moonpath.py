# Round 20: round 19's 3 (moon glints) with the moon's reflection redrawn, three takes (a to c).
# Everything but the moon's path is round 19's mini.py, water 3.
# python3 moonpath.py <a|b|c> out.svg
import math, os, sys, random
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, '..', 'round19'))
import mini
from mini import *

def moon_path(ic, take):
    """The moon's light on the pool: straight down from the moon, from the far shore toward you,
    narrow at the shore and wider near you (perspective), brightest at the shore."""
    r = random.Random(20); pc = v['pool']; x0 = MX; top = H + 3; bot = H + 200
    def col_glow(w, h, a, cy=None):
        """A soft column of light: a radial glow stretched tall, clipped to the pool so it starts at the shore."""
        clip_open(ic, P(pool_rect()))
        ic.circle(x0, cy or top + h*0.25, h, ic.rad([(0, '#fff6e2', a), (0.5, '#fff6e2', a*0.45), (1, '#fff6e2', 0)]), sx=w/h, sy=1)
        ic.raw('</g>')
    if take == 'a':
        # glitter: many small glints scattered across the column, thickest in the middle, larger and sparser toward you
        col_glow(26, 150, 0.4)
        lens(ic, x0, top + 1.2, 34, 1.5, '#ffffff', 0.95)
        y = top + 5
        while y < bot:
            t = (y - top)/(bot - top); half = 5 + 34*t
            for j in range(2 + int(3*t)):
                u = max(-1, min(1, r.gauss(0, 0.45)))
                w = (6 + 22*t)*r.uniform(0.5, 1.1)*(1.2 - abs(u)*0.6)
                lens(ic, x0 + u*half, y + r.uniform(-1.5, 1.5), w, 0.7 + 1.4*t, '#fffaf0', (0.95 - 0.6*t)*(1 - abs(u)*0.5))
            y += 4 + 12*t**1.1 + r.uniform(0, 2)
    elif take == 'b':
        # a pillar of light: one soft column, cut across by the calm swell
        col_glow(26, 170, 0.75)
        y = top + 8
        while y < bot:
            t = (y - top)/(bot - top)
            lens(ic, x0 + r.uniform(-2, 2), y, 40 + 40*t, 0.7 + 1.4*t, pc[1], 0.45)
            y += 8 + 14*t + r.uniform(0, 3)
        lens(ic, x0, top + 1.2, 34, 1.5, '#ffffff', 0.95)
    else:
        # a sparkle at the shore: a bright streak where the light meets the far shore, a few glints fading toward you
        col_glow(20, 90, 0.35)
        lens(ic, x0, top + 1.2, 40, 1.8, '#ffffff', 0.95)
        lens(ic, x0 + 2, top + 5, 26, 1.3, '#fffaf0', 0.85)
        y = top + 10
        while y < H + 130:
            t = (y - top)/(H + 130 - top)
            for j in range(1 + (t > 0.25)):
                u = 0 if j == 0 else r.choice((-1, 1))*r.uniform(0.5, 1)
                lens(ic, x0 + r.uniform(-2, 2) + u*(6 + 16*t), y, (20 - 6*t)*(1 if j == 0 else 0.5)*r.uniform(0.8, 1.1), 0.9 + 1.0*t, '#fffaf0', (0.85 - 0.75*t)*(1 if j == 0 else 0.7))
            y += 6 + 14*t + r.uniform(0, 2)

# round 19's build() with water 3's moon glints swapped out
src = open(os.path.join(HERE, '..', 'round19', 'mini.py')).read()
a = src.index("        for k in range(14):"); b = src.index("        ring(ic, 256, H + 6, 72, 11, 2.4")
src = src[:a] + "        moon_path(ic, TAKE)\n" + src[b:]
src = src[:src.index("if __name__ == '__main__':")]
G = dict(globals()); G['__name__'] = 'r20'
exec(compile(src, 'round19/mini.py (water 3, new moon path)', 'exec'), G)
G['moon_path'] = moon_path

if __name__ == '__main__':
    G['TAKE'] = sys.argv[1]
    open(sys.argv[2], 'w').write(G['build'](3).svg()); print(sys.argv[2])
