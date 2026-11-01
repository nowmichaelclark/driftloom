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
- **Vibe tuning** (`data/vibes.txt`): free again since item 78 (v130): the
  vibes are plain text both generators read, so tuning never waits for the
  port (`docs/VIBES.md`, "How to edit a vibe").
- **Generator changes** (`js/generator.js`, profiles, moods): since the
  port (items 77-82), made in both, `js/` and `core/src/generator/`, in one
  PR, held together by the generation goldens; Rust only once phase 3 retires
  the JavaScript generator. Hands' work, through the queue.
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
  tool call returns; `measure.mjs` takes a few seconds per loop, so 100
  loops is a background job. Playwright is installed globally
  (`NODE_PATH=$(npm root -g)`).

## Listening albums

The app's **Paste a code** button takes an album code (`DLA1-...`) and
imports every loop in order.

- `js/share.js` exports `encodeAlbum(title, specs)` and `decodeAlbum`.
  Build specs with `newSpec()`, filter on `render(spec)` (e.g.
  `meta.melodyVoice`, `meta.kit`, `developmentOf(spec)`), encode, and
  round-trip-check the order.
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
- Single-stage bisects mislead when stages interact. One fixed curve
  applied everywhere becomes a mannerism.

## Tools

- `test/generator.test.mjs` -- the tests; run three times.
- `tools/stats.mjs` -- corpus statistics. `--seed`, `--n`,
  `--lift-low/--lift-high`, `--choir-quiz [file]`, `--voice-codes <voice>`,
  `--check` and `--write-baseline` (the balance lock,
  `test/stats-baseline.json`, n=10000), `--drums <profiles>` (drum
  variety), `--depth` (composition depth, developing vs simple).
- `tools/measure.mjs` -- offline audio through the real Engine and Synth in
  headless Chromium: peak, RMS, LUFS, LRA, crest, per-layer levels,
  punch figures, drums over the music; `--voice all|names` (tone and
  level by layer, `--note` for note length), `--refusals [code]`
  (`--quality lite`), `--profile <id>`, `--endings` (cut-off notes),
  `--retire` (voices letting go changes nothing), `--selftest`,
  `--json`, `--jobs`, `--seed`. Seeded: same seed, same numbers.
- `tools/perf.mjs` -- live performance in headless Chromium under CPU
  throttling: audio health, main-thread cost, graph churn, memory, time
  to first sound, page weight, hidden tab; `--ab` for before/after.
  Baseline in `docs/archive/perf-baseline.md`.
- `tools/listen.mjs` -- one-command albums; still not built (roadmap 20).
- `tools/check-docs.mjs` -- the size limits on the live docs (see "The docs
  stay small"); CI runs it.

## State -- 2026-11-01

### Start here (for the next brain)

- **Session 29 (brain, Opus 5.5, 2026-11-01): phase 2 checked through item
  85 (83, 84, 85a-d, #298-#305); it passes.** `main` at v137 (82770b0). What
  the brain ran: the log (`docs/archive/brain-log.md`). Item 85 (with 85c and
  48's brief) and the note `85-fast-pins-lite.md` archived; its refusal
  reading is now a standing rule. **Next: the hands take 86, then 87.** **The
  brain's next check: after item 87**, as planned (the game's four needs);
  then the phase's end after 92.
- **Open, for Mikey (asked 2026-11-01):** the Lite Room Album (#303's Done
  line) not yet heard as far as the brain knows: the steal's 0.05 s fade,
  and **whether full quality steals too** (at full quality the core still
  refuses 986 of 5,056 chord notes, 19.5%, over the baseline's 100 loops;
  lite now 1,292, 25.6%). And a small theme edge: a theme whose own melody
  rests, bent by vibes that bring a choir, sings a tune of its own (2 of the
  brain's 600 theme cases, `docs/archive/brain-log.md`).
- **Broken Friends:** `docs/notes/82-for-broken-friends.md` is for the game's
  next session (repin the core from f654fdb, the calls, the hashes to
  refresh). It stays in `docs/notes/` until the game's session has used it;
  nothing for the brain to answer. As of 2026-11-01 the game still pins
  f654fdb; the brain's suggestion is one repin after item 87 (the change at a
  moment and rendering ahead both land then), not one now and one later.
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
- **New profiles** wait until phase 2 lands, then go in as data (before
  phase 3 and Broken Friends). **Before phase 5:** the block catalogue for
  Mikey (charter, promise 5).
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
