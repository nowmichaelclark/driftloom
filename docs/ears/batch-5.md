# Ears batch 5: the first synthesis

Ears 8 (Opus 5.5, high), 2026-10-18. The first synthesis of the album cards:
16 records from piles 1 and 2 (`cards-pile-1.md`, `cards-pile-2.md`) turned
into a ranked list of candidates, the top six sketched offline and rendered
against today's `main` (cf67c3f, js/ unchanged since Ears 4's a23153d). The
sketch is `tools/ears/sketches/ears8-sketch.patch`, applied on top of
`ears4-sketch.patch` in a local copy only (never app code). JavaScript
engine, Let the loop wander on, corpus seed 20261009, 90 s renders with
stems. Clips are in `~/ears8/clips/` on Mikey's computer (not committed),
every segment matched to -18 LUFS.

## Verdicts

Mikey, 2026-10-18: every sketched candidate approved, and the
`00-everything` sampler too.

| # | Candidate | What it is | Verdict |
|---|---|---|---|
| 0 | 00-everything | The sampler: all of them as each loop would draw them | **Approved** |
| 1 | tunedKick | The kick lands on the key's note (G1 to D2), short as today or a long boom | **Approved** |
| 2 | lateFloor | After a load the kit and the bass wait 8 to 30 s, then arrive | **Approved** |
| 3 | floorDrop shapes | The approved floor drop gets three more shapes: stagger, stutter, long | **Approved** |
| 4 | pedalBass | The bass holds the key's note in a rhythm under moving chords, with one neighbor | **Approved** |
| 5 | subDuck | The bass alone dips at each kick, so kick and sub take turns | **Approved** |
| 6 | padSweep | A lowpass on the chord layer swings open and shut every 12 to 28 s | **Approved** |
| 7 | echoTime | The echo's time drawn per loop (today: always a dotted eighth) | **Approved** |
| 8 | dropSpotlight | A low bell or a held lead plays alone in a floor drop | Not sketched, still unreviewed |
| 9 | widePad | A wide, noisy low pad over a mono sub | Not sketched, still unreviewed (for the space session) |
| 10 | dottedPulse | A percussion part every three eighths against the bar | Not sketched, still unreviewed |

**How they go in (Mikey, 2026-10-18).** Keep today's behavior as one
option in each choice; never replace it. Each approved idea becomes one
option in a per-loop choice, seeded so a loop replays the same way.

- The floor drop's style can follow the bar length.
- A wandering loop (Let the loop wander) can vary the kick at section
  boundaries between today's F1 kick, the short tuned kick and the long
  tuned boom.
- subDuck applies only when the loop has both a kick and a sub.
- The stagger and stutter drop shapes need a bass line.
- Most loops get only one or two of the changes, not all of them.
- A kick swap in the middle of a loop needs a crossfade, or a wait for the
  old kick to ring out.

dropSpotlight, widePad and dottedPulse were not sketched, so they are
still unreviewed: a later ears session sketches them for Mikey's ears
before anything is built from them.

**Next:** a builder (hands) session writes this mixing logic. Ears
sessions don't put code into the app or the core (`docs/ears/plan.md`,
Piles and sessions).

**Listen in this order** (folder `~/ears8/clips/`, each file without first,
then with): `00-everything` (all six as each loop would draw them, for a
first impression), then `01` to `07` in order. Every file name says the
candidate, the loop and the order of its segments; timestamps are in
`timestamps.json` there and under each candidate below.

## How the candidates were chosen

Every card's Guess and [JUDGMENT] lines, Stem check and Reliability notes
were read, and a number was used only where its card calls the tool
reliable for it. **Repetition across records is the strongest evidence**: a
candidate needs three or more records with a measurement that holds (echoTime
is the one exception, two and some leads, kept because it costs almost
nothing); one record is a lead for later. Where a card flags its own number (an empty bass
stem, a tempo octave, a key margin under 0.02, tuning, a bell label under
60 Hz, a pumping depth tied to the wrong kick period) the number was
checked again or left out. Two checks needed the audio, and Ears 7's Demucs
stems were still on disk for the second half of pile 2, so the new
`tools/ears/kick_check.py` measured them directly:

