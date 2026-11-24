import math, random, sys
STYLE = sys.argv[1] if len(sys.argv) > 1 else 'harvest'
OUT = sys.argv[2] if len(sys.argv) > 2 else 'h1.svg'
REFL = sys.argv[3] if len(sys.argv) > 3 else 'slivers'
R = random.Random(7)
def f(x): return f"{x:.1f}".rstrip('0').rstrip('.')
CROP = "64 136 400 400"

# ---- the moon ----
MX, MY, MR = 160, 272, 70
def blob(cx, cy, rx, ry, n=9, jitter=0.18, rot=0):
    pts=[]
    for i in range(n):
        a = 2*math.pi*i/n
        r = 1 + R.uniform(-jitter, jitter)
        x, y = rx*r*math.cos(a), ry*r*math.sin(a)
        c, s = math.cos(rot), math.sin(rot)
        pts.append((cx + x*c - y*s, cy + x*s + y*c))
    # a smooth closed curve through the points (Catmull-Rom as cubic Beziers)
    d=f"M{f(pts[0][0])},{f(pts[0][1])}"
    for i in range(n):
        p0,p1,p2,p3 = pts[i-1], pts[i], pts[(i+1)%n], pts[(i+2)%n]
        c1=(p1[0]+(p2[0]-p0[0])/6, p1[1]+(p2[1]-p0[1])/6)
        c2=(p2[0]-(p3[0]-p1[0])/6, p2[1]-(p3[1]-p1[1])/6)
        d+=f" C{f(c1[0])},{f(c1[1])} {f(c2[0])},{f(c2[1])} {f(p2[0])},{f(p2[1])}"
    return d+" Z"
# seas, placed as on the real moon's near side (in units of the radius)
# the near side's seas, as seen from the north up (center x, y, radii, in moon radii)
seas = [(-0.32,-0.40,0.27,0.24,0.2),   # Imbrium
        (-0.58,-0.02,0.22,0.42,0.15),  # Procellarum
        (0.16,-0.36,0.16,0.15,0),      # Serenitatis
        (0.32,-0.08,0.20,0.17,-0.4),   # Tranquillitatis
        (0.68,-0.24,0.10,0.09,0),      # Crisium
        (0.50,0.16,0.10,0.17,0.3),     # Fecunditatis
        (-0.20,0.36,0.17,0.12,0.2),    # Nubium
        (-0.46,0.40,0.10,0.09,0),      # Humorum
        (-0.04,-0.14,0.10,0.08,0),     # Vaporum
        (0.10,-0.62,0.20,0.06,0.1)]    # Frigoris
def crescent(cx, cy, r, ox, oy, r2):
    # the outer circle less a circle offset by (ox, oy): two arcs between their crossings
    d = math.hypot(ox, oy); a = (r*r - r2*r2 + d*d) / (2*d); h = math.sqrt(r*r - a*a)
    ux, uy = ox/d, oy/d
    px, py = cx + a*ux, cy + a*uy
    p1 = (px - h*uy, py + h*ux); p2 = (px + h*uy, py - h*ux)
    return f"M{f(p1[0])},{f(p1[1])} A{r},{r} 0 1 1 {f(p2[0])},{f(p2[1])} A{r2},{r2} 0 0 0 {f(p1[0])},{f(p1[1])} Z"
MOONSHAPE = crescent(MX, MY, MR, 34, -22, 62) if STYLE == 'crescent' else None
moon = [f'<circle cx="{MX}" cy="{MY}" r="150" fill="url(#halo)"/>']
if MOONSHAPE:
    moon.append(f'<circle cx="{MX}" cy="{MY}" r="{MR}" fill="#8a6088" opacity="0.22"/>')
    moon.append(f'<path d="{MOONSHAPE}" fill="url(#moon)"/>')
else:
    moon.append(f'<circle cx="{MX}" cy="{MY}" r="{MR}" fill="url(#moon)"/>')
moon += [f'<g fill="#c08a5c" clip-path="url(#moonclip)">']
for dx,dy,rx,ry,rot in seas:
    moon.append(f'<path d="{blob(MX+dx*MR, MY+dy*MR, rx*MR*1.45, ry*MR*1.45, n=12, jitter=0.14, rot=rot)}" opacity="0.09"/>')
    moon.append(f'<path d="{blob(MX+dx*MR, MY+dy*MR, rx*MR*1.2, ry*MR*1.2, n=12, jitter=0.14, rot=rot)}" opacity="0.1"/>')
moon.append('</g>')
if STYLE != 'crescent':
    tx, ty = MX-0.12*MR, MY+0.66*MR
    rays=[]
    for a in [-150,-110,-75,-40,-10,25,60,100,140]:
        L = R.uniform(14,30); r=math.radians(a)
        rays.append(f'<path d="M{f(tx+4*math.cos(r))} {f(ty+4*math.sin(r))} L{f(tx+L*math.cos(r))} {f(ty+L*math.sin(r))}"/>')
    moon.append(f'<g stroke="#fff4dc" stroke-width="1.1" stroke-linecap="round" opacity="0.13" clip-path="url(#moonclip)">{"".join(rays)}</g>')
    moon.append(f'<circle cx="{f(tx)}" cy="{f(ty)}" r="3.2" fill="#fff6e0" opacity="0.85"/>')
    moon.append(f'<circle cx="{f(MX-0.30*MR)}" cy="{f(MY-0.06*MR)}" r="2.2" fill="#fff2d8" opacity="0.6"/>')
    moon.append(f'<circle cx="{f(MX+0.42*MR)}" cy="{f(MY+0.46*MR)}" r="3" fill="none" stroke="#d8a46c" stroke-width="1.4" opacity="0.4"/>')
def pine(x, base, h, seed):
    r = random.Random(seed); w = h*0.44; n = r.choice([4,5,5,6])
    top = base - h; left=[]; right=[]
    for i in range(n):
        t = (i+1)/n; y = top + (h*0.92)*t
        reach = w/2*(0.35+0.65*t)
        left += [(x - reach*r.uniform(0.85,1.1), y), (x - reach*0.45, y - h*0.05)]
        right += [(x + reach*r.uniform(0.85,1.1), y + r.uniform(-2,2)), (x + reach*0.45, y - h*0.05)]
    pts=[(x, top)] + right[:-1] + [(x+2, base), (x-2, base)] + list(reversed(left[:-1]))
    return "M" + " L".join(f"{f(a)},{f(b)}" for a,b in pts) + " Z"
