# The queue

Written by the brain for Claude Code (the hands). Work top to bottom. Only
what is live is here: the status, the standing rules, the open items, the
known issues and the merge policy. Finished items, the old notes and the
Done list are in `docs/archive/` and `docs/DONE.md`; `docs/README.md` is the
map of all the docs. Mikey's ears come later, in listening albums the brain
builds from the Done list.

**Status, 2026-10-30:** `main` is at v128. Done: items 0-36 and 38-47 (13
stopped at its gate by design), the core's phase 1 (42-46c), the whole sound
batch (items 54-73, #245-#272, checked by the brain's Session 26), item 76
(the cold start, #277), and the last changes before the port: 76b (the vibes
to 88, #280), 76c (the bass level, #282), 76d (lateFloor's second arrival,
#283) and 74 (clippy pedantic at zero, held by CI, #284). **Next: phase 2,
from item 77.** Item 75 is low priority. 37 stays on hold. A new Claude Code
session starts here: read the standing rules and the merge policy, then take
the next item.

**Two tracks:** app work (fixes, redesigns, vibe tuning, new controls) goes
on alongside the core track, as separate PRs; app-track sessions stay out of
`core/`, and core-track items don't wait for the app (`docs/BRAIN.md`, "Two
tracks").

## Standing rules, for every item

- **One PR per item** (split an item into a/b if that is cleaner). Start
  each from the latest `main`, take the next version number, and bump
  `BUILD` and `CACHE` together.
- **Merging:** see "Merge policy" at the end.
- **Measured, not guessed.** Every new voice, texture or kit: level
  calibrated to its layer's median at 0.4 s notes (`measure.mjs --voice`),
  `VOICE_COST` measured by render timing, and the tone probe's 2-5 kHz
  share at or below the flute's. That last one is the coffee-shop test:
  Mikey's north star is that nothing should bother people in a coffee
  shop. Every new profile: its `level` solved so its loops land on the
  catalog median (`measure.mjs --profile <id>`).
- **Refusals** on full and lite for the loops the item touches: melody at
  full quality stays near zero, and lite no worse than the catalog.
- **The core baseline (item 32):** run `node tools/baseline.mjs`. It renders
  a fixed set on the core (40 loops full and lite, every voice, the
  refusals, the costs) and fails on any difference from
  `test/core-baseline.json`. A change that is not meant to change the sound
  must leave it passing; one that is runs `node tools/baseline.mjs
  --update` in the same PR and says in the body what moved and why.
  Nulling against JavaScript (`measure.mjs --null`) is no longer the gate
  for the core; the JavaScript synth is the fallback a phone without the
  core hears and keeps passing its own checks (`--engine js` with
  `--refusals`, `--clicks`, `--endings`) when its code is touched.
- **The balance lock:** run `stats.mjs --check`. If a locked figure moves,
  report which and by how much, re-baseline at `--n 10000`, and say so.
- **Existing loops:** say exactly what changes for specs that already
  exist. Anything not named in the item must render identically.
- **Tests** three times.
- **Taste:** where the item leaves a musical choice open, take the more
  conservative option and write it down in the PR, marked "for Mikey's
  ears". Never guess silently.
- **American English** in new comments, docs and PR text (Mikey,
  2026-09-28). Don't rename identifiers or spec fields for spelling:
  share codes and history depend on them.
- **Stop and leave a note**, rather than push on, if a check can't be met
  or the item turns out to need a decision. Each note is its own file,
  `docs/notes/<item>-<short-name>.md` (the item number leads, so notes
  sort with their items; `docs/notes/README.md` says what goes in one).
  Say so in the PR. Notes before 2026-10-26 are in
  `docs/archive/notes.md`.
- **When a PR is merged,** add one line to `docs/DONE.md`: the PR number,
  the commit preview URL, and what the ears should listen for. (Lines
  before #277 are in `docs/archive/done.md`.)
- **Items that touch `core/`** also follow `core/AGENTS.md` (the core's
  standing rules, and the rules for sound items).
- **Finished items** are in `docs/archive/queue-items.md`, by number: an
  older item cited as "item 46" is the text there.

## 75. The reverb, rate-true for steady tones (low priority, after the batch)

**Hands:** Opus 5.5, high. From item 62 (brain, 2026-10-20).

**Why:** the reverb's combs (feedback 0.8) catch a held tone differently at
each sample rate: a held sine into the chords channel spreads 1.1 dB across
rates, while the channel itself is the same at every rate. Matching the comb
loops' lengths in time alone didn't fix it (the loop's lowpass and the delay's
interpolation matter too). The chapel organ, a held instrument with a room
send, is the first sound it shows in: golden 16-grove (5 s, the organ) reads
+0.29 dB at 44.1 kHz and +0.34 at 96 kHz against 48 kHz, so
`core/golden/rates.txt` marks it `steady` and `tools/rates.mjs` holds it to
0.4 dB, every other list to 0.2.

**Aim:** 48 kHz bit-identical (goldens and baseline unchanged), and every rate
list back inside 0.2 dB, so the 0.4 dB tolerance (and the `steady` line) can be
removed. If 48 kHz must move, stop and leave a note for a decision.

## Phase 2: the generator in Rust (items 77-92)

Written by the brain, Session 26 (2026-10-26). They replace the placeholders
48-52, which are folded in below (48 → 85, 49 → 88, 50-52 → 89-91); 53 stays
in phase 3. **Read first:** `docs/CORE.md` (promise 11, the request; promise
10, Broken Friends; phase 2) and the brain's log entry "what Broken Friends
needs from the core" (`docs/archive/brain-log.md`, 2026-10-25).

**What the phase is for.** Broken Friends waits on it: today the game can
only play golden note lists. What the game asks of the core, in its order,
sets the order here:
1. **a loop from a seed** (each friend's theme), with no JavaScript at all
   (items 77-82);
2. **steering** by friend, mood, time of day and camp events, which are
   promise 11's requests (items 85-86);
3. **a cheap change of loop** at a musical moment, with a crossfade, in place
   of today's second core (item 86);
4. **rendering ahead for the browser**, where Godot's web build has no audio
   thread (item 87).
Then the rest of the request (length, key and scale, style, voices: 88-91)
and the wander's upper range (92). Two fixes ride in the phase, as the
charter says: the chords' octave (84) and the wander (92).

**How the port is proven: note for note** (Mikey, 2026-10-26). The generator draws only from
its own seeded `Rng` and does plain arithmetic, apart from about twenty
calls to `Math.pow`, `log`, `sin` and the like, whose last bits are each
browser's own. Item 77 moves those onto the core's math (one JavaScript
mirror of it), and records **generation goldens**: for a fixed set of seeds,
vibes and passes, a hash of everything the JavaScript decides (the spec,
every layer's events, the choices, each wandered pass, every setting the
engine sends at load). The Rust port is then held to those hashes, step by
step, the way the synth's port was held to Chromium's samples: equal, or a
named, explained difference. That is stricter than the charter's gate (the
balance lock and `stats.mjs` within tolerances, and a blind album), which
still applies at the end.

**The sequencer joins phase 2** (Mikey, 2026-10-26; `docs/CORE.md`, phase 2): the game's gate needs
the loop played from a seed, so phase 2 also takes the **sequencer**, what
`js/engine.js` and `js/synth.js` do between the generator's events and the
core's notes (item 82). Phase 3 still moves the app onto it and retires the
JavaScript; it just has less to build.

**Rules for the phase** (on top of the standing rules and `core/AGENTS.md`):
- **One source of truth at a time.** Until item 82's player is in the app
  (phase 3), the JavaScript generator plays; the Rust one is proven against
  it. Generator and vibe changes made during the port go in **both**, or wait
  (`docs/BRAIN.md`, "Two tracks"); after item 78 the data (profiles, vibes,
  voices) is one file both read, so tuning a vibe never waits.
- **When nothing is picked, nothing changes** (48's rule, kept): a loop with
  nothing pinned renders as before; a pick may only change loops that asked
  for it, and chance draws from the seed with no extra draws that shift the
  rest.
- **Requests bend the music, never break it** (promise 11); "can't do that"
  only when truly impossible, in a short plain ASCII line.
- **Picks shape loops to come, never the one playing,** and hold through
  re-rolls and wandering; in an album the album decides.
- **Measure that a pick is honored in the render**, not only in the spec
  (tempo from onsets, bars from the cycle, scale from pitch classes, voices
  from what sounds).
- **The core's rules hold for the generator too:** no allocation after
  `Core::new` (the generator's working memory sized there, and said in
  bytes), no panic, no `dyn` on the hot path, plain ASCII out, `no_std`,
  bit-identical on every target (goldens for generation on all five, as for
  audio), pedantic clippy clean in every file touched (CI holds it at zero since item 74).
- **Spread the work:** generating a loop never spikes one audio block
  (promise 1); say the worst block's cost on the slowest target.
- **Taste:** the conservative option, "for Mikey's ears", as always.

Hands, by item, as the brain suggests: Opus 5.5, high, for 77, 79-82, 85-88
and 92; Sonnet 5.5, high, for 74, 78, 83, 84, 89-91.

## Before 77

Item 74 is done (#284): pedantic clippy is zero in `core/` and CI holds it
there, so the generator lands clean. Item 75 (the reverb, rate-true) stays
low priority, for any session with usage to spare after 82. **Vibes and
generator changes from here on follow the phase's first rule** (one source
of truth at a time, above): after item 78 the data is one file both
generators read.

## 77. Generation made exact, and its goldens (JavaScript, no sound change)

**Why:** the port is proven note for note (above), which needs the
JavaScript's decisions to be reproducible anywhere.

**What:**
- `js/dlmath.js`: the core's math (`core/src/math.rs`: pow, exp, log,
  log2, log10, sin, cos, sqrt where used) mirrored in JavaScript, checked
  bit for bit against the wasm's own over a million inputs per function
  (export them from the core for the test if they aren't). The generator,
  `js/choices.js`, `js/theory.js`, `js/characters.js`, `js/picks.js`,
  `js/vibes.js` and the engine's load-time decisions (`stabOf`, `organOf`,
  `washOf`, `spaceOf`, the echo time, the kick's level fit) use it; nothing
  on those paths calls `Math.pow` and the rest. Say which loops move (a
  last bit flipping a comparison); expected: a handful in 10,000. Core
  baseline and balance lock updated on purpose if anything moves.
- **The generation goldens:** `tools/gen-golden.mjs` writes, for each case,
  one line: the seed, the vibes, and FNV-1a 64 hashes of (a) the spec, (b)
  each layer's events after `render`, (c) the choices and every per-loop
  draw the engine makes at load (arc, kick, subDuck, lateFloor, padSweep,
  echo, space, wash, stab, organ), (d) each of the first eight wandered
  passes' events (drift), in a canonical text form documented in the file.
  Cases: 2,000 corpus seeds with no vibes, 1,000 with 1, 3 or 5 vibes, the
  21 golden lists' seeds, and the album loops of `docs/DONE.md`. Kept in
  `core/golden/generation.hashes`; CI checks the JavaScript against it.
- No change to what anyone hears beyond the handful above.

**Yardsticks:** the mirror equal to the core on every input tested; the
generation goldens stable across three runs and across Node and Chromium;
the list of loops that moved (with what moved) in the PR; tests three
times.

## 78. The generation data as plain text, read by both

**Why:** promise 5 (profiles, vibes, moods and voices as data, a plain-ASCII
text form people can read and edit) and the two-tracks rule: if the data is
one file both generators read, vibe tuning never has to wait for the port.

**What:**
- One text format (propose it in the PR before converting: readable, plain
  ASCII, comments allowed, no dependency to parse it in Rust or JavaScript),
  and today's tables converted into it: the characters (profiles), vibes,
  moods, the voice table (layers, costs, levels) and the choice tables of
  `js/choices.js` (`ARC`, `KICK`, `SUBDUCK`, `LATEFLOOR`, `FLOORDROP`,
  `PEDAL`, `PADSWEEP`, `ECHO`, the change counts).
- The JavaScript reads it (fetched at start in the app, read from disk in
  Node); the tables in code go. `js/vibes.js` stays the brain's to edit in
  its new form (`AGENTS.md`'s rule moves with it).
- A compact binary form for the core is item 79's business; this item
  fixes the text.

**Yardsticks:** generation goldens, core baseline and balance lock
unchanged; a test that every entry parses and every name it uses exists;
`docs/VIBES.md` says how to edit a vibe now.

## 79. The Rust generator, 1: the spec and the harmony

**What:** in `core/` (its own module tree, `gen/`), from the seed and the
data: `newSpec` (the vibes' draw, drive, the profile mix, the feel, bars,
tempo and gait, key and mode) and `render`'s harmony and form (progression,
sections, rests, gaps, silent bars). The data compiled from item 78's text
into the core (a build step that writes a Rust table, or a small binary the
core reads at `Core::new`: say which and why). A host call returns the spec
(`dl_spec`, plain ASCII, for tests and the app's Now Playing).

**Yardsticks:** generation goldens (a) and the harmony part of (b) equal on
every case and every target; no allocation after `Core::new`; memory in
bytes reported.

## 80. The Rust generator, 2: the layers

**What:** the tracks: drums, bass (with item 56's register and item 71's
pedal), chords (with longChords, and `separateRegisters` exactly as today:
the octave fix is item 84, separate on purpose), melody (motifs, cells,
development), texture, the choir. The voice choice per layer, the entry
schedules, the repairs and gaps.

**Yardsticks:** generation goldens (b) equal on every case and target;
`stats.mjs`' corpus figures computed from the Rust tracks equal the
JavaScript's (a tool that reads both).

## 81. The Rust generator, 3: the choices, the load and the wander

**What:** item 54's per-loop choices and every choice's own draws (the arc's
shape, the kick, subDuck, lateFloor, the floor drop's shapes, the pedal,
padSweep, echoTime), the per-loop decisions the engine makes at load (stab,
organ, wash, space, kick level), and `drift` (each wandered pass, the floor
drop, item 67's kick swap).

**Yardsticks:** generation goldens (c) and (d) equal on every case and
target. With this, every decision the JavaScript makes before scheduling has
a Rust twin, proven.

## 82. The player: a loop from a seed, in the core

**Why:** Broken Friends' gate: a friend's theme, from a seed, with no
JavaScript. And the end of phase 2's port.

**What:**
- The sequencer: what `Engine._scheduleStep`, `_organ`, `_glass`,
  `_dubStab`, the floor drops and the late floor, and `Synth`'s note calls
  do between events and the core's notes (timing, swing, articulation
  values from `js/draw.js`'s streams, sends, choke, retune, damp, the
  per-loop settings and routes), in the core, sample-accurate.
- A host call to play a loop from a seed (and vibes): `Core::load(seed,
  request)` then render; the loop wanders pass after pass as the app's does
  with the wander on (the amount a setting). Mute per layer, tempo and
  volume as the app has them.
- `dlrender --seed N [--vibes ...] [--seconds S] [--stereo]` renders it
  natively; the C header (promise 4) gets the calls.
- The app does **not** switch to it yet (phase 3, item 53 and the rest);
  `?engine=player` plays it in the app for testing.
- Tell Broken Friends: a note in `docs/notes/82-for-broken-friends.md`
  with the calls the game uses, what changed since the pin at f654fdb, and
  the hashes to refresh.

**Yardsticks:**
- **The baseline's 40 loops, full and lite, mono and stereo, played from
  their seeds natively, equal the browser's renders bit for bit** (the
  same out hashes `tools/baseline.mjs` keeps), wander on, 45 s; where one
  can't be (a float order in the page's timing), say exactly where and by
  how much, and the brain decides.
- The goldens: every target renders the same samples from a seed.
- Cost: generating a loop spread so no block passes 25% of its deadline on
  the slowest target (promise 1); generation's share of a minute's render
  reported. No allocation after `Core::new`; memory in bytes.
- A seed-to-WAV run of the corpus's first 80 loops in CI (hashes kept).

## 83. Ears on the core: sketches before the JavaScript synth retires

**Why:** the ears sessions sketch new sounds as patches to the JavaScript
synth (`tools/ears/sketches/`) and hear them in offline renders; that synth
retires at the end of phase 3, and sketches in it already sound different
from the core. They need a way to sketch on the core itself, cheaply, with
no Rust beyond the sketch. And the brain needs clips the app can't make
(choices forced, a constant moved), as Session 26 did by hand.

**What:**
- **Sketch blocks:** a folder (`core/sketch/`) where an ears or brain
  session drops one block file (the kit's interface) and one line of data
  naming where it plays (a channel's own block, a voice for a layer, a send),
  built only with a `sketch` feature, never in the app's `.wasm`. The kit
  probes it as any block (level, tone, cost) with no test written.
- **`tools/clips.mjs`:** clips through the real engine on the core (from
  Session 26's scratch renderer, `docs/archive/session26-clips.md`): corpus loops by seed or `profile #n`,
  seconds, a window (`--from`), mono or stereo, wander on or off, **any
  choice forced** (`--force lateFloor=on`; through the request once item
  85 lands), any named setting pinned, a sketch block on, and a constant
  moved for the A/B (`--set bass.LIFT_DB=3`, through a sketch-only table of
  named constants), to WAV and Opus, with an index of what each loop drew.
- `docs/ears/sessions.md` and `tools/ears/README.md` say how; the old JS
  sketch path stays until phase 3 ends.

**Yardsticks:** a demonstration sketch (a test-only block, one file and one
line) heard in a clip and probed by the kit; the clips tool reproduces
Session 26's forced clips sample for sample; the app's `.wasm` unchanged
with the feature off (byte for byte).

## 84. The chords' octave from the progression

**Why:** Ears 4's finding (`docs/BRAIN.md`, "Start here"; the charter's
phase 2): `separateRegisters` decides the chords' octave from the sounding
events, so a change that doesn't touch the harmony can flip it (11 of 87
loops when only the chord lengths changed), and pins (85-91) are exactly that
kind of change. It goes in before the request.

**What:** decide the octave from the progression and its voicing before
form, rests and gaps, in Rust and JavaScript alike (the JavaScript still
plays the app). The sketch fix and its numbers: `docs/ears/batch-4.md`,
"Round 4", and `docs/EARS.md`.

**Yardsticks:** a corpus check that varies one non-harmonic choice per loop
(chord lengths, rests, tempo, the bass style, a muted layer) over 2,000
loops and counts octave flips: 0. The loops whose octave moves today
(the two ways disagree) listed with how many; generation goldens updated on
purpose; the brain's blind album covers them.

## 85. The request: vibes and tempo (was 48)

48's text (below, kept for reference until this item merges) is the brief,
with these changes:
- Built in the Rust generator, and in the JavaScript one while it still
  plays the app; the request is one shape both read (the app sends the same
  thing to either).
- **Pins for every named setting already built:** the arc (`ARC`), the kick,
  subDuck, lateFloor, padSweep, the echo time, the space recipes (item 65's
  32), and any of item 54's choices on or off. These are the game's and the
  ears' levers (item 83's `--force` moves onto them).
- **Steering, as a game asks it:** a request carries the vibes with weights
  (a friend's mood, the time of day: the game maps its state to vibes; the
  core never knows what a friend is), and a **theme**: a seed or a code
  whose loop the request bends rather than replaces (a friend keeps its
  theme as the day changes).
- Share codes carry the pins (the long code until phase 6).

**Yardsticks:** 48's, plus: a theme bent by a vibe keeps its key, its
motif and its profile mix (measured: pitch classes, the melody's first
cell, the profile's lead) unless a pin says otherwise.

## 86. Changing loops at a musical moment, with a crossfade

**Why:** Broken Friends' music changes at moments, not on a timer (its
decision 0038: a new room, a friend made well, a mood, surgery). Today a new
note list resets a core (about 15 ms on a laptop, more on a phone), too slow
for the audio thread, so the game keeps two cores and switches. The app's
Next would use the same (phase 3).

**What:**
- `Core::change(request, at)`: the next loop starts at a boundary the host
  names (now, the next beat, the next bar, the end of the phrase, the end
  of the pass), and the old one hands over by a **crossfade** (its
  length a setting, default the conservative choice, "for Mikey's ears")
  or by letting its notes ring out, no reset and no second core. Held
  instruments (the organ, the wash, padSweep, the arc) hand over as item
  72's padSweep does across a load (glide, never jump).
- An **event** is a request attached to a moment (promise 11: "when the sun
  rises"); several may wait, the latest for a boundary wins.
- The cost of a change spread across blocks: no block over 25% of its
  deadline on the slowest target while a change happens.

**Yardsticks:** a change costs no allocation and no block past the
budget; across 1,000 changes at every boundary kind no click (the sample
step at the join no larger than the loops' own), the new loop's samples
after the crossfade equal the same loop loaded alone (from the boundary
on, given the same start state of the held instruments: say what differs);
the game's two-core switch can go (the note for Broken Friends says how).

## 87. Rendering ahead, for the browser

**Why:** Godot's web build has no audio thread; music fed frame by frame
crackled on Mikey's phone, so the game renders a loop whole and plays it as
one sound. With wandering and endless loops it needs passes rendered ahead,
quickly, on a phone's browser: the Moto G Power (2022) and weaker, on the
lite mix.

**What:**
- Render ahead in chunks a host asks for (a pass, or N seconds), resumable:
  the next chunk continues exactly where the last stopped (wander, endless,
  a change waiting at a boundary), the same samples as rendering straight
  through.
- A host can ask how long a chunk will take to render (the core's own
  estimate from the loop's voices) and spread it across frames.
- **The Moto G Power (2022) joins promise 2's measured devices**, with the
  speed target the brain proposes once measured (the game needs a pass
  rendered well inside its playing time, lite, stereo 1.2-1.4 times mono).
- An Emscripten build of the core in CI (Broken Friends' web build links it
  with panic=abort), rendering the goldens to the same hashes.

**Yardsticks:** chunked equals straight through, sample for sample, over
every chunk size from 128 frames to a pass; render speed on the web host
(wasm in Node and in headless Chromium) reported per loop, lite and full,
mono and stereo, as times real time; the Emscripten goldens equal.

## 88. Length, up to 32,768 bars and Endless (was 49)

49's text (below) is the brief, built on 82's player and 87's chunks. Plus:
Endless and long pieces render ahead as 87 says.

## 89-91. Key and scale (was 50), style (was 51), voices and layers (was 52)

50, 51 and 52's texts (below) are the briefs, in that order, each on 85's
request. Sonnet 5.5, high, each.

## 92. "Let the loop wander", unrepeatable in the slider's upper range

**Why:** the known issue (below, "Known issues"; Mikey, 2026-10-18): all the
way up, wandering should feel experimental, not repeatable.

**What:** as the known issue sketches: the core's draws stay a function of
what the host sends; the host adds a per-play salt to the wander's seed
across the slider's upper range (where it starts is a named setting, first
at the conservative point, "for Mikey's ears"); a share code or export says
which it was. Item 54's choices stay the loop's own.

**Yardsticks:** the same code played twice in the upper range wanders
differently (passes 2-8 differ), below it identically (bit for bit); the
generation goldens (wander at the default) unchanged.

## The phase's end (brain)

When 92 is merged: the brain's check (generation goldens on every target,
the player against the browser, costs, the phone figures), and the
charter's blind album: the old generator against the new on the same seeds,
with the octave's movers in it. Then phase 3.

## Next picks, wired for real (items 48-53): the briefs folded into phase 2

**Folded (brain, Session 26):** 48 is item 85, 49 is 88, 50-52 are 89-91, built on
phase 2's request; 53 stays in phase 3. Their texts stay here as those items' briefs
until they merge.


**Read first:** `design/next-picks.md` (the design Mikey chose on
2026-10-13, Painter session 5) and `docs/CORE.md`, promise 11 (the
request). The design shows five lines of picks for new loops: moods
(today's vibes), tempo and length, key and scale, style (profile) and
voices. Each is **pinned**, **ranged** or left to **any** (chance).

**What already exists (app track, Painter 6):** the screens, with
moods wired to vibes, and length and tempo wired to today's JavaScript
generator only where cheap, clamped to what it can play (bars from the
profile tables, 2 to 32; tempo 32 to 190 bpm). Tempo, key, style and
voices can't be tapped until their pickers are designed. These items
replace that plumbing with the real thing, in the core.

**Where they sit in the phases.** They are not a phase of their own:
items 48-52 belong to **phase 2** (the generator in Rust, built around
the request) and item 53 to **phase 3** (off the page). The brain's
review orders them among phase 2's own items when it writes them
(Session 26, after the sound batch; moved from Session 24); 48
goes first, since the others stand on it. Don't start them before phase
2 starts: the generator they change is the Rust one.

**Rules for these items** (on top of the standing rules and the core
rules in `core/AGENTS.md`):
- **When nothing is picked, nothing changes.** Every loop with all five
  lines on "any" renders exactly as it did before the item (goldens,
  `tools/baseline.mjs`, the balance lock). A pick may only change loops
  that asked for it. Leaving a choice to chance draws from the seed as
  today, with no extra draws that shift the rest.
- **Requests bend the music, never break it** (promise 11). A pin that
  pulls against a profile's character finds its in-between, as vibes do.
  Only a request that truly can't be met says so, in a short plain line
  (the "can't do that" message; its look is a Painter question, until
  then plain text in the panel).
- **Picks shape loops still to come, never the one playing**, and they
  hold through re-rolls and "Let the loop wander": every new loop and
  every wandered pass honors them. In an album, the album decides and
  the picks wait (the design hides them there).
- **Measure that a pick is honored in the render**, not only in the
  spec: tempo from the note onsets, bar count from the cycle, scale and
  key from the pitch classes, voices and layers from what sounds. A test
  walks every pickable choice (promise 11's measure).
- **A picker the design doesn't have yet** (tempo, key and scale, style,
  voices, as of 2026-10-13): build the request side and its tests; the
  app's line stays untappable until a Painter session designs the
  picker. Don't invent the UI.
- **Taste:** where an item leaves a musical choice open, take the
  conservative one and mark it "for Mikey's ears", as always.

Hands: chosen at the phase review. The brain's suggestion: 48 and 49
Opus 5.5, high (the request's shape; sectioned generation); 50-52
Sonnet 5.5, high (clear specs on 48's pattern); 53 Opus 5.5, high
(playback, phase 3).

## 48. The request, end to end, with tempo (phase 2)

**Why:** promise 11. One request for everyone: the app's panel, a
developer's code and a game's events build the same thing. Tempo is
the simplest pick to prove it with.

**What:**
- The Rust generator takes a **request**: for each choice, pinned,
  ranged or chance. Start with vibes (as today's selection) and tempo;
  leave room for the rest without building them (48's design is what
  49-52 follow, so keep it small and documented).
- **Tempo:** pinned ("77 bpm") or a range ("70-85 bpm", the design's
  example). A range draws from the seed inside it. Profiles with gaits
  draw tempo and meter together today: a pinned or ranged tempo picks
  the gait that fits and bends the tempo inside it. When style is left
  to chance, prefer profiles whose band holds the tempo; when style is
  pinned too and its band doesn't, play the profile at the asked tempo
  (conservative: honor the pin; "for Mikey's ears"). The core's limit
  is the core's to set; the app's picker range is the Painter's.
- The app builds the request from the panel and sends it with each new
  loop; today's vibes selection becomes part of it. Remove Painter 6's
  clamped tempo plumbing.
- Share codes: a loop made from a request reproduces from its code.
  (Short codes that carry only the pins are phase 6.)
- The "can't do that" line, for whatever request truly can't be met.

**Yardsticks:**
- Nothing picked: goldens, baseline and balance lock unchanged.
- Pinned tempo: measured bpm within 0.5 of the pin on every target,
  across 200 seeds and every profile.
- Ranged tempo: every loop inside the range; over 500 seeds, the draws
  cover the range (no clumping at one end; report the spread).
- Same request and seed, same samples on every target.
- A test that walks the request's choices (it grows with 49-52).

## 49. Length, up to 32,768 bars and Endless (phase 2)

**Why:** Mikey, 2026-10-13: far longer loops than today's 32 bars, the
app's picker to 32,768 bars plus Endless; no cap in the core (charter,
promise 11). The design's Length picker already exists (Painter 6).

**What:**
- **Pinned length**, any whole number from 1 bar up, and **Endless**.
  The generator works a **section at a time**: memory and cost are the
  same for 4 bars, 32,768 or Endless, and any section is reachable
  straight from the seed and its number, without generating what comes
  before (so a long piece can later be jumped into).
- **What fills a long piece, until long-form composing exists:**
  sections drawn the way "Let the loop wander" draws its next pass,
  holding the request's other picks (key, tempo, style, voices, vibes).
  Writing a piece with an arc (themes that return, a symphony's
  journey, or long slow wandering) is later composing work by ear, its
  own item; this item only makes sure it needs no rework. Conservative,
  "for Mikey's ears".
- A short pinned length (1, 2, 3 bars) plays as a loop of that length,
  honored even where today's profile tables never draw it.
- Length left to chance: today's draws, unchanged.
- The Length picker's play time ("bars · ≈ 45-55 s", "≈ 3-4 hours")
  comes from the request's tempo range, from one function the app and
  the core share, so the two never disagree.
- Remove Painter 6's clamped length plumbing (bars beyond today's
  tables now come from the core).

**Yardsticks:**
- Nothing picked: unchanged, as above.
- Pinned length: the rendered cycle is exactly that many bars (1, 2, 3,
  7, 24, 32, 1,024 and 32,768, every profile).
- Section *n* rendered straight from the seed equals section *n* reached
  by rendering through, sample for sample (hash), for *n* up to 1,000
  and for the last section of a 32,768-bar piece.
- Memory after `Core::new` and cost per section flat from 4 bars to
  Endless (the core rules: no allocation after `Core::new`).
- Endless plays for a 24-hour render without drift or growth (in CI,
  the command-line renderer at speed).

## 50. Key and scale (phase 2)

**Why:** line three of the design: "any key", or a pinned home note and
scale (major, minor, dorian, pentatonic and the rest).

**What:**
- Key (home note) and scale, each pinned, ranged (a set: "D or G";
  "any minor-ish mode") or chance. Follow 48's request pattern.
- **Vibes and scale:** today's vibes pull the mode (and cinder and
  shatter borrow bright modes when glad). A pinned scale wins; the vibes
  then lean everything else. A pinned scale against a profile's
  character finds its in-between ("no pair is forbidden").
- Profiles with fixed modes (wayfare's minor v, cinder's added F over
  E: Mikey's decisions stand) keep those features where the pinned
  scale allows them; say per profile what a pin changes.
- The app's line: values shown ("D dorian"); untappable until the
  picker is designed.

**Yardsticks:**
- Nothing picked: unchanged.
- Pinned key and scale: the loop's pitch classes stay in the scale
  (borrowed chords the profile is known for excepted, listed) and its
  tonic is the pinned key, measured from the render across 200 seeds
  and every profile.
- Refusals on full and lite for pinned scales no worse than the
  catalog.

## 51. Style (profile) (phase 2)

**Why:** line four: "any style", or one or several of Driftloom's
profiles (cinder, tide, wayfare, shrine and the rest).

**What:**
- Style pinned to one profile, or a set ("tide or shrine"); chance as
  today. Each profile keeps its own character, instruments and drums;
  vibes bend it within that character, as today.
- A pinned style with other pins that pull against it (tempo outside
  its band, a scale it doesn't use) follows 48 and 50: honor the pins,
  find the in-between, "can't do that" only if truly impossible.
- The app's line: the profile names; untappable until the picker is
  designed.

**Yardsticks:**
- Nothing picked: unchanged (the balance lock guards what plays on
  chance).
- Pinned style: every loop is that profile; a set draws only from the
  set, each member reached across 500 seeds.
- Per profile, `stats.mjs` figures for pinned-style loops match the
  same profile's loops drawn on chance, within stated tolerances (a
  pin picks a profile; it doesn't change it).

## 52. Voices, and which layers play (phase 2)

**Why:** line five: "any voices", or chosen instruments per layer
(melody, chords, bass, texture, drums), and layers left out (fewer
layers is welcome variety).

**What:**
- Per layer: a voice pinned or a set, or chance; and the layer on, off
  or chance. Follow 48's pattern.
- **The rarities stay rare:** the choir (about 1 loop in 30) is not
  forced by picking melody and chords voices; it can be asked for only
  if the voice list offers it by name. Conservative: not offered until
  Mikey says so.
- A voice pinned onto a profile that never uses it plays at its
  measured level (each voice already sits at its layer's median) and
  passes the refusals; say which combinations needed care.
- All layers off is the one obvious "can't do that".
- The app's line: values shown ("kalimba, marimba"); untappable until
  the picker is designed.

**Yardsticks:**
- Nothing picked: unchanged.
- Pinned voices and layers: what sounds is what was asked, per layer,
  measured from the render across 200 seeds and every profile.
- Levels: pinned-voice loops land on the catalog's level spread
  (`measure.mjs --profile`), no layer louder than its median allows.
- Refusals on full and lite no worse than the catalog.

## 53. The picks through the core's commands (phase 3)

**Why:** phase 3 moves the whole core off the page; the page only sends
commands and draws events. The picks become a command like the rest,
and long pieces and Endless play from the core alone.

**What:**
- The panel's request is sent as a command; the core holds it and
  applies it to every new loop, re-roll and wandered pass. No pick
  logic left on the page.
- Long pieces and Endless play with the page asleep, hidden or busy
  (promise 1's stall test), with no lookahead on the page.
- Now Playing's pills, the panel and the Length picker's play time read
  from the core's events, so what they show is what the core will play.
- Long-piece display on Now Playing (where you are, time left, Endless
  with no "time left": `docs/PAINTER.md`, section 2) only once a Painter
  session has designed it; until then, today's clock.

**Yardsticks:**
- Promise 1's stall test clean with a 32,768-bar piece and with
  Endless.
- A pick made while a loop plays changes the next loop, never the
  current one.
- Picks survive the page being hidden and shown again, and a reload
  (stored as the vibes are today).

## 37. Word albums on `/rank`: one word, "does it fit?"

**On hold (Session 16): skip it unless the brain's State says otherwise.**
Vibes in the app (item 38) may do this job better: Mikey picks a vibe and
tells the brain which loops fit.

Mikey's idea (2026-10-11): "Is this playlist heartwarming? Which songs are
not?" The brain sends a word album (item 35b); Mikey answers each track
with one tap. The pilots, "Heartwarming?" and "Joyful?", went out as
plain albums (`docs/ALBUMS.md`); **if the brain's State says the pilot
didn't work, skip this item.**

- **`rank.html?album=<DLA1 code>`** (the word from the album's title,
  "Heartwarming?" -> heartwarming; `&word=` overrides): plays the album's
  loops in order, each as composed (`driftOn = false`, as now), looping
  until answered. Shows only the question, "Heartwarming?", and the track
  number of the total; no profile, mood or title per track.
- **Buttons:** Fits, Doesn't fit, Not sure; Undo goes back one. A track
  can be answered while it plays; the next starts at once.
- **Record** each answer as a line in the same export, so `ranks.mjs`
  reads it with the rest: `trial`, `build`, `code`, `album` (the album
  code's first block and the word, enough to tell albums apart), `track`,
  `words: [word]`, `taps: [word]` for fits and `[]` for doesn't fit,
  `answer: "fits" | "no" | "unsure"`, `seconds`, `quality`. Saved as
  `/rank` saves now; Export and Clear as they are.
- **`ranks.mjs`** counts album answers apart from the five-word trials
  (per album: fits, doesn't fit, not sure), and `--fit` uses them (item
  35a). The five-word trial stays as it is without `?album=`.
- No hidden repeats in albums (they are short); no generator, synth or
  share-code change; the balance lock doesn't move. The page plays the
  core like the app (item 32's note: `/rank` still builds the JavaScript
  `Synth`; wire it to the core here, with `?engine=js` as the fallback,
  and say what changed). Bump `BUILD` and `CACHE`.
- **Checks:** both pilot album codes in `docs/ALBUMS.md` open, play and
  record on the core and `?engine=js`; an export holding both kinds of
  line reads in `ranks.mjs`; tests three times.

## Later, not queued

Mikey liked the fiddle, accordion and drone as they are, and may want
more nuance in them later. Not now. New instruments are welcome again
(Mikey, 2026-10-09; the 2026-09-25 pause was temporary): the ears-approved
sounds are queued after the core becomes the default, built in the core
one at a time with the refusal and level checks (`docs/BRAIN.md`, State).
Inspiration comes only from wholesome, comforting music.

## Known issues

- **"Let the loop wander" repeats itself since item 44** (Mikey,
  2026-10-18). Since #211 each pass's wandering is drawn from the loop's
  code and the pass number, so replaying a saved loop wanders exactly as
  it did last time, at every slider setting. Mikey: the wander slider
  (0 to 2.4) says how far a loop wanders, and **all the way to the right
  it should feel experimental: not repeatable.** Not urgent. Brain's
  sketch for when it is queued (the generator's port, phase 2, is the
  natural place): the core's draws stay a pure function of what the host
  sends (promise 3 holds); the host adds a per-play salt to the wander's
  seed across the slider's whole upper range (**decided by Mikey,
  2026-10-18**; where that range starts is to be tuned by ear as we
  go), so the same code with the slider lower still wanders the same
  way, and a share code or export says which it was.
  **Queued as item 92 (brain, Session 26).** Where it was written down (brain, Session 24, 2026-10-19): to be
  built in phase 2's port, not before; carried in `docs/CORE.md` (phase
  2, beside the octave fix) and in `docs/BRAIN.md` ("Start here"), so
  whoever writes phase 2's items queues it. Until then the sound batch
  leaves it as it is: item 54's per-loop choices are drawn once a loop
  and are not redrawn by the wander, whatever the slider says (item 67's
  kick swap aside); with the salt, the upper range's wandering would vary
  play to play, while the loop's choices stay its own.

## Merge policy

Mikey decides this line:

- (yes, from Mikey) When CI is green and every check above passes, Claude
  Code merges its own PR and starts the next item.
