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
| `widePad/` | the chord layer's side: renders nothing in stereo yet (a whole-channel block gets no side; batch 6, Levers wanted) | chords |
| `everything/` | the sampler: lowBell, colorTone and padWow in one block; `use.sh b|c|w...` writes a variant with its parts beside it | chords |

deepDuck and wideSweep are not blocks: they are the approved subDuck and
padSweep with their settings pinned (`jobs/deepDuck.json`, `jobs/wideSweep.json`).

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
