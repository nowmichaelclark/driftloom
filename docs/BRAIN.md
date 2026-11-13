# Brain session handoff

Read this first in any new "brain" session. It is the working memory of the
project's planning side: how the work is organized, what he has heard, which
methods proved themselves, and where things stand. What Mikey has decided is
`docs/DECISIONS.md`; the log and history are in `docs/archive/brain-log.md`.
Keep **State** current as you go, not only at the end; a long session can
stop without warning.

## How the work is organized

Three roles.

- **Brain** -- a claude.ai chat in the *Driftloom* project (Mikey's
  nowmichaelclark Claude account since 2026-10-08). Analyzes, verifies, designs, and writes the queue. Does not write code.
- **Hands** -- Claude Code sessions. Write code, open PRs, measure, merge
  under the merge policy, and report.
- **Mikey** -- the ears. His listening is the acceptance test for anything
  that changes how the app sounds. He relays between brain and hands.
- **Ears** (since 2026-10-09) -- separate claude.ai chats that measure
  records Mikey loves against Driftloom renders and write findings with
  yardsticks (`docs/EARS.md`, `tools/ears/`, and the reports, plan and
  album cards in `docs/ears/`; read `docs/ears/sessions.md` for their
  scope and for the rule that their outputs live in the repo).
  They don't touch the queue or app code; the brain turns their findings
  into items.

The loop, since 2026-09-25:

1. The brain writes work into **`docs/QUEUE.md`**: numbered items with a
   why, a what, and yardsticks, under standing rules at the top (every new
   sound measured for level, cost and tone; conservative taste calls
   written down "for Mikey's ears"; stop and leave a note rather than
   guess).
2. Mikey tells the hands to continue with the queue. They work down it,
   one PR per item, and **merge their own PRs** when CI is green and every
   check passes (the queue's merge policy, "yes, from Mikey"). Claude
   Code's own safety check may flag a self-merge as "merge without
   review"; the policy line in the queue is the answer to that.
3. The brain **verifies independently** from GitHub -- fetches main, runs
   the tests and the balance lock, renders loops before and after with
   its own metrics. Reports from the hands have been wrong in instructive
   ways (see Methods).
4. The brain builds **listening albums** for what changed; Mikey listens
   when he has time, and his verdicts become new queue items.

Mikey's point, and the reason for step 4: the one thing nobody in this loop
can do except him is hear it. Numbers check the ear; they do not replace
it.

**Two tracks while the core is built (Mikey, 2026-10-13: yes, the app
keeps changing alongside the phases).** The **core track** is the
charter's phases (`docs/CORE.md`), queue items 42 on. The **app track**
is everything else: bug fixes, Painter redesigns, vibe tuning, new
buttons and menus. Both merge to `main`; the rules that keep them from
colliding:
- Each change is its own PR from the latest `main`; the second to merge
  takes the next `BUILD` / `CACHE` number. Core-track and app-track
  sessions don't share a branch.
- **Free any time:** the look (`css/`, `design/`, icons, themes),
  menus and views that don't touch playback, Library, covers, text.
- **Vibe tuning** (`data/vibes.txt`): free since item 78 (v130): the vibes
  are plain text the core's generator compiles in (`docs/VIBES.md`, "How to
  edit a vibe").
- **Generator changes** (profiles, moods, choices): since item 97 there is
  one generator, the core's (`core/src/generator/`, its data in
  `data/*.txt`), held by the generation goldens. Hands' work, through the
  queue.
- **Audio bugs:** in `core/`, they go through the brain into the core
  track's queue (the hands there own those files); in the JavaScript
  synth, only if testers actually hear the fallback.
- **New controls for core features** (request pickers, export, stereo):
  a Painter session can design them now; they are wired up once the
  core feature exists (requests after phase 2, export in phase 7).
  Wiring the bars and tempo pickers to today's generator first is fine
  if cheap; phase 3 replaces the plumbing anyway.
