# Long pieces, composed: the brief for sessions 11 to 16

Queue item 102. Written by the hands (Fable 5.2), 2026-11-10, from the code
as it stood after item 97, for the sessions that build it. **A starting
design, not a specification:** session 11 tests it against the code,
changes what doesn't hold, and writes the design that stands
(`docs/LONGFORM.md`). Where this and that file differ, that file wins.

## What Mikey asked for

"Make long pieces sound composed" (2026-11-10). Both journeys (2026-10-13):

- **An arc, like a symphony's:** themes that come back changed, builds and
  releases, related keys, a close.
- **Long, slow wandering:** one thing at a time moving, long held states,
  nothing abrupt.

Chosen per piece by the vibes and somewhat at random (his taste), and
pinnable by a request. Music computers are good at, never copies of real
music (2026-11-08): no borrowed tunes, forms as shapes, not quotations.
This is where his heart is; take the time it needs.

## What the code does today

- `core/src/generator/piece.rs`: a length up to 32 bars is the loop's own.
  Past that, and for Endless, the loop keeps the bars its profile drew and
  the piece is that loop played section after section; **section `n` is
  the loop's wandered pass `n`** (`Generator::section(n, amount)`,
  `wander::Pass`), drawn from the seed and the number alone. A finite
  piece comes round at its end (the player plays round after round and
  hands the host no end; the app's length slider decides when it stops).
  Endless numbers its sections on.
- `core/src/generator/wander.rs`: a pass never edits the loop. It marks
  the loop's own events (silent this pass, a hat opened, a chord opened or
  soft, pitches for the bass and the tune) and may add one ghost hit, so a
  pass costs a few bytes an event. Floor drops and their shapes, the
  pedal's passing tones and the tuned kick's swap live here.
- The request already has a **theme** (`theme=SEED`, item 85d): another
  seed's tune bent into this loop's style, and a theme that rests is kept
  resting (85f). That is the ready-made source of a second theme.
- The player (`core/src/player/`) emits a `Section` event at each
  section's start and changes things only at musical boundaries
  (`Moment`: beat, bar, phrase, pass, end).
- So today a long piece is one idea, wandering, forever at one level: no
  shape over minutes, no second idea, no ending.

## The rules that can't bend

1. **Promise 11.** Any section is reached straight from the seed and its
   number, without making the ones before it. Memory and cost are the same
   for 64 bars or Endless: a section's cost must not grow with its number.
   Nothing allocates after `Core::new`; the player still starts as zeros.
2. **Loops of up to 32 bars play exactly as today.** The balance lock
   holds, and every generation golden except the long-length cases is
   unchanged. The long-length cases move on purpose, said in each PR.
3. **Endless never ends.** Arcs in turn, or the wander.
4. **Changes at phrase lines,** never mid-phrase.
5. **Wholesome and calm.** Nothing that would bother people in a coffee
   shop: a build is a gentle swell, not a drop. No loudness swing past
   what the catalog already has (batch 6 turned down a 12.9 dB arc).

## A starting design

### One plan, read by number

The whole piece is described by a **plan** that is a pure function:

    plan(seed, request, n) -> what section n is

with no state carried from section to section. Everything below is built
so that function is cheap and closed-form: a section finds its place by
arithmetic and hashing (a seeded stream salted by the arc's or the
epoch's number), never by walking earlier sections. What a section is:

- its **journey** (arc or wander), the piece's;
- its **role** (an arc's: opening, theme, build, peak, release, return,
  close; a wander's: holding or turning);
- its **theme** (A, B, or a variation of one, and which variation);
- its **key** (the loop's own, or a related one) and its **tempo** (the
  loop's own to begin with: see "Left open");
- its **energy**, 0 to 1;
- what it **changes from the section before** (for the wander's rule and
  for the joins).

`dlgen --map SEED REQUEST` prints that, one line a section: the queue's
yardstick, and the tool every later session and the brain reads.

### The arc

An arc is a run of sections with a shape. A starting form, as roles over
its length (shares, not bar counts):

    opening (thin, theme A hinted) -> theme A -> build -> theme B, in a
    related key -> development (fragments of A and B, moving keys, the
    peak near two thirds) -> release -> return (A, changed, home key) ->
    close

- **How long:** an arc's length in sections is drawn once per piece from
  the seed and the piece's length, so that a finite piece holds a whole
  number of arcs and the last one closes; a short long piece (64 bars) is
  one small arc with fewer roles (opening, A, a lift, return, close).
  Endless plays arc after arc, each drawn from the seed and the arc's
  number (its second theme, its keys, where its peak falls), with a
  breath between them rather than a close. Equal arc lengths within a
  piece keep "which arc, and where in it" a division.
- **Energy** follows the role: a curve over the arc with one main peak
  (now and then two smaller ones). Energy is realized with what the core
  already has, cheaply, as marks and levels, never as new notes drawn at
  length: which layers play (the entry schedules and floor drops are the
  model), how many of a layer's notes sound (the wander's silent marks),
  velocities, the kit's pieces (hats in, the kick out), the chords'
  register and length, the melody an octave up or doubled at the peak,
  the arc's filter and the space opening with it. Level changes stay
  small; density and brightness carry the build.
- **Themes that return, changed.** Theme A is the loop's own tune. Theme B
  comes from a seed derived from the piece's (the request's theme
  machinery, 85d), so it is another tune in the same style. A return is
  never a copy: a small set of **variations**, each a pure function of
  the theme and a drawn number, such as: in the related key; in another
  register; handed to another voice of the profile; its rhythm stretched
  to twice the length over held chords; reharmonized (the same tune over
  the relative key's chords); answered (the first half kept, the second
  drawn anew, as `answerFigure` does); thinned to its outline. The
  development plays fragments (a phrase of A, then of B). The return is A
  with one variation, so it is known at once and still new.
- **Related keys.** From the loop's mode: the fifth above or below, the
  relative, and the parallel mode where the profile's scales allow it
  (cinder and shatter may brighten; decisions of 2026-10-11). A change of
  key happens only at a section line, through a join (below). The bass,
  the chords, the tune and any tuned kick move together.
- **The close** (finite pieces): the last section thins to the chords and
  the tune's last phrase, lands on the tonic, and rings out; the round
  after it (the player comes round) begins at the opening again.

### The wander

A slow walk through states with nothing abrupt. A starting model:

- The piece has a small set of **things that can move**: which layers
  play, a layer's voice (within the profile's pools), the tone (warmth,
  space), the chords' register and length, the tune's density, the key
  (to a related one, rarely), a theme handed over (A to B, rarely).
