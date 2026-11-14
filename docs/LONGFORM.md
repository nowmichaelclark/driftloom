# Long pieces, composed: the design that stands

Queue item 102. Written by hands session 11 (102a, 2026-11-12) after testing
the brief (`docs/sessions/long-pieces-brief.md`) against the code. Sessions
12 to 16 build from this file; where it and the brief differ, this wins.
The brief still holds the rules that can't bend and what Mikey asked for.

**Where things stand:** the plan is code (`core/src/generator/longform.rs`),
its numbers are data (`data/longform.txt`). **The wander is heard** (session
12, `core/src/generator/heard.rs`, below), **an arc's energy and its
close** (session 13, "How an arc is heard"), **its themes, their
variations, its keys and its movements' kinds** (session 14, "How an arc's
themes are heard"), and **the joins and the endings the length slider
makes** (session 15, "The joins" and "Endings that land"). Session 16 tunes
it all from Mikey's notes. Read a piece with
`dlgen --map SEED REQUEST [--from N] [--count N]` (in `core/`: `cargo run
--release --example dlgen -- --map 7 bars=256,journey=arc`), or in Node
with `tools/gen-lib.mjs`'s `pieceMap`; `--movements` prints a line a movement.
`node tools/longform.mjs` runs the corpus checks and
`node tools/longform-themes.mjs` the themes' on what is played;
`core/tests/longform.rs` and `core/tests/themes.rs` hold the same in the gate.

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
| release | B (or A), outline; its last phrase may hold the bass on the home fifth | the same | 0.85 to 0.55 |
| return | A with one variation (register, voice, stretched, reharmonized, answered) | home | 0.55 to 0.5 |
| close / breath | A outline, closing on the tonic / A thinning to its thinnest (session 15) | home | 0.4 to 0.15 / 0.3 to 0.2 |

- **How long:** an arc is 4 to 15 minutes (drawn per piece, by ratio,
  leaned longer by glacial, droning and the like). A finite piece holds a
  whole number of arcs of about that length; Endless plays arcs of exactly
  that length in turn, each ending in a breath.
- **The small form** (an arc under 4 minutes: most 64-bar pieces): opening,
  theme, lift, release, return, close. One theme, home key throughout
  (session 14 brought its release home: it had the related key for a
  phrase).
