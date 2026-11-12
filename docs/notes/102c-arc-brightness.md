# 102c: the arc's brightness, short of 9 in 10

Hands session 13, 2026-11-12, PR #377. Everything the session's prompt
names passes but two yardsticks, each by a few pieces. The hands did not
merge it: the merge policy holds a named check that fails. What was needed:
a decision, below (if this file is on `main`, #377 was merged on it).

## What was measured

`tools/longform.mjs` (`tools/longform-played.mjs`'s `arcChecks`,
`arcVerdict`), on corpus 20261009's first seeds, each piece rendered whole
at 44.1 kHz at the wander's amount 0 (the plan with no breathing) and
measured unit by unit against the same seed as a plain loop on the same
bars (so a loop's own form, loud in one section and quiet the next, is not
taken for the plan). Loudness by ear is K-weighted; brightness is the
spectral centroid of the magnitude spectrum over 250 Hz (above the bass,
whose level the loudness reads). "In the planned order": each role's mean
against its energy, Spearman's rank correlation 0.5 or more.

| | 64 bars (40) | 256 (16) | 1,024 (6) | in all (62) |
|---|---|---|---|---|
| loudness rises to the peak and falls | 39 | 16 | 6 | 61 (98%) |
| brightness rises to the peak and falls | 36 | 14 | 4 | **54 (87%)** |
| the peak at most 4 LU over the opening | 40 | 16 | 6 | 62 (at most 2.3 LU) |
| no step past 2 LU between sections (what the arc adds) | 40 | 16 | 5 | **61** (one at 2.7) |
| the last chord the tonic's, alone; a ring-out | 40, 40 | 16, 16 | 6, 6 | 62, 62 |

As heard at the app's amount (10 pieces at 64 bars, the loop's own form
and breathing in it): the loudness rises and falls on 9, the brightness on
9.

## Why the misses miss

The same four loops miss at every length:

- **tui-doask (176315227) and so-rou (2541830973):** no kit, piano chords,
  a swell or a bell; so-rou has no tune. Their music lies under about
  1 kHz, so the filter's darker aim has nothing to take away, and what the
  arc fades (the bass, a dark swell texture) leaves the rest reading a
  little *brighter*. tui-doask's sections are 50 s long, so at 1,024 bars
  a breath between two arcs is one section, and the tune resting in it is
  the 2.7 LU step.
- **gluith-vuisk (1068550449) and rong-wuisk (3372926893):** batch 6's
  slowOpening loops. Their kit sits out for the first 40 to 90 s in the
  plain loop and the piece alike, so the arc's opening has nothing to hold
  back there; and at the peak the doubled tune, low, lands under the bright
  kit's centroid and reads darker, though it is an octave higher. As heard
  (not against the plain loop) both rise and fall clearly: rong-wuisk's
  centroid goes 284 Hz, 1,640, 570.

## What was tried

The filter's darkest aim from 0.3 to 0.6 of its range (more is the deep
swing's muffle); the loop's own brightness placed at energy 0.6, 0.7 and 1;
the space held still (no effect); the kit's shimmer, then its backbeat,
coming in with the energy (each helped other loops); the double yielding to
the voice budget (no effect here); a filter that a shallower swing never
engages without a swing (no effect here). Each is in the PR.

## What to decide

1. **Accept 87% brightness and the one 2.7 LU step** (the misses explained: loops with
   nothing above 1 kHz to brighten, and slowOpening loops whose own opening
   coincides with the arc's), and merge #377 as it is. My recommendation:
   the measure is honest about what a centroid can show, and Mikey's ears
   are the real test of whether these four rise and fall.
2. **Or ask for more** before it merges: for example, a breath that keeps
   the tune at its thinnest rather than resting it (the 2.7 LU step), or
   the tune moved up an octave rather than doubled at a peak (a real
   register change, louder in the measure, a bigger change to hear).
