# Golden renders

The core's promise 3 (`docs/CORE.md`): the same code makes the same samples,
on every device. This directory is how that is measured (queue item 42).

| File | What it is |
|---|---|
| `codes.txt` | The fixed set of loops: 20 that between them play every voice on every channel, and one long busy one. One line each: name, seconds to render, quality, share code. |
| `NAME.dlnotes` | A loop as the core hears it: every call the app's host makes to the core, in order, as plain text. Everything needed to render it with no JavaScript, and nothing left to chance: each note is preceded by its `key` (the loop's seed, the pass, the step and its place among its layer's notes), and the core draws everything the note leaves to chance from that (queue item 44). The format is documented in `examples/dlrender/notelist.rs`. |
| `TARGET.hashes` | What each list renders to on that target: one line a list, with the FNV-1a 64 hash of the speaker output, of each of the five channels as it enters the mix, and of the signal at the bus compressor. |

## Checking

```sh
tools/golden.sh                              # this machine
tools/golden.sh aarch64-unknown-linux-gnu    # under QEMU (see core/.cargo/config.toml)
tools/golden.sh wasm32-unknown-unknown       # the app's engine, in Node
tools/golden.sh --report                     # every kept target against wasm32
tools/golden.sh --strict --flush             # flushing subnormals (x86-64, aarch64)
```

CI runs it for five targets (`.github/workflows/core.yml`, job `golden`),
with `--strict`: every target must make exactly wasm32's hashes. Since queue
item 43 they do, because the core carries its own math (`core/src/math`)
instead of each platform's `sin`, `exp` and `powf`, and never makes a
subnormal number. `--flush` renders with the processor flushing subnormals to
zero, as Chromium's audio threads do; it must change nothing, and CI checks it
on x86-64.

## Block sizes and rates (queue item 45)

```sh
tools/golden.sh --blocks [all] [TARGET]   # the lists, a host's way: any frames a call
tools/golden.sh --rates [TARGET]          # three lists at 22,050 to 192,000 Hz
tools/golden.sh --rates --update          # rewrite rates.hashes
node tools/rates.mjs                      # level, tone and aliasing against 48 kHz
```

`--blocks` renders each list through `Core::render` in calls of 1, 64, 128,
441 and 1,024 frames and of a different size every call, and fails unless
the speaker output is the plain render's, to the bit. `--rates` renders the
lists `rates.txt` names at the rates it names and compares with
`rates.hashes`, one file for every target (they must all make the same
samples); the 192 kHz renders in Node use the wide build
(`tools/build-core.sh --wide`), since the web build holds 96 kHz.

## Rendering by hand

```sh
cd core
cargo run --release --example dlrender -- golden/03-tide.dlnotes
cargo run --release --example dlrender -- --wav /tmp/tide.wav golden/03-tide.dlnotes
cargo run --release --example dlrender -- --compare a.wav b.wav
cargo run --release --example dlrender -- --rate 96000 golden/03-tide.dlnotes   # NAME@96000
cargo run --release --example dlrender -- --blocks 441,random golden/03-tide.dlnotes
```

`--wav` writes a mono 32-bit float WAV (`--signal melody` and so on for a
channel instead of the speaker output), which any editor opens. It is the seed
of the app's audio export (roadmap 17).

## Changing a golden

A sound that moves on purpose moves its hashes. Re-export the lists if the
app's host calls changed, then re-render on every target and say in the PR
what moved and why:

```sh
node tools/golden-export.mjs                           # the lists, from codes.txt (needs Chromium)
tools/golden.sh --update wasm32-unknown-unknown        # and the same for each target in CI
```

`golden-export.mjs` plays each code through the real Synth and Engine in
headless Chromium, records the host's calls, and checks the list: rendered in
Node through `js/dlcore.wasm`, its speaker output and its bus must hash to
exactly what the page rendered. (The channel taps need not: Chromium turns on
flush-to-zero on its audio threads, which the worklet's WebAssembly runs on,
so a tail decaying through subnormal numbers was zero in the page and not
elsewhere. Since item 43 the core never makes one, so they agree; the check
still compares only the two signals it always did.)
