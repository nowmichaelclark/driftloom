# The queue

Written by the brain for Claude Code (the hands). Work top to bottom. Only
what is live is here: the status, the standing rules, the open items, the
known issues and the merge policy. Finished items, the old notes and the
Done list are in `docs/archive/` and `docs/DONE.md`; `docs/README.md` is the
map of all the docs. Mikey's ears come later, in listening albums the brain
builds from the Done list.

**Status, 2026-11-08:** `main` is at v149. Done: items 0-36 and 38-47 (13
stopped at its gate by design), the core's phase 1 (42-46c), the sound batch
(54-73), 74 and 76-76d, and **phase 2, the generator in Rust (items 77-92)**,
checked by the brain's Session 29 and heard by Mikey in its blind album
(2026-11-07). **Phase 3: item 93 done** (the player's module, 636 KB; the
brain accepted the size, `docs/notes/93-module-size.md`) and **94a done**
(#337); **next: 94b, and 99 alongside it** (the sketch build only). Item
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
- **The core baseline (item 32):** run `node tools/baseline.mjs`. It renders
  a fixed set on the core (40 loops full and lite, every voice, the
  refusals, the costs) and fails on any difference from
  `test/core-baseline.json`. A change that is not meant to change the sound
  must leave it passing; one that is runs `node tools/baseline.mjs
  --update` in the same PR and says in the body what moved and why.
  Nulling against JavaScript (`measure.mjs --null`) is no longer the gate
  for the core; the JavaScript synth is the fallback a phone without the
  core hears and keeps passing its own checks (`--engine js` with
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

## 96. The checks on the player

**Hands:** Sonnet 5.5, high.

**What:** the core baseline's older parts (`tools/baseline.mjs`'s loops,
refusals and the rest), `tools/measure.mjs`, `tools/perf.mjs` and
`tools/views.mjs` render and measure through the player, so one way of
playing is checked ("For phase 3": the up-front harness retires for the
core). Say what each tool's figures were on the old path and on the
player, for the same loops, and update the baseline on purpose.

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

## After phase 3 (brain, 2026-11-07)

- **Ears 13's sound batch** (the second synthesis, piles 3-6), built and
  heard before phase 4 (Mikey, 2026-10-15: more instruments, sounds and
  vibes before the game). With one generator, each change is made once.
- **84b, the chords nearer the tune** (Mikey's blind album, 2026-11-07): of
  item 84's octave moves he preferred, wherever he heard a difference, the
  chords nearer the melody: where 84 lifted the chords up toward the tune
  (gap about 17 semitones to about 5) the new side won 3 of 3, and where it
  dropped them away (about 5 to 17) the old side won 2 of 3 (the third,
  sung-hal, "a slight difference, but I might not"). Keep 84's rule (the
  octave read from the progression, so a pin can't flip it) and change its
  aim: the chords sit within about a fifth to an octave under the tune,
  never pushed down to about 17 semitones under it. A small listening check
  on more of 84's downward movers before it merges.
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
