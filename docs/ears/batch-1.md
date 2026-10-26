# Ears, batch one: 20 albums measured against Driftloom

2026-10-09. A sounds-and-ideas brain chat, separate from the Rust brain; it
did not touch the repo or the queue. Written for Mikey and for whichever
brain turns this into queue items.

## What was measured

- **The records:** Mikey's first batch, 20 albums (the gentle list from this
  chat): 300 files, about 18.6 hours, 96 kbps Opus. *Drukqs* is its piano
  pieces plus a few others; *Neu! 2* is side one.
- **Driftloom:** `main` at `3149af1`, rendered offline through the real
  Engine and Synth in headless Chromium (JavaScript engine, full quality,
  "Let the loop wander" on at 1.0, as the app plays). 80 loops of 60 s
  (corpus seed 20261009, every profile represented) and the first 24 of
  them again at 150 s for the over-a-minute measures.
- **The same measures on both:** stereo width, energy by frequency band,
  how fast sound dies after a note, chord changes, tuning offset from
  A440, pitch wobble on held notes, note density, beat clarity,
  repetition, and how far the sound travels over 10 s and 60 s.
  Calibrated on test tones (a 5-cent wow read 3.3, a 12-cent wow 8.0;
  true values 3.5 and 8.5). Tools: `docs/ears/tools.md`.
- Spectrograms looked at by eye: "Quadrant Dub I", "lust", "Dawn Chorus",
  "Dino", "Petiatil Cx Htdui".

These numbers say what the records *do*, not whether it's pleasant.
That's still Mikey's call.

## The six differences, strongest first

### 1. Driftloom is mono; every album is stereo

All 20 albums are stereo: side-to-mid energy is -8 dB at the median, from
-2 (*Drukqs*, the piano wide across the room) to -15 (*Cupid's Head*,
nearly centered). Driftloom's left and right are identical. There is no
panner anywhere in `synth.js`, and NOTES.md (now `docs/archive/engineering-notes.md`) calls the mix "the mono bus".

