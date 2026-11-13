# 102: what brain B3 measured on the long pieces (2026-11-14)

For session 16, beside `docs/notes/102-mikey.md` (his words, when they come).
On `main` at v164 (88c98be). Everything here was rendered and measured by the
brain, not taken from the reports. The first part is bugs: fix them in session
16 whatever Mikey says. The rest is what his notes will most likely touch.

## Verified (item 102 against the brief, LONGFORM and promise 11)

- `build-core.sh --check`: the module matches its source (801,267 bytes).
- Generation goldens 5,135 of 5,135; the balance lock passes.
- Loops of up to 32 bars untouched: 15 renders (seeds 0, 3, 11, 42 and
  gluith-vuisk; no request, `bars=32`, `bars=16`; 40 s), bit for bit the same
  at dfe78c3 (before session 12) and at v164.
- `cargo test --release` for `longform`, `joins`, `length` and `themes`: 27
  of 27, among them a section the same reached straight or by playing (its
  joins too) and the last section of 32,768 bars reached straight. The
  player's own jump is 106a's (session 17); nothing to check there yet.
- `node tools/longform.mjs --plan` (240 seeds): every check. A played run
  (60 seeds, 4 wanders, 16 arcs, 8 slider closes): every check but "darker at
  the opening and the close than at the peak", 81% of 16 (gluith-vuisk and
  rong-wuisk, batch 6's slow opening, and 176315227), as session 15 found
  (91% of 47).
- A section's making (`dlgen --section-cost`, three seeds, 1,024 and 32,768
  bars, Endless arc and wander): 11 to 29 us at section 1, 1,000 and 10^9
  alike; it varies with the role, never with the number.
- Maps read at 64, 256, 1,024 and 32,768 bars and Endless (section 10^9 too):
  as LONGFORM describes.

## Bugs (1 and 2 fixed in session 16; 3 withdrawn by his ears)

1. **A pivot can be silent.** `heard.rs` takes the pivot's velocity from the
   loop's first chord event with notes (`chord`, near line 841), which is a
   silent one (vel 0, an entry's) in 20 of 240 corpus loops: every pivot in
   those pieces is struck at 0 while the loop's own chords are taken out for
   the join, so a key change is a hole (the landing's bug of 102c again; its
   fix took a chord the loop plays). fei-no (seed 2158762586, 256 bars): at
   the pivot into the peak (5:23 to 5:27) only a decaying chord sounds, 40 to
   58 dB under the loop there; loa-yer (2975389383, in the listening set) is
   another. Take the pivot's velocity as the landing does.
2. **"The tune alone" where the varied tune is silent.** fei-no's last return
   unit (stretched) joins the close "alone" at 8:08 to 8:11: every layer out
   but a tune that, stretched, plays nothing there: three seconds 30 to 50 dB
   under the loop. The check for "the tune plays there" must read the tune as
   the unit plays it (its variation), else take a breath or a glide.
3. **A development stutters.** (Withdrawn 2026-11-14: Mikey heard it in
   moan-teil's look-back, flagged, and kept it; joins "Welcome". Not changed.) Where the development's units are short (4 or
   8 bars), A's and B's fragments trade every unit and each trade is a
   theme's line, so nearly every unit ends in a join, mostly a breath (every
   layer out for half a beat, 8 to 13 dB under the loop): sheim-gling
   (1278568317, 1,024 bars) has 20 joins in a row from 7:13 to 9:40, a gap
   every 7.3 s; moan-teil's look-back (29:17 to 30:30, in the listening set)
   one every 8 s. Over 120 corpus seeds about 80% of development units end
   in a join and 40% (256 bars) to 56% (1,024) of arc pieces have four or more
   in a row (up to 31). A fragment's trade inside the development should
   glide; a join belongs where the role, the key or the stated theme changes.

## Where the measures and the plan disagree

- **The joins are uneven:** measured against the same moment before item
  102, a floor out takes 0.6 to 1.8 dB off in brum-yei and tha-vol (a long
  bass note rings across the line, the kit is sparse: nearly nothing goes),
  while in sparse loops a pivot or "alone" leaves near silence (above). The
  checks look for clicks and loudness steps, not for how much a join takes
  out, so both ends pass. A yardstick for the depth of a join (some dB, never
  silence) would catch both.
- **The arc's energy is about 1.5 to 2 LU against the loop:** brum-yei (256)
  sits 1.5 LU under the loop at its opening, 1.4 in the theme, 0 at the peak,
  1.9 in the breath; moan-teil's five movements the same (the full arc's
  peak 0.5 LU under the loop, the gentle one's 1.3, the light one's 0.3: the
  kinds barely differ in level). The opening and the theme are within 0.2 LU
  of each other. Its brightness at an opening 0.87 to 0.93 of the loop's.
