# Ears batch 3: the waiting sounds, and stereo against mono

Ears 2 (Opus 5.5), 2026-10-15. Renders of today's `main` (01bce2b) and
of the `ears/prototypes` branch, JavaScript engine, "Let the loop wander"
on, corpus seed 20261009. Clips went to Mikey in the Ears 2 chat.

## Verdicts

**Approved:** roundBass, glass, floorDrop, arc (Mikey, 2026-10-10;
unchanged).

**Tune** (Mikey, 2026-10-15, from the clips):

- **dubStab:** "definitely a bit much". Far too many stabs: it should
  usually be sparse, and only now and then this busy, on a busier loop.
  It sounds grating, "like nails on a chalkboard": it pierces the ears
  where it should tingle or excite them. It's missing roundness and
  warmth.
- **organ:** mostly nice, but a bit bland: it could use more character.
  Notes must not be refused, neither the organ's own nor other parts'
  because of it.
- **wash:** "I dig it", used sparingly as it is. One change: set its level
  against the loop (it vanished under a beat).
- **longChords:** worked in grove, not in wayfare, so it's an option, not
  the default. Short chords stay; very long chords (one held for the
  whole loop) are worth hearing next.
- **stereo (room, wide, wide + drifting):** mono sounded best on every
  loop. glade: all three okay, but too much bouncing around. vapor: none
  sounded good. cinder: wide the best of the three. undertow: wide and
  wide + drifting decent, and drifting could have done more. Drifting
  in general could do more without being overdone. With wide, a heavy
  sound leaning too far toward one ear gets annoying.

**Dropped:** none.

**The stereo goal (Mikey, 2026-10-15):** stereo becomes the default once
it's built, but only after it's tuned to sound better than mono about 95%
of the time, with default settings that always sound better. Expect
several rounds. Heard on Koss KPH30 wireless headphones; he can also
check on earbuds, so play each round on both.

### What the ears take from the verdicts (for the tune round)

- **dubStab:** draw how often it plays per loop, sparse by default
  (a stab every few bars), busy only in a small share of busy loops. A
  rounder source: a filtered saw blended with a triangle or sine, a
  softer attack, the band lower and less sharp, the echo darker; check it
  against "Quadrant Dub I" for warmth, not just level.
- **organ:** character from a per-loop registration (which drawbars),
  a slow rotary-style chorus, a little drive, maybe a soft key click; and
  a cost that can't refuse notes (one cheaper voice per chord, or room
  reserved for it at load, with the organ the one that yields).
- **wash:** level drawn against the loop's own loudness.
- **longChords:** one draw among several, today's lengths the most common;
  vibes like floating, still and meditative lean toward the long ones;
  include whole-loop holds in the clips.
- **stereo, round 2:** the bouncing is mostly the ping-pong echo and the
  per-hit hat and shaker placement, so narrow both. Keep heavy parts
  (chords, pads) centered and widen them symmetrically instead of
  placing them; place only light parts (bells, hats, sparkles), by less.
  Sparse, wet loops stay near mono. Drifting gets more to it but slowly:
  width that breathes with the arc, a light part crossing over 30–60 s,
  the room opening as the filter opens.

The ears flag two things whatever the verdict (measurements, not taste):
the **wash** has a fixed level, so it peaks 7 dB under a sparse loop and
28 dB under a beat, where it can't be heard (it needs a level relative to
the loop). In a dense loop the **organ** loses about half its chords to
the voice budget, and in a calm loop it cost one note of another part
(an FM voice: haven's piano or bells; the trace doesn't say which).

## The waiting sounds

Each clip plays loop A before and after, then loop B before and after,
every segment matched to -18 LUFS. "Added" is the energy of (after −
before) against the loop.

