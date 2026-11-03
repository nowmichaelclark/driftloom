# Prompt for Ears 11 (pile 5)

The first message of the Ears 11 session (Claude Code in the Driftloom folder, on the latest `main`), 2026-11-03. Written for Sonnet 5.5, medium effort; the session ran on Sonnet 5.5.

**Before the session (Mikey):** `tools/ears/driftloom-ears.sh` on the pile 5 records, output in `/home/kruu-mikey/Music/Drift Loom Inspiration/for-claude - pile 5/` (and in Drive, `for-claude - pile 5`).

```
Ears 11 (Sonnet 5.5, medium effort). You're an ears session for Driftloom: you measure records and write album cards. You never build into the app or the core.

Read first: docs/ears/sessions.md (including Ears 10's and 10b's new lessons), docs/ears/plan.md, docs/EARS.md, tools/ears/README.md, docs/ears/cards-pile-4.md (copy its format, Ears 10 and 10b) and docs/ears/batch-5.md. Standing rules: ears outputs live in the repo (docs/ears/, in a PR) and nowhere only in the Project; never real words in voices (vowels, hums, choirs only); no quotes at all; wholesome takes only, but harsh albums are fair game; ears sessions never put code into the app or core; Mikey decides what sounds good.

Your job: album cards for pile 5 (electronica), in a new docs/ears/cards-pile-5.md: Boards of Canada Music Has the Right to Children, Ulrich Schnauss Far Away Trains Passing By, The Future Sound of London Lifeforms, Bowery Electric Beat, Ulver Perdition City, and Harald Grosskopf Synthesist. The audio is in /home/kruu-mikey/Music/Drift Loom Inspiration/for-claude - pile 5/ (also in Drive, for-claude - pile 5).

Before the first album:
1. Check the separator: tools/ears/split.py on a 20 s clip with --mode demucs. If it fails, fall back to MDX-Net (README) and say so on every card.
2. Estimate machine time with Ears 10's figures (plan.md, fifth check). Most tracks here are under 11 minutes; Lifeforms has long ones. If the pile runs past about 2.5 hours, do the albums that fit, stop at an album boundary, and leave the rest for an Ears 11b.
3. Run tools/ears/twins.py on the new picker rows against docs/ears/raw/ (unpack the archives to a scratch folder) and docs/ears/data.csv. Ears 1 and 2 measured Boards of Canada at mix level (data.csv), so check for repeats there.

Per album: run tools/ears/album.py in the background (nohup, a chain script that resumes from cache), one stage per process on long albums, and poll with long sleeps; python3 -P, not -I (README); at most three heavy processes at once, and watch memory (Ears 10's lessons: journalctl -k for an OOM kill when a stage stalls). Read the Stem check at the top of each draft first, then check every number against docs/EARS.md's limits. A grid near 26 BPM is a bar, not a beat (Ears 10b). Leave the [JUDGMENT] lines for the synthesis with a short Guess; vibe guesses use only the vibe names in data/vibes.txt.

This pile has more drums than pile 4, and it sits close to what Driftloom already is (relaxing, nostalgic loops), so give these extra care:
- Drums: on every drum track, tools/ears/kick_check.py kick and duck, one line on the card. Where a card reports swing, say whether it is at the drum grid's eighth or sixteenth; batch-5.md's triplet-shuffle lead is still waiting for a real case.
- Tape and pitch: wow, flutter, detune and pitch drift (Boards of Canada especially), against Driftloom's own tape wobble.
- Stereo: width by band, and what the mix loses in mono (Ears 10b's lesson), for the stereo and mono switch.
- Long form, with tools/ears/longform.py on tracks of 8 minutes or more (Schnauss's slow builds, Lifeforms): Driftloom loops can now be up to 32,768 bars and Endless, and long-form composing comes later.
- Voices on these records (sampled speech, sung vocals) are described only as textures, never their words.

Save each card to docs/ears/cards-pile-5.md as soon as it is done, and note machine time per album.

At the end: archive the raw measurements as docs/ears/raw/pile-5-raw.tar.gz (every JSON, progress.log and draft card.md; no audio, stems or joined wavs) and add it to docs/ears/raw/README.md; save this prompt as docs/ears/prompt-ears-11.md; update docs/ears/plan.md (Ears 11 done, costs, twins found, any Ears 11b); open the PR with gh api (GraphQL is not available; unset GH_TOKEN if it is invalid) for Mikey to merge; and tell Mikey in plain words what each album gave. The next ears session is Ears 12 (pile 6, fusion and games), which can start once Mikey uploads pile 6; after it, Ears 13 is the second synthesis (Opus 5.5, high).
```

**What the session did differently from the prompt (for the next one to read):** Synthesist was left for an Ears 11b (the pile passed 2.5 hours because two album chains ran at once, which slowed Demucs to 0.9 to 1.2 times the audio); the brief's "Lifeforms has long ones" was wrong (its longest track is 5:49), and the 8-minute long-form reads are Schnauss's Molfsee, Bowery Electric's Postscript (16:41) and Perdition City's Tomorrow Never Knows (7:59). Run one chain at a time if the 2.5-hour rule matters.
