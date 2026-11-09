# Ears batch 6: the second synthesis

Ears 13 (Opus 5.5, high), 2026-11-08, in a cloud Claude Code session. The
second synthesis of the album cards: 25 records from piles 3 to 6
(`cards-pile-3.md` to `cards-pile-6.md`, Ears 9 to 12b) turned into a ranked
list of twelve candidates, with batch 5's three unreviewed ones ranked among
them. The top five that the sketch build can reach were sketched **on the Rust
core** (queue item 83: one block in `core/sketch/`, built only with the
`sketch` feature, never in the app) and rendered with `tools/clips.mjs` on
`main` at 8ee2f71: corpus seed 20261009 (the loop numbers Ears 4 and 8 used),
Let the loop wander on, the approved stereo baseline, 45 s a loop (the
sampler 50 s), every segment matched to -18 LUFS. The sketches, their jobs
and the scripts are in `tools/ears/sketches/ears13/`; `core/sketch/` is back
to the veil demonstration and `core/` is unchanged. The clips are on the
branch `ears13-clips` (below), deleted after the verdicts (2026-11-08); `tools/ears/sketches/ears13/README.md` rebuilds them.

**Ears 13b** (Opus 5.5, high), 2026-11-08, in a cloud Claude Code session,
on `main` at d9190bf: the seven candidates that waited on levers, sketched
with queue item 99's levers in the ranked order, and one new instrument,
**twang** (#13). Five are levers alone (no block): tripletKit, dottedPulse
and planingSevenths are forced levers, slowOpening and tuningOffset moved
constants, widePad a space pin. dropSpotlight and twang are blocks in
`core/sketch/` (`chords.RETUNE=1`, no silent organ). The same corpus, Let the
loop wander on, the stereo baseline, every segment at -18 LUFS; 45 s a loop
(slowOpening 100 s, dropSpotlight and the sampler 50 s). Each section below
ends with its **Ears 13b sketch**; "What the levers did" says plainly where a
lever falls short and what would do. The clips are on the branch
`ears13b-clips`, deleted after the verdicts (2026-11-09).

## Verdicts

**Mikey, 2026-11-08:** everything sounded fine, no notes: the sampler and all
five sketched candidates approved, at the sketched levels. Where a candidate
has two versions (lowBell struck and swelling, wideSweep wide and fast),
both go in as options weighted by the vibes (his taste: weighted by vibe or
somewhat random). Nothing stood out, so the in-app album after the build is
where any of them can still be tuned or dropped. Seven candidates could not
be sketched yet: each needs a lever the sketch build doesn't have (see
"Levers wanted", queue item 99), and their clips wait for an Ears 13b.