TREES_FAR = ''.join(f'<path d="{pine(x, 189, h, i)}"/>' for i,(x,h) in enumerate([(424,30),(458,44),(500,36),(540,48),(566,34)]))
TREES_NEAR = ''.join(f'<path d="{pine(x, 188, h, 40+i)}"/>' for i,(x,h) in enumerate([(404,30),(426,44),(449,34)]))
stars=[]
for x,y,r,o in [(84,160,2.2,.9),(132,188,1.4,.6),(214,150,1.8,.8),(262,182,1.2,.55),(300,146,2,.85),(352,170,1.4,.6),(410,150,1.6,.7),(446,196,1.2,.5),(100,214,1.1,.45),(236,226,1.3,.5),(388,224,1.1,.4)]:
    stars.append(f'<circle cx="{x}" cy="{y}" r="{r}" fill="#fff4dc" opacity="{o}"/>')
stars.append('<path d="M300 166 V186 M290 176 H310" stroke="#fff4dc" stroke-width="1.2" opacity="0.5" stroke-linecap="round"/>')

# ---- the mountains: a far range with a real ridge, a near hill with trees ----
def ridge(points):
    d=f"M{f(points[0][0])},{f(points[0][1])}"
    for (x,y) in points[1:]: d+=f" L{f(x)},{f(y)}"
    return d
far_pts=[(30,404),(30,378),(44,374),(52,376),(62,369),(70,371),(80,363),(88,366),(96,361),(104,364),(112,356),(118,357),(126,352),
         (134,356),(142,354),(152,360),(164,357),(176,363),(188,360),(198,364),(208,358),(218,350),(226,346),(232,339),(238,336),
         (244,330),(250,331),(256,322),(260,318),(264,320),(270,326),(276,325),(284,334),(292,340),(300,344),(312,348),(312,404)]
far_pts = [(x-24, y) for x, y in far_pts]
far = ridge(far_pts)+" Z"
# moonlit faces: the slopes that face the moon (left), a shade lighter
lit = [ridge([(112,356),(118,357),(126,352),(122,366),(114,370),(106,368)])+" Z",
       ridge([(238,336),(244,330),(250,331),(256,322),(260,318),(254,334),(246,346),(236,352),(228,350)])+" Z",
       ridge([(80,363),(88,366),(96,361),(92,372),(84,374)])+" Z"]
# gullies: thin darker lines down the big peak
gullies = ['M260 320 Q256 334 250 352', 'M270 327 Q268 340 266 356', 'M244 332 Q240 344 234 358']
# the near hill: a rolling line with treetops on it
near=[(30,404)]
x=30
def hill(x): return 394 - 9*math.sin((x-30)/52 + 0.6) - 4*math.sin((x-30)/19)
near.append((30, hill(30)))
nearp_d = f"M30,404 L30,{f(hill(30))}"
while x < 312:
    b = hill(x); w = R.uniform(7,13); h = R.uniform(2.5,6.5)
    b2 = hill(x+w)
    if R.random() < 0.1:
        nearp_d += f" L{f(x+w*0.5)},{f(min(b,b2)-h-8)} L{f(x+w)},{f(b2)}"
    else:
        nearp_d += f" C{f(x+w*0.1)},{f(b-h*1.3)} {f(x+w*0.9)},{f(b2-h*1.3)} {f(x+w)},{f(b2)}"
    x += w
nearp_d += " L312,404 Z"
nearp = nearp_d

# ---- the water: still, a few ripples ----
def mirror(points, horizon=404, squash=0.85):
    return ridge([(x, horizon + (horizon-y)*squash) for x,y in points])+" Z"
# the moon's light on still water: an uneven column of slivers, fading
slivers=[]
y=409
while y < 532:
    t=(y-409)/120
    half = (54*(1-t*0.6)) * R.uniform(0.25,1.0) * (0.6 if STYLE == 'crescent' else 1)
    off = R.uniform(-9,9)*(1-t*0.5) - (22 if STYLE == 'crescent' else 0)
    thick = R.uniform(2.0,3.6)*(1+t*0.5)
    o = 0.85*max(0,1-t)**1.6
    # some lines break in two
    if R.random()<0.3:
        gap=R.uniform(4,10)
        cut=R.uniform(-half*0.4,half*0.4)
        slivers.append((MX+off-half, MX+off+cut-gap/2, y, thick, o))
        slivers.append((MX+off+cut+gap/2, MX+off+half, y, thick, o))
    else:
        slivers.append((MX+off-half, MX+off+half, y, thick, o))
    y += R.uniform(3.5,6) + t*12
mrefl=['<g stroke-linecap="round">']
for x1,x2,y,t,o in slivers:
    if o < 0.04: continue
    mrefl.append(f'<path d="M{f(x1)} {f(y)} H{f(x2)}" stroke="#ffe0a4" stroke-width="{f(t)}" opacity="{o:.2f}"/>')
mrefl.append('</g>')
# the falls in the water: a short broken pale column under its foot
frefl=['<g stroke="#ffe8c8" stroke-linecap="round">']
y=414
while y < 480:
    t=(y-414)/66
    half=30*R.uniform(0.5,1.0)*(1-t*0.4); off=R.uniform(-4,4)
    frefl.append(f'<path d="M{f(290+off-half)} {f(y)} H{f(290+off+half)}" stroke-width="{f(R.uniform(2,3.4))}" opacity="{0.45*(1-t):.2f}"/>')
    y += R.uniform(4,6) + t*10
frefl.append('</g>')
# ripples: rings flattened by distance, around the foot only
rip=['<g fill="none" stroke-linecap="round" clip-path="url(#lakeclip)">']
for rx,ry,w,o,col in [(40,6,3.5,.9,'#fff0d4'),(62,9.5,3,.6,'#f4c898'),(88,13.5,2.4,.4,'#e8b088'),(118,18,1.8,.25,'#d89a80')]:
    rip.append(f'<ellipse cx="290" cy="405" rx="{rx}" ry="{ry}" stroke="{col}" stroke-width="{w}" opacity="{o}"/>')
