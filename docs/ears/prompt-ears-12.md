# Prompt for Ears 12 (pile 6, with Synthesist)

The first message of the Ears 12 session (Claude Code in the Driftloom folder, on the latest `main`), 2026-11-07. Written for Sonnet 5.5, medium effort; the session ran on Sonnet 5.5.

**Before the session (Mikey):** `tools/ears/driftloom-ears.sh` on the pile 6 records, output in `/home/kruu-mikey/Music/Drift Loom Inspiration/for-claude - pile 6/` (the Synthesist audio was already in `for-claude - pile 5`).

```
Ears 12 (Sonnet 5.5, medium effort). You're an ears session for Driftloom: you measure records and write album cards. You never build into the app or the core.

Read first: docs/ears/sessions.md (with Ears 10's, 10b's and 11's lessons), docs/ears/plan.md (the fifth and sixth cost checks), docs/EARS.md, tools/ears/README.md, docs/ears/cards-pile-5.md (copy its format, Ears 11) and docs/ears/batch-5.md. Standing rules: ears outputs live in the repo (docs/ears/, in a PR) and nowhere only in the Project; never real words in voices (vowels, hums, choirs only); no quotes at all; wholesome takes only, but harsh albums are fair game; ears sessions never put code into the app or core; Mikey decides what sounds good.

Use ~/ears12 as the working folder.

Your job, in this order:
1. Harald Grosskopf Synthesist, the album Ears 11 left (its picks are done: docs/ears/raw/pile-5-raw.tar.gz, 1980_Synthesist/pick.json; the audio is in /home/kruu-mikey/Music/Drift Loom Inspiration/for-claude - pile 5/). Append its card to docs/ears/cards-pile-5.md, with Ears 11's brief (docs/ears/prompt-ears-11.md) for this one album.
2. Pile 6 (fusion and games), in a new docs/ears/cards-pile-6.md: Nonlocal Forecast Bubble Universe!, Holographic Universe(s?)! and Conscious Agents (a single), Fire-Toolz and Nonlocal Forecast Triangular Reformat (a 32-track remix album), the Animal Crossing (GameCube) soundtrack, and Jaga Jazzist Pyramid. The audio is in /home/kruu-mikey/Music/Drift Loom Inspiration/for-claude - pile 6/. If that folder is missing or incomplete, do Synthesist only, open the PR, and stop.

Run one album chain at a time (Ears 11's lesson: two at once ran 48% over for little gain). Estimate machine time first with plan.md's figures; if the work passes about 2.5 hours, stop at an album boundary and leave the rest for an Ears 12b. Triangular Reformat is long; pick from it as the picker does, don't deep-pass everything.

Before the first album: check the separator (tools/ears/split.py on a 20 s clip, --mode demucs; fall back to MDX-Net and say so if it fails), and run tools/ears/twins.py on the new picker rows against docs/ears/raw/ (unpack the archives to a scratch folder) and docs/ears/data.csv (the Nonlocal Forecast albums and the remix album may share material).

Per album, as Ears 11 did: tools/ears/album.py in the background (nohup, a chain script that resumes from cache), one stage per process on long albums; python3 -P, not -I; watch memory (journalctl -k for an OOM kill); read the Stem check first and check every number against docs/EARS.md's limits; kick_check.py kick and duck on every drum track, with swing placed at the grid's eighth or sixteenth; monoloss.py and the album medians on every album; tools/ears/longform.py on tracks of 8 minutes or more. Leave the [JUDGMENT] lines for the synthesis with a short Guess; vibe guesses use only the vibe names in data/vibes.txt.

Give these extra care in this pile:
- Animal Crossing's hourly music: the heart of the album for Driftloom. Its game, Broken Friends, will steer Driftloom's music by time of day, friend and mood, and Driftloom can now keep a friend's theme while the vibes change. Card how the hours differ from each other (tempo, key and mode, instruments, brightness, density, swing) and what stays the same across them, as one table if you can, and how the music turns over at the hour if the rip shows it. K.K. Slider's songs and any animal voices are textures only.
- Fusion (Nonlocal Forecast, Jaga Jazzist): chord colors past the seventh, odd meters, and how a busy arrangement still sits softly, against Driftloom's coffee-shop rule (nothing should bother people in a coffee shop).
- Fire-Toolz's harsher parts (screamed or distorted vocals): describe them as textures, and give the wholesome take.

Save each card as soon as it is done, and note machine time per album.

At the end: archive the raw measurements (docs/ears/raw/pile-6-raw.tar.gz, and Synthesist's into pile-5b-raw.tar.gz; every JSON, progress.log and draft card.md, no audio, stems or joined wavs) and add them to docs/ears/raw/README.md; save this prompt as docs/ears/prompt-ears-12.md; update docs/ears/plan.md (Ears 11b folded into Ears 12, Ears 12 done, costs, twins, any Ears 12b); open the PR with gh api (GraphQL is not available; unset GH_TOKEN if it is invalid) for Mikey to merge; and tell Mikey in plain words what each album gave. Then, once the archives are committed, tell Mikey how big ~/ears12 is and that it can be deleted (it holds only rebuildable stems and caches), unless an Ears 12b still needs it. After pile 6 (and 12b if needed) comes Ears 13, the second synthesis of piles 3 to 6 (Opus 5.5, high).
```

**What the session did differently from the prompt (for the next one to read):**

- **Stopped at the 2.5-hour rule with two albums left:** Fire-Toolz and Nonlocal Forecast *Triangular Reformat* and Jaga Jazzist *Pyramid* are the **Ears 12b** (their picks are done; `raw/pile-6-raw.tar.gz`). Done: Synthesist, Animal Crossing, *Bubble Universe!*, *Holographic Universe(s)!* and *Conscious Agents*, in 2.2 hours of machine time, one chain at a time. The extra-care briefs for the 12b are the Fire-Toolz and the fusion ones above.
- **The machine's Python had moved to 3.15** and the old packages would not load; a uv-made 3.13 virtual environment (`~/ears12/venv`, about 6 GB) did the work. The Ears 12b can recreate it in about 10 minutes (`sessions.md`, Ears 12).
- **Animal Crossing was planned apart** (curated picker set, 8 deep passes, a light pass over the 24 hours, a Winter/Cherry comparison); the rest of the soundtrack (about 130 K.K. songs and 40 town tracks) is at picker level only.
- **The Synthesist picks' joined wavs were gone** (Ears 11's workspace); they were rebuilt with `pick.group_tracks` and the picks kept.
