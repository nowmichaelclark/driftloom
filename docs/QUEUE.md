# The queue

Written by the brain for Claude Code (the hands). Work top to bottom. Only
what is live is here: the status, the standing rules, the open items, the
known issues and the merge policy. Finished items, the old notes and the
Done list are in `docs/archive/` and `docs/DONE.md`; `docs/README.md` is the
map of all the docs. Mikey's ears come later, in listening albums the brain
builds from the Done list.

**Status, 2026-11-18:** `main` is at v188 (v188 the Android app with the screen off: a foreground service, the lock screen and notification with the loop's own cover, audio focus, saving through Android's picker, item 107d, #431; v187 links, an album from one seed and the plain lines, item 105c, #433; v186 share codes version 2 in the app, item 105b, #427, with a loop's own name in the code, #425; v185 the nightly green again, twang's seed renders and no join a hole on corpus piece #27, #430; v184 audio export, the small format (Opus in an Ogg file), item 104c, #424; v183 long pieces on screen, item 106b, #419; v182 the Android app on the native core, item 107c, #420; v181 audio export in the app, item 104b, #416; v180 the jump within a long piece, item 106a, #417; v179 twang soft only, #414; v178 codes version 2 in the core, item 105a, #412; v177 the chords nearer the tune, item 84b, #404; v176 the keepalive without its hiss, #403; session 01 done: v154 the tools
made steady, Diagnostics without the old scheduler's lines, Mikey's shorter
plain lines; v155 the width ceiling's rest fix; session 02 done: v156, item
101a, batch 6's far ends: deepDuck, wideSweep, slowOpening; session 03 done:
v157 tuningOffset and v159 padWow, item 101b, and v158 the nightly's
Emscripten step green again). Done: items 0-36 and 38-47 (13
stopped at its gate by design), the core's phase 1 (42-46c), the sound batch
(54-73), 74 and 76-76d, and **phase 2, the generator in Rust (items 77-92)**,
checked by the brain's Session 29 and heard by Mikey in its blind album
(2026-11-07). **Phase 3: item 93 done** (the player's module, 636 KB; the
brain accepted the size, `docs/archive/notes/93-module-size.md`) and **94a done**
(#337), **94b done** (#343, after Mikey's device check), 99 done (#341),
and **95** (the picks through the core's commands); Ears 13b approved
(2026-11-09). **The road to 1.0 (a long hands run, 2026-11-10; what waits
for Mikey is in `docs/notes/for-mikey.md`):** 103 done (the fast gate,
#354), 96 done (the checks on the player, #355), **97 done (the JavaScript
retired: one generator, one engine, the core's, #356)**, 100 done (the
too-slow line).
**Next: the sessions in `docs/sessions/README.md`** (Mikey, 2026-11-10: the
rest of the road cut into sessions, each with its prompt, its model and its
place in the order; the items below stay the briefs). Brain B1 done
(2026-11-11: 97 and 100 checked, phase 3's control rendered, the plan
reviewed); hands session 01 done (2026-11-11: the stall test clean, the
testers' steps written, the control's two loops explained and fixed); hands
session 02 done (2026-11-11: 101a, batch 6's far ends, #363); hands session
03 done (2026-11-12: 101b, tuningOffset #367 and padWow #369; the nightly's
stereo hashes #366 and Emscripten stack #368); hands session 11 done
(2026-11-12: 102a, v160, the long pieces' plan and its map, nothing heard
yet; the design that stands is `docs/LONGFORM.md`); hands session 12 done
(2026-11-12: 102b, v161, the wander heard, #375; holds kept within 300 s;
`tools/longform.mjs` checks what is played); hands session 13 done
(2026-11-12: 102c, v162, the arc's energy, an opening and a close heard,
#377); hands session 14 done (2026-11-13: 102d, v163, the arc's second
theme, its variations, its keys and movements of different kinds heard,
and the wander's theme turn on, #379; the brightness order's note
`docs/notes/102d-arc-brightness.md`); hands session 15 done (2026-11-13:
102e, v164, the joins, endings that land and the breath between arcs, #381;
the measures and the listening set, #382: 31 clips on the branch
`long-clips`); brain B3 done in part (2026-11-14: 102 checked on v164, three
bugs found for 102f, `docs/notes/102-b3-measured.md`; the "Long Pieces Album"
page out, `docs/ALBUMS.md`); Mikey heard it (2026-11-14: every piece kept,
`docs/notes/102-mikey.md`); hands session 16 done (2026-11-14: 102f, v165,
the two holes fixed and a yardstick against holes, nothing retuned; the
second listening set on `long-clips`); hands session 04 done (2026-11-14: 101c,
v166, widePad, both kinds, #387); hands session 19 done alongside
(2026-11-14: 104a, the FLAC writer in the core, #390; the app unchanged);
hands session 25 done alongside (2026-11-14: 107a, the Android project and its
CI build, `android/`, #389; no app change); hands session 05 done (2026-11-14: 101d, v167 tripletKit #391, v168
dottedPulse #392); hands session 16b done (2026-11-14: 102g, v169, no join
a hole on any piece, #395; the listening set on `long-clips`); hands session 06
done (2026-11-15: 101e, v170, colorTone, #396); hands session 07 done
(2026-11-16: 101f, v171, planingSevenths, #397); hands session 08 done
(2026-11-16: 101g, v172 lowBell #398, v173 dropSpotlight #399); hands
session 09 done (2026-11-16: 101h, v174 twang #400; the batch's locks
#401: the balance lock re-baselined, unchanged since session 07; then
Mikey's two calls, v175 #402: batch 6's draws count against a loop's
changes, way A of `docs/archive/notes/101h-change-counts.md`, and twang picks a
beat apart); hands session 10 done (2026-11-17: 84b, v177, the chords a
fifth under the tune where 84's whole octave left them past an octave, #404);
brain B2 done (2026-11-17: batch 6 and 84b checked on v177, the "Batch 6 Album" out, `docs/ALBUMS.md`; sessions 17 and 22's prompts adjusted); brain B-parallel (2026-11-17, in Claude Code): from 17 on the road runs as four chains, two sessions at a time (`docs/sessions/README.md`, "From 17 on"), started and overseen by that brain; Mikey heard the Batch 6 Album (2026-11-17: every sound kept, twang soft only, in session 17's first PR; 84b kept); hands session 22 done (2026-11-17: 105a, v178, codes version 2 in the core, #412; the app writes version 1 until session 23); hands session 26 done (2026-11-17: 107b, the core native on Android's own audio, AAudio from a Rust crate in `android/native`, a debug screen and tests, #411; no app change); hands session 17 done (2026-11-18: v179 twang soft only, Mikey's call, #414; 106a, v180, the core's jump within a long piece, where it is, and the piece's map for the page, #417; no screens yet, session 18's); hands session 20 done (2026-11-18: 104b, v181, audio export in the app, lossless, safe first, #416; the Small choice is session 21's); hands session 18 done (2026-11-18: 106b, v183, long pieces on screen: where you are, the piece's line and jumps, the journey pick, the Library's lengths, #419); hands session 27 done (2026-11-18: 107c, v182, the Android app plays on the native core, the page driving it by the worklet's messages, #420; the screen off and the notification are session 28's); Mikey's seven for 1.0 queued (2026-11-18: items 109 to 114, sessions 30a to 30f, before the polish); hands session 21 done (2026-11-18: 104c, v184, the Small choice in the export sheet: Opus through the browser's own encoder, an Ogg file the page muxes itself, `js/ogg.js`; hidden where the browser has no Opus encoder; #424); hands session 24 done (2026-11-18: 105c, v187, a link is the address with `#c=` and the code (no server sees it; offline works), a link preview, an album from one seed in Save, Share, the plain lines; #433); hands session 23 done (2026-11-18: 105b, a loop's own name travels with its code in any script, #425; v186, the app speaks version 2 only, the Library untouched, #427; v185, the nightly green again: #414's twang seed renders, and #402's three holes on corpus piece #27 fixed by a long note's life past 4 s, #430); hands session 28 done (2026-11-18: 107d, v188, the Android app plays on with the screen off: a media-playback foreground service with its notification and MediaSession carrying the loop's own cover, audio focus (a call pauses and returns), the headphones pulled, a wake lock only while playing or saving, nothing held paused; exports and MIDI and the backup saved through Android's own picker as they are made, the restore file's picker; the page kept running while it plays or saves; #431); next: the chains, and his testers' pastes
(`docs/notes/98-testers.md`). Item
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
  moved and why.
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
lookahead and the JavaScript synth and generator retired in item 97.
*Done when:* promise 1's stall test is clean and the testers' phones
report zero late notes. The notes under "For phase 3" below are part of
every item here; item 53's brief (below them) is item 95's.

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

## 100. When a browser is too slow even on lite (done, hands, 2026-11-10)

**Why:** a friend's Pixel 9 on GrapheneOS played "too fast and really
skippy" (v151): its browser runs WebAssembly on an interpreter unless JIT
is allowed for the site (54.7% of real time on lite interpreted, 9.3% with
JIT). **Mikey (2026-11-10):** no browser workarounds and no cushion or
lower tier; a plain line telling them to get the Driftloom app.

**Built (v153):** on lite, however it was chosen, when 8 of the
quarter-seconds in the last 20 s of playing each took longer than the
audio it made, or the device filled in 8 times in them where the browser
counts (`playoutStats`), the card's place says "This browser is too slow."
(Mikey, 2026-11-11; it first read "...too slow to play Driftloom smoothly",
changed in v154) (`quality.stuck`, js/quality.js, which says
how the line was set from the tablet's and the Pixel's figures). "Get the
Driftloom app." follows as a link once `APP_LINK` in js/main.js is set
(item 107); Play goes on, and Diagnostics reads `slow: lite past the line,
Ns (why)`. `tools/autolite.mjs` has the `stuck` case, and its other cases
check the line never shows.

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

## 97. The JavaScript retires (done, hands, 2026-11-10)

**Built (v152):** `js/synth.js`, `js/engine.js`, `js/clock.js`,
`js/generator.js`, `js/draw.js`, `js/dlmath.js` and `js/dlcore.wasm` are
gone, with `?engine=js` and `?engine=page`. The page reads a loop's record
and notes from one more instance of `js/dlplayer.wasm` (`js/gen.js`; the
request and the record are `js/request.js`), and `js/output.js` takes the
core's node to the speakers. A browser that can't run the core shows the
plain line in the card's place; a module that never arrives says
"Please refresh." (Mikey, 2026-11-11; v154, it first read "Driftloom
couldn't load. Check the connection and reload.") The page's
JavaScript went from 19,538 lines (835 KB) to 9,166 (363 KB), and 252 KB of
`.wasm` less. The tools read what the generator decides from the tools'
build (`tools/gen-lib.mjs`, `tools/build-core.sh --tools`): the generation
goldens (all 5,124 cases, `gen-golden.mjs --check`, the file now the list
of cases), the balance lock (`stats.mjs --check`), the dials, the octave
check. `tools/rows.mjs` gained the plain line's two rows.

## 98. The phase's proof

**Hands:** Sonnet 5.5, high; then Mikey and the testers.

**What:** promise 1's stall test, clean, on the shipped app; the testers'
phones (Mikey's Moto and tablet, and the friends' Pixels on GrapheneOS)
report zero late notes in Diagnostics over a listening session each, with
a few Nexts, a long loop, Endless and the screen off. Write the steps for
the testers in plain words, one paste each.

**Then the brain:** its phase check, and a control album (today's app
against the app before phase 3, the same codes): it should sound the same.
**Brain B1 (2026-11-11):** on `main` at v153 (d9d6ffe): `build-core.sh
--check`, the generation goldens (5,124 of 5,124, the Rust generator alone),
the balance lock (22 of 22), the Node tests, `tools/rows.mjs` (29 of 29), the core baseline (nothing
moved) and CI's gate all pass; the page's JavaScript 835,298 bytes (31 files) before
97, 362,742 (28) after, 125,911 gzipped against 272,991. The control: 30
corpus loops rendered by v149's own Engine and Synth on the core and by
today's player: names and starts the same, 28 null at -82 dB or lower, two
differ after a silent bar at the same loudness (session 01 explains them).
The album, "Phase 3 Control", is out (`docs/ALBUMS.md`).
**Hands session 01 (2026-11-11):** promise 1's stall test on the shipped app
(v154; `tools/stalls.mjs`: the main thread blocked 2 s and 5 s, the page
frozen, the tab hidden a minute; shatter, the 32,768-bar piece and Endless;
4x and 6x CPU; full and lite) reads zero late, left-out and dropped notes in
all 84 rows, the audio thread's busiest quarter-second at most 21% (Chromium
153 has no `playoutStats`, so that is the figure for underruns; the PR has
the table). The testers' steps: `docs/notes/98-testers.md`. The two loops
the control found different (tui-doask, soath-wa) are the width ceiling's
(`core/src/blocks/width.rs`): in a long rest its mid window rounds to
exactly zero beside a side window of float dust, it reads "far too wide" and
takes the side's gain toward nothing, and the loop returns narrow for about
ten seconds. It does that in v149 and in today's player alike (the same
samples at the player's 0.25 s hand-over); the brain's v149 renders handed
notes over far ahead, where the dust fell the other way. Fixed in v155: the
ceiling takes a window with next to no mid in it (under 1e-9 in sum of
squares, about -140 dB a sample) as nothing to hold the side against, so its
gain goes back up; the two loops now render within -88 dB of each other at
either hand-over, and one baseline loop (ri-houng, full, stereo) moved.

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

Items 106 and 104 are done (sessions 17 to 21, 2026-11-18): `docs/archive/queue-items.md`.

**The app's 1.0 comes before the game** (Mikey, 2026-11-10): Broken Friends
and the other games wait (the core already works in Broken Friends); the
music app is finished first, and **1.0 includes the Android app on
F-Droid** (Mikey, 2026-11-10). Order, after 103: phase 3's 96, 97, 100 and
98's hands part; then 101, 84b and 102; then 106, 104 and 105; then 107;
then Mikey's seven (109 to 114, 2026-11-18); then the known issues, the polish, and 108. **Screens:** the hands build
plain ones in the app's current style, keeping `docs/PAINTER.md`'s rules
(one screen, no new settings, scrolling in its two places); a Painter
session may restyle any of them later (Mikey, 2026-11-10). Each sound change merges under the policy and is
gathered into one listening album for Mikey; what needs his ears or a
decision goes in `docs/notes/for-mikey.md` and the work moves on.

**In sessions** (`docs/sessions/README.md`, 2026-11-10, reviewed by brain B1
2026-11-11): 98 is session 01;
101 is 101a to 101h (sessions 02 to 09: the far ends, pitch, width, the
kit, colorTone, planingSevenths, lowBell with dropSpotlight, twang and the
batch's locks); 84b is session 10; 102 is 102a to 102f (sessions 11 to 16:
the plan and its map, the wander, the arc's energy, its themes and keys,
the joins and the listening set, then tuning from Mikey's notes;
`docs/sessions/long-pieces-brief.md`); 106 is 106a and 106b (17, 18); 104
is 104a to 104c (19 to 21); 105 is 105a to 105c (22 to 24); 107 is 107a to
107e (25 to 29); 109 to 114 are 30a to 30f (Mikey's seven, 2026-11-18); the polish is 30, the new name is 31a (Mikey, 2026-11-11: the app is
renamed at 1.0, kept from the testers until then) and 108 is 31. Five brain sessions (B1 to
B5) check the work between them.

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

**Added by brain B1 (2026-11-11), from the code** (the brief has each):
the journey plays at any wander setting (a section is today's pass at the
slider's amount; the slider becomes only its breathing); pins, mutes and a
code's edits are never moved; the plan's spans in time, in phrases where
sections are few; endings that land when the length slider stops a piece
before its close; movements (a finite piece's last arc looks back);
theme B's making spread across blocks (the tablet's margin); and the
listening set as rendered excerpts with a blind before-and-after pair,
since the app can't jump or take a journey until 106.

**102f** (session 16; brain B3, 2026-11-14): Mikey kept every piece and
every first pick, so nothing is retuned. Two holes B3 measured on v164 are
fixed: a pivot struck at velocity 0 in 1 loop in 12 (its velocity from a
silent entry chord), and "the tune alone" over a varied tune that is silent
there (three seconds near silence in fei-no); and a yardstick holds a join
from silence. The developments' frequent breaths were heard and kept.
`docs/notes/102-mikey.md`, `docs/notes/102-b3-measured.md`. **Done**
(session 16, v165): the pivot struck at the landing's velocity, "the tune
alone" only over a tune that sounds there as the unit plays it (else a
breath), and the yardstick: `tools/longform.mjs` holds every arc's join,
as played, from a hole (25 dB under the loop as itself, and made by the
join), and `core/tests/joins.rs` every pivot struck and every tune alone
sounding.

**102g** (session 16b; brain B3, 2026-11-14): no join a hole on any piece.
Session 05 found three on Endless arcs that session 16's yardstick never
plays (ta-muin 1337654778 at 2:05 and 9:26, "the tune alone" 19 to 38 dB
under the loop for about 2 s; yi-yoar 819045617 at 6:14, a floor out 33 to
40 dB under), the same since v164: a join whose kept layers carry little of
the loop. A join that would be a hole falls back (decided from the unit's
notes, at a flat cost); every other join stays as Mikey kept it; the
yardstick plays at least 200 pieces' joins, Endless arcs among them.
**Done** (session 16b, v169): a tune alone, a floor out or a breath is read
from its notes when its unit is made (`Heard::under`: each voice's loudness
over a note's life, `[longform loudness]`, `tools/voice-loudness.mjs`; the
echoes as the mix sends them) and falls back to a breath, or to a glide,
where it would leave the mix 12 dB under (`[longform joins] hole`); over
200 pieces rendered, 88 holes before and none after (but three half-beat
breaths, the kind Mikey kept), and 704 of 29,729 written joins over the
corpus's maps changed, most of them breaths become glides. The wide run is the nightly's
(`tools/longform-holes.mjs`), the three places in `tools/longform.mjs`.

**102e done** (session 15, v164; #381, #382): every line where a unit meets
the next and something changes is written, chosen by what changes and the
energy either side (`longform::Join`, `Form::join_of`; heard in
`Heard::joins`): a chord both keys share held across a key's line, the
bass on its root; a fill into a rise; the floor out before a fall; the
tune alone, or a breath, where a theme changes; a wander's line move a
breath of its layer; what glides, its glide. A breath between arcs thins
the tune across it instead of resting it (the 2.7 LU step gone). Endings that land: where
the length slider stops a piece or Endless before its close, the phrases
before the stop play the close and land on the tonic, ringing out
(`Heard::stop_at`; `dlrender --track`). A landing struck at velocity 0
since 102c fixed. The arc's brightness judged on its opening, peak and
close (`docs/LONGFORM.md`, "The joins", "Endings that land";
`core/tests/joins.rs`). Measured over 240 seeds at 64, 256, 1,024 and
32,768 bars and Endless (#382's table): every check but one, a lite pass
block a microsecond over its lightest audio block on one seed, as on `main`;
promise 1's stall test clean on an arc of 32,768 bars and on Endless (120
rows). The listening set: 31 clips, 23 minutes, two blind pairs (before and
after item 102), on the throwaway branch `long-clips`
(`tools/longform-clips.mjs`; `docs/notes/for-mikey.md`).

**102d done** (session 14, v163): a second theme (another seed's tune bent
into the loop, made a stage a block a section ahead), the variations as
marks on the notes, the related keys (a fifth away, the relative and the
parallel along the mode), the return prepared on the home fifth, call and
response in the development, movements of four kinds in a cycle, a finale
that looks back, and the wander's hand-over of the tune
(`heard::Lift::themed`, `Generator::prepare`, `longform::Kind`;
`docs/LONGFORM.md`, "How an arc's themes are heard";
`node tools/longform-themes.mjs`, `core/tests/themes.rs`).

**102c done** (session 13, v162): the arc's energy heard, and a finite
arc's landing on the tonic (`heard::Lift`; `docs/LONGFORM.md`, "How an arc
is heard").

**102b done** (session 12, v161): the wander heard (`core/src/generator/heard.rs`;
`docs/LONGFORM.md`, "How the wander is heard").

**102a done** (session 11): the plan as code (`core/src/generator/longform.rs`,
`data/longform.txt`), `journey=arc` and `journey=wander`, `dlgen --map`,
`tools/longform.mjs` and `core/tests/longform.rs`. The design that stands
is `docs/LONGFORM.md`; sessions 12 to 16 build from it.

**Yardsticks:** a piece map (`dlgen --map SEED REQUEST`: each section's key,
tempo, energy, theme and journey) for any length; corpus checks that an
arc's themes return and its energy rises and falls, that a wander moves at
most one thing a section, and that a section's cost doesn't grow with its
number; then, for Mikey, a listening set: a few pieces each at 64, 256 and
1,024 bars and Endless, as album codes he plays in the app, with short
excerpts (the start, a quarter, half, three quarters, the close) so a piece
can be judged in minutes.

## 105. Short codes and links (codes, version 2)

**Hands:** Opus or Fable, high. Before 1.0, after 102, so it
carries everything 1.0 has (B-parallel, 2026-11-17: not after 104 or
106, since a code carries no export and no jump); **from 1.0 on, codes stay stable** (Mikey,
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

**Hands:** Fable, high. Alongside 104 to 106 (B-parallel, 2026-11-17:
the chains in `docs/sessions/README.md`; it reads no codes). Mikey (2026-10-12): from the same Rust
core, with the web screens inside it; the website stays. The core native,
on Android's own audio (AAudio, through a small Rust layer or Oboe) at a
high priority, the page's screens in a WebView talking to it by the same
commands and events as the worklet (94a); no Google services, nothing
F-Droid's rules refuse. Built in CI (GitHub's runners have the Android
SDK), an APK Mikey can install, `fastlane` metadata, and an F-Droid build
recipe with reproducible-build notes. **For Mikey:** the submission to
F-Droid (his account, after 1.0: the merge request is public and carries
the new name) and the signing choice. Test builds use a working id; the
release id and name come with the rename (31a). Then item 100's link points
at the listing. **Yardsticks:** the Moto and the tablet play with nothing
late (Diagnostics in the app), the screen off for ten minutes, a long
piece and an export; the APK builds from a clean checkout.

## 109 to 114. Mikey's seven, before the polish (2026-11-18)

Mikey's list for 1.0, in his words in each session's prompt
(`docs/sessions/30a-album-previous.md` to `30f-painter-final.md`); the order
and what may run together are `docs/sessions/README.md`, "Mikey's seven".

- **109. Previous works in an album** (his 2; session 30a). Previous does
  nothing in an album. Likely cause, to confirm: Previous is disabled at the
  history's start, and an album's tracks don't enter the history. In an
  album Previous goes to the track before, from every button (screen,
  keyboard, headset, lock screen, Android's notification). **Yardsticks:**
  rows for it, ten passes.
- **110. The layers show what is heard** (his 5; session 30b). The layers
  draw each pass as composed, so a note left out, faded or in a quiet
  section is drawn, and a layer fading in may be heard before it is drawn.
  The core reports each layer's notes as heard (after the section's level,
  the fades and the refusals); the page draws them when and as loud as
  heard; the cursor's latency checked on the web and on Android. Nothing
  heard moves. **Yardsticks:** a tool measuring drawn against heard, before
  and after, near zero after; the baseline unchanged; the tablet's margin.
- **111. Track length, its own view, to the second** (his 6; session 30c).
  A view like Bars, its key beside "any tempo" and "any bars"; the time to
  the second; the core's track a time, its close landing on that second;
  a fade in and a fade out, each on or off; the Sound view's Length slider
  and the export's "How much" gone (an export is the track length; with none
  set it opens the view first). **Yardsticks:** a track ends on its second at any
  tempo, on a loop, a piece and Endless; the export that long.
- **112. The info card** (his 3 and 4; session 30d). Every detail line
  shown is whole on any display; what doesn't fit is left off, and a tap
  shows the rest. Its playhead round, like the layers' dots, a little bigger.
  **Yardsticks:** no line clipped from 320 px to a tablet and at 200% text.
- **113. Fewer words** (his 1; session 30e). Icons, switches and two- or
  three-word phrases for explaining sentences, across the app; the share
  screen as keys (copy code, paste code, export audio), export audio a view
  of a key a format (flac, midi, the small one). Screen readers keep full
  names. **Yardsticks:** a before and after table for Mikey.
- **114. The Painter's final touches** (his 7; session 30f). The design once
  more as one app, and accessibility to WCAG 2.2 AA (contrast in every
  theme, focus, targets, names, reduced motion, 200% text). **Yardsticks:**
  a check in `tools/` on every theme; screenshots before and after.

## 108. 1.0

**Hands:** then Mikey. First the new name (session 31a, built and left open
until this moment, since a merge deploys). The version named 1.0 in the app and the APK, a short
changelog, codes frozen (105), the nightly's checks back as merge gates
(103), and `docs/notes/for-mikey.md` empty or answered.

**Also noted, not on the road to 1.0:**

- **The player's stack** (item 94a, PR #337; the generator's stages apart
  since 101a's frame passed it): its calls take 45 to 47 KB of the caller's
  stack (Emscripten, measured), under its 64 KB default; the header says
  `-sSTACK_SIZE`. Fine for the
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

Each sound's share, vibe by vibe (Mikey, 2026-11-17, after the Batch 6
Album): how often a sound plays is tuned mood by mood across the vibes, some
leaning into it (the soft twang, say) and some away, and the real share of
loops follows from that; not before the vibes are looked at one by one.
84b's 88 far-off held chords: maybe a later item, maybe never ("having
differing kinds of music is okay"); a flip-free way to read their octave if
it is ever wanted.

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

The melody twang (Mikey, 2026-11-11: after 1.0): batch 6's twang voice
playing the tune, beside session 09's chord twang.

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
- (Mikey, 2026-11-17, session 10) The gate is read from CI: green `core`
  and `docs` workflows on the PR's last commit are the gate, and
  `tools/gate.sh` locally is not required (CI runs the same script, side
  by side, in about five minutes); a session runs a part of it locally only
  to find a failure. The long pieces' full check is read from CI too: on a
  PR that moves what a loop plays, the `longform` workflow
  (`.github/workflows/longform.yml`, `tools/longform.mjs` in shards) green
  on the last commit, its table job saying it kept every check main kept.
  Heavy checks never run side by side on Mikey's machine (about 7 GB).
- (Mikey, 2026-11-18) The `export` workflow is not part of the gate, but the
  hands usually wait for it to pass before they merge. They can merge
  without it when the PR is docs-only or a minor change. Say in the PR
  when you do.
- (Mikey, 2026-11-18) The brain may merge any hands PR, not only a
  docs-only one, once CI is green on its last commit and it checks out
  against its prompt.