- **What carries the shape is the key:** the arc's related keys move the
  chords, the tune and the texture up (`shift_up`: a fifth above is seven
  semitones up), so from theme B to the release, about 45% of a full arc,
  the loop plays a fourth or a fifth higher (centroid 1.2 to 1.5 times the
  loop's) and drops back at the return. That is the most audible thing an arc
  does, more than its energy.
- **The wander starts as the old app did:** a piece starts as the loop's own
  and holds 1 to 5 minutes, so glor-ti's Endless is identical to before for
  its first 3:20 and differs only in its room until 5:58 (the old blind pair
  2, 5:20 to 6:05, was the same music within -29 dB: on the page it became
  the control, and a new pair 2 crosses the kit's fade). Over its first 25
  minutes the moves measured: the room (no change in loudness or brightness),
  the kit out for 4:53 (-2 LU, brightness 0.74), warmer (brightness 0.87),
  the bass's voice (-0.6 LU), the chords an octave down, the tune thinner.
- **A development key can last one unit:** brum-yei's goes A# aeolian (35 s),
  G# ionian (17.6 s), then the peak in F#: four key lines in 70 s, though
  `stay` asks 30 s. The peak's own key is the plan's, so the last development
  key before it can be short.

## The brain's judgment, plainly (before Mikey's ears)

- **Does an arc go somewhere and come home?** In its ideas, yes: a second
  tune in a new, higher key, a development, the first tune home and changed,
  a close that lands. In its energy, barely: the build is under the loop's own
  swing from bar to bar. If Mikey says "too little", the energy's range is
  the first number to open, before anything new is composed.
- **Is a return recognizable and still new?** By the measures (session 14:
  0.42 or more of A's notes kept, never a copy) and as heard in brum-yei and
  fei-no, yes; the variation that most risks "not recognized" is stretched.
- **Does a wander stay alive over twenty minutes?** It stays calm; whether it
  stays alive is the open question. Inside a hold it is the loop's own
  breathing, as before item 102; one move in three is hard to hear at all
  (the room). Nothing in it is abrupt.
- **A copy of a real form?** The full arc is sonata form's outline (two
  themes, the second in a related key, a development, a return, a coda) at
  fixed shares, in every full movement. That is a shape, not a quotation, as
  Mikey asked; its risk is sameness over hours, which the movements' kinds
  answer only in part.
- **Weakest:** the joins. They are the one place a listener might hear a
  mistake rather than a choice: holes where a pivot or "the tune alone" has
  nothing to play, a stutter through the development, and floor outs that
  take nothing. Second: the arc's energy, too gentle to feel.

## The listening set as it went out

The page (`docs/ALBUMS.md`, "Long Pieces Album") is session 15's 31 clips,
less its pair 2, plus two pairs the brain made: pair 2 is glor-ti Endless from
5:50 for 45 s (the kit's fade, now against before, level-matched), pair 3 the
old pair 2 as a control. Three of the clips carry a line of what was
measured (the floor out in brum-yei's peak, the silent pivot in fei-no's B,
the look-back's breaths).
