# python3 trace.py in.png out.svg [speckle] [colorprec] [layerdiff]: trace a painted scene (the -128..640 layer at 2 px per unit)
# into stacked color shapes, then wrap them so the SVG's view is the icon (0 0 512 512) with the scene out to -128..640.
import sys, re, vtracer
inp, out = sys.argv[1], sys.argv[2]
sp = int(sys.argv[3]) if len(sys.argv) > 3 else 6; cp = int(sys.argv[4]) if len(sys.argv) > 4 else 6; ld = int(sys.argv[5]) if len(sys.argv) > 5 else 12
vtracer.convert_image_to_svg_py(inp, '/tmp/_t.svg', colormode='color', hierarchical='stacked', mode='spline', filter_speckle=sp,
    color_precision=cp, layer_difference=ld, corner_threshold=60, length_threshold=4.0, max_iterations=10, splice_threshold=45, path_precision=1)
s = open('/tmp/_t.svg').read()
body = s[s.index('>', s.index('<svg')) + 1:s.rindex('</svg>')]
n = body.count('<path')
open(out, 'w').write(f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><g transform="translate(-128 -128) scale(0.5)">{body}</g></svg>\n')
print(out, n, 'paths', len(open(out).read())//1024, 'KB')
