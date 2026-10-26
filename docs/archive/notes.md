# Notes from Claude Code, until 2026-10-26

What the hands left at the bottom of the queue when an item stopped, needed a
decision, or finished with something the brain should read, as written. From
2026-10-26 each new note is its own file in `docs/notes/`, with the item number
in its name; this file is closed.

- **Item 30, the stall hunt: no cause in item 30 found; still left
  unmerged (2026-10-09, Session 10).** Decided by the brain (#154): the long quanta are
  the VM's, the fill-ins even; #148 merged. As the brain asked.
  - **What the long quanta are** (`perf.mjs --trace-extra --trace-dir`,
    V8 GC, wasm and Web Audio per-node categories, plus a sampler reading
    the audio thread's faults and context switches from /proc every 20 ms
    on the trace's clock). Every quantum over 15 ms, in both builds, is
    either off-CPU (15-322 ms of wall time for 0.5-20 ms of CPU) or
    "CPU" that lands in whatever happened to be running: in item 29 inside
    native DelayNode and gain processing (64 ms), in item 30 inside
    `process()` (56 ms), once inside Chromium's own "compare topology" step
    before any script (25 ms). The machine was 6-13% busy (31% at 4x). One
    item-29 quantum (34 ms) coincides with the sampler itself, a separate
    process, going 115 ms without a tick: the whole VM stopped. Others show
    the thread preempted (involuntary switches) or blocked (voluntary, no
    faults). The core's own `process()` takes 0.15-0.4 ms.
  - **What item 30 could have added, checked:** the worklet's script is
    item 29's but for comments; views are made once; the core has no
    allocator and its memory is fixed (232 pages, item 29 230), no
    `memory.grow`. The worklet isolate's GCs are all under 0.5 ms and none
    fall in a long quantum. Wasm tier-ups still happen mid-play (one or two
    a run, under 1 ms, on a background thread) and none coincide either.
    The curve swap is 2048 `tanh`s on `setTone` only. **The old JavaScript
    chain does not run**: with the core in, each quantum renders only the
    two compressors, five gains, the worklet and the harness's analyser (no
    delay, oscillator, waveshaper or biquad). The audio thread's minor
    faults (19 a second), voluntary switches (60 a second) and preemptions
    are the same in both builds.
  - **The bisect** was overtaken: with the order swapped (item 30 first)
    item 29 had the long quanta (2 of 30 against 1). So the A/B was rerun
    counterbalanced, each build first in half the passes.
  - **The rerun**, five loops x full/lite x 1x/4x, 80 runs a side: device
    fill-in runs, item 30 **4** (8 events) against item 29 **2** (3); runs
    with a quantum over 20 ms 7 against 1, all on full. Two more full-only
    passes with the sampler, 20 a side each: fill-in runs 0 against 3, long
    quanta 1 against 2. **All 120 a side:** fill-in runs 4 against 5
    (one-sided p 0.75), long quanta 8 against 3 (p 0.11; full 8 of 80
    against 2 of 80; lite 0 of 40 against 1). 90th-percentile worst quantum
    12.0 against 8.5 ms; render +4% (full 88.9 against 83.2 ms/s, lite 61.2
    against 63.2).
  - Not merged. The 80-a-side rerun alone is 4 against 2 fill-in runs, and
    the long quanta still lean against item 30 on full. Pooled with the
    full-only passes, the fill-in rates are even, and nothing found points
    at item 30's code. The brain's call.
- **Item 30, rerun: still left unmerged -- its fill-in rate is worse
  (2026-10-09, Session 10).** Decided by the brain (#154): the long quanta are
  the VM's, the fill-ins even; #148 merged. As the brain asked: #150 (the worklet's block
  count) merged on its own; #148's downsampler is now an FFT convolution as
  Chromium's is (a 256-point FFT, overlap-add; within 2e-6 of the direct
  sum; about half its cost natively), re-nulled (loops -95 to -121 dB,
  mix stage -77 to -85 dB, as before); #148 rebased on `main` (v80). Then
  `perf.mjs --ab`, item 29's core (`main` at #150) against item 30's,
  interleaved run by run, five loops x full/lite x 1x/4x, 30 s windows,
  three passes: 60 runs a side, alone on this 4-core machine.
  - Runs with device fill-ins: item 29 **1 of 60** (1 event: hollow full
    4x at 18.7 s); item 30 **5 of 60** (7 events: undertow lite 1x at
    29.2 s; tide lite 1x at 2.1 s and 22.7 s (2); hollow full 4x at
    13.7 s; shatter full 1x at 28.2 s; tide full 1x at 16.6 s). By
    quality: full 1 / 3, lite 0 / 2. One-sided Fisher p = 0.10 on these
    runs alone; with the earlier 40-run A/B (3 / 0), 8 of 100 against 1 of
    100, p = 0.017. Late ticks 0 everywhere.
  - Worst render quantum per run (Chromium's trace,
    `RealtimeAudioDestinationHandler::Render`, the whole quantum the
    worklet's `process()` runs in): median 4.5 / 5.5 ms, 90th percentile
    10.6 / 28.1 ms; runs over 20 ms 1 / 7, over 10 ms 6 / 15. Four of item
    30's five fill-in runs had a 38-71 ms quantum; the hollow one did not
    (4.4 ms).
  - Audio render, mean: full 82.3 -> 88.1 ms/s (+7%), lite 63.3 -> 61.4
    (-3%). So on lite the core with the chain costs less on average than
    item 29's core with Web Audio's chain, and still had the fill-ins and
    the long quanta: the cost of a block alone does not explain them. A
    block of the core is well under a millisecond; a 40-70 ms quantum is
    the audio thread held up, by something I have not found.
  Not merged, per the rule. #148 stays open with the work; the tally script
  is `perf.mjs`'s JSON (`fillTimes`, `worstRenderMs` per run).
- **Item 11, the temple bell as a chord voice: needs a decision, left as it
  is (2026-09-26).** Decided by the brain from Mikey's ear: item 15. The brief's yardstick is the chords layer's median at
  0.4 s notes, and there the chord templebell is already *above* it:
  -1.8 LU against a median of -2.6 (`measure.mjs --voice all --note 0.4`,
  LU against kalimba). Meeting the yardstick would mean turning it *down*,
  the opposite of "buried". At 1.6 s notes it is under: -5.3 against -3.4,
  1.9 LU. The bell rings 6.5 s whatever the note length, so a short-note
  probe flatters it against voices that stop, and a long one does not. So
  which length decides for a bell is a taste call: lift it about 1.9 LU to
  the 1.6 s median, or leave it and re-listen. The other three parts of
  item 11 went ahead without it.

- **Item 17, 32-bar loops: two thirds is out of reach through the weights;
  a decision if more is wanted (2026-09-27).** Three in four 32-bar loops
  (287 of 378 in 30,000) play their own layer cycles -- the micro-loops
  that slide short fragments against the long frame -- and those never
  develop, by item 12's design: their motion is the sliding. The weight
  reaches only the rest. At 0.72 (the same as 24 bars) 73% of those
  develop, 17% of all 32-bar loops (was 16% at 0.65). Getting
  near two thirds of all of them would mean developing micro-loops too,
  which is a new kind of form rather than a weight: say if you want it.

- **Item 18, short loops can still fall silent: a decision (2026-09-28).**
  The README said loops under 8 bars get no rests at all, "a hole in a
  two-bar loop is a glitch". That holds for the entry schedules
  (`genForm` returns early under 8 bars) but not for the short gaps:
  `genGaps` draws at every length. Over 6,000 loops, 30% of the 2-bar and
  34% of the 4-bar loops take a gap, and 7% and 8% a whole bar of silence
  -- half of a two-bar loop. Item 18 was comments and docs only, so the
  README now says what the code does; whether short loops should keep
  their gaps (or keep only the beat-long ones) is for Mikey's ears. Any
  change re-renders the short loops that have one.
- **Item 18, left alone (2026-09-28).** `ROADMAP.md` item 9 v2 still says
  the three wind voices are 12.4% of melody draws; that was true when it
  was written and reads as history there, so it stays (the README and
  `stats.mjs` now say about 16%). The synth and vowel-drift comments that
  keep articulation off the composer's stream "or every share code in
  circulation renders differently" are still true, and still the reason
  existing loops render identically, so they stay as written.

- **Item 32: pages that still play the JavaScript synth (2026-10-11).**
  `rank.html` and `tools/ears/render.mjs` build a `Synth` directly, so they
  play the JavaScript synth, not the core the app now plays (`rank.html`'s
  header says "always tests what the app plays"). The core nulls to it at
  the floor, so rankings and ears albums already made stay valid; wiring the
  two to the core (the context, `loadCore`, `attachCore`) is a small item if
  the brain wants it.
- **Item 32: device fill-ins in the A/B (2026-10-11).** v82 with the flag
  against this build, same query, 40 runs a side: fill-in runs 1 against 5
  (2 against 8 events), every one of this build's with a worst render quantum
  of 3.6-6.3 ms (no stall in the core). The second A/B (v82 on JavaScript
  against this build), 20 a side, had 0 against 0. Not chased, per the
  brief; the pooled rate for this build is 5 of 60 runs.

- **Item 36: stopped before building, for a decision on reflective
  (2026-10-11, the hands' Session 17).** Measured first, as the item asks:
  10,000 fresh loops whose top mood is reflective (128,471 drawn), by the
  item's own bands: dark 4.9%, inward 32.3%, **neutral 53.8%**, bright
  9.0%. Modes: dorian 31.0%, aeolian 14.4%, kumoi 10.4%, minor pentatonic
  9.8%, mixolydian 4.6%, hirajoshi 4.1%, akebono 3.9%, insen 3.3%,
  whole-tone 3.0%, ionian 3.0%, the rest under 3%. The bands put dorian
  (0.46) and kumoi (0.52) in neutral, while the item counts them as inward
  ("a reflective loop is inward (dorian, kumoi, minor pentatonic, insen)"),
  so the aim can't be read one way:
  - **as the bands are written,** reflective already reaches the neutral
    modes more than half the time and inward is not the largest; widening
    the pull would push neutral further from "inward still the largest";
  - **with dorian and kumoi as inward** (the edge at about 0.53), inward is
    about 81% and neutral about 6% (mixolydian, ritusen), which is the
    item's premise, and widening toward the 0.53-0.75 modes is the change.

  Which reading? Below `INWARD_FROM` it is mostly reflective, not only:
  of the whole draw, 7,939 reflective-topped loops against 523 comforting,
  403 peaceful and 171 soothing, so a change there touches those a little
  too. The other half is
  ready to build: loops all cinder or all shatter are never in a bright
  mode today (0% of 2,452 and 1,591 glad ones), while blends led by them
  borrow brightness from their partners already (46% of glad loops 50-80%
  cinder, 34% shatter). Cinder has no progressions for ionian or
  mixolydian yet (the generic shapes would play), so the build adds
  I-IV-V-I and I-V (mixolydian's V spelled major), the Andalusian sets
  staying in their own modes.
  **Decided (Mikey, same day):** skip the reflective half; build cinder
  and shatter only. Built without new cinder progressions after all: some
  cinder-led blends are already in ionian or mixolydian (borrowed from
  their partners), and progressions written for those modes would change
  how their codes play. A bright cinder loop plays the general shapes, on a
  major tonic, as those blends always have.
- **Item 41: what the harness reproduced, and what it did not
  (2026-10-12, the hands).** The table came before any fix
  (`docs/perf-baseline.md`, "The Pixel 10a on v91"). Read against the
  brain's four points:
  1. **Late notes without a stall: reproduced.** `--audio-buffer-size`
     does not reach headless Chromium's output, but a numeric
     `latencyHint` does (`views.mjs --buffer 0.37`: 186 ms callbacks, 0.56
     s output latency). That alone made late notes only after a cut (a
     pasted code: 4). Holding the core's messages 400 ms as well
     (`--delay 400`: the worklet takes each message 400 ms of audio late,
     so notes reach a core already past the page's clock) made every
     mute, re-roll, tempo, wander change, new loop and pasted code put
     3-14 notes into the core late, with no stall at all, as the tester's
     `sounds` row shows. Fixed: none in any view or change after. **But the tester's phone says `baseLatency:
     0.0213`, a 21 ms callback, so the bursty-buffer reading is not
     confirmed for his phone;** 728 late notes in 50 s on Library means
     nearly every note, which needs the core more than a second past the
     page's clock. The new Diagnostics say which: `renderAheadMs`,
     `leastSpareMs`, `coreClockLeadMs` (the worklet's count running past
     the context's clock, as item 31b allows) and late notes split into
     stall / inside the buffer. Whatever the cause, the page now plans
     from what it measures, up to the 8 s queue.
  2. **The 1.3-1.8 s stalls: not reproduced.** At 6x, under software
     graphics and on a GPU-backed canvas (SwiftShader), the longest task
     was 241 ms (rendering), and every pixel read on the main thread
     together took 37 ms at most in a view; there was no GPU readback of
     any size, because `cover.js` already drew on `willReadFrequently`
     canvases. The covers are off the main thread now regardless (all of
     them, in a Worker; 0 readbacks on the page), which removes the
     biggest script in Library (opening an album ran 66-128 ms of script;
     now 31-50 ms). If the phone still stalls for a second, the cause is
     something this machine cannot show: the next Diagnostics, with
     `gpu:` and the long tasks per view, are the way to find it. No blur
     was changed: under software graphics no frame's rendering was long
     enough to blame it.
  3. **`dropped`:** not reproduced (0 everywhere); now split by cause in
     Diagnostics.
  4. **`sw: installing`:** the page set that word once, at
     registration, and never again: a finished install still read
     `installing`. It now follows the worker and lists each step with
     its time.

- **Item 45, stopped at its tone yardstick, then finished on Mikey's
  decisions (Session 21, hands, Sonnet 5.5, 2026-10-13/14, #213).** The
  first pass stopped with three open decisions (the 2-5 kHz tone share of
  analogpad, softpad and stab at 44.1 and 22.05 kHz; the noise's colour
  being per-sample; 192 kHz memory) and Mikey answered them (see item 45).
  What the numbers were, for the record. **Block independence:** all 21
  goldens in calls of 1, 64, 128, 441, 1,024 frames and of random sizes hash
  to the plain render's speaker output (x86-64 and wasm32). **Level** within
  0.10 dB of 48 kHz at every rate. **What folds back** under -67 dB at every
  rate, never more than 1.4 dB worse than 48 kHz's. **Tone**, voice by voice:
  within 0.5 points at 96 and 192 kHz for all 19 voices; at 44.1 and 22.05
  kHz analogpad -1.06 and -1.28 (of 16.79%), softpad -0.38 and -0.65 (of
  4.66%), stab +0.64 and +0.97 (of 33.58%), which Mikey accepted up to 1.5
  points. Whole lists: 18-shatter at 22.05 kHz is 1.13 points under its 48
  kHz share, reported not held. I grepped for hidden rate constants: none in
  the code except the tests' `44100` and the noise's poles. Memory: the web
  build 14,745,600 bytes; the `wide` build 17,563,648 (2.8 MB more).

- **Item 46, the building-block interface: notes for the brain (Session 23,
  hands, Opus 5.5, 2026-10-18, #238).** The interface is three things: `Block`
  (with `Voice` for blocks played by note), `Context` and one registry list.
  The voices, the bodies and five parts sit behind it, and every block gets the
  test kit. The goldens, the baseline and every voice's samples are unchanged;
  the numbers are in the PR. **What felt awkward behind the interface:**
  1. **Modulation.** A block takes one signal in and puts one out. FM drives
     a carrier's frequency with a signal, pitches move frame by frame
     (`Param`), and filters step under a moving cutoff. So the voices call
     their parts' finer methods, not `Block::render`. A recipe format needs a
     way to feed a signal into a setting, as Web Audio connects a node into
     an `AudioParam`. That is the biggest decision phase 5 faces.
  2. **The envelope is not a block.** It is a `Param` (an event list) driving
     a gain. In a recipe it is naturally data: events relative to the note.
  3. **Shared processors.** The bodies are per channel and fed by a bus that
     every note of a kind sums into (`Voice::BODY`). Their kinds are a fixed
     array of three in the core. A recipe instrument with its own resonance
     needs a way to declare a shared processor.
  4. **Nylon's damp** is the one place the core still names a voice
     (`Core::damp` reaches `Held::Nylon`, and `Core::note` reads a strum from
     nylon's first extra value). A general "event to the sounding notes of a
     channel" (damp, release, bend) would retire it.
  5. **Size.** The sung voices are too big for a slot, so the registry has a
     second kind of line (`pooled`). A recipe voice of arbitrary size needs a
     memory story: size classes, or a budget per recipe.
  6. **The note's twelve extra values are positional,** and each voice
     documents its own layout. Recipes want named settings. Voice numbers are
     the host's ABI and `js/core.js` mirrors them, so a new voice still
     touches the page once.
  7. **Several voice numbers per type** (tone, lead, wind, struck, sung)
     branch on `note.voice`. As recipes they would be presets of one
     instrument.

  **For the block catalogue (promise 5):**
  - **Primitives the kit would welcome:**
    - a modal resonator: a bank of decaying sines, which the tubular and
      temple bells hand-code today;
    - a waveshaper: the master's saturator;
    - a delay line as a block: the mix has one, but it is not a block;
    - an LFO: every vibrato is a sine `TableOsc`;
    - a formant bank: the sung tract;
    - a waveguide or Karplus-Strong string: nylon is a wavetable now;
    - a granular source: the noise grains with drawn rate and offset are
      half of one already.
  - **Today's blocks worth a look:**
    - the oscillator's Chromium start rules, which are kept to the sample and
      complex;
    - coefficient recomputing under a moving cutoff, which is the cost;
    - the noise, a fixed two-second buffer.
  - **Stereo:** `Signal` is mono. The stereo render (promise 9, Session 24's
    items) decides whether a block gets a second signal or a stereo variant.

  The kit's first table, for the ears:
  - The frame drum peaks at +1.0 dBFS on its own at velocity 0.8, before
    the mix and the master's ceiling, and the kick at -1.1 dBFS.
  - The texture drop's 2-5 kHz share is 96% (a ringing bandpass in that
    band).
  - The shaker's share is 24% and the snare's 41%.

  None of these is new; the kit only makes them visible.

- **Item 46b, modulation and stereo seams: notes for the brain (Session
  25-0, hands, Opus 5.5, 2026-10-19, #241).** Three seams on `Block`, each
  optional with a default: a modulation input (`MODS`, `render_mod`,
  `Mods`: a signal summed into a setting, Web Audio's `AudioParam` rule),
  the stereo shape (`Stereo`, `render_stereo`, `STEREO`, `Pair`), and
  channel events (`Core::event`, `Voice::event`: `DAMP`, `RELEASE`, `BEND`),
  plus a channel's own blocks (the registry's `channel` list, today the
  three bodies). The parts that take modulation: `osc::Oscillator` (`FREQ`,
  `DETUNE`; any table, an automated frequency), `filter::Filter` (`FREQ`,
  `Q`, `GAIN`) and the new `lfo::Lfo`. FM, every vibrato and every sweep sit
  on them, bit for bit; nylon's damp is a `DAMP` event and the core names no
  voice. **Which batch items should use which part:**
  - **(a) instruments.**
    - **dubStab:** the tone pattern. A triangle and a saw as
      `Oscillator<0>`s summed into a `Filter` whose cutoff envelope (2.4-3.4
      kHz closing to 700 Hz) drives `FREQ`; the lows trimmed by a second,
      undriven `Filter`. The echo throw stays the mix's.
    - **roundBass:** `bass.rs`'s pattern, an envelope into a lowpass `Filter`'s
      `FREQ`.
    - **glass:** if it is FM or bell-like, `fm.rs`'s pattern: an `Lfo` at audio
      rate into an `Oscillator`'s `FREQ`.
    - **The chapel organ:**
      - the flute ranks are `Oscillator<0>`s, and the celeste a steady detune;
      - the chiff is a `noise_source` grain through a bandpass `Filter`;
      - the tremolo is an `Lfo` into an `amp.rs`-style gain;
      - the 800 Hz dip is an undriven peaking `Filter`.

      Its swaying delay needs a delay-line block, which isn't there yet
      (catalogue). Its retuning at each chord is the awkward part (below).
    - **wash:** `texture.rs`'s waves, two lowpass `Filter`s under one sweep into
      `FREQ`, with the 17-26 s rise and fall an `Lfo<0>` or an envelope.
      The two key-tuned resonances are undriven peaking `Filter`s. Its
      level follower is a new block whose output drives a gain (`amp.rs`
      is the template).
    - **floorDrop:** a `RELEASE` event to the drums' and the bass's channels
      would let held notes go at the drop, once those voices listen to it.
    - **arc:** a slow filter arc on a channel is a channel's own block that
      runs its own `Lfo` into its own `Filter` (one file, one line in
      `channel`). Its per-loop timing is the awkward part (below).
    - **longChords:** generator only; no core seam.
  - **(b) stereo:**
    - **Shape:** `Stereo` and `render_stereo`.
    - **Mono switch:** sources play centered, so the mono switch over mono
      blocks is exact (the kit's guard proves it on every block).
    - **Dual mono:** `Pair` puts a processor on each side.
    - **The room and the ensemble:** the first `STEREO` blocks (`pan.rs` is
      the template).
    - **Still to build:** the core's stereo render itself (`Core::start(rate,
      2)` is still refused) and stereo versions of the mix and master, which
      are still a fixed graph outside the blocks.
  - **(c) mixing logic:**
    - **padSweep:** a channel's own block on the chords: a `Filter` driven by
      its own `Lfo<0>` (12-28 s).
    - **tunedKick:** the drum's existing pitch `Param`; no seam needed.
    - **echoTime:** a mix parameter.
    - **subDuck:** a sidechain: the kick's signal driving the bass channel's
      gain. There is no path yet for one channel's signal to drive another's
      block (below).
    - **floorDrop shapes:** as floorDrop, with `RELEASE`.
    - **lateFloor and pedalBass:** generator only.

  **Still awkward for phase 5:**
  1. **A channel's own blocks are started once, with settings fixed in the
     registry, and take no modulation and no events.** Per-loop settings (the
     padSweep's period, the arc's length, the organ's registration) need a
     host path into a channel block, such as `Core::param` reaching it. That
     is not built.
  2. **"Each note hears each kind once" fits a damp, not a retune.** It is
     the rule that keeps today's damp exact. A held voice that must hear
     many bends (the organ retuned at each chord, a pitch wheel) can't. The
     recipe format should say each kind's reach (once, or every held note
     every time). A waiting note keeps one event of each kind.
  3. **Wiring is code.** Voices connect an `Lfo` to a part's `MODS` index in
     Rust, and several signals into one setting are summed by the caller. A
     recipe needs to describe connections (source block, target, setting) as
     data. `MAX_MODS` is 4 a block.
  4. **No cross-channel signals** (subDuck's sidechain, a follower listening
     to other layers, as the wash's does).
  5. **Voices are mono.** A stereo voice (a per-note ensemble spread) would
     need stereo buses and a stereo dispatch in the registry. Only blocks
     have the seam.
  6. **The oscillator's Chromium start rules leak into modulation.**
     Connecting a signal, even a zero one, makes the pitch "moving" and
     changes how a note's first block starts. It is exact today and worth a
     line in any recipe guide.
  7. **The `Lfo` is a sine.** A triangle or a sample-and-hold is a `Shape`
     setting away, since it is built on `Oscillator`.
  8. **Memory:** the core is 140,904 bytes bigger. That is mostly the waiting
     queue's event bookkeeping, plus the sung pool's `Lfo`s. The `.wasm` is
     147,599 bytes (146,313 before), and the stereo seam adds no byte to it.

- **Item 46c, the channel seams: notes for the brain (Session 25a, hands,
  Opus 5.5, 2026-10-19).** Three host paths into a channel's own blocks,
  each through one queue of things waiting for their time (`PENDING`, 256),
  handed over before the block holding their time renders:
  - **Settings for the loop:** `Block::SETTINGS` (named, each with the
    value the kit probes it at) and `Block::set`; the host calls
    `Core::set(channel, own::NAME, setting, value, time)` (`dl_set`). A
    block applies a setting from its time; the swell restarts its `Lfo`
    there.
  - **Reach:** `event::reach` says once (`DAMP`, `RELEASE`: 46b's path,
    untouched) or every time (`BEND`, and the new `RETUNE`, which carries a
    chord of up to six notes: `Event::retune`, `dl_retune`). An every-time
    event reaches every note of its channel held when its time comes
    (started by then, not yet ended), and the channel's own blocks hear
    every event then too. A waiting note now keeps only the once-kinds,
    which shrank the waiting queue by 64 KB.
  - **The retune rule** is `blocks::retune` (shared): notes the chords share
    stay on their voice, new notes take the voices left in order, a voice
    with none rests. Item 61's organ calls it from its `event`, as the
    chorale demonstration does.
  - **Cross-channel signals:** `Core::route(channel, own::NAME, setting,
    Some(Tap { channel, block }), time)` (`dl_route`); `None` undoes it. A
    tap is a channel as it goes into the mix, or one of its own blocks'
    buses. **The order: one block's delay** (a setting hears its source's
    previous block, 2.7 ms at 48 kHz), so channels render in any order and
    routes may run both ways; the same on every target and block size. At
    most 8 routes; one signal a setting.
  - **`Channel::WHOLE`:** a channel's own block may work on the whole
    channel (after its buses and the host's notes) rather than on a bus.
    padSweep, subDuck and the arc's filter are this.

  **For the batch:** item 61's organ is a channel source like
  `chorale.rs`, hearing `RETUNE` and taking its registration through
  `SETTINGS`; 62's wash is a source on the sixth channel with five routes
  in (one a channel) summed by itself, or the follower reading one route
  from each; 63's arc and 72's padSweep are `WHOLE` blocks like
  `swell.rs` with a `Filter`; 68's subDuck is `duck.rs` with depth and
  recovery as settings (a setting the kit can only hear with a key routed
  in is better as a start setting or probed with one: the kit sets
  settings with nothing routed, so the demo duck lists none).

  **Still awkward for phase 5:**
  1. A retune applies from the start of the block it falls in, in the
     chorale; an organ that wants it on the frame has to keep the frame
     itself (the time is in the event).
  2. A `WHOLE` block runs on its channel only. A processor across several
     channels (a bus) would be a channel of its own, or the mix's.
  3. A route is one signal a setting, and the tap is a whole channel or a
     bus: a kick alone (subDuck keyed by the kick, not the hats) needs the
     kick on a bus of its own, a `THROUGH` pass-through block.
  4. Every channel holds one of each channel block (5 a name; 6 after the
     wash). An idle one must cost nothing; each says so in its `render`.

- **Item 55, the frame drum's peak: the cause, and the other drums (Session
  25a, 2026-10-19).** The frame's +1.0 dBFS was luck, not its shape: a drum's
  moving pitch is read from its first block's start (Chromium's rule, kept to
  the sample in `osc`), so for up to a block before the strike the skin
  played at its resting pitch, Web Audio's default 440 Hz. How much of that
  blip a hit got depended on where it fell in its 128-frame block, and with it
  how the skin's first crest met the second mode's: the same frame hit alone
  peaked anywhere from -1.1 to +1.2 dBFS and its loudness wandered by half a
  dB, by its start time (16 start times at 44.1 and 48 kHz). A lag or a
  polarity on the mode lowered the probe's peak but moved each hit's loudness
  by up to 0.6 dB either way, and noan-nou (four frame hits in 20 s) failed the
  layer yardstick with every one tried. So the frame's pitch now rests at its
  own first frequency (no blip), and its skin rings 0.40 s instead of 0.32 to
  keep its loudness: every start time peaks -1.0 to -1.1 dBFS. **The kick,
  soft kick, snare, clap, rim and tap carry the same blip** (their pitch rests
  at 440 Hz too), and their strikes vary hit to hit the same way: at the kit's
  probe the kick and the soft kick peak at -1.1 and -1.2 dBFS, under the new
  ceiling, but across 16 start times at 44.1 and 48 kHz each reaches -0.8
  dBFS (the snare, clap and tap stay under -9). Item 55 leaves them, as it
  asks. Worth a look with the ears (a click by chance, or part of the kick
  Mikey likes?) before a later item removes it; tunedKick (item 67) rebuilds
  the kick's pitch anyway.

- **Item 61, the organ: where it stands (Session 25a, 2026-10-19).** Built
  in Session 25a-3 from the organ's files on the branch below, with the four
  fixes kept and the host side added (the item 61 PR); the branch
  `claude/item-61-organ-wip` is superseded and can be deleted. Not
  merged; the session stopped after item 55. The work is on the branch
  `claude/item-61-organ-wip`, stacked on unfinished wip commits for items
  56-60 (its item 55 commit is an early version that #246 replaced; take
  only the organ's files from it). Done there, core only: `organ.rs`, a
  channel block in the chords' channel (four voices of pipe and celeste
  from one wavetable the drawbars build, retuned by `blocks::retune` with
  a chiff on a new note, a soft shaper, low, carve and tone filters, a slow
  shimmer, a send into the room), 21 settings, and sends for channel
  blocks (`Channel::SENDS`). Not done: the host side (the engine's hold
  logic from the sketch; the synth driving it with `dl_set`, `dl_retune`
  and `dl_event`), the kit store (`KIT=update`), goldens, baseline, the
  cost and the yardsticks. Four fixes the kit's first run found, all in on
  the branch:
  1. **The probe's real level.** The probe first played at level 1.0,
     about 20 dB over the organ in a loop; it now plays at 0.098, the
     sketch's level in a loop of three or more notes a second.
  2. **The opening chord after the kit's settings.** The probe's chord
     started in `start`, before the kit set the setting under test, so
     `speak` and `chiff` changed nothing a probe could hear. The chord is
     now applied on the first rendered block, after the settings.
  3. **Released during the probe.** The probe held its chord to the end,
     so `release` was never heard; it now lets go at 1.2 s.
  4. **A NaN probe opts a setting out.** `verb` only reaches the room, which
     a mono probe does not hear, so it could never pass "heard". A setting
     whose `probe` is NaN is skipped by the kit and `dlrender` (documented
     on `Setting::probe`). Still open: `chiff` at the probe renders the same
     as without it, so check the chiff before storing the kit.

- **Item 56, roundBass: stopped at its yardsticks, which disagree (Session
  25a-2, hands, Opus 5.5, 2026-10-19).** Decided by Mikey (2026-10-20): (a),
  today's level, as built; #248 merged. The level goes to his ears in
  Session 26's album: the same loops with the bass at today's level, +3 dB
  and +5.5 dB, labeled (`LIFT_DB` in `core/src/blocks/bass.rs`, one number
  for every bass voice). Built and checked, but not merged:
  the item's two figures can't both be met with the sound Mikey heard, so it
  needs a decision. The work is the open item 56 PR (branch
  `claude/affectionate-knuth-x5pe4b`), finished apart from that: the kit
  stored, goldens (note lists re-exported for the new register), the
  baseline updated, tests three times. Items 57-60 are not started (wip
  commits still on `claude/item-61-organ-wip`).
  - **The figures,** on the first 12 corpus loops led by dust (seed
    20261009, 60 s each, full, the core's taps): today, the core build, and
    the sketch (the prototype's recipe, register and harmonics, pasted into
    today's JavaScript synth and rendered on the same loops):

    | | today | core build | sketch | item 56 asks |
    |---|---|---|---|---|
    | 40-60 Hz share of the mix | 27.6% | 5.5% | 3.9% | about 6% |
    | 120-250 Hz share (mean; median) | 10.1%; 9.8% | 23.4%; 18.6% | 31.1%; 28.1% | about 27% |
    | bass layer by ear (K-weighted, mean; median) | | +0.24; +0.40 dB | +5.76; +6.43 dB | within 1 dB of today |
    | the whole mix by ear | | -0.04 dB | +2.04 dB | |
    | refused notes | 393 | 393 | 393 | none added |

  - **Why:** the core's bass has the sketch's timbre exactly (inside the
    bass layer, 120-250 Hz is 52.6% of the core's energy and 52.8% of the
    sketch's; 20-60 Hz 0.2% in both). What differs is level: the prototype
    added its harmonics on top of the note, about 6 dB by ear, and its
    "about 27%" was measured with that louder bass. The build trims each
    voice (3.2 to 6.7 dB, and 1 dB for the register) to meet "within 1 dB
    of today", and at today's level the harmonics take a smaller share of
    the mix. The batch rule ("within 1.5 LU of the sketch's figure") sides
    with the sketch; item 56 sides with today.
  - **The choice** (a decision for the brain or Mikey's ears):
    (a) today's level, as built: the bass reads on a phone through its
    harmonics, at the loudness it has now (120-250 Hz about 23%);
    (b) the sketch's level, what Mikey heard: drop `round_trim` in
    `core/src/blocks/bass.rs` (bass about +5.5 dB by ear, the mix about
    +2 dB before the master, 120-250 Hz about 31%), and re-measure the
    other layers against it;
    (c) between, say +3 dB.
    Raising the harmonics against the note instead would meet both
    figures but is a brighter bass than the one Mikey approved, so I did
    not build it.
  - **Also in the PR:** "Let the loop wander" kept folding the bass's
    passing tones into the old 28-52 (the prototype's too); they now stay
    in 36-57, for Mikey's ears. The 40 baseline loops: the bass layer
    +0.31 dB on average (median +0.30, -2.38 to +5.88, noan-nou the most), the
    mix +0.04
    dB, refusals and the balance lock unchanged. **Cost:** in the core a
    bass note costs about 0.067% of real time more (the harmonics block,
    about 1.5 core sine notes; a bass note 40-70% more in the kit). `VOICE_COST`
    stays as it is: it prices the fallback's render time, whose bass
    gets only the register, and raising it would add refusals the item
    rules out. Worth a look when the budget is next revisited.

- **Item 59, glass: stopped at its level, for a decision (Session 25a-2,
  hands, Opus 5.5, 2026-10-20).** Decided by Mikey (2026-10-20): "both" (the
  chord's velocity, scaled by chords a bar as the sketch did), and glass only
  on chords of two or more distinct notes; item 59 amended to say so. Built
  on a local branch, not pushed: the
  bell (the FM voice at ratio 3.5 on 46b's modulation input, a kit probe
  `fm/glass`: 0.0% at 2-5 kHz, the flute 0.3%), the choice in item 54's list,
  and two fixes the wip lacked: glass yields (it asks whether there is room,
  its cost and 20 units more, and holds nothing, as item 60's stab does,
  through a check that reads the budget without pruning it), so with it on
  the twelve loops below refuse exactly the notes they refused without it
  (before the fix, keys and FM chords lost up to 4 notes a loop); and its
  notes carry keys of their own. **The level is the open question.** The item
  asks it "set against the chord voice it comes from" instead of the
  prototype's chords-a-bar scaling, and asks that on vapor (under one chord a
  bar) it add no more than about 1.3 LU. Measured on the core, 60 s, glass
  forced on against off, on the first six thaw and six vapor corpus loops:

  | level rule | thaw 1-2 kHz (mean) | vapor #4, #30, #70, #205 (under a chord a bar): the mix | all 12 loops: mix change, mean |
  |---|---|---|---|
  | the chord's velocity (0.62 at the median, 0.50) | 1.4% → 5.5% | +0.9, **+5.3**, +0.6, +0.7 dB | thaw +0.8, vapor +2.8 |
  | the sketch's (0.62 x chords a bar, at most 1) | 1.4% → 4.7% | +0.2, +0.8, +0.4, +0.0 dB | thaw +0.7, vapor +2.0 |
  | both (velocity x chords a bar) | 1.4% → 4.7% | +0.2, +0.9, +0.3, +0.0 dB | thaw +0.7, vapor +1.8 |

  The bells sit 3.6 to 15 dB over the chord layer's loudest 3 s by ear in
  every loop, thaw included, so a rule from the chord itself (its velocity,
  notes or length) can't tell vapor #30 from a thaw loop: what differs is
  that vapor #30's whole loop is sparse (-38.5 LUFS; 0.38 chords a bar).
  vapor #91 and #118 (keys playing one-note "chords", up to 9 a bar) gain
  +6.9 to +7.4 and +2.5 to +3.0 dB under each rule, since a one-note chord
  rings out as the same note two or three times. So: (a) "both", the conservative one,
  closest to what Mikey heard (it keeps the sketch's scaling the item calls a
  rough edge); (b) the chord's velocity alone, as the item reads, and accept
  vapor #30's +5 dB; or (c) a rule on the loop's density, as dubStab's
  level is (notes a second). And whether one-note chords should ring glass
  at all. Items 59 and 60 wait on it (60's stab shares the yield).

- **Item 63b, the arc's two shapes: what was built and decided (Session
  25a-4, hands, Opus 5.5, 2026-10-24).** Built as the item reads; three
  calls asked of Mikey on the way and answered the same day (2026-10-24),
  each the option recommended:
  1. **The dip's time within twice its floor** can't sit in 15-40 s at every
     corner of the ranges with the dip at 35% of each swing: on loops with
     no vibes it runs 15.5-44.6 s (mean 29.1), on calm vibes (3-5 minute
     swings) 22.9-44.7 s, on bright ones (1.5-2.5 minutes) 11.4-22.3 s. The
     40+ s cases all have a floor of 1.3-1.5 kHz (twice the floor 2.6-3 kHz,
     hardly muffled); the 11 s ones are bright loops' quick dips. **Kept at
     35%** (the other options: shrink the dip's share on long swings, or
     narrow the periods). Below 2 kHz stays at or under 14.8% of every
     swing (the 800 Hz floor's figure; 9.2% at 1.5 kHz), and it never goes
     under the floor.
  2. **Where a dip loop starts:** anywhere in the open part (uniform over
     its 65%), so the first dip comes from a moment after the start to
     about two minutes in, rather than only the first half of it.
  3. **The vibe leans:** toward x2, away x0.4 (mid-range). With them, 0.7
     strength: foggy loops draw deep 46% of the time (plain 33%), sunny ones
     24%; the arc itself is on in 32.5% of plain loops.
  - **How it is built.** `ARC` in `js/choices.js` holds every variable
    (weight, vibes, shapes and their leans, period ranges by vibe list,
    starts, floor, the dip's share); `arcOf(seed, vibes, pins)` draws on the
    arc's own stream (item 63's salt): the period, the start, then the shape
    and the floor, so a plain loop that draws deep keeps item 63's period
    and start to the bit; a pin fixes one value and moves no other draw. The
    core takes `ARC_SHAPE`, `ARC_FLOOR` and `ARC_DIP` beside `ARC_FROM`,
    `ARC_PERIOD` starting it. The dip is a raised cosine in log frequency
    centered on the swing's darkest point (`u = 0`, where deep is darkest);
    the echo's lift follows the cutoff as the deep swing's does (the same
    cutoff, the same lift: 0.14 at 9 kHz, 0.075 at 800 Hz).
  - **Checked.** 6-minute renders of 37 loops (corpus 20261009's first 36,
    six with wander on, and Mikey's), main against this build, speakers'
    output hashed: the 21 without the arc and the 2 that draw deep are bit
    for bit the same; the 14 dips differ. Goldens: 08-bloom and 17-bloom
    draw deep (hashes unchanged), 09-shrine draws the dip (its hash moved);
    `blocks.txt` gains `arc/dip`, `arc/breath` unchanged. One thing caught
    on the way: a nonzero default in the core's static state put the whole
    core (14 MB) into the `.wasm`'s data; the static values are zeros now
    (`arc::ZERO`) and the file grew 1,128 bytes. Core baseline: the 6 of
    its 40 loops that draw the dip moved (speakers' output only, 0.00 to
    -0.12 LU), its 4 deep ones didn't.
  - **Mikey's loop (seed 102356715)** draws the **dip**: a 3:02 swing, open
    for its first 1:33, darkest at 2:05.5 (1.15 kHz), open again from 2:37.
    Under item 63 it was deep, darkest at 1:41.7.

- **Item 64, the stereo render: what was built (Session 25b, hands, Opus
  5.5, 2026-10-24).** `Core::start(rate, 2)` (or `set_layout(2)`, the
  worklet's `layout: 2`) renders stereo; mono is untouched and pays almost
  nothing.
  - **How:** the master holds two lanes (`master::OUT` is now 2) and a mono
    render runs the first alone; stereo runs the wobble, saturator, tone,
    arc and highpass on each side, then a second pair of compressors keyed
    by the mid (`Compressor::process_keyed`: one gain for both sides from
    `(L + R) / 2`, so they press as mono's; Chromium links by the louder
    side), then the width ceiling (`blocks::width`) after the limiter. The mix
    is still mono (the space recipes, item 65, add the side), so today both
    lanes get the mono mix.
  - **Exact:** with nothing placed or widened, each side is the mono render
    bit for bit: the core's unit test (full, lite, 44.1 kHz, both
    compressors pressing, the arc on), the worklet test, all 21 goldens at
    every block size and the 12 at other rates (`left` = `right` = mono
    `out`, bus too), and the baseline's 40 loops full and lite through the
    real app (`stereo` part: `monoOnEachSide` true for all 80). Mono itself:
    goldens, rates, the stored block renders and the baseline's mono parts
    bit for bit unchanged.
  - **The width ceiling, where it must differ from the sketch:** the sketch
    measured over a 2 s window centered on each moment (`width_ceiling.py`
    read a second ahead in the finished file); the block's window is the last
    2 s. It moves its gain once a block (the sketch every 10 ms), gliding
    across it. A frame with no side passes untouched (so a `-0` stays `-0`).
    Its five settings are named (`width::SETTINGS`; `Core::param`,
    `WIDTH_TARGET` to `WIDTH_UP`): target -7 dB, cutoff 200 Hz (fourth-order
    Butterworth, as the sketch's), window 2 s, down 0.5 s, up 3 s.
  - **The kit:** a stereo processor is also fed two different sides
    (`Play::Wide`, rows `NAME/PROBE:wide`), since a side-only block can't
    differ from itself with the same signal on both. `blocks.txt` gains
    five rows (`width/ceiling`, `:stereo`, `:wide`, and the test-only pan's
    `:wide` two); no existing row moved.
  - **Goldens:** `TARGET.stereo.hashes` beside each target's mono file, and
    `rates.stereo.hashes`; `tools/golden.sh --stereo` (every mode) and CI's
    stereo steps on every target (PowerPC plain only, as for mono).
  - **Costs** (instruction counts, cachegrind, the native host): mono
    against main +0.31% (09-shrine), +0.40% (busy-cinder), +0.61%
    (15-wayfare), +0.63% (07-dust, lite), +0.73% (14-bloom, lite): the second
    lane's bookkeeping, inside the 1% asked. Stereo against mono 1.05 to
    1.25 times (lite 1.05-1.07), under the 1.5 target. `.wasm` 225,169 bytes
    (183,812 before): the second compressors' code, the keyed loop and the
    width block. Core memory about 140 KB more (the second lane's wobble
    line, the stereo compressors' lookahead, the ceiling's window).
  - **In the app:** still mono by default (item 66 adds the switch);
    `?stereo` in the address plays the stereo render, and Diagnostics shows
    the layout.

- **Item 65, the space recipes: what was built, what differs, what Mikey
  decided (Session 25b, hands, Opus 5.5, 2026-10-25).** The approved
  stereo baseline (round 4's blend: room 0.64, ensemble 0.5, steady, under
  the width ceiling) as data in the core (`core/src/space.rs`): 32 named
  settings, each a range a loop draws from its seed (`space::SETTINGS`),
  scaled by how busy and wet the loop is, which the host tells the core at
  load (`SPACE_SEED` to `SPACE_START`; `js/engine.js`, `spaceOf`); any of
  them narrowed or pinned by a host (`Core::space_range`, `dl_space_range`,
  `CoreHost.spaceRange`). The mix makes the side (`mix::Side`), riding the
  mid's own gains, duck and tails; the master gets mid + side and mid -
  side. The mid is the mono mix bit for bit; mono renders are unchanged.
  - **Where the queue's text and the sketch's code differ, the sketch's
    code was built:** the melody is spread too (the sketch's `ens3`, 0-0.12
    of width times 1.6; never placed); the light kit's shakers and jingles
    sit at 0.7 of the hats' place; "matched to the target width" is the
    blend's weights (0.64 and 0.5, exact by linearity); the spread's sway is
    the sketch's (12 ms, +-3 ms, 0.35/0.42/0.49 Hz for chords, melody,
    texture). Where the text is silent and the sketch has nothing: glass is
    placed as a light texture is (0.1-0.25 of width, to the other side from
    the light kit), alone when the loop's texture isn't light, the whole
    texture layer when it is (the core knows glass as the FM voice on the
    texture's channel, which only glass plays there outside the light
    textures).
  - **Where the core must differ from the sketch:** the sketch blended
    finished renders (after the master); the core sums the side before it,
    which differs only by the master's nonlinearity. Its allpasses run on
    exact delays (Chromium's loops add a block inside each), the same
    magnitude, a different phase scramble. The width ceiling's window is the
    last 2 s, not centered (item 64). Before the master, (L + R) / 2 is the
    mid but for rounding (`(m + s) + (m - s)`); after it the fold sits about
    80 dB under the mono render.
  - **Decided by Mikey (2026-10-25):** the wash's two noises go out as side
    above 200 Hz and scaled by the loop's width, as every other width. The
    sketch's full, unfiltered width failed two guards on vapor #30 (side
    2.8 dB under mid below 120 Hz, limit 20; correlation 5th percentile
    0.12, limit 0.3); above 200 Hz alone still failed (-19.0 dB, 0.23).
    With his choice: -32.7 dB and 0.94.
  - **Where each new part sits:** glass placed (above); dubStab on the
    chords' channel, widened with the chords, its echo throw centered (no
    echo width); the organ rides the chords' spread and reaches the room
    through its own send; the wash its own width, as decided; the arc darkens
    both sides alike and moves nothing in the image; roundBass and the frame
    drum centered (the bass and the heavy drums are never placed); longChords
    and floorDrop change nothing in the image.
  - **The guards** (`stereo_guard.py`; the report's four loops, 60 s, and
    the baseline's 40, 45 s; the organ in 18 of the 44, the wash in 17, the
    arc 12, dubStab 9, glass 7, a light texture 24): every guard passes on
    all 44. Fold within 0.00 LU, worst octave 0.01 dB; side 22.2 dB or more
    under mid below 120 Hz; correlation median 0.82 or more, 5th percentile
    0.62 or more (median 0.94); balance within 0.32 dB; refused notes as in
    mono in every loop. Side against mid above 200 Hz: median -18.2 dB over
    the 44 (the baseline's 40 -17.1, the report's 4 -21.1); bounce 1.31,
    travel 0.41 dB (round 4: 1.2, 0.5).
  - **The report's figures don't reproduce on today's loops, and the
    sketch's own code shows why.** The report's blend read -18.0 dB on its
    four loops; the core reads -21.1 there. The sketch's code pasted into
    today's JavaScript synth (`space-sketch.patch` and round 3's allpass
    room) and run with its own recipes (`space_jobs.py`, `draw2`) on the
    same four loops reads -20.8 (no wash or organ in the fallback). With the
    same recipe numbers pinned into the core: -22.8 without the wash. Loop
    by loop the space is the same where the music is (glade -23.9 against
    -23.6, vapor -23.2 against -24.8; undertow's room within 0.5 dB in every
    octave from 500 Hz to 4 kHz); where it isn't, the mid is: the core's
    mid above 200 Hz is 6.0 dB louder than the fallback's on undertow (9.4
    dB at 200-400 Hz, roundBass's harmonics) and 2.0 dB on cinder, and the
    same side reads that much narrower. The loops themselves have moved
    since the report (2026-10-16): roundBass and the rest thicken the mid.
    Built as the sketch's numbers; the baseline's 40 land on the target.
  - **Costs** (instruction counts, the native host, with the space): stereo
    1.18 to 1.38 times mono (lite 1.18 and 1.24), under 1.5; mono +0.13% to
    +0.27% against item 64. `.wasm` 238,357 bytes (225,186 before).

- **Item 66, the Mono switch (Session 25b, hands, Opus 5.5, 2026-10-25).**
  The app plays the stereo render by default; **Mono** (Advanced, under a
  new "Sound" heading, a plain switch like Lighter on processor's, for a
  Painter session to restyle) plays the mono render, exactly today's. Kept
  in the prefs (`mono`), never in a share code or a backup. The `?stereo`
  stand-in is gone.
  - **Switching while a loop plays:** nothing is rebuilt. The app's core
    node always has two output channels (the mono render in both; the
    measure harness's keeps one a lane, as before), and the switch is a
    message (`CoreHost.setLayout`, `dl_layout`). The core fades
    (`Core::set_layout`, `LAYOUT_FADE` 0.25 s): into stereo the master takes
    two lanes at once (the second takes the first's state, the stereo
    compressors the mono ones', `Master::switch_layout`) and the side starts
    from silence and fades in; into mono the side fades out, the master keeps
    two lanes until they agree (its wobble line and the compressors'
    lookahead still hold side for about 30 ms; within -100 dB, or at the
    latest 0.5 s), then one. The core's test flips a held chord to stereo and
    back: the side's second difference over each switch is 0.7 and 0.85
    times the stereo render's own over the same frames (switched at once,
    without the fade, 2.3 and 2.7 times; the test's bound is 1.5).
  - **Exact:** a render that starts mono and stays mono is the mono render
    bit for bit, and one that starts stereo the stereo one: every golden,
    mono and stereo, at every block size and rate, and the baseline's mono
    and stereo parts unchanged. Until a switch, the mono render bit for bit
    (the core's test and the worklet's).
  - **Diagnostics:** `layout: stereo  mono switch: off`, and a `render:`
    line, what the core's render costs this device in each layout, as a
    share of real time and over how long it played in each (the worklet's
    clock around `dl_process`). Headless Chromium on the sandbox: mono 4.6%,
    stereo 4.8-5.9%. **For Mikey's phone:** play a minute each way and read
    the line (promise 9: stereo must not double it).
  - **Costs** (instruction counts, cachegrind, the native host): mono against
    item 65 within 0.01% on the five loops; the whole group's mono against
    main before item 64 +0.34% (09-shrine), +0.46% (busy-cinder), +0.75%
    (15-wayfare), +0.80% (07-dust, lite), +0.99% (14-bloom, lite): inside the
    1% asked, with nothing to spare on lite. Stereo 1.18 to 1.38 times mono.
  - **The balance lock,** re-baselined for the group (`stats.mjs --check`,
    all 22 figures within tolerance; then `--n 10000 --write-baseline`):
    nothing moved, the file rewritten byte for byte. The generator wasn't
    touched in items 64-66. The core baseline: nothing moved (mono and
    stereo parts).

- **Item 67, tunedKick: what was built, where it differs from the sketch,
  what Mikey decided (Session 25c, hands, Opus 5.5, 2026-10-25).** A core
  voice of its own, `blocks::kick` (`TunedKick`, voice 29): today's kick or
  soft kick (`parts`), the same sweep and attack, landing on a target in Hz,
  falling at a drawn rate, and for a long boom today's punch (213 dB a
  second) down to a tail that rings on at the drawn rate (`extra`: target,
  fall, lift, tail). A new event kind, **`CHOKE`** (46c had none; every
  time, number 4): the host sends one to the drums' channel with every kick
  of a loop that plays the tuned kick, and a tuned kick still ringing falls
  away from the curve it is on (`cancelAndHoldAtTime`, then 5 ms to -80 dB,
  the sketch's) and plays nothing 60 ms later; the hit it was sent with
  passes it by. The host side: `KICK` in `js/choices.js` (every variable,
  named, with its range), `kickOf` (the note, both falls on the sketch's
  stream, and the sketch's level fit at load, about 30 ms), `kickKindOf`
  (a wandering pass's swap), the choice `tunedKick` in item 54's list
  (`today`, `short`, `long`; long leaned by batch 5's table through a new
  `optionVibes`), the engine handing each kick its kind, `Synth._coreKick`.
  The fallback plays today's kick. On 14.1% of 10,000 loops (short 6.9%,
  long 7.2%).
  - **Decided by Mikey (2026-10-25):**
    1. **The budget:** the tuned kick is held as the kick it replaces (0.45
       s the kick, 0.36 the soft kick), not the sketch's 0.36 for every
       kick. With 0.36, the 80 corpus loops (60 s, full) refused 3308 →
       3312 notes, more on three (+1 musicbox, +3 bell, +6 accordion: the
       freed room let a texture part in, which cost a chord note later) and
       fewer on one (-6). With today's holds: 3308 → 3308, every loop the
       same.
    2. **The low bands:** "within 3 dB of today's kick" is read as the
       sketch's cap (no band more than +3 dB over today's kick: at most
       +1.2 dB), since a kick on G1-D2 moves energy out of 20-60 Hz by
       design (the report: "short moves energy up out of 20 to 60 Hz"). The
       short kick reads 3.7 to 6.3 dB under today's there.
  - **Where it differs from the sketch (the sketch's code was built
    otherwise):** the kind (short or long) is item 54's draw, not the
    sketch's half and half on its own stream, and both falls are drawn on
    the sketch's stream (short's, then long's), so a loop's rate is not the
    one the sketch drew it; eligibility reads the loop's kicks, not the
    listener's mutes (a mute never changes a draw); the core chokes every
    kick of a tuned loop's, today's too when a wandering pass swaps to it
    (today's kick doesn't hear it); the choked kick stops 60 ms after (the
    sketch's ran on at -80 dB to its stop). **For Mikey's ears:** the
    kick's pitch rests at where its sweep starts, not at 440 Hz, so it never
    carries the chance blip item 55 found in today's kick and frame drum
    (the sketch, in Chromium, had it); the sketch's tuned kick has no click
    (today's kick has a 30 ms lowpassed grain), built as the sketch.
  - **The swap** (Mikey's rule, not in the sketch): a wandering loop that
    draws the tuned kick changes it at the top of a pass, one pass in four
    (`KICK.swap`, a taste call, for Mikey's ears), to today's or the other
    tuned one, half and half, drawn from the pass alone; the first kick of
    the pass chokes the boom still ringing (a 5 ms crossfade). A loop that
    doesn't draw it never swaps.
  - **Measured** (the report's four loops, corpus 20261009, 90 s, wander
    on, core, swaps off; `kick_check.py kick` on the drums' tap):

    | loop | kind | note | on the note | from 12 s | fall (drawn) | loop | under 60 Hz | 60-120 Hz |
    |---|---|---|---|---|---|---|---|---|
    | grove #62 | today | F1 | 82% | 98% | 200 | | | |
    | | short | A#1 | 87% | 100% | 207 (214) | -0.13 LU | -3.7 dB | -0.2 dB |
    | | long | A#1 | 89% | 100% | 92 (74, tail -12.2 dB) | +0.21 | +0.7 | +0.0 |
    | undertow #45 | today | F1 | 84% | 97% | 200 | | | |
    | | short | A#1 | 91% | 100% | 222 (230) | -0.28 | -4.2 | -0.4 |
    | | long | A#1 | 93% | 100% | 95 (77, -11.9) | +0.23 | +0.7 | +0.0 |
    | dust #69 | today | F1 | 76% | 94% | 200 | | | |
    | | short | C#2 | 86% | 100% | 171 (181) | +0.05 | -6.3 | -0.3 |
    | | long | C#2 | 88% | 100% | 57 (47, -9.9) | +0.78 | -1.9 | +1.2 |
    | halcyon #48 | today | F1 | 80% | 96% | 200 | | | |
    | | short | A#1 | 84% | 98% | 222 (236) | -0.32 | -4.3 | -0.4 |
    | | long | A#1 | 91% | 100% | 87 (67, -13.2) | +0.12 | +0.6 | -0.0 |

    The report read 89-94% on the note (today's F1 83-87%), falls 207-222
    short and 82-87 long, loudness -0.24 to +0.08 LU short, +0.21 to +0.27
    long and +0.95 on dust's C#2 boom: the core matches but for the note
    share over the whole render, which `kick_check.py` reads high in the
    first 10-12 s of every loop, today's kick too (48 Hz against its 44 in
    halcyon's first 10 s; the kick itself sits on 58.3 Hz from its first
    hit by zero crossings), so halcyon's short reads 84.2% whole and 97.6%
    from 12 s. Every loop within ±0.5 LU of today, dust's boom on C#2
    (above B1) +0.78 within +1. Refused notes: the same in every one of the
    80 corpus loops (above).
  - **Stereo** (`stereo_guard.py`, 60 s, the report's stereo four: grove
    #62, undertow #45, glade #21, dust #69; today, short and long, mono
    against stereo): fold 0.00 LU, worst octave 0.00 dB; side under mid
    below 120 Hz -38.8 to -46.9 dB; correlation median 0.99-1.00, 5th
    percentile 0.89 or more; balance within 0.15 dB. The kick is centered
    by construction (`place_of` places only hats, shakers, jingles and
    glass).
  - **The kit:** `kick/short`, `kick/long`, `kick/soft-long` and their
    `@choke` rows; peaks -1.53 to -1.64 dBFS at the probe (today's kick
    -1.08), 0.0% at 2-5 kHz; about 0.03-0.05% of real time a hit against
    today's kick's 0.09-0.13% (no click). `VOICE_COST` stays the kick's.
  - **Goldens:** 08-bloom and 17-bloom draw the tuned kick (10 kicks and 10
    chokes each); 01-glade, 16-grove, 19-glade and 20-tide move with the
    reshuffle a new choice brings (organ and wash come and go). **For item
    75:** 16-grove, the rates list's `steady` organ case, no longer draws
    the organ (it draws the wash); `tools/rates.mjs` still passes. 20-tide
    keeps its organ.

- **Item 68, subDuck: what was built, where it differs from the sketch
  (Session 25c, hands, Opus 5.5, 2026-10-25).** A channel's own block on the
  whole bass channel, `blocks::sub_duck` (`SubDuck`, `own::SUB_DUCK`), keyed
  by the drums' channel routed in (`Core::route`), as 46c's sidechain
  demonstration is. Its settings, set for the loop (`Core::set`): the depth
  in dB at a kick of velocity 0.8 or more (0 is off, and costs nothing) and
  the recovery in seconds. The host: `SUBDUCK` in `js/choices.js` (every
  variable named, with its range), `subDuckOf` (the sketch's stream and its
  draws, the "whether" draw spent, so a loop gets the depth and recovery the
  sketch drew it), the choice `subDuck` (only with a kick and a bass line;
  batch 5's vibe leans), and `Synth.subDuckSetup` (the settings and the
  route at the loop's first step, the wash's way; undone for a loop
  without). The fallback leaves it out. On 11.9% of 10,000 loops.
  - **Where it must differ from the sketch:** the sketch dipped the bass
    channel's gain at each kick's scheduled time, by the kick's velocity
    (`e8Duck`). Keyed through a route, the block hears the kick instead: the
    key's low band (under 150 Hz, where the drums' channel is the kick)
    rising 8 dB from where it rested and past -20 dBFS is a kick, and how
    loud it rises in its first 12 ms is its velocity (a key peak of 0.68 is a
    kick at 0.8, measured on today's kick and the tuned one; full depth from
    there, as the sketch's `min(1, vel / 0.8)`). Then the sketch's two lines:
    down in 8 ms, back over the recovery. It starts a block late (the
    route's order: 2.7 ms at 48 kHz, 2.9 at 44.1); a new kick's line starts
    from where the gain is (the sketch's jumped back to full first); a hit
    under -20 dBFS (a ghost hit, a kick's click a few ms ahead) doesn't count:
    at -30 dBFS one took two kicks' places in undertow #45, at its own small
    depth. The loop's first kick, if it falls in the route's first block,
    isn't heard (a route hears silence its first block, 46c's rule); nor is a
    second kick inside 40 ms of the first (halcyon's flams).
  - **Measured** (the report's loops, corpus 20261009, 90 s, wander on,
    core; the bass tap's 30-150 Hz band in 5 ms windows against the same
    loop without it, the deepest window 10-50 ms after each kick):

    | loop | drawn | kicks at 0.8+: dip, median (10-90%) | all kicks | back by 0.25 s, worst | bass layer |
    |---|---|---|---|---|---|
    | grove #62 | 5.04 dB, 0.200 s | -4.85 (-5.00 to -4.71) | -4.76 | -0.12 dB | -0.44 dB |
    | undertow #45 | 5.56, 0.230 | -5.38 (-5.86 to -5.27) | -4.94 | -0.22 | -1.04 |
    | halcyon #48 | 7.92, 0.163 | -7.52 (-7.87 to -7.33) | -7.34 | -0.33 | -0.65 |

    The gain itself reaches the drawn depth (the core test reads the deepest
    5 ms at 6.4-6.9 dB for 7, today's kick and the tuned one alike); the
    windows round the line's corner. Never deeper than 9 dB: the depth is
    drawn 5-9 and only scaled down. The report read the sketch 4.6-5.4 dB
    down 25 ms after, the bass layer 0.4-0.7 dB quieter. The other four
    layers' taps are the same to the bit; loudness -0.03 to -0.09 LU;
    refused notes the same (103, 198, 104). cinder #12 (no kick in the
    report) now has kicks, but no bass under them: nothing happens.
  - **Stereo** (`stereo_guard.py`, 60 s, grove #62, undertow #45, halcyon
    #48, glade #21): fold 0.00 LU; side under mid below 120 Hz -39.4 to
    -51.7 dB; correlation median 1.00, 5th percentile 0.89 or more; balance
    within 0.03 dB. The bass is never placed.
  - **Costs:** the kit's `sub_duck/kick~key` about 0.09% of real time while
    it ducks; unused, renders without it cost +0.08% to +0.21% in
    instructions (09-shrine, 15-wayfare, 07-dust against item 67): its early
    return on five channels, and the second pass a whole-channel block
    brings (`own::ANY_WHOLE`, now true outside the tests; bit for bit as
    before).
  - **Goldens:** 08-bloom and 17-bloom draw subDuck (and lose item 67's
    tuned kick to the reshuffle); 01-glade, 02-wayfare, 16-grove, 19-glade,
    20-tide and busy-cinder move with the reshuffle.

- **Item 69, lateFloor: what was built, what Mikey decided (Session 25c,
  hands, Opus 5.5, 2026-10-25).** The engine and the generator only. After a
  load the kit and the bass sit out for a wait counted from the loop's first
  step, then arrive together, the kit first and the bass two bars later, or
  the bass first (`js/engine.js`; the skipped notes still take their place
  among their layer's notes, so no draw moves). `LATEFLOOR` in
  `js/choices.js` holds every variable with its range; `lateFloorOf` draws on
  the sketch's stream and spends its "whether" draw, so a loop waits as long
  and in the order the sketch drew it (8-30 s rounded to two bars, at least
  two; 40/35/25). A reroll keeps the wait it is in; a loop resumed under a
  rebuilt engine (the quality switch, `load(spec, { resume: true })` in
  `js/main.js`) doesn't wait again; Pause and Play go on where they were. On
  21.7% of 10,000 loops.
  - **Decided by Mikey (2026-10-25):**
    1. **Eligibility, as the queue reads:** something else (chords, a tune
       or a texture) plays within the loop's first second, entry schedules
       included (`LATEFLOOR.first`). The sketch's test was anything else at
       all; with it, 81 of the 2,252 loops in 10,000 that drew lateFloor
       (3.6%) had nothing else sounding in their first second, 35 nothing in
       their first 4 s. Now none.
    2. **The budget, as the sketch:** the waiting kit and bass hold no
       budget, so chord and tune notes today's budget turns away get to play
       in the intro, and a texture note or two may be refused later in their
       place. The layers yardstick is read as refused notes equal or fewer in
       total, and the other layers within 0.5 dB but for that ripple,
       reported per loop.
  - **Measured** (the report's four loops, corpus 20261009, 90 s, wander on,
    core, lateFloor forced on against off):

    | loop | order, wait | kit, bass first sound | other layers | loop | refused |
    |---|---|---|---|---|---|
    | glade #21 | kit first, 12 bars | 25.09 s, 24.31 s (waits 18.83, 21.96: the loop plays neither until then, today too) | chords +0.16 dB; melody, texture 0.00 | -0.87 LU | 28 → 26 |
    | undertow #45 | kit first, 16 bars | 29.82 s, 33.54 s (waits 29.82, 33.54) | chords +0.07; melody 0.00; **texture -0.78** (ripple: 0:00-0:10 -0.9 dB, 0:30-0:40 -3.3) | -1.31 | 198 → 184 |
    | halcyon #48 | together, 6 bars | 12.46 s, 14.53 s (waits 12.46: the bass's note across the line is skipped, its next starts at 14.53) | chords +0.01; texture -0.20 | -0.53 | 104 → 104 |
    | dust #69 | bass first, 6 bars | 25.32 s, 19.00 s (waits 25.31, 19.00) | chords +0.02; melody, texture 0.00 | -1.02 | 60 → 57 |

    After its wait, each one's kit and bass are today's to the bit. The
    report read entries after 12.4-26 s as drawn, the loop 0.6-1.0 LU
    quieter over 90 s, refused notes 28 → 26, 272 → 260, 144 → 144, 60 → 57
    (halcyon's texture changed the same way there).
  - **Stereo** (`stereo_guard.py`, the same four, 60 s): fold 0.00 LU; side
    under mid below 120 Hz -36.9 to -51.7 dB; correlation median 0.98-0.99,
    5th percentile 0.77 or more (the intro is a little wider without the
    centered kit and bass); balance within 0.22 dB.
  - **Goldens:** nine lists move with the reshuffle a new choice brings, and
    those that draw lateFloor wait.

- **Item 70, floorDrop shapes: what was built, what Mikey decided (Session
  25c, hands, Opus 5.5, 2026-10-25).** The generator only. A loop that draws
  the choice `floorDrop` (`shapes`; loops of 4 bars or more with drums or a
  bass) gives each of its floor drops (item 58's, with "Let the loop wander"
  on) a shape: single (item 58's), stagger, stutter or long, 40/25/20/15,
  stagger and stutter only with a bass line (Mikey's rule), stutter and long
  leaned by batch 5's table. The shape is drawn at the very end of drift's
  draws (`shapeDrop` in `js/generator.js`, after "a single layer takes a
  breath"), laid out on the bars item 58's drop drew, so the drops fall on the
  same passes and start on the same bars, and nothing else the wander draws
  moves (tested on 400 loops, 5 passes each). `FLOORDROP` in `js/choices.js`
  holds every variable.
  - **The style follows the bar's length** (Mikey's rule), as the sketch's
    code did: a stagger's bass comes back two bars after the kick in a bar
    shorter than 2.5 s, one otherwise; a stutter's drops last two bars in a
    bar shorter than 2.2 s, one otherwise, one every two of their lengths,
    three or four of them (fewer where the loop has no room; under two, the
    drop is single); a long drop is about 12 s, 4-8 bars, or the whole pass
    in a loop of 8 bars or fewer, cut at the loop's end.
  - **Decided by Mikey (2026-10-25): in bars, as the sketch.** The seconds in
    the yardstick are a guide. Over the corpus (80 loops, 20 passes each,
    full wander, shapes forced on): 491 drops on 1,600 passes, the same 491
    as without the shapes; single 42.2%, stagger 25.5%, stutter 16.1%, long
    16.3% (the report: 48/24/14/14).

    | shape | the yardstick asks | the corpus, as built |
    |---|---|---|
    | stagger | the bass back 1-2 bars after the kick | 1-2 bars: 2.1-5.3 s (median 3.7) |
    | stutter | 3-4 drops of 2.5-4.5 s, one every 5-7 s | 2-4 drops (median 2), each 2.2-5.3 s (median 3.4; 64 of 79 in 2.5-4.5), one every 4.4-10.7 s (median 6.9; 33 of 79 in 5-7) |
    | long | 10-20 s | 4.3-42.7 s (median 11.6; 40 of 80 in 10-20): the whole pass of a slow short loop is the long end, a long drop cut at a loop's end the short |

    Never a full stop (the rest plays on, no bars marked silent).

- **Item 71, pedalBass: what was built, what Mikey decided (Session 25c,
  hands, Opus 5.5, 2026-10-25/26).** The generator only. A loop that draws the
  choice `pedalBass` (over a held, pulse, dub or walking bass) plays the key's
  tonic in a rhythm (held, half bars, beats, or off the beat between the
  kicks: 25/30/25/20) under whatever the chords do, stepping now and then to
  one neighbor its mode has (a whole tone up, the tone below, the flat second,
  the fifth), in the loop's bass voice, in the octave of the bass's register
  (36-57) nearest the loop's own bass's middle note, no note longer than its
  usual one (`pedalBass` in `js/generator.js`). `PEDAL` in `js/choices.js`
  holds every variable; `pedalOf` draws on the sketch's stream (the bass's
  seed and its salt) and spends its "whether" draw, so a loop gets the rhythm,
  neighbor and share the sketch drew it. The bass is drawn as it always was,
  then gives way to the pedal after the choices are drawn (they read the
  tracks), and the pedal plays in the bars the loop's own bass plays in,
  after its entry schedule, its repairs and the gaps. On 12.2% of 10,000
  loops. The fallback plays it too (it is notes).
  - **Drift spends the loop's own bass's draws** (`meta.pedal.n0`, the
    loop's own bass's notes): no other layer's variation moves, tested on
    600 passes. Its own passing tones are drawn last of all, after item 70's
    drop shape (the sketch drew them before the shape, which moved the shape
    in a loop with both; Mikey: "good catch").
  - **Where it differs from the sketch's code:** a metre with no grid of its
    own (5/4) borrows 4/4's, as the chord rhythms do (the sketch stretched
    4/4's grid by 20/12, past the bar's end); the register is roundBass's
    36-57 (the sketch's was 28-52).
  - **Decided by Mikey (2026-10-25/26), each as the sketch:**
    1. **The tonic's share:** the downbeat always on the tonic, the neighbor
       only between, never twice in a row. Over 400 pedal loops the tonic
       takes held 100%, half bars 91.8%, beats 88.7%, offbeat 86.5% of the
       notes (147 loops in the asked 75-90%, 245 over 90%; the records the
       report drew on sit at 87-98%). The 75-90% yardstick is reported, not
       met.
    2. **The budget:** the pedal keeps its notes as drawn. Over the 64 corpus
       loops it can take over (60 s, full, forced on) refused notes fall in
       total, 2658 → 2602 (held pedals -46, half bars -31, beats +8,
       offbeat +13), but 18 loops refuse more, 1 to 9 notes each, chord and
       bell notes (FM keys and bells, a pad, a texture), never the tune:
       loops 5 (+3), 7 (+3), 9 (+5), 14 (+9), 17 (+5), 19 (+6), 24 (+3), 29
       (+6), 42 (+1), 48 (+2), 52 (+1), 53 (+9), 62 (+1), 69 (+5), 73 (+6), 75
       (+3), 78 (+2), 79 (+2).
    3. **The low-end guard, one-sided from here on:** it protects against
       weight, so a lighter low end passes. On the report's four loops (90 s,
       wander on, core): dust #69 (dub → offbeat, whole tone up) 20-60 Hz
       7.1 → 7.6%, 60-120 Hz 53.1 → 54.8%; undertow #8 (held → half bars)
       6.8 → 7.7%, **61.0 → 64.5%** (+3.5: the steady tonic sits in the band;
       kept, as the sketch); haven #38 (pulse → held, the fifth) 0.3 → 0.5%,
       35.2 → 36.1%; clockwork #65 (held → held) 0.1 → 0.0%, 14.0 → 7.1% (in
       B the register holds only B2, 123 Hz, where its own bass sat half on
       G#2, 104 Hz: lighter, passes). The bass layer 0.6-4.2 dB quieter, the
       loop 0.2-1.4 LU (the report: 0.6-2.1 dB); the other layers within
       0.04 dB; refused notes 60 → 66 (dust), 69 → 69, 0 → 0, 34 → 34 (the
       report: 60 → 55, 231 → 231, 0 → 0, 34 → 34).
  - **Stereo** (`stereo_guard.py`, the same four, 60 s): fold 0.00 LU; side
    under mid below 120 Hz -29.2 to -43.9 dB; correlation median 0.99-1.00,
    5th percentile 0.92 or more; balance within 0.15 dB. The bass is never
    placed.
  - **The dials' scale** (`test/dial-scale.json`) rebuilt on purpose, as item
    57 did: the pedal moves the corpus's ground dial most (its largest
    quantile 3.1% of its range, the median 0.508 → 0.503), the others 0.1-2.1%.
- **Item 72, padSweep: what was built, where it differs from the sketch
  (Session 25c, hands, Opus 5.5, 2026-10-25).** A channel's own block on the
  whole chords channel, `blocks::pad_sweep` (`PadSweep`, `own::PAD_SWEEP`):
  its own `Lfo` into its own `Filter`'s frequency, as 46c's swell carries an
  `Lfo` into a gain. Its settings, set for the loop (`Core::set`): where it
  closes (Hz), how far it opens (octaves; 0 is off, and costs nothing), the
  period (s) and the Q (dB, as a `BiquadFilterNode`'s). The cutoff is lo ×
  2^(oct × (1 − cos) / 2), as the sketch's detuned lowpass: closed at its
  start, open half a period on. The host: `PADSWEEP` in `js/choices.js`
  (every variable named, with its range), `padSweepOf` (the sketch's stream
  and draws, its "whether" draw spent), the choice `padSweep` (sustained
  chord voices only, the sketch's list; batch 5's vibe leans), and
  `Synth.padSweepSetup` (the settings at the loop's first step; off for a
  loop without). The fallback leaves it out. On 5.3% of 10,000 loops.
  - **Where it differs from the sketch:** across a load the sketch's filter
    jumped to the new loop's settings (and offline, a second load's applied
    from the render's start, so it was never checked). Here the cutoff glides
    from where it was onto the new loop's curve over 0.5 s; a loop with no
    sweep glides it open (20 kHz) and then the block stands still, out of
    the path; a sweep coming in from still crossfades with the dry channel
    over 20 ms (a filter starting under a playing pad clicks). Measured
    across a load (grove #62 then undertow #45 at 45 s): sweep to sweep,
    the cutoff from grove's 980 Hz to undertow's closed 576 Hz within the
    half second, then opening on its curve; sweep to none, open within the
    half second; none to sweep, closed at 582 Hz from the load; no sample
    step near the load larger than the same render without it.
  - **Measured** (the report's loops, corpus 20261009, 90 s, wander on,
    core): the cutoff the chord layer was given, fitted each second as a
    lowpass's response (the drawn Q) to the chords' tap with the sweep over
    the same tap without it:

    | loop | drawn: lo, octaves, period | cutoff, 3rd-97th percentile | swing | fitted period | off the drawn curve, median |
    |---|---|---|---|---|---|
    | grove #62 | 762 Hz, 2.63, 13.89 s | 778-4800 Hz | 2.63 oct | 13.9 s | 0.05 oct |
    | undertow #45 | 576, 3.06, 25.33 | 583-4941 | 3.08 | 25.3 | 0.03 |
    | shrine #0 | 662, 2.91, 24.66 | 673-5085 | 2.92 | 24.7 | 0.03 |
    | thaw #71 | 658, 2.60, 14.10 | 657-3921 | 2.58 | 14.1 | 0.04 |

    The yardstick is met as the filter's swing. Read as the chords' 90%
    rolloff (`techniques.py`), the brightness hardly moves: these sustained
    voices keep over 90% of their energy under the closed cutoff (their
    rolloff sits at 108-1376 Hz with or without the sweep), so what the
    sweep opens and shuts is the upper harmonics, -1.9 to -4.5 dB at their
    darkest and +1.3 to +1.6 dB open (the Q) in the band from lo to 8 kHz.
    The sketch can't be read the same way: its stems are taken before its
    filter. The chords' tap is 0.05-0.37 dB louder with it (the Q's lift),
    the loop -0.004 to +0.036 LU (the report: 0.00-0.01); the other four
    layers' taps the same to 0.001 dB; refused notes the same (103, 198, 1,
    0).
  - **Stereo** (`stereo_guard.py`, the same four, 60 s): fold 0.00 LU; side
    under mid below 120 Hz -40.3 to -71.6 dB, as without it; correlation
    median 1.00, 5th percentile 0.97 or more; balance within 0.02 dB of
    without it. Thaw's side over mid +0.9 dB (its pad is most of the loop).
  - **Costs:** the kit's `pad_sweep/sweep` 0.12% of real time while it
    sweeps (a filter with its frequency driven costs the same); still, its
    early return.
- **Item 73, echoTime: what was built, what the brain decided (Session 25c,
  hands, Opus 5.5, 2026-10-26).** The core already took the echo's time per
  loop: `Synth.setEchoTime` sends `MIX.echo` (gliding, 50 ms), and the
  JavaScript echo's `delayTime` without the core. Nothing was missing there,
  so this is host only. `ECHO` in `js/choices.js` (the times in beats with
  their weights, and batch 5's vibe table), `echoTimeOf` (the sketch's one
  draw on its stream, 0xec40), and the engine at each load:
  `setEchoTime(60 / bpm × the loop's beats)`, kept as `Engine.echoBeats`, so
  the tempo slider moves the loop's own time rather than today's dotted
  eighth.
  - **Decided by the brain (2026-10-26): outside the list, as the sketch.**
    The queue's shares (today 45%, one beat 25%, half 15%, two thirds 15%)
    can't come out of item 54's list: there, as a choice of weight 1, it
    reached 15.3% of loops (one beat 7.3%, half 4.1%, two thirds 4.0%), and
    only a weight of 8 reached the queue's shares, taking 20-40% of every
    other choice's reach (4,000 loops simulated). So echoTime is drawn for
    every loop, not counted among a loop's changes, as roundBass, longChords,
    the single floorDrop and the stereo baseline aren't (the batch rules
    above say so now). It keeps its vibe leans: one beat ×1.5 toward
    spacious and floating, half a beat ×1.5 toward nimble and skipping. No
    other choice's reach moves. A loop with no vibes draws the time the
    sketch drew it.
  - **Shares** (4,000 loops): within two points of 45/25/15/15 (the test).
    The corpus's 80: 31 on the dotted eighth, 20 on one beat, 16 on two
    thirds, 13 on half.
  - **Measured** (the yardstick: the time as drawn, within 4 ms): each loop
    loaded through the real engine and synth on the core, then one rim hit
    alone; the echo's repeat found by cross-correlating the mix with the
    dry hit. Thirteen loops, 45-168 bpm, every time: all within 1.3 ms of
    drawn (178.6-1071.4 ms drawn; every one read 0.27-1.29 ms early). shrine #0 0.75 beat, 692.3 → 691.5 ms; haven #38 one beat,
    779.2 → 778.6; grove #62 two thirds, 449.4 → 449.0; thaw #10 half,
    447.8 → 447.5; vapor #4 one beat at 56 bpm, 1071.4 → 1070.4.
  - **Stereo** (`stereo_guard.py`, haven #38, undertow #45, grove #62, thaw
    #10, 60 s, each drawn time against the same loop at the dotted eighth):
    fold 0.00 LU; side under mid below 120 Hz the same (-30.3 to -66.6 dB);
    correlation median 1.00, 5th percentile 0.97 or more; balance within
    0.01 dB; side over mid within 0.4 dB; refused notes the same.
  - **The balance lock re-baselined** (the group's, as the item asks):
    `stats.mjs --check` read every one of its 22 figures at +0.0000 against
    the lock item 63 wrote, and `--n 10000 --write-baseline` wrote it again
    byte for byte. Nothing in items 67-73 reaches what it locks (the melody,
    its motifs, the choir, keys against melody): their changes are the kick,
    the bass, the floor, the chords' filter and the echo, all drawn after the
    tune.
  - **What moves:** 14 of the 21 golden lists, each by its echo time alone
    (one `param` line); 20 of the core baseline's 40 loops, by at most 0.4
    LU, their layers, voices, refusals and costs unchanged (side over mid
    within 0.5 dB, the fold's residual -68 dB or under).
