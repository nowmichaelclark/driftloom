# Ears cards, pile 6

Ears 12 (Sonnet 5.5, medium), 2026-11-07. Fusion and games. The cards come from `tools/ears/album.py` (run with `python3 -P`, one stage per process, one album chain at a time) and are checked against the limits in `docs/EARS.md`. Lines marked **[JUDGMENT]** are for the synthesis; each carries a short first guess marked **Guess**. Vibe names come only from `data/vibes.txt`. Voices on these records are described as textures only, never by their words.

**Environment.** The machine's Python moved from 3.14 to 3.15 since Ears 11, which broke the old user-site packages (numpy, torch, demucs); I made a Python 3.13 virtual environment with `uv` in the working folder and installed numpy, scipy, librosa, soundfile, torch, torchaudio and demucs into it (no changes to the repo). **Stems are Demucs htdemucs** on every card; I checked the separator with `split.py --mode demucs` on a 20 s clip before the first album (0.78 times real time, the weights downloaded fine), so there is no MDX-Net fallback.

**Twins.** `twins.py` over the new picker rows (all six pile 6 albums, Synthesist), every earlier pick folder in `docs/ears/raw/` (unpacked to a scratch folder) and `data.csv`: **no pile 6 track pairs with anything.** The 9 pairs it lists are the known Aphex repeats and one Girl/Boy pair, all from earlier piles. Nonlocal Forecast, Fire-Toolz, Jaga Jazzist and Animal Crossing are not in `data.csv`. The remix album and *Bubble Universe!* share song titles (Celestial Nervous System, Planck Lengths, Cloud-Hidden, Foam Vacuum Om, Classical Information) but the remixes differ in length and tone, so the picker lists no twin; the one near-original on the remix album, 29 The Path Straight Thru The Direct Path, is checked when that album's card is written.