- **Movements:** in a finite piece of several arcs every arc but the last
  ends in a breath; each has its own B (`Span::theme_seed`, from the seed
  and the arc's number); the last arc's development recalls the earlier
  movements' Bs (`Theme::Recall`, by number: one a section) and it alone
  closes.
- **Movements of different kinds** (Mikey, 2026-11-12; session 14,
  `longform::Kind`): a piece that would hold three arcs or more, and
  Endless, plays a repeating **cycle** drawn per piece: a **full** arc,
  then in a drawn order a **gentle** one (the full form, every role's
  energy between 0.32 and 0.6: the hats partway in, no doubled tune, no
  build to speak of), a **light** one (the small form) and, in pieces of
  four or more, an **interlude** (the wander's way for the span: up to
  three things go out one at a time, a hold apart, and come home in the
  same order; theme A, the home key). Each kind's length is a share of the
  full arc's (`kinds`: 1, 0.7, 0.4, 0.5). A finite piece is whole cycles
  and a last full arc, so the first and the last are full, the last alone
  recalls and closes, and no two neighbors are alike; three movements are
  full, gentle or light, full. A unit finds its movement by a division (a
  cycle, then a place in it). Pieces of one or two arcs are as they were;
  so is a piece too short to give every kind its roles. Mikey's 3,072 bars
  (seed 7, 1:31:42) were 17 alike; they are 17 in four kinds, 8:00, 4:03,
  5:36 and 3:13 long.
- **One theme or two:** 70% of arc pieces whose loop has a tune have a B
  (none where a theme was asked: it stays the tune); the rest play A in
  the related key where B would be. A second theme whose own loop's tune
  rests (queue item 85f: about 1 in 12) is never given one: that movement
  keeps theme A there, in the key, as a one-theme piece does. (Played as
  a rest of the tune it sat 4.4 LU under its neighbors in a tune-led
  loop.) In a wander, where a layer's rest is one of the moves, such a
  theme is the tune resting for the epoch.
- **The close** takes the last units, 10 seconds at least: a last section
  cut short borrows the bars before it. The round after begins at the
  opening again (a round's plan is the same every time).

### How an arc is heard (session 13)

`heard::Lift` and `Heard::hear_arc`: each unit's energy, smoothed over its
neighbors (a quarter each side, never across a round's end, so a role of
one unit is approached rather than stepped into), played as marks and
levels on the loop's own events and the mix, gliding over each unit from
where the one before ended. Nothing new is made but the landing's few
notes; a section costs one loop's making, as the wander's does.
**The loop as composed is the arc's fullest, near its peak**: the rest of
the arc is the loop held back a little, so the loudness climbs the whole
way, gently (`[longform energy]`).

| What | Heard as |
|---|---|
| the kit, the bass, the texture | their level in the mix: 0.85 of their own (the texture 0.7) at an opening's energy, their own near the peak, gone only as a close thins |
| the chords, the tune | a swell of under a decibel: 0.85 of their level at the lowest energy (they never rest, but the tune in a breath) |
| the kit's pieces | the shimmer (hats, shakers, jingles) comes in between 0.3 and 0.5; every hit up to a quarter softer below the loop's own |
| the tune's density | thinner under 0.45, up to 0.55 of its notes left out (the bar lines' kept longest, as the wander thins) |
| the chords' length | held longer under 0.42, where they have room (the wander's 1.5) |
| the tune's register | doubled an octave up, swelling from nothing at 0.6 to 0.3 of its velocity at 0.95, where the octave keeps it at 96 or under |
| brightness | the arc's filter (the loop's own slow swing, `blocks::arc`) takes the energy as its slow aim: the swing's centre lowered toward its darkest by up to 0.6 of its range under 0.7, its depth up to half shallower over it; a loop with no swing has the filter open, lowered the same way. One control, never a second filter |
| the space | opens with the brightness: up to 0.12 of the Sound panel's |
| a finite arc's close | thins to the chords and the tune (the energy's own); the tune's last phrase whole; then the **landing**: the round's last bars (4 s and a bar at least) play the loop's tonic chord (its own in root position, or the mode's triad, rolled where the loop plays its chords a note at a time) and the tune on the tonic, alone, held for half of them and ringing out. The voice budget gives the landing its whole ceiling |

The round after a close starts at its opening, gliding up from the close;
Endless plays arc after arc with a breath between (session 15: the tune
never rests, but thins across the breath, its thinnest, 0.55 of its notes
left out, at the breath's lowest energy, the rest thinned by its energy; it
rested before, and a breath one unit long, tui-doask's 50 s sections, stepped
2.7 LU from its neighbors; the tune held at its thinnest throughout stepped
2.04 LU between a breath's two units, its loop's tune strong there), and
never lands. **What was asked holds:** a layer asked on,
off or re-rolled is its own throughout; a code's warmth keeps the filter
the loop's own, its space the room's; an arc asked (`arc`, `arc.*`) keeps
its swing as asked. **At any slider setting**, as the wander. Every number
is for Mikey's ears. Measured (`node tools/longform.mjs`, 62 corpus pieces
against the loop as itself): the loudness rises to the peak and falls on
98%, the brightness on 87% (`docs/notes/102c-arc-brightness.md`); the peak
at most 2.3 LU over the opening; every last chord the tonic's, ringing out.

**Changed from the brief, measured:** the kit's shimmer comes in with the
energy and the kick and the backbeat stay, softer. The brief had it the
other way (hats in, the kick out): hats over a quiet kit made an opening
brighter, not thinner, and the kick and the backbeat carry so much of a
loop's loudness that an opening without them sat 6 to 8 LU under its peak.

### How an arc's themes are heard (session 14)

