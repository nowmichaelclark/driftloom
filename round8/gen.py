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
TREES_NEAR = ''.join(f'<path d="{pine(x, 189, h, 10+i)}"/>' for i,(x,h) in enumerate([(440,40),(478,58),(520,46),(552,40)]))
stars=[]
for x,y,r,o in [(84,160,2.2,.9),(132,188,1.4,.6),(214,150,1.8,.8),(262,182,1.2,.55),(300,146,2,.85),(352,170,1.4,.6),(410,150,1.6,.7),(446,196,1.2,.5),(100,214,1.1,.45),(236,226,1.3,.5),(388,224,1.1,.4)]:
    stars.append(f'<circle cx="{x}" cy="{y}" r="{r}" fill="#fff4dc" opacity="{o}"/>')
stars.append('<path d="M300 136 V156 M290 146 H310" stroke="#fff4dc" stroke-width="1.2" opacity="0.5" stroke-linecap="round"/>')

# ---- the mountains: a far range with a real ridge, a near hill with trees ----
def ridge(points):
    d=f"M{f(points[0][0])},{f(points[0][1])}"
    for (x,y) in points[1:]: d+=f" L{f(x)},{f(y)}"
    return d
far_pts=[(30,404),(30,378),(44,374),(52,376),(62,369),(70,371),(80,363),(88,366),(96,361),(104,364),(112,356),(118,357),(126,352),
         (134,356),(142,354),(152,360),(164,357),(176,363),(188,360),(198,364),(208,358),(218,350),(226,346),(232,339),(238,336),
         (244,330),(250,331),(256,322),(260,318),(264,320),(270,326),(276,325),(284,334),(292,340),(300,344),(312,348),(312,404)]
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
                    glints.append(sparkle(cx+GR.uniform(-half*0.5,half*0.5), y, GR.uniform(4,6.5), o))
            y += GR.uniform(4,7) + t*14
        if REFL == 'v3': RM=[soft]+glints
        elif REFL == 'v4':
            RM=[glowM, mirrored(f'<circle cx="{MX}" cy="{MY}" r="{MR*1.12:.0f}" fill="url(#mfade)"/>', 0.62, 0.32)]+glints
        else: RM=[glowM]+glints

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
  <clipPath id="moonclip">{f'<path d="{MOONSHAPE}"/>' if MOONSHAPE else f'<circle cx="{MX}" cy="{MY}" r="{MR}"/>'}</clipPath>
  <clipPath id="lakeclip"><rect x="0" y="404" width="640" height="200"/></clipPath>
  <clipPath id="w"><path d="M576 187 L512 188 L384 190 C330 192 292 214 278 262 C268 300 262 350 256 404 H324 Q318 340 320 274 C322 236 334 208 356 206 H576 Z"/></clipPath>
</defs>
<rect x="-64" y="-64" width="640" height="640" fill="url(#sky)"/>
{''.join(stars)}
{''.join(moon)}
<path d="{far}" fill="url(#far)"/>
<g fill="#a8667a" opacity="0.8">{''.join(f'<path d="{p}"/>' for p in lit)}</g>
<g fill="none" stroke="#7a3e62" stroke-width="1.6" stroke-linecap="round" opacity="0.6">{''.join(f'<path d="{g}"/>' for g in gullies)}</g>
<path d="{nearp}" fill="#6a3458"/>
<rect x="-64" y="404" width="640" height="200" fill="url(#lake)"/>
<path d="{mirror(far_pts, squash=0.7)}" fill="url(#farrefl)"/>
<rect x="-64" y="403" width="364" height="2" fill="#f0a070" opacity="0.5"/>
{''.join(RM)}
{''.join(calm)}
<path d="M300 404 L576 404 V536 H316 Z" fill="url(#cliffrefl)"/>
<g fill="#34204a">{TREES_FAR}</g>
<g fill="#1a0c28">{TREES_NEAR}</g>
<path d="M576 404 V204 Q540 202 512 206 Q446 200 384 206 L356 206 C334 206 322 226 318 256 L300 404 Z" fill="url(#rock)"/>
<path d="M356 209 H576" stroke="#4a2f5e" stroke-width="4"/>
<g fill="none" stroke="#24123a" stroke-width="5" stroke-linecap="round"><path d="M372 252 L440 248"/><path d="M412 302 L500 296"/><path d="M350 354 L420 350"/></g>
<ellipse cx="330" cy="380" rx="90" ry="70" fill="url(#spray)"/>
<path d="M576 187 L512 188 L384 190 C330 192 292 214 278 262 C268 300 262 350 256 404 H324 Q318 340 320 274 C322 236 334 208 356 206 H576 Z" fill="url(#water)"/>
<g fill="none" stroke-linecap="round" clip-path="url(#w)">
  <path d="M512 210 H356 C330 210 318 236 316 274 Q314 340 320 406" stroke="#e8986a" stroke-width="16" opacity="0.45"/>
  {FALL_STREAKS}
</g>
<g fill="#fff4e4">{FALL_SPRAY}</g>
{''.join(RF)}
{''.join(rip)}
<g transform="translate(290 396) scale(1 0.55) translate(-290 -396)"><circle cx="290" cy="396" r="70" fill="url(#mist)"/></g>
</svg>
'''
open(OUT,'w').write(svg)
