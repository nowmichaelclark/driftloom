# Ears cards, pile 1: Aphex Twin 1992 to 94

Ears 5 (Sonnet 5.5, medium), started 2026-10-16. Album cards from `tools/ears/album.py` (stage by stage), checked against the limits in `docs/EARS.md`. Lines marked **[JUDGMENT]** are for Ears 8; each carries a short first guess marked **Guess**. Machine time per album is on each card; the totals and the cost check are in `docs/ears/plan.md`. Five albums, 11,582 s of machine time in all (3.2 hours, 38.6 minutes an album).

| Album | Tracks (deep pass) | Machine time | Status |
|---|---|---|---|
| On (1993) | 4 (4) | 2,576 s | done |
| Selected Ambient Works 85-92 | 13 (4) | 2,388 s | done |
| Selected Ambient Works Volume II | 27 (4) | 2,371 s | done |
| Classics | 11 (3) | 1,693 s | done |
| Polygon Window, Surfing on Sine Waves (Expanded Edition, 2025) | 14 (4) | 2,554 s | done |

---

## Aphex Twin, On (1993)

Ears 5, 2026-10-16, Sonnet 5.5. Four tracks, every one given the deep pass (EP rule): 01 On (7:13), 02 73-Yips (4:19), 03 D-Scape (6:59), 04 Xepha (5:46). Stems by Demucs htdemucs. None of the four repeats on Classics (checked by title). Machine time: 2,576 s (43 min) for 24.3 min of audio on 2 cores: pick 99 s, Demucs 1,562 s (1.07 times audio length), measuring 904 s. Full numbers per track are in the run's JSON; this card keeps the ones that matter and says where each stops being reliable. **Guess** marks my first guesses under the lines left for Ears 8.

### The hook
- Four tracks, four different drum worlds (136 straight with a sub-kick, 135 straight with a pedal bass, 86 or 173 swung, 96 or 192 triplet-swung) that share one habit: a nearly mono core (mix width -14 to -20 dB) with chords drifting over a held bass note, and delays set to fractions of the beat.
- **[JUDGMENT]** One line a listener would say. **Guess:** low, round sub-kick and a held bass under slow major-seventh pads, echoing in time, close to mono.

### Sounds

