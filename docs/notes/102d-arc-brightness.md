# 102d: the arc's brightness order fell from 87% to 77%

Hands session 14, 2026-11-13, PR #379. Every yardstick the session's prompt
names passes. One check from session 13 (102c), already short of its bar on
`main`, is further short on this branch, so the hands did not merge: the
decision is below (if this file is on `main`, #379 was merged on it).

## What was measured

`node tools/longform.mjs --arcs 30`: 47 corpus pieces (30 at 64 bars, 12 at
256, 5 at 1,024), each unit against the loop as itself on the same bars
(`docs/notes/102c-arc-brightness.md` says how).

| | `main` (62 pieces) | this branch (47) |
|---|---|---|
| loudness rises to the peak and falls | 98% | 98% |
| brightness rises to the peak and falls (9 in 10 asked) | 87% | **77%** |
| the same on the units in the loop's own voice and register (reported) | | 20 of 27 judged |
| the peak over the opening | at most 2.3 LU | at most 3.2 LU (4 asked) |
| as heard at the app's amount (10 at 64 bars) | 9 loudness, 9 brightness | 9, 9 |

## Why

The four loops of 102c's note still miss (nothing above 1 kHz to brighten;
a slow opening of the loop's own). The new misses are what this session
adds, read by a centroid:

- **A return an octave up** reads 500 to 900 cents brighter than the loop,
  which is brighter than the peak (4154528281, 1086007177 at 256 and 1,024
  bars): the rank order of roles against energy breaks, by design.
- **A lift handed to another voice** reads darker than the loop's own
  voice, though louder (4154528281: 450 cents darker at +0.7 LU).
- **A release in outline** (half the tune's notes) reads about 170 cents
  darker than the return after it (1877895177, 4157082253).

Leaving the varied units out does not settle it: where a piece still has
units to judge, 20 of 27 rise and fall, and six of the seven misses are
102c's four loops.

## What was tried

Keys reached the way up for every layer over the bass (68% to 77%; kept).
A lift's or a peak's register variation left as session 13's double, and a
build's as the double, so no tune sits under the theme there (kept).

## What to decide

1. **Accept it and merge #379** (my recommendation): the brief asks for a
   return that is changed, and an octave or a voice is what a centroid
   hears most. Whether the piece still rises and falls is for Mikey's ears;
   as heard at the app's amount it does on 9 of 10, as on `main`.
2. **Or ask for more first:** for example, a return's register only ever
   downward or no more than the peak's; the voice variation kept out of the
   lift; the outline's share in the release smaller (`outline` in
   `data/longform.txt`, now 0.5).
3. **Or change the yardstick** to judge brightness on the opening, the
   peak and the close alone (the tool reports the figure beside the check).
