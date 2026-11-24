# Round 18: a lit, textured painter for the New Mexico scene, traced afterwards into vector shapes.
# The rock is a height field (flutes worn by water, blocks, ledges at the strata) lit by one light; the strata follow
# Ghost Ranch's order; junipers are clumps shaded as forms; the water is streaked and foams at the foot; the pool mirrors it all.
# python3 ren.py <variation> out.png
import math, os, sys, random
import numpy as np
from PIL import Image, ImageDraw, ImageFilter
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'round16'))
from geo import geo, H, LIP, LX0, LX1, FX0, FX1, MOON, edges, Noise
S = 2; N = 768*S
def px(x): return (x + 128)*S
rng = np.random.default_rng(18)

def hexrgb(h): return np.array([int(h[i:i+2], 16)/255 for i in (1, 3, 5)], np.float32)
def noise(sx, sy, oct=4, seed=0):
    """Fractal value noise; sx, sy: the feature size in pixels at the first octave (stretch it to make flutes)."""
    r = np.random.default_rng(seed); out = np.zeros((N, N), np.float32); a = 1.0; tot = 0
    for o in range(oct):
        w = max(2, int(N/sx) + 2); h = max(2, int(N/sy) + 2)
        g = Image.fromarray((r.random((h, w))*255).astype(np.uint8)).resize((N, N), Image.BICUBIC)
        out += a*(np.asarray(g, np.float32)/255 - 0.5); tot += a; a *= 0.5; sx /= 2.1; sy /= 2.1
    return out/tot
def mask(polys, smooth_=False):
    im = Image.new('L', (N*2, N*2), 0); d = ImageDraw.Draw(im)
    for poly in polys: d.polygon([(px(x)*2, px(y)*2) for x, y in poly], fill=255)
    return np.asarray(im.resize((N, N), Image.LANCZOS), np.float32)/255
from scipy.ndimage import gaussian_filter
def blur(a, r): return gaussian_filter(a.astype(np.float32), r)
class Layer:
    def __init__(self): self.rgb = np.zeros((N, N, 3), np.float32); self.a = np.zeros((N, N), np.float32)
    def save(self, path):
        a = np.clip(self.a, 0, 1); rgb = np.where(a[..., None] > 1e-4, self.rgb/np.maximum(a, 1e-4)[..., None], 0)
        a8 = (a > 0.5).astype(np.uint8)*255     # hard edges: the tracer keys out the rest
        Image.fromarray(np.dstack([(np.clip(rgb, 0, 1)*255).astype(np.uint8), a8])).save(path)
def comp(img, col, alpha):
    a = alpha[..., None]
    if isinstance(img, Layer):
        img.rgb *= (1 - a); img.rgb += col*a; img.a = img.a*(1 - alpha) + alpha; return
    img *= (1 - a); img += col*a
def ramp(stops, t):
    t = np.clip(t, 0, 1); out = np.zeros(t.shape + (3,), np.float32)
    for (a, ca), (b, cb) in zip(stops, stops[1:]):
        m = (t >= a) & (t <= b); u = ((t - a)/((b - a) or 1))[..., None]
        out[m] = (hexrgb(ca) + (hexrgb(cb) - hexrgb(ca))*u)[m]
    return out
YY, XX = np.mgrid[0:N, 0:N].astype(np.float32)
Y = YY/S - 128; X = XX/S - 128