rip.append('</g>')
EXTRA_DEFS=[]
calm=['<g stroke="#8a5a80" stroke-width="1.5" stroke-linecap="round" opacity="0.35">',
      '<path d="M70 418 H120"/><path d="M200 426 H236"/><path d="M84 446 H110"/><path d="M372 440 H430"/><path d="M220 470 H262"/><path d="M90 490 H128"/><path d="M360 486 H410"/>','</g>']


# ---- the reflections, one of five ways ----
def lake_at(y):
    t = max(0, min(1, (y-404)/132)); a=(0x4a,0x2a,0x58); b=(0x1c,0x0e,0x2c)
    return '#%02x%02x%02x' % tuple(round(a[i]+(b[i]-a[i])*t) for i in range(3))
def lens(x1, x2, y, t):
    xm=(x1+x2)/2
    return f"M{f(x1)},{f(y)} Q{f(xm)},{f(y-t)} {f(x2)},{f(y)} Q{f(xm)},{f(y+t)} {f(x1)},{f(y)} Z"
MOONDISC = ''.join(moon[1:]) if True else ''
DROPD = "M392 189.8 L384 190 C330 192 292 214 278 262 C268 300 262 350 256 404 H324 Q318 340 320 274 C322 236 334 208 356 206 H392 Z"
FALLD = "M576 187 L512 188 L384 190 C330 192 292 214 278 262 C268 300 262 350 256 404 H324 Q318 340 320 274 C322 236 334 208 356 206 H576 Z"
def mirrored(content, squash, opacity):
    return f'<g clip-path="url(#lakeclip)" opacity="{opacity}"><g transform="translate(0 404) scale(1 {-squash}) translate(0 -404)">{content}</g></g>'
RM=[]; RF=[]
glowM = f'<g transform="translate({MX} 450) scale(0.5 1) translate(-{MX} -450)"><circle cx="{MX}" cy="450" r="90" fill="url(#moonglow)"/></g>'
if REFL == 'slivers':
    RM=[glowM]+mrefl; RF=frefl
elif REFL == 'mirror':
    RM=[glowM, mirrored(f'<circle cx="{MX}" cy="{MY}" r="{MR*1.12:.0f}" fill="url(#mfade)"/>', 0.62, 1)]
    RF=[mirrored(f'<path d="{DROPD}" fill="url(#ffade)"/>', 0.62, 1)]
    lines=[]
    for y in [413, 419, 426, 434, 443, 453, 464, 477, 492]:
        w = 1 + (y-404)/70
        for x1,x2 in [(MX-80, MX+80), (246, 340)]:
            lines.append(f'<path d="M{f(x1+R.uniform(-8,8))} {y} H{f(x2+R.uniform(-8,8))}" stroke="{lake_at(y)}" stroke-width="{w:.1f}" stroke-linecap="round"/>')
    RF.append('<g>'+''.join(lines)+'</g>')
elif REFL == 'glow':
    RM=[f'<g transform="translate({MX} 456) scale(0.42 1.25) translate(-{MX} -456)"><circle cx="{MX}" cy="456" r="80" fill="url(#moonglow2)"/></g>']
    for x1,x2,y,t,o in [(118,204,409,1.6,.8),(134,186,414,1.3,.6),(126,170,420,1.1,.45)]:
        RM.append(f'<path d="{lens(x1,x2,y,t)}" fill="#ffe8b8" opacity="{o}"/>')
    RF=[f'<g transform="translate(290 440) scale(0.4 1) translate(-290 -440)"><circle cx="290" cy="440" r="60" fill="url(#fallglow)"/></g>']
elif REFL == 'glints':
    RM=[glowM]
    y=410
    while y < 520:
        t=(y-410)/110
        n = 1 if R.random() < 0.6 else 2
        for _ in range(n):
            half = R.uniform(8, 34)*(1-t*0.55); cx = MX + R.uniform(-28,28)*(1-t*0.4)
            RM.append(f'<path d="{lens(cx-half, cx+half, y, R.uniform(1.2,2.2)*(1+t*0.4))}" fill="#ffe2a8" opacity="{0.9*max(0,1-t)**1.4:.2f}"/>')
        y += R.uniform(4,7) + t*14
    y=414
    while y < 476:
        t=(y-414)/62; half=R.uniform(8,22)*(1-t*0.4); cx=290+R.uniform(-8,8)
        RF.append(f'<path d="{lens(cx-half,cx+half,y,R.uniform(1,1.8))}" fill="#ffecd0" opacity="{0.55*(1-t):.2f}"/>')
        y += R.uniform(5,8)+t*10
elif REFL == 'broken':
    bands=[]; y=406
    while y < 520:
        h = R.uniform(2.5,4.5) + (y-406)/18
        dx = R.uniform(-3.5,3.5)*(1+(y-406)/60)
        bands.append((y, h, dx)); y += h + R.uniform(1.2,2.2) + (y-406)/45
    clips=[]; parts=[]
    for i,(y,h,dx) in enumerate(bands):
        clips.append(f'<clipPath id="band{i}"><rect x="0" y="{f(y)}" width="640" height="{f(h)}"/></clipPath>')
        parts.append(f'<g clip-path="url(#band{i})"><g transform="translate({f(dx)} 0)">{mirrored(f"""<circle cx="{MX}" cy="{MY}" r="{MR*1.12:.0f}" fill="url(#mfade)"/>""", 0.62, 1)}</g></g>')
        parts.append(f'<g clip-path="url(#band{i})"><g transform="translate({f(dx*0.7)} 0)">{mirrored(f"""<path d="{DROPD}" fill="url(#ffade)"/>""", 0.62, 1)}</g></g>')
    EXTRA_DEFS.extend(clips)
    RM=[glowM]+parts
