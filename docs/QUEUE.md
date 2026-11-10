# The queue

Written by the brain for Claude Code (the hands). Work top to bottom. Only
what is live is here: the status, the standing rules, the open items, the
known issues and the merge policy. Finished items, the old notes and the
Done list are in `docs/archive/` and `docs/DONE.md`; `docs/README.md` is the
map of all the docs. Mikey's ears come later, in listening albums the brain
builds from the Done list.

**Status, 2026-11-10:** `main` is at v151. Done: items 0-36 and 38-47 (13
stopped at its gate by design), the core's phase 1 (42-46c), the sound batch
(54-73), 74 and 76-76d, and **phase 2, the generator in Rust (items 77-92)**,
checked by the brain's Session 29 and heard by Mikey in its blind album
(2026-11-07). **Phase 3: item 93 done** (the player's module, 636 KB; the
brain accepted the size, `docs/notes/93-module-size.md`) and **94a done**
(#337), **94b done** (#343, after Mikey's device check), 99 done (#341),
and **95** (the picks through the core's commands); Ears 13b approved
(2026-11-09). **The road to 1.0 (a long hands run, 2026-11-10; what waits
for Mikey is in `docs/notes/for-mikey.md`):** 103 done (the fast gate, #354), 96 done (the checks on the player).
**Next: 97**, then 100, 98's hands part, 101, 84b, 102. Item
75 is low priority. 37 stays on hold. A new Claude Code session starts here:
read the standing rules and the merge policy, then take the next item.

**Two tracks:** app work (fixes, redesigns, vibe tuning, new controls) goes
on alongside the core track, as separate PRs; app-track sessions stay out of
`core/`, and core-track items don't wait for the app (`docs/BRAIN.md`, "Two
tracks").

## Standing rules, for every item

- **One PR per item** (split an item into a/b if that is cleaner). Start
  each from the latest `main`, take the next version number, and bump
  `BUILD` and `CACHE` together.
- **Merging:** see "Merge policy" at the end.
- **Measured, not guessed.** Every new voice, texture or kit: level
  calibrated to its layer's median at 0.4 s notes (`measure.mjs --voice`),
  `VOICE_COST` measured by render timing, and the tone probe's 2-5 kHz
  share at or below the flute's. That last one is the coffee-shop test:
  Mikey's north star is that nothing should bother people in a coffee
  shop. Every new profile: its `level` solved so its loops land on the
  catalog median (`measure.mjs --profile <id>`).
- **Refusals** on full and lite for the loops the item touches: melody at
  full quality stays near zero, and lite no worse than the catalog. For a
  requested loop the rule is read per profile: a request may not make any
  profile's lite refusals worse than that profile's own (brain, 2026-10-31;
  `docs/archive/notes/85-fast-pins-lite.md`).
- **The core baseline (items 32 and 96):** run `node tools/baseline.mjs`.
  It plays a fixed set on the player in Node (40 loops full and lite, mono
  and stereo, every voice, the refusals, the costs) and fails on any
  difference from `test/core-baseline.json`. A change that is not meant to
  change the sound must leave it passing; one that is runs `node
  tools/baseline.mjs --update` in the same PR and says in the body what
  moved and why. The JavaScript synth is the fallback until item 97 and
  keeps passing its own checks (`tools/measure-js.mjs --engine js` with
  `--refusals`, `--clicks`, `--endings`) when its code is touched.
- **The balance lock:** run `stats.mjs --check`. If a locked figure moves,
  report which and by how much, re-baseline at `--n 10000`, and say so.
- **Existing loops:** say exactly what changes for specs that already
  exist. Anything not named in the item must render identically.
- **Tests** three times.
- **Taste:** where the item leaves a musical choice open, take the more
  conservative option and write it down in the PR, marked "for Mikey's
  ears". Never guess silently.
- **American English** in new comments, docs and PR text (Mikey,
  2026-09-28). Don't rename identifiers or spec fields for spelling:
  share codes and history depend on them.
- **Stop and leave a note**, rather than push on, if a check can't be met
  or the item turns out to need a decision. Each note is its own file,
  `docs/notes/<item>-<short-name>.md` (the item number leads, so notes
  sort with their items; `docs/notes/README.md` says what goes in one).
  Say so in the PR. Notes before 2026-10-26 are in
  `docs/archive/notes.md`.
- **When a PR is merged,** add one line to `docs/DONE.md`: the PR number,
  the commit preview URL, and what the ears should listen for. (Lines
  before #277 are in `docs/archive/done.md`.)
- **Items that touch `core/`** also follow `core/AGENTS.md` (the core's
  standing rules, and the rules for sound items).
- **Finished items** are in `docs/archive/queue-items.md`, by number: an
  older item cited as "item 46" is the text there.

## 75. The reverb, rate-true for steady tones (low priority, after the batch)

**Hands:** Opus 5.5, high. From item 62 (brain, 2026-10-20).

**Why:** the reverb's combs (feedback 0.8) catch a held tone differently at
each sample rate: a held sine into the chords channel spreads 1.1 dB across
rates, while the channel itself is the same at every rate. Matching the comb
loops' lengths in time alone didn't fix it (the loop's lowpass and the delay's
interpolation matter too). The chapel organ, a held instrument with a room
send, is the first sound it shows in: golden 16-grove (5 s, the organ) reads
+0.29 dB at 44.1 kHz and +0.34 at 96 kHz against 48 kHz, so
`core/golden/rates.txt` marks it `steady` and `tools/rates.mjs` holds it to
0.4 dB, every other list to 0.2.

**Aim:** 48 kHz bit-identical (goldens and baseline unchanged), and every rate
list back inside 0.2 dB, so the 0.4 dB tolerance (and the `steady` line) can be
removed. If 48 kHz must move, stop and leave a note for a decision.

## Phase 3: off the page (items 93-98)

`docs/CORE.md`, phase 3: the whole core runs on the audio side; the page
sends commands and draws what the core reports. The scheduler, the
lookahead and the JavaScript synth and generator retire at the end of this
phase. *Done when:* promise 1's stall test is clean and the testers' phones
report zero late notes. The notes under "For phase 3" below are part of
every item here; item 53's brief (below them) is item 95's. Until 97, the
standing rule holds: a change to what is generated goes into both
generators.

**Decided for phase 3 (Mikey, 2026-11-07):** once the JavaScript synth is
gone, a browser that can't run the core (no WebAssembly or no AudioWorklet)
shows one plain line instead of playing, "This browser can't play
Driftloom; try a recent Chrome, Firefox or Safari", in the info card's
place. No fallback synth is kept for it.

## 94a. Everything the app does, as a command or an event

**Hands:** Opus 5.5, high. Core and its wasm only, no app change yet.

**Why:** today `?engine=player` plays a code's seed and request, and the
page's engine still does the rest. Before the app can move, every action
and every thing it shows must have its way through the core.

**What:** list every control and display the app has (Play, Next,
Previous and history, a re-rolled layer (`rerollLayer`, the layers'
seeds), mutes, tempo, the Sound panel's warmth, space and wobble (`tone`),
the length slider (`playFor`), "Let the loop wander" and its salt, the
picks, volume, mono, lighter on processor, export, the cursor and the
layers' grid, the info card (name, style mix, vibes, key, bpm, metre,
bars), the cover, share codes, Diagnostics), and give each its command or
event in the player and the wasm (and the C header where a game would
want it): what exists (`load`, `change`, `set_mute`, `set_tempo`,
`set_volume`, `set_wander`, `position`, `pass`, `piece`), what is missing
(at least: a layer's seed, the tone, the loop's description for the info
card and the cover, the grid's notes for a pass, the events that say where
the loop is: step, pass, phrase line, section). Build what is missing,
with tests. Say which of `js/share.js`, `js/cover.js` and the rest still
need the JavaScript generator, and how they stop needing it (a code is
decoded to a seed, a request, layer seeds and tone; what it shows comes
from the core).

**Yardsticks:** a table in the PR, every control and display against its
command or event; the new ones tested natively and in wasm; nothing that
plays moved (goldens, seeds, baseline).

## 94b. The app plays the player

**Hands:** Opus 5.5, high. App and worklet.

**What:** the app's default engine becomes the core's player: the page
sends 94a's commands and draws its events; the page's engine keeps no
lookahead and decides no notes (`?engine=page` keeps today's path, for
comparison, until 97). Next, Previous and the length slider's hand-over
are `Player::change` at the moments the app uses (a crossfade; 88b's
"next phrase line with a short fade" when the slider ends a long loop);
the automatic lite switch (87b) keeps today's fade and the loop from its
top, now on the player (brain, Session 30: a crossfade needs two cores at
once, twice the cost on the slow device that is switching, or a mix
switchable while it plays, which is core work; Mikey, 2026-11-03: the fade
"doesn't bother" him). A one-core crossfade is a later item. The page is
free to stall: the worklet has what it needs. If 94b needs a core change,
keep it small: item 99 may be in `core/` at the same time (the sketch build
only).

**Yardsticks:** every row of 94a's table works in the app (a check per
row, scripted where it can be); promise 1's stall test on the player (the
page's main thread blocked for 2 s, hidden for a minute, frozen): no late
note, no underrun; `tools/views.mjs` on every view; then Mikey on his
phone and tablet with Diagnostics ("For phase 3": the tablet's new-loop
case, the phones' costs). **Merge after Mikey's check** on the PR's preview
(Mikey, 2026-11-03: a check on the tablet before the app switches), not on
green CI alone; then the brain's phase check.

## 95. The picks through the core's commands (was 53)

**Hands:** Opus 5.5, high. Item 53's brief, below, with 88b's meaning of
the bars picker (the loop's length; the length slider says when it ends).

**Built (hands, 2026-11-08, v151):** the panel's request is a command
(`Player::set_picks`, `dl_play_picks`), held by the core with a number the
page gives it; the core makes every new loop itself (`Player::next` at Next,
and a track's end with nothing of the page's armed), its seeds drawn from a
seed the page sends each core (`set_draws`), and says which (`Kind::Own`,
the picks' number, before the `Loop` with the seed); the page draws the same
spec for its card and history and sends nothing. Picks the core can't meet
are held as their vibes, and the panel's line is the core's own words. The
pills and the Bars picker's time read the picks the core holds. An armed
loop carries its own track (`change_with_track`), so a page asleep at a
hand-over leaves the core playing what it would have set. The opening loop
(before Play) is still drawn on the page: there is no core before a tap.

## 100. When a browser is too slow even on lite

**Hands:** any. App only; after 97.

**Why:** a friend's Pixel 9 on GrapheneOS played "too fast and really
skippy" (v151, 2026-11-10): its browser, Vanadium, runs WebAssembly on an
interpreter unless JIT is allowed for the site; the player took 54.7% of
real time on lite interpreted and 9.3% with JIT allowed (about 6x). The
skips are the audio output running dry, which the core's counts can't see.
**Mikey (2026-11-10):** no browser workarounds and no cushion or lower tier;
a browser too slow even on lite gets a plain line telling them to get the
Driftloom app.

**What:** notice it (lite past a line on the switch's own measure, set from
the tablet's and the phones' figures so Mikey's tablet never trips it; or
fill-ins where `playoutStats` reports them); one line where the "can't play
Driftloom" line goes (Mikey, 2026-11-07): "This browser is too slow to play
Driftloom smoothly. Get the Driftloom app." The link is one constant, empty
until the Android app has a page (F-Droid), and the second sentence shows
only when it is set. Play still works; no pop-up, no new setting.
Diagnostics: `slow: lite past the line, Ns`.

**Yardsticks:** `tools/autolite.mjs` with a `stuck` case shows the line;
"steady", "slow" and "yours" never show it; `tools/rows.mjs` passes. BUILD
and CACHE together.

## 96. The checks on the player (done, hands, 2026-11-10)

**Built:** `tools/measure.mjs` and `tools/baseline.mjs` play loops from
their seeds on `js/dlplayer.wasm` in Node (`tools/play-lib.mjs`, the meters
in `tools/meter.mjs`): no browser, and the up-front harness is gone for the
core (the old tool is `tools/measure-js.mjs`, for the JavaScript fallback
until 97). The voice probe, a request as text and the budget's counts are
the core's `tools` feature (`tools/build-core.sh --tools`, never shipped;
`player/probe.rs` hands the sequencer a loop of one note). For the same 40
loops the old path and the player agree to 0.005 dB in loudness and peak;
the baseline's `voices`, `player` and `costs` parts did not move, and
`loops`, `stereo` and `refusals` are now the player's 45 s renders, on
purpose. `tools/perf.mjs` and `tools/views.mjs` drive the app, which plays
the player since 94b.

## 97. The JavaScript retires

**Hands:** Opus 5.5, high.

**What:** `js/synth.js`'s JavaScript voices, the page's scheduler and
lookahead (`js/engine.js`, `js/clock.js`), and the JavaScript generator
(`js/generator.js` and what only it uses: vibes, moods, characters,
theory, choices, pins on the page) are removed, with their tests and the
`?engine=js` and `?engine=page` paths; the generation goldens stay, held
by the Rust generator alone (`tools/gen-golden.mjs` keeps `--check`
against the hashes). The plain line for a browser that can't run the core
(Decided, above). Data the page still reads (`data/vibes.txt` for the
picker's words, and the like) comes from the core or stays as data.

**Yardsticks:** the page's JavaScript before and after (lines and bytes);
every test that remains passes; the generation goldens in Rust; the app on
Mikey's phone and tablet as in 94b; the plain line shown in a browser with
WebAssembly turned off.

## 98. The phase's proof

**Hands:** Sonnet 5.5, high; then Mikey and the testers.

**What:** promise 1's stall test, clean, on the shipped app; the testers'
phones (Mikey's Moto and tablet, and the friends' Pixels on GrapheneOS)
report zero late notes in Diagnostics over a listening session each, with
a few Nexts, a long loop, Endless and the screen off. Write the steps for
the testers in plain words, one paste each.

**Then the brain:** its phase check, and a control album (today's app
against the app before phase 3, the same codes): it should sound the same.

## 99. The sketch levers for Ears 13b (alongside phase 3)

**Hands:** Sonnet 5.5, high. After 94a merges, from the latest `main`; it
may run alongside 94b. The sketch build only (`--features sketch`): the
app's and the player's modules stay byte for byte (`tools/build-core.sh
--check`), and nothing that plays moves (goldens, seeds, baseline).

**Why:** Ears 13 (`docs/ears/batch-6.md`, "Levers wanted") sketched five of
its twelve candidates; the other seven need a hook item 83's levers don't
have. The ears session never writes core code, so the hands add the hooks
and an Ears 13b sketches with them.

**What,** each as small as the batch's table says, in `core/sketch/`'s way
(a forced choice, a `tuning.rs` name, a pin, or a clip lever in
`tools/clips.mjs`), all under `#[cfg(feature = "sketch")]`:
- **tripletKit:** a forceable choice that re-times the kit's hats and ghost
  hits from four sixteenths to three thirds of the beat (no straight
  eighth), the kick on the beat.
- **dottedPulse:** a forceable choice adding a soft part (a rim or a tuned
  tick) every three eighths, or the kick on a 3-3-2.
- **planingSevenths:** a forceable option of the progression draw: every
  chord a major seventh, roots a whole step or a minor third apart.
- **slowOpening:** `latefloor.SECONDS_MAX` and `latefloor.FADE` in
  `tuning.rs`'s `NAMES`.
- **tuningOffset:** `master.CENTS` in `NAMES`, read where a note becomes Hz.
- **widePad:** the side reaches whole-channel blocks in a stereo render
  (`render_side`, as the buses get it), or a clip lever that sets a space
  setting past its range; take whichever is smaller.
- **dropSpotlight:** the sketch hears the floor drop (a route into the
  sketch's `MODS`, as subDuck is keyed, or a floor-drop event to the
  channels).
- **Chord sketches:** in the sketch build, the chords channel gets a
  retune at every chord (today only while the organ plays), so a chord
  sketch sees arpeggiated chords whole without a silent organ.

**Yardsticks:** each lever tested under `--features sketch` (it moves what
it says, and off it is bit for bit today); `core/sketch/README.md` and the
header of `tools/clips.mjs` document the new levers; clippy pedantic clean
with the feature; the standing checks unchanged.

## The road to 1.0 (Mikey, 2026-11-10)

**The app's 1.0 comes before the game** (Mikey, 2026-11-10): Broken Friends
and the other games wait (the core already works in Broken Friends); the
music app is finished first, and **1.0 includes the Android app on
F-Droid** (Mikey, 2026-11-10). Order, after 103: phase 3's 96, 97, 100 and
98's hands part; then 101, 84b and 102; then 106, 104 and 105; then 107;
then the known issues, the polish, and 108. **Screens:** the hands build
plain ones in the app's current style, keeping `docs/PAINTER.md`'s rules
(one screen, no new settings, scrolling in its two places); a Painter
session may restyle any of them later (Mikey, 2026-11-10). Each sound change merges under the policy and is
gathered into one listening album for Mikey; what needs his ears or a
decision goes in `docs/notes/for-mikey.md` and the work moves on.

## 103. The fast gate (done, hands, 2026-11-10)

**Built:** `.github/workflows/core.yml` is the gate, on every PR and push,
its jobs side by side, each one part of `tools/gate.sh` (what a session runs
locally): `core` (`build-core.sh --check`, `cargo test --locked`, clippy
pedantic), `sketch` (the same, sketch build), `node` (every
`test/*.test.mjs`, `gen-golden.mjs --check`, the balance lock on the Rust
generator) and `baseline` (the "nothing moved" guard, 11 minutes in one
piece, so in four groups of its parts). `.github/workflows/nightly.yml`, on
a schedule and by hand, never gating: every target's golden renders,
Emscripten, Endless for 24 hours, `gen-golden --chromium`, `ahead --sweep`,
`rates.mjs` and `stats.mjs --check`. A session looks at the nightly's
latest run before it starts an item and fixes a red one first. **Back as
gates at 1.0 on F-Droid** (item 108).

## 101. Batch 6, built

**Hands:** Opus or Fable, high. After 97 (one generator, so each change is
made once). Split as 101a, 101b and on, one PR each, in batch-6's ranked
order or the order the code wants.

**What:** the fourteen approved (`docs/ears/batch-6.md`, Verdicts; Mikey,
2026-11-08 and 2026-11-09, all at the sketched levels, every version as an
option weighted by the vibes): tripletKit, lowBell (struck, swelling),
padWow, widePad, colorTone, deepDuck, wideSweep (wide, fast), dottedPulse
(pulse, kick332), slowOpening, planingSevenths, dropSpotlight (struck,
swelling), tuningOffset, and twang (soft, slow, bright), each as its
section and "For the brain" say. Batch 5's rule (Mikey, 2026-10-18): today's
behavior stays one option of every choice; weights from the batch's
[JUDGMENT] lines, never 0. Where a lever fell short ("What the levers
did"), the build finishes it and says how. The standing rules for sound
items (level, cost, the coffee-shop probe, refusals); the balance lock and
the baseline move on purpose, said in each PR. Then the batch album
(Session 26's way): loops with each, for Mikey.

- **84b, the chords nearer the tune** (Mikey's blind album, 2026-11-07): of
  item 84's octave moves he preferred, wherever he heard a difference, the
  chords nearer the melody: where 84 lifted the chords up toward the tune
  (gap about 17 semitones to about 5) the new side won 3 of 3, and where it
  dropped them away (about 5 to 17) the old side won 2 of 3 (the third,
  sung-hal, "a slight difference, but I might not"). Keep 84's rule (the
  octave read from the progression, so a pin can't flip it) and change its
  aim: the chords sit within about a fifth to an octave under the tune,
  never pushed down to about 17 semitones under it. It merges under the
  policy; more of 84's downward movers go in the album for Mikey, and a
  revert follows if he hears worse.

## 102. Long pieces, composed

**Hands:** Fable, high. After 101; the composing work promise 11 left for
later (`docs/CORE.md`, "Any length, up to endless";
`core/src/generator/piece.rs`).

**Why** (Mikey, 2026-11-10): the longer lengths should sound composed.
Today a piece past 32 bars is its loop played section after section, each
the loop's next wandered pass: one idea, wandering. Mikey wants both
journeys (2026-10-13): **an arc**, as a symphony's (themes that return,
changed; a build and a release; related keys; a close), and **long, slow
wandering** (one thing at a time moving, long held states, nothing
abrupt), chosen per piece by the vibes and somewhat at random (his taste),
and pinnable by a request.

**Rules:** promise 11 holds: any section reached straight from the seed and
its number, the same memory and cost for 64 bars or Endless; Endless never
ends (arcs in turn, or the wander); loops of up to 32 bars play exactly as
today (the balance lock holds); changes at phrase lines, never mid-phrase;
music computers are good at, not copies (Mikey, 2026-11-08). The
generation goldens' long-length cases move on purpose, said in the PR.

**Yardsticks:** a piece map (`dlgen --map SEED REQUEST`: each section's key,
tempo, energy, theme and journey) for any length; corpus checks that an
arc's themes return and its energy rises and falls, that a wander moves at
most one thing a section, and that a section's cost doesn't grow with its
number; then, for Mikey, a listening set: a few pieces each at 64, 256 and
1,024 bars and Endless, as album codes he plays in the app, with short
excerpts (the start, a quarter, half, three quarters, the close) so a piece
can be judged in minutes.

## 106. Long pieces on screen

**Hands:** any, high. After 102. Plain screens (above), from
`docs/PAINTER.md`, "Long pieces": where you are (bar 812 of 3,072, time
played and left; for Endless, time played), moving within a piece (a jump
to a section or a point, from the seed and the section's number, promise
11), section markers from 102's piece map, and a piece's length or
duration in the Library. **Yardsticks:** a jump to three hours in starts
within a second, nothing late (`tools/rows.mjs` gains the rows).

## 104. Audio export

**Hands:** Opus or Fable, high. `docs/CORE.md` phase 7, "Exports, safe
first" (Mikey, 2026-10-13), and `docs/PAINTER.md`, "Exporting audio": the
core renders in small chunks straight to the file (memory the same for a
minute or three hours); free space checked first; progress with Cancel;
it survives the app going to the background. **Lossless** (FLAC, written
by the core) and **small** (Opus, through the browser's own encoder where it
has one, else that choice is hidden); the format chosen in the export sheet
each time; "lighter on processor" sets the speed (on: gentle; off: as fast
as is safe). Endless asks how much. **Yardsticks:** a three-hour lossless
export on the Moto-class figures in `tools/autolite.mjs`'s slow case
finishes with flat memory and nothing glitching in what plays; the file
decodes and nulls against a straight render; cancel and a full disk leave
nothing broken.

## 105. Short codes and links (codes, version 2)

**Hands:** Opus or Fable, high. Before 1.0, after 102 and 104, so it
carries everything 1.0 has; **from 1.0 on, codes stay stable** (Mikey,
2026-11-10). `docs/CORE.md` phase 6 and promise 8, `docs/PAINTER.md`,
"Share codes, version 2": short plain codes (what was pinned beside the
seed, nothing else), plain links that open the loop, an album from one
seed, and plain "needs a newer version" lines. Old codes stop working
(Mikey, 2026-10-12: that's fine before 1.0).

**Room to grow** (Mikey, 2026-11-10): a code carries a version; everything
in it is named by a stable id, never by its place in a list, and ids and
lengths are variable-width, so new instruments, profiles, vibes, styles,
picks and later a modder's recipe fit without a new format. **Additions
leave old codes as they sound:** each instrument, profile, vibe and option
records the version it joined, and a code's draws see only what existed at
its version (a pool is filtered by version before the draw, so a newer
member can't shift an older code's picks). A deliberate improvement to an
existing sound still changes old codes, and the release notes say so. A
code naming something this app doesn't have says "needs a newer version",
never plays the wrong thing.

**Yardsticks:** a loop with nothing pinned has a code under about 20
characters; every pick, length, journey and edit round-trips; the
generation goldens hold; a test adds a dummy instrument, profile and vibe
at a later version and shows every golden code renders bit for bit the
same, while a new code can draw them; a code from a "newer" version is
refused plainly.

## 107. The Android app, on F-Droid

**Hands:** Fable, high. After 105. Mikey (2026-10-12): from the same Rust
core, with the web screens inside it; the website stays. The core native,
on Android's own audio (AAudio, through a small Rust layer or Oboe) at a
high priority, the page's screens in a WebView talking to it by the same
commands and events as the worklet (94a); no Google services, nothing
F-Droid's rules refuse. Built in CI (GitHub's runners have the Android
SDK), an APK Mikey can install, `fastlane` metadata, and an F-Droid build
recipe with reproducible-build notes. **For Mikey:** the submission to
F-Droid (his account) and the signing choice. Then item 100's link points
at the listing. **Yardsticks:** the Moto and the tablet play with nothing
late (Diagnostics in the app), the screen off for ten minutes, a long
piece and an export; the APK builds from a clean checkout.

## 108. 1.0

**Hands:** then Mikey. The version named 1.0 in the app and the APK, a short
changelog, codes frozen (105), the nightly's checks back as merge gates
(103), and `docs/notes/for-mikey.md` empty or answered.

**Also noted, not on the road to 1.0:**

- **The player's stack** (item 94a, PR #337): its calls take 56 to 60 KB
  of the caller's stack (wasm32, measured; `main` the same), near
  Emscripten's 64 KB default; the header says `-sSTACK_SIZE`. Fine for the
  browser (1 MB) and native games. Before a port to a small device (the
  charter's handhelds and consoles), an item trims the generator's frames
  with a measured aim; for Broken Friends' web build, the header's line is
  enough.

## For phase 3 (from item 82; brain, Session 28)

- **When notes reach the core** (`docs/archive/notes/82-hand-over.md`): the
  core sums its voices in an order that depends on when notes were handed
  over, so the up-front harness and the player differ in their last bits (77
  dB or more under the music). **Brain's call: no canonical order now.** The
  player's fixed hand-over is what every check holds, and the app's own
  engine already varies in its last bits play to play, inaudibly. In phase 3,
  when the app plays the player and the page's hand-over goes away, move the
  core baseline's older parts (`tools/baseline.mjs`'s loops, refusals and
  the rest) onto player renders, so one way of playing is checked, and retire
  the up-front harness for the core (it stays for the JavaScript fallback).
  A canonical order only if a host turns up that must hand notes over itself
  and needs bit-identity.
- **The automatic lite switch becomes a crossfade** (Mikey, 2026-11-03: the
  fade and restart from the top doesn't bother him, a proper crossfade is a
  good idea): item 87b's `quality.handOff` moves onto `Player::change`.
- **A new loop's first notes on a slow device** (brain, 2026-11-03; Mikey:
  wait for phase 3 rather than fix the page's scheduling now). On v142 his
  Lenovo tablet left out 10 notes in a minute at two new loops (87b's switch
  at 19.5 s, a new loop at 56.0 s): `304-350 ms behind the core, posted
  250-339 ms ahead (inside the buffer), left out; decided by play, new loop
  [margin]`, each followed by a render-ahead jump to 600-643 ms
  (`worstRenderAheadMs: 643`, `leastSpareMs: -315`); the Moto had none.
  Phase 3 removes the page's note hand-over, so the notes themselves go; but
  find what made the core jump at those moments (the new loop's work on the
  page, or the new worklet compiling on the audio thread at the switch), since
  a stall of the audio thread there would still be heard. Yardstick before
  the app switches: the tablet shows no late notes and no underruns over a
  few minutes, a few Nexts and an automatic switch.
- **Phones:** the player's costs were measured on a cloud machine; a block
  while a loop is made is 9.6% of its deadline in wasm. Check a phone
  (`?engine=player`, Diagnostics: late and dropped notes) before the app
  switches.

## 53. The picks through the core's commands (phase 3)

**Why:** phase 3 moves the whole core off the page; the page only sends
commands and draws events. The picks become a command like the rest,
and long pieces and Endless play from the core alone.

**What:**
- The panel's request is sent as a command; the core holds it and
  applies it to every new loop, re-roll and wandered pass. No pick
  logic left on the page.
- Long pieces and Endless play with the page asleep, hidden or busy
  (promise 1's stall test), with no lookahead on the page.
- Now Playing's pills, the panel and the Length picker's play time read
  from the core's events, so what they show is what the core will play.
- Long-piece display on Now Playing (where you are, time left, Endless
  with no "time left": `docs/PAINTER.md`, section 2) only once a Painter
  session has designed it; until then, today's clock.

**Yardsticks:**
- Promise 1's stall test clean with a 32,768-bar piece and with
  Endless.
- A pick made while a loop plays changes the next loop, never the
  current one.
- Picks survive the page being hidden and shown again, and a reload
  (stored as the vibes are today).

## 37. Word albums on `/rank`: one word, "does it fit?"

**On hold (Session 16): skip it unless the brain's State says otherwise.**
Vibes in the app (item 38) may do this job better: Mikey picks a vibe and
tells the brain which loops fit.

Mikey's idea (2026-10-11): "Is this playlist heartwarming? Which songs are
not?" The brain sends a word album (item 35b); Mikey answers each track
with one tap. The pilots, "Heartwarming?" and "Joyful?", went out as
plain albums (`docs/ALBUMS.md`); **if the brain's State says the pilot
didn't work, skip this item.**

- **`rank.html?album=<DLA1 code>`** (the word from the album's title,
  "Heartwarming?" -> heartwarming; `&word=` overrides): plays the album's
  loops in order, each as composed (`driftOn = false`, as now), looping
  until answered. Shows only the question, "Heartwarming?", and the track
  number of the total; no profile, mood or title per track.
- **Buttons:** Fits, Doesn't fit, Not sure; Undo goes back one. A track
  can be answered while it plays; the next starts at once.
- **Record** each answer as a line in the same export, so `ranks.mjs`
  reads it with the rest: `trial`, `build`, `code`, `album` (the album
  code's first block and the word, enough to tell albums apart), `track`,
  `words: [word]`, `taps: [word]` for fits and `[]` for doesn't fit,
  `answer: "fits" | "no" | "unsure"`, `seconds`, `quality`. Saved as
  `/rank` saves now; Export and Clear as they are.
- **`ranks.mjs`** counts album answers apart from the five-word trials
  (per album: fits, doesn't fit, not sure), and `--fit` uses them (item
  35a). The five-word trial stays as it is without `?album=`.
- No hidden repeats in albums (they are short); no generator, synth or
  share-code change; the balance lock doesn't move. The page plays the
  core like the app (item 32's note: `/rank` still builds the JavaScript
  `Synth`; wire it to the core here, with `?engine=js` as the fallback,
  and say what changed). Bump `BUILD` and `CACHE`.
- **Checks:** both pilot album codes in `docs/ALBUMS.md` open, play and
  record on the core and `?engine=js`; an export holding both kinds of
  line reads in `ranks.mjs`; tests three times.

## Later, not queued

Mikey liked the fiddle, accordion and drone as they are, and may want
more nuance in them later. Not now. New instruments are welcome again
(Mikey, 2026-10-09; the 2026-09-25 pause was temporary): the ears-approved
sounds are queued after the core becomes the default, built in the core
one at a time with the refusal and level checks (`docs/BRAIN.md`, State).
Inspiration comes only from wholesome, comforting music.

SIMD in the core (brain, 2026-11-02): four samples at once in the voices,
filters and mix, in wasm (SIMD128) and natively (NEON, SSE2), for promise 2
and the weaker phones (87's tablet). Bit-identity holds only with the same
operations in the same order per lane (no fused multiply-add, no reordered
sums), proven on the goldens on every target; worth an item once phase 3
is done, or sooner if a phone needs it. A native Android app's own audio
(AAudio through Oboe, a fast core, a high priority) is its platform phase's.

A sketch voice for a layer (archive/notes/83-sketch-voice.md): option 1, sketches
stay channel sources until an ears session needs a voice's note-by-note play
(brain, 2026-10-31).

## Known issues

- **"Let the loop wander" repeats itself since item 44** (Mikey,
  2026-10-18). Since #211 each pass's wandering is drawn from the loop's
  code and the pass number, so replaying a saved loop wanders exactly as
  it did last time, at every slider setting. Mikey: the wander slider
  (0 to 2.4) says how far a loop wanders, and **all the way to the right
  it should feel experimental: not repeatable.** Not urgent. Brain's
  sketch for when it is queued (the generator's port, phase 2, is the
  natural place): the core's draws stay a pure function of what the host
  sends (promise 3 holds); the host adds a per-play salt to the wander's
  seed across the slider's whole upper range (**decided by Mikey,
  2026-10-18**; where that range starts is to be tuned by ear as we
  go), so the same code with the slider lower still wanders the same
  way, and a share code or export says which it was.
  **Built as item 92 (hands, 2026-11-03); the rest of this entry is history.**
  **Queued as item 92 (brain, Session 26).** Where it was written down (brain, Session 24, 2026-10-19): to be
  built in phase 2's port, not before; carried in `docs/CORE.md` (phase
  2, beside the octave fix) and in `docs/BRAIN.md` ("Start here"), so
  whoever writes phase 2's items queues it. Until then the sound batch
  leaves it as it is: item 54's per-loop choices are drawn once a loop
  and are not redrawn by the wander, whatever the slider says (item 67's
  kick swap aside); with the salt, the upper range's wandering would vary
  play to play, while the loop's choices stay its own.

## Merge policy

Mikey decides this line:

- (yes, from Mikey) When CI is green and every check above passes, Claude
  Code merges its own PR and starts the next item.
- (Mikey, 2026-11-10) Until the app is 1.0 and on F-Droid, "CI is green"
  means the fast gate (item 103: the `core` and `docs` workflows, locally
  `tools/gate.sh`); the nightly's slow renders (`nightly.yml`) never hold a
  merge, and a red nightly is fixed by the next session that sees it.
