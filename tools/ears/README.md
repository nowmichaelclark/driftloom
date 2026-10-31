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
| `render2.mjs` | `render.mjs` with a job list (`node render2.mjs REPO OUTDIR JOBS.json`): per job a corpus index, length, tag, `proto` (prototype switches) and `space` (a space recipe for a sketch synth); writes 32-bit float WAVs and records refused notes and asks per voice, chord changes a minute and render time. Ears 4 added `stems` (a second WAV with each layer and each sketch on its own channel), `then`/`thenAt` (load another loop partway through, as Next does), `reserve` (price every note as if N more units were spent), sketch yields (a sketch that asks for room and holds none is counted apart from refused notes) and `PLAYWRIGHT_ROOT` |
| `../clips.mjs` | **Item 83:** clips from the core's own player (natively, no Chromium) with the levers a sketch or an A/B needs: a sketch block on the core (`core/sketch/`), any choice forced, a constant moved, a setting pinned, a window; WAV and Opus and an index of what each loop drew: `node tools/clips.mjs JOBS.json OUTDIR --opus`. The old JavaScript sketch path (`render2.mjs` and the patches) stays until phase 3 ends |
| `scan.mjs` | The corpus without rendering: per loop its profile, tempo, length, notes a second per layer, drums or not, voices, reverb space and chord changes, for picking busy and sparse loops (`node scan.mjs REPO > scan.json`) |
| `sketch_check.py` | What a sketch adds and takes, against the same loop without it: added level, its loudest and median 2 s window against the loop, bands, refused notes per voice (any voice that loses a note is listed), each layer's own level from the stems, cost |
| `low_guard.py` | **Ears 8:** the low-end guard and a sketch's level by ear, against the same loop without it: the 20-60 Hz and 60-120 Hz shares, the whole loop's loudness change, the part's own K-weighted loudness (render minus none) and its loudest 3 s, the 20-60 Hz level of the loudest 3 s, and the share of the part a phone can play (above 150 Hz): `python3 -I low_guard.py INDEX.json WAVDIR [OUT.json] [--from S]` |
| `kick_check.py` | **Ears 8:** from one track's stems (split.py's, or a render's stems split into drums.wav and bass.wav): `kick` gives each kick's note (the strongest 30-130 Hz line 40-160 ms after the onset) and its fall in dB/s; `duck` gives the bass stem's level against time since the kick, averaged over every kick (a duck, or a bass playing between kicks); `swell` gives bass-note rise times. Written to check card numbers the tools flag (pitch under 60 Hz, pumping that reads 70% of the truth or locks on the wrong period) |
| `warmth.py` | How warm a stab is, the same way on a record and a sketch: centroid, 2–4 kHz and 1–2 kHz against 250 Hz–1 kHz, attack rise time |
| `seq_clips.py` | Clips from a list of segments (any files, any start and length), each matched to -18 LUFS, with timestamps |
| `clips.py` | Before/after clips from a `render2.mjs` index (tags `<sound>-before`, `<sound>-after`): loudness-matched MP3s and what moved (bands, added level, refused notes) |
| `space_jobs.py` | Draws each loop's space recipe from its seed and writes `render2.mjs` jobs. Round 1: room, placed, drifting, wide, widedrift. Round 2 (Ears 4, needs a `scan.mjs` file instead of `{index: seed}`): wide2 and drift2, scaled by how busy and how wet the loop is |
| `stereo_guard.py` | The stereo guards against the mono render: fold loudness and tone, side/mid (whole, above 200 Hz, below 120 Hz), correlation, balance, cost; Ears 4 added bounce (how fast the image jumps) and slow drift (how far balance and width travel over 4 s windows) |
| `scale_side.py` | A recipe's width turned up or down offline: every stereo move in the sketch is linear in its width settings, so scaling a render's side by K is the same as scaling them all by K (run before the ceiling) |
| `blend_side.py` | Two space recipes blended offline: one render's mid and a weighted sum of both renders' sides (exact, as every recipe keeps the mono mix as its mid) |
| `width_ceiling.py` | The width ceiling, offline: turns the side down where it rises above a target against the mid; the fold is unchanged. `auto` takes each recipe's own target |
| `stereo_clips.py` | Mono, room, wide and wide + drifting of one loop in one loudness-matched clip (any order of tags with lengths, and a start offset) |
| `album.py` | **Ears 3:** runs the album-card pipeline end to end (stages `pick,split,events,techniques,theory,space,check,card`), cached in `WORKDIR/<album>/`, and writes `card.md`: `python3 -I album.py ALBUM_DIR WORKDIR [--stages a,b] [--only TRACK] [--workers 2] [--no-vocals]`. Run it in the background (`nohup`) and poll; the split stage is the slow one |
| `pick.py` | Track picking from `analyze.py`'s mix numbers (**Ears 6b:** a track with the same length, loudness and tone as an earlier one on the album is listed under `duplicates` and left out of the deep pass): the most typical track plus the 3 least like the rest (every track for an EP of 6 or fewer) |
| `split.py` | Demucs htdemucs on CPU into drums, bass, other, vocals (`split.json` records the mode and time); if Demucs fails (its weights were blocked in the Ears 6 workspace) it tries the four MDX-Net `kuielab_a` models (ONNX, run through `audio-separator`, files in `$MDX_DIR` or `/tmp`; about 2.3 to 2.8 times the audio length), and only then the harmonic/percussive plus band split, with no vocals. `--mode auto\|demucs\|mdx\|hpss` forces one; `split.json` says which ran. MDX-Net is weaker: the drums stem swallows the sub, the bass stem is thin or empty where kick and bass share it, and the vocals stem on an instrumental is hat and pad bleed |
| `events.py` | Note events per stem, grouped into instrument candidates: register, attack, decay, brightness, harmonicity, vibrato, detune, level against the mix, left-right position, timestamps of the clearest examples |
| `techniques.py` | Sections, stem entries and exits, brightness sweeps, echo times, pumping locked to the kick, tape wobble and speed, stereo movement, floor drops (from stems) |
| `theory.py` | Tempo and bar length, swing, cross-rhythm, key and mode, chords (changes, settled chords, on-beat share), bass behavior, top line, repetition and form |
| `space.py` | Width, correlation, pan and mono loss by band and by stem, and width over time |
| `card.py` | Drafts the album card (hook, sounds, techniques, theory, space, vibes, wholesome take, For later) from the JSON; judgment lines are marked **[JUDGMENT]** |
| `check_stems.py` | **Ears 5:** spot checks on one track's stems before its card is written: each stem's share of the power (empty bass stem, no beat), band shares per stem, the strongest 20–200 Hz lines (subharmonic misreads), the drum-onset grid (tempo octave), and runs where the low end of all stems summed is 15 dB down (floor drops the stems-only count misses): `python3 -I check_stems.py WORKDIR/<album>/stems/<track> [--drop-db 15]`. **Ears 6b:** `album.py` now runs it as stage `check` (`stem_check.json` per track) and `card.py` puts the void flags and the meter-tool-against-drum-grid tempo note at the top of the card; `check_stems.report()` is the function behind it |
| `twins.py` | **Ears 6b:** probable twin tracks across albums (length within 3 s, loudness within 0.6 LU, tone alike) from the picker rows in a workdir and, with `--csv docs/ears/data.csv --artist Aphex`, from earlier piles: `python3 -I twins.py WORKDIR [--csv FILE] [--artist TEXT]` |
| `cardlib.py`, `calsig.py` | Shared helpers; the test-signal makers |
| `calibrate.py`, `cal_*.py` | Calibration: test signals with known answers through every card tool; `python3 -I calibrate.py [--out FILE]` (about 80 s). Rows with tolerance `"limit"` are reported, not graded |

