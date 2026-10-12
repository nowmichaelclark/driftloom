# The core: charter

**Status: draft for Mikey's approval** (brain, Session 18, 2026-10-13).
Nothing here is decided until he says so; his answers to the four
questions at the end go in, then the brain writes the first phase into
`docs/QUEUE.md`.

The core is Driftloom's engine: the part users never see but developers
do. It decides the notes and makes the sound, and it is the same code in
the website, the Android app, a desktop build, or someone's game. It is
the number one priority (Mikey, 2026-10-12): shoot for the moon, take the
time it needs.

## Where it starts

`core/` today (`dlcore`, about 11,000 lines of Rust, no dependencies) is
the synth: every voice, the mix and the master chain, built to WebAssembly
and now the app's default engine. It is good work with a
reference's habits: it copies Chromium to the sample, runs in 128-frame
blocks because Web Audio does, and is fed by JavaScript.

What is still in JavaScript, on the page's main thread or near it:

- **The generator** (`js/generator.js`, ~3,000 lines, plus vibes, moods
  and characters) decides every note.
- **Dozens of articulation draws** from `Math.random` (strum spread,
  jitter, slides, the noise buffer, bell detunes, reed delays) are made in
  `js/synth.js` and `js/engine.js` and handed to the core with each note.
  So today the same code does **not** make the same audio twice.
- **The scheduler** hands notes to the core 2-8 s ahead (items 39-41).
  It works, but it is a patch: the page still decides, and a long enough
  stall would still starve the music.

## What the core promises

Each promise has a measure, so "done" is a number or a listening verdict,
never a feeling. Numbers marked *proposed* are the brain's; phase 1
baselines them and Mikey can move them.

### 1. It never glitches, whatever the screen does

The page only sends commands (play, next, vibes, mute) and draws what the
core reports. Notes are decided and played on the audio side, so screen
work cannot reach them.

- *Measure:* `tools/perf.mjs` with a stall injected on the main thread
  (5 s of busy work while playing, visible and hidden): **zero late or
  dropped notes, zero underruns.** On the testers' phones: Diagnostics
  shows zero late notes across a Library and Save session.
- *Measure:* the core's worst block, not its average: no block's work
  above **25% of its deadline** on the slowest tester phone (*proposed*).
  Generating a new loop is spread across blocks so it never spikes one.

### 2. Light: battery and speed

Everything allocated once, at start; nothing allocated, locked or freed
while playing (true today for the synth; extended to the generator).

- *Measure:* audio-thread time per second of the heaviest loops, against
  today's core and the old JavaScript synth (`docs/perf-baseline.md`:
  54-95 ms/s): target **under half of today's core** (*proposed*).
- *Measure:* the whole engine's memory, fixed and reported in bytes;
  the `.wasm` size (159 KB today) kept under a stated budget.

### 3. Deterministic: a code is a recipe

Same core version + same code = **the same notes and the same samples,
bit for bit.** All randomness comes from seeded generators inside the
core; `Math.random` leaves the sound path.

- *Measure:* golden tests: a fixed set of codes render to stored hashes
  of their notes and their audio. Any change to a hash is deliberate and
  says so in its PR (like the balance lock).
- *Across devices* -- see question 1. The recommendation is that the
  hashes match on every target (wasm, x86-64, ARM phones, a 32-bit
  big-endian build standing in for a GameCube-style port), which needs
  the core's own math functions instead of each platform's.

### 4. Embeddable anywhere

So good that developers could put it in any game engine, mod, operating
system or device, and iOS and macOS stay easy.

- `no_std`, no heap after start, no dependencies, no threads, no
  floating-point surprises: it runs on a microcontroller or a console as
  happily as in a browser.
- Any sample rate (22,050 to 192,000 Hz) and any block size; the 128-frame
  quantum becomes the web host's business, not the core's.
- A small, documented host API -- start, command, render, read events --
  in Rust and as a plain C header, so Godot, Unity, Unreal, Bevy, a
  Swift app or a homebrew port can each wrap it in an afternoon.
- *Measure:* CI builds and runs the golden tests on wasm32, x86-64,
  aarch64, a 32-bit ARM target and a big-endian target; a tiny C program
  renders a loop to a WAV through the header alone.

### 5. Easy to change

Mikey will keep adding vibes, sounds and profiles, so the core must make
change cheap, not freeze the app.

- **Instruments are building blocks plus recipes** (decided 2026-10-12):
  hand-written Rust blocks (oscillators, filters, envelopes, noise,
  formants, the reed, the bell) and small data recipes that wire and set
  them, like a synth's presets.
- Vibes, profiles, moods and voices are data too, in a plain-ASCII text
  form people can read and edit, compiled to a compact binary the core
  and the codes carry.
- *Measure:* a new instrument or vibe ships as a recipe file with **no
  Rust change**, and CI checks every recipe (it parses, it stays within
  its level, cost and tone limits).