- **Around phase 3** (the page stops scheduling notes and only sends
  commands), changes to playback controls wait for it, or go through
  the brain.

Only one brain chat edits these files at a time. Long chats get slow and
lose detail; start a fresh brain chat per phase (the words, the Rust port)
and a fresh hands chat per phase too, since the queue file carries
everything a session needs.

**The docs stay small (since 2026-10-26).** Each role loads only its own
live docs (`docs/README.md` is the map), so a doc holds what is live and
nothing else; the history sits in `docs/archive/`, which nothing loads by
default. Move text, don't rewrite decisions.

- **Every brain session ends with an archive pass.** Move what is finished
  out of the live docs: finished items from `docs/QUEUE.md` to
  `docs/archive/queue-items.md`; the notes it has read and answered from
  `docs/notes/` to `docs/archive/notes/`; Done lines older than the current
  phase from `docs/DONE.md` to `docs/archive/done.md`; the session's own log
  entry to the top of `docs/archive/brain-log.md` (newest first); the long
  reasoning behind a decision to `docs/archive/decisions-reasoning.md`,
  leaving the decision itself as one dated line in `docs/DECISIONS.md`. Then
  run `node tools/check-docs.mjs`.
- **The limits,** checked in CI by that script: `AGENTS.md` 6 KB,
  `core/AGENTS.md` 10 KB, `docs/QUEUE.md` 80 KB, `docs/BRAIN.md` 50 KB,
  `docs/DECISIONS.md` 40 KB. Over one, CI fails with "archive the finished
  parts".
- **Each new decision of Mikey's** is one dated line in `docs/DECISIONS.md`
  under its topic, written in his words, the day he made it.

## Getting oriented

- Repo (public): `https://github.com/nowmichaelclark/driftloom`. A PR:
  `git fetch origin pull/N/head:prN`. It moved from `Kruu-Mikey/driftloom`
  on 2026-10-08 with its history, PR numbers and branches.
- Live: `https://driftloom.nowmichaelclark.workers.dev/`. Cloudflare
  Workers, assets only (`wrangler.jsonc`). Push to `main` deploys; only
  testers see it. Every PR gets preview deploys: the Cloudflare bot's PR
  comment has the branch and **commit** preview URLs (use the commit one;
  it never moves), and when the bot doesn't comment, the Workers Builds
  check run's output carries the commit preview URL. An old commit's
  preview stays live, which is what makes A/Bs across merged PRs possible.
  Since the move, PR branches build as Cloudflare **Previews**: the commit
  link is the Deployment URL, `<deployment-id>-driftloom...`; the
  branch's Preview URL moves (see Session 5 in State). Previews from the
  old account's links in the Done lists (`docs/DONE.md`,
  `docs/archive/done.md`) depend on that account.
- The app's Diagnostics panel reports `build` (bump `BUILD` in `js/main.js`
  and `CACHE` in `sw.js` together on every app change), `sw` state, and
  `choir`.