**`python3 -P` instead of `-I` (Ears 7):** `-I` also hides the user site-packages, which is where `pip install --break-system-packages` puts numpy, torch and demucs on a machine without root, so every script died on `import numpy`. `-P` keeps the same protection that matters here (no current-folder or script-folder entry on `sys.path`, so a planted `json.py` next to downloaded audio is not imported) and still sees installed packages. Use `-I` where the packages are system-wide; use `-P` where they are in `~/.local`. If pip fails with a disk quota error, set `TMPDIR` to a folder on the home disk (`/tmp` was a small tmpfs).

Card tools need `pip install --break-system-packages demucs scipy` too (Demucs is about 1 GB and fetches its weights on first run). If the Demucs weights are blocked, install `audio-separator onnxruntime audioread` instead and download the four models `kuielab_a_{drums,bass,other,vocals}.onnx` from `https://github.com/TRvlvr/model_repo/releases/download/all_public_uvr_models/` into `/tmp` (or `$MDX_DIR`); `split.py` picks them up on its own. One album takes about 40 minutes of machine time on 2 CPU cores (28 to 43 over five albums), about 1.6 seconds per second of deep-pass audio plus 25 s per track in the album, mostly Demucs; see `docs/EARS.md`.

Definitions the reports use: medians per file; "steady beat" means pulse
0.6 or more for records, drums on for Driftloom; drift ratio is the median
of drift60/drift10 per file. The measurements for batches one and two are
in the repo as `docs/ears/data.csv`.

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
behind `globalThis.SPACE` (see `render2.mjs`). `ears4-sketch.patch`
(Ears 4, against `a23153d`) holds everything Ears 4 rendered: the space
sketch with round 2's breath and drifting light kit and round 3's
allpass reverb side (`verbSmooth`), and the dub stab, organ, wash and long
chords behind `globalThis.PROTO` (`dubStab`, `organ`, `wash`,
`longChords`: `'on'` for round 4, `'r3'`, `'r2'` or `'r1'` for the earlier rounds
where they differ, or a kind to force one draw, such as `dubStab: 'busy'`
or `longChords: 'whole'`; `organDrive`, `washRel`, `stabLevel` override
one value). The approved organ is `organVariant: 'chapelOrgan'`
(round 7's mix; `organMix` reweights it). `ears8-sketch.patch` (Ears 8,
applied on top of `ears4-sketch.patch`) holds the first synthesis's
candidates behind `globalThis.PROTO`: `tunedKick`, `subDuck`, `lateFloor`,
`padSweep`, `echoTime`, `pedalBass` and `floorDrop` (Ears 1's approved
recipe as `'on'`, the new shapes as `'plus'` or by name); its header lists
every switch. With it, `render2.mjs`'s index has an `ears8` list (what each
load drew, and every floor drop's pass, time and shape) and `meta.e8pedal`.

Render cost: two renders at once on a two-core workspace take about
1–4 s per second of audio; the densest loops (cinder #16) about 4 s. Keep
150 s renders to loops that need them.

**Memory (Ears 8):** a 90 s render with `stems` holds about 175 MB of float
audio, passed through the page as a base64 string, so each render process
peaks near 1 GB. On Mikey's computer (7.6 GB) eight at once ran out of memory
(the kernel killed a Chromium); three at a time with stems, or two alongside
other work, is safe. Turn stems on only for the pairs whose per-layer levels
you need. Playwright installs fine into a local folder (`npm install
playwright@1.48.2` there, then `npx playwright install chromium`;
`PLAYWRIGHT_ROOT=<folder>/node_modules`).
