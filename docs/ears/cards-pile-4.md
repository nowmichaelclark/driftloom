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
| Kankyo Ongaku (compilation, 1980 to 1990) | 23 (4) | 1,120 s | done |

---

## Various artists, Kankyo Ongaku: Japanese Ambient, Environmental and New Age Music 1980-1990 (2019 compilation)

Ears 10, 2026-11-02. The folder holds 23 tracks, 114.4 minutes; the brief counted 25, so two may be missing from the upload. The files carry no artist tags, so this card names tracks only. Every track got the mix-level pass (picker). Four got the deep pass: 07 Park (typical, 4:06), and the three least like the rest, 14 Apple Star (6:01), 06 Variation III (3:36) and 22 Umi no Sunatsubu (1:12). `longform.py` ran on those four and on 23 Original BGM (15:55, the only long track, mix level only). Stems by Demucs htdemucs.

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
