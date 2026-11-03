# Ears cards, pile 5

Ears 11 (Sonnet 5.5, medium), 2026-11-03. Electronica. The cards come from `tools/ears/album.py` (run with `python3 -P`, one stage per process) and are checked against the limits in `docs/EARS.md`. Lines marked **[JUDGMENT]** are for the synthesis; each carries a short first guess marked **Guess**. Vibe names come only from `data/vibes.txt`. Voices on these records are described as textures only, never by their words.

**Stems are Demucs htdemucs** on every card; there was no MDX-Net fallback. I checked the separator with `split.py --mode demucs` on a 20 s clip before the first album (0.53 times real time).

**Twins.** I ran `twins.py` over the new picker rows, every earlier pick folder in `docs/ears/raw/` (unpacked to a scratch folder) and `data.csv`. No pile 5 track pairs with anything: the 27 pairs it lists are the known Aphex repeats and some 240.0 s coincidences between 4-minute upload pieces, all from earlier piles. Ears 1 measured Boards of Canada's *Geogaddi* at mix level (22 tracks in `data.csv`); *Music Has the Right to Children* is a different record and shares no track with it. No other pile 5 artist is in `data.csv`.

**Machine time:** about 9,370 s (2.6 hours) for five cards: 8,488 s of album chains (708, 2,992, 1,706, 1,315 and 1,767 s, from the first split to the card) plus 881 s of picks (all six albums were picked; Synthesist's 87 s is not in this sum). Ears 9's formula for the five says 6,338 s, so the pile ran 48% over it. The cause is that I ran **two album chains at once** to save wall time: Demucs then ran at 0.9 to 1.2 times the audio length instead of 0.5 to 0.65, and the measuring stages slowed too. The elapsed time was about 95 minutes. Memory held (two chains plus Firefox, nothing killed); a lone chain would have been about the formula. By the 2.5-hour rule I stopped at an album boundary and **left Synthesist (8 tracks, all deep-passed by the EP rule: about 3,030 s by the formula) for an Ears 11b**; its picks are in the cache (`docs/ears/raw/pile-5-raw.tar.gz`, `pick.json`).

**What changed in how the work was done:**

- `kick_check.py` ran on every track with a drum stem (kick and duck: one line on each card). On five tracks the bass stem was empty or near empty (Smokes Quantity, 1-06 Lifeforms, Looped, and two Perdition City tracks), so those ducks are void, and the "kick" lines there include sub bass.
- A new scratch script, `beatphase.py` (in the raw archive, under `ears11-session/`), puts the drum stem's onsets into twelfths of the meter tool's beat, to answer where a card's swing is. It found **one real triplet case, Lifeforms' Vertical Pig** (hits on all three thirds of the beat, none on a straight eighth), and read the others as straight (Smokes Quantity, Schnauss's four, Fear of Flying, The Future Sound of Music's sixteenths).
- `monoloss.py` and `med.py` (from Ears 10b) gave the mono loss by band and the album medians on every album; `lf_album.py` ran `longform.py`. Four scratch helpers are kept in the archive, not in `tools/ears/`.
- **Lifeforms has no tracks of 8 minutes or more** (the longest is 5:49), so the brief's long ones are not there; the 4-minute upload pieces are the only long-looking things. Bowery Electric's Postscript (16:41) and Schnauss's Molfsee (8:07) are the long ones, and Perdition City's Tomorrow Never Knows is 7:59.

| Album | Tracks (deep pass) | Machine time | Status |
|---|---|---|---|
| Boards of Canada, Music Has the Right to Children (1998) | 18 (4) | 708 s | done |
| Ulrich Schnauss, Far Away Trains Passing By (2005) | 12 (4) | 3,217 s (two chains at once) | done |
| The Future Sound of London, Lifeforms (1994) | 19 (4) | 1,930 s (two chains at once) | done |
| Bowery Electric, Beat (1997) | 11 (3) | 1,473 s (two chains at once) | done |
| Ulver, Perdition City (2000) | 9 (3) | 1,877 s (two chains at once) | done |
| Harald Grosskopf, Synthesist (1980) | 8 (8, EP rule) | not run (picks done, 87 s) | left for Ears 11b |


---

## Boards of Canada, Music Has the Right to Children (1998)

Ears 11, 2026-11-03. 18 tracks, 70.9 minutes, all at mix level (`monoloss.py` and the album medians too). Four got the deep pass: 15 Smokes Quantity (typical, 3:08) and the three least like the rest, 09 Bocuma (1:36), 01 Wild Analysis (1:18) and 17 One Very Important Thought (1:19). The picker chose short tracks, so the deep pass is 7.3 minutes: **it describes little of the record**, and the sections of "For later" matter more than usual. No track is 8 minutes long, so `longform.py` has nothing to add (it ran on the four picks, all of 1 to 3 minutes; the whole record is short tracks, the longest 6:23). Stems by Demucs htdemucs.

Machine time: 872 s (14.5 min) for 7.3 minutes of deep-pass audio:

- pick 164 s (18 tracks);
- Demucs, measuring, the stem check and the card 708 s (Demucs 187 s of audio in 233 s for Smokes Quantity alone, run alongside another chain).

Ears 9's formula (1.1 x 440 s + 12 x 18) says 700 s, so this ran 25% over it.

### Stem check (read first)
- **15 Smokes Quantity**: drums 56%, bass **0%**, other 44%, vocals 0%. The bass stem is empty, so bass notes, pumping and duck numbers are void; the low end is in the drums stem (below). The drum grid's beat is 0.302 s (198.7 BPM); the meter tool's 99 BPM (0.606 s) is half of it. The grid is the eighth of the felt beat, so I use 99 BPM felt, with a hit every eighth.
- **09 Bocuma**: other 100%. No drums, no bass: every beat number is void (the meter tool's 90 BPM and the 48 chords a minute lean on the pad's own movement).
- **01 Wild Analysis**: other 100%, bass 0% (the "E1" group has its notes 44 dB under the mix: noise). No beat.
- **17 One Very Important Thought**: drums 0%, bass 13%, other 80%, vocals 8%. The vocals stem plays 60% of the time from 0:21 to 1:08: a voice-like texture or a pad's upper part; I did not take any words from it, and the stem alone cannot say which. Its swing line ("0.67, triplet feel") has no drum stem under it: void.
- No subharmonic misreads claimed. Smokes Quantity's drum lines sit under 60 Hz (below).

### The hook
- Short, warm, dim and slightly out of tune. A median track is 3:46 long and quiet (-16.2 LUFS) and dark (centroid 357 Hz; 61% of the energy below 120 Hz, 1.3% in 2 to 5 kHz, 0.7% above 5 kHz). Tails are long (-1.2 dB a second median, the same family as ambient). Tunings sit **off A440 by a different amount on each track**: from -47 to +46 cents, median +6.5.
- **[JUDGMENT]** One line a listener would say. **Guess:** an old tape of a summer that never quite happened: soft pads a little flat or sharp, and a slow, dusty beat that wanders.

### Sounds

**S1. Pads that hold one color (Bocuma, One Very Important Thought): other-1, 231 and 57 notes, 53% and 43% of note energy**
- In synth terms: Bocuma has two groups, one around F2 (89 Hz) and one around G#3 (213 Hz), both slow swells (attacks 337 and 286 ms), falling 3 to 5 dB/s, mixed partials 26 to 39 cents off whole multiples (not pure), half-correlated stereo (0.73 and 0.79). One Very Important Thought's pad is about C#2 to A2 (register 71 Hz), soft attack (107 ms), falls 8 dB/s, partials 43 cents off, nearly mono (correlation 0.99). Both are weak groups (silhouette 0.20 and 0.21): one blended texture.
- Hear it at Bocuma 0:03.9 (A2, 1.54 s), 0:29.1 (F2, 1.14 s), 0:38.3 (G3, 1.53 s); One Very Important Thought 0:03.8 (A2, 0.27 s).
- Building blocks: **[JUDGMENT]** **Guess:** softpad with a 0.3 s attack and a slightly inharmonic top; exists.

**S2. A bell-like upper line (One Very Important Thought): other-2, 118 notes, 29%**
- In synth terms: about C#4 (279 Hz), bell- or metal-like (partials 87 cents off whole multiples, harmonic share 0.60), soft attack (136 ms), falls 19 dB/s, nearly mono.
- Hear it at 0:03.5 (B3, 0.28 s), 0:04.9 (D4, 0.28 s), 0:02.2 (C#4, 0.43 s).
- Building blocks: **[JUDGMENT]** **Guess:** glass keys or a soft bell; exists (Ears 4's tone, 0.5 to 2 kHz).

**S3. Slow kick and low hum that share the drums stem (Smokes Quantity): drums-1, 218 notes, 49%**
- In synth terms: a low thump, about D1 to F1 (44 Hz median, 37 to 53), quick attack (19 ms), falls 169 dB/s, brightening over the note by 1.05 octave, nearly centered (pan 0.02, correlation 0.91). A second group (drums-2, 70 notes) is a high tick at about 3.3 kHz, falling 94 dB/s.
- **Kick line (kick_check):** 251 kicks, a median of 39 Hz (D1, 33% of hits; A1 9%, G#1 8%, D#1 6%), spread 34 to 96 Hz, falling 162 dB/s (133 to 286), median spacing 0.315 s against the 0.302 s grid. Under 60 Hz pitch is good to a semitone only, and with the bass stem empty the "kicks" very likely include the sub bass that Demucs left in the drums stem. **Duck:** void (no bass stem). Read the line as: a kick-and-sub that falls at 160 dB/s and sits on the D, not as a clean kick.
- Hear it at 0:37.9 (F1, 0.27 s), 1:19.1 (D#1, 0.6 s), 1:48.2 (D1, 0.32 s).
- Building blocks: **[JUDGMENT]** **Guess:** tunedKick, on the root, with a sub held under it; the tick is a hat. exists.

### Techniques

- **T1. Tape: each track tuned differently, with wow and slow drift.** On the mix, the median tuning is +6.5 cents (10th to 90th percentile -39 to +29.5) and the median wobble 5.4 cents (4.0 to 6.8). Per track the tunings run: Olson -47, Pete Standing Alone -44, Aquarius -37, Triangles & Rhombuses -28, Rue the Whirl -27, The Color of the Fire -25, An Eagle in your Mind -21; and on the other side One Very Important Thought +46, Sixtyten +33, Happy Cycling +28, Bocuma +24. At stem level (deep pass):
  - Smokes Quantity: the other stem wobbles about 19 cents at 0.6 Hz (wow, a slow one; the tool's calibration reads a 12-cent wow as 13.7, so about right) and its speed moves up to 48 cents in 4 s;
  - Wild Analysis: other 13 cents at 1.0 Hz, speed up to 20 cents in 4 s; its bass 14 cents at 1.9 Hz;
  - One Very Important Thought: bass 15 cents at 0.9 Hz, speed up to 51 cents in 4 s;
  - Bocuma: nothing at 4 cents or more in the partials that could be tracked: the held pads are steady.
  - **Against Driftloom's own wobble:** the 104 loops in `data.csv` have a median wobble of 3.6 cents and a tuning of 0 to 3 cents (range -17 to +10, absolute median 3). The 502 records in `data.csv` have a median wobble of 4.4 and an absolute tuning median of 12 cents. This record sits above Driftloom in both: 5.4 cents of wobble on the mix, 19 cents of wow where it is clearly audible on a stem, and tracks 20 to 47 cents off, flat more often than sharp (7 tracks are more than 20 cents flat, 5 more than 20 cents sharp, up to +46). One mix-level number cannot tell wow from flutter: the stem readings put the wow at 0.6 to 1.0 Hz and a flutter band at 1.9 Hz (bass only).
  - **[JUDGMENT]** **Guess:** a per-loop tuning offset of -45 to +45 cents (a spread, not a rule), plus a slow wow of 10 to 20 cents at 0.5 to 1 Hz on the pads, and a faster drift of up to 50 cents over 4 s; Driftloom's own 3.6 cent wobble is a different, much smaller thing. It is a tape or detuned-instrument cue, and a vibe like hazy or wistful should lean on it.
- **T2. A hit on every eighth, almost straight.** Smokes Quantity: the meter's eighth sits at 0.513 of the beat, which is straight (0.5). The drum grid (0.302 s) is that same eighth, so the swing is read **at the drum grid's eighth**: 0.513, no triplet shuffle. This is not the case batch-5.md's triplet-shuffle lead is waiting for. One Very Important Thought reads 0.67 (a triplet feel) at its meter's beat, but there are no drums: not counted.
- **T3. Floor drops and sections.** Smokes Quantity 1.07 drops a minute (0:09.2 for 9.8 s, 1:36.8 for 9.5 s, 2:42.2 for 15.5 s), sections of 17 s median, and the drums leave at 1:37.5 and 2:43.0 and return at 0:18.5 and 1:45.5: changes at section lines (10 to 32 s apart). Other tracks have no drums.
- **T4. Sweeps.** Smokes Quantity: the mix goes down 3.2 octaves from 2:50.0 to 3:06.5 (the end of the track: a filter closing). Wild Analysis's bass moves up 2.6 octaves 0:50.0 to 0:57.5. The album median sweep is 0.71 octave; Aquarius 1.93.
- **T5. No echo or pump claimed.** Smokes Quantity's "other dips 2.5 dB at the kick" is a floor and sits close to the noise; the bass dip (10.1 dB) is void (empty stem).
- **T6. Long form: nothing to say.** Every deep-pass track is under 3.2 minutes. Drift ratio (60 s against 10 s) 1.53 median (17 of 17 above 1).

### Theory
- **Th1. Sus chords and minor sevenths.** Smokes Quantity: C#sus2 40%, C#min7 17%, C#min 13% (C# dorian, margin 0.013, a guess; fits C# minor pentatonic), 17 changes a minute, 16 settled. Wild Analysis: Emin7 44%, Asus2 31% over an E pedal (E aeolian, margin 0.002, a guess). Bocuma: G5 39%, Fmaj7, Dmin (G mixolydian, margin 0.028). One Very Important Thought: Dsus2 37%, E7 23% over E, A and C (a locrian reading with margin 0.002: a guess). 58%, 70%, 41% and 44% of changes land on a beat against chances of 50%, 45%, 32% and 51%: Bocuma's 70% is the only clear one, since its chords change every second on the pad.
  - **[JUDGMENT]** **Guess:** sus2 and minor seventh chords with a pedal; the major sevenths and ninths in plan.md's row are not seen in these four short picks (not claimed).
- **Th2. A wide-leaping top line on Bocuma.** Bocuma's top line is 58% leaps with a median interval of 12 semitones (an octave): arching phrases of about 7 notes. The other three move by step (50 to 52%).
- **Th3. Short repeated loops.** Smokes Quantity: bars of 1.2 s repeating every 1 bar (0.59) and 8 bars (0.47): a short loop at 99 BPM with 30% near-duplicate bars.

### Space
- **Mostly mono at the bottom, a little wide at the top.** Mix width by band (dB; 0 = unrelated channels): Smokes Quantity sub -8, 120 to 500 -14, 500 to 2 kHz -10, 2 to 5 kHz -8, above 5 kHz -2 (corr 0.25: the hat and air band are wide); Bocuma -4, -7, -9, -7, -8 (wide in the sub: the pad); Wild Analysis -20, -15, -14, -7, -8; One Very Important Thought -18, -20, -17, -7, -16. The album's median width is -10.6 dB.
- **What the mix loses in mono** (corr and loss in dB, below 120 Hz, 120 to 500, 500 to 2 k, 2 to 5 k, above 5 k): Smokes Quantity 0.7, 0.2, 0.4, 0.6, **2.6**; Wild Analysis 0.0, 0.1, 0.2, 0.8, 0.6; Bocuma 1.5, 0.8, 0.5, 0.7, 0.6; One Very Important Thought 0.1, 0.0, 0.1, 0.7, 0.1. On the other 14 tracks of the album the worst are **Olson (4.8 dB below 120 Hz, corr -0.57: the sub is out of phase between the channels)**, Kaini Industries (4.4 dB at 2 to 5 kHz, corr -0.28), Triangles & Rhombuses (3.6 dB below 120 Hz, corr -0.13), Happy Cycling (up to 2.4). The loss is small at the bottom on most tracks, but three tracks fold badly in mono below 120 Hz or at 2 to 5 kHz.
- **[JUDGMENT]** **Guess:** a mono switch would lose under 1 dB on most of the record, but out-of-phase sub or top (Olson, Kaini Industries) is a case to design against: keep the bass and kick centered, as the guards already say.

### Vibes
Numbers: Smokes Quantity 99 BPM (hit every eighth), 16 settled chords a minute, 3% above 2 kHz, width -9 dB; Bocuma 90 BPM (no drums), 12, 0%, -8 dB; Wild Analysis 63 BPM (no beat), 12, 0%, -14 dB; One Very Important Thought 101 BPM (no drums), 10, 1%, -19 dB.
- **[JUDGMENT]** Lean toward (x1.5 to x3): hazy, wistful, dreamy, misty, mellow, dusky, reflective, hushed, rainy, soothing. Lean away (x0.2 to x0.5): crisp, bright, triumphant, lively, enthusiastic, bouncy. Never 0. **Guess:** the reference record for hazy with a little wobble.

### Wholesome take
- Take the slightly off-tuning (the soft wow of 10 to 20 cents and a per-loop offset), the dusty slow beat on every eighth, the sus2 and minor seventh pads and the short, repeating loops; leave the out-of-phase sub (Olson) and the tracks that are pure noise. **[JUDGMENT]** **Guess:** wholesome; nostalgic without any weight. Detuning past 40 cents will sound wrong, not warm, to many listeners: keep the offset's tail short (for Mikey's ears).

### For later
- 08 Kaini Industries (0:59, mix-level 96 BPM): 18.2 chord changes a minute (album median 5.2), the worst top-band fold (4.4 dB). 10 Roygbiv (2:31): 15.5 chord changes. 13 Olson (1:32): 12.4 chord changes and the out-of-phase sub. 16 Open the Light (4:25): 13.6 chord changes and the shortest tails (-6.2 dB a second). **[JUDGMENT]** **Guess:** a deep pass on Open the Light would test the short-tail, high-change side.
- 12 Aquarius (5:58): the biggest sweep (1.93 octaves) and the highest drum grid (172 BPM at mix level). 06 Sixtyten (5:48): the most pitch bends (37.9 a minute, album median 8.5); a likely tape-speed track. 02 An Eagle in your Mind (6:23): 12.8 chord changes. **[JUDGMENT]** **Guess:** Sixtyten and Aquarius first, for tape bends and a filter sweep, since the four deep picks are the short ones.
- Groups left over: 17's bass-1 (E1, 172 notes) and vocals-1 (a texture on A3, 152 notes, quick). **[JUDGMENT]** **Guess:** wait.

### Reliability notes for this card
- The picker chose the four shortest or odd tracks; the four deep-pass tracks cover 10% of the album's time. Tape numbers on Smokes Quantity's other stem and One Very Important Thought's bass are from stems of 1 to 3 minutes. Per-stem wobble and speed are calibrated on test signals (12-cent wow read 13.7) and unrelated to the mix-level tuning, which is a single-file reading good to about 8 cents.
- Smokes Quantity's bass stem is empty, so "bass" in its numbers means nothing; the kick line may include sub bass.
- The 4 stem groups are weak; the chord names are guesses on pads that are mostly sus2.

---

## Ulrich Schnauss, Far Away Trains Passing By (2005)

Ears 11, 2026-11-03. 12 tracks, 83 minutes (50 to 8:07 each; 6 to 7.6 minutes typical), all at mix level (`monoloss.py` and the album medians too). Four got the deep pass: 08 Suddenly The Trees Are Giving Way (typical, 6:23) and the three least like the rest, 07 Sunday Evening In Your Street (6:45), 01 Knuddelmaus (7:01) and 02 Between Us And Them (7:29). `longform.py` ran on the four and on 06 Molfsee (8:07, the only track of 8 minutes or more, mix level only). Stems by Demucs htdemucs.

Machine time: 3,217 s (53.6 min) for 27.6 minutes of deep-pass audio, run alongside another album's chain (two processes at once on 8 cores, so wall time is more than the time it would take alone):

- pick 225 s (12 tracks);
- Demucs 1,744 s (383, 405, 421 and 449 s of audio in 390, 366, 501 and 488 s: 0.9 to 1.2 times the audio length while sharing the cores, against 0.5 to 0.65 alone);
- measuring, the stem check and the card 1,248 s (4 tracks; slowed by sharing the cores); the kick checks, the long form (5 tracks) and the mono table about 5 minutes more, not counted.

Ears 9's formula (1.1 x 1,658 s + 12 x 12) says 1,968 s, so this ran 63% over it. The cause is the two chains competing for the cores (Demucs ran at 0.9 to 1.2 instead of 0.5 to 0.65 times the audio); the pile's total is set against the formula at the end.

### Stem check (read first)
- **08 Suddenly The Trees Are Giving Way**: drums 43%, bass 37%, other 20%, vocals 0% (bleed). The drum grid's beat is 0.302 s (198.7 BPM); the meter tool's 100 BPM (0.600 s) is half of it, and the kick spacing (0.605 s) matches 100 BPM. So the felt beat is 100 and the grid is its eighth.
- **07 Sunday Evening In Your Street**: drums 51%, bass 28%, other 20%, vocals 0.2% (bleed, though the stem plays 11% of the time). The card's "25.0 BPM" grid is a bar, not a beat (Ears 10b's lesson): four beats of 0.600 s is 2.4 s, and the 4-beat grid reads 99.9 BPM; the meter tool says 100 and the kicks fall every 1.19 s, two beats.
- **01 Knuddelmaus**: drums 20%, bass 62%, other 18%. Grid beat 0.302 s (198.7 BPM), meter 100 BPM: the same grid.
- **02 Between Us And Them**: drums 16%, bass 56%, other 28%. The grid's 172.4 BPM (0.348 s) and the meter tool's 87 BPM (0.690 s) are a factor of 2 apart; the 4-beat grid says 173.7 BPM. I use 87 as the felt beat. Its kicks fall every 0.520 s, three quarters of that beat, so the drum pattern is broken (a dotted pattern), not four on the floor; the bar length is "unclear" (score 0.09).
- No empty stems. No subharmonic misreads claimed. Between Us And Them's bass group (D#1, register 39 Hz, 35 to 40) is under 40 Hz: good to a semitone only.

### The hook
- Big, bright-in-the-middle shoegaze-electronica: a steady 100 BPM kit, a bass that moves under it, and walls of wide pads and guitar-like layers that hold the 500 Hz to 5 kHz band. It is loud (-11.0 LUFS median, 5 dB hotter than Boards of Canada's record) and dark in the mean (centroid 395 Hz; 57% of energy under 120 Hz), but with 13% in 500 Hz to 2 kHz (Boards of Canada 8%). The beat is clear (pulse 0.84 median) and the tuning is exact (+1.5 cents median, wobble 3.3 cents: digital, no tape). It moves a lot: 12.7 chord changes a minute (median, 9 to 17), sweeps of 1.2 octaves.
- **[JUDGMENT]** One line a listener would say. **Guess:** a train window at dusk: a clean steady beat under a wide, glowing wash that keeps rising and falling.

### Sounds

**S1. Bass that walks under the kit (Knuddelmaus, Between Us And Them, Trees): bass-1, 424, 403 and 318 notes, 66%, 48% and 39%**
- In synth terms: Knuddelmaus C2 (66 Hz), slow swell (attack 179 ms), falls 8 dB/s, 1.12 notes a second, roots G 54%, C 35%, D 11%; Between Us And Them D#1 (39 Hz) with a second group at A#1 (58 Hz), soft attack, falls 10 dB/s, 1.07 notes a second, D# 47%, A# 28%, C# 12%, G# 10%, 85% repeats; Trees F#1 (46 Hz), quick attack (39 ms), falls 4 dB/s, partials within 3 cents (a pure sine-like tone), F# 57%, B 36%, 61% repeats. All near-mono (correlation 0.92 to 1.00).
- Hear it at Knuddelmaus 0:55.3 (C2, 0.57 s), 0:52.6 (G2, 1.2 s); Between Us And Them 3:15.5 (G#1, 0.14 s), 3:05.2 (C#1, 0.56 s); Trees 5:42.5 (F#1, 2.4 s), 3:44.8 (B1, 1.47 s).
- Building blocks: **[JUDGMENT]** **Guess:** sub or round bass with a slow-to-quick attack by track, on the root and the fifth below or above (G and C, F# and B); exists; the fourth and fifth moves (25 to 30%) are bassStyle moving.

**S2. Wide glassy pads and guitar-like layers: other-1, 253 to 500 notes, 16% to 24% of note energy**
- In synth terms: Knuddelmaus G2 (99 Hz), soft attack (145 ms), falls 15 dB/s, brightness 682 Hz, partials 48 cents off whole multiples, correlation 0.52; Sunday Evening E2 (82 Hz) and a second group at B2 (125 Hz), slow swell (173 ms), falls 6 to 8 dB/s, brightness 336 and 791 Hz, correlation 0.46 to 0.47; Trees F#3 (186 Hz), slow swell (164 ms), falls 11 dB/s, **brightness 1,317 Hz, correlation 0.04**: fully decorrelated, the widest sound on the album. All are weak groups (mixed or noisy partials): textures, not one instrument.
- Hear it at Knuddelmaus 0:29.8 (G2, 0.27 s); Sunday Evening 0:00.0 (E2, 2.82 s), 0:07.4 (A2, 1.18 s); Trees 2:59.7 (F#3, 0.3 s), 3:07.8 (F#3, 0.38 s).
- Building blocks: **[JUDGMENT]** **Guess:** softpad or analogpad doubled by a slightly detuned copy and a decorrelated side; the missing piece is a pad that is almost fully decorrelated at 1 to 2 kHz (stereo's baseline spreads by ensemble; this goes further).

**S3. The kit (all four): drums, 20% to 51% of the power**
- **Kick lines (kick_check):**
  - Trees: 440 kicks, a median of 59 Hz (A1 28%, A#1 11%; 49 to 113), falling 146 dB/s, every 0.605 s (the beat). **Duck:** the bass is 3.5 dB down at the kick and back to the top by 50 ms (-1 dB), highest at 480 ms: a short duck.
  - Knuddelmaus: 439 kicks, a median of 67 Hz (C3 9%, G1 8%, B1 8%: spread wide), falling 158 dB/s, spacing 0.490 s, which is not the 0.600 s beat (kick pattern, with the extra hits). **Duck:** 6.4 dB down at the kick, back at the top by 100 ms: the clearest pump on the album.
  - Sunday Evening: 288 kicks, a median of 63 Hz (many hits under 60 Hz: B0 7%, G#1 5%), falling 194 dB/s (41 to 667: a wide spread), every 1.19 s, two beats. **Duck:** the bass is lowest at 400 ms (-9.7 dB) and highest at 920 ms, just before the next kick: the bass plays off the kick, taking turns with it.
  - Between Us And Them: 499 kicks, a median of 108 Hz (C3 21%, B2 17%: well above a sub, so the kick is a tom-like low hit), falling 95 dB/s, spacing 0.520 s. **Duck:** none (1.7 dB at the kick, flat after).
  - Under 60 Hz pitch is good to a semitone only.
- Building blocks: **[JUDGMENT]** **Guess:** tunedKick, 60 to 70 Hz, with subDuck on a short recovery (50 to 100 ms) for Knuddelmaus's pump; Sunday Evening's bass playing between kicks is the pedalBass family; exists.

### Techniques

- **T1. Sweeps that never stop.** Brightness sweeps of 0.7 octave or more at mix level: Trees 46 in 6.4 minutes (7.2 a minute; the first minute swings 2 to 3 octaves up and down every 5 to 6 s: 3.0 up 0:12.0 to 0:18.5, 3.3 down 0:29.0 to 0:36.5), Sunday Evening 69 (a minute-by-minute filter, up to 3.4 octaves), Knuddelmaus 35, Between Us And Them 22. The mix-level median sweep is 1.2 octaves (Boards of Canada 0.7). Many of these are the pad's filter, not a sustained arc: a swing every 5 to 10 s.
  - **[JUDGMENT]** **Guess:** a pad filter that swings 2 to 3 octaves on a 5 to 8 s period under the kit; the padSweep exists as a slow arc, and this is the fast version (for Mikey's ears: it is a lot of movement).
- **T2. The kit comes and goes in blocks.** Trees: kit in at 3:14.5, out at 2:56.0 and 6:13.0; one floor drop of 20.8 s at 2:57.0 (0.17 a minute). Knuddelmaus: kit in at 0:35.5 and 1:15.5, bass in at 0:37.5, a floor drop of 17.5 s at 0:21.0. Sunday Evening: floor drops 1.81 a minute, mostly 3 to 4 s, in a run of six at 4:31.8 to 4:55.8 (a stutter: kick and bass out every 4.8 s, one bar of 4.8 s). **[JUDGMENT]** **Guess:** floorDrop at a bar, long (17 to 21 s) early in a track; the stutter run is a shorter version.
- **T3. A tempo of 100.** Three of the four tracks share 99.9 to 100 BPM and one is at 87: a record at one tempo. Straight eighths: the meter's off-beat sits at 0.501 to 0.508 on every track, and the drum grid (0.302 s) is that eighth, so **swing at the drum grid's eighth is straight (0.50)**: nothing for batch-5.md's triplet-shuffle lead. The meter tool's triplet share reads 0.79 on Trees and 0.26 to 0.31 on Knuddelmaus and Sunday Evening, but with the off-beat at 0.50 that is hats or pads in triplets (a 3-against-2), not a shuffled eighth; "no cross-rhythm claim" stands.
- **T4. Tape: none.** Mix tuning +1.5 cents median (-3.8 to +7.8) and wobble 3.3 cents, the same as Driftloom (tuning 0 to 3 cents, wobble 3.6). On stems, the readings are small: Trees other 8 cents at 0.7 Hz and bass 12 cents at 3.3 Hz; Knuddelmaus other 10 cents at 0.4 Hz; Between Us And Them bass 19 cents at 2.9 Hz; Sunday Evening 6 and 9 cents at 4.6 and 6.5 Hz. The stems' speeds move 7 cents at most. These sit near the tool's reading of detune and vibrato in a mixed stem and are not claimed as tape. No echo: Knuddelmaus's "drum echo at 494 ms (one repeat)" is its own kick spacing (0.490 s).
- **T5. Long form: slow builds are fades.** Level swells of 14 to 33 dB on Between Us And Them (310 s), Molfsee (350 s) and Knuddelmaus (230 s) are the tracks' own fade-in and fade-out (Ears 10b's lesson): Molfsee runs -31, -20, -17, -13, -15, -12, -12, -30 dB a minute; Between Us And Them -32, -19, -13, -14, -13, -13, -22, -52. So the **entry builds over 1 to 2 minutes** (the first minute is 12 to 19 dB under the peak, centroid 1,400 to 1,560 Hz against 270 to 550 later: the opening is thin and bright, then the low end arrives) and the end fades over the last minute. Inside the body (minutes 2 to 6) each track is nearly level (within about 3 dB) and spectrally steady; Molfsee ends at the opening's brightness (it returns to the opening). Returns: 0.78 Molfsee, 0.56 Between Us And Them, 0.50 Trees, 0.46 Sunday Evening, 0.23 Knuddelmaus. The strongest pitch class changes 1.4 to 1.6 times in 10 minutes on Knuddelmaus and Trees (a home key held), 6.8 on Between Us And Them, 9.0 on Sunday Evening and 18.8 on Molfsee.
  - **[JUDGMENT]** **Guess:** for a long loop (up to 32,768 bars, Endless): a thin, bright opening that thickens into the full layers over about 100 s, a steady middle and a fade; a key that holds, or changes every 30 to 60 s. The long-form composing comes later; this is the shape to read.

### Theory
- **Th1. Simple chords on a held root.** Knuddelmaus: G5 47%, C5 26%, Csus2 21% (G ionian, margin 0.042), only 4 chords and the bass on G and C. Trees: F#5 26%, B 25%, F#maj7 17%, Bsus2 9% (F# ionian, 0.023). Between Us And Them: D#sus2 27%, D#5 14%, A#min7 14% (D# mixolydian, 0.051). Sunday Evening: Dmin7 34%, Amin7 18%, Fmaj7 15% over an E bass (E phrygian, 0.047: the 31 distinct chords are a sign of the dense wall, a guess). 21 to 26 changes a minute; 18 to 21 settled on Trees and Knuddelmaus; 9 on Sunday Evening. Chord names on dense music are a guess.
  - **[JUDGMENT]** **Guess:** I, IV and a sus2, with a bass that moves between the root and the fourth or fifth; a major or mixolydian home, a minor seventh in the other.
- **Th2. A top line that moves by fourths and fifths.** Trees 39% fourths and fifths (median interval 5), Knuddelmaus 43% (median 5): the line steps in open fifths with 44 to 57% arches in phrases of 11 to 12 notes. Sunday Evening and Between Us And Them use steps and thirds (43 to 55%).
- **Th3. Repeats at 2 and 4 bars.** The strongest repeats are at 2, 4 and 1 bars (0.62, 0.73, 0.66) and then 8 (0.54): short, nearly-exact phrases, with 5 to 22% of bars near-duplicating an earlier one.

### Space
- **A wall in the middle.** Mix width by band (dB; 0 = unrelated channels): Trees sub -26, 120 to 500 -8, 500 to 2 kHz -1, 2 to 5 kHz -4, above 5 kHz -13; Sunday Evening -24, -7, -5, -10, -21; Knuddelmaus -18, -7, -5, -6, -10; Between Us And Them -12, -5, -3, -3, -4. The album's median width is -7.8 dB, wider than Boards of Canada (-10.6) and Mirror's Edge (-9.3), and it peaks at 500 Hz to 5 kHz, as on Solar Fields but wider.
- **By part:** the drums and bass are narrow (-13 to -44 dB, mono loss 0.0 to 0.2 dB); the other stem is wide (-1 to -4 dB), with mono loss 1.4 dB (Sunday Evening, Knuddelmaus), 2.0 dB (Between Us And Them) and 2.6 dB (Trees).
- **What the mix loses in mono** (below 120 Hz, 120 to 500, 500 to 2 k, 2 to 5 k, above 5 k): the sub is fine on every track (0.0 to 0.2 dB, correlation 0.91 to 1.00). The loss grows with pitch: 120 to 500 Hz 0.4 to 1.8 dB, 500 Hz to 5 kHz **up to 2.8 dB** (Crazy For You 2.6 and 2.7 dB at 500 Hz to 5 kHz; Wherever You Are 2.4 and 2.8; Trees 2.1 dB at 500 Hz to 2 kHz), and above 5 kHz up to 2.8 dB (Crazy For You). The four deep picks lose 1.1 to 2.1 dB at 500 Hz to 2 kHz. So in mono the wall thins by 2 to 3 dB in the middle, but the bass and the kit do not move.
- **[JUDGMENT]** **Guess:** a space reference for width that rises through the mids and is almost fully decorrelated at 1 to 2 kHz on the pads, with a mono floor; Ears 4's baseline could go wider here; the mono switch loses about 2 dB of the middle (acceptable, as the guards say).

### Vibes
Numbers: Trees 100 BPM, 21 settled chords a minute, 9% above 2 kHz (mix level), width -5 dB (median of 2 s windows); Sunday Evening 100 BPM, 9, 5%, -7 dB; Knuddelmaus 100 BPM, 20, 3%, -7 dB; Between Us And Them 87 BPM, 16, 5%, -4 dB.
- **[JUDGMENT]** Lean toward (x1.5 to x3): dreamy, soaring, golden, floating, spacious, hazy, wondrous, flowing, pulsing, steady, moonlit. Lean away (x0.2 to x0.5): cozy, intimate, hushed, minimal, plucky, jazzy, goofy. Never 0. **Guess:** the reference record for soaring over a steady beat.

### Wholesome take
- Take the steady 100 BPM kit with a short duck, the exact tuning, the wide mid-band pads, the open-fifth top line, the sus2 and major seventh chords and the thin-to-full opening; leave the tension of the stutter run for a moment that needs it. **[JUDGMENT]** **Guess:** wholesome; bright and open throughout, and the loud master (-11 LUFS) needs a gentler level.

### For later
- 06 Molfsee (8:07): the biggest travel (3.3 octaves in brightness, a 24 dB level swell that is its fade, 18.8 key changes in 10 minutes). **[JUDGMENT]** **Guess:** listen for the long form; it is the one track over 8 minutes.
- 11 Crazy For You (6:32) and 12 Wherever You Are (6:37): the widest in the 500 Hz to 5 kHz band (correlation 0.04 to 0.17) and the biggest mono loss (2.4 to 2.8 dB). **[JUDGMENT]** **Guess:** a width test case for the mono switch.
- Groups left over: Knuddelmaus other-2 and bass-2, Trees other-2, Sunday Evening bass-1 (drums-heavy, 0.38 notes a second). **[JUDGMENT]** **Guess:** wait.

### Reliability notes for this card
- The four picks are the typical track and three outliers; on a record of one tempo and one palette the other eight (05 Nobody's Home, 03 passing By, 09 and 10) are alike in their mix numbers. Between Us And Them's beat level is a guess (87 or 174; kicks at 0.52 s fit neither); its swing and chord-on-beat numbers lean on the meter, so read them as weak.
- Wobble readings of 6 to 19 cents on stems at 2.9 to 6.5 Hz are above the flutter range of tape (0.5 to 4 Hz for wow and flutter) and near the tool's floor on mixed stems; I did not call them tape.
- The vocals stem is bleed on Trees and Knuddelmaus; on Sunday Evening it plays 11% of the time (3:05 to 3:16 and 5:01 to 5:37) at 0.2% of the power, a texture I did not describe further.

---

## The Future Sound of London, Lifeforms (1994)

Ears 11, 2026-11-03. 19 tracks, 87.2 minutes (a double album's single disc of 55 s to 5:49 tracks), all at mix level (`monoloss.py` and the album medians too). Four got the deep pass: 1-06 Lifeforms (typical, 5:18) and the three least like the rest, 13 Cerebral (3:31), 09 Domain (2:48) and 12 Vertical Pig (5:49). **No track on this record is 8 minutes long** (the brief expected long ones; the folder's longest, 12 Vertical Pig, is 5:49, and the 4-minute upload pieces make some look longer), so `longform.py` ran only on the four picks (all under 6 minutes) and I read it as minute-by-minute shape only. Stems by Demucs htdemucs.

Machine time: 1,930 s (32.2 min) for 17.4 minutes of deep-pass audio, run alongside another album's chain (two processes at once on 8 cores):

- pick 224 s (19 tracks);
- Demucs, measuring, the stem check and the card 1,706 s (Demucs 0.9 to 1.2 times the audio while sharing the cores); the kick, beat-phase and mono checks about 4 minutes more, not counted.

Ears 9's formula (1.1 x 1,047 s + 12 x 19) says 1,379 s, so this ran 40% over it, again with two chains competing for the cores.

### Stem check (read first)
- **1-06 Lifeforms**: drums 52%, **bass 0.2% (empty)**, other 44%, vocals 4%. The bass stem is empty, so bass notes, pumping and duck numbers are void; the low end is in the drums and other stems. Meter 133 BPM (0.452 s) agrees with the grid beat (132.5 BPM). The vocals stem plays 51% of the time at 4% of the power: a voice-like texture or a sample, not described further.
- **13 Cerebral**: drums 6%, bass 21%, other 50%, vocals 24%. The meter's 97 BPM is half of the grid's 191.7 BPM (0.313 s): the grid is the eighth; I use 97 (0.617 s). The vocals stem, at a quarter of the power, is a voice-like texture (no words taken).
- **09 Domain**: drums 14%, bass 13%, other 72%, vocals 2% (bleed). The meter's 133 BPM (0.450 s) is two thirds of the 198.7 BPM grid (0.302 s): a 3-over-2 relation, or a grid that is not the beat; the drum stem has only 117 onsets, so I do not claim either.
- **12 Vertical Pig**: drums 22%, bass 33%, other 28%, vocals 16% (the stem plays 83% of the time: a voice-like texture, no words taken). The meter's 123 BPM (0.489 s) is two thirds of the 184.6 BPM grid (0.325 s). That is a triplet grid, and the beat-phase check below confirms it.
- No subharmonic misreads claimed. Vertical Pig's bass group at C1 (register 33 Hz, 32 to 34) is under 40 Hz: good to a semitone only.

### The hook
- Dense, strange, slightly sinister electronica made of low, bell-like and noisy pads, and drums that move between a broken break and a triplet pattern. It is quiet (-18.4 LUFS median, the quietest of this pile so far) and dark (centroid 475 Hz; 43% of the energy under 120 Hz, 18% in 500 Hz to 2 kHz, the most of the four records). Chords change slowly (3.5 a minute median), with big sweeps (1.6 octaves median: the pile's biggest) and an arc of 6.7 dB. Tunings sit off A440 by a different amount on each track (-44 to +28 cents, median +7), like Boards of Canada but at half the wobble (5.1 cents).
- **[JUDGMENT]** One line a listener would say. **Guess:** a night walk through a dream machine: slowly moving low pads, odd bells, and a beat that skips between a break and a triplet.

### Sounds

**S1. Low bells and noisy pads (Domain, Cerebral, 1-06 Lifeforms, Vertical Pig): other-1, 162 to 442 notes, 20% to 73% of note energy**
- In synth terms: Domain C2 (65 Hz), bell- or metal-like (partials far off whole multiples, harmonic share 0.42), slow swell (attack 159 ms), falls 12 dB/s, **fully decorrelated** (correlation 0.15) and leaning right (+0.15). Cerebral F#1 (46 Hz), bell-like (0.38), soft attack (101 ms), falls 12 dB/s, correlation 0.62; a second group at B2 (125 Hz), harmonic 0.81, 3.5 s notes. 1-06 Lifeforms A#1 (58 Hz), mixed partials, slow swell (211 ms), falls 6 dB/s, correlation 0.72. Vertical Pig C2 (65 Hz), mixed, soft attack, falls 16 dB/s, correlation 0.74. All weak groups (one blended texture each).
- Hear it at Domain 0:19.3 (F#1, 2.45 s), 0:15.8 (C2, 0.64 s); Cerebral 2:32.9 (E1, 0.9 s), 0:45.5 (B2, 3.49 s); 1-06 Lifeforms 5:03.5 (A#1, 3.32 s); Vertical Pig 0:54.2 (F#2, 0.13 s).
- Building blocks: **[JUDGMENT]** **Guess:** an inharmonic low bell or glass pad, one to two octaves below Ears 1's glass keys (which sit at 0.5 to 2 kHz); the missing piece is a low, inharmonic bell with a long tail. A bell-like pad at 40 to 90 Hz is a sound Driftloom does not have.

**S2. Bass that holds one note (Vertical Pig)**: bass-1, 288 notes, 27%
- In synth terms: C1 (33 Hz, 32 to 34), harmonic 0.94, soft attack (116 ms), falls 10 dB/s, near-mono; the roots are C 98% (an octave pedal, 50% octave moves). Wobbles 21 cents at 1.9 Hz and its speed moves 24 cents in 4 s (see T1).
- Hear it at 5:23.5 (C1, 0.24 s), 5:15.6 (C1, 0.39 s).
- Building blocks: **[JUDGMENT]** **Guess:** sub with bassStyle held, with a slow octave move; exists; below 40 Hz.

**S3. The kit (1-06 Lifeforms, Vertical Pig, Cerebral, Domain)**
- **Kick lines (kick_check):**
  - 1-06 Lifeforms: 372 kicks, a median of 53 Hz (C3 16%, B0 10%, G#1 9%, G1 9%: spread 32 to 130), falling 207 dB/s (200 to 214: a very steady, synthesized kick), every 0.455 s (the beat). **Duck:** void (the bass stem is empty); the drum stem's "bass" is the sub bass in the kick.
  - Vertical Pig: 228 kicks, a median of 59 Hz (A#1 12%, C3 11%), falling 250 dB/s (72 to 500), every 0.830 s (1.7 beats of 0.489 s: the kick plays in the triplet pattern). **Duck:** the bass is 2.5 dB down at the kick and back at the top by 145 ms: a small, short duck.
  - Cerebral: only 36 kicks, a median of 50 Hz (B0 22%), every 0.615 s (the beat), falling 92 dB/s; duck 3.7 dB down at the kick, recovered by 40 ms.
  - Domain: 19 kicks, every 3.9 s: not a kick pattern (a rare low hit; 12 of them: not claimed).
  - Under 60 Hz pitch is good to a semitone only.
- Building blocks: **[JUDGMENT]** **Guess:** tunedKick on the root with a quick, steady fall (200 dB/s); exists. The triplet pattern is below.

### Techniques

- **T1. Tape and pitch: per-track tuning offsets, little wobble.** Mix-level tuning across the 19 tracks runs -44 (Vit), -36 (Flak), -33 (Life Form Ends) on the flat side to +28 (Among Myselves) on the sharp side, median +7 (10th to 90th percentile -34 to +21); the four deep picks sit at -7 (Domain), -1 (Vertical Pig), +15 (Cerebral) and +21 (1-06 Lifeforms). The mix wobble is 5.1 cents median (4.0 to 6.0), a little above Driftloom's 3.6 and below Boards of Canada's 5.4 (Smokes Quantity's 19-cent wow). On stems: Vertical Pig's bass wobbles 21 cents at 1.9 Hz (speed moves up to 24 cents in 4 s); no other stem shows wobble at 4 cents or more. The most pitch bends a minute are on Among Myselves (38), Bird Wings (34), Omnipresence (33) and Room 208 (27); none of these four is a deep-pass track.
  - **[JUDGMENT]** **Guess:** the tuning offset again (a spread of about -45 to +30 cents per loop), but not the wow; a bass flutter of about 20 cents at 2 Hz is the one stem reading. See Boards of Canada's T1.
- **T2. A triplet kit: the first real case (Vertical Pig).** The drum stem's onsets fall on a grid of three to the beat, not two: of 1,475 onsets in the 12 twelfths of a beat (the meter tool's beat of 0.489 s, 122.6 BPM), 0, 3, 4, 7, 8 and 11 hold 15 to 18% each and the rest 1% (`beatphase.py`, in the raw archive). Positions 0, 4 and 8 are the three thirds of the beat (52% of the hits); 3, 7 and 11 are the same three a quarter-beat later (45%): so every hit is a pair a twelfth of a beat (41 ms) apart, a flam or a hit and its attack twice. There is **no straight eighth at all (0%)**, and the meter tool's "swung: off-beat at 0.67 of the beat" is this triplet grid, not swung eighths. In its terms: **the swing is at the drum grid's triplet (0.325 s, two thirds of a beat), not at an eighth or a sixteenth of the beat**: the whole kit is in threes, a 12/8 shuffle at 122.6 BPM. This is the case batch-5.md's triplet-shuffle lead was waiting for. Two cautions: it is one track; and the double hits may be the drums stem splitting one hit into attack and tail.
  - 1-06 Lifeforms reads "swung 0.65" in the card, but its beat-phase histogram is scattered (10 to 15% at 0, 1, 5, 10 and 11 twelfths, 4 to 8% elsewhere): a break, not a swung grid; the 0.65 is a broken-beat artifact, **not claimed**. Cerebral (straight eighths, 0.507) and Domain (no peak) are not swung.
  - **[JUDGMENT]** **Guess:** a `triplet` pattern option for a profile or a vibe (all three thirds of the beat, with an eighth never played): it is not what Driftloom's 0.3 swing of a sixteenth does (which puts the off-sixteenth at 0.65 of a step); a triplet grid is a different pattern family. Wait for a second record with it (batch-5.md listed about a dozen leads: *Analogue Bubblebath 5* 02 and 04, GAK 1 and 2, Polygon Window).
- **T3. The kit drops in and out, with long floor drops.** Vertical Pig: one drop of 29.8 s at 0:17.2 and kit in at 0:21.0 and 0:32.0, 3:03.0; 1-06 Lifeforms: drops of 14.8 s (2:02.5), 13.8 s (2:53.0) and 21.0 s (4:41.5); Domain 1.99 drops a minute (3 to 6 s); Cerebral's kit plays 26% of the time. The kit and bass enter on the downbeat of a phrase (at 3:03.0 and 3:04.0 on Vertical Pig, one second apart).
- **T4. Sweeps: large and frequent.** Vertical Pig has sweeps of 1.0 to 2.3 octaves every 5 to 10 s from 0:13.5; Domain's mix swings 1.6 octaves up (0:21.5 to 0:37.0) and 1.4 down; 1-06 Lifeforms is milder (0.9 to 2.1 octaves). The album's median sweep is 1.6 octaves, the largest of this pile (Boards of Canada 0.7, Schnauss 1.2). **[JUDGMENT]** **Guess:** padSweep at the fast end, like Schnauss.
- **T5. Echo and pump.** Vertical Pig's "drum echo at 319 ms (two repeats)" is 2/3 of the 0.489 s beat, a triplet quarter: the pattern repeating itself, not an echo claim. 1-06 Lifeforms' 5.8 dB "bass pump" is void (the bass stem is empty).
- **T6. Long form: nothing past 6 minutes.** The four picks run 2.8 to 5.8 minutes with a thin, bright opening and a fade, as on Schnauss; the drift ratio (60 s against 10 s) is 1.33 median, 18 of 18 above 1.

### Theory
- **Th1. One pedal, major sevenths, sus2.** Vertical Pig: C5 69%, Gsus2 10%, Cmaj7 5% over a C pedal (C lydian, margin 0.018, a guess), 9 changes a minute. Domain: D#maj7 26%, Cmin7 13%, D#5 12%, G#maj7 9% (D# ionian, 0.015, a guess), 27 a minute. 1-06 Lifeforms: C5 22%, Cmin7 21%, D#5 21%, A#min7 15% over D# and C (D# mixolydian, 0.014, a guess), 12 a minute, a median chord of 6.3 s. Cerebral: Esus2 28%, Emin 14%, Asus2 12% over E and B (E aeolian, 0.011, a guess), 32 a minute (15 settled: dense). Major sevenths show up here (Domain, Vertical Pig), where Boards of Canada's four picks gave minor sevenths and sus2.
  - **[JUDGMENT]** **Guess:** a pedal root with a major seventh or a sus2 on top, 9 to 27 changes a minute.
- **Th2. Top lines in fourths and fifths.** 1-06 Lifeforms 46% fourths and fifths (median interval 5), phrases of 17 notes, 48% arches and 36% rising; Cerebral 30%, 53% rising phrases of 10 notes; Vertical Pig and Domain move by step, third and leap in a mix (range 14 to 19 semitones).
- **Th3. Short loops.** Vertical Pig repeats every 1 bar (0.71) of 1.0 s, then at 21 bars (0.21): one tight loop that rarely repeats exactly (4% near-duplicate bars). 1-06 Lifeforms 1 bar (0.60) and 8 bars (0.43); bar length unclear on all four.

### Space
- **Domain is the widest record in this pile so far.** Mix width by band (dB; 0 = unrelated channels): Domain sub -2, 120 to 500 -3, 500 to 2 kHz -1, 2 to 5 kHz -2, above 5 kHz -2 (corr 0.11 to 0.36 in every band), the drums, bass and other all -2 dB (so even the sub and the kit are almost decorrelated); Cerebral -13, -8, -10, -3, -1; Vertical Pig -10, -8, -7, -5, -11; 1-06 Lifeforms -16, -12, -9, -9, -15. The album's median width is -11.1 dB.
- **What the mix loses in mono** (below 120 Hz, 120 to 500, 500 to 2 k, 2 to 5 k, above 5 k): Domain **2.4, 1.6, 2.5, 2.0, 2.0 dB** (worst on the sub: correlation 0.15); Cerebral 0.2, 0.6, 0.5, 1.8, 2.2; Vertical Pig 0.5, 0.7, 0.8, 1.1, 0.4; 1-06 Lifeforms 0.1, 0.3, 0.5, 0.5, 0.1. Across the other 15 tracks: Bird Wings loses 1.7 to 2.3 dB in every band (corr 0.18 to 0.35, sub 1.7 dB), Interstat 1.8 to 2.5 dB above 120 Hz, the rest under 1.1 dB. So three tracks have a sub that is not mono (Domain 0.15, Bird Wings 0.34, Interstat 0.81), and **a kick or bass in the drums stem that loses 2 dB in mono (Domain: drums 2.0, bass 1.9 dB)**.
- **[JUDGMENT]** **Guess:** a stereo case where bass and kick should stay centered (the guards say so); Domain is a record where they do not, and loses 2 dB in the fold: a design edge, not a goal.

### Vibes
Numbers: 1-06 Lifeforms 133 BPM, 8 settled chords a minute, 2.5% above 2 kHz (mix level), width -11 dB; Cerebral 97 BPM, 15, 5%, -8 dB; Domain 133 BPM, 17, 3%, -2 dB; Vertical Pig 123 BPM (triplet), 6, 4%, -9 dB.
- **[JUDGMENT]** Lean toward (x1.5 to x3): mysterious, strange, experimental, hazy, dreamy, cosmic, spacious, foggy, moonlit, hypnotic. Lean away (x0.2 to x0.5): cozy, sunny, merry, cheeky, bright, sweet. Never 0. **Guess:** the reference record for strange and moonlit (and its width and low bells fit the "experimental" vibe).

### Wholesome take
- Take the low inharmonic bells and the slow pads, the held C pedal with major sevenths, the fast filter sweeps, the steady synthesized kick, the triplet kit (one pattern family, softened) and the tuning offsets; leave the double-hit flam of the kit for a listen, and the out-of-phase or wide sub (Domain, Bird Wings). **[JUDGMENT]** **Guess:** wholesome at a gentle level; the record's mood is eerie, and mysterious is the nearest vibe, not peaceful.

### For later
- 04 Bird Wings (1:30): the worst mono loss across the record (1.7 to 2.3 dB in every band), 34 pitch bends a minute, a sub that is not mono. 11 Interstat (0:55): the second-worst fold. **[JUDGMENT]** **Guess:** wait; both are short.
- 08 Among Myselves, 16 Omnipresence, 17 Room 208: the most pitch bends a minute (38, 33 and 27); 08 is also the sharpest track (+28 cents). **[JUDGMENT]** **Guess:** a deep pass on Among Myselves would test tape-speed bends (the deep-pass tracks here show few).
- 15 Vit (-44 cents), 03 Flak (-36) and 14 Life Form Ends (-33): the flattest tracks. **[JUDGMENT]** **Guess:** listen for what a track 40 cents flat sounds like next to a track on pitch.
- Groups left over: Vertical Pig bass-2 and other-2, Cerebral bass-1 and vocals-1 (a quarter of the power: a texture), Domain bass-1 (D#2, bell-like, 235 notes). **[JUDGMENT]** **Guess:** wait.

### Reliability notes for this card
- The picker took the typical track and the three least like it, so the deep pass covers 20% of the album's time. Tracks are short, so every long-form line is a 3 to 6 minute shape.
- The beat-phase result rests on the meter tool's beat and phase; for Vertical Pig the beat (0.489 s) and the grid (0.325 s) agree on three to a beat, and the histogram has twelve bins of 1/12 beat (±0.04 s). A drum stem that doubles each hit would produce the paired peaks; it could not produce the empty straight-eighth bin.
- 1-06 Lifeforms and Cerebral have a bass stem that is empty or thin; the kick lines may include sub bass.

---

## Bowery Electric, Beat (1997)

Ears 11, 2026-11-03. 11 tracks, 70.9 minutes, all at mix level (`monoloss.py` and the album medians too). Three got the deep pass: 03 Without Stopping (typical, 5:13), and the two least like the rest, 06 Looped (2:40) and 05 Fear of Flying (5:40). `longform.py` ran on the three and on 10 Postscript (16:41, mix level only: the one track of 8 minutes or more; no stems, so no stem check on it). Stems by Demucs htdemucs.

Machine time: 1,473 s (24.6 min) for 13.6 minutes of deep-pass audio, run alongside another album's chain:

- pick 158 s (11 tracks);
- Demucs 911 s (313, 160 and 340 s of audio in 408, 186 and 317 s);
- measuring, the stem check and the card about 404 s (1,315 s from the first split to the card); the kick, beat-phase, mono and long-form checks about 5 minutes more, not counted.

Ears 9's formula (1.1 x 813 s + 12 x 11) says 1,026 s, so this ran about 44% over it (Demucs was slowed by the other chain; run alone it would be 0.5 to 0.65 times the audio).

### Stem check (read first)
- **03 Without Stopping**: drums 26%, bass **68%**, other **3%**, vocals 3%. The other stem is nearly empty, so its top line, chord, echo and pumping numbers are void or guesses (the chord counter below reads the bass and kit). The vocals stem plays 71% of the time at 3% of the power: a texture, not described further. The meter tool's 138.8 BPM (0.432 s) is the beat; the card's "35.1 BPM" grid is a **bar of four beats**, not a beat (Ears 10b's lesson): the 4-beat grid reads 140.6 BPM.
- **05 Fear of Flying**: drums 65%, bass 28%, other 7% (thin: top line and chords are guesses), vocals 0.2% (bleed). Meter 94.5 BPM (0.635 s) and grid 93.9 BPM agree: the beat. One kick a bar (every 2.545 s = four beats).
- **06 Looped**: other **100%**: drums and bass stems are empty. No beat: meter, swing and on-beat numbers are void (the meter tool's "swung 0.77" is on the mix).
- **10 Postscript**: no stems (mix level only).
- No subharmonic misreads claimed. Without Stopping's bass is on E1 at 41 Hz (41 to 41) with an E2 above it, under 60 Hz and good to a semitone only; Fear of Flying's kick is at 46 Hz.

### The hook
- Dark, heavy and slow: an E or B drone-pedal in the bass, a kit that never leaves, and a wide, distorted wall. It is the darkest record of the pile: centroid 223 Hz median (Boards of Canada 357, Schnauss 395), 55% of the energy under 120 Hz and 35% in 120 to 500 Hz, only 0.6% in 2 to 5 kHz and 0.1% above 5 kHz. Loud-ish (-14.5 LUFS), in tune (tuning -3 cents median, wobble 3.4 cents), almost no chord movement at mix level (1.8 changes a minute median) and the smallest sweeps of the pile (0.39 octave median).
- **[JUDGMENT]** One line a listener would say. **Guess:** a long drive at night: one low note, one slow beat, and a haze of noise and bells that never lifts.

### Sounds

**S1. The held, repeated sub bass (Without Stopping): bass-1 and bass-2, 214 and 179 notes, 32% and 29%**
- In synth terms: E1 (41 Hz, 41 to 41) and E2 (82 Hz, 80 to 83), harmonic share 1.00 and 0.94 (pure), slow swell (attack 210 and 208 ms), falls 0 to 1 dB/s, mono (correlation 1.00). One root: E 100%, 97% repeats, 1.10 notes a second. The bass stem holds 68% of the track's power, -0.8 dB under the mix. Wobbles 16 cents at 1.2 Hz (a slow wow on the bass only, see T1).
- Hear it at 2:46.7 (E1, 0.34 s), 2:43.2 (E1, 1.02 s), 0:42.2 (E2, 0.62 s), 4:39.9 (E2, 1.73 s).
- Building blocks: **[JUDGMENT]** **Guess:** sub with bassStyle held and a repeated pulse (a note a second), with its octave above: exists, as on Mirror's Edge's New Eden.

**S2. Out-of-phase bells (Looped): other-1, 475 notes, 96%**
- In synth terms: about C#3 (136 Hz, 132 to 138: one narrow pitch), bell- or metal-like (harmonic share 0.47, partials off whole multiples), attack 86 ms, falls 19 dB/s, pan -0.13, **left-right correlation -0.34** (the channels are partly inverted); a second group at about B1 to F1 (61 Hz).
- Hear it at 1:33.0 (C#3, 0.95 s), 0:35.8 (C#3, 0.11 s), 1:27.5 (C3, 0.87 s).
- Building blocks: **[JUDGMENT]** **Guess:** glass keys or a bell at 136 Hz, held in one chord (Amaj7 98% of the time); the inverted stereo is a space recipe Driftloom must not do (see Space).

**S3. The long soft kick and a sub swell (Fear of Flying): drums-1 and drums-2, 234 and 58 notes, 30% and 10%**
- In synth terms: drums-1 a low thump at F#1 (46 Hz, 46 to 47), 33 ms attack, brightening over the note; drums-2 the same pitch with a slow swell (attack 312 ms), 0.6 s long.
- **Kick line (kick_check):** 112 kicks, a median of 46 Hz (F#1 66%, G1 12%), soft (58 dB/s: a long, boomy kick), **every 2.545 s, one a bar** at 94.5 BPM. **Duck:** the bass drops hard after the kick, 5 dB by 50 ms, **10 dB by 200 ms, 15 dB at the lowest (1.44 s)**, and comes back to within 0 dB by 750 to 850 ms, then dips again before the next kick: a deep, long sidechain-like duck; the bass and the kick take turns (the technique tool's 3.6 dB "pump" is a floor tied to the wrong kick period, 0.48 s).
- For the other two: Without Stopping 419 kicks, median 126 Hz (C3 47%, B2 18%: at the top of the 30 to 130 Hz window, so this is a low tom or snare body, not a sub), spacing 0.575 s, falling 107 dB/s, **no duck** (the bass stays at -0.5 dB). Looped has no kit (16 "kicks" are bell notes).
- Hear it at 0:17.6 (G1, 0.37 s), 0:12.5 (F#1, 0.39 s); the swell at 0:48.5 (F#1, 0.62 s).
- Building blocks: **[JUDGMENT]** **Guess:** tunedKick at 46 Hz with a long tail, one a bar, with subDuck at 10 to 15 dB, a recovery of 600 to 700 ms (long) and a second dip before the next kick; the duck is much deeper than Schnauss's 3 to 6 dB.

### Techniques

- **T1. Tape: none; a slow bass wow on one track.** The mix tuning is -3 cents median (-28 to +10) and the wobble 3.4 cents (2.2 on Postscript): the same as Driftloom (3.6). Stems: Without Stopping's bass wobbles 16 cents at 1.2 Hz and its other stem 10 cents at 0.7 Hz; Looped's other stem 6 cents at 2.0 Hz with speed moves up to 27 cents in 4 s; Fear of Flying nothing at 4 cents or more. These are small and on single stems; not claimed as tape. Against Boards of Canada, this record is in tune.
- **T2. The deep duck (Fear of Flying)**: see S3: 10 to 15 dB, long.
- **T3. A kit that never leaves, and no floor drops.** Floor drops 0.00 a minute on all three; Without Stopping's kit plays 99% of the time (out only at 5:09.5), Fear of Flying's 97% (out at 5:30.0). **[JUDGMENT]** **Guess:** this is a "steady" record: a loop with no drops.
- **T4. Openings that sweep.** Without Stopping: the mix rises 3.3 octaves in the first 6.5 s and falls 3.1 octaves from 0:13.5 to 0:24.5; Fear of Flying rises 2.1 octaves (0:00 to 0:09.0) and 2.8 (0:14 to 0:23), then a sweep of 1.5 to 3 octaves every 5 to 9 s (37 sweeps in 5.7 minutes). Without Stopping's later sweeps are 0.8 to 1.7 octaves (20 in 5.2 minutes). **[JUDGMENT]** **Guess:** an opening filter arc of 3 octaves in the first 7 s; padSweep exists.
- **T5. No swing claim.** Without Stopping: the meter tool says "swung 0.66" at a beat of 0.432 s, but the drum onsets (897) are spread evenly through the beat (7 to 10% in each twelfth): no grid at the eighth or the triplet; **not claimed** (a hat roll or a sampled break). Fear of Flying: the meter's off-beat is 0.509, and the hits sit on the beat, the off-beat and a twelfth of a beat early (11/12, 5/12): **straight eighths at the drum grid's eighth** (the same phase offset on both), not a shuffle.
- **T6. Long form: one key, one pulse, three small events (Postscript, 16:41).** The track holds **B as the strongest pitch class in 15 of 16 minutes** (the key changes 1.2 times in 10 minutes), the centroid at 150 to 250 Hz (a dip to 115 Hz at 3:00 and 148 Hz at 14:00: darker low moments), and the level within 5 dB of -21 LUFS from minute 1 to 14 (-28.6 in the first minute, -17.9 at its loudest, -43 in the last: the fade-in and fade-out). The width opens from -4.5 dB to -0.8 dB at 6:00 to 8:00 and returns. Spectral flux is the lowest of the tracks read so far (0.16 against 0.29 to 0.42). Sections at 3:00, 4:40, 8:50, 10:30, 12:50, 14:10 and 15:30; returns 0.27; the end does not come back to the opening. Without Stopping and Fear of Flying (5 minutes) are as static: level range 8.2 and 1.0 dB.
  - **[JUDGMENT]** **Guess:** for a 32,768-bar loop or an Endless one: a drone loop whose only long-form events are three slow darkenings and one widening over 16 minutes; the shape is "almost nothing happens", and what a listener hears as change is a 1 to 3 minute width or brightness drift of a few dB. The long-form composing comes later; this is the quietest template so far.

### Theory
- **Th1. A pedal and almost nothing else.** Without Stopping: E5 84%, Emaj7 5%, Fmaj7 4% (E phrygian, margin 0.005, a guess; fits E pentatonic), E 100% in the bass; 9 changes a minute with a median chord of 3.1 s, 6 distinct chords. Looped: Amaj7 98% (C# aeolian, margin 0.024; A major and C# minor share the notes), 1 change a minute. Fear of Flying: E5 37%, Dsus2 18%, A5 15%, Asus2 7% over a walking bass (E 51%, A 22%, D 9%, F# 7%; steps 32%, octaves 16%; 1.84 notes a second), E mixolydian (0.021). The 66 changes a minute on Fear of Flying are the counter reading a wall of noise (2 settled a minute); not trusted.
  - **[JUDGMENT]** **Guess:** a one-chord loop (the fifth or a major seventh on top of the root); a long hold with 1 to 9 changes a minute.
- **Th2. Fourths and fifths.** Fear of Flying's top line is 46% fourths and fifths (median interval 5 semitones, phrases of 14 notes, 43% arches), from a thin other stem (7%): a guess. Looped's is 51% thirds in a range of 4 semitones.
- **Th3. Long repeats.** Fear of Flying repeats at 24 bars (0.56) and 2 bars (0.47) of 5.1 s: a phrase of 2 bars and a section of 24 (2 minutes). Without Stopping repeats every bar (0.44) and 3 bars of 1.7 s (the bar of four beats at 139 BPM).

### Space
- **Mono bass, wide and inverted top.** Mix width by band (dB; 0 = unrelated channels): Without Stopping sub -25, 120 to 500 -7, 500 to 2 kHz -4, 2 to 5 kHz -7, above 5 kHz -7; Fear of Flying -25, -3, -3, 0, -7 (the 2 to 5 kHz band is nearly unrelated: correlation 0.05); **Looped +2, +4, +2, -2, -2** (a width above 0 dB means the side is louder than the mid: the channels are partly inverted). By part: the drums and bass are narrow (-24 to -38 dB, mono loss 0.0 dB); the other stem is wide (-2 to +3 dB) and loses 2.0, 3.7 and 4.8 dB in mono.
- **What the mix loses in mono** (below 120 Hz, 120 to 500, 500 to 2 k, 2 to 5 k, above 5 k): Without Stopping 0.0, 0.8, 1.6, 0.7, 0.7; Fear of Flying 0.0, 1.6, 1.9, 2.7, 0.8; **Looped 6.9, 5.1, 4.5, 2.5, 2.8** (correlation -0.60, -0.39, -0.31 in the three lower bands: **the bass of the whole track is out of phase between the channels, and the fold loses 7 dB of it**). The album's other tracks: Empty Words 2.9 and 3.1 dB at 500 Hz to 5 kHz (corr 0.03 and -0.03), Black Light 3.1, Coming Down 2.5 to 2.7, Fear of Flying 2.7; Postscript 1.4 to 2.6 dB in every band including the sub (corr 0.44: the sub is not mono there). The pile's album width median is -7.1 dB.
- **[JUDGMENT]** **Guess:** Looped is the case the mono switch and the guards (stereo fold within limits; nothing essential on one side only) exist for: never invert a channel; a hard fold of the other tracks costs 2 to 3 dB in the middle bands.

### Vibes
Numbers: Without Stopping 139 BPM (the kit; felt 69), 6 settled chords a minute, 0.6% above 2 kHz (mix level), width -9 dB (median of 2 s windows); Looped 139 BPM (no kit), 1 chord a minute, bells, width +3 dB (inverted); Fear of Flying 94.5 BPM, 2 settled, -2 dB; Postscript 123 BPM by the mix-level tool, 1.8 chords a minute, width -3.7 dB.
- **[JUDGMENT]** Lean toward (x1.5 to x3): hypnotic, droning, pulsing, steady, mysterious, dusky, hushed, glacial, still, booming. Lean away (x0.2 to x0.5): sunny, merry, bouncy, bubbly, zany, goofy, twinkling. Never 0. **Guess:** the reference record for droning with a pulse (darker than Mirror's Edge, a mono floor and a very deep duck).

### Wholesome take
- Take the held E sub, the one-a-bar kick with its deep duck, the single-chord loop with bells, the opening sweep and the quiet long-form drift; leave the inverted stereo (never), the wall of noise on Fear of Flying (its 66 chord changes a minute and 0 dB width at 2 to 5 kHz) and the heaviest low end (55% under 120 Hz). **[JUDGMENT]** **Guess:** wholesome in small doses: the dark weight sits behind droning, still and hushed; drop the loudness (-14.5 LUFS).

### For later
- 10 Postscript (16:41): a 16-minute drone. A deep pass would need a windowed stem check (Ears 10's lesson: `check_stems.py` ran out of memory at 13 minutes on this computer). **[JUDGMENT]** **Guess:** a deep pass of a 10-minute window (3:00 to 13:00) after the long-form read above, for what makes its three darkenings.
- 02 Empty Words, 07 Black Light, 09 Coming Down: the next-worst mono folds (2.5 to 3.1 dB at 500 Hz to 5 kHz). **[JUDGMENT]** **Guess:** wait.
- Groups left over: Looped other-2 (B1 to F1, 24 notes, out of phase), Without Stopping's vocals texture (71% of the time), Fear of Flying drums-2 (the sub swell). **[JUDGMENT]** **Guess:** wait.

### Reliability notes for this card
- Three picks of eleven; Postscript is read at mix level only, and the other seven tracks are known only by the picker's numbers.
- Without Stopping's other stem holds 3% of the power; every number from it is void. Looped has one stem; its "other" numbers are the mix.
- The beat-phase check uses the meter tool's beat and phase; a phase error of a twelfth of a beat (about 40 to 50 ms) is seen on Fear of Flying (hits at 5/12 and 11/12). It does not change straight against swung.
- The kick lines on Without Stopping (126 Hz) are a low tom, not a kick; the duck there is void.

---

## Ulver, Perdition City (2000)

Ears 11, 2026-11-03. 9 tracks, 53.7 minutes, all at mix level (`monoloss.py` and the album medians too). Three got the deep pass: 05 The Future Sound of Music (typical, 6:40) and the two least like the rest, 06 We Are The Dead (3:41) and 07 Dead City Centres (7:11). `longform.py` ran on the three and on 04 Tomorrow Never Knows (7:59: a second under 8 minutes, read as one; mix level only). Stems by Demucs htdemucs. The vocals on this record are described only as textures; no words.

Machine time: 1,877 s (31.3 min) for 17.5 minutes of deep-pass audio, run alongside another album's chain:

- pick 110 s (9 tracks);
- Demucs 1,096 s (400, 221 and 431 s of audio in 354, 234 and 508 s: about 1.0 times the audio while sharing the cores);
- measuring, the stem check and the card about 671 s; the kick, beat-phase, mono and long-form checks about 4 minutes more, not counted.

Ears 9's formula (1.1 x 1,052 s + 12 x 9) says 1,265 s, so this ran 48% over it (the two chains competing for the cores, as on the other four).

### Stem check (read first)
- **05 The Future Sound of Music**: drums 23%, bass 56%, other 17%, vocals 4% (bleed). The meter tool's 91 BPM (0.658 s) is the beat; the drum grid's 184.6 BPM (0.325 s) is its eighth.
- **06 We Are The Dead**: drums 2%, **bass 1%** (empty), other 41%, **vocals 56%**. A voice-like texture holds more than half the power; the track is a vocal piece over a pad. No beat to speak of: its meter, swing and pumping lines are void, and the 30 "kicks" are not kicks.
- **07 Dead City Centres**: drums 5%, **bass 1%** (empty), other 81%, vocals 13%. The meter's 71 BPM does not match the drum grid's 206.9 BPM by a simple ratio, and the drum stem is thin (243 onsets in 7 minutes): the beat is weak or hidden; I do not claim one.
- No subharmonic misreads claimed. The Future Sound of Music's bass at F#1 (46 Hz, 37 to 46) is under 60 Hz: good to a semitone only.

### The hook
- Cold, wide, slowly changing night music. A hushed pad and a voice-like texture over a very low bass, and a kit that comes in for a few minutes and leaves. It has **the biggest dynamics of the pile**: an arc of 12.9 dB median (Schnauss 6.3, Lifeforms 6.7, Bowery 0.8), sweeps of 1.7 octaves, and 12.3 chord changes a minute. It is in tune (tuning -2 cents median, wobble 3.4 cents: digital) except one track (We Are The Dead, -29 cents), mid-loud (-15.1 LUFS), and dark (centroid 500 Hz; 56% of the energy under 120 Hz).
- **[JUDGMENT]** One line a listener would say. **Guess:** an empty city at 3 a.m.: a soft, wide hush that suddenly gets loud and then goes quiet again.

### Sounds

**S1. A voice-like texture and a bright wide pad (We Are The Dead): vocals-1 and other-1, 270 and 165 notes, 43% and 49%**
- In synth terms: the pad is about C4 (258 Hz; 51 to 1,093: a wide spread), slow swell (attack 387 ms, up to 1.4 s), falls 5 dB/s, mixed partials (harmonic share 0.34), **left-right correlation -0.09** (almost unrelated, partly inverted), moving from side to side (pan range 1.06, cycling every 35 s). The voice-like group sits at F#2 (91 Hz), soft attack, falls 27 dB/s, correlation 0.75. No words taken.
- Hear it at 0:05.0 (F4, 0.55 s), 0:05.4 (G4, 0.63 s); the voice texture at 2:13.1 (F2, 1.1 s), 3:18.9 (D3, 2.8 s).
- Building blocks: **[JUDGMENT]** **Guess:** wide, slowly panning pad (stereo that moves was liked as an idea, disliked in practice: Ears 4); the voice-like layer is hum or choir, with no words.

**S2. Low, noisy, wide pads (Dead City Centres, The Future Sound of Music): other-1 and other-2, 246, 203, 167 notes, 25% to 57%**
- In synth terms: Dead City Centres A1 (54 Hz; 34 to 196), slow swell (176 ms), falls 10 dB/s, mixed partials, correlation 0.85 and a second group at A2 (108 Hz) that falls 45 dB/s (a plucked or struck sound); The Future Sound of Music B2 (124 Hz), bell- or metal-like, slow swell (194 ms), falls 12 dB/s, correlation 0.65. Dead City Centres' other stem holds 84% of the power: the whole track is pad.
- Hear it at Dead City Centres 6:01.9 (C3, 1.41 s), 6:03.2 (C3, 1.58 s), 6:50.0 (0.33 s); The Future Sound of Music 3:09.1 (B2, 2.02 s).
- Building blocks: **[JUDGMENT]** **Guess:** softpad, low (A1 to A2), slow attack; exists; the 45 dB/s struck one is a pluck.

**S3. A low bass and the kit (The Future Sound of Music): bass-1, 194 notes, 26%**
- In synth terms: F#1 (46 Hz), harmonic share 0.98, soft attack (135 ms), falls 2 dB/s, centered; roots D 50%, F# 32%, E 10% (0.59 notes a second). The bass stem holds 56% of the power.
- **Kick line (kick_check):** 331 kicks, a median of 61 Hz (A#1 20%, B1 14%, E2 10%), falling 92 dB/s (26 to 185), spacing 0.343 s (about half a beat). **Duck:** none (the bass stays within 0.4 dB). The other two tracks' kick and duck lines are void (empty bass stems).
- Hear it at 5:40.7 (2.26 s), 5:35.1 (D1, 3.31 s), 5:45.2 (F#1, 5.82 s).
- Building blocks: **[JUDGMENT]** **Guess:** sub with bassStyle held on the root and the major third (D and F#), with a tunedKick that is not ducked; exists.

### Techniques

- **T1. Huge dynamics: quiet, then a wall.** The Future Sound of Music: the level goes -32, -20, -18, -23, **-10**, -10.5, -15 dB a minute (a 12 dB rise at 4:00; swells at 1:50 and 5:00, 5.4 and 6.0 dB), and the kit enters at 1:11.5 and 3:58.5 and leaves at 2:57.0 and 6:22.0, so **the drums and the 12 dB rise land together at about 4:00**. Dead City Centres: -29, -37, -25, -22, -22, -19.5, -36 dB, the second minute a bright, thin gap (centroid 2,488 Hz, 12 dB under the first minute, width -11.7 dB) before the full pad, an 18 dB swell at 5:30 and a one-minute fade at the end (the level is -36 at 6:00 and -56 at 7:00). Tomorrow Never Knows: -24, -19, -17, -14, **-13** (the peak, at 4:00) and then -19, -32, -35: the last two and a half minutes are 15 to 20 dB quiet, so the track ends in a long quiet tail rather than a fade.
  - **[JUDGMENT]** **Guess:** the arc this record gives is a build over 3 to 4 minutes to a loud section, and a drop to a quiet tail; the album's 12.9 dB arc is the largest read so far. A long loop could take the quiet-to-loud arc as a long-form shape (for Mikey's ears).
- **T2. Fast sweeps in the pad.** The Future Sound of Music: the mix rises 2.8 octaves in the first 5 s (0:04.5 to 0:09.5) and falls 2.3 octaves 0:55 to 1:04; Dead City Centres: 4.1 octaves down (2:29.5 to 2:40.5), 3.1 up (2:50.5 to 3:02); We Are The Dead: 3.5 octaves up 3:01.5 to 3:08.5 and the other stem 4.0 up and 3.6 down (the pad opens and closes by 4 octaves in 8 s). The album's median sweep is 1.7 octaves (Lifeforms 1.6, Schnauss 1.2).
- **T3. Straight sixteenths (The Future Sound of Music).** The drum stem's onsets (1,225) sit at 2, 5, 8 and 11 twelfths of a beat: four equal hits to the beat, 0.163 s apart, shifted by a sixteenth-ish phase error (the meter's phase, 2 twelfths): so the grid is **straight sixteenths at 91 BPM**, no swing at the sixteenth (the 4 positions are 3 twelfths apart: 0.250 of a beat each) and none at the eighth (the meter's off-beat is 0.501). Swing at the drum grid's sixteenth: **straight (0.50)**. This is not a triplet-shuffle case. Dead City Centres and We Are The Dead have no clear grid (the meter's "swung 0.57 and 0.58" are on the mix and void).
- **T4. Tape: none.** Mix wobble 3.4 cents (1.8 to 4.5): the same as Driftloom. Stems: The Future Sound of Music's other stem wobbles 23 cents at 0.3 Hz, a slow wow, with its bass 5 cents at 0.4 Hz and a speed of up to 20 cents in 4 s; Dead City Centres 6 cents at 5.5 Hz; We Are The Dead nothing at 4 cents or more. The 23 cents at 0.3 Hz is the one slow drift; with the pad slowly gliding, I did not call it tape.
- **T5. Echo.** The Future Sound of Music's "drum echo at 479 ms, two repeats, +7.5 dB" is 0.73 of the beat (0.658 s): close to Driftloom's three-quarter-beat echo, and the drum pattern itself could make it; **not claimed**, but worth a second look in a record where echo times matter (batch-5.md's echoTime lead).
- **T6. Long form.** Dead City Centres: 3 brightness swells, 19.5 pitch-class changes in 10 minutes (the most of any track read), returns 0.35. Tomorrow Never Knows: its width is near-mono (-20.5 dB) for 5 minutes and opens to -2.8 dB in the last two; the strongest pitch class changes 6.4 times in 10 minutes. The Future Sound of Music: no returns (0.00), 4.6 changes. **[JUDGMENT]** **Guess:** a track that travels and does not come back; the quiet-to-loud arc and the opening of the width in the last two minutes are the long-form shapes to take.

### Theory
- **Th1. Major sevenths and sus chords.** We Are The Dead: Cmaj7 41%, D#maj7 19%, G#maj7 6%, C#maj7 6% (B phrygian, 0.006, a guess), 23 changes a minute. Dead City Centres: G#maj7 21%, D#maj7 10%, Gmaj7 8% (G aeolian, 0.003, a guess), 25 changes, 44 distinct chords. The Future Sound of Music: D5 22%, F#7 16%, F#min 12% over D and F# (D ionian, 0.003, a guess), 25 changes (12 settled). Chord names on dense or noisy music are a guess.
  - **[JUDGMENT]** **Guess:** major sevenths on a few roots (C, D#, G#, C#) with no minor or dominant chords: a pad of stacked major sevenths; the chord names are a guess, and the pad is dense.
- **Th2. A wide top line.** The Future Sound of Music's top line spans 21 semitones (10th to 90th percentile) with fourths and fifths 24%; Dead City Centres 28 semitones and 27% leaps; We Are The Dead 30 semitones. These are the widest ranges of the pile (from a mixed other stem: read by ear first).
- **Th3. Repetition.** The Future Sound of Music repeats every bar of 5.3 s (0.81) and 48% of its bars are near-duplicates: a loop; Dead City Centres 4%, We Are The Dead 0%: through-composed.

### Space
- **Mostly narrow, with two very wide tracks.** Mix width by band (dB; 0 = unrelated channels): The Future Sound of Music sub -18, 120 to 500 -10, 500 to 2 kHz -6, 2 to 5 kHz -7, above 5 kHz -9; **We Are The Dead -17, -8, -1, -9, -22** (almost unrelated at 500 Hz to 2 kHz: correlation 0.11, the stem width 0 dB); Dead City Centres -9, -8, -3, -7, -10. The album's median width is -13.8 dB, narrower than Schnauss (-7.8) and Lifeforms (-11.1), with a spread (Catalept -22.7, We Are The Dead -2.5).
- **What the mix loses in mono** (below 120 Hz, 120 to 500, 500 to 2 k, 2 to 5 k, above 5 k): The Future Sound of Music 0.1, 0.4, 1.0, 0.8, 0.4; We Are The Dead 0.1, 0.6, **2.6**, 0.5, 0.0; Dead City Centres 0.6, 0.6, 1.8, 0.9, 0.3; Tomorrow Never Knows 0.0, 0.1, 0.2, 0.1, 0.1 (near-mono, loses nothing). The other tracks: at most 1.2 dB at 2 to 5 kHz (Scars Of Cold Kisses). So the fold costs 2.6 dB in the 500 Hz to 2 kHz band of the voice-and-pad track and under 2 dB elsewhere; the low end folds cleanly on every track.
- **[JUDGMENT]** **Guess:** a record with a mono bottom, a mono-ish middle on most tracks and one very wide vocal pad; the mono switch is almost free here, and the width "opening late" in Tomorrow Never Knows (T6) is a design idea for a long loop.

### Vibes
Numbers: The Future Sound of Music 91 BPM (felt; sixteenth grid), 12 settled chords a minute, 9% above 2 kHz (mix level), width -7 dB (median of 2 s windows); We Are The Dead no beat, 14, 10%, -1 dB; Dead City Centres no beat, 15, 23% (the brightest of the pile), -5 dB.
- **[JUDGMENT]** Lean toward (x1.5 to x3): mysterious, moonlit, spacious, misty, hushed, dusky, strange, still, cosmic, foggy. Lean away (x0.2 to x0.5): sunny, merry, bouncy, zesty, cheeky, goofy, bubbly. Never 0. **Guess:** the reference record for moonlit with a very large dynamic arc.

### Wholesome take
- Take the quiet-to-loud arc, the fast pad sweeps, the chain of major sevenths, the held low root and the width that opens late; leave the sung texture and the loudest minutes (-10 LUFS), and keep a nearly unrelated pad (We Are The Dead's, correlation -0.09 at 500 Hz to 2 kHz) mild. **[JUDGMENT]** **Guess:** wholesome in its quiet half; the loud half is heavy (56% under 120 Hz), so tame it. The brief says "wholesome takes only, but harsh albums are fair game": this one is dark but not harsh.

### For later
- 04 Tomorrow Never Knows (7:59): the near-mono track that opens late. 01 Lost In Moments (7:17) and 03 Hallways Of Always (6:36): the loudest (-12.7 and -11.3 LUFS) and the most chord changes (12.9 and 18.2 a minute). 09 Nowhere - Catastrophe (4:49): 21.6 chord changes a minute, a beat at 136 BPM (mix level). 08 Catalept (2:17): the steadiest pulse (0.80), the widest wobble (7.8 cents), the narrowest width (-22.7 dB, tied with Tomorrow Never Knows). **[JUDGMENT]** **Guess:** a deep pass on Hallways Of Always and Nowhere - Catastrophe would add the chord-heavy tracks; the picks here are the quiet, strange ones.
- Groups left over: The Future Sound of Music other-2 and vocals, We Are The Dead drums (2%), Dead City Centres vocals (13%, a texture). **[JUDGMENT]** **Guess:** wait.

### Reliability notes for this card
- Two of three deep-pass tracks have an empty bass stem and almost no drums, so the low-end and beat numbers rest on one track (The Future Sound of Music). Chord names are guesses on dense pads (margins 0.003 to 0.006).
- Tomorrow Never Knows is 479.4 s; I ran `longform.py` on it by hand at mix level because it is one second under the 8-minute line.
- The sixteenth reading rests on the meter's beat and phase; a phase error of two twelfths of a beat is visible (hits at 2, 5, 8, 11), which does not change straight against swung.
