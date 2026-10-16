# Ears prototypes: the first sounds from batches one and two

> **Warning (Mikey, 2026-10-10):** on the live preview, with or without
> `arc`, some of the old sounds go missing. Not diagnosed. Prime suspect:
> the voice budget. `organ`, `glass` and `dubStab` take budget, and the
> organ holds it for whole chords, so the regular voices get refused.
> Also suspect: `arc` starts every loop closed (300 Hz). Treat this branch
> as ideas only. Redo the sketches one at a time, with the refusal and
> level checks the queue uses.

2026-10-10. The sounds-and-ideas chat built nine of the ideas from
`docs/ears/batch-1.md` and `docs/ears/batch-2.md`. They're on the branch
`ears/prototypes`, draft PR #152
(https://github.com/nowmichaelclark/driftloom/pull/152), off by default
behind `?proto=`, in the JavaScript engine only. The queue and `main`
weren't touched. The full description is in the branch's `docs/EARS.md`.

- **Preview:** https://ears-prototypes-driftloom.nowmichaelclark.workers.dev/?proto=all
  (or `?proto=stereo,glass`, etc.)
- **Mikey's verdict** on the first five (stereo, roundBass, glass,
  floorDrop, arc), from before/after clips: "all the new stuff is sounding
  really great! We can tweak more later, but good enough for now."
  The last four (dubStab, organ, wash, longChords) went to him as clips the
  same night.
- **Checks:** with the switches off, renders null against `main` at -79 dB
  (main's own run-to-run noise), and `test/generator.test.mjs` passes.
- **Sequencing:** stereo, bass/presence and tails come after queue item 31
  (the master chain in Rust); build them in the core then, with this
  branch as the reference.

## The sketches and what they measured

| Name | What it does | Measured, before → after |
|---|---|---|
| `roundBass` | Bass register 36–57 instead of 28–52, never the extra octave down; soft 2nd, 3rd and 4th harmonics under a 900 Hz lowpass on every bass voice but `rhodesbass`, so the bass reads on a phone | "dust": 40–60 Hz 18% → 6% of the energy, 120–250 Hz 10% → 27% |
| `glass` | On a chord's first beat (55% of the time), two or three of its notes ring out in E5–G6 as a soft FM bell (ratio 3.5), a dotted eighth apart, into the texture channel | "thaw": 1–2 kHz 2.8% → 6.2% |
| `floorDrop` | Drift's "the whole thing stops" becomes kick and bass out for one or two bars while everything else plays on, at 0.35 a pass instead of 0.1; rests no longer fade the tails | "grove": floor drops 0 → 0.62 a minute (records: 0.42); full stops 0.62 → 0 a minute |
| `stereo` | Channels panned (chords -0.6, melody +0.5, texture -0.25, bass and drums in the middle), a ping-pong echo, reverb combs split left and right | "halcyon": side/mid -99 → -19 dB (records: -5 to -19) |
| `arc` | A second lowpass after the tone filter that breathes 300 Hz → 9 kHz → 300 Hz, lingering on the dark side, once every 120 s; echo feedback rises a little as it opens | "haven": -15 dB above 500 Hz at the closed points, unchanged when open |
| `dubStab` | After a chord's attack, a short sawtooth chord (G3–C5) through a 820 → 560 Hz bandpass, thrown hard into the echo, on the offbeat of beat one (60%) and of beat three (35%) | "undertow": the stabs and their echoes add -14 dB against the loop |
| `organ` | Soft drawbars (16', 8', 5 1/3', 4'), notes folded under G4, swelling in over 0.6 s and held until the chord changes, with a 5.4 Hz tremolo | "haven": -11 dB against the loop; 120–250 Hz 19% → 29% |
| `wash` | Two decorrelated noise loops through a bandpass sweeping 500 Hz → 2 kHz → 500 Hz, swelling in and out once every 20 s, into the texture channel | "vapor": swells peak about -10 dB against the loop |
| `longChords` | Each loop draws how long its chords last: as before (35%), twice that (30%), about 8 s (20%) or about 16 s (15%, which can hold one chord for the whole loop); three in ten shorten the progression's last chord (a turnaround). A stream of its own, so the other draws are unchanged | 400 loops, chord changes a minute (10th / median / 90th percentile): 11 / 26 / 47 → 1.4 / 14 / 37; loops under 2 a minute 2 → 42; one-chord loops 0 → 10% |

All five together, 150 s: "grove" side/mid -98 → -18 dB, drift 60/10
0.97 → 1.53, 1–5 kHz 0.2% → 1.5%, 40–60 Hz 36% → 3%.

All nine together, 150 s: "glade" side/mid -99 → -12 dB, chord changes
18.0 → 12.8 a minute, 1–5 kHz 0.2% → 1.7%, 40–60 Hz 24% → 4%; "vapor"
side/mid -79 → -2 dB (too wide: a sparse, wet loop is mostly reverb,
split left and right), chord changes 8.0 → 1.6 a minute, 1–5 kHz 0% → 14%.

`glass` scales itself down in loops with fewer than one chord a bar, so
the bells don't outweigh the chords they come from (in "vapor" they had
added 5.7 LU; now 1.3).


## Known rough edges, for the tweaks later

- On a sparse, wet loop ("vapor" with all nine), the stereo reverb makes it
  too wide (side/mid -2 dB, left/right correlation 0.25). Split the combs
  less hard, or narrow the reverb on sparse loops.
- Glass bells scale down when a loop has fewer than one chord a bar; the
  real build should set their level against the chord voice itself.
- The arc's period is 120 s for the demo; the app wants 2–5 minutes.
- The wash and organ levels were set by measurement, not by ear.
- `longChords` was first one length for every loop (about 8 s, and 48% of
  loops on a single chord); Mikey asked whether that cost variety, and it
  became a per-loop spread (2026-10-10).

## How the clips were made

`render-proto.mjs` (in `docs/ears/tools.md`): the batch-one renderer
plus `PROTO`, `PICK` and `TAG` environment variables. Each pair is the
same corpus loop (seed 20261009) rendered with and without a switch,
then loudness-matched to -16 LUFS and limited at -1 dBFS before MP3
encoding.