| Sound | Loops | Added | Bands that moved | Refused notes (before → after) |
|---|---|---|---|---|
| dubStab | undertow #1, grove #31 | -8.9, -8.3 dB | 1–5 kHz 0.0 → 1.8% and 0.2 → 2.5%; 250–1k 14.8 → 23.3% and 2.8 → 13.1% | 0 → 0, 0 → 0 |
| organ | haven #38, glade #21 | -10.8, -18.8 dB | haven 120–250 Hz 20.4 → 27.6% | haven 0 → 3 (2 organ, 1 other FM-voice note); glade 15 → 28 (13 of 25 organ chords refused) |
| wash | vapor #4, grove #62 | peaks -7 dB (vapor), -28 dB (grove) | none worth naming | unchanged |
| longChords | grove #47, wayfare #25 | (a different loop) | chord changes a minute 19.7 → 3.3, 20.6 → 3.4 (records: 2.4 / 4.3) | unchanged |

Across the 80-loop corpus, longChords moves the median from 26 to 16
chord changes a minute (generator count, not audio).

Both the organ and the wash keep sounding through a loop's full stops:
in haven the organ holds through the rest about 6 s in (where the
original is silent), and in grove the wash plays on through the stop
about 26 s in. Whether that's welcome is a listening question; it changes
what a full stop is.

## Stereo: three space recipes

Sketched in a local copy of today's JavaScript synth (never pushed; the
diff is in `docs/ears/space-sketch.md`). Every stereo move is built so
that (L + R) / 2 before the master chain **is** today's mono mix:

- **Placement** is a balance above 150 Hz: L = part + p·HP(part),
  R = part − p·HP(part). Low notes stay centered by construction.
- **Spread** (width without a pan) adds a slowly swaying 9–15 ms copy of
  a part, above 200 Hz, as side only (+ left, − right).
- **Room:** the reverb's side is even combs minus odd combs; the mid is
  today's reverb. Above 180 Hz only.
- **Echo:** two delays in the loop, so odd repeats and even repeats come
  apart. Mid = today's echo; side = their difference, a ping-pong.
- **Width ceiling** (sketched offline on the renders, a block for the
  core): the side is turned down wherever it rises above -7 dB against
  the mid (above 200 Hz, 2-second windows; down in about 0.5 s, back over
  about 3 s). The mid is untouched, so the fold doesn't change.

Recipes (ranges; each loop draws within them from its seed):

| Recipe | What it draws |
|---|---|
| **room** | reverb width 0.5–0.9, echo width 0.5–0.9; everything else centered |
| **wide** | reverb 0.8–1.0, echo 0.8–1.0; chords placed 0.3–0.55 to one side; texture 0.25–0.45 to the other; melody within 0.12 of the middle; hats 0.25–0.45, shakers and jingles 0.3–0.5 opposite, taps and rims within 0.4; spread: chords 0.3–0.5, texture 0.4–0.7, melody 0–0.2; bass, kick, snare, clap and frame drum centered; width ceiling on |
| **wide + drifting** | wide, plus the texture sways 0.2–0.4 (period 16–48 s); half the loops also sway the chords 0.1–0.25 (period 24–64 s) |

### Guards, measured (12 loops, 60 s each, against their mono renders)

Ranges are min / median / max across the 12 loops. "Below 120 Hz" uses a
steep zero-phase filter; a gentler one let side just above the cutoff
leak in and read -16 dB in two loops.

