# Ears

What the ears sessions found, and what they're for. An ears session
"listens" by measuring: it takes records Mikey loves and Driftloom renders,
measures both the same way, and reports where they differ, each
difference with a yardstick a hands session can test against. Mikey is
still the one who decides whether anything sounds good.

The full reports are in the Project knowledge: `claude/ears-batch-1.md`
(20 albums), `claude/ears-batch-2.md` (12 more), `claude/ears-batch-3.md`
and `claude/ears-batch-4.md` (the sketches and stereo, by round),
`claude/ears-sessions.md`
(how to run an ears session), `claude/ears-prototypes.md`, and the raw
measurements in `claude/ears-data.csv`. The tools are in `tools/ears/`.

## Who does what

- **Ears:** measure records and renders, find gaps, pick reference tracks,
  render before/after clips, and check changes by measurement. Findings go
  to the Project and here.
- **Brain:** turns findings into queue items and sequences them.
- **Hands:** build them, with the level, refusal and balance-lock checks.

The first ears session also built sketches into the app (draft PR #152,
`?proto=` on the `ears/prototypes` branch). The offline clips sounded right,
but in the live preview some of the old sounds went missing (not
diagnosed; the voice budget is the prime suspect). So ears sessions don't
build features. #152 stays unmerged, as ideas and a reference only.

## What 32 albums say, measured against Driftloom

Driftloom measured at `3149af1` (JavaScript engine, 80 loops of 60 s and
24 of 150 s, "Let the loop wander" on). Medians per file.

| Difference | Records (batch one / two) | Driftloom | Yardstick |
|---|---|---|---|
| **Stereo** | side/mid -7.9 / -9.0 dB; all 32 albums stereo | mono | side/mid -12 to -5 dB, channels positively correlated; bass and kick in the middle |
| **Low end** | drum tracks: 40–60 Hz 11% / 16%, 60–120 Hz 26% / 23% | drum loops 60–120 Hz 59%; drumless loops 40–60 Hz 37% | drum loops under ~35% at 60–120 Hz; drumless loops under ~15% at 40–60 Hz |
| **Upper presence** | 1–5 kHz 3.2% / 7.5% of the energy | 0.2% | 1–5 kHz at 1% or more in most loops (75% / 92% of record files are) |
| **Harmonic rhythm** | 2.4 / 4.3 chord changes a minute | 17.5 measured (≈29 true) | some loops under 2 a minute, a spread rather than one length |
| **Travel** | drift 60/10: 1.34 / 1.45; 32 of 32 albums above 1 | 0.96 (loops circle) | 1.2 or more on long renders: something moving slowly (a filter, echo feedback, a layer's level) over 2–5 minutes |
| **Tails** | -1.2 / -1.9 dB a second after a note | -4.5 to -8.8 | calm profiles toward 1–2 dB a second; dry stays possible (4 of 32 albums are) |
| **Pauses** | full stops 0.03 / 0.06 a minute; in 43% of beat tracks with a steady low end the kick and bass drop out for ~3 s while the rest plays (0.42 a minute) | full stops 1.11 a minute (60 s loops); floor drops almost never | some full stops become floor drops, with the tails left ringing |

**Stereo across the board, with a Mono switch (Mikey, 2026-10-15;
replaces "mono only" of 2026-10-11).** `docs/CORE.md`, promise 9. Ears
sessions propose stereo treatments as offline clips against today's mono
first; see "Stereo, measured" below.

A tuning difference was reported in batch one and corrected in batch
two: only 8 of 32 albums are clearly off A440, so it's optional at most.

Already in range: note density, repetition, pitch wobble, beat clarity.

## Ideas Mikey liked, as clips

Heard as before/after clips on 2026-10-10; "all the new stuff is
sounding really great". These are directions, not specs. Each needs
building properly, one at a time, with nothing lost:

stereo placement with a ping-pong echo (dropped 2026-10-11, back 2026-10-15 as space recipes); a rounder bass with harmonics
that reads on a phone; soft high bells from the chord's notes; floor
drops instead of full stops; a slow filter arc; dub chord stabs thrown
into the echo; a soft drawbar organ under held chords; a noise wash
every ~20 s; chord lengths drawn per loop from a spread.

Approved: roundBass, glass, floorDrop, arc. dubStab, organ, wash and
longChords went to Mikey again on 2026-10-15 (Ears 2), with two flags
from the measurements: the wash's fixed level is inaudible under a beat
(-28 dB at its peak), and the organ loses about half its chords to the
voice budget in a dense loop. Verdicts: `claude/ears-batch-3.md`.

### The tune round (Ears 4, 2026-10-15)

Each of the four reworked from Mikey's notes (sketch:
`tools/ears/sketches/ears4-sketch.patch`; report and verdicts:
`claude/ears-batch-4.md`). Checked on nine loops at 60 s, four at 150 s
and four that load a second loop partway through: **no sketch refused a
note, its own or any other part's**, and every layer's own level was
unchanged (per-layer stems, `sketch_check.py`).

