# Ears batch 4: the tune round

Ears 4 (Opus 5.5), 2026-10-15 to 16, seven rounds. Each sketch from batch 3 reworked from
Mikey's notes, on today's `main` (a23153d), in a local copy only. The
sketch is `tools/ears/sketches/ears4-sketch.patch` (never app code).
JavaScript engine, "Let the loop wander" on, corpus seed 20261009. Clips
went to Mikey in the Ears 4 chat, every segment matched to -18 LUFS.

## Verdicts

**Approved:**

- roundBass, glass, floorDrop, arc (2026-10-10; unchanged).
- **dubStab** as in round 4 ("sounding pretty great now"; good enough
  for now, tunable later like every instrument). Mikey, 2026-10-16.
- **longChords** as in round 4 (the spread, the held-chord trim, tied
  pads, and held chords kept in today's octave). Mikey, 2026-10-16.
- **wash** as in round 5 ("good enough for now"). Mikey, 2026-10-16.
- **Stereo's baseline:** round 4's blend of room and ensemble, steady
  ("sounds much nicer"). Moving, experimental stereo kept as a later
  option for users and developers, in a session of its own. Mikey,
  2026-10-16.
- **The chapel organ** (round 7): one organ, mixed 55% chapel, 25%
  round 5's pipe, 10% distant cathedral, 10% celeste ("good enough");
  `organVariant: 'chapelOrgan'`. Mikey, 2026-10-16.

**Tune:** none (every instrument can be tuned again later).

**Dropped:** none.

**For the brain (Mikey asked):** the chords' octave flip found in round
4 is written into `docs/BRAIN.md` ("Start here") and the charter's phase
2 (`docs/CORE.md`), so the Rust generator is built without it.

### Earlier verdicts

**Round 2 (Mikey, 2026-10-16):** longChords' spread approved; the rest
tune.

- **dubStab:** only the busy wayfare #23 stab sounded good, and even
  that could be more pleasant; everywhere else "like knocking on a steel
  roof", disorienting and headache-inducing.
- **organ:** haven #38's (hollow registration) "really nice and cool";
  the low notes elsewhere sounded "almost like the note was clipping",
  almost broken.
- **wash:** far too loud in every tuned clip; it takes over the music.
  (Round 1's level was liked except under a beat.)
- **longChords:** sounding better; a long-held chord sometimes gets a
  bit too loud.
- **stereo round 2:** about matches mono ("not better, not worse"; a
  much closer call than round 1); mono still generally wins. The long
  drifting clip (grove #62) is a good idea that doesn't work in practice.
  Stereo needs a round 3 and maybe a round 4.

**Rounds 4–6 (Mikey, 2026-10-16), the organ:** round 4 still needed
better articulation (inspiration from other organs welcome); round 5
leaned to the pipe over the tonewheel, but at times too busy, not
enveloping, burying other sounds without an organ's power; round 6's
variations led to round 7's mix. The wash: round 4 lost brightness and
could use color; round 5 approved.

**Round 3 (Mikey, 2026-10-16):**

- **dubStab:** much better; a bit too full, it dominates the loop
  instead of blending in. Tweak, don't overhaul.
- **organ:** the texture still isn't right, and it dominates the
  background instead of livening it. Haven #38 the best, but even there
  improvable. Softer without losing presence, if possible.
- **wash:** the texture is too harsh and heavy, still too loud or too far
  in front; it should feel like soft waves or wind, gently behind.
- **longChords:** held chords can still sound clipped or too loud, or
  rough; they take over instead of blending in (not dust #51's level, but
  its texture was rough too).
- **stereo:** room and ensemble both liked, leaning slightly to room; asks
  for a happy medium between the two. Panning and drifting ("experimental"
  stereo) isn't thrown away, but needs careful engineering and probably a
  session of its own.

## What changed, and what was measured

Checks: nine loops at 60 s (busy and sparse, with and without drums),
four at 150 s, and four renders that load a second loop at 70 s (as Next
does), every sketch against the same loop without it (`sketch_check.py`).
`render2.mjs` seeds the synth's randomness, so a pair differs only by the
sketch.

**Refused notes: none, from any sketch, anywhere.** In every render, each
voice was asked for its notes as often as without the sketch and lost no
more of them. With every sketch on at 150 s, each layer's own level
(from per-layer stems, with the sketches taken out of their channels) was
unchanged to 0.01 dB in glade #21 and grove #62. In haven #38 the bass
moved 0.25 dB because long chords changed its chords, and the bass
follows them. Round 1 for comparison: the organ lost 5–24 of its own
chords per loop and cost grove #62 one pad and one texture note, and vapor
#30 a bell note.

### dubStab

- **How often, drawn per loop:** rare (34%), sparse (36%), some (20%),
  and, on busy loops only (drums and 7 or more notes a second), busy at
  a weight of 14 against those three. Per bar: rare 0.22 stabs, sparse 0.5, some 0.8, busy 1.15
  (round 1: about 1). Across 400 loops: 155 rare, 154 sparse, 76 some, 15
  busy. A stab only lands over a groove (drums or bass in the bar, and four
  notes among the other layers), never in a rest or full stop.
- **Rounder:** a triangle under a softened saw (round 1: three saws), a
  12 ms attack (round 1: 4 ms), a lowpass closing from 2.4–3.4 kHz to 700
  Hz (round 1: a narrow 820 → 560 Hz bandpass), a darker and lighter throw
  into the echo, the lows trimmed under 220 Hz, voiced G3–C5. A sparse
  loop's stabs come down by up to 8 dB.
- **Warmth against Basic Channel, "Quadrant Dub I"** (`warmth.py`, the
  stab and its echo alone, 150 Hz–4 kHz):

  | | Centroid | 2–4 kHz vs 250 Hz–1 kHz | 1–2 kHz vs 250 Hz–1 kHz | Attack rise |
  |---|---|---|---|---|
  | Quadrant Dub I (whole track) | 377 Hz | -11.8 dB | -7.7 dB | 12 ms |
  | Round 1 (three loops) | 740–860 Hz | -12 to -23 dB | -7 to -9 dB | 1–3 ms |
  | Tuned (three loops) | 320–338 Hz | -24 to -29 dB | -14 to -16 dB | 13–18 ms |

  Round 1 sat an octave brighter than the record with a click of an
  attack; the tuned stab sits near the record's centroid and attack, with
  less 1–4 kHz bite than the record (the track's shimmer is more than its
  stab, so this is a direction, not a match).
- **Cost:** it asks whether there's room (its cost plus 20 units under the
  full ceiling) and holds nothing, so it can't cost anything a note; the
  core would allow for it as a small fixed overhead.
- **Level:** adds -11 to -20 dB against the loop over 60–75 s (round 1:
  -8 to -12); its loudest 2 s window sits -11 to +4 dB against the loop.

### organ

- **Character, drawn per loop:** a registration from nine (flute,
  stopped, mellow, choral, full, hollow, airy, round, bright: drawbar
  settings from 8' alone to a full 16'–1' stack), a slow rotary-style
  chorus (a swaying delay with a slower tremolo, 0.55–1.15 Hz), a little
  drive, a tone filter, a swell from instant to 0.6 s, and a soft key click
  in 45% of loops.
- **Zero refusals by construction:** one instrument per loop, four voices
  built at load and retuned at each chord, never asking the voice budget.
  It costs four oscillators and about a dozen nodes for as long as the loop
  plays; the core should count it as fixed overhead outside the voice
  budget. Reserving room for it inside the budget instead (20 units) would
  have refused other notes: glade #21 4 more pad notes, grove #62 12,
  wayfare #23 40.
- **Full stops stay full stops:** the organ lets go in a bar the loop
  leaves silent, and after a bar or so where the chords rest for more
  than two bars (round 1 held through haven's stop).
- **Level:** -12 to -20 dB against the loop; a sparse loop's organ comes
  down by up to 11 dB.

### wash

- **Level against the loop:** a follower listens to the loop's own five
  layers (not the wash) over about 1.4 s and sets the wash's level from
  it, aimed 9–13 dB under the loop (drawn per loop). Period drawn 17–26 s
  (round 1: 20). It has a channel of its own with the texture's level and
  sends.
- **Measured, loudest 2 s window against the loop (round 1 → tuned):**
  grove #62 (beat) -21.6 → -8.3 dB; vapor #4 -12.2 → -3.9; haven #43
  -21.6 → -9.8. Median windows stay -22 to -35 dB: still sparing, now
  audible under a beat.

### longChords

- **An option:** today's lengths 56%, twice today 16%, about 8 s 10%,
  about 16 s 9%, the whole loop 9%. Calm loops (energy under 0.35, or
  floating, meditative, hushed, misty, foggy, hypnotic) take the long
  ones 1.8 times as often; driving loops (energy over 0.65) half as often.
- **Across the 80-loop corpus:** 44 today, 14 twice, 9 eight, 4 long, 9
  whole. Across 400 loops, chord changes a minute (median) 22.5 today,
  16.4 round 1, 19.4 tuned; loops under 2 a minute 18 → 39; one-chord
  loops 11 → 31. Wayfare #25 (where round 1 didn't work) now draws twice
  today's length.
- **Note:** in developing loops and arpeggiated ones the chord counter
  counts sections and notes, so "whole loop" holds one chord per section
  there.

## Stereo, round 2

The space sketch (Ears 2) plus round 2's changes; the recipes are drawn
per loop from the seed and from what the loop is (`space_jobs.py`, `wide2`
and `drift2`):

- **No bouncing:** echo width 0.15–0.3 (round 1: 0.8–1.0); the light
  kit placed 0.08–0.15 to one side as one (round 1: hats and shakers on
  opposite sides, 0.25–0.5).
- **Heavy parts centered:** chords, pads, the melody and the bass never
  placed; chords and texture widened symmetrically (spread 0.3–0.5 and
  0.35–0.6), only a light texture (chimes, bells, drops) placed, by 0.1–0.25.
- **Sparse, wet loops near mono:** every width scaled by how busy the loop
  is (0.25 under half a note a second up to 1 at 5.5), and by 0.75 more
  when the reverb space is over 0.7; the width ceiling -7 dB on busy
  loops, -9 on sparse, -12 on very sparse.
- **Drifting, slow:** the reverb, echo and spread widths breathe together
  between 55–70% and 100% over 60–140 s; a light texture and the light
  kit cross over 30–60 s.
- **Wider:** the same recipe with every width 1.8 times larger (side +5
  dB). Every stereo move is linear in its width, so this is exact
  (`scale_side.py`).

### Guards (12 loops, eight of them 150 s; against each loop's mono render)

| Measure (median; worst) | Round 1 wide | Round 2 drifting | Round 2 wider, drifting | Proposed limit |
|---|---|---|---|---|
| Fold loudness vs mono (LU) | -0.01; 0.16 | 0.00; 0.10 | 0.00; 0.10 | within ±0.5 |
| Worst octave, fold vs mono (dB) | 0.47 | 0.33 | 0.33 | within ±1 |
| Side/mid above 200 Hz (dB) | -10.1; -7.5 | -19.3; -11.9 | -14.3; -8.0 | (round 1's band: -14 to -6) |
| Side/mid below 120 Hz (dB) | -30.6; -20.4 | -40.4; -28.8 | -35.4; -23.7 | -20 or lower |
| Correlation, median / 5th pct | 0.94 / 0.68; worst 5th 0.45 | 0.99 / 0.96 | 0.98 / 0.88; worst 5th 0.75 | 0.7+ / 0.3+ |
| Left/right balance (dB) | worst -2.79 | worst -0.15 | worst -0.26 | within ±2 |
| Bounce (image jump per 100 ms, 90th pct, dB) | 4.00 | 1.80 | 2.79 | new; lower is steadier |
| Balance travel over 4 s windows (dB) | 3.12 | 0.58 | 1.02 | new |
| Refused notes | same as mono | same | same | same |
| Render time vs mono (JS, rough) | 1.21× | 1.40× | 1.40× | under 2× |

Round 2 is much steadier than round 1 (half the bounce, a sixth of the
balance travel) and much narrower: as drawn it sits 9 dB under round 1's
width, near mono on sparse loops, which is why the wider version is in
the clips. Vapor #30 now reads -24.7 dB of side (round 1: -9.8, with a 2.8
dB lean).

## Clips

1. **dubStab:** undertow #1 without / round 1 / tuned (drew sparse), then
   grove #31 round 1 / tuned (drew rare), then wayfare #23 round 1 / tuned
   (drew busy). 25 s each, the window with the most stab in it.
2. **organ:** haven #38 and glade #21 without / round 1 / tuned; grove #62
   and vapor #30 without / tuned. 20 s each.
3. **wash:** grove #62 (beat), vapor #4 (sparse), haven #43, round 1 /
   tuned. 42 s each (two swells).
4. **longChords:** dust #51 and vapor #70 (whole loop), thaw #2 (about 16
   s), grove #47 and wayfare #25 (twice today), today / long. 30 s each.
5. **stereo** (undertow #8, cinder #16, glade #57, vapor #30, round 1's
   loops): mono 30 s, round 1 wide 20 s, round 2 40 s, round 2 wider 40 s.
6. **stereo, long:** grove #62 mono 40 s, then round 2 wider and drifting
   for the whole 150 s render.

## Round 3 (2026-10-16, same session)

What was behind each verdict, and the fix. No sketch refused a note in
any round 3 render either.

- **organ, "broken" low notes: it was clipping.** Its four voices summed
  to about 2 into a drive stage whose input ends at 1, so every chord was
  squared off; the distortion under the notes read -19 dB against the
  organ (glade #21). Now the sum enters at most about 0.8, through a soft
  curve normalized to full scale (2× oversampled), with gain made up after
  it and a gentle 75 Hz highpass so a 16' stop doesn't thicken the bass.
  Distortion under the notes -40 dB (glade #21), -43 dB (grove #62). Level
  kept at round 2's: -13 to -15 dB against the loop.
- **dubStab, "knocking on a steel roof" except on a busy groove:** it now
  plays only on busy loops (drums and 7 or more notes a second: 143 of 400
  loops), drawing rare, sparse, some or busy as before; every other loop
  has none. The sound is the busy wayfare stab Mikey liked, made softer
  and blooming: a 22 ms attack (round 2: 12), a 0.32–0.45 s tail (round 2:
  0.18–0.3), two saws 6 cents either side under the triangle, a lighter
  and darker echo throw, and some reverb. Tone against "Quadrant Dub I":
  centroid 285–302 Hz (the record 377), 2–4 kHz -22 to -23 dB against the
  body (the record -12). Level about round 2's.
- **wash, far too loud:** aimed 8 dB lower (17–21 dB under the loop).
  Loudest 2 s window against the loop, round 1 → round 2 → round 3: grove
  #62 (beat) -21.6 → -8.3 → -16.3 dB; vapor #4 -12.2 → -3.9 → -11.9;
  haven #43 -21.6 → -9.8 → -17.7. So: round 1's level on the sparse loop
  Mikey liked, and 5 dB more than round 1 under a beat, where round 1
  vanished.
- **longChords, a held chord a bit too loud:** measured on the chord layer
  alone (stems, seven loops) the change is small: -0.1 to +1.6 dB (grove
  #47 the most), the whole mix within ±0.7 dB. A chord that rings on while
  the rest moves reads louder than its level, so held chords are trimmed:
  twice today's length -0.7 dB, about 8 s -1.3 dB, 16 s and whole-loop -2
  dB. The spread itself is unchanged (approved).
- **stereo, round 3: where the width lives.** Three static versions at
  the same width (side about 18 dB under mid above 200 Hz, round 2's), so
  only where the width sits differs:
  - **steady:** round 2 held still (no breath, no drift).
  - **room:** every dry part centered; the width only in the reverb, whose
    side is now the reverb through four delay allpasses (5–23 ms), a
    smoother spread than round 2's even-minus-odd combs; no echo width.
  - **ensemble:** no room width; chords and texture spread 1.6 times
    round 2's (then the whole matched to round 2's width), the light kit
    as in round 2.

  | Measure (median of 4 loops) | steady | room | ensemble |
  |---|---|---|---|
  | Side/mid above 200 Hz | -18.4 dB | -18.2 dB | -18.2 dB |
  | Correlation, 5th percentile | 0.96 | 0.89 | 0.96 |
  | Bounce (90th pct, dB per 100 ms) | 1.69 | 1.02 | 1.82 |
  | Balance travel (4 s windows) | 0.41 dB | 0.70 dB | 0.47 dB |

  Every guard passes on all four loops: fold within 0.16 LU and 0.47 dB
  per octave of mono, side 26 dB or more under mid below 120 Hz, balance
  within 0.5 dB, refused notes as in mono. (The guard script read a false
  fold offset when the stereo render was shorter than the mono one; fixed.)

### Round 3 clips

1. **dubStab:** wayfare #23, grove #62, cinder #12 (all busy loops),
   round 2 / round 3, 25 s each.
2. **organ:** haven #38, glade #21, grove #62, vapor #30, round 2 / round
   3, 20 s each.
3. **wash:** grove #62 (beat), vapor #4, haven #43, round 1 / round 3, 35
   s each.
4. **longChords:** grove #47, dust #51, thaw #2, round 2 / round 3, 25 s
   each.
5. **stereo** (undertow #8, cinder #16, glade #57, vapor #30): mono,
   steady, room, ensemble, 25 s each.

## Round 4 (2026-10-16, same session)

- **longChords, rough or too loud when held: they jumped an octave.**
  Not the master: nothing reaching its saturator went past 0.84 of full
  scale, and holding chords didn't change its distortion. The app moves
  the keys down an octave when their centre sits within five semitones of
  the melody's (`separateRegisters`); holding chords nudges that centre
  across the line, so in 11 of 87 held loops (200 scanned) the chords
  landed an octave away from today's: grove #47's pads went from 104 Hz to
  208 Hz at the bottom, into the range the ear is most sensitive to,
  brighter and rougher (its chord layer's 15–75 Hz flutter at 150–400 Hz
  doubled, and 2.5–6 kHz appeared). Now held chords take the octave
  today's progression gets: 0 of 87 move. Grove #47's chord layer is back
  to today's roughness and 0.6 dB under today's level. Repeated pad
  chords are also tied into one note rather than struck again over their
  own tail.
- **dubStab, too full:** the same sound, less of it: a shorter tail
  (0.24–0.34 s), half the reverb, the lows trimmed under 320 Hz (round 3:
  220), and about 5 LU quieter by ear (its loudest 3 s against the loop:
  wayfare #23 -4.9 → -10.0 LU, grove #62 -7.9 → -13.5, cinder #12 -9.0 →
  -14.0).
- **organ, wrong texture, in front:** the soft registrations only (choral,
  full and bright fall back to mellow, stopped and round), no drive, a
  darker tone (1.4–2.5 kHz), every chord swelling in over 0.25–0.7 s, no
  pitch-bending chorus (a slow level shimmer instead), and some of it sent
  into the room. Level as round 3 (-12 to -15 dB against the loop); it
  should read as further back without being quieter.
- **wash, harsh and heavy:** a pair of gentle lowpasses sweeping 350 Hz →
  1.3 kHz (rounds 1–3: a bandpass 500 Hz → 2 kHz, a hiss), with a slower
  rise and fall. By ear (loudest 3 s against the loop, K-weighted), round
  1 → round 3 → round 4: grove #62 (beat) -20.8 → -12.7 → -18.4 LU; vapor
  #4 -12.4 → -8.4 → -14.2; haven #43 -18.5 → -11.7 → -16.9. Round 4 sits
  2 dB under round 1 on the sparse loop and 2–3 dB above it under a beat.
- **stereo, a blend of room and ensemble:** both sides added, room
  weighted 0.64 and ensemble 0.5, matched to the same width (exact: every
  move is linear in its width; `blend_side.py`). Median of four loops:
  side 18.0 dB under mid above 200 Hz, correlation 5th percentile 0.93,
  bounce 1.21 dB (room 1.02, ensemble 1.82), every guard passing.

No sketch refused a note in round 4 (fewer pad notes asked for where held
chords are tied, so fewer refused: grove #47 6 → 4, dust #51 9 → 2).

### Round 4 clips

1. **dubStab:** wayfare #23, grove #62, cinder #12, round 3 / round 4.
2. **organ:** haven #38, glade #21, grove #62, vapor #30, round 3 / round 4.
3. **wash:** grove #62, vapor #4, haven #43, round 1 / round 4.
4. **longChords:** grove #47 today, then grove #47, dust #51, thaw #2,
   round 3 / round 4.
5. **stereo** (undertow #8, cinder #16, glade #57, vapor #30): mono, room,
   blend.

## Round 5 (2026-10-16, same session): organ and wash

- **organ, articulation after real organs.** Both flavors keep round 4's
  soft registrations, darker tone, shimmer and room send, and change how
  notes start and end. A note two chords share now keeps sounding through
  the change, untouched; only the notes that change are let go and
  spoken again (round 4 dipped and re-swelled every voice on every chord).
  - **tonewheel** (after a Hammond): notes speak at once and let go in
    about 0.1 s; each newly spoken note gets a soft percussion, a 4' or
    2 2/3' partial that rings and fades in about 0.1 s (the Hammond's
    percussion tab), so a chord change is a touch, not a swell.
  - **pipe** (after a church or chamber organ): notes speak over 60–120
    ms with a soft chiff (a breath of noise at the pipe's second
    harmonic), each with a celeste rank 3–6 cents sharp at half level (a
    slow beat, not a wobble), and let go over a third of a second.
  Each loop draws one (half and half for now). Level as round 4 (-13 to
  -15 dB against the loop); no refusals (the organ is still outside the
  voice budget; the percussion and chiff are a few short-lived nodes per
  chord).
- **wash, brightness and color back.** The sweep opens to about 2.5 kHz
  again (round 4: 1.3 kHz), still through gentle lowpasses rather than
  round 1's narrow band, and two soft resonances tuned to the loop's key
  (the root or its fifth in 600–1200 Hz, and an octave and a fifth above
  it) color it, like wind across a bottle. Spectral centroid of the wash:
  round 1 1.9 kHz, round 4 0.5 kHz, round 5 0.9 kHz. By ear (loudest 3 s
  against the loop), round 1 → 4 → 5: grove #62 (beat) -20.8 → -18.4 →
  -16.9 LU; vapor #4 -12.4 → -14.2 → -13.0; haven #43 -18.5 → -16.9 →
  -15.9.

### Round 5 clips

1. **organ:** haven #38, glade #21, grove #62: round 4 / tonewheel /
   pipe, 20 s each.
2. **wash:** grove #62, vapor #4, haven #43: round 1 / round 4 / round 5,
   25 s each.

## Round 6 (2026-10-16, same session): organ variations

Mikey wants one organ in the end, picked quickly from a few variations.
Every variation is a pipe organ (his lean) and shares three changes,
aimed at his notes:

- **Less busy:** it moves at most once a bar, and holds through a passing
  chord (one that keeps the lowest note and two of the notes), instead of
  following every change of the keys.
- **Stops burying the tune:** a 5 dB dip around 800 Hz, where melodies
  live.
- **Heard in the approved stereo baseline**, since enveloping is partly
  width: the organ rides the chords' spread and the room's width.

The variations:

| Variation | Ranks (drawbars or stops) | Speech | Room | Idea |
|---|---|---|---|---|
| chapel | flutes 8' and 4' | 120 ms, soft chiff | a little | small, sweet |
| cathedral | principal chorus 16' 8' 4' 2' with a mixture (1 1/3', 1') | 200 ms, soft chiff | big | power |
| distant cathedral | the same | 250 ms, no chiff | most of it; direct sound 8 dB down | from the back of the nave: envelops |
| celeste | a string rank (8' with upper partials) and a celeste 7 cents sharp at near level | 300 ms, no chiff | medium | surrounds, shimmers slowly |

Levels matched by ear to within about 2 dB of each other, about 2 dB
under round 5's pipe. No refusals (outside the voice budget as before).

Clip: haven #38, glade #21, grove #62, each as no organ (12 s), round 5's
pipe, chapel, cathedral, distant cathedral, celeste (18 s each).

## Round 7 (2026-10-16, same session): the organ mix

Mikey asked for one organ that sounds 55% like chapel, 25% like round
5's pipe, 10% like the distant cathedral and 10% like celeste
(`organVariant: 'mix'`, weights in `organMix`). How the recipes mix:

- **Stops** mix as pipe loudness: each drawbar's amplitude, weighted, so
  the chapel's 8' and 4' flutes lead, with a little 16', 2 2/3', 2' and
  mixture from the others. Round 5's pipe brings the loop's own soft
  registration and tone, so loops still differ (by 25%).
- **Timing and room** as weighted means: speech about 0.14 s, a chiff about
  half as strong as round 5's, release about 0.47 s, reverb send about 0.53, direct
  sound 6% down, celeste about 3.7 cents sharp at about 0.36 level.
- **Behavior:** holds through passing chords and moves at most once a bar
  (75% of the recipes do), and a 3.75 dB dip at 800 Hz (75% of the
  recipes' 5 dB).

Level by ear: within 1–2 dB of the others (haven #38 -14.0 LU against the
loop, glade #21 -10.7, grove #62 -11.1). No refusals.

Clip: haven #38, glade #21, grove #62, each as no organ (12 s), chapel and
round 5's pipe (18 s each), then the mix (25 s), in the stereo baseline.

### For a later session: experimental stereo

Mikey keeps the idea of stereo that moves (panning, drifting) for loops
and vibes that want it, engineered carefully, in a session of its own;
rounds 1 and 2 showed it needs slow, deliberate motion and a reason. The
default stays steady.

## For the brain (if approved)

- **organ:** a voice outside the voice budget, priced as fixed overhead;
  registration, rotary rate and depth, drive, tone, swell and click are
  per-loop data.
- **dubStab:** a probe of the budget that holds nothing; density kind and
  tone per loop; the groove rule (drums or bass, four notes) as data.
- **wash:** a level follower on the five layers (a slow envelope of their
  sum), the wash's gain proportional to it; a channel of its own.
- **longChords:** a draw from the chords' own stream; weights per vibe.
- **stereo:** round 2's building blocks are round 1's plus one slow LFO on
  all side sends (the breath) and a sway on the light kit.

## Not checked yet

The Rust core, a phone's cost, rerolls of a single layer, and a blind A/B
against mono (the brain's call). The 150 s renders cover four loops (the
two densest loops, wayfare #23 and cinder #12, at 60 s: their 150 s renders
take 5–10 minutes each here).

## Files

`tools/ears/`: `render2.mjs` (stems, mid-render loads, reserve, sketch
yields, index written after every job), `scan.mjs`, `sketch_check.py`,
`warmth.py`, `seq_clips.py`, `scale_side.py`, round 2 in `space_jobs.py`,
bounce and drift in `stereo_guard.py`, per-recipe ceilings in
`width_ceiling.py`, `sketches/ears4-sketch.patch`. In the PR for Mikey to
merge, with the `docs/EARS.md` update.
