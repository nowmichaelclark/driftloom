# Ears, batch two: Music Pile 2 against Driftloom and batch one

2026-10-09. From the same sounds-and-ideas brain chat as batch one. It did
not touch the repo or the queue. Written for Mikey and for whichever brain
turns this into queue items. Read `docs/ears/batch-1.md` first; this
report checks its findings against a second, very different pile and adds
what's new.

## What was measured

- **The records:** Music Pile 2, 12 albums: 203 files, about 11.2 hours,
  96 kbps Opus. A few tails of a minute or less (cut-off ends of long
  tracks, under 300 KB) were skipped.
- **The pile is different from batch one.** Batch one was mostly beatless
  and gentle. Here 153 of 203 files have a steady beat (pulse 0.6 or more),
  against 126 of 299 before, and the music is brighter, busier and heavier
  in the low end. If batch one's findings hold here too, they aren't
  about one kind of record.
- **Driftloom:** the same renders as batch one (`main` at `3149af1`; 80
  loops of 60 s, 24 of 150 s, "Let the loop wander" on at 1.0). The Rust
  brain has since merged item 29 (`fa0f9f3`); these renders are from the
  JavaScript engine, before it.
- **The same measures and the same definitions as batch one.** One new
  measure, run on both batches and on Driftloom: how often the music
  **drops its low end** while the rest keeps playing, and how often it
  **stops completely** (see "What's new" below).
- Spectrograms looked at by eye: "New Colour" (Lone), "LFO" (LFO),
  "Thor's Stone" (Forest Swords), "Over the Ice" (The Field), "Port
  Gentil" (Porter Ricks), "Boring Angel" (OPN), "nil" (Autechre),
  "Thousand Knives" (Sakamoto), and two Driftloom loops ("undertow",
  "dust").

## The short version

All six of batch one's differences show up again, five of them strongly.
The sixth, tuning, was overstated in batch one and is corrected below.
Pile 2 adds two things: a clearer picture of the top end, and a new
difference in how the music pauses. Driftloom's pauses are full stops,
while the records keep sounding and drop only the kick and bass.

| | Batch one (299 files) | Pile 2 (203) | Driftloom |
|---|---|---|---|
| Stereo width, side/mid (dB) | -7.9 | -9.0 | mono |
| Energy at 1–5 kHz | 3.2% | 7.5% | 0.2% |
| Chord changes a minute | 2.4 | 4.3 | 17.5 measured (≈29 true) |
| Drift after 60 s ÷ after 10 s | 1.34 | 1.45 | 0.96 (long loops) |
| Tail after a note (dB/s) | -1.2 | -1.9 | -4.5 to -8.8 |
| Loudness range (LU) | 7.4 | 5.1 | 3.1 to 3.4 |
| Full stops of 1 s or more, a minute | 0.03 | 0.06 | 1.11 (60 s), 0.62 (150 s) |

Medians per file. Width: every one of the 12 albums is stereo, from -5
(*Thousand Knives*) to -19 (*Expert Knob Twiddlers*).

## The six differences, checked again

### 1. Stereo: holds

All 12 albums are stereo (album medians -5 to -19 dB side/mid; -9 per file). The
narrowest are the most club-like (*Expert Knob Twiddlers* -19, *LFO* -17,
*From Here We Go Sublime* -15), and they're still clearly stereo. Nothing
changes in batch one's item 1.

### 2. Bass register and upper presence: holds, and the picture is clearer

**The top end.** Pile 2 is brighter than batch one: 7.5% of its energy is
at 1–5 kHz against 3.2%. Driftloom has 0.2%, about **30 times less**. The
brightest of Driftloom's 80 loops has 2.5% there. Eleven of the twelve
albums sit above that at the median; the only one below is *Biokinetics*
(1.4%), a deliberately submerged dub record. *Thousand Knives* has 21%,
*Amber* 13%, *Galaxy Garden* 10%.

Median share of energy by band:

