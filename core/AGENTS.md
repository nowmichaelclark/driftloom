# The core: standing rules

For any session that touches `core/` (the Rust engine; `js/dlcore.wasm` is
built from it). Read the root `AGENTS.md` first. The queue's standing rules
(`docs/QUEUE.md`) apply too; these come on top. The charter is
`docs/CORE.md` (ten promises, each with its measure); how a host drives the
core, and how to add a block, is `core/README.md`.

## Commands

- `tools/build-core.sh` rebuilds `js/dlcore.wasm`; `--check` fails if the
  committed file is not exactly what the source builds. Commit both.
- `cd core && cargo test --locked`
- `node tools/baseline.mjs` (the core's renders against
  `test/core-baseline.json`; `--update` on purpose, saying what moved). It
  needs Playwright's Chromium: `npm install --no-save playwright@1.56.1`.
- `tools/golden.sh --strict wasm32-unknown-unknown` (also `--flush`,
  `--blocks`, `--rates`, `--stereo`; CI runs every target).
  `node tools/rates.mjs`.
- `node tools/stats.mjs --check` (the balance lock) when generation moves.

## Standing rules for core items

In addition to the queue's rules; from the charter's promises 2 and 6, held
on every PR.

- Nothing allocates, locks or frees after `Core::new`; nothing reachable
  from rendering can panic. Say how you checked.
- Every public item the PR touches is documented, with the reason, not
  only the what. `cargo clippy --all-targets -- -W clippy::pedantic` reports
  zero in every file, and CI holds it there (`-D warnings`); allow a lint
  locally, with a one-line reason, only where following it would make the
  code worse.
- No dependencies in `core/` (dev-only tools in CI are fine).
- Everything the core emits (names, errors, logs) is plain ASCII.
- Costs: the core baseline's render costs may not rise by more than 5%
  on any loop without a note saying why it is worth it.
- The JavaScript synth is the fallback until the end of phase 3: leave it
  working, untouched unless an item says otherwise.
- The core is no longer a copy of Chromium. Where a doc comment says a
  block "follows Chromium to the sample", leave it if still true, but
  new work is judged by the charter's measures, not by nulling.

## Rules for sound items

The sound batch's rules (items 54-73, 2026-10-18; the items are in
`docs/archive/queue-items.md`). They are the pattern for any new voice,
block or mixing choice.

- **Where it is decided, where it sounds:** *when* a sound plays and
  *which loops* get it is the JavaScript side's (the generator, or the
  engine at load where the sketch decides it), allowed until phase 2
  starts and then ported with everything else, as part of what the port
  must match. *What* plays is the core's: each new sound a block or a
  voice in `core/src/blocks/`, one file and one line, probed by the kit,
  with its stored render.
- **Core only.** The JavaScript synth (the fallback) builds nothing new:
  for a new voice number it plays the nearest existing voice, or leaves
  the part out where nothing fits; each item says which. `js/core.js`
  mirrors the new numbers.
- **Mikey's rules for every choice** (2026-10-18): today's behavior stays
  one option in every choice, never replaced; each choice is drawn per
  loop, seeded from the loop's code on a stream of its own (so no other
  draw moves and a loop replays the same way); **most loops get one or
  two of the changes** (item 54 sets how). Exceptions, as approved:
  roundBass is every loop's bass, longChords keeps its own approved
  spread (today's lengths 56%), and floorDrop replaces full stops as
  approved; the stereo baseline is every loop's.
- **Measured against the ears.** Each new sound is measured the way its
  report measured the sketch, on the report's corpus loops (corpus seed
  20261009; `tools/ears/`: `render2.mjs`, `sketch_check.py`, `warmth.py`,
  `low_guard.py`, `kick_check.py`, `stereo_guard.py`), core build against
  sketch: level by ear (its loudest 3 s against the loop, K-weighted)
  within 1.5 LU of the sketch's figure, its own tone figure (centroid,
  attack, band shares) where the report gives one, and refused notes no
  more than the sketch's (none, in every report). Plus the standing
  rules: `VOICE_COST` from render timing, the tone probe's 2-5 kHz share
  at or below the flute's for anything that plays in the melody's
  register, refusals on full and lite. Say where the core's version must
  differ from the sketch, and why.
- **Peaks:** from item 55 on, no voice peaks over -1 dBFS alone at the
  kit's probe (the kick's neighborhood today).
- **Baselines:** each item that changes sound updates the core baseline
  and says what moved; **the balance lock is re-baselined once per group**
  (`stats.mjs --check`, then `--n 10000`), on purpose, at the group's
  last item, saying what moved.
- **For the album:** each Done line names two corpus loops that draw the
  item's sound, with a time, so the brain can build a labeled album
  (keep / tune / drop) without searching.
- **Stop and leave a note** (`docs/notes/<item>-<name>.md`) where a recipe
  can't be met in the core as heard, rather than build something Mikey
  didn't hear.
