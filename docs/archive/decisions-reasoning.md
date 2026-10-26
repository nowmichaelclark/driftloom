# Decisions: the reasoning

The long entries that `docs/DECISIONS.md` keeps in short form, as they stood in
`docs/BRAIN.md` ("What Mikey has decided") before 2026-10-26, word for word.
Each heading is what the short line links to.


## Engine candidates (brain, 2026-10-10)

- Candidates (brain, 2026-10-10, unranked): a warm compressor start (no
  first-note dip; done in 31b); smooth curve and parameter changes where Chromium
  switches abruptly (the saturator's curve swap); better oversampling
  and a true-peak ceiling; a reverb designed for this music rather than
  six plain combs (a sound change: blind A/B); an offline renderer for audio export (roadmap
  17) from the same core; a documented host API for native and game
  hosts (`no_std`-friendly, sample-rate independent, deterministic);
  the generator in Rust when the words settle.


## Stereo possible, mono the default (superseded 2026-10-15)

- *(Superseded 2026-10-15.)* **Stereo possible, mono the default (Mikey, 2026-10-13)** -- replaces
  the line below. The app stays mono for now; stereo comes much later,
  experimental first, and the core must make it implementable without a
  rebuild. Stereo's variables (width, per-instrument pan that changes
  track to track, how much and how fast things move, and more) must be
  easy for developers and users to tweak. Carefully: "stereo holds its
  own set of infinite possibilities". `docs/CORE.md`, promise 9.


## Mono only, for now (superseded 2026-10-13)

- *(Superseded 2026-10-13.)* **Mono only, for now (Mikey, 2026-10-11).** The ears put stereo first;
  Mikey decided against it: mono gives a controlled soundscape that
  sounds the same from system to system, where stereo can sound wildly
  different. The output stays one channel. The core's per-channel
  structure (one lane today) costs nothing and can stay, but nothing is
  built toward stereo: no ping-pong echoes, no panning, no width.


## A seed or share code is a recipe (brain, 2026-10-12)

- **For the charter (brain, 2026-10-12; Mikey's question):** a seed or
  share code is a recipe, so "same seed, same music" holds only with the
  same core version and the same data. Mikey asked whether a code could
  carry a modder's new instrument to someone who doesn't have it. Since
  Driftloom's instruments are synthesized, not sampled, an instrument as
  **data** (a small patch description, not code) could be a few hundred
  bytes and could travel inside the code or be named by it with a hash.
  Instruments are Rust code today; making them data is a charter-level
  choice. The code should at least say what it needs (core version, packs
  with hashes) and degrade clearly when something is missing.
  **Mikey likes doing both** (2026-10-12): hand-written Rust building
  blocks, and instruments as small data recipes (patches) that wire and
  set them -- like a synth's presets.


## Shorter share codes (Mikey, 2026-10-12)

- **Shorter share codes** (Mikey, 2026-10-12): an album fills half a phone
  screen in Discord; not tidy. Brain's proposal for the charter: with a
  deterministic, versioned core, a loop's code is the version, the seed
  and only what the listener changed (vibes, mutes, edits) -- about 10-16
  characters; an album, a list of those (about 60 for eight loops, against
  over a thousand now). Shared as a link, so Discord shows a tidy preview.
  Explicit parameters were written down only so a code survived generator
  changes; the version in the code does that job now that anything may
  break.
  **No online server for albums** (Mikey, 2026-10-12): codes carry
  everything. He'd like them shorter still. Levers, biggest first: fewer
  bits (an album drawn from one album seed is a single seed; seeds sized to
  what's needed); then a wider alphabet (base32's 5 bits a character;
  URL-safe base64, 6; a Unicode alphabet such as base2048 or emoji, 10-11,
  so an eight-loop album of hand-picked seeds in about 25-30 symbols, not
  typeable but fun). Keep one plain, link-safe form whatever else exists.
  **Decided (Mikey, 2026-10-12): codes never need reading aloud, but must
  show correctly on almost any device of the last 30 years** (a DS, a PSP,
  an old computer, "maybe some wild dev will want to port the core to
  something like GameCube"). So **plain ASCII only**: no emoji, no other
  Unicode. Brain's recommendation: the 64 link-safe characters
  (`A-Z a-z 0-9 - _`), 6 bits each; the wider ASCII sets (base85, base91)
  save only about 8% and carry characters that break links and Discord's
  formatting. The same rule for everything the core emits (names, codes,
  errors).


## Next picks, the design (Mikey, 2026-10-13)

- **Next picks, the design (Mikey, 2026-10-13):** chosen after four
  rounds and 70 options (Painter session 5); the brief is
  `design/next-picks.md` (it wins over the renders). In short: a **New
  loops** panel of five lines -- moods (the vibes), tempo and length,
  key and scale, style, voices -- each value an outline pill in its
  line's color, with an **any** key per line; **Now Playing** outside an
  album shows the picked values as pills (one button into the panel)
  and drops "Next: something new"; in an album the picks are hidden and
  Now Playing stays as today; a **Length** picker with cancel / done,
  -1 / +1, a typed number, a doubling slider 1 to 32,768, and Endless.
  Moods past three end in "...". Tempo, key, style and voices pickers
  are not designed yet. **Painter 6** (app track) builds the screens:
  moods wired to vibes; length and tempo to today's generator only if
  cheap, clamped. The real wiring is queue items 48-53 (phases 2 and 3).
  **Open, for Mikey:** (1) the way into New loops during an album
  (Painter suggests today's top-left Vibes key renamed "new loops", in
  albums only; until he decides, the Vibes key stays); (2) how to set
  only length, or only tempo, back to "any" (line two's any key clears
  both; until he decides, the line-level key alone is built).
