# The core: charter

**Status: approved by Mikey, 2026-10-13** (drafted by the brain,
Session 18). Phase 1 (items 42-46, done) is in `docs/archive/queue-items.md`. Changes to the
charter are his to make; the brain proposes them, with the reason.

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
  today's core and the old JavaScript synth (`docs/archive/perf-baseline.md`:
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
- **Across devices too** (Mikey, 2026-10-13): the hashes match on every
  target (wasm, x86-64, ARM phones, a 32-bit big-endian build standing
  in for a GameCube-style port). So the core carries its own math
  functions (sine, exp, tanh and the rest) instead of each platform's,
  which is also what lets it run with no standard library.

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
- **The building blocks themselves are open to change** (Mikey,
  2026-10-13): a developer, an AI agent or Mikey can add a block or make
  a fundamental change to one without understanding the whole core.
  Each block is one file behind one small shared interface (what it
  takes in, what it puts out, its settings), registered in one list, and
  comes with its own test kit: level, cost, tone, determinism, and a
  null test against its previous version so a change is either proven
  identical or flagged as a sound change for an A/B. Nothing else in the
  core knows a block's insides.
- **A catalogue of block ideas** before phase 5: what blocks could exist
  (granular, modal resonators for bells and wood, waveguides for
  strings and winds, wavefolding, a tape model, a spring or plate reverb,
  a vocal tract...), which of today's blocks are worth changing, and what
  each would open up musically -- written for Mikey to pick from, with
  short clips where a sound can be shown. He doesn't need to know the
  answer now; the design keeps every answer cheap.
- Vibes, profiles, moods and voices are data too, in a plain-ASCII text
  form people can read and edit, compiled to a compact binary the core
  and the codes carry.
- *Measure:* a new instrument or vibe ships as a recipe file with **no
  Rust change**, and CI checks every recipe (it parses, it stays within
  its level, cost and tone limits).
- *Measure:* a new building block is **one new file plus one line** in
  the list; its test kit runs without being written by hand.

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
  generator); the refusal and level checks hold; the mono render keeps
  the approved balance (promise 9).
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

### 9. Stereo across the board, with a mono switch

Mikey, 2026-10-15 (replaces "stereo possible, mono the default" of
2026-10-13 and "mono only" of 2026-10-11): **Driftloom is stereo**,
everywhere, with an optional switch that makes everything mono for
listeners and developers who prefer it. Stereo has endlessly many
variables, and each should be easy for developers and users to tweak.

- **Two renders, both first-class.** The core renders stereo by default
  and mono when asked. Mono is not a stereo mix folded down; it is its
  own render, with the balance Mikey approved, so the mono switch gives
  exactly today's sound. Both are deterministic and golden-tested.
- **Space is data, like instruments.** Where a sound sits and how it
  moves is a "space recipe", separate from the instrument: width of the
  whole mix, per-instrument pan, whether a part moves this track and not
  the next, how far, how fast and in what shape it moves, stereo echoes
  and reverb, and whatever else gets thought up. Recipes give ranges; the
  core draws within them from the loop's seed (it has the seed since
  item 44), so the variation is part of the music and the code
  reproduces it.
- **Every variable is a named setting** a developer or a user can turn,
  layered: the core's defaults, a profile's or vibe's lean, the request's
  picks (promise 11), the host's (a game) and the listener's.
- **In the app:** a **Mono** switch joins the settings (Mikey's
  exception to "theme and lighter on processor are the only settings").
  Whether finer space controls are ever shown is his call later.
- **Guards:** the stereo render, folded to mono, must sound like the
  mono render (measured: level and tone within stated limits), so
  stereo never hurts a phone speaker, a Bluetooth speaker or a mono
  venue; nothing essential lives on one side only; low notes (bass,
  kick) stay centered; and stereo must not double the cost on a phone.
