# Port round 20's c into the app's icon source: python3 port.py <path to icons/icon.svg>
# Splits the drawing into #background (the sky) and #foreground (all on it; the pool too, since the cliffs' feet
# reach under its edge), marks the key
# (the moon, the stream, its first ring) and adds #monochrome, drawn by hand for Android's themed icon.
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.argv = [sys.argv[0], 'c', os.devnull, sys.argv[1]]
import moonpath
G = moonpath.G; G['TAKE'] = 'c'
from geo import geo, P, smooth, f, pool_rect, sky_rect, H
ic = G['build'](3); g = geo()
body = list(ic.body)

def take(pred, what):
    for i, el in enumerate(body):
        if pred(el): return i
    raise SystemExit(f'not found: {what}')
sky = body.pop(take(lambda e: f'd="{P(sky_rect())}"' in e, 'the sky'))
def key(i): body[i] = body[i].replace('<path ', '<path class="key" ', 1).replace('<circle ', '<circle class="key" ', 1)
key(take(lambda e: e.startswith('<circle') and f'cx="{f(G["MX"])}" cy="{f(G["MY"])}" r="{f(G["MR"])}"' in e, 'the moon'))
key(take(lambda e: e.startswith('<path') and f'd="{smooth(g["water"])}"' in e, 'the stream'))
t = G['Icon'](); G['ring'](t, 256, H + 6, 72, 11, 2.4, '#ffffff', 0.35, gap=(G['math'].pi*1.15, G['math'].pi*1.85))
key(take(lambda e: e == t.body[0], 'the first ring'))

# the one-color drawing: the moon, the stream, two rings where it lands, the moon's light at the shore
m = G['Icon']()
m.circle(G['MX'], G['MY'], G['MR'], '#fff')
m.path(smooth(g['water']), '#fff')
G['ring'](m, 256, H + 10, 74, 12, 7, '#fff', 1, gap=(G['math'].pi*1.1, G['math'].pi*1.9))
G['ring'](m, 256, H + 14, 108, 18, 6, '#fff', 1, gap=(G['math'].pi*1.05, G['math'].pi*1.95))
for y, w, th in ((H + 6, 52, 4.5), (H + 20, 34, 4), (H + 34, 20, 3.5)):
    G['lens'](m, G['MX'], y, w, th, '#fff', 1)
mono = '\n  '.join(e.replace(' fill="#fff"', '') for e in m.body)

head = '''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
<!-- The app's icon (item 120, Mikey's pick: round 20's c). The one source of
     every icon the app uses: node tools/icons.mjs makes them all from this
     file and checks the safe zones, so a change to the icon is an edit here
     and a run of the script. Drawn by round20/port.py on the s30l-shots
     branch, from the scripts of the rounds before it.

     The view is the icon itself (0 0 512 512), but the scene is drawn out to
     -128..640 each way: Android's adaptive icon is a 108 dp layer of which
     72 dp shows, and the feature graphic is wider, so both crop more of the
     same scene. Three groups: #background (the sky, Android's background
     layer), #foreground (everything on it, the pool too), and #monochrome
     (one color, hidden here: Android's themed icon and the status bar's
     icon). Android's layers are vector drawables made from it, so it keeps
     to what they can draw: shapes and absolute paths, flat fills, linear
     gradients, a radial one only on a circle, a clip, a group's translate
     and scale. The class "key" marks what must stay inside every safe zone (the
     moon, the stream, its first ring); the script fails if it doesn't. -->
'''
out = head + '<defs>\n  ' + '\n  '.join(ic.defs) + '\n</defs>\n<g id="background">\n  ' + sky + \
      '\n</g>\n<g id="foreground">\n  ' + '\n  '.join(body) + '\n</g>\n<g id="monochrome" display="none" fill="#fff">\n  ' + mono + '\n</g>\n</svg>\n'
open(sys.argv[3], 'w').write(out); print(sys.argv[3], len(out), 'bytes')
