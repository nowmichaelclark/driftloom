# Ears cards, pile 2, first half

Ears 6 (Sonnet 5.5, medium), started 2026-10-18. Album cards from `tools/ears/album.py` (stage by stage), checked against the limits in `docs/EARS.md`. Lines marked **[JUDGMENT]** are for Ears 8; each carries a short first guess marked **Guess**. Machine time per album is on each card; the totals and the cost check are in `docs/ears/plan.md`.

**The stems on these cards did not come from Demucs.** The workspace's network policy blocks the Demucs weights (`dl.fbaipublicfiles.com` and Hugging Face both answer 403), so `album.py` quietly fell back to the harmonic/percussive split on its first run (I threw that run away). The UVR MDX-Net four-stem models (`kuielab_a_drums`, `_bass`, `_other`, `_vocals`, ONNX, from the TRvlvr model_repo GitHub release) do download, so `tools/ears/split.py` now has a middle step, `mdx`, between Demucs and the fallback (see the PR). They are real drums, bass, other and vocals stems, but a weaker separator than Demucs on this music: the drums stem swallows most of the sub (it holds 78 to 89% of its energy under 120 Hz on the dense tracks), the bass stem is thin or empty wherever the kick and the bass share the sub, and the vocals stem on an instrumental is hi-hat and pad bleed. Every card says which stem each number came from, and per-stem figures that rest on a near-empty stem are marked void. The mix-level numbers (loudness, bands, width, tuning, tempo from `pick`) do not depend on the separator.

| Album | Tracks (deep pass) | Machine time | Status |
|---|---|---|---|
| I Care Because You Do (1995) | 12 (4) | 4,352 s | done |
| Ventolin (EP) | 14 (4) | 2,620 s | done |
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