- Time is cut into **holds** of a drawn length (several sections each,
  long: minutes). At each hold's end **exactly one** thing moves, chosen
  by the hold's number (a seeded draw that never picks the thing that
  moved last), to a value drawn from the seed, the thing and the hold's
  number. So "at most one thing a section" holds by construction, and any
  section's whole state is found from its number: for each thing, the
  last hold at which it moved, by arithmetic.
- A move is heard as a glide where it can be (the tone, a layer fading in
  over a section) and at a phrase line where it can't (a voice, a key).
- Inside a hold the loop still wanders as today (the pass's marks), so a
  held state breathes.
- Endless is the natural home of the wander; a finite wander ends with
  the same close as the arc, softer.

### Which journey

Per piece: a draw from the seed on its own stream, leaned by the vibes
(`data/`'s way: toward x1.5 to x3, away x0.2 to x0.5, never 0). A first
table, for Mikey's ears: still, serene, meditative, glacial, droning,
floating, hazy, misty, dreamy, restful toward the wander; adventurous,
triumphant, soaring, lively, playful, wondrous, joyful, bouncy toward the
arc; even otherwise. A request pins it: `journey=arc`, `journey=wander`
(and the app's picker, later, in item 106 or after: not a setting).

### The joins

Where two sections meet and something changed (a theme, a key, the layers
that play), the join is written, not left to chance. A small set: a fill
into the line; the floor out for the last bar and back on the line; a
held chord across the line (a pivot the two keys share); a bar of the
tune alone; a breath of air. Chosen by what changes and by the energy
either side.

### Cost

- A section still costs one loop's making, as today. A theme-B section
  costs the same (another seed's tune bent into this loop). What must not
  happen: anything that walks earlier sections, or a variation that makes
  several loops to compare.
- The player holds two passes and the generator's lists today; the plan
  adds a few numbers. Check `tests/zeros.rs` and the baseline's costs.

## Yardsticks (the queue's, made concrete)

- `dlgen --map SEED REQUEST` for any length: each section's key, tempo,
  energy, theme and journey.
- Corpus checks (a tool or a Rust test, over a few hundred seeds at 64,
  256, 1,024 and 32,768 bars and Endless's first thousands of sections):
  an arc's themes return (A after B, changed, in the home key); its
  energy rises to a peak and falls; every finite piece closes on the
  tonic; a wander moves at most one thing a section, with holds of the
  drawn length; no key outside the related set; the same map whether a
  section is reached straight or through; **a section's cost flat** from
  section 1 to section 10^9 (timed).
- Promise 11's existing tests (`tests/length.rs`) still pass; loops of up
  to 32 bars hash as before; the balance lock holds.
- For Mikey: pieces at 64, 256 and 1,024 bars and Endless as album codes
  he plays in the app, each with excerpt times (the start, a quarter,
  half, three quarters, the close), so a piece is judged in minutes.

## Left open (decide by measuring, or mark for Mikey's ears)

- **Tempo.** A symphony's movements change tempo; a loop machine's listener
  may hate it. Start with the tempo held for the whole piece, and at most
  a slight easing into the close. A slower middle movement is an option to
  sketch and offer, not to ship unheard.
- **How strong the arc is.** The conservative end: a build someone working
  in a coffee shop notices only when it has passed.
- **How often theme B appears**, and whether some pieces (simple loops:
  Mikey's favorite in the Depth album was a simple 24-bar loop) have one
  theme only.
- **Whether a wander ever changes key.**
- **What the app shows** is item 106's; this item only makes the map.