| | 40–60 Hz | 60–120 | 120–250 | 250–1k | 1–2k | 2–5k | 5k+ |
|---|---|---|---|---|---|---|---|
| Batch one, beat (126 files) | 11% | 26% | 15% | 16% | 1.9% | 0.7% | 0.2% |
| Pile 2, beat (153) | 16% | 23% | 13% | 17% | 4.2% | 3.0% | 1.4% |
| Driftloom with drums (49) | 20% | **59%** | 10% | 6% | 0.1% | 0.1% | 0.0% |
| Batch one, no steady beat (173) | 0.7% | 11% | 21% | 32% | 2.3% | 0.6% | 0.0% |
| Pile 2, no steady beat (50) | 15% | 20% | 12% | 22% | 4.8% | 1.9% | 0.2% |
| Driftloom without drums (31) | **37%** | 30% | 14% | 19% | 0.1% | 0.0% | 0.0% |

**The low end, refined.** Pile 2 is heavier down low than batch one, so
it softens one half of batch one's claim and sharpens the other:

- **Loops with drums:** Driftloom's 40–60 Hz (20%) is in range: pile 2's
  beat records have 16%, and *From Here We Go Sublime* has 61%. The excess
  is one octave up, at **60–120 Hz: 59%**, against 23% in pile 2 and 26%
  in batch one. Every one of the 12 albums is lower (12% to 43%, the
  top being *Biokinetics*).
- **Loops without drums:** 37% at 40–60 Hz is still the outlier. Batch
  one's calm records have 0.7% there; pile 2's beatless pieces have 15%.
  These are mostly beat records with a weaker pulse (*R Plus Seven*,
  *Engravings*), not ambient.

**Yardstick, updated:** drum loops under about 35% at 60–120 Hz;
drumless loops under about 15% at 40–60 Hz; 1–5 kHz at 1% or more in most
loops, where the records are (75% of batch-one files, 92% of pile-2 files,
10% of Driftloom loops). It's still presence, not sizzle, and still a
blind A/B.

### 3. Chords change several times as often: holds, softer

Pile 2 changes chords more often than batch one (4.3 a minute at the
median against 2.4). Only three albums are under 2 a minute: *Biokinetics*
(0), *From Here We Go Sublime* (1.0) and *Fashion Week* (1.7). The busiest
are *Galaxy Garden* (11.9) and *R Plus Seven* (10.0). Driftloom measures
17.5 with the same counter (about 29 by the generator's own count), so it's
still around four times busier than this pile, and more than that in
truth. Batch one's item 4 stands. Pile 2 just shows that the slow end
isn't the only place records live.

### 4. Records travel, loops circle: holds, strongly

All 12 albums are above 1 (1.45 per file, album values from 1.13 for *From Here We Go
Sublime* to 1.73 for *Galaxy Garden*); Driftloom's long loops are 0.96.
Across both batches that's 32 of 32 albums. The spectrograms show what it
looks like. In "Over the Ice", a filter sweep arcs through 2–4 kHz over
about 15 seconds, above a loop that otherwise barely changes. In "Port
Gentil", a noise wash rises through 500 Hz–2 kHz and falls away again,
twice in the minute, about 20 seconds apart.

### 5. Notes ring longer in the records: holds

Pile 2's median tail is -1.9 dB per second, against Driftloom's -4.5 to
-8.8. Seven of the 12 albums fall in batch one's 0.3–2.4 range (one,
*From Here We Go Sublime*, right at its edge). Two show no
clean decay at all (*Biokinetics* +0.1, *Blank Banshee 0* +0.4): the
sound never stops long enough to measure one, a continuous wash. Dry is
again a real flavor, and pile 2 adds two examples: *LFO* (-8.7, as dry as
Driftloom) and *Expert Knob Twiddlers* (-5.1). That's 4 dry albums out
of 32, so dry should stay possible. It's just rare in the records and
the default in Driftloom.

### 6. Tuning offset: overstated in batch one; corrected