| Sketch | What changed | Measured |
|---|---|---|
| dubStab | How often drawn per loop (rare, sparse, some; busy only on busy loops), only over a groove; a triangle and soft saw through a closing lowpass, a slower attack, a darker echo; asks the budget for room and holds none | Centroid 320–338 Hz (round 1: 740–860; "Quadrant Dub I": 377); attack 13–18 ms (round 1: 1–3; the record: 12); adds -11 to -20 dB against the loop |
| organ | One instrument per loop outside the voice budget: a registration from nine, a slow rotary-style chorus, a little drive, a swell, a key click in some loops; lets go in full stops | Zero refusals (round 1: 5–24 of its chords per loop, and other parts' notes); -12 to -20 dB against the loop |
| wash | Its level follows the loop's own layers, aimed 9–13 dB under | Loudest 2 s window under a beat -8 dB (round 1: -22) |
| longChords | An option: today's lengths 56%, twice 16%, 8 s 10%, 16 s 9%, the whole loop 9%; calm loops lean long, driving ones away | Median chord changes a minute 22.5 → 19.4 (round 1: 16.4); one-chord loops 11 → 31 of 400 |

For the core, if approved: the organ is fixed overhead outside the voice
budget (reserving room for it inside would refuse other notes: 4–40 more
per loop); the stab is a budget probe that holds nothing.

## Stereo, measured (Ears 2, 2026-10-15)

**The goal (Mikey, 2026-10-15):** stereo becomes the default once the
brain and hands build it, but only once it's tuned: it should sound
better than mono about 95% of the time, and the default settings should
always sound better. In round 1 (below) mono won on every loop, judged on
Koss KPH30 wireless headphones (he can also check on earbuds), so stereo
needs more rounds of clips and listening before any default is chosen.
Mikey's notes and the next round's directions: `claude/ears-batch-3.md`.
The sketch used for the clips: `tools/ears/sketches/space-sketch.patch`
(a local, offline sketch of `js/synth.js`; never app code).

Three space recipes (room; wide; wide + drifting), sketched on a local
copy of the JavaScript synth so that (L + R) / 2 before the master chain
is exactly the mono mix: placement only above 150 Hz, width from a side-
only chorus above 200 Hz, the reverb and echo split into mid and side,
and a width ceiling (side turned down above -7 dB against the mid).
Twelve loops, 60 s, against their mono renders (`tools/ears/stereo_guard.py`):

| Guard | Proposed limit | Wide, measured (worst of 12) |
|---|---|---|
| Fold to mono: loudness | within ±0.5 LU of the mono render | 0.18 LU |
| Fold to mono: tone | every octave 63 Hz–8 kHz within ±1 dB | 0.5 dB |
| Low end centered | side 20 dB or more under mid below 120 Hz | -20.7 dB or lower, except one loop with almost no low end (-19.8) |
| Width | side/mid above 200 Hz -14 to -6 dB for most loops; ceiling -7 | -15.6 to -7.5 (median -10.1) |
| Nothing essential on one side | correlation median 0.7+, 5th percentile 0.3+; balance within ±2 dB | 0.77, 0.45, -2.9 dB |
| Cost | under 2× mono | 1.14× median (JavaScript render time; the core measures its own) |
| Notes | refused notes same as mono | same in all 12 |

Whole-mix side/mid reads lower than records (-18 against -8/-9) because
Driftloom's energy sits in the centered bass; above 200 Hz it matches.
Recipes, clips and the sketch: `claude/ears-batch-3.md` and
`claude/ears-space-sketch.md` in the Project.

### Round 2 (Ears 4, 2026-10-15)

From Mikey's round 1 notes (mono won every loop; bouncing and a heavy
part leaning to one ear annoyed; sparse loops wanted no width; drifting
could do more): the ping-pong narrowed to 0.15–0.3 and the light kit
placed as one, by 0.08–0.15; chords, pads, melody and bass never placed,
only widened symmetrically; every width scaled by how busy and how wet the
loop is, with a tighter ceiling on sparse loops; and a slow breath on all
the side sends (60–140 s) with light parts crossing over 30–60 s. Also
rendered 1.8 times wider (exact: every move is linear in its width).
Twelve loops, eight at 150 s (`stereo_guard.py`, which now also measures
bounce and slow drift):

| Measure (median) | Round 1 wide | Round 2 | Round 2 wider |
|---|---|---|---|
| Side/mid above 200 Hz | -10.1 dB | -19.3 dB | -14.3 dB |
| Correlation, 5th percentile | 0.68 | 0.96 | 0.88 |
| Bounce (image jump per 100 ms, 90th pct) | 4.0 dB | 1.8 dB | 2.8 dB |
| Balance travel over 4 s windows | 3.1 dB | 0.6 dB | 1.0 dB |

Every guard passes in both: fold within 0.10 LU and 0.33 dB per octave,
side 23.7 dB or more under mid below 120 Hz, balance within 0.26 dB, the
same refused notes as mono. Mikey's verdicts: `claude/ears-batch-4.md`.

## The album list (2026-10-15)

Mikey's full list is 73 records, plus everything Aphex Twin released
between *SAW 85-92* and *Drukqs*; he wants sounds, techniques and theory
from every one, harsh records included (wholesome takes only), so that a
listener sometimes hears "that album" in a loop. Rules for what comes
out of it: never real words in voices; no quotes, public domain
included; new sounds join existing vibes by weight, harsher ones sit
behind particular vibes (rare elsewhere, never banned), and open vibes
like "zany" take almost anything. The plan, the album card and the
session split: `claude/ears-plan.md`.

## Reference tracks

In Mikey's Drive (the ears batches). Measure each new sound against its
reference when it's built.

| Sound or move | Reference |
|---|---|
| Slow arc | Basic Channel, "Quadrant Dub I"; The Field, "Over the Ice" |
| Floor drop | LFO, "LFO"; Autechre, "nil"; The Field, "Over the Ice" |
| Dub stab and echo | Basic Channel, "Quadrant Dub I"; Porter Ricks, "Port Gentil" |
| Bells | Autechre, "nil" |
| Organ, long chords | Oneohtrix Point Never, "Boring Angel"; Harmonia, "Dino" |
| Wash | Porter Ricks, "Port Gentil" |
| Bright top end | Ryuichi Sakamoto, *Thousand Knives*; Iasos; Lone, *Galaxy Garden* |
| Bass that slides | Forest Swords, "Thor's Stone" |
