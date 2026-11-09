# Prompt for Ears 13b (batch 6's levered candidates, and twang)

The first message of the Ears 13b session, 2026-11-08: a cloud Claude Code session on the repo (no audio of Mikey's needed), Opus 5.5, high effort. The session ran on Opus 5.5 and added the Ears 13b sections to `docs/ears/batch-6.md`.

```
Ears 13b (Opus 5.5, high effort), in a cloud Claude Code session. You finish Ears 13's batch: sketch on the Rust core the seven candidates that waited on levers, plus one new instrument, for Mikey to judge by ear. You never build into the app or the core: a sketch is item 83's one block in core/sketch/ and item 99's levers, built only with the sketch feature, and Mikey decides what sounds good.

Read first: docs/ears/batch-6.md (the whole batch, the five already approved, "Levers wanted" and "For the brain"), docs/ears/prompt-ears-13.md (how Ears 13 worked; copy its clip format), docs/ears/sessions.md (with Ears 13's lessons on core sketches and the cloud setup), core/sketch/README.md (including "The levers of item 99"), the header of tools/clips.mjs, tools/ears/sketches/ears13/README.md, and docs/DECISIONS.md's "The music" (Mikey, 2026-11-08: Driftloom makes music computers are good at, not copies of real music; an instrument that recalls a real one is a synth in its spirit, not an imitation). Standing rules: ears outputs live in the repo (docs/ears/, in a PR); never real words in voices; no quotes; wholesome takes only; ears sessions never put code into the app or core; Mikey decides what sounds good.

Setup as Ears 13 did (pip install --break-system-packages numpy scipy librosa soundfile; python3 -P; ffmpeg and node checked; cargo test --lib --features sketch kit and one clips.mjs job with and without "sketch"). If something is missing, stop and say exactly what.

What to do:
1. Sketch the seven levered candidates with item 99's levers, in batch-6's ranked order: tripletKit, widePad (the space lever), dottedPulse (both the pulse and kick332), slowOpening, planingSevenths, dropSpotlight (a sketch that declares its route, keyed by the floor drop), tuningOffset. Use chords.RETUNE=1 wherever a chord sketch needs whole chords. For each: the corpus loops with and without, loudness-matched (-18 LUFS), sketch_check.py, low_guard.py and stereo_guard.py (mono fold, the side under 120 Hz, correlation), and its cost from the kit. Say plainly if a lever doesn't do what the batch needs, and what would.
2. One new instrument: twang, a synth voice in the spirit of a tremolo baritone guitar, not an imitation of one. Low and mid register, a bright plucked attack with a quick decay into a rounder tail, a slow amplitude tremolo (try 4 to 6 Hz and a slow, tempo-synced one), at home in a big room with the wobble. Build it as a channel source on the melody layer following the layer's events (lowBell's way), so it sounds like a synth that recalls the guitar, never like a sampled one. Two or three versions (the tremolo's depth and rate, the brightness of the attack); try it on loops with no kit and slow tempos, and say which vibes it suits (names only from data/vibes.txt, x1.5 to x3 toward, x0.2 to x0.5 away, never 0) and its cost.
3. Add the new sketches to tools/ears/sketches/ears13/ (block, where line, jobs, README), and put core/sketch/ back to the veil before the PR (git diff --stat main -- core/ must be empty).
4. A sampler (00-everything-13b) as Ears 13 made one, then add each candidate's section and verdict row to docs/ears/batch-6.md (verdicts blank for Mikey), keeping the five approved rows as they are.

Clips: on an orphan branch ears13b-clips holding only a clips/ folder, pushed but never merged; give Mikey https://github.com/nowmichaelclark/driftloom/archive/refs/heads/ears13b-clips.zip and say the branch is deleted after his verdicts. No audio on any other branch.

At the end: archive the measurements (docs/ears/raw/batch-6b-raw.tar.gz, no audio) and list it in docs/ears/raw/README.md; save this prompt as docs/ears/prompt-ears-13b.md; update docs/ears/plan.md and sessions.md (a lesson if there is one); node tools/check-docs.mjs passes; open the PR for Mikey to merge; and tell Mikey in plain words what each clip holds and in what order to listen. The hands build the approved batch after phase 3.
```

**What the session found, in one line:** twang went on the chords channel, not the melody's, because the melody channel hears no note events; batch 6's "What the levers did" says why and what would change it.