Batch one said Driftloom's loops "sit within ±2" cents of A440. That was
wrong. Per loop, the measure reads Driftloom from -17 to +10 cents (middle
half -5 to +2). Driftloom does play at A440, but its doubled voices are
detuned against each other (pairs at -7/+6 cents; FM layers at +4 to +9),
and
the measure itself wobbles on a single file. So per-file readings within
about ±8 cents prove nothing. What survives is the albums that sit well
beyond that: in batch one *Iasos* (+32), *Blau* (+22), *Harmonia* (-18),
*Narkopop* (+16), *Sowiesoso* (-13), *Cupid's Head* (-12) and *Strom*
(+12); in pile 2 only *Thousand Knives* (+14). That's 8 of 32 albums, not
12 of 20. **Item 6 moves to last place and becomes optional:** a few
loops shifted 10–30 cents is still a fair "old tape" color, but the
evidence for it is modest.

## What's new in pile 2

### Driftloom pauses by stopping; the records drop the floor

This is the new measure. It counts two things:

- **Full stops:** a second or more where the whole mix falls 30 dB under
  its usual level.
- **Floor drops:** two seconds or more where the kick and bass (below
  120 Hz) fall away while everything above keeps playing. This is only
  counted where the low end is normally there.

The records almost never stop: 0.03 full stops a minute in batch one,
0.06 in pile 2 (9 of 297 and 20 of 203 pieces have any). Driftloom's 60-s
loops stop **1.11 times a minute**, and the 150-s loops 0.62 times a
minute: roughly 10 to 40 times as often, depending on which batch and
which renders you compare. 36 of the 80 loops have at least one full
stop.

Instead, records with a beat drop the floor. Of the 227 beat pieces
whose low end is normally steady, 43% have at least one moment where the
kick and bass go out for about 3 seconds while the rest plays on, about
once every two to three minutes on average
(0.42 a minute). Among albums with at least five such pieces, *Galaxy
Garden* does it most (1.9 a minute), then *Inner Song* (0.95) and
*Incunabula* (0.85). Driftloom's drum loops did it **zero** times in the
37 measurable loops of 60 s (12 of the 49 have too sparse a low end to
count); of the 11 measurable loops of 150 s, one ("glade") did it
twice.

The spectrograms show it plainly:

- **"undertow" (Driftloom):** every band goes black at the same moment,
  a 1–2 second gap about every 8 seconds. Each pass ends in silence, and
  the notes stop dead instead of ringing into the gap.
- **"LFO" (LFO):** at about 20 s everything above 100 Hz cuts out and a
  bare sub-bass line plays alone for 4 seconds; earlier, the top above
  1 kHz drops out for three seconds while the bass and middle keep
  going. Parts leave and return; the track itself never stops.
- **"nil" (Autechre):** for 19 seconds the drums and bass drop out and a
  held low drone stays under soft bell-like notes at 250 Hz–2 kHz, each
  with a long tail and a slight bend, then everything comes back at once.
- **"Over the Ice" (The Field):** the kick stops for 7 seconds while the
  chords keep going, then the kick returns and a filter sweep opens above
  it.

In the code: Drift's "now and then the whole thing stops" (1–2 bars,
`generator.js` about line 2705), the entry schedule's silent bars, and
short tails all leave full gaps. Drift's single-layer breath picks
melody, chords or drums, never bass, and "the kick sits one out" removes
a single hit. Nothing drops the kick and bass together for a bar or two.
The full stop is a deliberate design choice (the comment says so), so
this is a taste call for Mikey, not a bug. But the records are clear about
how they pause. **Yardstick:** on long drum renders, about 0.3–0.5 floor
drops a minute, and full stops under 0.1 a minute, with echo or reverb
carrying through any that remain.

### The top end is where the records' detail lives

Pile 2 shows what lives above 1 kHz. In "Thousand Knives" the lead
glides and wavers around 1–2.5 kHz over a bass line that changes note
several times a second. In "nil" the bell notes sit at 0.5–2 kHz. In
"Thor's Stone" a brighter layer opens up to 2–4 kHz in the last third.
"New Colour" is full from 40 Hz to 8 kHz, dense with short repeated
notes. Driftloom has almost nothing there, so this is where
new instruments (the glassy FM electric piano, the dub stab, the electric
pluck) would add the most.