This is the cheapest large change on the list. Bass and kick stay
centered, voices take places left and right by role and register, and the
reverb and echo get different left and right delay times (a ping-pong
echo, a reverb that isn't the same on both sides). **Yardstick:** side/mid
between -12 and -5 dB, like the records, with the channels still
positively correlated so a phone's single speaker sums cleanly.

### 2. The low end sits an octave below the records'

Share of energy by band, median per file:

| | 40–60 Hz | 60–120 | 120–250 | 250–1k | 1–2k | 2–5k |
|---|---|---|---|---|---|---|
| Records without a steady beat (173 files) | 0.7% | 11% | 21% | 32% | 2.3% | 0.6% |
| Driftloom loops without drums (31) | **37%** | 30% | 14% | 19% | 0.1% | 0.0% |
| Records with a steady beat (126) | 11% | 26% | 15% | 16% | 1.9% | 0.7% |
| Driftloom loops with drums (49) | 20% | **59%** | 10% | 6% | 0.1% | 0.1% |

The quiet records put their weight at 120 Hz–1 kHz. Driftloom's quiet
loops put more than a third of theirs at 40–60 Hz, a register most phone
and laptop speakers barely reproduce, and the phone app is the
deliverable. Only the dub-techno records go that low (*Narkopop* 62%,
*Cupid's Head* 51%), and they have a kick drum doing it.

The cause, from the code: the bass is clamped to MIDI 28–52 (41–131 Hz),
a quarter of loops drop a further octave, and the usual bass voice is a
pure-sine "sub" with nothing above its fundamental (`generator.js`, the
bass `place()` around line 819). The master lowpass closing toward 2.6 kHz
on warm loops (`synth.js` line 891) adds a little at the top.

Above 1 kHz, Driftloom sits at the very darkest end of these records all
the time. Some albums are as dark (*On Land* 0.2%, *Blau* 0.1%), but the
records range up to 26% (*Iasos*), with a median of 4%. Measured against
its own midrange rather than its bass, Driftloom's 1–5 kHz band is about
17 times weaker than the quiet records'.

**Yardstick:** for calm, beatless loops, most bass energy at 80–250 Hz
(up an octave, or a sub with a 2nd and 3rd harmonic so it reads on small
speakers), and the 1–2 kHz band brought up toward the records' 1–3%.
The gap is presence, not sizzle: the records' share above 2 kHz is small
too, so this stays within the coffee-shop rule. A blind A/B, since it
changes sounds Mikey likes.

### 3. Chords change about ten times as often

The generator's own data for the 24 long loops: a chord change about **29
times a minute** at the median, one every two seconds. The records,
measured: **2.4 a minute** at the median; 8 of the 20 albums are
under 1 a minute (*BCD*, *Narkopop*, *Cupid's Head*, *Pendant*,
*Harmonia*, *Stack Music*, *Blau*, *Neu! 2*) and two more under 2
(*On Land*, *Orbus Terrarum*). The
counter undercounts fast changes (it read Driftloom as 17), so the real
gap is at least this big.

Busy harmony isn't wrong (*Drukqs* piano 15, *Inner Song* 12, *[lust]*
11 a minute). But Driftloom has almost no loops at the slow end, where
half these records live. That's idea 3 from this chat: **a share of loops
on one chord or a pedal bass**, and in calm profiles chords held 2–4
bars. **Yardstick:** some loops under 2 changes a minute, without
lowering the share of tuneful loops (the balance lock).

### 4. The records travel; Driftloom loops circle

How far the sound has moved after 60 seconds, divided by how far after 10:
every album is above 1 (median 1.34: the music keeps going somewhere),
while Driftloom's long loops sit at **0.96**. After a minute, a loop is no
further from where it started than after 10 seconds. "Wander" changes
notes, but each pass starts again from the saved loop, so nothing
accumulates. Loudness range agrees: 7.4 LU in the records, 3.4 in the long
loops.

Repetition itself isn't the difference. Over a few seconds, Driftloom (0.59)
is about as self-similar as the records (0.50); *Cupid's Head* is more
(0.66). The records repeat just as much; what they add is something
moving slowly underneath. "Quadrant Dub I" shows it: one chord stab in a
narrow band at 520–900 Hz, and over a minute, faint upper partials creep
in above 1.5 kHz as a filter opens.

That's the slow-arc idea: one thing (a filter, echo feedback, a layer's
level, density) travels over 2–5 minutes and comes back. **Yardstick:** a
60-to-10 ratio of 1.2 or more on long renders, with the repetition figure
unchanged.

### 5. Notes ring 4–7 times longer in the records

After an isolated note, the records' sound dies at **1.2 dB per second**
(median; 18 of 20 albums between 0.3 and 2.4). Driftloom's dies at 4.5–9
dB per second (the 150-s and 60-s samples). The two dry albums are *Drukqs*
piano (7.0) and *[lust]* (5.8), so dry is a real flavor, but most of these
records let reverb and echo carry each note. **Yardstick:** calm profiles
toward 1–2 dB per second; echo throws (idea 2) are one way to get there.

### 6. Some records are tuned well off A440 (corrected in batch two)

*Correction, from `docs/ears/batch-2.md`:* the line below saying
Driftloom's loops "sit within ±2" was wrong. Per loop the measure reads
Driftloom from -17 to +10 cents (middle half -5 to +2), because its
doubled voices are detuned against each other and the measure wobbles on
a single file. Only offsets well beyond about ±8 cents mean anything: 7
of these 20 albums (the seven named below), not 12. This item is now
last and optional.

12 of 20 albums sit 8 or more cents from concert pitch: *Iasos* +32,
*Blau* +22, *Narkopop* +16, *Harmonia* -18, *Cluster* -13, *Cupid's Head*
-12, *Strom* +12. Per file, the middle half runs from -12 to +16 cents.
Driftloom's loops sit within ±2. Nobody hears 15 cents as out of tune with
itself; it reads as an old record, a tape running slightly fast or slow.
**Yardstick:** some loops shifted as a whole by 10–30 cents.

The tape wobble already in the chain is in range: held notes waver 3.6
cents in Driftloom against 4.5 in the records (median), 5.4–6.9 on the
tape-heavy ones (*On Land*, *Pendant*, *Blau*, *BCD*). It could go deeper
now and then; no change is needed.

