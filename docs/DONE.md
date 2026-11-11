# Done

One line for each PR that merged, newest last: the PR number, the version, the
commit preview URL, and what the ears should listen for. Add yours when your PR
merges (`docs/QUEUE.md`, standing rules). Entries before #331 are in
`docs/archive/done.md`.

- #331, the player's module at 636 KB (item 93, v149, no app change), preview
  URL: <https://f7cc7064-driftloom.nowmichaelclark.workers.dev> (commit
  e5635c1): nothing should sound different (the baseline and the 80 seeds are
  bit-identical); `js/dlplayer.wasm` is 636,125 bytes, down from 1,938,596,
  and loads only with `?engine=player` (try it: Diagnostics, late and dropped
  notes). The rest is for the hands: a new player is all zero bytes
  (`tests/zeros.rs`), and one module can replace the two at 94b.
- #337, everything the app does as a command or an event (item 94a, v149, no
  app change), preview URL: <https://33d5bb10-driftloom.nowmichaelclark.workers.dev>
  (commit 0727c51): nothing should sound different (the baseline, the 80
  seeds and the generation goldens' 4,884 cases are unchanged). Nothing to
  hear yet: the new commands (a re-roll in place, the tone, the length
  slider's end, a code's edits) and events (step, pass, phrase, section) are
  for 94b. For Mikey's ears in 94b: a re-roll on the player comes in up to a
  quarter second later than today's. The PR's table maps each control and
  display to its command.
- #341, the sketch levers for Ears 13b (item 99, v149, no app change), preview
  URL: <https://40e97708-driftloom.nowmichaelclark.workers.dev> (commit
  174499e): nothing should sound different (both wasm modules are byte for
  byte, and the 80 seeds and the 5,124 generation cases are unchanged).
  Nothing to hear yet: the sketch build has seven new levers (tripletKit,
  dottedPulse, planingSevenths, slowOpening's wait and fade, tuningOffset,
  widePad as `--space`, dropSpotlight as `--route`, and a retune at every
  chord), all off until a clip pulls them, for an Ears 13b session
  (`core/sketch/README.md`).
- #346, the picks through the core's commands (item 95, v151), preview URL:
  <https://8218f17b-driftloom.nowmichaelclark.workers.dev> (commit 4b6bf50):
  nothing should sound different (the baseline, the 80 seeds and the 5,124
  generation cases are unchanged). The core holds the New loops panel's
  picks and makes every new loop itself, at Next and where the length
  slider's time runs out, so a hand-over plays with the screen off. Listen
  for: Next and a length's end feel as before; a new loop's card comes up
  about a tenth of a second after the tap.
- #354, the fast gate (item 103, no version: CI only), commit 94e34b8: CI on
  a PR is now `tools/gate.sh`'s parts side by side, 10 minutes (it was 70,
  behind the PowerPC renders); every target's golden renders, Endless for 24
  hours and the other slow checks run nightly (`nightly.yml`) and hold no
  merge until 1.0. Listen for nothing. No album loop.
- #355, the checks on the player (item 96, no version: tools only), commit
  ca8fe56: `tools/measure.mjs` and `tools/baseline.mjs` play loops from
  their seeds on `js/dlplayer.wasm` in Node, no browser; the old path and
  the player agree to 0.005 dB on the same 40 loops. Listen for nothing. No
  album loop.
- #356, the JavaScript retires (item 97, v152), commit c9af54e: one engine
  and one generator, the core's; the page's JavaScript 19,538 lines to
  9,166; a browser that can't run the core gets a plain line. Listen for
  nothing: it should sound as v151 did. No album loop.
- #357, the too-slow line (item 100, v153), commit 30a23b9: on lite, when
  the render runs behind real time for two seconds in twenty (or the device
  fills in that often), the card's place says the browser is too slow to
  play Driftloom smoothly; Play goes on. Listen for nothing. No album loop.
- #360, session 01: the tools made steady, the stall test on the shipped app,
  the testers' steps (item 98's hands part, v154), preview URL: the branch's
  preview (no hash kept): nothing should sound different. `tools/rows.mjs`
  starts from a fixed loop (ten runs, ten passes); Diagnostics no longer prints
  the old scheduler's lines; the module that never arrives says "Please
  refresh." and the too-slow line says "This browser is too slow." The stall
  table (84 rows, full and lite, the main thread blocked, the page frozen,
  hidden a minute) reads zero late, left-out and dropped notes. The phones:
  `docs/notes/98-testers.md`.
- #361, the width ceiling after a rest (the control's two loops, v155), preview
  URL: the branch's preview (no hash kept): after a long rest the stereo
  picture comes back at once, not narrow for ten seconds. Two loops (corpus
  20261009) that draw it: **tui-doask at 31.1 s** and **soath-wa at 27.3 s**
  (25 to 50 dB under the music, loudness the same); the baseline's ri-houng
  (full, stereo) moved too.
