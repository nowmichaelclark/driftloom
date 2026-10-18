# Ears sessions: what they're for

Written 2026-10-10 at the end of the first one (Ears 1, on the other
account: the chat that produced `docs/ears/batch-1.md`, `batch-2.md`,
`prototypes.md`, `tools.md` and `data.csv`; they lived in the Project
until 2026-10-17, when they moved into the repo). Updated 2026-10-15 in
Ears 2 for the stereo decision, the album list and Mikey's answers, in
Ears 3 for the card tools, in Ears 4 (2026-10-16) for the tune round's
outcome and lessons, and in Ears 5 (2026-10-17) for the repo rule. Ears 1 also built sketches into the app, which went wrong (see
below); its reports are sound, its app changes are not a model. Read this
first in any new ears session, then `docs/ears/plan.md`.

**The rule: ears outputs live in the repo (Mikey, 2026-10-17).** Everything
an ears session writes goes into this repo, in `docs/ears/`, in a PR, the
same session it is written: album cards (`docs/ears/cards-<pile>.md`),
batch reports (`docs/ears/batch-N.md`), the plan and this guide, measurement
data, tool write-ups, sketch notes. The scripts and the sketch patch go in
`tools/ears/` as before. Nothing is kept only in the Project: the repo is
the source of truth, anyone can read it without the Project, and git keeps
the history. A copy in the Project is optional (it is handy for search, and
the Project syncs from the repo's main branch anyway); if one is made, the
repo copy wins any disagreement. A session that finds something living only
in the Project (a doc, a number, a decision) moves it into the repo before
it ends. Docs stay in `docs/ears/`, not the app, core or `design/` folders;
ears sessions still never put code into the app or core.

**Keep the raw measurements too (Mikey, 2026-10-18).** The cards keep only a
summary; the per-track JSON (`pick.json`, `events.json`, `techniques.json`,
`theory.json`, `space.json`, `stem_check.json`, `split.json`), `progress.log`
and the unedited draft `card.md` are what a later synthesis can re-query. At
the end of a session, tar.gz them (everything in the workdir except audio,
stems and the `joined/` wavs, about 3.5 MB for six albums) into
`docs/ears/raw/<pile-and-half>-raw.tar.gz` and add a line to
`docs/ears/raw/README.md`, in the same PR as the cards. Stems and audio are
never committed (about 1 GB an album; `split.py` rebuilds them). Pile 1's raw
files were not kept (the workspace forgot them); Ears 6 onward are.

**The card tools exist (Ears 3, 2026-10-15; PR #221).** `tools/ears/album.py`
runs pick, split, events, techniques, theory, space and the card writer for
one album and drafts the card, with **[JUDGMENT]** marking what a person or
a later session must write (hook, vibes, wholesome take, why a thing waits).
Test cards: `docs/ears/cards-test.md`. How to run one: `tools/ears/README.md`.
Cost on 2 CPU cores: about 30 minutes of machine time for a four-track
album (Demucs about 1.0 to 1.3 times the audio length, then about 2.5
minutes of measuring a track), so start it in the background and poll with
long sleeps. Every measure is calibrated on test signals
(`python3 -I calibrate.py`, 163 of 163 graded checks, 10 limit rows);
`docs/EARS.md` says where each stops being reliable.

**The tune round is done (Ears 4, 2026-10-16; `docs/ears/batch-4.md`).**
Every waiting sketch is approved: dubStab, longChords, the wash, and one
organ, the **chapel organ**; plus **stereo's baseline**, a steady blend of
a smooth room and an ensemble spread (moving stereo kept for a later,
experimental session). The recipes are in
`tools/ears/sketches/ears4-sketch.patch` (an offline sketch, never app
code), with levels, costs and checks in the batch doc. A generator
finding went to the brain at Mikey's request: the chords' octave can flip
when anything moves the sounding chord centre (`docs/BRAIN.md`, the
charter's phase 2).

In the repo: `docs/EARS.md` (the findings and yardsticks, for the brain
and hands) and `tools/ears/` (the scripts), merged in #158. Mikey lets
ears sessions update docs and tools in the repo (2026-10-10), not the
app code; keep both current at the end of each session.

**Stereo across the board, with a Mono switch (Mikey, 2026-10-15;
replaces "mono by default" of 2026-10-13 and "mono only" of
2026-10-11).** `docs/CORE.md` promise 9. Ears sessions propose stereo
treatments ("space recipes") as offline clips, stereo against today's
mono, before anything is built. Guards: the stereo render folded to mono
sounds like the mono render (level and tone within stated limits; Ears 2
proposed the limits in `docs/ears/batch-3.md`); nothing essential on
one side only; bass and kick centered; stereo must not double the cost on
a phone; blind A/B against mono for sounds he already likes. The core's
stereo render, the Mono switch and the first space recipes come after
the instrument batch, before phase 2. Ears 4's approved baseline is the
first recipe.

**The album list (Mikey, 2026-10-15).** Mikey's full list is about 73
records (`docs/ears/plan.md`). He wants a few things from every one of
them: sounds and instruments (several from some albums), techniques,
compositional ideas and music theory. The goal: a listener now and then
thinks "this reminds me of that album." Everything Driftloom does today
stays possible; the albums widen it.

**Wholesome, but no longer coffee-shop only (Mikey, 2026-10-15).**
Driftloom never brings a mood down or sounds depressing. "Pleasant in a
coffee shop" is no longer the bar for every loop; a listener who wants
that picks a vibe for it. Harsh or dark albums are fair game for sounds
and ideas ("even a harsh album can have wonderful sounding instruments
and compositions"): take what is wholesome from them, leave the rest.
This replaces "mark harsh albums as technique-only." Harsher sounds sit
behind particular vibes, by weight, never by ban: they shouldn't turn up
at random under "happy" and "peaceful", but any vibe may get a brief or
subtle touch, and open vibes like "zany" take almost anything. Never real
words in voices; no quotes, public domain included (all Mikey,
2026-10-15).

**Now (2026-10-15):** Mikey wants new instruments (and later profiles)
before the Luanti step. Ears sessions find candidates now; the sounds he
approves are built in the core after queue item 46 (`docs/BRAIN.md`,
"The plan to phase 2"). For each candidate: a reference track, a
measurement, an offline clip, what it would add, and a yardstick. Write
each session's picks to `docs/ears/batch-N.md` in the repo with a
short list at the top: approved / tune / dropped, so the brain can turn
it into queue items.

## The three kinds of session

- **Ears:** listens by measuring. Takes records Mikey loves and Driftloom
  renders, measures both the same way, and says where they differ, with
  numbers and a yardstick for each difference. Renders before/after clips
  of Driftloom for Mikey to hear. Since 2026-10-15 also writes album
  cards (sounds, techniques, theory per album; `docs/ears/plan.md`).
- **Brain:** decides. Turns ears findings into queue items, sequences them
  around other work (the core phases), verifies the hands' PRs, owns
  `docs/BRAIN.md` and `docs/QUEUE.md`.
- **Hands:** builds. Claude Code sessions working through the queue, with
  the level, refusal and balance-lock checks.

Ears sessions are labeled Ears 1, Ears 2, ... and don't take numbers from
the brain and hands counter (Session 1, 2, ... in `docs/BRAIN.md`).

## What ears sessions are good at

- **Measuring records against Driftloom:** stereo width, energy by band,
  tails (how long notes ring), chord changes a minute, tuning, wobble,
  beat clarity, repetition, how far the sound travels over a minute,
  floor drops and full stops. The tools are in `tools/ears/`; the numbers
  for 32 albums and 104 Driftloom loops are in `docs/ears/data.csv`.
- **Finding the big gaps,** each with a yardstick a hands session can
  test against. Batches one and two found six: stereo, the low end and
  the missing 1–5 kHz, chords changing too often, loops that circle,
  short tails, full stops where records drop the floor.
- **Reading spectrograms** of chosen tracks to describe *how* a record
  does something (where a filter sweeps, when the kick drops out).
- **Picking reference tracks** for a sound before it's built (see below).
- **Rendering clips** of Driftloom offline, before and after a change,
  loudness-matched, so Mikey can judge by ear without opening the app.
- **Checking a change by measurement:** render with and without it, and
  measure what moved, including what shouldn't have (refused notes per
  voice, each layer's own level from stems: `render2.mjs` and
  `sketch_check.py`).

## What they're not for

- **Judging whether something sounds good.** That's Mikey. Numbers say
  what records do, not whether it's pleasant.
- **Building features into the app.** The first ears session did, on a
  draft branch (`ears/prototypes`, PR #152, now closed), and in the live
  preview old sounds went missing. The offline clips sounded fine; the
  app didn't. An ears session's sketches stay as offline clips and
  written specs (a local sketch of the JavaScript synth is fine for
  rendering clips; it is never pushed as app code); the hands build them,
  from the queue, with the usual checks.
- **The queue.** Findings go to the repo (`docs/ears/`); the brain writes queue items.

## Lessons from the first sessions

- Clips rendered one loop at a time don't show what happens across loads,
  rerolls or a long listen, or what a new layer takes from the voice
  budget. Any sketch must be checked for refused notes and for every
  layer still sounding, not only for what it adds.
- Every new layer needs a level relative to the loop, not a fixed one: a
  quiet, sparse loop can be drowned by it, and a busy one can bury it
  (Ears 2: the wash peaks 7 dB under a sparse loop and 28 dB under a
  beat).
- One fixed value for a whole catalog shrinks variety (long chords first
  held every loop's chords for 8 s, and put 48% of loops on one chord).
  Draw per loop from a spread.
- Check claims against the data before publishing. Batch one overstated
  the tuning difference; a separate fact-check caught two errors in batch
  two.
- Stereo is judged by ear against mono, and mono won the first round on
  every loop (Ears 2): bouncing echoes and a heavy part leaning to one
  ear annoyed; sparse loops wanted no width at all. Passing the measured
  guards is necessary, not enough. Stereo that moves was liked as an
  idea and disliked in practice (Ears 4); a steady image, width from the
  room and a symmetric spread, won.
- Hear a complaint as a symptom and measure before tuning (Ears 4).
  "Broken low notes" on the organ was clipping into a WaveShaper (its
  input clamps at ±1); "held chords too loud and rough" was the
  generator flipping their octave, not the master. Check what reaches
  every shaper, and look at stems before blaming a layer.
- Level a new part by ear, not by energy: a darker sound reads quieter at
  the same energy (the wash's lowpass was 4 dB louder by energy and only
  1 dB louder by ear). Use K-weighted loudness of the part alone (the
  render minus the same loop without it) against the loop.
- When Mikey says "too much", "in front" or "burying", it has meant level,
  busyness (how often a part re-articulates) and the 500 Hz–2 kHz band
  where tunes live. When the direction isn't clear, offer a few
  variations side by side: it turned the organ from five rounds into a
  pick (and he can then ask for a mix by percent).
- Real music has no truth to check a tool against, so a tool that passes
  its test signals can still read differently from an older one on a
  record (Ears 3: chord changes on *Geogaddi*, 16 to 28 a minute against
  Ears 1's "about 10"). Report both, say why they differ, and give the
  check that exists (the share of changes on a beat against chance).
- Read each tool's output on real music against its own rule before
  trusting it. Ears 3's floor drops listed "1.0 s" drops under a "2 s"
  rule: the edges sat half a second inside the true ones. Test signals
  gave the right counts but looser edges than the rule implied.

## Practical notes

- **Pile folders:** `for-claude - pile 1` (made 2026-10-16) is owned by
  this Project's account (kruu.mikey.thaiculture@gmail.com), not by
  nowmichaelclark; search for it by title. It holds one subfolder per
  album.
- **Getting audio in:** Mikey runs `driftloom-ears.sh` (in
  `tools/ears/`) on a zip or folder: 96 kbps Opus, long tracks cut into
  4-minute pieces, everything under 5.5 MB. He uploads the output folder
  to Drive. The Drive connector carries files up to about 6 MB, one per
  call; parallel batches of 20–40 work.
- **Which account sees the Drive:** the "Driftloom" folder belongs to
  nowmichaelclark@gmail.com; since 2026-10-15 it's shared by name with
  this Project's account, so its folders can be listed and searched
  (`parentId = '<folder id>'`; the connector returns about five entries a
  page, so follow `pageToken`). Batch one is in `for-claude` (all 20
  albums, complete: every track of *Drukqs* and *Neu! 2* is there, though
  Ears 1 measured only part) and `for-claude-split`, which holds only the
  files that were too big for the connector, cut into pieces under 5.5 MB.
  So for batch one, take each file from `for-claude` when it's under about
  5.5 MB and its pieces from `for-claude-split` when it isn't. Batch two
  is in `for-claude - Music Pile 2`. New piles go in `for-claude - pile
  1`, `for-claude - pile 2`, ... (numbers from `docs/ears/plan.md`).
- **The workspace forgets.** Audio, renders and scratch files vanish when
  the session ends; only the repo, the Project and Drive keep things. Commit
  measurements to the repo as you go (a branch and a PR), not only the Project.
- **Context fills fast.** Downloads and long measuring runs are heavy.
  One pile of albums (about 6–8) per ears session is about right; start a
  new session for the next pile.
- **Long renders:** a shell command stops after 10 minutes; run big
  render batches in the background and poll. Two renders at once on two
  cores take about 1–4 s per second of audio (the densest loops more);
  `render2.mjs` writes its index after every job, so a cut-short batch
  keeps what it rendered.
- **Card runs:** same rule. Demucs (about 1 GB, installs with `pip install
  --break-system-packages demucs`) is the slow part; stages are cached
  under the work folder, so a stopped run resumes. Keep audio in its own
  folder and run Python that reads it with `python3 -I`. Save each
  album's card into the repo (`docs/ears/`) when it is done.
- **Pushing to GitHub:** if a push is refused by the git proxy, attach the
  repo to the session first (`add_repo`, push access); open PRs with `gh
  api` (GraphQL isn't available), with `GH_TOKEN` unset if it is stale.

## Reference tracks, for when each sound gets built

All are in the Drive folder (batches one and two). Re-pull these when the
hands build the matching sound, and measure the new sound against them.

| Sound or move | Reference | What to listen for |
|---|---|---|
| Slow arc | Basic Channel, "Quadrant Dub I"; The Field, "Over the Ice" | A filter opening over a loop that barely changes |
| Floor drop | LFO, "LFO"; Autechre, "nil"; The Field, "Over the Ice" | Kick and bass out for a few seconds, everything else on |
| Dub stab and echo | Basic Channel, "Quadrant Dub I"; Porter Ricks, "Port Gentil" | A chord stab thrown into echo (warmth measured with `warmth.py`) |
| Glass keys / bells | Autechre, "nil" | Soft bell notes at 0.5–2 kHz with long tails |
| Organ, long chords | Oneohtrix Point Never, "Boring Angel"; Harmonia, "Dino" | Held chords changing every 15 s or so |
| Wash | Porter Ricks, "Port Gentil" | A noise swell every ~20 s |
| Bright top end | Sakamoto, *Thousand Knives*; Iasos; Lone, *Galaxy Garden* | Where 1–5 kHz detail lives |
| Bass that slides | Forest Swords, "Thor's Stone" | Bass gliding between notes |
| Dry and playful | LFO; Mike & Rich | Dry is fine when it's a choice |