### 6. Beautifully written

The part developers see. Clear names, a reason in every doc comment,
small files, one way of doing each thing.

- Documentation on every public item (`missing_docs` denied); Clippy's
  pedantic lints clean; `unsafe` only at the C boundary; no panic
  reachable from rendering (true today; kept).
- A short guide (`core/README.md`): what the core is, how to host it,
  how to write a recipe, how to add a building block.
- *Measure:* a fresh Claude session, given only the guide, adds a new
  instrument recipe and a new building block correctly on the first try.

### 7. It still sounds like Driftloom

Freedom from Chromium is freedom to be better, not different by
accident.

- The balance lock holds (`stats.mjs --check`, rebuilt for the Rust
  generator); the refusal and level checks hold; mono only.
- Changes to sounds Mikey already likes go to **blind A/Bs**; new sounds
  to labeled albums. His ears are the acceptance test.
- Improvements beyond Chromium (a warm reverb designed for this music,
  better oversampling, a true-peak ceiling, smooth parameter changes)
  are each their own item, judged by measurement and by ear.

### 8. Short, plain codes

- A loop's code is the core version, its seed and only what the listener
  changed (vibes, mutes, edits): about **10-16 characters**. An album
  drawn from one album seed is a single seed; a hand-picked album, a list
  of seeds (about 60 characters for eight loops, against over a thousand
  now).
- **Plain ASCII only**, the 64 link-safe characters (`A-Z a-z 0-9 - _`),
  shared as a link so Discord shows a tidy preview. No server: codes
  carry everything. The same rule for everything the core emits (names,
  errors, logs).
- A code says what it needs (core version, recipe packs by hash) and,
  when something is missing, says so plainly instead of playing the
  wrong thing. A modder's instrument can travel inside the code as a
  recipe of a few hundred bytes.
- Anything may break while in testing (2026-10-12): old codes are not
  protected.

## The phases

One phase at a time, each with a fresh hands session or several, each
proven before the next starts. Order chosen so every phase stands on the
one before it.

1. **Foundations.** Seeded randomness inside the core (the
   `Math.random` draws move in); the core's own math (`no_std`,
   bit-identical across targets, if question 1 says yes); any sample rate
   and block size; golden-hash tests; CI on all targets; a native
   command-line renderer that writes a WAV (the seed of roadmap 17's
   export). Chromium parity retires as a goal; the JavaScript synth stays
   only as a fallback. *Done when:* the golden hashes match on every
   target, and Mikey hears no change on an album (a control album: it
   should sound the same).
2. **The generator in Rust.** `generator.js`, vibes, moods and characters
   ported, with profiles and vibes as data. *Done when:* the balance lock
   and `stats.mjs` corpus numbers match the JavaScript generator's within
   stated tolerances, and a blind album (old vs new generator) passes his
   ears.
3. **Off the page.** The whole core runs on the audio side; the page
   sends commands and draws events. The scheduler, the lookahead and the
   JavaScript synth and generator are deleted. *Done when:* promise 1's
   stall test is clean and the testers' phones report zero late notes.
4. **Instruments as recipes.** The building blocks and the recipe format;
   today's voices re-expressed as recipes where it fits (each proven to
   null against its Rust original, or A/B'd if it changes); then the
   ears-approved sounds (roundBass, glass, floorDrop, arc) as the first
   new instruments built only from recipes.
5. **Codes, version 2.** Short codes, album seeds, recipe packs by hash,
   plain links. Old codes stop working; that's fine.
6. **The host kit.** The C header, the WAV export in the app (roadmap
   17), a minimal example host for one game engine (the brain suggests
   Godot: free, open, popular with indie and chill-game makers), the
   guide. *Done when:* promise 4's C program and promise 6's fresh-session
   test pass.
7. **Better than Chromium.** The sound improvements, one item each, blind
   A/Bs where they touch what he likes.

Then the platforms, each its own phase: Android on F-Droid (alpha/beta),
itch.io desktop, later Google Play. iOS and macOS stay a recompile away.

Promises 2 and 6 aren't a phase: every PR in every phase is held to them.

## Questions for Mikey

1. **Bit-identical everywhere?** Recommended: **yes** -- the same code
   makes the same samples on a phone, a PC and a hypothetical GameCube.
   It costs the core its own sine, exp and tanh (a few hundred lines,
   tested), and it is also what makes `no_std` possible. The alternative:
   the same notes everywhere, and sound that matches only within one
   build (tiny differences, nobody could hear them).
2. **Retire the JavaScript synth at the end of phase 3?** Recommended:
   **yes**. Every browser from about 2021 on runs the core; keeping a
   second engine in step doubles every change.
3. **Promise 1's and 2's numbers** (worst block under 25% of its
   deadline; under half of today's core's cost): fine as targets, or
   aim higher?
4. **Anything missing?** Something the core should promise that isn't
   here, or a phase in the wrong order.