**S1. Tuned sub-kick (D-Scape, 03), group drums-1: 399 notes, 62% of the track's note energy**
- In synth terms: a near-pure low thump on one pitch per beat. Register 34 Hz (34 to 35), attack 78 ms (77 to 87), decays at 258 dB/s (about 0.2 s to fall 50 dB), harmonic share 0.04 (almost no upper partials), brightness at the start 90 Hz, centered (pan -0.05, correlation 0.98), level against the mix -0.3 dB.
- Hear it at 6:20.8 (D1, 0.28 s, +32 dB alone), 6:35.4 (C#1, 0.49 s, +30 dB alone), 5:03.6 (C#1, 0.44 s, +30 dB alone). It lands every 0.44 s, which is one beat at 136 BPM.
- Check: the bass stem holds 1% of this track's power and the drums stem holds the whole low end (0.156 of 0.166 rms below 120 Hz), so the separator put the sub in the drums stem. Read it as a tuned sub-kick, or a sub note played on the beat; I can't tell which without listening. Pitch below 40 Hz is good to about a semitone at best.
- Building blocks: **[JUDGMENT]** **Guess:** roundBass or sub is the closest; missing is a tuned kick with a 0.3 to 0.5 s tail that can sit on a chosen low note.
- Yardstick: the ranges above, on the drums stem.

**S2. Pedal bass with a slow swell (On, 01), group bass-1: 438 notes, 54% of the track**
- In synth terms: a soft sine- or triangle-like bass around A#1 (58 Hz, 58 to 61), harmonic share 1.00 (0.64 to 1.00), attack 261 ms (77 to 364), decay 467 dB/s, centered (correlation 0.98), level -1.0 dB. The bass stem sits on A# 96% of the time, 0.62 notes a second, repeats 97%.
- Hear it at 4:55.6 (A#1, 0.45 s, +13 dB alone), 4:59.2 (+12 dB), 4:41.3 (+9 dB).
- Building blocks: **[JUDGMENT]** **Guess:** a held-bass voice with a slow attack, repeated about twice a bar; exists in some form (held bass style), the 260 ms swell is the new part.

**S3. Heavy near-mono bass under a bright tick break (73-Yips, 02), group bass-1: 261 notes, 29% of the note energy; bass stem is 58% of the power**
- In synth terms: G#1 / F#1 (52 Hz, 43 to 56), harmonic share 0.98, attack 194 ms (91 to 380), decay 271 dB/s, level -1.8 dB, pan +0.06. The bass stem is G# 83% of the time, A 9%, G 8%; steps 29%, repeats 52%, 1.29 notes a second.
- Hear it at 1:55.6 (F#1, 0.42 s, +9 dB alone), 3:42.8 (+10 dB), 4:13.6 (+6 dB).
- Caution: the tool called these partials bell-like (83 cents off whole multiples). A note at 43 to 56 Hz leaves too few cycles to measure partials, so treat that label as unproven.
- The track is bright on top: 31% of its energy sits above 2 kHz (the highest flag on this EP), carried by high ticks (drums group, 188 notes, falls 17 dB/s, hear 1:26.5, 1:28.1).
- Building blocks: **[JUDGMENT]** **Guess:** roundBass for the low note; the tick is a closed hat with a very short decay, close to what exists.

**S4. Opening pad that blooms (73-Yips, 02), group other-1: 395 notes, 16% of the track**
- In synth terms: mixed partials (harmonic share 0.84, 0.21 to 1.00), soft attack 112 ms, slow fall 31 dB/s, starts at 1,375 Hz brightness (311 to 3,057), level -10.1 dB, centered. The mix's brightness opens 4.9 octaves in the first 7.5 s (0:00 to 0:07.5).
- Hear it at 0:01.0 (A3, 2.36 s, +47 dB alone), 0:09.8 (G2, 0.28 s, +34 dB alone), 0:04.2 (G2, 0.22 s, +24 dB alone).
- Building blocks: **[JUDGMENT]** **Guess:** a pad plus the approved arc (filter opens over the first 8 s); no new block.

(Also heard in the numbers: Xepha has a tuned low tom or kick, drums-1, 251 notes, F#2 at 87 Hz (55 to 92), attack 41 ms, level -0.3 dB, hear 4:40.9 (F#2, 0.11 s, +48 dB alone), 4:57.0, 5:35.4. It sits in For later.)

### Techniques

**T1. Delays set to fractions of the beat (On, D-Scape)**
- On, 135 BPM (beat 444 ms): other stem echoes at 454 ms (one beat, 2% off; +4.6 dB, 2 later repeats); drums echo at 334 ms (a dotted eighth, 333 ms; +15.1 dB, 1 later repeat).
- D-Scape, 136 BPM (beat 441 ms): other echoes at 220 ms (an eighth; +4.9 dB, 5 later repeats); drums echo at 444 ms (a beat; +9.3 dB, 4 later repeats).
- 73-Yips and Xepha: none found reliably (too few notes ringing alone for 1.5 s). The tool's echo times are good to about 4 ms where it finds them.
- **[JUDGMENT]** **Guess:** this is the one to keep: echo time drawn from beat, dotted eighth, eighth.

**T2. The floor falls out and comes back (On, 01)**
- Checked in the stems: the low end (below 120 Hz) is absent from 0:15 to 0:29 (under -86 dB), then a faint bed about 35 dB under the body to about 0:54, then full level (about -16 dB) from 0:54 to 5:42, gone again from 5:42 to 6:13 (31 s, below -85 dB, pads continue), and back to the end. The tool's list reads 0:21.5 for 36.2 s, 0:57.8 for 7.0 s, 1:05.0 for 8.0 s and 5:41.8 for 32.0 s (0.62 a minute); the 0:21.5 one is a quiet bed, not silence. Edges are good to about 0.5 s.
- Stem entries agree: drums in at 0:54.0, bass in at 1:16.0 and 6:15.0, drums out at 5:43.0.
- D-Scape: not measured (low end on under 75% of the time, the tool skips it). 73-Yips and Xepha: 0 a minute.
- **[JUDGMENT]** **Guess:** the approved floorDrop covers the 5:42 drop; the slow entry (pads alone, then a quiet floor, then the beat 54 s in) is a long-intro form worth a card line of its own.

**T3. Whole-mix brightness moves (all four)**
- On: 60 sweeps of 0.7 to 1.6 octaves in 7 minutes (for example up 1.6 oct 1:30.5 to 1:43.0, down 1.0 oct 1:57.0 to 2:02.5). 73-Yips: up to 4.9 octaves (up 4.9 oct 0:00 to 0:07.5, down 3.9 oct 0:49.5 to 0:56.5). D-Scape: up 2.6 oct 0:05.0 to 0:14.5, down 2.5 oct 1:59.5 to 2:08.0, 39 more. Xepha: down 2.4 oct 0:36.0 to 0:43.0, 36 more.
- Caution: this is the whole mix's brightness, so a hat entering counts as a sweep; it is not proof of a filter.
- **[JUDGMENT]** **Guess:** repeated 1 to 2 octave swells every 8 to 16 s are a good fit for the approved arc at a finer scale.

### Theory

**Th1. Chords drift over a held bass note, off the grid**
- Bass behavior: pedal on a single note in 01 (A# 96%), 02 (G# 83%) and 04 (A 90%). D-Scape's reading (root movement, E 63%, D# 33%) comes from a stem with 1% of the power; ignore it.
- Chords above it (names on dense music are a guess): On: D#min7 44%, C#maj7 26%, G#sus2 8%; 73-Yips: G#maj7 46%, C#maj7 35%, Gmaj7 9%; Xepha: Bmaj7 38%, Cmaj7 23%, C#maj7 15%; D-Scape: C7 16%, D#5 15%, E5 15%. The two clearest moves are major sevenths stepping by a semitone (Bmaj7, Cmaj7, C#maj7; G#maj7 to Gmaj7).
- Settled chords (1.5 s or longer): 12 a minute in On, 4 in 73-Yips, 14 in D-Scape, 12 in Xepha. The all-changes counts (34 to 46 a minute in three tracks) count fast note flicker as chords and are not a harmony rate.
- Share of chord changes landing on a beat, against chance: On 71% (chance 67%), 73-Yips 32% (43%), D-Scape 46% (68%), Xepha 47% (48%). Only 73-Yips and D-Scape sit clearly below chance: their harmony moves off the beat.
- Key reads: On A# aeolian (margin 0.051, usable); 73-Yips G# ionian (0.014), D-Scape E lydian (0.013), Xepha B ionian (0.014): the last three are guesses.
- **[JUDGMENT]** **Guess:** draw a held low note, then chords that step by semitone over it, with change times drawn apart from the beat grid.

**Th2. Swing as a dial**
- Off-beat at 0.57 of the beat in 73-Yips (50% is straight, 67% is triplet), 0.65 in Xepha (near triplet); On and D-Scape straight. Tempo octave is not settled: 135 (or 67), 86 (or 173), 136 (or 68), 96 (or 192). Bar length: 8 beats in On and Xepha; unclear in 73-Yips and D-Scape, so their repetition numbers are weak evidence.
- Top lines (read by ear before trusting; only reliable when clear of the accompaniment): D-Scape's looks real (median interval 3 semitones, steps 20%, thirds 28%, arch phrases 36%); 73-Yips' (median 13 semitones, 64% leaps) is not a tune, probably alternating octaves or stems bleeding.
- **[JUDGMENT]** **Guess:** swing from 0.50 to 0.65 as a per-loop draw.

### Space
- All four are close to mono: median width -10 dB in On, -17 in 73-Yips, -18 in D-Scape, -17 in Xepha (two-second windows). Sub is mono (correlation 0.97 to 1.00); the air band above 5 kHz is nearly mono too (0.98 to 1.00).
- What width there is sits in the mids (500 Hz to 2 kHz): On -7 dB (correlation 0.71); its other stem leans left (pan -0.22, width -8 dB, mono loss 0.6 dB). Drums and bass are centered everywhere (pans within 0.16).
- Against the 32 measured albums this is at the narrow end (Mike & Rich -10 to -29 and The Field -10 to -25 are as narrow; Drukqs piano -1 to -8, Amber -4 to -14, Geogaddi -2 to -24).
- Movement: 73-Yips drums pan range 0.12 on a 33 s cycle; D-Scape vocals pan range 0.42; Xepha vocals 0.16 on a 3 s cycle. Small.
- **[JUDGMENT]** **Guess:** a recipe with everything centered and a single mid-band layer parked 0.2 left, no drift.

### Vibes (all guesses; weights lean x1.5 to x3 toward, x0.2 to x0.5 away, never 0)
- S1 sub-kick: toward booming x2.5, hypnotic x2, steady x1.5, cool x1.5; away from airy, twinkling, frosty x0.4.
- S2 pedal bass swell: toward hazy, foggy, misty x1.5 to 2, still, restful x1.5 (held bass style), floating x1.5; away from bouncy, plucky, energetic x0.4.
- S3 bright tick break with heavy bass: toward experimental, strange x2, cheeky x1.5; away from cozy, comfy, tender x0.3 (a brief touch under a cozy loop is fine).
- S4 blooming pad: toward floating, spacious, wondrous x2.
- T1 beat-synced echo: toward spacious, hypnotic x2; no vibe leans away.
- T2 long intro and floor drop: toward hypnotic, minimal, meditative x2.
- New vibe or profile: none suggested yet; Ears 8 may want a dub-pedal profile (held bass, slow major-seventh planing, synced echo) once Classics and SAW II are in.
- **[JUDGMENT]** Ears 8 settles all of this.

### Wholesome take
- **[JUDGMENT]** **Guess:** take the sub-kick, pedal bass, planing major sevenths and synced echoes; take the long intro and the floor drop; keep 73-Yips' ticks at a gentle level (31% of its energy above 2 kHz is the brittle part) rather than leaving them.

### For later
- Xepha, drums-2 (141 notes): mid snap, soft attack, falls 104 dB/s, centered; hear 2:04.3, 0:24.3. **[JUDGMENT]** **Guess:** waits, close to existing snares.
- Xepha, vocals-1 (371 notes; the separator's name for a lead, not sung words as far as the numbers say): B3, falls 18 dB/s, slow swell; hear 4:19.7, 0:16.2. **[JUDGMENT]** **Guess:** waits, listen before taking it, and any voice must be vowels or hums only.
- Xepha, tuned low tom or kick (drums-1, 251 notes, F#2); hear 4:40.9, 4:57.0. **[JUDGMENT]** **Guess:** waits, too close to the sub-kick for a second sound.
- On, drums-2 (181 notes): mid snap, falls 400 dB/s; hear 4:56.8, 4:22.8. **[JUDGMENT]** **Guess:** waits, snare-like.
- On, other-1 (268 notes): F#2, mixed partials, falls 39 dB/s; hear 5:57.9, 5:48.2. **[JUDGMENT]** **Guess:** waits, a plain pad.
- 73-Yips, bass-2 (91 notes): G#1, harmonic, falls 18 dB/s; hear 3:44.4, 0:55.6. **[JUDGMENT]** **Guess:** waits, a second bass layer.
- D-Scape, other (233 notes, weak groups, silhouette 0.21): one blended texture, E2, brightness 476 Hz; its loudest note is D#2 at 1:19.1 (+57 dB alone). **[JUDGMENT]** **Guess:** waits, listen at 1:13.9 to 1:19.1 first.
- Xepha, bass-2 (106 notes): D1, falls 38 dB/s; hear 1:12.8, 4:22.2. **[JUDGMENT]** **Guess:** waits.

### Reliability notes for this card
- Tempo octave open on all four; bar length unclear on 73-Yips and D-Scape (their repetition lags are weak).
- Key margins under 0.02 on three of four (guesses). Chord names are a guess on this dense material; the on-beat shares are the check available on real music.
- D-Scape's bass stem is near-empty (1%), so its bass behavior, bass pumping (7.7 dB) and S1's label need a listen.
- Pumping figures read about 70% of the true depth: 73-Yips bass ducks 12.1 dB at the kick (peak 25 ms after; may be mutual masking or the separator sharing the sub between stems), On bass 7.7 dB (peak 239 ms after with a kick every 0.26 s, not a clean lock), D-Scape other 5.9 dB (peak 170 ms after, a believable slow duck), Xepha bass 3.8 dB.
- Tape wobble: only Xepha's other stem reads 4 cents at 1.5 Hz, right at the tool's limit.
- The stem Demucs calls vocals is its guess at voice-like sound; on these tracks it follows lead lines that enter for 3 to 20 s about every 30 to 40 s. It is not evidence of a voice either way; listen before trusting it.

---

## Aphex Twin, Selected Ambient Works 85-92

Ears 5, 2026-10-16, Sonnet 5.5. 13 tracks, all measured at mix level (the file lengths match the record, including i at 1:17). Deep pass on four, as the picker chose: the most typical, Xtal (4:54), and the three least like the rest, i (1:18, no drums or bass at all), Tha (9:07) and Hedphelym (6:04). The other nine feed For later. Machine time: 2,388 s (40 min) for 21.4 min of deep-pass audio: pick 286 s (13 tracks), Demucs 1,325 s (1.03 times real time), measuring 766 s. **Guess** marks my first guesses under the lines left for Ears 8.

### The hook
- Mono sub and bass under a wide, decorrelated top (above 500 Hz the left and right are nearly independent), with slow analog pitch wobble on the held parts (5 to 14 cents at 0.5 to 2.9 Hz), over beats at 125 to 135 BPM and a 20 to 38 s opening build.
- **[JUDGMENT]** One line a listener would say. **Guess:** warm, slightly out-of-tune analog pads that shimmer wide over a mono sub and a soft kick.

### Sounds

**S1. Wide, wobbling pad layer (Tha, Hedphelym, i), the other stems**
- In synth terms: mixed partials (harmonic share 0.64 to 0.76 median), slow swell (attack 184 ms in Tha, 327 ms in Hedphelym, 464 ms in i), barely falling (3 to 16 dB/s), left and right only half related (correlation 0.31 to 0.46), level -2 to -10 dB against the mix. Pitch wobbles 14 cents at 1.7 Hz in Tha, 5 cents at 1.7 Hz in Hedphelym, 9 cents at 2.9 Hz in Xtal, 7 cents at 0.5 Hz in i.
- Hear it at: Hedphelym 5:54.7 (A#2, 5.71 s, +54 dB alone, the closing held chord) and 5:52.7 (G2, 0.96 s); Tha 8:35.6 (E3, 0.73 s, +13 dB alone); i 0:59.4 (B1, 1.08 s, +54 dB alone), 0:56.2 (D2, 1.98 s, +53 dB alone).
- Caution: Tha's group is weak (silhouette 0.17, one group): one blended texture, not one instrument. i's note energy is 89% one group of 27 notes, a slow drone at 61 Hz (48 to 73) whose left and right are unrelated even in the sub (sub correlation 0.06), so it is a wide, phasey drone.
- Building blocks: **[JUDGMENT]** **Guess:** an analog pad with the Sound panel's wobble raised to 5 to 14 cents and a slow 180 to 460 ms attack; the new part is a per-voice 1.7 Hz wobble and a decorrelated left and right.

**S2. Clean pentatonic bass riff (Tha), bass-1: 320 notes, 34% of the note energy**
- In synth terms: a bass with a quick attack (30 ms, 9 to 132) and almost perfect harmonics (harmonic share 1.00, partials 7 cents off whole multiples, so a saw or square, not a bell), around A1 (55 Hz, 46 to 62), falls 24 dB/s, level -2.7 dB, centered (correlation 0.95).
- Hear it at 8:03.1 (A1, 1.78 s, +18 dB alone), 8:02.7 (B1, 0.43 s, +23 dB alone), 8:12.7 (F#1, 0.19 s, +22 dB alone).
- Behavior: it moves (time on A 38%, F# 22%, B 14%, E 11%; 1.44 notes a second; steps 19%, thirds 11%, octaves 13%, repeats 45%). The notes A, F#, B, E are inside A major pentatonic.
- Building blocks: **[JUDGMENT]** **Guess:** moogbass or a pluck bass with a fast attack; a riff drawn from the pentatonic set. Nothing new.

**S3. Round sub pedal and deep kick (Xtal), bass-1: 438 notes, 58%; drums-1: 360 notes, 15%**
- In synth terms: bass around A1 (55 Hz, 52 to 56), harmonic share 0.99, attack 41 ms, falls 23 dB/s, level -0.7 dB, correlation 0.99; the bass stem carries 74% of the track's power and sits on A 94% of the time (1.34 notes a second, repeats 77%). The kick is a deep thump at 55 Hz (41 to 60), attack 48 ms, falls 64 dB/s, level -6.1 dB.
- Hear it at: bass 3:26.1 (A1, 1.05 s, +14 dB alone), 3:28.2, 3:32.4; kick 1:48.0 (C#1, 0.4 s, +18 dB alone), 1:14.4 (A#1, 0.2 s), 1:15.5 (G1, 0.27 s).
- Caution: the tool called the bass bell-like (73 cents off whole multiples); at 52 to 56 Hz that is unproven (too few cycles to measure partials).
- Building blocks: **[JUDGMENT]** **Guess:** roundBass for the pedal; the kick needs a sub-weighted version of what exists.

**S4. Slow-swelling sub thump (Hedphelym), drums-1: 453 notes, 50% of the note energy**
- In synth terms: a low pulse at 46 Hz (38 to 57), attack 213 ms (186 to 232), falls 103 dB/s, brightness 104 Hz, harmonic share 0.26, level -1.9 dB, correlation 0.98. The separator put Hedphelym's whole low end here (its bass stem holds 0% of the power).
- Hear it at 2:26.9 (0.5 s, +17 dB alone), 2:26.4 (0.51 s, +15 dB alone), 3:23.5 (0.54 s, +14 dB alone); pitch could not be read.
- Building blocks: **[JUDGMENT]** **Guess:** a sub with a 200 ms swell; close to roundBass with its attack opened up.

### Techniques

**T1. Analog pitch wobble and drift (all four)**
- Held-partial wobble in the other stem: Tha 14 cents at 1.7 Hz, Xtal 9 cents at 2.9 Hz (speed also moves up to 6 cents in 4 s), i 7 cents at 0.5 Hz (speed up to 13 cents in 4 s), Hedphelym 5 cents at 1.7 Hz plus a 6-cent wobble at 7.0 Hz on the bass. For comparison, On has none above 4 cents. The tool read a 12-cent wow as 13.7 with the rate exact, so the 9 and 14 cent readings are believable; 4 to 6 cents is near its limit.
- **[JUDGMENT]** **Guess:** a per-voice slow wobble of 5 to 14 cents at 0.5 to 3 Hz is the signature; it sits close to what the Sound panel's wobble already does.

**T2. Pumping at the kick (Tha)**
- The bass dips 7.6 dB at the kick (peak 195 ms after, kick every 0.44 s); the tool reads about 70% of the true depth, so the dip is at least 7.6 dB and probably near 11. Xtal 2.0 dB, Hedphelym 2.8 dB (bass) and 1.6 dB (other) are small. The drums also echo at 444 ms, which is one beat at 135 BPM (+12.1 dB, 2 later repeats).
- **[JUDGMENT]** **Guess:** a 195 ms release duck on the bass, drawn between 2 and 11 dB.

**T3. Floor drops, then a long periodic brightness swing (Hedphelym), plus an opening build in all four**
- Hedphelym floor drops (0.55 a minute): 0:18.0 for 14.0 s, 1:34.0 for 32.2 s, 5:12.0 for 12.2 s; the stems agree (drums out at 1:35.0, back at 2:05.5). Xtal has one at the end (4:29.0 for 10.2 s) and a 3 s one at 0:14.5; Tha and i none.
- From 2:19.5 to 2:50 Hedphelym's mix brightness swings in a regular cycle of about 12 s (down 3.8 oct, up 3.2, down 2.9, up 2.6, down 1.8). Caution: this is the whole mix, not proof of one filter.
- Every track opens with a build of 20 to 38 s (Xtal 0:00 to 0:23, Tha 0:00 to 0:24, i 0:00 to 0:20, Hedphelym 0:00 to 0:38); Xtal swings the mix down 3.2, up 3.2 and down 5.4 octaves inside its first 20 s.
- **[JUDGMENT]** **Guess:** floorDrop at 12 to 32 s is the approved move; the 12 s brightness cycle and the 20 to 38 s opening build both belong with the approved arc.

### Theory

**Th1. Tha: A major pentatonic over a moving bass, 135 BPM**
- The five most used pitch classes are A 0.18, F# 0.13, E 0.10, B 0.09, C# 0.08, which is A major pentatonic. Chords (names are a guess on dense music): A5 25%, F#5 14%, Asus2 13%, Fmaj7 8%; settled chords 15 a minute. Key margin is 0.006, so the key name is a guess, but the pentatonic content does not depend on it.
- Tempo: the meter tool said 67 (or 135); the kick spacing (0.44 s) and the drum echo (444 ms) both give 135, so 135 is the better read.
- Only 23% of Tha's chord changes land on a beat (chance 34%): its harmony moves away from the grid.
- **[JUDGMENT]** **Guess:** a pentatonic pitch set for both bass and pad, with a mid-tempo 135 and chord changes drawn off the beat.

**Th2. Xtal: chords locked to the beat over an A pedal**
- 78% of Xtal's chord changes land on a beat against 57% by chance: the strongest beat-locked harmony in pile 1 so far. Chords above the A pedal (94% of the bass): Cmin7 27%, Dsus2 15%, C#maj7 12%, D#maj7 11%; only 6 a minute last 1.5 s or longer (the all-changes count of 40 a minute is note flicker). The key has no answer (margin 0.000). Hedphelym: C aeolian (margin 0.025, usable), Fsus2 22%, G#maj7 18%; chord changes 61% on the beat (chance 63%). i: A mixolydian (margin 0.056, usable), 16 settled chords a minute.
- Tempo octave open: Xtal reads 114 (or 57, or 229), but the kick spacing the tool found (0.34 s) points at about 176, so 114 is unresolved. Hedphelym 125 (kick every 0.48 s agrees). Swing readings of 0.75 in i and 0.76 in Hedphelym are off-beat hits at the third sixteenth; i has no drums, so its meter and swing are meaningless.
- **[JUDGMENT]** **Guess:** a held low note under chords that move on the beat, as one of two harmonic styles (Tha is the other, off the beat).

### Space
- This album is wide. Median width (two-second windows): Xtal -5 dB, Tha -8, Hedphelym -5, i -3 (On is -10 to -18). The pattern repeats: sub is mono (width -14 to -17 dB, correlation 0.93 to 0.96), the low mids are half-related (-6 to -9), and everything from 500 Hz up is close to independent left and right (width -5 to +3 dB). Checked from the stems: Xtal 2 to 5 kHz correlation -0.10, above 5 kHz -0.15 (folding to mono drops those bands 3.4 and 3.6 dB); Hedphelym above 5 kHz -0.04 (3.2 dB); Tha 0.21 and 0.36 (about 2 dB).
- By part: bass and drums are centered (bass width -23 dB in Xtal, drums -12 to -18 dB; Hedphelym drums pan -0.20, the only off-center drum); the other stem carries the width (-3 to -5 dB, mono loss 1.3 to 1.7 dB).
- Movement: Xtal drums pan range 0.36, vocals 0.25; Tha drums, bass and other all show a 4 s pan cycle of 0.12 to 0.28. The same period in three parts looks like the tool's window, not an auto-pan; unconfirmed. Hedphelym stays put.
- **[JUDGMENT]** **Guess:** a recipe with bass and kick mono below 120 Hz and a decorrelated top (above 500 Hz) at width -5 to 0 dB; the wide-top recipe is the main thing this album adds.

### Vibes (all guesses; weights lean x1.5 to x3 toward, x0.2 to x0.5 away, never 0)
- S1 wide wobbling pad: toward hazy, foggy, misty, floating x2; away from crisp, steady x0.4.
- S2 pentatonic bass riff: toward bouncy, happy, nimble x1.5; away from still, hushed x0.4.
- S3 sub pedal and kick: toward booming, hypnotic x2; away from airy, twinkling x0.4.
- S4 swelling sub: toward booming, floating x1.5.
- T1 wobble: toward hazy, foggy, golden x2 (golden already asks for wobble); away from crisp x0.3.
- T2 pumping: toward energetic, lively x1.5; away from still, restful x0.3.
- T3 floor drops, long build: toward hypnotic, wondrous x2.
- **[JUDGMENT]** Ears 8 settles all of this.

### Wholesome take
- **[JUDGMENT]** **Guess:** take all of it: the numbers raise no brightness flag (1% of the energy above 2 kHz in three of four tracks, 0% in i). The wide top should be used lightly under the cozier vibes.

### For later
- Tha, drums-1 (288 notes): low thump, falls 161 dB/s; hear 5:52.7, 6:35.4. **[JUDGMENT]** **Guess:** waits, close to the Xtal kick.
- Tha, drums-2 (178 notes): low thump falling 16 dB/s, gets darker; hear 6:21.2, 7:35.9. **[JUDGMENT]** **Guess:** waits, a longer boom than anything built.
- Tha, bass-2 (143 notes): A2, harmonic; hear 8:02.5, 8:24.2. **[JUDGMENT]** **Guess:** waits, an octave-up second bass.
- Xtal, other-1 (267 notes): A1, slow swell, falls 7 dB/s; hear 0:13.5, 0:13.8. **[JUDGMENT]** **Guess:** waits, near the opening build.
- Xtal, bass-2 (62 notes): C#1; hear 4:08.8, 1:37.5. **[JUDGMENT]** **Guess:** waits.
- Xtal, drums-2 (140 notes): low thump falling 71 dB/s; hear 4:28.3, 4:04.3. **[JUDGMENT]** **Guess:** waits.
- Hedphelym, drums-2 (47 notes): low thump, slow swell, falls 119 dB/s, gets brighter; hear 3:50.0, 2:29.3. **[JUDGMENT]** **Guess:** waits.
- Hedphelym, vocals-1 (49 notes; the separator's guess at a lead, whether any voice is in it needs a listen): D4, falls 12 dB/s; hear 0:03.3, 4:47.1. **[JUDGMENT]** **Guess:** waits, and any voice must be vowels or hums.
- Not given a deep pass (mix-level only): We Are the Music Makers (7:41, 136 BPM, brightness sweeps 2.25 octaves against an album median of 0.68, tails -6.5 dB a second against -1.7); Heliosphan (4:54, 129 BPM, 16 chord changes a minute against a median of 3.1); Ageispolis (5:23, 136 BPM, tails -6.8 dB a second). **[JUDGMENT]** **Guess:** Music Makers and Ageispolis would show the longest tails and the biggest sweeps; worth a deep pass later if Ears 8 wants them.

### Reliability notes for this card
- Tempo octave open on all four (Tha settled at 135, Hedphelym at 125 by kick spacing; Xtal unresolved). Bar length unclear in all four, so repetition lags are weak.
- Key margins: Xtal 0.000 (none), Tha 0.006, i 0.056 (usable), Hedphelym 0.025 (usable).
- Hedphelym's bass stem is empty (0%) and Xtal and Tha's bass are real; Hedphelym's bass behavior and the 6-cent, 7 Hz bass wobble are not trustworthy.
- Top lines (Xtal range 8 semitones, arch 52%, looks real; Tha, Hedphelym 47% leaps, probably bleed) need a listen before use.
- The stem Demucs calls vocals follows lead lines; it is not evidence of a voice.

---

## Aphex Twin, Selected Ambient Works Volume II

Ears 5, 2026-10-16, Sonnet 5.5. The folder holds 27 tracks, all measured at mix level. The record has 25; the last two, 26 th1 [evnslower] (11:08) and 27 Rhubarb Orc. 19.53 Rev (6:42), look like extras (I did not check which edition this is). Most tracks have no title, so they go by number (11 is [Domino] and 13 is Blue Calx in these files). Deep pass on four, as the picker chose: the most typical, track 21 (4:15), and the three least like the rest, track 16 (4:46), track 15 (5:34) and track 12 (2:42). The other 23 feed For later. Machine time: 2,371 s (39.5 min) for 17.3 min of deep-pass audio: pick 698 s (27 tracks, 25.9 s a track), Demucs 1,127 s (1.09 times audio length), measuring 546 s (82 to 191 s a track, 32 s per audio minute). No track repeats on On, Selected Ambient Works 85-92, or (by title) Classics. **Guess** marks my first guesses under the lines left for Ears 8.

### The hook
- Two worlds on one record. Tracks 21 and 16 have no real beat (track 21's drums and bass stems are 0.0002 and 0.0001 against 0.128 for everything else), sit in the low-mids (61% of track 21's energy is 120 to 500 Hz, 58% of track 16's is 500 Hz to 2 kHz), and are wide (mix width -2 dB for both). Tracks 15 and 12 are the opposite: 81% and 82% of the energy under 120 Hz, near mono (width -23 and -17 dB), a drum machine at 95 BPM in 15.
- **[JUDGMENT]** One line a listener would say. **Guess:** slow, hazy drones that drift in the low-mids, or a deep sub under a ticking machine rhythm; nothing in between.

### Sounds

**S1. Slow drone pad in the low-mids (track 21): other-1 and other-2, 57% and 43% of the note energy**
- In synth terms: one pad read twice. Strongest spectral lines of the whole stem are 111.7 Hz (A2, 26 cents sharp of A=440), 132.8, 99.6 and 88.6 Hz. Attack 149 to 163 ms (25 to 457), falls 9 to 10 dB/s, harmonic share 0.93 and 0.68, brightness at the start 330 to 384 Hz, level -1.8 to -1.9 dB (it is the whole track), left and right only a third related (correlation 0.30 to 0.33), pan -0.09 to -0.13. Only 0.06% of the energy is above 2 kHz.
- Hear it at: other-2 2:32.2 (F2, 1.86 s, +55 dB alone), 1:34.4 (G2, 1.33 s, +55 dB alone), 1:21.6 (A2, 0.92 s, +56 dB alone); other-1 1:04.7, 3:03.0, 1:16.5 (+56 to +58 dB alone).
- Check: other-1's register of 37 Hz (33 to 50) is 111.7 divided by 3, so that is the pitch tracker reading a low subharmonic, not a bass note (the stem has no strong line below 88 Hz). Ignore the D1 and C1 labels on its timestamps.
- Building blocks: **[JUDGMENT]** **Guess:** a pad with a 150 ms swell and long sustain, left and right decorrelated; the wide, slowly-changing low-mid body is the new part, and the existing wash is the closest.

**S2. Glass and bell layers in the mids (track 16): other-1 37% and other-2 26% of the note energy**
- In synth terms: two layers. A low-mid tone around G2 (97 Hz, 55 to 198), attack 54 ms, falls 47 dB/s, harmonic share 0.30, partials 80 cents off whole multiples, level -5.1 dB, correlation 0.74. And bell notes around C5 (524 Hz, 201 to 1837), brightness 1.5 kHz, attack 123 ms, falls 14 dB/s, harmonic share 0.32, partials 76 cents off whole multiples, level -1.8 dB, correlation 0.43. Partials that far off whole multiples, above 60 Hz, mean bell or glass, not a saw.
- Hear it at: low layer 2:45.5 (G2, 1.78 s, +54 dB alone), 2:49.3 (F#2, 6.7 s, +54 dB alone), 2:43.2; bell layer 4:18.7 (C5, 0.25 s, +54 dB alone), 4:19.8 (C5, 0.92 s), 4:19.2 (G#4, 0.59 s).
- Caution: both groups are weak (silhouette 0.24, two groups in the stem). Read them as one blended texture, not two instruments. The stem the separator calls vocals holds 22% of this track's power (39% of the time); it is the separator's guess at voice-like synth, and whether a real voice is in it needs a listen.
- Building blocks: **[JUDGMENT]** **Guess:** glass is the closest approved sound; the missing part is the low-mid layer under it at 100 Hz with a quick fall.

**S3. Deep sub (tracks 15 and 12): bass-1, 58% and 43% of the note energy**
- In synth terms: a near-pure low tone. Track 15: register 33 Hz (31 to 35), attack 110 ms (31 to 202), falls 45 dB/s, harmonic share 0.10, level -1.0 dB, centered (correlation 1.00); the bass stem carries 72% of the track's power. Track 12: 32 Hz (31 to 34), attack 57 ms (19 to 152), falls 23 dB/s, harmonic share 0.77, level -0.6 dB, centered; the bass stem carries 57%.
- Hear it at: track 15 4:29.7 (C1, 0.29 s, +12 dB alone), 4:31.0, 4:52.3; track 12 0:07.8 (D1, 0.08 s, +22 dB alone), 0:07.3 (0.35 s, +19 dB alone), 0:07.7 (C1, 0.12 s, +18 dB alone).
- Behavior: track 15 is a pedal (one note held or repeated; 0.50 notes a second, 87% repeats, 13% octaves); the bass stem's strongest line is 60.9 Hz (B1, 22 cents flat) with others at 30.1, 58.6 and 63.3 Hz, so the pedal is B. Track 12's bass stem has lines at 30.8, 41.2, 51.6 and 62.1 Hz (B0, E1, G#1, B1), an E major stack, which fits the tool's E7 67% of the time.
- Caution: below 40 Hz pitch is good to about a semitone at best, so C1 against B0 is not settled. Partials below 60 Hz are not measurable: ignore the tool's "bell-like" (139 cents off) on track 15's bass. In track 15 the separator splits the sub across the drums stem (also 60.9 Hz) and the bass stem.
- Building blocks: **[JUDGMENT]** **Guess:** roundBass pitched to 31 to 33 Hz with a 60 to 110 ms attack; closest to what exists already.

**S4. Sub-register thump (track 12): drums-1, 254 notes, 24% of the note energy**
- In synth terms: register 42 Hz (36 to 54), attack 44 ms (19 to 132), falls 96 dB/s (14 to 422), brightness 115 Hz, centered (correlation 1.00), level -6.5 dB. Drums stem lines are at 41.2 and 51.6 Hz, the same E and G# as the bass stem.
- Hear it at: 1:36.1 (G#1, 0.22 s, +9 dB alone), 1:42.1 (F1, 0.57 s, +6 dB alone), 2:12.0 (G#1, 0.21 s, +8 dB alone). Only +6 to +9 dB alone, so these are not clean examples.
- Building blocks: **[JUDGMENT]** **Guess:** a tuned sub-kick like the one on On's D-Scape; waits for that to exist.

### Techniques

**T1. Floor falls away and comes back with a bang (track 15)**
- Low end (under 120 Hz, drums, bass and other stems summed) is 15 dB or more below its normal level at 1:17.5 (7.0 s), 2:29.0 (5.0 s), 3:27.5 (44 s), and 4:42.0 (9.5 s), then stutters at the end (5:05, 5:18, 5:22, 5:27). The 44 s one comes back at 4:11.5, where the low band jumps from -35 to -12 dB in about a second. The low end is on 75% of the time.
- Check: the tool reports one drop (4:04.8 for 2.8 s, 0.20 a minute) from the drums and bass stems alone; the separator parks the leftover sub in the other stem, so its count is wrong. During the 44 s the low band is about 24 dB down, not gone, and the drums stem plays 98% of the time.
- Track 12's low end goes only at the end (from 2:28.0 for 14 s, the tail of the file). Track 21 has no floor.
- **[JUDGMENT]** **Guess:** floorDrop is approved; a long one (40 s) with a hard return is the move to add, with two to four of them in a 5.5-minute track.

**T2. Echo at three sixteenths, and a steady sixteenth grid (track 15)**
- The drums echo at 469 ms (+10.6 dB, 3 later repeats). Drum onsets sit on a grid of 0.157 s (52 onsets in 7.9 s from 0:40), which is sixteenths at 95 BPM; three sixteenths is 472 ms. So it is a dotted-eighth echo. The mix-level tempo (95.7 BPM) and pulse 0.92 agree.
- Pumping: the tool reads the bass dipping 7.9 dB at the kick (peak 10 ms after, kick every 0.32 s, which is two sixteenths); it reads about 70% of the true depth, so this is a floor. Caution: the drums and bass stems share the 60.9 Hz line, so part of the dip may be the separator trading energy between stems. I did not verify it another way.
- **[JUDGMENT]** **Guess:** a beat-synced delay at three sixteenths is the same family as On's T1; the pump is probably real but unproven.

**T3. Slow pitch wobble on the pads (tracks 21, 16)**
- Track 21: the other stem wobbles about 11 cents at 0.4 Hz and its speed moves up to 16 cents in 4 s. Track 16: about 32 cents at 0.4 Hz. The tool read a 12-cent wow as 13.7, so the 11-cent reading is believable; 32 cents is beyond anything I calibrated, so read it as large and slow, not as the number. The bass wobbles the tool lists for both (16 and 27 cents) come from stems with 0% of the power: ignore them. Track 12 shows 8 cents at 3.8 Hz in the other stem, near the tool's limit.
- **[JUDGMENT]** **Guess:** a per-voice wobble of 10 to 30 cents at 0.4 Hz, slower than Selected Ambient Works 85-92 (5 to 14 cents at 0.5 to 2.9 Hz).

**T4. Five-octave openings (tracks 12 and 15), plus a regular brightness drift (track 21)**
- Track 12's mix brightness rises 5.1 octaves from 0:03 to 0:21, then falls 0.9, 1.7 and 0.9; track 15 falls 4.9 octaves from 0:00 to 0:08.5, rises 1.1, then swings up 3.3 and down 4.5 in 14 s at 1:14 to 1:28.5. Track 21 makes 56 sweeps of 0.8 to 1.4 octaves in 4:15. These are whole-mix numbers: in 12 and 15 more than 80% of the energy is under 120 Hz, so the swing is hats and mids appearing and leaving over a sub, not proof of a filter.
- **[JUDGMENT]** **Guess:** the arc already covers the five-octave opening; the 56 small sweeps in track 21 are worth a listen before anything is built.

### Theory

**Th1. Track 15: semitone cluster over a B pedal, 95 BPM, off-grid pattern**
- The pitch classes crowd around the pedal: B 0.18, C 0.15, A# 0.10, C# 0.09, F# 0.07. The tool names Cmaj7 54% and Bmaj7 24% (names on dense music are a guess); only 4 settled chords a minute and 42% of changes land on a beat against 38% by chance, so harmony is not beat-locked. Key margin 0.006: B phrygian is a guess.
- Tempo: the meter tool said 76 (or 38 or 152), but the drum onsets sit on a 0.157 s grid that gives 95 BPM, matching the mix-level 95.7 and a pulse of 0.92. The drum pattern repeats strongest at 8 sixteenths (1.265 s, 0.93), with side peaks at 5 (0.789 s) and 3 (0.476 s) sixteenths, so the pattern is off the four-on-the-floor grid.
- **[JUDGMENT]** **Guess:** a semitone-crowded pitch set above a single pedal note, with a 3 plus 5 sixteenth rhythm; the cluster is the usable idea.

**Th2. Track 21: A minor pentatonic over a pad, no beat**
- Pitch classes C 0.22, A 0.18, E 0.12, G 0.12, F 0.09: the tool says C mixolydian (margin 0.021, barely usable) and fits C major pentatonic, which is the same notes as A minor pentatonic. Chords: Amin 38%, C 13%, Cmaj7 11%, F 9% (names are a guess); 13 settled chords a minute. Tuning is 24 cents sharp (single-file readings within 8 cents are noise, so this is real).
- The tool's 111 BPM, 0.57 swing and 70% of chord changes on a beat (chance 56%) mean nothing here: there are no drums.
- **[JUDGMENT]** **Guess:** a five-note set with a long, slow chord rhythm (a change every 3 s) and a tuning offset of about +25 cents as a dial.

### Space
- Tracks 21 and 16 are wide. Median width (two-second windows): track 21 -1 dB (10th to 90th percentile -4 to +1), track 16 -1 dB (-7 to 0). Track 21 by band: sub correlation 0.90, 120 to 500 Hz 0.28, 500 Hz to 2 kHz 0.06; the 2 to 5 kHz band has correlation -0.39 but holds 0.06% of the energy. The mono fold costs the other stem 2.0 dB. Slow movement: the other stem's pan range is 0.39 cycling every 21 s (long enough to be real).
- Tracks 15 and 12 are mono at the bottom. Track 15 has correlation 0.97 to 1.00 in every band (width -18 to -25 dB, bass -29 dB), a fully mono track. Track 12 is mono in the sub (-25 dB, correlation 1.00) and opens up through the mids (500 Hz to 2 kHz -7 dB, correlation 0.69).
- **[JUDGMENT]** **Guess:** two space recipes: wide low-mid pad (width about -2 dB) and mono sub with a rhythm part that is also mono (-18 dB or narrower).

### Vibes (all guesses; weights lean x1.5 to x3 toward, x0.2 to x0.5 away, never 0)
- S1 low-mid drone: toward still, restful, meditative, hazy, foggy x2; away from bouncy, energetic x0.3.
- S2 glass and bell layers: toward mysterious, strange, twinkling x1.5; away from booming x0.4.
- S3 deep sub: toward booming, hypnotic x2; away from airy, twinkling x0.4.
- T1 long floor drop: toward hypnotic, experimental x2; away from cozy x0.5.
- T2 dotted-eighth echo: toward floating, spacious x1.5.
- T3 slow wobble: toward hazy, floating x2; away from steady x0.4.
- **[JUDGMENT]** Ears 8 settles all of this.

### Wholesome take
- Brightness flags: none. Energy above 2 kHz is 0.1% in track 21, 12% in track 16 (the brightest), 5% in track 15, 4% in track 12; no polarity flags.
- **[JUDGMENT]** **Guess:** take the drones, the sub, the long floor drop and the dotted echo. Track 16's mid glass layer needs a softer top. Anything from a "vocals" stem waits for a listen, and if a real voice is in there, use only its pitch and texture, never words.

### For later
- Track 15, drums-1 (355 notes): low thump, falls 147 dB/s; hear 2:37.0, 2:37.2. **[JUDGMENT]** **Guess:** waits, a second kick next to the sub.
- Track 15, drums-2 (145 notes): low thump, falls 30 dB/s, gets darker; hear 5:03.5, 3:23.8. **[JUDGMENT]** **Guess:** waits.
- Track 15, bass-2 (63 notes): B1, harmonic, falls 35 dB/s; hear 4:58.0, 0:24.1. **[JUDGMENT]** **Guess:** waits, the octave-up B.
- Track 12, other-1 (266 notes): E1, mixed or noisy, falls 21 dB/s; hear 1:19.0, 1:19.3. **[JUDGMENT]** **Guess:** waits.
- Track 12, bass-2 (67 notes): E1, falls 27 dB/s; hear 0:09.5, 0:09.6. **[JUDGMENT]** **Guess:** waits.
- Track 16, drums-1 (73 notes): mid snap, falls 13 dB/s; hear 0:38.5, 4:11.4. **[JUDGMENT]** **Guess:** waits.
- Track 16, vocals-1 (47 notes, D3, falls 15 dB/s; hear 4:05.8, 4:05.2) and vocals-2 (19 notes, A#6; hear 4:04.2, 4:03.6): the separator's guess at voice-like sound; a listen decides. **[JUDGMENT]** **Guess:** waits, and any voice must be vowels or hums.
- Not given a deep pass (mix-level only): track 9 (6:54.7, 112 BPM, tail -19.0 dB a second against an album median of -0.8, 40 pitch bends a minute against 6.5); track 8 (5:08.5, tail -11.7, 30 bends a minute); track 2 (6:34.2, 129 BPM, tail -9.7); track 24 (11:27.2, 83 BPM, tail -9.0, 24 bends a minute); track 20 (5:57.5, brightness sweep 1.76 octaves against a median of 0.63); track 1 (7:27.8, 19.3 chord changes a minute against a median of 4.3). **[JUDGMENT]** **Guess:** tracks 9 and 8 are the strongest candidates for a short-decay or pitch-bend sound; worth a deep pass if Ears 8 wants them.

### Reliability notes for this card
- Tempo octave: track 21 has no beat so its tempo and swing are meaningless; track 16 is 24% drums by time (3% of power) and its 109 BPM is weak; track 15 settled at 95 BPM from the drum grid (the meter tool's 76 is wrong or an octave off); track 12's 98 BPM (or 49 or 196, mix-level 123, pulse 0.26) is unresolved.
- Key margins: track 21 0.021 (usable, barely), track 16 0.002, track 15 0.006, track 12 0.007 (all guesses). Chord changes on a beat against chance: 70 against 56 (meaningless, no beat), 48 against 55, 42 against 38, 38 against 49: none is beat-locked.
- Top lines: ranges of 10, 36, 11 and 43 semitones (and 41% leaps in track 12, 665 notes in 16) mean tracks 16 and 12 are mostly bleed; track 21's (10 semitones, 53% thirds) is the plausible one; track 15's other stem holds 1% of the power, so its 751 notes are leakage.
- Separator: track 15's sub is split across drums, bass and other stems, so its floor-drop count and pump depth are not trustworthy as printed. Tracks 21 and 16 have empty bass stems (0% of the power), so their bass behavior and bass wobbles are void. Vocals stems are the separator's guess.
- Pitch under 40 Hz is good to about a semitone; partial readings under 60 Hz (the "bell-like" labels on 15's and 12's bass) are not measurable.
- Tuning: the mix tool reads track 21 at +24 cents (matches its 111.7 Hz line, A2 +26 cents) and track 12 at +41 cents, but track 12's bass lines (E1 at 41.2 Hz) sit on A=440, so the +41 is doubtful and its pitch-class weights (D, D#, E, C#, C spread over neighbors) are best read as a guess.

---

## Aphex Twin, Classics

Ears 5, 2026-10-16, Sonnet 5.5. 11 tracks, all measured at mix level. Deep pass on three, as the picker chose: the most typical, 05 Polynomial-C (4:47), and the two least like the rest, 11 We Have Arrived (Aphex Twin QQT mix) (4:23) and 07 Phlange Phace (5:22); the picker found only two unlike tracks (the third would have been too close to one already taken). It is a 12-inch-style collection: drum-led, 5 of the 11 tracks have 58% or more of the energy under 120 Hz. Machine time: 1,693 s (28 min) for 14.5 min of deep-pass audio: pick 243 s (11 tracks, 22 s a track), Demucs 948 s (1.09 times audio length), measuring 502 s (148 to 182 s a track, 35 s per audio minute). **Guess** marks my first guesses under the lines left for Ears 8.

**Shared tracks.** No title repeats on On, Selected Ambient Works 85-92, Volume II or Polygon Window, and no pair of tracks across the five pile 1 albums matches in both length (within 3 s) and loudness (within 0.6 LU) except Ptolemy (SAW 85-92, 7:14, -10.4 LUFS) against Volume II's track 18 (7:17, -10.7), which are different tracks by title (I did not listen to compare). Pile 2 should compare its tracks against these (seconds, LUFS, from this run's `pick.json`): Digeridoo 432.1 s, -8.3; Flaphead 420.4 s, -8.6; Phloam 333.2 s, -9.7; Isopropanol 383.8 s, -10.0; Polynomial-C 286.5 s, -10.5; Tamphex (Hedphuq mix) 391.2 s, -8.7; Phlange Phace 322.3 s, -7.2; Dodeccaheedron 368.7 s, -7.2; Analogue Bubblebath 1 286.3 s, -10.7; Metapharstic 273.4 s, -9.0; We Have Arrived (QQT mix) 263.0 s, -5.2. Pile 2's list holds Analogue Bubblebath 4, 5 and 3.1 (not 1), so none of these should be its twin by title; if a pile 2 EP repeats a track here, give it one deep pass, in whichever card runs first.

### The hook
- Drums carry the record (drums stems hold 52%, 83% and 73% of the three deep-pass tracks' power), the sub-kick is tuned to pitches (A#1, A1, F#1, C#2), the grid is 141 to 146 BPM with broken, off-grid hits, and a chord (a major seventh) is held for up to a minute with a note a semitone below its root nearly as loud as the root.
- **[JUDGMENT]** One line a listener would say. **Guess:** a tuned sub-kick and a bright snap over long held major-seventh chords, at 143, with the floor dropping out for a few seconds.

### Sounds

**S1. Tuned sub-kick that opens up (Phlange Phace): drums-2, 103 notes, 15%; drums-1, 141 notes, 18%**
- In synth terms: drums-2 is a low thump at 58 Hz (34 to 68), attack 75 ms, falls 60 dB/s, brightness 297 Hz and rising 1.45 octaves over the note, level -0.7 dB, correlation 0.81. drums-1 sits above it: a mid snap at 59 Hz fundamental (58 to 69) with brightness 1.1 kHz, rising 1.07 octaves, falls 47 dB/s, correlation 0.71. The drums stem holds 83% of the track's power and 64% of the stem is under 120 Hz, 15% above 5 kHz.
- Hear it at: drums-2 4:37.3 (A1, 0.49 s, +32 dB alone), 4:37.0 (F#1, 0.43 s, +32 dB alone), 5:05.3 (C#2, 0.29 s, +29 dB alone); drums-1 2:30.4 (A#1, 0.22 s, +38 dB alone), 2:25.5 (A#1), 3:42.3 (A#1).
- Check: the bass stem holds 1% of the power and the drums stem has the whole sub, so the separator put the sub in drums, as on On's D-Scape. The kick is tuned: it lands on A#1, A1, F#1 and C#2 (pitch under 60 Hz is good to about a semitone). The "partials 134 to 147 cents off" cannot be measured below 60 Hz; ignore it. The tool's bass line for this track (a pedal on E, 0.06 notes a second) comes from the empty bass stem: void.
- Building blocks: **[JUDGMENT]** **Guess:** roundBass pitched to follow the chords, with a short rising click; "tuned sub-kick" was already on the list from On.

**S2. Bright mono ticks and noise over a sub kick (We Have Arrived, QQT mix): drums-1, 312 notes, 49%**
- In synth terms: the group fuses a sub kick (register 43 Hz) with bright hits (up to 1.3 kHz fundamental; 10th to 90th percentile 43 to 1,286 Hz), so it is not one instrument. Brightness at the start 3.0 kHz (2.1 to 4.6), attack 97 ms, falls 17 dB/s, level -0.7 dB, correlation 0.99. I checked the drums stem: 26% of its energy is under 120 Hz, 35% is 2 to 5 kHz and 26% is above 5 kHz. The whole track has 59% of its energy above 2 kHz and is near mono (width -22 dB).
- Hear it at: 1:20.0 (F#1, 1.71 s, +26 dB alone), 1:18.3 (F#1, 0.86 s, +27 dB alone), 1:19.2 (F1, 0.44 s, +28 dB alone).
- Building blocks: **[JUDGMENT]** **Guess:** a noise-hat layer sitting on a sub-kick, mono; the closest existing sounds are the hats, with the bright level the new thing. A harsh-leaning sound: keep it behind particular vibes.

**S3. Bright acid-like bass (We Have Arrived): bass-1, 135 notes, 12%**
- In synth terms: around C2 (65 Hz, 42 to 92), harmonic share 0.89 (a saw or square, not a bell), attack 123 ms (34 to 365), falls 36 dB/s, brightness at the start 2.7 kHz, level -5.9 dB, centered (correlation 1.00); the bass stem holds 19% of the power.
- Hear it at: 0:51.1 (F#2, 1.68 s, +9 dB alone), 0:49.4 (C#2, 0.88 s, +10 dB alone), 3:06.4 (C#2, 0.84 s, +10 dB alone). Only +9 to +10 dB alone, so these are not clean.
- Building blocks: **[JUDGMENT]** **Guess:** moogbass with a bright start (2.7 kHz) that falls quickly; it needs a listen.

**S4. Low-mid organ-like pad with a fast wobble (Polynomial-C): other-1, 381 notes, 37%**
- In synth terms: register 83 Hz (76 to 135), attack 118 ms (25 to 369), falls 19 dB/s, brightness 483 Hz, harmonic share 0.58, partials 61 cents off whole multiples, level -3.0 dB, left and right two-thirds related (correlation 0.67). The other stem is 56% 120 to 500 Hz and 35% 500 Hz to 2 kHz.
- Hear it at: 0:52.8 (A#2, 0.34 s, +53 dB alone), 0:52.6 (A2, 0.13 s, +54 dB alone), 0:52.0 (A#2, 0.12 s, +53 dB alone).
- Caution: the group is weak (silhouette 0.23, two groups in the stem). The track is 45 cents sharp of A=440 by the mix tool (single-file readings within 8 cents are noise, so this is real): note labels like A#2 are the nearest semitone, not the pitch.
- Building blocks: **[JUDGMENT]** **Guess:** a pad with a 120 ms attack and a 3.5 Hz wobble of 15 cents; the wobble rate is much faster than the pads of Selected Ambient Works Volume II.

### Techniques

**T1. A sub-less intro, then the floor drops out for 3 to 11 s (all three)**
- Low end (under 120 Hz, drums, bass and other summed) is more than 15 dB down at the start of all three: Phlange Phace 0:00 for 13 s, We Have Arrived 0:00 for 13.5 s, Polynomial-C in pieces (0:09.5 for 12 s, 0:22 for 7 s, 0:33.5 for 24.5 s, so the full floor arrives about 0:58, when the drums stem comes in at 0:57). Later drops: Polynomial-C 2:24 (3.5 s), 2:31.5 (3 s), 2:38 (3 s), 3:30.5 (3.5 s), 3:44 (3 s), 4:43.5 (3 s); Phlange Phace 1:31.5 (3 s) and 4:07 (11 s); We Have Arrived 2:16 (3.5 s) and 2:50.5 (3 s). The low end is on 73% (Polynomial-C), 86% (Phlange Phace) and 90% (We Have Arrived) of the time.
- Check: the tool lists Polynomial-C at 1.17 drops a minute (0:17.5 for 40.2 s, 2:24.8 for 4.2 s, 2:29.0 for 13.2 s) and Phlange Phace at 4:08.2 for 4.0 s; my all-stem check gives 4:07 for 11 s for the latter. The tool counts drums and bass stems only, and the separator parks sub in whatever stem it likes. We Have Arrived: tool 0.00 a minute, my check finds two 3 s dips. The ends of Phlange Phace (5:06 for 16 s) and We Have Arrived (4:15.5 for 7.5 s) are tails, not drops.
- **[JUDGMENT]** **Guess:** floorDrop at 3 to 11 s is the approved move; a run of three 3 s drops in 14 s (Polynomial-C, 2:24 to 2:41) is the version that is new, and a sub-less 13 s intro belongs with the arc.

**T2. Dotted-quarter echo, and whole-mix brightness swings (We Have Arrived, Phlange Phace, Polynomial-C)**
- We Have Arrived: the drums echo at 624 ms (+3.8 dB, one later repeat), which is a dotted quarter at 143.6 BPM (627 ms). Phlange Phace: the drums echo at 339 ms (+3.2 dB, 2 later repeats), equal to two thirds of a beat at the mix tempo of 117.5 BPM (340 ms) but not a clean fraction of the 146 BPM the drum grid gives, so its tempo is unresolved. Polynomial-C: none found.
- Brightness: Polynomial-C swings up 2.1 octaves at 3:28 and down 2.0 at 3:33 to 3:48; We Have Arrived falls 2.9 octaves at 2:15.5, rises 1.8, then falls 3.5 octaves from 4:04.5 to 4:11; Phlange Phace has 47 sweeps (two to 2.3 octaves). These are whole-mix numbers on drum-led tracks, so the swings are hats and fills coming and going over the sub; they are not proof of a filter.
- **[JUDGMENT]** **Guess:** a dotted-quarter echo on drums only; the brightness swings are drum fills, not a technique to build.

**T3. Off-pitch tuning and a fast wobble (Polynomial-C)**
- Polynomial-C is 45 cents sharp, so each note sits nearly between two semitones (its top pitch classes are D# 0.15, E 0.12, C 0.12, C# 0.11). The other stem wobbles about 15 cents at 3.5 Hz. The tool read a 12-cent wow as 13.7, so 15 is believable. Phlange Phace is 10 cents sharp and We Have Arrived 27 cents flat. Among the 11 tracks, the readings range from -41 to +45 cents.
- **[JUDGMENT]** **Guess:** a tuning offset of up to a quarter-tone, as a draw per loop, with a 3 to 4 Hz wobble of 10 to 15 cents.

### Theory

**Th1. A 141 to 146 BPM grid with broken hits, and the tool's half-time**
- Drum-stem onset repeats: Polynomial-C 1.66 s (strongest, 0.80), 0.824 s (0.69) and 0.313 s (0.65); Phlange Phace 1.637 s (0.86), 0.813 s (0.60); We Have Arrived 1.707 s (0.80) and 0.848 s (0.81). So each track has a 4-beat bar of 1.64 to 1.71 s, which is 144.5, 146.5 and 141 BPM. The tool's meter reads 72, 73 and 71 BPM (the same bars, counted as 2 beats), the mix-level count gives 143.6 for We Have Arrived (agrees) and 117.5 for the other two (does not). Each track also shows many unrelated periodicities (3:2, 5:2, 5:4, 7:4), which the tool reads as irregular timing: the pattern is broken, not four on the floor. Swing: Polynomial-C 0.62 of the beat (triplet feel), We Have Arrived straight, Phlange Phace 0.76 (a late hit, not swing).
- **[JUDGMENT]** **Guess:** a tempo draw of 140 to 147 with a bar of 4 beats and a drum pattern that lands on 3, 5 and 8 sixteenths.

**Th2. A long-held major seventh with the note below the root nearly as loud**
- We Have Arrived: C#maj7 87% of the time (2 changes a minute; median chord 5.4 s, up to 60 s); the pitch classes are C# 0.15 and C 0.14 with everything else under 0.08. Polynomial-C: G#maj7 64% (8 settled chords a minute; median chord 2.5 s), with 49% of changes on a beat against 36% by chance. Phlange Phace: 41 changes a minute (median chord 1.0 s, 37 distinct chords: Emaj7, Fmaj7, Amaj7, F#min7), 54% on a beat against 37%; this is about one change a bar (36 bars a minute) with the changes slightly beat-locked.
- Check: key margins are 0.002 (We Have Arrived), 0.013 (Polynomial-C) and 0.035 (Phlange Phace: usable, E major): only the last is more than a guess. Chord names on dense music are a guess. In Phlange Phace the drums stem holds 83% of the power and the other stem 8%, so its pitch classes and chords may be tracking the tuned kicks (A#, A, F#, C#) rather than a pad. In We Have Arrived the drums hold 73% and the other stem 3%, so the same worry applies. Polynomial-C (drums 52%, other 47%) is the one with real harmony.
- **[JUDGMENT]** **Guess:** one held major-seventh chord for 30 to 60 s, with a short-lived neighbor tone a semitone under the root; Polynomial-C's pad is the clean version.

### Space
- Three different spaces. We Have Arrived is nearly mono: mix width median -25 dB (-28 to -16), correlation 0.94 to 0.99 in every band. Polynomial-C is half-wide: median -7 dB (-8 to -6); sub -17 dB (correlation 0.96), 120 to 500 Hz -9, mids -7, 2 to 5 kHz -2 (0.26), above 5 kHz +6 dB with correlation -0.58 (under 1% of the energy). The other stem carries the width (-6 dB; drums -13 dB). Phlange Phace is wide: median -1 dB (-2 to 0); sub -26 dB (correlation 1.00), low-mids -7, mids -6, 2 to 5 kHz -4, above 5 kHz +3 dB (correlation -0.33); drums -6 dB (mono loss 1.1 dB), the vocals stem +3 dB (mono loss 4.8 dB, 7% of the power).
- Movement: Phlange Phace's other stem has a pan range of 0.42, cycling every 3 s, and the drums 0.05 every 3 s. A 3 s period shared by two parts looks like the tool's window, not an auto-pan; unconfirmed. The other two tracks stay put.
- **[JUDGMENT]** **Guess:** three recipes: a mono mix (-25 dB) for the bright, hard track; a half-wide pad over a mono sub; and a wide top over a mono sub (the sub under 120 Hz stays mono in all three).

### Vibes (all guesses; weights lean x1.5 to x3 toward, x0.2 to x0.5 away, never 0)
- S1 tuned sub-kick that opens: toward booming, energetic, hypnotic x2; away from restful, still x0.4.
- S2 bright mono ticks: toward energetic, strange, experimental x2; away from cozy, comfy, tender, hazy x0.3 (a harsh-leaning sound, so it stays behind its own vibes).
- S3 bright acid-like bass: toward cheeky, bouncy x2; away from restful, meditative x0.4.
- S4 low-mid pad with a fast wobble: toward hazy, hypnotic x1.5.
- T1 floor drops and the sub-less intro: toward hypnotic, minimal x1.5.
- T3 quarter-tone offset: toward strange, mysterious x2.
- **[JUDGMENT]** Ears 8 settles all of this.

### Wholesome take
- Brightness flags: We Have Arrived has 59% of its energy above 2 kHz (stem sum 59%), and its drums stem is 61% above 2 kHz (bright or harsh). Phlange Phace is 21% above 2 kHz by the direct spectrum (the draft's vibes line says 39%; the two measures disagree and I did not find out why), 15% of that above 5 kHz. Polynomial-C is 2%. No polarity flag; the high bands of Polynomial-C and Phlange Phace are out of phase (correlation -0.58 and -0.33) but hold under 1% and about 15% of the energy.
- **[JUDGMENT]** **Guess:** take the tuned kick, the held chord, the echo and Polynomial-C's pad; take the bright ticks only behind energetic, strange and experimental vibes, at a gentle level. The out-of-phase highs of Phlange Phace (a 15% share above 5 kHz) need the stereo guard, because a mono fold would thin them.

### For later
- Polynomial-C, drums-2 (114 notes): mid snap, falls 87 dB/s, gets brighter; hear 4:42.0, 1:56.8. **[JUDGMENT]** **Guess:** waits, close to Phlange Phace's snap.
- Polynomial-C, other-2 (119 notes): around E1, mixed or noisy, falls 18 dB/s; hear 0:42.4, 0:42.6. **[JUDGMENT]** **Guess:** waits, a low rumble under the pad.
- Phlange Phace, other-1 (375 notes): C#2, bell-like, slow swell, falls 20 dB/s; hear 0:03.1, 4:10.3. **[JUDGMENT]** **Guess:** waits, a low bell pad (at 69 Hz the bell-like label is near the tool's limit).
- Phlange Phace, other-2 (90 notes): B4, harmonic, slow swell, falls 16 dB/s; hear 0:00.0, 0:01.6. **[JUDGMENT]** **Guess:** waits, a high pad opening the track.
- We Have Arrived, drums-2 (110 notes): low thump, falls 53 dB/s, brightens; hear 1:21.7, 2:19.2. **[JUDGMENT]** **Guess:** waits, the kick under the ticks.
- We Have Arrived, bass-2 (340 notes): F1, falls 134 dB/s; hear 4:15.0, 4:14.1. **[JUDGMENT]** **Guess:** waits, a short bass pluck.
- We Have Arrived, other-1 (245 notes): F#1, falls 35 dB/s; hear 2:13.7, 2:10.4. **[JUDGMENT]** **Guess:** waits.
- We Have Arrived, vocals-1 (359 notes, G#6, falls 51 dB/s; hear 2:58.3, 1:58.7): the separator's guess at voice-like sound on an instrumental; it is probably the highest part of the ticks. **[JUDGMENT]** **Guess:** waits, and if a real voice is in there it never comes in as words.
- Not given a deep pass (mix-level only): Analogue Bubblebath 1 (4:46, 123 BPM, brightness sweep 1.53 octaves against an album median of 0.92, 4.8 chord changes a minute against 0.9); Metapharstic (4:33, 144 BPM, pulse 0.60 against a median of 0.81); Phloam (5:33, 136 BPM, tail +5.5 dB a second against a median of -1.4: it rises after notes, worth a listen); Tamphex (Hedphuq mix) (6:31, 129 BPM, pulse 0.68); Dodeccaheedron (6:09, 25% of its energy above 2 kHz, tail -3.9, 38 pitch bends a minute); Digeridoo, Flaphead and Isopropanol (the first two at 136 to 144 BPM; Isopropanol 58% sub). **[JUDGMENT]** **Guess:** Phloam and Dodeccaheedron are the two that look unlike the rest; worth a deep pass if Ears 8 wants them.

### Reliability notes for this card
- Tempo: the tool's 71 to 73 BPM is the half-time of a 141 to 146 BPM grid I read off the drum-stem onsets. The mix-level 117.5 for Polynomial-C and Phlange Phace is wrong.
- Key margins: We Have Arrived 0.002, Polynomial-C 0.013, Phlange Phace 0.035. Tuning readings run from -27 to +45 cents, and a half-semitone offset splits pitch classes across neighbors.
- Bass stems: Polynomial-C (1% of the power) and Phlange Phace (1%) are near empty, so their bass behavior (pedal on E, root movement on D#) is void and the bass pump numbers (4.5 dB in Phlange Phace) are void. We Have Arrived's bass stem is real (19%), but its 7.4 dB dip at the kick and "kick every 0.26 s" do not fit its 0.42 s beat, so that kick spacing is not the main kick.
- Top lines: ranges 7, 43 and 20 semitones (We Have Arrived has 38% leaps): only Polynomial-C's (7 semitones, 39% thirds, 28% fourths and fifths) is plausible; the others are bleed from ticks and kicks.
- Pitch under 60 Hz: partial-off labels are not measurable; kick pitches are good to about a semitone.
- Separator: the sub lives in the drums stem for all three tracks; the floor drops from stems are underestimates (see T1).

---

## Polygon Window, Surfing on Sine Waves (Expanded Edition, 2025)

Ears 5, 2026-10-16, Sonnet 5.5. 14 tracks, all measured at mix level. **Quoth is inside this edition:** all four Quoth EP tracks are here, as 03 Quoth, 12 Iketa, 13 Quoth (Wooden Thump Mix) and 14 Bike Pump Meets Bucket, so Quoth needs no folder of its own. Deep pass on four, as the picker chose: the most typical, 01 Polygon Window (5:28), and the three least like the rest, 09 Quino-phec (4:44), 03 Quoth (5:37) and 04 If It Really Is Me (7:04). So the Quoth EP got one deep pass (its title track); the other three Quoth tracks are in For later with their mix-level numbers. Machine time: 2,554 s (42.6 min) for 22.9 min of deep-pass audio: pick 322 s (14 tracks, 23 s a track), Demucs 1,504 s (1.10 times audio length; the first track ran 1.42 times while I ran other jobs), measuring 728 s (182 s a track, 32 s per audio minute). No track repeats on the other pile 1 albums (titles, then length and loudness). **Guess** marks my first guesses under the lines left for Ears 8.

### The hook
- Heavy, tuned, mono-at-the-bottom techno: sub energy is 43% to 83% of the mix in 13 of the 14 tracks (Quino-phec has none), the kick and sub are tuned to a pitch (A1, G#1, B0), and breakdowns where the whole floor leaves run 20 to 46 s. Tuning wanders: 36 cents flat in the title track, 48 flat to 40 sharp across the album.
- **[JUDGMENT]** One line a listener would say. **Guess:** a deep tuned kick and a gated sub under chords that blur and echo in triplets, with a long breakdown and a hard return.

### Sounds

**S1. Gated sub (Quoth, 03): drums-1 168 notes, 38%; drums-2 138 notes, 30%**
- In synth terms: a sub on B0 and C1 (33 Hz, 31 to 64) that rises in about 30 ms to a flat top held about 180 ms, then cuts (I checked the low-band envelope at 3:37.2: -35 to -13 dB in about 40 ms, flat at -13 for about 170 ms, then falling). The tool's "slow swell, attack 187 ms" is the length of that flat top, not a slow attack. Falls 151 dB/s, level -0.1 dB, centered (correlation 0.98). drums-2 is the same gated shape one octave up and brighter (62 Hz, brightness 668 Hz, flat top 187 ms, falls 143 dB/s).
- Hear it at: drums-1 3:37.2 (C1, 0.29 s, +46 dB alone), 3:32.4 (B0, 0.23 s, +46 dB alone), 1:00.1 (B0, 0.23 s, +45 dB alone); drums-2 1:01.8 (B1, 0.24 s, +43 dB alone), 1:15.6, 1:18.7.
- Check: the drums stem holds 99.8% of Quoth's power (the separator put the whole track there), so "drums" here means the mix; the tool's bass behavior, bass pumping and "other dips 3.6 dB" for this track are void.
- Building blocks: **[JUDGMENT]** **Guess:** roundBass with a hard gate (30 ms rise, 180 ms flat top, hard cut) at 31 to 33 Hz plus its octave; closest to what exists.

**S2. Tuned kicks (Polygon Window, 01; If It Really Is Me, 04)**
- Polygon Window drums-1 (207 notes, 21%): register 54 Hz (54 to 58), attack 67 ms (22 to 110), falls 26 dB/s, brightness 410 Hz, level -0.5 dB, correlation 0.95. The track is about 36 cents flat (mix tool, -36 cents), and 54 Hz is A1 at that offset (53.9 Hz), the key root. Hear 4:00.9 (A1, 0.27 s, +43 dB alone), 3:57.4, 4:08.1.
- If It Really Is Me drums-1 (461 notes, 35%): register 58 Hz (50 to 59), attack 10 ms (9 to 12), falls 50 dB/s, brightness 183 Hz, level -1.2 dB, correlation 1.00; I checked the envelope at 6:04: -42 to -16 dB in 10 ms, then a tail of about 0.4 s. Hear 6:04.3 (G#1, 0.14 s, +42 dB alone), 6:16.6 (A1, 0.13 s, +38 dB alone), 6:08.4 (A1, 0.13 s, +38 dB alone).
- Check: the bass stem is empty in 04 (0% of the power), so the sub lives in the drums stem. "Partials 181 cents off" is unmeasurable under 60 Hz: ignore it. Pitch under 60 Hz is good to about a semitone.
- Building blocks: **[JUDGMENT]** **Guess:** the tuned sub-kick already on the list from On and Classics, with a 10 ms click for the sharp one and a 70 ms softer one for the other; it is a third confirmation, not a new sound.

**S3. Low struck bell (If It Really Is Me, 04): other-1, 335 notes, 37%**
- In synth terms: around F3 (175 Hz, 131 to 176), attack 51 ms (25 to 131), falls 27 dB/s, brightness 528 Hz, harmonic share 0.55, partials 98 cents off whole multiples (this is above 60 Hz, so bell- or metal-like holds), level -1.8 dB, centered (correlation 1.00).
- Hear it at: 4:12.5 (F3, 0.26 s, +58 dB alone), 4:12.4 (F3, 0.18 s, +58 dB alone), 4:20.8 (F3, 0.18 s, +56 dB alone). All three sit inside the 4:04 to 4:23 breakdown, with the drums out, so this is the part that carries it.
- Building blocks: **[JUDGMENT]** **Guess:** glass pitched down two octaves with a 50 ms attack; the low register is the new part.

**S4. Drone pad (Quino-phec, 09): other-1 57% and other-2 43% of the note energy**
- In synth terms: one pad read twice. other-1 is around A#2 (115 Hz, 73 to 230), other-2 around A#4 (465 Hz, 452 to 517): the same note two octaves apart, so most likely one drone and its fourth harmonic. Attack 196 to 205 ms, falls 11 to 19 dB/s, harmonic share 0.47 and 0.81, level -1.1 to -1.2 dB, left and right half related (correlation 0.52 to 0.58). The other stem holds 100% of the power and there is no sub at all (0.2% of the energy under 120 Hz, the rest split 50/49 between 120 to 500 Hz and 500 Hz to 2 kHz).
- Hear it at: other-1 2:32.7 (A2, 0.4 s, +58 dB alone), 1:03.9 (A#2, 0.85 s, +56 dB alone), 3:06.1 (A#2, 0.87 s, +56 dB alone); other-2 1:33.5 (C5, 1.21 s, +56 dB alone), 1:12.3 (A#4, 1.05 s, +56 dB alone), 1:40.3 (A#4, 0.17 s, +59 dB alone).
- Caution: the groups are weak (silhouette 0.22, two groups in the stem). The track is the quietest of the album (-20.2 LUFS against a median of about -13.3).
- Building blocks: **[JUDGMENT]** **Guess:** a pad with a 200 ms swell, left and right half related; the wide low-mid drone from Volume II's track 21 is the nearest cousin.

### Techniques

**T1. Long breakdowns where the floor leaves and returns (01, 03, 04), and a stepped intro (01)**
- Low end (under 120 Hz, drums, bass and other summed) 15 dB or more down: Polygon Window 2:05.5 for 43 s (back at 2:48.5; the bass stem leaves at 2:05.5 and the drums at 2:20.5, return 2:29.5); Quoth 2:03 for 13.5 s and 2:18.5 for 46 s (back at 3:04.5); If It Really Is Me 4:03.5 for 20.5 s (drums out 4:04.0, back 4:23.0). Polygon Window's intro brings the sub in steps: out for 9.5 s, then 0:10 for 5.5 s, 0:19.5 for 3.5 s, 0:24.5 for 2.0 s. Endings: Polygon Window 5:15.5 (12 s), Quoth 5:06 and 5:21.5 (15 s each), If It Really Is Me 6:18.5 (45 s) are tails, not drops.
- Check: the tool lists Polygon Window at 2:19.8 (2.5 s), 2:26.0 (12.5 s) and 2:38.0 (3.5 s), Quoth at 0.00 a minute (its low end is on only 69% of the time, under the 75% rule, so it skips), and If It Really Is Me at 4:03.5 for 20.8 s (agrees). The 43 s Polygon Window drop is real but my all-stem check at 15 dB finds one piece where the tool finds three, because the separator parks a C#2 and E2 rumble in the other stem during the breakdown (hear 2:21.3, 2:20.0, 2:28.7, +27 to +31 dB alone).
- **[JUDGMENT]** **Guess:** floorDrop is approved at 2 to 12 s; the 20 to 46 s breakdown with a hard return is the longer version; allow it once per 5 minutes.

**T2. Echoes set to the beat (01, 04)**
- Polygon Window: other echoes at 674 ms (+3.4 dB, 2 later repeats), which is one beat at 89.1 BPM (674 ms), and drums at 454 ms (+8.5 dB, 2 later repeats), two thirds of a beat (449 ms, a triplet quarter). If It Really Is Me: drums at 384 ms (+19.5 dB, 1 later repeat), a dotted eighth at 117 BPM (385 ms). Quoth and Quino-phec: none found. Echo times are good to about 4 ms; the 454 reading is 4.5 ms off two thirds of a beat, at the edge.
- **[JUDGMENT]** **Guess:** the same family as On's and Classics' echoes: a delay at a beat fraction chosen per loop (one beat, two thirds, three quarters).

**T3. Slow wobble and a tuning offset on the pads (09, 04, 01)**
- Quino-phec: the other stem wobbles about 14 cents at 2.5 Hz; If It Really Is Me 9 cents at 3.0 Hz with speed moves up to 18 cents in 4 s. The tool read a 12-cent wow as 13.7. Tuning by the mix tool: Polygon Window -36 cents, Quoth -32, If It Really Is Me +2, Quino-phec -7; across the album -48 to +40 (single-file readings within 8 cents are noise).
- **[JUDGMENT]** **Guess:** a 2.5 to 3 Hz wobble of 9 to 14 cents, and a per-loop tuning offset of up to a third of a semitone flat.

### Theory

**Th1. Chord rhythm as a dial: fast and off the beat against slow and locked (01 against 03)**
- Polygon Window: 89.1 BPM with triplet feel (off-beat at 0.67 of the beat), 6 beats a bar; the drum onsets repeat at 0.453, 0.673, 0.894 and 1.347 s, which are 2, 3, 4 and 6 steps of a 0.2245 s grid (so 89 BPM in triplets is the same grid as 133.7 straight; the mix tool's 136 agrees with the second). Key A mixolydian (margin 0.023, usable); bass A 59%, E 12%, C# 11%, D 8%, 0.82 notes a second. Chords change 56 times a minute (median chord 0.9 s, 28 distinct chords: Dmaj7, Emaj7, C#maj7, Fmaj7; names are a guess), and only 37% of the changes land on a beat against 45% by chance.
- Quoth: 139 BPM, a 4-beat bar of 1.73 s (drum-onset repeat strength 0.90, so the tempo octave is settled), chord changes 6 a minute (median 5.9 s) with 85% on a beat against 70% by chance: slow and beat-locked. Key margin 0.001, so the key is a guess (D phrygian); chord names a guess (D#maj7 24%, G#7 20%, Fmin7 18%, Dmin7 11%). The mix-level tempo (112.3) is four fifths of the real grid, as in Classics (117.5 against about 145).
- **[JUDGMENT]** **Guess:** chord changes per minute drawn from 6 to 56, with beat-locking linked to slowness; triplet feel as a swing setting of 0.67.

**Th2. Drone with a sus2 and major-seventh swap (09)**
- One root, A#: pitch classes A# 0.26, C 0.15, D 0.14, A 0.08, C# 0.07. Chords A#maj7 54%, A#sus2 36%, A# 5%, Cmin7 2% (15 changes a minute, 13 settled, median chord 4.3 s). Key margin 0.012: A# ionian is a guess. There is no beat (drums and bass stems empty), so the tool's 50 BPM, 0.80 swing and 21% of changes on a beat (chance 25%) mean nothing.
- **[JUDGMENT]** **Guess:** a fixed root note with the chord above it swapping between a sus2 and a major seventh every 4 s.

### Space
- Mono to wide on one record. If It Really Is Me is nearly mono: median width -29 dB (-30 to -27), correlation 1.00 in every band, drums -35 dB and other -28 dB. Quoth: median -9 dB (-12 to -7); sub -25 dB (correlation 1.00), 120 to 500 Hz -13, mids -8, 2 to 5 kHz -7, above 5 kHz -11; the drums stem (which is the whole track) -14 dB. Polygon Window: median -6 dB (-8 to -5); sub -25 dB (0.99), low-mids -7, mids -5, 2 to 5 kHz -3, air -3 (correlation 0.46); drums -14 dB, bass -23 dB (mono loss 0.0), other -5 dB (mono loss 1.2 dB). Quino-phec: median -5 dB (-8 to -2), correlation 0.28 to 0.84 in every band, mono loss 1.3 dB. At mix level the Wooden Thump Mix is nearly mono (-31 dB).
- Movement: Quino-phec's other stem has a pan range of 0.21 cycling every 4 s, which looks like the tool's window rather than an auto-pan (unconfirmed); the others stay put.
- **[JUDGMENT]** **Guess:** the sub under 120 Hz stays mono in every recipe; a mono mix (-29 dB) is one recipe and a half-wide mix (-5 to -9 dB) with a mono bottom is the other.

### Vibes (all guesses; weights lean x1.5 to x3 toward, x0.2 to x0.5 away, never 0)
- S1 gated sub: toward booming, hypnotic, minimal x2; away from airy, twinkling x0.4.
- S2 tuned kicks: toward booming, steady x2; away from restful, meditative x0.5.
- S3 low struck bell: toward mysterious, strange x1.5; away from bouncy x0.5.
- S4 drone pad: toward still, restful, meditative, hazy, foggy x2; away from energetic x0.3.
- T1 long floor-out: toward hypnotic, experimental x2; away from cozy, comfy x0.5.
- T2 beat-set echo: toward floating, spacious x1.5.
- Th1 fast and slow chord rhythm: slow and locked toward steady, still; fast and loose toward bouncy, strange x1.5.
- **[JUDGMENT]** Ears 8 settles all of this.

### Wholesome take
- Brightness flags: none. Energy above 2 kHz: Polygon Window 3%, Quino-phec 0%, Quoth 17%, If It Really Is Me 4%. No polarity flags (no band has a negative correlation).
- **[JUDGMENT]** **Guess:** take everything. Quoth's 17% above 2 kHz is the brightest; keep its top soft. The Wooden Thump Mix and Bike Pump Meets Bucket need a listen before anything is taken.

### For later
- If It Really Is Me, other-2 (164 notes): C2, mixed or noisy, falls 24 dB/s; hear 4:12.2, 0:48.3. **[JUDGMENT]** **Guess:** waits, a low rumble under the bell.
- Polygon Window, other-2 (235 notes): E3, harmonic, falls 31 dB/s; hear 5:20.4, 5:20.2. **[JUDGMENT]** **Guess:** waits, a mid pad in the closing seconds.
- Polygon Window, drums-2 (101 notes): low thump, slow swell, falls 86 dB/s, brightens; hear 0:53.1, 0:56.5. **[JUDGMENT]** **Guess:** waits, a second kick.
- Polygon Window, bass-1 (222 notes): A1; hear 3:22.4, 3:08.0. **[JUDGMENT]** **Guess:** waits; the "bell-like" label is under the 60 Hz limit.
- If It Really Is Me, drums-2 (39 notes): mid snap, 10 ms attack, falls 68 dB/s, brightens; hear 6:11.9, 6:03.7. **[JUDGMENT]** **Guess:** waits.
- Quoth, other-1 (397 notes, C4, falls 136 dB/s; hear 0:00.1, 2:20.7): the other stem holds 1% of the power, so this is probably leakage. **[JUDGMENT]** **Guess:** waits.
- The other three Quoth EP tracks, mix level only: Iketa (4:30, sub 83%, tail -7.0 dB a second, 35 pitch bends a minute against an album median of 15, tuning +40 cents); Quoth (Wooden Thump Mix) (7:57, 144 BPM, pulse 0.91, width -31 dB, 35 bends a minute); Bike Pump Meets Bucket (5:58, 123 BPM, 53 pitch bends a minute, wobble 7.3 cents). **[JUDGMENT]** **Guess:** Bike Pump Meets Bucket is the best candidate for a pitch-bend sound; Iketa for a long-tail one.
- Not deep-passed: Redruth School (2:44, 162 BPM, pulse 0.64 against a median of 0.83); Audax Powder (4:40, sub 78%); UT1 - dot (5:21, sub 81%, -17.4 LUFS); Supremacy II (4:08, 152 BPM, 7.5 chord changes a minute); Quixote (6:04, 23 bends a minute); Portreath Harbour (4:45, tail -5.3 dB a second); Untitled (6:25, tuning -48 cents). **[JUDGMENT]** **Guess:** waits.

### Reliability notes for this card
- Tempo: Polygon Window's 89 BPM (or 134 straight) and Quoth's 139 and If It Really Is Me's 117 are confirmed by the drum-onset grid and the echo times; Quino-phec's 50 BPM is meaningless (no beat). The mix-level tempo (the picker's) is off by a factor of 4/5 on Quoth (112.3 for 139).
- Key margins: Polygon Window 0.023 (usable), If It Really Is Me 0.042 (usable), Quino-phec 0.012, Quoth 0.001 (guesses). Chord changes on a beat against chance: 37 against 45, 21 against 25 (meaningless, no beat), 85 against 70, 48 against 59: only Quoth's is a lock.
- Empty or lumped stems: Quino-phec has empty drums, bass and vocals stems, so its "bass sweeps" (down 2.4 octaves at 0:03 and others) and "bass dips 1.8 dB" are void; Quoth's drums stem holds 99.8% of the power and its bass stem 0%; If It Really Is Me's bass stem is 0%, so its "bass dips 15.1 dB at the kick" is void.
- Top lines: 752, 673, 689 and 1,036 notes with ranges of 15 to 18 semitones are bleed in all four; none is a melody.
- Partials under 60 Hz and pitch under 40 Hz: unmeasurable and good to a semitone.
