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

## A smaller one: experimental's refusals at lite

Not a stop on its own, but for the brain: experimental (one of the pairs,
not a new vibe) loses 2.2% of its tune at lite over 150 loops, against the
catalog's 1.3% on the same seeds. On `main` it lost 2.8%, so this item makes
it better, not worse. The rest is prepared piano and vowel, the voices the
item asks it to lead with (cost 25 and 22); getting to the catalog would
mean leaning away from them. At full it stays near zero. Every new vibe is
at or under the catalog at lite (the PR's table).

## The brain's answer (2026-10-29): option 3

**Glacial's arc is drawn outside the choices,** as echoTime is. The arc is
what glacial is (Mikey, 2026-10-22: "the deep arc, the longest swings"),
not one change among several, and option 3 is the only one that gets it
there without bending batch 5's x3 rule or moving any other loop's draws.
- A loop with glacial among its vibes that didn't draw the arc through the
  choices draws it with chance **0.6 x glacial's weight in the loop** (its
  share of the loop's vibes, 0.05 to 1), on the arc's own stream, after
  every other draw; not counted among the loop's changes. Glacial alone:
  31% plus 0.6 of the rest, about 72%. A named setting in `ARC`
  (`glacialChance`, 0.6), "for Mikey's ears".
- Keep glacial's x2.5 lean inside the choices as built.
- The yardstick becomes: glacial alone, the arc in at least 60% of its
  loops (report the figure); deep about 3 in 4; the periods as measured.
  Loops without glacial, unchanged (the choices' shares and every other
  draw).

**Experimental at 2.2% at lite:** accepted as is. It's better than `main`
and comes from the voices the item asks it to lead with.

**`/rank`'s pool:** leave it at version 4 without the 21 new words. The
five-word ranking rests (word albums replaced it, 2026-10-11).

Then finish, merge as the item says, and add the Done line. The brain
moves this note to the archive at its next pass.
