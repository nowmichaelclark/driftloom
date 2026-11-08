# Prompt for Ears 12b (pile 6 second part, with the colors reading)

The first message of the Ears 12b session (Claude Code in the Driftloom folder, on the latest `main`), 2026-11-08. Written for Sonnet 5.5, medium effort; the session ran on Sonnet 5.5.

**Before the session (Mikey):** nothing new; the pile 6 audio was already in `/home/kruu-mikey/Music/Drift Loom Inspiration/for-claude - pile 6/` and Synthesist's in `for-claude - pile 5`. `~/ears12` had been deleted.

```
Ears 12b (Sonnet 5.5, medium effort). You're an ears session for Driftloom: you measure records and write album cards. You never build into the app or the core.

Read first: docs/ears/sessions.md (with Ears 11's and 12's lessons), docs/ears/plan.md (the seventh cost check, and Ears 12b's row), docs/EARS.md, tools/ears/README.md, docs/ears/cards-pile-6.md (copy its format) and docs/ears/prompt-ears-12.md (its extra-care briefs, and what Ears 12 did differently). Standing rules: ears outputs live in the repo (docs/ears/, in a PR) and nowhere only in the Project; never real words in voices (vowels, hums, choirs only); no quotes at all; wholesome takes only, but harsh albums are fair game; ears sessions never put code into the app or core; Mikey decides what sounds good.

Use ~/ears12b as the working folder; it starts empty (Ears 12's ~/ears12 was deleted). Setup:
- Build the Python 3.13 venv in ~/ears12b/venv as sessions.md's Ears 12 lesson says (uv; the machine's own Python is 3.15 and the old packages don't load); run python3 -P from it.
- Unpack docs/ears/raw/pile-6-raw.tar.gz (and pile-5b-raw.tar.gz) to a scratch folder. The picks are done; rebuild their joined wavs from each pick.json with pick.group_tracks, as Ears 12 did for Synthesist.
- Check the separator on a 20 s clip (split.py --mode demucs; fall back to MDX-Net and say so).

Your job, in this order:
1. Chord colors past the seventh. theory.py's CHORD_TYPES stop at sevenths and sus, so Ears 12 couldn't measure Bubble Universe!'s colors. Add a separate colors reading, not new chord templates (the labels on earlier cards must not move): over each chord segment theory.chords() already finds, report which tones past the chord's own are present and held (9, 11, 13 / add9, 6), with a strength. Calibrate it in cal_theory.py on synthesized chords (triads, sevenths, add9, maj9, m9, m11, 6/9, 13), with false-positive rates on plain triads and sevenths, and check that cal_theory's existing figures are unchanged. Keep it short; it's a tool for these cards.
2. Fire-Toolz and Nonlocal Forecast Triangular Reformat (Fire_Toolz_Nonlocal_Forecast_Triangular_Reformat/pick.json). Fire-Toolz's harsher parts (screamed or distorted vocals) as textures, with the wholesome take.
3. Jaga Jazzist Pyramid (Jaga_Jazzist_2020_Pyramid/pick.json; four long tracks, longform.py on all four). Fusion: the new colors reading, odd meters (check the meter tool by hand where the bar seems to change, reading the spacing between hit clusters as Ears 12's lesson says), and how a busy arrangement still sits softly against the coffee-shop rule.
4. The colors reading on the records Ears 12 carded, on their deep-passed tracks (from the unpacked picks), with the light pass (hpss split plus theory.py, about 24 s a track) rather than new Demucs stems: Bubble Universe!, Holographic Universe(s)!, Conscious Agents, and Synthesist if its audio is still in for-claude - pile 5 (skip it and say so if not). Add a short colors line to each card.
5. If time allows, in this order: a deep pass on six more K.K. Slider songs of different rhythms for the Animal Crossing card (its K.K. rhythm range is thin; for Broken Friends this album matters most), then Holographic's two six-minute tracks.

Audio is in /home/kruu-mikey/Music/Drift Loom Inspiration/for-claude - pile 6/. One album chain at a time. Estimate machine time first from plan.md's seventh check (the formula is a ceiling, about 15% under on average), counting the setup; stop at a task boundary before about 2.5 hours. Per album, as Ears 12 did: album.py in the background with a resuming chain; watch memory (journalctl -k); read the Stem check first and check every number against docs/EARS.md's limits; kick_check.py kick and duck on every drum track; monoloss.py and the album medians; [JUDGMENT] lines left for the synthesis with a short Guess; vibe guesses only from data/vibes.txt. Save each card as soon as it's done, and note machine time per album.

At the end: archive the raw measurements (docs/ears/raw/pile-6b-raw.tar.gz: every JSON, progress.log and draft card.md, no audio, stems or joined wavs) and add it to docs/ears/raw/README.md; save this prompt as docs/ears/prompt-ears-12b.md; update docs/ears/plan.md (Ears 12b done, costs, the colors tool) and tools/ears/README.md (the colors reading); add a lesson to sessions.md if setup from an empty folder taught one; open the PR with gh api (GraphQL is not available) for Mikey to merge; and tell Mikey in plain words what each album gave. Then tell Mikey how big ~/ears12b is: everything but the venv can be deleted (stems and caches rebuild), and the venv is worth keeping if Ears 13 follows soon. Next comes Ears 13, the second synthesis of piles 3 to 6 (Opus 5.5, high).
```

**What the session did differently from the prompt (for the next one to read):**

- **The colors reading went in `theory.py` as `colors()`**, with its output in `theory.json` as `colors`; the first version (any held tone counts) read dense pads as full of colors, so it reports `share_all_three` apart (a wash) and compares each tone with the chord's tones but the root (the bass and harmonics stack on it).
- **Two extra deep passes on Triangular Reformat** (16 Screamography and 10 Toiret Status) chosen from a scan of all 32 mixes for bright, noise-like stretches (`harsh.py`), because the title track turned out almost voiceless. Pyramid's meter was checked by hand with `barlen.py` and `barwin.py` (autocorrelation of the drum onsets, whole track and 30 s windows).
- **Done from the optional list:** the six K.K. songs (March, Waltz, Bossa, Calypso, Reggae, Tango; no kit in any of them). **Not done:** Holographic's two six-minute tracks.
- **Two systemd-oomd kills** hit album.py runs while other jobs shared the machine; the chain was resumed (`sessions.md`, Ears 12b).
- Synthesist's audio was still in `for-claude - pile 5`, so its colors line is in `cards-pile-5.md`.
