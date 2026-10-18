# Ears

What the ears sessions found, and what they're for. An ears session
"listens" by measuring: it takes records Mikey loves and Driftloom renders,
measures both the same way, and reports where they differ, each
difference with a yardstick a hands session can test against. Mikey is
still the one who decides whether anything sounds good.

The full reports are in `docs/ears/` (index: `docs/ears/README.md`):
`batch-1.md` (20 albums), `batch-2.md` (12 more), `batch-3.md` and
`batch-4.md` (the sketches and stereo, by round), `sessions.md` (how to run
an ears session, and the rule that ears outputs live in the repo),
`plan.md`, `cards-pile-1.md` and `cards-test.md` (album cards),
`prototypes.md`, and the raw measurements in `data.csv`. The tools are in
`tools/ears/`.

## Who does what

- **Ears:** measure records and renders, find gaps, pick reference tracks,
  render before/after clips, and check changes by measurement. Findings go
  to `docs/ears/` and here, in a PR, the same session.
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

Approved: roundBass, glass, floorDrop, arc; **dubStab and longChords
(2026-10-16, as in Ears 4's round 4)**; **stereo's baseline** (round 4's
blend of room and ensemble, steady); **wash** (2026-10-16, round 5);
**the chapel organ** (2026-10-16, round 7: one organ, mixed 55% chapel,
25% round 5's pipe, 10% distant cathedral, 10% celeste; it holds through
passing chords, moves at most once a bar, dips 800 Hz, and sits outside
the voice budget). Every Ears 4 sketch is approved; the recipes are in
`docs/ears/batch-4.md` and `tools/ears/sketches/ears4-sketch.patch`. dubStab, organ, wash and
longChords went to Mikey again on 2026-10-15 (Ears 2), with two flags
from the measurements: the wash's fixed level is inaudible under a beat
(-28 dB at its peak), and the organ loses about half its chords to the
voice budget in a dense loop. Verdicts: `docs/ears/batch-3.md`.

### The tune round (Ears 4, 2026-10-15)

Each of the four reworked from Mikey's notes (sketch:
`tools/ears/sketches/ears4-sketch.patch`; report and verdicts:
`docs/ears/batch-4.md`). Checked on nine loops at 60 s, four at 150 s
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

Mikey's verdicts on round 2 (2026-10-16): longChords' spread approved;
the rest tune. Round 3, same session: the organ's "broken" low notes were
clipping (four voices summed into a drive with no headroom; fixed, the
distortion under the notes from -19 to -40 dB); the stab plays only on
busy loops, softer and blooming; the wash 8 dB lower (round 1's level on a
sparse loop, 5 dB above it under a beat); held chords trimmed by up to 2
dB. Lesson for any sketch with a drive or shaper: a WaveShaper clamps its
input at ±1, so check what reaches it.

Round 3 verdicts: the stab much better but too full; the organ's and the
wash's texture still wrong (the wash harsh, heavy, in front); held chords
still rough or loud. Round 4: the held chords' roughness was **an octave
jump**, not the master (nothing past 0.84 reaches its saturator):
`separateRegisters` drops the keys an octave when their centre is within
five semitones of the melody's, and holding chords moved that centre
across the line in 11 of 87 held loops; held chords now take today's
octave (0 move). The stab is about 5 LU quieter and shorter; the organ
soft registrations only, no drive, darker, swelling, behind the loop; the
wash a gentle lowpass (350 Hz → 1.3 kHz) instead of a bandpass hiss, about
2 dB under round 1 on a sparse loop. A finding for the generator whatever
the sketches' fate: any change that moves the chord centre can flip that
octave decision, so it wants hysteresis or a decision from the
progression rather than the events. Written into `docs/BRAIN.md` and
the charter's phase 2 (`docs/CORE.md`) at Mikey's request.

Round 4 verdicts: dubStab and longChords approved; the organ wants
better articulation, the wash its brightness back and some color. Round
5: the organ keeps a note two chords share sounding through the change
and speaks only the notes that change, in two flavors after real organs
(tonewheel: instant speech and a soft percussion partial; pipe: a slower
speech with a chiff and a celeste rank); the wash opens to about 2.5 kHz
again and takes two soft resonances tuned to the loop's key (centroid 0.9
kHz; round 1 1.9, round 4 0.5).

## Stereo, measured (Ears 2, 2026-10-15)

**The goal (Mikey, 2026-10-15):** stereo becomes the default once the
brain and hands build it, but only once it's tuned: it should sound
better than mono about 95% of the time, and the default settings should
always sound better. In round 1 (below) mono won on every loop, judged on
Koss KPH30 wireless headphones (he can also check on earbuds), so stereo
needs more rounds of clips and listening before any default is chosen.
Mikey's notes and the next round's directions: `docs/ears/batch-3.md`.
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
Recipes, clips and the sketch: `docs/ears/batch-3.md` and
`docs/ears/space-sketch.md`.

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
same refused notes as mono. Mikey's verdicts: `docs/ears/batch-4.md`.