elif REFL == 'waves':
    def wave(x1, x2, y, amp, wl):
        d=f"M{f(x1)},{f(y)}"; x=x1; up=True
        while x < x2:
            nx=min(x+wl/2, x2); d+=f" Q{f((x+nx)/2)},{f(y-amp if up else y+amp)} {f(nx)},{f(y)}"; x=nx; up=not up
        return d
    RM=[glowM, '<g fill="none" stroke-linecap="round">']
    y=410; i=0
    while y < 524:
        t=(y-410)/114
        half=(56*(1-t*0.5))*R.uniform(0.7,1.0); cx=MX+R.uniform(-4,4)
        col = ['#ffe6b0','#f8cc88','#eab07a'][i%3]
        RM.append(f'<path d="{wave(cx-half,cx+half,y,1.6+t,12+t*8)}" stroke="{col}" stroke-width="{f(2.2+t*0.8)}" opacity="{0.85*max(0,1-t)**1.3:.2f}"/>')
        y += 6 + t*9; i+=1
    RM.append('</g>')
    RF=['<g fill="none" stroke="#ffecd0" stroke-linecap="round">']
    y=414
    while y < 474:
        t=(y-414)/60; half=26*(1-t*0.4)
        RF.append(f'<path d="{wave(290-half,290+half,y,1.4,10)}" stroke-width="2" opacity="{0.5*(1-t):.2f}"/>')
        y += 7+t*6
    RF.append('</g>')

def bez(p0,p1,p2,p3,t):
    u=1-t; return (u**3*p0[0]+3*u*u*t*p1[0]+3*u*t*t*p2[0]+t**3*p3[0], u**3*p0[1]+3*u*u*t*p1[1]+3*u*t*t*p2[1]+t**3*p3[1])
def quad(p0,p1,p2,t):
    u=1-t; return (u*u*p0[0]+2*u*t*p1[0]+t*t*p2[0], u*u*p0[1]+2*u*t*p1[1]+t*t*p2[1])
def outer(s):
    if s < 0.35: t=s/0.35; return (576+(384-576)*t, 187+3*t)
    if s < 0.6: return bez((384,190),(330,192),(292,214),(278,262),(s-0.35)/0.25)
    return bez((278,262),(268,300),(262,350),(256,404),(s-0.6)/0.4)
def inner(s):
    if s < 0.35: t=s/0.35; return (576+(356-576)*t, 206)
    if s < 0.6: return bez((356,206),(334,208),(322,236),(320,274),(s-0.35)/0.25)
    return quad((320,274),(318,340),(324,404),(s-0.6)/0.4)
def flow(u, a, b, n=28):
    pts=[]
    for i in range(n+1):
        s=a+(b-a)*i/n; o=outer(s); q=inner(s); pts.append((o[0]+(q[0]-o[0])*u, o[1]+(q[1]-o[1])*u))
    return "M"+" L".join(f"{f(x)},{f(y)}" for x,y in pts)
FR = random.Random(11)
streaks=[]
# the glassy part: long, faint, smooth
for u,a,b,c,w,o in [(0.07,0.28,0.62,'#ffffff',5,0.95),(0.3,0.1,0.66,'#ffe8c4',2.5,0.55),(0.55,0.2,0.62,'#ffe8c4',2,0.45),(0.8,0.3,0.6,'#e8a070',3,0.35)]:
    streaks.append(f'<path d="{flow(u,a,b)}" stroke="{c}" stroke-width="{w}" opacity="{o}"/>')
# the falling part: many short streaks, white and peach, longer lower down
for i in range(26):
    u=FR.uniform(0.05,0.95); a=FR.uniform(0.56,0.86); b=min(1.0, a+FR.uniform(0.08,0.3))
    if FR.random()<0.62: c,w,o='#fffaf2',FR.uniform(2,4.5),FR.uniform(0.6,1)
    else: c,w,o='#e6a072',FR.uniform(1.5,3),FR.uniform(0.3,0.5)
    streaks.append(f'<path d="{flow(u,a,b,12)}" stroke="{c}" stroke-width="{f(w)}" opacity="{o:.2f}"/>')
FALL_STREAKS=''.join(streaks)
# spray: drops off the edges near the foot
drops=[]
for i in range(7):
    s_=FR.uniform(0.88,0.99); side=FR.choice([0,1]); o=outer(s_); q=inner(s_)
    x = (o[0]-FR.uniform(2,6)) if side==0 else (q[0]+FR.uniform(2,6)); y=o[1]
    L=FR.uniform(3,5.5)
    drops.append(f'<path d="M{f(x)},{f(y-L)} Q{f(x+1.2)},{f(y)} {f(x)},{f(y+L*0.3)} Q{f(x-1.2)},{f(y)} {f(x)},{f(y-L)} Z" opacity="{FR.uniform(0.45,0.85):.2f}"/>')
FALL_SPRAY=''.join(drops)

def fall_glints(rng, strength=0.55):
    out=[]; y=414
    while y < 478:
        t=(y-414)/64; half=rng.uniform(7,20)*(1-t*0.4); cx=290+rng.uniform(-9,9)
        out.append(f'<path d="{lens(cx-half,cx+half,y,rng.uniform(1,1.7))}" fill="#ffecd0" opacity="{strength*max(0,1-t):.2f}"/>')
        y += rng.uniform(5,8)+t*10
    return out
def sparkle(x, y, r, o):
    return f'<path d="M{f(x)},{f(y-r)} Q{f(x+r*0.18)},{f(y-r*0.18)} {f(x+r)},{f(y)} Q{f(x+r*0.18)},{f(y+r*0.18)} {f(x)},{f(y+r)} Q{f(x-r*0.18)},{f(y+r*0.18)} {f(x-r)},{f(y)} Q{f(x-r*0.18)},{f(y-r*0.18)} {f(x)},{f(y-r)} Z" fill="#fff8e8" opacity="{o:.2f}"/>'
