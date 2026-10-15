# Ears tools

Measuring records and Driftloom renders the same way. Used by the ears
sessions (`docs/EARS.md`). Python needs
`pip install --break-system-packages librosa soundfile` and ffmpeg; the
renderer needs Playwright's Chromium. Run everything from a working folder
outside the repo, with `python3 -I` for anything that reads downloaded
audio.

| Script | What it does |
|---|---|
| `driftloom-ears.sh` | Mikey's side: unzips (nested too), makes 96 kbps Opus, cuts tracks over 5 minutes into 4-minute pieces, keeps every file under 5.5 MB for the Drive connector |
| `sweep.py` | Decodes saved Drive download results into `audio/<album>/` (one album per batch). Needs `EARS_RESULTS` |
| `sweep_rules.py` | The same, choosing the album by title rules (write new rules per pile) |
| `render.mjs` | Renders Driftloom loops to WAV through the real Engine and Synth in headless Chromium, with "Let the loop wander" on: `node render.mjs REPO OUTDIR N SECONDS SEED`; `PICK=3,8` renders only those corpus indices, `TAG` names the files |
| `analyze.py` | The main measures: loudness, bands, stereo width, tails, onsets and pulse, chord changes, tuning, wobble, repetition, drift over 10 s and 60 s, arc, brightness sweep |
| `extra.py` | Finer energy bands (40–60 Hz, 60–120, ... 5k+) and pitch classes above the bass |
| `sections.py` | Floor drops (the low end out for 2 s or more while the rest plays) and full stops (the whole mix 30 dB down for 1 s or more) |
| `summarize.py`, `summarize2.py` | Per-album medians; batch two's tables in batch one's definitions |
| `spec.py` | A 60-second log-frequency spectrogram, for looking at by eye |
| `render2.mjs` | `render.mjs` with a job list (`node render2.mjs REPO OUTDIR JOBS.json`): per job a corpus index, length, tag, `proto` (prototype switches) and `space` (a space recipe for a sketch synth); writes 32-bit float WAVs and records refused notes per voice, chord changes a minute and render time |
| `clips.py` | Before/after clips from a `render2.mjs` index (tags `<sound>-before`, `<sound>-after`): loudness-matched MP3s and what moved (bands, added level, refused notes) |
| `space_jobs.py` | Draws each loop's space recipe (room, placed, drifting, wide, widedrift) from its seed and writes `render2.mjs` jobs; needs a `{index: seed}` file from a scan render |
| `stereo_guard.py` | The stereo guards against the mono render: fold loudness and tone, side/mid (whole, above 200 Hz, below 120 Hz), correlation, balance, cost |
| `width_ceiling.py` | The width ceiling, offline: turns the side down where it rises above a target against the mid; the fold is unchanged |
| `stereo_clips.py` | Mono, room, wide and wide + drifting of one loop in one loudness-matched clip |
| `album.py` | **Ears 3:** runs the album-card pipeline end to end (stages `pick,split,events,techniques,theory,space,card`), cached in `WORKDIR/<album>/`, and writes `card.md`: `python3 -I album.py ALBUM_DIR WORKDIR [--stages a,b] [--only TRACK] [--workers 2]`. Run it in the background (`nohup`) and poll; the split stage is the slow one |
| `pick.py` | Track picking from `analyze.py`'s mix numbers: the most typical track plus the 3 least like the rest (every track for an EP of 6 or fewer) |
| `split.py` | Demucs htdemucs on CPU into drums, bass, other, vocals (`split.json` records the mode and time); falls back to harmonic/percussive plus band splits, with no vocals, if Demucs is missing |
| `events.py` | Note events per stem, grouped into instrument candidates: register, attack, decay, brightness, harmonicity, vibrato, detune, level against the mix, left-right position, timestamps of the clearest examples |
| `techniques.py` | Sections, stem entries and exits, brightness sweeps, echo times, pumping locked to the kick, tape wobble and speed, stereo movement, floor drops (from stems) |
| `theory.py` | Tempo and bar length, swing, cross-rhythm, key and mode, chords (changes, settled chords, on-beat share), bass behavior, top line, repetition and form |
| `space.py` | Width, correlation, pan and mono loss by band and by stem, and width over time |
| `card.py` | Drafts the album card (hook, sounds, techniques, theory, space, vibes, wholesome take, For later) from the JSON; judgment lines are marked **[JUDGMENT]** |
| `cardlib.py`, `calsig.py` | Shared helpers; the test-signal makers |
| `calibrate.py`, `cal_*.py` | Calibration: test signals with known answers through every card tool; `python3 -I calibrate.py [--out FILE]` (about 80 s). Rows with tolerance `"limit"` are reported, not graded |

Card tools need `pip install --break-system-packages demucs scipy` too (Demucs is about 1 GB and fetches its weights on first run). One four-track album takes about 30 minutes of machine time on 2 CPU cores, mostly Demucs; see `docs/EARS.md`.

Definitions the reports use: medians per file; "steady beat" means pulse
0.6 or more for records, drums on for Driftloom; drift ratio is the median
of drift60/drift10 per file. The measurements for batches one and two are
in the Project as `claude/ears-data.csv`.

A shell command stops after 10 minutes: run long render batches in the
background (`nohup ... &`) and poll.

Calibration and limits (Ears 1): the wobble measure read a 5-cent wow as 3.3 and a
12-cent wow as 8.0. The chord counter undercounts fast changes (it read
Driftloom as 17 a minute against the generator's 29). Single-file tuning
readings within about ±8 cents are noise. Renders of the same loop differ
from run to run at about -79 dB (the wobble's LFOs).

`sketches/` holds offline sketches used only to render clips, as patches
against `main` at the commit named inside. Apply one to a local copy, never
to the app: `space-sketch.patch` (Ears 2) adds the stereo space recipes
behind `globalThis.SPACE` (see `render2.mjs`).
