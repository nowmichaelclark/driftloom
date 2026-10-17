# Ears cards, pile 2, first half

Ears 6 (Sonnet 5.5, medium), started 2026-10-18. Album cards from `tools/ears/album.py` (stage by stage), checked against the limits in `docs/EARS.md`. Lines marked **[JUDGMENT]** are for Ears 8; each carries a short first guess marked **Guess**. Machine time per album is on each card; the totals and the cost check are in `docs/ears/plan.md`.

**The stems on these cards did not come from Demucs.** The workspace's network policy blocks the Demucs weights (`dl.fbaipublicfiles.com` and Hugging Face both answer 403), so `album.py` quietly fell back to the harmonic/percussive split on its first run (I threw that run away). The UVR MDX-Net four-stem models (`kuielab_a_drums`, `_bass`, `_other`, `_vocals`, ONNX, from the TRvlvr model_repo GitHub release) do download, so `tools/ears/split.py` now has a middle step, `mdx`, between Demucs and the fallback (see the PR). They are real drums, bass, other and vocals stems, but a weaker separator than Demucs on this music: the drums stem swallows most of the sub (it holds 78 to 89% of its energy under 120 Hz on the dense tracks), the bass stem is thin or empty wherever the kick and the bass share the sub, and the vocals stem on an instrumental is hi-hat and pad bleed. Every card says which stem each number came from, and per-stem figures that rest on a near-empty stem are marked void. The mix-level numbers (loudness, bands, width, tuning, tempo from `pick`) do not depend on the separator.

| Album | Tracks (deep pass) | Machine time | Status |
|---|---|---|---|
| I Care Because You Do (1995) | 12 (4) | 4,352 s | done |
| Ventolin (EP) | 14 (4) | | |
| Donkey Rhubarb (EP) | 5 | | |
| Hangable Auto Bulb (AFX) | 10 | | |
| GAK (AFX) | 9 | | |
| Analogue Bubblebath 4 | 5 | | |

---

## Aphex Twin, I Care Because You Do (1995)

Ears 6, 2026-10-18, Sonnet 5.5. 12 tracks, all measured at mix level, 36 minutes. Deep pass on four, as the picker chose: the most typical, 01 Acrid Avid Jam Shred (7:38), and the three least like the rest, 07 Start as You Mean to Go On (6:05), 12 Next Heap With (4:43) and 10 Alberto Balsalm (5:11). The record is dark and low: the stems of the three drum tracks hold 55 to 83% of their energy under 120 Hz and only 1 to 3% above 2 kHz (loudness -15.3 to -8.5 LUFS). Next Heap With is the odd one: no drums at all.

Machine time: 4,352 s (72.5 min) for 23.6 minutes of deep-pass audio (1,417.5 s): pick 195 s (12 tracks, 16 s a track), MDX-Net split 3,495 s (2.47 times the audio; the four models are 0.41 times real time), measuring 642 s (events, techniques, theory and space, 115 to 232 s a track, 0.45 times the audio). The plan's formula (1.6 s per second of deep-pass audio plus 25 s a track) gives 2,568 s; the measured time is 1.7 times that, all of it the slower separator (Demucs would run at about 1.05 times the audio, so I expect the formula to hold with Demucs; not measured).

**Shared tracks.** No title on this album repeats another pile 2 album yet except two that the plan flags: 04 Icct Hedral (Edit), 367.1 s, -10.6 LUFS, and 05 Ventolin (Video Version), 269.2 s, -8.7 LUFS. Neither was picked for a deep pass here (the picker took the other four), so Donkey Rhubarb (Icct Hedral, Philip Glass orchestration and dry versions) and the Ventolin EP will compare against these two rows and give any twin its deep pass in their own cards. None of the 12 tracks matches a pile 1 Classics row (the list in `cards-pile-1.md`) in both length and loudness.

