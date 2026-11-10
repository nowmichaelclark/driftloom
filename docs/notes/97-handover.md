# Item 97, half done: where it stands (hands, 2026-11-10)

On the branch `ccr-759f44f6-r74si1` (not a PR yet: the gate would fail
until the sweep below is done). `main` is at #355 (96 done).

## Done on the branch, and checked

- **The app plays only the core's player.** `js/engine.js`, `js/clock.js`,
  `js/synth.js`, `js/draw.js`, `js/generator.js`, `js/dlmath.js` and
  `js/dlcore.wasm` are deleted. `js/output.js` (the core's node into the
  speakers) replaces the synth; `js/core.js` and `js/worklet.js` are the
  player's path only; `?engine=js` and `?engine=page` are gone.
- **The page reads loops from the core's generator:** `js/gen.js` is one
  more instance of `js/dlplayer.wasm` on the page (`newSpec`, `render`),
  `js/request.js` the record and the request (what was `generator.js`'s
  request code, with `askOf`, `askFor`, `picksAsk`). Against the old
  generator, before it was deleted: 300 of 300 specs and patterns the same
  over 14 kinds of request, 120 of 120 edited and decoded records.
- `js/theory.js`, `characters.js`, `choices.js`, `pins.js` trimmed to names,
  labels and ranges. `rank.html` plays the player.
- The plain line (`ui.plainLine`, `#plainLine`): "This browser can't play
  Driftloom; try a recent Chrome, Firefox or Safari" in the card's place;
  and "Driftloom couldn't load. Check the connection and reload." where the
  module did not arrive (a new line: for Mikey).
- BUILD and CACHE are v152. `node tools/rows.mjs`: all 27 pass.
- Page JavaScript: 14,652 lines before, 9,166 after (362,854 bytes).

## Left

1. **Tests:** delete `test/engine`, `budget`, `draw`, `generator`, `dlmath`
   (keep `test/draw-vectors.txt`, the Rust test reads it); rewrite
   `worklet.test` (player only), `core-host.test`, `player.test` (it
   compared the core with the page's generator and engine: make it the
   page generator's test), `quality.test` (`synth` is `output`), and
   whatever else imports the deleted modules.
2. **Tools:** delete `measure-js`, `player`, `golden-export`, `core-paths`,
   `coldstart`, `deep`, `ending`, `switch`, `tools/ears/render*.mjs`; move
   `stats`, `gen-golden*`, `dials`, `wordalbum`, `octave` onto the core
   (`js/gen.js` or the tools build); point `rates`, `golden-wasm`,
   `golden.sh`, `golden-lib` at `js/dlplayer.wasm` (it makes the same
   golden hashes: checked on six lists); `build-core.sh` builds one module;
   drop the engine options from `rows`, `views`, `stalls`, `perf`,
   `core-start`. Then `tools/gate.sh`, both workflows, `AGENTS.md`.
3. **Checks the item names:** the plain line in a Chromium with WebAssembly
   off; `tools/views.mjs`; the generation goldens on the Rust generator
   alone; the gate.
4. Docs: the queue's status and item 97, `AGENTS.md`'s first paragraph,
   `core/AGENTS.md` (the JavaScript rules), #355's Done line, for-mikey
   (the device check of 97, the new "couldn't load" line). Delete this
   note.

Diagnostics still prints the page scheduler's lines as constants
(`lateTicks: 0` and the rest): the tools read them. They go with the
tools' sweep.
