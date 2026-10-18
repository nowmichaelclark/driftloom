# Ears cards, pile 2, first half

Ears 6 (Sonnet 5.5, medium), started 2026-10-18. Album cards from `tools/ears/album.py` (stage by stage), checked against the limits in `docs/EARS.md`. Lines marked **[JUDGMENT]** are for Ears 8; each carries a short first guess marked **Guess**. Machine time per album is on each card; the totals and the cost check are in `docs/ears/plan.md`.

**The stems on these cards did not come from Demucs.** The workspace's network policy blocks the Demucs weights (`dl.fbaipublicfiles.com` and Hugging Face both answer 403), so `album.py` quietly fell back to the harmonic/percussive split on its first run (I threw that run away). The UVR MDX-Net four-stem models (`kuielab_a_drums`, `_bass`, `_other`, `_vocals`, ONNX, from the TRvlvr model_repo GitHub release) do download, so `tools/ears/split.py` now has a middle step, `mdx`, between Demucs and the fallback (see the PR). They are real drums, bass, other and vocals stems, but a weaker separator than Demucs on this music: the drums stem swallows most of the sub (it holds 78 to 89% of its energy under 120 Hz on the dense tracks), the bass stem is thin or empty wherever the kick and the bass share the sub, and the vocals stem on an instrumental is hi-hat and pad bleed. Every card says which stem each number came from, and per-stem figures that rest on a near-empty stem are marked void. The mix-level numbers (loudness, bands, width, tuning, tempo from `pick`) do not depend on the separator.

| Album | Tracks (deep pass) | Machine time | Status |
|---|---|---|---|
| I Care Because You Do (1995) | 12 (4) | 4,352 s | done |
| Ventolin (EP) | 14 (4) | 2,620 s | done |
| Donkey Rhubarb (EP) | 5 (4) | 4,170 s | done |
| Hangable Auto Bulb (AFX) | 10 (3) | 2,602 s | done |
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

## Aphex Twin, Ventolin (EP, 14 mixes)

Ears 6, 2026-10-18, Sonnet 5.5. 14 mixes, all measured at mix level, 59 minutes. Deep pass on four, as the picker chose (this is an 11-or-more-track set, so one typical plus three unlike): the most typical, the crowsmengegus mix (5:53), and the three least like the rest, the praze-an-beeble mix (3:22), the asthma beats mix (1:40) and hilow (2:56). It is a bass record: in the two tracks where the bass stem is real it holds 44% (crowsmengegus) and 72% (hilow) of the power, with 94 to 98% of that under 120 Hz, and hilow has 88% of its energy under 120 Hz.

Machine time: 2,620 s (43.7 min) of compute for 13.8 minutes of deep-pass audio (830.3 s): pick 255 s (14 tracks, 18 s a track), MDX-Net split 2,048 s (2.47 times the audio), measuring 317 s (0.38 times the audio). The plan's formula gives 1,678 s, so this is 1.56 times it, again all separator. Wall clock was 3,907 s: the workspace restarted at 04:51 and the second split was lost and redone (about 20 minutes), which is not counted above.

**Shared tracks.** ICBYD's 05 Ventolin (Video Version) is 269.2 s and -8.7 LUFS; no mix on this EP is within 3 s of that length (the closest are salbutamol 346.6 s, -8.4 LUFS, and the Cylob mix 306.0 s). Salbutamol, wheeze and the short ventolin1 un e,ru,ec+2 (81.5 s) share its mix tempo of 143.6 BPM, and salbutamol is within 0.3 LU of its loudness, so the Video Version is probably an edit of the salbutamol mix; I did not listen to compare. None of the 14 matches a pile 1 Classics row. The Video Version therefore has no deep pass anywhere in the pile, and it stays a mix-level row on both cards.

