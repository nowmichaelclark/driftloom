# 76b: glacial's arc reaches a third of its loops, not half

**Hands, 2026-10-29, PR for item 76b (v125). Stopped at one yardstick; not
merged.** Everything else in the item is built and passes (the PR lists
it). One check can't be met as the item writes it, and meeting it needs a
decision.

## The check

"Glacial's arc: alone, the arc in at least half its loops, deep about 3 in
4 of those, every period 5-8 minutes; with one adjacent vibe, 8-12."

Measured over 600 loops of glacial alone (rendered, so the choices are
drawn with each loop's real eligibility):

| | measured | yardstick |
|---|---|---|
| loops that draw the arc | **31%** (the catalog: 17%) | at least half |
| deep, of those | 72% | about 3 in 4 |
| periods, glacial alone | 5.0-8.0 min | 5-8 |
| periods, with still or droning | 8.0-12.0 min | 8-12 |

## Why x2.5 can't get there

The arc is one of the eleven per-loop choices (item 54). A loop draws how
many changes it gets (none 15%, one 45%, two 30%, three 10%), then which,
by weight among the six to nine it is eligible for. A lean is raised to
the loop's strength (0.5-0.9) and held to x0.2-x3 (`LEAN_MIN`, `LEAN_MAX`:
batch 5's rule), so glacial's x2.5 is worth about x1.9 on the draw. The
wash and padSweep also lean toward glacial (x1.5 each, from the same item),
and they compete with the arc. Even at full strength, x2.5 would give
under 40% (between the x2.5 and x5 columns below).

The same 600 loops with `LEAN_MAX` lifted, glacial's arc lean at:

| lean | x2.5 | x5 | x10 | x20 | x40 |
|---|---|---|---|---|---|
| draws the arc | 31% | 42% | 52% | 61% | 67% |

The most any lean can give is 85%: 15% of loops draw no change at all.

## What's needed: one of

1. **Glacial's arc lean at about x12, above batch 5's x3 cap** for this one
   lean (`GLACIAL.arc` in `js/choices.js`, with `leanOf` letting it past
   `LEAN_MAX`). About 55% of glacial loops get the arc. Breaks the batch's
   x3 rule for one vibe.
2. **Keep x2.5 as the item says** and drop the yardstick to "about a third
   (twice the catalog)". Nothing more to build. This is what the PR holds
   now.
3. **Glacial's arc drawn outside the choices**, as echoTime is: a glacial
   loop that didn't draw the arc gets it with a named chance, on the arc's
   own stream, not counted among its changes. It bends Mikey's rule that
   most loops get one or two changes, for glacial only.

The hands would take 2 (the item's own number, nothing new) unless the arc
being most of glacial's loops matters to Mikey, then 1. Say which, and the
PR is finished and merged as the item says.
