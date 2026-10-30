# 82: when notes reach the core moves its last bits

For the brain, from item 82 (PR #296). Not blocking: the item's yardstick is
met. A decision for later, if it is worth one.

## What was found

The core keeps the notes waiting to start in no fixed order: when one
starts, the last waiting note fills its place (a swap-remove,
`Core::process`), and each note that starts takes the first free voice slot
in the order the queue holds them (`begin_note`). The voices are summed slot
by slot, and a float sum depends on its order. So the same notes make
slightly different samples depending on **when** the host handed them over,
not only what they were.

Three ways of handing over exist today:

1. **The core's player** (item 82): before each block, every step due within
   0.25 s (`player::LOOKAHEAD`). Fixed, so its renders are the same on every
   target and every run.
2. **The app's own engine**, playing: on its timer's ticks, with a lookahead
   that adapts to the page (hidden, stalled). The ticks fall where the page's
   timers fall, so the app's output varies in its last bits from one play to
   the next, today, inaudibly. No harness can repeat it.
3. **The measure harness** (`tools/measure.mjs`, which `tools/baseline.mjs`'s
   older parts use): the whole render decided and handed over before it
   starts.

The player's yardstick holds against the app's engine handing its notes over
as the player does (`tools/player.mjs`): all 160 renders (the baseline's 40
loops, full and lite, mono and stereo, 45 s, wander on) equal natively, in
`js/dlplayer.wasm` and through the C header, bit for bit; the baseline keeps
them as its `player` part.

## By how much the up-front way differs

`node tools/player.mjs --upfront` renders each loop both ways in Chromium and
compares the samples. Across the 160 renders:

| way | renders that differ | first difference | largest difference | the difference, under the render |
|---|---|---|---|---|
| full, mono | 39 of 40 | 0.08 to 17.4 s | 6.9e-5 (-83.2 dBFS) | 87.1 to 92.3 dB (median 89.0) |
| full, stereo | 39 of 40 | 0.08 to 17.4 s | 9.8e-5 (-80.2 dBFS) | 86.8 to 91.8 dB (median 88.2) |
| lite, mono | 38 of 40 | 0.08 to 38.9 s | 5.2e-5 (-85.7 dBFS) | 91.2 to 105.4 dB (median 94.8) |
| lite, stereo | 38 of 40 | 0.08 to 38.9 s | 6.3e-4 (-64.0 dBFS) | 77.3 to 103.4 dB (median 91.1) |

Six renders come out the same both ways (their-roask lite, taing-doan in all
four). The largest, -64 dBFS, is one loop (sang-hoth, lite, stereo); the
rest stay under -80 dBFS. Both ways make the same notes (the voice budget
judges each note at its own time, not the clock's), so the differences start
in the order of the core's sums, and the mix carries them on: the echo's and
the reverb's feedback, the compressors, the stereo width ceiling. None comes
near hearing (they sit 77 dB or more under the music, most near 90). Not the
same bits, though, and the older parts of the baseline are up-front renders:
the harness's way of playing, not the app's.

## The decision, if wanted

A canonical order in the core (the waiting notes kept sorted by start time,
then by when they arrived, or the voices summed in a fixed order) would make
every way of handing over give the same samples, the app's own engine
included. It would move every golden and baseline hash once (a deliberate
`--update`, no audible change), and costs a little in the queue. The hands'
suggestion: worth it in phase 3, when the app moves onto the player and the
page's own hand-over goes away anyway; until then the player's fixed way is
what every check holds.
