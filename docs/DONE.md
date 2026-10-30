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
  notes over as the player does (the brain re-checked it on 200 other loops,
  2026-10-31). The app still plays the JavaScript: `js/dlcore.wasm` is the
  same bytes. Listen for nothing in the app. To hear the player, open the
  preview with `?engine=player`: it should sound exactly as the app does
  (Play, Next, Pause, a mute, the tempo). Tester: Diagnostics there,
  `late: 0`. No album loop.