def rock(img, m, top_y, v, depth, seed, sharp=1.0, cap=None):
    """A cliff: buttresses, flutes and grain as a height field, lit by v['L'], colored by the strata; broken ledges
    with a lit lip and a shadow under; depth fades it toward the haze."""
    warp = noise(260, 120, 3, seed)*30
    span = (H - top_y + 1)
    rel = np.clip((Y - top_y + warp)/span, 0, 1.2)
    butt = noise(80, 700, 3, seed + 1); flut = noise(26, 240, 2, seed + 2); grain = noise(30, 8, 2, seed + 3)
    # the lower, softer beds weather to gullied slopes: sharp ridges and rills that run down the fall line
    low = np.clip((rel - 0.48)/0.18, 0, 1)
    rill = 1 - np.abs(noise(26, 150, 2, seed + 7))*2.2
    hgt = (30*butt + 9*flut + 1.2*grain*(1 - low) + 12*rill*low)*sharp
    bm = blur(m, 22)
    gy, gx = np.gradient(hgt + bm*26)
    nx, ny, nz = -gx, -gy, np.ones_like(gx)*1.3
    n = np.sqrt(nx*nx + ny*ny + nz*nz); nx /= n; ny /= n; nz /= n
    L = np.array(v['L'], np.float32); L /= np.linalg.norm(L)
    lam = np.clip(nx*L[0] + ny*L[1] + nz*L[2], 0, 1)
    alb = ramp(v['bands'], rel + noise(40, 40, 3, seed + 4)*0.05)
    var = np.clip((noise(9, 150, 2, seed + 5) - 0.1)*5, 0, 1)*(rel < 0.4)*(rel > 0.06)
    alb *= (1 - 0.3*var)[..., None]
    shade = np.ones_like(lam)
    brk = noise(90, 90, 2, seed + 6)
    for k, r in enumerate(v.get('ledges', (0.05, 0.36, 0.5, 0.64))):
        yk = top_y + span*r - warp + 6*noise(50, 50, 2, seed + 10 + k)
        dd = Y - yk; on = (brk + 0.08*k) > -0.04
        shade -= (np.clip(1 - dd/9, 0, 1)*(dd > 0))*0.45*on*sharp     # the shadow under the ledge
        lam = lam + (np.exp(-((dd + 1.5)/1.6)**2))*0.35*on*sharp      # its lit lip
    if cap is not None:
        alb = np.where((Y < cap)[..., None], alb*0.8, alb)
    col = alb*(v['amb'] + v['dif']*np.clip(lam, 0, 1.2))[..., None]*hexrgb(v['lightc'])*shade[..., None]
    col = col*(1 - depth) + hexrgb(v['haze'])*depth
    col *= (1 - 0.15*np.clip((Y - (H - 60))/60, 0, 1)*(1 - depth))[..., None]
    comp(img, col, m)
    return lam

def juniper_layer(img, spots, v, light_dx):
    """Junipers: clumps of foliage, each shaded as a rough sphere, a shadow cast on the ground."""
    jm = np.zeros((N, N), np.float32); sh = np.zeros((N, N), np.float32)
    im = Image.new('L', (N, N), 0); d = ImageDraw.Draw(im); ims = Image.new('L', (N, N), 0); ds = ImageDraw.Draw(ims)
    r_ = random.Random(3)
    for x, y, r in spots:
        cx, cy, rr = px(x), px(y), r*S
        for k in range(10):
            a = r_.uniform(0, 2*math.pi); dd = r_.uniform(0, rr*0.55); q = rr*r_.uniform(0.35, 0.6)
            ox, oy = cx + dd*math.cos(a), cy - rr*0.4 + dd*math.sin(a)*0.8
            d.ellipse((ox - q, oy - q, ox + q, oy + q), fill=255)
            ds.ellipse((ox - q - light_dx*rr*0.6, oy - q*0.4 + rr*0.9, ox + q - light_dx*rr*0.6, oy + q*0.4 + rr*0.9), fill=255)
    jm = np.asarray(im, np.float32)/255; sh = blur(np.asarray(ims, np.float32)/255, 3)
    comp(img, hexrgb(v['shade'])*0.6, sh*0.45*(1 - jm))
    tex = noise(4, 4, 2, 9)
    jb = blur(jm, 5); gy, gx = np.gradient(jb*14 + tex*3)
    lam = np.clip(-gx*light_dx*0.9 - gy*0.6 + 0.35, 0, 1)
    col = ramp([(0, v['jun0']), (0.5, v['jun1']), (1, v['jun2'])], lam + tex*0.4)
    jm = np.clip((jm - 0.5)*2 + tex*0.8, 0, 1)
    comp(img, col, jm)

