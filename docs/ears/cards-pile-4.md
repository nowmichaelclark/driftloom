# Ears cards, pile 4

Ears 10 (run on Opus 5.5; the prompt was written for Sonnet 5.5, medium), 2026-11-02. Ambient and digital haze. The cards come from `tools/ears/album.py` (run with `python3 -P`) and are checked against the limits in `docs/EARS.md`. Lines marked **[JUDGMENT]** are for the synthesis; each carries a short first guess marked **Guess**. Vibe names come only from `data/vibes.txt`.

**Stems are Demucs htdemucs** on every card; there was no MDX-Net fallback. I checked the separator with `split.py --mode demucs` on a 20 s clip before the first album (0.58 times real time). Two tool changes, both needed for long tracks on a 7.6 GB machine (`tools/ears/README.md`):

- `split.py` now runs tracks over 11 minutes through Demucs in 10-minute pieces, with 10 s crossfades. A 10-minute piece peaks near 3 GB. On a 70 s test, the stitched stems differ from a one-piece run by no more than two one-piece runs differ from each other: Demucs is not repeatable from run to run (-10 to -23 dB of difference per stem).
- `check_stems.py` reads stems as float32. Float64 ran out of memory on a 17-minute track. The report is identical to the old one on a test clip.

**New tool: `tools/ears/longform.py`.** The brief asks for long form, slow swells and returns. The deep pass measures at the scale of seconds, so this tool looks at minutes. It takes the mix in 10 s windows and reads:

- level and brightness swells, smoothed over 60 s (count, spacing, depth, period);
- sections at the minute scale;
- returns: the share of windows that come back to material at least 2 minutes old;
- travel from the opening, and whether the end comes back to it;
- how often the strongest pitch class changes.