### Smaller observations

- **Bass that slides.** In "Thor's Stone" the bass glides between notes,
  as diagonal lines at 60–200 Hz under every phrase. Driftloom already has
  bass glide (`glideChance`); this is a reference for how it can sound
  when it's the bass's main move.
- **Chords in blocks of 4 to 15 seconds over a fixed groove.** "Over the
  Ice" changes chord every 4–11 seconds and "Boring Angel" every 15 or so,
  while the pulse underneath stays fixed: a middle ground between batch
  one's one-chord records and Driftloom's chord every two seconds.
- **Wobble** is still in range (4.3 cents against Driftloom's 3.6).
  *Biokinetics* (6.4) and *Blank Banshee 0* (6.0) waver most.
- **Fuller chords** again: 3 pitch classes above the bass at the median,
  against Driftloom's 2.

## Album by album

Median per file. Width is side/mid in dB; band columns are shares of
energy; tail is how fast sound dies after a note (~0 means a continuous
wash, no clean decay); drift 60/10 above 1 means the music keeps moving;
pulse is beat clarity, 0–1.

| Album | Width (dB) | 40–60 Hz % | 60–120 % | 1–5 kHz % | Chords/min | Tail (dB/s) | Tuning (cents) | Drift 60/10 | Pulse |
|---|---|---|---|---|---|---|---|---|---|
| Autechre, *Amber* | -6 | 5.8 | 19 | 13.1 | 6.5 | -1.4 | +2 | 1.46 | 0.76 |
| Autechre, *Incunabula* | -8 | 31.1 | 23 | 9.6 | 6.8 | -2.1 | -8 | 1.51 | 0.76 |
| Blank Banshee, *Blank Banshee 0* | -7 | 14.9 | 25 | 5.0 | 7.2 | ~0 | +1 | 1.35 | 0.71 |
| Death Grips, *Fashion Week* (instr.) | -7 | 15.9 | 22 | 12.1 | 1.7 | -0.7 | +2 | 1.16 | 0.60 |
| Forest Swords, *Engravings* | -10 | 9.9 | 15 | 8.0 | 6.4 | -1.8 | +2 | 1.71 | 0.58 |
| LFO, *Frequencies* | -17 | 26.6 | 29 | 6.8 | 2.2 | -8.7 | -3 | 1.39 | 0.74 |
| Lone, *Galaxy Garden* | -8 | 18.8 | 12 | 10.0 | 11.9 | -1.6 | +0 | 1.73 | 0.72 |
| Mike & Rich, *Expert Knob Twiddlers* | -19 | 11.8 | 30 | 6.7 | 7.0 | -5.1 | +2 | 1.60 | 0.70 |
| Oneohtrix Point Never, *R Plus Seven* | -7 | 11.8 | 18 | 6.7 | 10.0 | -2.5 | -1 | 1.54 | 0.46 |
| Porter Ricks, *Biokinetics* | -12 | 13.1 | 43 | 1.4 | 0.0 | ~0 | -8 | 1.58 | 0.79 |
| Ryuichi Sakamoto, *Thousand Knives* | -5 | 0.8 | 17 | 20.8 | 7.5 | -1.4 | +14 | 1.38 | 0.67 |
| The Field, *From Here We Go Sublime* | -15 | 61.3 | 18 | 3.1 | 1.0 | -2.4 | +7 | 1.13 | 0.87 |
| **Driftloom, 80 loops** | **mono** | **25.4** | **54** | **0.2** | **17.5 (≈29)** | **-8.8** | **-2** | **0.96** | **0.67** |

## Which albums to draw from

Mikey asked for wholesome, comforting sources. Everything was measured,
but two are for technique only:

- ***Fashion Week*** (Death Grips instrumentals) is abrasive; nothing in
  it needs borrowing that a gentler album doesn't also show.
- ***Blank Banshee 0***: the music is mostly mellow, but several track
  titles aren't wholesome. Use it for sound only, if at all.

The most useful for Driftloom, by what they show best:

- ***Galaxy Garden*** (Lone): warm, bright, busy, and the most floor drops
  of any album.
- ***Thousand Knives*** (Sakamoto): the brightest album in this pile, with a
  gliding lead and a gentle old-tape tuning (+14).
- ***Amber*** (Autechre): gentle electronic textures, bell-like
  breakdowns, and tails in the records' usual range.
- ***Engravings*** (Forest Swords): echo, space, sliding bass, and a slow
  arc that opens the top end.
- ***From Here We Go Sublime*** (The Field): the loop that never changes
  while a filter sweeps over it, the slow arc at its plainest.
- ***Biokinetics*** (Porter Ricks): the far end of "tails": a wash that
  never decays, and one chord.
- ***R Plus Seven*** (OPN): long held organ chords changing every 15
  seconds or so, over a weak pulse.
- ***Frequencies*** (LFO) and ***Expert Knob Twiddlers*** (Mike & Rich):
  the dry, narrow, playful corner, close to Driftloom's own sound. They
  show that dry can work, and that dry tracks still drop the floor rather
  than stop.

## For the brain: candidate queue items, updated

Same order of evidence as batch one, with one item added and one moved
to last. The sequencing note stands: 1, 2 and 5 touch the effects and
master chain that queue items 29–31 move into Rust, so they come after
31 or fold into it. Blind A/Bs for anything that changes a sound Mikey
already likes.

1. **Stereo.** Unchanged. Now 32 of 32 albums are stereo.
2. **Bass register and upper presence.** Drum loops under about 35% at
   60–120 Hz; drumless loops under about 15% at 40–60 Hz; 1–5 kHz at 1% or
   more in most loops. Check on a phone.
3. **Slow arcs.** Unchanged. 32 of 32 albums travel; Driftloom circles.
4. **Harmonic rhythm.** Unchanged in direction. Include the middle
   ground: chords held 4–15 seconds over a steady groove, not only
   one-chord loops.
5. **Tails and echo throws.** Unchanged, and now tied to item 6: a tail
   is what fills a pause.
6. **Floor drops instead of full stops (new).** In loops with drums, now
   and then drop kick and bass together for one or two bars while the
   rest keeps playing; turn some of Drift's full stops into these.
   Yardstick: about 0.3–0.5 floor drops a minute on long drum renders;
   full stops under 0.1 a minute. A generator change that touches the
   Drift contract, so it needs Mikey's yes before anyone builds it.
7. **Tuning offset (moved last, optional).** Some loops shifted 10–30
   cents as a whole. The evidence is modest (8 of 32 albums).

New instruments: the case for building them after 1 and 2 is stronger
now, since nearly all of the detail pile 2 adds sits above 1 kHz.

## Caveats

- Same as batch one: LUFS isn't comparable and isn't used; the chord
  counter undercounts fast changes; tails come only from isolated notes;
  band shares are energy, not loudness as heard.
- Pile 2 is beat-heavy, so its "no steady beat" group (50 files) is
  mostly beat records with a weak pulse, not ambient music. Batch one
  stays the reference for calm loops.
- The floor-drop and full-stop measures are new and were checked by eye
  on the spectrograms listed above, not against hand-marked timings. The
  Driftloom 60-s loops give only 48 s of measurable music each, after
  trimming the first and last 10%, so per-minute rates are the fair
  comparison, not "how many loops had one".
- Driftloom's renders predate the Rust effects work (item 29); its
  echo and reverb may already sound different, which would mostly touch
  tails and full stops.

## Tools

`docs/ears/tools.md` has the scripts. Added for this batch: `sweep3.py`
(files Drive downloads by title), `sections.py` (floor drops and full
stops), `summarize2.py` (this report's tables) and `spec.py`
(spectrograms).
