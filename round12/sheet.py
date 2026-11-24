# python3 sheet.py out.png before.png name:svg:viewBox ...
import sys, subprocess, os
from PIL import Image, ImageDraw, ImageFont
out, before = sys.argv[1], sys.argv[2]; items = [a.split(':', 2) for a in sys.argv[3:]]
def render(svg, size, vb, path):
    subprocess.run(['node', 'render.mjs', svg, path, str(size)] + ([vb] if vb else []), check=True)
    return Image.open(path).convert('RGBA')
try: font = ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf', 20); big = ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf', 44)
except: font = big = ImageFont.load_default()
W = 1180; rowh = 290
bimg = None
H = 450 + rowh*len(items) + 50
sheet = Image.new('RGBA', (W, H), '#f8f8f8'); d = ImageDraw.Draw(sheet)
# the top: the three at 370 px, with Android's 66 dp safe circle and the maskable 40% circle drawn
for i, (name, svg, vb) in enumerate(items):
    im = render(svg, 370, vb or None, f'/tmp/_t{i}.png'); dd = ImageDraw.Draw(im)
    r1 = 370*235/512; r2 = 370*0.4*0.5*512/512*1.0
    c = 185
    dd.ellipse((c - r1, c - r1, c + r1, c + r1), outline=(255, 255, 255, 170), width=2)
    sheet.paste(im, (14 + i*388, 10)); d.text((14 + i*388 + 150, 386), f'12-{name}', fill='#333', font=font)
d.text((14, 416), 'white circle: the safe zone (Android 66 of 108 dp); the main shapes sit inside it', fill='#555', font=ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf', 15))
y = 450
def circle(img, size):
    img = img.resize((size, size), Image.LANCZOS); m = Image.new('L', (size*4, size*4), 0)
    ImageDraw.Draw(m).ellipse((0, 0, size*4-1, size*4-1), fill=255); img.putalpha(m.resize((size, size), Image.LANCZOS)); return img
for name, svg, vb in items:
    vb = vb or None
    full = render(svg, 512, vb, '/tmp/_f.png')
    d.text((10, y + 110), f'{name}', fill='#333', font=big)
    sheet.paste(full.resize((256, 256), Image.LANCZOS), (100, y))
    x = 372
    for bg in ('#e8e6f0', '#202024'):
        tile = Image.new('RGBA', (144, 144), bg); tile.alpha_composite(circle(full, 128), (8, 8)); sheet.paste(tile, (x, y + 56)); x += 160
    for bg in ('#e8e6f0', '#202024'):
        s48 = render(svg, 48, vb, '/tmp/_48.png')
        tile = Image.new('RGBA', (64, 64), bg); tile.alpha_composite(s48 if bg == '#e8e6f0' else circle(s48, 48), (8, 8)); sheet.paste(tile, (x, y + 96)); x += 80
    s16 = render(svg, 16, vb, '/tmp/_16.png')
    tile = Image.new('RGBA', (32, 32), '#e8e6f0'); tile.alpha_composite(s16, (8, 8)); sheet.paste(tile, (x, y + 112)); x += 46
    sheet.paste(s16.resize((64, 64), Image.NEAREST), (x, y + 96)); x += 80
    # one color, rough: the bright parts of the drawing at 48 px
    g = render(svg, 192, vb, '/tmp/_m.png').convert('L').point(lambda v: 255 if v > 150 else 0).resize((48, 48), Image.LANCZOS)
    for bg, fg in (('#dfe3ff', '#2a3a8a'), ('#16181e', '#c8d0ff')):
        tile = Image.new('RGBA', (64, 64), '#f8f8f8'); td = ImageDraw.Draw(tile); td.ellipse((0, 0, 63, 63), fill=bg)
        col = Image.new('RGBA', (48, 48), fg); col.putalpha(g); tile.alpha_composite(col, (8, 8)); sheet.paste(tile, (x, y + 96)); x += 80
    y += rowh
d.text((14, H - 44), '512 px  |  launcher circle on light and dark (128)  |  48 px square and round  |  16 px, and 16 px enlarged  |  one color at 48 px (a rough preview)', fill='#555', font=ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf', 15))
sheet.convert('RGB').save(out)