GR = random.Random(5)
if REFL.startswith('v'):
    RM=[]; RF=fall_glints(GR)
    soft = f'<g transform="translate({MX} 456) scale(0.42 1.25) translate(-{MX} -456)"><circle cx="{MX}" cy="456" r="80" fill="url(#moonglow2)"/></g>'
    glints=[]
    if REFL == 'v1':   # dense shimmer
        y=409
        while y < 520:
            t=(y-409)/111
            for _ in range(GR.choice([1,2,2,3])):
                half=GR.uniform(4,16)*(1-t*0.5); cx=MX+GR.gauss(0,16)*(1-t*0.3)
                glints.append(f'<path d="{lens(cx-half,cx+half,y+GR.uniform(-1,1),GR.uniform(0.9,1.6))}" fill="#ffe2a8" opacity="{0.95*max(0,1-t)**1.5:.2f}"/>')
            y += GR.uniform(2.5,4.5) + t*9
        RM=[glowM]+glints
    elif REFL == 'v2':   # glitter path, widening toward the viewer
        y=409
        while y < 532:
            t=(y-409)/123; width=10+60*t
            for _ in range(1 if t < 0.3 else GR.choice([1,2])):
                half=GR.uniform(5,12)*(1+t*1.6); cx=MX+GR.uniform(-width/2,width/2)
                glints.append(f'<path d="{lens(cx-half,cx+half,y,GR.uniform(1.1,1.8)*(1+t*0.8))}" fill="#ffe2a8" opacity="{0.9*max(0,1-t)**1.2:.2f}"/>')
            y += GR.uniform(4,6) + t*14
        RM=[glowM]+glints
    elif REFL in ('v3','v4','v5'):
        y=410
        while y < 520:
            t=(y-410)/110
            for _ in range(1 if GR.random() < 0.6 else 2):
                half=GR.uniform(8,34)*(1-t*0.55); cx=MX+GR.uniform(-28,28)*(1-t*0.4)
                col = '#d8c0e8' if (REFL=='v5' and GR.random()<0.25) else '#ffe2a8'
                o = 0.9*max(0,1-t)**1.4
                glints.append(f'<path d="{lens(cx-half,cx+half,y,GR.uniform(1.2,2.2)*(1+t*0.4))}" fill="{col}" opacity="{o:.2f}"/>')
                if REFL=='v5' and o > 0.6 and GR.random() < 0.35:
                    glints.append(sparkle(cx+GR.uniform(-half*0.5,half*0.5), y, GR.uniform(2.8,4.2), o*0.9))
            y += GR.uniform(4,7) + t*14
        if REFL == 'v3': RM=[soft]+glints
        elif REFL == 'v4':
            RM=[glowM, mirrored(f'<circle cx="{MX}" cy="{MY}" r="{MR*1.12:.0f}" fill="url(#mfade)"/>', 0.62, 0.32)]+glints
        else: RM=[glowM]+glints

# ======== round 9: the finishing pass ========
# ---- the moon, from its map: the seas by latitude and longitude, projected ----
NR = random.Random(42)
def proj(lat, lon):
    la, lo = math.radians(lat), math.radians(lon)
    return (MX + MR*math.cos(la)*math.sin(lo), MY - MR*math.sin(la))
def noise(seed, terms=4):
    r = random.Random(seed)
    ph = [(r.uniform(0, 2*math.pi), r.uniform(0.06, 0.16)/(k+1)**0.8, k+2) for k in range(terms)]
    return lambda t: sum(a*math.sin(k*t + p) for p, a, k in ph)
def sea(lat, lon, r_lat, r_lon=None, seed=0, n=56, rough=1.0, lobes=()):
    r_lon = r_lon or r_lat; nz = noise(seed); pts = []
    jr = random.Random(seed + 99)
    for i in range(n):
        t = 2*math.pi*i/n
        k = 1 + rough*nz(t) + rough*jr.uniform(-0.025, 0.025)
        for (lt, amt, width) in lobes:   # a bay or a bulge at angle lt
            d = math.atan2(math.sin(t-lt), math.cos(t-lt)); k += amt*math.exp(-(d/width)**2)
        la = lat + r_lat*k*math.sin(t)
        lo = lon + r_lon*k*math.cos(t)/max(0.3, math.cos(math.radians(la)))
        pts.append(proj(la, lo))
    return pts
def poly(points_ll, seed=0, n_sub=3):
    # a lat/lon outline, its edges broken up a little, projected
    r = random.Random(seed); pts = [proj(*p) for p in points_ll]
    for _ in range(n_sub):
        out = []
        for i in range(len(pts)):
            a, b = pts[i], pts[(i+1) % len(pts)]
            mx, my = (a[0]+b[0])/2, (a[1]+b[1])/2; L = math.hypot(b[0]-a[0], b[1]-a[1])
            nx, ny = -(b[1]-a[1])/(L or 1), (b[0]-a[0])/(L or 1); d = r.uniform(-0.12, 0.12)*L
            out += [a, (mx+nx*d, my+ny*d)]
        pts = out
    return pts
def smooth_closed(pts):
    n = len(pts); d = f"M{f(pts[0][0])},{f(pts[0][1])}"
    for i in range(n):
        p0, p1, p2, p3 = pts[i-1], pts[i], pts[(i+1) % n], pts[(i+2) % n]
        c1 = (p1[0]+(p2[0]-p0[0])/6, p1[1]+(p2[1]-p0[1])/6); c2 = (p2[0]-(p3[0]-p1[0])/6, p2[1]-(p3[1]-p1[1])/6)
        d += f" C{f(c1[0])},{f(c1[1])} {f(c2[0])},{f(c2[1])} {f(p2[0])},{f(p2[1])}"
    return d + " Z"
def ccw(pts):
    a = sum(pts[i][0]*pts[(i+1) % len(pts)][1] - pts[(i+1) % len(pts)][0]*pts[i][1] for i in range(len(pts)))
    return pts if a > 0 else list(reversed(pts))
