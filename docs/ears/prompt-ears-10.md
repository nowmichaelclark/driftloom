# Prompt for Ears 10 (pile 4)

The first message of the Ears 10 session (Claude Code in the Driftloom folder, on the latest `main`), 2026-11-02. Written for Sonnet 5.5, medium effort; the session ran on Opus 5.5.

**Before the session (Mikey):** `tools/ears/driftloom-ears.sh` on the pile 4 records, output in `/home/kruu-mikey/Music/Drift Loom Inspiration/for-claude - pile 4/` (and in Drive, `for-claude - pile 4`), one folder per record.

```
Ears 10 (Sonnet 5.5, medium effort). You're an ears session for Driftloom: you measure records and write album cards. You never build into the app or the core.

Read first: docs/ears/sessions.md, docs/ears/plan.md, docs/EARS.md, tools/ears/README.md, docs/ears/cards-pile-3.md (copy its format, Ears 9) and docs/ears/batch-5.md. Standing rules: ears outputs live in the repo (docs/ears/, in a PR) and nowhere only in the Project; never real words in voices (vowels, hums, choirs only); no quotes at all; wholesome takes only, but harsh albums are fair game; ears sessions never put code into the app or core; Mikey decides what sounds good.

The pause is over: Mikey heard the sound batch in the app on 2026-10-29 and kept every new sound, so piles 4 to 6 go ahead. Mark that in docs/ears/plan.md.

Your job: album cards for pile 4, in a new docs/ears/cards-pile-4.md: Steve Roach Structures from Silence, Kankyo Ongaku (the Japanese ambient compilation), Kyle Jameson View From Above, 2814 Birth of a New Day, Windows 96 Enchanted Instrumentals and Whispers, and Solar Fields Mirror's Edge. The audio is in ~/Music/Drift Loom Inspiration/for-claude - pile 4/ (also in Drive, for-claude - pile 4).

Before the first album:
1. Check the separator: tools/ears/split.py on a 20 s clip with --mode demucs. If it fails, fall back to MDX-Net (README) and say so on every card.
2. Estimate machine time with Ears 9's figures (1.1 s per second of deep-pass audio plus 12 s per track). This pile is long: Kankyo Ongaku alone is nearly two hours across 25 tracks. If the whole pile runs past about 2.5 hours, do the albums that fit, stop at an album boundary, and say which are left for an Ears 10b; don't rush a card to fit.
3. Run tools/ears/twins.py on the new picker rows against docs/ears/raw/ (unpack the archives to a scratch folder) and docs/ears/data.csv. Pile 1 has no archive, so compare by hand against its card numbers, allowing 20 s of length difference (Ears 9's lesson).

Per album: run tools/ears/album.py in the background (nohup, a chain script that resumes from cache) and poll with long sleeps; python3 -P, not -I (README); at most three heavy processes at once. Read the Stem check at the top of each draft first, then check every number against docs/EARS.md's limits. On drum tracks (this pile has few), run tools/ears/kick_check.py kick and duck and put one line on the card, as Ears 9 did. Leave the [JUDGMENT] lines for the synthesis with a short Guess; vibe guesses use only the vibe names in data/vibes.txt.

This pile is mostly beatless ambient, so give these extra care on each card, because they feed work that is coming soon:
- Long form: how a long piece moves over 10 to 20+ minutes (what changes, how often, how slowly, whether it returns). Driftloom is about to get pieces up to 32,768 bars and an Endless mode (queue item 88), and a glacial vibe (very slow and vast).
- Space and stereo: width, depth, how the reverb and echo behave, for the space recipes.
- Slow filter and level swells, against Driftloom's arc (a slow muffled swing).
- Mirror's Edge is a game score: note anything about how its music changes at moments (Driftloom's core now changes loops at a beat, bar, phrase or pass, with a crossfade, for the game Broken Friends).
- Windows 96 samples voices: describe them only as textures, never their words.

Save each card to docs/ears/cards-pile-4.md as soon as it is done, and note machine time per album.

At the end: archive the raw measurements as docs/ears/raw/pile-4-raw.tar.gz (every JSON, progress.log and draft card.md; no audio, stems or joined wavs) and add it to docs/ears/raw/README.md; save this prompt as docs/ears/prompt-ears-10.md; update docs/ears/plan.md (Ears 10 done, costs, twins found, any Ears 10b); open the PR with gh api (GraphQL is not available; unset GH_TOKEN if it is invalid) for Mikey to merge; and tell Mikey in plain words what each album gave. The next ears session is Ears 11 (pile 5, electronica), which can start as soon as Mikey uploads pile 5.
```
