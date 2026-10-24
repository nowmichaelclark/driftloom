# Ears cards, pile 3

Ears 9 (Sonnet 5.5, medium), started 2026-10-24. Seven Aphex Twin records: Richard D. James Album, Girl/Boy EP, Come to Daddy EP, 51/13 (only tracks not elsewhere in the pile), Windowlicker EP, 2 Remixes by AFX, Drukqs. Cards come from `tools/ears/album.py` (run with `python3 -P`), checked against the limits in `docs/EARS.md`. Lines marked **[JUDGMENT]** are for the synthesis; each carries a short first guess marked **Guess**. Vibe names are only ones in `js/vibes.js`.

**Stems are Demucs htdemucs** on every card. The weights downloaded fine (no HTTP 403), so there is no MDX-Net fallback to report. I checked this with `split.py --mode demucs` on a 20 s clip before the first album. The machine has 2 cores and 7 GB, so one album ran at a time.

**Twins.** `twins.py` (row length within 3 s and loudness within 0.6 LU, `--artist Aphex`) over the new picker rows, `docs/ears/raw/` and `data.csv` found 22 pairs:
- Girl/Boy 01 (NLS mix) with Richard D. James Album 09 (Girl_Boy Song): 292.2 s against 289.4 s, both -8.6 LUFS. A 30 s cross-correlation peaks at 0.37 (far above chance), so it is the same piece in a slightly different edit. It got its deep pass once, on the Richard D. James Album card (it runs first); the Girl/Boy card refers back to it.
- 51/13 tracks 02, 04, 06, 08, 09, 10, 11 repeat pile 2 (Donkey Rhubarb, Ventolin and their pick lists); the 51/13 card covers only 01 On, 03 Crowsmangegus edit, 05 Donkey Rhubarb, 07 73-Yips (and 12 Respect List at mix level).
- Drukqs: 14 tracks match the "Drukqs (piano)" rows of `data.csv` (Ears 1). That is the same record, not twins; Ears 1 measured only those, so this is the first full card.
- **Correction after the 51/13 run.** I first wrote that 51/13 On (413.0 s) and 73-Yips (256.1 s) are not twins of pile 1's *On* (7:13 and 4:19) because the lengths differ by 20 s and 3 s. The measurements say they are the same pieces in shorter edits: On has the same pedal bass on A# (82% against 96%), 0.69 notes a second (0.62), a 334 ms drum echo with one repeat (+16.9 dB against +15.1 dB), a 7.3 dB bass dip peaking 239 ms after a kick every 0.26 s (7.7 dB, 239 ms); 73-Yips has the same 86 BPM triplet-swung meter and a bass dip at the kick (8.4 dB against 12.1, peak 20 ms against 25). `twins.py` could not see them because `docs/ears/raw/` holds no pile 1 pick folder (pile 1 is on its cards only). The deep passes were already done by then (about 5.4 minutes of Demucs). Lesson for the plan: compare against the pile 1 card numbers by hand, and match within 20 s when a title repeats. Donkey Rhubarb (51/13 05, 359.9 s) is probably pile 2's Donkey Rhubarb 01 (368 s): same 139.5 against 139.7 BPM and a mono file.

| Album | Tracks (deep pass) | Machine time | Status |
|---|---|---|---|
| Richard D. James Album (1996) | 10 (4) | 882 s | done |
| Girl/Boy EP (1996) | 9 (2, plus the twin on the previous card) | 192 s | done |
| Come to Daddy (EP, 1997) | 8 (8) | 1,856 s | done |
| 51/13 Aphex Singles Collection (2 non-twin tracks new; On, 73-Yips, Donkey Rhubarb repeat) | 12 (4) | 1,100 s | done |
| Windowlicker EP (1999) | 5 (5) | 1,241 s | done |
| 2 Remixes by AFX (2001) | 3 (3) | 772 s | done |
| Drukqs (2001) | 30 (4) | 959 s | done |

---

## Aphex Twin, Richard D. James Album (1996)

Ears 9, 2026-10-24, Sonnet 5.5. 10 tracks, all at mix level (picker), four given the deep pass: 10 Logon Rock Witch (typical), 03 Peek 824545201 and 08 Yellow Calx (least like the rest), plus 09 Girl_Boy Song (twin with Girl/Boy 01, deep pass done here for both). Stems by Demucs htdemucs. Machine time: 882 s (14.7 min) for 14.4 min of deep-pass audio on 2 cores: pick 64 s, Demucs 450 s (124, 87, 96, 143 s; 1.04 times audio length), measuring 368 s. Ears 7's formula (1.1 s per second of deep-pass audio + 12 s per track) says 1,001 s, so this ran 12% under it.

### Stem check (read first)
- **Logon Rock Witch**: power drums 40%, bass 1%, other 59%, vocals 0%. The bass stem is near empty (0.7%), so bass notes, pumping and wobble are void; the low end (96% of the bass stem is under 120 Hz, drums 58% under 120 Hz) lives in drums and other. Vocals 0.1% is bleed. Meter tool 132 BPM agrees with the drum grid's 132.5 BPM (beat 0.453 s).
- **Peek 824545201**: drums 44%, bass 7%, other 49%, vocals 0.3% (bleed, in the 2 to 5 kHz band). Meter tool 85 BPM is half of the grid's 172.4 BPM (beat 0.348 s); I use the grid.
- **Yellow Calx**: drums 55%, bass 6%, other 36%, vocals 3% (bleed, not a voice). Meter tool 83 BPM is half of the grid's 166.7 BPM (beat 0.360 s); I use the grid.
- **Girl_Boy Song**: drums 8%, bass 71%, other 19%, vocals 2% (bleed). The bass stem is 67% in 120 to 500 Hz: it carries the melodic bass-and-lead, not just the sub, and the drums are a minor part. Meter tool 113 BPM is two thirds of the grid's 166.7 BPM; the grid has several near-equal repeats (0.36, 0.708 s), so read the tempo as uncertain.

### The hook
- Four tracks with drum grids at 132, 172, 167 and 167 BPM, a straight-ish 4 or 8 pulse over fast, tuned, bell-like bass and top lines; three of the four have the bass or sub sitting in the "other" or drums stem, so the low end is one fused sound.
- **[JUDGMENT]** One line a listener would say. **Guess:** fast, busy beats under soft, round melodies in minor-ish modes, nearly mono and close.

### Sounds

**S1. Bell-like slow sub-pad (Peek 824545201): other-1, 421 notes, 46%**
- In synth terms: soft attack (148 ms, 33 to 459), falls 18 dB/s, register about 73 Hz (D2; 43 to 174), bell- or metal-like (partials off whole multiples by 60 cents), harmonic share 0.51, brightness at the start 498 Hz, centered (correlation 1.00, mono file), level -2.8 dB.
- Hear it at 2:41.8 (D2, 0.49 s, +54 dB alone), 2:38.6, 2:38.6 (F3, 0.19 s).
- Check: low pitches under 60 Hz are good to a semitone only, and the note at 2:38.6 reads "?" (no pitch).
- Building blocks: **[JUDGMENT]** **Guess:** bell or softBell with a low register; missing is a bell-like low note (an FM bass with off-integer partials).

**S2. Sub-heavy drone texture (Logon Rock Witch): other-1, 418 notes, 46%**
- In synth terms: A1 (56 Hz; 41 to 110), mixed or noisy partials, attack 110 ms, falls 15 dB/s, harmonic share 0.69, brightness 964 Hz, level -5.0 dB, pan +0.10, correlation 0.09 (a wide, uncorrelated stereo field, the widest of the four).
- Hear it at 2:55.6 (F2, 0.88 s, +51 dB alone), 2:41.3 (C1, 1.03 s), 1:42.7 (F1, 1.03 s).
- Check: the bass stem is void (0.7%), so this group IS the low end; under 60 Hz good to a semitone only.
- Building blocks: **[JUDGMENT]** **Guess:** sub or roundBass with slow attack; the wide stereo field on a bass-register voice is the new part.

