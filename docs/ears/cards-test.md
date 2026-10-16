# Ears 3 test cards: Geogaddi and Galaxy Garden

Two draft album cards written by `tools/ears/album.py` (Ears 3, 2026-10-15, Sonnet 5.5 high). Nothing here was listened to: every line is a measurement or a threshold read off one, and lines marked **[JUDGMENT]** are for a person or a later session (hook, vibes, wholesome take, why-it-waits). Format follows `docs/ears/plan.md`. Rules kept: American English; no real words from voices; no quotes.

## Checked against Ears 1

| Number | Ears 1 | This run | Reading |
|---|---|---|---|
| Geogaddi chord changes a minute (whole album) | about 10 | deep-pass tracks 16 to 28 changes; 11 to 20 settled (chords of 1.5 s or more) | Higher, not the same. Same-run old counter vs new: Redpoint 17 vs 28, Gyroscope 11 vs 16, Sunshine Recorder 7 vs 18, 1969 27 vs 28. Ears 1's per-track 2-s counter: 10.2, 5.6, 5.5, 24.6. The new counter sees movement inside slow pads that the 2-s window skips. 71% of Redpoint's changes and 78% of 1969's land on a beat (chance 36% and 37%), so those are probably real moves; Gyroscope (44% vs 47%) and Sunshine Recorder (56% vs 50%) are not clearly. No truth exists for real music, so "about 10" and "about 20" are both still open until someone listens. Calibrated range: exact for a chord every 0.5 s or longer on clean chords; 0.35 s undercounts (see calibration). |
| Galaxy Garden floor drops a minute (whole album) | 1.9; Crystal Caverns 1991 4.87; other three picks "low end too sparse" | Crystal Caverns 1991 2.74; Lying in the Reeds 0.74; New Colour and Stands Tidal Waves "too sparse" | Same kind of answer: Crystal Caverns is the album's drop track (a run of seven 2.5 to 3 s drops from 3:40, plus 7.5 s at 0:25), and the two sparse tracks stay sparse. The count differs because this tool reads the drums and bass stems (Ears 1 read a 30 to 120 Hz band of the mix) and counts a drop only from 2 s. Lying in the Reeds now has a number, probably because the stems give the low end a firmer on-share than the band did (not checked). |

Known weak spots in these cards: bar length is unclear on Redpoint, Lying in the Reeds and Stands Tidal Waves (the card says so); keys with a margin under 0.02 are marked as guesses; "top line" is only reliable when the line sits clearly above its accompaniment; tempo can be half or double; Gyroscope's bass stem is nearly empty, so its bass lines are thin.

---

## Geogaddi (draft card from Ears 3 tools, 2026-10-15)

How it was made: 19 usable tracks: the most typical plus the 3 least like the rest. Deep pass on: Boards of Canada - The Beach at Redpoint; Boards of Canada - Gyroscope; Boards of Canada - Sunshine Recorder; Boards of Canada - 1969. Typical track: Boards of Canada - The Beach at Redpoint. Least like the rest: Boards of Canada - Gyroscope, Boards of Canada - Sunshine Recorder, Boards of Canada - 1969.
Stems from Demucs htdemucs (drums, bass, other, vocals). Lines marked **[JUDGMENT]** need a person or a later session; every other line is a measurement or a threshold read off one.

### The hook
- Numbers that repeat across the deep-pass tracks: tempo 73 to 100 BPM (octave-uncertain); keys G# phrygian, G phrygian, E phrygian, C# ionian; mix width -14 to -6 dB; chord changes 16 to 28 a minute.
- **[JUDGMENT]** The one thing that makes a listener say "that's them", in one line, after listening at the timestamps under Sounds.

### Sounds
Groups are ranked by share of the track's note energy, at most two per track. Examples are the clearest notes (loud against everything else playing).
#### S1. Boards of Canada - The Beach at Redpoint: other group `other-1` (418 notes, 48% of the track's note energy)
- In synth terms: around G2, mixed or noisy partials, slow swell, falls 6 dB/s, centered.
- Hear it at: 0:03.1 (B3, 2.03 s, +21 dB alone), 1:17.0 (B1, 0.74 s, +19 dB alone), 0:33.2 (B1, 6.26 s, +15 dB alone).
- Fingerprint, median (10th to 90th percentile):
  - register 97 (51 to 205) Hz; attack 236 (65 to 745) ms; decay -6 (-70 to 8) dB/s
  - brightness at the start 656 (496 to 847) Hz; slope over the note 0.00 (-0.29 to 0.48) octaves
  - harmonic share 0.51 (0.15 to 0.83); partials off whole multiples 59 (29 to 119) cents
  - vibrato 0 (0 to 11) cents; detune beating 0.0 (0.0 to 0.0) Hz
  - level against the mix -6.7 (-11.2 to -1.9) dB; pan -0.02 (-0.06 to 0.01); left-right correlation 0.94 (0.85 to 0.97)
- Group quality: weak (silhouette 0.15, 1 group in the stem). The notes in this stem do not fall into clearly separate instruments; treat this as one blended texture and listen before trusting the split.
- Building blocks it would need: **[JUDGMENT]** name the closest existing Driftloom block, or the one missing.
- Yardstick: the percentile ranges above, measured on the other stem (Demucs), not on the raw mix.