| Measure | room | wide | wide + drifting | Proposed limit |
|---|---|---|---|---|
| Fold loudness vs mono (LU) | -0.01 / 0.00 / 0.18 | -0.03 / -0.01 / 0.18 | -0.04 / -0.01 / 0.10 | within ±0.5 |
| Worst octave, 63 Hz–8 kHz, fold vs mono (dB) | 0.50 | 0.50 | 0.27 | within ±1 |
| Side/mid above 200 Hz (dB) | -20.5 / -17.7 / -12.2 | -15.6 / -10.1 / -7.5 | -15.5 / -10.2 / -7.5 | -14 to -6 for most loops; ceiling -7 |
| Side/mid, whole mix (dB) | -30.5 / -25.6 / -15.4 | -24.1 / -18.1 / -9.3 | -24.1 / -18.0 / -9.3 | (records: -7.9 / -9.0; see note) |
| Side/mid below 120 Hz (dB) | -65 / -37 / -28 | -57 / -33 / -20 | -58 / -33 / -20 | -20 or lower |
| L–R correlation, median | 0.89 / 0.99 / 1.00 | 0.77 / 0.96 / 0.99 | 0.77 / 0.96 / 0.99 | 0.7 or more |
| L–R correlation, 5th percentile | -0.27 / 0.83 / 0.97 | 0.45 / 0.71 / 0.88 | 0.39 / 0.70 / 0.88 | 0.3 or more |
| Left/right balance (dB) | -0.4 / 0.0 / 0.2 | -2.9 / 0.0 / 0.5 | -2.9 / 0.0 / 0.7 | within ±2 |
| Render time vs mono (JS, rough) | 0.92 / 1.01 / 1.19 | 1.01 / 1.14 / 1.60 | 1.03 / 1.16 / 1.37 | under 2× (core to measure its own) |
| Refused notes | same as mono in all 12 | same | same | same as mono |

Notes:

- **Whole-mix width reads low** because Driftloom's energy sits in the
  bass (60–120 Hz: 59% in drum loops), which stays centered. Above 200
  Hz, wide matches records. Once roundBass moves bass energy up, the
  whole-mix figure will rise on its own.
- **The outliers are vapor #30** (a sparse loop that's mostly one pad):
  without the ceiling, wide reached side/mid -2.2 dB, correlation 0.22
  and a 5 dB lean to one side. The ceiling brought it to -9.3 dB, 0.79
  and 2.9 dB. It has almost no low end (31 dB under the rest of the
  mix), so its -19.8 dB below 120 Hz is the worst figure there but means
  little; every other loop reads -20.7 dB or lower. Its room-only
  render has a 5th-percentile correlation of -0.27 (moments that are
  mostly reverb). Two rules for
  the recipes: **the ceiling stays on**, and **the dominant part is
  placed least** (a part carrying most of the loop shifts the whole
  image when it moves).
- Fold residual (the difference that remains, not just loudness): -20 to
  -41 dB against mono. It comes from the saturator and compressors acting
  on each side; the mix going into them folds exactly.
- **Not checked yet:** the Rust core (these are JavaScript renders), a
  phone's cost, long listens and rerolls (60 s renders only), and a
  blind A/B against mono for sounds Mikey already likes (the brain's
  call, per promise 9).

### For the core (if approved)

All of it is data, per promise 9: per-part placement and spread, reverb
and echo width, movement (depth, period, which parts), the ceiling's
target. The building blocks it needs: a mid/side pair, a high-passed
side send, a two-tap echo loop, a chorus delay, and a side gain driven
by a slow side/mid follower (the ceiling). Mono render: all side gains at
zero.

## Side note for the brain

In offline renders of today's `main` (60 s), several loops have many
notes refused by the voice budget, identically in mono and stereo:
wayfare #23 367 of 1,124 (all `fm`), undertow #45 166 of 819, undertow
#8 154 of 527 (`voice`), hollow #73 67 of 464. Ears 2 didn't check
whether the app refuses the same notes or whether it's the offline
harness; worth one look before the instrument batch adds layers.

## Files

Tools written in Ears 2, for `tools/ears/`: `render2.mjs` (job
lists, PROTO and SPACE per job, refused notes per voice, chord changes,
render time), `clips.py` (before/after clips and numbers),
`space_jobs.py` (recipe draws per loop), `stereo_guard.py` (the table
above), `width_ceiling.py`, `stereo_clips.py`. They and the `docs/EARS.md`
update are in PR #219 (branch `ears-2-stereo-plan`), for Mikey to merge.