**S3. Soft bell-bass blend (Yellow Calx): other-1, 332 notes, 32%**
- In synth terms: E2 (81 Hz; 54 to 92), bell- or metal-like (60 cents off), attack 94 ms, falls only 5 dB/s, harmonic share 0.69, brightness 344 Hz, centered (0.99), -4.8 dB.
- Hear it at 1:24.8 (F#2, 0.29 s, +38 dB alone), 1:25.5, 1:25.1.
- Check: weak group (silhouette 0.24, 4 groups). Treat as one blended texture; listen before trusting the split.
- Building blocks: **[JUDGMENT]** **Guess:** a softBell low in register.

**S4. Fast ticking hat/snare layer (Yellow Calx): drums-1, 330 notes, 28%**
- In synth terms: high tick, 16 ms attack, falls 92 dB/s, brightness 4,128 Hz (2,466 to 6,557), centered. 55% of this track is drums and 31% of its energy is above 2 kHz.
- Hear it at 0:51.3 (F4, 0.28 s), 0:51.2, 2:00.4.
- Building blocks: **[JUDGMENT]** **Guess:** hat/tick exists; the rate (a pulse every 0.36 s with a swung sixteenth) is the point.

**S5. Bell melody (Girl_Boy Song): other-1, 445 notes, 23%**
- In synth terms: C#3 (bell-like), attack soft, falls 25 dB/s, centered.
- Hear it at 4:27.5 (C4, 1.22 s, +38 dB alone), 4:30.2 (E4), 4:30.2 (G#2).
- Building blocks: **[JUDGMENT]** **Guess:** bell or softBell; exists.

**S6. Slow thump (Logon Rock Witch): drums-1, 249 notes, 21%**
- In synth terms: low thump, soft attack, falls 32 dB/s.
- Hear it at 0:05.8 (A#1, 0.25 s, +9 dB alone), 0:13.1, 3:14.9.
- **Kick line (kick_check, kick/duck):** the kick is B1 (40%) or A#1 (35%), median 59 Hz (52 to 62), falling 21 dB/s (a slow, long thump), spaced 1.572 s apart (a kick every 3.5 grid beats; kicks are far apart, with the busy drums between). The bass stem is empty (0.7%), so "duck" is void: the number it gave (-9 dB at the kick, lowest -17.7 dB at 1,535 ms) is the other-stem low end, not a bass.
- Building blocks: **[JUDGMENT]** **Guess:** a tuned sub-kick with a 0.5 s tail; the long slow decay is the point.

**S7. Low kick, bass ducks then swells (Peek 824545201): drums-1, 235 notes, 20%**
- In synth terms: low thump, quick attack, falls 73 dB/s, gets darker over the note.
- Hear it at 0:18.8 (G1, 0.23 s, +24 dB alone), 2:56.0, 2:57.6.
- **Kick line:** G1 60% (median 50 Hz, 32 to 54), falls 49 dB/s, spaced 0.885 s (about 2.5 grid beats). The bass sits 10 dB under its peak at the kick, dips further to -15 dB at 675 ms, and peaks 195 ms after the kick: it ducks at the kick and swells back, plays in the gaps (the techniques tool's 2.0 dB "pumping", peak 429 ms, is the same reading, a floor).
- Building blocks: **[JUDGMENT]** **Guess:** a tuned kick on G1; ducking exists.

**S8. Fast melodic bass (Girl_Boy Song): bass-1, 112 notes, 15%**
- In synth terms: D3 (harmonic, string/reed-like), soft attack, falls 52 dB/s, centered.
- Hear it at 0:19.6 (D3, 0.37 s, +22 dB alone), 0:18.9, 0:18.2 (B2).
- **Kick line:** the "kick" here is not a tuned sub: pitch median 116 Hz (30 to 130), notes C3 21%, A#2 18%, B0 18%, B2 16%, falls 240 dB/s, spaced 0.440 s (a short punch). The bass stays within 4 dB of its peak after the kick (lowest -3.8 dB at 0 ms, peak at 435 ms): no ducking, the bass plays through the kicks (drums are 8% of the power).
- Building blocks: **[JUDGMENT]** **Guess:** a plucked bass in the 120 to 500 Hz band; the fast repeat (1.38 notes a second, 75% repeats) is the new part.

### Techniques
- **T1. Floor drops that are only the big gaps.** Logon Rock Witch: 1.88 drops a minute (1:27.2 for 3.8 s, 1:36.2 for 12.8 s, 2:32.8 for 12.8 s, 2:54.0 for 7.8 s); Girl_Boy Song 0.46 a minute (0:24.8 for 4.8 s, 4:26.0 for 8.8 s, the stems-only count shows 23 s); Peek 0.37 a minute (2:39.8 for 5.2 s); Yellow Calx none. **[JUDGMENT]** **Guess:** floorDrop exists; waits for the synthesis.
- **T2. Big filter arcs.** Yellow Calx mix up 3.3 octaves at 0:09.0 to 0:15.5 and again 0:19.0 to 0:26.5; Peek mix down 2.8 octaves 0:38.5 to 0:53.5, up 2.0 at 0:15.5; Logon Rock Witch up 2.0 at 1:48.5 to 1:55.0, down 2.3 at 1:55.0 to 2:00.0; Girl_Boy Song down 2.4 at 0:04.0 to 0:16.5 (40 more in techniques.json). Sweeps of the whole mix of 0.7 octave or more, many per track. **[JUDGMENT]** **Guess:** padSweep covers the chord layer; a whole-mix filter arc at a few seconds long is new.
- **T3. Builds.** Peek 0:00 to 0:28; Yellow Calx 0:00 to 0:25; Girl_Boy Song 0:08 to 0:34 and 2:49 to 3:13. **[JUDGMENT]** **Guess:** waits.
- **T4. Stem entrances.** Logon Rock Witch: drums plays 82% of the time, bass 25% (but the bass stem is void here); Peek drums 89% bass 88%; Girl_Boy Song drums 63% (in at 0:21, 2:41.5, 4:01), bass 82%. Section lengths: median 24 s (Logon), 13 s (Peek), 26 s (Yellow Calx), 23 s (Girl_Boy). **[JUDGMENT]** **Guess:** waits.
- **T5. Echoes I accept.** Peek: drums echo 180 ms (+5.6 dB, 5 later repeats); Girl_Boy Song drums echo 354 ms (+8.7 dB, 3 later repeats). Neither is claimed a delay: the 354 ms is within a beat's multiple of the 0.36 s grid beat, so it is the pulse. Logon and Yellow Calx: none reliable.
- **T6. Tape wobble.** Peek's bass wobbles about 10 cents at 8.0 Hz; the tool reads about 70% of true depth, and the bass is in the stem the tool trusts (7%). Yellow Calx's other stem moves up to 10 cents in 4 s. **[JUDGMENT]** **Guess:** waits (fast shallow vibrato).

### Theory
- **Th1. Meter and swing (with where it sits against the drum grid).** Logon Rock Witch: 132 BPM both ways, straight eighths. Peek: tool beat 85 BPM = 2 grid pulses (172.4 BPM), off-beat at 0.67 of the tool's beat, a triplet feel: the swing sits at the grid's **eighth** (the grid pulse is the eighth of an 85 BPM beat, and the off-beat lands at a third and two thirds rather than a half), the triplet-shuffle lead of batch-5. Yellow Calx: tool beat 83 BPM = 2 grid pulses (166.7), off-beat at 0.75 of the beat: that is half a grid pulse in, so the swing sits at the grid's **sixteenth**, not the eighth. Girl_Boy Song: tool beat 113 BPM against grid 166.7 (ratio 1.47, not a simple one), off-beat 0.67: swing cannot be placed on the grid's eighth or sixteenth; treat as uncertain.
- **Th2. Keys and modes.** Logon Rock Witch E phrygian (margin 0.026, a guess); Peek G mixolydian (0.043, a guess); Yellow Calx B aeolian (0.003, guess, fits B minor pentatonic); Girl_Boy Song G lydian (0.003, guess). Margins under 0.02 are guesses.
- **Th3. Chord rates (names are a guess on dense music).** Logon 43 changes a minute (13 settled, median chord 1.0 s; C, Emin, A5, C#min); Peek 33 (17 settled; Fmaj7, Dmin7, Csus2); Yellow Calx 35 (15 settled; E5, A5, D5, Bmin); Girl_Boy Song 25 (17 settled, median 2.6 s; Dmaj7, G5, C5, Cmaj7). Only Logon (75% on a beat against 66% chance) and Yellow Calx (53% against 42%) show changes landing on beats.
- **Th4. Bass lines.** Girl_Boy Song 1.38 notes a second, 75% repeats, D/G/C/B; Yellow Calx 0.33 a second, B/A/E/F#, 20% fourths and fifths; Peek 0.12 a second, G 66% (a held G). Logon's is void.
- **Th5. Top lines.** Logon 351 notes, 50% leaps, phrases of 11 notes; Peek 373 notes, thirds and fourths; Yellow Calx 239 notes, 7-semitone range, steps and fourths (the narrowest, the most hummable); Girl_Boy Song 447 notes, 28% leaps. Reliable only where the line sits clearly above its accompaniment.
- **Th6. Repetition.** Logon 235 bars of 0.9 s, repeats at 8 and 16 bars (0.54, 0.52), 13% near-duplicate; Girl_Boy Song 271 bars of 1.1 s, repeats at 1 and 11 bars. Bar length is uncertain on Peek, Yellow Calx and Girl_Boy, so those lags are weak.
- **[JUDGMENT]** Which of these to draw. **Guess:** Th4's slow held bass (Peek) and Th1's grid-sixteenth swing (Yellow Calx) as spreads.

### Space
- Logon Rock Witch is wide: mix width -1 dB in every band (correlation 0.06 to 0.2 in the mids), drums -1 dB with 2.8 dB mono loss. Peek is a mono file: -30 dB in every band, correlation 1.00. Yellow Calx is near-mono in the low end (-21 dB, correlation 0.98) and fully mono in the top (-27 to -28 dB). Girl_Boy Song opens with frequency: low -16 dB (0.95), air above 5 kHz +8 dB (correlation -0.73, anti-phase), over time -15/-11/-6 dB.
- **[JUDGMENT]** **Guess:** mostly near-mono with one track (Logon) fully wide and Girl_Boy's air band flipped.

### Vibes
Numbers to start from: Logon Rock Witch 132 BPM, 13 settled chords a minute, 10% of energy above 2 kHz, width -0 dB; Peek 85 BPM, 17 a minute, 21%, -30 dB; Yellow Calx 83 BPM, 15 a minute, 31%, -22 dB; Girl_Boy Song 113 BPM, 17 a minute, 3%, -13 dB.
- **[JUDGMENT]** Lean toward (x1.5 to x3): quirky, nimble, skipping, strange, bouncy, mischievous, bright (Yellow Calx). Lean away (x0.2 to x0.5): still, restful, hushed, meditative, soft. Never 0. **Guess:** the record is busy and playful, not calm.

### Wholesome take
- Take the small, fast, round melodies and the gentle chords under them; the curious, busy playfulness. Leave the bright edge: Yellow Calx has 31% of its energy above 2 kHz, and Logon Rock Witch has a very wide, uncorrelated field that can feel unsettled. **[JUDGMENT]** **Guess:** take the quirky bounce, leave the brittle top.

### For later
- Peek drums-2 (265 notes, high tick, 68 dB/s; hear 0:22.6, 0:19.1) **[JUDGMENT]** **Guess:** waits, a second hat layer.
- Yellow Calx drums-2 (170 notes, mid snap; hear 1:55.6, 2:13.0) **[JUDGMENT]** **Guess:** waits.
- Girl_Boy Song bass-2 (68 notes, B2, slow swell, 4 dB/s; hear 3:13.4, 1:49.8) **[JUDGMENT]** **Guess:** waits, a pad-like swell.
- Girl_Boy Song drums-1 (340 notes, low thump, 233 dB/s; hear 1:12.2, 2:42.4) **[JUDGMENT]** **Guess:** waits, a short punch not a sub.
- Logon Rock Witch drums-2, other-2; Yellow Calx other-2 (E1 harmonic, 5 dB/s; hear 1:24.0): **[JUDGMENT]** **Guess:** wait.
- Cornish Acid (2:14.8, 144 BPM mix-level; repetition 0.28, low) and Corn Mouth (2:30.9, 112 BPM; repetition 0.56, high): not given a deep pass. **[JUDGMENT]** **Guess:** listen first.

### Reliability notes for this card
- Logon Rock Witch: bass numbers (notes, pumping, wobble) void; the "duck" line is the other-stem low end. Peek and Yellow Calx: the tempo is the drum grid's, not the meter tool's. Girl_Boy Song: the tempo is uncertain, drums are only 8% of its power, and the "kick" pitch is not a tuned sub.
- Girl_Boy Song is also Girl/Boy EP 01 (NLS mix, 2.8 s longer): see that card.
- Chord names, key (margins 0.003 on two tracks), top lines and the swing numbers on dense tracks are guesses. Pitch under 60 Hz is good to a semitone only.
- Pumping reads about 70% of true depth; all depths here are floors.

---

## Aphex Twin, Girl/Boy EP (1996)

Ears 9, 2026-10-24, Sonnet 5.5. 9 tracks at mix level; deep pass on 03 Inkey$ (typical) and 05 Beetles (least like the rest). The third pick, 01 Girl/Boy (NLS mix), is the twin of Richard D. James Album 09 (Girl_Boy Song; 292.2 s against 289.4 s, -8.6 LUFS both, cross-correlation 0.37), which got its deep pass on the previous card, so I did not run it again: read that card's Girl_Boy Song lines (drums 8%, bass 71% of the power; grid 167 BPM, tempo uncertain; fast melodic bass; G lydian, margin 0.003). The NLS mix is 2.8 s longer, so it is a slightly different edit, not byte-for-byte the same audio. Stems by Demucs htdemucs. Machine time: 192 s for 175.5 s of deep-pass audio on 2 cores: pick 41 s, Demucs 104 s (56 and 49 s; 0.59 times audio, short tracks), measuring about 47 s. Ears 7's formula says 217 s (1.1 x 175.5 + 12 x 2), so this ran 12% under it.

### Stem check (read first)
- **03 Inkey$**: power drums 32%, bass 17%, other 49%, vocals 2% (bleed). Meter tool 85 BPM is half the drum grid's 172.4 BPM (beat 0.348 s); I use the grid. The grid, the 354 ms echo and the 172.4 BPM are the same numbers as Peek on the previous card.
- **05 Beetles**: drums 0.01%, bass 0.1%, other 99.6%, vocals 0.3%. Drums and bass are empty: there is no beat, so meter, swing, on-beat counts and any kick or duck number are void. It is a pad-and-melody piece.

### The hook
- Two tracks: Inkey$ has a fast hat/tick grid at 172 BPM over a held bass note and bell chords that change about 21 times a minute; Beetles has no drums, only soft harmonic notes in a wide high band and slow chords in G major.
- **[JUDGMENT]** One line a listener would say. **Guess:** a held low note under bright, bell-like chords, then a drumless music-box piece.

### Sounds

**S1. Soft pad-melody (Beetles): other-1, 310 notes, 99%**
- In synth terms: B2 (harmonic, string/reed-like), soft attack, falls 24 dB/s, centered (mix is wide only in the top: width -5 dB at 2 to 5 kHz, +5 dB above 5 kHz with correlation -0.56).
- Hear it at 0:46.9 (A2, 0.27 s, +55 dB alone), 0:05.2, 0:47.1 (G2).
- Check: weak group (silhouette 0.18, one group): one blended texture; the chords are real music, the separate parts are not.
- Building blocks: **[JUDGMENT]** **Guess:** padSweep or a soft chord pad; the anti-phase air band is the unusual part.

**S2. Bell-like bass-and-chords (Inkey$): other-1, 151 notes, 45%**
- In synth terms: C2 (bell- or metal-like), soft attack, falls 26 dB/s, centered.
- Hear it at 0:33.1 (D2, 0.29 s, +18 dB alone), 0:16.2 (C1, 0.24 s, +21 dB alone), 1:16.6 (no pitch, 2.31 s).
- Check: weak group (silhouette 0.22); notes under 60 Hz good to a semitone only.
- Building blocks: **[JUDGMENT]** **Guess:** softBell or roundBass in the low register.

**S3. High tick (Inkey$): drums-1, 294 notes, 22%**
- In synth terms: high tick, quick attack, falls 78 dB/s, centered.
- Hear it at 1:05.7 (B6, 0.38 s, +32 dB alone), 1:06.1, 1:05.4.
- **Kick line:** no tuned kick. The tool found only 5 kick events, 10 s apart (median pitch 118 Hz, C3/C#2), so the drums stem is the ticks and the low end sits in the bass stem. The bass stem is a pedal on C# (75% of the time), so the "duck" result is not meaningful: the kick events are too few (the curve is noise, -19 dB at the kick). The techniques tool's 1.7 dB dip at 279 ms is a floor from the beat spacing.
- Building blocks: **[JUDGMENT]** **Guess:** hat/tick exists.

### Techniques
- **T1. Filter arc on every phrase.** Inkey$ is the busiest sweeper of the pile so far: the mix goes up and down 1.7 to 4.9 octaves roughly every 8 to 12 s (down 4.3 octaves at 0:12.0 to 0:19.5, up 3.5 at 0:19.5 to 0:28.5, down 4.9 at 0:45.0 to 0:53.0; 14 more in techniques.json). **[JUDGMENT]** **Guess:** padSweep covers the idea; the 12 s period is the new part.
- **T2. Floor drop.** Inkey$ 0.80 a minute (1:14.5 for 4.2 s). Beetles too sparse to say. **[JUDGMENT]** **Guess:** floorDrop exists.
- **T3. Echo.** Inkey$ drums 354 ms (+10.8 dB, 3 repeats) equals the 0.348 s grid beat: that is the pulse, not an echo. Beetles drums 494 ms is noise from an empty stem.
- **T4. Entrances.** Inkey$ bass plays 52% of the time (in at 0:16, 0:50, 1:11.5); Beetles bass 23% of the time. Sections median 16 s and 20 s. **[JUDGMENT]** **Guess:** waits.

### Theory
- **Th1. Meter and swing.** Inkey$: tool beat 85 BPM = two grid pulses (172.4); straight eighths, no swing, so there is nothing to place at the grid's eighth or sixteenth. Beetles: no beat; the meter tool's 92 BPM is a guess.
- **Th2. Keys.** Inkey$ C# ionian (margin 0.012, a guess); Beetles G ionian (margin 0.022, a guess just above the 0.02 line).
- **Th3. Chords.** Inkey$ 21 changes a minute (14 settled, median 1.8 s; Cmaj7, C#maj7, F#5), 70% on a beat (chance 43%); Beetles 18 a minute (13 settled, median 3.9 s; Bmaj7, D#maj7, B5, C5), 67% on a beat (chance 46%). Chord names on dense music are a guess.
- **Th4. Bass.** Inkey$ is a pedal: one note held or repeated, C# 75% of the time, 81% repeats, 0.39 notes a second. **[JUDGMENT]** **Guess:** held pedal bass exists.
- **Th5. Repetition.** Beetles is the repeating one: 38% of bars near-duplicate an earlier one, repeat at 8 bars (0.82); Inkey$ 12% (bar length uncertain).

### Space
- Inkey$ is near-mono by part (bass -27 dB, drums -19, other -19) but its mix over time ranges -31 to -7 dB, so some sections open wide. Beetles is mono below 500 Hz (-27 and -24 dB) and opens above: -16 dB in the mids, -5 dB at 2 to 5 kHz, +5 dB above 5 kHz with correlation -0.56 (anti-phase air). **[JUDGMENT]** **Guess:** mono low end with a wide, slightly anti-phase top.

### Vibes
Numbers: Inkey$ 85 BPM (grid 172), 14 settled chords a minute, 21% of energy above 2 kHz, width -20 dB; Beetles 13 settled chords a minute, 1% above 2 kHz, width -20 dB.
- **[JUDGMENT]** Lean toward (x1.5 to x3): twinkling, starry, tender, floating, serene (Beetles); nimble, skipping (Inkey$). Lean away (x0.2 to x0.5): booming, energetic. **Guess:** gentle and twinkly.

### Wholesome take
- Take the music-box chords of Beetles and the tiny hat grid of Inkey$. No brightness or polarity flag from the numbers apart from the anti-phase air band on Beetles, which is a mix choice to leave out; listening decides. **[JUDGMENT]** **Guess:** take the pad, leave the anti-phase top.

### For later
- Inkey$ bass-1 (132 notes, bell-like C2, 34 dB/s; hear 1:12.1, 0:37.3), other-2 (37 notes), drums-2 (42 notes, low thump): **[JUDGMENT]** **Guess:** wait.
- Not given a deep pass (mix level only): 09 growth inst. (2:37.8, repetition 0.79, high), 04 £18 Snarerush mix (width -0.9 dB against an album median of -20.1), 06 Redruth mix (a short tail). **[JUDGMENT]** **Guess:** growth inst. is the one to hear.

### Reliability notes for this card
- Beetles: all beat numbers void (no drums, no bass). Inkey$: 5 kick events only, so kick and duck are not usable. Keys: margins 0.012 and 0.022. The NLS mix is covered by the previous card's Girl_Boy Song, not re-measured.

---

## Aphex Twin, Come to Daddy (EP, 1997)

Ears 9, 2026-10-25, Sonnet 5.5. 8 tracks, every one given the deep pass (EP rule): 01 Pappy Mix (4:23), 02 Flim (2:57), 03 Little Lord Faulteroy Mix (3:50), 04 Bucephalus Bouncing Ball (5:45), 05 Contour Regard (5:11), 06 Funny Little Man (3:59), 07 Mummy Mix (4:25), 08 IZ-US (2:57). No title repeats another record in the pile. Stems by Demucs htdemucs. Machine time: 1,856 s (31 min) for 33.4 min of audio on 2 cores: pick 67 s, Demucs 959 s (0.48 times audio length), measuring 830 s. Ears 7's formula (1.1 x 2,006 s + 12 x 8) says 2,302 s, so this ran 19% under it.

**Voices.** Pappy Mix, Little Lord Faulteroy, Funny Little Man and IZ-US carry a sung or shouted part (vocals stem 5.8%, 15%, 7%, 4%). I do not write down what is said anywhere on this card. Where I mention it, it is a "voice-like part", described only by pitch range and level.

### Stem check (read first)
- **01 Pappy Mix**: drums 56%, bass 15%, other 23%, vocals 6%. The meter tool's 108 BPM is two thirds of the drum grid's 161.3 BPM (beat 0.372 s); use the grid.
- **02 Flim**: drums 54%, bass 21%, other 25%, vocals 0%. Meter tool 74 BPM is half of the grid's 147.8 BPM (beat 0.406 s); use the grid.
- **03 Little Lord Faulteroy**: drums 25%, bass 50%, other 10%, vocals 15% (a real voice-like part). The meter tool and the grid agree at 120 BPM. The bass stem is the biggest part and carries the low line and some lead.
- **04 Bucephalus Bouncing Ball**: drums 72%, bass 13%, other 15%, vocals 0.3% (bleed). The meter tool's 112 BPM is two thirds of the grid's 166.7 BPM (0.360 s); use the grid.
- **05 Contour Regard**: drums 38%, bass 29%, other 31%, vocals 2%. Meter tool 81 BPM is half the grid's 161.3 BPM.
- **06 Funny Little Man**: drums 72%, bass 14%, other 8%, vocals 7%. Meter 133 BPM agrees with the grid's 132.5 BPM (beat 0.453 s).
- **07 Mummy Mix**: drums 25%, bass 68%, other 7%, vocals 0.3%. Meter tool 79 BPM is half the grid's 156.7 BPM. The bass stem holds most of the music.
- **08 IZ-US**: drums 28%, bass 46%, other 22%, vocals 4%. The meter tool's 60 BPM is a quarter of the grid's 235.3 BPM (four pulses of 0.255 s to a tool beat); I use the grid, but listen.
- None of the eight has an empty bass or drums stem, so every bass number is usable. Pitches under 60 Hz are good to a semitone only.

### The hook
- Eight tracks of very busy drums (grids 120 to 235 BPM, drums 25% to 72% of the power) with a bass that is the real lead in three of them (03, 07, 08: 46% to 68%), and fast chord changes (23 to 58 a minute, the most of any record so far) over modes that mostly sit on a flat second or a sharp fourth (phrygian, lydian, locrian).
- **[JUDGMENT]** One line a listener would say. **Guess:** frantic little drum patterns under a warm, oddly tuned bass, with a gentle bell-like layer underneath.

### Sounds

**S1. Noisy sub drone (IZ-US): other-1, 412 notes, 41%**
- In synth terms: D#2, mixed or noisy partials, soft attack, falls 9 dB/s, centered. Hear it at 2:52.1 (D#2, 2.43 s, +54 dB alone), 2:51.9 (G#2, 0.52 s), 2:31.0 (C#2, 0.77 s). Check: a group in the other stem on a track whose bass stem is 46%: part of the low end is parked here. **[JUDGMENT]** **Guess:** roundBass or sub with a noise layer; the noisy partials are the new part.

**S2. Snappy mid snare (Bucephalus Bouncing Ball): drums-1, 297 notes, 32%**
- In synth terms: mid snap, quick attack, falls 74 dB/s, centered. Hear it at 3:33.3 (G1, 0.18 s, +47 dB alone), 4:25.1, 3:33.1. **Kick line:** kick is E1 17% (median 58.5 Hz, 41 to 128), falls 68 dB/s, spaced 0.795 s. The bass stays within 4.5 dB all the way round (-3.0 dB at the kick, peak 190 ms): at most a mild dip, so it plays on and between the kicks (the techniques tool reads a 3.5 dB dip, peak 314 ms). **[JUDGMENT]** **Guess:** snare exists; the fast pattern is the point.

**S3. Bell-bass (Mummy Mix): bass-1, 272 notes, 32%**
- In synth terms: E1, bell- or metal-like (partials off whole multiples), quick attack, falls 76 dB/s. Hear it at 1:02.1 (C#1, 0.67 s, +26 dB alone), 0:53.8 (B0, 4.57 s), 2:36.7 (F1). Pitch under 60 Hz good to a semitone only. **Kick line:** kick median 78 Hz (47 to 115, G#1 14%: not a tuned note), falls 74 dB/s, every 0.470 s. The bass stays within 4.6 dB and climbs to its peak at 400 ms, just before the next kick: it plays between kicks, no duck. **[JUDGMENT]** **Guess:** an FM-style bell bass; missing block is a low note with off-integer partials.

**S4. Slow-swell noisy pad (Contour Regard): other-1, 287 notes, 29%**
- In synth terms: F#2, mixed or noisy partials, slow swell, falls 7 dB/s, centered. Hear it at 1:42.2 (F#2, 0.35 s, +48 dB alone), 1:42.6 (F#3), 1:43.3. Check: weak group (silhouette 0.21, 4 groups): one blended texture. **Kick line:** kick G1 30% / F#1 21% (median 49 Hz, 46 to 89), falls 114 dB/s, every 0.290 s (one grid pulse every 1.6). Bass is 4.6 dB under its peak at the kick and peaks 215 ms later: a small dip with recovery (the techniques tool finds under 1.5 dB, none). **[JUDGMENT]** **Guess:** padSweep-like; the slow swell on a noisy tone is new.

**S5. Deep sub thump (Funny Little Man): drums-1, 252 notes, 27%**
- In synth terms: low thump, quick attack, falls 114 dB/s, gets brighter over the note. Hear it at 3:22.5 (F1, 0.41 s, +33 dB alone), 3:17.1, 3:18.9. **Kick line:** kick F1 31% (median 45 Hz, 41 to 125), falls 122 dB/s, every 0.450 s (the grid beat 0.453 s). Bass is -4.3 dB at the kick and falls to -10 dB by 220 ms: the bass sounds with the kick and decays after, not a duck. The pump tool reads the other way (bass 5.3 dB lower at the kick, peak 215 ms); the two disagree, so call it unclear. **[JUDGMENT]** **Guess:** a tuned sub-kick on F1.

**S6. Short punchy kick (Flim): drums-1, 335 notes, 27%**
- In synth terms: low thump, quick attack, falls 284 dB/s (short, about 0.18 s to fall 50 dB). Hear it at 1:54.8 (D2, 0.13 s, +21 dB alone), 1:18.2, 1:17.9. **Kick line:** kick median 69 Hz, wide (33 to 105), notes D#2 12%, D2 10%, C2 10%; falls 250 dB/s; every 0.405 s (the grid beat). The bass is -4.9 dB at the kick and back within 1 dB by 50 ms: a short duck, quick recovery, bass plays between the kicks. **[JUDGMENT]** **Guess:** a click kick; short.

**S7. Pad-like bass (Mummy Mix): bass-2, 95 notes, 27%**
- In synth terms: F#2, harmonic (string/reed/pad-like), quick attack, falls 20 dB/s. Hear it at 1:02.7 (F2, 2.86 s, +15 dB alone), 0:58.2 (G#2, 4.46 s), 0:50.2 (B1). Long held notes under the faster bell bass. **[JUDGMENT]** **Guess:** held bass style with a long note; exists.

**S8. Bright mid snare with a tail (Pappy Mix): drums-1, 293 notes, 26%**
- In synth terms: mid snap, quick attack, falls 40 dB/s, gets brighter over the note. Hear it at 3:59.6 (B0, 0.25 s, +42 dB alone), 3:59.4, 3:56.7. **Kick line:** kick G#1 45% (median 53 Hz, 50 to 79), falls 65 dB/s, every 0.748 s. The bass is -6.8 dB at the kick, lowest -8.5 dB at 50 ms, back to its peak at 150 ms: it ducks about 7 dB at the kick and recovers in 150 ms, then plays between kicks (the pump tool: 3.6 dB, peak 60 ms). **[JUDGMENT]** **Guess:** a tuned kick plus ducking; both exist.

**Kick lines for the two tracks without a sound above.** 03 Little Lord Faulteroy: kick median 87 Hz (61 to 107; F2 22%), falls 143 dB/s, every 0.375 s; the bass is -0.3 dB at the kick and 4.9 dB lower at 175 ms, so it plays on the kick and falls away between kicks: no duck. 08 IZ-US: kick median 81 Hz (38 to 102; E2 28%), falls 353 dB/s (the shortest here), every 0.502 s; the bass starts on the kick (-6.6 dB, back to 0 by 50 ms), then falls 9 dB by 485 ms: a bass note per kick, no duck.

### Techniques
- **T1. Fast filter arcs on whole mixes.** Counts of sweeps of 0.7 octave or more, past the eight shown per track: Bucephalus 55 more (it is the busiest, e.g. up 3.0 octaves 0:12.0 to 0:17.0, down 2.7 at 1:30.0), Contour Regard 48, Mummy Mix 47 (down 5.3 octaves at 0:44.0 to 0:54.0, the deepest; up 3.7 at 1:16.5), Pappy 33, Funny Little Man 24, IZ-US 26, LLF 20, Flim 12. **[JUDGMENT]** **Guess:** padSweep covers the slow ones; a 5-octave drop in 10 s is new.
- **T2. Floor drops.** Contour Regard 2.15 a minute (0:15.2 for 3.5 s, 1:39.2 for 4.5 s, 3:55.8 for 3.8 s); Flim 1.51 (2:15.2 for 3.5 s, 2:38.2 for 4.2 s); Pappy 0.76 (2:26.5 for 4.0 s); Funny Little Man 0.56 (3:33.8 for 10.0 s, the longest); LLF 0.58; Bucephalus 0.39; IZ-US 0.38 (2:28.5 for 4.5 s); Mummy 0.25. **[JUDGMENT]** **Guess:** floorDrop exists; frequent drops (every 30 s) are the Contour Regard habit.
- **T3. Tape wobble.** Mummy Mix bass wobbles about 23 cents at 2.3 Hz (the tool reads about 70% of the true depth, so nearer 30); Flim's other stem wobbles 11 cents at 1.4 Hz with a 16-cent speed move in 4 s; Contour Regard's other 9 cents at 2.6 Hz; Bucephalus 4 cents at 2.3 Hz. **[JUDGMENT]** **Guess:** a slow bass vibrato (Mummy) is the one worth keeping.
- **T4. Echoes.** Pappy drums 364 ms (+3.4 dB; the grid pulse is 0.372 s, so it is the beat, not an echo); Contour Regard drums 574 ms (+8.2 dB, 1 repeat; 574 is 2 x 0.29 = 0.58 s, again the pulse); Mummy Mix drums 190 ms (+3.8 dB, 3 repeats; half the grid pulse of 0.383 s, so maybe a real eighth-of-a-beat echo, unverified). None claimed.
- **T5. Builds and entrances.** Pappy builds 0:00 to 0:29; vocals-like part plays 60% of Pappy, 61% of LLF, 75% of Funny Little Man, 54% of IZ-US; Bucephalus bass plays only 36%. **[JUDGMENT]** **Guess:** waits.
- **T6. Stereo movement.** Pappy: drums pan range 0.36, bass 0.17 cycling every 37 s; voice-like parts pan 0.27 to 0.40 on Pappy, LLF, Bucephalus and Contour Regard. **[JUDGMENT]** **Guess:** waits.

### Theory
- **Th1. Meter and swing, placed against the drum grid.**
  - Straight: Flim (74 BPM tool = two grid pulses), LLF (120, grid agrees), Funny Little Man (133, grid agrees).
  - Pappy Mix (tool 108, grid 161.3) and Bucephalus (tool 112, grid 166.7): the tool reports a triplet off-beat at 0.67 / 0.66 of its beat. The grid pulse is 0.67 of the tool's beat (0.372 / 0.556 s; 0.360 / 0.536 s), so that "swing" is the grid pulse itself, not a swing of the grid's eighth or sixteenth. There is a 3-against-2 relation between the tool's beat and the drum grid here, nothing more.
  - Contour Regard (tool 81, grid 161.3) and Mummy Mix (tool 79, grid 156.7): off-beat at 0.75 of the tool beat, i.e. halfway between the second grid pulse and the next: a lean at the grid's **sixteenth**, not its eighth.
  - IZ-US (tool 60, grid 235): 0.75 of the tool beat is the grid's third pulse of four, so it sits on the grid: no swing.
  - So of the eight, two are a sixteenth-level lean (05, 07), two are a 3:2 relation (01, 04), none is a swing at the grid's eighth. For the triplet-shuffle lead in batch-5, this EP gives no clean case at the eighth.
- **Th2. Keys and modes.** Contour Regard C# ionian (margin 0.027, reliable above the 0.02 line); Flim D# lydian (0.020, borderline); Mummy Mix F phrygian (0.021, borderline); the rest are guesses: Pappy G locrian (0.003), LLF G mixolydian (0.005), Bucephalus A aeolian (0.002), Funny Little Man G phrygian (0.000), IZ-US D# dorian (0.009).
- **Th3. Chord rates.** Funny Little Man 58 changes a minute (only 10 settled, median chord 0.9 s, 41 distinct chords: the fastest on any record so far; 68% on a beat); Flim 40; LLF 41 (38 distinct); IZ-US 42; Mummy 34; Pappy 29 (54% on a beat, chance 54%: chords not tied to beats); Contour Regard 28; Bucephalus 23. Flim lands only 22% of changes on a beat and IZ-US 15%, so their chords are not on the grid. Chord names (mostly maj7, sus2, 5) are a guess on dense music.
- **Th4. Bass lines.** LLF holds G 67% of the time (0.93 notes a second, 64% repeats); IZ-US holds D# 68%, plus A# 27% (a tonic-and-fifth pedal); Bucephalus holds A 66% at 0.24 notes a second; Contour Regard C# 56% and F# 22%; Mummy Mix and Pappy wander. **[JUDGMENT]** **Guess:** held tonic with an occasional fifth (IZ-US, Bucephalus) as a spread.
- **Th5. Repetition and top lines.** See techniques.json for lines; bar length is unclear on most tracks (best guesses 2 to 12 beats, scores 0.00 to 0.20), so lags are weak evidence.

### Space
- Widest: Little Lord Faulteroy's air band is +9 dB (correlation -0.80, anti-phase) with a mid width of -13 dB and sub -25 dB; Mummy Mix's 2 to 5 kHz band is +7 dB (-0.65); the over-time width for Mummy ranges -10 to -1 dB. Pappy is moderately wide in every band (-13 to -4 dB, correlation 0.98 down to 0.47) with its bass panned +0.39. Contour Regard opens upward (sub -19 dB, air -2). Near-mono: Funny Little Man (-22 to -28 dB, air -18), Bucephalus (-18 to -24 dB), Flim except its mids (-8 dB).
- Pattern: the low end is nearly mono on every track (sub -13 to -25 dB), the top opens up; LLF and Mummy go anti-phase above 2 kHz. **[JUDGMENT]** **Guess:** mono low end, a wide and slightly anti-phase top on two tracks.

### Vibes
Numbers: Funny Little Man 133 BPM, 10 settled chords a minute; Pappy 161 BPM grid, 19 settled; Flim 148 grid, 17; Contour Regard 161 grid, 15; Bucephalus 167 grid, 14; Mummy 157 grid, 13; IZ-US 14; LLF 16; mix width -24 to -9 dB.
- **[JUDGMENT]** Lean toward (x1.5 to x3): zany, quirky, nimble, mischievous, strange, experimental, skipping. Lean away (x0.2 to x0.5): still, restful, hushed, serene, meditative. Never 0. **Guess:** frantic and funny, with a few warm bass moments (IZ-US, Mummy: intimate, velvety).

### Wholesome take
- Take the funny, quick drum patterns and the warm held basses (IZ-US, Bucephalus, Mummy Mix). Leave the anti-phase top (Faulteroy, Mummy), the 5-octave filter falls, and the shouted voice-like parts, which are the harsh edge of the record. **[JUDGMENT]** **Guess:** take the bouncing pulse and the warm bass; leave the shouting and the harsh top.

### For later
- Bucephalus bass and other groups, Flim other-2, Funny Little Man drums-2, IZ-US drums-2, and the voice-like parts on Pappy, LLF, Funny Little Man and IZ-US: **[JUDGMENT]** **Guess:** wait; voice-like parts are for a later vowel or choir block, not words.
- Mummy Mix drum echo at 190 ms (+3.8 dB, 3 repeats) is worth a listen. **[JUDGMENT]** **Guess:** check by ear.

### Reliability notes for this card
- Every key except Contour Regard (0.027) is a guess or borderline. Chord names and top lines on dense tracks are guesses. The meter tool's tempo is wrong or ambiguous on seven of eight tracks (use the grid); IZ-US's 60 against 235 BPM is a factor of four, so check by ear.
- Kick and duck readings come from two tools that disagree on Funny Little Man and Contour Regard; I give both. Pumping reads about 70% of true depth, so those are floors. Pitches under 60 Hz good to a semitone only. Fingerprint percentiles are in `raw/` (per-track JSON).

---

## Aphex Twin, 51/13 Aphex Singles Collection (only tracks not elsewhere in the pile)

Ears 9, 2026-10-25, Sonnet 5.5. 12 tracks at mix level; seven repeat pile 2 (02 Pancake Lizard, 04 Cylob, 06 Deep Gong, 08 Icct Hedral, 09 Marazavose, 10 Asthma Beats, 11 Carharrack: not measured again). Deep pass on 01 On, 03 Ventolin (Crowsmangegus mix edit), 05 Donkey Rhubarb and 07 73-Yips; 12 Respect List (38.9 s) is too short and stays at mix level. **Only 03 Crowsmangegus is new music.** On and 73-Yips repeat pile 1's *On* (see the correction at the top) and Donkey Rhubarb probably repeats pile 2; I keep short lines for them so the pile has a second reading, nothing more. Stems by Demucs htdemucs. Machine time: 1,100 s (18 min) for 18.9 min of audio on 2 cores: pick 107 s, Demucs 536 s (210, 50, 159, 117 s; 0.47 times audio), measuring 457 s. Ears 7's formula says 1,293 s (1.1 x 1,132 s + 12 x 4), so this ran 15% under it.

### Stem check (read first)
- **01 On**: drums 28%, bass 60%, other 10%, vocals 2% (bleed). Meter tool 135 BPM agrees with the drum grid's 136.1 BPM (beat 0.441 s).
- **03 Crowsmangegus mix edit**: drums 73%, bass 3% (thin: bass notes, pumping and wobble are guesses), other 16%, vocals 7% (bleed). Meter tool 84 BPM agrees with the grid's 83.3 BPM (beat 0.360 s).
- **05 Donkey Rhubarb**: drums 61%, bass 31%, other 8%, vocals 0%. Meter tool 140 BPM agrees with the grid's 139.5 BPM (beat 0.430 s).
- **07 73-Yips**: drums 35%, bass 52%, other 12%, vocals 1% (bleed). The meter tool says 86 BPM; the drum grid's "shortest strong repeat" came out at 1.59 s, which is not a beat (the kicks are 1.59 s apart), so I give no grid tempo for this track.

### The hook
- Three of the four are bass-led, nearly mono tracks (On bass 60%, 73-Yips 52%, Donkey Rhubarb mono at -30 dB) with a single pedal note (A# 82%, G# 87%, A#/B) and sparse chords (6 to 11 changes a minute on 73-Yips and Donkey Rhubarb); the new one, Crowsmangegus, is mostly drums (73%) with bell-like notes on a dominant seventh chord (A#7 57%).
- **[JUDGMENT]** One line a listener would say. **Guess:** a long held low note, a dry steady kick, and a few slow chords.

### Sounds

**S1. Swelling pedal bass (On; same as pile 1's On): bass-1, 372 notes, 53%**
- A#1, harmonic (string/reed-like), slow swell, falls 65 dB/s, centered. Hear it at 4:59.4 (A#1, 0.46 s, +14 dB alone), 5:02.9, 5:27.8. **Kick line:** the kick is a short, wide-pitched thump (median 96 Hz, 57 to 116; G#2 17%, A2 14%, A#1 11%), falls 273 dB/s, every 1.000 s in the kick finder (the grid beat is 0.441 s). The bass is 23 dB under its peak at the kick and swings 23 dB across the second, loudest at 410 ms: it plays in pulses and is off at the kick, so it ducks hard (the pump tool's 7.3 dB, peak 239 ms, is the same dip read at 70% depth). **[JUDGMENT]** **Guess:** heldBass with a 260 ms swell and a hard duck.

**S2. Held G# bass (73-Yips; same as pile 1): bass-1, 423 notes, 45%**
- G#1, harmonic, slow swell, falls 252 dB/s. Hear it at 3:49.1 (G#1, 0.5 s, +6 dB alone), 3:42.4, 3:47.6. **Kick line:** kick median 63 Hz (53 to 109; A1 16%), falls 158 dB/s, every 1.59 s; the bass is 15 dB under its peak at the kick, recovers to -6 dB by 100 ms and to its top at 250 ms: a duck of about 9 to 15 dB with a quick recovery, then it drops 29 dB at 555 ms (the bass has gaps). **[JUDGMENT]** **Guess:** held bass with a duck; exists.

**S3. Bell texture (Crowsmangegus): other-1, 195 notes, 39%**
- G#2, bell- or metal-like (partials off whole multiples), soft attack, falls 16 dB/s, centered. Hear it at 1:13.9 (D#2, 0.27 s, +45 dB alone), 1:14.3 (D3), 1:14.2 (D3). Check: weak group (silhouette 0.17, one group): one blended texture. Other wobbles about 22 cents at 3.8 Hz (the tool reads 70% of true depth), a clear slow vibrato. **[JUDGMENT]** **Guess:** softBell with a 3.8 Hz wobble; the wobble is the point.

**S4. Gliding bass (Donkey Rhubarb; probably the pile 2 track): bass-1, 398 notes, 36%**
- A#1, harmonic, soft attack, falls 60 dB/s. Hear it at 5:41.4 (A#1, 0.49 s, +8 dB alone), 5:15.7, 4:48.3 (C#2). It slides between B and A# (0.94 notes a second, 62% repeats). **Kick line:** kick median 72 Hz, wide (30 to 123; B0 16%, E2 14%), falls 136 dB/s, every 0.325 s in the finder (grid beat 0.430 s). The bass is 8 to 10 dB under its peak for the first 60 ms, then climbs to its top by 300 ms: a duck of about 8 dB recovering in about 150 ms, then it plays between kicks (the pump tool: 6.8 dB, peak 195 ms). **[JUDGMENT]** **Guess:** glide bass and ducking exist.

**S5. Soft sub thump (Crowsmangegus): drums-1, 107 notes, 24%**
- Low thump, soft attack, falls 18 dB/s. Hear it at 1:15.2 (A1, 0.23 s, +18 dB alone), 0:55.9, 0:56.1. **Kick line:** only 35 kicks, 2.84 s apart (median 79 Hz, 57 to 112; E2 31%, A#1 20%), falls 70 dB/s. The bass stem is thin (3%), so the "bass" curve (-8 dB at the kick, flat within 5 dB after) is a guess: no duck found. **[JUDGMENT]** **Guess:** a gentle kick, wait.

**S6. High tick (73-Yips): drums-1, 426 notes, 23%** B5, falls 36 dB/s. Hear it at 1:23.3 (0.2 s, +38 dB alone), 1:34.4, 1:26.5. 31% of this track's energy above 2 kHz in pile 1 is the brittle part. **[JUDGMENT]** **Guess:** hat exists; keep it gentle.

**S7. Short click kick (On): drums-1, 335 notes, 19%** Low thump, falls 454 dB/s, brighter over the note (about 0.1 s to fall 50 dB). Hear it at 4:18.2 (G1, 0.12 s, +35 dB alone), 4:57.3, 4:14.6. **[JUDGMENT]** **Guess:** a click kick; short.

**S8. Snap (Donkey Rhubarb): drums-1, 179 notes, 17%** Mid snap, quick attack, falls 104 dB/s, brighter over the note. Hear it at 5:22.9, 5:33.2 (G1, 0.22 s, +40 dB alone), 5:36.6. **[JUDGMENT]** **Guess:** snare exists.

### Techniques
- **T1. Floor drops.** Crowsmangegus 2.65 a minute, short (0:08.8 for 2.5 s, 0:14.8 for 2.2 s, 1:12.0 for 2.8 s, 1:18.2 for 2.8 s); On 0.65 a minute but two very long ones (0:20.5 for 38 s, 5:41.8 for 32 s: an intro and an outro, not drops); 73-Yips and Donkey Rhubarb none. **[JUDGMENT]** **Guess:** floorDrop exists; a 30-s low-end-out intro (On) is the idea to keep.
- **T2. Echoes.** On drums 334 ms (a dotted eighth, +16.9 dB, 1 repeat); Crowsmangegus other 220 ms (+3.0 dB, 1 repeat; 0.61 of the 0.36 s beat, no match) and drums 349 ms (+4.1 dB, 3 repeats; within 3% of the 0.36 s grid beat, so the beat); Donkey Rhubarb other 454 ms (+10.6 dB, 2 repeats; 5% over the 0.43 s beat, about a beat); 73-Yips drums 115 ms (+6.0 dB, 2 repeats; no matching pulse). **[JUDGMENT]** **Guess:** On's dotted-eighth echo is the one to keep (pile 1 already flagged it).
- **T3. Tape wobble.** Crowsmangegus other 22 cents at 3.8 Hz; 73-Yips other 16 cents at 1.6 Hz. **[JUDGMENT]** **Guess:** waits.
- **T4. Builds.** Crowsmangegus 0:26 to 0:51. **[JUDGMENT]** **Guess:** waits.

### Theory
- **Th1. Meter and swing against the grid.** On: tool and grid agree (135 against 136.1 BPM); the off-beat falls at 0.79 of the beat, later than a triplet (0.67) and past the sixteenth point (0.75): a lean at the grid's **sixteenth** level. Crowsmangegus: tool 84 = grid 83.3; off-beat at 0.74 of the beat, also at the grid's sixteenth lean. Donkey Rhubarb: straight eighths, no swing. 73-Yips: the tool says an 86 BPM triplet feel (0.72), but I have no grid to place it on, so the triplet-shuffle question for batch-5 is open here (pile 1 read it as 86 or 173 swung).
- **Th2. Keys.** On A# aeolian (margin 0.043, reliable); Crowsmangegus A# mixolydian (0.048, reliable); Donkey Rhubarb A# dorian (0.003, guess); 73-Yips G# ionian (0.008, guess).
- **Th3. Chords.** On 46 changes a minute (12 settled; D#min7 38%, C#maj7 33%: names are a guess); Crowsmangegus 22 (A#7 57%); Donkey Rhubarb 11 (C#maj7 70%, 84% of changes on a beat against 70% chance); 73-Yips 6 a minute (G#maj7 50%, C#maj7 30%; one chord lasts up to 26 s).
- **Th4. Bass.** On a pedal on A# (82%, 95% repeats); 73-Yips a pedal on G# (87%); Crowsmangegus moves A#, G#, C# at 0.14 notes a second (guess: thin bass stem); Donkey Rhubarb slides B / A#. **[JUDGMENT]** **Guess:** pedal bass is already in; the slide is new.

### Space
- All four are near-mono at the sub (-25 to -50 dB). Donkey Rhubarb and Crowsmangegus are mono files in every band (-28 to -32 dB). On opens in the mids (-8 dB at 0.5 to 2 kHz, other stem -8 dB) and stays mono below 120 Hz (bass -32 dB); 73-Yips is mono at the top (-30 dB) with a mild low-mid width (-21 to -23 dB). **[JUDGMENT]** **Guess:** mono bass and kick, a little width in the mids only.

### Vibes
Numbers: On 136 BPM, 12 settled chords a minute, width -12 dB; Crowsmangegus 83 BPM, 17 settled, -29 dB; Donkey Rhubarb 140 BPM, 8 settled, -30 dB; 73-Yips 86 BPM, 5 settled, -27 dB.
- **[JUDGMENT]** Lean toward (x1.5 to x3): hypnotic, steady, hushed, intimate, minimal, spacious. Lean away (x0.2 to x0.5): energetic, bouncy, zany. **Guess:** slow and dry, a held-note groove.

### Wholesome take
- Take the held bass, the dry kick and the long intro with the low end held out. Leave the brittle ticks of 73-Yips (a harsh top in pile 1) and the 3.8 Hz bell wobble of Crowsmangegus if it sounds seasick. **[JUDGMENT]** **Guess:** take the pedal groove, leave the tick.

### For later
- 73-Yips other-1 (390 notes, G2 harmonic), Donkey Rhubarb other-1 (398 notes, F3 noisy), bass-2 (58 notes, F#1 bell) and drums-2, On other-1 (371 notes, F#2 noisy) and drums-2, Crowsmangegus drums-2: **[JUDGMENT]** **Guess:** wait.
- Not measured again: 08 Icct Hedral (Philip Glass Orchestration), 02 Pancake Lizard (high chord rate), 09 Marazavose (brightness sweep 2.5 octaves): all in pile 2. 12 Respect List (38.9 s, pulse strength 0.07) is under 45 s. **[JUDGMENT]** **Guess:** Respect List is a short listen.

### Reliability notes for this card
- Crowsmangegus: bass stem thin (3%), so bass notes, pumping and wobble are guesses; only 35 kicks. 73-Yips: no usable drum grid. Two keys are guesses (margins 0.003 and 0.008). Pitch under 60 Hz good to a semitone only; pumping reads 70% of true depth.
- Three of the four tracks repeat earlier records (see the correction at the top), so this card adds one new reading (Crowsmangegus) and three cross-checks that match pile 1 and pile 2 to within the tool's error.

---

## Aphex Twin, Windowlicker EP (1999)

Ears 9, 2026-10-25, Sonnet 5.5. 5 tracks, every one given the deep pass (EP rule): 01 Windowlicker (6:07), 02 Equation (5:47), 03 Nannou (4:16), 04 Windowlicker [Original Demo] (2:37), 05 Equation [Early Version, Edit] (1:46). No title repeats another record in the pile (04 and 05 are earlier versions of 01 and 02, measured as their own tracks). Stems by Demucs htdemucs. Machine time: 1,241 s (20.7 min) for 20.6 min of audio on 2 cores: pick 43 s, Demucs 623 s (182, 163, 139, 77, 61 s; 0.51 times audio), measuring 575 s. Ears 7's formula (1.1 x 1,233 s + 12 x 5) says 1,416 s, so this ran 12% under it.

**Voices.** The Windowlicker versions carry a sung or spoken part (vocals stem 19% on 01, 28% on 04, 8% on 02). I leave out what it says; it appears below only as a "voice-like part" with levels.

### Stem check (read first)
- **01 Windowlicker**: drums 15%, bass 48%, other 18%, vocals 19%. Meter tool 123 BPM agrees with the drum grid's 123.0 BPM (beat 0.488 s). The bass stem is the biggest part and holds the melodic bass.
- **02 Equation**: drums 53%, bass 4% (thin: bass notes, pumping and wobble are guesses), other 35%, vocals 8%. Meter tool 83 BPM is half of the grid's 166.7 BPM (0.360 s); use the grid.
- **03 Nannou**: drums 20%, bass 0.0% (empty: bass numbers void), other 80%, vocals 0%. Meter tool 101 BPM is two thirds of the grid's 151.9 BPM (0.395 s); use the grid.
- **04 Windowlicker [Original Demo]**: drums 40%, bass 19%, other 13%, vocals 28%. Meter 123 BPM agrees with the grid's 123.0 BPM.
- **05 Equation [Early Version, Edit]**: drums 64%, bass 26%, other 11%, vocals 0.1% (bleed). Meter tool 105 BPM is two thirds of the grid's 156.7 BPM (0.383 s); use the grid.

### The hook
- A bass-led, medium-tempo groove (123 BPM on both Windowlicker versions) with a voice-like part on top, a dry kick and a very busy chord layer (31 to 50 changes a minute on four of five), against a drumless-bass bell pad (Nannou).
- **[JUDGMENT]** One line a listener would say. **Guess:** a bouncing, wobbly bass under a wordless voice and shimmering bells.

### Sounds

**S1. Noisy bell-pad (Nannou): other-1, 312 notes, 43%**
- A3, mixed or noisy partials, quick attack, falls 26 dB/s, centered. Hear it at 2:11.6 (E4, 0.56 s, +59 dB alone), 2:07.8 (A3, 0.43 s), 2:16.9 (A3, 0.37 s). Check: weak group (silhouette 0.23); one blended texture. The pad sits in the other stem on the whole track (80% of the power). **[JUDGMENT]** **Guess:** softBell or pad; the wide, noisy tone.

**S2. Snappy snare with a right lean (Equation Early Version): drums-1, 381 notes, 38%**
- Mid snap, quick attack, falls 134 dB/s, leans right (pan +0.41). Hear it at 1:21.6 (F1, 0.24 s, +41 dB alone), 0:55.8, 0:07.0 (B1). The mix of this track is the widest here (width -4 dB over time, 3 dB at 2 to 5 kHz with correlation -0.46). **[JUDGMENT]** **Guess:** snare exists; the panned placement is the new part.

**S3. Metal bell (Equation): other-1, 338 notes, 36%**
- G1, bell- or metal-like (off whole multiples), soft attack, falls 84 dB/s, centered. Hear it at 0:47.3 (F#2, 0.19 s, +50 dB alone), 3:18.5 (G2, 0.58 s), 3:19.1. **[JUDGMENT]** **Guess:** bell in a low register.

**S4. High bell (Nannou): other-2, 188 notes, 31%**
- E5, bell- or metal-like, soft attack, falls 20 dB/s. Hear it at 2:21.1 (F5, 0.35 s, +58 dB alone), 2:08.4, 2:10.9. Weak group (0.23). **[JUDGMENT]** **Guess:** bell exists.

**S5. Dry kick (Equation): drums-1, 322 notes, 26%**
- Low thump, quick attack, falls 61 dB/s. Hear it at 1:50.8 (A1, 0.25 s, +43 dB alone), 1:17.1, 1:17.3. **Kick line:** median 75 Hz, wide (42 to 123; D#2 8%, A2 7%), falls 141 dB/s, every 0.545 s. The bass stem is thin (4%), so the curve (-3.3 dB at the kick, flat within 6.6 dB) is a guess: no duck. **[JUDGMENT]** **Guess:** kick exists.

**S6. Click-kick (Equation Early Version): drums-2, 119 notes, 25%**
- Low thump, quick attack, falls 85 dB/s, brighter over the note. Hear it at 0:55.7 (E2, 0.11 s, +44 dB alone), 0:31.4, 0:56.3. **Kick line:** median 71 Hz (57 to 105; B1 16%, E2 14%), falls 85 dB/s, every 0.565 s. The bass is 8 dB under its peak at the kick, 18 dB under at 90 ms, and climbs back to its top by 500 ms: a deep, slow duck of about 18 dB over half a second, then it plays through the rest of the gap. This is the clearest sidechain-style pump on the record (the pump tool reads only 4.7 dB, peak 85 ms, at 70% depth: the two disagree on size, agree it ducks). **[JUDGMENT]** **Guess:** ducking exists; the 500 ms recovery is the long version.

**S7. Wobbly bell-bass (Windowlicker): bass-1, 346 notes, 24%**
- G1, bell- or metal-like, slow swell, falls 72 dB/s. Hear it at 3:01.7 (F#1, 0.19 s, +21 dB alone), 5:52.7 (B0, 0.41 s), 5:53.4 (D#2). Weak group (0.20). Other stem wobbles about 16 cents at 4.5 Hz (a clear fast vibrato). **Kick line:** kick median 54.5 Hz (30 to 129; B0 28%, which sits on the tool's 30 Hz floor, so under 40 Hz is a guess), falls 162 dB/s, every 0.490 s (the grid beat). The bass (48% of the power) stays within 5 dB all round: -4.3 dB at the kick, back to its top by 275 ms: no real duck, it plays on and between the kicks. **[JUDGMENT]** **Guess:** an FM-style bell bass with a 4.5 Hz vibrato; missing block is a swelling bell-bass.

**S8. Low thump (Windowlicker Original Demo): drums-1, 199 notes, 23%**
- Low thump, falls 102 dB/s, brighter over the note. Hear it at 0:45.9, 1:24.9, 1:48.3 (pitch not read, +12 dB alone). **Kick line:** kicks every 0.490 s (the same as the final version); 65% read B0 (the 30 Hz floor, not a trusted pitch), falls 231 dB/s. The bass is 8 dB under its peak at the kick, back to the top by 50 ms and stays up to 250 ms, then falls to -16 dB by 465 ms: a short duck, then the bass swells off the kick and fades before the next. **[JUDGMENT]** **Guess:** same bass as the final; the shorter duck.

**Nannou kick line.** The bass stem is empty, so the duck is void (the curve's -19 dB is the other stem). Kicks median 68 Hz (30 to 118), falls 261 dB/s, every 0.395 s (the grid beat).

### Techniques
- **T1. Floor drops.** Equation 2.12 a minute (0:45.0 for 3.8 s, 3:13.5 for 6.0 s, 4:32.0 for 7.2 s, 4:40.0 for 23.2 s: the last is an outro); Windowlicker 0.91 a minute (2:34.0 for 3.8 s, 4:50.0 for 3.0 s); the Demo and the Early Version none; Nannou too sparse to say. **[JUDGMENT]** **Guess:** floorDrop exists; Equation's habit of dropping the floor for a few seconds then a long outro is the idea.
- **T2. Echo that equals the beat.** Windowlicker and the Demo have a 489 ms drum echo (+18.3 and +24.0 dB, 2 repeats): that is the 0.488 s grid beat, so it is the beat, not a separate delay. Equation 359 ms (+11.8 dB, 3 repeats) is the 0.36 s grid beat. Nannou 394 ms (+16.9 dB, 2 repeats) is the 0.395 s grid beat. None claimed as a delay.
- **T3. Tape wobble.** Windowlicker other 16 cents at 4.5 Hz; Equation bass 15 cents at 0.5 Hz and other 6 cents at 0.3 Hz (slow); Nannou other 6 cents at 4.3 Hz. The tool reads 70% of true depth. **[JUDGMENT]** **Guess:** the slow bass drift (0.5 Hz) is a pitch-drift idea; waits.
- **T4. Builds.** Equation 1:02 to 1:22 and 1:45 to 2:09; Windowlicker 2:35 to 2:55. **[JUDGMENT]** **Guess:** waits.

### Theory
- **Th1. Meter and swing against the grid.** Windowlicker and the Demo: tool and grid agree at 123 BPM; the final version's off-beat lands at 0.55 of the beat, a 10% lean (near straight) at the grid's **eighth**; the Demo has no clear off-beat. Equation: straight eighths. Nannou (tool 101, grid 151.9) and the Early Version (tool 105, grid 156.7): the tool reports a triplet (0.66 / 0.67) but the grid pulse is exactly 2/3 of the tool's beat (0.395 / 0.594 s, 0.383 / 0.571 s), so that "swing" is the grid pulse itself, not a lean on the eighth or the sixteenth. So this EP has no clear triplet shuffle: the batch-5 lead gets nothing here.
- **Th2. Keys.** Nannou F lydian (margin 0.037, reliable); the rest are guesses: Windowlicker C aeolian (0.011), Demo D# ionian (0.009), Early Version G locrian (0.008), Equation G locrian (0.002).
- **Th3. Chord rates.** Demo 50 changes a minute (only 10 distinct chords, 83% on a beat against 61% chance: clean and rhythmic); Nannou 40; Windowlicker 39 (33 distinct; 64% on a beat); Equation 31 (43 distinct, 45% on a beat: not tied to beats); Early Version 20 (C#maj7 52%). Chord names (maj7, min7, sus2) are a guess on dense music.
- **Th4. Bass.** Windowlicker wanders C, D#, G#, B at 0.69 notes a second; Demo thirds and fifths on G#, D#, F; Early Version slides F# / G at 0.08 notes a second (the slowest pile 3 bass); Equation bass is a guess (thin stem). **[JUDGMENT]** **Guess:** a slide bass with a deep duck (Early Version) is worth a note.

### Space
- Equation Early Version is the widest: width -4 dB over time (-2 to -5), the 2 to 5 kHz band +3 dB (correlation -0.46), drums panned +0.26, sub -15 dB. Nannou's sub is wide (-1 dB, correlation 0.13) while 120 Hz and up is near-mono (-17 to -16 dB): a wide low end is unusual and the stem split is empty there, so treat as the separator's work. Equation is near-mono (-19 to -33 dB). Windowlicker is moderately wide in every band (-18 to -10 dB); the Demo mono in the bass (-45 dB) and opens above 5 kHz (-7 dB). **[JUDGMENT]** **Guess:** mono bass, wide top, one track (Early Version) wide throughout.

### Vibes
Numbers: Windowlicker 123 BPM, 14 settled chords a minute, width -18 dB; Equation 166 grid, 16 settled, -29 dB; Nannou 152 grid, 13 settled, -11 dB; Demo 123 BPM, 15 settled; Early Version 157 grid, 13 settled, -4 dB.
- **[JUDGMENT]** Lean toward (x1.5 to x3): bouncy, quirky, twinkling, chiming, strange, mischievous. Lean away (x0.2 to x0.5): still, hushed, meditative, solemn-like vibes such as reverent. Never 0. **Guess:** playful and shimmery.

### Wholesome take
- Take the wobbling bell-bass, the clean fast chords of the Demo and the chiming pad of Nannou. Leave the deep 18 dB duck of the Early Version unless it sounds good (it can feel seasick) and the voice-like part (not wordless enough to use). **[JUDGMENT]** **Guess:** take the bounce and the bells; leave the voice-like part.

### For later
- Windowlicker bass-2 and the Demo's other groups; Equation drums-2; Nannou drums: **[JUDGMENT]** **Guess:** wait.
- The voice-like parts on Windowlicker (19%) and the Demo (28%): a later vowel block, never words. **[JUDGMENT]** **Guess:** wait.

### Reliability notes for this card
- Nannou: bass void. Equation: bass thin, bass numbers guesses. Kick pitches read B0 (the 30 Hz floor) on Windowlicker and the Demo, so those low notes are not trusted. Keys: only Nannou's is above 0.02. Two tracks' meter tools disagree with the grid by 3:2; I used the grid. Pumping depths read 70% of true; the kick_check duck curve and the pump tool differ on depth for the Early Version and the Demo, so call both.

---

## AFX, 2 Remixes by AFX (2001)

Ears 9, 2026-10-25, Sonnet 5.5. 3 tracks, every one given the deep pass (EP rule): 01 Flow Coma (AFX Remix) (4:59), 02 Box Energy (AFX Remix) (6:30), 03 Bonus High Frequency Sounds (1:58). No title repeats another record in the pile. Stems by Demucs htdemucs. Machine time: 772 s (12.9 min) for 13.5 min of audio on 2 cores: pick 30 s, Demucs 424 s (172, 196, 56 s; 0.53 times audio), measuring 318 s. Ears 7's formula (1.1 x 808 s + 12 x 3) says 925 s, so this ran 17% under it.

### Stem check (read first)
- **01 Flow Coma**: drums 71%, bass 14%, other 10%, vocals 4% (bleed). Meter tool 120 BPM against the drum grid's 161.3 BPM (beat 0.372 s): the grid's pulse is 0.744 of the tool's beat, so the two are in a 3:2 relation (use the grid; check by ear).
- **02 Box Energy**: drums 53%, bass 2% (thin: bass notes, pumping and wobble are guesses), other 38%, vocals 7% (bleed). Meter tool 85 BPM agrees with the grid's 84.7 BPM (beat 0.708 s).
- **03 Bonus High Frequency Sounds**: drums 0.0%, bass 0.0%, other 93%, vocals 7%. Drums and bass are empty: no beat, so meter, swing, on-beat counts, kick and duck numbers are void. It is a drone.

### The hook
- Two heavy, nearly mono drum tracks (drums 71% and 53%, width -32 and -29 dB) and one drone of high bell tones; the chord layer is slow (22 and 26 changes a minute, long stretches on one chord: Flow Coma G#maj7 48%, Bonus Dmaj7 92%).
- **[JUDGMENT]** One line a listener would say. **Guess:** busy mono drums over a slow chord, then a long, shimmering high drone.

### Sounds

**S1. High held tone (Bonus High Frequency Sounds): other-1, 177 notes, 59%**
- D6 (harmonic, string/reed-like), slow swell, falls 0 dB/s (it does not decay; it is held), centered, mono (width -38 dB). Hear it at 0:47.4 (D6, 0.51 s, +47 dB alone), 0:09.7 (D6, 0.83 s), 0:46.4. Pitch is D major pentatonic (D 0.37, C# 0.26); the track is Dmaj7 92% of the time. **[JUDGMENT]** **Guess:** a high sine-like held tone, a pad voice an octave and a half up; exists as pad.

**S2. Bell texture (Box Energy): other-1, 484 notes, 42%**
- D2, bell- or metal-like (off whole multiples), soft attack, falls 42 dB/s, centered; the other stem wobbles about 17 cents at 8.0 Hz (the tool reads 70% of true depth: a fast, clear vibrato). Hear it at 4:27.8 (no pitch, 0.45 s, +34 dB alone), 4:32.8, 4:27.2. **[JUDGMENT]** **Guess:** softBell with a fast 8 Hz wobble; the wobble is the new part.

**S3. Kick-and-tom layer (Flow Coma): drums-1, 374 notes, 37%**
- Low thump, soft attack, falls 69 dB/s. Hear it at 0:30.1 (B1, 0.12 s, +48 dB alone), 0:29.3 (F#1), 2:47.9 (A1, 0.56 s). **Kick line:** kick A1 24% (median 60 Hz, 50 to 98), falls 97 dB/s, every 0.582 s. The bass is 4.5 dB under its peak at the kick, 10.6 dB under at 95 ms, and back near its top by 265 ms: a duck of about 6 to 10 dB with a 250 ms recovery, then it plays between the kicks (the pump tool: 4.1 dB, peak 95 ms; both agree). **[JUDGMENT]** **Guess:** tuned kick on A1 with ducking; both exist.

**S4. Shimmer (Bonus): other-2, 86 notes, 24%**
- B6, mixed or noisy partials, slow swell, held (0 dB/s). Hear it at 1:10.7 (A#6, 0.48 s, +47 dB alone), 0:12.9, 0:12.3. **[JUDGMENT]** **Guess:** a high airy layer; waits.

**S5. Bell-bass (Flow Coma): bass-1, 476 notes, 22%**
- G#1, bell- or metal-like, quick attack, falls 79 dB/s. Hear it at 2:51.7 (G#1, 0.28 s, +18 dB alone), 2:52.0, 2:52.4. Bass numbers: bass plays E 45% and D# 29% (0.17 notes a second, 60% repeats). Pitch under 60 Hz good to a semitone only. **[JUDGMENT]** **Guess:** same bell-bass as the Windowlicker card; waits.

**S6. Wide-tail snare (Box Energy): drums-1, 186 notes, 20%**
- Mid snap, soft attack, falls 26 dB/s. Hear it at 5:14.1 (0.38 s, +34 dB alone), 5:11.3, 5:12.0. **Kick line:** the bass stem is thin (2%) so the duck is a guess. Kick median 87 Hz, wide (30 to 130; B0 20% is the tool's 30 Hz floor), falls 89 dB/s, every 0.535 s; the bass sits within 5.5 dB all round (-5.1 dB at the kick): no duck found. **[JUDGMENT]** **Guess:** snare exists.

### Techniques
- **T1. Floor drops.** Box Energy 0.68 a minute (3:27.0 for 10.8 s, 4:30.2 for 4.0 s, 4:35.8 for 4.5 s); Flow Coma and Bonus none. **[JUDGMENT]** **Guess:** floorDrop exists.
- **T2. Echo.** Flow Coma drums 718 ms (+11.9 dB, 1 repeat; about 1.9 grid pulses of 0.372 s, so about two pulses: the beat); Bonus drums 444 ms (void: the drum stem is empty). **[JUDGMENT]** **Guess:** none to claim.
- **T3. Wobble and speed.** Box Energy other 17 cents at 8.0 Hz; Bonus bass 13 cents at 0.4 Hz with a 7-cent speed move in 4 s (bass stem empty here, so the number is read off the drone's low partials, a guess). **[JUDGMENT]** **Guess:** the 8 Hz wobble is a vibrato rate; waits.
- **T4. Builds.** Box Energy 2:29 to 2:53. Sections: Flow Coma median 22 s, Box Energy 20 s (16 boundaries). **[JUDGMENT]** **Guess:** waits.

### Theory
- **Th1. Meter and swing against the grid.** Flow Coma: tool says swung, off-beat at 0.78 of its beat; the grid pulse is 0.744 of the tool's beat (0.372 / 0.500 s), so that is the grid pulse again, not a lean on the eighth or sixteenth. Box Energy: straight eighths, tool and grid agree. Bonus: no beat. So no swing at the grid's eighth or sixteenth on this record.
- **Th2. Keys.** All guesses: Flow Coma D# ionian (margin 0.016), Box Energy D phrygian (0.006), Bonus D ionian (0.014, fits D major pentatonic; the drone is the clean reading).
- **Th3. Chords.** Flow Coma 22 changes a minute (27 distinct, G#maj7 48%, 56% on a beat against 60% chance: not tied to beats); Box Energy 26 (40 distinct, 43% on a beat against 42%); Bonus 2 a minute (Dmaj7 92%; one chord lasts 66 s at the 90th percentile). Chord names are a guess.
- **Th4. Bass.** Flow Coma and Box Energy move slowly (0.17 and 0.13 notes a second; Box Energy repeats 81%, D 39%, G 23%). Bonus has none.

### Space
- Flow Coma is a mono file in every band (-31 to -34 dB; drums -34, bass -38). Bonus is mono (-38 dB mids and presence, -20 dB above 5 kHz). Box Energy is near-mono (-16 to -23 dB) but its width over time goes from -32 to -3 dB (10th to 90th percentile), so some sections open wide. **[JUDGMENT]** **Guess:** mono groove, wide moments only in Box Energy.

### Vibes
Numbers: Flow Coma 161 BPM grid, 11 settled chords a minute, width -32 dB; Box Energy 85 BPM, 14 settled, -29 dB; Bonus 2 settled a minute, width -38 dB.
- **[JUDGMENT]** Lean toward (x1.5 to x3): hypnotic, floating, starry, twinkling, still (Bonus), quirky (Box Energy). Lean away (x0.2 to x0.5): bouncy, energetic. **Guess:** hypnotic and a little odd.

### Wholesome take
- Take the held high drone of Bonus (very gentle, mono, one chord) and the steady dry groove of Flow Coma. Leave the fast 8 Hz wobble of Box Energy if it sounds nervous. **[JUDGMENT]** **Guess:** take the drone and the groove; leave the nervous wobble.

### For later
- Flow Coma other-1 (418 notes, A2 noisy), drums-2; Box Energy bass-1 (425 notes, D2, 222 dB/s), drums-2, and the vocals-labelled groups (228 and 166 notes at C3 and A#4, bell-like): these are bleed, not voices. Bonus's vocals group (218 notes at F#5, 415 dB/s) is a bleed from high clicks. **[JUDGMENT]** **Guess:** all wait.

### Reliability notes for this card
- Bonus: no beat, every kick, duck and swing number is void. Box Energy: thin bass stem, so bass notes, pumping and the duck are guesses. Keys are all guesses (margins 0.006 to 0.016). Flow Coma's tempo comes from the grid (the tool is in a 3:2 relation). Pumping depths read about 70% of true depth.

---

## Aphex Twin, Drukqs (2001)

Ears 9, 2026-10-25, Sonnet 5.5. 30 tracks at mix level (the picker's row for every track), four given the deep pass: 01 Jynweythek (typical, 2:24), 27 Afx237 v.7 (4:23), 12 Orban Eq Trx 4 (1:35) and 16 54 Cymru Beats (6:07) (the three least like the rest). Ears 1 measured only 14 of the piano pieces at mix level (the "Drukqs (piano)" rows of `data.csv`: 01, 03, 05, 06, 09, 14, 17, 19, 22, 23, 25, 26, 30 and the piano half); these are the same record, not twins, so this is its first full card. Jynweythek was in Ears 1's rows at mix level; the deep pass is new. Stems by Demucs htdemucs. Machine time: 959 s (16 min) for 14.5 min of deep-pass audio on 2 cores: pick 201 s (30 tracks), Demucs 440 s (80, 126, 47, 187 s; 0.51 times audio), measuring 318 s. The download of the 30 tracks through Drive is not counted. Ears 7's formula (1.1 x 869 s + 12 x 4) says 1,004 s, so this ran 4% under it.

### Stem check (read first)
- **01 Jynweythek**: drums 0.2%, bass 0.0%, other 99.8%, vocals 0.0%. Drums and bass are empty: it is a solo piano, no beat. The meter tool's 120 BPM and "straight eighths" are void; so are on-beat counts, kick and duck numbers and floor drops.
- **27 Afx237 v.7**: drums 53%, bass 10%, other 36%, vocals 1% (bleed). Meter tool 112 BPM is two thirds of the drum grid's 166.7 BPM (beat 0.360 s); use the grid.
- **12 Orban Eq Trx 4**: drums 38%, bass 61%, other 1.6%, vocals 0.0%. The other stem is under 2% of the power: top line, chords, other-stem pumping and echoes are void or guesses. The meter tool's 90 BPM agrees with the grid's 90.6 BPM (0.662 s).
- **16 54 Cymru Beats**: drums 51%, bass 17%, other 28%, vocals 4% (bleed). Meter tool 112 BPM against the grid's 166.7 BPM: the same two-thirds relation.

### The hook
- A solo piano that jumps 40 chord changes a minute with a very wide stereo image (Jynweythek), against three dry, near-mono drum tracks (grids at 91 and 167 BPM) with a held bass note.
- **[JUDGMENT]** One line a listener would say. **Guess:** a wide, quick piano and then crisp, dry, mono beats.

### Sounds

**S1. Prepared-piano-like notes (Jynweythek): other-1, 227 notes, 64%**
- E2, mixed or noisy partials, soft attack, falls 12 dB/s, centered by the pan number but wide by correlation (the mix is -5 dB over time, correlation 0.03 at 2 kHz and up). Hear it at 1:29.0 (G#2, 1.09 s, +55 dB alone), 1:17.1 (B1, 1.05 s), 1:34.1 (F#2, 0.51 s). Weak group (silhouette 0.20, 2 groups): one blended piano texture. **[JUDGMENT]** **Guess:** a plucked or struck string voice with a long decay; roundBass or pluck is nearest.

**S2. Bell-bass drone (Orban Eq Trx 4): bass-1, 169 notes, 50%**
- C#2, bell- or metal-like (off whole multiples), quick attack, falls only 14 dB/s, centered (width -51 dB: a mono bass). Hear it at 1:01.4 (C2, 0.17 s, +23 dB alone), 0:18.8, 1:14.5 (A#1). Pitch under 60 Hz good to a semitone only. Bass plays C# 85% of the time (a pedal, 0.82 notes a second). **[JUDGMENT]** **Guess:** a bell-like pedal bass; exists.

**S3. Upper piano (Jynweythek): other-2, 106 notes, 36%**
- F#3, mixed or noisy partials, soft attack, falls 14 dB/s. Hear it at 0:12.0 (C#4, 0.82 s, +58 dB alone), 2:08.0 (D#3, 5.59 s, a long held note), 1:16.1 (B3). Weak group (0.20). **[JUDGMENT]** **Guess:** the same piano voice, an octave up.

**S4. Noisy mid layer (54 Cymru Beats): other-1, 480 notes, 32%**
- G#1, mixed or noisy partials, soft attack, falls 54 dB/s. Hear it at 5:02.4 (no pitch, 0.21 s, +40 dB alone), 4:57.4 (A#1), 4:57.1. **[JUDGMENT]** **Guess:** noise-layer; waits.

**S5. Wide noisy lead (Afx237 v.7): other-1, 280 notes, 30%**
- A#3, mixed or noisy partials, soft attack, falls 43 dB/s. Hear it at 0:59.5 (no pitch, 0.1 s, +32 dB alone), 0:51.5 (A2), 3:47.4 (D#1, 0.89 s). Weak group (silhouette 0.23, 4 groups): one blended texture; the top line has a 31-semitone range (42% leaps), the widest in the pile. **[JUDGMENT]** **Guess:** wait.

**S6. Mid snare (Afx237 v.7): drums-1, 327 notes, 26%**
- Mid snap, quick attack, falls 55 dB/s. Hear it at 2:05.5 (C1, 0.18 s, +32 dB alone), 2:33.6, 2:34.7. **Kick line:** kick median 60 Hz, wide (30 to 127; B0 22% is the 30 Hz floor, so under 40 Hz is a guess), falls 150 dB/s, every 0.620 s. The bass is 4.1 dB under its peak at the kick and 0 dB by 105 ms; it drifts to -8 dB at 420 ms: a mild dip, bass plays between kicks. **[JUDGMENT]** **Guess:** snare exists.

**S7. Click kick (54 Cymru Beats): drums-1, 328 notes, 22%**
- Low thump, soft attack, falls 81 dB/s, brighter over the note. Hear it at 0:22.5 (G#2, 0.1 s, +34 dB alone), 1:03.1, 0:18.8. **Kick line:** median 91 Hz, wide (34 to 129; C3 14%, F2 9%: not a tuned note), falls 76 dB/s, every 0.540 s. The bass is -4.3 dB at the kick, -5.8 dB at 150 ms, back to its top by 450 ms: a small 4 to 6 dB dip, then it plays through; the pump tool finds under 1.5 dB. No ducking to speak of. **[JUDGMENT]** **Guess:** kick exists.

**S8. High tick (Orban Eq Trx 4): drums-1, 159 notes, 21%**
- High tick, quick attack, falls 92 dB/s. Hear it at 0:32.6 (E1, 0.32 s, +43 dB alone), 0:21.9, 1:04.6. **Kick line:** kick C2 44% (median 64 Hz, 36 to 66: the tightest tuned kick in the pile), falls 120 dB/s, every 0.680 s. The curve shows the bass 14 dB under its peak at the kick, at its top 50 ms later and falling smoothly to -34 dB by 500 ms: the bass is hit together with the kick and decays across the gap (a drop-per-kick bass). The pump tool reads the opposite (bass 21.1 dB lower at the kick, peak 499 ms after, kick every 0.62 s): the two tools disagree, so the answer is unclear, and the first needs a listen. **[JUDGMENT]** **Guess:** a tuned kick on C2 with a decaying sub; both exist.

### Techniques
- **T1. Floor drops.** Afx237 v.7 2.03 a minute (1:21.8 for 5.5 s, 3:46.5 for 6.8 s, and a run of drops at 3:55.8, 3:59.0, 4:04.5 and 4:06.8 of 2.2 to 2.8 s each: a stuttering ending); 54 Cymru Beats 0.18 a minute (5:07.2 for 2.2 s); Orban none; Jynweythek void. **[JUDGMENT]** **Guess:** floorDrop exists; the stutter run at the end is the idea.
- **T2. Huge filter arcs.** Orban Eq Trx 4: the other stem climbs 6.0 octaves from 0:08.5 to 0:22.0, the mix falls 4.4 octaves at 1:01.5 to 1:06.5 and 3.5 octaves at 1:24.5 to 1:30.5. Jynweythek's bass stem (the low piano) falls 5.0 octaves at 1:18.0 to 1:25.5 and rises 4.2 at 1:25.5 to 1:33.5 (that is a piano register jump, not a filter: a bass-stem reading on a piano). 54 Cymru Beats has 55 more sweeps (mix down 2.6 octaves at 1:43.5, up 2.5 at 1:53.0) and Afx237 v.7 39 more. **[JUDGMENT]** **Guess:** padSweep covers the slow ones; 6 octaves in 14 s is new.
- **T3. Echoes and wobble.** Afx237 v.7 drum echo 339 ms (+7.7 dB, 3 repeats; the grid beat is 0.36 s, 6% off: about the beat). Orban other-stem echo 693 ms (+4.8 dB, 2 repeats; void: the other stem is 1.6% of the power). Jynweythek other wobbles 7 cents at 1.2 Hz (piano tuning, not tape). Afx237 other 5 cents at 0.8 Hz. Nothing claimed. **[JUDGMENT]** **Guess:** waits.
- **T4. Sections.** Jynweythek median 15 s; Afx237 v.7 20 s; 54 Cymru Beats 20 s (17 boundaries); Orban 13 s (a 58 s opening). **[JUDGMENT]** **Guess:** waits.

### Theory
- **Th1. Meter and swing against the grid.** Afx237 v.7 and 54 Cymru Beats: the tool reports a triplet off-beat (0.67 of its 112 BPM beat) but the grid pulse (0.360 s) is 0.67 of the tool's beat (0.536 s), so that is the grid pulse itself, a 3:2 relation, not a swing on the grid's eighth or sixteenth. Orban: straight eighths, tool and grid agree. Jynweythek: void (no beat). So this record adds no swing case for the triplet-shuffle lead.
  **A pattern across the pile:** a grid pulse of exactly 0.360 s (166.7 BPM) turns up as the drum grid on Yellow Calx, Girl_Boy Song, Bucephalus Bouncing Ball, Equation, Afx237 v.7 and 54 Cymru Beats, and a 0.372 s (161.3) on Pappy and Contour Regard; each time the meter tool reads two thirds of it (111 BPM). Six or more tracks landing on one pulse is a possible artifact of the grid reading, not necessarily music; the synthesis should not take 166.7 BPM as a real habit of the artist without hearing it.
- **Th2. Keys.** Jynweythek D# aeolian (margin 0.036, reliable); Orban C# lydian (0.014, guess; the other stem is void, so this is the bass pedal on C#); Afx237 v.7 A# lydian (0.003) and 54 Cymru Beats D locrian (0.003) are guesses.
- **Th3. Chord rates.** Jynweythek 40 changes a minute (14 settled; 27 distinct; G#min7 23%; 78% on a beat against a chance well below that: the most beat-locked chords on the record); Afx237 v.7 43 (54 distinct); 54 Cymru Beats 31 (40 distinct); Orban 10 (C#maj7 83%; the other stem is void, so these are bass-stem chords). Chord names are a guess on dense music.
- **Th4. Bass.** Orban holds C# 85%; Afx237 v.7 A# 43% (0.32 notes a second); 54 Cymru Beats moves D, F, D# (0.18 a second, steps 47%).
- **Th5. Top lines.** Jynweythek 257 notes, 14 semitones, fourths and fifths 37% (a jumpy line, arch phrases of 12 notes, 67% arches); Afx237 v.7 562 notes, 31 semitones, 42% leaps; 54 Cymru Beats 898 notes (the most), 25 semitones. Reliable only where the line sits clearly above its accompaniment, which for a solo piano it does.

### Space
- Jynweythek is the widest here: sub -18 dB but 0 dB at 2 kHz and above (correlation 0.01 to 0.21), over time -7/-5/-2 dB, mono loss 1.2 dB: a piano with unrelated left and right channels. Afx237 v.7 ranges from -28 to -6 dB over time (its other stem -12 dB, drums -29 dB). Orban is near-mono with a mono bass (-51 dB). 54 Cymru Beats is near-mono (-18 to -26 dB). **[JUDGMENT]** **Guess:** a wide piano, mono beats.

### Vibes
Numbers: Jynweythek 14 settled chords a minute, width -5 dB; Afx237 v.7 11 settled, -19 dB; Orban 1 settled, -20 dB; 54 Cymru Beats 15 settled, -24 dB.
- **[JUDGMENT]** Lean toward (x1.5 to x3): tender, reflective, wistful, intricate (Jynweythek), crisp, nimble, strange (the beats). Lean away (x0.2 to x0.5): booming, comforting. Never 0. **Guess:** the piano is a candidate for tender / wistful; the beats for crisp / strange.

### Wholesome take
- Take the piano pieces: a tender, quick solo piano (Jynweythek) and the dry, mono beats with a held bass. Leave the stuttering floor drops of Afx237 v.7's ending and the 6-octave filter climb of Orban if they sound harsh. **[JUDGMENT]** **Guess:** take Jynweythek's piano; leave the stutter.

### For later
- Not given a deep pass: Gwely Mernans (5:08, repetition 0.80, high: a piano piece Ears 1 saw), Gwarek2 (6:46.9, brightness sweep 5.2 octaves, high), Father (57 s, a very short tail), Lornaderek (31 s), Aussois (13 s), Bit 4 (25 s): under 45 s are too short. **[JUDGMENT]** **Guess:** Gwarek2 is the one to hear.
- Groups left over: Orban drums-2 and bass-2; 54 Cymru Beats drums-2, bass-1 (F1, 439 notes) and a vocals-labelled group (405 notes at C3, bell-like: bleed from percussion, not a voice); Afx237 v.7 drums-2, other-2, bass-1: **[JUDGMENT]** **Guess:** wait.

### Reliability notes for this card
- Jynweythek: no beat, so meter, kick and drop numbers are void; the bass stem is the low end of the piano. Orban: the other stem is void (1.6%) and the two kick tools disagree. Keys: two guesses at 0.003. Kick pitches that read B0 are the 30 Hz floor. The 166.7 BPM grid is flagged as a possible artifact above. Pumping depths read about 70% of true depth.
