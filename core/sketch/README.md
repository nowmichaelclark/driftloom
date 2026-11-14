# Sketches on the core (queue item 83)

For ears and brain sessions. A sketch is a new sound tried on the core itself,
not on the JavaScript synth that retires at the end of phase 3. It is one block
file and one line of data, built only with the `sketch` feature, so the app's
`.wasm` never carries it.

## Drop one in

1. **`block.rs`** here: the kit's interface (`core/README.md`, "Adding a
   building block"), and nothing else. It defines `pub struct Sketch` (with a
   `const fn new()` of all zeros and `Default`), `pub const REST` (the
   settings it starts with), `impl Block for Sketch` (at least one probe)
   and `impl Channel for Sketch {}` (`WHOLE` when it works on the whole
   channel; `SENDS` when it sends into the reverb and the echo; a source or a
   processor on the channel's bus otherwise). The file in this folder now is
   the demonstration: a veil, a lowpass mixed under the melody.
2. **`where.txt`**: one line, `channel LAYER`, the layer it plays on (melody,
   chords, bass, texture or drums).

`src/blocks/sketch.rs` is the one fixed line between them and the registry.
It adds a setting, `on`, in front of the sketch's own. Every channel holds a
copy of the block, so `dlrender --sketch` and `tools/clips.mjs` turn it on in
the layer `where.txt` names and off in the rest. Without them it plays
nowhere, and a sketch build renders every loop bit for bit as the plain build
does.

Not built yet: a sketch **voice** (a block a layer's notes play instead of its
own voice, by number). The registry's `voices` list takes no `#[cfg]` yet;
see `docs/archive/notes/83-sketch-voice.md`. A sketch of a new instrument today is a
channel source that follows the layer's events.

## Probe it

```sh
cd core && cargo test --lib --features sketch kit
KIT=report cargo test --release --lib --features sketch every_block -- --nocapture | grep sketch
cargo run --release --features sketch --example dlrender -- --block sketch/soft --wav soft.wav
```

The kit plays it like any block (level, tone with the 2-5 kHz share against
the flute's, cost, determinism, modulation, stereo, events, settings) and
fails what any block would fail. A sketch has no stored render in
`golden/blocks.txt`: it changes whenever its session does, and the stored
renders are the app's.

## Hear it

```sh
node tools/clips.mjs jobs.json OUTDIR --opus
```

with a job such as `{"name":"veil","corpus":[20261009,57],"seconds":30,
"sketch":true}`. The jobs, the levers (`force`, `set`, `pin`, `from`, `lite`,
`wander`, `vibes`) and the index are documented in the header of
`tools/clips.mjs`. Compare a clip with the same job without `"sketch"`.

## The A/B levers

- **Force a choice:** `"force": {"lateFloor": "on"}` (or a lever, below) makes every loop in the
  job draw that option, eligible or not (`"today"`: the first option). Once
  item 85 lands this moves onto the request.
- **Move a constant:** `"set": {"bass.LIFT_DB": 3}`. The table is
  `core/src/tuning.rs` (`NAMES`); a constant joins it with one line there and
  one `tuning::get` where the core reads it, under `#[cfg(feature = "sketch")]`.
- **Pin a setting:** `"pin": {"sub_duck.depth": 0.5}` holds a channel block's
  setting at that value whenever the loop sets it (organ, wash, sub_duck,
  pad_sweep, sketch).

### The levers of item 99 (Ears 13b)

Each is off until a clip pulls it, and off it renders bit for bit as the
plain build does. All are `force`, `set`, `space` and `route` in a job (and
`--force`, `--set`, `--space`, `--route` in `dlrender`); the header of
`tools/clips.mjs` lists them. The forced ones show in the index's `drew`.

| Candidate | Lever |
|---|---|
| tripletKit | Now a choice of the data (queue item 101d, `generator::kit`): `"force": {"tripletKit": "on"}` forces it as any choice, and a request pins it (`tripletKit=on`). The lever's re-timing is gone: the build moves the eighth to two thirds of the beat and writes the triplet hats. |
| dottedPulse | `"force": {"dottedPulse": "pulse"}` adds a soft rim (0.2) every 6 steps, across the kit's layer; `"kick332"` puts the kick on 0, 6, 12 of a 16-step bar. |
| planingSevenths | `"force": {"planingSevenths": "on"}`: every chord a major seventh, the first on the tonic, each root a whole step or a minor third from the last (turning back past a fourth from the tonic). The bass follows the root, the tune keeps the scale (so it can clash: the sketch finding), and the loop's other choices are drawn over the new harmony, so they can differ. |
| slowOpening | `"set": {"latefloor.SECONDS_MAX": 90}` (the same draw, over a wider range; force `lateFloor: on` to have it) and `"latefloor.FADE": 12` (seconds the kit and the bass take to rise once they arrive, a velocity ramp). |
| tuningOffset | `"set": {"master.CENTS": -25}`: every pitched voice, where a note becomes Hz (`blocks::midi_to_freq`). The tuned kick and the noise voices keep theirs. |
| widePad | the smaller of the two: `"space": {"spread-chords": 1.4}` (also `"spread-max"`, `"spread-scale"`; `"0.6:1.2"` draws a range) sets a space setting past its range. On corpus loop 45 (the whole mix, above 250 Hz) the side against the mid went from -14.5 dB to -6.9 dB with `spread-chords=2, spread-max=4, spread-scale=3`, the mid unchanged to four places. A block that adds its own width to the side from a whole-channel block is *not* possible: the core calls `render_side` on bus blocks only. |
| dropSpotlight | `"route": {"sketch": "drums"}` (or `bass`, ...; `"sketch.1"` for mod 1) routes the layer's signal into the sketch's mod, as the kick is into the bass's duck. The sketch declares `const MODS` (see `sub_duck.rs`); without one `dlrender` says so. |
| every chord sketch | `"set": {"chords.RETUNE": 1}`: the chords channel gets a retune to the whole chord at each chord of the progression (an arpeggio's notes together), where today only the organ's loops retune. A loop with the organ keeps its own; held voices that hear a retune (the choir's) follow it too. |

## Rules

- A sketch is for hearing and measuring. It never goes in the app: the hands
  build the approved sound as a real block, with the standing rules.
- Plain ASCII, no allocation, no panic, no dependencies, as every block.
- Clean under `cargo clippy --all-targets --features sketch -- -W
  clippy::pedantic`, or CI fails (it builds the folder as it stands).