### The hook
- Sub-heavy and slow-moving: kicks tuned to notes (A#1, B1, G1, A1), a sub bass that sits on G (G phrygian) in the fastest track, a bass that outlines a D# major-seventh chord in the 94 BPM track, and one beatless, wide, low-mid drone with no drums.
- **[JUDGMENT]** One line a listener would say. **Guess:** a tuned sub-kick and sub bass under slow drifting pads, at 150 to 160 or 94 BPM, with the low end dropping out for a few seconds before it comes back.

### Sounds

**S1. Tuned sub thump (Start as You Mean to Go On): drums-1, 404 notes, 53%; drums-2, 96 notes, 19%**
- In synth terms: drums-1 is a low thump at 60 Hz (50 to 73), brightness 98 Hz, level -0.5 dB, correlation 0.98. The attack time of 165 ms is not believable for a kick (under 60 Hz the tool cannot see the attack well); the fall of 135 dB/s is. drums-2 is the long version: 60 Hz (59 to 62), falls 26 dB/s, brightness 159 Hz and darkening 0.67 octave over the note, level -0.5 dB. The drums stem holds 88% of the track's power and 89% of it is under 120 Hz; the bass stem holds 2% (void, see below), so the sub lives in the drums stem.
- Hear it at: drums-1 1:49.8 (A#1, 0.28 s, +32 dB alone), 1:56.8 (B1, 0.26 s, +28 dB alone), 3:24.2 (B1, 0.5 s, +26 dB alone); drums-2 1:26.6 (B1, 0.47 s, +28 dB alone), 2:03.2 (A#1, 0.47 s), 3:22.6 (A#1, 0.48 s).
- Check: A#1 is 58.3 Hz and B1 61.7 Hz, a semitone apart, and pitch under 60 Hz is good to about a semitone, so the kick is tuned to A#1 and B1 and moves between them. The tool's bass line for this track (B 0.47, A# 0.29 of the time, 0.58 notes a second) comes from the 2% bass stem, so it may be the kick's pitches: void.
- Building blocks: **[JUDGMENT]** **Guess:** the tuned sub-kick already on the list from On and Classics, with a short and a long version.

**S2. Sub bass on G with a rising brightness, and a ducking kick (Acrid Avid Jam Shred): bass-1, 473 notes, 28%**
- In synth terms: 49 Hz (41 to 49), harmonic share 1.00 (a clean sine-like sub), attack 107 ms, falls 32 dB/s, brightness 103 Hz rising 0.56 octave over the note, level -7.6 dB, centered (correlation 1.00). The bass stem holds 16% of the power and 96% of that is under 120 Hz. Its strongest lines are 48.6 Hz (G1, 49.0 Hz, 14 cents flat), 45.0 Hz and 41.1 Hz (E1, 41.2 Hz).
- Hear it at: 3:03.9 (D#1, 0.68 s, +5 dB alone), 5:32.2 (E1, 1.4 s, -3 dB alone), 2:04.7 (F#1, 1.27 s, -3 dB alone). None is clean (the best is +5 dB alone, the others are under the rest), so these are not good examples; listen for the sub under the kick instead.
- Check: the drums stem (76% of the power) has most of the low end, so the bass stem here is the part of the sub that is not kick. The tool's bass behavior (G 0.76 of bass time, steps 54%, 1.15 notes a second) is believable because the stem is real. Notes at 39 Hz and below are good to about a semitone only.
- Building blocks: **[JUDGMENT]** **Guess:** a sine-like sub bass pitched to the key, with a slow brightness rise of half an octave per note; closest to roundBass.

**S3. Beatless low-mid drone (Next Heap With): other-1, 192 notes, 38%; other-2, 158 notes, 37%**
- In synth terms: two overlapping low-mid pads. other-1 sits at 86 Hz (50 to 137), attack 257 ms, falls 13 dB/s, brightness 331 Hz, harmonic share 0.85, level -1.9 dB, left and right unrelated (correlation 0.42). other-2 sits at 108 Hz (58 to 188), attack 403 ms (slow swell), falls 8 dB/s, brightness 624 Hz, harmonic share 0.61, vibrato 2 cents, correlation 0.34. The other stem holds 78% of the power: 56% in 120 to 500 Hz and 41% in 0.5 to 2 kHz. Drums stem 0.0% of the power.
- Hear it at: other-1 2:02.1 (A1, 1.21 s, +45 dB alone), 2:20.7 (E2, 0.97 s, +47 dB alone), 2:27.8 (F2, 1.05 s, +41 dB alone); other-2 2:00.2 (E2, 0.76 s, +47 dB alone), 2:18.4 (F2, 0.8 s, +43 dB alone), 2:01.2 (A1, 1.01 s, +45 dB alone). These are clean (+41 to +47 dB alone) because nothing else plays.
- Check: the two groups are weak (silhouette 0.19, so the notes do not separate into two instruments): treat it as one blended, wide, slowly swelling pad. The vocals stem holds 9% of the power, 86% of it in 120 to 500 Hz: that is low pad bleed, or possibly a hummed pad, and I did not listen to decide. The drums stem is empty, so everything about the drums for this track (21% on, entries at 1:01.5 and so on) is void.
- Building blocks: **[JUDGMENT]** **Guess:** a wide, slow-attack low-mid pad with unrelated left and right (correlation 0.3 to 0.4); the closest existing sound is the pads, with the width and low register being the new part.

**S4. Right-leaning mid pad that stays when the low end leaves (Alberto Balsalm): other-1, 500 notes, 38%**
- In synth terms: D#3 area (158 Hz, 79 to 529), harmonic share 0.73, attack 96 ms, falls 18 dB/s, brightness 749 Hz, level -6.3 dB, leans right (pan 0.16 for the group, 0.35 for the whole other stem), correlation 0.53. The other stem holds 29% of the power, 78% of it in 0.5 to 2 kHz.
- Hear it at: 2:11.2 (G3, 0.19 s, +41 dB alone), 2:11.0 (G3, 0.16 s, +38 dB alone), 4:46.9 (D3, 0.5 s, +32 dB alone). The first two sit inside the low-end break at 2:09.5, which is why they are clean.
- Check: the group is weak (silhouette 0.17, one group), so this is a blended texture of several notes, not one instrument.
- Building blocks: **[JUDGMENT]** **Guess:** a mid pad panned a third to the right, kept apart from the sub; needs a listen.

**S5. Two kicks: a fast low thump at 48 Hz (Acrid Avid Jam Shred) and a 56 Hz kick that steps in pitch (Alberto Balsalm)**
- In synth terms: Acrid drums-1, 312 notes, 32%: 48 Hz (43 to 52), attack 29 ms, falls 39 dB/s, brightness 181 Hz, harmonic share 0.94, level -0.7 dB, correlation 1.00. Alberto drums-1, 376 notes, 32%: 56 Hz (54 to 62), attack 36 ms, falls 88 dB/s, brightness 101 Hz (75 to 542), correlation 0.99, pan -0.05. The drums stem holds 76% (Acrid) and 56% (Alberto) of the power.
- Hear it at: Acrid 6:52.4 (F1, 0.35 s, +20 dB alone), 6:52.3 (G#1), 1:26.2 (G#1); Alberto 3:57.8 (G1, 0.47 s, +37 dB alone), 3:35.9 (A1, 0.31 s, +36 dB alone), 3:57.7 (B1, 0.17 s, +37 dB alone).
- Check: Acrid's groups are weak (silhouette 0.20); Alberto's kick lands on G1, A1 and B1, so it moves in pitch and is not one fixed note. The same tuned-kick idea as S1.
- Building blocks: **[JUDGMENT]** **Guess:** the tuned sub-kick again; the new part is that the pitch steps from hit to hit by a whole tone.

### Techniques

**T1. A brightness rise, then the floor drops out in four-bar steps (Acrid Avid Jam Shred, 2:51 to 3:30)**
- Brightness of the mix rises 2.8 octaves from 2:51.5 to 3:01.0 (and 2.0 octaves at 2:29.0, 2.6 at 3:53.0); the low end (all stems summed, under 120 Hz) then goes 15 dB down for 3.5 s at 3:00.5, 3:06.5 and 3:12.5, for 2.5 s at 3:25.0 and 3:30.0. The first three are 6 s apart, which is four of the 1.486 s drum bars (5.94 s). The bass enters at 3:17.0.
- Check: the stems-only count in `techniques.json` finds two drops (3:07.0 for 3.0 s and 3:12.2 for 3.8 s, 0.29 a minute); the sums find five, so the stems-only count undercounts, as `docs/EARS.md` warns. The 6.0 s run at 7:32 is the tail. Brightness swings are whole-mix numbers on a drum-led track (45 sweeps in all), so they are mostly hats and fills; the 2.8-octave rise at 2:51 is large enough to read as a build.
- **[JUDGMENT]** **Guess:** a 3 to 4 s floor drop on the bar, repeated every four bars three times, after a rising bright build of about 10 s.

**T2. The sub ducks at the kick (Acrid Avid Jam Shred)**
- The bass stem dips 4.8 dB at the kick (peak 0 ms after it; kicks every 0.37 s) and the other stem 4.1 dB (peak 344 ms after). The tool reads about 70% of the true depth, so the real dip is about 7 dB for the bass and 6 dB for the pad. Start as You Mean to Go On reads 1.8 dB on both, but its bass stem is 2% of the power, so that number is void. Next Heap With (no kick) and Alberto Balsalm read under 1.5 dB.
- **[JUDGMENT]** **Guess:** a kick-locked duck of 5 to 7 dB on the sub and pad at 161 BPM, only in the fastest loops.

**T3. A tuned-up, slightly sharp mix (Alberto Balsalm) and slow wobbles**
- Alberto Balsalm reads +38 cents at mix level; its bass lines confirm it: 74.8 Hz is 32 cents above D2, 79.2 Hz is 31 above D#2, 88.8 Hz is 29 above F2 and 100.4 Hz is 42 above G2. So the whole piece is about a third of a semitone sharp. The other three tracks read +4 to +5 cents (noise). Elsewhere on the album the mix-level readings run from -20 (The Waxen Pith) and -18 (Come On You Slags!) to +38; none of those was deep-passed.
- Start as You Mean to Go On: the other stem wobbles 12 cents at 0.4 Hz (the tool read a 12-cent wow as 8.0 in calibration, so the true wobble may be near 18 cents). Next Heap With: the bass wobbles 13 cents at 1.4 Hz. The 10 cents at 1.2 Hz on Start's bass stem is void (2% stem). Acrid and Alberto: nothing at 4 cents or more.
- **[JUDGMENT]** **Guess:** a tuning offset draw of up to a third of a semitone, and a slow 0.4 Hz pad wobble of 10 to 20 cents on some loops.

**T4. A quarter-note drum echo (Start as You Mean to Go On), not claimed elsewhere**
- The drums echo at 399 ms (+6.2 dB, 2 later repeats), a quarter note at 152 BPM (395 ms) and an eighth at the tool's 76. Acrid's other stem echoes at 230 ms (+3.1 dB, one repeat), which is 0.62 of a beat and not a clean fraction at 161.5 BPM, so I do not claim it. Echo needs six or more notes ringing alone for 1.5 s, which a drums stem rarely gives; treat the 399 ms as unconfirmed until heard.
- **[JUDGMENT]** **Guess:** a quarter-note echo on a drum or percussion layer at about +3 to +6 dB; unconfirmed.

**T5. A 10 s break with the low end out (Alberto Balsalm, 2:09.5) and a track with no drums (Next Heap With)**
- Alberto: the low end is 15 dB down from 2:09.5 for 11 s (the tool's floor-drop count says 6.0 s; the sums say 11 s), while the right-leaning mid pad (S4) keeps going. More short drops follow at 3:37.5 (4.5 s), 3:52.5 (4.5 s), 4:22.5 (4.0 s) and 4:44.5 (4.0 s): in all 0.65 a minute on the stems and eight runs by the sums; the first and last runs (0:00 for 7.5 s, 5:08 to the end) are the intro and tail. The stem list agrees: drums out at 2:10.0, back at 2:20.0.
- Next Heap With: there are no drums, so the entries and exits the tool lists for drums (21% of the time) are leakage. Its low end is on only 68% of the time, so the floor-drop count is skipped (rule: under 75%).
- **[JUDGMENT]** **Guess:** a 10 s break where the low end leaves and one mid pad stays, once in the middle of a loop.

### Theory

**Th1. Two grids: 150 to 160 BPM and 94 BPM, and the tool's half-time**
- Drum-stem onset repeats: Acrid 1.486 s (0.83) with a beat every 0.372 s, which is 161.5 BPM as a 4-beat bar (the tool reads 81, half-time; the mix-level count agrees with 161.5); Start 1.591 s (0.58), 150.8 BPM as a 4-beat bar (the tool reads 76, half-time; the mix-level count gives 152.0), swung 0.62 of the beat (triplet feel); Alberto 1.277 s (0.72), 94 BPM as a 2-beat bar (the tool reads 94, picker 95.7), straight. Acrid and Alberto show many unrelated periodicities (3:2, 4:3, 5:2, 5:3), which the tool reads as irregular timing, not a cross-rhythm.
- Next Heap With has no beat (drums stem 0.0%), so its tool tempo (77) and picker tempo (117.5) are void.
- **[JUDGMENT]** **Guess:** a tempo draw of 150 to 162 (felt at half time as 75 to 81) or 94, with a triplet swing of 0.6 on the slower of the fast ones.

**Th2. A phrygian pedal on G, and a bass that outlines D#maj7 (Acrid Avid Jam Shred, Alberto Balsalm)**
- Acrid: the top five pitch classes are D# 0.13, F 0.12, G# 0.11, G 0.11, D 0.10, all inside G phrygian (G, G#, D#, F, D); key margin 0.051 (usable). The bass stem sits on G for 0.76 of its time (F 0.09, A# 0.07, F# 0.07). So the bass is a G pedal with the flat second (G#) and flat sixth (D#) above it. The chord names (Dmin7 20%, Csus2 17%, D#maj7 9%, Fmin7 9%; 29 changes a minute, 12 of them lasting 1.5 s or more) are a guess on dense music, and drums are 76% of the power, so some pitch classes may be kick pitches.
- Alberto: the bass stem's four main notes, D# 0.24, G 0.24, A# 0.14 and D 0.13, are 75% of its time and are exactly the chord tones of D#maj7 (D#, G, A#, D); the chord tool gives D#maj7 as the most common chord (20%), with 24% fourth and fifth leaps and 5% octaves in the bass. Key margin 0.008: a guess (D# major). 43 changes a minute but only 11 lasting 1.5 s or more; 43% of changes land on a beat against 47% by chance, so the chord timing is not beat-locked.
- Start as You Mean to Go On: the five biggest pitch classes (G 0.18, D 0.14, A 0.12, B 0.09, E 0.08) are 61% of the weight and form a G-major pentatonic; key margin 0.009 (the tool says B aeolian, a guess); bass void.
- **[JUDGMENT]** **Guess:** a bass that arpeggiates a major-seventh chord (root, third, fifth, seventh) at about one note a second, and a phrygian pedal draw with the flat second held over it.

**Th3. Long pad changes with no beat to hang on (Next Heap With)**
- The only track with real harmony and no kick: 29 changes a minute, 20 of them lasting 1.5 s or more, median chord 1.9 s (0.9 to 3.4 s), 26 distinct chords (C#maj7 13%, A5 10%, C#min7 9%, E5 9%). The bass stem (13% of the power) sits on C 0.64 of the time, A 0.17, C# 0.11. Key C major, margin 0.042, but the chord names lean on C# more than on C, so the key and the chord names disagree: the names are a guess. Only 38% of the changes land on a beat, the chance rate, which is what you expect when no beat is playing.
- **[JUDGMENT]** **Guess:** a beatless drone loop with the bass pedal on one note and a pad that changes chord every 2 s.

### Space
- Four spaces. Acrid is nearly mono: mix width median -21 dB (-30 to -9), sub -26 dB (correlation 0.99), air -26; the width is in the 120 Hz to 2 kHz bands (-12 dB) and in the other stem (width -7 dB, 8% of the power); drums -28 dB, bass -38 dB.
- Start is wide above the sub: median -4 dB (-20 to -2), sub -17 (0.96), low-mids -5 (0.54), mids -2 (0.23), 2 to 5 kHz -8, air -12; the other stem is at 0 dB width (pan -0.12, mono loss 2.9 dB, 14% of the power) over a drums stem at -17 dB.
- Next Heap With is wide in every band, including the sub: median -4 dB (-7 to -1), sub -7 dB (correlation 0.70), 2 to 5 kHz -3 (0.36). The other stem carries it (-4 dB, mono loss 1.5 dB); the bass stem is narrower (-11 dB).
- Alberto Balsalm is mid-wide with a mono top: median -7 dB (-13 to -5), sub -13 (0.93), mids -5 (0.55), 2 to 5 kHz -15, air -28 (1.00); the other stem leans right (pan +0.35, 35% of the power), the bass slightly left (-0.17).
- **[JUDGMENT]** **Guess:** a near-mono recipe (Acrid), a wide pad over a mono-ish sub (Start), a fully wide drone with a wide sub (Next Heap With, which breaks the keep-the-sub-mono rule of pile 1: a mono fold would thin it by 1.5 dB), and a right-leaning pad over a centered kick (Alberto).

### Vibes (all guesses; weights lean x1.5 to x3 toward, x0.2 to x0.5 away, never 0)
- S1 and S5 tuned sub-kick: toward booming, hypnotic, energetic x2; away from restful, still x0.4.
- S2 sub bass on G: toward hypnotic, mysterious x1.5; away from bright x0.5.
- S3 beatless wide drone: toward hazy, restful, meditative x2; away from energetic, bouncy x0.3.
- S4 right-leaning mid pad: toward strange, dreamy x1.5.
- T1 build then four-bar floor drops: toward energetic, hypnotic x1.5.
- T2 kick-locked duck: toward bouncy, energetic x1.5.
- Th2 major-seventh arpeggio bass: toward dreamy, warm x1.5.
- **[JUDGMENT]** Ears 8 settles all of this.

### Wholesome take
- Brightness flags: none. 1 to 3% of each deep-pass track's energy is above 2 kHz (Acrid 3%, Start 2%, Next Heap With 1%, Alberto 3%), and nothing has a polarity flag. The flags this record raises are about weight, not harshness: the stems of Acrid, Start and Alberto hold 81%, 83% and 55% of their energy under 120 Hz, so any use of the sub-kick or sub bass needs the low-end level guard.
- **[JUDGMENT]** **Guess:** take the tuned sub kick (at a modest level), the sub bass on G, the D#maj7 arpeggio bass, the beatless drone and the four-bar floor drops; the whole record is dark but not harsh. Leave the unconfirmed drum echo until heard.

### For later
- Acrid Avid Jam Shred, drums-2 (188 notes): low thump, soft attack, falls 54 dB/s; hear 4:47.8, 4:36.0. **[JUDGMENT]** **Guess:** waits, a second kick layer.
- Acrid Avid Jam Shred, other-1 (145 notes) and other-2 (233 notes): G2, slow swell, falls 13 dB/s; and G3, falls 164 dB/s (a short stab); hear 2:58.3, 3:48.0 and 7:08.4, 6:35.9. **[JUDGMENT]** **Guess:** waits, short mid stabs.
- Alberto Balsalm, bass-1 (177 notes): D#2, harmonic, vibrato about 13 cents at 3.3 Hz; hear 5:04.8, 5:06.1. **[JUDGMENT]** **Guess:** waits, a sub bass with vibrato.
- Alberto Balsalm, drums-2 (124 notes): mid snap, falls 233 dB/s; hear 3:58.4, 0:41.8. **[JUDGMENT]** **Guess:** waits, the snare-like hit.
- Next Heap With, bass-1 (315 notes): C2, harmonic, soft attack, falls 17 dB/s; hear 0:37.0, 0:45.7. **[JUDGMENT]** **Guess:** waits, the C pedal under S3.
- Start as You Mean to Go On, other-1 (163 notes, G3, slow swell) and bass-1 (325 notes, A1, bell-like falls 69 dB/s; the bass stem is 2% so it may be kick bleed); hear 2:47.1, 2:47.4 and 5:51.1, 1:37.9. **[JUDGMENT]** **Guess:** waits.
- Not given a deep pass (mix-level only): 04 Icct Hedral (Edit) (6:07, width -6.3 dB, 8.8 chord changes a minute, sub 25% of the energy: the widest, least bassy of the rest, and shared with Donkey Rhubarb); 05 Ventolin (Video Version) (4:29, -8.7 LUFS, the loudest, shared with Ventolin); 09 Mookid (3:51, width -25 dB, nearly mono, the least wobble 1.4); 06 Come On You Slags! (5:45, 103 BPM, tuning -18 cents); 02 The Waxen Pith (4:50, sub 85% of the energy, tuning -20); 11 Cow Cud Is a Twin (5:34, 161.5 BPM, 10.3 chord changes a minute). **[JUDGMENT]** **Guess:** Icct Hedral and Mookid are the two worth a listen.

### Reliability notes for this card
- Separator: MDX-Net, not Demucs (see the top of this file). The drums stem holds the sub on Acrid (78% under 120 Hz), Start (89%) and Alberto (78%).
- Empty stems: Start's bass stem holds 2% of the power, so its bass line (B and A#), its bass pump (1.8 dB), its bass wobble (10 cents at 1.2 Hz) and its bass-stem floor behavior are void. Next Heap With's drums stem holds 0.0%, so its drum entries and exits, drum grid and tempo are void. Alberto's vocals stem holds 0.0% and the vocals stems of the other three hold 0.8 to 9%, mostly hats (Acrid, Start: 31 to 38% above 5 kHz) or low pad (Next Heap With): not voices, and no words anywhere.
- Tempo: the tool's meter is half the drum grid on Acrid (81 against 161.5) and Start (76 against 150.8); on Alberto it agrees with the grid (94). Bar length: the tool's 8-beat and 4-beat bars on Acrid and Start do not match the drum bars (1.486 s and 1.591 s); I read the bars off the drum stem.
- Key margins: Acrid 0.051, Next Heap With 0.042 (usable but its chord names disagree with it), Start 0.009 and Alberto 0.008 (guesses). Chord names on all four are guesses on dense music. Tuning readings run from -20 to +38 cents across the 12 mix-level rows.
- Top lines: ranges of 12 to 23 semitones with 13 to 39% leaps; Start's phrases of 49 notes are a sign of bleed. None is trustworthy on tracks where the drums or pad are loud. Pitch under 60 Hz: partial-off labels are not measurable; kick pitches are good to about a semitone.
- Separator and floor drops: the stems-only count undercounts (Acrid 2 against 5, Alberto 3 against 8), as the docs say.

---
