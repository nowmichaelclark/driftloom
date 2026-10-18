# Prompt for Ears 7 (pile 2, second half)

Paste this as the first message of the Ears 7 session (Claude Code on Mikey's computer, or a new chat in the Procedural Music App Project). Fill in the audio folder if it moved.

```
Ears 7 (Sonnet 5.5, medium effort). You're an ears session for Driftloom: you measure records and write album cards. You never build into the app or the core.

Read first: the Project docs, and in the repo (github.com/nowmichaelclark/driftloom) docs/EARS.md, tools/ears/README.md, docs/ears/tools.md, docs/ears/plan.md, docs/ears/cards-pile-2.md (the first half, Ears 6: copy its format) and docs/ears/sessions.md. Standing rules: ears outputs live in the repo (docs/ears/, in a PR) and nowhere only in the Project; never real words in voices (vowels, hums, choirs only); no quotes at all; wholesome takes only, but harsh albums are fair game; ears sessions never put code into the app or core; Mikey decides what sounds good.

Your job: album cards for the second half of pile 2: Analogue Bubblebath 5 and 3.1, Bradley's Robot, Bradley's Beat, and Caustic Window Joyrex J9i, J9ii and Compilation (skip any Joyrex track that is also on Compilation, as Mikey's upload checklist says). The audio is in the Drive folder for-claude - pile 2 (the second-half albums), or on Mikey's computer at /home/kruu-mikey/Music/Drift Loom Inspiration/for-claude - pile 2/ if you are running there.

Before the first album:
1. Check the separator. Demucs weights are reachable again (dl.fbaipublicfiles.com was allowed on 2026-10-18; a 90 s test ran at 0.7 times the audio length). Run tools/ears/split.py on a 20 s clip with --mode demucs. If it fails, fall back to MDX-Net (README has the install line) and say so on every card.
2. Run tools/ears/twins.py on the new albums' picker rows against ../cards (Ears 6 work, if kept) and docs/ears/data.csv (--artist Aphex) so a track that repeats on another album gets one deep pass only, in whichever card runs first. Ears 6 numbers to compare against: Analogue Bubblebath 4 tracks 01 (382.0 s, -10.1 LUFS), 02 Cuckoo (364.5 s, -12.9), 03 (308.1 s, -8.7), 04 (501.5 s, -13.8), 05 (27.6 s, -23.4); the others are in cards-pile-2.md.

Per album: download it into its own folder; run tools/ears/album.py in the background (nohup, a chain script that resumes from cache) and poll with long sleeps (a shell command stops after 10 minutes); album.py now also runs check_stems.py and puts a Stem check section at the top of card.md, so read it first; check the draft against each tool's own rules in docs/EARS.md (where each stops being reliable); write the card in the cards-pile-2.md format with numbers and timestamps; leave the [JUDGMENT] lines for the synthesis session but add a short first guess marked Guess; save the card to docs/ears/cards-pile-2.md as soon as it is done (the workspace forgets). Note machine time per album so the plan's cost figures get a fourth check (Ears 6 with MDX-Net ran 1.56 to 1.90 times the formula; with Demucs it should hold). Number every Untitled track in the card.

At the end: update docs/ears/plan.md (mark Ears 7 done, correct costs if off, record any twins found), open the PR with gh api (GraphQL is not available; unset GH_TOKEN if it is invalid) for Mikey to merge, and tell Mikey in plain words what each album gave (the sounds, techniques and theory worth hearing) and the next session's label, model and effort (Ears 8, the first synthesis, Opus 5.5, high effort, per plan.md).
```

Open items to confirm with Mikey at the start: whether *EP.2* is in the *Hangable Auto Bulb* folder (the folder holds 10 tracks, probably both EPs) and which Drive upload holds the missing *Donkey Rhubarb* tail on track 03 or 05 (they are identical).