### The hook
- A bass-led record: sine-like bass notes tuned to a line that moves (A#1, C2, D2, D#2, E2, F2 in hilow; A1 to D2 in crowsmengegus), kicks and ticks over them, near-mono mixes, and in two of the four tracks the low end leaves for a few seconds at a time in a regular pattern.
- **[JUDGMENT]** One line a listener would say. **Guess:** a rolling sine-wave bass at 86 or 136 BPM under dry ticks, with the sub dropping out for 3 to 4 s every few bars.

### Sounds

**S1. Sine-like bass notes that move over a fifth or so (hilow): bass-1, 176 notes, 49%; bass-2, 104 notes, 24%**
- In synth terms: bass-1 sits at 83 Hz (66 to 88), attack 52 ms, falls 21 dB/s, brightness 83 Hz (the same as its register: no overtones to speak of), no change over the note, level -1.4 dB, centered (correlation 1.00). bass-2 is a longer boom at 58 Hz (54 to 59), attack 186 ms, falls 50 dB/s, brightness 69 Hz, level -1.5 dB. The bass stem holds 72% of the power, 98% under 120 Hz.
- Hear it at: bass-1 0:20.4 (F2, 0.75 s, +22 dB alone), 1:05.2 (F2, 0.43 s, +22 dB alone), 1:16.0 (F2, 0.76 s, +21 dB alone); bass-2 0:15.6 (A1, 0.51 s, +23 dB alone), 0:18.4 (A#1, 0.47 s, +22 dB alone), 0:07.3 (A#1, 0.51 s, +19 dB alone).
- Check: the bass stem's strongest lines are 58.2 Hz (A#1, 2 cents flat), 65.0 (C2), 73.5 (D2), 78.2 (D#2, 9 cents sharp), 82.7 (E2) and 87.6 (F2): six notes within an octave, all within 3 dB of each other, so the line walks. It is real, not a subharmonic misread (the stem is 72% of the power and the lines are ratios of the same scale, not 1:2).
- Building blocks: **[JUDGMENT]** **Guess:** roundBass or a plain sine sub, pitched to a walking line, with a short boom version at 58 Hz.

**S2. Bass with a short fall, over a ducking kick (crowsmengegus): bass-1, 351 notes, 35%**
- In synth terms: 66 Hz (55 to 74), attack 51 ms, falls 48 dB/s, brightness 77 Hz, level -2.7 dB, centered. The bass stem holds 44% of the track's power and 94% of it is under 120 Hz; the drums stem holds 24% and is 91% under 120 Hz.
- Hear it at: 3:28.2 (C#2, 1.11 s, +10 dB alone), 4:30.9 (C#2, 1.94 s, +9 dB alone), 3:27.3 (D2, 0.85 s, +11 dB alone). Only +9 to +11 dB alone, so these are not clean.
- Check: bass lines at 73.4 Hz (D2, exact), 61.7 (B1, exact), 65.5 (C2), 69.3 (C#2), 55.6 (A1, 19 cents sharp): the line moves among A, B, C, C#, D (1.03 notes a second, steps 37%, thirds 26%, fourths and fifths 22%).
- Building blocks: **[JUDGMENT]** **Guess:** the same sub bass as S1 with a faster fall; "tuned bass line" and "tuned sub-kick" are already on the list.

**S3. A mid tone the separator calls vocals, and wide ticks (praze-an-beeble): vocals-1, 233 notes, 30%; drums-1, 309 notes, 27%**
- In synth terms: vocals-1 is a bell-like tone around 154 Hz (48 to 315), partials off whole multiples, attack 48 ms, falls 311 dB/s (a short pluck), brightness 666 Hz (267 to 1,384), level -0.0 dB, centered. The vocals stem holds 51% of the power: 61% of it in 120 to 500 Hz and 36% in 0.5 to 2 kHz. drums-1 is a high tick, fundamental 52 Hz (37 to 741: the group fuses a thump and ticks), attack 30 ms, falls 123 dB/s, brightness 2.8 kHz (1.2 to 3.9), level -0.9 dB, correlation 0.89, wide.
- Hear it at: vocals-1 3:20.0 (G3, 0.32 s, +49 dB alone), 3:19.4 (G3, 0.45 s, +48 dB alone), 3:18.8 (F2, 0.96 s, +48 dB alone); drums-1 0:10.4 (G1, 0.26 s, +39 dB alone), 0:11.2 (A1, 0.16 s, +37 dB alone), 0:11.3 (A1, 0.22 s, +32 dB alone).
- Check: the vocals stem on an instrumental is a guess; with half the power and a 120 Hz to 2 kHz tone it is the track's main pitched voice (a plucked or bell-like lead), put in the wrong stem, and I did not listen to decide whether it is voice-like. It never has words in it that I can tell. The examples at 3:18 to 3:20 are clean because the low end is out and nothing else plays.
- Building blocks: **[JUDGMENT]** **Guess:** a short bell-like pluck at G3 (falls 300 dB/s) and a wide tick-and-thump layer; the pluck is close to the existing bell/pluck sounds.

**S4. A mono tick kit and a bell pad (asthma beats): drums-1, 307 notes, 23%; other-1, 250 notes, 25%**
- In synth terms: drums-1 is a bright tick, fundamental 57 Hz (36 to 1,212: fused), attack 12 ms, falls 212 dB/s, brightness 4.8 kHz (1.8 to 6.8), level -9.8 dB (-18.9 to -0.8), correlation 1.00. other-1 is a bell-like pad around B2 (123 Hz, 77 to 312), attack 125 ms, falls 13 dB/s, brightness 542 Hz, level -6.9 dB. The drums stem holds 70% of the power, 73% under 120 Hz, 5% above 5 kHz.
- Hear it at: drums-1 0:29.5 (C#2, 0.21 s, +22 dB alone), 0:30.0 (E2, 0.15 s, +22 dB alone), 0:28.9 (A#1, 0.51 s, +18 dB alone); other-1 0:56.5 (A#2, 0.26 s, +11 dB alone), 1:13.6 (D3, 0.12 s, +12 dB alone), 0:55.1 (D#2, 0.48 s, +10 dB alone; not clean).
- Check: the whole file is mono (every band correlation 1.00; width -33 to -27 dB), so this is a mono recording or fold. The bell pad's partials are 77 to 312 Hz, near the tool's lower limit, so the bell label is soft.
- Building blocks: **[JUDGMENT]** **Guess:** bright mono ticks (closer to hats than to a kick) over a bell pad; a harsh-leaning sound at 4.8 kHz brightness, so it waits behind its vibes.

### Techniques

**T1. The low end leaves in short regular gaps (crowsmengegus and praze-an-beeble)**
- crowsmengegus: sums of the stems under 120 Hz go 15 dB down for 3.0 to 4.5 s at 0:10.5, 0:15.5, 0:21.5 and 0:27.0 (5 to 6 s apart) and again at 1:13.5, 1:19.0, 1:24.5, 1:30.0 (3.0, 3.5, 2.5, 2.5 s, 5.5 s apart), with a 7.0 s one at 1:36.5; from 5:04.0 the low end is out for the last 48.5 s. The tool's floor-drop count (1.71 a minute, from the kick and bass stems only) lists most of them (0:21.8, 0:27.0, 1:13.5, 1:18.8, 1:24.5, 1:30.0, 4:44.5 for 2.5 s, 5:03.8 for 6.2 s). The low end is on 71% of the time, below the 75% line, because of the 8 s intro and the 48 s ending, so the count is a floor by the docs' rule; I trust the repeated 5.5 s pattern because the sums and the stems agree on it.
- praze-an-beeble: the low end is on only 39% of the time (the bass stem is empty), so the tool skips the count. The sums show 2.5 to 3.0 s gaps about every 11 s from 0:03 to 2:06 (0:03, 0:14.5, 0:25.5, 0:36.5, then 0:59, 1:10, 1:21.5, 1:32.5, 1:43.5, 1:55, 2:06), then long gaps: 2:15 for 7.5 s, 2:23.5 for 25.5 s, 2:49.5 for 7.5 s, 2:59 for 10.5 s, 3:10 for 11.5 s.
- **[JUDGMENT]** **Guess:** a 3 to 4 s floor drop every 5.5 s for four times in a row, in a block, then back; and a pulse with the low end off for 3 s every 11 s.

**T2. Kick-locked ducking (crowsmengegus, hilow, asthma beats)**
- Bass dips 3.8 dB at the kick in crowsmengegus (kick every 0.44 s), 3.4 dB in hilow (kick every 0.35 s, peak 0 ms after) and 4.2 dB in asthma beats (peak 150 ms after). The tool reads about 70% of the true depth, so the real dip is about 5 to 6 dB. praze-an-beeble's reading (other dips 6.7 dB, kick every 0.66 s) does not fit its 0.348 s beat, so I do not trust it; its bass stem is empty.
- **[JUDGMENT]** **Guess:** a 5 to 6 dB duck on the bass at the kick, in loops at 86 BPM and above.

**T3. A slow tuning wobble on the pad (hilow, crowsmengegus)**
- hilow: the other stem wobbles 16 cents at 0.4 Hz (the tool read a 12-cent wow as 8.0 in calibration, so the real wobble is nearer 24 cents). crowsmengegus: other 8 cents at 1.0 Hz and bass 12 cents at 0.7 Hz; praze-an-beeble's bass 5 cents at 0.8 Hz is void (empty stem); asthma beats: none. Mix-level tuning: asthma beats +21, ventolin1 un e,ru,ec+2 -43 (81.5 s, not deep-passed), crowsmengegus -18, praze-an-beeble -19; the crowsmengegus bass lines come out at 0 to +19 cents, so its -18 is mix-level noise (pile 1 readings ran from -48 to +45).
- **[JUDGMENT]** **Guess:** a 0.4 Hz pad wobble of 15 to 25 cents on some loops.

**T4. Sections, and echoes I do not claim**
- crowsmengegus has 14 section boundaries (a median of 20 s), with drums and vocals coming in at 0:07.5 and a big whole-mix brightness rise of 3.6 octaves over the first 11 s and a fall of 3.2 octaves at 1:37.5 to 1:43.5, when drums, bass and pad go out together (1:37.5 and 1:38.0) and come back at 1:42.5 to 1:43. asthma beats has a breakdown: the pad leaves at 0:27.5, the mix falls 4.2 octaves at 0:30 and comes back up 2.8 octaves from 0:35.5, and the pad returns at 0:46.
- Echoes: asthma beats other 180 ms (+4.5 dB, 2 later repeats), which is a sixteenth note at 90.6 BPM (165 ms) but 9% off, so not claimed; the tool's drum echo at 664 ms (+21.4 dB) is the beat itself (the beat is 0.662 s) and is not an echo. None found elsewhere.
- **[JUDGMENT]** **Guess:** a whole-mix brightness arc of about three octaves over 10 s at the start of a loop, and a 15 s breakdown where the pad leaves.

### Theory

**Th1. Tempos: 136, 172, 90.6 and 86, and a possible three against four (asthma beats)**
- Beat grids from the drum stems: crowsmengegus 0.441 s (136 BPM; the tool says 68, half-time; the picker says 136.0), praze-an-beeble 0.348 s (172.4 BPM; tool 86, picker 172.3), asthma beats 0.662 s (90.6 BPM; tool 90, picker 89.1), hilow 0.697 s (86.1 BPM; tool 86, picker 86.1). Swing (off-beat at 0.75 of the beat) in asthma beats and hilow is a late hit rather than a swing, as in pile 1.
- asthma beats also shows 0.499 s (strength 0.58) next to the 0.662 s beat, and both land together at 1.997 s (0.73): four ticks of 0.499 against three beats of 0.666 in 2 s, which would be a 4:3 cross-rhythm. The tool claims no cross-rhythm there, and I have not listened, so it is a lead for a listen, not a finding.
- **[JUDGMENT]** **Guess:** a tempo draw of 86 to 90 or 136 to 172; the 4:3 pattern is worth Ears 8 hearing before it goes on the list.

**Th2. Major sevenths again, and a walking bass (praze-an-beeble, asthma beats, hilow)**
- praze-an-beeble: Emaj7 12%, C#maj7 7%, F#maj7 7%, F#sus2 6% of the time, 39 changes a minute (15 lasting 1.5 s or more), 56% of changes on a beat against 43% by chance, so the chord changes are beat-locked; but the pitch classes are flat (every one near 0.09; key margin 0.001), so there is no key to read, and the drums and the vocals-stem tone are what the pitch tool hears.
- asthma beats: C#maj7 34%, F#min7 19%, G#maj7 9%, Dmaj7 6% (26 changes a minute, 14 lasting 1.5 s or more, 45% on a beat = chance); key margin 0.005, a guess. The same shape as Classics, a long major seventh with minor sevenths beside it.
- hilow: F5 24%, D5 13%, G#5 12%, A# 12% (fifths, not full chords; 42 changes a minute, 15 settled, 33% on a beat against 43% by chance, so less beat-locked than chance); bass F 0.20, D 0.18, D# 0.16, A 0.14, repeats 45%, 1.43 notes a second; key F dorian, margin 0.017, a guess that fits an F pentatonic.
- crowsmengegus: key C lydian, margin 0.011, a guess; chord names (A#7 22%, Csus2 21%) at 38 changes a minute but only 8 lasting 1.5 s or more, and the median chord is 0.9 s: far too fast for the bass line to be playing them, so the names are the pad's rapid movement or a mis-read.
- **[JUDGMENT]** **Guess:** a bass line that walks six scale notes within an octave at 1 to 1.4 notes a second, over fifths or sevenths; major-seventh chords with a minor-seventh neighbor for the pad.

### Space
- Four spaces. crowsmengegus is near-mono with a wider pad: median -18 dB (-28 to -7), sub -22 (0.99), 120 to 500 Hz -9 (0.78), mids -14, 2 to 5 kHz -18; bass and drums -23 and -21, the other stem -9.
- praze-an-beeble is the widest, with a wide sub: median -13 dB (-25 to -9), sub -6 dB (correlation 0.80), low-mids -15, mids -10, 2 to 5 kHz -11, air -12; the drums stem is wide (-8 dB, pan +0.35, widest in the sub at -6 dB, mono loss 0.6 dB) and the vocals-stem tone is narrow (-22 dB). Its drums pan range is 0.46 and the other stem's 0.27, so the image moves.
- asthma beats is mono: width -29 dB median (-30 to -27) and correlation 1.00 in every band, drums -30, bass -39, other -33.
- hilow is near-mono at the bottom and wider above it: median -11 dB (-14 to -7), sub -20 (0.98), low-mids -10, mids -11, 2 to 5 kHz -21, air -25; the other stem is -9 dB (mono loss 0.5 dB), bass -21, drums -18.
- **[JUDGMENT]** **Guess:** three recipes: a mono file for the ticks (asthma beats); a mono-ish low end with a wider pad (crowsmengegus, hilow); and a wide moving drum image with a wide sub (praze-an-beeble), where the mono fold would cost 0.6 dB on the drums, so the wide sub is a small risk.

### Vibes (all guesses; weights lean x1.5 to x3 toward, x0.2 to x0.5 away, never 0)
- S1 and S2 sine-like walking bass: toward hypnotic, warm, cozy x2; away from bright, energetic x0.5.
- S3 bell-like pluck over wide ticks: toward dreamy, strange x1.5.
- S4 mono tick kit: toward strange, experimental x2; away from cozy, tender x0.3 (harsh-leaning at 4.8 kHz).
- T1 regular floor-drop gaps: toward hypnotic, minimal x1.5.
- T2 bass duck: toward bouncy x1.5.
- Th1 4:3 cross-rhythm if confirmed: toward strange, hypnotic x2.
- **[JUDGMENT]** Ears 8 settles all of this.

### Wholesome take
- Brightness flags: praze-an-beeble has 10% of its energy above 2 kHz by the mix (16% by the stems sum; the two measures differ and I did not find out why), asthma beats 8% (6% by the stems), crowsmengegus 6%, hilow 1%. Not harsh. asthma beats' tick kit has a brightness of 4.8 kHz and a 5% share above 5 kHz in the drums stem, the one bright layer. No polarity flag; praze-an-beeble's sub is wide (-6 dB, correlation 0.80), but it holds only 12% of the track's energy.
- **[JUDGMENT]** **Guess:** take the sine-like walking bass, the duck, the regular floor drops and the pad wobble; take the mono ticks only at a gentle level behind strange or experimental vibes; the wide sub of praze-an-beeble needs the stereo guard.

### For later
- praze-an-beeble, other-1 (353 notes): C3, mixed or noisy, falls 94 dB/s; hear 2:54.7, 0:30.7. **[JUDGMENT]** **Guess:** waits.
- praze-an-beeble, drums-2 (191 notes): mid snap, falls 48 dB/s, leans right (+0.21); hear 0:06.3, 0:06.2. **[JUDGMENT]** **Guess:** waits, a right-hand snap.
- asthma beats, bass-1 (332 notes): D1, partials off whole multiples (below 60 Hz this is not measurable), falls 34 dB/s; hear 0:38.7, 0:37.9. **[JUDGMENT]** **Guess:** waits, a very low bass; the bell label is the tool's limit.
- asthma beats, drums-2 (88 notes): low thump, brightens; hear 1:27.5, 0:01.4. **[JUDGMENT]** **Guess:** waits.
- hilow, drums-1 (250 notes): low thump, falls 164 dB/s, brightens; hear 0:32.4, 0:43.5. **[JUDGMENT]** **Guess:** waits, the kick under the bass.
- crowsmengegus, drums-1 (267 notes): low thump, falls 230 dB/s, brightens; hear 4:07.5, 1:07.4. **[JUDGMENT]** **Guess:** waits.
- crowsmengegus, vocals-1 and vocals-2 (164 and 167 notes): D#4 and C#6, bell-like; hear 5:44.8 and 5:21.4: the separator's guess on an instrumental, probably the pad's top. **[JUDGMENT]** **Guess:** waits; never as words.
- Not given a deep pass (mix-level only): plain-an-gwarry mix (4:38, 161.5 BPM, sub 71%, width -32 dB near mono, 14.5 chord changes a minute); the coppice mix (4:36, 20.7 chord changes a minute, the most); deep gong mix (6:21, sub 80%, 92 BPM, 11 chord changes a minute); salbutamol mix (5:47, -8.4 LUFS, the loudest, 143.6 BPM, related to the Video Version); wheeze mix (7:14, 143.6 BPM, sub 65%, the longest); Cylob mix (5:06, 71.8 BPM, no chord changes at all); carharrack mix (2:54, 95.7 BPM); marazanvose mix (3:21, 112 BPM, sub 30%, the least bass); probus mix (4:16, 136 BPM, width -6 dB, the widest); ventolin1 un e,ru,ec+2 (1:21, tuning -43 cents, 0 chord changes). **[JUDGMENT]** **Guess:** the Cylob mix (a long track with no chord change) and the coppice mix (the fastest chord movement) are the two to hear if Ears 8 wants more.

### Reliability notes for this card
- Separator: MDX-Net, not Demucs (see the top of this file). The sub lives in the drums stem on crowsmengegus (91% under 120 Hz) and hilow (88%), asthma beats (73%).
- Empty or odd stems: praze-an-beeble's bass stem holds 0.0% of the power, so its bass behavior (A 0.52, C 0.44; 0.10 notes a second), bass pump (2.4 dB) and bass wobble (5 cents) are void, and its vocals stem holds 51%, which is a pitched tone and not a voice (no words). hilow's vocals stem holds 0.0%. The vocals stems of crowsmengegus (8%) and asthma beats (1.4%) are hats and pad bleed.
- Tempo: the tool's meter is half the drum grid on crowsmengegus (68 against 136) and praze-an-beeble (86 against 172); on asthma beats and hilow it agrees.
- Key margins: crowsmengegus 0.011, praze-an-beeble 0.001, asthma beats 0.005, hilow 0.017: all guesses. Chord names on all four are guesses on dense music, and the chord changes of crowsmengegus (0.9 s median) are faster than the 0.5 s limit allows trusting.
- Top lines: ranges of 10 to 24 semitones; asthma beats' phrase of 98 notes with arch 100% and praze-an-beeble's 40% leaps are a sign of bleed. Not trustworthy.
- Floor drops: crowsmengegus is on 71% of the time and praze-an-beeble on 39% (the rule skips under 75%); I read the sums instead, which is why T1 gives times. Pitch under 60 Hz: partial-off labels are not measurable.
- A workspace restart at 04:51 killed the run; the second split was redone from scratch and the first album's results were unaffected.

---

## Aphex Twin, Donkey Rhubarb (EP, 5 tracks)

Ears 6, 2026-10-18, Sonnet 5.5. 5 tracks, all measured at mix level; an EP of 6 or fewer, so the plan says every track gets the deep pass, but I gave four: 01 Donkey Rhubarb (6:08), 02 Vaz Deferenz (5:50), 03 Icct Hedral (Philip Glass Orchestration) (8:00) and 04 Pancake Lizard (4:31). Track 05 (Icct Hedral, Phillip Glass dry version plus bonus DAT glitches) is the same audio as 03 in the copy I have (see below), so it has no pass of its own. It is a record of four different things: two near-mono drum tracks that share one 1.718 s bar, a wide beatless orchestral-synth piece, and a bass-led groove in G mixolydian with big filter-like swings.

Machine time: 4,170 s (69.5 min) for 24.5 minutes of deep-pass audio (1,469.5 s): pick 150 s (5 tracks, 30 s a track), MDX-Net split 3,415 s (2.32 times the audio), measuring 605 s (0.41 times the audio). The plan's formula gives 2,476 s, so this is 1.68 times it, all separator. (I killed the run 19 s into track 05.)

**Shared tracks.** Tracks 03 and 05 decode to the identical audio in the files I have: part00 and part01 give the same MD5 for the decoded streams (c36959b1... and 001e6965...), and the picker rows are identical to every digit (480.0 s, -14.6 LUFS, same bands). In both cases the third piece (about 2 s) is missing from my copy: two 21 KB tail files came back inline from the Drive connector and were not saved. So 05's name promises a dry version and bonus DAT glitches that could only be in that missing tail (or the Drive file is a duplicate upload); **Mikey: it is worth checking in Drive whether the two files really differ.** I gave the audio one pass, under 03. ICBYD's 04 Icct Hedral (Edit) is 367.1 s and -10.6 LUFS against 03's 480 s and -14.6, wider than most (width -6.3 dB against -3.8), with 8.8 chord changes a minute against 25.4: not a twin, probably a shorter, louder edit of the same piece; both report a mix tempo of 143.6 BPM, which is the picker's, not a finding. I did not listen to compare. No other title repeats.

### The hook
- Mono drum tracks where the sub kick lands on G1/A1 and a pumping dip of 7 dB or more shows at the kick; a wide, beatless synth-orchestra piece with a bass that moves between F and F# (a phrygian flat second); a bass-led groove in G mixolydian with 3 to 4 octave brightness swings every 10 to 20 s.
- **[JUDGMENT]** One line a listener would say. **Guess:** dry mono drums with a deep sub kick, then a wide string-like pad piece with a bass that leans on the flat second.

### Sounds

**S1. Two sub-kick layers on G1 (Donkey Rhubarb): drums-1, 217 notes, 28%; drums-2, 283 notes, 23%**
- In synth terms: drums-1 is a thump at 50 Hz (44 to 51), attack 33 ms, falls 128 dB/s, brightness 111 Hz and rising 1.15 octaves over the note, level -0.7 dB, correlation 1.00. drums-2 sits at 53 Hz (45 to 61), attack 39 ms, falls 74 dB/s, brightness 199 Hz, level -0.4 dB. The drums stem holds 71% of the power and 83% of it is under 120 Hz; the bass stem holds 5%.
- Hear it at: drums-1 2:26.3 (G1, 0.32 s, +25 dB alone), 5:30.6 (E2, 0.13 s, +24 dB alone), 3:14.2 (G1, 0.36 s, +19 dB alone); drums-2 5:07.1 (G1, 0.21 s, +29 dB alone), 5:33.2 (B1, 0.11 s, +29 dB alone), 5:07.1 (A#1, 0.13 s, +29 dB alone).
- Check: pitch under 60 Hz is good to about a semitone, so G1 (49 Hz) is the kick's home with A#1 and B1 above it. The whole file is mono (correlation 1.00 in every band, width -29 dB) at +43 cents on the mix-level tuning reading, which is the pile 1 range (-48 to +45) and a quarter-tone-ish offset; the kick pitches (G1) are not tested against it.
- Building blocks: **[JUDGMENT]** **Guess:** the tuned sub-kick again, as two layers: a short fast-falling one that brightens, and a longer plain one.

**S2. A sub boom on A1 and F#1 (Vaz Deferenz): drums-1, 401 notes, 59%**
- In synth terms: 55 Hz (46 to 60), attack 148 ms (slow, so a boom, not a click), falls 46 dB/s, brightness 92 Hz, level -0.4 dB, centered. The drums stem holds 80% of the power, 87% under 120 Hz; the bass stem holds 2% (void).
- Hear it at: 5:22.2 (A1, 0.41 s, +19 dB alone), 5:19.6 (F#1, 0.41 s, +18 dB alone), 5:20.4 (A1, 0.43 s, +16 dB alone). All three are in the last 30 seconds, where the track thins out.
- Building blocks: **[JUDGMENT]** **Guess:** a tuned boom with a slow attack on two notes a minor third apart.

**S3. A wide, noisy mid pad over a walking bass, no drums (Icct Hedral, Philip Glass Orchestration): other-1, 388 notes, 46%; bass-1, 392 notes, 34%**
- In synth terms: other-1 sits at 105 Hz (58 to 174), attack 127 ms, falls 21 dB/s, brightness 632 Hz, level -2.3 dB, left and right related only partly (correlation 0.41). The group is weak (silhouette 0.21): a blended texture of overlapping parts. bass-1 sits at 55 Hz (33 to 86), attack 54 ms, falls 27 dB/s, brightness 97 Hz, level -5.7 dB, correlation 0.86. The other stem holds 68% of the power, the bass 26%, the drums 1.3%.
- Hear it at: other-1 0:12.3 (D3, 1.25 s, +46 dB alone), 7:35.2 (G2, 0.26 s, +47 dB alone), 7:47.2 (G#2, 0.19 s, +45 dB alone); bass-1 7:03.6 (F1, 0.43 s, +7 dB alone), 7:09.0 (F1, +8 dB alone), 7:09.1 (F1, 1.24 s, +5 dB alone). The pad examples are clean (nothing else plays at the start and end); the bass examples are not (+5 to +8 dB alone).
- Check: the drums stem is near empty (1.3%), so the drum entries and exits the tool lists (44% of the time) are void. The vocals stem holds 5%, 89% of it in 0.5 to 2 kHz: a pitched mid part, not hats; it never has words that I can tell.
- Building blocks: **[JUDGMENT]** **Guess:** a wide organ- or string-like pad with a partly unrelated left and right; the closest existing sounds are the pads, with the width (correlation 0.4) the new part.

**S4. A bass that swells and a bell pad (Pancake Lizard): bass-1, 103 notes, 19%; other-1, 446 notes, 27%**
- In synth terms: bass-1 sits at 49 Hz (44 to 62), attack 356 ms (153 to 760: a slow swell), falls 5 dB/s (a long sustain), brightness 108 Hz, leans left (-0.15; stem -0.13), correlation 0.99. other-1 is a bell-like voice at 86 Hz (38 to 218), attack 96 ms, falls 43 dB/s, brightness 484 Hz, level -7.0 dB, correlation 0.82; its group is weak (silhouette 0.15). The bass stem holds 54% of the power (82% under 120 Hz).
- Hear it at: bass-1 1:11.7 (F1, 0.27 s, +24 dB alone), 1:11.6 (F1, 0.24 s, +24 dB alone), 1:16.9 (G1, 0.17 s, +18 dB alone); other-1 2:27.1 (C3, 0.24 s, +33 dB alone), 2:27.4 (C3, 0.57 s, +21 dB alone), 2:25.3 (C3, 0.54 s, +20 dB alone).
- Building blocks: **[JUDGMENT]** **Guess:** a slow-swelling sine-like bass on F1 and G1, with a bell pad above it.

### Techniques

**T1. Deep kick ducking (Donkey Rhubarb)**
- The other stem dips 7.3 dB at the kick (peak 180 ms after; kick every 0.43 s) and the bass stem 9.8 dB (peak 364 ms after). The tool reads about 70% of the true depth, so the real dip is 10 dB or more on the other stem; the bass stem is 4.6% of the power, so its 9.8 is the less certain number. Vaz Deferenz reads 3.2 dB (other) and 3.8 dB (bass; void, 2% stem), and Icct Hedral's 3.7 dB at a kick every 0.45 s is void (there are no drums). Pancake Lizard's other stem dips 3.0 dB at a kick every 0.45 s, which does not match its 0.511 s beat; unconfirmed.
- **[JUDGMENT]** **Guess:** a deep kick duck of 10 dB or more on the pad, as the strongest end of the duck range (the ICBYD and Ventolin ones were 5 to 7 dB).

**T2. Brightness swings of 3 to 4 octaves every 10 to 20 s while the bass plays on (Pancake Lizard, 0:08 to 1:17)**
- The mix gets brighter 2.2 octaves at 0:08 to 0:15.5, darker 3.0 from 0:15.5 to 0:24.0, brighter 4.0 from 0:24.0 to 0:36.0, darker 3.1 from 0:36.0 to 0:41.0, brighter 2.7 at 0:50.0 to 0:57.5, darker 3.6 from 0:57.5 to 1:08.5, brighter 1.7 to 1:17.0 (37 more sweeps in `techniques.json`). The bass is on the whole time (it goes out at 1:38.0, comes back at 2:27.5) and the other stem plays 98% of the time, so these are not entries and exits: that makes it the best case so far for a real filter movement, though they are whole-mix numbers on a track whose bass holds 54% of the power.
- Donkey Rhubarb has a break: the mix falls 3.1 octaves at 2:15.5 to 2:26.0 and the kick (S1) returns alone at 2:26.3; it falls 2.1 octaves at 2:52 and rises 2.3 from 2:59.5 to 3:11.0.
- **[JUDGMENT]** **Guess:** a filter sweep of 3 to 4 octaves over 8 to 12 s, repeated, with a rising half then a falling half.

**T3. A slow pan cycle and a tape-speed drift (Vaz Deferenz, Pancake Lizard)**
- Vaz Deferenz: the other stem wobbles 15 cents at 2.4 Hz (the tool reads a 12-cent wow as 8.0, so the real wobble is nearer 22 cents) and its pan range is 0.38, cycling every 50 s. A 50 s cycle is not the tool's 3 to 4 s window artifact, so it is a slow pan. Pancake Lizard: the bass wobbles 11 cents at 1.2 Hz and its speed moves by up to 14 cents in 4 s; the other stem wobbles 6 cents at 6.5 Hz; the bass pan range is 0.46 and the other stem's 0.45 (both move). Icct Hedral: the bass wobbles 10 cents at 0.5 Hz. Donkey Rhubarb: nothing at 4 cents or more.
- **[JUDGMENT]** **Guess:** a 50 s slow pan on a mid layer, and a few cents of drift on the bass that moves up to 14 cents in 4 s.

**T4. Low-end entrances and gaps, and echoes I do not claim (Icct Hedral, Donkey Rhubarb, Vaz Deferenz)**
- Icct Hedral has no low end at all for the first 41 s (the bass stem enters at 0:59.5); later gaps in the low end are 12 s at 1:44.5, 9.5 s at 2:27, 21.5 s at 3:42 and 10 s at 5:06. The low end is on only 59% of the time, so the tool skips the count (rule: under 75%). The last 34.5 s, from 7:25, is the end. Donkey Rhubarb and Vaz Deferenz have no floor drops (only intro and tail runs); Pancake Lizard has two 9 s ones, at 3:56.0 and 4:08.5, as the drums go out at 3:35.0 and the bass at 3:57 and 4:12.5 (the low end is on 74% of the time, just under the line, so the count is borderline).
- The tool's drum echo at 434 ms (+9.8 dB) on Donkey Rhubarb is its kick spacing (0.43 s); Pancake Lizard's 514 ms other-stem echo (+3.1 dB) is its beat (0.511 s); neither is an echo. Vaz Deferenz's 444 ms other-stem echo (+8.9 dB, two repeats) is 3% over a quarter note at 139.7 BPM (429 ms); unconfirmed.
- **[JUDGMENT]** **Guess:** a piece that holds the low end back for the first 40 s and brings the bass in under a pad; no echo claimed.

### Theory

**Th1. One 1.718 s bar felt at two speeds, and a 117 BPM groove**
- Drum-stem onset repeats: Donkey Rhubarb 1.718 s (0.82) with a beat every 0.430 s (0.75): 139.7 BPM as a 4-beat bar (the tool's 140 agrees, the picker's 143.6 does not); swung 0.63 of the beat (triplet feel). Vaz Deferenz 0.859 s (0.77) and 1.718 s (0.75): the same bar, felt at 69.8 BPM (the tool says 70; the picker's 112.3 is wrong), with a long-short split (0.325 s plus 0.534 s) and swing 0.62. Pancake Lizard 2.055 s (0.65) and 0.511 s (0.58): 117 BPM, 4 beats a bar, straight eighths (the tool and picker agree). Icct Hedral has no beat (drums 1.3%), so its tool tempo (94) and picker tempo (143.6) are void.
- **[JUDGMENT]** **Guess:** a bar of 1.7 s drawn at full or half time (140 or 70), and a straight 117.

**Th2. A phrygian flat second in the bass; a mixolydian groove**
- Icct Hedral: bass F# 0.29 and F 0.26 of its time (55% together), with A 0.18 and C 0.18; 0.83 notes a second; key F phrygian, margin 0.021 (usable). The bass sits on the root and the flat second a semitone above it, with the third and fifth (A and C) above: the phrygian flat-second sound over a major-looking triad, which clashes. 25 chord changes a minute, 19 lasting 1.5 s or more (a median of about 3 s); the repeat structure is a two-bar loop (3.2 s bars, lag 2 at 0.65). Chord names are a guess on a dense pad.
- Pancake Lizard: pitch classes G 0.17, C 0.15, F 0.11, A 0.11, B 0.10, all in G mixolydian; key margin 0.049 (usable). The bass sits on G 0.38 of the time (F 0.19, C 0.16, B 0.16), with 24% fourths and fifths: a G pedal with the b7 (F) and the fourth (C). 23% of its bars near-duplicate an earlier bar, at 8 bars (0.50).
- Donkey Rhubarb and Vaz Deferenz: key margins 0.008 and 0.001, guesses; Donkey Rhubarb's bass notes (G 0.30, G# 0.25, A# 0.25, A 0.12, 0.10 a second) are a semitone cluster from a 5% stem, so a guess; Vaz Deferenz's bass is void (2% stem). The top lines of both (47 and 48% leaps) are bleed from the drums.
- **[JUDGMENT]** **Guess:** a bass that alternates root and flat second under a major triad (in a phrygian loop), and a G pedal with the flat seventh and the fourth over it in a mixolydian set; pad changes about every 3 s.

### Space
- Four spaces. Donkey Rhubarb is a mono file: median width -29 dB (-32 to -26), correlation 1.00 in every band, every part under -27 dB. Vaz Deferenz is near-mono with a wider mid layer: median -17 dB (-18 to -14), sub -24, mids -13 (0.92); drums -23, the other stem -13 and leaning right (+0.18).
- Icct Hedral is the widest of the album, and wide even in the sub: median -4 dB (-5 to -2), sub -7 (correlation 0.69), low-mids -4, mids -3 (0.36), highs -4; the other stem is -3 dB (mono loss 1.7 dB), the bass stem -9 dB (mono loss 0.5 dB), the vocals stem -3 dB and leaning right (+0.31, mono loss 1.8 dB).
- Pancake Lizard is half-wide with a moving image: median -12 dB (-20 to -10), sub -17 (0.96), mids -12, highs -24; the bass stem leans left (pan -0.13) with a 0.46 range, the other stem -10 dB.
- **[JUDGMENT]** **Guess:** a mono recipe for the drum tracks, a wide-in-every-band recipe with a wide sub for the orchestral piece (the mono fold costs 1.7 dB on the pad, so it needs the stereo guard), and a half-wide recipe with a drifting bass for the groove.

### Vibes (all guesses; weights lean x1.5 to x3 toward, x0.2 to x0.5 away, never 0)
- S1 and S2 sub kicks: toward booming, hypnotic, energetic x2; away from restful, tender x0.4.
- S3 wide synth-orchestra pad: toward dreamy, stately, hazy x2; away from bouncy, cheeky x0.4.
- S4 slow-swelling bass: toward hypnotic, warm x1.5.
- T1 deep duck: toward bouncy, energetic x1.5.
- T2 repeated big filter sweeps: toward hypnotic, dreamy x2; away from still x0.5.
- Th2 phrygian flat-second bass: toward mysterious, strange x2.
- **[JUDGMENT]** Ears 8 settles all of this.

### Wholesome take
- Brightness flags: 11% (Donkey Rhubarb) and 12% (Vaz Deferenz) of the energy is above 2 kHz by the mix, the highest on the record, but the stems show it is the hats: the vocals stems are 83 to 84% above 5 kHz on both. Icct Hedral 4%, Pancake Lizard 5%. No polarity flag; Icct Hedral is wide but not out of phase (correlation 0.36 to 0.69). The weight flag: the drums stems of Donkey Rhubarb and Vaz Deferenz are 83% and 87% under 120 Hz.
- **[JUDGMENT]** **Guess:** take the tuned sub kick (modest level), the deep duck (as a loop's top end), the repeated filter sweep, the root-and-flat-second bass and the wide pad; leave the hats on the drum tracks behind a gentle level.

### For later
- Donkey Rhubarb, other-1 (445 notes): F3, noisy, falls 31 dB/s; hear 3:32.8, 3:53.4. **[JUDGMENT]** **Guess:** waits.
- Donkey Rhubarb, bass-1 (436 notes): G#1, falls 273 dB/s and brightens; hear 5:16.9, 5:20.3. **[JUDGMENT]** **Guess:** waits, a short bass pluck (below 60 Hz the bell label is the tool's limit).
- Pancake Lizard, drums-1 (182 notes): low thump, falls 103 dB/s, brightens; hear 2:42.8, 1:41.5. **[JUDGMENT]** **Guess:** waits.
- Pancake Lizard, bass-2 (36 notes): C2, slow swell, falls 8 dB/s; hear 0:54.6, 1:02.1. **[JUDGMENT]** **Guess:** waits.
- Icct Hedral, other-2 (112 notes): C#4, bell-like; hear 0:23.9, 7:41.6; and vocals-1 (469 notes, D4, noisy; hear 6:58.2, 6:58.1): a pitched mid part in the vocals stem. **[JUDGMENT]** **Guess:** waits, a high bell over the pad.
- Donkey Rhubarb vocals-1 and Vaz Deferenz vocals-1 (439 and 415 notes, B2 and C#3, falls 428 and 773 dB/s; hear 5:38.7, 5:13.0 and 5:44.0, 3:48.4): short ticks the separator put in the vocals stem. **[JUDGMENT]** **Guess:** waits; not voices.
- Not given a deep pass: 05 Icct Hedral (dry version plus bonus DAT glitches), the same audio as 03 in my copy (see Shared tracks). **[JUDGMENT]** **Guess:** Mikey checks the Drive file; if it differs, a second pass on the part that differs is worth one more run.

### Reliability notes for this card
- Separator: MDX-Net, not Demucs (see the top of this file). The sub lives in the drums stem on Donkey Rhubarb (83% under 120 Hz) and Vaz Deferenz (87%); Pancake Lizard's drums stem is 32% under 120 Hz (mids), and its bass stem is real (54%).
- Empty stems: Vaz Deferenz's bass stem holds 1.8%, so its bass behavior (G# 0.65, 0.04 notes a second), bass pump and bass lines are void. Icct Hedral's drums stem holds 1.3%, so its drum entries, pump numbers and drum-based tempo are void. Donkey Rhubarb's bass stem (4.6%) is thin: treat its bass notes as a guess. The vocals stems of Donkey Rhubarb, Vaz Deferenz and Pancake Lizard are hats (61 to 84% above 5 kHz); Icct Hedral's is a mid part (89% in 0.5 to 2 kHz). No words anywhere.
- Tempo: the tool's meter agrees with the drum grid on Donkey Rhubarb, Vaz Deferenz and Pancake Lizard but bar length is unclear on the first two (scores 0.12 and 0.01); the picker's mix-level tempo is wrong on Vaz Deferenz (112.3 against 69.8).
- Key margins: Donkey Rhubarb 0.008 and Vaz Deferenz 0.001 (guesses), Icct Hedral 0.021 and Pancake Lizard 0.049 (usable). Chord names are guesses on dense pads. Tuning readings run from -1 to +43 cents.
- Floor drops: Icct Hedral (59% on) is skipped by the rule; Pancake Lizard (74%) is borderline. Pitch under 60 Hz: kick pitches are good to about a semitone, partial-off labels are not measurable.
- Missing audio: the 2 s tails of 03 and 05 (and any DAT glitches in them) are not in this analysis.

---

## AFX, Hangable Auto Bulb (10 tracks)

Ears 6, 2026-10-18, Sonnet 5.5. The Drive folder holds 10 tracks, numbered 01 to 10; 04 Bit is 11 s long (too short for a pass), so 9 are usable. Deep pass on three, as the picker chose: the most typical, 01 Children Talking (5:22), and the two least like the rest, 09 Get a Baby (2:27) and 07 Every Day (3:50). The title track, 02 Hangable Auto Bulb (6:53), got no deep pass. The folder looks like two records joined: tracks 01 to 06 are near-mono (width -31 to -35 dB) and sub-heavy (sub 0.54 to 0.66 except 03), while 07 to 10 are wider (-15, -30, -15 and -21 dB); that fits *Hangable Auto Bulb* as the first six and *EP.2* as the last four, but the audio cannot tell me where the split is, so the plan's *EP.2* may already be inside this folder, unconfirmed.

Machine time: 2,602 s (43.4 min) for 11.7 minutes of deep-pass audio (700.2 s): pick 368 s (10 tracks, 37 s a track; slow because I was running other work at the same time), MDX-Net split 1,971 s (2.8 times the audio; also slowed by that), measuring 263 s (0.38 times the audio). The plan's formula gives 1,370 s, so this is 1.9 times it.

**Shared tracks.** No title repeats another pile 2 album. 03 Laughable Butane Bob (182.2 s, -9.9 LUFS), 05 Custodian Discount (270.5 s, -11.0), 06 Wabby Legs (335.1 s, -12.1) and 10 choirDrill (252.0 s, -9.7) are not within 3 s and 0.6 LU of any pile 1 Classics row.

### The hook
- Mono, sub-heavy tracks at 161 or 168 BPM with a sine-like sub bass that sits at 32 to 41 Hz (C1 to E1), a tritone between the bass notes, a kick that ducks the bass by 6 to 8 dB, and a few passages where the bass plays alone for 5 s.
- **[JUDGMENT]** One line a listener would say. **Guess:** very low sine bass under fast, mono, dry drums, with the pad and voices as small mid-range accents.

### Sounds

**S1. Sub-sine bass at C1 to C#1 (Children Talking): bass-1, 358 notes, 38%**
- In synth terms: 35 Hz (31 to 49), attack 72 ms, falls 37 dB/s, brightness 114 Hz, level -7.5 dB, centered (correlation 1.00). The bass stem holds 31% of the power and 96% of it is under 120 Hz; the drums stem holds 60% (53% under 120 Hz).
- Hear it at: 5:12.8 (G#1, 1.53 s, +42 dB alone), 5:13.6 (G1, 4.72 s, +32 dB alone), 4:52.9 (C1, 4.73 s, +14 dB alone). From 5:13 the drums are out and the bass plays alone for about 5 s.
- Check: the stem's strongest lines are 32.2 Hz (C1, 27 cents flat), 35.2 Hz (C#1, 27 cents sharp), 48.5 Hz (G1, 18 cents flat) and 26.8 Hz. Pitch under 40 Hz is good to about a semitone, and the bell-like label is the tool's guess below 60 Hz: ignore it. So the sub bass is a plain tone around 32 to 35 Hz, with G above it.
- Building blocks: **[JUDGMENT]** **Guess:** a sine sub bass pitched to C1 and C#1; below most small speakers' range, so it needs a level guard.

**S2. A short sub thump and a walking bass at 31 to 41 Hz (Get a Baby): drums-1, 231 notes, 16%; bass-1, 208 notes, 35%**
- In synth terms: drums-1 is a thump at 41 Hz (32 to 55), attack 25 ms, falls 277 dB/s (very short), brightness 179 Hz, level -3.8 dB, correlation 0.99. bass-1 sits at 37 Hz (31 to 42), harmonic, attack 38 ms, falls 15 dB/s, brightness 101 Hz, level -3.7 dB, centered. The bass stem holds 40% of the power, 95% of it under 120 Hz.
- Hear it at: drums-1 0:44.1 (0.19 s, +16 dB alone), 0:31.0 (0.21 s, +16 dB alone), 0:57.1 (0.18 s, +15 dB alone; the tool gives no pitch for them); bass-1 2:15.7 (C1, 0.6 s, +11 dB alone), 2:16.3 (0.3 s, +10 dB alone), 2:15.0 (C1, 0.63 s, +6 dB alone; the bass is not clean).
- Check: the bass lines are 31.0 Hz (B0), 36.9 (D1), 41.4 (E1): the tool's bass notes E 0.27, B 0.23, C 0.22, D 0.18, at 1.61 notes a second with 64% repeats. A four-note bass cycle at about 1.6 notes a second, all under 45 Hz.
- Building blocks: **[JUDGMENT]** **Guess:** the tuned sub-kick again, here very short (277 dB/s) and low (41 Hz), over a four-note sub-bass cycle.

**S3. A noisy low-mid pad with the voice-like part in the vocals stem (Every Day): other-1, 331 notes, 27%; vocals-1, 339 notes, 19%**
- In synth terms: other-1 sits at 93 Hz (78 to 185), noisy partials, attack 110 ms, falls 15 dB/s, brightness 598 Hz, level -4.5 dB, correlation 0.82; the group is weak (silhouette 0.19). The other stem holds 33% of the power, 61% of it in 120 to 500 Hz. vocals-1 sits at D4 (286 Hz, 233 to 343), noisy partials, attack 86 ms, falls 67 dB/s, brightness 828 Hz, level -6.8 dB, centered. The vocals stem holds 20% of the power, 90% of it in 0.5 to 2 kHz.
- Hear it at: other-1 2:40.1 (F#2, 0.18 s, +27 dB alone), 2:39.8 (F#2, 0.23 s, +22 dB alone), 2:39.7 (F#2, 0.21 s, +21 dB alone), right as the drums come back in at 2:39.5; vocals-1 2:38.0 (D4, 0.36 s, +7 dB alone), 0:43.7 (A#3, 1.28 s, +3 dB alone), 0:32.2 (A#3, 1.06 s, +4 dB alone). The vocals examples are not clean (+3 to +7 dB alone).
- Check: the vocals stem on an instrumental is a guess. A pitched 0.5 to 2 kHz tone with 20% of the power is a lead or voice-like part, and I did not listen to decide which. If it is a voice, the rule applies: vowels, hums and choirs only, never words.
- Building blocks: **[JUDGMENT]** **Guess:** a noisy low-mid pad (the existing pads) and a mid-range lead around D4 that waits for a listen.

**S4. Kick-and-snap drums (Children Talking): drums-1, 145 notes, 15%**
- In synth terms: the group fuses a low thump and a snap: fundamental 52 Hz (32 to 68), brightness 496 Hz (176 to 1,848), attack 36 ms, falls 34 dB/s, level -0.3 dB, centered. The drums stem is 53% under 120 Hz, 26% in 120 to 500 Hz, 8% in 2 to 5 kHz and 7% above 5 kHz: a kit with hats.
- Hear it at: 2:08.4 (C#2, 0.1 s, +23 dB alone), 4:08.3 (D1, 0.4 s, +22 dB alone), 4:35.0 (C1, 0.21 s, +23 dB alone).
- Building blocks: **[JUDGMENT]** **Guess:** the existing kick and snap, dry and mono.

### Techniques

**T1. A long intro with no low end, builds and big whole-mix swings (Children Talking)**
- The low end (all stems summed, under 120 Hz) is out for the first 4 s and again 4.5 to 21.5 s: the bass stem enters at 0:19.5. A build is marked at 1:44.0 to 2:09.0, with the bass out at 1:47.5 and back at 1:51.0 and a 4.5 s floor drop at 1:46.5 (the tool's own count lists 1:47.8 for 2.0 s, 0.21 a minute); the mix falls 4.6 octaves from 1:49.0 to 1:54.5 and 3.2 octaves from 1:06.0 to 1:16.5 (57 more sweeps in `techniques.json`, so a lot of motion in the mix). The bass is out for 20 s from 3:09.0 to 3:29.0 and again from 5:18.5 to the end at 5:22 (the 5 s sums run at 5:17 is the tail). The low end is on 79% of the time.
- Brightness numbers here are whole-mix, on a drum-led track.
- **[JUDGMENT]** **Guess:** a 15 to 20 s intro without the low end, then a bass entrance; a 25 s build with a 4 s floor drop and a mix fall of about 4 octaves.

**T2. The bass ducks at the kick (Every Day)**
- Bass dips 5.8 dB at the kick (peak 165 ms after; kick every 0.35 s, which matches the 168 BPM beat of 0.357 s). The tool reads about 70% of the true depth, so the real dip is about 8 dB. Get a Baby reads 2.9 dB (kick every 0.30 s; the drum grid is 0.41/0.615 s, so that spacing does not fit and I do not trust it). Children Talking's other stem dips 3.6 dB but holds 2% of the power: void.
- **[JUDGMENT]** **Guess:** an 8 dB duck on the bass at the kick, in loops at 168 BPM.

**T3. Bass wobble (Children Talking), a moving pan (Get a Baby), and an echo I do not claim**
- Children Talking: the bass wobbles 13 cents at 0.7 Hz (the tool read a 12-cent wow as 8.0, so closer to 20 cents). Every Day: the other stem wobbles 5 cents at 0.3 Hz. Get a Baby: nothing at 4 cents or more, but the other stem's pan range is 0.65, the widest movement in the pile so far (12% of the power).
- The drum echo at 414 ms (+11.5 dB) on Get a Baby is the drum spacing (0.406 s), not an echo. Children Talking's 559 ms other-stem echo comes from a stem with 2% of the power: void.
- **[JUDGMENT]** **Guess:** a 0.7 Hz bass wobble of 15 to 20 cents, and a wide pan on one pad layer (a range of 0.6).

### Theory

**Th1. Three grids: 161.5, 98 and 168 BPM (with a 3:2 pair)**
- Children Talking: the drum grid has a beat of 0.372 s (0.58), 161.3 BPM; the tool says 80 (half-time) and the picker 161.5. Bar length unclear. Get a Baby: 0.615 s (0.56) = 97.6 BPM (the tool says 98, the picker 99.4); a second peak at 0.406 s (0.55) is two thirds of the beat, and the repeat at 1.637 s (0.57) is 4 of those, so the triplet feel (swing 0.67) is a full triplet; no cross-rhythm is claimed by the tool.
- Every Day: the drum onsets at 0.360 s (0.68), 0.534 s (0.69), 0.894 s (0.69), 1.254 s (0.67) and 1.428 s (0.72) fit a beat of 0.357 s (168 BPM) with onsets on the half beats (1.5, 2.5, 3.5 beats); the kick spacing of 0.35 s (T2) agrees with 168. The tool and picker both say 112 (112 is 2/3 of 168, which is the 0.534 s peak), so it could be read as 112 with triplets or 168 with syncopation; the kick says 168. Swing 0.66.
- **[JUDGMENT]** **Guess:** tempo draws of 161 to 168 for the fast pair and 98 for the slow one, with a triplet swing on the slow one.

**Th2. A tritone between the bass notes, and a cycle of four notes**
- Children Talking: the bass sits on G 0.48 of the time with C# 0.23, C 0.11 and G# 0.11 (0.37 notes a second, repeats 54%): G to C# is a tritone and C to C# a semitone, which the tool reads as G locrian (margin 0.015, a guess). Get a Baby: E 0.27, B 0.23, C 0.22, D 0.18 at 1.61 notes a second (E aeolian, margin 0.010, a guess). Every Day: E 0.47, G# 0.32, F# 0.12, D# 0.07 at 0.28 notes a second (E lydian, margin 0.004, a guess): a slow bass on a major third.
- Chord names and top lines on these tracks come from stems with 2 to 33% of the power and are not reliable.
- **[JUDGMENT]** **Guess:** a one-note bass pedal with a tritone neighbor held for several seconds; a four-note bass cycle at 1.6 notes a second.

### Space
- Mostly mono. Children Talking is a mono file: median width -27 dB (-29 to -25), correlation 1.00 in every band; every part under -26 dB. Get a Baby: median -13 dB (-17 to -9), sub -25 (0.99), low-mids and mids -11 (0.86), 2 to 5 kHz -15; the other stem is -6 dB (mono loss 0.9 dB), drums -17, bass -31. Every Day: median -13 dB (-15 to -11), sub -27, low-mids -12 (0.88); the other stem is -10 dB (mono loss 0.4 dB), drums -18, bass -31.
- **[JUDGMENT]** **Guess:** a mono recipe for the sub-and-drums core and a half-wide recipe for the pad (Get a Baby, Every Day).

### Vibes (all guesses; weights lean x1.5 to x3 toward, x0.2 to x0.5 away, never 0)
- S1 and S2 very low sine bass: toward booming, hypnotic x2; away from restful, bright x0.4 (needs a level guard).
- S3 noisy low-mid pad: toward hazy, dreamy x1.5.
- T1 long intro without the low end, then a bass entrance: toward hypnotic, anticipating x1.5.
- T2 8 dB duck: toward bouncy, energetic x1.5.
- Th2 tritone bass neighbor: toward mysterious, strange x2.
- **[JUDGMENT]** Ears 8 settles all of this.

### Wholesome take
- Brightness flags: Children Talking has 10% of its energy above 2 kHz by the stems sum (the hats), Get a Baby 1.4% and Every Day 11%; no polarity flag. Weight flag: 66% of Children Talking's energy and 69% of Get a Baby's is under 120 Hz, and the lowest bass notes (C1 at 32 Hz, B0 at 31 Hz) are under what small speakers play, so the level guard matters here more than anywhere in pile 1.
- Voices: Children Talking is named for speech and carries a spread-out vocals stem (33% in 120 to 500 Hz, 34% in 0.5 to 2 kHz, 19% in 2 to 5 kHz); I have not listened. Get a Baby and Every Day have pitched mid-range vocals stems (57% and 90% in 0.5 to 2 kHz). Any real speech is words, which the rule keeps out of the app: only the pitched, wordless side (a vowel or hum) could ever be taken.
- **[JUDGMENT]** **Guess:** take the very low sine bass (behind the level guard), the tritone neighbor, the 8 dB duck and the long no-low-end intro; leave any speech sample out entirely.

### For later
- Children Talking, vocals-1 (354 notes, D#3, noisy, falls 77 dB/s; hear 1:48.8, 0:01.5): the speech-like layer. **[JUDGMENT]** **Guess:** waits, and if it is speech it never enters as words.
- Children Talking, drums-2 (221 notes): mid snap, falls 158 dB/s; hear 1:56.7, 4:33.3. **[JUDGMENT]** **Guess:** waits.
- Every Day, other-2 (169 notes, C#4, noisy; hear 2:37.0, 3:31.0) and bass-1 (322 notes, G#1, falls 151 dB/s; hear 0:01.1, 0:46.5). **[JUDGMENT]** **Guess:** waits.
- Get a Baby, drums-2 (269 notes, mid snap, falls 90 dB/s; hear 1:09.1, 1:29.0), bass-2 (70 notes, B1, harmonic; hear 0:57.3, 1:15.3), vocals-1 (176 notes, C4; hear 0:50.7, 0:30.3) and other-1 (112 notes, G2, slow swell; hear 2:18.0, 1:05.3). **[JUDGMENT]** **Guess:** waits.
- Not given a deep pass, mix level only: 02 Hangable Auto Bulb (6:53, 161.5 BPM, tuning -41 cents, width -35 dB, 9.6 chord changes a minute); 03 Laughable Butane Bob (3:02, 107.7 BPM, tuning -38, 16.1 chord changes); 05 Custodian Discount (4:30, tuning -47, the largest offset, 19.7 chord changes); 06 Wabby Legs (5:35, 0.9 chord changes a minute, the least); 08 Arched Maid Via RDJ (5:23, sub 25%, 1.9 chord changes); 10 choirDrill (4:12, 172 BPM, width -21.5 dB, the widest of 01 to 06 and 10; the name suggests a choir, vowels and no words only); 04 Bit (11.1 s; wobble 8.3 cents against a folder median of 4.1, repetition 0.23 against 0.48). Four of the ten tracks read 38 to 47 cents off at mix level (flat on 02, 03 and 05; 01 sharp at +43), which is a lead for a quarter-tone detune. **[JUDGMENT]** **Guess:** 02 and 05 are the two worth a deep pass if Ears 8 wants one; the offset is the interesting part.

### Reliability notes for this card
- Separator: MDX-Net, not Demucs (see the top of this file). Children Talking's other stem holds 2% of the power and its vocals stem 7.6%; its pump, echo and top line are void.
- Tempo: the tool's meter is half the drum grid on Children Talking (80 against 161.3); on Get a Baby the tool, picker and grid agree at 98; on Every Day the kick says 168 and the tool and picker say 112.
- Key margins: 0.015, 0.010 and 0.004, all guesses. Chord names on all three are guesses.
- Pitch under 60 Hz: partial-off labels (the bell-like on the sub basses) are not measurable; pitch under 40 Hz is good to about a semitone, so C1 (32.7 Hz) and B0 (30.9 Hz) are good to that.
- Floor drops: Get a Baby and Every Day have none apart from the tail; Children Talking's are in T1.
- Mix-level tuning readings run from -47 to +43 cents on this folder; the bass lines of Children Talking read -27 to +27 cents from the nearest note, which is noise-level at this register.

---
