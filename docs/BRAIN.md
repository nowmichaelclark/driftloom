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
- **Generator changes** (`js/generator.js`, profiles, moods): fine
  until phase 2, then made in Rust only (otherwise made twice).
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
  green: a test checks every recipe's names). Brain's call, Session 16,
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

## State -- 2026-10-13

### Start here (for the next brain)

- **Session 26 (brain, Opus 5.5, 2026-10-26): done; Mikey's verdicts
  pending.** The sound batch (46b-73) checked: it passes (the log,
  `docs/archive/brain-log.md`). Item 72's big baseline moves are the
  reshuffle: shil-doask, reisk-foa and their-roask drew lateFloor at item 71
  and not at 72, and every loop whose choices didn't change is bit for bit
  unchanged. **The albums** (`docs/ALBUMS.md`, "Session 26"): three in-app
  codes and four sets of clips on one private page whose verdicts save to its
  database. **Read them back** with `ArtifactData` (`list`, collection
  `verdicts`, the page's URL in `docs/ALBUMS.md`); each verdict becomes a
  queue item ahead of 77 (tune and drop) or nothing (keep); the bass pick
  sets `LIFT_DB`; the blind A/B's key is in `docs/ALBUMS.md`. **Phase 2
  written** (items 77-92, #275), with Mikey's three decisions of the day:
  phase 2 takes the sequencer, the port is proven note for note, glacial
  (76b). **Next:** the hands take 76, 76b, 74, then 77. The brain's next
  check: after item 82 (the player), or sooner if the verdicts call for one.
  Done lines #241-#274 stay live until the verdicts are in, then go to the
  archive.
- **Since Session 26 (brain, 2026-10-29):** item 76 merged (#277, v124;
  archived with its note). Mikey took the vibes to 88: queue item 76b now
  holds glacial and twenty more, four dropped (`docs/VIBES.md`, "The vibes
  to 88"; `docs/DECISIONS.md`). And no compatibility bloat for old codes.
  **The album was heard the same day** (`docs/ALBUMS.md`): everything kept;
  two tunes queued as 76c (the bass level) and 76d (lateFloor's second
  arrival). Done lines #241-#274 archived.
- **2026-10-30:** 76b, 76c, 76d and 74 merged (#280-#285, v125-v128) and
  archived. **Next: phase 2 from item 77** (hands, Opus 5.5, high). The
  brain's next check: after item 82, or sooner if a note comes in.
- **Since 2026-10-26 (hands, docs restructure and item 76).** All three
  groups of the sound batch are merged (items 54-73, #245-#272, v103-v123;
  group (b) is items 64-66, group (c) 67-73), and the album loops in the
  Done lines were re-checked against `main` after item 73 (#273). `main` is
  at v123. **Item 76** is queued from Mikey's Moto Diagnostics (v116,
  2026-10-25): the first loop after the app starts loses its first notes
  (app track, Sonnet 5.5, high). The docs were split by role: this file
  keeps the orientation and this "Start here"; the decisions are
  `docs/DECISIONS.md`; the log, History, finished items, old notes and Done
  lines are in `docs/archive/`; `docs/README.md` is the map. **For Session
  26:** end the session with the archive pass (see "The docs stay small").
- **The plan to phase 2 (Mikey, 2026-10-15).** He wants more
  instruments, profiles and vibes before the Broken Friends step, with ears
  sessions to find them. Timed so nothing is built twice:
  - **Now, alongside everything:** ears sessions (records he loves ->
    candidate sounds as offline clips; his verdicts, including on
    dubStab, organ, wash and longChords). Vibes may be added until phase
    2 starts (data in `data/vibes.txt`).
  - **Session 22** (brain, Opus, 2026-10-18): **done.** Items 42-45
    verified, 47's PR read; the control album "Phase 1 check" heard by
    Mikey the same day: no difference. See the log
    (`docs/archive/brain-log.md`); one known issue for phase 2
    (wander across the slider's upper range should not repeat; `docs/QUEUE.md`,
    "Known issues").
  - **Session 23** (hands, Opus, high): item 46, the building-block
    interface. **Done** (#238, v100).
  - **Session 24** (brain, Opus 5.5, 2026-10-19): **done.** Item 46
    checked by the brain (passes; phase 1 is done); **item 46b** queued
    (modulation input, stereo signal shape, channel events and a
    channel's own blocks; bit-identical); **the sound batch** written as
    items 54-73, with item 74 (clippy) low priority; the wander note
    carried into the charter's phase 2. See the log
    (`docs/archive/brain-log.md`). As planned before it: write **the sound
    batch** into the queue, three groups, from `docs/ears/`
    (batch-4 and batch-5 and Mikey's verdicts):
    (a) **instruments**: dubStab, longChords, wash, the chapel organ,
    roundBass, glass, floorDrop, arc; built in the core on 46's
    interface, core-only (the JavaScript-synth fallback substitutes an
    existing voice), each measured for level, cost and tone against its
    ears reference;
    (b) **stereo in the core**: the approved stereo baseline, the mono
    switch, the first space recipes (charter promise 9);
    (c) **the mixing logic** (Ears 8, batch 5): tunedKick, lateFloor,
    floorDrop shapes, pedalBass, subDuck, padSweep, echoTime, under
    Mikey's rules (today's behavior stays one option in every choice;
    seeded per-loop choices; most loops get one or two changes; subDuck
    only with a kick and a sub; stagger and stutter only with a bass
    line; drop style may follow the bar length; a wandering loop may
    swap the kick at section boundaries with a crossfade or a wait).
    **Where it lands (brain, 2026-10-18, answering Ears 8's question):**
    the choices go in today's JavaScript generator, which is allowed
    until phase 2 starts and then ported with everything else (they
    become part of what the port must match); what they play (the tuned
    kick and boom, the bass-only duck, the chord-layer sweep, the echo
    time as a parameter) goes in the core only, after item 46, so
    nothing collides with 46. One builder session for both halves.
    Then phase 2's items (ordering 48-53 among them). The balance lock
    is re-baselined on purpose, once per group, and says what moved.
  - **Since Session 24 (2026-10-19 to 22), for the next brain:** 46b
    split into 46b and 46c; items 54-63 merged (#245-#256, v103-v112),
    so group (a) is done and the balance lock is re-baselined. Decided
    on the way: glass follows its chord's velocity times chords a bar,
    and rings only on chords of two or more notes (Mikey); a 0.4 dB rate
    tolerance for golden lists holding a steady tone through the reverb,
    with **item 75** (the reverb rate-true for steady tones, low
    priority) and a rule to re-check every Done line's album loops at
    the end of group (c) (brain). **The arc (Mikey, 2026-10-21):** a
    loop of his sank under 700 Hz for most of a 3-minute swing and he
    thought the instruments were gone; once he knew it returns he liked
    it. So **item 63b**: two shapes drawn per loop, "deep" (today's,
    about 1 in 3) and "dip" (mostly open, one gentle dip to 800 Hz-1.5
    kHz, about 2 in 3), with vibe weights (calm and vast vibes lean deep
    and long; bright and lively lean dip, short and rarer). **And Mikey
    (2026-10-22):** the arc's variables (shape, period, floor, start,
    how often) should become controls for listeners and game developers
    eventually; for now the generator weights them. So 63b keeps them as
    named, ranged settings (promise 9: every variable a named setting),
    and phase 2's requests (promise 11) can pin them; sliders in the app
    are his call later (a Painter design), game hosts reach them through
    the host API (phases 4 and 7). **For Session 26:** a "glacial" vibe
    (Mikey's idea: very slow and vast; the deep arc, the longest swings),
    designed with him there. **Two accounts (Mikey, 2026-10-22):** this
    Claude account ran low for the week, so the hands sessions (63b,
    group b, group c) and Ears 9 may run on his other account. The repo
    carries the state: only one brain edits these files at a time, and a
    hands session that hits a taste call takes the conservative option,
    writes it down "for Mikey's ears", and goes on (decisions batch up
    for Session 26), unless the item says to stop.
  - **Session 25-0** (hands, Opus 5.5, high): item 46b. **Done (#241,
    v101, 2026-10-19)**: every yardstick met; the stereo shape is a second
    signal beside the mono one, and the web `.wasm` rebuilt without the
    stereo code is the same bytes, so mono pays nothing.
  - **Item 46c** (brain, 2026-10-19, from 46b's notes): channel blocks
    with per-loop settings and modulation, events that reach a held note
    every time (the organ's retune), cross-channel signals (subDuck).
    Items 61, 62, 63, 68 and 72 depend on it. Session 25a takes it first.
  - **Session 25a / 25b / 25c** (hands, Opus 5.5, high): the three
    groups in that order (a: items 54-63; b: 64-66; c: 67-73), one PR per
    item; a new session per group (or when usage runs out), same brief.
    Item 74 whenever a session has usage to spare (Sonnet is enough).
  - **Session 26** (brain, Opus; **done 2026-10-26**, see the first bullet): check all three groups (the hands' notes
    on items 55-73 are in `docs/archive/notes.md`; their Done lines in
    `docs/DONE.md`); a labeled
    album (keep / tune / drop) of the new sounds and mixing, and a blind
    stereo-vs-mono A/B; **the bass level** (item 56, #248, Mikey chose
    today's level for now, 2026-10-20): the same few loops with the bass
    at today's level, +3 dB and +5.5 dB (the prototype's), labeled, his
    pick sets the per-voice trims; in the same album, item 56's wander
    no longer folding the bass's passing tones down; new vibes with Mikey, last before the port.
    **Then write phase 2's items** (not written yet): the generator's
    port, ordering 48-53 among them, with the chords' octave fix and the
    wander's upper range that must not repeat (both in the charter's
    phase 2 and in "Known issues"), and item 54's per-loop choices and
    the batch's generator choices as part of what the port must match.
  - **Open for Mikey (Session 24's taste calls, in the queue "for his
    ears"):** the new parts (glass, dubStab, the organ, the wash, the
    arc) count toward "most loops get one or two changes" (item 54: none
    15%, one 45%, two 30%, three 10%), so each is in fewer loops than in
    the ears' clips; the arc starts each loop bright, not closed (item
    63). His answer moves a weight, not the design.
  - **Ears cadence (brain's advice, 2026-10-18):** after Ears 9, pause
    new piles until the batch is built and heard in the app. The
    approved backlog is large, and sketches can sound different in the
    app than as clips (the first ears session's lesson); hearing it in
    the app first makes later syntheses sharper and saves tokens.
    **Agreed by Mikey, 2026-10-18:** Ears 10 on wait until Session 26's
    album has been heard.
  - **Session 27** (hands): the first item of phase 2.
  - **New profiles wait until phase 2 lands**, then go in as data
    (before phase 3 and Broken Friends). Phase order in the charter unchanged.
- **Next picks (2026-10-13):** Mikey chose the design
  (`design/next-picks.md`, from Painter 5, PR #205); see "Next picks,
  the design" under "The core". Painter 6 (app track) is building the
  screens now. Real wiring queued as items 48-52 (phase 2) and 53
  (phase 3), for the brain to order among phase 2's own items when it
  writes them (Session 26, after the sound batch; moved from Session 24). **Waiting on Mikey:** the two open questions (the way in
  during an album; setting only length or only tempo to "any"). **Next
  Painter work:** the tempo, key and scale, style and voices pickers,
  and the "can't do that" line; long-piece display (`docs/PAINTER.md`,
  section 2) before item 53.
- **Where it is:** `main` at v100 (2026-10-19; phase 1's items 42-47
  merged and checked, Painter 6's next-picks screens). The Rust core plays by default (the
  JavaScript synth is the fallback, `?engine=js`); Vibes are in the app
  (items 38a-b, v86-87; recipes in `data/vibes.txt`, tuned once, v91); the
  dials, the word fit and the word-album builder are tools (items 34-35);
  cinder and shatter borrow bright modes when glad (item 36).
- **Glitches on GrapheneOS phones (items 39-41, v89-v92):** the core now
  gets notes 2-8 s ahead, planned from the phone's measured render-ahead;
  late notes past 30 ms are dropped, not bunched; covers are drawn in a
  Worker. **Open:** the Pixel 10a's 1.3-1.8 s main-thread stalls were never
  reproduced; the tester's next Diagnostics (v92: `gpu`, render-ahead,
  late and dropped by cause) will say whether anything is left. If it
  calls for a fix, that comes before the core work.
- **Item 45 stopped for decisions (#213, draft); decided by Mikey
  2026-10-14** (written under item 45 in the queue): tone tolerance 1.5
  points at 44.1/22.05 kHz; the noise made rate-true (baseline update);
  192 kHz native-only; PowerPC CI runs plain goldens only. Items 42-44
  merged (44 is #211).
- **Emergency (2026-10-13): queue item 47**, glitches when minimized
  or locked (Pixel 9, GrapheneOS, v92). All 10 late notes "inside the
  buffer", none from stalls; render-ahead usually ~20 ms but 1.4 s at
  worst. Brain's lead: Android deepens the output buffer in the
  background and the planner only adapts after measuring. App track,
  runs alongside the core track (outside `core/`). Hands: Opus 5.5, high.
- **The core charter is approved (Session 18, 2026-10-13):
  `docs/CORE.md`.** Phase 1 is queued: items 42-46 in `docs/QUEUE.md`.
  Hands, by item: 42 Sonnet (Session 19), 43 Opus (20), 44-45 Sonnet
  (21), 46 Opus (23), all high. **Brain checks, to save tokens, at
  two points rather than every PR:** Session 22, after 45 (goldens
  equal on all five targets, the moved draws' list and ranges, the
  baseline, rates and block sizes; then the control album for Mikey);
  Session 24, after 46 (phase review, then phase 2's items). Each a
  new brain chat, Opus 5.5. Session 18 closed after this. Before phase 5: the block
  catalogue for Mikey (charter, promise 5).
- **Next: the core** -- a new brain chat. Read **"The core: aims and design"**
  in `docs/DECISIONS.md` first: the number one priority, shoot for the moon,
  embeddable anywhere (game engines, mods, any OS or device, iOS and
  macOS kept easy), notes decided and played away from the page so the
  website never glitches from screen work, instruments as Rust building
  blocks plus data recipes, short plain-ASCII share codes, anything may
  break. **Start by drafting the core charter for Mikey's approval**: what
  the core promises, how each promise is measured, and the phases. The
  platforms (F-Droid, itch.io, later Google Play) are later phases of
  their own.
- **Not brain-verified** (Mikey chose to save tokens): #164 and #166, and
  items 38-41's PRs (the hands' reports are in them).
- **From the ears, for the generator port (Ears 4, 2026-10-16; Mikey
  asked that the brain hear of it): the chords' octave can flip.**
  `separateRegisters` (`js/generator.js`) moves the keys down an octave
  when the mean pitch of the *sounding* chord notes is within five
  semitones of the melody's mean. That mean depends on which chord
  events survive the form, rests and gaps, so a change that doesn't
  touch the harmony can still push it across the line: holding chords
  longer (the ears' longChords sketch) flipped the octave in 11 of 87
  held loops (200 scanned), and Mikey heard those loops' chords as too
  loud and rough (grove #47's pads jumped from 104 Hz to 208 Hz at the
  bottom). Pins and requests (promise 11: length, tempo, voices) are
  exactly that kind of change. **Wanted in phase 2, not after it:**
  decide the octave from the progression itself (its chords and voicing
  before form, rests and gaps) rather than from the events, so pinning or
  varying anything else never moves it; and a corpus check that varies
  one non-harmonic choice per loop and counts octave flips (0 expected).
  Today's loops render as they always have (the flip needs a change to
  happen); the fix itself will move the octave in the few loops where the
  two ways of deciding disagree, which the phase's blind album covers; the ears' sketch fix and numbers: `docs/ears/batch-4.md`,
  "Round 4", and `docs/EARS.md`. Written into the charter's phase 2.
- **Waiting, not urgent:** item 37 (on hold; Vibes cover it); the
  ears-approved sounds -- roundBass, glass, floorDrop, arc (2026-10-10);
  **dubStab and longChords approved 2026-10-16** (as in Ears 4's round 4,
  `tools/ears/sketches/ears4-sketch.patch`), and **stereo's baseline
  approved** (the blend of room and ensemble, steady; moving stereo kept
  for a later, experimental option); **the wash and the chapel organ
  approved the same day** (Ears 4's rounds 5 and 7; every recipe, with
  levels, costs and checks, in `docs/ears/batch-4.md`, the sketch in
  `tools/ears/sketches/ears4-sketch.patch`; the organ belongs outside
  the voice budget as fixed overhead, the stab is a budget probe that
  holds nothing) -- under the charter they may become the first
  instruments built as recipes; vibe tuning from the loops Mikey sends
  (`data/vibes.txt`, the brain's to edit).

### The log

Moved to `docs/archive/brain-log.md` (2026-10-26), newest first. A brain
session's own entry goes there at its archive pass.

## History

Moved to `docs/archive/brain-log.md`, below the log.