- **Kick pitch and decay, per hit** (seven tracks): the strongest 30 to 130 Hz
  line 40 to 160 ms after each kick, and the low band's fall. Five of seven sit
  mostly on one note between G1 and D2 (Joyrex J4 C2 61%, Robot B1 A1 53%,
  Bradley's Beat Two G1 48%, Analogue Bubblebath 5 track 09 C2 31%,
  Analogue Bubblebath 3 .000890569 D2 24% and D#2 18%); AB5 track 02 sits
  lower (B0 38%) and track 05's boom moves (G1, F#1, A#1, A1, 12 to 19% each).
  Two decay families: short clicks at 200 to 330 dB/s (AB5 09 214, AB5 02 333,
  AB3 .000890569 261) and long booms at 43 to 79 dB/s (Robot B1 43, AB5 05 55,
  Bradley's Beat Two 61, J4 79). The cards' pitches agree within a semitone;
  their decays differ by up to three times (J4: 24 dB/s on the card), since a
  kick group on a card can hold sub bleed.
- **The bass around the kick** (eight tracks), measured on the bass stem
  itself rather than as the 70%-of-true pumping floor: two shapes. A short
  duck, 4 to 8 dB right after the kick and back within 150 to 250 ms (J4 -6
  dB at 85 ms; AB5 06 -5 dB at 100 ms, back by 200 ms; Bradley's Beat One -7
  dB at 150 to 200 ms, back by 250 ms). And a bass that plays between kicks
  (Robot A2: 20 dB down at the kick, loudest 330 ms after it on a 0.49 s
  kick; AB5 09: 7 to 10 dB down at the kick, loudest 230 ms after on a 0.44
  s kick). The 22 dB hard pump the card reads on Robot A2 is the second shape, a
  bass on the off-beat, not a gate.
- **The slow bass swell** (200 to 360 ms attacks on five cards) did not hold:
  a plucked sine reads a clean 30 ms, but J4's notes read 20 to 640 ms
  because the duck's recovery looks like a swell. It is not a candidate on
  its own (see Dropped).

## Candidates, ranked

Each with its reference, the measurement and how far to trust it, what it
would add, a yardstick a hands session can test, the guards it needs, and
the closest thing Driftloom has today. The sketch numbers come from
`sketch_check.py` (refused notes per voice, each layer's own level from
stems), `low_guard.py` (new: the 20 to 60 Hz and 60 to 120 Hz shares, and
the part's own loudness by ear) and `stereo_guard.py` (all six together in
the approved stereo baseline).

### 1. tunedKick: the kick on the key's note

- **Records (14 of 16):** every record but *Ventolin* and *Hangable Auto Bulb*
  (whose kicks have no clear pitch) has a kick or sub thump on a note: *On*
  (D-Scape, C#1 to D1), *SAW 85-92* (Xtal, 55 Hz), *SAW II* (track 12, on the
  bass's E and G#), *Classics* (A#1, A1, F#1, C#2), Polygon Window (A1 on the
  title track), *I Care Because You Do* (A#1 and B1; G1 to B1 on Alberto
  Balsalm), *Donkey Rhubarb* (G1), GAK (A1, A#1),
  *Analogue Bubblebath 4* (A1, C2, G1), *Analogue Bubblebath 3* (D2 and D#2),
  *Analogue Bubblebath 5* (C2; a long boom on 05), Bradley's Beat (G1),
  Bradley's Robot (A1), Caustic Window (C2 on Joyrex J4).
- **Reference:** *Analogue Bubblebath 5* track 09, 1:06.1 (C2, short) and track
  05, 0:08.4 (a long boom); Joyrex J4 1:08.8 (C2); Bradley's Robot B1 5:52.8 (A1).
- **Measurement and how far to trust it:** pitch under 60 Hz is good to about a
  semitone (`docs/EARS.md`); the per-hit check above puts five of seven checked
  tracks mostly on one note between G1 and D2, and gives two fall rates: short clicks at 200 to 330 dB/s and long booms at 43 to 79 dB/s.
  Kick groups on the cards can fuse sub bleed, so their decays are read as
  families, not exact rates.
- **Would add:** today's kick lands on F1 (it sweeps 128 to 44 Hz) whatever the
  key, and today's soft kick on 46 Hz. A kick on the key's note sits with the
  bass instead of under it, and the long boom gives booming loops the records'
  low thump.
- **The sketch:** today's kick and soft kick, the same sweep, landing on the key's
  note between G1 and D2 (49 to 73 Hz; a key of D# to F# takes its fifth). Drawn
  per loop, half and half: **short**, today's shape falling at 180 to 260 dB/s;
  **long**, today's punch, then a tail 7 to 17 dB down that rings at 45 to 80
  dB/s, choked by the next kick (one kick voice at a time, held in the budget
  0.36 s a hit, as the soft kick is; today's kick holds 0.45 s).
  Level by ear, not by peak: each kick's loudness (K-weighted energy) is matched
  to today's at load, and the long boom's tail is as loud as fits within +1 dB by
  ear and within +3 dB of today's kick's own energy under 60 Hz and at 60 to 120
  Hz. On the corpus: 43 loops have a kick, 25 drew short and 18 long.
- **Measured** (grove #62, undertow #45, dust #69, halcyon #48; cinder #12 has no
  kick and is untouched): on the rendered drum stems the kick lands on its note
  89 to 94% of the time (today's: F1, 83 to 87%) and falls 207 to 222 dB/s short,
  82 to 87 dB/s long (the records' booms read 43 to 79 dB/s by the same measure).
  Refused notes: 103 → 103, 272 → 272, 60 → 60, 144 → 140, both kinds (the
  shorter hold turns a few notes fewer away); the other layers within 0.2 dB
  (grove's texture +0.2 dB, the same reason). Loudness -0.24 to +0.08 LU short; +0.21 to +0.27 LU
  long, and +0.95 LU on dust #69, whose boom sits on C#2 (69 Hz), where the ear
  is a little keener. The low end: short moves energy up out of 20 to 60 Hz
  (grove 16% → 12%, halcyon 12% → 5%) and adds 0.4 to 1.7 points at 60 to 120
  Hz; long keeps 20 to 60 Hz within 0.2 points of today on the A#1 loops (its
  loudest 3 s up 1 to 2 dB there) and on dust #69 moves 8 points from under 60
  Hz to 60 to 120 Hz.
- **Two rounds in this session, so the brain knows why the recipe looks this
  way:** a first long boom matched only by peak made the loop 2.6 to 3.2 LU
  louder and doubled grove's sub share; matched by ear but scaled as a whole, it
  lost today's punch (the drum layer 3.6 to 4 dB down). Today's punch plus a
  guarded tail is the version in the clips. A boom held in the budget for its
  whole length cost grove two pad notes; one kick voice at a time, held 0.36 s
  a hit, costs none.
- **Yardstick:** `kick_check.py kick` on the drum stem: 85% or more of kicks on
  the drawn note, 40 to 160 ms after the onset; falls within 20% of the drawn
  family (short 180 to 260, long 70 to 100 dB/s to 30 dB down); the loop within
  ±0.5 LU of today (+1 for a boom above B1); the kick's energy under 60 Hz and
  at 60 to 120 Hz within 3 dB of today's kick; refused notes no more than today.
- **Guards:** the level and band guards above, built into the recipe; centered
  (the kit's kick is never placed in the stereo baseline); the core plays the
  kick as one voice that each hit retriggers.
- **Closest today:** `synth.drum('kick')` and `'softkick'`.

### 2. lateFloor: the floor arrives late

- **Records (10):** *On* (track 01: pads alone, then a quiet floor, the drums
  at 0:54 and the bass at 1:16), *SAW 85-92* (opening builds of 20 to 38 s on
  all four), *Classics* (a 13 s intro with no sub on all three; Polynomial-C's
  full floor at 0:58), Polygon Window (the title track's sub in steps over 26
  s), *Children Talking* (the bass at 0:19.5), *Icct Hedral* (no low end for 41
  s), *Analogue Bubblebath 4* (Cuckoo 30.5 s, 01 15.5 s, 04 21.5 s),
  *Analogue Bubblebath 3* .000890569 (35 s), Bradley's Robot A1 (54 s),
  *Analogue Bubblebath 5* (06 24.5 s, 09 30.5 s).
- **Reference:** *Analogue Bubblebath 5* track 09, 0:00 to 0:36 (the floor
  enters about 30 s in, after a build from 0:13); Cuckoo 0:00 to 0:30.5.
- **Measurement and how far to trust it:** the low end of all stems summed,
  15 dB down for 2 s or more, and the stem entries; edges within 0.5 s on
  test signals. Intros are counted as intros by every card (none is a
  stems-only count), so these are sound. Typical length 13 to 35 s, about 22.
- **Would add:** an arrival. Today every loop starts with everything in it;
  the records open with the pads and let the beat come to them. It also gives
  a long listen a first minute that differs from the rest (the travel
  yardstick, `docs/EARS.md`).
- **The sketch:** after a load (not a reroll, which keeps its place), the kit
  and the bass sit out for 8 to 30 s, rounded to two bars, then enter
  together (40%), the kit first and the bass two bars later (35%), or the bass
  first (25%). Drawn for 3 loops in 10 that have drums or bass and something
  else that plays from the first second (29 of the 80 corpus loops).
- **Measured** (glade #21, undertow #45, halcyon #48, dust #69): entries after
  12.4 to 26 s as drawn; the loop 0.6 to 1.0 LU quieter over 90 s (the intro);
  refused notes 28 → 26, 272 → 260, 144 → 144, 60 → 57. Halcyon's 144 has one
  texture note refused in place of a voice note in the first 10 s, where the
  chords and tune already sit at the soft cap (160 to 168 of 170 units): with
  the kit silent the synth's own random draws (the hats' noise) shift, so a
  different soft note lands at the cap. A diagnostic run showed the three
  refusals that differ all fall in that window.
- **Yardstick:** drums and bass absent from the load for the drawn time
  (within one step), present after it; the other layers' notes and levels
  unchanged (stems: chords, melody and texture within 0.5 dB); refused notes
  equal or fewer in total.
- **Guards:** never on a loop with nothing but drums and bass (it would start
  silent); not on a reroll or a resumed loop; the vibe weights below.
- **Closest today:** the entry schedules (`genForm`): a layer can start out for
  2 to 6 bars, but on every pass and each layer on its own.

### 3. floorDrop shapes: stagger, stutter, long

- **Records (11):** stutter: *Acrid Avid Jam Shred* (3.5 s drops every four
  bars, three times, 3:00.5 to 3:12.5), *Ventolin*'s crowsmengegus (four drops
  5.5 s apart, twice), praze-an-beeble (one every 11 s), *Classics*'
  Polynomial-C (three 3 s drops in 14 s), *Analogue Bubblebath 4* track 04
  (four in 26 s). Stagger: Cuckoo (drums back at 4:13, bass at 4:35),
  *Analogue Bubblebath 4* track 03 (drums back 3 to 4 s before the bass),
  *Analogue Bubblebath 5* track 01 (drums back 5:26.5, bass 5:56.5), *On*.
  Long: Polygon Window (43 and 46 s), *SAW II* track 15 (44 s, a hard return),
  *Cordialatron* (30 s), *Analogue Bubblebath 5* track 01 (39 s), Bradley's
  Robot A1 (11 to 23 s), Bradley's Beat (12.5 s).
- **Reference:** *Acrid Avid Jam Shred* 2:51 to 3:17 (stutter, the bass in at
  3:17); Cuckoo 4:06.5 to 4:35 (stagger); Polygon Window 2:05.5 to 2:48.5 (long).
- **Measurement:** summed stems, 15 dB down for 2 s or more, edges within 0.5
  s; the stems-only count undercounts (every card says so) and is not used.
- **Would add:** the approved floor drop (Ears 1: kick and bass out for one or
  two bars, at 0.35 a pass) in more than one shape, so a long listen doesn't
  hear the same gesture each time.
- **The sketch:** Ears 1's approved recipe ported as `floorDrop: 'on'`; with
  `'plus'` each drop draws a shape: single as approved (40), stagger (25: the
  kick back on time, the bass one or two bars later), stutter (20: three or
  four drops of one bar, or two at fast tempos, one every two or four bars), long
  (15: 10 to 20 s, or the whole pass in a loop of 8 bars or fewer). Same rate,
  same passes, same rests turned into drops; only the shape differs. Across the
  corpus (1,600 passes): 48% single, 24% stagger, 14% stutter (it needs room),
  14% long.
- **Measured** (grove #62, wayfare #23, undertow #45, glade #21, at 0.5 a pass
  for the clips): against the approved recipe at the same passes, refused
  notes equal or fewer everywhere (grove 103 → 97 to 103, wayfare 543 → 531 to
  543, glade 28 → 27 to 28, undertow 321 in all). Undertow's 321 against today's
  272 comes from the approved recipe itself: a full stop no longer silences the
  chords there, so they ask for more in a loop that already loses 272 of 1,259
  notes. Loudness over 90 s -0.1 to -0.7 LU.
- **Yardstick:** per drop, as drawn: stagger, the bass back one or two bars
  after the kick; stutter, three or four drops of 2.5 to 4.5 s, one every 5 to 7
  s; long, 10 to 20 s; the rate per pass unchanged; never a full stop; tails ring.
- **Guards:** loops of 4 bars or more only (as approved); refusals no more than
  the approved recipe's.
- **Closest today:** drift's rest (`drift()`), and the approved floor drop.

### 4. pedalBass: one note under moving chords

- **Records (10), bass stems with real notes only (6% of the power or more):**
  *On* 01 (A# 96% of the bass's time), *SAW 85-92* Xtal (A 94%), *SAW II* 15 (B,
  87% repeats), *Acrid Avid Jam Shred* (G 76%), *Icct Hedral* (F and F#: the
  flat second), Pancake Lizard (G 38% with F and C), *Analogue Bubblebath 4*
  04 (A 67%, B 29%: a whole tone up), Bradley's Beat One (E 41%,
  F 23%), *Cordialatron* (E 98%), *Analogue Bubblebath 5* 01 (E 94%, plucked,
  1.5 notes a second) and 06 (A# 93%).
- **Reference:** *Analogue Bubblebath 5* track 01, 0:20 to 0:32 (E2, a plucked
  sine); *Analogue Bubblebath 4* track 04, 1:18.8 (A1, stepping to B1);
  Bradley's Beat Part One, 5:39 to 5:52 (E with the flat second, F).
- **Measurement:** bass behavior classified 100% on test signals, pitch within
  0.01 semitone, when the bass stem has notes; cards whose bass stem is empty
  or thin (D-Scape, Hedphelym, Quino-phec, *Analogue Bubblebath 4* 03 at 2.4%
  and others) were left out.
- **Would add:** the drone style already holds the tonic (and its fifth) under
  the chords in one profile (tide, 1 loop in 3); held, pulse, dub and walk all
  follow the chord's root. The records' pedal plays the tonic in a rhythm
  while the chords move, and steps to one neighbor and back now and then.
- **The sketch:** a bass style `pedal` that takes over about 3 in 10 held,
  pulse, dub and walking basses (22 of the 80 corpus loops): the tonic in a
  rhythm drawn per loop (held, half bars, beats, or off the beat between the
  kicks, as on Robot A2 and AB5 09), stepping in 10 to 25% of the notes to one
  neighbor drawn per loop: a whole tone up, the tone below, the flat second in
  a mode that has one, or the fifth. It keeps the loop's bass voice, the
  octave nearest its own bass and no note longer than its usual one; drift's
  passing tones still apply, and drift replays the original bass's draws so
  no other layer's variation moves (checked: 0 of 24 passes differ).
- **Measured** (dust #69, undertow #8, haven #38, clockwork #65): refused notes
  60 → 55, 231 → 231, 0 → 0, 34 → 34; the bass layer 0.6 to 2.1 dB quieter;
  the other layers within 0.1 dB. The low end moves either way: where only one
  octave of the tonic fits the bass's range (E1 to E3), the pedal sits up to an
  octave from the loop's bass (haven #38: under 60 Hz 39% → 2%, 60-120 Hz 33%
  → 63%; dust #69 26% → 10% and 58% → 75%; undertow #8 the other way, 38% →
  44%). With roundBass approved (bass at 36 to 57), the build's range differs
  anyway; the guard below holds there.
- **Yardstick:** the bass on the tonic 75 to 90% of its notes, its neighbor 10
  to 25%, 0.4 to 2 notes a second; the chords above unchanged; refused notes
  no more than with the loop's own bass.
- **Guards:** the low-end shares within 3 points of the loop's own bass in the
  build's register; the flat-second neighbor only in modes that have it.
- **Closest today:** the drone and pulse bass styles.

### 5. subDuck: kick and sub take turns

- **Records (7):** checked on the stems (kick_check.py): Joyrex J4 (the bass
  -6 dB 85 ms after each kick, back by 200 ms), *Analogue Bubblebath 5* 06 (-5
  dB at 100 ms, back by 200 ms), Bradley's Beat One (-7 dB at 150 to 200 ms,
  back by 250 ms). From the cards, the kick period fitting the beat:
  *Acrid Avid Jam Shred* (about 7 dB), *Every Day* (about 8 dB), *Analogue
  Bubblebath 4* 04 (about 8 dB), *Analogue Bubblebath 5* 09, GAK 2.
- **Reference:** *Analogue Bubblebath 5* track 06 (any minute of it); Joyrex J4,
  3:44 onward.
- **Measurement:** the cards' pumping depth reads about 70% of the truth and
  sometimes locks on the wrong period; kick_check.py measures the bass stem's
  own level around each kick instead. A separator can trade sub between the
  kick and bass stems, so read the depth as a guide.
- **Would add:** today's pump ducks every layer but the drums, and only
  undertow asks for it (0.55). This ducks the bass alone, so pads and tunes
  stay steady while the kick and the sub take turns.
- **The sketch:** at each kick the bass channel dips 5 to 9 dB (scaled by the
  kick's velocity) and comes back in 0.15 to 0.25 s; drawn for half the loops
  with a kick and a bass (20 of 80).
- **Measured** (grove #62, undertow #45, halcyon #48; cinder #12 has no kick,
  so nothing happens): the bass 4.6 to 5.4 dB down 25 ms after each kick
  against the same moment without it, back within 0.5 dB by 150 ms; the bass
  layer 0.4 to 0.7 dB quieter over 90 s; no refusals; loudness within 0.1 LU.
  It is subtle: the part sits 21 to 27 LU under the loop.
- **Yardstick:** the bass stem 5 to 9 dB down 10 to 50 ms after each kick
  (against the same loop without it), back within 1 dB by 0.25 s; other layers
  untouched.
- **Guards:** only with a kick and a bass; no deeper than 9 dB (the 20 dB
  gates the cards read are a bass playing off the beat, see Dropped).
- **Closest today:** the pump (`synth.duck`, the pump bus).

### 6. padSweep: the pad opens and closes

- **Records (3 at stem level, 3 more at mix level):** *Analogue Bubblebath 3*
  .942937 (the other stem falls 3.5 octaves in 7 s), Caustic Window *AFX 114*
  (the other stem 2.7 to 4.1 octaves over 5 to 9 s), Pancake Lizard (3 to 4
  octaves every 10 to 20 s with the bass on throughout); at mix level
  *Analogue Bubblebath 5* 01 (6.5 octaves over 14 to 25 s), GAK 2 (a 30 s arc),
  Hedphelym (a 12 s cycle).
- **Reference:** Pancake Lizard 0:08 to 1:17; .942937 0:12 to 0:34.
- **Measurement:** sweeps calibrated (4.3 octaves read 4.2); whole-mix sweeps
  on drum tracks are mostly hats coming and going, so only the stem-level ones
  and Pancake Lizard (bass steady) are claimed.
- **Would add:** the approved arc is one slow lowpass over the whole mix every
  2 to 5 minutes; this is the chord layer alone, every 12 to 28 s, while the
  rest stays as bright as it was.
- **The sketch:** a lowpass on the chord layer, starting closed at 450 to 800
  Hz and swinging 2 to 3.5 octaves up and back once every 12 to 28 s (Q 0.7 to
  1.2); drawn for 3 in 10 loops whose chords are a sustained voice (10 of 80).
- **Measured** (grove #62, undertow #45, shrine #0, thaw #71): no refusals; no
  change in loudness (0.00 to 0.01 LU) or in any other layer; the part sits 13
  to 28 LU under the loop on busy loops and level with it on sparse thaw #71,
  where the pad is most of the loop.
- **Yardstick:** the chord layer's brightness swinging two octaves or more
  with the drawn period; the other layers and the refused notes unchanged.
- **Guards:** sustained chord voices only; the vibe weights keep it away from
  still and restful loops.
- **Closest today:** the approved arc; the wash's own sweep.

### 7. echoTime: the echo's time, per loop

- **Records (2 at stem level, 4 more as leads):** *On* (track 01's other stem
  echoes one beat at 135 BPM, 454 ms; D-Scape's an eighth, 220 ms) and Polygon
  Window (one beat at 89.1, 674 ms); a dotted eighth (today's) on *SAW II* 15
  and *If It Really Is Me*. Leads, on drum stems only: one beat on *SAW 85-92*'s
  Tha, Bradley's Beat One and *Analogue Bubblebath 5* 04; a triplet eighth on
  *Cordialatron*; a dotted quarter on *Classics*' We Have Arrived. Many
  drum-stem echoes on the cards are the pattern itself and were not counted.
  The weakest evidence of the seven, kept because it costs almost nothing.
- **Measurement:** echo times within 4 ms where the tool finds six notes
  ringing alone; rare on busy stems.
- **Would add:** every loop's echo today is three quarters of a beat.
- **The sketch:** drawn per loop: 3/4 (45%), one beat (25%), half a beat (15%),
  two thirds (15%).
- **Measured** (shrine #0, haven #38, one beat and half a beat): no refusals,
  loudness unchanged; the change sits 16 to 20 LU under the loop. The quietest
  of the six: a detail for a long listen, not a headline.
- **Yardstick:** the echo time as drawn, within 4 ms (techniques.py's echo
  finder on the texture or melody stem).
- **Closest today:** `setEchoTime` at load (0.75 of a beat).

### 8. dropSpotlight: one part plays alone in a floor drop (not sketched)

- **Records (5):** Polygon Window, *If It Really Is Me* 4:04 to 4:23 (a low
  struck bell at F3, 175 Hz, carries the breakdown); Bradley's Robot A1 2:06.5
  (bell-pad notes at F#3 in each 11 to 23 s drop); *Analogue Bubblebath 3*
  .942937 4:07 to 4:36 (held F2 bell notes of 6 s in the drops); Caustic
  Window, *Fantasia* 2:22.5 (a held lead alone for 7 s); *Alberto Balsalm*
  2:09.5 (a mid pad stays through an 11 s break).
- **Measurement:** the bells' partials sit 76 to 98 cents off whole multiples
  at 88 to 186 Hz, above the 60 Hz limit, so the bell label holds; attack 31
  to 51 ms, falls 25 to 44 dB/s. The examples at +40 dB alone are clean
  because they sit in the drops.
- **Would add:** a reason for the floor to leave: something steps forward.
  Driftloom's floor drop leaves everything else as it was.
- **Why it waits:** it builds on the floor drop shapes (3) and needs a low
  bell voice: glass (approved, E5 to G6) two octaves down with a slower
  attack. Sketch it once 3 has a verdict.
- **Closest:** glass; the templebell and tubular voices.

### 9. widePad: a wide, noisy low pad over a mono sub (not sketched)

- **Records (8):** *SAW 85-92* (Tha, Hedphelym: the other stem's left and
  right correlate 0.31 to 0.46), *SAW II* tracks 21 and 16, *Quino-phec*,
  *Next Heap With*, *Icct Hedral* (Philip Glass orchestration), *Cordialatron*,
  *Analogue Bubblebath 5* track 08, Bradley's Robot A1. The pad sits at 85 to
  160 Hz with a 100 to 400 ms swell; the sub under it stays mono.
- **Measurement:** space by band and by stem is calibrated (width within 0.1
  dB); folding these pads to mono costs them 1.3 to 2.0 dB.
- **Would add:** width where the records have it, on the pad only. The
  approved baseline keeps the side about 18 dB under the mid; these pads
  are close to as wide as they are loud.
- **Why it waits:** it is a space recipe, and stereo isn't in the core yet.
  It belongs in the space session as a vibe-weighted option on top of the
  baseline (hazy, floating, spacious), A/B'd against the baseline, with
  the fold guard (within 0.5 LU) holding.
- **Closest:** the baseline's ensemble spread on the chords.

### 10. dottedPulse: a part every three eighths against the bar (not sketched)

- **Records (3, and a lead):** GAK 1 and GAK 2 (drum onsets repeat at 0.743 s and 0.372 s
  against a 1.985 s bar at 121 BPM: the dotted quarter and the dotted eighth,
  strengths 0.63 to 0.73), *Analogue Bubblebath 4* track 01 (onsets 1.5
  beats apart), *Analogue Bubblebath 5* track 07 (a kick every 1.5 beats).
  *Ventolin*'s asthma beats 4:3 is a lead only (not heard).
- **Measurement:** drum-onset repeats are how the cards settled every tempo
  octave; they are sound. What plays the dotted part (kick, hat, a stab) is
  not measured.
- **Why it waits:** drum patterns are the generator's own, and Driftloom has
  the move in places already (the bass pulse grid [0, 6, 10]; the 6/8
  cross-rhythm Mikey likes). Worth a generator look after the instrument
  batch.
- **Closest:** the bass pulse grids; the drum patterns' cross-rhythms.

## The [JUDGMENT] lines, settled for these candidates

Vibe weights use today's vibe names (`js/vibes.js`); the cards' guesses used a
few that don't exist (dreamy, deep, tense, sparkly, stately), mapped here to
the nearest real ones or left out. Toward is x1.5 to x3, away x0.2 to x0.5,
never 0; a vibe not named keeps x1.

| Candidate | Toward | Away | Wholesome |
|---|---|---|---|
| tunedKick, short | (none: a retune of today's kick, for every vibe) | | Yes. The low-end guard is built in (energy by band). |
| tunedKick, long | booming x2.5, hypnotic x2, steady x1.5, cool x1.5 | hushed, tiptoeing, twinkling x0.4; serene, soothing x0.5 | Yes, with the guard; harsh no. |
| lateFloor | hypnotic x2, meditative x1.5, spacious x1.5, wondrous x1.5, adventurous x1.5 | energetic, enthusiastic, skipping x0.4; bouncy x0.5 | Yes, as long as something plays from the first second (the guard). |
| floorDrop, stagger | (as the approved floor drop) | | Yes. |
| floorDrop, stutter | energetic x1.5, hypnotic x1.5, lively x1.5, mischievous x1.5 | serene, soothing, restful x0.4 | Yes. |
| floorDrop, long | hypnotic x2, spacious x2, meditative x1.5, reflective x1.5 | bouncy, skipping x0.5 | Yes. |
| pedalBass | hypnotic x2, steady x1.5, meditative x1.5, cool x1.5, mysterious x1.5 | joyful, triumphant, merry x0.5 | Yes. The flat-second neighbor leans mysterious and strange x2 and away from happy, sunny, sweet, peaceful x0.3. |
| subDuck | bouncy x2, energetic x2, lively x1.5, skipping x1.5, steady x1.5 | restful, serene, soothing, hushed x0.4 | Yes. |
| padSweep | hypnotic x2, floating x1.5, wondrous x1.5, hazy x1.5, strange x1.5 | still, restful, hushed, serene x0.4 | Yes. |
| echoTime | one beat: spacious, floating x1.5; half a beat: nimble, skipping x1.5 | | Yes. |

**New vibe or profile:** none needed for these. The four floor and bass ideas
(tunedKick long, pedalBass, lateFloor, floorDrop shapes) together sound like
the dub-pedal profile *On*'s card guessed at; worth proposing when profiles
open after phase 2, not before.

**Hooks**, one line a listener might say, for each album the batch drew on:

- *On*: a round, tuned sub-kick and a held bass note under slow major sevenths, echoing in time, close to mono.
- *Selected Ambient Works 85-92*: warm, slightly wobbling pads shimmering wide over a mono sub and a soft kick.
- *Selected Ambient Works Volume II*: slow low drones drifting wide with no beat, or a deep mono sub under a ticking machine.
- *Classics*: a tuned sub-kick under a long-held major seventh at about 143, the floor dropping out for a few seconds.
- Polygon Window, *Surfing on Sine Waves*: a deep tuned kick and gated sub under blurred chords, a long breakdown, a hard return.
- *...I Care Because You Do*: a tuned sub-kick and sub bass under drifting pads, the low end leaving and coming back.
- *Ventolin*: a rolling sine bass under dry ticks, the sub stepping out for a few seconds every few bars.
- *Donkey Rhubarb*: dry mono drums on a deep kick, and a wide synth-orchestra pad over a bass leaning on the flat second.
- *Hangable Auto Bulb*: very low sine bass under fast, dry, mono drums, with small mid-range accents.
- GAK: a deep tuned kick and a sub bass taking turns at 121, and almost nothing else.
- *Analogue Bubblebath 4*: a soft tuned kick and one held sub note under a quiet pad, the floor leaving and coming back.
- *Analogue Bubblebath 3*: a pad whose filter sweeps three octaves in seconds, and long floor drops with the pad ringing.
- *Analogue Bubblebath 5*: a bare sine kick and a plucked sine bass on one note while a filter slowly opens above them.
- Bradley's Beat: a tuned sine kick almost alone at 144 to 148, the floor falling out near the middle while a pad builds.
- Bradley's Robot: a slow, swung, near-mono groove whose bass plays between the kicks, and one wide side of bells over a vanishing floor.
- Caustic Window, *Compilation*: a mono shuffle at 148 on a C2 kick and a closing bass, one held lead left alone when the floor drops.

**Wholesome takes** for these albums, where this batch draws on them: every
candidate here comes from the gentle side (kicks, basses, floors, a pad's
filter). Left out on purpose: the bright tick kits and the 8.4 kHz tone,
width that cancels in mono, the 20 dB gates, and anything from a vocals
stem. None of these records is harsh in what was taken; the guard they all
need is weight (most are 60 to 95% under 120 Hz), which is why the tuned kick
and the pedal carry low-end guards.

## Dropped, one line each

- **Pitch wobble on pads** (5 to 30 cents at 0.4 to 4 Hz, 12 records): mostly
  there; the tape wobble's wow runs at 0.32 Hz with about 1 to 13 cents of
  depth (the profile's `wobble`), plus flutter at 6.3 Hz, and Ears 1 found
  wobble in range. The records' 1 to 4 Hz rates are at most a tune of that
  wobble, not a new block.
- **A whole-loop tuning offset** (up to a quarter tone: *Classics*, Polygon
  Window, *Hangable Auto Bulb*, Caustic Window): can't be heard inside a loop,
  only against the next one; Ears 1 already called it optional, and Bradley's
  Robot's +29 cents is the vinyl's speed.
- **A triplet shuffle** (0.62 to 0.67 on about a dozen tracks: Robot B1, *Analogue
  Bubblebath 5* 02 and 04, .000890569, Joyrex J4, GAK 1 and 2, Polygon Window and
  more): a lead, not dropped for good. The cards read swing at the meter tool's
  beat, which they show is often half or two thirds of the drum grid, so whether
  the records swing eighths or sixteenths is unsettled; Driftloom swings its
  sixteenths (`swing` per profile, up to 0.3 of a step in dust, which puts the
  off-sixteenth at 0.65). The next synthesis should check onsets against the
  drum grid (as kick_check.py does for kicks) before proposing it.
- **A long-held major seventh** (*Classics*' We Have Arrived holds one for up to
  60 s): covered by longChords (approved; the whole-loop length is 9%).
- **The lydian I to II pair** (Fmaj7 to Gmaj7 on *Analogue Bubblebath 3*):
  already there (progressions with degree 1 in lydian loops, sevenths spelled).
- **A slow-swelling sub bass** (200 to 360 ms attacks on five cards): the
  attack can't be read under 60 Hz, and J4's swell is the duck's recovery
  (kick_check.py); the fifths voice already swells in.
- **The gated sub** (*Quoth*: a 30 ms rise, a 180 ms flat top, a hard cut): one
  record.
- **Chord changes off the beat** (*On*'s 73-Yips and D-Scape, *SAW 85-92*'s Tha:
  on-beat shares under chance): pile 1 only, and the chord tool on dense pads
  may be following pad movement, not chords.
- **A drone a semitone under the tonic** (*Cordialatron*: E under F minor):
  one record, and a clash; strange vibes only, later.
- **Bright tick kits and the 8.4 kHz tone** (GAK 3, *Ventolin*'s asthma beats,
  *Analogue Bubblebath 3* .942937): the harsh part; Driftloom's hats cover
  what is wholesome in them.
- **Width that cancels in mono** (*Analogue Bubblebath 3* track 12, *AFX 114*'s
  hats, .000890569's air band): fails the fold guard by design.
- **A 20 dB sub gate** (GAK 2, Robot A2, *Analogue Bubblebath 5* track 07):
  kick_check.py shows Robot A2 and AB5 09 are a bass playing between kicks,
  not a gate; that part went into pedalBass as its offbeat rhythm.
- **Form spans of 4, 8, 16 and 32 bars**: the bar length is unclear on most
  tracks, so the lags are weak evidence.
- **Voice-like leads from the vocals stems**: never confirmed by ear; if any
  comes in, vowels and hums only, after a listen.

## Stereo: all six in the approved baseline

Four loops (grove #62, undertow #45, glade #21, dust #69), every sketched
candidate on (the long boom, the duck, the pedal, the sweep, the floor-drop
shapes), rendered mono and in the approved baseline (room3 and ens3 blended
0.64 and 0.5, then each recipe's width ceiling, which cut nothing), against the
same baseline with no candidate (`stereo_guard.py`):

| Measure (median of 4; worst) | Baseline alone | With the candidates | Limit |
|---|---|---|---|
| Fold to mono, loudness | 0.00 LU | 0.00 LU | within ±0.5 |
| Fold, worst octave | 0.00 dB | 0.00 dB | within ±1 |
| Side under mid below 120 Hz | -41.7 dB; -39.0 | -39.4 dB; -38.6 | -20 or lower |
| Side/mid above 200 Hz | -20.8 dB | -19.8 dB | (round 4: -18) |
| Correlation, median / 5th pct | 1.00 / 0.96 | 0.99 / 0.97; worst 5th 0.93 | 0.7+ / 0.3+ |
| Balance | within 0.01 dB | within 0.04 dB | within ±2 |
| Bounce (90th pct, dB per 100 ms) | 1.67 | 1.65 | lower is steadier |
| Width travel over 4 s (dB) | 1.0 to 14.1 | 1.0 to 13.2 | |

(The stereo set was rendered just before the boom's budget price was settled:
it held a beat there instead of 0.36 s, which can change which pad notes the
budget turns away, not where anything sits.)

Every guard passes, and the candidates change nothing the guard can see: the
kick, the bass and the floor stay in the middle. The width travel is the loops'
own (glade's layers come and go on their entry schedules), as large without the
candidates as with them.

## Across a load

Grove #62 with every candidate on, then undertow #45 loaded at 45 s as Next
does: the chords, tune and texture asked for exactly the same notes as the
same pair without the sketches (fm 187, pad 96, texture 31, voice 201), the
budget turned away 191 notes against 215, and the second loop drew its own
sketches, a second late floor included (its bass after 26 s). Nothing is left
behind from the first loop. (The pad sweep's handover across a load is not
checked: offline, a second load's filter settings apply from the render's
start.)

## Clips

Were in `~/ears8/clips/` (never committed; 79 MB, 29 MP3s and `timestamps.json`;
deleted after Mikey's verdicts, 2026-10-18). To hear them again, apply the two
patches to a copy of `cf67c3f` and re-render from the job lists and clip plan in
the raw archive (`work/make_jobs.py`, `work/clip_plan.py`); the timestamps are
kept there as `work/clips-timestamps.json`.
Every segment is matched to -18 LUFS, so a change in loudness is not what you
hear; a balance change is. Loop names are the corpus numbers Ears 4 used.

- **00-everything** (glade #21, grove #62, undertow #45, haven #38): today 0:00,
  then all six as each loop draws them 0:46. Glade drew a long boom on G1 and a
  late floor (the kit at 18.8 s, the bass two bars later); grove a short kick on
  A#1, the duck, a pad sweep and a 2/3-beat echo; undertow a long boom, the duck,
  a sweep and a one-beat echo; haven only a one-beat echo (it has no kick).
- **01-tunedKick** (grove #62, undertow #45, dust #69, halcyon #48): today 0:00,
  short 0:21, long 0:42. All on A#1 but dust (C#2).
- **02-lateFloor** (glade #21 kit first after 12.5 s, undertow #45 together
  after 26 s, halcyon #48 bass first after 12.4 s, dust #69 kit first after 12.6
  s): today 0:00, with it at 0:28 (undertow 0:42), each from the loop's start.
- **03-floorDrop** (grove #62, wayfare #23, undertow #45, glade #21): today 0:00,
  approved 0:17 to 0:18, stagger 0:34 to 0:37, stutter 0:51 to 0:56, long 1:18 to
  1:22. Wayfare is the clean A/B: today has a full stop at 0:06 of its first
  segment, and the approved recipe turns that same stop into a floor drop.
- **04-pedalBass** (dust #69 half bars with the fifth, undertow #8 half bars with
  a whole tone up, haven #38 half bars with the fifth, clockwork #65 held with a
  whole tone up): today 0:00, pedal 0:26.
- **05-subDuck** (grove #62, undertow #45, halcyon #48): today 0:00, with it 0:21.
  Listen on headphones or speakers with some low end; it is subtle.
- **06-padSweep** (grove #62, undertow #45, shrine #0, thaw #71): today 0:00, with
  it 0:41; 40 s each, about two sweeps.
- **07-echoTime** (shrine #0, haven #38): today 0:00, one beat 0:21, half a beat 0:42.

## Not checked yet

The Rust core, a phone's cost, rerolls of a single layer, the lite budget, and
renders longer than 90 s. The floor-drop clips use 0.5 drops a pass (the recipe
keeps 0.35) so each 90 s render has one. Echo times were not re-measured on the
renders (the tool needs notes ringing alone). Pile 3 onward may change the
ranks.

## For the brain (approved 2026-10-18; see the note under Verdicts)

- **tunedKick:** a kick building block with a target note, a fall rate, and an
  optional tail level; one kick voice per loop that each hit retriggers (choked),
  priced as one short drum hit. Its level and tail come from the guards at load, so
  they need no hand tuning per note.
- **lateFloor:** data on the loop (a start offset for the drums and bass
  layers, and an order), applied from a load only.
- **floorDrop shapes:** the drop's shape drawn at the end of drift's draws, so
  no other variation moves; build after the approved floor drop.
- **pedalBass:** a bass style with its own stream; drift must spend the
  original bass's draws (or drift's bass draws move to a stream of their own),
  or every later variation shifts.
- **subDuck:** a duck on the bass channel only, keyed by the kick: depth and
  recovery as per-loop data.
- **padSweep:** a lowpass with a slow LFO on the chord channel, per-loop data.
- **echoTime:** a per-loop draw at load.

## Files

`tools/ears/sketches/ears8-sketch.patch` (apply after `ears4-sketch.patch`; its
header lists every switch), `tools/ears/low_guard.py` and `kick_check.py` (new),
`tools/ears/render2.mjs` (logs the Ears 8 draws and drops), this report, and
the measurements behind it in `docs/ears/raw/ears8-synthesis-raw.tar.gz` (no
audio; since 2026-10-18 it also holds the session's helper scripts and render
logs). The renders and stems never left Mikey's computer and were deleted after
his verdicts; the patches and job lists rebuild them.
