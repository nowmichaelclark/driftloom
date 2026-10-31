# dlcore: Driftloom's audio core

The engine under Driftloom: the code that makes the sound. It is the same
code in the website (as WebAssembly), in a native build, and in someone's
game. It is plain Rust with `no_std`, no dependencies and no threads, and
it allocates nothing after `Core::new`. The same notes make the same samples,
bit for bit, on every target. Its promises and how each is measured are in
[`docs/CORE.md`](../docs/CORE.md).

This guide is for developers and AI agents. It covers how a host drives the
core, how the sound is built, and how to add to it. The rules for changing
the core (what every PR must hold, and the rules for new sounds) are in
[`AGENTS.md`](AGENTS.md) beside this file.

## Hosting it

A host owns one `Core` and makes a handful of calls:

```rust
use dlcore::{Core, Key, MELODY, KALIMBA, EXTRA};

let mut core = Box::new(Core::new());          // megabytes of zeros: keep it off the stack
core.start(48_000.0, 1)?;                      // any rate from 22,050 to 192,000 Hz; mono (2: stereo)
core.key(Key { seed: 7, pass: 0, step: 0, ord: 0 });      // where the next notes sit in their loop
core.note(KALIMBA, MELODY, 0.5, 69.0, 0.4, 0.8, 3, [f64::NAN; EXTRA]);
let mut out = vec![0.0f32; 441];               // any number of frames a call
core.render(&mut [&mut out])?;
```

- **`start(rate, channels)`** sets the rate and the output layout: mono (1) or stereo (2); anything else returns a plain error. The two are renders of their own (queue item 64; `docs/CORE.md`, promise 9). The mono render is the one the core has always made and pays nothing for stereo; in the stereo render the mix's mid is the mono mix, the master chain runs on each side (its compressors keyed by the mid, so they press as in mono), and the width ceiling (`blocks::width`) turns the side down where it gets wider than its target. With nothing placed or widened, each side is the mono render, bit for bit. `set_layout` switches between them: before the first block it is the render from the start; while the core plays (the app's Mono switch, queue item 66) it fades, so the sound goes on without a click: into stereo the master takes two lanes at once and the side fades in from silence over `LAYOUT_FADE` (0.25 s); into mono the side fades out, and once the two lanes agree the master takes one. `lanes()` says how many the last block rendered (two until then); a mono host is given the left lane meanwhile.
- **`note(voice, channel, time, midi, dur, vel, parts, extra)`** queues a note. It answers whether the note is on time, late, missed or refused (`Taken`).
- **`key(key)`** says where the notes that follow sit in their loop. Every chance a note leaves open is drawn from this key, so the same loop sounds the same everywhere.
- **`render(out)`** fills the host's buffers with the next frames: one buffer a channel of the layout last asked for.
- **`event(channel, event)`** sends a damp, a release, a bend, a retune or a choke (`Event::new`, `Event::retune`) to a channel. Each kind says how far it reaches the channel's notes (`event::reach`):
  - **once** (a damp, a release): each note hears it at most once, the notes that arrived since the channel's last event of that kind;
  - **every time** (a bend, a retune, a choke): every note held when its time comes hears every one.

  The channel's own blocks hear every event when its time comes. A voice or a block decides what it does with one; today a strummed nylon string listens to a damp, the chords' organ (`blocks::organ`, the first channel block the app plays) to retunes and releases, and the tuned kick (`blocks::kick`) to a choke: the host sends one with every kick of a loop that plays it, and a kick still ringing falls away in a few ms (one kick at a time), while the hit it was sent with passes it by.
