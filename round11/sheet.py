# python3 sheet.py out.png before.png name:svg:viewBox ...
import sys, subprocess, os
from PIL import Image, ImageDraw, ImageFont
out, before = sys.argv[1], sys.argv[2]; items = [a.split(':', 2) for a in sys.argv[3:]]
def render(svg, size, vb, path):
    subprocess.run(['node', 'render.mjs', svg, path, str(size)] + ([vb] if vb else []), check=True)
    return Image.open(path).convert('RGBA')
try: font = ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf', 20); big = ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf', 44)
except: font = big = ImageFont.load_default()
W = 1180; rowh = 300
bimg = Image.open(before).convert('RGBA')
H = 690 + rowh*len(items) + 60
sheet = Image.new('RGBA', (W, H), '#f8f8f8'); d = ImageDraw.Draw(sheet)
# the top: the first item large, and round 9 for comparison
name, svg, vb = items[0]
sheet.paste(render(svg, 520, vb or None, '/tmp/_big.png'), (10, 10)); d.text((200, 536), f'11{name}', fill='#333', font=font)
bb = bimg.crop((10, 10, 530, 530)).resize((290, 290), Image.LANCZOS)
sheet.paste(bb, (560, 10)); d.text((630, 304), 'before (round 10a)', fill='#333', font=font)
for (px, py, vb_, cap) in [(870, 10, '300 150 150 150', 'the middle, close'), (560, 340, '0 250 150 150', 'the far shore, close'), (870, 340, '300 280 150 150', 'the foot, close')]:
    sheet.paste(render(svg, 290, vb_, '/tmp/_z.png'), (px, py)); d.text((px + 50, py + 294), cap, fill='#333', font=font)
if len(items) > 1:
    name2, svg2, vb2 = items[1]
    sm = render(svg2, 380, vb2 or None, '/tmp/_b2.png').resize((150, 150)) if False else None
y = 700
def circle(img, size):
    img = img.resize((size, size), Image.LANCZOS); m = Image.new('L', (size*4, size*4), 0)
    ImageDraw.Draw(m).ellipse((0, 0, size*4-1, size*4-1), fill=255); img.putalpha(m.resize((size, size), Image.LANCZOS)); return img
for name, svg, vb in items:
    vb = vb or None
    full = render(svg, 512, vb, '/tmp/_f.png')
    d.text((14, y + 110), f'11{name}', fill='#333', font=big)
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
