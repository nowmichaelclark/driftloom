# Long pieces, composed: the design that stands

Queue item 102. Written by hands session 11 (102a, 2026-11-12) after testing
the brief (`docs/sessions/long-pieces-brief.md`) against the code. Sessions
12 to 16 build from this file; where it and the brief differ, this wins.
The brief still holds the rules that can't bend and what Mikey asked for.

**Where things stand:** the plan is code (`core/src/generator/longform.rs`),
its numbers are data (`data/longform.txt`), and it is reported, not heard:
a section still plays as the loop's wandered pass. Read a piece with
`dlgen --map SEED REQUEST [--from N] [--count N]` (in `core/`: `cargo run
--release --example dlgen -- --map 7 bars=256,journey=arc`), or in Node
with `tools/gen-lib.mjs`'s `pieceMap`. `node tools/longform.mjs` runs the
corpus checks; `core/tests/longform.rs` holds the same in the gate.

## What a plan is

`Generator::form()` gives a piece's `Form`: a few numbers made from the
spec, the request's edits and the layers the loop has. `Form::at(section,
phrase)` gives a `Span`: the journey, the role, the theme and its
variation, the key, the tempo, the energy (0 to 1), a wander's state, and
what changed from the unit before (`changed`, a bit a thing). Nothing is
kept and nothing walks: a unit finds its arc or its epoch by a division and
draws on streams salted by that number, so a section costs the same at 1
and at 10^9 (measured natively: about 0.7 microseconds an arc's unit, 2.3
a wander's, at section 1, 10^9 and 2^52 alike). A loop of up to 32 bars, or with no length, has no plan.

**Units, set in time.** Spans are drawn in minutes and seconds and turned
into units at the loop's tempo. The unit is the section; where an arc would
have fewer than 12 sections, or a wander's mean hold fewer than 3, it is
the **phrase line** inside a section (the player's own: 4 bars, or 2, or
the section), so `Form::units_in(section)` is 1 or the section's phrases. A
last section cut short has fewer, and its last unit fewer bars.

**Never the slider.** The plan reads the spec alone. The wander slider's
amount stays how much a section breathes inside its role (today's pass
marks); sessions 12 and 13 must keep it so.

## The arc

Roles in order, as shares of the arc (`[longform arc]`): opening 7%, theme
15%, build 8%, second 15%, development 15%, peak 7%, release 9%, return 16%,
close 8%. Each arc draws where its peak ends (0.60 to 0.72 of the way) and
how high it is (0.85 to 1); every role has a unit at least.

| Role | Theme | Key | Energy (peak = 1) |
|---|---|---|---|
| opening | A, outline | home | 0.25 to 0.4 |
| theme | A, plain | home | 0.45 to 0.5 |
| build | A, plain or register or voice | home | 0.5 to 0.65 |
| second | B (one-theme pieces: A in the related key) | the arc's related key | 0.65 |
| development | fragments: A, then its answer (B; in a finale, an earlier arc's B) | through the related keys, each at least 30 s | 0.65 to 0.9 |
| peak | A, register | the arc's related key | 1 |
| release | B (or A), outline | the same | 0.85 to 0.55 |
| return | A with one variation (register, voice, stretched, reharmonized, answered) | home | 0.55 to 0.5 |
| close / breath | A outline, closing on the tonic / a rest | home | 0.4 to 0.15 / 0.3 to 0.2 |

- **How long:** an arc is 4 to 15 minutes (drawn per piece, by ratio,
  leaned longer by glacial, droning and the like). A finite piece holds a
  whole number of arcs of about that length; Endless plays arcs of exactly
  that length in turn, each ending in a breath.
- **The small form** (an arc under 4 minutes: most 64-bar pieces): opening,
  theme, lift, release, return, close. One theme, home key throughout.
- **Movements:** in a finite piece of several arcs every arc but the last
  ends in a breath; each has its own B (`Span::theme_seed`, from the seed
  and the arc's number); the last arc's development recalls the earlier
  arcs' Bs (`Theme::Recall`) and it alone closes.
- **One theme or two:** 70% of arc pieces have a B; the rest play A in the
  related key where B would be.
- **The close** takes the last units, 10 seconds at least: a last section
  cut short borrows the bars before it. The round after begins at the
  opening again (a round's plan is the same every time).

## The wander

Time is cut into **epochs**: each thing that may move takes one turn an
epoch, in an order drawn from the epoch's number, at the end of a **hold**
of drawn length (60 to 300 s; measured mean about 3 minutes). So exactly
one thing moves at each hold's end, and a unit's state is read from this
epoch and the one before, no further back.

- The things: a layer rests and later returns (two turns); a layer's voice
  (bass, chords or tune: a number over the pool's other voices); warmth;
  space (each -1 to 1 around the loop's own); the chords' register and
  their length (-1, 0, +1); the tune's density (0.55 to 1); rarely the key
  (15% of epochs visit a related key) and the theme (20% hand the tune to
  B).
- Voice, register and length are the loop's own every other epoch, so they
  always move and always come home. A turn that would move nothing is not
  taken.
- An epoch never starts on the thing the last one ended on. A layer's rest
  lasts one hold or two (it returns in the next turn, or the one after),
  and is never an epoch's last turn. A layer whose voice is changed
  does not rest.
- The unit after a move is `turning` (the glide's room); the rest of a hold
  is `holding`. A piece starts with everything the loop's own.
- A finite wander's last two epochs bring the key and the theme home, and
  its last units are the close. Its energy is level (about 0.5), lower
  while a layer rests or the tune is thin.

## Which journey

Drawn per piece on its own stream: arc and wander even, the arc's weight
x0.6 in Endless, leaned by the vibes (`[longform journey]`: still, serene,
meditative, glacial, droning, floating, hazy, misty, dreamy, restful toward
the wander; adventurous, triumphant, soaring, lively, playful, wondrous,
joyful, bouncy toward the arc). Measured: 49% arcs at finite lengths, about
37% in Endless; one vibe moves it to 29% (still) or 68% (adventurous). `journey=arc` or `journey=wander` in a request pins it
(`Request::with_journey`). The page's request and the codes don't carry it
yet (sessions 17, 18 and 22).

## Keys

The related set is found once, from the loop's key: the same mode a fifth
above and a fifth below, where at least 60% of the home notes stay
(wholeTone has none); the relative (the mode of the same notes with the
other third: a third away where one exists, else the nearest in
brightness); and the parallel, only to brighten a minor home, from the
loop's first profile's scales. No related mode is darker than 0.2
(phrygian, harmonicMinor) unless home is. Some loops have no related key;
they stay home.

## What was asked holds

A pinned key is the home key (the arc visits and returns; the wander never
leaves it). A pinned scale rules out the parallel. A layer asked on or off,
or re-rolled, never rests. A layer's voice asked is never changed, and the
tune's never handed on. A tone edited (warmth, space) is never moved. A
theme asked stays the tune. The tempo is the loop's own for the whole piece.
`core/tests/longform.rs` checks each.

## The joins (session 15)

Not code yet. The plan gives what a join needs: `changed` and the energy
either side. The starting set, by what changed: a key (a chord both keys
share, held across the line; before a return, the bass on the home fifth);
a theme alone (a bar of the tune alone, or a breath); a role with the
energy rising (a fill into the line) or falling (the floor out for the last
bar); a wander's move (the glide over the `turning` unit).

## The cost

The plan allocates nothing and adds no state to the player (it starts as
zeros still). The app's module grew 16 bytes (the request's journey); the
plan itself is not linked in until something plays it. What sessions 12 to
14 must watch is the brief's "Cost": a section is still one loop's making,
a theme-B section another seed's tune spread across blocks.

## Changed from the brief, and why

- **Epochs, not free holds.** "For each thing, the last hold at which it
  moved" has no closed form if any thing may be picked at any hold. An epoch
  gives every thing one turn, so the look back is one epoch.
- **Phrases only where needed**, one grain a piece, so a section is one
  role wherever sections are short enough.
- **The small form has a release** and one theme: without it the energy
  fell from the peak in a single line.
- **The parallel only brightens**, and no related mode is very dark: calm.
- **A peak is its own role**, in the arc's related key; the home key is
  kept for the return, so coming home is heard.
- **Rounds repeat their plan.** The loop's breathing still differs.

## Left open

- **Tempo:** held throughout. A slight easing into the close, or a slower
  middle movement, is a sketch to offer, not built.
- **How strong the arc sounds** (session 13) and what each variation does
  to the notes (session 14): the plan only names them.
- **A second, smaller peak** now and then: not drawn.
- **The tempo slider while playing:** the plan is made with the tempo the
  piece loads with; moving the slider changes how long the spans last.
- Every number in `data/longform.txt` is a first pick for Mikey's ears.

## For sessions 12 to 16

- Read the plan where a section is drawn: `Generator::form()` then
  `at(n, phrase)`. Keep `Form::of` free of the player's state.
- A change to the plan's draws moves the maps: say so, and keep
  `core/tests/longform.rs` and `tools/longform.mjs` passing (add what is
  played to the tool).
- Fill "Pieces to hear" as each journey is heard.

## Pieces to hear

For session 15's listening set: seed, request, where to listen, what for.

(empty: sessions 12 to 14 fill this)