- **`set(channel, block, setting, value, time)`** sets a setting of one of a channel's own blocks for the loop: a sweep's period, a duck's depth, an organ's registration.
- **`route(channel, block, setting, from, time)`** routes another channel's signal (`Tap`) into a setting a signal can drive on one of a channel's own blocks: the kick into the bass's duck. `None` undoes it.
- Also:
  - `param` (the mix and the master chain);
  - `damp` (a new strum: a damp event, the web host's call);
  - `cancel` and `cancel_from` (the music jumped);
  - `quality`;
  - the counts: `late`, `missed`, `dropped`.

`examples/dlrender` is a whole native host in a few hundred lines. It reads
a note list and writes a WAV:

```sh
cargo run --release --example dlrender -- --wav out.wav golden/03-tide.dlnotes
```

The web host is `js/worklet.js`, through the exports in `src/wasm.rs`.

## How the sound is built: blocks

Everything that makes or shapes sound is a **block**: one file in
`src/blocks/`, behind one small interface in `src/block.rs`. There are only
three things to know.

1. **`Block`**:
   - `start(settings, cx)`: the block is set once by its `Settings`.
   - `render(block, cx, signal)`: the block renders 128 frames at a time into one `Signal`, in place. A *source* (an oscillator, a voice) adds its sound into the signal. A *processor* (`INPUT = true`: a filter, a body) replaces the signal with its output.
   - `span()`: the frames the block sounds in.
   - `PROBES`: how the test kit plays it.

   A **`Voice`** is a block whose settings are a `Note`. It is the kind a host plays by number. It may also settle the note's chances (`settle`), say that it plays the noise (`uses_noise`), play through one of its channel's own blocks (`THROUGH`), send some of itself past its channel straight into the reverb and the echo (`SENDS`, `render_sends`: dubStab's throw), and hear channel events (`event`).

   Three seams are optional parts of `Block`, each with a default, so a block that doesn't use one writes nothing for it:
   - **Modulation:** `MODS` names the settings a signal can drive, and `render_mod(block, cx, signal, mods)` renders with them driven. At every frame a driven setting is its own value plus the signal, as Web Audio sums an `AudioParam` and the node connected into it. A vibrato is an `Lfo` into an `Oscillator`'s `DETUNE`; FM is an `Lfo` at audio rate into its `FREQ`; a filter sweep is an envelope into a `Filter`'s `FREQ`.
   - **Stereo:** `render_stereo(block, cx, &mut [left, right])`. The core's mono render never calls it, so mono pays nothing. By default a mono source plays centered (its mono render on each side, bit for bit), and a mono processor hears the two sides folded to one; `Pair` runs one copy of a processor on each side. A block with a stereo render of its own (a panner, a room) says `STEREO = true`.
   - **Channel events:** `HEARS` lists the kinds a block listens to, and `event(&Event)` hears them.
   - **Settings for the loop:** `SETTINGS` names the numbers a host sets on a channel's own block, and `set(setting, value, time, cx)` takes them when their time comes.
2. **`Context`**: everything a block may read besides its own state: the sample rate, the wavetables and the noise.
3. **The registry**, the `registry!` list in `src/blocks/mod.rs`: one line a block. The line declares the block's file, and the core finds every voice through it, with no `dyn`. The kit finds everything there too. Its `channel` list names a channel's own blocks (`Channel`): blocks each channel keeps for as long as the core runs, outside the voice pool. One runs on a bus its voices sum into (the fiddle, nylon and accordion bodies), on the whole channel (`Channel::WHOLE`: a sweep, a duck), or adds its own sound (a held organ, a wash). A shared resonance, space or held instrument is one file and one line there.

Every block keeps all of its state inline, in a plain struct whose
`const fn new()` is all zeros. Nothing in a block allocates or panics.
Inside a voice, a setting that moves over time is a `param::Param`, the
core's AudioParam, and a setting another block drives is one of the part's
`MODS`. The voices are built from the parts in the same directory (`osc`,
`lfo`, `filter`, `noise_source`) and call them directly, by type.

## Adding a building block

`src/blocks/tremolo.rs` is a demonstration: a tremolo, in the tests only.
It is the whole recipe:

1. **One new file**, `src/blocks/NAME.rs`, with:
   - a struct holding the block's state;
   - a `const fn new()` (all zeros) and `Default`;
   - its `Settings`;
   - `impl Block`, with at least one probe.
2. **One line** in the registry's `parts` list in `src/blocks/mod.rs`, for example `tremolo::Tremolo;`. The tremolo's line also carries `#[cfg(test)]`, which keeps it out of the app.
3. **Store its renders** for the kit: `KIT=update cargo test --lib kit`. This adds lines to `golden/blocks.txt`; commit them.

That is all; the kit's test (`each_demonstration_block_is_one_file_and_one_line`)
checks that the tremolo is named nowhere else.

Two more demonstrations show the seams are one file and one line too:

- `src/blocks/amp.rs`, an amplifier whose gain a signal can drive. It lists its setting in `MODS` (with the swing the kit should probe it at) and writes `render_mod`. The kit probes the input with no test written for it.
- `src/blocks/pan.rs`, a stereo panner. It says `STEREO = true` and writes `render_stereo`. The kit checks and stores its two sides.

Three more show a channel's own blocks are one file and one line too (queue item 46c). Each line in the registry's `channel` list carries `#[cfg(test)]`:

- `src/blocks/swell.rs`, **settings for the loop.** A swell on a whole channel (`Channel::WHOLE`) carries its own `Lfo`, and lists the rate and depth a host sets in `SETTINGS`. `set` starts the swing again from the setting's time. A host calls `core.set(MELODY, own::SWELL, 0, 1.5, time)`. The kit probes each setting (`swell/slow=rate`).
- `src/blocks/chorale.rs`, **a held instrument retuned many times.** Four voices a channel holds. It lists `RETUNE` in `HEARS` and, in `event`, moves by the shared rule `blocks::retune`: a note two chords share sounds on untouched, and only the changed notes are let go and spoken again. A host calls `core.event(CHORDS, Event::retune(time, &[60.0, 64.0, 67.0]))` at each chord. The kit sends five chords (`chorale/triad@retune`).
- `src/blocks/duck.rs`, **one channel's signal into another's block.** A gain on a whole channel with a `key` in `MODS`. A host routes the drums into it: `core.route(BASS_CHANNEL, own::DUCK, 0, Some(Tap { channel: DRUMS_CHANNEL, block: None }), time)`. A routed signal is its source's previous block (one block's delay), so channels render in any order and the result is the same on every target and block size. The kit keys it with a sine (`duck/half~key`).

The chapel organ (`src/blocks/organ.rs`, queue item 61) is the chorale grown up, and the first of these the app plays: one line in the `channel` list (`organ::Organ = ORGAN (organ::REST)`), silent at rest, with its registration and the rest in `SETTINGS` (the host sets all 21 at a loop's first step), `RETUNE` at each chord and `RELEASE` to let go. It sends some of itself into the room past the channel's own sends (`Channel::SENDS`, `render_sends`, as a voice's `SENDS` does). Two things it shows: a channel's own block lives in the core's static memory in every channel, so its `new()` must be all zeros (a NaN or a 110 Hz default there puts the whole core into the `.wasm`, 14 MB) and it sets its real starting values in `start`; and an idle one returns at once from `render`, so a channel that never plays it costs nothing and renders bit for bit as before.

subDuck (`src/blocks/sub_duck.rs`, queue item 68) is the duck the app plays: on the whole bass channel, keyed by the drums routed in, it hears a kick as its key's low band (under 150 Hz) rising, and dips the bass by its depth (a setting, scaled by how loud the kick rose) in 8 ms and back over its recovery (a setting), as the sketch's gain did at each kick's time. Its depth at 0 costs nothing.

padSweep (`src/blocks/pad_sweep.rs`, queue item 72) is the swell's way with a filter: on the whole chords channel, its own `Lfo` drives its own `Filter`'s frequency (`Mods`), so the cutoff swings from its closed point (a setting) up by its octaves (a setting) and back once a period (a setting), with the Q a setting too. A new sweep starts closed at its time; across a load the cutoff glides onto the new loop's curve over half a second, and a loop without one glides it open and then stands still, costing nothing. `Filter::retarget` changes a running filter's settings and keeps its memory, so the change doesn't click.

The wash (`src/blocks/wash.rs`, queue item 62) is the duck grown up: a source on the core's sixth channel (`WASH_CHANNEL`, which no host note plays into, set like the textures' in the mix), whose level follows the other five layers through five routes into its `MODS`, one a channel, each weighted by a setting (the channel's gain in the mix, 0 for a muted one, since a route taps a channel before its gain). A block may follow every channel at once: `MAX_MODS` is 6. Note lists carry the routes (`route`), and hash the sixth channel (`wash=`).

The width ceiling (`src/blocks/width.rs`, queue item 64) is a stereo processor in the master chain, after the ceiling: it measures the mid and the side above 200 Hz over the last 2 s and turns the side down where it rises above its target against the mid (down in about half a second, back over about three), leaving the mid, and so the fold to mono, untouched. Its settings are named (`width::SETTINGS`; `Core::param`, `WIDTH_TARGET` to `WIDTH_UP`). The kit feeds a stereo processor two different sides too (`width/ceiling:wide`), since a side-only block has nothing to do with the same signal on both.

The space (`src/space.rs`, queue item 65) is where each part sits in the stereo render, as data: a recipe of named settings, each with the range a loop draws it from (`space::SETTINGS`), drawn from the loop's seed, how busy and how wet it is, and whether its texture is light (`Core::param`, `SPACE_SEED` to `SPACE_START`). A host narrows or pins any setting (`Core::space_range`). The mix makes the side (`mix::Side`): the room (the reverb through four allpasses), the spreads (a swaying copy of the chords, texture and melody), the placements (the light kit, a light texture, glass) and the wash's own width (`Channel::render_side`), all highpassed so the low end stays centered, and riding the same gains, duck and tails as the mid, which is the mono mix bit for bit. The master gets left = mid + side and right = mid - side. With no space, or in a mono render, none of it runs.

The bass layer's level is a setting of the loop (queue item 76c): `Core::bass_lift(db)`, sent by the host at each load (`dl_bass_lift`; `js/choices.js`, `BASS_LIFT`: +3 dB in a loop that plays a kick, 0 without). The core stamps it on each bass note as it arrives, so every bass voice but the FM `rhodesbass` is lifted alike, a loop handed over keeps its own, and a host that never sends one plays today's level.

The arc (`src/blocks/arc.rs`, queue item 63) is a processor in the master chain rather than a channel's: a lowpass between the tone and the highpass, one in each of the master's lanes (`Pair`'s way, ready for stereo), breathing over minutes and lifting the echo's feedback as it opens (the mix takes the lift, frame by frame). A host starts it through `Core::param` (`ARC_FROM`, then `ARC_PERIOD` at the loop's start). With no arc it is no filter at all.

**A caution on modulation:** an oscillator's start follows Chromium's rule. Connecting any signal into its frequency, even a zero one, makes the pitch "moving", which changes how the note's first block starts. That is exact, and the goldens depend on it, but a recipe that connects a modulator "just in case" changes the sound.

A **voice** is the same, with three differences:
- It implements `Voice` as well as `Block`.
- Its line goes in the `voices` list with the numbers a host plays it by: `kalimba::Kalimba = KALIMBA 0;`. The numbers are the host's, fixed for good, so take the next free one.
- The page needs to know the number too (`CORE_VOICES` in `js/core.js`).

A voice too big for a slot of the pool goes in `pooled`, with the size of
its own pool.

## The test kit

`cargo test` plays every block in the registry alone, at 48 kHz, through
each of its probes. For each probe it checks:
- **level:** the samples are finite, the block is not silent, and it is not running away;
- **tone:** the 2-5 kHz share of the A-weighted energy (the coffee-shop test);
- **determinism:** two renders are the same;
- **cost:** a release build checks it against a ceiling;
- **a null test:** a hash against the stored render in `golden/blocks.txt`;
- **modulation:** each setting in `MODS`, driven on every probe by a slow sine (`osc/sine~freq`), checked the same way. A setting that changes nothing on any probe fails: it isn't wired;
- **stereo:** a mono block in a stereo signal must put its mono render on each side, bit for bit (a source centered, a processor fed the same on both sides, and as a `Pair`). A `STEREO` block's two sides are checked and stored (`pan/left:stereo`), and must differ on at least one probe;
- **events:** each kind in `HEARS`, sent on every probe (`block::probe_events`: a damp, a release, three bends, five chords), checked the same way (`nylon/strum@damp`). A kind that changes nothing on any probe fails;
- **settings:** each of `SETTINGS`, set to its probe value at the start (`swell/slow=rate`). A setting that changes nothing on any probe fails.

An equal hash proves a change identical. Any other hash is a sound change.
The test fails with the stored and new level and tone. A change on purpose
is stored with `KIT=update`, and its PR says what moved and why, for an A/B.
To hear a probe from two versions:

```sh
cargo run --release --example dlrender -- --block fiddle/long --wav new.wav   # or osc/sine~freq, pan/left:stereo, nylon/strum@damp
# (the same on the old version, to old.wav)
cargo run --release --example dlrender -- --compare old.wav new.wav
```

`KIT=report cargo test --release --lib every_block -- --nocapture` prints
the whole table.

## The generator (phase 2)

`src/generator/` is `js/generator.js` in Rust, ported note for note (queue
items 79 to 81): a loop's spec, the plan `render` draws before the layers,
the layers themselves and `render`'s choices, each choice's own draws
(`js/choices.js`), what the engine decides at load (`Engine.load`: the
per-loop settings the host sends the core) and each wandered pass
(`drift`), from its seed, its vibes and the generation data: every decision
the JavaScript makes before scheduling. The data, `data/*.txt`, is compiled
in by `build.rs` as read-only tables, so a host hands nothing over and an
edit to a data file rebuilds the core. A host keeps one
`generator::Generator` (fixed lists, 1,406,840 bytes on 64-bit targets,
1,090,016 on 32-bit; nothing allocates) and calls `load(seed, vibes)`, then
`settings()` for the load's decisions and `wander(pass)` for a pass as it
wanders (any pass, without the ones before it: a pass is the loop's own
events with what it changed marked on them, a few bytes an event); `hash`
and `write` give each part's canonical text (`tools/gen-golden.mjs`
documents it). A load takes about 0.5 ms on a laptop, 11 ms at most: a loop
with a tuned kick works out the kick's level by ear there (`kickOf`'s fit,
about 10 ms, a tenth of the JavaScript's), and that is nearly all of it;
the player spreads it across blocks (`Generator::begin`, then `step` a stage
at a time; the fit itself resumes in slices, `choices::KickFit`). The app's
`.wasm` leaves the generator out until the app plays the player (phase 3);
the `generator` feature builds its exports (`dl_gen_load`, `dl_spec`, ...)
for the checks.

## The player (phase 2)

`src/player/` plays a loop from its seed with no JavaScript (queue item 82):
the generator makes it, the sequencer (`sequencer.rs`, `Engine` in
`js/engine.js`) walks it step by step, pass after pass, wandering as the
app's does, and the synth's note calls (`synth.rs`, with the voice budget,
`budget.rs`, and the sung voices' trims, `formant.rs`) hand each note to the
core with everything it carries. The baseline's 40 loops, full and lite,
mono and stereo, 45 s with the wander on, make the same samples as the app's
own engine in Chromium, bit for bit (below).

```rust
use dlcore::Core;
use dlcore::generator::Layer;
use dlcore::player::{Player, Request};

let mut core = Box::new(Core::new());           // 17.4 MB, and the player 1.5 MB:
let mut player = Box::new(Player::new());       // keep both off the stack
player.start(&mut core, 48_000.0, 2, true)?;    // rate, channels, full mix (false: lite's)
player.load(1234, &Request::new().with_vibe("peaceful").unwrap_or_default());
player.render(&mut core, &mut [&mut left, &mut right])?;  // again and again, any frames a call
player.set_mute(&mut *core, Layer::Drums, true);          // and set_wander, set_tempo, set_volume
```

- **`load(seed, request)`** asks for a loop; it is made a stage a block over
  the blocks rendered next (a few, up to about a hundred: the spec, the plan,
  the render in five parts, the load's decisions, the kick's fit in slices,
  the first pass), then starts, its first step `START` (0.05 s) on. A loop
  playing stops being decided at once and what it handed over plays out.
  `load_at` starts it at a time on the core's clock instead (the page's
  cursor keeps in step that way), and `ready` makes it at once for a host
  with no deadline (an offline render, `dlrender --seed`).
- **Before each block** the player decides every step due within
  `LOOKAHEAD` (0.25 s) and hands its notes over (`decide`; `render` calls it
  for a host, `Core::render_with`). Each pass's wandering is drawn a pass
  ahead, in a block where no step is decided.
- **The controls** are the app's: `set_wander` (0 plays the loop as
  composed), `set_mute`, `set_tempo`, `set_volume`, `stop` and `resume`.
- **The C header**, `include/dlcore.h` (promise 4), has the same calls
  (`dl_player_start`, `dl_player_load`, `dl_player_render`, ...): build the
  library with `cargo rustc --release --lib --features capi --crate-type
  staticlib`; the host gives each player its memory. `examples/c/seed.c` is
  a whole C host, a seed to a WAV.
- **On the web** the `player` feature builds `js/dlplayer.wasm` with the
  `dl_play_*` exports; the app plays it with `?engine=player`, for testing,
  its own engine keeping only the cursor and the controls.

Why a player's render depends on when it hands notes over: the core keeps its
waiting notes in no fixed order (a note that starts takes the last one's
place), so which voice a note takes, and the order the voices are summed in,
depend on what is waiting. The player's way is fixed (a block at a time,
`LOOKAHEAD` ahead), so its renders are the same everywhere; handing a whole
render over before it starts, as `tools/measure.mjs` does, sums the same
notes in another order (inaudibly: about 88 dB under the render).

Nothing allocates (the generator's lists, two passes and the budget are
fixed). What it costs (`dlrender --seed N --costs`, and
`tools/player-wasm.mjs --costs` for the wasm), at 44.1 kHz, where a block's
deadline is 2.9 ms: natively, the worst block is 0.45 ms (16%), all of it
the core's audio; a load's worst stage is 0.14 ms (5%), a slice of the
tuned kick's fit; making loops is 0.4% of a minute's render. In wasm (Node)
the worst block while a loop is made is 0.28 ms (10%), its audio included.
On every target, QEMU's included, a load's worst stage costs less than the
same loop's worst audio block, so making a loop never makes a loop's worst
block.

## Sketches (queue item 83)

`core/sketch/` is where an ears or brain session drops one block file and one
line of data (`core/sketch/README.md`); the `sketch` feature builds it in as a
channel's own block (`src/blocks/sketch.rs`), probed by the kit with no test
written. The same feature carries the clips' levers: a choice forced
(`Generator::force`), a named constant moved (`src/tuning.rs`), a setting
pinned. `dlrender --seed|--corpus` takes them (`--sketch`, `--force`,
`--set`, `--pin`, `--from`, `--pick`, `--choices`), and `tools/clips.mjs` drives
them from a job list to WAV and Opus clips with an index of what each loop
drew. The feature is never in the app's `.wasm`, the player's or the C
library: with it off, every file that matters is the same bytes, and with it
on and the sketch off, every loop renders bit for bit as before.

## The other checks

- **`golden/`:** whole loops as the app plays them. Each must hash the same on wasm32, x86-64, aarch64, 32-bit ARM and big-endian PowerPC, at every block size and at other rates (`golden/README.md`).
- **`tools/gen-core.sh [TARGET]`:** the Rust generator against the generation goldens (`golden/generation.hashes`): every case, every part (the load and the eight wandered passes too) equal to the JavaScript's hash, on every target CI renders (`examples/dlgen`; wasm32 through a `generator` build in Node). `node tools/stats.mjs --check --core` draws the balance lock's corpus from that build and fails unless every loop's figures equal the JavaScript's.
- **`tools/seeds.sh [TARGET]`:** the corpus's first 80 loops from their seeds, each to a WAV, against `golden/seeds.hashes`, the same on every target (the first few under QEMU); `--browser` holds the baseline's 40 loops in the four ways to the browser's renders (`tools/baseline.mjs`'s `player` part), natively and in `js/dlplayer.wasm`; `--c` builds the C host against the header alone and holds it to the same hashes.
- **`tools/baseline.mjs`:** the core in the real app, in Chromium: every voice, 40 loops, the refusals, and the 40 loops played from their seeds as the player plays them (`tools/player.mjs`).
- **`tools/build-core.sh --check`:** the committed `js/dlcore.wasm` and `js/dlplayer.wasm` are exactly what this source builds. Run `tools/build-core.sh` after any change here.
- **Clippy, pedantic:** run `cargo clippy --all-targets -- -W clippy::pedantic`; it reports zero, and CI fails on any warning (queue item 74). Every public item has a doc comment that gives the reason, not only the what.
