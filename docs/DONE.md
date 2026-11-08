# Done

One line for each PR that merged, newest last: the PR number, the version, the
commit preview URL, and what the ears should listen for. Add yours when your PR
merges (`docs/QUEUE.md`, standing rules). Entries before #287 are in
`docs/archive/done.md`.

- #287, the generator's math on the core's own and the generation goldens
  (item 77, v129), preview URL:
  <https://8c88ca90-driftloom.nowmichaelclark.workers.dev> (commit 2d1bfdf):
  pow, log, sin and the rest on the generator's paths now come from
  `js/dlmath.js`, the core's own math in JavaScript, so a loop's decisions are
  the same in every browser (Node and Chromium disagreed on 571 of 3,060
  cases before); `core/golden/generation.hashes` holds what the generator
  decides for 3,060 cases, and the Rust port is held to it. Nothing should
  sound different: in Chromium no decision moved in 11,060 loops (only last
  bits of a few values); in Node one loop (corpus seed 1, loop 894) moved its
  energy one 1/255 step, same notes. Listen for nothing. No album loop.
- #289, the generation data as plain text, read by both (item 78, v130),
  preview URL: <https://2200135e-driftloom.nowmichaelclark.workers.dev>
  (commit 223262b): profiles, vibes, moods, the voice table and the choice
  tables are now `data/*.txt` (format in `js/data.js`; `docs/VIBES.md` says
  how to edit a vibe), fetched at start in the app and read from disk in
  Node. Nothing should sound different: the generation goldens (3,060 cases,
  Node and Chromium), the core baseline and the balance lock are unchanged.
  Listen for nothing; if the app fails to start offline or on a first visit,
  that is this. No album loop.
- #291, the Rust generator, 1: the spec and the harmony (item 79, v131),
  preview URL: <https://3cf773f6-driftloom.nowmichaelclark.workers.dev>
  (commit 1aa9feb): `core/src/generator/` makes a loop's spec and what render
  plans before the layers (development, progression, long chords, entry
  schedules, gaps) from its seed, with the data compiled in; it equals the
  JavaScript on all 3,060 generation-golden cases on all five targets. The
  goldens gained a `plan` hash; nothing else moved, and the app's `.wasm` is
  the same bytes. Listen for nothing. No album loop.
- #292, the Rust generator, 2: the layers (item 80), preview URL:
  <https://eb02d0f6-driftloom.nowmichaelclark.workers.dev> (commit a1cf9e4):
  the generator in the core now makes every layer (bass and its pedal,
  melody and its development, drums and wayfare's grooves, texture) and the
  rest of render, the per-loop choices included; it equals the JavaScript on
  all 3,060 generation-golden cases on all five targets, and the balance
  lock's corpus drawn from it gives every loop the JavaScript's figures
  (`node tools/stats.mjs --check --core`). The app still plays the
  JavaScript, and its `.wasm` is the same bytes. Listen for nothing. No
  album loop.
- #294, the Rust generator, 3: the choices, the load and the wander (item
  81), preview URL: <https://6683c1c1-driftloom.nowmichaelclark.workers.dev>
  (commit fd59ae7): the generator in the core now makes every decision the
  JavaScript makes before scheduling: each choice's own draws (the arc, the
  tuned kick and its level by ear, subDuck, lateFloor, the floor drop's
  shapes, padSweep, the echo time, the bass's level), what the engine
  decides at load (dubStab, the organ, the wash, the space and the rest),
  and each wandered pass (drift, its floor drop, the kick swap). It equals
  the JavaScript on all 3,060 generation-golden cases on all five targets,
  the load and the eight passes included. The app still plays the
  JavaScript, and its `.wasm` is the same bytes. Listen for nothing. No
  album loop.