Its self-test reads a 120 s swell, a 240 s filter swing and an A-B-A form correctly. Beyond that it is not calibrated, so treat it as a guide. It ran on every deep-pass track and on every other track of 8 minutes or more. Its "change rate" (spectral flux against the window's own level, 0 = static) is only comparable within this pile so far.

**Twins.** I ran `twins.py` over the new picker rows, every earlier pick folder in `docs/ears/raw/` (unpacked) and `data.csv`. No pile 4 track pairs with anything: the 27 pairs it lists are the known Aphex repeats and some 240.0 s coincidences between 4-minute upload pieces. Pile 1 (Aphex Twin, 1992 to 94) shares no artist or title with this pile, so the check by hand finds nothing. Within the pile, no two tracks match.

| Album | Tracks (deep pass) | Machine time | Status |
|---|---|---|---|
| Steve Roach, Structures from Silence (1984) | 3 (3) | about 4,850 s | done |
| Kankyo Ongaku (compilation, 1980 to 1990) | 23 (4) | 1,120 s | done |
| Kyle Jameson, View From Above | 12 (4), cut from one 73:40 file | 1,515 s | done |
| Windows 96, Enchanted Instrumentals and Whispers | 12 (4) | 908 s | done |

---

## Steve Roach, Structures from Silence (1984)

Ears 10, 2026-11-02. Three tracks, 58.8 minutes, all given the deep pass (EP rule): 01 Reflections in Suspension (16:48), 02 Quiet Friend (13:23) and 03 Structures from Silence (28:38). `longform.py` ran on all three. Stems by Demucs htdemucs: 01 in two 10-minute pieces, 03 in three, crossfaded.

**Two measures ran on a 10-minute window, for memory:** the stem check on 01 (3:00 to 13:00), and the stem check and theory on 03 (9:00 to 19:00). Both full-length runs were killed for lack of memory. Every other stage ran on the whole track.

Machine time: about 4,850 s (81 min) for 58.8 minutes of audio:

- pick 298 s;
- Demucs 2,282 s (704, 499 and 1,079 s; 0.65 times the audio length);
- measuring about 2,270 s, of which about 260 s went to the three runs killed for memory, and 791 s to the space stage on the 28-minute track alone.

Ears 9's formula (1.1 x 3,528 s + 12 x 3) says 3,917 s, so this ran 24% over it. On tracks this long the measuring stages grow faster than the audio.

### Stem check (read first)
- **01 Reflections in Suspension** (window): drums 0%, bass 21%, other 74%, vocals 4.5% (bleed: the vocals stem plays 47% of the time, but the record has no voice; it is the pad's upper partials). No beat: meter, swing and pumping lines are void.
- **02 Quiet Friend**: drums 0%, bass 3.2% (thin: bass notes are guesses), other 97%. No beat.
- **03 Structures from Silence** (window): other 100%. Drums and bass are empty, so every beat number is void. Its other stem gave only 6 note events: the music is one continuous swelling drone with no onsets, so its sound is described below from its spectrum, not from note groups. The draft's two drums groups sit 40 and 59 dB under the mix: noise in an empty stem, left out.
- No drum track on the record, so no kick check.

### The hook
- Very slow, very wide, very still. Notes barely decay (mix-level tails -0.5 to -0.6 dB a second, the longest in any pile so far; Kankyo -2.9). It is quiet (-23 to -27 LUFS) and dark (centroid 324 to 368 Hz; at most 1.1% of the energy above 2 kHz). The channels are almost unrelated (width about 0 dB in every band of the title track). The harmony holds one home for a whole track (E on top for all 28 minutes of the title track).
- **[JUDGMENT]** One line a listener would say. **Guess:** a vast, slowly breathing chord that seems to have always been there; the album the glacial vibe is about.

### Sounds

**S1. The breathing drone (Structures from Silence), from the spectrum (Welch, 1 Hz bins, 9:00 to 19:00)**
- The strongest partials fall into two layers about 18 cents apart:
  - one at A440 tuning: G2 98 Hz, G3 196 Hz, G4 392 Hz, A4 440 Hz;
  - one 16 to 20 cents sharp: C2 66 Hz, E3 166 Hz, E4 333 Hz, F#4 374 Hz, D4 297 Hz, G4 396 Hz, A4 444 Hz.
- Where the layers share a note, they beat about 4 times a second (392 and 396 Hz, 440 and 444 Hz). The pitch set is C, D, E, F#, G, A (C lydian, the theory tool's reading too).
- The mix's other stem wobbles 16 cents at 1.8 Hz and drifts up to 26 cents in 4 s.
- Building blocks: **[JUDGMENT]** **Guess:** two detuned pad layers holding a lydian set, with no attacks at all, only level and brightness moving. softpad or moogpad doubled about 18 cents apart is the nearest; the missing piece is a pad that never re-articulates.

**S2. Slow-swell pad (Reflections in Suspension): other-1, 500 notes, 73%**
- In synth terms: harmonic share 0.72, attack 417 ms (93 to 1,908), falls 5 dB/s, brightness at the start 483 Hz and opening by 0.14 octave over the note. Fully decorrelated (correlation -0.03), level -4.3 dB. Register about 93 Hz (F#2), with the spectrum's strongest lines at 47 to 53 Hz and 94 to 125 Hz, all 20 to 35 cents sharp (the track's tuning reads +18 cents).
- Weak group (silhouette 0.17): one texture.
- Hear it at 5:11.8 (E2, 2.98 s), 11:16.6, 5:19.6.
- Building blocks: **[JUDGMENT]** **Guess:** softpad with a 0.5 to 2 s attack; exists, and longChords' whole-loop chord suits it.

**S3. Held sub tone (Reflections in Suspension): bass-1, 241 notes, 18%**
- In synth terms: about F#1 to G#1 (47 to 53 Hz; F# 47% and G# 36% of the time), purely harmonic (share 1.00, partials 4 cents off), falling only 1 dB/s, held for 2.7 to 6.6 s. Level -7.8 dB. Its pan leans +0.17, inside the track's +18 cent tuning; the mix's sub is -4 dB wide (correlation 0.55).
- Under 60 Hz pitch is good to a semitone only.
- Hear it at 2:05.0 (6.64 s), 2:56.4 (3.2 s), 5:58.1.
- Building blocks: **[JUDGMENT]** **Guess:** sub with bassStyle held; exists.

**S4. Soft pad with a low swell under it (Quiet Friend): other-1 and other-2, 290 and 89 notes**
- In synth terms: other-1 is about G#2 (103 Hz), attack 278 ms, falls 9 dB/s, correlation 0.45. other-2 is a low swell (attack 418 ms, falling only 4 dB/s, harmonic 0.84) that reads 35 Hz (C#1). The spectrum's strongest line is 69 Hz (C#2), so read it as about C#2.
- The other stem wobbles 15 cents at 3.6 Hz.
- Hear it at 9:06.9, 9:02.0, 8:58.7; the swell at 9:41.6, 10:07.5.
- Building blocks: **[JUDGMENT]** **Guess:** pad over a held C# pedal (the bass holds C# 71%); exists.

### Techniques
- **T1. Three long forms (`longform.py`, a line a minute).**
  - **Reflections in Suspension (16.7 min), an arch.** A 4-minute fade in (-62 to -27 dB), a plateau from minute 4 to 12 (-26 to -30 dB) over a B home, then a 4-minute fade out that also darkens (centroid about 450 to 250 Hz). Brightness swells twice (5:20 and 9:50, 0.66 and 0.52 octave). Minute-scale boundaries at 1:20, 8:10 and 14:50.
  - **Quiet Friend (13.3 min), a recession.** A 2.5-minute fade in, a plateau to about 8:00, then a second part 6 to 12 dB quieter and narrower (width 0 to -4.8 dB) for the last 5 minutes. Boundaries at 1:00, 8:50, 12:10.
  - **Structures from Silence (28.5 min), a held breath.** No real fade in (-32 dB in the first minute). The level stays within 6.5 dB (10th to 90th percentile) for 27 minutes, with five swells of 3 to 7 dB at 3:50, 6:40, 11:00, 20:10 and 22:40 (about 215 s apart). Brightness breathes ten times, every 140 to 150 s (autocorrelation 0.68 at 140 s, the clearest period in the pile), by 0.3 to 0.8 octave. The home pitch class is E in every minute. 94% of windows return to material at least 2 minutes old, and the end comes back near the opening. A fade only in the last 1.5 minutes.
  - **[JUDGMENT]** **Guess:** for glacial and Endless, the title track is the model: one home held for half an hour, a brightness breath every 2 to 2.5 minutes and a level swell every 3 to 4 minutes, nothing else. Glacial's deep arc (5 to 8 minutes) is slower and deeper; the record suggests a shallower, faster breath on top of it.
- **T2. Notes that do not end.** Mix-level tails of -0.5 to -0.6 dB a second: a note would take about a minute to fall 30 dB. No echo is found (no notes ring alone). The pads cross-fade into each other rather than decaying into a room. The arc at mix level is 6.7 to 12 dB, mostly the fades in and out.
  - **[JUDGMENT]** **Guess:** an option for the longest pad release Driftloom allows, in glacial and still.
- **T3. Slow pitch drift.** The bass's speed moves up to 24 cents in 4 s on Reflections; the title track's other stem 26 cents. Wobbles of 6 to 16 cents at 1.2 to 3.6 Hz throughout (floors: the tool reads low). The title track's other stem pans over a range of 0.68, cycling every 16 s: slower than the 3 to 4 s window artifact, so possibly a real slow drift; a listen decides.
  - **[JUDGMENT]** **Guess:** analog drift on held pads, a small slow pitch wander; take it with the detune pair of S1.
- The deep pass also lists 90 to 270 "brightness sweeps" of 0.7 to 2 octaves per track, lasting 5 to 17 s. On pads with no onsets these are the drone's colour changing, not filter moves; the 140 s breath above is the reliable reading.

### Theory
- **Th1. One home per track.**
  - Structures from Silence, C lydian (margin 0.030, on the 9 to 19 minute window): C 52% of the time and Amin7 24%, two colours of one pitch set. 9 settled changes a minute (the old counter 3), median chord 4.2 s; the strongest pitch class stays E.
  - Quiet Friend, C# dorian (0.038): C#5 44%, C#sus2 13%, C#min7 12%, over a C# pedal (71%, 85% repeats). 8 settled changes a minute.
  - Reflections, F# mixolydian by the tool (0.010, a guess); the spectrum's strongest notes are B, D#, E, F# and G#, and the long-form home is B: read it as a B-centered set. Chords B 21%, G#min 18%, Bsus2 13%.
  - **[JUDGMENT]** **Guess:** a draw of one chord, or two that share most notes, held for the whole piece, in glacial; lydian and dorian as the modes.
- **Th2. Lines are swells, not tunes.** The top-line tool finds 1,500 to 2,000 "notes" per track. On these drones they are partials moving inside the pad, not a melody, so no top-line claim. Thirds and fourths dominate the intervals (27% and 21 to 30%).

### Space
- **As wide as two channels can be.** Structures from Silence: about 0 dB in every band, the sub included (+1 dB, correlation -0.10), mono loss 3.2 dB. Reflections: 0 dB from 120 Hz to 2 kHz (correlation -0.02), -3 dB above, sub -4 dB (0.55). Quiet Friend: sub -8 dB (0.79), the mids 0 dB.
- Over time the width barely moves: -1/0/+1 dB (10th/50th/90th percentile of 2 s windows) on 01 and 03. In the last 5 minutes of Quiet Friend it narrows to -4.8 dB.
- Folded to mono, a fully decorrelated mix loses 3 dB and can comb; Driftloom's fold guard allows 0.5 LU.
- **[JUDGMENT]** **Guess:** a reference for the widest end of a calm space recipe above 200 Hz, never for the sub. The record would fail the fold guard and the centered low end as it is.

### Vibes
Numbers: Reflections, no beat, 11 settled chords a minute (5 by the old counter), 1.1% above 2 kHz, width -1 dB; Quiet Friend, 8 settled (3), 0.0%, -2 dB; Structures from Silence, 9 settled (3), 0.1%, 0 dB.
- **[JUDGMENT]** Lean toward (x1.5 to x3): glacial, still, meditative, spacious, serene, droning, hushed, reverent, cosmic, floating, restful. Lean away (x0.2 to x0.5): bouncy, plucky, skipping, zesty, lively, groovy, enthusiastic. Never 0. **Guess:** glacial's reference record.

### Wholesome take
- Take the held home, the slow breath of brightness and level, the detuned drone pair, the near-endless releases and the steady image, and the arch and recession forms for long pieces. Leave the decorrelated sub and the full 3 dB mono loss. **[JUDGMENT]** **Guess:** wholesome and calm through and through; the risk is only stillness becoming dull without the breath.

### For later
- Reflections bass-2 (45 notes, D#2, 7 dB/s) and bass-3 (16 notes, a bell-like G#1 with a quick attack); its vocals groups are pad partials, not a voice. **[JUDGMENT]** **Guess:** wait.
- The title track's 16 s pan cycle on the other stem (range 0.68). **[JUDGMENT]** **Guess:** listen; if real, a slow auto-pan reference for the later moving-stereo session.
- Full-length theory and stem check on 01 and 03 need a machine with more memory, or a theory tool that streams. **[JUDGMENT]** **Guess:** not needed; the windows agree with the long-form homes.

### Reliability notes for this card
- No beat anywhere: every meter, swing, on-beat and pumping number is void.
- The title track has no note groups; its sound comes from the spectrum.
- The stem check on 01 and 03, and theory on 03, cover a 10-minute window each.
- Top lines are partials, not melodies. Registers under 60 Hz are good to a semitone.
- The long-form readings come from a new tool checked only on test signals.

---

## Various artists, Kankyo Ongaku: Japanese Ambient, Environmental and New Age Music 1980-1990 (2019 compilation)

Ears 10, 2026-11-02. The folder holds 23 tracks, 114.4 minutes. `uploads.md` lists 25 tracks and 1:54:11, so the length matches and nothing looks missing; only the count differs (perhaps two tracks are joined in this edition). The files carry no artist tags, so this card names tracks only. Every track got the mix-level pass (picker). Four got the deep pass: 07 Park (typical, 4:06), and the three least like the rest, 14 Apple Star (6:01), 06 Variation III (3:36) and 22 Umi no Sunatsubu (1:12). `longform.py` ran on those four and on 23 Original BGM (15:55, the only long track, mix level only). Stems by Demucs htdemucs.

Machine time: 1,120 s (18.7 min) for 14.9 minutes of deep-pass audio:

- pick 277 s (23 tracks);
- Demucs 564 s (173, 210, 134 and 46 s; 0.63 times the audio length);
- measuring and the stem check 273 s;
- long form 6 s.

Ears 9's formula (1.1 x 895 s + 12 x 23) says 1,260 s, so this ran 11% under it.

### Stem check (read first)
- **07 Park**: power drums 0.7%, bass 0.4%, other 98%, vocals 0.5% (bleed). No beat: meter, swing, on-beat counts and the "pumping" line (bass 9.8 dB at a kick every 1.10 s) are void. The other stem's strongest low lines are 165 to 197 Hz, so its note groups' 74 Hz register is a low misread on a soft pad; read the pad as sitting around G3.
- **14 Apple Star**: drums 0.0%, bass 0.0%, other 97%, vocals 3% (bleed; the vocals stem plays 17% of the time in short runs, a listen decides). No beat: meter, swing, on-beat counts and pumping are void. The bass and other pan cycles of 6 s and 3 s look like the analysis window, not an auto-pan.
- **06 Variation III**: drums 14%, bass 2.4% (thin: bass notes are guesses), other 83%, vocals 0.7%. The drum grid is weak: every lag scores 0.21 to 0.25, so no grid stands out, and the meter tool's 109 BPM is unsupported. The low hits are sparse, so floor drops can't be read.
- **22 Umi no Sunatsubu**: drums 11%, bass 0.9% (void), other 88%. The meter tool's 115 BPM agrees with a weak drum grid (0.348 s, strength 0.16).

### The hook
- Soft, dark, close music: a median of 0.7% of the energy above 2 kHz across 23 tracks, and a median centroid of 410 Hz. The tails ring long (median -2.9 dB a second; Driftloom -4.5 to -8.8). The image is wide (side/mid median -4.2 dB). The harmony is slow (6.7 chord changes a minute by the mix-level count; 0.2 to 1.5 on six tracks). Most tracks have no beat.
- **[JUDGMENT]** One line a listener would say. **Guess:** quiet, round tones in a wide, soft room, one gentle idea per track, with lots of air between the notes.

### Sounds

**S1. Swelling bell-pad (Variation III): other-1, 106 notes, 77%**
- In synth terms: a slow swell (attack 506 ms, 95 to 1,617), falling only 4 dB/s, with partials off whole multiples by 69 cents (bell- or metal-like). Brightness at the start is 406 Hz. Wide: pan +0.07, correlation 0.24. Level -2.7 dB.
- Register: the tool reads 56 Hz, but the other stem's strongest lines are 99, 166 and 199 Hz, so read the register as about G2 to G3, not A1.
- Hear it at 3:21.0, 3:20.7, 3:21.1, and 2:02.9 for a 2 s held note from other-2.
- Building blocks: **[JUDGMENT]** **Guess:** softBell or templebell with a slow attack; the new part is a bell that swells in rather than strikes.

**S2. Soft pad (Park): other-1, 260 notes, 76%**
- In synth terms: harmonic (share 0.74), attack 304 ms, falls 11 dB/s, dark (311 Hz at the start), half-wide (correlation 0.55), level -1.3 dB. Wobbles about 13 cents at 1.5 Hz; the tool reads low, so that is a floor.
- Weak group (silhouette 0.23): one blended texture.
- Hear it at 0:40.7, 1:29.9, 1:31.8.
- Building blocks: **[JUDGMENT]** **Guess:** softpad, slightly detuned; exists.

**S3. Bright glassy keys (Apple Star): other-1 and other-2, 378 and 122 notes**
- In synth terms: soft attacks (116 and 164 ms). They start bright (2,129 and 1,997 Hz) and darken by 0.2 to 0.5 octave over each note. Fully decorrelated left and right (correlation -0.03 and 0.07), level -3 dB.
- This is the album's bright outlier: 34% of the energy above 2 kHz, against a median of 0.7%.
- Hear it at 1:01.1, 0:31.7 and 0:44.0 (low part); 0:31.9, 0:42.6 and 2:47.8 (G3 to G#3).
- Building blocks: **[JUDGMENT]** **Guess:** an FM or glass voice (glass exists) whose brightness falls over the note.

**S4. Tiny high plucks (Umi no Sunatsubu): other-1, 310 notes, 58%**
- In synth terms: around C6 (1,048 Hz; 312 to 1,682), harmonic (0.86), quick attack (23 ms), falling fast (104 dB/s). It leans right (pan +0.50 as a stem; the mix's 500 Hz to 2 kHz band is uncorrelated).
- Hear it at 0:00.0 (D#6, 0.96 s), 0:56.3, 1:02.7.
- **Kick line (kick_check):** 40 low hits at a median of 97 Hz (A#2 30%), falling 65 dB/s, about 1.0 s apart. The bass stem is empty (0.9%), so the duck is void.
- Building blocks: **[JUDGMENT]** **Guess:** kalimba, music box or pluck high in register; exists. Leaning one part to one side is against stereo's guards (nothing essential on one side), so take the sound, not the placement.

**S5. Low hand-drum hits (Variation III): drums, 21 hits**
- **Kick line:** a median of 87 Hz (F2 24%, A#2 14%; B0 14% is the 30 Hz floor), falling 52 dB/s, every 2.16 s. The thin bass stem is at its top 15 ms after each hit and falls about 12 dB by 300 ms: it is struck with the hit and decays, a guess on a 2.4% stem.
- The draft's pumping line (bass 4.6 dB at a kick every 0.53 s) locks onto another period: not trusted.
- Building blocks: **[JUDGMENT]** **Guess:** a soft, tuned low tom far apart; the spacing (one hit every two seconds) is the point.

### Techniques
- **T1. One move at a time, very slowly.** Sections run a median of 14 to 19 s at the deep-pass scale. At the minute scale, a 4-minute track holds one boundary (Park at 1:50, Variation III at 1:10). Original BGM (16 min) holds seven, about 110 s apart.
- **T2. Gentle level breathing, no filter arc.** Original BGM's level swells five times (3:30, 6:00, 8:10, 11:30, 13:50), each 3 to 6 dB, about 145 s apart. Its brightness moves only 0.3 to 0.5 octave (280 to 450 Hz minute by minute). Park swells twice (4.8 and 3.3 dB, 100 s apart). The mix-level arc is a median of 4.9 dB. The deep pass lists many 0.8 to 2 octave "sweeps" lasting 5 to 9 s, but on soft pads these are notes changing register, not filters.
  - **[JUDGMENT]** **Guess:** this is a slower, shallower cousin of the arc: a level breath of 3 to 6 dB every 2 to 3 minutes, with the filter nearly still.
- **T3. Returning, not travelling.** Original BGM comes back to material at least 2 minutes old in 96% of its windows, and its last minute is close to its opening. Its strongest pitch class settles on C from minute 6 on, and its drift ratio is 0.91 (it circles). The shorter tracks do travel: 18 of 23 have a drift ratio above 1 (median 1.13).
  - **[JUDGMENT]** **Guess:** for Endless, a home that the piece leaves and comes back to every few minutes is the album's model. It is not a journey.
- **T4. Echo and wobble.** No echo is claimed: only one repeat each, at 728 ms on Umi and 289 ms on Variation III. Variation III's other stem wobbles 21 cents at 5.7 Hz (a vibrato speed, not tape); Apple Star's 11 cents at 2.1 Hz.

### Theory
- **Th1. Slow harmony, spread wide.** Mix-level chord changes run from 0.2 to 1.5 a minute on Islander, Praying for Mother Earth, Nemureru Yoru, Snow, Nube and Ear Dreamin', up to 20 to 25 on Seiko 3 and Meet Me in the Sheep Meadow. Deep pass: Apple Star 12 a minute (11 settled, median chord 3.8 s, C#maj7 52% of the time, a bass pedal on C# 73%); Park 32 (15 settled, median 1.3 s; G5, C5, G, Csus2: open fifths and sus chords, no thirds); Variation III 17 (14 settled; Emin7, Fmaj7, Gsus2); Umi 22 (14 settled; D#maj7 47%). Chord names are a guess on soft music.
  - **[JUDGMENT]** **Guess:** draw the share of open-fifth and sus chords up, and the chord length long (4 to 10 s), in calm vibes.
- **Th2. Keys and scales.** Park G ionian (margin 0.025); Apple Star C aeolian (0.008, a guess; its pitch-class weights are flat across F, C, D, G and C#, so read it as unclear); Variation III G dorian (0.005, a guess), which also fits G pentatonic; Umi D# ionian (0.015, a guess). Tuning runs from -49 to +45 cents across the 23 tracks, as expected for a compilation from many sources. Pentatonic fits the plan's guess on one track of four only.
- **Th3. Top lines.** Park's line is mostly fourths and fifths (36%) and thirds (22%), with very few steps (2%) and arch phrases of about 10 notes. Apple Star's is mostly steps (28%) and repeats (32%). Variation III's falls or stays level (77%). Reliable only where the line sits above its accompaniment; on these pads, listen first.

### Space
- **Width grows with pitch.** Park: sub -10 dB (correlation 0.81), 120 to 500 Hz -5 dB, 500 Hz to 2 kHz -4 dB, above 2 kHz 0 dB (correlation 0.0). Umi: below 500 Hz near mono (-23 and -18 dB, correlation 0.97 to 0.99), 500 Hz to 2 kHz -1 dB, 2 to 5 kHz -11 dB, above 5 kHz 0 dB. Variation III and Apple Star are wide everywhere above 120 Hz (-3 to +1 dB). Apple Star's +4 dB sub is a quiet band next to a wide one, a known leak.
- Over time the width barely moves: Park -4.5 to -2.0 dB, Original BGM -5.7 to +0.8 dB (10th to 90th percentile of 10 s windows). That is a steady image, which fits the approved baseline.
- Mono loss of the main stem is 1.3 to 3.4 dB.
- **[JUDGMENT]** **Guess:** a space recipe of a centered low end and a soft room whose width rises with frequency; the record's 0 dB above 2 kHz is wider than the baseline's ceiling, so a calm vibe could lean to the wide end.

### Vibes
Numbers: Park, no beat, 15 settled chords a minute, 1% of energy above 2 kHz, width -5 dB; Apple Star, no beat, 11 settled, 34%, +1 dB; Variation III, 14 settled, 8%, -2 dB; Umi, 14 settled, 14%, -3 dB; Original BGM, 12 changes a minute (mix level), 0.0%, -2.6 dB.
- **[JUDGMENT]** Lean toward (x1.5 to x3): serene, peaceful, still, restful, meditative, airy, dewy, rippling, spacious, soothing, reflective, and glacial for the slow-breathing Original BGM. Lean away (x0.2 to x0.5): booming, spicy, enthusiastic, groovy, triumphant. Never 0. **Guess:** the calmest record so far; its home is the calm cluster.

### Wholesome take
- Take the soft round tones, the slow open chords, the long tails, the width that grows with pitch and the gentle breathing level. Leave Apple Star's brightness at its full level (34% above 2 kHz) unless a vibe asks for it, and Umi's part leaning to one side. **[JUDGMENT]** **Guess:** all of it is wholesome.

### For later
- 23 Original BGM (15:55, mix level and long form only): the album's long-form model, above. **[JUDGMENT]** **Guess:** worth a deep pass in a synthesis that builds Endless.
- Nemureru Yoru (karaoke version) (6:26), 34 pitch bends a minute (album median 3.3), and Loom (5:23), 24: something glides. **[JUDGMENT]** **Guess:** a gliding lead; listen first.
- Meet Me in the Sheep Meadow (3:26): the shortest tails on the album (-15.9 dB/s), near mono (-14.6 dB), 24.5 chord changes a minute. **[JUDGMENT]** **Guess:** a dry, plucked piece; waits.
- Seiko 3 (1:20): the biggest brightness sweep (2.5 octaves) and the biggest drift ratio (2.14). A Dream Sails Out to Sea, Scene 3 (6:48): a 1.8 octave sweep. **[JUDGMENT]** **Guess:** listen for a filter that opens.
- Glass Chattering (5:57): near mono (-13.7 dB), a 19 dB arc, the album's largest. **[JUDGMENT]** **Guess:** a build; listen.
- Islander (5:13): the darkest (centroid 139 Hz), 0.2 chord changes a minute. **[JUDGMENT]** **Guess:** a drone; listen.
- Groups left over: Umi other-2 (60 notes, C2, quick), drums-2 (129 notes); Apple Star vocals-1 (281 notes at F4, soft; bleed or a voice, a listen decides); Variation III bass-1 (23 notes, D3). **[JUDGMENT]** **Guess:** wait.

### Reliability notes for this card
- Park and Apple Star have no beat, so every beat number is void. Variation III and Umi have weak grids and void or thin bass stems, so their duck and pumping numbers are void.
- Registers under 100 Hz on pads read below the stems' strongest lines: read them as an octave or more too low.
- Keys: three of the four margins are under 0.02 (guesses).
- The long-form readings come from a new tool checked only on test signals; listen at the timestamps.

---

## Kyle Jameson, View From Above (remaster)

Ears 10, 2026-11-02. The album came as one continuous 73:40 file, so I cut it into its 12 tracks at the timestamps in the folder's track list (`audio/`, scratch only). Every track got the mix-level pass. Four got the deep pass: 02 Skywriting (typical, 5:59), and the three least like the rest, 04 The Dreams Remain The Same (6:07), 11 Your Amber Eyes (4:48) and 07 Uncharted Spaces (6:55). `longform.py` ran on those four, on 12 In Ascent / Breaking Barriers (9:02, mix level only) and on the whole hour as one piece. Stems by Demucs htdemucs.

Machine time: 1,515 s (25.3 min) for 23.8 minutes of deep-pass audio:

- pick 159 s (12 tracks);
- Demucs 854 s (218, 218, 171 and 248 s; 0.60 times the audio length);
- measuring and the stem check 456 s;
- long form 46 s, the whole hour included.

Ears 9's formula (1.1 x 1,429 s + 12 x 12) says 1,716 s, so this ran 12% under it.

### Stem check (read first)
- **02 Skywriting**: drums 52%, bass 27%, other 21%, vocals 0.0%. A real beat: the drum grid's beat is 0.731 s (82.1 BPM, strength 0.71), and the meter tool's 123 BPM is 1.5 times that; I use the grid. The bass stem's strongest lines are 49 to 58 Hz and 98 to 110 Hz: a real sub.
- **04 The Dreams Remain The Same**: drums 6%, bass 7%, other 87%. A light pulse: the grid's 0.267 s (224.7 BPM), and the meter tool's 112 is half of it.
- **11 Your Amber Eyes**: drums 1%, bass 29%, other 69%. No drum grid. The pulse here (the album's strongest at mix level, 0.91) is the bass itself, so the drum entries and pumping lines are void.
- **07 Uncharted Spaces**: drums 20%, bass 14%, other 62%, vocals 3% (bleed). The meter tool's 77 BPM agrees with the grid's 76.0 BPM (0.789 s).
- **The low end leans left on the master.** Under 120 Hz the left channel is 2 to 7.5 dB louder than the right on 9 of the 12 tracks; above 500 Hz the balance stays within ±2.4 dB. So the stems' pans (bass -0.71 on Skywriting; drums -0.44 and bass -0.55 on Uncharted) are the record's, not the separator's.

### The hook
- A steady, soft pulse on every track (mix-level pulse median 0.71, the lowest 0.51) under slow open chords, with the energy low and round: 59% below 120 Hz, a centroid of 324 Hz and 1.8% above 2 kHz. Long tails (-2.8 dB a second). Every one of the 12 tracks travels (drift ratio 1.10 to 1.49, median 1.19).
- **[JUDGMENT]** One line a listener would say. **Guess:** a calm, warm glide over a soft heartbeat, like slow flight over open land.

### Sounds

**S1. Swelling wide pad (Uncharted Spaces): other-1, 500 notes, 66%**
- In synth terms: attack 269 ms (58 to 768), falls 8 dB/s, harmonic share 0.70, brightness at the start 480 Hz. Wide: correlation 0.16, the mix -1 to -3 dB in every band. Level -3.3 dB.
- Register about C2 (66 Hz, matching the other stem's strongest line at 65 Hz). It wobbles 16 cents at 3.8 Hz.
- Weak group (silhouette 0.17): one texture.
- Hear it at 6:28.9, 6:29.7, 6:28.8.
- Building blocks: **[JUDGMENT]** **Guess:** moogpad or softpad, low and wide; exists.

**S2. Soft held pad (The Dreams Remain The Same): other-1, 372 notes, 63%**
- In synth terms: attack 201 ms, falls 9 dB/s, wide (correlation 0.14), level -2.7 dB; the mix is wide in every band, the sub included (-2 dB, correlation 0.18).
- Hear it at 1:51.4 (A2, 0.88 s), 1:58.1, 1:27.8 (A#2, 1.22 s).
- Building blocks: **[JUDGMENT]** **Guess:** pad; exists.

**S3. Repeating round bass that is the beat (Your Amber Eyes): bass-1, 460 notes, 34%**
- In synth terms: around F2 (87 Hz; 73 to 124), harmonic, attack 110 ms, falls 26 dB/s, mono (correlation 0.97), level -6.1 dB.
- It plays 1.49 notes a second with 78% repeats (F 39%, E 20%, D 18%). The drums stem is empty, so this repeating bass carries the pulse (0.91).
- Hear it at the start; for the other-1 keys above it (attack 112 ms, 15 dB/s, wide, correlation 0.09), 2:23.8 and 3:42.0.
- Building blocks: **[JUDGMENT]** **Guess:** roundBass with bassStyle pulse, the beat with no drums at all; the drumless pulse is the idea.

**S4. Low triplet pulse (Skywriting): drums-1, 367 notes, 27%**
- In synth terms: very short low hits (50 Hz, falling 601 dB/s, attack at the tool's floor), level -2.8 dB, leaning left (-0.17).
- **Kick line (kick_check):** 881 low hits at a median of 60 Hz (A1 16%, G#1 10%), falling 182 dB/s, every 0.250 s: a third of the 0.731 s grid beat, so a triplet pulse, not a four-on-the-floor kick. The bass stays within 7 dB of its top around each hit (-4.7 dB at the hit, lowest -6.9 dB at 210 ms): no duck, the bass plays through.
- The draft's pumping line (4.9 dB at 215 ms) is the same reading as a floor.
- Building blocks: **[JUDGMENT]** **Guess:** a soft, tuned low tick on a triplet grid; tunedKick's short kind is the nearest.

**S5. Half-wide mid pad (Skywriting): other-1, 499 notes, 29%**
- In synth terms: register about 111 Hz (A2; the other stem's strongest line is 111 Hz), attack 170 ms, falls 13 dB/s, correlation 0.39, level -6.6 dB.
- Hear it at 3:33.9 (A2), 4:09.6 (E2, 0.75 s, just before the floor drop), 3:49.4 (D3).
- **[JUDGMENT]** **Guess:** softpad; exists.

**S6. Low tom every second (Uncharted Spaces and The Dreams Remain The Same)**
- **Kick lines:** Uncharted has 243 low hits at a median of 86 Hz (G#2 14%), falling 66 dB/s, every 0.98 s. That is not the 0.789 s grid beat (1.25 times it), so listen for whether it sits on the grid. Dreams has 113 hits at 65 Hz (B1 20%, C2 16%) every 1.0 s. On both the bass stays within 2.5 and 4 dB: no duck.
- The draft's 33 Hz register for Uncharted's drums-1 is under the 40 Hz floor: read the kick line instead.
- **[JUDGMENT]** **Guess:** a soft low tom; waits behind tunedKick.

### Techniques
- **T1. An hour as a journey of rooms.** Across the whole album as one piece (`longform.py`, 10 s windows):
  - each track is its own room, 4 to 9 minutes long, with its own home pitch class (F for minutes 1 to 6, G for 7 to 9, E for 10 to 13, D for 21 to 26, F# for 28 to 32, D for 33 to 39, E for 55 to 59, and so on);
  - the joins are dips of 5 to 20 dB, under a minute long, at 6, 12, 20, 26, 32, 39, 46, 54, 59 and 63 to 64 minutes: short fades, not hard cuts and not seamless blends;
  - only 49% of windows come back to material 2 minutes old, the farthest point from the opening is at 63:50, and the end does not come home.
  - **[JUDGMENT]** **Guess:** for Endless, this is the other model from Kankyo's Original BGM: change the room (key, pad, pulse) every 4 to 9 minutes through a short dip, rather than circling one home.
- **T2. Brightness breaths of 2 to 3 minutes inside a track.** Skywriting's brightness swells at 1:40 and 3:50 (0.3 to 0.4 octave, 130 s apart); Uncharted at 3:00 and 5:20 (0.4 and 0.9 octave, 140 s); In Ascent at 3:50 and 7:00 (190 s). The level stays within 2.6 to 4.2 dB on Skywriting and Dreams. The mix-level arc is a median of only 2.8 dB.
  - **[JUDGMENT]** **Guess:** padSweep's 12 to 28 s period is far faster; this is an arc-like breath of 130 to 190 s and 0.3 to 0.9 octave.
- **T3. One floor drop, a slow tape-like bass wobble, two builds.**
  - Skywriting drops the drums and bass from 4:10.5 to 4:20.5 (the stems say 4:11.8 for 3.2 s and 4:14.8 for 6.2 s), with the pad going on.
  - The bass wobbles 18 to 19 cents at 1.1 Hz on Dreams and Uncharted (a floor: the tool reads low), and Uncharted's bass speed moves up to 7 cents in 4 s.
  - Builds: Uncharted 0:20 to 0:44, Your Amber Eyes 4:19 to 4:41.
  - No echo is claimed: Dreams' 264 ms drum echo is the 0.267 s grid pulse.
  - **[JUDGMENT]** **Guess:** floorDrop exists; the bass wobble could be a small drift on roundBass.

### Theory
- **Th1. Open fifths and sus chords, slow.** Skywriting: Asus2, D5, C5, A#sus2 (15 settled changes a minute; 35 by the fast counter; 59% on a beat against 62% chance). Dreams: C5, G5, Csus2, Gsus2 (14 settled). Uncharted: F5, Fmin7, C5 (11 settled). The mix-level count is a median of 8.8 a minute.
  - **[JUDGMENT]** **Guess:** like Kankyo's Park, a lean to fifths and sus2 in calm vibes, here under a beat.
- **Th2. Your Amber Eyes, the one sure key.** F lydian (margin 0.049, reliable): Fmaj7 37% of the time, Dmaj7 11%, Cmaj7 6%. 65% of its chord changes land on a beat against 39% chance, the strongest lock in the pile so far. The bass repeats F, E, D. The other keys are guesses: Skywriting A aeolian (0.017), Dreams G ionian (0.005), Uncharted C aeolian (0.007).
  - **[JUDGMENT]** **Guess:** lydian with a major-seventh tonic chord is a ready mode for dreamy and floating.
- **Th3. Bass.** Skywriting moves by roots (1.18 notes a second; fourths and fifths 26%, octaves 19%). Uncharted holds C 53% (81% repeats). Dreams G 36% (72% repeats). The meter tool reports 3 beats a bar on Your Amber Eyes and Uncharted, but with many unrelated periods (irregular timing), so no meter claim.

### Space
- **Wide all the way down, and leaning left below 120 Hz.** Dreams is wide in every band (sub -2 dB, correlation 0.18; mids -0 to -3 dB). Uncharted's sub is -3 dB (0.34) and Your Amber Eyes' -5 dB (0.52). Skywriting is the most centered low end (-9 dB, 0.90), and it is narrower in the air band (-8 dB) than in the mids (-3 dB).
- Over the hour the width holds between -3.9 and -0.6 dB (10th to 90th percentile).
- Mono loss of the main stem is 2.3 to 2.8 dB (the other stem); Skywriting's main stem, the drums, loses 0.5 dB.
- **[JUDGMENT]** **Guess:** take the steady, wide pads above 200 Hz; leave the wide sub and the left lean, both against stereo's guards (bass and kick centered).

### Vibes
Numbers: Skywriting 82 BPM (grid), 15 settled chords a minute, 2% above 2 kHz, width -6 dB; Dreams 112 or 225 BPM (a light pulse), 14, 1%, -1 dB; Your Amber Eyes 78 BPM (the bass), 15, 2%, -2 dB; Uncharted 76 BPM, 11, 2%, -2 dB.
- **[JUDGMENT]** Lean toward (x1.5 to x3): dreamy, floating, soaring, cruising, mellow, warm, velvety, steady, swaying, chill, hazy. Lean away (x0.2 to x0.5): zany, goofy, spicy, sour, tart, jazzy. Never 0. **Guess:** calm with a pulse, between chill and dreamy.

### Wholesome take
- Take the soft pulse (a triplet tick, a low tom every second, or the bass alone), the slow fifth-and-sus chords, the lydian colour, the 2 to 3 minute brightness breaths, and the hour of rooms joined by short dips. Leave the left-leaning, wide low end. The sub share (59% under 120 Hz) is above Driftloom's low-end yardstick (drum loops under about 35% at 60 to 120 Hz), so take the weight at a lower level. **[JUDGMENT]** **Guess:** all of it is wholesome; the low end needs taming.

### For later
- Across The Heartland (7:06): the most chord changes (23.1 a minute against a median of 8.8), a 1.3 octave sweep. **[JUDGMENT]** **Guess:** the busiest track; listen.
- In Ascent / Breaking Barriers (9:02): the last and longest. A 9.6 dB swell peaking at 3:30, a 1.5 octave brightness range, and the end coming back near its opening. Its tuning is -36 cents against the album's +17. **[JUDGMENT]** **Guess:** a build-and-return; worth a deep pass with the synthesis.
- Ocean Bleu (4:21): the darkest (centroid 169 Hz) and loudest (-11.7 LUFS). **[JUDGMENT]** **Guess:** a sub-heavy one to leave or tame.
- Groups left over: Skywriting bass-1 and bass-2 (F#1 and G2, leaning left); Your Amber Eyes other-2 (77 notes, bell-like C3, 9 dB/s); Dreams other-2 (34 Hz: under 40 Hz and half the other stem's 66.5 Hz line, so a subharmonic misread) and bass-1 (24 notes, G1, a 5 dB/s swell); Uncharted bass-2 (G1, 4 dB/s) and vocals-1 (214 notes at G4; bleed or a pad, a listen decides). **[JUDGMENT]** **Guess:** wait.

### Reliability notes for this card
- Your Amber Eyes' drum numbers are void (1% stem); its pulse is the bass. Dreams' grid is a light pulse (drums 6%).
- Uncharted's low tom doesn't sit on the grid beat by a simple ratio (1.25 times): listen.
- Pans below 120 Hz are the master's lean, not the separator's.
- Three of four keys are guesses. Registers under 40 Hz are misreads. Wobble depths are floors.
- The whole-hour long form runs across track joins, so its swells at a 6-minute spacing are the tracks themselves.

---

## Windows 96, Enchanted Instrumentals and Whispers (2019)

Ears 10, 2026-11-02. 12 tracks, 40.3 minutes, all at mix level. Four got the deep pass: 12 Lead (typical, 2:33), and the three least like the rest, 09 Wasting Time (3:59), 04 Venus Aire (4:49) and 10 Golden Triangle (2:40). `longform.py` ran on the four; no track is 8 minutes long. Stems by Demucs htdemucs.

Machine time: 908 s (15.1 min) for 14.0 minutes of deep-pass audio:

- pick 75 s (12 tracks);
- Demucs 530 s (117, 143, 171 and 99 s; 0.63 times the audio length);
- measuring, the stem check and the long form 303 s.

Ears 9's formula (1.1 x 840 s + 12 x 12) says 1,068 s, so this ran 15% under it.

**Voices.** The record samples voices. I write nothing of what is said, here or anywhere. Where a voice comes up, it is described as a texture: pitch range, level, breath, width.

### Stem check (read first)
- **12 Lead**: drums 36%, bass 37%, other 25%, vocals 1.4% (bleed). The meter tool's 92 BPM agrees with the drum grid's 92.3 BPM (0.650 s, strength 0.81).
- **09 Wasting Time**: drums 35%, bass 12%, other 41%, **vocals 12%**: a real voice texture (S6). The meter tool's 80 BPM agrees with the grid's 79.5 BPM.
- **04 Venus Aire**: drums 40%, bass 39%, other 18%, vocals 3% (bleed). The meter tool's 64 BPM is three quarters of the grid's 84.7 BPM (0.708 s); I use the grid, so the tool's swing number is void.
- **10 Golden Triangle**: drums 26%, bass 26%, other 44%, vocals 3% (bleed). The meter tool's 75 BPM agrees with the grid's 74.9 BPM.
- The bass stems' pans (-0.32 on Lead, -0.53 on Wasting Time) are not in the mix. Under 120 Hz the mix is within 1.5 dB of center on 11 of 12 tracks (Bonita Camisa +3.1 dB), so read those pans as the separator's.

### The hook
- Slow, heavy, hazy beats: grids at 75 to 92 BPM; loud (-12.3 LUFS median); 57% of the energy below 120 Hz; a centroid of 361 Hz; 1.5% above 2 kHz. Faster harmony than the rest of the pile (17 chord changes a minute at mix level), much of it minor sevenths. The bass slides in pitch like slow tape (10 to 20 cents in 4 s on three of four tracks). The image is narrower than the pile (-8.5 dB median), with the width rising with pitch.
- **[JUDGMENT]** One line a listener would say. **Guess:** a warm, slowed-down memory of late-90s computer music, under a soft heavy beat.

### Sounds

**S1. Half-wide soft keys (Golden Triangle): other-1, 438 notes, 54%**
- In synth terms: harmonic (string or reed-like), about E2 (82 Hz, matching the other stem's strongest line at 82 Hz), attack 102 ms, falls 21 dB/s, correlation 0.66, level -2.5 dB. The mix is -5 to -7 dB in every band above 120 Hz.
- Hear it at 0:01.2, 0:01.6, 0:02.9.
- Building blocks: **[JUDGMENT]** **Guess:** rhodes or keys, low and soft; exists.

**S2. Wide hazy texture (Wasting Time): other-1, 377 notes, 38%**
- In synth terms: about C#2 (70 Hz, the other stem's strongest line), soft attack (134 ms), falls 22 dB/s, decorrelated (0.07), level -5.8 dB.
- Weak group (silhouette 0.19): one blend.
- Hear it at 2:39.8, 0:00.0 (1.49 s), 2:39.5.
- Building blocks: **[JUDGMENT]** **Guess:** a pad with a wide, smeared top; the wash or a detuned softpad.

**S3. Slow-swell pad (Lead): other-1, 359 notes, 36%**
- In synth terms: about 101 Hz (59 to 194), attack 164 ms, falls 20 dB/s, correlation 0.43, level -7.0 dB.
- Hear it at 0:05.1 (D#2, 1.41 s), 0:09.7 (1.36 s), 0:00.7.
- Building blocks: **[JUDGMENT]** **Guess:** softpad or moogpad; exists.

**S4. Round, quick bass (Lead and Venus Aire): bass-1, 174 and 217 notes, 25% each**
- In synth terms: Lead about A#1 (59 Hz, the bass stem's strongest line), attack 38 ms, falls 29 dB/s, mono (correlation 0.99), 1.99 notes a second with 70% repeats. Venus Aire about F1 (44 Hz; strongest line 55.5 Hz, so read it as about A1 to F1), attack 68 ms, falls 30 dB/s, with 26% octave jumps.
- Hear Lead at 1:44.7, 1:34.2, 0:42.0; Venus at 3:50.4, 0:35.2, 2:20.3.
- Building blocks: **[JUDGMENT]** **Guess:** roundBass; the octave-jumping line (Venus) is a bassStyle option.

**S5. Punchy kicks pitched high (all four)**
- **Kick lines (kick_check):**
  - Lead: 168 kicks on G2 (median 92.5 Hz), falling 211 dB/s, every 0.655 s (the grid beat). The bass dips to -7.9 dB at 310 ms, between kicks, and is back at its top by 530 ms: a mild half-beat dip.
  - Wasting Time: 262 kicks on C3 (110 Hz), falling 273 dB/s, every 0.565 s (three quarters of the 0.755 s grid beat: listen). The bass is struck with the kick: -25 dB at the hit, at its top 50 to 120 ms later, then falling to -29 dB by 545 ms, a drop-per-kick bass.
  - Venus Aire: 376 kicks on B2 to C3 (116 Hz), falling 130 dB/s, every 0.705 s (the grid beat). The same shape, milder: -9 dB at the hit, top at 110 ms, -12.5 dB by 695 ms.
  - Golden Triangle: 161 kicks, a median of 50 Hz spread wide (35 to 130 Hz), every 0.400 s (half the grid beat). The bass plays through (within 4.3 dB).
- These kicks sit at 90 to 116 Hz, an octave above the tunedKick range (G1 to D2).
- Building blocks: **[JUDGMENT]** **Guess:** a short, high, tuned kick; tunedKick's short kind, moved up, with a bass that is struck with each kick and fades.

**S6. A breathy voice texture (Wasting Time): vocals-1, 216 notes**
- In synth terms: centered (correlation 0.98), median pitch about B3 (247 Hz; 100 to 490), half noisy (harmonic share 0.54, a breathy tone), brightness about 830 Hz, swelling in over 163 ms (45 to 480), phrases of about 0.7 s, 11 dB under the mix. The vocals stem plays 59% of the time, in short runs from 0:29.5.
- Building blocks: **[JUDGMENT]** **Guess:** a wordless, breathy vowel pad around B3, synthesized, never a sample: the choir or vowel family, kept low and centered.

### Techniques
- **T1. The kit comes in late.** Lead's bass enters at 0:12 and its drums at 0:19 (the stems read a floor drop of 6.5 s at 0:07.5). Wasting Time's drums and bass enter together at 0:23.5. Venus Aire's drums enter at 0:29.5. On three of the four, the kit arrives 12 to 30 s in.
  - **[JUDGMENT]** **Guess:** lateFloor (approved) is this move: these are reference tracks for it.
- **T2. Many floor drops.**
  - Golden Triangle: 2.10 a minute (0:25.5 for 4.2 s, 0:36.8 for 2.5 s, 1:05.0 for 11.5 s, 1:46.0 for 5.0 s, 1:52.5 for 6.2 s).
  - Wasting Time: 1.40 a minute (1:33.2 for 2.2 s, 2:00.2 for 5.2 s, 2:06.2 for 5.0 s, 2:38.2 for 3.8 s, after the 12 s intro).
  - Venus Aire: one (4:28.0 for 2.2 s).
  - **[JUDGMENT]** **Guess:** floorDrop and its stagger shape exist; Golden Triangle's 11.5 s drop is a long one.
- **T3. Tape drift on the bass.** The bass's speed moves up to 20 cents in 4 s on Venus Aire, 12 on Golden Triangle and 10 on Lead. Golden Triangle's other stem wobbles 14 cents at 3.8 Hz. The mix-level wobble is a median of 4.3 cents. Deep Swim's tuning reads -46 cents (nearly a quarter tone flat), a slowed-down source maybe; a listen decides.
  - **[JUDGMENT]** **Guess:** a slow pitch drift on the bass alone (not the kit) is new; it suits hazy.
- **T4. Builds, and no echo claimed.** Builds at Lead 0:08 to 0:28 and Golden Triangle 1:12 to 1:36. The drum "echoes" are the grid: 648 ms on Lead (beat 0.650 s) and 384 ms on Wasting Time (half its 0.755 s beat). Venus Aire's 484 ms (2 repeats) is about two thirds of its beat, too few repeats to claim. The other stem's 6 dB "pumping" peaking 30 to 35 ms after the kick is likely kick bleed in that stem: not claimed.
- **T5. Long form: song length.** Tracks run 2.5 to 4.8 minutes, each with one minute-scale boundary: Lead at 1:00, Venus Aire at 1:00, Wasting Time at 2:00. Each has at most one swell (Venus Aire's level at 3:00, 4 dB; Wasting Time's brightness at 2:30, 0.75 octave). They travel and do not return (returns 0.0 to 0.55; drift ratio median 1.25, above 1 on 10 of 12).

### Theory
- **Th1. Chains of minor sevenths.** Wasting Time: C#min7 26%, G#min7 11%, D#min7 10%, a chain a fifth apart, in C# dorian (margin 0.022, just over the guess line). Lead: D#min7 25%, A#5 18%, C#5 12% (A# aeolian, 0.003, a guess). Venus Aire: E5, Fmaj7, A5 (A aeolian, 0.033, reliable). Golden Triangle: C#, G#5, C#sus2 (C# lydian, 0.003, a guess). Settled chords run 13 to 17 a minute; by the fast counter 20 to 39. Chord names are a guess on dense music.
  - **[JUDGMENT]** **Guess:** a dorian vamp of min7 chords a fifth apart, for hazy and cool.
- **Th2. Bass and top line.** Lead's bass moves 1.99 notes a second (70% repeats); Venus Aire's jumps octaves (26%); Wasting Time's swings between C# and G# (47% and 41%). The top lines of Lead and Venus Aire leap a lot (39 to 41%) over 26 semitones. Wasting Time's moves in thirds (33%). Reliable only where the line is clear of its accompaniment.

### Space
- **The low end centered, width rising with pitch.** Lead: sub -16 dB (correlation 0.95), 120 to 500 Hz -7, 500 Hz to 2 kHz -4, 2 to 5 kHz -2, above 5 kHz -1. Venus Aire: -13, -5, -4, -4, -0. Wasting Time: -10, -4, -4, -3, -2. Golden Triangle is flat at -5 to -7 dB above 120 Hz.
- By part: the drums and bass are narrow (-10 to -19 dB, mono loss 0.1 to 0.4 dB), the other stem -2 to -6 dB. Over time (2 s windows), -10/-5/-3 dB on Lead and -9/-3/-0 dB on Wasting Time.
- **[JUDGMENT]** **Guess:** the closest record in the pile to the approved stereo baseline, a reference for a width that grows with pitch.

### Vibes
Numbers: Lead 92 BPM, 13 settled chords a minute, 1.4% above 2 kHz, width -8.6 dB; Wasting Time 80 BPM, 17, 2.9%, -5.8 dB; Venus Aire 85 BPM (grid), 13, 3.5%, -8.0 dB; Golden Triangle 75 BPM, 17, 2.5%, -8.3 dB.
- **[JUDGMENT]** Lean toward (x1.5 to x3): hazy, dreamy, chill, cool, mellow, groovy, misty, dusky, moonlit, velvety, swaying, cruising. Lean away (x0.2 to x0.5): still, hushed, glacial, meditative, restful, crisp. Never 0. **Guess:** a new profile could be a hazy-beats character; that is the brain's call.

### Wholesome take
- Take the slow heavy beat at a kinder low-end level, the min7 vamps, the bass's tape drift, the late kit and floor drops, the width that grows with pitch, and a breathy wordless vowel texture. Leave every sampled voice as a voice: only its texture is taken, rebuilt as a vowel, never a sample, never words. **[JUDGMENT]** **Guess:** wholesome, with the low end tamed (57% under 120 Hz).

### For later
- 01 Deep Swim (4:16): tuning -46 cents, a 1.4 octave sweep. **[JUDGMENT]** **Guess:** listen for a slowed source.
- 11 Bonita Camisa (3:19): the biggest brightness sweep (1.85 octaves); its low end leans left 3.1 dB. **[JUDGMENT]** **Guess:** listen for a filter opening.
- 08 Illyuziya (2:46): the narrowest (-11.1 dB) and the most travel (drift ratio 1.57). 02 Hazy Trip (3:20): the most chord changes (24.3 a minute), narrow (-10.8 dB). 05 Hello Earth (2:55): the loudest (-9.7 LUFS) and the steadiest pulse (0.84). **[JUDGMENT]** **Guess:** wait.
- 03 Exordium (2:33): the brightest (centroid 495 Hz) and one of two tracks that circle (drift ratio 0.88). **[JUDGMENT]** **Guess:** wait.
- Groups left over: Venus Aire bass-2 (E2, 36 dB/s) and drums-2 (a mid snap, 52 dB/s); Golden Triangle drums-1 (a mid snap, 156 dB/s); Wasting Time other-2 (bell-like C#3, 28 dB/s) and vocals-2 (C#2, bleed). **[JUDGMENT]** **Guess:** wait.

### Reliability notes for this card
- Venus Aire's meter and swing numbers are void (tool at 3/4 of the grid). Wasting Time's kicks are three quarters of its grid beat: listen.
- The bass stems' pans are the separator's. Two of four keys are guesses.
- Kick pitch at 90 to 116 Hz is above the sub range, so it is reliable to a few Hz.