- **Judged by ear first:** ears sessions propose stereo treatments as
  offline clips (stereo against today's mono) before anything is built;
  Mikey's verdicts decide which space recipes exist.

### 10. One real game proves it: Broken Friends

Mikey, 2026-10-24: Broken Friends, his own game, is the one and only
game that proves the core. "I don't want to make music mods for those
other games anymore anyways. Maybe another time." It replaces the three
games picked on 2026-10-13 (Luanti, OpenRCT2, Stardew Valley).

Broken Friends (`github.com/nowmichaelclark/brokenfriends`) is a 2D,
mobile-first creature-care game in Godot 4: stitched creatures you
repair, feed and play with, then send on quests. Android first, the
browser too, maybe desktop later. Each creature gets its own generated
theme; the music follows time of day and what is happening at camp.
It suits the core: calm, state-driven music, no combat.

- **The way in:** a GDExtension written in Rust (godot-rust), linking
  the core directly: `rust/driftloom-godot/` in the game's repo. The C
  header (promise 4) stays the way in for every other host.
- **The hard case is the browser.** Godot's standard web build cannot
  load extensions; its `web_dlink` build can, compiled with Emscripten
  to match the engine. The game's first spike tests this (2026-10-24).
- *Measure:* in Broken Friends, on an Android phone and in a phone's
  browser, the music follows the game (creature, time, events) with no
  glitches and no measurable frame-time cost; the golden lists render
  inside the game with exactly the golden hashes; the integration stays
  small and readable; and Mikey judges it "seamless". What was hard
  goes back into the core and the guide, so the next developer never
  meets it.

### 11. Ask for the music you want

Mikey, 2026-10-13: listeners choose more than vibes for the next loop --
how many bars, the tempo, the rhythm (meter and feel), the scale, and so
on -- alongside the vibes; and developers ask the same way: "a peaceful,
zany loop at 77 bpm, 16 bars, when the sun rises."

- **One request, for everyone.** The generator takes a request: each
  musical choice (bars, tempo, meter, rhythm feel, scale and key,
  profile, instruments, layers, vibes...) is either **pinned** to a
  value, given a **range**, or **left to chance**. The app's pickers, a
  developer's code and a game's events all build the same request; what
  is left to chance is drawn from the seed as today.
- **Requests bend the music, never break it.** A pin that pulls against
  a profile's character finds its in-between (as vibes do: "no pair is
  forbidden"), and a request that truly can't be met says so in plain
  words instead of playing something else.
- **Events:** a host can attach a request to a moment ("when the sun
  rises", "entering the cave", "turn 12"), and the core changes over at
  the next musical boundary rather than mid-phrase.
- **Any length, up to endless** (Mikey, 2026-10-13: longer than today's
  32 bars, even something like 3,072 -- about two and a half hours at
  80 bpm). So the generator works a section at a time, never the whole
  piece up front: memory and cost are the same for 4 bars or for
  endless, and any point of a long piece can be reached straight from
  the seed and the section number, without generating what comes
  before. The length in a code is a variable-width number, not a byte,
  so it costs only what it uses: up to 32 bars, one character; up to
  1,024, two; up to 32,768, three; and only when the length was pinned.
  **No cap in the core.** The app's picker (decided, Mikey, 2026-10-13;
  3,072 came from Beethoven's Ninth):
  up to 32,768 bars (three characters, about 27 hours at 80 bpm), plus
  **Endless**; nothing longer would differ from Endless for a listener.
  Both journeys are wanted: an arc like a symphony's, and long, slow
  wandering.
  Making a very long piece *musically* good (sections, returns, an arc
  over an hour, not one idea repeated or aimless wandering) is new
  composing work, a later item of its own with Mikey's ears; the core
  is built so it needs no rework when it comes.
- **Codes carry only what was pinned** beside the seed, so a pinned
  loop's code stays short (promise 8).
- **The balance lock** keeps guarding what plays when nothing is pinned;
  what plays when the listener pins is theirs.
- In the app these are **picks for the next loop, like the vibes**, not
  settings: the rule that theme and "lighter on processor" are the only
  settings stands. How the pickers look is a Painter session's job.
- *Measure:* every musical choice the generator makes can be pinned or
  ranged through the request (a test walks the list); a pinned value is
  honored in the render (measured: tempo, bar count, meter, scale); and
  the same request and seed give the same loop on every target.

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
   should sound the same). Also: the output layout becomes a host's
   choice (one channel today), and today's blocks move behind the shared
   block interface (promise 5), so phase 5 starts from a clean place.
2. **The generator in Rust.** `generator.js`, vibes, moods and characters
   ported, with profiles and vibes as data, built around the request
   (promise 11) from the start. *Done when:* every choice can be pinned
   (promise 11's measure), and the balance lock
   and `stats.mjs` corpus numbers match the JavaScript generator's within
   stated tolerances, and a blind album (old vs new generator) passes his
   ears. **Ported with one known fix** (Ears 4, 2026-10-16): the chords'
   octave is decided from the progression, not from the sounding events,
   so a pin or any non-harmonic change can't flip it (today's
   `separateRegisters` flipped it in 11 of 87 loops when only the chord
   lengths changed); *measured* by a corpus check that varies one
   non-harmonic choice per loop and finds no octave flips. Details in
   `docs/BRAIN.md`, "Start here". **And one known issue** (Mikey,
   2026-10-18; `docs/QUEUE.md`, "Known issues"): across the wander
   slider's whole upper range, "Let the loop wander" wanders in an
   experimental way that does not repeat from one play to the next (since
   phase 1's seeded draws, every replay wanders identically); the core's
   draws stay a function of what the host sends, the host salting the
   wander's seed per play in that range, where the range starts tuned by
   ear. Lower on the slider, a loop still wanders the same way every
   time. *Measured:* the same code played twice with the slider in the
   upper range wanders differently; below it, identically.
3. **Off the page.** The whole core runs on the audio side; the page
   sends commands and draws events. The scheduler, the lookahead and the
   JavaScript synth and generator are retired at the end of this phase
   (Mikey, 2026-10-13). If retiring the JavaScript synth earlier saves
   work, it happens on its own branch, merged only when the replacement
   is ready; `main` keeps the fallback until then. *Done when:* promise
   1's stall test is clean and the testers' phones report zero late
   notes.
4. **The game: Broken Friends** (Mikey, 2026-10-24; was Luanti).
   Pulled forward so a real game tests the host API while it is still
   cheap to change: the Godot extension, a native audio output, and
   steering (the game tells the core which creature, what time and what
   is happening; the core leans its vibes), with requests on events
   (promise 11). Earlier, from phase 1 on, the native command-line
   renderer and a tiny test host stand in for a game in CI. *Done
   when:* Broken Friends plays Driftloom following its creatures, time
   and events on an Android phone and in a phone's browser, with promise
   1's no-glitch measure.
5. **Instruments as recipes.** First the catalogue of block ideas for
   Mikey (promise 5). Then the building blocks and the recipe format;
   today's voices re-expressed as recipes where it fits (each proven to
   null against its Rust original, or A/B'd if it changes), the
   ears-approved sounds among them: since 2026-10-15 those are built
   before phase 2 as Rust blocks (the sound batch, items 54-73 in
   `docs/archive/queue-items.md`), so here they become recipes like today's voices.
6. **Codes, version 2.** Short codes, album seeds, recipe packs by hash,
   plain links. Old codes stop working; that's fine.
7. **The host kit.** The finished C header, the audio export in the app
   (roadmap 17; below) and the guide, with what Broken Friends taught
   fed back. *Done when:* promise 4's C program, promise 6's
   fresh-session test and promise 10 (Broken Friends) pass.
   **Exports, safe first** (Mikey, 2026-10-13): an export must never
   crash or break anything on the listener's device, old Android phones
   and weak chips included; speed is welcome only where it is safe. So
   the export renders in small chunks straight to the file (memory the
   same for one minute or three hours), checks free space before it
   starts, shows progress, can be cancelled, and survives the app going
   to the background. **Lossless** (FLAC, about half a WAV's size) is the
   safe format the core writes itself; a **small** format (Opus or AAC,
   about 70 MB for 2.5 hours mono) uses the encoder the phone or browser
   already has, where it has one. Decided (Mikey, 2026-10-13), keeping
   the settings rule: the format is chosen in the export sheet each time, and the
   existing "lighter on processor" setting decides the speed (on: gentle
   and cool; off: as fast as the device safely allows).
8. **Better than Chromium.** The sound improvements, one item each, blind
   A/Bs where they touch what he likes.
9. *(Stereo moved earlier, 2026-10-15: see "Stereo" below the phases.)*

**Stereo (Mikey, 2026-10-15), in steps:** ears sessions propose stereo
treatments as clips now; after the instrument batch (before phase 2),
the core gets its stereo render, the mono switch and the first space
recipes, drawn from the seed in the core (the mix, the echo and the
reverb in stereo; per-channel and per-instrument pan and width); phase 2
adds space to the request; space recipes keep growing like instruments.
Blind A/B against the mono render for anything that changes sounds
Mikey already likes.

Then the platforms, each its own phase: Android on F-Droid (alpha/beta),
itch.io desktop, later Google Play. iOS and macOS stay a recompile away.

Promises 2 and 6 aren't a phase: every PR in every phase is held to them.

## Targets

The numbers in promises 1 and 2 are agreed as starting targets (Mikey,
2026-10-13). They move as the work teaches: the brain tells Mikey when a
target looks wrong, or when aiming higher looks possible, with the
measurement that says so.

## Answered (Mikey, 2026-10-13)

- **Bit-identical everywhere: yes** (promise 3).
- **The JavaScript synth retires at the end of phase 3**; earlier only
  on its own branch, merged when the replacement is ready.
- **The targets stand**, to be revisited (above).
- **The building blocks are open to change** (promise 5).
- **Stereo across the board, with a mono switch** (promise 9; changed
  2026-10-15).
- **One game proves the core: Broken Friends** (promise 10; Mikey,
  2026-10-24). It replaces the three games of 2026-10-13 (Luanti,
  OpenRCT2, Stardew Valley), and is phase 4.
- **Requests: listeners and developers ask for bars, tempo, rhythm,
  scale and more, alongside vibes** (promise 11; added after approval).
