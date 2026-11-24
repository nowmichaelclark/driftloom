# python3 sheet18.py : round 18's three (top) beside round 17's 1, 2 and 4 (bottom), each at 300 px,
# in a launcher circle at 96, and at 48 px; then round18-large.png, the three at 768
import subprocess
from PIL import Image, ImageDraw, ImageFont
S = [('dusk', '1  dusk, realistic'), ('glow', '2  afterglow, realistic'), ('okeeffe', "3  O'Keeffe, realistic"),
     ('../round17/dusk', 'before: round 17, 1'), ('../round17/glow', 'before: round 17, 2'), ('../round17/okeeffe', 'before: round 17, 4')]
font = ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf', 17)
W, TH = 330, 470
sheet = Image.new('RGB', (W*3 + 20, TH*2 + 20), '#f8f8f8'); d = ImageDraw.Draw(sheet)
def circle(img, size):
    img = img.resize((size, size), Image.LANCZOS).convert('RGBA'); m = Image.new('L', (size*4, size*4), 0)
    ImageDraw.Draw(m).ellipse((0, 0, size*4 - 1, size*4 - 1), fill=255); img.putalpha(m.resize((size, size), Image.LANCZOS)); return img
for i, (n, cap) in enumerate(S):
    x = 10 + (i % 3)*W; y = 10 + (i//3)*TH
    big = Image.open(f'{n}.png').convert('RGB').resize((300, 300), Image.LANCZOS); sheet.paste(big, (x + 5, y))
    d.text((x + 6, y + 306), cap, fill='#333', font=font)
    for j, bg in enumerate(('#e8e6f0', '#202024')):
        t = Image.new('RGBA', (112, 112), bg); t.alpha_composite(circle(big, 96), (8, 8)); sheet.paste(t, (x + 5 + j*118, y + 336))
    subprocess.run(['node', 'render.mjs', f'{n}.svg', '/tmp/_48.png', '48'], check=True)
    s48 = Image.open('/tmp/_48.png').convert('RGB'); t = Image.new('RGB', (64, 64), '#e8e6f0'); t.paste(s48, (8, 8)); sheet.paste(t, (x + 245, y + 360))
sheet.save('round18.png')
large = Image.new('RGB', (768*3 + 40, 788), '#f8f8f8')
for i, n in enumerate(('dusk', 'glow', 'okeeffe')):
    subprocess.run(['node', 'render.mjs', f'{n}.svg', '/tmp/_768.png', '768'], check=True)
    large.paste(Image.open('/tmp/_768.png').convert('RGB'), (10 + i*778, 10))
large.save('round18-large.png')