- Read `docs/README.md` (the map of the docs), `docs/DECISIONS.md`,
  `docs/QUEUE.md`, `docs/MOODS.md`, `docs/ALBUMS.md`, `ROADMAP.md` and
  `README.md` (a short front page since 2026-10-08). The engineering notes
  (`docs/archive/engineering-notes.md`, the README's old body) and the
  performance baseline (`docs/archive/perf-baseline.md`) are archive
  reading: open them when a question needs them. `Mikey's Thoughts.md` is
  his.
- Credentials: Mikey pastes a GitHub token in the session's opening
  message. With it the brain commits and merges **docs-only** changes --
  `docs/*.md`, `ROADMAP.md`, `README.md` -- through small PRs it merges
  itself. Once item 38a lands, also **`data/vibes.txt`** (the vibe recipes,
  data only, since item 78 in plain text; bump `BUILD` and `CACHE` with it, and merge only when CI is
  green: a test checks every recipe's names; since item 82 the same PR
  rewrites `core/golden/generation.hashes` and rebuilds `js/dlplayer.wasm`,
  `docs/VIBES.md`). Brain's call, Session 16,
  so tuning a vibe is an edit rather than a hands session. Code stays with the hands. Never write the token into a file, a
  commit, memory, Project knowledge, or a brief. Mikey manages his own
  tokens. Commits are authored as
  `Mikey <162073748+nowmichaelclark@users.noreply.github.com>` (Kruu
  Mikey before 2026-10-08).
- In a claude.ai brain session the GitHub API answers 403 for the repo
  until it is attached to the session (`add_repo` with push access);
  after that `gh api` (REST only, no GraphQL) and pushes work with the
  token. The shell can't reach workers.dev: check the live site with
  WebFetch.
- Things that tripped the hands before: a Claude Code session may push
  only to its one designated branch, so an open draft PR on that branch
  (like the never-to-merge drum knob, #42) blocks the next PR -- close it
  first. The hands can't retry a failed Cloudflare build (one failed with
  no log, #51); Mikey retries it from the check's Details link, or simply
  merges, since main redeploys. Their sandbox can't fetch workers.dev, so
  the brain checks previews. Sessions hit usage limits mid-item; the
  queue and the branch make resuming cheap.
- The brain's container: one core; `/tmp` does not survive between
  sessions; a background run must start with `setsid` or it dies when the
  tool call returns; `measure.mjs` takes a second or two per loop (in Node,
  on the player), so 100 loops is a few minutes. Playwright, for the tools
  that drive the app, is installed globally (`NODE_PATH=$(npm root -g)`).

## Listening albums

The app's **Paste a code** button takes an album code (`DLA1-...`) and
imports every loop in order.

- `js/share.js` exports `encodeAlbum(title, specs)` and `decodeAlbum`, and
  runs in Node. Since item 97 the specs come from the core: in Node,
  `tools/gen-lib.mjs`'s `newSpec(seed, request)` and `render(spec)` (the
  tools' build, `tools/build-core.sh --tools`; filter on `meta`, `choices`),
  or `decodeSong(code)`; encode, and round-trip-check the order. To render
  what the app plays: `tools/play-lib.mjs` (`player`, `render`), in Node.
  To render an old build's app, a worktree of it and its own page in
  Chromium (B1's scratch was a copy of that build's `tools/golden-export.mjs`
  writing the core node's stereo output).
- **Never type a code out.** Print it in tool output and copy it from
  there, then decode what you are about to send. The first brain session
  produced a plausible code from nothing; the app's checksum rejected it.
- **Blind tests:** shuffle with a fixed seed. Prefer albums whose key can
  be read back from the code itself (the loop's mood, voice, profile or
  `developmentOf`), so nothing needs storing. Albums out for listening go
  in `docs/ALBUMS.md` with how to read their key. **Never write a key into
  a public file before Mikey has answered** -- the brain slipped once
  (#35, taken back in #36).
- **A/Bs:** play the same album on two commit previews, labeled X and Y,
  randomly assigned. If the change is synth-only, both previews render the
  same loops. A control track that didn't change is worth including: it
  measures his test-retest noise.
- **Labeled albums** (keep / tune / drop) for new sounds; **blind A/Bs**
  only when a change alters sounds he already likes (the middle path).
- He rates in one word per track, and describes freely when asked.

## What Mikey has decided

Moved to `docs/DECISIONS.md` (2026-10-26): one dated line each, grouped by
topic. The long reasoning is in `docs/archive/decisions-reasoning.md`.

## What he has heard

| album | verdict |
|---|---|
| Choir quiz (seed 4127) | 5/5 found, no false picks |
| Lead audition (blind, 12) | saw, moog, analoglead harsh; musicbox, kalimba, stab fine |
| Drums (blind, 12) | drums within a loop mostly right; two mild flags at the far end of drums-over-music |
| Coming in (14) | never reached up; every complaint a drum loop |
| Shrine bells (5) | melody bells fine; chord temple bell buried (lifted in #106) |
| Missing layers (6) | five "complete": fewer layers are variety |
| Six eight (A/B, 8) | new offbeat and stutter won; old won where chords entered on a weak eighth |
| Six eight, part two (A/B, 8) | rhythm section: keep the cross-rhythm; new hat accents won |
| Drum knob (3 levels x 14) | drum level not the cause; the pluck and saw leads were |
| Lead trims (A/B, 14) | volume reaches 5 -> 0 |
| Softer winds (A/B, 8) | trims kept; the ocarina over drums preferred louder (watch) |
| Fiddle and accordion, take one (6) | "a toy fiddle and accordion played badly by a child" -> rebuilt |
| Review album (19) | take two keep; nylon smeared and pan flute static (both fixed, #74, #75); cinder keep; wayfare drums wrong (#79, then #82: "pretty nice") |
| Depth (blind, 8) | development heard at 24 and 32 bars; weak at 16 (second pass #108) |
| Depth two (blind, 6 x 16 bars) | 4/4 developing "goes somewhere"; simple: 1 goes somewhere, 1 loops (and looping is welcome sometimes) |
| Rust, all voices (10, core on, phone) | "it sounded great!" |
| Phase 1 check (blind A/B, 9, v92 vs v99) | "everything sounds the same on both versions"; no difference he could hear |
| Session 26 (the sound batch, 3 albums + clips, v123) | every new sound and mixing idea kept; bass +3 dB with a kick, today's without; lateFloor's second arrival should vary; stereo vs mono: "same" on 5 of 6 (and the null), stereo "just barely" nicer on the sixth |
| Lite Room Album (item 85c, 6 loops before and after, lite) | "it sounded great for lite mode"; the steal goes to full quality too (item 85e) |
| 87b on his devices (v142 preview) | Moto stays on full (busiest 18-24%, no late notes); tablet moved to lite by itself (75% at full; on lite 55%, 40% of real time); the switch "doesn't bother" him, a crossfade in phase 3; the tablet lost 10 notes at two new loops: folded into phase 3 (the queue's "For phase 3") |
| Phase 2 Blind Album (v128 against v149, 11 pairs, 2026-11-07) | the null pair Same; no pair worse for phase 2 apart from item 84's downward octave moves (old preferred 2 of 3); upward moves new preferred 3 of 3: 84b after phase 3; the stab chords "Fine" in all three |

## Methods that proved themselves

- **Verify every report from the hands**, with the brain's own measure where
  possible. Past misses: `figureRepeat` measured the motif before its
  transforms; a claimed cause was worth a third of what was said; a probe
  "showed the budget coping" and was wrong.
- **Render, don't guess.** The brain's hypotheses were wrong often enough
  to label them "unproven": the loud-and-squeezed theory (#24), "drums too
  loud" (it was the melody voice), all three pan-flute suspects (it was
  grace notes shorter than the attack). Queue items name suspects and ask
  the hands to find the cause by rendering.
- **Group ratings by feature.** The drum complaints resolved when the
  flagged loops were grouped by melody voice.
- **Briefs get taken literally.** "Steady, the same shapes on every kit"
  produced static drums (#79). Say the intent and give yardsticks for
  what must not be lost (variety, dynamics), not only constraints.
- **Targets come from something Mikey likes** -- the flute's 2-5 kHz share
  for tone, tide's hand-kit jig for drum level and variety, kalimba for
  loudness -- not a number the brain guessed.
- **Measure what reaches the ear** -- emitted events and rendered audio --
  not internal state. Short notes (0.4 s, near the app's median melody
  note) calibrate level; long ones flatter or hide ringing voices.
- **Controls and repeats.** Identical audio on two links drew different
  ratings; a control track and patterns consistent across passes separate
  signal from noise.
- **Span is the guard metric** for melodic changes; **move the figure,
  never a note**; **articulation lives in the synth** (drawn from
  `Math.random`), and the composer annotates with derived values only.
- **Seed-derived decisions use a salted RNG** (`spec.seed ^ SALT`) so loops
  that don't take the feature render exactly as before; it keeps A/Bs
  clean.
- **Measure against the same moment before** (brain B3): a join, an arc's
  energy, a wander's move, rendered at the commit before the change and now
  and compared span by span, separates what the plan did from what the loop
  always did; the hands' checks (clicks, steps) passed three holes.
- Single-stage bisects mislead when stages interact. One fixed curve
  applied everywhere becomes a mannerism.

## Tools

- `test/generator.test.mjs` -- the tests; run three times.
- `tools/stats.mjs` -- corpus statistics. `--seed`, `--n`,
  `--lift-low/--lift-high`, `--choir-quiz [file]`, `--voice-codes <voice>`,
  `--check` and `--write-baseline` (the balance lock,
  `test/stats-baseline.json`, n=10000), `--drums <profiles>` (drum
  variety), `--depth` (composition depth, developing vs simple).
- `tools/measure.mjs` -- offline audio on the player, in Node (no
  browser; queue item 96): LUFS, LRA, peak, crest, per-layer levels, punch
  figures, what the master chain did; `--voice all|names` (tone and level
  by layer, `--note` for note length), `--refusals` (`--quality lite`),
  `--profile <id>`, `--vibes`, `--request`, `--vibe-mix`, `--stereo`,
  `--endings` (cut-off notes), `--json`, `--seed`. Same options, same
  numbers, to the bit. `node tools/meter.mjs --selftest` checks the meter.
- `tools/perf.mjs` -- live performance in headless Chromium under CPU
  throttling: audio health, main-thread cost, graph churn, memory, time
  to first sound, page weight, hidden tab; `--ab` for before/after.
  Baseline in `docs/archive/perf-baseline.md`.
- `tools/listen.mjs` -- one-command albums; still not built (roadmap 20).
- `tools/check-docs.mjs` -- the size limits on the live docs (see "The docs
  stay small"); CI runs it.

## State -- 2026-11-14

### Start here (for the next brain)

- **Brain B3 (Opus 5.5, 2026-11-14): item 102 checked, the album out, his
  notes still to come.** On `main` at v164 (88c98be), measured, not taken
  from the reports: module matches (801,267 bytes); goldens 5,135; balance
  lock; 15 short loops bit for bit as at dfe78c3; 27 Rust tests (straight or
  played, 32,768 bars); `longform.mjs --plan` all, a played run all but the
  arc's darker ends (81% of 16, batch 6's slow openings); a section's making
  flat at 1, 1,000 and 10^9. **Three bugs** (to session 16's first PR):
  silent pivots, "alone" over a silent varied tune, developments stuttering
  with a join a unit. **The arc's energy** is 1.5 to 2 LU against the loop;
  its shape is carried by the key a fourth or fifth up from B to the
  release. **Endless's wander** is the old app for its first minutes (the
  old pair 2 was a null: kept as the control). All in
  `docs/notes/102-b3-measured.md`. **The page:** "Long Pieces Album"
  (`docs/ALBUMS.md`; verdicts in its db `verdicts`, the key in `brain/key`).
  **Next brain:** read the verdicts (`db_list` on slug
  2280beaa-39db-4a83-90d6-6becf933d8f0), write `docs/notes/102-mikey.md`
  (his words first, then actions by importance; new composing gets its own
  queue item and session prompt), record his answers in `docs/DECISIONS.md`,
  write the pairs' key into `docs/ALBUMS.md`, delete `long-clips`. B3's
  scratch scripts were not kept; the method (Methods): render the same span
  at dfe78c3 and now, compare unit by unit and join by join.
- **Brain B1 (Opus 5.5, 2026-11-11): the road to 1.0's kickoff.** The plan
  is `docs/sessions/README.md`; each brain session has its prompt there.
  **Checked on `main` at v153 (d9d6ffe), not taken from the reports:** items
  97 and 100 (the queue's item 98 has the figures); the plain lines and the
  too-slow rule: the switch to lite resets the count and skips 5 s of
  warm-up, and the tablet (busiest about 55% of its time on lite) needs to
  run about 1.8 times slower for 2 s in 20 to trip it, so it is safe today;
  the margin is now a rule for every session (the README). **The control
  album** ("Phase 3 Control", `docs/ALBUMS.md`) is out; 28 of 30 renders
  null, two differ after a silent bar (session 01 explains). **The plan
  reviewed:** sessions 11 and 14 on Fable 5.2; the long pieces' brief gained
  the slider and pins rules, time not sections, endings that land,
  movements, three musical ideas, the cost on the audio thread, and the
  listening set as rendered excerpts with a blind before-and-after pair;
  session 31a (the new name) added; smaller fixes in 01, 09, 22, 24, 25, 29,
  30, 31 and B3, each file saying what changed at its top. **Mikey decided**
  (`docs/DECISIONS.md`, 2026-11-11): the two plain lines shorter; the melody
  twang after 1.0; the app is renamed at 1.0, a surprise for the testers, so
  the name stays out of the repo (it is in the Project's private doc
  `claude/new-name-at-1-0.md`, with the brain's plan: domain at Cloudflare
  Registrar, release id from the domain, test builds on a working id).
  **After B1:** hands session 01 done (#360 v154, #361 v155): the stall
  table clean; the control's two loops were the width ceiling after a rest,
  fixed; the brain re-rendered all 30 against v149: every loop -82 dB or
  lower. Underruns can only be answered by the phones (no `playoutStats`
  in Chromium 153): `docs/notes/98-testers.md`. Session 02 done (#363,
  v156, run on Mikey's own machine): the brain checked that all 751 moved
  golden cases (of 5,132) draw a new option (slow 415, deep 195, wide 117,
  fast 45) and none that draws one stayed; goldens and balance lock pass;
  the refusals moved a little (bass asked 1,368 to 1,352; lite chords
  refused 1,187 to 1,165), from slow openings, nothing worse. Its three
  choices (how often, the calm vibes, deepDuck's depth) wait for Mikey's
  ears at B2. Session 03 done (#366 to #369, v159): the brain checked that
  the stack fix (#368) moved no golden, seed or baseline; that against
  v156's generator every one of 5,132 cases is the same apart from padWow's
  choice (on in 776) and the load's tuning; tuningOffset on 168 of 1,000
  corpus loops, 15 to 35 cents; goldens and balance lock pass. padWow moves
  three baseline loops' mix past 0.2 LU, as the approved sketch does: for
  Mikey's ears at B2. **Order changed 2026-11-12** (Mikey): the long pieces
  (11 to 15, 11 and 14 on Fable) before batch 6's 04 to 10. **Next:**
  session 11. Session 11 done (#373, v160): the brain checked the goldens
  unchanged (5,135), `tools/longform.mjs` all passing, and read maps: arcs
  sound in shape; the wander's longest stretch with nothing moving reached
  about 400 s (to session 12); a 3,072-bar piece was 21 alike movements, so
  movements differ in kind (Mikey, to session 14). Session 12 done (#375,
  v161): goldens unchanged, the module matches its source; the longest
  stretch with nothing moving now 300 s (was 587); the stall test clean on
  two wander pieces in Chromium (autolite's slow case can't ask a journey
  yet). The wander's tune hand-over waits for theme B: turned on in session
  14. Session 13 done (#377, v162): two yardsticks short (brightness 87%
  of 9 in 10; one 2.7 LU step); Mikey merged it as it is. The brain checked
  main after: module matches, goldens 5,135 of 5,135. The step goes to
  session 15. Session 14 (#379, v163): brightness order 77% (9 in 10
  asked), from the variations by design; the brain checked CI all green on
  its last push (baselines included) and every moved golden (120) a long
  piece or Endless; recommended merging; Mikey merges. Session 15 judges
  brightness on the opening, the peak and the close. **Next:** session 15,
  then brain B3.
- **Not done in B1:** the commit previews couldn't be fetched from here (no
  approval for WebFetch while Mikey was away); the v149 link in the album is
  from `docs/DONE.md` (old commit previews stay live). Cargo's own tests were
  left to CI (green on `main`).
- Sessions 29 and 30's "Start here" bullets (phase 2's proof, phase 3's
  items, the road to 1.0 written): `docs/archive/brain-log.md`, "Start here
  before B1".
- **Waiting on Mikey, not urgent:** the Full Room Album (85e,
  <https://claude.ai/artifact/NmfNUM1hj6buNUHKZzG9of>, on his second
  account); the blind album's in-app checklist for bars and length (88b),
  which he left blank.
- **Ears:** Ears 10, 10b and 11 done (piles 4 and 5, #314, #316, #326),
  on Mikey's second account; Ears 12 done (#329, 2026-11-07: Synthesist,
  Animal Crossing and three Nonlocal Forecast cards); Ears 12b done (#332,
  2026-11-08: *Triangular Reformat*, *Pyramid*, six more K.K. songs, and
  `theory.py`'s colors reading past the seventh). Ears 13 (the second
  synthesis) done in a cloud session (#336, 2026-11-08): batch 6, twelve
  candidates; five sketched on the core (lowBell, padWow, colorTone,
  deepDuck, wideSweep), clips on the throwaway branch `ears13-clips`
  (deleted; Mikey approved all five and the sampler, 2026-11-08, both
  versions where there are two); seven wait on hooks, queued as item 99
  (the sketch build only), then an Ears 13b sketches them. The approved
  batch is built after phase 3, the second synthesis (Opus 5.5, high).
  Mikey asked (2026-11-07) whether the ears must finish by a phase: no; Ears
  12-13 by the end of phase 3, for the batch after it; piles 7-18 have no
  deadline (phase 5's block catalogue, long-form composing, phase 8).
- **Broken Friends:** `docs/notes/82-for-broken-friends.md` is for the game's
  next session (repin the core from f654fdb, the calls, the hashes to
  refresh). It stays in `docs/notes/` until the game's session has used it;
  nothing for the brain to answer. As of 2026-11-01 the game still pins
  f654fdb, and Mikey is not concerned (the game is early; it updates later).
- **Vibe edits since item 82** move the generation goldens and
  `js/dlplayer.wasm` too: `docs/VIBES.md`, "How to edit a vibe". This
  container runs Node, the pinned Rust (`rustup` installs it from
  `core/rust-toolchain.toml`) and Chromium (`/opt/pw-browsers`), so the
  brain does both steps itself in a vibe PR.
- **Next picks:** the design's two open questions were answered by Mikey on
  2026-10-14 (`design/next-picks.md`, "Decided": leave an album to see the
  picks; an any key in each picker; the key reads "vibe"). Next Painter work:
  the tempo, key and scale, style and voices pickers and the "can't do that"
  line; long-piece display (`docs/PAINTER.md`, section 2) before item 53.
- **Ears:** the pause (until Session 26's album was heard) is over; it was
  heard 2026-10-29. Item 83 gives them sketches and clips on the core.
- **New profiles and sounds** go in with Ears 13's batch, after phase 3 and
  before phase 4 (one generator by then). **Before phase 5:** the block
  catalogue for Mikey (charter, promise 5).
- **Still open, not urgent:** the Pixel 10a's 1.3-1.8 s main-thread stalls
  (item 41) were never reproduced, and no report has come since v92; item
  75 (the reverb, rate-true) for a session with usage to spare; item 37 on
  hold.
- Older "Start here" bullets (Sessions 18-26 and the plan to phase 2), as
  they stood: `docs/archive/brain-log.md`, "Start here before Session 28";
  Session 28's own bullet is its log entry.

### The log

Moved to `docs/archive/brain-log.md` (2026-10-26), newest first. A brain
session's own entry goes there at its archive pass.

## History

Moved to `docs/archive/brain-log.md`, below the log.
