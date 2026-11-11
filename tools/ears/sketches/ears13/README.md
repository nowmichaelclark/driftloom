# Ears 13 sketches (batch 6)

Sketches on the core (queue item 83), never in the app: each folder is one
`block.rs` and one `where.txt` for `core/sketch/`, built only with the
`sketch` feature. `docs/ears/batch-6.md` says what each is, what it measured
and why. `core/sketch/` holds the veil demonstration in the repo (CI builds it);
copy a sketch in only to hear it, and put the folder back after
(`git checkout core/sketch`).

| Folder | What | Where |
|---|---|---|
| `lowBell/` | a struck low bell (D2 to C#3) on each chord's root | chords |
| `lowBellSwell/` | the same with a 0.4 s attack | chords |
| `colorTone/` | a held 13 over major chords, an 11 over minor ones | chords |
| `padWow/` | a slow wow on the chord layer only | chords (whole channel) |
| `widePad/` | the chord layer's side: renders nothing in stereo (a whole-channel block gets no side). Superseded in Ears 13b by item 99's `space` lever, no block | chords |
| `everything/` | the sampler: lowBell, colorTone and padWow in one block; `use.sh b|c|w...` writes a variant with its parts beside it | chords |

| `dropSpotlight/` | Ears 13b: lowBell alone in a floor drop, keyed by the drums (mod 0) and the bass (mod 1) routed in | chords |
| `dropSpotlightSwell/` | the same with lowBell's swelling attack (0.4 s) | chords |
| `twang/` | Ears 13b: a synth in the spirit of a tremolo baritone guitar, picking each chord; `variant.py VERSION BPM` writes soft, slow or bright for one loop's tempo | chords |

deepDuck and wideSweep are not blocks: they are the approved subDuck and
padSweep with their settings pinned (`jobs/deepDuck.json`, `jobs/wideSweep.json`).
Since queue item 101a they are in the core, with slowOpening: `"force":
{"subDuck": "deep"}`, `{"padSweep": "wide"}` or `"fast"`, `{"lateFloor":
"slow"}` play the built ones, and a job's `"stem": "bass"` (or `"chords"`)
writes that layer alone, as item 101a's PR measured them.

## Rebuild the clips

From the repository's root, with Node 22, ffmpeg, Python with numpy, scipy,
soundfile and pyloudnorm (`python3 -P`), and the core's toolchain:

```sh
cp tools/ears/sketches/ears13/lowBell/{block.rs,where.txt} core/sketch/
cd core && cargo build --release --locked --features sketch --example dlrender && cd ..
cp core/target/release/examples/dlrender /tmp/dl-lowBell
node tools/clips.mjs tools/ears/sketches/ears13/jobs/lowBell.json /tmp/r/lowBell --bin /tmp/dl-lowBell
python3 -P tools/ears/sketches/ears13/ab.py /tmp/r/lowBell
git checkout core/sketch
```

One binary per sketch (and per sampler variant: `everything/use.sh b`, `cw`,
`c`). deepDuck's and wideSweep's jobs run on any sketch build (they do not turn
the sketch on). Render each candidate into `RENDERDIR/<candidate>/` (lowBell's
swelling jobs, `jobs/lowBellSwell.json`, with the lowBellSwell build, into the
same folder as lowBell's; the sampler's four `jobs/every-*.json` into
`RENDERDIR/everything/`), then:

```sh
python3 -P tools/ears/sketches/ears13/make_clips.py tools/ears/sketches/ears13/clip_plan.json RENDERDIR CLIPDIR
```

`make_jobs.py OUTDIR` writes the job lists again. `ab.py` turns a clips.mjs
folder into a render2-style index (`r2index.json`) that `sketch_check.py` and
`low_guard.py` read, and prints its own A/B table. For `stereo_guard.py`, the
core writes a mono render as one channel: duplicate it to two first.

**The silent organ.** The core sends the chords channel a retune at each chord
only while the chapel organ plays, so lowBell's and colorTone's jobs force the
organ on and pin its `level`, `dry` and `verb` to 0. That renders a loop bit
for bit as without it (six loops checked). On a loop whose keys arpeggiate the
organ hears one note at a time.

`kit-report.txt`: each sketch's lines from
`KIT=report cargo test --release --lib --features sketch every_block` (rms,
peak, the 2-5 kHz share, cost). `clips-timestamps.json`: where each loop starts
in each clip.

## Ears 13b (batch 6's levered candidates, and twang)

`make_jobs_13b.py OUTDIR` writes the job lists for the seven candidates that
waited on item 99's levers, twang and the sampler (`docs/ears/batch-6.md`
says what each measured). Five are levers only and render on any sketch
build without `"sketch"`: tripletKit, dottedPulse and planingSevenths are
forced levers, slowOpening and tuningOffset are `set` constants, widePad is
a `space` pin (with the width ceiling raised too: `spread-chords` alone
stops at the ceiling). dropSpotlight needs its build (and
`dropSpotlightSwell/` for the swelling kind), twang one build per version
and loop:

```sh
python3 -P tools/ears/sketches/ears13/make_jobs_13b.py /tmp/j
node tools/clips.mjs /tmp/j/tripletKit.json /tmp/r/tripletKit     # and the other levers
cp tools/ears/sketches/ears13/dropSpotlight/{block.rs,where.txt} core/sketch/
cd core && cargo build --release --locked --features sketch --example dlrender && cd ..
cp core/target/release/examples/dlrender /tmp/dl-drop
node tools/clips.mjs /tmp/j/dropSpotlight.json /tmp/r/dropSpotlight --bin /tmp/dl-drop
python3 -P tools/ears/sketches/ears13/twang/variant.py slow 58   # thaw #71's tempo
cd core && cargo build --release --locked --features sketch --example dlrender && cd ..
git checkout core/sketch
```

widePad's 13b jobs are `widePad-13b.json` (Ears 13's `widePad.json` is its
block's). twang's loops and tempos: vapor #70 53, thaw #10 67, shrine #13 45, thaw #71
58 BPM; its jobs are `twang-<loop>.json` (the `none` tag on any build). The
sampler's five `every13b-*.json` render with the plain build except grove #47
(the dropSpotlight build) and thaw #71 (twang soft at 58). The clips:
`make_clips.py clip_plan_13b.json RENDERDIR CLIPDIR` (twang's renders from
`RENDERDIR/twang/`, the sampler's from `RENDERDIR/everything13b/`).
`kit-report-13b.txt` has the two blocks' kit lines.

Every dropSpotlight and twang job sets `chords.RETUNE=1` (no silent organ).
The melody channel hears no note events at all, so twang follows the chords,
as lowBell does; a melody voice needs a lever (batch 6, "Levers wanted").
