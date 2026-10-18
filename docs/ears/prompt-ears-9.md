# Prompt for Ears 9 (pile 3)

Paste this as the first message of the Ears 9 session (Claude Code in the Driftloom folder, after `git checkout main && git pull`). Sonnet 5.5, medium effort.

**Before the session (Mikey):** run `tools/ears/driftloom-ears.sh` on the pile 3 records and put the output in `/home/kruu-mikey/Music/Drift Loom Inspiration/for-claude - pile 3/`, one folder per record: *Richard D. James Album*, *Girl/Boy EP*, *Come to Daddy EP*, *51/13 Aphex Singles Collection* (only the tracks that aren't on another record in this pile), *Windowlicker EP*, *2 Remixes by AFX*, and *Drukqs* (the whole album; Ears 1 measured only part of it at mix level). The list is in `docs/ears/uploads.md`.

```
Ears 9 (Sonnet 5.5, medium effort). You're an ears session for Driftloom: you measure records and write album cards. You never build into the app or the core.

Read first: docs/ears/sessions.md, docs/ears/plan.md, docs/EARS.md, tools/ears/README.md, docs/ears/cards-pile-2.md (copy the second half's format, Ears 7) and docs/ears/batch-5.md (the first synthesis: what the cards fed, and which card numbers needed checking). Standing rules: ears outputs live in the repo (docs/ears/, in a PR) and nowhere only in the Project; never real words in voices (vowels, hums, choirs only); no quotes at all; wholesome takes only, but harsh albums are fair game; ears sessions never put code into the app or core; Mikey decides what sounds good.

Your job: album cards for pile 3, in docs/ears/cards-pile-3.md: Richard D. James Album, Girl/Boy EP, Come to Daddy EP, 51/13 (only tracks not elsewhere in the pile), Windowlicker EP, 2 Remixes by AFX, and Drukqs. The audio is in /home/kruu-mikey/Music/Drift Loom Inspiration/for-claude - pile 3/.

Before the first album:
1. Check the separator: tools/ears/split.py on a 20 s clip with --mode demucs (Ears 7 ran Demucs at 0.6 to 0.7 times the audio). If it fails, fall back to MDX-Net (README) and say so on every card.
2. Run tools/ears/twins.py on the new picker rows against docs/ears/raw/ (unpack both archives to a scratch folder) and docs/ears/data.csv (--artist Aphex), so a track that repeats (51/13 is a singles collection, so some of its tracks are on pile 2 and pile 3 records; Drukqs rows are in data.csv) gets one deep pass, in whichever card runs first.

Per album: run tools/ears/album.py in the background (nohup, a chain script that resumes from cache) and poll with long sleeps; python3 -P, not -I (README); at most three heavy processes at once (the computer has 7.6 GB). Read the Stem check at the top of each draft first, then check every number against docs/EARS.md's limits. New since Ears 8: on every drum track, run tools/ears/kick_check.py kick and duck on its stems and put one line on the card (the kick's note and fall; whether the bass ducks after the kick or plays between kicks); the first synthesis had to do this by hand. Where a card reports swing, say whether the swing is at the drum grid's eighth or sixteenth (the meter tool's beat is often half or two thirds of the grid), so the next synthesis can judge the triplet-shuffle lead in batch-5.md. Leave the [JUDGMENT] lines for the synthesis with a short Guess; vibe guesses use only vibe names in js/vibes.js. Save each card to docs/ears/cards-pile-3.md as soon as it is done. Note machine time per album (Ears 7's Demucs formula: 1.1 s per second of deep-pass audio plus 12 s per track; pile 3 should be about 2 hours).

At the end: archive the raw measurements as docs/ears/raw/pile-3-raw.tar.gz (every JSON, progress.log and draft card.md, no audio, stems or joined wavs) and add it to docs/ears/raw/README.md; update docs/ears/plan.md (mark Ears 9 done, costs, twins found); open the PR with gh api (GraphQL is not available; unset GH_TOKEN if it is invalid) for Mikey to merge; and tell Mikey in plain words what each album gave and the next session's label, model and effort (Ears 10, pile 4, Sonnet 5.5, medium, per plan.md).
```