MARIA = [
    sea(33, -16, 17, seed=1, lobes=((2.4, 0.18, 0.35), (0.3, -0.12, 0.4))),          # Imbrium, Sinus Iridum to the NW
    poly([(42,-52),(34,-62),(22,-74),(6,-80),(-8,-72),(-18,-58),(-12,-44),(-4,-36),(4,-26),(12,-24),(18,-34),(26,-40),(30,-34),(38,-28),(44,-36)], seed=2),  # Procellarum
    sea(28, 17.5, 9.5, seed=3),                                                        # Serenitatis
    sea(8.5, 31, 11, 13, seed=4, rough=1.4, lobes=((3.6, 0.25, 0.4), (5.6, 0.2, 0.35))), # Tranquillitatis
    sea(17, 59, 7.5, 9, seed=5, rough=0.6),                                            # Crisium
    sea(-6, 51, 11, 6.5, seed=6, rough=1.3),                                           # Fecunditatis
    sea(-15, 35, 5, seed=7),                                                           # Nectaris
    sea(-20, -17, 9, 11, seed=8, rough=1.4),                                           # Nubium
    sea(-24, -39, 5.5, seed=9),                                                        # Humorum
    sea(-10, -23, 5, seed=10, rough=1.5),                                              # Cognitum
    sea(8, -30, 6, seed=11, rough=1.5),                                                # Insularum
    sea(13, 4, 4.5, seed=12, rough=1.5),                                               # Vaporum
    sea(2, 1, 3.5, seed=13),                                                           # Sinus Medii
    poly([(51,-42),(55,-20),(56,0),(55,20),(53,38),(58,40),(61,22),(61,0),(60,-22),(57,-44)], seed=14, n_sub=3),  # Frigoris
]
SEAS_D = ' '.join(smooth_closed(ccw(p)) for p in MARIA)
def grow(pts, by):
    cx = sum(p[0] for p in pts)/len(pts); cy = sum(p[1] for p in pts)/len(pts)
    return [(cx+(x-cx)*(1+by/ max(4, math.hypot(x-cx, y-cy))), cy+(y-cy)*(1+by/ max(4, math.hypot(x-cx, y-cy)))) for x, y in pts]
SEAS_SOFT_D = ' '.join(smooth_closed(ccw(grow(p, 2.2))) for p in MARIA)
CORES_D = ' '.join(smooth_closed(ccw(p)) for p in [sea(34,-17,9,seed=21,rough=1.4), sea(28,18,5,seed=22), sea(17,59,4.5,6,seed=23), sea(10,28,6,seed=24,rough=1.6), sea(-24,-39,3,seed=25), sea(30,-62,10,8,seed=26,rough=1.8)])
def rays(lat, lon, n, lmin, lmax, w, seed):
    r = random.Random(seed); cx, cy = proj(lat, lon); out = []
    for i in range(n):
        a = 2*math.pi*i/n + r.uniform(-0.25, 0.25); L = r.uniform(lmin, lmax); x2, y2 = cx+L*math.cos(a), cy+L*math.sin(a)
        nx, ny = -math.sin(a)*w, math.cos(a)*w; mx, my = cx+L*0.35*math.cos(a), cy+L*0.35*math.sin(a)
        out.append(f"M{f(cx)},{f(cy)} Q{f(mx+nx)},{f(my+ny)} {f(x2)},{f(y2)} Q{f(mx-nx)},{f(my-ny)} {f(cx)},{f(cy)} Z")
    return ' '.join(out)
RAYS_D = rays(-43, -11, 22, 18, 52, 0.8, 31) + ' ' + rays(9.6, -20, 14, 8, 20, 0.6, 32) + ' ' + rays(8, -38, 10, 5, 12, 0.5, 33)
craters = []
cr = random.Random(77)
for i in range(22):
    lat = cr.uniform(-75, 70); lon = cr.uniform(-80, 80)
    if math.cos(math.radians(lat))*math.cos(math.radians(lon)) < 0.25: continue
    x, y = proj(lat, lon); s = cr.uniform(0.9, 2.6)*math.sqrt(max(0.2, math.cos(math.radians(lat))*math.cos(math.radians(lon))))
    craters.append((x, y, s))
CRATER_DARK = ' '.join(f"M{f(x-s)},{f(y+s*0.25)} A{f(s)},{f(s)} 0 1 0 {f(x+s)},{f(y+s*0.25)} A{f(s)},{f(s*0.7)} 0 0 1 {f(x-s)},{f(y+s*0.25)} Z" for x, y, s in craters)
CRATER_LIT = ' '.join(f"M{f(x-s)},{f(y)} A{f(s)},{f(s)} 0 0 1 {f(x+s)},{f(y)} A{f(s)},{f(s*0.6)} 0 0 0 {f(x-s)},{f(y)} Z" for x, y, s in craters)
bright = [proj(-43,-11), proj(9.6,-20), proj(8,-38), proj(24,-47.5)]
moon = [f'<circle cx="{MX}" cy="{MY}" r="150" fill="url(#halo)"/>',
        f'<circle cx="{MX}" cy="{MY}" r="{MR}" fill="url(#moon)"/>',
        f'<g clip-path="url(#moonclip)">',
        f'<path d="{SEAS_D}" fill="#b07c54" opacity="0.2"/>',
        f'<path d="{CORES_D}" fill="#9c6a48" opacity="0.12"/>',
        f'<path d="{RAYS_D}" fill="#fff6e2" opacity="0.07"/>',
        f'<path d="{CRATER_DARK}" fill="#a87450" opacity="0.13"/>',
        f'<path d="{CRATER_LIT}" fill="#fff4dc" opacity="0.16"/>',
        ''.join(f'<circle cx="{f(x)}" cy="{f(y)}" r="{r_}" fill="#fff8e8" opacity="{o}"/>' for (x, y), r_, o in zip(bright, [2.2, 1.7, 1.2, 1.4], [0.7, 0.55, 0.45, 0.65])),
        f'<circle cx="{MX}" cy="{MY}" r="{MR}" fill="url(#limb)"/>',
        '</g>']

# ---- the lines on the still water: long tapered strokes, a little wavy ----
def stroke_line(x1, x2, y, thick, amp, seed):
    r = random.Random(seed); n = 18; ph = r.uniform(0, 6.28); wl = r.uniform(28, 60)
    top, bot = [], []
    for i in range(n+1):
        t = i/n; x = x1+(x2-x1)*t; yy = y + amp*math.sin(x/wl*6.28+ph); w = thick*math.sin(math.pi*t)**0.8/2
        top.append((x, yy-w)); bot.append((x, yy+w))
    pts = top + list(reversed(bot))
    return "M" + " L".join(f"{f(a)},{f(b)}" for a, b in pts) + " Z"
LR = random.Random(9); dim = []; lit = []
y = 414
while y < 540:
    t = (y-414)/126; L = LR.uniform(24, 64)*(1+t*1.4); x = LR.uniform(50, 460)
    if 250 < x < 335 and y < 440: x += 110   # leave the falls' foot to its ripples
    (lit if abs(x - MX) < 62 else dim).append(stroke_line(x-L/2, x+L/2, y, 1.0+t*1.4, 0.15+t*0.3, int(y*7)))
    y += 9 + t*22
