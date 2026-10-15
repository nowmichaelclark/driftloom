# Ears

What the ears sessions found, and what they're for. An ears session
"listens" by measuring: it takes records Mikey loves and Driftloom renders,
measures both the same way, and reports where they differ, each
difference with a yardstick a hands session can test against. Mikey is
still the one who decides whether anything sounds good.

The full reports are in the Project knowledge: `claude/ears-batch-1.md`
(20 albums), `claude/ears-batch-2.md` (12 more), `claude/ears-sessions.md`
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

## Album-card tools (Ears 3, 2026-10-15)

`tools/ears/album.py` turns one album into the plan's album card:
it picks tracks, splits them into stems, and measures sounds, techniques,
theory and space, then drafts the card with numbers and timestamps filled
in. Judgment lines (hook, vibes, wholesome take, why a thing waits) are
marked **[JUDGMENT]** for a person or a later session. Test cards for
*Geogaddi* and *Galaxy Garden*: `claude/ears-cards-test.md`.

Every measure was run on test signals with known answers
(`python3 -I calibrate.py`, about 80 s): **163 of 163 graded checks within
tolerance; 10 further rows show known limits** (reported, not graded).

| Measure | Tested on | Worst graded error | Stops being reliable |
|---|---|---|---|
| Note groups (events) | strings, bells, ramps, vibrato, detune, pan, levels | f0 within 0.2 cents; attack within 4 ms (reads a little long); decay within 0.02 dB/s; vibrato within 0.8 cents and 0.04 Hz; detune within 0.4 cents; pan and level exact | groups are candidates, not instruments; attacks under ~5 ms read as about 9 ms |
| Sections, stem entries | sections of 20 to 40 s and 10 to 14 s, chord-only changes with 4-s crossfades; stems gated in and out | all boundaries found, timing error 0 s; stem edges within 0.5 s | sections under ~10 s |
| Sweeps, echo, pumping, wobble, pan | cutoff sweeps, echoes, ducked pads, wow, panned stems | sweep 4.3 oct read 4.2; echo times within 4 ms; 12-cent wobble read 13.7 and rate exact; pan range within 0.05 | pumping reads about 70% of the true depth (a floor); echo needs 6+ notes ringing alone for 1.5 s; sweeps under 0.7 octave not reported |
| Floor drops (from stems) | gated kick and bass | edges exact (0 s) | drops under 2 s not counted; skipped where the low end is on under 75% of the time |
| Tempo | 6 tempos, 72 to 160 BPM | within 0.04 BPM, within a factor of 2 on 6 of 6; the exact octave 4 of 6 | octave ambiguity (reported as half and double) |
| Bar length, swing, cross-rhythm | accented bars, swung grooves, 4:3 and 3:2 | swing within 0.01 of the beat | bar length needs an accent; the score says when it is unclear |
| Key and mode | 7 modes x 3 keys | mode 19 of 21 without bass, 20 of 21 with; tonic 20 of 21 and 21 of 21 | margin under 0.02 is a guess; pentatonic fits are flagged |
| Chords | clean chord sequences, dry and with 1.5 s reverb | change count exact down to a chord every 0.5 s dry and every 1 s with reverb; labels 100% for triads, 88% sevenths and sus | faster than 0.5 s (0.35 s reads 127 for 170 a minute); chord names on dense or noisy music are a guess |
| Bass behavior | pedal, ostinato, walking | 100% classified; pitch within 0.01 semitone | needs a bass stem with notes |
| Top line | melody alone, over a pad 12 dB down, and at equal level | notes found with the right pitch 85% alone, 90% over a quiet pad; median interval exact; contour (arch, rising, falling) right | only when the line is clear of its accompaniment (pad at equal level: 31% found, step share 0.19 for 0.28) |
| Form and repetition | repeated phrases | period found; bars counted exactly | the bar length must be within ~1% (5% off loses the lags) |
| Space | independent, mono and half-correlated channels by band | width within 0.1 dB, mono loss 3.01 for 3 dB | a quiet band next to a much louder wide band leaks (-40 dB reads -1); `share_of_stem` says when to distrust |

*Real music has no truth to check against.* For chord changes the card
prints the share that land on a beat next to chance; for the rest, listen
at the timestamps.

**Against Ears 1:** Galaxy Garden floor drops agree in kind (Crystal
Caverns is the drop track, two tracks too sparse), though the counts
differ (2.74 against 4.87 a minute) because stems replace a 30 to 120 Hz
band. Geogaddi's chord changes do not match: the new counter reads 16 to
28 a minute (11 to 20 settled) against Ears 1's "about 10". It sees
movement inside slow pads that the 2-second window skipped, and the
on-beat shares support it on two of four tracks. Treat chord counts from
Ears 1 as floors; do not mix the two without saying which.

**Cost of one album** (2 CPU cores): picking 19 tracks about 3 minutes;
Demucs about 1.0 to 1.3 times the audio length (4 tracks of ~4.5 minutes
is ~20 minutes; it ran at 0.7 when other jobs shared the cores); the
measuring stages about 2.5 minutes a track (events 45 s, theory 30 s,
techniques 15 s, space 10 s on a 4-minute track). A four-track album is
about 30 minutes of machine time, run in the background, polled with
sleeps. Disk: about 1 GB for Demucs, ~160 MB of stems per track.

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