- #296, the player: a loop from a seed, in the core (item 82, v132), preview
  URL: <https://b48e5008-driftloom.nowmichaelclark.workers.dev> (commit
  2c5bd8b): the core now plays a loop from its seed and vibes on its own
  (`core/src/player/`): the app's sequencer ported, wandering pass after pass,
  with mute, tempo, volume, pause, a C header (`core/include/dlcore.h`) and
  `dlrender --seed`. The baseline's 40 loops, full and lite, mono and stereo,
  45 s, equal the app's engine in Chromium bit for bit when the page hands its
  notes over as the player does (the brain re-checked it on 1,104 renders
  the hands never ran, vibes and 300 s passes included, 2026-10-31). The app
  still plays the JavaScript: `js/dlcore.wasm` is the same bytes. Listen for
  nothing in the app. To hear the player, open the preview with
  `?engine=player`: it should sound exactly as the app does (Play, Next,
  Pause, a mute, the tempo). Tester: Diagnostics there, `late: 0`. No album
  loop.
- #298, ears on the core: sketch blocks and `tools/clips.mjs` (item 83, no
  app change), preview
  URL: <https://6f19a3ea-driftloom.nowmichaelclark.workers.dev> (commit
  2e148a0): `core/sketch/` takes one block file and one line of data, built only
  with the `sketch` feature, probed by the kit; `tools/clips.mjs` makes clips
  from the core's player with choices forced, a constant moved (`bass.LIFT_DB`),
  a setting pinned or the sketch on. The app still plays the JavaScript:
  `js/dlcore.wasm` is the same bytes. Listen for nothing in the app. For the
  ears sessions: try `docs/ears/sessions.md`'s new path with the demonstration
  veil (`"sketch": true` on a corpus loop, melody lowpassed); undertow #45
  with `lateFloor` forced brings the low end back at 0:29-0:30. A sketch voice
  is not built (`docs/archive/notes/83-sketch-voice.md`). No album loop.
- #300, the chords' octave from the progression (item 84, v133), preview URL:
  <https://09c939e1-driftloom.nowmichaelclark.workers.dev> (commit 583f5a0):
  the chords' octave is one decision per loop, read from the progression and
  the whole tune before form, rests and gaps, in both generators, so changing
  chord lengths, rests, tempo or the bass never moves it (0 flips in 2,000
  loops each; the old way flipped up to 44). 88 of 2,000 loops (4.4%) take
  another octave today, reisk-foa among them (corpus seed 1, loop 6). Listen
  for those loops' chords an octave away from before and nothing else; they
  go in the phase's blind album. Watch: keys and melody now sit closer than 5
  semitones in a few more loops (the balance lock's "not a choir, under 5",
  2.34% to 3.73%); listen there for chords crowding the tune.