## What already matches

Note density (3.6 a second against 4.0), repetition and pitch wobble are
inside the records' range. Driftloom's beat is clearer than the records'
median (0.67 against 0.50 on a 0–1 scale), but still within their range,
from *Private Parts* (0.20) to *Cupid's Head* (0.85). The rhythm side
isn't where the difference is.

## Album by album

Median per file. Width is side/mid in dB (0 would be fully wide); the
band columns are shares of energy; tail is how fast sound dies after a
note; drift 60/10 above 1 means the music keeps moving.

| Album | Width (dB) | 40–60 Hz % | 1–5 kHz % | Chords/min | Tail (dB/s) | Tuning (cents) | Drift 60/10 |
|---|---|---|---|---|---|---|---|
| Aphex Twin, *Drukqs* (piano) | -2 | 0.1 | 3.2 | 15.1 | -7.0 | -9 | 1.19 |
| Basic Channel, *BCD* | -12 | 11.6 | 1.4 | 0.0 | -1.6 | -2 | 1.70 |
| Boards of Canada, *Geogaddi* | -8 | 0.8 | 10.3 | 10.1 | -0.7 | +0 | 1.27 |
| Brian Eno, *On Land* | -6 | 8.5 | 0.2 | 1.5 | -1.2 | +8 | 1.09 |
| Cluster, *Sowiesoso* | -5 | 0.9 | 10.6 | 3.1 | -2.4 | -13 | 1.29 |
| Gas, *Narkopop* | -11 | 62.3 | 0.8 | 0.0 | -0.8 | +16 | 1.48 |
| Harmonia, *Musik von Harmonia* | -10 | 0.5 | 4.5 | 0.3 | -1.4 | -18 | 1.53 |
| Iasos, *Inter-Dimensional Music* | -8 | 0.1 | 26.1 | 3.3 | -1.4 | +32 | 1.43 |
| Kelly Lee Owens, *Inner Song* | -11 | 30.5 | 2.2 | 12.0 | -0.6 | +3 | 1.44 |
| Konrad Sprenger, *Stack Music* | -3 | 0.7 | 13.8 | 0.6 | -0.7 | +5 | 1.44 |
| Monoton, *Blau* | -9 | 15.6 | 0.1 | 0.7 | -1.7 | +22 | 1.33 |
| Neu!, *Neu! 2* | -4 | 0.9 | 13.4 | 0.5 | -1.4 | +6 | 1.55 |
| Nine Inch Nails, *Ghosts V* | -5 | 0.0 | 2.8 | 6.0 | -1.4 | +1 | 1.45 |
| Pauline Anna Strom, *Trans-Millenia* | -4 | 0.2 | 7.3 | 6.3 | -1.6 | +12 | 1.17 |
| Pendant, *Make Me Know You Sweet* | -9 | 9.2 | 0.7 | 0.0 | -0.3 | -9 | 1.51 |
| Rei Harakami, *[lust]* | -6 | 5.5 | 6.8 | 10.6 | -5.8 | +0 | 1.23 |
| Robert Ashley, *Private Parts* | -7 | 0.0 | 4.4 | 11.2 | -1.7 | -8 | 1.22 |
| The Field, *Cupid's Head* | -15 | 50.9 | 2.8 | 0.0 | -1.2 | -12 | 1.56 |
| The Orb, *Orbus Terrarum* | -9 | 24.9 | 4.4 | 1.9 | -1.7 | +9 | 1.56 |
| Yialmelic Frequencies, *Yililok* | -6 | 6.2 | 0.6 | 3.3 | -0.8 | -1 | 1.22 |
| **Driftloom, 80 loops** | **mono** | **25.4** | **0.2** | **17.5 (true ≈29)** | **-8.8** | **-2** | **0.96** (24 long loops) |

## Seen in the spectrograms

- **"Quadrant Dub I"** (Basic Channel): one chord stab in a narrow
  520–900 Hz band, repeating; over the minute, faint partials appear above
  1.5 kHz as a filter opens. The slow arc in its simplest form.
- **"lust"** (Rei Harakami): the middle layer drops out for a beat about
  every 17 seconds, a regular breath in the arrangement.