#### S2. Boards of Canada - Gyroscope: other group `other-1` (333 notes, 37% of the track's note energy)
- In synth terms: around G1, harmonic (string/reed/pad-like), slow swell, falls 11 dB/s, centered.
- Hear it at: 1:37.0 (F1, 1.27 s, +7 dB alone), 2:40.9 (G1, 1.14 s, +9 dB alone), 1:38.1 (F#1, 0.88 s, +8 dB alone).
- Fingerprint, median (10th to 90th percentile):
  - register 50 (34 to 51) Hz; attack 305 (52 to 588) ms; decay -11 (-55 to 3) dB/s
  - brightness at the start 213 (162 to 279) Hz; slope over the note 0.00 (-0.20 to 0.18) octaves
  - harmonic share 0.91 (0.33 to 0.99); partials off whole multiples 18 (4 to 92) cents
  - vibrato 0 (0 to 6) cents; detune beating 0.0 (0.0 to 0.0) Hz
  - level against the mix -6.0 (-10.2 to -2.4) dB; pan 0.03 (-0.04 to 0.11); left-right correlation 0.79 (0.58 to 0.89)
- Building blocks it would need: **[JUDGMENT]** name the closest existing Driftloom block, or the one missing.
- Yardstick: the percentile ranges above, measured on the other stem (Demucs), not on the raw mix.

#### S3. Boards of Canada - 1969: drums group `drums-1` (468 notes, 31% of the track's note energy)
- In synth terms: low thump, soft attack, falls 151 dB/s, centered.
- Hear it at: 0:59.4 (A1, 0.29 s, +8 dB alone), 2:50.2 (B1, 0.21 s, +7 dB alone), 0:46.3 (D#1, 0.22 s, +8 dB alone).
- Fingerprint, median (10th to 90th percentile):
  - register 53 (36 to 88) Hz; attack 54 (20 to 97) ms; decay -151 (-227 to -63) dB/s
  - brightness at the start 89 (78 to 257) Hz; slope over the note 0.00 (-0.20 to 2.30) octaves
  - harmonic share 0.79 (0.14 to 0.99); partials off whole multiples 169 (56 to 333) cents
  - vibrato 0 (0 to 0) cents; detune beating 0.0 (0.0 to 0.0) Hz
  - level against the mix -2.7 (-4.0 to -1.6) dB; pan 0.02 (-0.01 to 0.07); left-right correlation 1.00 (0.99 to 1.00)
- Building blocks it would need: **[JUDGMENT]** name the closest existing Driftloom block, or the one missing.
- Yardstick: the percentile ranges above, measured on the drums stem (Demucs), not on the raw mix.

#### S4. Boards of Canada - Sunshine Recorder: other group `other-1` (337 notes, 28% of the track's note energy)
- In synth terms: around E2, mixed or noisy partials, slow swell, falls 10 dB/s, centered.
- Hear it at: 0:12.3 (E2, 1.56 s, +50 dB alone), 5:54.0 (E2, 1.33 s, +37 dB alone), 0:03.8 (E2, 1.76 s, +32 dB alone).
- Fingerprint, median (10th to 90th percentile):
  - register 84 (74 to 168) Hz; attack 176 (55 to 465) ms; decay -10 (-52 to 3) dB/s
  - brightness at the start 641 (452 to 897) Hz; slope over the note 0.00 (-0.07 to 0.27) octaves
  - harmonic share 0.50 (0.12 to 0.87); partials off whole multiples 55 (21 to 112) cents
  - vibrato 0 (0 to 0) cents; detune beating 0.0 (0.0 to 0.0) Hz
  - level against the mix -2.6 (-5.7 to -0.5) dB; pan -0.03 (-0.14 to 0.01); left-right correlation 0.91 (0.69 to 0.99)
- Group quality: weak (silhouette 0.21, 2 groups in the stem). The notes in this stem do not fall into clearly separate instruments; treat this as one blended texture and listen before trusting the split.
- Building blocks it would need: **[JUDGMENT]** name the closest existing Driftloom block, or the one missing.
- Yardstick: the percentile ranges above, measured on the other stem (Demucs), not on the raw mix.

#### S5. Boards of Canada - 1969: bass group `bass-1` (275 notes, 28% of the track's note energy)
- In synth terms: around C#2, harmonic (string/reed/pad-like), soft attack, falls 0 dB/s, centered.
- Hear it at: 0:36.8 (C#2, 1.59 s, +11 dB alone), 0:17.3 (C#2, 1.56 s, +10 dB alone), 0:31.1 (C#2, 0.41 s, +10 dB alone).
- Fingerprint, median (10th to 90th percentile):
  - register 68 (68 to 68) Hz; attack 99 (25 to 327) ms; decay -0 (-4 to 1) dB/s
  - brightness at the start 72 (70 to 75) Hz; slope over the note -0.00 (-0.03 to 0.00) octaves
  - harmonic share 1.00 (1.00 to 1.00); partials off whole multiples 2 (1 to 8) cents
  - vibrato 0 (0 to 3) cents; detune beating 0.0 (0.0 to 0.0) Hz
  - level against the mix -4.1 (-5.4 to -1.5) dB; pan 0.00 (-0.00 to 0.00); left-right correlation 1.00 (1.00 to 1.00)
- Building blocks it would need: **[JUDGMENT]** name the closest existing Driftloom block, or the one missing.
- Yardstick: the percentile ranges above, measured on the bass stem (Demucs), not on the raw mix.

#### S6. Boards of Canada - Gyroscope: drums group `drums-1` (250 notes, 27% of the track's note energy)
- In synth terms: low thump, quick attack, falls 20 dB/s, gets brighter over the note, centered.
- Hear it at: 0:03.0 (?, 0.34 s, +23 dB alone), 0:02.9 (?, 0.16 s, +22 dB alone), 0:13.2 (?, 0.34 s, +12 dB alone).
- Fingerprint, median (10th to 90th percentile):
  - register 30 (30 to 30) Hz; attack 23 (23 to 58) ms; decay -20 (-77 to -6) dB/s
  - brightness at the start 147 (120 to 245) Hz; slope over the note 0.31 (-0.32 to 1.81) octaves
  - harmonic share 0.53 (0.50 to 0.54); partials off whole multiples 276 (225 to 293) cents
  - vibrato 0 (0 to 0) cents; detune beating 0.0 (0.0 to 0.0) Hz
  - level against the mix -2.0 (-2.6 to -1.7) dB; pan -0.14 (-0.16 to -0.12); left-right correlation 0.49 (0.44 to 0.53)
- Building blocks it would need: **[JUDGMENT]** name the closest existing Driftloom block, or the one missing.
- Yardstick: the percentile ranges above, measured on the drums stem (Demucs), not on the raw mix.

#### S7. Boards of Canada - Sunshine Recorder: bass group `bass-1` (88 notes, 18% of the track's note energy)
- In synth terms: around A#1, harmonic (string/reed/pad-like), quick attack, falls 6 dB/s, beats like a detuned pair (about 8 cents), centered.
- Hear it at: 5:44.7 (A#1, 0.95 s, +6 dB alone), 2:43.5 (G1, 4.77 s, +0 dB alone), 4:44.3 (G1, 5.24 s, -0 dB alone).
- Fingerprint, median (10th to 90th percentile):
  - register 59 (42 to 85) Hz; attack 25 (10 to 281) ms; decay -6 (-17 to -3) dB/s
  - brightness at the start 112 (97 to 152) Hz; slope over the note 0.03 (-0.28 to 0.60) octaves
  - harmonic share 0.99 (0.87 to 1.00); partials off whole multiples 7 (3 to 70) cents
  - vibrato 6 (0 to 15) cents; detune beating 0.3 (0.0 to 0.6) Hz
  - level against the mix -8.0 (-24.5 to -3.5) dB; pan 0.00 (-0.00 to 0.00); left-right correlation 1.00 (1.00 to 1.00)
- Building blocks it would need: **[JUDGMENT]** name the closest existing Driftloom block, or the one missing.
- Yardstick: the percentile ranges above, measured on the bass stem (Demucs), not on the raw mix.

#### S8. Boards of Canada - The Beach at Redpoint: bass group `bass-1` (123 notes, 16% of the track's note energy)
- In synth terms: around B1, harmonic (string/reed/pad-like), soft attack, falls 11 dB/s, centered.
- Hear it at: 3:12.4 (G#2, 1.44 s, +2 dB alone), 3:43.6 (B1, 1.66 s, -0 dB alone), 3:37.1 (G#2, 1.64 s, +2 dB alone).
- Fingerprint, median (10th to 90th percentile):
  - register 61 (46 to 103) Hz; attack 48 (16 to 200) ms; decay -11 (-23 to -3) dB/s
  - brightness at the start 105 (87 to 134) Hz; slope over the note -0.01 (-0.26 to 0.46) octaves
  - harmonic share 0.99 (0.97 to 1.00); partials off whole multiples 6 (2 to 33) cents
  - vibrato 3 (0 to 13) cents; detune beating 0.0 (0.0 to 0.0) Hz
  - level against the mix -6.0 (-14.0 to -3.2) dB; pan 0.02 (0.01 to 0.04); left-right correlation 1.00 (0.98 to 1.00)
- Building blocks it would need: **[JUDGMENT]** name the closest existing Driftloom block, or the one missing.
- Yardstick: the percentile ranges above, measured on the bass stem (Demucs), not on the raw mix.

### Techniques
- **Boards of Canada - The Beach at Redpoint sections:** 11 boundaries at 0:05.0, 0:54.0, 1:06.0, 1:18.0, 1:37.0, 2:03.0, 2:24.0, 2:43.0, 3:03.0, 3:22.0, 3:56.0; section lengths median 20 s (range 5 to 49). Reliable for sections of 10 s or more.
- **Boards of Canada - The Beach at Redpoint stem entries and exits:** drums plays 76% of the time (in: 0:43.5; out: 4:01.0); bass plays 67% of the time (in: 0:47.5, 1:10.5, 1:40.0; out: 1:00.0, 1:34.0, 3:56.0); other plays 99% of the time (in: 0:02.5; out: never); vocals plays 94% of the time (in: 0:03.0, 1:17.0; out: 1:12.5, 4:11.0).
- **Boards of Canada - The Beach at Redpoint floor drops (kick and bass out 2 s or more while the rest plays):** 1.03 a minute: 0:12.8 for 32.0 s, 0:45.2 for 3.0 s, 0:51.8 for 3.0 s, 3:56.8 for 8.8 s.
- **Boards of Canada - The Beach at Redpoint brightness sweeps (0.7 octave or more):** mix down 2.2 oct 0:00.0 to 0:07.0; mix up 0.8 oct 1:58.5 to 2:09.5; mix down 1.2 oct 2:10.5 to 2:22.0; mix up 0.9 oct 2:22.0 to 2:27.0; mix up 0.8 oct 2:38.0 to 2:43.5; mix up 0.8 oct 2:51.0 to 2:56.0; mix up 0.7 oct 3:19.5 to 3:25.0; mix down 0.9 oct 3:35.0 to 3:43.0 (11 more in techniques.json).
- **Boards of Canada - The Beach at Redpoint echo times:** drums: 414 ms (+6.5 dB, 2 later repeats).
- **Boards of Canada - The Beach at Redpoint pumping locked to the kick:** under 1.5 dB; none found.
- **Boards of Canada - The Beach at Redpoint tape wobble and speed:** nothing at 4 cents or more in the held partials that could be tracked.
- **Boards of Canada - The Beach at Redpoint stereo movement:** drums pan range 0.34.
- **Boards of Canada - Gyroscope sections:** 12 boundaries at 0:03.0, 0:36.0, 1:11.0, 1:24.0, 1:37.0, 1:49.0, 2:06.0, 2:23.0, 2:34.0, 2:46.0, 2:57.0, 3:28.0; section lengths median 13 s (range 3 to 35). Reliable for sections of 10 s or more.
- **Boards of Canada - Gyroscope stem entries and exits:** drums plays 96% of the time (in: 0:02.0; out: 3:28.0); bass plays 30% of the time (in: 1:04.5, 1:40.0, 1:49.5, 1:58.5, 2:18.5; out: 1:14.5, 1:46.5, 1:52.0, 2:07.0, 2:22.5); other plays 96% of the time (in: 0:02.0; out: 3:29.0); vocals plays 74% of the time (in: 0:47.5; out: 3:26.5).
- **Boards of Canada - Gyroscope floor drops (kick and bass out 2 s or more while the rest plays):** 0.00 a minute.
- **Boards of Canada - Gyroscope brightness sweeps (0.7 octave or more):** mix down 3.3 oct 0:00.0 to 0:05.5; mix up 1.3 oct 0:11.0 to 0:17.5; mix down 1.0 oct 0:18.0 to 0:23.5; mix up 0.9 oct 0:34.5 to 0:39.5; mix down 0.9 oct 0:39.5 to 0:45.0; mix up 0.7 oct 1:28.0 to 1:34.5; mix down 0.9 oct 1:34.5 to 1:39.5; mix down 1.0 oct 1:44.5 to 1:50.0 (30 more in techniques.json).
- **Boards of Canada - Gyroscope echo times:** none found reliably (needs 6 or more notes that ring alone for 1.5 s).
- **Boards of Canada - Gyroscope pumping locked to the kick:** bass dips 2.7 dB at the kick (peak 244 ms after, kick every 0.32 s). The tool reads about 70% of the true depth, so this is a floor.
- **Boards of Canada - Gyroscope tape wobble and speed:** other wobbles about 14 cents at 0.6 Hz; other speed moves up to 11 cents in 4 s.
- **Boards of Canada - Gyroscope stereo movement:** drums pan range 0.13, cycling every 5 s.
- **Boards of Canada - Sunshine Recorder sections:** 17 boundaries at 0:02.0, 0:25.0, 0:37.0, 0:49.0, 1:09.0, 1:25.0, 1:35.0, 1:49.0, 2:09.0, 2:44.0, 3:14.0, 3:38.0; section lengths median 20 s (range 2 to 35). Reliable for sections of 10 s or more.
- **Boards of Canada - Sunshine Recorder stem entries and exits:** drums plays 82% of the time (in: 0:23.0, 3:36.5; out: 3:25.5, 5:38.5); bass plays 62% of the time (in: 1:49.0, 3:37.5; out: 3:25.5, 5:51.0); other plays 99% of the time (in: from the start; out: 6:08.0); vocals plays 35% of the time (in: 2:50.0, 3:24.5, 3:36.0, 3:48.0, 4:03.0; out: 3:18.5, 3:29.5, 3:43.5, 3:55.5, 4:12.5).
- **Boards of Canada - Sunshine Recorder floor drops (kick and bass out 2 s or more while the rest plays):** 0.54 a minute: 0:18.5 for 5.2 s, 3:24.5 for 12.5 s, 5:49.0 for 4.8 s.
- **Boards of Canada - Sunshine Recorder brightness sweeps (0.7 octave or more):** mix up 1.4 oct 0:00.0 to 0:09.0; mix down 1.9 oct 0:59.5 to 1:04.5; mix down 1.0 oct 1:40.0 to 1:48.5; mix up 1.8 oct 2:03.5 to 2:12.5; mix up 0.8 oct 4:07.0 to 4:12.5; mix down 0.9 oct 4:36.5 to 4:42.0; mix up 1.3 oct 4:44.0 to 4:53.0; mix down 0.7 oct 5:25.5 to 5:32.0 (21 more in techniques.json).
- **Boards of Canada - Sunshine Recorder echo times:** none found reliably (needs 6 or more notes that ring alone for 1.5 s).
- **Boards of Canada - Sunshine Recorder pumping locked to the kick:** other dips 2.0 dB at the kick (peak 25 ms after, kick every 0.58 s); bass dips 3.6 dB at the kick (peak 264 ms after, kick every 0.58 s). The tool reads about 70% of the true depth, so this is a floor.
- **Boards of Canada - Sunshine Recorder tape wobble and speed:** other wobbles about 12 cents at 2.0 Hz; bass wobbles about 7 cents at 0.9 Hz; bass speed moves up to 30 cents in 4 s.
- **Boards of Canada - Sunshine Recorder stereo movement:** vocals pan range 0.35.
- **Boards of Canada - 1969 sections:** 9 boundaries at 0:14.0, 1:08.0, 1:33.0, 1:59.0, 2:23.0, 2:52.0, 3:51.0, 4:04.0, 4:16.0; section lengths median 24 s (range 4 to 59). Reliable for sections of 10 s or more.
- **Boards of Canada - 1969 stem entries and exits:** drums plays 88% of the time (in: 0:24.5; out: 4:14.0); bass plays 94% of the time (in: 0:11.5; out: 4:17.5); other plays 99% of the time (in: from the start; out: 4:17.5); vocals plays 45% of the time (in: 1:04.5, 1:17.5, 1:56.5, 2:10.0, 2:55.5; out: 1:15.0, 1:28.5, 2:07.5, 2:20.5, 3:06.0).
- **Boards of Canada - 1969 floor drops (kick and bass out 2 s or more while the rest plays):** 0.51 a minute: 1:53.5 for 3.0 s, 2:52.2 for 2.2 s.
- **Boards of Canada - 1969 brightness sweeps (0.7 octave or more):** mix down 3.0 oct 0:07.5 to 0:18.5; mix down 1.4 oct 0:24.0 to 0:30.0; mix up 3.0 oct 0:38.0 to 0:44.5; mix up 2.9 oct 0:59.5 to 1:09.0; mix up 3.2 oct 1:49.0 to 1:54.0; mix down 2.9 oct 2:19.0 to 2:28.5; mix up 2.9 oct 2:47.5 to 2:54.5; mix down 1.4 oct 2:54.5 to 2:59.5 (7 more in techniques.json).
- **Boards of Canada - 1969 echo times:** other: 279 ms (+3.6 dB, 1 later repeats); drums: 599 ms (+8.6 dB, 1 later repeats).
- **Boards of Canada - 1969 pumping locked to the kick:** under 1.5 dB; none found.
- **Boards of Canada - 1969 tape wobble and speed:** bass wobbles about 6 cents at 2.0 Hz; bass speed moves up to 34 cents in 4 s.
- **Boards of Canada - 1969 stereo movement:** other pan range 0.35, cycling every 50 s.
- **[JUDGMENT]** Pick the 1 to 3 moves above that define the record and name them (a filter arc, a floor drop, tape speed changes, chopped loops, a build).

### Theory
- **Boards of Canada - The Beach at Redpoint meter:** about 73 BPM (could be 36 or 146; the tool cannot tell the beat level), bar length unclear (best guess 4, score 0.26); straight eighths; no cross-rhythm claim; many unrelated periodicities found (3:2, 5:2, 7:4, 7:5), which means irregular timing, not a cross-rhythm claim.
- **Boards of Canada - The Beach at Redpoint key and mode:** G# phrygian (low confidence: the key signature margin is 0.004, so read this as a guess); pitch-class weights B 0.13, F# 0.10, E 0.10, C# 0.09, G# 0.09.
- **Boards of Canada - The Beach at Redpoint chords:** 28 changes a minute (20 a minute are chords that last 1.5 s or more); median chord 1.8 s (10th to 90th percentile 1.0 to 3.3 s); 20 distinct chords; most time on Bmaj7 18%, Emin7 11%, G#min7 10%, F#min7 9%. The old counter in this same run said 17; Ears 1's 2-s counter said 10.  71% of the changes land on a beat (chance would be 36%), which is the check available on real music. Counts are calibrated on clean synthetic chords (exact down to a chord every 0.5 s); chord names on dense or noisy music are a guess.
- **Boards of Canada - The Beach at Redpoint bass:** root-note movement / mixed; 0.52 notes a second; most time on G# 0.41, B 0.20, E 0.20, F# 0.16; steps 14%, thirds 2%, fourths and fifths 11%, octaves 20%, repeats 30%.
- **Boards of Canada - The Beach at Redpoint top line (from the 'other' stem; read by ear before trusting):** 642 notes, range 16 semitones (10th to 90th percentile); steps 12%, thirds 19%, fourths and fifths 18%, leaps 32%, repeats 20%; median interval 4 semitones; 50% of moves go up; phrases of about 7 notes, arch 28%, rising 22%, falling 22%, flat or wavy 28%. Reliable only when the line sits clearly above its accompaniment.
- **Boards of Canada - The Beach at Redpoint repetition:** 78 bars of 3.3 s; strongest repeat distances 5 bars (0.50), 1 bars (0.45), 10 bars (0.31); 9% of bars near-duplicate an earlier bar. Bar length is uncertain, so these lags are weak evidence.
- **Boards of Canada - Gyroscope meter:** about 94 BPM (could be 47 or 188; the tool cannot tell the beat level), 2 beats a bar; swung: off-beat at 0.76 of the beat; no cross-rhythm claim.
- **Boards of Canada - Gyroscope key and mode:** G phrygian (key signature margin 0.063); fits G pentatonic; pitch-class weights D# 0.15, G 0.14, G# 0.14, D 0.13, C 0.08.
- **Boards of Canada - Gyroscope chords:** 16 changes a minute (11 a minute are chords that last 1.5 s or more); median chord 2.4 s (10th to 90th percentile 0.8 to 7.8 s); 13 distinct chords; most time on G#maj7 59%, D#maj7 18%, C#maj7 9%, Emaj7 4%. The old counter in this same run said 11; Ears 1's 2-s counter said 6.  44% of the changes land on a beat (chance would be 47%), which is the check available on real music. Counts are calibrated on clean synthetic chords (exact down to a chord every 0.5 s); chord names on dense or noisy music are a guess.
- **Boards of Canada - Gyroscope bass:** root-note movement / mixed; 0.06 notes a second; most time on G 0.32, G# 0.23, D# 0.20, C# 0.10; steps 33%, thirds 8%, fourths and fifths 17%, octaves 0%, repeats 25%.
- **Boards of Canada - Gyroscope top line (from the 'other' stem; read by ear before trusting):** 434 notes, range 8 semitones (10th to 90th percentile); steps 45%, thirds 6%, fourths and fifths 15%, leaps 8%, repeats 25%; median interval 1 semitones; 50% of moves go up; phrases of about 14 notes, arch 5%, rising 29%, falling 29%, flat or wavy 38%. Reliable only when the line sits clearly above its accompaniment.
- **Boards of Canada - Gyroscope repetition:** 168 bars of 1.3 s; strongest repeat distances 1 bars (0.39), 18 bars (0.14), 12 bars (0.12); 1% of bars near-duplicate an earlier bar.
- **Boards of Canada - Sunshine Recorder meter:** about 100 BPM (could be 50 or 199; the tool cannot tell the beat level), 4 beats a bar; straight eighths; no cross-rhythm claim; many unrelated periodicities found (3:2, 4:3, 5:2, 5:3), which means irregular timing, not a cross-rhythm claim.
- **Boards of Canada - Sunshine Recorder key and mode:** E phrygian (low confidence: the key signature margin is 0.004, so read this as a guess); pitch-class weights E 0.13, D 0.12, G 0.10, B 0.10, A# 0.10.
- **Boards of Canada - Sunshine Recorder chords:** 18 changes a minute (16 a minute are chords that last 1.5 s or more); median chord 2.9 s (10th to 90th percentile 1.2 to 6.0 s); 20 distinct chords; most time on Emin7 33%, Gmin7 26%, Fmaj7 11%, Bmin7 9%. The old counter in this same run said 7; Ears 1's 2-s counter said 6.  56% of the changes land on a beat (chance would be 50%), which is the check available on real music. Counts are calibrated on clean synthetic chords (exact down to a chord every 0.5 s); chord names on dense or noisy music are a guess.
- **Boards of Canada - Sunshine Recorder bass:** root-note movement / mixed; 0.37 notes a second; most time on E 0.51, A# 0.25, G 0.22; steps 7%, thirds 6%, fourths and fifths 15%, octaves 1%, repeats 63%.
- **Boards of Canada - Sunshine Recorder top line (from the 'other' stem; read by ear before trusting):** 699 notes, range 18 semitones (10th to 90th percentile); steps 7%, thirds 29%, fourths and fifths 9%, leaps 35%, repeats 20%; median interval 4 semitones; 50% of moves go up; phrases of about 5 notes, arch 39%, rising 30%, falling 19%, flat or wavy 12%. Reliable only when the line sits clearly above its accompaniment.
- **Boards of Canada - Sunshine Recorder repetition:** 154 bars of 2.4 s; strongest repeat distances 1 bars (0.50), 5 bars (0.48), 10 bars (0.31); 7% of bars near-duplicate an earlier bar.
- **Boards of Canada - 1969 meter:** about 74 BPM (could be 37 or 147; the tool cannot tell the beat level), 2 beats a bar; straight eighths; no cross-rhythm claim; many unrelated periodicities found (3:2, 4:3, 5:2, 5:3), which means irregular timing, not a cross-rhythm claim.
- **Boards of Canada - 1969 key and mode:** C# ionian (major) (low confidence: the key signature margin is 0.018, so read this as a guess); fits C# pentatonic; pitch-class weights C# 0.29, C 0.14, E 0.09, F# 0.09, B 0.06.
- **Boards of Canada - 1969 chords:** 28 changes a minute (20 a minute are chords that last 1.5 s or more); median chord 1.6 s (10th to 90th percentile 1.4 to 3.4 s); 12 distinct chords; most time on C#maj7 49%, Emaj7 22%, Bsus2 9%, F#5 6%. The old counter in this same run said 27; Ears 1's 2-s counter said 25.  78% of the changes land on a beat (chance would be 37%), which is the check available on real music. Counts are calibrated on clean synthetic chords (exact down to a chord every 0.5 s); chord names on dense or noisy music are a guess.
- **Boards of Canada - 1969 bass:** ostinato (a 3-note figure repeats); 0.44 notes a second; most time on C# 0.52, E 0.24, F# 0.24; steps 31%, thirds 0%, fourths and fifths 31%, octaves 0%, repeats 4%.
- **Boards of Canada - 1969 top line (from the 'other' stem; read by ear before trusting):** 721 notes, range 12 semitones (10th to 90th percentile); steps 18%, thirds 12%, fourths and fifths 25%, leaps 23%, repeats 22%; median interval 4 semitones; 50% of moves go up; phrases of about 18 notes, arch 21%, rising 16%, falling 37%, flat or wavy 26%. Reliable only when the line sits clearly above its accompaniment.
- **Boards of Canada - 1969 repetition:** 159 bars of 1.6 s; strongest repeat distances 8 bars (0.54), 4 bars (0.48), 32 bars (0.47); 21% of bars near-duplicate an earlier bar.
- **[JUDGMENT]** Pick 1 or 2 of the readings above and write each as something the generator could draw (a spread, not a rule).

### Space
- **Boards of Canada - The Beach at Redpoint mix width by band** (width dB; 0 = unrelated channels, below -20 = near mono): sub -13 (corr 0.91); low_120_500 -14 (corr 0.93); mid_500_2k -14 (corr 0.93); pres_2k_5k -10 (corr 0.82); air_gt5k -7 (corr 0.68); over time (2-s windows) 10th/50th/90th percentile -15/-14/-11 dB.
- **Boards of Canada - The Beach at Redpoint by part:** drums: 27% of the power, width -10 dB, pan +0.02, mono loss 0.4 dB, widest in low_120_500 (-10 dB); bass: 24% of the power, width -24 dB, pan +0.05, mono loss 0.0 dB, widest in sub_lt120 (-24 dB); other: 32% of the power, width -14 dB, pan -0.03, mono loss 0.2 dB, widest in low_120_500 (-14 dB); vocals: 18% of the power, width -17 dB, pan -0.09, mono loss 0.1 dB, widest in mid_500_2k (-16 dB). Bands holding under 10% of a part's energy are left out of 'widest' (they are too quiet to trust).
- **Boards of Canada - Gyroscope mix width by band** (width dB; 0 = unrelated channels, below -20 = near mono): sub -5 (corr 0.56); low_120_500 -8 (corr 0.70); mid_500_2k -2 (corr 0.22); pres_2k_5k -0 (corr 0.00); air_gt5k -0 (corr 0.00); over time (2-s windows) 10th/50th/90th percentile -8/-6/-5 dB.
- **Boards of Canada - Gyroscope by part:** drums: 69% of the power, width -5 dB, pan -0.17, mono loss 1.1 dB, widest in sub_lt120 (-5 dB); bass: 0% of the power, width -16 dB, pan -0.00, mono loss 0.1 dB, widest in sub_lt120 (-13 dB); other: 26% of the power, width -8 dB, pan +0.06, mono loss 0.6 dB, widest in low_120_500 (-8 dB); vocals: 4% of the power, width -3 dB, pan +0.10, mono loss 1.7 dB, widest in mid_500_2k (-2 dB). Bands holding under 10% of a part's energy are left out of 'widest' (they are too quiet to trust).
- **Boards of Canada - Sunshine Recorder mix width by band** (width dB; 0 = unrelated channels, below -20 = near mono): sub -27 (corr 1.00); low_120_500 -6 (corr 0.59); mid_500_2k -10 (corr 0.82); pres_2k_5k -12 (corr 0.88); air_gt5k -15 (corr 0.94); over time (2-s windows) 10th/50th/90th percentile -17/-13/-4 dB.
- **Boards of Canada - Sunshine Recorder by part:** drums: 31% of the power, width -24 dB, pan +0.05, mono loss 0.0 dB, widest in air_gt5k (-18 dB); bass: 16% of the power, width -46 dB, pan +0.00, mono loss 0.0 dB, widest in low_120_500 (-42 dB); other: 47% of the power, width -9 dB, pan -0.11, mono loss 0.5 dB, widest in low_120_500 (-8 dB); vocals: 6% of the power, width 2 dB, pan +0.06, mono loss 4.1 dB, widest in low_120_500 (4 dB). Bands holding under 10% of a part's energy are left out of 'widest' (they are too quiet to trust).
- **Boards of Canada - 1969 mix width by band** (width dB; 0 = unrelated channels, below -20 = near mono): sub -27 (corr 1.00); low_120_500 -9 (corr 0.77); mid_500_2k -3 (corr 0.32); pres_2k_5k -5 (corr 0.49); air_gt5k -12 (corr 0.90); over time (2-s windows) 10th/50th/90th percentile -10/-5/-2 dB.
- **Boards of Canada - 1969 by part:** drums: 39% of the power, width -22 dB, pan +0.05, mono loss 0.0 dB, widest in low_120_500 (-23 dB); bass: 36% of the power, width -59 dB, pan +0.00, mono loss 0.0 dB, widest in sub_lt120 (-62 dB); other: 22% of the power, width -2 dB, pan -0.16, mono loss 2.2 dB, widest in mid_500_2k (-2 dB); vocals: 3% of the power, width -7 dB, pan +0.03, mono loss 0.9 dB, widest in pres_2k_5k (-1 dB). Bands holding under 10% of a part's energy are left out of 'widest' (they are too quiet to trust).

### Vibes
- **[JUDGMENT]** For each sound and technique: vibes that lean toward it (x1.5 to x3), ones that lean away (x0.2 to x0.5, never 0), any new vibe or profile. Numbers to start from: Boards of Canada - The Beach at Redpoint: 73 BPM, 20 settled chords a minute, 4% of energy above 2 kHz, width -14 dB; Boards of Canada - Gyroscope: 94 BPM, 11 settled chords a minute, 1% of energy above 2 kHz, width -6 dB; Boards of Canada - Sunshine Recorder: 100 BPM, 16 settled chords a minute, 11% of energy above 2 kHz, width -9 dB; Boards of Canada - 1969: 74 BPM, 20 settled chords a minute, 6% of energy above 2 kHz, width -10 dB.

### Wholesome take
- **[JUDGMENT]** What to take, what to leave. Measured flags to consider: no brightness or polarity flags from the numbers; listening still decides.

### For later
- Boards of Canada - 1969, bass group `bass-2` (204 notes): around F#1, harmonic (string/reed/pad-like), soft attack, falls 0 dB/s, centered; hear it at 0:20.5, 0:36.1. **[JUDGMENT]** why it waits.
- Boards of Canada - Sunshine Recorder, other group `other-2` (163 notes): around B3, bell- or metal-like (partials off whole multiples), slow swell, falls 11 dB/s, centered; hear it at 3:31.7, 3:34.8. **[JUDGMENT]** why it waits.
- Boards of Canada - Sunshine Recorder, drums group `drums-1` (221 notes): low thump, soft attack, falls 96 dB/s, gets brighter over the note, centered; hear it at 2:24.8, 0:48.3. **[JUDGMENT]** why it waits.
- Boards of Canada - Sunshine Recorder, drums group `drums-2` (258 notes): high tick, soft attack, falls 122 dB/s, centered; hear it at 1:25.0, 0:36.7. **[JUDGMENT]** why it waits.
- Boards of Canada - Gyroscope, drums group `drums-2` (250 notes): low thump, soft attack, falls 139 dB/s, centered; hear it at 0:36.7, 0:16.2. **[JUDGMENT]** why it waits.
- Boards of Canada - Gyroscope, other group `other-2` (142 notes): around G2, bell- or metal-like (partials off whole multiples), slow swell, falls 13 dB/s, centered; hear it at 2:41.5, 2:40.4. **[JUDGMENT]** why it waits.
- Boards of Canada - 1969, other group `other-1` (429 notes): around E3, mixed or noisy partials, soft attack, falls 14 dB/s, centered; hear it at 0:02.7, 0:03.3. **[JUDGMENT]** why it waits.
- Boards of Canada - The Beach at Redpoint, drums group `drums-1` (299 notes): low thump, soft attack, falls 58 dB/s, centered; hear it at 3:02.3, 1:50.0. **[JUDGMENT]** why it waits.
- **Boards of Canada - Diving Station** (1:26.7, 99 BPM by the mix-level count): stands out for pitch bends a minute 74.04 (high, album median 10.36). **[JUDGMENT]** listen and decide if it is worth a deep pass; why it waits is not measurable.
- **Boards of Canada - Ready Lets Go** (0:59.0, 123 BPM by the mix-level count): stands out for wobble (cents) 10.85 (high, album median 5.07). **[JUDGMENT]** listen and decide if it is worth a deep pass; why it waits is not measurable.
- **Boards of Canada - In the Annexe** (1:22.1, 144 BPM by the mix-level count): stands out for width (side/mid dB) -23.60 (low, album median -8.00). **[JUDGMENT]** listen and decide if it is worth a deep pass; why it waits is not measurable.
- **Boards of Canada - The Smallest Weird Number** (1:17.2, 129 BPM by the mix-level count): stands out for width (side/mid dB) -20.70 (low, album median -8.00). **[JUDGMENT]** listen and decide if it is worth a deep pass; why it waits is not measurable.
- **Boards of Canada - Beware the Friendly Stranger** (0:37.6, 108 BPM by the mix-level count): no outstanding number. Too short for the deep pass (under 45 s). **[JUDGMENT]** listen and decide if it is worth a deep pass; why it waits is not measurable.
- **Boards of Canada - Energy Warning** (0:35.3, 112 BPM by the mix-level count): no outstanding number. Too short for the deep pass (under 45 s). **[JUDGMENT]** listen and decide if it is worth a deep pass; why it waits is not measurable.
- **Boards of Canada - I Saw Drones** (0:27.2, 112 BPM by the mix-level count): no outstanding number. Too short for the deep pass (under 45 s). **[JUDGMENT]** listen and decide if it is worth a deep pass; why it waits is not measurable.


---

## Galaxy Garden (draft card from Ears 3 tools, 2026-10-15)

How it was made: 12 usable tracks: the most typical plus the 3 least like the rest. Deep pass on: Lying in the Reeds; Crystal Caverns 1991; New Colour; Stands Tidal Waves. Typical track: Lying in the Reeds. Least like the rest: Crystal Caverns 1991, New Colour, Stands Tidal Waves.
Stems from Demucs htdemucs (drums, bass, other, vocals). Lines marked **[JUDGMENT]** need a person or a later session; every other line is a measurement or a threshold read off one.

### The hook
- Numbers that repeat across the deep-pass tracks: tempo 99 to 140 BPM (octave-uncertain); keys A phrygian, C aeolian, A dorian, F phrygian; mix width -13 to -1 dB; chord changes 28 to 49 a minute.
- **[JUDGMENT]** The one thing that makes a listener say "that's them", in one line, after listening at the timestamps under Sounds.

### Sounds
Groups are ranked by share of the track's note energy, at most two per track. Examples are the clearest notes (loud against everything else playing).
#### S1. Stands Tidal Waves: other group `other-1` (108 notes, 60% of the track's note energy)
- In synth terms: around A#3, mixed or noisy partials, slow swell, falls 7 dB/s, centered.
- Hear it at: 1:23.3 (G#3, 1.48 s, +56 dB alone), 1:38.4 (G#3, 0.54 s, +58 dB alone), 1:31.1 (G#3, 0.76 s, +57 dB alone).
- Fingerprint, median (10th to 90th percentile):
  - register 233 (155 to 312) Hz; attack 186 (41 to 479) ms; decay -7 (-35 to -0) dB/s
  - brightness at the start 1527 (974 to 1766) Hz; slope over the note 0.00 (-0.33 to 0.39) octaves
  - harmonic share 0.72 (0.46 to 0.93); partials off whole multiples 38 (15 to 82) cents
  - vibrato 0 (0 to 3) cents; detune beating 0.0 (0.0 to 0.0) Hz
  - level against the mix -1.4 (-2.2 to -1.1) dB; pan 0.07 (0.02 to 0.13); left-right correlation 0.48 (0.28 to 0.56)
- Building blocks it would need: **[JUDGMENT]** name the closest existing Driftloom block, or the one missing.
- Yardstick: the percentile ranges above, measured on the other stem (Demucs), not on the raw mix.

#### S2. New Colour: bass group `bass-1` (142 notes, 41% of the track's note energy)
- In synth terms: around G1, harmonic (string/reed/pad-like), soft attack, falls 10 dB/s, centered.
- Hear it at: 0:41.5 (F#1, 1.83 s, +1 dB alone), 0:34.2 (C#1, 1.79 s, +1 dB alone), 0:52.7 (E1, 1.81 s, -0 dB alone).
- Fingerprint, median (10th to 90th percentile):
  - register 49 (41 to 55) Hz; attack 129 (44 to 538) ms; decay -10 (-22 to -2) dB/s
  - brightness at the start 126 (109 to 141) Hz; slope over the note -0.18 (-0.38 to 0.00) octaves
  - harmonic share 0.98 (0.94 to 1.00); partials off whole multiples 8 (4 to 16) cents
  - vibrato 7 (0 to 18) cents; detune beating 0.0 (0.0 to 0.6) Hz
  - level against the mix -4.0 (-6.1 to -2.4) dB; pan 0.00 (-0.00 to 0.01); left-right correlation 1.00 (1.00 to 1.00)
- Building blocks it would need: **[JUDGMENT]** name the closest existing Driftloom block, or the one missing.
- Yardstick: the percentile ranges above, measured on the bass stem (Demucs), not on the raw mix.

#### S3. Stands Tidal Waves: other group `other-2` (160 notes, 38% of the track's note energy)
- In synth terms: around A#1, mixed or noisy partials, slow swell, falls 10 dB/s, centered.
- Hear it at: 0:01.5 (A#1, 1.27 s, +52 dB alone), 0:08.0 (A#1, 0.89 s, +52 dB alone), 0:15.1 (A#1, 0.89 s, +52 dB alone).
- Fingerprint, median (10th to 90th percentile):
  - register 58 (39 to 93) Hz; attack 194 (61 to 530) ms; decay -10 (-30 to 1) dB/s
  - brightness at the start 398 (343 to 480) Hz; slope over the note 0.00 (-0.11 to 0.32) octaves
  - harmonic share 0.63 (0.19 to 0.86); partials off whole multiples 49 (28 to 91) cents
  - vibrato 0 (0 to 5) cents; detune beating 0.0 (0.0 to 0.0) Hz
  - level against the mix -4.6 (-8.1 to -3.2) dB; pan -0.13 (-0.26 to 0.02); left-right correlation -0.18 (-0.35 to 0.24)
- Building blocks it would need: **[JUDGMENT]** name the closest existing Driftloom block, or the one missing.
- Yardstick: the percentile ranges above, measured on the other stem (Demucs), not on the raw mix.

#### S4. Lying in the Reeds: bass group `bass-1` (279 notes, 31% of the track's note energy)
- In synth terms: around A1, harmonic (string/reed/pad-like), soft attack, falls 31 dB/s, centered.
- Hear it at: 2:53.5 (A1, 0.49 s, +13 dB alone), 2:54.6 (A#1, 0.26 s, +12 dB alone), 2:53.4 (A1, 0.28 s, +11 dB alone).
- Fingerprint, median (10th to 90th percentile):
  - register 55 (55 to 65) Hz; attack 119 (43 to 336) ms; decay -31 (-48 to -5) dB/s
  - brightness at the start 94 (85 to 113) Hz; slope over the note -0.13 (-0.33 to 0.00) octaves
  - harmonic share 0.99 (0.95 to 1.00); partials off whole multiples 8 (2 to 31) cents
  - vibrato 0 (0 to 0) cents; detune beating 0.0 (0.0 to 0.0) Hz
  - level against the mix -2.7 (-4.3 to -1.7) dB; pan -0.00 (-0.01 to 0.00); left-right correlation 1.00 (1.00 to 1.00)
- Building blocks it would need: **[JUDGMENT]** name the closest existing Driftloom block, or the one missing.
- Yardstick: the percentile ranges above, measured on the bass stem (Demucs), not on the raw mix.

#### S5. New Colour: other group `other-1` (325 notes, 26% of the track's note energy)
- In synth terms: around F2, bell- or metal-like (partials off whole multiples), soft attack, falls 28 dB/s, centered.
- Hear it at: 3:23.3 (G#2, 0.12 s, +53 dB alone), 3:20.6 (G3, 0.21 s, +51 dB alone), 3:28.3 (B2, 0.33 s, +48 dB alone).
- Fingerprint, median (10th to 90th percentile):
  - register 88 (48 to 143) Hz; attack 73 (15 to 235) ms; decay -28 (-77 to -6) dB/s
  - brightness at the start 844 (413 to 1412) Hz; slope over the note 0.00 (-0.54 to 0.44) octaves
  - harmonic share 0.49 (0.18 to 0.85); partials off whole multiples 64 (30 to 134) cents
  - vibrato 0 (0 to 0) cents; detune beating 0.0 (0.0 to 0.0) Hz
  - level against the mix -2.4 (-4.1 to -0.7) dB; pan 0.03 (-0.02 to 0.08); left-right correlation 0.87 (0.70 to 0.93)
- Building blocks it would need: **[JUDGMENT]** name the closest existing Driftloom block, or the one missing.
- Yardstick: the percentile ranges above, measured on the other stem (Demucs), not on the raw mix.

#### S6. Crystal Caverns 1991: drums group `drums-1` (335 notes, 26% of the track's note energy)
- In synth terms: mid snap, soft attack, falls 54 dB/s, gets brighter over the note, centered.
- Hear it at: 0:12.4 (F1, 0.32 s, +52 dB alone), 0:12.0 (F1, 0.43 s, +52 dB alone), 0:13.1 (F1, 0.22 s, +52 dB alone).
- Fingerprint, median (10th to 90th percentile):
  - register 43 (43 to 44) Hz; attack 58 (55 to 81) ms; decay -54 (-68 to -36) dB/s
  - brightness at the start 1989 (1317 to 2941) Hz; slope over the note 0.42 (-0.49 to 1.51) octaves
  - harmonic share 0.99 (0.91 to 1.00); partials off whole multiples 86 (21 to 220) cents
  - vibrato 0 (0 to 0) cents; detune beating 0.0 (0.0 to 0.0) Hz
  - level against the mix -1.7 (-3.0 to -0.1) dB; pan 0.01 (0.00 to 0.01); left-right correlation 1.00 (0.99 to 1.00)
- Building blocks it would need: **[JUDGMENT]** name the closest existing Driftloom block, or the one missing.
- Yardstick: the percentile ranges above, measured on the drums stem (Demucs), not on the raw mix.

#### S7. Crystal Caverns 1991: bass group `bass-1` (306 notes, 24% of the track's note energy)
- In synth terms: around D#1, bell- or metal-like (partials off whole multiples), slow swell, falls 50 dB/s, centered.
- Hear it at: 3:16.7 (F1, 2.18 s, +4 dB alone), 1:39.0 (D#1, 2.14 s, +3 dB alone), 1:49.5 (F1, 0.18 s, +7 dB alone).
- Fingerprint, median (10th to 90th percentile):
  - register 39 (36 to 44) Hz; attack 168 (54 to 446) ms; decay -50 (-278 to 6) dB/s
  - brightness at the start 97 (82 to 122) Hz; slope over the note 0.00 (-0.24 to 0.35) octaves
  - harmonic share 0.90 (0.08 to 1.00); partials off whole multiples 184 (51 to 296) cents
  - vibrato 0 (0 to 0) cents; detune beating 0.0 (0.0 to 0.0) Hz
  - level against the mix -3.6 (-5.1 to -1.7) dB; pan -0.02 (-0.03 to -0.01); left-right correlation 1.00 (0.99 to 1.00)
- Building blocks it would need: **[JUDGMENT]** name the closest existing Driftloom block, or the one missing.
- Yardstick: the percentile ranges above, measured on the bass stem (Demucs), not on the raw mix.

#### S8. Lying in the Reeds: bass group `bass-2` (78 notes, 13% of the track's note energy)
- In synth terms: around E1, bell- or metal-like (partials off whole multiples), soft attack, falls 24 dB/s, centered.
- Hear it at: 2:52.5 (E1, 0.36 s, +11 dB alone), 2:50.2 (E1, 0.28 s, +9 dB alone), 2:52.0 (E1, 0.49 s, +6 dB alone).
- Fingerprint, median (10th to 90th percentile):
  - register 41 (41 to 50) Hz; attack 136 (85 to 351) ms; decay -24 (-40 to -8) dB/s
  - brightness at the start 109 (91 to 117) Hz; slope over the note -0.18 (-0.38 to -0.06) octaves
  - harmonic share 0.67 (0.03 to 0.99); partials off whole multiples 86 (4 to 259) cents
  - vibrato 0 (0 to 12) cents; detune beating 0.0 (0.0 to 0.0) Hz
  - level against the mix -2.4 (-3.1 to -1.7) dB; pan -0.00 (-0.01 to -0.00); left-right correlation 1.00 (1.00 to 1.00)
- Building blocks it would need: **[JUDGMENT]** name the closest existing Driftloom block, or the one missing.
- Yardstick: the percentile ranges above, measured on the bass stem (Demucs), not on the raw mix.

### Techniques
- **Lying in the Reeds sections:** 9 boundaries at 0:33.0, 0:49.0, 1:07.0, 1:44.0, 2:01.0, 2:32.0, 2:48.0, 3:12.0, 3:35.0; section lengths median 24 s (range 16 to 56). Reliable for sections of 10 s or more.
- **Lying in the Reeds stem entries and exits:** drums plays 86% of the time (in: 2:55.5; out: 2:46.5, 4:03.5); bass plays 57% of the time (in: 0:31.5, 2:15.5; out: 1:45.0, 3:37.0); other plays 97% of the time (in: from the start; out: 4:23.0); vocals plays 55% of the time (in: 1:03.5, 1:34.5, 3:11.0, 3:43.0; out: 1:21.5, 2:49.0, 3:40.0, 4:12.0).
- **Lying in the Reeds floor drops (kick and bass out 2 s or more while the rest plays):** 0.74 a minute: 0:29.8 for 2.2 s, 1:44.0 for 16.0 s, 3:36.0 for 37.0 s.
- **Lying in the Reeds brightness sweeps (0.7 octave or more):** mix down 1.6 oct 0:26.5 to 0:37.0; mix up 0.9 oct 1:01.0 to 1:06.0; mix down 0.9 oct 1:14.5 to 1:25.0; mix up 0.8 oct 1:33.0 to 1:38.5; mix up 0.7 oct 1:41.0 to 1:46.0; mix down 0.7 oct 2:10.5 to 2:15.5; mix down 0.9 oct 2:36.5 to 2:41.5; mix down 2.4 oct 2:45.0 to 2:52.5 (7 more in techniques.json).
- **Lying in the Reeds echo times:** none found reliably (needs 6 or more notes that ring alone for 1.5 s).
- **Lying in the Reeds pumping locked to the kick:** other dips 8.3 dB at the kick (peak 25 ms after, kick every 0.50 s); bass dips 2.5 dB at the kick (peak 35 ms after, kick every 0.50 s). The tool reads about 70% of the true depth, so this is a floor.
- **Lying in the Reeds tape wobble and speed:** nothing at 4 cents or more in the held partials that could be tracked.
- **Lying in the Reeds stereo movement:** other pan range 0.26; vocals pan range 0.25.
- **Crystal Caverns 1991 sections:** 11 boundaries at 0:01.0, 0:14.0, 1:03.0, 1:19.0, 1:35.0, 2:53.0, 3:09.0, 3:22.0, 3:40.0, 4:21.0, 4:48.0; section lengths median 16 s (range 1 to 78). Reliable for sections of 10 s or more.
- **Crystal Caverns 1991 stem entries and exits:** drums plays 97% of the time (in: 2:50.5; out: 2:45.0, 4:48.5); bass plays 65% of the time (in: 1:01.0, 4:20.0; out: 3:41.5, 4:49.0); other plays 90% of the time (in: 0:13.0, 3:04.5; out: 2:52.0, 4:49.5); vocals plays 29% of the time (in: 1:45.5, 1:57.5, 2:13.0, 2:26.5, 2:54.0; out: 1:52.0, 2:09.5, 2:18.0, 2:36.5, 3:28.0).
- **Crystal Caverns 1991 floor drops (kick and bass out 2 s or more while the rest plays):** 2.74 a minute: 0:25.5 for 7.5 s, 3:40.0 for 2.5 s, 3:43.5 for 2.8 s, 3:47.0 for 2.5 s, 3:50.2 for 2.8 s, 3:53.5 for 2.8 s, 3:57.0 for 3.0 s, 4:00.5 for 2.8 s.
- **Crystal Caverns 1991 brightness sweeps (0.7 octave or more):** mix up 2.2 oct 0:25.5 to 0:33.5; mix down 2.1 oct 1:00.5 to 1:08.0; mix down 1.0 oct 1:45.0 to 1:51.5; mix up 2.9 oct 1:51.5 to 2:00.0; mix down 1.3 oct 2:33.5 to 2:40.0; mix up 2.8 oct 2:40.0 to 2:45.0; mix down 3.3 oct 2:45.0 to 2:53.5; mix down 1.5 oct 2:57.5 to 3:04.5 (16 more in techniques.json).
- **Crystal Caverns 1991 echo times:** none found reliably (needs 6 or more notes that ring alone for 1.5 s).
- **Crystal Caverns 1991 pumping locked to the kick:** other dips 4.4 dB at the kick (peak 85 ms after, kick every 0.44 s); bass dips 5.8 dB at the kick (peak 45 ms after, kick every 0.44 s). The tool reads about 70% of the true depth, so this is a floor.
- **Crystal Caverns 1991 tape wobble and speed:** nothing at 4 cents or more in the held partials that could be tracked.
- **Crystal Caverns 1991 stereo movement:** parts stay near where they started (pan range under 0.25 and no clear cycle).
- **New Colour sections:** 9 boundaries at 0:02.0, 0:16.0, 0:31.0, 1:07.0, 1:21.0, 1:48.0, 2:14.0, 3:14.0, 3:29.0; section lengths median 20 s (range 2 to 71). Reliable for sections of 10 s or more.
- **New Colour stem entries and exits:** drums plays 70% of the time (in: 0:15.0; out: 3:32.0); bass plays 59% of the time (in: 0:29.5; out: 3:14.0); other plays 95% of the time (in: from the start; out: 4:26.5); vocals plays 31% of the time (in: 0:45.0, 1:01.0, 1:15.5, 1:33.5, 1:41.0; out: 0:48.5, 1:11.5, 1:23.5, 1:37.0, 1:45.0).
- **New Colour floor drops:** low end too sparse to say (on in under 75% of the track).
- **New Colour brightness sweeps (0.7 octave or more):** mix down 1.8 oct 1:23.5 to 1:31.5; mix up 1.6 oct 1:31.5 to 1:39.5; mix up 1.2 oct 2:02.0 to 2:09.5; mix down 1.0 oct 2:16.0 to 2:24.0; mix down 1.2 oct 2:39.0 to 2:49.0; mix up 1.3 oct 2:49.0 to 2:58.0; mix up 1.8 oct 3:10.0 to 3:23.0; other up 2.0 oct 0:41.0 to 0:47.0 (12 more in techniques.json).
- **New Colour echo times:** other: 274 ms (+3.5 dB, 2 later repeats).
- **New Colour pumping locked to the kick:** other dips 3.7 dB at the kick (peak 45 ms after, kick every 0.40 s); bass dips 2.2 dB at the kick (peak 55 ms after, kick every 0.40 s). The tool reads about 70% of the true depth, so this is a floor.
- **New Colour tape wobble and speed:** nothing at 4 cents or more in the held partials that could be tracked.
- **New Colour stereo movement:** parts stay near where they started (pan range under 0.25 and no clear cycle).
- **New Colour builds:** 0:00.0 to 0:21.0.
- **Stands Tidal Waves sections:** 3 boundaries at 0:02.0, 0:35.0, 0:58.0; section lengths median 28 s (range 2 to 49). Reliable for sections of 10 s or more.
- **Stands Tidal Waves stem entries and exits:** drums plays 17% of the time (in: 0:20.5, 0:49.0; out: 0:33.0, 0:54.5); bass plays 21% of the time (in: 0:35.0; out: 0:57.5); other plays 94% of the time (in: from the start; out: 1:40.5); vocals plays 26% of the time (in: 0:29.5, 1:39.5; out: 0:54.5, 1:42.0).
- **Stands Tidal Waves floor drops:** low end too sparse to say (on in under 75% of the track).
- **Stands Tidal Waves brightness sweeps (0.7 octave or more):** mix down 1.0 oct 0:28.0 to 0:37.5; mix up 2.1 oct 0:52.5 to 1:01.0; other up 1.6 oct 0:53.5 to 1:01.0.
- **Stands Tidal Waves echo times:** none found reliably (needs 6 or more notes that ring alone for 1.5 s).
- **Stands Tidal Waves pumping locked to the kick:** under 1.5 dB; none found.
- **Stands Tidal Waves tape wobble and speed:** bass wobbles about 6 cents at 6.3 Hz; bass speed moves up to 30 cents in 4 s.
- **Stands Tidal Waves stereo movement:** drums pan range 0.57; other pan range 0.34; vocals pan range 0.50.
- **[JUDGMENT]** Pick the 1 to 3 moves above that define the record and name them (a filter arc, a floor drop, tape speed changes, chopped loops, a build).

### Theory
- **Lying in the Reeds meter:** about 120 BPM (could be 60 or 240; the tool cannot tell the beat level), bar length unclear (best guess 8, score 0.04); straight eighths; no cross-rhythm claim; many unrelated periodicities found (4:3, 5:2, 5:3, 5:4), which means irregular timing, not a cross-rhythm claim.
- **Lying in the Reeds key and mode:** A phrygian (key signature margin 0.037); pitch-class weights A 0.12, D 0.11, E 0.11, C 0.10, G 0.10.
- **Lying in the Reeds chords:** 28 changes a minute (16 a minute are chords that last 1.5 s or more); median chord 1.9 s (10th to 90th percentile 0.8 to 5.9 s); 19 distinct chords; most time on Amaj7 13%, Dsus2 13%, Dmin7 11%, Gmin7 10%. The old counter in this same run said 14; Ears 1's 2-s counter said 12.  78% of the changes land on a beat (chance would be 60%), which is the check available on real music. Counts are calibrated on clean synthetic chords (exact down to a chord every 0.5 s); chord names on dense or noisy music are a guess.
- **Lying in the Reeds bass:** root-note movement / mixed; 1.17 notes a second; most time on A 0.58, E 0.14, C 0.08, A# 0.08; steps 22%, thirds 2%, fourths and fifths 13%, octaves 0%, repeats 62%.
- **Lying in the Reeds top line (from the 'other' stem; read by ear before trusting):** 925 notes, range 17 semitones (10th to 90th percentile); steps 12%, thirds 12%, fourths and fifths 33%, leaps 36%, repeats 7%; median interval 7 semitones; 53% of moves go up; phrases of about 16 notes, arch 58%, rising 15%, falling 19%, flat or wavy 8%. Reliable only when the line sits clearly above its accompaniment.
- **Lying in the Reeds repetition:** 67 bars of 4.0 s; strongest repeat distances 2 bars (0.74), 4 bars (0.54), 28 bars (0.39); 56% of bars near-duplicate an earlier bar. Bar length is uncertain, so these lags are weak evidence.
- **Crystal Caverns 1991 meter:** about 140 BPM (could be 70 or 280; the tool cannot tell the beat level), 4 beats a bar; swung: off-beat at 0.75 of the beat; no cross-rhythm claim.
- **Crystal Caverns 1991 key and mode:** C aeolian (minor) (low confidence: the key signature margin is 0.010, so read this as a guess); pitch-class weights C 0.15, D# 0.11, A# 0.10, D 0.09, C# 0.08.
- **Crystal Caverns 1991 chords:** 49 changes a minute (10 a minute are chords that last 1.5 s or more); median chord 1.1 s (10th to 90th percentile 0.6 to 2.0 s); 25 distinct chords; most time on Fmaj7 13%, F5 12%, Gmin7 11%, D#maj7 9%. The old counter in this same run said 21; Ears 1's 2-s counter said 6.  73% of the changes land on a beat (chance would be 70%), which is the check available on real music. Counts are calibrated on clean synthetic chords (exact down to a chord every 0.5 s); chord names on dense or noisy music are a guess.
- **Crystal Caverns 1991 bass:** root-note movement / mixed; 0.77 notes a second; most time on C 0.36, A# 0.30, F 0.16, D# 0.14; steps 15%, thirds 0%, fourths and fifths 13%, octaves 0%, repeats 71%.
- **Crystal Caverns 1991 top line (from the 'other' stem; read by ear before trusting):** 736 notes, range 22 semitones (10th to 90th percentile); steps 32%, thirds 32%, fourths and fifths 12%, leaps 9%, repeats 16%; median interval 3 semitones; 50% of moves go up; phrases of about 17 notes, arch 36%, rising 27%, falling 18%, flat or wavy 18%. Reliable only when the line sits clearly above its accompaniment.
- **Crystal Caverns 1991 repetition:** 170 bars of 1.7 s; strongest repeat distances 4 bars (0.72), 8 bars (0.58), 1 bars (0.46); 38% of bars near-duplicate an earlier bar.
- **New Colour meter:** about 130 BPM (could be 65 or 260; the tool cannot tell the beat level), 4 beats a bar; swung: off-beat at 0.68 of the beat (triplet feel); no cross-rhythm claim; many unrelated periodicities found (3:2, 7:4, 7:5), which means irregular timing, not a cross-rhythm claim.
- **New Colour key and mode:** A dorian (low confidence: the key signature margin is 0.005, so read this as a guess); pitch-class weights B 0.16, A 0.10, C 0.09, E 0.09, D 0.09.
- **New Colour chords:** 49 changes a minute (14 a minute are chords that last 1.5 s or more); median chord 0.9 s (10th to 90th percentile 0.5 to 2.0 s); 20 distinct chords; most time on Asus2 28%, Bsus2 13%, Cmaj7 11%, Fmaj7 11%. The old counter in this same run said 22; Ears 1's 2-s counter said 15.  48% of the changes land on a beat (chance would be 65%), which is the check available on real music. Counts are calibrated on clean synthetic chords (exact down to a chord every 0.5 s); chord names on dense or noisy music are a guess.
- **New Colour bass:** root-note movement / mixed; 0.98 notes a second; most time on G 0.27, A 0.27, F# 0.24, E 0.21; steps 50%, thirds 0%, fourths and fifths 15%, octaves 0%, repeats 35%.
- **New Colour top line (from the 'other' stem; read by ear before trusting):** 794 notes, range 17 semitones (10th to 90th percentile); steps 34%, thirds 13%, fourths and fifths 13%, leaps 26%, repeats 14%; median interval 3 semitones; 46% of moves go up; phrases of about 15 notes, arch 30%, rising 43%, falling 4%, flat or wavy 22%. Reliable only when the line sits clearly above its accompaniment.
- **New Colour repetition:** 151 bars of 1.8 s; strongest repeat distances 8 bars (0.74), 2 bars (0.70), 4 bars (0.61); 45% of bars near-duplicate an earlier bar.
- **Stands Tidal Waves meter:** about 99 BPM (could be 50 or 198; the tool cannot tell the beat level), bar length unclear (best guess 3, score 0.17); straight eighths; no cross-rhythm claim; many unrelated periodicities found (3:2, 5:3, 5:4, 7:4), which means irregular timing, not a cross-rhythm claim.
- **Stands Tidal Waves key and mode:** F phrygian (key signature margin 0.061); pitch-class weights F 0.19, A# 0.13, D# 0.12, C 0.11, C# 0.10.
- **Stands Tidal Waves chords:** 35 changes a minute (21 a minute are chords that last 1.5 s or more); median chord 1.7 s (10th to 90th percentile 1.0 to 2.6 s); 9 distinct chords; most time on D#sus2 25%, C#maj7 18%, C#sus2 14%, A#sus2 10%. The old counter in this same run said 18; Ears 1's 2-s counter said 18.  52% of the changes land on a beat (chance would be 50%), which is the check available on real music. Counts are calibrated on clean synthetic chords (exact down to a chord every 0.5 s); chord names on dense or noisy music are a guess.
- **Stands Tidal Waves top line (from the 'other' stem; read by ear before trusting):** 255 notes, range 21 semitones (10th to 90th percentile); steps 26%, thirds 17%, fourths and fifths 34%, leaps 9%, repeats 14%; median interval 4 semitones; 50% of moves go up; phrases of about 9 notes, arch 17%, rising 67%, falling 17%, flat or wavy 0%. Reliable only when the line sits clearly above its accompaniment.
- **Stands Tidal Waves repetition:** 58 bars of 1.8 s; strongest repeat distances 4 bars (0.83), 8 bars (0.63), 1 bars (0.50); 58% of bars near-duplicate an earlier bar. Bar length is uncertain, so these lags are weak evidence.
- **[JUDGMENT]** Pick 1 or 2 of the readings above and write each as something the generator could draw (a spread, not a rule).

### Space
- **Lying in the Reeds mix width by band** (width dB; 0 = unrelated channels, below -20 = near mono): sub -18 (corr 0.97); low_120_500 -5 (corr 0.54); mid_500_2k -7 (corr 0.65); pres_2k_5k -10 (corr 0.81); air_gt5k -11 (corr 0.87); over time (2-s windows) 10th/50th/90th percentile -9/-7/-3 dB.
- **Lying in the Reeds by part:** drums: 16% of the power, width -18 dB, pan +0.00, mono loss 0.1 dB, widest in low_120_500 (-15 dB); bass: 37% of the power, width -35 dB, pan -0.01, mono loss 0.0 dB, widest in sub_lt120 (-36 dB); other: 45% of the power, width -5 dB, pan -0.04, mono loss 1.2 dB, widest in low_120_500 (-4 dB); vocals: 2% of the power, width -18 dB, pan +0.04, mono loss 0.1 dB, widest in low_120_500 (-17 dB). Bands holding under 10% of a part's energy are left out of 'widest' (they are too quiet to trust).
- **Crystal Caverns 1991 mix width by band** (width dB; 0 = unrelated channels, below -20 = near mono): sub -24 (corr 0.99); low_120_500 -8 (corr 0.74); mid_500_2k -9 (corr 0.77); pres_2k_5k -12 (corr 0.87); air_gt5k -12 (corr 0.88); over time (2-s windows) 10th/50th/90th percentile -22/-9/-7 dB.
- **Crystal Caverns 1991 by part:** drums: 42% of the power, width -25 dB, pan +0.01, mono loss 0.0 dB, widest in low_120_500 (-26 dB); bass: 33% of the power, width -27 dB, pan -0.04, mono loss 0.0 dB, widest in sub_lt120 (-27 dB); other: 24% of the power, width -7 dB, pan +0.09, mono loss 0.8 dB, widest in low_120_500 (-5 dB); vocals: 0% of the power, width -18 dB, pan +0.02, mono loss 0.1 dB, widest in pres_2k_5k (-18 dB). Bands holding under 10% of a part's energy are left out of 'widest' (they are too quiet to trust).
- **New Colour mix width by band** (width dB; 0 = unrelated channels, below -20 = near mono): sub -27 (corr 1.00); low_120_500 -12 (corr 0.88); mid_500_2k -8 (corr 0.74); pres_2k_5k -11 (corr 0.84); air_gt5k -9 (corr 0.76); over time (2-s windows) 10th/50th/90th percentile -13/-10/-8 dB.
- **New Colour by part:** drums: 18% of the power, width -22 dB, pan -0.00, mono loss 0.0 dB, widest in mid_500_2k (-20 dB); bass: 30% of the power, width -29 dB, pan +0.01, mono loss 0.0 dB, widest in low_120_500 (-23 dB); other: 51% of the power, width -9 dB, pan +0.07, mono loss 0.6 dB, widest in mid_500_2k (-7 dB); vocals: 1% of the power, width -17 dB, pan -0.01, mono loss 0.1 dB, widest in pres_2k_5k (-14 dB). Bands holding under 10% of a part's energy are left out of 'widest' (they are too quiet to trust).
- **Stands Tidal Waves mix width by band** (width dB; 0 = unrelated channels, below -20 = near mono): sub -9 (corr 0.76); low_120_500 -0 (corr 0.02); mid_500_2k -1 (corr 0.12); pres_2k_5k -4 (corr 0.44); air_gt5k -4 (corr 0.44); over time (2-s windows) 10th/50th/90th percentile -5/-2/2 dB.
- **Stands Tidal Waves by part:** drums: 0% of the power, width -5 dB, pan +0.71, mono loss 1.3 dB, widest in pres_2k_5k (-5 dB); bass: 13% of the power, width -37 dB, pan +0.00, mono loss 0.0 dB, widest in low_120_500 (-36 dB); other: 82% of the power, width 0 dB, pan -0.17, mono loss 3.1 dB, widest in low_120_500 (1 dB); vocals: 5% of the power, width -5 dB, pan +0.01, mono loss 1.2 dB, widest in low_120_500 (-5 dB). Bands holding under 10% of a part's energy are left out of 'widest' (they are too quiet to trust).

### Vibes
- **[JUDGMENT]** For each sound and technique: vibes that lean toward it (x1.5 to x3), ones that lean away (x0.2 to x0.5, never 0), any new vibe or profile. Numbers to start from: Lying in the Reeds: 120 BPM, 16 settled chords a minute, 4% of energy above 2 kHz, width -9 dB; Crystal Caverns 1991: 140 BPM, 10 settled chords a minute, 8% of energy above 2 kHz, width -13 dB; New Colour: 130 BPM, 14 settled chords a minute, 8% of energy above 2 kHz, width -12 dB; Stands Tidal Waves: 99 BPM, 21 settled chords a minute, 3% of energy above 2 kHz, width -1 dB.

### Wholesome take
- **[JUDGMENT]** What to take, what to leave. Measured flags to consider: no brightness or polarity flags from the numbers; listening still decides.

### For later
- Crystal Caverns 1991, bass group `bass-2` (194 notes): around B1, bell- or metal-like (partials off whole multiples), slow swell, falls 21 dB/s, centered; hear it at 3:05.1, 1:37.2. **[JUDGMENT]** why it waits.
- Crystal Caverns 1991, other group `other-1` (319 notes): around D#2, bell- or metal-like (partials off whole multiples), soft attack, falls 53 dB/s, centered; hear it at 4:02.5, 4:16.8. **[JUDGMENT]** why it waits.
- Lying in the Reeds, drums group `drums-1` (274 notes): low thump, quick attack, falls 252 dB/s, centered; hear it at 0:29.0, 0:27.0. **[JUDGMENT]** why it waits.
- New Colour, other group `other-2` (175 notes): around D#4, harmonic (string/reed/pad-like), odd harmonics stronger (square/clarinet-like), quick attack, falls 72 dB/s, centered; hear it at 3:25.1, 3:23.2. **[JUDGMENT]** why it waits.
- Lying in the Reeds, other group `other-1` (28 notes): around C2, bell- or metal-like (partials off whole multiples), slow swell, falls 18 dB/s, centered; hear it at 3:37.8, 3:42.4. **[JUDGMENT]** why it waits.
- Crystal Caverns 1991, other group `other-2` (181 notes): around D3, bell- or metal-like (partials off whole multiples), soft attack, falls 52 dB/s, centered; hear it at 4:17.7, 4:18.3. **[JUDGMENT]** why it waits.
- New Colour, drums group `drums-1` (350 notes): mid snap, quick attack, falls 100 dB/s, centered; hear it at 0:22.5, 0:25.4. **[JUDGMENT]** why it waits.
- Lying in the Reeds, other group `other-2` (138 notes): around A#2, harmonic (string/reed/pad-like), soft attack, falls 96 dB/s, centered; hear it at 0:05.4, 0:11.7. **[JUDGMENT]** why it waits.
- **Dragon Blue Eyes** (1:10.5, 123 BPM by the mix-level count): stands out for tail (dB a second) -7.70 (low, album median -1.30); repetition 0.41 (low, album median 0.61). **[JUDGMENT]** listen and decide if it is worth a deep pass; why it waits is not measurable.
- **Earth’s Lungs** (4:19.6, 129 BPM by the mix-level count): stands out for brightness sweep (octaves) 2.71 (high, album median 1.34); pitch bends a minute 22.88 (high, album median 5.66). **[JUDGMENT]** listen and decide if it is worth a deep pass; why it waits is not measurable.
