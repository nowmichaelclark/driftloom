# Golden renders

The core's promise 3 (`docs/CORE.md`): the same code makes the same samples,
on every device. This directory is how that is measured (queue item 42).

| File | What it is |
|---|---|
| `codes.txt` | The fixed set of loops: 20 that between them play every voice on every channel, and one long busy one. One line each: name, seconds to render, quality, share code. |
| `NAME.dlnotes` | A loop as the core hears it: every call the app's host makes to the core, in order, as plain text. Everything needed to render it with no JavaScript. The format is documented in `examples/dlrender/notelist.rs`. |
| `TARGET.hashes` | What each list renders to on that target: one line a list, with the FNV-1a 64 hash of the speaker output, of each of the five channels as it enters the mix, and of the signal at the bus compressor. |

## Checking

```sh
tools/golden.sh                              # this machine
tools/golden.sh aarch64-unknown-linux-gnu    # under QEMU (see core/.cargo/config.toml)
tools/golden.sh wasm32-unknown-unknown       # the app's engine, in Node
tools/golden.sh --report                     # every kept target against wasm32
```

CI runs it for five targets (`.github/workflows/core.yml`, job `golden`).
Each target is held to its own `TARGET.hashes`: until the core carries its own
math (queue item 43) the platforms' `sin`, `exp` and `powf` differ in their
last bits, so the targets do not agree with each other, and `--strict` (which
fails on any disagreement with wasm32) is switched on in item 43.

## Rendering by hand

```sh
cd core
cargo run --release --example dlrender -- golden/03-tide.dlnotes
cargo run --release --example dlrender -- --wav /tmp/tide.wav golden/03-tide.dlnotes
cargo run --release --example dlrender -- --compare a.wav b.wav
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
so a tail decaying through subnormal numbers is zero in the page and not
elsewhere. Item 43 decides how the core treats them.)