**Machine time (run alone, one chain at a time):** about 7,910 s (2.2 hours) for the work done: 1,445 s of picks (all six pile 6 albums and the Animal Crossing seasons; Synthesist's 87 s from Ears 11 is not in it), Synthesist 2,958 s, Animal Crossing's deep pass 1,052 s and its 24-hour pass 579 s, Bubble Universe! 804 s, Holographic Universe(s)! 802 s, Conscious Agents 274 s. Demucs ran 0.64 to 0.75 times the audio length when the chain ran alone (the 20 s clip: 0.78). By the 2.5-hour rule I stopped at an album boundary: *Triangular Reformat* (formula about 1,830 s) and *Pyramid* (about 2,630 s, four tracks of 8 to 14 minutes) would have taken the session to about 3.4 hours, so **they are left for an Ears 12b** (their picks are done and are in `raw/pile-6-raw.tar.gz`: for *Triangular Reformat* 26 Edge Of Sleep Remix (typical), 32 Dam Ambience, 03 Machinegirl Full Moon Future Mix and 31 Conglomerate Remix; all four *Pyramid* tracks by the EP rule).

**What changed in how the work was done:**

- The Python on the machine moved to 3.15 (the old packages were for 3.14): a uv-made 3.13 environment holds the tools; nothing in the repo changed.
- The Synthesist picks from Ears 11 pointed at joined wavs that were gone; I rebuilt them with `pick.group_tracks` and kept the picks.
- **Animal Crossing got its own plan** (see its card): a curated picker set, a lighter hpss-plus-theory pass over all 24 hours, and a Winter/Cherry comparison. Four scratch helpers for it (`hours_pass.py`, `hours_table.py`, `seasons.py`, `seasons_chroma.py`) and the others from Ears 11 are in the raw archive, not in `tools/ears/`.
- `kick_check.py`, `beatphase.py`, `monoloss.py` and `med.py` ran on every deep-passed album; `longform.py` did not run (no track of 8 minutes or more among the albums done).

| Album | Tracks (deep pass) | Machine time | Status |
|---|---|---|---|
| Animal Crossing (GameCube) Original Soundtrack (2001) | 238 files (8 deep, 20 hours light) | 1,052 s deep + 579 s hours + 914 s picks | done |
| Nonlocal Forecast, Bubble Universe! | 10 (3) | 804 s | done |
| Nonlocal Forecast, Holographic Universe(s)! | 12 (4) | 802 s | done |
| Nonlocal Forecast, Conscious Agents (single) | 1 (1) | 274 s | done |
| Fire-Toolz & Nonlocal Forecast, Triangular Reformat | 32 (4 picked) | picks 284 s | **left for Ears 12b** |
| Jaga Jazzist, Pyramid | 4 (4, EP rule) | picks 83 s | **left for Ears 12b** |

The order below is the order the work was done in (Animal Crossing first, since it matters most to Driftloom).

---

## Animal Crossing (GameCube) Original Soundtrack (2001)

Ears 12, 2026-11-07. 238 files, 8.2 hours: 24 hourly tracks, the same 24 hours again as a Winter set (194 to 217) and a Cherry-blossom set (218 to 241), about 40 town, shop and event tracks, and about 130 K.K. Slider songs (most twice: a studio cut and an "Aircheck"). **This record got a different plan from the others**, because its heart for Driftloom is the hourly music: (1) the picker ran on a curated set of 84 tracks (the 24 base hours, tracks 6 to 56 of town and event music, and eight K.K. songs; picker rows for the Winter and Cherry sets too, 48 tracks); (2) `album.py`'s deep pass (Demucs, events, techniques, theory, space, stem check) ran on 8 tracks: the typical track (06 Title) and the picker's three least-like (143 K.K. Lullaby, 164 K.K. Ragtime, 175 Only Me, all three K.K. songs), plus four hours I chose to span the day (3AM, 9AM, 3PM, 9PM); (3) the other 20 hours got a lighter pass (`split.py --mode hpss`, then `theory.py`, about 25 s each, plus the level at the start and end of the file); (4) the three sets were compared for sameness. The 130 K.K. songs and the Aircheck repeats are mostly **not measured**: they are one man's songs sung over a band, and the K.K. voice is a texture only. `longform.py` was not run (the longest hour is 3:15 and the longest town track about 5 minutes).

Stems by Demucs htdemucs. **Caution:** this is a synthesized, sample-based score, and Demucs does not separate it into instruments the way it does a recorded band (see the stem check): its "drums", "bass" and "other" are bands of the same sound, so read the **groups** (low plucked notes, mallet-like notes, swells) and not the stem labels.

### Stem check (read first)
- **06 Title**: drums 51%, bass 0%, other 49%. The bass stem is empty; the "drums" stem holds the low plucked notes and the percussion together. The meter tool's 112 BPM agrees with the grid (112.4). No duck is claimed.
- **143 K.K. Lullaby**, **164 K.K. Ragtime**, **175 Only Me**: drums 0%, bass 0 to 1%; **vocals 42%, 61% and 70% of the power**. That is K.K. Slider's sung voice (a wordless-sounding mumble in the rip, described only as a texture). No beat reading is claimed from the drum stem (the meter tool's tempos, 75, 81 and 96 BPM, rest on the other stem's attacks).
- **3AM**: drums 8%, bass 91%, other 1%. **9AM**: drums 99%. **3PM**: drums 79%, bass 0%, other 21%. **9PM**: drums 60%, bass 34%, other 7%. These are not drum shares: all the low plucked and mallet sound lands in whichever stem Demucs likes. The tempo grids agree with the meter tool for all four (97.6, 105.4, 93.9, 68.9 BPM), so the **tempo reading is good**; the **kick lines and ducks are mostly bass notes** (9PM's "kick" is G#1 82% of the time, a bass note; 3PM's is C2 68%) and are not drum numbers.
- **Mono**: every track read is near mono (below).
- No subharmonic misreads claimed.

### The hook
- Small and domestic: a median hour is 2:24 long, quiet (-25.7 LUFS in the rip), dark (centroid 338 Hz; 74% of the energy under 120 Hz, 17% from 120 to 500, 0.8% in 2 to 5 kHz), with a steady pulse (0.86) and about 3.4 notes a second, played by a band that never gets loud. 19 of the 24 hours have a major seventh or a sus2 as their top chord (below). The album median width is **-19.5 dB (near mono)**.
- **[JUDGMENT]** One line a listener would say. **Guess:** a tiny band playing in the next room all day, a different small tune every hour, always warm.

### The hours, as one table

How the 24 hours differ and what stays the same (base set; the Winter and Cherry sets are in T1). Tempo, bar, swing and chords come from `theory.py` on rough (hpss) stems, so the **tempo is good to a simple ratio** (it agreed with the Demucs grid on 3AM, 9AM, 3PM and 9PM; each could be half or double), **swing is read the same way** (the same four hours agree with `beatphase.py` on the Demucs drum stems, where 3AM is a triplet shuffle and the others straight), the **chord names are a floor** and the **keys are not in the table**: the key reading on these loops is a guess (tonic margins 0.02 to 0.82), and C#maj7 is the top chord in seven hours, which looks like the tool's favorite for a low, pitch-ambiguous mix, not seven hours in C#. "Chord color" is the share of chord time on seventh and sus2 chords. "Tail" is the last second's level against the hour's median level.

| Hour | Length | BPM (bar) | Eighth | Top chords (share of time) | Chord color | Mix centroid | Notes/s | LUFS | Tail |
|---|---|---|---|---|---|---|---|---|---|
| 12AM | 2:15 | 92 (12) | shuffle (0.66) | Edim 24%, Cmin 9%, Fdim 9% | 40% | 463 Hz | 3.6 | -30.4 | -23 dB |
| 1AM | 2:24 | 115 (4) | straight | C#sus2 100% | 100% | 153 Hz | 4.0 | -22.5 | -24 dB |
| 2AM | 3:07 | 118 (8) | straight | C#maj7 85%, Dmaj7 4%, Amaj7 2% | 97% | 96 Hz | 3.4 | -23.6 | -22 dB |
| 3AM | 3:07 | 97 (4) | shuffle (0.67) | G#5 19%, Emaj7 14%, G#dim 11% | 40% | 107 Hz | 3.6 | -19.2 | -38 dB |
| 4AM | 2:26 | 111 (2) | shuffle (0.68) | C#maj7 41%, F#5 7%, Fmaj7 7% | 72% | 545 Hz | 3.5 | -27.5 | -30 dB |
| 5AM | 3:15 | 103 (4) | shuffle (0.67) | Dmaj7 25%, Gmaj7 24%, Emin7 14% | 99% | 364 Hz | 3.5 | -25.1 | -38 dB |
| 6AM | 2:55 | 151 (8) | none read | Dmaj7 82%, Gmaj7 6%, B5 6% | 89% | 555 Hz | 3.6 | -25.7 | -26 dB |
| 7AM | 2:26 | 91 (4) | straight | D#maj7 13%, Fmaj7 13%, Fmin7 13% | 84% | 575 Hz | 3.0 | -27.2 | -28 dB |
| 8AM | 2:31 | 108 (2) | straight | A#maj7 19%, Fmaj7 14%, Dmin7 12% | 63% | 466 Hz | 3.6 | -27.5 | -29 dB |
| 9AM | 2:26 | 105 (7) | straight | G#maj7 85%, C#maj7 6%, Gdim 3% | 95% | 135 Hz | 4.3 | -22.5 | -27 dB |
| 10AM | 2:05 | 107 (2) | straight | F5 34%, A#sus2 21%, A#maj7 16% | 64% | 252 Hz | 4.5 | -25.3 | -25 dB |
| 11AM | 2:31 | 101 (2) | straight | Dmaj7 81%, Emin7 5%, Bmin 3% | 89% | 682 Hz | 3.3 | -27.2 | -24 dB |
| 12PM | 2:02 | 86 (2) | straight | C#maj7 84%, Fmaj7 5%, Csus2 3% | 93% | 261 Hz | 3.2 | -25.4 | -26 dB |
| 1PM | 2:35 | 99 (4) | shuffle (0.67) | C#maj7 78%, Csus2 5%, A#maj7 3% | 92% | 1092 Hz | 3.3 | -27.5 | -29 dB |
| 2PM | 2:19 | 103 (2) | straight | D#maj7 99%, Csus2 1% | 100% | 194 Hz | 6.7 | -24.3 | -27 dB |
| 3PM | 1:33 | 94 (4) | straight | C#maj7 55%, C7 8%, Gmin7 7% | 90% | 1237 Hz | 2.6 | -29.6 | -24 dB |
| 4PM | 2:19 | 103 (4) | shuffle (0.66) | Csus2 94%, C 4%, Gsus2 1% | 95% | 313 Hz | 2.6 | -25.3 | -26 dB |
| 5PM | 2:24 | 115 (2) | straight | D#maj7 22%, Gmin7 14%, A#5 11% | 58% | 218 Hz | 3.7 | -23.5 | -26 dB |
| 6PM | 2:23 | 103 (2) | straight | C#maj7 52%, Csus2 10%, F#5 10% | 73% | 441 Hz | 3.3 | -24.8 | -25 dB |
| 7PM | 2:15 | 113 (8) | none read | Dmaj7 25%, Amaj7 23%, C#maj7 21% | 71% | 388 Hz | 2.3 | -27.3 | -28 dB |
| 8PM | 2:09 | 99 (4) | straight | D#maj7 53%, C#maj7 23%, Cdim 8% | 91% | 245 Hz | 3.4 | -26.5 | -25 dB |
| 9PM | 2:43 | 69 (8) | straight | C#maj7 32%, Fmaj7 21%, Fsus2 17% | 84% | 84 Hz | 3.3 | -25.6 | -31 dB |
| 10PM | 1:51 | 72 (3) | straight | F5 34%, Dmaj7 33%, C5 8% | 40% | 394 Hz | 2.8 | -25.9 | -25 dB |
| 11PM | 1:29 | 96 (2) | none read | D#5 23%, Csus2 10%, Bmaj7 9% | 55% | 179 Hz | 1.6 | -27.9 | -33 dB |

**What stays the same across the 24 hours**
- **Pulse and meter:** a steady beat every hour (pulse median 0.86, 0.26 only at 11PM), a tempo of 90 to 115 BPM in **19 of 24 hours** (median 103; 69 at 9PM, 72 at 10PM and 151 at 6AM are the ends), almost all in 2 or 4 beats a bar (10PM reads 3).
- **Chord language:** the top chord is a major seventh or a sus2 in **19 of 24 hours**; the median hour spends 87% of its chord time on seventh and sus2 chords (14 hours spend 80% or more); a few hours are one held chord (1AM: C#sus2 100%; 4PM: Csus2 93%; 2PM: D#maj7 99%). This is the "jazzy sevenths and ninths" of plan.md's row, measured as sevenths and sus2, not ninths (the chord tool names no ninths).
- **Sound:** a quiet, dark, **near-mono** score (width -13.8 to -37.9 dB, median -26.7; mono loss under 0.5 dB on every deep-passed hour), 1.6 to 6.7 notes a second, 89 s to 3:15 long, never loud (-19 to -30 LUFS in the rip, all within 11 dB), tuning within a few cents of A440 (median +0.5), no tape wobble to speak of.
- **Ends:** every hour **ends by dying away**: the last second is 21 to 38 dB under the hour's median level (median 27 dB), and 20 of 24 hours start at about the median level (the first second within 5 dB of it, 15 within 3 dB; the exceptions are 5PM, 10PM, 11PM and 3AM). So the rip shows each hour as a loop with a fade-out or a held decay at the end; **how the game turns one hour into the next is not in the rip** (a handover crossfade would not show in a file that holds one hour).

**What changes across the day**
- **Tempo:** the slowest are 9PM (69 BPM), 10PM (72) and noon (86); the median of each six-hour block is 107 (0 to 5), 106 (6 to 11), 101 (12 to 17), 97 (18 to 23), so the day slows by about 10 BPM. 6AM is the one fast hour (151, or 75 at half time).
- **Brightness (centroid):** the **morning is the brightest block** (median centroid 510 Hz for 6 to 11 against 259 (0 to 5), 287 (12 to 17), 317 (18 to 23)); the brightest single hour is **3PM, 1,237 Hz** with 8.3% of its energy above 2 kHz (the album median is 0.8%), and the darkest are 9PM (84 Hz), 2AM (96 Hz) and 3AM (107 Hz).
- **Density:** the busiest hour is 2PM (6.7 notes a second, a held D#maj7 under a fast line), the sparsest 11PM (1.6 notes a second, pulse 0.26, 1:29 long, the weakest beat of the day).
- **Swing:** a shuffle (eighth off at 0.66 to 0.68 of the beat) in six hours, **12AM, 3AM, 4AM, 5AM, 1PM, 4PM**; straight in 15; three (6AM, 7PM, 11PM) show no off-beat onsets and two (7AM, 5PM) give no reading. The shuffle clusters in the small hours (four of the first six hours).
- **Chord rate:** settled chords (1.5 s or more) from 0 a minute (10PM) and 0.4 (1AM) to 17 (6PM), and a held-chord hour is next to a changing one (1AM 0.4, 2AM 9.6, 3AM 9.6, 4AM 8.6, 5AM 12.6): there is no day-long trend in harmony, it is per hour.
- **Mode and key:** keys wander from hour to hour with no scheme the tool can read; hour pairs are not one key apart. The deep-passed hours: 3AM G# lydian (margin 0.05), 9AM D# lydian (0.03), 3PM A# ionian (a guess, 0.001), 9PM C ionian (0.04, fits C major pentatonic).

### Sounds

All from the deep-passed hours and Title; stem labels are unreliable (above), so groups are described by register, attack and fall.

**S1. A round low pluck (bass) under everything (3AM, 9AM, 9PM, Title):** bass-like groups at **37 to 82 Hz**, nearly pure (harmonic share 0.96 to 1.00; partials 10 to 61 cents off), attacks 25 to 282 ms and falls of **7 to 266 dB/s**. 3AM has a long sine (37 Hz, falls 7 dB/s, a 282 ms swell), a medium one (52 Hz, 17 dB/s) and a short pluck (82 Hz, 266 dB/s); 9PM's is 46 Hz, a 61 ms attack, falls 68 dB/s; 9AM's is 58 Hz, falls 81 dB/s; Title's is 75 Hz, 54 ms, 22 dB/s. All centered (correlation 1.00). Hear it at 9PM 2:38.8 (G#1, 0.88 s), 1:15.0 (G#1, 0.66 s); 9AM 2:14.3 (A#1, 0.39 s); Title 0:40.0 (F2, 0.34 s).
- Building blocks: **[JUDGMENT]** **Guess:** sub with bassStyle plucked or held, a 50 to 80 Hz note with a 25 to 60 ms attack and a medium fall; exists. A phone cannot play the 37 to 52 Hz ones: the octave above carries the line.

**S2. A mallet-like upper voice (all hours):** the melody layer sits at **175 to 520 Hz** (3PM: groups at 175, 520 and about 600 Hz), attacks 30 to 60 ms, falls of 23 to 380 dB/s (two kinds: a sustained one at 23 to 28 dB/s with brightness 1.2 kHz, and a quick one at 378 dB/s), partials 20 to 280 cents off whole multiples (so mixed, bell-wood, not pure), bright at the start (0.5 to 1.9 kHz), centered. A third, soft family: swells with 320 to 605 ms attacks at 600 to 1,500 Hz: 3PM's other-4 (15 notes) has a median 322 ms attack, about 608 Hz, falls 6 dB/s and carries 14 cents of vibrato: a held whistle or flute.
- Hear it at 3PM (other-1 to -5); Title 0:23.1 (D#2, 0.25 s), 0:18.9 (G2, 0.26 s).
- Building blocks: **[JUDGMENT]** **Guess:** a marimba or glass-keys voice at 175 to 520 Hz with a 30 to 60 ms attack and a 20 to 400 dB/s fall (the plucky end is the marimba; the long end is a vibes-like tone); the soft swell with vibrato is a flute or whistle at 600 to 1,500 Hz; exists (flute, whistle and marimba are in the melody voices).

**S3. A light kit, mostly one tick and one low thump (3AM, 3PM, 9PM):** drums groups at 39 Hz (3AM, falls 635 dB/s, 156 notes), 66.6 Hz (3PM, falls 254 dB/s, 73 notes), 53 Hz (9PM, falls 21 dB/s: really a bass note), and ticks at 227 Hz (falls 410 dB/s), 297 Hz and 4 to 5 kHz (3AM: 493 Hz and a 5 kHz brightness, attack 20 ms, falls 284 dB/s: a brush or hat). Hear it at 3PM 0:00.6 (C2, 0.27 s), 1:30.2 (C2, 0.7 s).
- **Kick lines (kick_check):** 06 Title: 111 kicks, 77 Hz (D#2 26%, D2 24%), falling 429 dB/s, every 0.80 s; 3AM: 196, 73 Hz, falling 429 dB/s, every 1.03 s; 9AM: 292, 77 Hz, falling 56 dB/s, every 0.425 s; 3PM: 72, 67 Hz (C2 68%), 245 dB/s, every 0.97 s; 9PM: 94, 53 Hz (G#1 82%), 34 dB/s, every 1.315 s. **Duck:** void on all five (the bass stem is empty or holds the melody).
- **Swing placed (beatphase.py)**, on the meter tool's beat (and its phase, which reads a twelfth of a beat early here): **3AM** has hits at 0, 4 and 8 twelfths of the beat (33%, 33%, 6%: 72% of all hits on the thirds): **a triplet kit, swing at the drum grid's triplet (the eighth at 0.66), a 12/8-like shuffle at 97 BPM**, the second real triplet case after Lifeforms' Vertical Pig (45% of hits at the third, none on a straight eighth). **9AM** puts its hits at 5, 8 and 11 twelfths (40%, 19%, 40%), a quarter beat apart, and **9PM** at 5, 8 and 11 (29%, 27%, 37%): a **straight sixteenth grid** (16ths 13% and 32%, triplet 0% and 0%); **3PM** at 11, 0 and 5 (61%, 18%, 18%): the beat and the **straight eighth** (20%, triplet 0%). So **straight, at the drum grid's sixteenth (9AM, 9PM) and eighth (3PM)**.
- Building blocks: **[JUDGMENT]** **Guess:** a triplet shuffle kit for the shuffle hours (hats on the thirds, a soft kick on the beat), a straight eighth kit otherwise; the generator's swing parameter covers both; exists.

**S4. K.K. Slider's voice (Lullaby, Ragtime, Only Me): vocals-1, 158 and 113 notes, 41% and 33% of note energy**
- In synth terms: about D#3 to D3 (a baritone mumble), harmonic, soft attack, falls 9 to 29 dB/s; centered. It is a **voice texture only**, and I took no words from it.
- Building blocks: **[JUDGMENT]** **Guess:** a hum or "ah" pad as a vocal layer; the voice is not copied; exists as a texture idea (no vowel voice is in Driftloom yet: for the synthesis).

### Techniques

- **T1. The same tune, three times (Base, Winter, Cherry).** The Winter set (194 to 217) and the Cherry set (218 to 241) have **the same length as the base hour to the second in all 24 hours and the same tempo in 21 and 22 of 24** (within 2 BPM), and the same pitch-class profile (cosine 0.95 or more against the base in 19 of 24 hours for Winter), so it is **the same tune re-dressed**. The audio is different (waveform correlation under 0.45 everywhere): Winter has a median centroid of 315 Hz against the base's 339 (13 of 24 hours darker), little above 2 kHz (0.2% against 0.8%) and 9.6 chord changes a minute at mix level against 5.8; **Cherry is the darkest, centroid 197 Hz (0.74 of the base, darker in 19 of 24 hours)**, with 18.6 chord changes a minute (a busier arrangement or more notes), width about the same. In **five hours the Winter and Cherry files are the same** (frame-by-frame cosine of chroma 1.00): 4AM, 5AM, 10AM, 5PM, 9PM. Frame-by-frame cosine of chroma against the base is 0.45 to 0.95 (median about 0.78): the notes and chords are mostly the same, the instruments and the ornaments are not. This is the case for Driftloom's **"keep a friend's theme while the vibes change"**: the game keeps the tune and the length and changes the timbre (darker for Cherry, thinner at the top for Winter), and the weather or season moves a colour, not the tune. **[JUDGMENT]** **Guess:** a vibe layer that changes the instrument set and the brightness (a tone shift of about 25% in centroid) and adds or removes ornament notes, with the melody, tempo and length held fixed.
- **T2. How the hours turn over.** In the rip each hour is its own file that **starts at normal level (the first second within 5 dB of the median in 20 of 24 hours, within 3 dB in 15) and ends 21 to 38 dB down**, so every hour ends in a fade or a decay; the hours that start quiet are 5PM (-10.6 dB), 10PM (-7.5 dB), 3AM (-5.0 dB) and 5AM (-4.7 dB); 11PM starts 6 dB above. The game's handover is not in the rip, so this is all the card can say. **[JUDGMENT]** **Guess:** at an hour boundary the old loop dies away (a 3 to 6 s fade) while the next one starts at full level; a tempo/key handover is a short overlap, with the new hour's first bar landing on the beat.
- **T3. Flat hours, no arcs.** Album median sweep 0.88 octave; drift (60 s against 10 s) 1.03 (39 of 66 above 1): each hour is a loop that stays where it is.
- **T4. Echo and reverb: none read.** The mix tail is -16 dB a second at the median (short, dry; 36 of 83 tracks measurable): this is a dry, close score. Tape wobble 3.1 cents median on the mix; tuning +1.0 cent.

### Theory

- **Th1. Seventh chords and sus2 over a pedal.** Hour examples: 2AM C#maj7 85% (a held major seventh), 6AM Dmaj7 81% with Gmaj7 and B5, 11AM Dmaj7 81%, 12PM C#maj7 83%, 2PM D#maj7 99%, 4PM Csus2 93%, 1AM C#sus2 100%, 3PM Fmin7 46% with Gmin7 18%, C7 10% and Cmin7 7% (a minor-seventh and dominant hour), 9PM Fsus2 21%, Fmaj7 21%, C5 12% over C and F bass (C 49%, F 38%). The K.K. songs add dominant sevenths and ii-V motion: Lullaby A7 12%, D5, C#dim; Ragtime Dmaj7 28%, A7 8%; Only Me A7 18%, Bmin7 12%, Gmaj7 9% (all three in D major, margin 0.001 to 0.026). **[JUDGMENT]** **Guess:** the long chords of Ears 4 and a pedal bass, with sus2/maj7 weights dominant; the generator's seventh chords exist; the K.K. songs' A7-to-Dmaj7 pulls are a different thing (dominant motion) and need an A/B.
- **Th2. Quiet harmony changes fast on top of slow chords.** At mix level K.K. Lullaby counts 67 chord changes a minute (11 settled), Ragtime 47 (9), Only Me 51 (11), Title 53 (10): the fast count is the melody walking through the chord, the settled counts are the usable figures (9 to 11 a minute for songs; 0.4 to 17 per hour).
- **Th3. Meter.** Mostly two or four beats a bar; Only Me reads 3 beats a bar (96 BPM, a waltz feel), 10PM reads 3 (72 BPM, F5 and Dmaj7 equal at 33%), Lullaby reads 6 beats a bar (75 BPM, a 6/8 feel). 12AM reads a 12-beat bar at 92 BPM (shuffle: 4 x 3). No odd meter beyond 3 and 6.

### Space

- **Near mono.** Mix width at the album median is -19.5 dB (10th to 90th percentile -33 to -12); the hours' width runs -13.8 (12AM) to -37.9 (11PM) dB. Mono loss on the deep-passed tracks is 0.0 to 0.5 dB in every band (K.K. songs 0.0; 3AM 0.0 to 0.5 dB; 3PM up to 0.4 dB). Title, K.K.'s Intro and a few town tracks have a little width above 5 kHz (corr 0.24 to 0.95, loss up to 2.1 dB).
- **[JUDGMENT]** **Guess:** this is the case for the Mono switch being free: a Driftloom "Animal Crossing" profile should be near mono with a small, steady room, which matches Ears 4's "steady image".

### Vibes
Numbers: tempo 69 to 151 BPM (19 hours at 90 to 115); centroid 84 to 1,237 Hz; 19 of 24 hours with a major seventh or sus2 on top.
- **[JUDGMENT]** Lean toward (x1.5 to x3): cozy, comforting, sunny, whimsical, warm, playful, mellow (morning and afternoon hours); for the small hours dreamy, moonlit, hushed, restful, tender. Lean away (x0.2 to x0.5): booming, triumphant, cosmic, strange. Never 0. **Guess:** a time-of-day axis: hours 0 to 5 dreamy/moonlit/hushed (dark, 100 to 550 Hz, shuffle in 4 of 6), hours 6 to 11 sunny/bright (the brightest block, 510 Hz), 12 to 17 playful (3PM the brightest hour, 2PM the busiest), 18 to 23 cozy/restful (the slowest tempos: 69, 72, 86 BPM).

### Wholesome take
- Take all of it: small bands, held major sevenths and sus2, a mallet voice and a round low pluck, a shuffle in some hours and a straight eighth in the rest, tunes dressed three ways. Nothing is harsh; the one thing to leave is the K.K. voice, which is only a texture. **[JUDGMENT]** **Guess:** wholesome, the closest record to Driftloom's north star in plan.md.

### For later
- The ~130 K.K. songs: only three were deep-passed; all three in D major, with A7 pulls, so the picker's "least like" choice may have found the K.K. outliers, not K.K. on the whole. **Guess:** a second pass on 6 other K.K. songs (a latin one, a march, a jazz one) for the broader rhythm range (Bossa, Samba, Calypso, Reggae, Ska, Tango) before Ears 13 if the synthesis wants swing vocabulary.
- The 40 town and event tracks (Nook's Cranny, Rainy Day, the festivals, the New Year's countdown that speeds up) were in the picker only: 84-track medians in the table below. **Guess:** deep-pass Rainy Day, Nook's Cranny and the 5-minute countdown first.
- A real hour-boundary test needs the game's own transition (a play-through), not the rip.
- Group quality: the Demucs split is not instrument-faithful on this score; a note-level transcription would be better than stems here. **Guess:** wait.

### Reliability notes for this card
- Four hours (3AM, 9AM, 3PM, 9PM) have a full Demucs deep pass; the other 20 have tempo, swing, key and chords from `theory.py` on rough stems (a floor), plus the picker's mix numbers. The mix-level `tempo` in the picker rows disagrees with the meter tool by an octave on some hours (6AM reads 152 and 151); the table uses the meter tool.
- The picker and `med.py` medians (83 tracks): length 2:16, -23.5 LUFS, centroid 419 Hz, width -19.5 dB, pulse 0.72, chord changes 10 a minute (mix level), repeat 0.49, tuning +1.0 cent, wobble 3.1 cents, drift ratio 1.03.
- The Winter/Cherry comparison used the picker rows, a whole-file waveform correlation, and 0.25 s chroma frames; a 1.00 means a byte-level-near copy.

---

## Nonlocal Forecast, Bubble Universe! (2024)

Ears 12, 2026-11-07. 10 tracks (11 files: Celestial Nervous System is two upload pieces), 39.2 minutes (2:10 to 5:53 each; 4:03 median), all at mix level (\`monoloss.py\` and the album medians too). Three got the deep pass: 02 Planck Lengths (typical, 4:05) and the two least like the rest, 08 Foam, Vacuum, Om (4:59) and 10 Wave Nature (4:02): 13.1 minutes. **It describes three tracks of ten**, and the deep pass skips the ones the remix album covers (Celestial Nervous System, Cloud-Hidden, Triangular Format, Classical Information). No track is 8 minutes long (the longest, Celestial Nervous System, is 5:53), so \`longform.py\` was not run. Stems by Demucs htdemucs.

Machine time: 804 s (13.4 min) for 13.1 minutes of deep-pass audio, run alone: pick 73 s (10 tracks), Demucs about 0.63 times the audio (245 s of audio in 160 s for Planck Lengths), measuring and the stem check the rest. Ears 9's formula (1.1 x 786 + 12 x 10) says 985 s, so this ran 18% under it.

### Stem check (read first)
- **02 Planck Lengths**: drums 49%, bass 19%, other 31%, vocals 0% (0.2%: bleed). The meter tool's 143 BPM agrees with the drum grid (143.5).
- **08 Foam, Vacuum, Om**: drums **0%**, bass 23%, other 76%. **No drum track: the beat numbers are void** (its "10 kicks" are bass notes, its "swung 0.56" and the 12 even twelfths in \`beatphase.py\` are the pad's own movement).
- **10 Wave Nature**: drums 12%, bass **2.5% (thin)**, other 82%, vocals 3.9% (bleed). The meter tool's 118 BPM **does not match** the grid (215 BPM) by a simple ratio, and the drum onsets spread evenly over every twelfth of the beat (4 to 12% each): **the beat is unclear**, the bass numbers are guesses, and the "kick" line (B0 71% at 30 Hz) is a subharmonic read of a low pad (below).
- Vocals stems hold 0.2 to 3.9%: bleed on a mostly instrumental record. The voice-like groups below (vocals-1 at 248 to 352 Hz, 205 to 429 notes, 0.2 to 6% of the power) are bell- or pad-like high parts, not sung words.
- Subharmonic misread: Wave Nature's drum group at 33 Hz and bass-1 at 36 Hz sit far under the strongest line of the stem; I do not claim them as a sub kick.

### The hook
- A bright-mid, dense, wide, loud electronic record: a median track is 4:03, **loud** next to Animal Crossing (-14.75 LUFS in the rip against its -25.7) though not next to Synthesist (-12.4), dark for a loud track (centroid 393 Hz; 51% of the energy under 120 Hz, 34% from 120 to 500, 1.7% in 2 to 5 kHz, 1.1% above 5 kHz), wide (median width -8.4 dB), a pulse of 0.62, 11 chord changes a minute at mix level, **tuning dead on A440** (median 0 cents, -4.1 to +1.4) and a wobble of 4.7 cents, with long tails (-1.75 dB a second).
- **[JUDGMENT]** One line a listener would say. **Guess:** a busy bubble of synths in a soft room: lots of notes, none of them sharp.

### Sounds

**S1. A wide, soft mid-low synth stack (Planck Lengths, Foam, Vacuum, Om, Wave Nature): other-1, 417, 233 and 278 notes, the biggest group of each track**
- In synth terms: Planck Lengths about 84 Hz (E2), a 81 ms attack, falls 25 dB/s, brightness at the start 623 Hz, half-correlated stereo (0.49); Foam, Vacuum, Om about 67 Hz and 37 Hz drones (attacks 81 to 93 ms, falls 10 to 12 dB/s, brightness 210 to 220 Hz), correlation 0.31 to 0.37; **Wave Nature about 78 Hz with a slow 303 ms attack, falls 4 dB/s, correlation -0.02**: a pad whose left and right are unrelated, held for seconds. Partials are 14 to 53 cents off whole multiples (mixed, chorus-like).
- Hear it at Planck Lengths from 0:03 on; Wave Nature 0:07.5 (the pad enters), and the 1.3 octave upward sweep at 1:34.5 to 1:40.
- Building blocks: **[JUDGMENT]** **Guess:** softpad with a 80 to 300 ms attack and a chorus/detune spread; exists, but at a pad-width that the mono guard would test (Space).

**S2. A sine-like bass and a sub (Planck Lengths, Foam, Vacuum, Om): bass-1, 217 and 314 notes, 42% and 27% of note energy**
- In synth terms: Planck Lengths 48 Hz (G1), harmonic share 0.97, a 167 ms swell, falls 12 dB/s, centered; a second at 80 Hz falls 21 dB/s. Foam, Vacuum, Om has two: 74 Hz (D2) and 44 Hz (F1), pure (harmonic 1.00), attacks 191 and 247 ms, falls 15 dB/s, correlation 0.95 to 0.96. Under 60 Hz pitch is good to a semitone only.
- Hear it at Planck Lengths 1:33.5 (the bass comes back after the floor drop), Foam, Vacuum, Om 0:50.5, 1:09.5.
- Building blocks: **[JUDGMENT]** **Guess:** sub with bassStyle held, 44 to 80 Hz, a 170 to 250 ms swell; exists.

**S3. A bright, quick lead above it (Planck Lengths, Wave Nature): other-2, 61 and 101 notes, 10% and 26% of note energy**
- In synth terms: Planck Lengths about 392 Hz (G4), a 39 ms attack, falls 69 dB/s, brightness 2.1 kHz, correlation 0.59; Wave Nature about 1.4 kHz, a 123 ms attack, falls 16 dB/s, brightness 3.8 kHz, harmonic share 0.29 (bell-like). The bright top is thin: the album median has 2.8% of its energy above 2 kHz (1.7% in 2 to 5 kHz).
- Building blocks: **[JUDGMENT]** **Guess:** glass keys or a soft bell (Ears 4's tone, 0.5 to 2 kHz) for the quick lead; exists.

**S4. A kit that is mostly low thump and tick (Planck Lengths): drums-1 to -4, 180, 164, 60 and 69 notes**
- **Kick line (kick_check):** 162 kicks, median 60 Hz (A#1 11%, A1 10%, G#1 8%, A2 7%: spread over four notes, not one root), spread 38 to 108 Hz, falling **94 dB/s** (31 to 140: long and soft), median spacing 0.625 s (1.5 beats of the 0.419 s grid: a dotted-quarter pulse). **Duck:** none to speak of (the bass level after the kick moves only -0.0 to -3.5 dB, lowest at 10 ms, no recovery shape); pumping under 1.5 dB.
- Building blocks: **[JUDGMENT]** **Guess:** a soft tunedKick (a 94 dB/s fall) on the bass note, no subDuck; the hat/tick is at 630 Hz with brightness 5.4 kHz (27 notes); exists.

### Techniques

- **T1. Huge brightness sweeps, a few per track.** Planck Lengths: the mix goes **down 4.2 octaves 0:00 to 0:10** (an opening filter closing), up 3.5 octaves 1:24.5 to 1:30.5 and down 3.0 octaves 1:30.5 to 1:37.5 (the up-down peak sits on the floor drop below). Wave Nature: down 3.1 octaves 0:17.5 to 0:27.5, down 2.3 octaves 3:29.5 to 3:35.5 and **up 4.2 octaves 3:35.5 to 3:42.0**. Foam, Vacuum, Om: 1.3 to 1.8 octaves. Album median sweep 0.98 octave (p90 1.9). Ears 8's padSweep (a lowpass on the chord layer, every 12 to 28 s) was built from records that swing 2.7 to 6.5 octaves in 5 to 25 s; these 3 to 4 octave, 6 to 10 s moves are the same family. **[JUDGMENT]** **Guess:** padSweep with a 3 to 4 octave range, at the start and end of a section; exists.
- **T2. Floor drops and stem entries on phrase lines.** Planck Lengths: 1.36 a minute, 3.0 s at 1:23.8, 7.0 s at 1:27.0, 4.0 s at 1:48.8, 7.8 s at 2:12.2 and **20.2 s at 2:23.2**; drums enter and leave at 0:05.0, 0:39.5, 0:46.5, 1:22.5, 1:28.5, 1:48.0, 2:14.5, 2:24.5, 2:43.0 and 3:44.5 (gaps of 5 to 40 s), bass in at 0:06.0, 1:33.5 and 2:43.0 and out at 1:25.0, 2:13.5 and 3:43.5: the kit and bass leave together at section lines and return a beat or bar apart. Foam, Vacuum, Om: its drums-stem entries (0:41 to 1:55) are not a kit (drums 0%). Floor drops there and on Wave Nature: low end too sparse to say. 1.36 floor drops a minute is the highest rate of any track carded this session (Synthesist's 1847 - Earth has 0.50). **[JUDGMENT]** **Guess:** floorDrop, with the 20 s one as the long end of the draw.
- **T3. Chorus-like wobble.** Planck Lengths' other stem wobbles 15 cents at 1.7 Hz and its bass 22 cents at 0.7 Hz; Foam, Vacuum, Om's other 19 cents at 1.1 Hz (speed up to 29 cents in 4 s) and bass 21 cents at 1.2 Hz; Wave Nature 7 cents at 1.2 Hz. That is 2 to 6 times Driftloom's 3.6 cents of mix wobble and in the vibrato range (0.7 to 1.7 Hz), on tuning that sits at 0 cents. **[JUDGMENT]** **Guess:** a 10 to 20 cent slow wobble on pad and bass layers, 0.7 to 1.7 Hz (a chorus), tuning left at zero; do not put it on the lead.
- **T4. Tempo-synced echo, small.** Wave Nature's drums echo at 354 ms (+4.2 dB, 3 repeats); none found on the other two. **[JUDGMENT]** **Guess:** not a feature of this record.
- **T5. Stereo movement.** Foam, Vacuum, Om's parts move across 0.37 to 0.47 of the pan range, with a 3 s cycle on the other stem (0.47); Wave Nature's drums and vocals 0.35 to 0.37; Planck Lengths stays put (under 0.25). Ears 4 liked a steady image and disliked stereo that moves.

### Theory

- **Th1. Power chords and sus2 on the busy track, maj7 on the quiet ones.** Planck Lengths: G5 10%, C5 9%, F#sus2 7%, F5 6% over a G, C, A, F bass (G 24%, C 16%, A 12%, F 10%), G phrygian (margin 0.016: a guess), **44 distinct chords in 4 minutes, 23 changes a minute (15 settled), 84% of the changes land on a beat (chance 72%)**. Foam, Vacuum, Om: G 16%, Cmaj7 16%, C5 14%, D5 12% over a two-note bass ostinato (F and E, 89% repeats), F lydian (margin 0.095: the clearest key of the three), 6 changes a minute, a 4.6 s median chord. Wave Nature: Cmaj7 14%, Fsus2 8%, G#maj7 7%, Bmaj7 7% (36 distinct chords, 16 a minute, 12 settled), C aeolian (a guess). **[JUDGMENT]** **Guess:** power chords with sus2 and major sevenths, chord changes on the beat; exists in the generator.
- **Th2. Colors past the seventh: not measurable here.** The chord tool names triads, sevenths, sus2 and power chords, never a ninth or an eleventh, so "chords past the seventh" can only be read from the pitch-class weights: Planck Lengths has five classes within 0.09 to 0.13 (G 0.13, C 0.11, D# 0.11, F 0.09, D 0.09), a flat spread that fits dense harmony (a G minor with added tones and a flat second), and Wave Nature's top five are 0.09 to 0.14. This is a lead, not a measurement. **[JUDGMENT]** **Guess:** a tension-tone layer over the held chord (a 9th or 11th one octave up, one in four bars); for the synthesis to decide with Jaga Jazzist's numbers.
- **Th3. A fast arc melody.** Wave Nature's top line: phrases of **22 notes, 82% arch**, 622 notes in a range of 12 semitones, fourths and fifths 35%; Planck Lengths' phrases of 11 notes (40% arch, fourths and fifths 41%); Foam, Vacuum, Om's thirds 34% and phrases of 9 notes. Fourths and fifths are the main interval (19 to 41%) and steps are rare (5 to 14%): a bell-like, leaping line. Repetition: Planck Lengths and Wave Nature repeat at 1 bar (0.46 and 0.73) and nowhere else; 0% near-duplicate bars: sequenced, never copied.
- **Th4. Meter.** Planck Lengths: 143 BPM, read as 8 beats a bar (weak), the drum onsets sit on **a straight sixteenth grid** (\`beatphase.py\`: hits at 0, 3, 6, 9 twelfths and the early doubles at 5 and 11; sixteenths 22%, triplet 8%, swung eighth 1%), so the meter tool's "swung 0.76" is the sixteenths, **straight at the drum grid's sixteenth**. Foam, Vacuum, Om: bar length unclear (best 9, score 0.17) and the periodicities are unrelated (4:3, 5:2, 7:4, 7:5): irregular, not an odd meter; Wave Nature: bar length unclear (4, score 0.07). **No odd meter is claimed.** No triplet kit on this record.

### Space

- **Wide at the top and the middle, centered below 120 Hz.** Mix width (dB; 0 = unrelated channels), sub / 120 to 500 / 500 to 2 k / 2 to 5 k / above 5 k: Planck Lengths -23 / -7 / -5 / -5 / -7; Foam, Vacuum, Om -7 / -1 / -1 / -2 / -2; Wave Nature -11 / 0 / 0 / -1 / -1. Album median width -8.4 dB (p90 -2.4); bass and kick centered (bass stem width -10 to -29 dB, drums -16 to -17).
- **What the mix loses in mono** (corr and loss in dB, below 120 Hz, 120 to 500, 500 to 2 k, 2 to 5 k, above 5 k): Planck Lengths 0.99/0.0, 0.69/0.7, 0.51/1.2, 0.51/1.2, 0.67/0.8; Foam, Vacuum, Om 0.69/0.7, **0.10/2.6**, 0.06/2.8, 0.24/2.1, 0.33/1.8; Wave Nature 0.88/0.3, **-0.04/3.2**, **-0.00/3.0**, 0.08/2.7, 0.09/2.6. Other tracks: Celestial Nervous System 0.0 to 1.8; The Direct Path 1.1 to 1.6; Triangular Format 0.0 to 2.1; the album's worst is Wave Nature (the left and right of its pad are unrelated, not inverted). So the busy arrangement is **soft in mono** (3.2 dB at worst in any band) although it is wide in stereo.
- **[JUDGMENT]** **Guess:** a mono switch would lose under 1.2 dB on the beat track and about 3 dB on the two pad tracks; that is inside the limit the guards set (loss within a few dB), so a wide pad of this kind is allowed if the bass and kick stay centered.

### Vibes
Numbers: Planck Lengths 143 BPM, 15 settled chords a minute, 4% above 2 kHz, width -10 dB; Foam, Vacuum, Om no beat, 5, 2%, -2 dB; Wave Nature unclear beat, 12, 2%, -1 dB.
- **[JUDGMENT]** Lean toward (x1.5 to x3): bubbly, floating, cosmic, twinkling, wondrous, hypnotic, airy, spacious. Lean away (x0.2 to x0.5): sour, crisp, booming, tart. Never 0. **Guess:** the reference record for bubbly and cosmic when the pad is wide and the bass is round.

### Wholesome take
- Take the wide soft pad stack with a round sine bass, the huge slow filter sweeps, the 3 s pan cycle on the pad, the long 20 s floor drop, the chorus wobble (10 to 20 cents) and the power-chord/sus2/maj7 changes on the beat; leave the pad whose left and right are unrelated if it fails the fold guard, and the loudness (-14.75 LUFS in the rip, against a relaxing 20 to 25). Nothing is harsh here. **[JUDGMENT]** **Guess:** wholesome and sweet, a lot of motion at a low brightness.

### For later
- The remixed songs: 01 Celestial Nervous System (5:53, 117.5 BPM at mix level, 12 chord changes a minute), 03 Cloud-Hidden (2:10, 14.8 a minute), 07 Classical Information (2:19, 152 BPM) and 09 Conscious Agent Combos (4:30, 14.9 a minute) are not deep-passed here; their remixes on *Triangular Reformat* make a good pair to compare for "keep the tune, change the clothes". **Guess:** deep-pass 01 and 09 in an Ears 12b only if the synthesis wants more of this record.
- Wave Nature's pad with unrelated channels (3 dB mono loss, correlation -0.04) is the album's width case; a short A/B of the mono fold would show whether it still sounds right. **Guess:** wait.

### Reliability notes for this card
- Three of ten tracks were deep-passed (13 of 39 minutes); Foam, Vacuum, Om has no kit and Wave Nature a thin bass stem, so only Planck Lengths carries the kick, duck and swing readings.
- Tempo-octave: Planck Lengths 143 BPM is the grid; the picker's mix tempo for other tracks varies by octave. The tuning number is a mix-level reading good to about 8 cents.

---

## Nonlocal Forecast, Holographic Universe(s)! (2024)

Ears 12, 2026-11-07. 12 tracks (15 files: three tracks are two upload pieces), 41.9 minutes (1:07 to 6:10 each; 3:16 median), all at mix level (\`monoloss.py\` and the album medians too). Four got the deep pass: My Big T.O.E. (typical, 2:50) and the three least like the rest, My Incomplete T.O.E. (feat. Fire-Toolz) (2:47), We're Smeared Across a 2D Surface (Part -1) (2:21) and The Bubbling Up of Duality On an Autumn Night By a Forest Stream (5:16): 13.2 minutes. No track is 8 minutes long (the longest, Interactions Between Brains & the Foam of Potentiality, is 6:10), so \`longform.py\` was not run. Stems by Demucs htdemucs.

Machine time: 802 s (13.4 min) for 13.2 minutes of deep-pass audio, run alone (Demucs 170 s of audio in 127 s, 0.75 times). Ears 9's formula (1.1 x 794 + 12 x 12) says 1,018 s, so it ran 21% under it. Picks (76 s, 12 tracks) are not in the sum.

### Stem check (read first)
- **My Big T.O.E.**: drums 62%, bass 11%, other 27%, vocals 0%. The meter tool's 115 BPM is two thirds of the grid (172.4): the hits fall in thirds of the beat (below).
- **My Incomplete T.O.E. (feat. Fire-Toolz)**: drums 60%, bass 30%, other 10%, **vocals 0.4%**. Meter 121 BPM agrees with the grid (120.2). **The vocals stem is bleed: no sung or screamed voice is separated on this track** (its vocals-1 group at 91 Hz falls at 127 dB/s with correlation 0.98: a low drum or bass part, not a voice). If Fire-Toolz sings on it, the voice is in the other stem, or not on this stretch; nothing harsh was found (below).
- **We're Smeared Across a 2D Surface (Part -1)**: drums 6%, **bass 0%**, other 88%, vocals 7%. The bass stem is empty: bass notes and the duck are void; the "17 kicks" are rare hits. Meter 127 BPM is two thirds of the grid (191.7).
- **The Bubbling Up of Duality...**: drums 32%, bass 32%, other 35%. The meter tool's 77 BPM does not match the grid (235.3) by a simple ratio, but 235.3 / 77.3 = 3.04: **the grid is the triplet of the beat** (below).
- Vocals stems hold 0.0 to 0.9% except Part -1 (7%: a voice-like texture at 261 Hz, 341 notes, level -18 dB under the mix, a hum or a high pad; I took no words from it).
- No subharmonic misreads claimed (the drum groups at 43 to 58 Hz sit with a bass note under them; the kick lines below start at 40 to 83 Hz).

### The hook
- Loud, dark, steady and swung in threes. A median track is 3:16, **loud** (-11.35 LUFS in the rip, the loudest album carded this session), dark (centroid 251 Hz; 70% of the energy under 120 Hz, 22% from 120 to 500, 1.3% in 2 to 5 kHz, 0.4% above 5 kHz), **tuned to A440** (median 0 cents, -1.9 to +2.0), a wobble of 3.8 cents (the same as Driftloom's 3.6), 15 chord changes a minute at mix level, **big brightness sweeps** (median 1.29 octaves, p10 0.9, p90 1.6, the largest median of the albums carded this session) and a drift ratio of 1.24 (11 of 11 above 1).
- **[JUDGMENT]** One line a listener would say. **Guess:** a loud, round, bubbly club track played from the next room: a triplet bounce in the floor, a slow filter breathing on top.

### Sounds

**S1. A soft, round kit with a tonal kick (My Big T.O.E., The Bubbling Up of Duality): drums-1, 264 and 188 notes, 44% and 29% of the stem's weight**
- In synth terms: My Big T.O.E.'s kick about 49 Hz (G1), a 55 ms attack, falls 180 dB/s, harmonic share 0.80, centered (0.99); the Bubbling Up's about 43 Hz, 44 ms, falls 188 dB/s, harmonic 0.99 (a sine kick). Next to it a snare-like thump at 150 Hz (16 ms, falls 79 dB/s, 98 notes) on the Bubbling Up and a hat/tick with brightness above 4.6 kHz on both.
- **Kick lines (kick_check):** My Big T.O.E.: 187 kicks, median 56.5 Hz (G1 20%, G#1 13%, F#1 13%, plus C3 18%, a bass note), falling 261 dB/s, every 0.525 s (a kick every beat of the 0.523 s beat). Bubbling Up: 246 kicks, 83 Hz (C3 23%: mixed with bass notes), 240 dB/s, every 0.515 s (two thirds of its 0.776 s beat). My Incomplete T.O.E.: 98 kicks, **39.7 Hz (D#1 56%)**, falling 83 dB/s (long and soft), every 0.990 s (two beats of 0.496 s). Part -1: 17 kicks, 87.5 Hz (F2 76%), 136 dB/s, every 2.53 s. Under 60 Hz pitch is good to a semitone only.
- **Duck (kick_check duck):** My Big T.O.E. none (the bass level after the kick moves -1 to -5 dB with no shape); Bubbling Up none (-1 to -2.6 dB); Part -1 void (no bass stem). **My Incomplete T.O.E.: a deep one**: the bass is 9.5 dB down at the kick, 17 dB down at 100 ms, lowest -27 dB at 975 ms (just before the next kick) and highest at 395 ms: **the bass plays between the kicks and drops out around each one** (the pumping tool reads 21.7 dB at a 0.49 s period, a floor). That is deeper than Fear of Flying's 10 to 15 dB.
- Hear it at My Incomplete T.O.E. from 0:32 (the kit returns after the 23.5 s floor drop that starts at 0:08.2), My Big T.O.E. 0:31.
- Building blocks: **[JUDGMENT]** **Guess:** tunedKick at 40 to 56 Hz with a 80 to 260 dB/s fall; subDuck of 3 dB for the triplet tracks and up to 20 dB with a 400 ms recovery for the Fire-Toolz track; exists, the deep duck needs checking against the low-end guards.

**S2. A held pad and a bright upper line (all four): other-1 and other-2, 63 to 500 notes**
- In synth terms: My Big T.O.E.'s pad about 49 Hz (a 324 ms swell, falls 3 dB/s, brightness 440 Hz, correlation 0.69); My Incomplete T.O.E.'s two groups at 98 Hz (342 ms, falls 5 dB/s, brightness 460 Hz) and 137 Hz (179 ms, falls 3 dB/s, brightness 1.3 kHz), both **unrelated channels (correlation 0.25 to 0.27)**; Part -1's at 87 Hz (90 ms, falls 22 dB/s, correlation **0.13**: 67% of the weight); the Bubbling Up's at 99 Hz (135 ms, falls 11 dB/s, brightness 720 Hz, 500 notes, the biggest group on the record). Partials 31 to 85 cents off whole multiples.
- Hear it at the Bubbling Up 0:12.5 to 0:22.0 (a 3.8 octave upward sweep), My Incomplete T.O.E. 0:23.5 to 0:37.5 (a 4.4 octave downward one).
- Building blocks: **[JUDGMENT]** **Guess:** softpad with a 90 to 340 ms swell and a wide, unrelated-channels spread; exists, but the decorrelated width is a Space guard case.

**S3. A baritone bass line (Bubbling Up, Incomplete T.O.E.): bass-1 to -4, 263, 118 and 125 notes**
- In synth terms: Bubbling Up about 55 Hz (A1), harmonic 0.99, a 104 ms swell, falls 6 dB/s, correlation 0.92; a second at 110 Hz falls 30 dB/s. Incomplete T.O.E. has eight small groups at 51 to 96 Hz, attacks 22 to 116 ms, falls 5 to 219 dB/s: a bass that plays many short notes (harmonic 0.04 to 0.99 mixed).
- Building blocks: **[JUDGMENT]** **Guess:** sub with bassStyle plucked and a held octave; exists.

### Techniques

- **T1. Triplet kits: three cases on one record.** \`beatphase.py\` on the drum stems, against the meter tool's beat (whose phase reads about a quarter of a beat off here, so I read the spacing, not the position): **My Big T.O.E.** hits at 3, 7, 11 twelfths (22% each) plus 0, 4, 8 (11, 10, 10%): **every hit sits on the thirds of a beat (spacing a third), triplet 26%, swung eighth 27%, straight eighth 1%**; **The Bubbling Up** 3, 7, 11 (25, 27, 26%), straight eighth 1%, triplet 18%, swung 21%: the same grid at 77 BPM, the meter tool's grid 3.04 times the beat; **Part -1** 0, 3, 4, 7, 8, 11 (19, 14, 23, 14, 16, 15%), straight eighth 0%, triplet 34%. In every case there is no straight eighth: **the swing is at the drum grid's triplet (a 12/8 shuffle at 77 to 127 BPM)**, not the eighth or sixteenth. These are the second to fourth real triplet cases after Lifeforms' Vertical Pig and Animal Crossing's 3AM; this record is the first where most of the deep-passed tracks are triplet. **My Incomplete T.O.E.** is the contrast: hits at 2, 8 and 0, 6 (28, 25, 13, 11%): sixteenths 58%, triplet 0%, swung 0%: **straight, at the drum grid's sixteenth**. **[JUDGMENT]** **Guess:** a triplet-shuffle feel as a swing option at 0.66 (the generator's swing); the kits in this record are three-per-beat, which is a different thing from swinging the eighth.
- **T2. Very long, very wide filter rides.** My Incomplete T.O.E.: the mix goes **down 4.4 octaves 0:23.5 to 0:37.5, up 4.1 octaves 0:38.0 to 0:44.0, down 4.1 octaves 0:44.0 to 1:02.0 and up 3.4 octaves 1:02.5 to 1:23.0** (and 3.5 octaves up 1:33.5 to 1:44.0): an almost continuous 14 to 20 s filter ride. The Bubbling Up: down 3.8 octaves 0:04.5 to 0:12.5, up 3.8 octaves 0:12.5 to 0:22.0, down 2.6, up 2.3. My Big T.O.E.: up 3.6 octaves 2:43.5 to 2:49.0, down 3.3 octaves 1:55.0 to 2:01.0. Album median sweep 1.29 octaves. Ears 8's padSweep references swing 2.7 to 6.5 octaves in 5 to 25 s (Pancake Lizard 3 to 4 octaves every 10 to 20 s), so this is the same move, here 14 to 20 s long and repeated four times in a row. **[JUDGMENT]** **Guess:** padSweep (approved) at 3 to 4.4 octaves; the back-to-back four-ride run is the part to try behind hypnotic or lively vibes, never calm ones.
- **T3. Long floor drops.** My Incomplete T.O.E.: **23.5 s at 0:08.2** (the rest plays; the kit and bass come in at 0:31.5); My Big T.O.E. 0.79 a minute (2.5 s at 0:10.0, 4.8 s at 1:52.2). Part -1 and the Bubbling Up: low end too sparse to say. **[JUDGMENT]** **Guess:** floorDrop, with a very long intro drop (20+ s) as the far end of the draw.
- **T4. Tape wobble: bass only, vibrato-fast.** My Big T.O.E.'s bass wobbles 16 cents at 3.3 Hz and the Bubbling Up's 9 cents at 2.8 Hz (vibrato speed, on the bass); My Incomplete T.O.E.'s other stem 23 cents at 0.3 Hz (a slow wow); Part -1 nothing at 4 cents or more. **[JUDGMENT]** **Guess:** a bass vibrato of 9 to 16 cents at 3 Hz is too much for a calm vibe; skip.
- **T5. An echo on the beat.** Part -1's drums echo at 474 ms (+10.1 dB, 3 repeats), exactly one beat of its 126.7 BPM. **[JUDGMENT]** **Guess:** echoTime at one beat; exists.
- **T6. Sections.** Section lengths 17 to 24 s medians; the Bubbling Up has 16 boundaries (a section every 17 s); My Incomplete T.O.E. 7 at 23 s median. Bars never repeat exactly (the repetition tool finds 1-bar lags only).

### Theory

- **Th1. Planing major sevenths and sus2 over a pedal (My Incomplete T.O.E.).** Gmaj7 26%, Csus2 20%, Fmaj7 20%, **Amaj7 18%** (12 distinct chords, 13 changes a minute, 9 settled, a 4.1 s median chord, 64% of the changes on a beat against 60% by chance) over a G bass (65%), A (19%) and F: the major sevenths sit a whole step apart (F, G, A), a **parallel major seventh** color, G mixolydian by a thin margin (0.007: a guess). My Big T.O.E.: F#sus2 19%, C#5 15%, D# 14%, F#5 9% over a **C# pedal 85%** (C# ionian, margin 0.022). Part -1: Adim 15%, C#maj7 12%, F5 11%, A5 8% (25 distinct chords, 29 a minute, 47% on a beat against 63%: off the beat; F phrygian, a guess). The Bubbling Up: Gsus2 14%, D5 10%, A#5 10%, C5 7% (**41 distinct chords**, 33 a minute, 14 settled; D dorian, a guess), a bass that moves by root (D 27%, A 17%, A# 15%, B 11%). **[JUDGMENT]** **Guess:** sus2, major sevenths and power chords in a planing figure (three adjacent roots), a pedal or a wandering root; exists in the generator as plain chords, but planing is a figure not a chord: a candidate.
- **Th2. Past the seventh: not measurable.** The chord tool names no ninths or elevenths, and the pitch-class weights are spread (My Big T.O.E. C# 0.17, D# 0.16, G# 0.11, F# 0.11, G 0.09; Incomplete T.O.E. G 0.12, D 0.11, C 0.10, F 0.10, E 0.09), a flat enough set that added tones are likely, but it is a lead, not a measurement. The same lead as Bubble Universe!.
- **Th3. Top lines.** The Bubbling Up's line is the widest and the most leaping (703 notes, a 21 semitone range, **leaps 39%**, median interval 5, phrases of 8 notes, 63% arches); My Incomplete T.O.E.'s: 21 semitones, leaps 27%, phrases of 5 notes, rising 47%; My Big T.O.E.'s: 11 semitones, steps 25%, fourths and fifths 25%, 7-note phrases; Part -1's: 9 semitones, fourths and fifths 40%, phrases of 27 notes (long, wavy). A bell-and-hook language, arches and rises. 
- **Th4. Meter.** Bar lengths are unclear on all four (best guesses 9, 4, 3 and 2, scores 0.07 to 0.21): **no odd meter is claimed**. The threes are in the beat, not the bar.

### Space

- **Dense and wide, centered below 120 Hz.** Mix width (dB; 0 = unrelated channels), sub / 120 to 500 / 500 to 2 k / 2 to 5 k / above 5 k: My Big T.O.E. -13 / -6 / -5 / -4 / -5; My Incomplete T.O.E. -24 / -7 / -5 / -5 / -4; Part -1 -7 / -2 / -2 / -1 / -1; Bubbling Up -15 / -8 / -6 / -6 / -8. Album median width -9.85 dB (-12.8 to -7.9).
- **What the mix loses in mono** (corr and loss in dB, below 120 Hz, 120 to 500, 500 to 2 k, 2 to 5 k, above 5 k): My Big T.O.E. 0.92/0.2, 0.64/0.9, 0.53/1.2, 0.45/1.4, 0.46/1.4; My Incomplete T.O.E. 0.99/0.0, 0.69/0.7, 0.52/1.2, 0.52/1.2, 0.34/1.8; **Part -1 0.64/0.9, 0.18/2.3, 0.23/2.1, 0.13/2.5, 0.11/2.6**; Bubbling Up 0.94/0.1, 0.76/0.6, 0.63/0.9, 0.62/0.9, 0.72/0.7. The other eight tracks lose 0.0 to 1.8 dB. So **the loudest, most swung record is the one that folds softest** (worst 2.6 dB, on a wide pad).
- **[JUDGMENT]** **Guess:** a decorrelated pad (correlation 0.1 to 0.27) loses 2 to 2.6 dB in mono and stays inside the guard, if the bass and kick stay centered (they do here: bass stem correlation 0.65 to 1.00).

### Vibes
Numbers: My Big T.O.E. 115 BPM (triplet), 18 settled chords a minute, 2% above 2 kHz, width -10 dB; My Incomplete T.O.E. 121, 9, 5%, -13 dB; Part -1 127 (triplet), 17, 1%, -2 dB; Bubbling Up 77 (triplet), 14, 4%, -10 dB.
- **[JUDGMENT]** Lean toward (x1.5 to x3): bubbly, bouncy, skipping, swaying, twinkling, pulsing, hypnotic, floating, cosmic. Lean away (x0.2 to x0.5): crisp, hushed, still, glacial, reverent. Never 0. **Guess:** the reference record for swaying and skipping with a triplet shuffle.

### Wholesome take
- Take the three-to-a-beat kit, the sine-like kick at 40 to 56 Hz, the planing major sevenths, the long filter rides, the 23 s intro floor drop and the one-beat echo; leave the vibrato-fast bass wobble (3 Hz, 9 to 16 cents) and the decorrelated pad if it fails the fold. **The harsher parts of the Fire-Toolz track are not there to describe**: its vocals stem holds 0.4% (bleed), and the loudest features are the bass-and-kick duck (21 dB) and the 4 octave filter rides, which are textures, not harshness; on this stretch nothing is screamed or distorted that the stem split can show. If a scream or growl turns up elsewhere on the record, the wholesome take is to use it as a rough texture on a sustained note (a quiet, filtered noise swell), not as a voice. **[JUDGMENT]** **Guess:** wholesome and bouncy.

### For later
- The other eight tracks are mix-level only: Interactions Between Brains & the Foam of Potentiality (6:10, the longest), Space-Time = Infinity-Eternity Objectified (6:08), Extended to Disparate Fields (3:47), Imprinted, Encoded, Shone (Emergence) (3:58), and the Missing Link (1:07, the shortest) and We're Smeared parts 0 and 1. **Guess:** deep-pass the two 6-minute tracks in an Ears 12b if the synthesis wants a long-form read (\`longform.py\` needs 8 minutes, so these would run on the deep-pass stages only).
- A listen to My Incomplete T.O.E. at 0:08 to 0:35 for the 23 s intro floor drop and its return. **Guess:** include it in the clips for the floor-drop item.

### Reliability notes for this card
- Four of twelve tracks were deep-passed (13 of 42 minutes). Part -1 has no bass stem and 6% drums, so its kick line is a few hits.
- The meter tool's phase is a quarter beat off on the triplet tracks, so I read spacings from \`beatphase.py\`, not absolute positions.
- The tuning number is a mix-level reading good to about 8 cents.

---

## Nonlocal Forecast, Conscious Agents (single, 2024)

Ears 12, 2026-11-07. One track, 4:19 (259 s), a single that follows *Holographic Universe(s)!*; deep-passed by the EP rule (every track of a record of six or fewer). No 8-minute track, so \`longform.py\` was not run. Stems by Demucs htdemucs. Machine time: 274 s for 4.3 minutes of audio (Demucs 259 s of audio in 177 s, 0.68 times; pick 15 s not included). Ears 9's formula (1.1 x 259 + 12) says 297 s, so it ran 8% under it.

### Stem check (read first)
- Drums 25%, bass 44%, other 31%, vocals 0% (0.4%: bleed on an instrumental). The meter tool's 80 BPM (0.752 s) is **three quarters of the drum grid's** 107.7 BPM (0.557 s): the grid is a dotted-eighth pulse of the beat. But \`beatphase.py\` finds the drum onsets spread evenly over the beat (7 to 12% in every twelfth, 596 onsets), so **there is no clear grid, and no swing or triplet claim**; the meter tool's "swung 0.71 (triplet feel)" is this spread. Kicks come every 0.755 s, one a beat.
- No subharmonic misread claimed (the bass groups at 40 to 62 Hz are harmonic 0.99 for the big two).

### The hook
- A loud, dark, wide single with long chords: -11.0 LUFS in the rip, centroid 405 Hz (56% of the energy under 120 Hz, 27% from 120 to 500, 12% from 500 to 2 k, 2.9% in 2 to 5 kHz, 1.2% above 5 kHz), width -8.3 dB, a pulse of 0.45 (looser than the album), tuning +4 cents (mix-level, good to about 8), 9.5 chord changes a minute at mix level, **a sweep of 0.78 octave in the median and 2.3 octaves at the biggest**.
- **[JUDGMENT]** One line a listener would say. **Guess:** the album's bubble slowed down and made rounder: more bass, longer chords.

### Sounds

**S1. A deep sine bass in two registers: bass-1 and -2, 117 and 46 notes, 54% and 25% of the stem's weight**
- In synth terms: 42 Hz (F1) with a 128 ms swell, falling 8 dB/s, harmonic 0.99, correlation 0.97; and 62 Hz (B1) with a 272 ms swell, 8 dB/s, harmonic 0.99. Two more small ones: a noisy 42 Hz pad (313 ms, 3 dB/s) and a 40 Hz pluck falling 50 dB/s. Under 60 Hz pitch is good to a semitone only.
- Building blocks: **[JUDGMENT]** **Guess:** sub with bassStyle held, a 130 to 270 ms swell; exists. A phone cannot play the 42 Hz one: carry it by the octave.

**S2. A wide, soft mid pad (other-1): 284 notes, 80% of the stem's weight**
- In synth terms: about 82 Hz (E2), a 192 ms swell, falls 9 dB/s, brightness 640 Hz, correlation 0.38 (wide), partials 46 cents off; a second at 228 Hz (176 ms, 4 dB/s, correlation 0.48).
- Building blocks: **[JUDGMENT]** **Guess:** softpad with a 200 ms attack and a chorus spread; exists.

**S3. A soft kit (drums-1, 361 notes, 36%)**
- **Kick line (kick_check):** 157 kicks, median 69 Hz (C#2 10%, D2 8%, E2 7%, C3 6%: spread over many notes, so it is partly bass), falling 107 dB/s (37 to 162), every 0.755 s. **Duck: none** (the bass level after the kick stays within -0.3 to -1.4 dB, no dip). A hat/tick group at 54 Hz with brightness 1.5 kHz, 139 notes.
- Building blocks: **[JUDGMENT]** **Guess:** a soft tunedKick, 107 dB/s fall, no subDuck; exists.

### Techniques
- **T1. Floor drops 1.29 a minute**, including **16.5 s at 3:00.5** and 12.0 s at 0:12.8 (plus 4.2 s at 0:50.0, 2.2 s at 1:56.0, 2.5 s at 4:03.0); the kit and bass leave and return on 26 s sections (a boundary median of 26 s). Drums in at 0:28.0, 0:53.5, 3:21.0, 3:48.5; bass in at 0:21.5, 1:57.5, 3:13.5. **[JUDGMENT]** **Guess:** floorDrop, long end.
- **T2. Sweeps up to 2.3 octaves.** Up 2.2 octaves 0:07.5 to 0:15.5; down 1.6 octaves 0:23.0 to 0:31.5; up 2.3 octaves 1:45.5 to 1:52.5 (the 8 to 9 s kind). **[JUDGMENT]** **Guess:** padSweep.
- **T3. No wobble, no echo, no pump** (nothing at 4 cents or more; none found; under 1.5 dB). A clean mix.

### Theory
- **Th1. Sus2 and major seventh chords over a moving root.** Bsus2 11%, Asus2 8%, F#sus2 8%, Emaj7 6% (40 distinct chords, 29 changes a minute, 12 settled, a 1.3 s median chord, **51% of the changes on a beat against 40% by chance**), E ionian (margin 0.024), the bass on E 20%, F# 18%, B 16%, A 10% (root movement, 0.83 notes a second, repeats 61%). **[JUDGMENT]** **Guess:** sus2/major seventh chords changing on the beat every 1 to 4 s over a root-moving bass; exists.
- **Th2. The top line leaps and arches.** 644 notes, range 15 semitones, steps 30%, fourths and fifths 23%, leaps 20%, phrases of 11 notes, 41% arches. No repetition: 0% near-duplicate bars; bar lengths unclear (best guess 8, score 0.03): **no odd meter**.

### Space
- Mix width by band (dB; 0 = unrelated channels): sub -16 (corr 0.96), 120 to 500 -5 (0.56), 500 to 2 k -4 (0.39), 2 to 5 k -5 (0.53), above 5 k -6 (0.59); width over time -7/-5/-1 (10th/50th/90th). **Mono loss** by band: 0.1, 1.0, 1.6, 1.2, 0.9 dB: soft in mono (1.6 dB at worst). The bass (-16 dB) and drums (-14 dB) are centered; the pad is the wide part (-4 dB, loss 1.6 dB).
- **[JUDGMENT]** **Guess:** the standard stereo pattern Ears 4 liked (centered low end, wide pad), nothing to guard against.

### Vibes
Numbers: 80 BPM (grid not clear), 12 settled chords a minute, 4% above 2 kHz, width -8 dB.
- **[JUDGMENT]** Lean toward (x1.5 to x3): floating, cosmic, hypnotic, spacious, airy, dreamy. Lean away (x0.2 to x0.5): crisp, bouncy, lively, zesty. Never 0. **Guess:** the slow-chord, deep-bass side of the Nonlocal Forecast family.

### Wholesome take
- Take all of it: the deep sine bass, the wide soft pad, the sus2 and major seventh chords on the beat, the 16.5 s floor drop and the clean mix. Nothing is harsh. **[JUDGMENT]** **Guess:** wholesome.

### For later
- Chord changes at 29 a minute that still land on a beat 51% of the time (40% by chance) are a pattern worth a look in Ears 13 against the generator's chord grid. **Guess:** wait.

### Reliability notes for this card
- One track, one sample of the record; it is a single, so the album medians do not apply. The tuning number is a mix-level reading good to about 8 cents.
