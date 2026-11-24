# python3 sheet20.py : round 20's three moon paths (top) and round 19's 3 (bottom), each at 300 px,
# in a launcher circle at 96, and at 48 px; then round20-large.png, the three at 768
import subprocess
from PIL import Image, ImageDraw, ImageFont
S = [('a', 'a  glitter'), ('b', 'b  pillar of light'), ('c', 'c  sparkle at the shore'), ('../round19/m3', 'before: round 19, 3')]
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
sheet.save('round20.png')
large = Image.new('RGB', (768*3 + 40, 788), '#f8f8f8')
for i, n in enumerate(('a', 'b', 'c')):
    subprocess.run(['node', 'render.mjs', f'{n}.svg', '/tmp/_768.png', '768'], check=True)
    large.paste(Image.open('/tmp/_768.png').convert('RGB'), (10 + i*778, 10))
large.save('round20-large.png')
