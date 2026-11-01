# 85e: the worst block's cost, at full

For the brain, from the hands' item 85e. The item is built and every other
yardstick holds; one does not, so the PR waits for a decision rather than
merging.

## The yardstick

"The worst block's cost no higher than today on the slowest target."

The slowest target measured is the player in wasm, in Node
(`tools/player-wasm.mjs --costs`, each block's least time over the runs),
as 85c measured it. Over the baseline's first 40 loops at full, mono,
best of 5 runs, two runs each:

| | worst block, run 1 | run 2 |
|---|---|---|
| `main` (v138) | 517 us (17.8%) | 537 us (18.5%) |
| 85e | 569 us (19.6%) | 571 us (19.7%) |

The worst loop on both is vei-koan (tide, corpus #37, the loop with the
most notes let go). Alone, best of 9 runs:

| vei-koan | `main` | 85e |
|---|---|---|
| mono | 521 us (17.9%) | 554 us (19.1%) |
| stereo | 562 us (19.4%) | 594 us (20.5%) |

About 33 us, 6%, on the busiest loop; the other loops rise less or not at
all. Lite is unchanged (its renders are the same bits as before).

## Why, as far as I can tell

It's the notes that now play. At full quality the budget used to refuse a
new chord's notes while the old ones' tails finished; now the old ones go
and the new ones sound (chords refused 986 -> 151 of 5,056 over the
baseline's 100 loops). The ceiling is the same 260 units, but in the
busiest moments of the busiest loop more voices now sound in the block
that costs the most.

It isn't the fade's own rendering: a note let go rendered through a buffer
for the channel and one for each of its seven own blocks' buses; it now
renders through one (`blocks::render_to`, the same sum, bit for bit: the
baseline is unchanged by it), and the worst block didn't move (574 us).

## What needs deciding

1. **Accept it**: the cost of the notes the item lets play, at 19-21% of
   the deadline at worst, under the 25% item 86 holds a change of loop to,
   and on lite (the phones' mix) nothing moves. 85c read the same
   yardstick on lite as "unchanged within the noise" while noting 5-9%
   more instructions for the notes that now play; here the extra notes
   show in the worst block.
2. **Or hold the worst block where it was**, at full only: a lower full
   ceiling than 260, so fewer voices sound at once. That gives back some of
   the refusals (how many, I'd measure), and is for Mikey's ears.

## For the Done line, when it merges

`#PR`, the budget takes room from old notes at full quality too (item 85e,
v139): full refusals over the baseline's 100 loops, chords 19.5% -> 3.0%
of notes, melody 13 -> 0, bass 4 -> 0, texture 59 -> 48; shatter at
`bpm=180`, chords 39.8% -> 6.8% (per loop), melody 0.6% -> 0. Lite
unchanged, bit for bit. Listen at full for chords that now come in (their-roask
and ri-houng, keys about 2.5 LU up) and for long tails let go on a slow loop
(theith-yaing): the Full Room Album,
<https://claude.ai/artifact/NmfNUM1hj6buNUHKZzG9of>.
