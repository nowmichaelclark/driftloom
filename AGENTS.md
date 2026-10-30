# Driftloom: guide for agents

Read first by every Claude session in this repo (`CLAUDE.md` points here).
`docs/README.md` maps every doc and says who reads it.

## What Driftloom is

A procedural loop machine at <https://driftloom.nowmichaelclark.workers.dev/>:
relaxing, nostalgic loops, generated and synthesized in the browser, played
forever. A loop is a small spec (`js/generator.js`); its notes play on a Rust
audio core compiled to WebAssembly (`core/`, built to the committed
`js/dlcore.wasm`) in an AudioWorklet; the JavaScript synth is the fallback
(`?engine=js`). The page has no build step and no dependencies, and it
deploys as static files (`wrangler.jsonc`; pushing to `main` deploys). Mikey
(nowmichaelclark) owns it. His ears are the acceptance test for anything
that changes how it sounds.

## The roles, and where each starts

- **Brain** (a claude.ai chat in the Driftloom Project): plans, verifies,
  writes the queue, builds listening albums; does not write code, and
  merges only docs-only changes (and `js/vibes.js`). Starts at
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
- Tests (Node 22; run the generator's three times, it uses random seeds):
  `node test/generator.test.mjs`, and `worklet`, `core-host`, `engine`,
  `dials`, `fit`, `golden`, `draw`, `picks`, `dlmath`, each as
  `node test/<name>.test.mjs`.
- Generation goldens (item 77): `node tools/gen-golden.mjs --check` (and
  `--chromium`); after a change meant to move what is generated, run it
  without `--check` and say in the PR which cases moved. The generator's
  paths call `js/dlmath.js` (the core's math), never `Math.pow`, `log`,
  `sin` and the rest.
- Balance lock: `node tools/stats.mjs --check`. Core baseline:
  `node tools/baseline.mjs` (`--update` on purpose, saying what moved).
- Core: `tools/build-core.sh` rebuilds `js/dlcore.wasm`, and `--check`
  proves it matches the source; `cd core && cargo test --locked`; golden
  renders `tools/golden.sh --strict <target>`. More in `core/AGENTS.md`.
- Measuring: `tools/measure.mjs`, `tools/perf.mjs`, `tools/views.mjs`;
  usage in the queue's standing rules and the file headers.
- Docs: `node tools/check-docs.mjs`.