def build(v, layout):
    g = geo(); img = np.zeros((N, N, 3), np.float32)
    # ---- the sky
    t = np.clip((Y + 128)/(H + 128), 0, 1); img[:] = ramp(v['sky'], t)
    mx, my, mr = v.get('moon', MOON)
    if v['obj'] in ('moon', 'moonw'):
        d1 = np.sqrt((X - mx)**2 + (Y - my)**2); d2 = np.sqrt((X - mx + mr*0.55)**2 + (Y - my + mr*0.36)**2)
        comp(img, hexrgb('#fff6e6'), np.exp(-(d1/(mr*2.6))**2)*0.35)
        cres = np.clip((mr - d1)*S, 0, 1)*np.clip((d2 - mr*0.86)*S, 0, 1)
        comp(img, hexrgb('#ffffff' if v['obj'] == 'moonw' else '#fff4dc'), cres)
    if v['obj'] == 'star':
        d1 = np.sqrt((X - mx)**2 + (Y - my)**2)
        comp(img, hexrgb('#fff4d8'), np.exp(-(d1/18)**2)*0.6)
        cross = np.exp(-(np.abs(X - mx)/0.9)**2)*np.exp(-(np.abs(Y - my)/11)**2) + np.exp(-(np.abs(Y - my)/0.9)**2)*np.exp(-(np.abs(X - mx)/11)**2)
        comp(img, hexrgb('#ffffff'), np.clip(cross + np.exp(-(d1/2.6)**2), 0, 1))
    # wisps of cloud lit by the low light
    if v.get('wisps'):
        cl = np.clip((noise(300, 26, 4, 70) + 0.02)*4, 0, 1)*np.exp(-((Y - v['wisps'])/30)**2)
        comp(img, hexrgb(v['wispc']), cl*0.7)
    # ---- the mesa, flat-topped, in the haze
    mesa = [(-130, 200), (-40, 196), (20, 172), (60, 152), (90, 145), (170, 144), (196, 152), (250, 174), (330, 188), (420, 182), (520, 194), (660, 198), (660, 400), (-130, 400)]
    mm = mask([mesa]); mcol = ramp([(0, v['mesa']), (1, v['haze'])], np.clip((Y - 140)/90, 0, 1))
    mcol *= (0.92 + 0.16*noise(30, 30, 3, 40))[..., None]; comp(img, mcol, mm)
    # ---- the far cliff and its plateau
    fm = mask([g['far']])
    rock(img, fm, LIP - 4, v, v['hz'], 100, 0.8)
    rim = np.clip(fm - np.roll(fm, 5, axis=0), 0, 1); comp(img, hexrgb(v['lightc']), rim*0.5*(1 - v['hz']))
    # mist lying over its foot
    comp(img, hexrgb(v['haze']), np.clip((Y - (H - 110))/110, 0, 1)*0.5)
    # ---- the water: the wet rock behind it, then the falling sheet
    wm = mask([g['water']])
    wet = blur(wm, 10); comp(img, hexrgb(v['shade'])*0.7, np.clip(wet*1.4, 0, 1)*0.5*(1 - wm))
    streak = noise(3.5, 160, 4, 200); streak2 = noise(8, 60, 3, 201)
    u = np.clip((Y - LIP)/(H - LIP), 0, 1)
    wcol = ramp([(0, v['water0']), (0.5, v['water1']), (1, '#ffffff')], 0.5 + streak*1.6 + streak2*0.6 + u*0.35)
    lx_ = np.clip((X - LX0 - (FX0 - LX0)*u)/((LX1 - LX0) + (FX1 - FX0 - LX1 + LX0)*u), 0, 1)
    gloss = np.exp(-((lx_ - v['gloss'])/0.1)**2)*(1 - u)*0.5
    wcol = wcol + gloss[..., None]
    edge = np.clip(1 - np.abs(lx_ - 0.5)*2, 0, 1)
    wa = wm*np.clip(edge*6 + streak*2, 0, 1)
    comp(img, wcol, wa)
    lip = np.exp(-((Y - LIP - 2)/3.5)**2)*wm; comp(img, hexrgb('#ffffff'), lip*0.85)
    # ---- a spire of rock standing free, in front of the far cliff (Ghost Ranch has its Chimney Rock)
    sp = layout.get('spire')
    if sp:
        x0, w, ytop = sp; spn = Noise(77, 3)
        L_ = [(x0 - w/2 - 14*(t**1.6) + 4*spn(t*5) - 3*math.sin(t*9), ytop + (H + 2 - ytop)*t) for t in [i/20 for i in range(21)]]
        R_ = [(x0 + w/2 + 18*(t**1.6) + 4*spn(t*5 + 3) + 2*math.sin(t*7), ytop + (H + 2 - ytop)*t) for t in [i/20 for i in range(21)]]
        capp = [(x0 - w/2 - 4, ytop + 6), (x0 - w*0.3, ytop - 2), (x0 + w*0.25, ytop - 3), (x0 + w/2 + 5, ytop + 5)]
        spm = mask([capp + R_ + L_[::-1]])
        rock(img, spm, ytop, v, 0.15, 700, 1.0, cap=ytop + 10)
    # ---- the near cliffs, and talus at their feet
    tal = []
    for pts, dsg in ((g['ledge'], 1), (g['redge'], -1)):
        a = [p for p in pts if p[1] > H - 90]
        sk = [(x + dsg*(6 + 60*((y - (H - 90))/90)**1.5), y) for x, y in a]
        tal.append(a + sk[::-1] + [(a[0][0], H + 2)])
    lm = mask([g['left']]); rm = mask([g['right']])
    rock(img, lm, min(y for x, y in g['lt']), v, 0, 300, 1.2)
    rock(img, rm, min(y for x, y in g['rt']), v, 0, 400, 1.2)
    for top, mm_ in ((g['lt'], lm), (g['rt'], rm)):
        rim = np.clip(mm_ - np.roll(mm_, 4, axis=0), 0, 1); comp(img, hexrgb(v['lightc']), rim*0.55)
    tm = mask(tal)
    rub = noise(5, 5, 3, 500)
    tcol = ramp([(0, v['bands'][-2][1]), (1, v['bands'][-1][1])], np.clip((Y - (H - 90))/90 + rub*0.6, 0, 1))*(0.8 + rub[..., None]*0.8)
    comp(img, tcol, tm)
    # ---- junipers on the tops, the plateau and the talus
    spots = []
    rr = random.Random(5)
    for x, base, h in g['trees']:
        for k in range(rr.randint(1, 3)): spots.append((x + rr.uniform(-12, 12), base - 3 + rr.uniform(-2, 2), h*rr.uniform(0.12, 0.2)))
    for x, y, hh in [g['far_trees'][i] for i in (1, 2, 4, 9, 11, 12)]: spots.append((x, y + 2, rr.uniform(3, 5)))
    for poly in tal:
        for k in range(3):
            x, y = poly[rr.randrange(1, len(poly)//2)]; spots.append((x + rr.uniform(4, 22)*(1 if x < 256 else -1), y, rr.uniform(4, 7)))
    juniper_layer(img, spots, v, 1 if v['L'][0] > 0 else -1)
    # ---- the pool: everything above it, mirrored, broken by ripples, darkened toward its color
    top = img[:int(px(H)), :].copy(); h0 = top.shape[0]; ph = N - h0
    rows = np.arange(ph); src = np.clip(h0 - 1 - rows, 0, h0 - 1)
    ref = top[src]
    shift = (np.sin(rows*0.31)*1.5 + np.sin(rows*0.09 + 1)*2)*(rows/ph*2 + 0.2)
    for i in range(ph): ref[i] = np.roll(ref[i], int(shift[i]), axis=0)
    pc = ramp(v['pool'], np.linspace(0, 1, ph)[:, None].repeat(N, 1))
    ref = blur(ref.reshape(-1, N, 3)[..., 0], 0)[..., None]*0 + ref
    ref = np.stack([blur(ref[..., c], 2.5) for c in range(3)], -1)*0.22 + pc*0.78
    img[h0:] = ref
    comp(img, hexrgb('#ffffff'), np.exp(-((Y - H)/1.2)**2)*0.5)
    # ripples where the water lands
    for k, (rx, o) in enumerate([(60, 0.5), (92, 0.35), (128, 0.2)]):
        e = np.abs(np.sqrt(((X - 256)/rx)**2 + ((Y - H - 6)/(rx*0.18))**2) - 1)
        comp(img, hexrgb('#ffffff'), np.clip(1 - e*rx*0.7, 0, 1)*o*(Y > H))
    bank = layout.get('bank')
    if bank:
        bn = Noise(91, 4)
        bp = [(-140, H - 6)] + [(x, H - 8 + 10*bn(x/40) + max(0, (x - 40))*0.16) for x in range(-130, 150, 10)] + [(170, H + 60), (150, 660), (-140, 660)]
        bm_ = mask([bp]); rub = noise(8, 8, 3, 800)
        comp(img, ramp([(0, v['bands'][4][1]), (1, v['bands'][-1][1])], np.clip((Y - H)/120 + rub*0.5, 0, 1))*(0.75 + rub[..., None]*0.6), bm_)
        for bx, by, br in [(70, H + 34, 26), (120, H + 54, 18), (20, H + 70, 34)]:
            bmask = np.clip((1 - np.sqrt(((X - bx)/(br*1.2))**2 + ((Y - by)/br)**2))*S*4, 0, 1)*(Y < by + br*0.35)
            gyb, gxb = np.gradient(blur(bmask, 6)*30 + noise(6, 6, 2, int(bx))*4)
            lb = np.clip(-gxb*np.sign(v['L'][0])*0.5 - gyb*0.5 + 0.45, 0, 1)
            comp(img, ramp([(0, v['shade']), (0.5, v['bands'][4][1]), (1, v['bands'][1][1])], lb), bmask)
        juniper_layer(img, [(46, H + 12, 30), (98, H + 26, 16), (-24, H + 30, 26)], v, 1 if v['L'][0] > 0 else -1)
    # ---- the mist at the foot, boiling up
    ms = np.zeros((N, N), np.float32)
    for dx, dy, r in [(-50, -8, 30), (48, -8, 30), (-20, -26, 38), (22, -24, 36), (0, -2, 56), (-80, 2, 24), (80, 2, 24), (0, -44, 30)]:
        ms += np.exp(-(((X - 256 - dx)/(r*1.4))**2 + ((Y - H - dy)/(r*0.7))**2))
    ms = np.clip(ms, 0, 1)*(0.7 + 0.5*noise(20, 14, 3, 600)); comp(img, hexrgb('#ffffff'), np.clip(ms, 0, 0.95))
    # ---- a soft vignette
    dv = np.sqrt((X - 256)**2 + (Y - 250)**2)/660; comp(img, hexrgb(v['vig']), np.clip((dv - 0.33)/0.4, 0, 1)*0.4)
    return np.clip(img, 0, 1)

BANDS = {
 'dusk': [(0, '#f8ead0'), (0.05, '#f2d6a8'), (0.33, '#ecc290'), (0.38, '#e0986a'), (0.5, '#cc6a44'), (0.62, '#bc5a40'), (0.78, '#a85a5c'), (1.2, '#8a4a5a')],
 'glow': [(0, '#ffeec0'), (0.05, '#ffd690'), (0.33, '#ffc070'), (0.38, '#f49050'), (0.5, '#dc5a32'), (0.62, '#c04a30'), (0.78, '#9a4048'), (1.2, '#7a3448')],
 'okeeffe': [(0, '#fdf2e2'), (0.1, '#f8dcc4'), (0.4, '#f0bca4'), (0.55, '#e49a88'), (0.75, '#d48484'), (1.2, '#c08090')],
}
V = {
 'dusk': dict(sky=[(0, '#14305a'), (0.35, '#2f7c9c'), (0.62, '#a8c0b4'), (0.78, '#f2bc98'), (1, '#f8d8b8')], bands=BANDS['dusk'],
              haze='#d8bcbc', hz=0.5, L=(0.55, -0.45, 0.7), lightc='#fff2e0', amb=0.42, dif=0.8, shade='#3a1424', mesa='#7a78aa',
              pool=[(0, '#7ad0cc'), (0.4, '#2a8a9a'), (1, '#0f3a4a')], water0='#cfe8ea', water1='#e8f6f4', water_dark='#5a9aa8', gloss=0.72, obj='moon', tier=292,
              jun0='#1e3026', jun1='#3e5a44', jun2='#8aa47a', vig='#1a0a18'),
 'glow': dict(sky=[(0, '#1e1a4a'), (0.35, '#5a4a8a'), (0.6, '#c87a8a'), (0.76, '#f6a86a'), (1, '#ffd090')], bands=BANDS['glow'],
              haze='#e8a890', hz=0.45, L=(-0.75, -0.25, 0.5), lightc='#ffd8a0', amb=0.45, dif=0.85, shade='#3a0c18', mesa='#5a4a80',
              pool=[(0, '#f0a878'), (0.4, '#7a5a8a'), (1, '#1e1a40')], water0='#f4dcc8', water1='#fff0e0', water_dark='#b07a7a', gloss=0.25, obj='star',
              jun0='#1c2a1c', jun1='#3a4a2a', jun2='#a8a050', vig='#1a0812', wisps=60, wispc='#ffb890'),
 'okeeffe': dict(sky=[(0, '#7ab0d0'), (0.55, '#b8d4dc'), (1, '#f8e4d0')], bands=BANDS['okeeffe'],
              haze='#f0d4c8', hz=0.55, L=(0.4, -0.5, 0.8), lightc='#ffffff', amb=0.7, dif=0.4, shade='#7a3a48', mesa='#9aa0c8',
              pool=[(0, '#b0e0dc'), (0.4, '#6ab0b8'), (1, '#3a7a8a')], water0='#e4f4f4', water1='#f4fbfb', water_dark='#7ab0b8', gloss=0.72, obj='moonw',
              jun0='#4a6a58', jun1='#6a8a76', jun2='#b4c8a8', vig='#6a4a5a', ledge=60.0, spec=0.3),
}
LAYOUT = {'dusk': dict(bank=1), 'glow': dict(bank=1), 'okeeffe': dict(bank=1)}
if __name__ == '__main__':
    name, out = sys.argv[1], sys.argv[2]
    a = build(V[name], LAYOUT[name]); Image.fromarray((a*255).astype(np.uint8)).save(out); print(out)