**Mikey, 2026-11-09 (Ears 13b):** "all the clips sound great": the 13b
sampler and all eight approved, every version (dottedPulse's pulse and
kick332, dropSpotlight struck and swelling, twang soft, slow and bright), at
the sketched levels. As with the first five, the versions go in as options
weighted by the vibes. What a lever could not do yet ("What the levers did":
tripletKit on eighth-note kits, the quiet pulse, kick332 on most kits, the
tune under planing sevenths, the drop's late bell, twang on the melody) is
the build's to settle, with the levers' other halves; the clips are what was
approved.

| # | Candidate | What it is | Clip | Verdict |
|---|---|---|---|---|
| 0 | 00-everything | The sampler: each loop with one or two of the sketched five | 00 | Approved |
| 1 | tripletKit | A kit in threes: hats and ghost hits on the thirds of the beat, a 12/8 shuffle at 77 to 127 BPM | 13b 01 | Approved |
| 2 | lowBell | A struck low bell (D2 to C#3) on each chord's root; a swelling kind too | 01 | Approved, both versions |
| 3 | padWow | A slow tape-like wow on the chord layer only (14 cents at 0.5 Hz), the kick and bass steady | 02 | Approved |
| 4 | widePad | The chord layer much wider, on the side only, so mono stays the same (batch 5's) | 13b 02 | Approved |
| 5 | colorTone | A held 13th over major chords and an 11th over minor ones, on about half the chords | 03 | Approved |
| 6 | deepDuck | subDuck's deep, long kind: the bass dips 14 dB at each kick and takes 0.6 s to come back | 04 | Approved |
| 7 | wideSweep | padSweep's two far ends: a 4-octave ride every 16 s, and a 2.6-octave swing every 6.5 s | 05 | Approved, both versions |
| 8 | dottedPulse | A part every three eighths, or a 3-3-2 kick (batch 5's) | 13b 03 (two versions) | Approved, both versions |
| 9 | slowOpening | lateFloor's long end: the floor arriving 40 to 90 s in, fading up | 13b 04 | Approved |
| 10 | planingSevenths | Major sevenths moving in parallel, a whole step apart | 13b 05 | Approved |
| 11 | dropSpotlight | A low bell or held lead alone in a floor drop (batch 5's) | 13b 06 (two versions) | Approved, both versions |
| 12 | tuningOffset | A whole loop a little off A440 (up to about 40 cents), drawn per loop | 13b 07 | Approved |
| 13 | twang | New (Ears 13b): a synth in the spirit of a tremolo baritone guitar, picking each chord low and slow | 13b 08 (three versions) | Approved, all three versions |
| 0b | 00-everything-13b | The 13b sampler: five loops, each with one or two of the eight | 13b 00 | Approved |

**Listen in this order:** `00-everything` (without, then with), then `01` to
`05`. Each candidate has one file per version, `-without` first: every file of
a candidate holds the same four loops at the same times (0:00, 0:46, 1:32 and
2:18; the sampler 0:00, 0:51, 1:42 and 2:33), so you can switch between files
at the same point. `timestamps.json` in the download lists them. Every segment
is matched to -18 LUFS, so a change in loudness is not what you hear; a
change in balance is.

**Ears 13b, listen in this order:** `00-everything-13b` (without, then
with), then `01` to `08` in the batch's ranked order (tripletKit, widePad,
dottedPulse, slowOpening, planingSevenths, dropSpotlight, tuningOffset) and
twang last. As before, every file of a candidate holds the same loops at the
same times (0:00, 0:46, 1:32, 2:18; the sampler 0:00, 0:51, 1:42, 2:33,
3:24; dropSpotlight 0:00, 0:51, 1:42, 2:33; slowOpening 0:00, 1:41, 3:22),
listed in the download's `timestamps.json`. A re-timing lever (tripletKit,
kick332, planing, tuning) changes notes rather than adding a part, so its
"part" reading in the measurements is the difference of two renders, not
something added.

**How they would go in, if approved** (the rule Mikey set on 2026-10-18 for
batch 5): today's behavior stays one option of every choice; each approved
idea becomes one option in a seeded per-loop choice, weighted by the vibes;
most loops get one or two of the changes.

## How the candidates were chosen

Every card's summary, Stem check, Reliability notes, Guess and [JUDGMENT]
lines were read, and the cards in full where a candidate leaned on them. A
number was used only where its card calls the tool reliable for it: no bass
number from an empty or thin bass stem, no beat number from a track without a
kit, tempos from the drum grid where the meter tool is at a ratio, pitches
under 60 Hz to a semitone, pumping depths from `kick_check.py`'s own curve
rather than the 70%-of-true pumping floor, and mix-level sweeps only where a
card says they are a filter rather than notes changing register (Pyramid's
are not). **Repetition across records is the strongest evidence**: every
candidate has four records or more with a reading that holds, and the counts
below are records, not tracks.

Each lead the brief named, checked against what Driftloom does today:

| Lead | Records | Today | Here |
|---|---|---|---|
| Triplet and shuffle kits | 5 (Lifeforms, Animal Crossing and its K.K. songs, Holographic, Triangular Reformat, View From Above) | swing up to 0.3 of a sixteenth; no three-to-a-beat grid | #1, lever wanted |
| Chords past the seventh | 6 (Pyramid, Bubble Universe!, Holographic, Conscious Agents, Triangular Reformat, Synthesist) | a ninth added now and then (`plan.rs`), no 11 or 13 | #5, sketched |
| Planing major sevenths | 4 | progressions in a key | #10, lever wanted |
| Deep, long ducks | 7 | subDuck 5 to 9 dB, back in 0.15 to 0.25 s | #6, sketched |
| Very large filter rides | 10 | padSweep 2 to 3.5 octaves every 12 to 28 s | #7, sketched |
| Long floor drops | 9 | floorDrop's long shape about 12 s; lateFloor 8 to 30 s | #9 (the intro ones), lever wanted |
| Tuning offsets and slow wow | 6 and 10 | 0 to 3 cents; the master's wow 1 to 11 cents on everything | #12 and #3 |
| Quiet-to-loud arcs, late width | 4 and 2 | the arc (a filter), lateFloor | the wholesome part in #9 and #4; the loud swing dropped |
| Tracks that die away | Animal Crossing's 24 hours | the core's changes crossfade or ring out | dropped, see Broken Friends |

## Candidates, ranked

### 1. tripletKit: a kit in threes (sketched in Ears 13b)

- **Records (5):** Lifeforms, Vertical Pig (1,475 drum onsets: 52% on the three
  thirds of the 122.6 BPM beat, 0% on a straight eighth); Animal Crossing's 3AM
  (72% of hits on the thirds at 97 BPM) and a shuffle read in five more hours
  (12AM, 4AM, 5AM, 1PM, 4PM); Holographic Universe(s)!, three of four deep
  passes (My Big T.O.E. at 115, The Bubbling Up at 77, Part -1 at 127: straight
  eighth 0 to 1%); Triangular Reformat, three of four gridded tracks (She Was
  Me 89, One Byte 85, Toiret Status 117: straight eighth 0 to 2%); View From
  Above's Skywriting (a low tick every 0.250 s, a third of the 0.731 s beat);
  and on Animal Crossing again, K.K. Reggae (triplet at 57, mix level only).
- **Reference:** Vertical Pig 3:03 (the kit and bass return on the phrase);
  My Big T.O.E. 0:31; She Was Me 1:04 to 1:07; Animal Crossing 3AM, any minute.
- **Measurement and how far to trust it:** `beatphase.py` on Demucs drum stems,
  read by the spacing of the hit clusters (the meter tool's phase is a quarter
  beat off on these records, so not by their position). The empty straight
  eighth bin is what makes it a triplet grid rather than a swung one; a drum
  stem that doubles each hit could make the paired peaks, not the empty bin.
  The K.K. and the 20 light-pass hours are mix-level readings: leads.
- **Would add:** a groove family Driftloom does not have. Its swing moves the
  off-sixteenth up to 0.65 of a step; these kits put three equal hits in every
  beat and never play the eighth, the swaying shuffle the small hours of Animal
  Crossing and the whole Nonlocal Forecast circle share.
- **Yardstick:** on the drum stem, 60% or more of the hits on the beat's thirds
  (within a twelfth of a beat), under 5% on the straight eighth; the kick on the
  beat; refused notes no more than the loop's straight kit.
- **Guards:** a soft kick and quiet ticks as the records have (Animal Crossing's
  near-mono, -26 LUFS score); the hats' share above 2 kHz no higher than the
  loop's own kit.
- **Closest today:** the profiles' `swing` and the 6/8 cross-rhythms; the melody
  has a `six8` rhythm (`melody.rs`), the drums no grid of threes.
- **How it would go in:** an option of a seeded per-loop choice (today's kit
  first), weighted by the vibes; the generator's drum patterns gain a
  three-to-a-beat family.
- **Ears 13b sketch** (`"force": {"tripletKit": "on"}`; dust #69 at 76 BPM,
  bloom #74 at 87, tide #52 at 92, undertow #75 at 118, kits that play
  sixteenths): from the player's own note list (`dlrender --calls`), every
  hit off the beat lands on a third (the straight eighth 0 to 1%, the
  sixteenths 0 to 1%, against 32 to 41% and 27 to 37% today), and the kick
  stays on the beat (100%; tide #52's hand kit has none). The thirds hold 62%
  of the non-kick hits on dust #69 and 40 to 52% on the others, where the
  hats on the beat stay: the 60% yardstick holds on one loop of four. The kit
  loses about a third of its hits (dust #69 116 to 69, bloom #74 180 to 124),
  since the eighths are dropped, not moved. The 2-5 kHz share no higher (0.1
  to 0.0%, 0.3 to 0.2%); loudness -0.65 to -0.01 LU; no notes stolen beyond
  today's (75 to 72 on dust #69, the same elsewhere); the render's time within
  its noise. **It does not do what the batch needs on most kits** (see "What
  the levers did").

### 2. lowBell: a struck low bell on the chord's root (sketched)

- **Records (11):** Richard D. James Album (Peek, D2, partials 60 cents off;
  Yellow Calx, E2), Girl/Boy (Inkey$, C2 to D2), Come to Daddy (Mummy Mix, E1
  to F#2), Windowlicker (Equation, G1 to G2; Windowlicker's bell-bass),
  2 Remixes (Flow Coma, G#1), Drukqs (Orban Eq Trx 4, C#2, 14 dB/s),
  51/13 (Crowsmangegus, G#2 to D3), Lifeforms (low bells at 46 to 65 Hz with
  long tails on all four picks), 2814 (05, G#2, partials 92 cents off), Bowery
  Electric (Looped, C#3), and Kankyo Ongaku (Variation III, a bell that swells
  in over 0.5 s around G2 to G3).
- **Reference:** Lifeforms, Domain 0:15.8 (C2) and 0:19.3; Cerebral 0:45.5 (B2,
  3.5 s); Peek 2:41.8 (D2); Crowsmangegus 1:14.3; 2814's 05 1:17.1;
  Variation III 3:21.0 (the swelling kind).
- **Measurement and how far to trust it:** the bell label (partials off whole
  multiples) holds above 60 Hz, which is where these sit (65 to 165 Hz; the
  ones under 60 Hz were left out of the count). Attacks 30 to 150 ms, falls 5
  to 26 dB/s. Most groups are blends (silhouettes about 0.2): one sound, not a
  separated instrument.
- **Would add:** a new instrument in a register Driftloom's bells never use.
  Glass, the templebell, tubular and the chimes all ring above it.
- **The sketch** (`lowBell/block.rs`, on the chords channel): at each chord
  whose root changes (or the same root after 4 s), it strikes the root's pitch
  class between D2 and C#3 (73 to 139 Hz, so the fundamental is above the 60 Hz
  guard), four partials at 1, 2.08, 3.11 and 5.2 times (60 to 68 cents off
  whole multiples) falling 11, 19, 29 and 44 dB/s, a 35 ms attack (the swelling
  kind: 0.4 s), two rings at once, centered, damped at a full stop. Its level
  follows the chord layer's, with a floor. It hears the chords through the
  chapel organ, forced on with its level, dry and room pinned to 0: that silent
  organ renders every loop bit for bit as without it (checked on six loops, the
  hashes equal).
- **Measured** (haven #38, thaw #71, grove #62, dust #69): the part sits 14.0
  to 21.6 LU under the loop by ear (haven -14.2, thaw -14.0, grove -21.1, dust
  -21.6), its loudest 3 s 3 to 18 dB under (thaw's -3.0: a sparse pad, where
  the bell is nearly as loud as the loop when it strikes, for Mikey's ears);
  the loop's loudness +0.20 LU at most; 20-60 Hz unchanged (thaw's goes from
  0.0% to under 0.1%), 60-120 Hz up 0.1 to 2.7 points. A phone hears 12 to 15%
  of it (the upper partials). Cost on the kit: 0.09 to 0.22% (tubular 0.17%,
  the organ 0.25%).
- **Yardstick:** a note on each chord change at the root's pitch class, 65 to
  165 Hz; partials 50 to 90 cents off whole multiples; falling 8 to 25 dB/s; 14
  to 22 LU under the loop; energy under 60 Hz within 1 dB of the loop without
  it.
- **Guards:** the low-end guard (its register starts at D2); centered; a level
  that follows the loop; never in a full stop.
- **Closest today:** tubular and the templebell (higher); the FM `rhodesbass`.
- **How it would go in:** a new instrument, a voice of the core; a seeded
  per-loop choice like glass's (today's none first), weighted by the vibes; its
  struck and swelling kinds drawn per loop.

### 3. padWow: a slow wow on the pads alone (sketched)

- **Records (10):** Music Has the Right to Children (Smokes Quantity's other
  stem, 19 cents at 0.6 Hz, and drifts up to 48 to 51 cents over 4 s),
  Synthesist (1847 - Earth 27 cents at 0.3 Hz; the title track 17 at 1.2),
  Perdition City (The Future Sound of Music, 23 cents at 0.3 Hz),
  Holographic (My Incomplete T.O.E., 23 at 0.3), Triangular Reformat (Dam
  Ambience, 17 at 0.4), Bubble Universe! (15 to 22 cents at 0.7 to 1.7 Hz),
  Pyramid (Spiral Era, 16 at 1.1), View From Above (the bass, 18 to 19 at
  1.1), Structures from Silence (drifts of 24 to 26 cents in 4 s) and
  Windows 96 (the bass sliding 10 to 20 cents in 4 s).
- **Reference:** Smokes Quantity, any minute; 1847 - Earth 6:04 to 6:12;
  The Future Sound of Music 3:09 (the pad).
- **Measurement and how far to trust it:** per-stem wobble is calibrated (a
  12-cent wow reads 13.7); the depths are floors (the tool reads about 70%).
  Readings at 2.6 to 6.5 Hz were left out: the cards call them vibrato or the
  tool's floor on a mixed stem, not tape.
- **Would add:** the warm, slightly seasick drift of an old tape on the pads,
  with the kick and the bass steady. Driftloom's wobble moves the whole mix
  together, by 1 to 11 cents at 0.32 Hz.
- **The sketch** (`padWow/block.rs`, on the whole chords channel): a 3 to 5 ms
  delay whose time two sines move, 14 cents at 0.5 Hz and a third of that at
  0.185 Hz, so the drift does not repeat exactly; nothing else moves. The
  records' median is nearer 20 cents; 14 is the conservative pick (for
  Mikey's ears; the block's `depth` setting turns it).
- **Measured** (shrine #0, thaw #71, grove #62, undertow #45): loudness -0.05 to
  0.00 LU; the bands within 0.5 points. The "part" readings (-5 to -18 LU) are
  the chords' 3 to 5 ms shift, not something added. Cost 0.13 to 0.15%.
- **Yardstick:** the chord layer's pitch moving 10 to 20 cents at 0.3 to 1 Hz
  (`techniques.py`'s wobble on the chords stem); the drums and bass stems'
  wobble unchanged; the loop within 0.2 LU.
- **Guards:** never on the kick or the bass; no faster than 1.2 Hz (faster
  reads as vibrato); a cap of 20 cents.
- **Closest today:** the master's wobble (`master.rs`, the profile's `wobble`).
- **How it would go in:** an option of a seeded per-loop choice on the chords
  channel, today's none first, weighted toward the hazy vibes.

### 4. widePad: the chords wide on the side only (batch 5's; sketched in Ears 13b)

- **Records:** batch 5's eight (piles 1 and 2), and 10 more here: Far Away
  Trains Passing By (Trees' pad at correlation 0.04 at 1 to 2 kHz; 2.1 dB mono
  loss), Kankyo Ongaku (width rising with pitch, 0 dB above 2 kHz on Park),
  Mirror's Edge (the width in the middle), Windows 96, Bubble Universe! (Wave
  Nature, correlation -0.02), Holographic (Part -1, 0.13), Triangular Reformat,
  Conscious Agents, Synthesist and Pyramid (pads -3 to -7 dB wide, 1.2 dB mono
  loss at worst). Two records open their width late: Perdition City's Tomorrow
  Never Knows (-20.5 dB for five minutes, -2.8 dB in the last two) and
  Postscript (-4.5 to -0.8 dB at 6:00 to 8:00).
- **Reference:** Trees 2:59.7; Wave Nature 0:07.5; Kankyo Ongaku, Park 0:40.7;
  Tomorrow Never Knows, the last two minutes.
- **Measurement:** space by band and stem, calibrated (width within 0.1 dB).
  Every one of these keeps the sub centered (correlation 0.9 or more below 120
  Hz) and loses 1 to 3 dB of its pad in mono; the ones that lose more (2814 up
  to 9 dB, Tai Ki 31 dB) are left out.
- **Would add:** the width Mikey asked for that stays sound in mono, on the pad
  only: the approved baseline keeps the side 18 to 20 dB under the mid; these
  pads are 1 to 7 dB.
- **What the sketch found:** a block that leaves the channel's signal (the mid)
  as it is and adds an allpassed, 250 Hz-highpassed copy to the side
  (`widePad/block.rs`) renders mono bit for bit as without it, as it should,
  but adds nothing in stereo: the core calls `render_side` only on a channel's
  bus blocks, never on a whole-channel one, and a clip can pin the space's
  `spread-chords` only within its range (0.3 to 0.5). See "Levers wanted".
- **Yardstick (for the 13b):** the chord stem's side/mid above 250 Hz at -8 to
  -4 dB; the fold within 0.1 LU of the mono render and every octave within 0.5
  dB; side under mid below 120 Hz at -30 dB or lower; correlation over 0.4 at
  the 5th percentile; no bounce added.
- **Guards:** all the stereo guards; never on the bass or the kick; sparse loops
  at the narrow end (Ears 2: sparse loops wanted no width).
- **How it would go in:** a space option, drawn per loop by the vibes, beside the
  baseline; its late-opening kind (width growing over 60 to 120 s) as a second
  option.
- **Ears 13b sketch** (no block: `"space"` pins `spread-chords` 1.4,
  `spread-max` 2, the width ceiling raised to -4, -5 and -7 dB for busy,
  sparse and very sparse loops, and `spread-cutoff` 250 Hz; shrine #0, thaw
  #71, undertow #45, grove #62). `spread-chords` alone stops at the ceiling:
  the chord stem reached -6.0 to -8.8 dB however far it went. With the
  ceiling raised, the chord layer alone (the other layers asked off, a close
  stand-in for its stem) goes from -10.9 to -18.5 dB to -5.1 (shrine), -8.7
  (thaw, the sparse one, so the narrow end), -3.6 (undertow) and -1.8 dB
  (grove: wider than the yardstick's -4, for Mikey's ears). The whole mix:
  the fold to mono is exact (0.00 LU, every octave 0.00 dB: the spread is side
  only); side under 120 Hz -31.9 to -71.4 dB (with the baseline's 200 Hz
  highpass grove read -27.7, so the 250 Hz one is needed for the -30
  yardstick); correlation's 5th percentile 0.71 to 0.92 (median 0.93 to
  1.00); the side above 200 Hz from -16.0 to -31.5 dB to -12.4 to -25.6;
  balance within 0.1 dB; stereo loudness +0.00 to +0.14 LU. The image moves
  more on shrine #0 (bounce 0.8 to 2.6 dB, the spread's slow sway made wider);
  under 0.5 dB more elsewhere. No block, so no cost of its own (the render
  within its timing's noise). The late-opening kind cannot be sketched: see
  "What the levers did".

### 5. colorTone: a held 11th or 13th over the chord (sketched)

- **Records (6):** Pyramid (one or two of a 9, 11 or 13 held over 26 to 46% of
  the chord time: a 13 over Amaj7 95% of its time on Tomita, a dominant 13 on
  The Shrine's A#7 66%, a 9 over Fmaj7 on Apex), Bubble Universe! (38 to 46%:
  a 13 over G#maj7, a held 11 over Cmaj7), Holographic (51 to 95%: Csus2 with an
  11, Amaj7 with an 11 and 13), Conscious Agents (41%: G#7 with an 11),
  Triangular Reformat (She Was Me 60%: an 11 held over A#maj7 and Fmaj7;
  Classical Information 52%: a 13 over G#maj7), Synthesist (So Weit, So Gut
  86%: dominant 13ths).
- **Reference:** Tomita's Amaj7 (the 13); The Shrine's A#7; She Was Me 1:04 to
  1:07; So Weit, So Gut from the start.
- **Measurement and how far to trust it:** Ears 12b's colors reading
  (calibrated on synthesized chords, untested on real music; no false colors on
  60 plain chords). Where all three are held at once it is a wash of the scale,
  not a voicing (`share_all_three`), and those were not counted. Three of the
  six records are one circle (Nonlocal Forecast), so the count is closer to
  four families.
- **Would add:** the soft jazz color of a major seventh with its 13 on top, or a
  minor chord with its 11, which Driftloom never plays (it adds a ninth now and
  then).
- **The sketch** (`colorTone/block.rs`, on the chords channel, hearing the
  chords through the silent organ): over a chord with a major third, the 13 (a
  major sixth over the root); over one with a minor third, the 11 (a fourth).
  Both are in the scale over those chords in every diatonic mode, so it needs no
  key. The root is the note with a third and a fifth over it (so an inversion
  keeps its root); chords with no third, a flat fifth, or the color already in
  them get none. It sounds on about 6 chords in 10 (a draw from the chord's own
  notes), voiced just above the chord's top note (C6 at most), as two sines 5
  cents apart and a little second harmonic, swelling in over 0.35 s and letting
  go over 0.6 s. Its level follows the chord layer's.
- **Measured** (vapor #4, shrine #0, tide #39, grove #62): the part 13.4 to 22.4
  LU under the loop (vapor -13.4, shrine -18.2, tide -20.0, grove -22.4), its
  loudest 3 s 8 to 20 dB under; loudness +0.14 LU at most; 88 to 100% of it
  above 150 Hz (a phone plays it); it adds nothing under 120 Hz. Cost 0.11%.
  **Two limits found:** it reads the chords as the organ is sent them, and on a
  loop whose keys arpeggiate (haven #38) the organ hears one note at a time, so
  no chord forms and it stays silent; and undertow #45 holds one chord for the
  whole render, which drew no color. Neither is a limit of the idea: the
  generator knows its chords.
- **Yardstick:** on the chord stem, one color held over 25 to 50% of the chord
  time (`theory.py` colors), the 13 over major chords and the 11 over minor;
  no color the chord's mode lacks; 14 to 22 LU under the loop.
- **Guards:** never a flat ninth, a flat thirteenth over a minor chord or a
  natural 11 over a major one (the clashes); the 500 Hz to 2 kHz band where the
  tune lives no more than 1 dB up.
- **Closest today:** the ninth `plan.rs` adds to a chord now and then.
- **How it would go in:** in the generator, as the ninth is: an option of a
  seeded per-loop choice (today's chords first) that adds the color to a share
  of the chords, weighted toward the jazzy vibes.

### 6. deepDuck: the bass dips deep and comes back slowly (sketched)

- **Records (7), from the bass stem's own curve (`kick_check.py duck`):** Beat
  (Fear of Flying: 10 dB by 200 ms, 15 dB at the lowest, back by 750 to 850
  ms), Holographic (My Incomplete T.O.E.: 17 dB at 100 ms, lowest 27 dB just
  before the next kick), Windowlicker (Equation, Early Version: 18 dB at 90 ms,
  back by 500 ms), 51/13 (On, also pile 1's: 23 dB, the bass in pulses between
  kicks; 73-Yips, also pile 1's: 9 to 15 dB, back by 250 ms), Triangular Reformat (One Byte:
  12.7 dB at 100 ms, back to -1 dB by 500 ms), Synthesist (Transcendental
  Overdrive: 10.7 dB, back by 200 ms), Richard D. James Album (Peek: 10 dB at
  the kick, 15 dB at 675 ms). Two more play the bass off the kick (2814's 05,
  -13.6 dB; Schnauss's Sunday Evening, -9.7 dB).
- **Reference:** Fear of Flying 0:17.6; My Incomplete T.O.E. from 0:32; One
  Byte, any minute.
- **Measurement:** the bass stem's level around every kick, averaged; where a
  card's two tools disagree on depth, the stem's own curve is used. A
  separator can trade sub between the kick and bass stems, so read the depths
  as a range (10 to 27 dB), not exact.
- **Would add:** the pumping, breathing bass of these records. The approved
  subDuck (5 to 9 dB, back in 0.15 to 0.25 s) is a light touch; batch 5 kept it
  under 9 dB on purpose, reading the 20 dB gates of piles 1 and 2 as basses
  playing off the beat. Here the deep ones are real ducks with a slow return.
- **The sketch:** no new block. The approved subDuck, forced on, with its depth
  pinned to 14 dB and its recovery to 0.6 s (`jobs/deepDuck.json`). The clips
  put today, the approved subDuck as each loop draws it, and the deep one side
  by side.
- **Measured** (grove #62, undertow #45, halcyon #48, dust #69): the change
  against today 10.1 to 15.1 LU under the loop (the approved duck: 18.5 to 22.8);
  the loop 0.18 to 0.61 LU quieter (the bass is down more of the time); 20-60 Hz
  energy down in level (its share rises 0.4 to 0.8 points because the bass's
  upper partials drop more). No new block, so no cost.
- **Yardstick:** the bass stem 12 to 16 dB down 50 to 150 ms after each kick,
  back within 1 dB by 0.5 to 0.7 s; the other layers untouched.
- **Guards:** only with a kick and a bass (as subDuck); 16 dB at the most; never
  under the calm vibes (the coffee-shop rule: a deep pump is noticeable); the
  recovery never longer than the kick spacing.
- **Closest today:** subDuck (`sub_duck.rs`).
- **How it would go in:** a second option of subDuck's choice ("deep"), with its
  own depth and recovery ranges in `data/choices.txt`.

### 7. wideSweep: padSweep's far ends (sketched)

- **Records (10), stem level or a card that calls it a filter:** Holographic
  (My Incomplete T.O.E.: 4.4, 4.1, 4.1 and 3.4 octaves back to back over 14 to
  20 s each; The Bubbling Up 3.8 down and 3.8 up in 18 s), Bubble Universe!
  (Planck Lengths 4.2 octaves down in its first 10 s; Wave Nature 4.2 up in 6.5
  s), Triangular Reformat (She Was Me 4.1 octaves in 15 s), Come to Daddy
  (Mummy Mix 5.3 down in 10 s), Drukqs (Orban's other stem 6.0 up in 13.5 s),
  Girl/Boy (Inkey$, 1.7 to 4.9 octaves every 8 to 12 s), and the fast swing,
  2 to 3 octaves every 5 to 8 s: Far Away Trains Passing By (Trees, the first
  minute), Lifeforms (Vertical Pig, every 5 to 10 s), Beat (Fear of Flying,
  every 5 to 9 s), Synthesist (5 to 13 s).
- **Reference:** My Incomplete T.O.E. 0:23.5 to 1:23; Planck Lengths 1:24.5 to
  1:37.5; Trees 0:12 to 0:36.5 (fast).
- **Measurement:** sweeps calibrated (4.3 octaves read 4.2). Mix-level sweeps on
  drum tracks are often hats or a melody changing register; Pyramid's were
  dropped for that reason, and only the stem-level ones and the ones a card
  calls a filter are counted.
- **Would add:** padSweep's two ends that it doesn't reach: wider and slower (4
  octaves over 16 s), and smaller and faster (2.6 octaves over 6.5 s).
- **The sketch:** no new block. padSweep forced on with its settings pinned:
  wide, closed at 380 Hz, 4.2 octaves, a 16 s period, Q 1.0; fast, 550 Hz, 2.6
  octaves, 6.5 s, Q 0.9 (`jobs/wideSweep.json`). The clips play today, the
  approved padSweep as drawn, wide and fast.
- **Measured** (shrine #0, thaw #71, grove #62, undertow #45): loudness within
  0.09 LU of today; nothing under 120 Hz moves; the change 11.4 to 26.6 LU under
  the loop for wide (the approved: 17.0 to 31.7), clearest on the sparse pads
  (shrine -11.4, thaw -12.5), faint on the busy loops (grove -26.6, undertow
  -24.7, where the pad is quiet). Fast on thaw reads -10.4 LU, its loudest 3 s
  at the loop's level: for Mikey's ears, it is a lot of movement.
- **Yardstick:** the chord layer's brightness swinging 3.5 to 4.5 octaves on a
  14 to 20 s period (wide) or 2 to 3 octaves on 5 to 8 s (fast); the other
  layers untouched.
- **Guards:** sustained chord voices only (as padSweep); never under the calm
  vibes; fast only on loops with a kit.
- **Closest today:** padSweep (`pad_sweep.rs`).
- **How it would go in:** two more options of padSweep's setting ("wide",
  "fast"), each with its ranges, drawn per loop by the vibes.

### 8. dottedPulse: a part every three eighths (batch 5's; sketched in Ears 13b)

- **Records:** batch 5's three (GAK 1 and 2, Analogue Bubblebath 4 and 5), and
  five more here: Bubble Universe! (Planck Lengths' kick every 0.625 s, 1.5 beats
  of 0.419 s), Pyramid (Spiral Era's 3-3-2 kick at 120 BPM, a tresillo),
  Far Away Trains (Between Us And Them's kick every 0.520 s, three quarters of
  the beat), Conscious Agents (a 0.557 s dotted grid against a 0.752 s beat),
  Windows 96 (Wasting Time's kick at three quarters of the grid beat).
- **Measurement:** drum-onset repeats and kick spacing (sound, as every tempo
  octave was settled by them); what plays the dotted part is measured only
  where it is the kick.
- **Would add:** a lilt against the bar, the move Driftloom has in its bass
  pulse grid ([0, 6, 10]) but not in its kits.
- **Yardstick:** the drum stem's strongest repeat at 1.5 beats (or 3-3-2 over two
  beats); the beat still clear (the meter tool on the right tempo).
- **How it would go in:** an option of a seeded per-loop choice in the drum
  patterns, today's first.
- **Ears 13b sketch** (`"force": {"dottedPulse": "pulse"}` and `"kick332"`;
  undertow #45 at 129 BPM, dust #69 at 76, grove #47 at 105, undertow #1 at 94,
  all 16 steps a bar). **pulse:** a rim every six steps, 32 to 66 more notes
  in 45 s; the rims alone repeat at 1.5 beats (0.29 to 0.88), while the whole
  kit's strongest repeat stays at 1, 2 or 4 beats (the lilt is a layer under
  the beat, as the records have it). At velocity 0.2 it is 38 to 43 LU under
  the loop: a phone would play it (98% above 150 Hz), but it is likely too
  quiet to hear as a lilt, for Mikey's ears. **kick332:** on undertow #45's
  four on the floor the kick's repeat moves from 1 beat (0.82) to 1.5 beats
  (0.63), a 3-3-2; on the other three it only moves the kicks the loop has to
  the nearest of 0, 6 and 12, so a kit with no kick near the bar's middle
  gets 0 and 12 (dust #69: the kick on a 3-beat repeat; grove #47 and undertow
  #1: 1.5-beat repeats 0.00 and 0.24). Loudness -0.03 to -0.34 LU; on dust #69
  the 20-60 Hz share rises 6.7 to 8.4% (kicks moved together), past the low
  guard's 1 dB on its loudest window (+7.3 dB). No notes stolen beyond
  today's; the render's time within its noise.

### 9. slowOpening: the floor arrives late and slowly (sketched in Ears 13b)

- **Records (9):** 2814 (01: 90 s of pad alone, the floor at 1:29.5; the title
  track's bass at 1:28.5, the low end full by 1:48.5), Mirror's Edge
  (Introduction: no low end for 40.5 s, the bass at 0:40.8, the kit at 1:16.6),
  Pyramid (Tomita: the kick and bass out for 134 s from 0:41; The Shrine 36.5 s
  at 0:27), Holographic (My Incomplete T.O.E., 23.5 s from 0:08.2), Triangular
  Reformat (One Byte 22.5 s; Classical Information's floor out 13.5 s at 0:25.8),
  Far Away Trains (a thin, bright first minute, 12 to 19 dB under the peak, the
  low end arriving over 1 to 2 minutes), Perdition City (the kit at 1:11.5, the
  full level by 4:00), 51/13 and pile 1's On (38 s from 0:20.5), Windows 96
  (Lead, Venus Aire: the kit 12 to 30 s in).
- **Measurement:** the low end of all stems summed, 15 dB down for 2 s or more,
  and the stem entries: edges within 0.5 s.
- **Would add:** lateFloor's long end. Its wait is 8 to 30 s and it arrives at
  once; these records wait 40 to 135 s and, on half of them, fade the floor up
  over 4 to 20 s.
- **Yardstick:** drums and bass absent for the drawn time (40 to 90 s), then
  rising to their level over the drawn fade; the other layers unchanged.
- **Guards:** as lateFloor's (something plays from the first second; not on a
  reroll); a long wait only under the calm vibes; the fade never over 20 s.
- **How it would go in:** a wider range and a fade in lateFloor's setting
  (`data/choices.txt`, `[setting latefloor]`), the long end leaned by the vibes.
- **Ears 13b sketch** (`"force": {"lateFloor": "on"}`, `"set":
  {"latefloor.SECONDS_MAX": 90, "latefloor.FADE": 12}`; dust #69, tide #39,
  dust #49, 100 s each, against today and the approved lateFloor). From the
  note list: the bass and the kit arrive at 56.9 and 69.5 s (dust #69), 51.7
  and 54.6 s (tide #39), 62.3 s (dust #49), where the approved lateFloor brings
  them at 19 to 32 s; at their first hits their velocity is 0.03 to 0.22 of
  the loop's, about 0.4 six seconds on, and full by 12 to 14 s. Something
  plays from the first second (the first note at 0.1 s). The 100 s are 2.5 to
  4.1 LU quieter than today's (the floor is out longer; the approved: 0.6 to
  1.2), and the clips match each segment's loudness, so the floorless start is
  played louder than it would be in the app: listen for the arrival. No notes
  stolen beyond today's; no cost (a wait). It does what the batch needs.

### 10. planingSevenths: major sevenths a step apart (sketched in Ears 13b)

- **Records (4):** Holographic (My Incomplete T.O.E.: Gmaj7, Fmaj7, Amaj7, with
  their 11s and 13s, over a G bass), Triangular Reformat (She Was Me: C#maj7,
  A#maj7, Fmaj7, D#maj7, five chords, all major sevenths), Perdition City (We
  Are The Dead: Cmaj7, D#maj7, G#maj7, C#maj7; Dead City Centres G#maj7,
  D#maj7, Gmaj7), Animal Crossing (2AM C#maj7 and Dmaj7; 7PM Dmaj7, Amaj7,
  C#maj7).
- **Measurement:** chord names on dense pads are a guess; the share of major
  sevenths is the sound part (She Was Me 100% of its chords, We Are The Dead
  72%), the exact roots less so.
- **Would add:** the dreamy, floating move of a major seventh chord sliding as a
  shape, not as a progression in a key.
- **Yardstick:** 80% or more of the chord time on major sevenths, roots moving by
  a whole step or a minor third; the melody kept in the scale of the chord it
  sits over.
- **How it would go in:** an option of the progression's per-loop draw, today's
  first.
- **Ears 13b sketch** (`"force": {"planingSevenths": "on"}`; shrine #0, tide
  #39, thaw #71, grove #47; the chords read from the retunes of
  `chords.RETUNE=1`, the tune from the note list). Every chord is a major
  seventh (100% of the chord time, 0% today). The roots move a whole step or
  a minor third in 50 to 100% of the changes (thaw 100%, shrine 86%, grove
  76%, tide 50%; a chord repeated not counted); the rest are 1, 4 or 5 semitones, where the progression's
  cycle turns back to the tonic. **The clash is real:** the tune keeps the
  key's scale, so the share of its time a semitone over a chord tone goes from
  0 to 19% (grove), 9.6 to 15.8% (shrine) and 0 to 47% (thaw), and from 38% to
  0.1% on tide #39, where the planed chords happen to suit the tune better
  than today's. The four-note chords take more of the budget: notes stolen 3
  to 15 (grove), 1 to 5 (shrine). Loudness -1.15 to +0.29 LU; the bass follows
  the new roots (its loudest window under 60 Hz up 8 to 19 dB on shrine and
  tide, the whole loop's 20-60 Hz share within 0.8 points). The loop's other
  choices drew the same here. Render time within its noise.

### 11. dropSpotlight: one part alone in the floor drop (batch 5's; sketched in Ears 13b)

- **Records:** batch 5's five (Polygon Window, Bradley's Robot, Analogue
  Bubblebath 3, Caustic Window, Alberto Balsalm). Piles 3 to 6 add drops but not
  what plays alone in them, so the count stands. lowBell is its instrument.
- **Would add:** a reason for the floor to leave: the bell steps forward when the
  kit and bass drop.
- **How it would go in:** with floorDrop's shapes and lowBell, a share of the
  drops in a loop that plays the bell.
- **Ears 13b sketch** (`dropSpotlight/block.rs`, on the chords channel; the
  drums routed into its mod 0 and the bass into mod 1: `"route": {"sketch":
  "drums", "sketch.1": "bass"}`; floorDrop's shapes forced; dust #9, glade #21,
  tide #39, grove #47, 50 s). The approved lowBell (its partials and register,
  D2 to C#3), struck only in a drop: it hears the floor as the drums' kick
  band (twice through a 150 Hz lowpass, so hats that keep playing in a drop
  do not count) plus the bass, and when both stay 25 dB under their recent
  peak for 1.2 s it strikes the chord's root, and each new root while the drop
  lasts; the floor's return damps it over 1.5 s. A second version swells in
  over 0.4 s (lowBell's swelling kind). **Measured:** 100% of its energy
  inside the drops (plus a 2 s ring) on all four loops; it catches every drop
  of 3 s or more, 1.5 to 2 s after it starts (it must hear the floor go
  quiet first). The part 15.5 to 19.2 LU under the loop (swell 19.1 to 22.3);
  in a drop its loudest 3 s are 1.0 to 8.1 dB under what is left (glade #21:
  nearly as loud as the drop, for Mikey's ears); loudness within 0.11 LU; the
  whole loop's 20-60 Hz unchanged (its loudest added window 17 to 36 dB under
  the loop's median one). Cost on the kit 0.07 to 0.19% (lowBell's 0.09 to
  0.22).

### 12. tuningOffset: a whole loop a little off A440 (sketched in Ears 13b)

- **Records (6):** Music Has the Right to Children (-47 to +46 cents by track,
  median +6.5), Lifeforms (-44 to +28), Kankyo Ongaku (-49 to +45, a
  compilation), 2814 (-38 to +25, median -20), Windows 96 (Deep Swim -46), View
  From Above (In Ascent -36 against the album's +17). Mirror's Edge, Schnauss,
  Pyramid and the Nonlocal Forecast records sit at 0.
- **Measurement:** a single file's tuning is good to about 8 cents, so every one
  of these is well past it.
- **Would add:** batch 5 dropped this as heard only against the next loop. Two
  things have changed: Next and the game play loops one after another, and
  Mikey named it. It is the "old tape" cue at the scale of a loop.
- **Guards:** within ±40 cents (Boards of Canada's card: past 40 sounds wrong,
  not warm); never when a loop must sit with another (a game's own sounds);
  weighted toward the hazy vibes only.
- **How it would go in:** a per-loop draw at load, beside echoTime's.
- **Ears 13b sketch** (`"set": {"master.CENTS": ...}`, drawn by hand to cover
  both ways inside ±40: vapor #4 -32, dust #69 +18, shrine #0 -24, haven #38
  +27). The tuning read from each render (`librosa.estimate_tuning`, good to
  about 8 cents) moves -31, +14, -26 and +28 cents (dust #69's tuned kick and
  noises keep their pitch, so it reads short). Loudness -0.30 to -0.04 LU; the
  bands within 2 points; no cost. The "without" file has all four at A440, the
  "with" file each at its own offset, so it is heard as Next would play them:
  one loop after another, each a little off. It does what the batch needs.

### 13. twang: a tremolo baritone guitar's spirit, picking each chord (new, Ears 13b)

- **What it is:** not from a record; Mikey's instrument idea, on the line he
  drew on 2026-11-08 (`docs/DECISIONS.md`, "The music": a synth in the spirit
  of a real instrument, never an imitation). Low and mid register, a bright
  plucked attack falling quickly into a rounder tail, a slow amplitude
  tremolo, at home in a big room with the wobble.
- **The sketch** (`twang/block.rs`, on the chords channel with
  `chords.RETUNE=1`): nothing sampled. Each string is two band-limited saws 4
  cents apart through a two-pole lowpass whose cutoff starts at a multiple of
  the note and falls over 0.12 s to a rounder one, a two-stage fall (most of
  the pluck gone in a quarter second, a tail of about 3 s), and a pitch
  settling from 8 cents sharp over 40 ms. At each chord it picks the chord
  slowly, a beat apart: the lowest note's pitch class between E2 and D#3 (82
  to 156 Hz), its fifth, then its tenth; six strings ring. One tremolo moves
  everything that rings. **Three versions:** soft (4.5 Hz, 35% deep, the
  pick's cutoff 18 times the note), slow (tempo-synced, two cycles a beat, 55%
  deep, rounder: 12 times), bright (5.5 Hz, 45% deep, 30 times). Its level
  follows the chord layer's, as lowBell's does.
- **Why the chords and not the melody:** the melody channel hears no note
  events at all (the core sends a channel only retunes, damps, bends and
  chokes, and the melody none of them), so a source there cannot follow the
  tune. It follows the chord layer's events, as lowBell does; see "What the
  levers did" for what a melody twang needs.
- **Measured** (no kit, slow, big rooms: vapor #70 at 53 BPM, thaw #10 at 67,
  shrine #13 at 45, thaw #71 at 58; one build per version and loop, since the
  picks and the synced tremolo need the tempo): the part 14.5 to 18.7 LU under
  the loop (bright and soft the loudest, slow the quietest), its loudest 3 s
  1.2 to 15.8 dB under; loudness within 0.07 LU. Its energy 62 to 77% at 60 to
  250 Hz and 23 to 35% at 250 Hz to 1 kHz (low and mid); 20-60 Hz unchanged
  (0.0 to 0.2%), 60-120 Hz within 0.8 points. The tremolo, read from the
  part's level where the picks leave room for it: 4.50 and 4.54 Hz (soft),
  5.42 and 5.50 Hz (bright), 1.93 Hz on thaw #71 at 58 BPM and 1.61 Hz on
  shrine #13 at 45 (slow: two a beat); depth read 0.30 to 0.52 against 0.35 to
  0.55 asked; on the busier loops the picks' own rhythm hides it in that
  reading. The pick against the tail, on the kit's probes: 7.2% of the energy
  above 1 kHz in the first 50 ms against 1.4% at 0.3 s (soft), 4.0 against
  0.1% (slow), 9.1 against 3.4% (bright). Cost on the kit 0.21 to 0.30%.
- **Yardstick (for the build):** a note at each chord change on the chord's
  root, fifth and tenth, a beat apart, 82 to 400 Hz; the pick's share above 1
  kHz falling to a third or less by 0.3 s; the level swinging 30 to 55% at 4 to
  6 Hz or one or two cycles a beat; 14 to 19 LU under the loop; energy under 60
  Hz within 1 dB of the loop without it.
- **Guards:** the low-end guard (its register starts at E2); centered; never
  over a kit busier than a soft one at first (it was heard on loops with none);
  a level that follows the loop; never in a full stop (a release damps it).
- **Closest today:** the nylon's strum (a plucked chord, brighter and higher)
  and the harp; nothing with a tremolo.
- **How it would go in:** a voice of the core, as a sound a layer plays (the
  chords' or the melody's, once a voice can follow the tune), drawn per loop
  like glass, today's none first, its three versions weighted by the vibes.

## Levers wanted

The smallest hook that would let an Ears 13b sketch each. All live in the
sketch build only (`--features sketch`), as item 83's levers do.

| Candidate | What the sketch build lacks | The smallest hook |
|---|---|---|
| tripletKit | a drum rhythm on a grid of three to the beat | a forceable choice (`--force tripletKit=on`) whose option re-times the kit's hats and ghost hits from the beat's four sixteenths to its three thirds (dropping the straight eighth), the kick on the beat |
| widePad | a side from a whole-channel block | pass the side to whole-channel blocks too (`blocks::Channels::whole` calling `render_side` in a stereo render, as `buses` does), or a clip lever `--space NAME=V` that sets a space setting past its range |
| dottedPulse | a drum part on the dotted grid | a forceable choice adding a soft percussion part (a rim or a tuned tick) every three eighths, or the kick on a 3-3-2 |
| slowOpening | lateFloor past 30 s, and a fade | `latefloor.SECONDS_MAX` and `latefloor.FADE` in `core/src/tuning.rs`'s NAMES (or a clip `--pin` that may leave the data's range) |
| planingSevenths | a progression of parallel major sevenths | a forceable option of the progression draw: every chord spelled as a major seventh, roots a whole step or a minor third apart |
| dropSpotlight | a sketch that hears the floor drop | a clip lever `--route sketch=drums` (`Core::route` into the sketch's `MODS`, as subDuck is keyed), or a floor-drop event sent to the channels |
| tuningOffset | the loop's tuning | `master.CENTS` in NAMES, read where the core turns a note into Hz |
| (every chord sketch) | the chords, without the organ | in the sketch build, send the chords channel a retune at every chord (today only while the organ plays), so a chord sketch needs no silent organ and sees arpeggiated chords whole |

### What the levers did (Ears 13b)

| Candidate | Did the lever do what the batch needs? | What would |
|---|---|---|
| tripletKit | Only on kits that play sixteenths. It re-times the hits on a beat's second and fourth sixteenths and drops the eighth, so an eighth-note kit loses its off-beats and gains no thirds (dust #9: 101 hits to 44, 98% on the beat, none on a third; dust #32 and halcyon #17 alike). Of the corpus's 27 kits in 16 steps, 21 have under 20% of their off-beat hits on sixteenths. In a 6/8 loop its thirds of the dotted beat are the eighths it already plays, so it only thins the hats (grove #62: 239 to 164, no thirds). | A lever that writes the triplet hats itself (three a beat at the loop's hat velocities) and moves the eighth to two thirds of the beat (the shuffle's place) rather than dropping it; 6/8 left alone |
| widePad | Yes, with the width ceiling raised too: `spread-chords` alone stops at it (the chord stem at -6 to -9 dB). Its 250 Hz highpass is needed for the side under 120 Hz | The late-opening kind needs a space setting that moves over the loop (`spread-chords` from the baseline to the wide value over 60 to 120 s); a space pin holds one value |
| dottedPulse, pulse | The rim repeats every 1.5 beats, but at velocity 0.2 it is 38 to 43 LU under the loop | Its velocity as a constant (`dottedpulse.VEL` in `tuning.rs`'s NAMES), or a tuned tick of its own |
| dottedPulse, kick332 | Only on a four-on-the-floor kick (undertow #45). It moves the kicks a loop has to the nearest of 0, 6 and 12, so one with no kick near the bar's middle gets 0 and 12 | Writing the 3-3-2 kick itself (all three hits) rather than moving the loop's |
| slowOpening | Yes | (none) |
| planingSevenths | The chords, yes; the tune keeps the key's scale, so it clashes (up to 47% of its time a semitone over a chord tone) | A melody that follows the planed chord (its tones, the ninth, and the chord's own Lydian or Ionian scale), the lever's other half |
| dropSpotlight | Yes, 1.5 to 2 s late: the sketch must hear the floor go quiet before it knows a drop has begun | A floor-drop event sent to the channels when a drop starts and ends (the other hook this table named) |
| tuningOffset | Yes | (none) |
| twang (new) | Not on the melody: the melody channel hears no note events, so it follows the chords | A note event to the melody channel in the sketch build (as `chords.RETUNE` sends the chords a retune), or item 83's sketch voice |
| (every job) | `"force": {"CHOICE": "today"}` does not force a choice off: `dlrender` reads `today` as not forced, so a loop that drew the choice keeps it (shrine #0 drew lateFloor, so its "today" render was the approved one; slowOpening's clips use loops that draw none). `tools/clips.mjs`'s header says `today` is the first option | `dlrender --force` passing `today` through as the option's name (it is the first option's name in `data/choices.txt`), and a separate word for not forced |

## Dropped, one line each

- **Tracks that die away** (every Animal Crossing hour ends 21 to 38 dB down):
  the core's loop changes already crossfade or ring out (`dlrender --changes`);
  what the hours say is for the game phase (below).
- **A breathy vowel texture** (Windows 96's Wasting Time at B3, 2814's 05 at
  C#4): the vowel and hum voices exist; a breathier, lower one is a tune of them
  for a later instrument round.
- **Level breaths of 3 to 6 dB every 2 to 4 minutes** (Structures from Silence,
  Kankyo's Original BGM, View From Above, Mirror's Edge): longer than any clip,
  and the arc already breathes over minutes; for Endless's long form.
- **A quiet-to-loud arc of 12.9 dB** (Perdition City; Tomita's 11-minute build):
  a loudness swing a relaxing loop should not make at that depth; slowOpening
  takes the wholesome part.
- **A higher tuned kick at 90 to 116 Hz** (Windows 96's four tracks, Synthesist's
  64 to 91 Hz): tunedKick's note range moved up an octave; a lead for a
  hazy-beats profile (Windows 96's card), not a candidate alone.
- **Open fifths and sus2 chords** (Kankyo's Park, View From Above, Mirror's
  Edge): the chord tool reads sus2 and fifths where a soft third may hide; the
  quartal chords cover the open sound.
- **A meter that changes partway** (Tomita, 4 beats then 3) and **a 3:4
  hemiola** (The Shrine): one track each.
- **A bass vibrato of 9 to 21 cents at 3 Hz** (My Big T.O.E., 2814's 05, One
  Byte): the cards call it too much for a calm vibe.
- **Width that cancels in mono** (Tai Ki 31 dB, Looped 7 dB, Olson, 2814's top):
  fails the fold guard by design.
- **The chopped formant noise** (Toiret Status, Dam Ambience): one record, rough;
  a texture behind experimental vibes for a later round, not under the
  coffee-shop rule now.
- **Dominant pulls** (K.K.'s A7 to Dmaj7): needs an A/B of its own; for the game
  phase's friend themes.
- **Rooms and returns** (View From Above's hour of rooms, Kankyo's returning
  home): long form, for Endless.
- **The 166.7 BPM grid** (six pile 3 tracks): flagged on its card as a possible
  artifact.

## The [JUDGMENT] lines, settled for these candidates

Vibe names are `data/vibes.txt`'s. Toward is x1.5 to x3, away x0.2 to x0.5,
never 0; a vibe not named keeps x1.

| Candidate | Toward | Away | Wholesome |
|---|---|---|---|
| tripletKit | swaying x2, skipping x2, bubbly x1.5, groovy x1.5, playful x1.5, whimsical x1.5, cozy x1.5, dusky x1.5 | glacial x0.3, still x0.3, reverent x0.4, crisp x0.5 | Yes, soft and quiet as the records play it. |
| lowBell, struck | mysterious x2, moonlit x2, chiming x2, reflective x1.5, starry x1.5, wistful x1.5, strange x1.5, cosmic x1.5 | zesty x0.4, goofy x0.5, bouncy x0.5, sunny x0.5 | Yes; the low-end guard is in its register. |
| lowBell, swelling | glacial x2, serene x1.5, still x1.5, floating x1.5, meditative x1.5 | lively x0.4, zesty x0.4, bouncy x0.5 | Yes. |
| padWow | hazy x2.5, wistful x2, dreamy x2, misty x1.5, dusky x1.5, rainy x1.5, mellow x1.5, foggy x1.5 | crisp x0.3, bright x0.4, triumphant x0.4, frosty x0.5, refreshing x0.5 | Yes at 14 cents; the records' 20 to 27 can sound seasick. |
| widePad | spacious x2, floating x2, soaring x2, dreamy x1.5, hazy x1.5, cosmic x1.5, glacial x1.5 | intimate x0.3, minimal x0.4, cozy x0.5, plucky x0.5 | Yes, only with the fold guard holding. |
| colorTone | jazzy x2.5, intricate x2, mellow x1.5, velvety x1.5, dreamy x1.5, cosmic x1.5, bubbly x1.5, golden x1.5 | minimal x0.4, still x0.5, reverent x0.5, earthy x0.5 | Yes. |
| deepDuck | pulsing x2.5, groovy x2, hypnotic x2, droning x1.5, booming x1.5, cruising x1.5 | hushed x0.3, serene x0.3, restful x0.3, soothing x0.3, tender x0.4, peaceful x0.4 | Yes, at 16 dB at most and kept from the calm vibes. |
| wideSweep, wide | hypnotic x2, cosmic x1.5, bubbly x1.5, experimental x1.5, groovy x1.5 | still x0.3, restful x0.3, hushed x0.3, serene x0.3, glacial x0.4, meditative x0.4 | Yes. |
| wideSweep, fast | lively x1.5, pulsing x1.5, soaring x1.5, zesty x1.5 | the same as wide | Yes, on loops with a kit. |
| dottedPulse | groovy x2, quirky x1.5, cheeky x1.5, mischievous x1.5, intricate x1.5, jazzy x1.5 | still x0.4, restful x0.4, glacial x0.4 | Yes. |
| slowOpening | glacial x2, meditative x2, spacious x2, misty x1.5, rainy x1.5, moonlit x1.5, cosmic x1.5 | bouncy x0.4, lively x0.4, zesty x0.4, enthusiastic x0.4 | Yes, as long as something plays from the first second. |
| planingSevenths | dreamy x2, floating x2, jazzy x1.5, cosmic x1.5, wondrous x1.5, bubbly x1.5 | earthy x0.5, reverent x0.5, minimal x0.5 | Yes. |
| dropSpotlight | mysterious x1.5, reflective x1.5, spacious x1.5, chiming x1.5 | (none) | Yes. |
| tuningOffset | hazy x2, wistful x2, dusky x1.5, misty x1.5 | crisp x0.3, frosty x0.4, bright x0.5, chiming x0.5 | Yes, within ±40 cents. |
| twang, soft | dusky x2, wistful x2, moonlit x1.5, spacious x1.5, mellow x1.5, cruising x1.5, reflective x1.5, swaying x1.5, hazy x1.5 | zany x0.4, goofy x0.4, bouncy x0.5, frosty x0.5, crisp x0.5, sour x0.5 | Yes: low, round and slow. |
| twang, slow | meditative x2, dusky x1.5, droning x1.5, floating x1.5, dreamy x1.5, moonlit x1.5 | the same as soft, and lively x0.5 | Yes. |
| twang, bright | cruising x2, plucky x1.5, golden x1.5, warm x1.5, adventurous x1.5, dusky x1.5 | the same as soft, and hushed x0.5 | Yes; the pick's brightness is the guard (a third or less above 1 kHz by 0.3 s). |

**New profile:** the Windows 96 card's hazy-beats character (slow heavy beats,
min7 vamps, a high tuned kick, the bass's tape drift) now has three of its
parts here (padWow, tuningOffset, the dropped higher kick); worth proposing when
profiles open after phase 2. The Nonlocal Forecast circle (tripletKit,
colorTone, planingSevenths, wideSweep's wide ride) is a second.

**Hooks**, one line a listener might say, for each album the batch drew on:

- *Richard D. James Album*: fast, busy beats under soft, round tunes and a low bell-like bass, close and nearly mono.
- *Girl/Boy*: a held low note under bright bell chords whose filter rises and falls every phrase, then a drumless music box.
- *Come to Daddy*: frantic little drum patterns over a warm, oddly tuned bell-bass, the filter diving five octaves.
- *51/13*: a long held low note, a dry steady kick, a bass that ducks hard and pulses between the kicks.
- *Windowlicker*: a bouncing, wobbly bell-bass under shimmering bells and a deep, slow-breathing duck.
- *2 Remixes by AFX*: busy mono drums over one slow chord, then a long, shimmering high drone.
- *Drukqs*: a wide, quick solo piano, then crisp, dry, mono beats over a bell-like held bass.
- *Structures from Silence*: one vast chord breathing for half an hour, two layers a little apart in tune.
- *Kankyo Ongaku*: quiet, round tones in a wide soft room, a bell that swells in, lots of air between the notes.
- *View From Above*: a calm, warm glide over a soft heartbeat in threes, the pads breathing every two minutes.
- *Enchanted Instrumentals and Whispers*: a warm, slowed-down memory of late-90s computer music under a soft, heavy beat.
- *Birth of a New Day*: a city at night through a rainy window, the floor arriving after a minute and a half.
- *Mirror's Edge*: cold, clean city air over a deep hum, a beat switching on and off on the phrase.
- *Music Has the Right to Children*: soft pads a little out of tune and drifting, over a dusty, slow beat.
- *Far Away Trains Passing By*: a steady beat at 100 under a wide, glowing wall that swings bright and dark every few seconds.
- *Lifeforms*: low, strange bells and slow pads over a kit that skips in threes.
- *Beat*: one low note, one slow kick a bar, and a bass that ducks deep and swells back.
- *Perdition City*: an empty city at night, a soft wide hush that drifts in pitch and swells.
- *Synthesist*: a warm sequencer that keeps going while a slow pad drifts and breathes over it.
- *Animal Crossing*: a tiny band in the next room, a small tune every hour, a shuffle in the small hours.
- *Bubble Universe!*: a busy bubble of synths in a soft room, held 13ths over major sevenths, huge slow filter rides.
- *Holographic Universe(s)!*: a round, bubbly kit in threes from the next room, major sevenths sliding, a deep pump.
- *Conscious Agents*: the bubble slowed and rounder, deep sine bass and long chords with an 11 held over them.
- *Triangular Reformat*: the same dreamy tunes re-dressed many ways, dusky and low, swaying in threes.
- *Pyramid*: a big band playing a long, patient tune with its highs turned down and a 13th on top.

**Wholesome takes** for these albums, where the batch draws on them: every
candidate comes from the gentle side (bells, a drift, colors, width, a pump, a
filter, a groove). Left out on purpose: the anti-phase and inverted stereo, the
fast bass vibrato, the chopped noise, the 12 dB loudness swings, the shouted
voice-like parts of Come to Daddy and Windowlicker, and every voice as a voice.

## Stereo: the sampler in the approved baseline

Four loops with the sampler's draws (below), mono against the stereo baseline
(`stereo_guard.py`), the same with nothing drawn:

| Measure (median of 4; range) | Today | With the sampler | Limit |
|---|---|---|---|
| Fold to mono, loudness | 0.00 LU | 0.00 LU | within ±0.5 |
| Fold, worst octave | 0.00 dB | 0.00 dB | within ±1 |
| Side under mid below 120 Hz | -37.5 to -71.8 dB | -37.0 to -59.9 dB | -20 or lower |
| Side/mid above 200 Hz | -11.0 to -31.1 dB | -10.7 to -30.2 dB | (baseline -18) |
| Correlation, median / 5th pct | 0.92 to 1.00 / 0.72 to 0.99 | 0.92 to 1.00 / 0.71 to 0.98 | 0.7+ / 0.3+ |
| Balance | within 0.11 dB | within 0.12 dB | within ±2 |

Every guard passes, as today's do. The side below 120 Hz rises on thaw #71
(-71.8 to -47.6 dB) because the space's chord spread takes a little of the low
bell; it stays 27 dB inside the limit. Nothing the guards see is placed: every
sketch is centered.

**Ears 13b's sampler** (dust #69, shrine #0, grove #47, thaw #71, tide #39,
each with its draws below), mono against stereo as above:

| Measure (5 loops) | Today | With the 13b sampler | Limit |
|---|---|---|---|
| Fold to mono, loudness; worst octave | 0.00 LU; 0.00 dB | 0.00 LU; 0.00 dB | ±0.5; ±1 |
| Side under mid below 120 Hz | -38.1 to -71.8 dB | -34.3 to -56.1 dB | -20 or lower |
| Side/mid above 200 Hz | -11.3 to -31.1 dB | -10.4 to -29.9 dB | (baseline -18) |
| Correlation, median / 5th pct | 0.94 to 1.00 / 0.72 to 0.99 | 0.89 to 1.00 / 0.70 to 0.98 | 0.7+ / 0.3+ |
| Balance | within 0.12 dB | within 0.17 dB | within ±2 |

Every guard passes. Shrine #0 (widePad) is the one that widens (-23.5 to
-12.4 dB above 200 Hz); thaw #71's side below 120 Hz rises (-71.8 to -48.8
dB), the chords' spread taking a little of twang as it took of lowBell.

## Broken Friends: what Animal Crossing's hours say

For the brain's game phase, not for building now. Animal Crossing's 24 hours
(`cards-pile-6.md`, "The hours, as one table") are the clearest record on the
list of music that follows the time of day while a world keeps its character.

**What changes from hour to hour:**

- **Tempo:** 69 to 151 BPM, but 19 of 24 hours at 90 to 115; the day slows by
  about 10 BPM (block medians 107, 106, 101, 97 from midnight); the slowest are
  9PM (69), 10PM (72) and noon (86).
- **Brightness:** the morning block is the brightest (median centroid 510 Hz for
  6AM to 11AM against 259 to 317 for the rest); 3PM is the brightest hour (1,237
  Hz, 8.3% above 2 kHz); 9PM, 2AM and 3AM the darkest (84 to 107 Hz).
- **Density:** 1.6 notes a second (11PM, the weakest beat) to 6.7 (2PM).
- **Swing:** a shuffle in six hours, four of them in the small hours (12AM, 3AM,
  4AM, 5AM, then 1PM and 4PM); straight in fifteen.
- **Harmony:** per hour, not by the clock: a held chord next to a changing one
  (1AM 0.4 settled chords a minute, 2AM 9.6).

**What stays:** a steady pulse every hour (0.86), two or four beats a bar, a
major seventh or sus2 as the top chord in 19 of 24, a quiet, dark, near-mono
small band, about 2:24 a loop, in tune, and every hour dying away at its end
(the last second 21 to 38 dB under the hour's level).

**How Winter and Cherry re-dress a tune:** the same tune, the same length to the
second and the same tempo (21 and 22 of 24 hours within 2 BPM), the same
pitch-class profile (cosine 0.95 or more in 19 of 24); a different sound: Cherry
26% darker (centroid 197 Hz against 339), Winter thinner at the top (0.2% above
2 kHz against 0.8%), both with more notes on top (9.6 and 18.6 chord changes a
minute at mix level against 5.8: ornaments). In five hours the two are the same
file. The season moves a color and the ornaments, not the tune.

**What a game could ask Driftloom's core for**, in today's request where it
exists (`Request::parse`):

| Ask | From the hours | Today's request |
|---|---|---|
| Keep a friend's theme | the tune, length and tempo held through every season | `theme=SEED` |
| The time of day as vibes | 0-5 dreamy, moonlit, hushed, tender; 6-11 sunny, bright, refreshing; 12-17 playful, whimsical, bubbly; 18-23 cozy, mellow, restful | vibe weights |
| Tempo by the hour | 90 to 115, about 10 slower by night, 69 to 72 late | `bpm=LO-HI` |
| Brightness by the hour | morning brightest, the afternoon peak, nights dark | the arc's pins (`arc.floor`, `arc.shape`); a brightness ask of its own is a lever for the game phase |
| Density | 1.6 to 6.7 notes a second | `layer.NAME=on/off`; a density ask is a lever |
| Swing | a shuffle in the small hours | none yet: a `swing` pin, and tripletKit (#1) as the shuffle |
| A season | the same theme, a darker or thinner dress and more ornaments | `voice.LAYER=...` for the dress; brightness and ornament density are levers |
| The hour turning over | the old loop dies away over a few seconds, the new one starts at full level | the core's changes at a bar or phrase with a crossfade or a ring (`dlrender --changes`) |

## Clips

On the orphan branch `ears13-clips` (a `clips/` folder and nothing else), never
opened as a PR or merged: download
<https://github.com/nowmichaelclark/driftloom/archive/refs/heads/ears13-clips.zip>.
The branch is deleted after Mikey's verdicts; the job lists and the sketches
rebuild every clip (`tools/ears/sketches/ears13/README.md`). Opus at 96 kbps,
16 files and `timestamps.json`, about 38 MB. Loop names are the corpus numbers.

- **00-everything** (`-without`, `-with`; 0:00 grove #62, 0:51 shrine #0, 1:42
  thaw #71, 2:33 tide #39): each loop with two of the five, as Mikey's rule has
  most loops take one or two: grove lowBell and deepDuck; shrine colorTone and
  padWow; thaw lowBell and wideSweep's wide ride; tide colorTone and deepDuck.
  The draws were chosen by hand to cover all five, not drawn by the generator.
- **01-lowBell** (`-without`, `-with`, `-with-swell`; 0:00 haven #38, 0:46 thaw
  #71, 1:32 grove #62, 2:18 dust #69).
- **02-padWow** (`-without`, `-with`; shrine #0, thaw #71, grove #62, undertow
  #45). Headphones help; it is subtle on the busy loops.
- **03-colorTone** (`-without`, `-with`; vapor #4, shrine #0, tide #39, grove
  #62).
- **04-deepDuck** (`-without`, `-approved-subDuck`, `-with`; grove #62, undertow
  #45, halcyon #48, dust #69). Listen with some low end.
- **05-wideSweep** (`-without`, `-approved-padSweep`, `-with-wide`,
  `-with-fast`; shrine #0, thaw #71, grove #62, undertow #45). Clearest on the
  first two.

**Ears 13b** (the branch `ears13b-clips`, a `clips/` folder and nothing
else, never opened as a PR or merged): download
<https://github.com/nowmichaelclark/driftloom/archive/refs/heads/ears13b-clips.zip>.
Deleted after Mikey's verdicts; `tools/ears/sketches/ears13/README.md` ("Ears
13b") rebuilds every clip. Opus at 96 kbps, 23 files and `timestamps.json`,
about 62 MB.

- **00-everything-13b** (`-without`, `-with`; 0:00 dust #69 with tripletKit
  and tuningOffset +18 cents; 0:51 shrine #0 with planingSevenths and widePad;
  1:42 grove #47 with dropSpotlight and dottedPulse's pulse, its floor drops
  forced; 2:33 thaw #71 with twang soft and tuningOffset -20; 3:24 tide #39
  with slowOpening, from 30 s in, so the floor arrives about 22 s into its
  segment). Chosen by hand to cover all eight.
- **01-tripletKit** (`-without`, `-with`; dust #69, bloom #74, tide #52,
  undertow #75: kits that play sixteenths, where the lever works).
- **02-widePad** (`-without`, `-with`; shrine #0, thaw #71, undertow #45,
  grove #62). Headphones or two speakers; in mono the two files are the same.
- **03-dottedPulse** (`-without`, `-with-pulse`, `-with-kick332`; undertow
  #45, dust #69, grove #47, undertow #1). The pulse is very quiet; kick332 is
  clearest on the first loop.
- **04-slowOpening** (`-without`, `-approved-lateFloor`, `-with`; 0:00 dust
  #69, 1:41 tide #39, 3:22 dust #49, 100 s each). The floor arrives 52 to 70 s
  into each.
- **05-planingSevenths** (`-without`, `-with`; shrine #0, tide #39, thaw #71,
  grove #47). Listen for the tune rubbing against the chords on thaw #71.
- **06-dropSpotlight** (`-without`, `-with`, `-with-swell`; 0:00 dust #9, 0:51
  glade #21, 1:42 tide #39, 2:33 grove #47; every file with floorDrop's shapes
  forced, so the drops are the same).
- **07-tuningOffset** (`-without`, `-with`; vapor #4 -32, dust #69 +18,
  shrine #0 -24, haven #38 +27 cents).
- **08-twang** (`-without`, `-with-soft`, `-with-slow`, `-with-bright`; vapor
  #70, thaw #10, shrine #13, thaw #71: no kit, slow, big rooms).

## Not checked yet

A phone's cost of the core build (the kit's cost column is a desktop's), the
lite budget, rerolls, a load partway through, renders longer than 50 s (padWow's
slower swing and wideSweep's 16 s ride want a long listen), refused notes per
voice (the core's player has no per-voice report yet; every sketch here is a
channel block outside the voice budget, or a pinned setting of one, so neither
can turn a note away), and lowBell and colorTone on arpeggiated loops (they hear
chords through the organ, which hears one note at a time there).
For Ears 13b's: twang and dropSpotlight on loops with a busy kit, twang on a
6/8 loop and over long chords, renders longer than 100 s (slowOpening's wait
plus a minute), the levers on a phone, and tripletKit's and dottedPulse's
beat by the meter tool (read here from the note list, not the audio).

## For the brain (if approved; see the note under Verdicts)

- **lowBell:** a new voice of the core, a struck inharmonic bell at D2 to C#3 with
  a struck and a swelling kind, its level by ear against the loop; a seeded
  per-loop choice like glass's. It needs the chord's root at each change, which
  the generator has.
- **padWow:** a modulated delay on the chords channel (a whole-channel block, as
  padSweep is), depth and rate as per-loop data.
- **colorTone:** in the generator, beside the ninth `plan.rs` adds: the 13 over
  major chords and the 11 over minor ones, on a drawn share of the chords, voiced
  above the chord; then the chords sound it themselves, arpeggiated loops
  included.
- **deepDuck and wideSweep:** options of the approved choices' settings
  (`data/choices.txt`), no new block.
- **The seven levered candidates:** the levers above first (sketch build only),
  then an Ears 13b to sketch them; widePad's lever also opens the late-opening
  width.
- **Anything a hands session must know:** a chord-following block hears chords
  only while the organ plays (`player/synth.rs`, `organ_chord`); a whole-channel
  block gets no side in a stereo render (`blocks/mod.rs`, `whole`); the core
  writes mono renders as one channel, and `stereo_guard.py` wants two.

- **Ears 13b, if approved:** slowOpening and tuningOffset as their
  levers are (a wider lateFloor range with a fade; a tuning draw at load).
  widePad as a space option with the ceiling raised and a 250 Hz highpass;
  its late-opening kind needs a moving space setting first. dropSpotlight as
  lowBell keyed by the generator's floor drop (an event, so it strikes on the
  drop's first beat). twang as a voice of the core. tripletKit, dottedPulse
  and planingSevenths need their levers' other halves (above) before a build
  can be judged fairly; the clips show what each does today.
- **For a hands session:** `dlrender --force CHOICE=today` does not force a
  choice off (`examples/dlrender/clip.rs`); the melody channel gets no note
  events; a sketch's own settings cannot be pinned, so a tempo-reading sketch
  needs a build per loop (twang's twelve).

## Files

`tools/ears/sketches/ears13/`: one folder per sketch (`block.rs`, `where.txt`),
`everything/` (the sampler's block and `use.sh`), `jobs/`, `make_jobs.py`,
`ab.py` (a clips index as a render2 one, for `sketch_check.py` and
`low_guard.py`), `make_clips.py`, `clip_plan.json`, `kit-report.txt`,
`clips-timestamps.json` and a README. The measurements are in
`docs/ears/raw/ears13-synthesis-raw.tar.gz` (no audio). Ears 13b added
`dropSpotlight/`, `dropSpotlightSwell/`, `twang/` (with `variant.py`),
`make_jobs_13b.py`, `clip_plan_13b.json`, `kit-report-13b.txt` and
`clips-timestamps-13b.json`; its measurements are in
`docs/ears/raw/batch-6b-raw.tar.gz` (no audio).