Verdict (2026-10-16): round 2 about matches mono ("not better, not
worse"; a much closer call than round 1); the long drifting version didn't
work in practice. Round 3 holds the width at round 2's and moves where it
lives: **steady** (round 2 held still), **room** (dry parts centered, a
smoother allpass-decorrelated reverb side carrying all the width) and
**ensemble** (chords and texture spread, no room width). All guards pass.

Round 3 verdict: room and ensemble both liked, leaning to room. Round 4
blends the two at the same width (room weighted 0.64, ensemble 0.5;
`blend_side.py`): bounce 1.21 dB, between room (1.02) and ensemble (1.82),
every guard passing. **Approved as the baseline (Mikey, 2026-10-16).**
Stereo that moves (panning, drifting) is kept as a later option for
users and developers, in a session of its own; the default stays
steady.

## Album-card tools (Ears 3, 2026-10-15)

`tools/ears/album.py` turns one album into the plan's album card:
it picks tracks, splits them into stems, and measures sounds, techniques,
theory and space, then drafts the card with numbers and timestamps filled
in. Judgment lines (hook, vibes, wholesome take, why a thing waits) are
marked **[JUDGMENT]** for a person or a later session. Test cards for
*Geogaddi* and *Galaxy Garden*: `docs/ears/cards-test.md`; the pile 1 cards: `docs/ears/cards-pile-1.md`.

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

**Cost of one album** (2 CPU cores; five pile 1 albums, Ears 5): about 40
minutes of machine time for 3 or 4 deep-pass tracks (28 to 43 minutes; 11,582 s
over five albums), which is about **1.6 seconds per second of deep-pass
audio plus 25 seconds per track in the album**. The parts: picking
(mix-level numbers for every track) 22 to 26 s a track (an album of 27 tracks
takes 11.6 minutes); Demucs 1.0 to 1.1 times the audio length (1.42 once,
while other jobs shared the cores); the measuring stages about 3 minutes a
track (134 to 226 s; across 19 runs events 6 to 184 s, theory 11 to 82 s,
techniques 6 to 39 s, space 3 to 66 s) or 32 to 37 s per audio minute. Ears 6
(six pile 2 albums, 2,602 to 4,434 s each) had no Demucs: its weights were
blocked, so `split.py` used MDX-Net, 2.3 to 2.8 times the audio length, and the
formula came out 1.56 to 1.90 times low; with Demucs the formula is the
figure to use, with MDX-Net multiply by about 1.7. Demucs became reachable again on 2026-10-18; a 90 s clip took 0.7 times its length on 2 cores
(a 30 s clip 2.9 times, with the weights downloading). MDX-Net stems are weaker:
the drums stem holds 60 to 93% of the sub, the bass stem is empty or thin
wherever kick and bass share it (`album.py` now runs `check_stems.py` and the card opens with
its flags; read them before trusting a bass number), and the vocals stem on an instrumental is bleed. A four-track album is
run in the background and polled with sleeps (a shell command stops after
10 minutes). Disk: about 1 GB for Demucs, ~160 MB of stems per track.

**What the Ears 5 cards found beyond the table** (`tools/ears/check_stems.py`
checks most of these before a card is written):

- The separator moves the sub: on several tracks the whole low end is in the
  drums stem (On's D-Scape, Classics' Phlange Phace) or the drums stem holds
  the whole track (Quoth, 99.8%). A bass stem under about 2% of the power
  makes bass behavior, bass pumping and bass wobble void, and floor drops
  counted from drums and bass alone come out too small (Volume II track 15:
  one drop of 2.8 s reported against 44 s found with the stems summed).
- A track with empty drums and bass stems has no beat: its meter, swing and
  chord-on-beat numbers mean nothing.
- A note group whose register is a half or a third of the strongest spectral
  line is a subharmonic misread (Volume II track 21: 37 Hz against a 111.7 Hz
  line). Partials under about 60 Hz cannot be measured: ignore "bell-like"
  labels there; pitch under 40 Hz is good to about a semitone.
- A flat-topped gated note reads its flat-top length as "attack" (Quoth's
  sub: 187 ms against a real rise of about 30 ms).
- Tempo: the meter tool reports half-time (72 for a 144 BPM grid); the
  picker's mix-level tempo can be off by a factor of 4/5 on broken-beat
  tracks (112.3 for 139, 117.5 for 145). The drum-onset repeat and echo times
  settle it (`check_stems.py`).
- "Kick every x s" that does not fit the beat means the kick finder locked on
  something else; do not trust the pumping depth tied to it.
- A pan or stereo cycle of exactly 3 to 4 s shared by several parts looks like
  the analysis window, not an auto-pan.
- The vocals stem on an instrumental is the separator's guess at voice-like
  sound; a listen decides whether any voice is in it.
- The tuning reading (mix level) ran from -48 to +45 cents across the pile; a
  half-semitone offset splits a note's weight over two neighboring pitch
  classes, so key margins and chord names on those tracks are a guess. Check
  it against a direct line (a kick's Hz) when the two disagree.
- The vibes line's "energy above 2 kHz" can disagree with the direct spectrum
  (Classics' Phlange Phace: 39% against 21%); use the picker's band shares or
  the stem sum.

## The album list (2026-10-15)

Mikey's full list is 73 records, plus everything Aphex Twin released
between *SAW 85-92* and *Drukqs*; he wants sounds, techniques and theory
from every one, harsh records included (wholesome takes only), so that a
listener sometimes hears "that album" in a loop. Rules for what comes
out of it: never real words in voices; no quotes, public domain
included; new sounds join existing vibes by weight, harsher ones sit
behind particular vibes (rare elsewhere, never banned), and open vibes
like "zany" take almost anything. The plan, the album card and the
session split: `docs/ears/plan.md`.

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