- **"Dawn Chorus"** (Boards of Canada): the chord pad plays identical
  4.3-second blocks and stops for about a second before each repeat, and
  the drums and bass drop with it.
- **"Dino"** (Harmonia): the organ holds one steady band while short,
  bright figures climb a little higher phrase by phrase.
- **"Petiatil Cx Htdui"** (Aphex Twin): chords at 250–550 Hz and a high
  figure at 900–1,500 Hz, with a clear gap between them; almost nothing
  above 2.5 kHz; a low thump from the instrument's mechanism under every
  note; one note left ringing for about 10 seconds at the end.

## The ten ideas, with what the numbers say

1. **Slow arcs:** strongly supported (difference 4).
2. **Dub throws:** supported by tails (difference 5) and *BCD*/*Orb*.
3. **One chord, free melody:** strongly supported (difference 3).
4. **Tape that bends:** the tuning half is supported (difference 6); the
   wobble half is already in range.
5. **Euclidean and stacked pulses:** nothing in these numbers argues for or
   against; a taste call.
6. **Motorik:** nothing in these numbers decides it; a taste call.
7. **Childlike repeating figures:** "Dino" shows the shape; a composition
   choice for the ear.
8. **Submerged mix:** careful. Driftloom is already low and dark; this
   fits only a few loops, and only after difference 2 is fixed.
9. **Sparkle:** *Iasos* has 26% of its energy at 1–5 kHz, *Stack Music*
   14%, *Neu! 2* 13%; that's where sparkle lives and where Driftloom is
   nearly empty.
10. **Night chords:** fuller chords fit the records: 3 pitch classes
    sounding above the bass at the median, against 2 for Driftloom.

## For the brain: candidate queue items

In order of evidence, then cost. All but the first touch sounds Mikey
already likes, so per BRAIN.md they get blind A/Bs. **Sequencing:** queue
items 29–31 move the effects and master chain into the Rust core, and 1,
2 and 5 below touch exactly that chain, so they should come after 31 or
be folded into it.

1. **Stereo.** Pan by role and register; a ping-pong echo and a stereo
   reverb. Side/mid -12 to -5 dB; channels positively correlated.
2. **Bass register and upper presence.** Bass up an octave or with
   harmonics on calm profiles; 1–2 kHz toward 1–3%. Check on a phone.
3. **Slow arcs.** One parameter traveling over 2–5 minutes. Drift 60/10
   at 1.2 or more.
4. **Harmonic rhythm.** A share of one-chord and pedal loops; 2–4 bar
   chords in calm profiles. Some loops under 2 changes a minute.
5. **Tails and echo throws.** Calm profiles toward 1–2 dB per second.
6. **Tuning offset.** Some loops shifted 10–30 cents as a whole. (Weaker
   than first stated; see the correction under difference 6.)

New instruments (this chat's list: liquid lead, glassy FM electric piano,
soft drawbar organ, dub stab, synthesized nature, tabla-like drum,
electric pluck, blurred strings) are best built after 1 and 2, so they
are born into a stereo mix with room above 1 kHz.

## Caveats

- Records are mastered: their loudness (LUFS) isn't comparable with
  Driftloom's unnormalized output and isn't used here.
- The chord counter undercounts fast changes; checked against the
  generator's data.
- Tails are measured only after isolated notes, so dense textures give
  fewer samples; the 60-s and 150-s Driftloom samples differ (8.8 and 4.5
  dB/s), and both are far from the records.
- The pitch-bend figure was near zero everywhere and isn't used.
- Band shares describe energy, not loudness as heard; the ear is far less
  sensitive at 50 Hz. That cuts both ways: a loop whose energy is mostly
  at 50 Hz sounds quieter and thinner than its meter says, especially on
  a phone.

## Next batch

Mikey plans 30–40 more albums. The route that worked: `driftloom-ears.sh`
(Opus at 96 kbps), then `driftloom-ears-split.sh` (pieces under 5.5 MB,
since the Drive connector carries files up to about 6 MB), uploaded to
the Driftloom folder in Drive. A coffee-shop room recording is still
wanted for difference 2.