- #302, the request: weighted vibes and tempo, pinned or a range (item 85a,
  v134), preview URL: <https://c42f2cad-driftloom.nowmichaelclark.workers.dev>
  (commit 0e7fece): one request read the same by both generators, the
  player, the C header and the app (`glacial=0.7,misty=0.3,bpm=70-85` as
  text); share code format 5 carries the tempo asked. Nothing asked changes
  nothing: the 3,060 old goldens, the baseline and the balance lock are
  unchanged, so listen for nothing in today's loops. The app has no tempo
  picker yet, so a listener can't ask for one; the ears can (`dlrender --seed
  N --request bpm=77`). For Mikey's ears when a picker exists: a range is
  drawn across all of it, the mix leans toward profiles whose band holds the
  tempo (a profile 6 bpm off keeps half its weight), and a gait profile with
  no gait at the tempo plays its nearest. Lite's refusals at a fast pin are
  shatter's own (item 85c).
- #303, the budget takes room from old notes, not new ones, on lite (item
  85c, v135), preview URL: <https://6073ae0c-driftloom.nowmichaelclark.workers.dev>
  (commit 9e61779): on lite, a note that doesn't fit lets its own layer's
  oldest notes from other steps go, each falling to nothing over 0.05 s as
  it starts (`Core::steal`), instead of being refused. Lite, 100 loops:
  melody refusals 1.4% to 0% (none lose a note), chords 62% to 36%; shatter
  asked for 180 bpm, melody 7.7% to 0%, chords 76% to 48%. Full quality and
  the JavaScript fallback unchanged. Listen on lite (a phone, or the
  Lite Room Album: <https://claude.ai/artifact/XwAWYWFdVi55KdGyccjjEW>, six
  loops before and after) for the tune keeping its notes and chords that
  now come in (toask-ha, hai-waim: their keys 3.6 LU up), and for long tails
  cut short on a slow loop (theith-yaing). For Mikey's ears: the fade's
  length, and whether full quality follows.
- #304, the request's pins (item 85b, v136), preview URL:
  <https://9e43ee20-driftloom.nowmichaelclark.workers.dev> (commit 204700f):
  a request can fix any of item 54's choices (`organ=on`, `arc=today`), the
  settings each draws by (`arc.period=240`, `kick.note=33`,
  `lateFloor.order=kitFirst`), the echo's time (`echo.beats=1`) and the
  space recipe's 32 (`space.room=0.8`), each held to what the loop could
  draw; share code format 6 carries them. Nothing pinned changes nothing:
  the 3,420 old goldens, the baseline and the balance lock are unchanged, so
  listen for nothing in today's loops. The app has no pin picker; the ears
  can (`tools/clips.mjs` and `dlrender --seed N --request organ=on`). For
  Mikey's ears: a choice pinned on plays only where its eligibility rule
  allows, and a pinned kick never swaps while the loop wanders. The theme is
  item 85d.
- #305, the theme (item 85d, v137), preview URL:
  <https://c7afce29-driftloom.nowmichaelclark.workers.dev> (commit 5ea90e3):
  `theme=SEED` in a request bends that seed's loop, keeping its profile mix,
  key and mode, metre and tune (the melody's figure and rhythmic cell) while
  the vibes, the tempo, the pins and the load's own seed bend the rest; a
  friend keeps its tune as the day changes. Nothing asked changes nothing
  (3,660 old goldens, the baseline and the balance lock unchanged), and the
  app has no theme picker, so listen for nothing in today's loops. The ears
  can: `dlrender --seed 4272732017 --request theme=4272732017` is noan-nou
  itself, and with `,cozy` or `,glacial,bpm=60` added the same tune in the
  same key, bent (`tools/clips.mjs` takes the same request). For Mikey's
  ears: a tempo asked plays in the theme's metre, and the theme's melody
  stays itself while its phrasing, register and voice follow the mood.
- #308, changing loops at a musical moment (item 86, v138), preview URL:
  <https://9a61df58-driftloom.nowmichaelclark.workers.dev> (commit 3c5ddd4):
  `Player::change` starts the next loop on the playing loop's next beat,
  bar, phrase or pass (or now), on one core, the old loop crossfading out
  over 1 s or ringing out; Broken Friends' two-core switch can go. The app
  doesn't call it yet, so listen for one thing in today's app: at a Next
  where either loop has the arc, no click and no one-block jump of the
  master's lowpass (both fixed in `blocks/arc.rs`; a loop alone renders as
  before). The ears can: `dlrender --changes 1000 --change 7 --wav-to DIR`
  (a bar change, the theme bent) and `--change 13` (a phrase), each writing
  the change, the new loop alone and the old one alone. For Mikey's ears:
  the crossfade's 1 s, and the new loop at its own level on the boundary
  (no fade in).
- #309, the budget takes room from old notes at full quality too (item 85e,
  v139), preview URL: <https://9ec9c54e-driftloom.nowmichaelclark.workers.dev>
  (commit 9a9638d): at full quality a note that doesn't fit lets its layer's
  old notes go, as lite has since 85c, the ones already releasing first (in
  both qualities); a drone's notes are never let go. Full, the baseline's 100
  loops: chords refused 19.5% -> 3.0% of notes, melody 13 -> 0, bass 4 -> 0;
  shatter at `bpm=180`, chords 39.8% -> 6.8% (per loop), melody 0.6% -> 0.
  Lite unchanged, bit for bit. Listen at full (the app on a laptop) for
  chords that now come in (their-roask, ri-houng: keys about 2.5 LU up) and
  for long tails let go on a slow loop (theith-yaing): the Full Room Album,
  <https://claude.ai/artifact/NmfNUM1hj6buNUHKZzG9of>, six loops before and
  after. The worst block at full rises 6% on the busiest loop (decided:
  `docs/archive/notes/85e-worst-block.md`); item 87 checks it on the Moto G Power.
- #310, a theme's rest is kept, choir or not (item 85f, v140), preview URL:
  <https://84b65ca7-driftloom.nowmichaelclark.workers.dev> (commit 2e2ff25):
  a theme whose own loop has no tune keeps its rest when it is bent, even on
  a seed that draws a choir; such a loop simply isn't a choir loop (its keys
  play the voice they drew, and it may develop as other loops of its length
  can). The brain's case, 2428270433 bent onto theme 3570153259 with
  `booming=0.31,cheeky=0.26,experimental` (dail-vu), had a vowel choir
  singing 87 notes; now no melody, its keys on pad. Nothing asked changes
  nothing: the corpus goldens, the baseline and the balance lock unchanged,
  and the app has no theme picker, so listen for nothing in today's loops.
  The ears can: `dlrender --seed 2428270433 --request
  theme=3570153259,booming=0.31,cheeky=0.26,experimental`. For Mikey's ears:
  whether a friend with no tune should still get the choir's keys (today it
  doesn't).
- #311, rendering ahead, for the browser (item 87, v141), preview URL:
  <https://21a0568f-driftloom.nowmichaelclark.workers.dev> (commit a575532):
  nothing to listen for (nothing that plays changed: the baseline, goldens
  and seeds unchanged). New: rendering ahead in chunks, the same samples as
  straight through (`Player::pass_frames`, `estimate`; the C header's
  `dl_player_pass_frames`, `dl_player_estimate`), an Emscripten build held
  to wasm32's goldens in CI, and the speed test, `speed.html` (Diagnostics,
  "Speed test"). Mikey's figures: the Moto G Power at full 27.9% of the
  deadline at worst (theith-yaing, stereo), lite 17.1%, rendering ahead lite
  stereo 10.5-16.4x real time; the Lenovo tablet at full 89-132%, lite
  47-76%, 2.0-3.1x. Decided: accepted, with promise 1's and 2's measures in
  `docs/CORE.md` (`docs/archive/notes/87-phone-figures.md`).
- #313, lighter on processor chosen by the app on slow devices (item 87b,
  v142), preview URL: <https://43bdf88c-driftloom.nowmichaelclark.workers.dev>
  (commit 5e1248d): nothing to listen for in a loop (no sound changed, in
  either quality). On a device that can't keep up at full (the busiest
  quarter-second over 20 s of playing past half its deadline), the app turns
  "lighter on processor" on by itself at the next pass: the music fades out
  over half a second and the loop starts again from its top, a breath later,
  on lite. Diagnostics: `lite: true, auto (busiest 63% at full)`. The
  listener's own choice is never changed. For Mikey: on the tablet, that
  breath at the switch (for his ears), and that the Moto stays on full.
- #315, a `now` change made later than the old loop decided ahead still
  lands on its grid (item 86b, v143), preview URL:
  <https://ea25aa34-driftloom.nowmichaelclark.workers.dev> (commit 7c0c48b):
  nothing to listen for in the app (the core's player only, which the app
  plays with `?engine=player`; the baseline, goldens and seeds unchanged). A
  change to a loop whose tuned kick takes longer to fit than the old loop
  had decided ahead started a fraction of a step off its beat; now it starts
  on the old loop's next step still to come. The brain's 300 changes on
  corpus 20261101 and corpus 1's 1,000 (full, lite, ring-out, 48 kHz): 0
  missed.
- #318, length up to 32,768 bars and Endless (item 88, v144), preview URL:
  <https://e77c1806-driftloom.nowmichaelclark.workers.dev> (commit 93164e2):
  the Length picker now asks the generator for real. Up to 32 bars the loop
  is that long (try 1, 3 and 7 bars: a loop of that length, played once,
  then the next); past 32, and Endless, the loop as drawn plays in sections,
  each wandering as a pass does. For his ears: 100 bars (a loop of 16 or 24
  ends on a section cut short at a bar line, four bars, then the next loop);
  1,024 bars and Endless with "Let the loop wander" on and off (off, a long
  piece is its loop repeated). Now Playing counts down to the piece's end,
  as before. Nothing changes with length on "any".
- #319, key and scale (item 89, v145), preview URL:
  <https://d5e8a6dc-driftloom.nowmichaelclark.workers.dev> (commit b44c693):
  nothing to listen for in the app yet (no picker; the key line shows what a
  request asks and cannot be tapped). The core's player and the generators
  take `key=D|G` and `scale=dorian|aeolian` (`?engine=player`, `dlrender
  --request`). For Mikey's ears when a picker exists: a mode a profile
  never draws, pinned onto it (the floor of 0.25 decides how often the
  profile's own modes still win among a set); the bass's fifth and pickups
  sit a fixed interval from a chord's root and fall outside a strange mode
  (whole tone, phrygian dominant) a little more than in the catalog (1.2%
  of the bass's notes against 0.85%). Nothing changes with key and mode on
  "any".
- #320, style (item 90, v146), preview URL:
  <https://0c1ec87a-driftloom.nowmichaelclark.workers.dev> (commit c9d9b61):
  nothing to listen for in the app yet (the style line shows what a request
  asks and cannot be tapped). The generators and the player take
  `style=tide|shrine` (`?engine=player`, `dlrender --request`). For Mikey's
  ears when a picker exists: a style pinned onto a tempo outside its band
  (the profile plays its nearest gait at the tempo asked), and a set whose
  profiles are far from the loop the seed would have made (which of them
  each piece of the blend lands on). Nothing changes with style on "any".
- #321, voices and layers (item 91, v147), preview URL:
  <https://915ad235-driftloom.nowmichaelclark.workers.dev> (commit 5e0af1e):
  nothing to listen for in the app yet (the voices line shows what a request
  asks and cannot be tapped). The generators and the player take
  `voice.melody=kalimba|marimba` and `layer.drums=off` (`?engine=player`,
  `dlrender --request`). For Mikey's ears when a picker exists: a voice
  pinned onto a profile that never draws it (the floor of 0.25 decides how
  often the profile's own voices still win in a set), a layer asked on (the
  drums come in late where the loop has the lateFloor choice), and the quiet
  and loud voices of `docs/notes/91-voice-levels.md` (stab, tubular, nylon,
  templebell, drops, wind, swell). Nothing changes with voices and layers on
  "any".
- #324, the bars picker sets the loop's length; the length slider sets when
  it ends (item 88b, v148), preview URL:
  <https://3937712d-driftloom.nowmichaelclark.workers.dev> (commit d5502ed):
  pick 33 or 1,024 bars and the card says so; the loop plays on round after
  round until the Sound panel's length slider hands over. Listen for: a
  1,024-bar loop with the slider at 5 minutes ends at the first phrase line
  past 5:00, its notes letting go over a second while the next loop starts
  on the line (`tools/ending.mjs` renders it); 14 bars with the slider off
  loops on; the picker now reads "Bars". For Mikey's ears: the fade's
  length, and a long loop's repeat with wander on being a fresh round.
- #325, "Let the loop wander" unrepeatable in the slider's upper range (item
  92, v149), preview URL:
  <https://f280ce1a-driftloom.nowmichaelclark.workers.dev> (commit 693d54f,
  the app as #325 left it): with the wander slider
  at 2.0 or above, play the same loop twice (Next, then back to it, or open
  its code twice): its passes from the second on wander differently each
  time; at 1.9 and below every play wanders the same. Diagnostics says
  `wander: 2.2, unrepeatable (salt N)`. For Mikey's ears: where the range
  starts (2.0, the top sixth), and whether a code shared from such a play
  should replay the sharer's wander (today it draws its own).
- #331, the player's module at 636 KB (item 93, v149, no app change), preview
  URL: <https://f7cc7064-driftloom.nowmichaelclark.workers.dev> (commit
  e5635c1): nothing should sound different (the baseline and the 80 seeds are
  bit-identical); `js/dlplayer.wasm` is 636,125 bytes, down from 1,938,596,
  and loads only with `?engine=player` (try it: Diagnostics, late and dropped
  notes). The rest is for the hands: a new player is all zero bytes
  (`tests/zeros.rs`), and one module can replace the two at 94b.
