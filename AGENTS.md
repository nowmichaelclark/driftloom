# Driftloom: guide for agents

Read first by every Claude session in this repo (`CLAUDE.md` points here).
`docs/README.md` maps every doc and says who reads it.

## What Driftloom is

A procedural loop machine at <https://driftloom.nowmichaelclark.workers.dev/>:
relaxing, nostalgic loops, generated and synthesized in the browser, played
forever. A loop is a seed and a request; the core (`core/`, a Rust audio
core compiled to WebAssembly, built to the committed `js/dlplayer.wasm`)
generates it, and its player decides and plays the notes in an
AudioWorklet. The page sends commands and draws events (`js/player.js`),
and reads a loop's record and notes from the same module (`js/gen.js`).
Since item 97 there is no JavaScript generator or synth: a browser that
can't run the core gets one plain line. The page has no build step and no dependencies, and it
deploys as static files (`wrangler.jsonc`; pushing to `main` deploys). Mikey
(nowmichaelclark) owns it. His ears are the acceptance test for anything
that changes how it sounds.

## The roles, and where each starts

- **Brain** (a claude.ai chat in the Driftloom Project): plans, verifies,
  writes the queue, builds listening albums; does not write code, and
  merges only docs-only changes (and `data/vibes.txt`, the vibes). Starts at
  `docs/BRAIN.md`, "Start here".
- **Hands** (Claude Code): write code, open PRs, measure, merge, report.
  Start at `docs/QUEUE.md`: status, standing rules, the next item. A
  session that touches `core/` also reads `core/AGENTS.md`.
- **Ears** (claude.ai chats): measure records Mikey loves against Driftloom
  and write findings and sketches, never app or core code. Start at
  `docs/ears/sessions.md`, then `docs/ears/plan.md`.
- **Painter** (design sessions): the look and the screens. Starts at
  `docs/PAINTER.md`.

## Rules that hold everywhere

- **Merge policy** (Mikey: yes). The hands merge their own PRs when CI is
  green and every check the item names passes (`docs/QUEUE.md`, "Merge
  policy"). Red or unfinished checks: don't merge, say what is failing.
- **Never write a token anywhere:** not in a file, commit, PR, note, memory
  or the Project. Mikey pastes one into a session's opening message; use
  it, never keep it.
- **American English** in new comments, docs and PR text. Don't rename
  identifiers or spec fields for spelling: share codes depend on them.
- **Plain ASCII from the core:** everything the core emits (names, codes,
  errors, logs) is ASCII.
- **BUILD and CACHE together:** on every app change bump `BUILD` in
  `js/main.js` and `CACHE` in `sw.js` to the same next version, taken from
  the latest `main`.
- **One PR per item**, from the latest `main`. Take the conservative option
  where a musical choice is open and mark it "for Mikey's ears". If a
  check can't be met or a decision is needed, stop and leave a note in
  `docs/notes/<item>-<name>.md`.
- **Keep the docs small:** the live docs hold what is live; finished
  items, old notes and history go to `docs/archive/`. CI enforces sizes
  (`node tools/check-docs.mjs`).

## Build and test

- Page: `python3 -m http.server 8000`, then <http://localhost:8000>
  (ES modules don't load over `file://`).
- **The gate** (item 103): `tools/gate.sh` runs what CI holds a merge to
  (`core`, `sketch`, `node`, `baseline`, `docs`, or all); the slow renders
  are the nightly's (`.github/workflows/nightly.yml`) and hold nothing.
- Tests (Node 22): `worklet`, `core-host`, `data`, `dials`, `fit`,
  `golden`, `picks`, `quality`, `player`, each as
  `node test/<name>.test.mjs`. Those that read what the generator decides
  (and `stats`, `dials`, `gen-golden`, `octave`: `tools/gen-lib.mjs`) need
  the tools' build first: `tools/build-core.sh --tools`.
- Generation goldens (item 77): `node tools/gen-golden.mjs --check`; after
  a change meant to move what is generated, run it without `--check` and
  say in the PR which cases moved.
- Balance lock: `node tools/stats.mjs --check`. Core baseline:
  `node tools/baseline.mjs` (`--update` on purpose, saying what moved).
- Core: `tools/build-core.sh` rebuilds `js/dlplayer.wasm`, and `--check`
  proves it matches the source; `cd core && cargo test --locked`; golden
  renders `tools/golden.sh --strict <target>`. More in `core/AGENTS.md`.
- Measuring: `tools/measure.mjs` (on the player, in Node: loudness, the
  voice probe, refusals, endings), `tools/perf.mjs`, `tools/views.mjs`;
  usage in the queue's standing rules and the file headers. The app's
  controls on the player, a check each: `node tools/rows.mjs` (`--long`
  adds the length slider's minute).
- Docs: `node tools/check-docs.mjs`.
