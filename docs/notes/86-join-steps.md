# 86: the join's step, against the loops' own

For the brain, from the hands' item 86. The item is built and every other
yardstick holds; one, as written, does not, so the PR waits for a decision
rather than merging.

## The yardstick

"Across 1,000 changes at every boundary kind no click (the sample step at
the join no larger than the loops' own)."

`dlrender --changes 1000` (corpus 1, 44.1 kHz, full, wander 1, the default
crossfade of 1 s; the five kinds of moment in turn, every other change
bending the old loop's seed as a theme) measures, for each change, the
largest step between two samples from 20 ms before the boundary to 20 ms
after the crossfade, against the largest step each loop takes alone over the
same length (the old loop with no change asked, the new one loaded alone at
the boundary):

| | changes | worst |
|---|---|---|
| over the larger loop's own | 88 of 1,000 (22 by more than 10%, 8 by more than 25%) | 2.01x |
| over the two loops' own summed | 4 of 1,000 | 1.43x |
| lite, 200 changes | 18 over the larger, 0 over the sum | 1.17x, 0.94x |
| 48 kHz, 200 changes | 25 over the larger, 0 over the sum | 1.39x, 0.99x |
| ring-out, 200 changes | 0 | 0.49x |

## What the steps were, and what was fixed

Two were real clicks, both the arc's (the master's slow lowpass), and both
in the app's own loads as well; they are fixed in this PR (`blocks/arc.rs`):

- **An arc starting while music plays** (a loop with the arc after one
  without) switched its filter on from rest: a step 10 ms after the boundary
  (the master's lookahead), up to 6x the loops' own. The filter now runs,
  open, while there is no arc, leaving the signal untouched, so it takes
  over in step with it.
- **A new swing read before its start**: in the block a swing begins, the
  part before its time read the new swing, so an arc's end opened the
  cutoff for one block and glided back from the old one: a ringing buzz, 2.3x
  the loops' own.

Neither moves anything that exists: the core baseline (Chromium, costs
included), the golden lists at every rate, block size and in stereo, the 80
seeds and the browser's 160 renders are all unchanged, since a loop alone
has no music before its arc starts.

What is left is two loops sounding together. A crossfade lets the old
loop's last notes ring, falling, under the new loop's first, and the master
(tape wow, the saturator, the compressor) works on their sum. Every case
over the larger loop's own that I opened was an old hit's tail under the new
loop's first kick or chord. For two of the 4 over the summed bound (changes
304 and 354), I left out each call the player makes for the boundary in turn
(every parameter but the kick's pump, every setting and event, the crossfade
itself, the bass lift): none moves the step by more than 0.01, so no setting
jumps there; the other two (271, 773, 1.19x and 1.02x) I have not opened. A ring-out had none over the larger loop's
own in 200 changes; I have not looked at why.

## What needs deciding

1. **Read "the loops' own" as the two loops sounding together** (the sum of
   their own largest steps): 996 of 1,000 hold, and the 2 of the 4 that
   don't that I opened are the master's chain working on two loops at once
   (by 1.43x at worst). This is
   what the PR measures and reports beside the literal figure.
2. **Or change the crossfade** so the overlap is smaller at the boundary:
   the old loop's fall starting before the boundary (centered on it), or
   the new loop fading in. Either is a small change in `Player::begin`. But
   it changes how the change sounds, which is for Mikey's ears: the default
   here lands the new loop on the moment at its own level, the conservative
   reading of "a change at a musical moment".

## For the Done line, when it merges

`#PR`: Item 86, a change of loop at a musical moment (`Player::change`; the
app is unchanged but for the arc: at a Next where either loop has the arc
there is no click or one-block jump). For the ears: `dlrender --changes 1000
--change 7 --wav-to DIR` (a bar change, theme bent) and `--change 13` (a
phrase); the crossfade's 1 s and the new loop at full on the boundary are
for Mikey's ears.