calm = [f'<path d="{" ".join(dim)}" fill="#a07098" opacity="0.28"/>', f'<path d="{" ".join(lit)}" fill="#f8d098" opacity="0.38"/>']

# ---- the ripples where the falls lands: broken rings, thick in front, thin at the sides ----
def arc(cx, cy, rx, ry, a1, a2, wmax, seed):
    r = random.Random(seed); n = 30; wob = r.uniform(0, 6.28); outer_, inner_ = [], []
    for i in range(n+1):
        t = i/n; a = a1+(a2-a1)*t; k = 1 + 0.025*math.sin(3*a+wob)
        w = wmax*max(0.15, math.sin(a))*math.sin(math.pi*t)**0.7
        x, y = cx+rx*k*math.cos(a), cy+ry*k*math.sin(a); s = w/2/ max(1, ry)  # thickness along the ellipse's normal, mostly vertical
        outer_.append((x, y+w/2)); inner_.append((x, y-w/2))
    pts = outer_ + list(reversed(inner_))
    return "M" + " L".join(f"{f(a)},{f(b)}" for a, b in pts) + " Z"
RR = random.Random(3); bright_arcs = []; dim_arcs = []
for rx, wmax, o in [(36, 3.2, 0.95), (58, 3.0, 0.8), (84, 2.6, 0.6), (114, 2, 0.4)]:
    ry = rx*0.16; a = 0.08
    segs = []
    if rx < 40: segs = [(0.12, math.pi-0.12)]; a = math.pi
    while a < math.pi - 0.08:
        L = RR.uniform(0.5, 1.3); b = min(math.pi-0.04, a+L); segs.append((a, b)); a = b + RR.uniform(0.08, 0.22)
    for a1, a2 in segs:
        mid = (a1+a2)/2; d = arc(290, 405, rx, ry, a1, a2, wmax, int(rx*100+a1*10))
        (bright_arcs if math.cos(mid) < -0.2 or rx < 40 else dim_arcs).append((d, o))
rip = ['<g clip-path="url(#lakeclip)">']
for d, o in bright_arcs: rip.append(f'<path d="{d}" fill="#fff0d4" opacity="{o:.2f}"/>')
for d, o in dim_arcs: rip.append(f'<path d="{d}" fill="#e8b48c" opacity="{o*0.85:.2f}"/>')
rip.append('</g>')

FOREST = random.Random(21); trees_d = []
x = 28
while x < 314:
    b = hill(x)
    if FOREST.random() < 0.45:
        h = FOREST.uniform(9, 17); trees_d.append(pine(x, b+2, h, int(x*31)))
        x += FOREST.uniform(3.5, 6.5)
    else:
        r_ = FOREST.uniform(3.2, 6.2); cy = b - r_*FOREST.uniform(0.5, 0.95)
        trees_d.append(smooth_closed(ccw([(x + r_*math.cos(a)*(1+FOREST.uniform(-0.12,0.12)), cy + r_*0.9*math.sin(a)*(1+FOREST.uniform(-0.12,0.12))) for a in [i*math.pi/5 for i in range(10)]])))
        x += r_*FOREST.uniform(1.1, 1.6)
GROUND_D = f"M28,404 L28,{f(hill(28)+1)} " + ' '.join(f"L{f(xx)},{f(hill(xx)+1)}" for xx in range(32, 316, 4)) + " L314,404 Z"
FOREST_D = ' '.join(trees_d)
# soft shaded faces on the big peak (the side away from the moon), instead of lines
PEAK_SHADE = "M260,318 L264,320 L270,326 L276,325 L284,334 L292,340 L300,344 L312,348 L312,404 L292,404 L284,388 L278,372 L274,360 L268,350 L266,338 L262,328 Z"
# the cliff: faint tapered ledges instead of dashes
LEDGES = ' '.join(stroke_line(x1, x2, y, th, 0.4, int(y)) for x1, x2, y, th in [(352,446,250,4),(404,506,298,4.5),(340,430,350,4),(460,560,334,3.5),(470,570,232,3)])

