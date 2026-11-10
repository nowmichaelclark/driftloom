# Long pieces, composed: the brief for sessions 11 to 16

**Changed by brain B1 (2026-11-11), after reading the code:** two rules
added (the wander slider and the pins, rules 6 and 7), "Time, not
sections", "Endings that land", "Movements" and three musical ideas (the
return prepared on the fifth, a call and response, the arc block as one
control), the cost on the audio thread, and the listening set as rendered
excerpts with a before-and-after pair. Why: a section is the wander's pass
at the slider's amount (`Generator::section(n, amount)`), so at 0 a long
piece would stop moving; a section is the loop's own 4 to 32 bars, 8
seconds to 2.5 minutes, so a plan in sections alone breathes at the wrong
speed; the length slider can end a piece before its close; and until
sessions 17 and 18 the app can neither jump inside a piece nor take a
journey, so Mikey can't hear "three quarters in" there.

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
6. **The journey is the piece; the wander slider is its breathing.** Today
   section `n` is the pass drawn at the slider's amount, so with the slider
   at 0 a long piece is the loop over and over. The plan (journey, roles,
   themes, keys, energy, the wander's holds and moves) is drawn from the
   seed and the request alone and plays at any slider setting; the slider
   sets only how much each section breathes inside its role (today's pass
   marks). Above 2.0 (item 92's unrepeatable range) the salt varies that
   breathing, never the plan, so a code's piece stays its piece (for
   Mikey's ears).
7. **What was asked is never moved.** A pinned voice, layer, tempo, key,
   scale or style, a muted layer, and a code's edits (layers re-rolled, the
   tone) hold for the whole piece. A pinned key is the home key: an arc may
   visit related keys and comes home; a pinned scale rules out the parallel
   mode. A wander's move never picks a pinned thing.

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

### Time, not sections

A section is the loop's own bars: 4 bars at 120 bpm is 8 seconds, 32 at 50
is 2.5 minutes. So the plan's spans are set in **time** and turned into
sections at the loop's tempo: an arc of about 4 to 15 minutes (leaned by the
vibes; a symphony's movement is about 10), a wander's hold of about 1 to 5
minutes, roles as shares of the arc. Where a piece has fewer sections than
its form needs (64 bars of a 32-bar loop is two sections), the plan's unit
is the **phrase line** inside a section, still found by arithmetic from the
section's number and the phrase's. A last section cut short (a length the
loop doesn't divide) still plays the close: the close is a role, fitted to
the bars that are left.

### Endings that land

The length slider decides when a track stops (Mikey, 2026-11-03), and it
can stop a long piece before its close; today that is a cut at a phrase
line with a short fade. The player knows the bar it will stop at (the
spec's `playBars` or `playFor`), so the plan takes it: the phrase before
that bar plays the close (thinned, on the tonic, ringing out) instead of a
fade. The same for Endless with the slider set. With the slider off nothing
changes: rounds come round. Session 15 builds it; for Mikey's ears.

### Movements (finite pieces of several arcs)

Mikey's 3,072 bars came from Beethoven's Ninth: one work in movements whose
finale looks back at what came before. So in a finite piece that holds
several arcs, the arcs are movements of one piece: they share theme A (the
loop's tune), each brings its own second theme (arc `k`'s B from the seed
and `k`), and the last arc, before its close, recalls fragments of earlier
arcs' B themes, reached by number in closed form. A shape, not a quotation
(Mikey, 2026-11-08). Endless's arcs stay free of this: nothing ends.

### Three ideas for the arc

- **The return prepared:** in the bars before theme A comes home, the bass
  holds the fifth of the home key (the chords over it), so the return is
  felt coming. One join among the others, chosen for returns.
- **A call and response:** in the development, B's fragment answers A's in
  A's rests (one voice, so no second melody's cost), rather than both
  stacked.
- **The arc block as one control:** the loop's slow muffled swing (the arc,
  which Mikey likes) and the piece's energy both move brightness. Make the
  energy curve the arc block's slow aim (its centre and depth) rather than a
  second filter beside it, so the two never fight or cancel.

### Cost

- A section still costs one loop's making, as today. A theme-B section
  costs the same (another seed's tune bent into this loop). What must not
  happen: anything that walks earlier sections, or a variation that makes
  several loops to compare.
- The player holds two passes and the generator's lists today; the plan
  adds a few numbers. Check `tests/zeros.rs` and the baseline's costs.
- **On the audio thread.** The player makes the next pass in the blocks
  before it, in stages (`Generator::step`, a load's way). A theme-B section
  makes another seed's tune, and a variation more work: spread them the
  same way, a section ahead, so no block at a section line costs more than
  the loop's own worst audio block. Mikey's tablet has about 1.8 times
  headroom on lite before item 100's too-slow line (`js/quality.js`), and
  it lost notes at new loops before phase 3: measure a section line on
  `tools/autolite.mjs`'s slow case too.

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
- For Mikey (session 15, then brain B3): **rendered excerpts**, since the
  app can't jump inside a piece or take a journey until sessions 17 and
  18. About eight pieces (64, 256 and 1,024 bars and Endless; arcs and
  wanders; several styles), each as excerpts of about 40 s where something
  happens (the start, B's entry, the peak, the return, the close; a
  wander's moves), plus one 64-bar piece whole; under about 30 minutes in
  all. And **a blind pair or two:** the same seed and length before item
  102 (today's wandered sections) and after, so his ears answer the real
  question, whether composed beats today's. Clips go on a throwaway branch
  (the Ears' way: only the clips, deleted after his verdicts); brain B3
  builds the page.

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