`heard::Lift::themed`, on the unit its energy made. **Theme B is another
seed's tune bent into this loop** by the request's theme machinery (85d):
that seed's own spec, its figure and its cell, and the loop's melody made
again around them, over the same chords, then finished as the loop's own is
(spelled over the chords, its slides, ornaments and harmony line; held out
of the bars the loop's tune is held out of; an octave away where its centre
would sit within five semitones of the chords'; a note that starts a
semitone over a chord tone takes that tone). The generator keeps two such
tunes, the section playing's and the next's, and makes one in three
stages, a block each, before the next pass is drawn (`Generator::prepare`):
the worst block that makes a stage or draws a pass 44 to 73 us natively,
under the loop's own worst audio block (78 to 331 us), full and lite. **A
section plays one tune beside the loop's own**: where its units name more
(a phrase-planned piece whose section holds B and an answer), the first
stands in for the others.

| What the map says | Heard as |
|---|---|
| theme B, a recall | the second tune in the loop's tune's voice, whole |
| a fragment | each phrase's first half, then a rest |
| a call and response (35% of the development's pairs) | theme A's fragment, then the other theme's in A's rests: one voice |
| in the related key | a fifth away, the whole loop moved (the wander's way); the relative or the parallel, every note along the mode (`key_map`, a table by pitch class), an octave over where a layer would leave its range, so the bass, the chords, the tune, its graces, the texture and a tuned kick stay in the key together |
| register, in a return (or a one-theme second at home) | the tune an octave up or down by the movement's number, the way that keeps it in 48 to 91 and five semitones from the chords |
| register, where the energy rises (a build, the lift, the peak) | the tune doubled an octave up, softly (session 13's double; a build's from its first unit): the tune itself moved an octave there sat under the theme before it, either way |
| voice | the tune in another voice of its pool (a tune with none moves an octave) |
| stretched | each phrase's first half at half speed, the chords held longer |
| reharmonized | the bass, the chords and the texture in the relative key, the tune where it was |
| answered | each phrase's first half kept, its second the answer's: the loop's own figure through `answerFigure`, made as a second tune |
| outline | in a release, at least half the tune's notes left out, the bar lines' kept longest (an opening and a close are as thin as their energy makes them) |
| the return prepared (half the movements) | the bass on the home key's fifth for the release's last phrase |

A theme stated (B at the second, A at the return) comes in whole on its
line; everything else glides as session 13's energy does. **The octave
rule** (item 84) is kept in every key: a unit whose moves would bring the
tune's centre within five semitones of the chords' moves the chords an
octave away, or the tune. **What was asked holds:** a theme asked has no B;
a tune's voice asked is never handed on; a pinned key is home, a pinned
scale rules out the parallel; the landing is the home key's.

Measured (`node tools/longform-themes.mjs`, 44 corpus pieces played): no
note outside its unit's key beyond the loop's own; 43 second themes, none
theme A again, 3 resting; every return keeps 0.42 or more of theme A's
notes and none is a copy; every variation played; the tune's time a
semitone over a chord tone 3.6% against 4.4% for theme A's own statements
(the parallel key's units 4.4%, a second tune 2.7%); the tune's centre five
semitones from the chords' wherever the loop keeps it. The arc's
brightness order, short of 9 in 10 since session 13, is 77% with the
returns varied (`docs/notes/102d-arc-brightness.md`). A theme-B section
costs what a theme-A one does (12 to 17 us, within 4%, the same at section
10^9); its tune made from nothing, a section ahead, 26 to 40 us more, about
a loop's own making. **A unit that plays other notes than the loop does in
those bars** (a fragment, half of each phrase a rest; another key; another
tune) may sit up to 2.9 LU from its neighbor where the energy alone steps
under 2: the arc's step check holds those to 3 LU and the loop's own notes
to 2. For Mikey's ears: every number, and whether a second theme that
rests should rather be given a tune. A second theme's notes are as loud
as the loop's own tune's (their mean velocity matched).

## The wander

Time is cut into **epochs**: each thing that may move takes one turn an
epoch, in an order drawn from the epoch's number, at the end of a **hold**
of drawn length (60 to 300 s; measured mean about 3 minutes). So exactly
one thing moves at each hold's end, and a unit's state is read from this
epoch and the one before, no further back. **The holds stay as drawn**
(session 12): they are fitted to the epoch around its mean, drawn in only
as far as keeps each within 60 to 300 s in whole units, where 102a scaled
them and one long draw beside short ones, or an epoch with turns skipped,
stretched a hold to 587 s. Over the corpus (241 seeds at five lengths,
88,522 stretches) the longest stretch with nothing moving is now 300 s
(the median 179 s); it was 587 s, with 2,092 past 330 s.

- The things: a layer rests and later returns (two turns); a layer's voice
  (bass, chords or tune: a number over the pool's other voices, only a
  layer whose pool has another and that does not sing); warmth; space; the
  chords' register and their length (-1, 0, +1); the tune's density; rarely
  the key (15% of epochs visit a key **a fifth away**: the whole loop moved,
  every chord the same; the relative and the parallel are the arc's) and
  the theme (20% of epochs hand the tune to the piece's second theme and
  bring it home in the next: on since session 14 made theme B).
- **Every thing goes out in the even epochs and comes home in the odd**
  (session 12; 102a gave each its own parity, so the first epoch moved
  half of them and its holds ran long): voice, register, length and
  density are the loop's own every other epoch; the warmth and the space
  take the other side of the loop's own each epoch, at least `apart` (0.4)
  of the way out, and the tune thins to 0.55 to 0.82 of its notes, so every
  turn is a step that is heard (a fresh draw, as 102a drew them, moved by
  less than a fifth of the range one turn in five). A turn that would move
  nothing is not taken.
- An epoch never starts on the thing the last one ended on. A layer's rest
  lasts one hold or two (it returns in the next turn, or the one after),
  and is never an epoch's last turn. A layer whose voice is changed
  does not rest.
- The unit after a move is `turning` (the glide's room); the rest of a hold
  is `holding`. A piece starts with everything the loop's own.
- A finite wander's last two epochs bring the key and the theme home, and
  its last units are the close. Its energy is level (about 0.5), lower
  while a layer rests or the tune is thin.

### How the wander is heard (session 12)

`heard::Heard`, on each pass (`wander::Pass::heard`), drawn a section ahead
with the pass: each unit of the section as the plan has it, as marks and
levels on the loop's own events and the mix. Nothing new is made, so a
section costs one loop's making (measured: 7 to 12 us a section natively,
the same at 1, 10^4 and 10^9; the block that draws it 28 to 39 us, under
the loop's own worst audio block of 49 to 187 us, full and lite).

| Thing | Heard as | When |
|---|---|---|
| a layer rests, returns | its channel's gain in the mix, beside the mute (`Synth::set_plan_level`); its notes left out once it is down | a glide |
| the warmth, the space | the lowpass and the room glide (`Synth::glide_tone`), `reach` 0.25 of the Sound panel's at most; the drive stays the loop's own | a glide |
| the tune's density | its highest-ranked notes left out (a rank a note, the bar lines' kept longest; at least one as soon as it thins) | a glide |
| a voice | the same notes, another voice of the layer's pool | the line |
| the chords' register | an octave, the other way where that leaves 36 to 96 | the line |
| the chords' length | 0.6 or 1.5 of their own (longer never into the next chord; the other way where a loop's chords can't) | the line |
| the key | every pitched layer, the tune's harmony and graces, a tuned kick: a fourth up or down, an octave over where a layer would leave its range | the line |
| the theme (session 14) | the tune's notes the second theme's (another seed's tune bent into the loop), in the same voice; a rest where that theme rests | the line |

A glide takes `glide` (24 s) over the units after its move; a unit starts
where the one before ended, so nothing jumps. The line is the unit's first
step: a phrase line, or a section's. Inside a hold the loop wanders as
before; **at any slider setting** (at 0 the player still draws a wander's
sections as passes, with no breathing and the loop's own kick). A tone the
listener sets while it plays holds from then on. **A finite wander closes
softly:** over its close every layer eases to 0.7 and the tune thins toward
its thinnest; the round after starts as the loop's own, gliding back up.
**A voice moved changes the voice budget's room:** a layer the budget held
back may come in (2 sections in 8 pieces played). For Mikey's ears: every
number here.

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

Where two units meet and something changes, the line is written, not left
to chance. The plan chooses each unit's join toward the unit after it
(`Span::join`, `Form::join_of`: the unit after by number, so a section's
last join is known a section ahead and a section reached straight joins as
one reached by playing) from what changes and the energy either side;
`heard` plays it over the unit's last bar (`Heard::joins`); the map and the
pass's text name it (`dlgen --map` writes "then fill"). The numbers are
`[longform joins]`.

| Join | When | Heard as |
|---|---|---|
| **pivot** | a key changes (and the loop has chords) | a chord both keys share (`longform::pivot`: a triad of the new key's mode on its tonic, its fourth, its fifth or the rest, every note in both keys; else its root and fifth), struck at the join's first step in the voice of the loop's first chord it plays, at that chord's velocity (else the chords' loudest: never a silent entry chord's 0, session 16), and held a beat past the line; the loop's chords out under it, the bass on its root |
| **prepared** | the return, prepared (half the movements) | the bass on the home fifth for the release's last phrase (session 14) |
| **fill** | a theme or how it is played changes and the energy rises by `step` (0.04) or more, or a role alone and twice that (a kit plays) | the kit's backbeat (else its shimmer) on each step of the last beat (every other in 6/8) where the loop has no hit of it, from 0.35 to 0.65 of its loudest, under the arc's kit marks |
| **floor** | the same, the energy falling from the peak or the lift (a kit or a bass plays); a key with no chords | the kit and the bass out for the join |
| **alone** | a theme changes, the energy level or falling after the height | every layer but the tune out for the join, where the tune sounds there as the unit plays it (whose half of the phrase, stretched, thinned: session 16); else a breath |
| **breath** | a theme changes where nothing else can join it; every wander's theme; a wander's line move (a voice, the chords' register or length) | the join's last half left out: every layer, or in a wander only the layer that moves |
| **glide** | what changes glides (a layer, the tone, a wander's density, the energy by a little) | nothing at the line: the unit after glides (sessions 12 and 13) |

- **How long:** a join is a bar, at most a sixteenth of its unit (a beat in
  a four-bar phrase, half a bar in eight bars, a bar in sixteen; measured: a
  bar of a four-bar unit took 1 LU off it and broke the loudness order on
  three 64-bar arcs of eight, half a bar still on five of thirty). A breath
  is half of that, a fill the last beat. The floor out comes once, from the
  peak or the lift: at every step down after it, it hollowed the release
  too, and two in a row is one too many.
- **What it takes out:** only notes that end by the line (a chord or a bass
  note that would ring across it plays: leaving out pads held into the
  next unit starved that unit by 2 LU), and only a layer heard in the unit
  before the join (two notes struck there at least: tui-doask's one bass
  note in a sparse unit's last bar was the unit's bass, and its floor out
  sat 6 LU under the loop). A join with nothing to take out is a glide.
- **What the loop can't play** is the next of its kind: a fill with no kit
  a glide, the tune alone where it is silent a breath, a pivot with no
  shared chord or no chords the floor out (or a breath).
- **None after a close:** the landing ends the round, and the round after
  starts from it. The yardstick holds: every line where a theme, a key or
  a line move changes has a join that is not a glide (`tools/longform.mjs`,
  `core/tests/joins.rs`).
- **Never a hole** (session 16): a join thins the mix, it never leaves it
  silent where the loop sounds. `tools/longform.mjs` reads every arc's join
  as played (`dl_gen_played`: the join the loop can play of the plan's) that
  strikes something or takes a note out, in 50 ms frames against the loop
  as itself: a hole is a window whose median frame is 25 dB under the loop
  and 12.5 dB or more under the beat before it (an arc's breath sits up to
  22 dB under; the two holes brain B3 found, 40 and 42 from 18 and 20).
  Where the beat before was already that far under, the quiet is the
  unit's, and is reported. `core/tests/joins.rs` holds every pivot struck
  above 0 and every tune alone over a tune that sounds there.
- **Never a hole, on any piece** (session 16b): a join can still be a hole
  where what it keeps carries little of the loop (ta-muin's tune alone, a
  stab's one note over a kit and a bass that are most of it; yi-yoar's
  floor out where the kit and the bass are the mix; a breath in a dry loop
  with nothing ringing). So a tune alone, a floor out or a breath is read,
  when its unit is made, from its notes (`Heard::under`): each note sounding
  over the join's second half, its voice's loudness over its life
  (`[longform loudness]`, measured by `tools/voice-loudness.mjs`), its
  velocity squared and its layer's level, and their echoes as the mix sends
  them; what the join keeps against the unit there without it (or the loop
  as itself, 10 dB less). Where it would leave the mix `[longform joins]
  hole` (12) dB under, the join falls back to a breath where that reads
  under half the line, else to a glide: nothing written. Fitted and checked
  against renders of 200 pieces (7,035 joins): 88 holes before, none after
  that stays 25 dB under for more than half a beat (mon-sha's three
  half-beat breaths in a four-bar loop remain, the gesture Mikey kept in
  moan-teil); 141 of the 7,035 changed, the 85 holes and 56 others (median
  15 dB under before). Over the corpus's maps 704 of 29,729 written joins
  changed: 558 breaths, 59 floors and 28 tunes alone to glides, 47 floors
  and 12 tunes alone to breaths. The wide run is the nightly's
  (`tools/longform-holes.mjs`: 140 Endless arcs, their first 30 minutes, and
  60 arcs of 1,024 bars, every join as played); `tools/longform.mjs` plays
  the three places session 05 found among its own, and
  `core/tests/joins.rs` holds them fallen back and every join played under
  the line by its notes.

## Endings that land (session 15)

When the length slider will stop a long piece (or Endless) before its own
close, the phrases before the stop play the close in place of the fade
(`Heard::stop_at`): from the energy there, thinning to the close's (0.15)
at home on theme A, the line into it a join like any other (a pivot home,
the floor out), the tune's last phrase whole, and the landing in its last
bars (the tonic chord and the tune on the tonic, alone, ringing out, as a
finite arc's). The close lasts the data's least (`[longform arc] close`, 10
s) in whole phrases. The player hears the stop with each section it draws
(`Player::hear_pass`, from the track's end) and again when the slider
moves (`set_track`); at the stop the next loop follows with the old one
ringing out, never faded. Where the landing would begin in steps already
played (the slider moved too late), the track ends as it always did. With
the slider off nothing changes; a stop at a finite piece's own end is its
own close. Measured (`tools/longform.mjs`, twenty pieces stopped part way,
Endless arcs and wanders, 256 and 1,024 bars): every one lands on the tonic,
alone, and rings. `dlrender --track bars=N` plays it, with the core's own
next loop after, as the app does. For Mikey's ears.

**Found on the way:** a landing could take its chord from one the loop
leaves silent (an entry's or a rest's) and strike it at velocity 0, so some
finite arcs since session 13 landed on the tune's tonic alone. The landing
now takes a chord the loop plays (else the chords' loudest velocity), and
the tune's last note that sounds.

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
- **An answer is a tune made, not marks** (session 14): "the second half
  drawn anew" needs notes that fit the chords, so the answer is the loop's
  melody made again around `answerFigure`'s figure, a stage a block, like
  theme B. Every other variation is marks on the notes.
- **One tune a section beside the loop's own** (session 14), so the
  generator keeps two and a block makes one stage; a finale's recalls are
  one movement's a section.
- **A gentle movement is the full form held between two energies**, not
  fewer layers by name: the energy already decides which layers play.
- **An interlude is its own small wander** (out and home inside the span),
  not a slice of the piece-long wander's epochs, which run 20 minutes and
  more: a movement must begin and end as the loop's own.

## Left open

- **Tempo:** held throughout. A slight easing into the close, or a slower
  middle movement, is a sketch to offer, not built.
- **How strong the arc sounds:** session 13 took the conservative end (the
  peak about 1 to 2 LU over the opening by ear; its numbers are first
  picks).
- **The parallel key** clashes a little more than the loop (4.5% of the
  tune's time against 3.8 to 4.6%): a minor tune's fourth over a major
  third. Kept, as the brief asks; one line of `related` to rule out.
- **A second, smaller peak** now and then: not drawn.
- **The tempo slider while playing:** the plan is made with the tempo the
  piece loads with; moving the slider changes how long the spans last.
- Every number in `data/longform.txt` is a first pick for Mikey's ears.

## For sessions 12 to 16

- Read the plan where a section is drawn: `Generator::form()` then
  `at(n, phrase)`. Keep `Form::of` free of the player's state.
- Session 16 tunes from Mikey's notes: every number in `data/longform.txt`
  (`[longform joins]` too) is a first pick. The joins are the plan's
  (`Form::join_of`) and heard (`Heard::joins`); a slider's close is
  `Heard::stop_at`.
- `node tools/longform.mjs` checks what is played too
  (`tools/longform-played.mjs`; `--plan` for the plan alone).
- A change to the plan's draws moves the maps: say so, and keep
  `core/tests/longform.rs` and `tools/longform.mjs` passing (add what is
  played to the tool).
- Fill "Pieces to hear" as each journey is heard.

## Pieces to hear

For session 15's listening set: seed, request, where to listen, what for.
The set itself (session 15: 31 clips, 23 minutes, on the throwaway branch
`long-clips`, made by `tools/longform-clips.mjs`) reads every place from the
piece's map; where a time below differs from its index, the index is
current (the plan moved under sessions 13 and 14).

**The wander** (session 12). Each at bars=256 and Endless, `journey=wander`;
times from the piece's start (the 256-bar piece's moves are its first, then
its close). Listen for: each move alone, slow, never a jump; the rest
between moves is the loop as it always breathed.

1. **Seed 7, glor-ti** (134 bpm, F# mixolydian; 256 bars is 7:38). 3:20 the
   room glides drier; 5:58 the kit fades out over 24 s (256 bars: it stays
   out into the close, which eases down). Endless goes on: 10:51 the kit
   fades back; 15:31 the tone warms; 19:13 the bass changes voice (its last
   half beat before it a breath, session 15); 21:58 the chords move an
   octave; 24:35 the tune thins (times as of v164).
2. **Seed 1086007177, mu-meth** (138 bpm, F major pentatonic; 256 bars is
   5:33). 256 bars: 1:36 the tune thins; 5:00 the tune fades out, then the
   close. Endless: 1:02 the tone warms; 4:00 the chords up an octave; 5:54
   the tune fades out; 10:00 it fades back; **11:07 the key: the whole loop
   to C** (the one key move of the three); 15:13 the chords shorter; 16:26
   the room; 20:31 the chords change voice.
3. **Seed 2541830973, so-rou** (56 bpm, F# ionian, 34-s sections; 256 bars
   is 18:17). 2:17 the room; 4:00 the chords up an octave; 6:52 the texture
   fades out; 10:52 it comes back; 13:42 a layer changes voice; 17:06 the tone moves,
   then (256 bars) the close. Endless: 21:42 the chords' lengths; 24:36 the
   texture rests again.

**The arc** (sessions 13 and 14). Each `journey=arc`, at the length given;
times from the piece's start. Listen for the energy (session 13): an
opening thin and a little dark, the build noticed only once it has passed,
the release, then the close thinning to the chords and the tune and the
landing on the tonic, alone, ringing out. And the ideas (session 14): the
second theme arriving as another tune in the same voice, in a new key; the
development trading fragments; theme A coming home whole, known at once
and changed; the bars before it where the bass holds one note.

1. **Seed 2728346932, tha-vol** (131 bpm, A# lydian; 64 bars is 1:27, the
   small form: one theme, at home). The theme 0:10; the lift 0:32, the
   tune doubled an octave up as it rises; the release 0:49, the bass holding F (the home fifth)
   until the return 0:54, **A answered** (each phrase's second half the
   answer's); the close 1:16; the landing 1:23; the opening again 1:27.
2. **Seed 1151594917, brum-yei** (109 bpm, C# ionian; 256 bars is 9:23, two
   arcs as movements). **B enters 1:10, in F#** (a busier tune, the loop a
   fourth up); the development 1:45, a call and response in A# aeolian (the
   relative: A's fragment, B's in its rests), then A's fragment alone in G#
   at 2:20; the peak 2:38; the release 2:56 (B's outline, the bass holding
   G# from 3:05); **A returns 3:13, register** (an octave away, home); a
   breath 4:06. The second arc: its own B is a theme that rests (85f), so
   at 6:09 it states theme A in G#; the development 6:45 **recalls the
   first arc's B** as a call and response; the peak 7:37;
   **A returns 8:13, answered**; the close 9:06; the landing 9:19.
3. **Seed 1278568317, sheim-gling** (98 bpm, G kumoi; 1,024 bars is 31:20,
   two arcs of about 15 minutes). B enters 4:53 in D; the development 7:20;
   the peak 9:47; the release 10:53; **A returns 12:14, voice** (a bell for
   the keys); a breath 14:34. The second arc: B 20:04; the development
   22:17 recalls the first arc's B; the peak 24:29; the release 25:35; **A
   returns 27:11, answered**; the close 30:00; the landing 31:15.
4. **Seed 2060199680, moan-teil** (118 bpm, G mixolydian; 1,024 bars is
   34:42, **five movements of four kinds**). The full arc 0:00: B 2:42 in
   D, the development 3:55, the peak 5:17, **A returns 6:54, answered**. The
   gentle one 9:37 (no build to speak of): B 11:39, a call and response
   12:44, **A returns 14:46, register**. The light one 16:16 (the small
   form): the lift 17:37, **A returns 18:50, stretched** (each phrase's
   first half at half speed over held chords). The interlude 20:12: the
   tune thins at 21:49 and fills again at 23:27, nothing else moves. The
   finale 24:56: its own B 27:47 in C; **the look back 29:17 to 30:38**,
   fragments of the first movement's B (29:25, 29:57, 30:30) and the gentle
   one's (29:33, 30:06, as calls and responses) through D dorian (the
   relative), D and C; the peak 30:38; **A returns 32:16, reharmonized**
   (the tune as it was, the chords and the bass in D dorian); the close
   33:53, landing in its last bars.
5. **Seed 2158762586, fei-no** (88 bpm, A major pentatonic; 256 bars is
   8:43, one arc). **B enters 2:43 in F# minor pentatonic** (the relative:
   every note two steps of the mode down); the development 4:21 in E; the
   peak 5:27; the release 6:00; **A returns 7:05, stretched**; the close
   8:10, landing in its last bars.

**The joins and the endings** (session 15). Listen for each line written,
none clicking: brum-yei at 256 bars (`journey=arc`) has a fill into its
theme (0:17), a pivot chord into B (1:10), the floor out after the peak
(2:56), the return prepared (3:13) and the breath between its movements
(4:06); tha-vol at 64 bars has every arc join but the pivot in 1:27; mu-meth
in Endless (`journey=wander`) has the wander's key move over a pivot
(11:49). A close the length slider makes: fei-no at 256 bars with the
slider at 120 bars (it lands at 4:05), glor-ti in Endless at 100 bars
(2:59): `dlrender --seed SEED --request REQUEST --track bars=N`.