MOONGRAD = {
 'harvest': '<radialGradient id="moon" cx="0.4" cy="0.36" r="0.72"><stop offset="0" stop-color="#fff4d4"/><stop offset="0.55" stop-color="#ffdc98"/><stop offset="1" stop-color="#f2bc74"/></radialGradient>',
 'pale': '<radialGradient id="moon" cx="0.4" cy="0.36" r="0.72"><stop offset="0" stop-color="#fffaee"/><stop offset="0.6" stop-color="#fbecc8"/><stop offset="1" stop-color="#ecd0a4"/></radialGradient>',
 'crescent': '<radialGradient id="moon" cx="0.3" cy="0.5" r="0.8"><stop offset="0" stop-color="#fff4d4"/><stop offset="0.6" stop-color="#ffdc98"/><stop offset="1" stop-color="#f2bc74"/></radialGradient>',
}[STYLE]
svg=f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="{CROP}">
<defs>
  <linearGradient id="sky" gradientUnits="userSpaceOnUse" x1="0" y1="0" x2="0" y2="512"><stop offset="0" stop-color="#2c1a4a"/><stop offset="0.43" stop-color="#5a3060"/><stop offset="0.79" stop-color="#e08a5a"/></linearGradient>
  <linearGradient id="lake" gradientUnits="userSpaceOnUse" x1="0" y1="404" x2="0" y2="536"><stop offset="0" stop-color="#4a2a58"/><stop offset="1" stop-color="#1c0e2c"/></linearGradient>
  <linearGradient id="rock" gradientUnits="userSpaceOnUse" x1="0" y1="200" x2="0" y2="404"><stop offset="0" stop-color="#2a1740"/><stop offset="1" stop-color="#150a20"/></linearGradient>
  <linearGradient id="far" gradientUnits="userSpaceOnUse" x1="0" y1="318" x2="0" y2="404"><stop offset="0" stop-color="#8c4c6c"/><stop offset="1" stop-color="#b8706c"/></linearGradient>
  <linearGradient id="water" gradientUnits="userSpaceOnUse" x1="0" y1="190" x2="0" y2="404"><stop offset="0" stop-color="#f4c48e"/><stop offset="0.35" stop-color="#fcd8a8"/><stop offset="1" stop-color="#fff0dc"/></linearGradient>
  <linearGradient id="cliffrefl" gradientUnits="userSpaceOnUse" x1="0" y1="404" x2="0" y2="536"><stop offset="0" stop-color="#120818" stop-opacity="0.55"/><stop offset="1" stop-color="#120818" stop-opacity="0.05"/></linearGradient>
  <linearGradient id="farrefl" gradientUnits="userSpaceOnUse" x1="0" y1="404" x2="0" y2="480"><stop offset="0" stop-color="#6e3658" stop-opacity="0.55"/><stop offset="1" stop-color="#6e3658" stop-opacity="0"/></linearGradient>
  {MOONGRAD}
  <radialGradient id="halo"><stop offset="0.42" stop-color="#ffe2a8" stop-opacity="0.3"/><stop offset="1" stop-color="#ffe2a8" stop-opacity="0"/></radialGradient>
  <radialGradient id="mist"><stop offset="0" stop-color="#fff4dc" stop-opacity="0.6"/><stop offset="1" stop-color="#fff4dc" stop-opacity="0"/></radialGradient>
  <radialGradient id="spray"><stop offset="0" stop-color="#ffc89a" stop-opacity="0.22"/><stop offset="1" stop-color="#ffc89a" stop-opacity="0"/></radialGradient>
  <radialGradient id="moonglow"><stop offset="0" stop-color="#ffd890" stop-opacity="0.22"/><stop offset="1" stop-color="#ffd890" stop-opacity="0"/></radialGradient>
  <radialGradient id="moonglow2"><stop offset="0" stop-color="#ffd890" stop-opacity="0.4"/><stop offset="1" stop-color="#ffd890" stop-opacity="0"/></radialGradient>
  <radialGradient id="fallglow"><stop offset="0" stop-color="#ffe6c0" stop-opacity="0.32"/><stop offset="1" stop-color="#ffe6c0" stop-opacity="0"/></radialGradient>
  {''.join(EXTRA_DEFS)}
  <radialGradient id="mfade"><stop offset="0" stop-color="#ffdc98" stop-opacity="0.5"/><stop offset="0.65" stop-color="#ffd08a" stop-opacity="0.32"/><stop offset="1" stop-color="#ffd08a" stop-opacity="0"/></radialGradient>
  <linearGradient id="ffade" gradientUnits="userSpaceOnUse" x1="0" y1="404" x2="0" y2="300"><stop offset="0" stop-color="#ffe6c4" stop-opacity="0.5"/><stop offset="1" stop-color="#ffe6c4" stop-opacity="0"/></linearGradient>
  <radialGradient id="limb"><stop offset="0.7" stop-color="#c08850" stop-opacity="0"/><stop offset="1" stop-color="#c08850" stop-opacity="0.28"/></radialGradient>
  <clipPath id="moonclip">{f'<path d="{MOONSHAPE}"/>' if MOONSHAPE else f'<circle cx="{MX}" cy="{MY}" r="{MR}"/>'}</clipPath>
  <clipPath id="lakeclip"><rect x="0" y="404" width="640" height="200"/></clipPath>
  <clipPath id="w"><path d="M576 187 L512 188 L384 190 C330 192 292 214 278 262 C268 300 262 350 256 404 H324 Q318 340 320 274 C322 236 334 208 356 206 H576 Z"/></clipPath>
</defs>
<rect x="-64" y="-64" width="640" height="640" fill="url(#sky)"/>
{''.join(stars)}
{''.join(moon)}
<path d="{far}" fill="url(#far)"/>
<g transform="translate(-24 0)"><g fill="#a8667a" opacity="0.8">{''.join(f'<path d="{p}"/>' for p in lit)}</g></g>
<g transform="translate(-24 0)"><path d="{PEAK_SHADE}" fill="#7a3e62" opacity="0.35"/></g>
<path d="{GROUND_D}" fill="#6a3458"/>
<path d="{FOREST_D}" fill="#6a3458"/>
<rect x="-64" y="404" width="640" height="200" fill="url(#lake)"/>
<path d="{mirror(far_pts, squash=0.7)}" fill="url(#farrefl)"/>
<rect x="-64" y="403" width="364" height="2" fill="#f0a070" opacity="0.22"/>
{''.join(RM)}
{''.join(calm)}
<path d="M300 404 L640 404 V640 H322 Q318 520 300 404 Z" fill="url(#cliffrefl)"/>

<g fill="#120a1e">{TREES_NEAR}</g>
<path d="M576 404 V204 Q540 202 512 206 Q446 200 384 206 L356 206 C334 206 322 226 318 256 L300 404 Z" fill="url(#rock)"/>
<path d="M356 209 H576" stroke="#4a2f5e" stroke-width="4"/>
<path d="M326,214 L372,212 L384,258 L368,306 L380,356 L366,404 L322,404 Z" fill="#2e1c48" opacity="0.75"/>
<path d="M430,214 L576,210 V404 L452,404 L470,348 L446,296 L462,246 Z" fill="#120a1e" opacity="0.35"/>
<path d="{LEDGES}" fill="#3a2652" opacity="0.7"/>

<path d="M576 187 L512 188 L384 190 C330 192 292 214 278 262 C268 300 262 350 256 404 H324 Q318 340 320 274 C322 236 334 208 356 206 H576 Z" fill="url(#water)"/>
<g fill="none" stroke-linecap="round" clip-path="url(#w)">
  <path d="M512 210 H356 C330 210 318 236 316 274 Q314 340 320 406" stroke="#e8986a" stroke-width="16" opacity="0.45"/>
  {FALL_STREAKS}
</g>
<path d="M436,208 Q436,186 452,180 Q466,175 476,182 Q490,176 500,186 L500,208 Z" fill="#241438"/>
<path d="M438,192 Q446,181 458,179" stroke="#4a3060" stroke-width="2.5" fill="none" stroke-linecap="round"/>


{''.join(RF)}
{''.join(rip)}
<g transform="translate(290 396) scale(1 0.55) translate(-290 -396)"><circle cx="290" cy="396" r="70" fill="url(#mist)"/></g>
</svg>
'''
open(OUT,'w').write(svg)
