# The queue

Written by the brain for Claude Code (the hands). Work top to bottom. Only
what is live is here: the status, the standing rules, the open items, the
known issues and the merge policy. Finished items, the old notes and the
Done list are in `docs/archive/` and `docs/DONE.md`; `docs/README.md` is the
map of all the docs. Mikey's ears come later, in listening albums the brain
builds from the Done list.

**Status, 2026-11-01:** `main` is at v137. Done: items 0-36 and 38-47 (13
stopped at its gate by design), the core's phase 1 (42-46c), the sound batch
(54-73), 74 and 76-76d, and phase 2 so far: **the port, items 77-82**
(#287-#296; checked by the brain's Session 28), **83** (ears on the core),
**84** (the chords' octave) and **85** (the request: 85a #302, 85c #303, 85b
#304, 85d #305; checked by the brain's Session 29). **Next: item 86, then 85e** (the steal at full quality) **and 85f** (a theme's rest kept), then 87. Item
75 is low priority. 37 stays on hold. A new Claude Code session starts here: read the standing
rules and the merge policy, then take the next item.

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

## Phase 2: the generator in Rust (items 77-92)

Written by the brain, Session 26 (2026-10-26). They replace the placeholders
48-52, which are folded in below (48 → 85, 49 → 88, 50-52 → 89-91); 53 stays
in phase 3. **Read first:** `docs/CORE.md` (promise 11, the request; promise
10, Broken Friends; phase 2) and the brain's log entry "what Broken Friends
needs from the core" (`docs/archive/brain-log.md`, 2026-10-25).

**What the phase is for.** Broken Friends waits on it: today the game can
only play golden note lists. What the game asks of the core, in its order,
sets the order here:
1. **a loop from a seed** (each friend's theme), with no JavaScript at all
   (items 77-82);
2. **steering** by friend, mood, time of day and camp events, which are
   promise 11's requests (items 85-86);
3. **a cheap change of loop** at a musical moment, with a crossfade, in place
   of today's second core (item 86);
4. **rendering ahead for the browser**, where Godot's web build has no audio
   thread (item 87).
Then the rest of the request (length, key and scale, style, voices: 88-91)
and the wander's upper range (92). Two fixes ride in the phase, as the
charter says: the chords' octave (84) and the wander (92).

**How the port is proven: note for note** (Mikey, 2026-10-26). The generator draws only from
its own seeded `Rng` and does plain arithmetic, apart from about twenty
calls to `Math.pow`, `log`, `sin` and the like, whose last bits are each
browser's own. Item 77 moves those onto the core's math (one JavaScript
mirror of it), and records **generation goldens**: for a fixed set of seeds,
vibes and passes, a hash of everything the JavaScript decides (the spec,
every layer's events, the choices, each wandered pass, every setting the
engine sends at load). The Rust port is then held to those hashes, step by
step, the way the synth's port was held to Chromium's samples: equal, or a
named, explained difference. That is stricter than the charter's gate (the
balance lock and `stats.mjs` within tolerances, and a blind album), which
still applies at the end.

**The sequencer joins phase 2** (Mikey, 2026-10-26; `docs/CORE.md`, phase 2): the game's gate needs
the loop played from a seed, so phase 2 also takes the **sequencer**, what
`js/engine.js` and `js/synth.js` do between the generator's events and the
core's notes (item 82). Phase 3 still moves the app onto it and retires the
JavaScript; it just has less to build.

**Rules for the phase** (on top of the standing rules and `core/AGENTS.md`):
- **One source of truth at a time.** Until item 82's player is in the app
  (phase 3), the JavaScript generator plays; the Rust one is proven against
  it. Generator and vibe changes made during the port go in **both**, or wait
  (`docs/BRAIN.md`, "Two tracks"); after item 78 the data (profiles, vibes,
  voices) is one file both read, so tuning a vibe never waits.
- **When nothing is picked, nothing changes** (48's rule, kept): a loop with
  nothing pinned renders as before; a pick may only change loops that asked
  for it, and chance draws from the seed with no extra draws that shift the
  rest.
- **Requests bend the music, never break it** (promise 11); "can't do that"
  only when truly impossible, in a short plain ASCII line.
- **Picks shape loops to come, never the one playing,** and hold through
  re-rolls and wandering; in an album the album decides.
- **Measure that a pick is honored in the render**, not only in the spec
  (tempo from onsets, bars from the cycle, scale from pitch classes, voices
  from what sounds).
- **The core's rules hold for the generator too:** no allocation after
  `Core::new` (the generator's working memory sized there, and said in
  bytes), no panic, no `dyn` on the hot path, plain ASCII out, `no_std`,
  bit-identical on every target (goldens for generation on all five, as for
  audio), pedantic clippy clean in every file touched (CI holds it at zero since item 74).
- **Spread the work:** generating a loop never spikes one audio block
  (promise 1); say the worst block's cost on the slowest target.
- **Taste:** the conservative option, "for Mikey's ears", as always.

Hands, by item, as the brain suggests: Opus 5.5, high, for 77, 79-82, 85-88
and 92; Sonnet 5.5, high, for 74, 78, 83, 84, 89-91.

**Since item 82 (brain, Session 28, 2026-10-31).** The port is done and proven
(items 77-82, archived): `dlcore::generator::Generator` makes every decision
the JavaScript makes, held to the 3,060 generation goldens on all five
targets, and `dlcore::player::Player` plays a loop from a seed as the app's
engine does. What that means for the items below:
- **Every generator change is now made twice,** in `js/` and in
  `core/src/generator/`, in the same PR, and the goldens hold the two
  together. A data edit (`data/*.txt`) reaches both by itself, but moves the
  hashes and the player's build: rewrite `core/golden/generation.hashes`
  (`node tools/gen-golden.mjs`) and rebuild the modules
  (`tools/build-core.sh`; `js/dlplayer.wasm` compiles the data in), both in
  the same PR, or CI fails.
- **Build on the player:** 85-92 extend `Player` and its `Request`
  (since 85a: vibes with weights and the tempo; `core/src/generator/
  request.rs`), and reach the C
  header and `dlrender --seed` with them.
- **The player's renders are the reference.** It hands notes to the core
  before each block, everything due within `player::LOOKAHEAD`; the measure
  harness (`tools/measure.mjs`) hands a whole render over up front, and the
  core then sums the same notes in another order (differences 77-105 dB under
  the music; `docs/archive/notes/82-hand-over.md`). Compare player renders with
  player renders, or the browser's reference made the same way
  (`tools/player.mjs`), not with the harness's.

## 86. Changing loops at a musical moment, with a crossfade

**Why:** Broken Friends' music changes at moments, not on a timer (its
decision 0038: a new room, a friend made well, a mood, surgery). Today a new
note list resets a core (about 15 ms on a laptop, more on a phone), too slow
for the audio thread, so the game keeps two cores and switches. The app's
Next would use the same (phase 3).

**What:**
- `Core::change(request, at)`: the next loop starts at a boundary the host
  names (now, the next beat, the next bar, the end of the phrase, the end
  of the pass), and the old one hands over by a **crossfade** (its
  length a setting, default the conservative choice, "for Mikey's ears")
  or by letting its notes ring out, no reset and no second core. Held
  instruments (the organ, the wash, padSweep, the arc) hand over as item
  72's padSweep does across a load (glide, never jump).
- An **event** is a request attached to a moment (promise 11: "when the sun
  rises"); several may wait, the latest for a boundary wins.
- The cost of a change spread across blocks: no block over 25% of its
  deadline on the slowest target while a change happens.

**Yardsticks:** a change costs no allocation and no block past the
budget; across 1,000 changes at every boundary kind no click (the sample
step at the join no larger than the loops' own), the new loop's samples
after the crossfade equal the same loop loaded alone (from the boundary
on, given the same start state of the held instruments: say what differs);
the game's two-core switch can go (the note for Broken Friends says how).

**Since item 82:** `Player::load` already changes loops with no reset and no
second core (the new loop made a stage a block, the old one's notes playing
out, then the new one starts). This item adds the moment and the crossfade to
it: `Player::change(request, at)` (or `load` gaining `at`), not a call on
`Core`.

## 85e. The budget takes room from old notes at full quality too

**Hands:** Opus 5.5, high. After 86, before 87 (brain, 2026-11-01; Mikey
heard 85c's lite album, "it sounded great for lite mode", and asked for the
same at full quality: `docs/DECISIONS.md`, 2026-11-01).

**Why:** at full quality the core still turns away 986 of 5,056 chord notes
(19.5%) over the baseline's 100 loops, melody 13 and texture 59: a new chord
loses notes while the old one's tails finish. Letting old notes go is how
synths and samplers handle a full pool; refusing the new note is the odd
choice. Full quality's room is larger, so the notes let go there are mostly
ones already ending, which makes the order of what goes first matter more.

**What:**
- `STEAL_FULL` on (and `_steal` in `js/synth.js` the same), so full quality
  takes room from old notes as lite does.
- **What goes first, in both qualities:** a call whose notes are all past
  their end (in the budget's `TAIL`, already releasing) before one still
  held, oldest first within each. That is the field's usual order (JUCE's
  `findVoiceToSteal`, FluidSynth's: released voices before held ones). If it
  changes lite's renders, say how much and in which loops; lite was heard and
  liked, so lite stays as 85c built it if the change there is more than
  small, and the order applies at full only ("for Mikey's ears").
- **Never let go** of the held instruments (the organ, the wash, padSweep's
  bed, the drone): say whether any of them sits in the budget today, and
  exempt it if so.
- `STEAL_FADE` stays 0.05 s, ending as the new note starts.

**Yardsticks:** per-layer refusals at full over the baseline's 100 loops
before and after (chords well under today's 19.5%, melody 0), and shatter at
a `bpm=180` pin at full; lite unchanged, or the change said and measured;
no clicks at a let-go note's fade (the sample step at the fade no larger
than the loop's own, full and lite); the worst block's cost no higher than
today on the slowest target; the core baseline updated on purpose; the
player against the browser bit for bit; the JavaScript fallback's own checks
(`--engine js`, `--refusals`, `--clicks`, `--endings`) if its code is touched.
Then a short labeled album at full for Mikey (a busy chords loop, a slow
bell or pad loop where long tails are the point, shatter), before and after.

## 85f. A theme's rest is kept, choir or not

**Hands:** Sonnet 5.5, high. After 85e, before 87 (Mikey, 2026-11-01:
"keep a friend's loop tuneless if it has none"; `docs/DECISIONS.md`).

**Why:** item 85d keeps a theme's tune, "or its rest". But a choir never
takes the rest, so when a theme's own loop has no melody (about 8% of loops)
and the vibes asked bring a choir, the bent loop's choir sings a figure of
its own: a friend with no tune suddenly has one. The brain found it in 2 of
600 random theme requests (e.g. theme 3570153259, do-vom, no melody, asked
with `booming=0.31,cheeky=0.26,experimental` from seed 2428270433: dail-vu,
a vowel choir, 87 notes).

**What:** in both generators (`genMelody` in `js/generator.js` and its port
in `core/src/generator/`), a theme with no figure gives no melody, choir or
not; every draw is still made, so nothing else moves. Say what the choir's
other parts do in such a loop (the keys' detune it shares with the melody):
the conservative choice is that the loop simply isn't a choir loop, "for
Mikey's ears".

**Yardsticks:** the generation goldens: only `theme` cases may move, and only
those whose theme rests and whose loop drew a choir (say which and how many,
and that every other case is unchanged); the brain's two cases above now have
no melody; nothing asked changes nothing (the 3,060 corpus cases, the
baseline and the balance lock unchanged); the Rust and JavaScript generators
equal on all cases.

## 87. Rendering ahead, for the browser

**Why:** Godot's web build has no audio thread; music fed frame by frame
crackled on Mikey's phone, so the game renders a loop whole and plays it as
one sound. With wandering and endless loops it needs passes rendered ahead,
quickly, on a phone's browser: the Moto G Power (2022) and weaker, on the
lite mix.

**What:**
- Render ahead in chunks a host asks for (a pass, or N seconds), resumable:
  the next chunk continues exactly where the last stopped (wander, endless,
  a change waiting at a boundary), the same samples as rendering straight
  through.
- A host can ask how long a chunk will take to render (the core's own
  estimate from the loop's voices) and spread it across frames.
- **The Moto G Power (2022) joins promise 2's measured devices**, with the
  speed target the brain proposes once measured (the game needs a pass
  rendered well inside its playing time, lite, stereo 1.2-1.4 times mono).
- An Emscripten build of the core in CI (Broken Friends' web build links it
  with panic=abort), rendering the goldens to the same hashes.

**Yardsticks:** chunked equals straight through, sample for sample, over
every chunk size from 128 frames to a pass; render speed on the web host
(wasm in Node and in headless Chromium) reported per loop, lite and full,
mono and stereo, as times real time; the Emscripten goldens equal.

## 88. Length, up to 32,768 bars and Endless (was 49)

49's text (below) is the brief, built on 82's player and 87's chunks. Plus:
Endless and long pieces render ahead as 87 says.

## 89-91. Key and scale (was 50), style (was 51), voices and layers (was 52)

50, 51 and 52's texts (below) are the briefs, in that order, each on 85's
request. Sonnet 5.5, high, each.

## 92. "Let the loop wander", unrepeatable in the slider's upper range

**Why:** the known issue (below, "Known issues"; Mikey, 2026-10-18): all the
way up, wandering should feel experimental, not repeatable.

**What:** as the known issue sketches: the core's draws stay a function of
what the host sends; the host adds a per-play salt to the wander's seed
across the slider's upper range (where it starts is a named setting, first
at the conservative point, "for Mikey's ears"); a share code or export says
which it was. Item 54's choices stay the loop's own.

**Yardsticks:** the same code played twice in the upper range wanders
differently (passes 2-8 differ), below it identically (bit for bit); the
generation goldens (wander at the default) unchanged.

## The phase's end (brain)

When 92 is merged: the brain's check (generation goldens on every target,
the player against the browser, costs, the phone figures), and the
charter's blind album: the old generator against the new on the same seeds,
with the octave's movers in it. Then phase 3.

**From item 84 (brain, 2026-10-31):** the album also takes a few loops whose
keys and melody now sound under 5 semitones apart (the balance lock's "not a
choir, under 5" rose from 2.34% to 3.73% when the octave stopped being read
from what sounds). If Mikey hears them crowd the tune, the fix is a wider
margin in the progression's decision, not a return to reading the events.

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
- **`js/dlplayer.wasm` is 1.8 MB** against the app's 249 KB, because
  `Player::new()` is not all zeros and its 1.5 MB is stored in the module.
  Make it zeros, as `Core::new()` is, before the app ships it (first-visit
  weight on phones).
- **Phones:** the player's costs were measured on a cloud machine; a block
  while a loop is made is 9.6% of its deadline in wasm. Check a phone
  (`?engine=player`, Diagnostics: late and dropped notes) before the app
  switches.

## Next picks, wired for real (items 48-53): the briefs folded into phase 2

**Folded (brain, Session 26):** 48 is item 85, 49 is 88, 50-52 are 89-91, built on
phase 2's request; 53 stays in phase 3. Their texts stay here as those items' briefs
until they merge (48's, done as item 85, is in `docs/archive/queue-items.md`).


**Read first:** `design/next-picks.md` (the design Mikey chose on
2026-10-13, Painter session 5) and `docs/CORE.md`, promise 11 (the
request). The design shows five lines of picks for new loops: moods
(today's vibes), tempo and length, key and scale, style (profile) and
voices. Each is **pinned**, **ranged** or left to **any** (chance).

**What already exists (app track, Painter 6):** the screens, with
moods wired to vibes, and length and tempo wired to today's JavaScript
generator only where cheap, clamped to what it can play (bars from the
profile tables, 2 to 32; tempo 32 to 190 bpm). Tempo, key, style and
voices can't be tapped until their pickers are designed. These items
replace that plumbing with the real thing, in the core.

**Where they sit in the phases.** They are not a phase of their own:
items 48-52 belong to **phase 2** (the generator in Rust, built around
the request) and item 53 to **phase 3** (off the page). The brain's
review orders them among phase 2's own items when it writes them
(Session 26, after the sound batch; moved from Session 24); 48
goes first, since the others stand on it. Don't start them before phase
2 starts: the generator they change is the Rust one.

**Rules for these items** (on top of the standing rules and the core
rules in `core/AGENTS.md`):
- **When nothing is picked, nothing changes.** Every loop with all five
  lines on "any" renders exactly as it did before the item (goldens,
  `tools/baseline.mjs`, the balance lock). A pick may only change loops
  that asked for it. Leaving a choice to chance draws from the seed as
  today, with no extra draws that shift the rest.
- **Requests bend the music, never break it** (promise 11). A pin that
  pulls against a profile's character finds its in-between, as vibes do.
  Only a request that truly can't be met says so, in a short plain line
  (the "can't do that" message; its look is a Painter question, until
  then plain text in the panel).
- **Picks shape loops still to come, never the one playing**, and they
  hold through re-rolls and "Let the loop wander": every new loop and
  every wandered pass honors them. In an album, the album decides and
  the picks wait (the design hides them there).
- **Measure that a pick is honored in the render**, not only in the
  spec: tempo from the note onsets, bar count from the cycle, scale and
  key from the pitch classes, voices and layers from what sounds. A test
  walks every pickable choice (promise 11's measure).
- **A picker the design doesn't have yet** (tempo, key and scale, style,
  voices, as of 2026-10-13): build the request side and its tests; the
  app's line stays untappable until a Painter session designs the
  picker. Don't invent the UI.
- **Taste:** where an item leaves a musical choice open, take the
  conservative one and mark it "for Mikey's ears", as always.

Hands: chosen at the phase review. The brain's suggestion: 48 and 49
Opus 5.5, high (the request's shape; sectioned generation); 50-52
Sonnet 5.5, high (clear specs on 48's pattern); 53 Opus 5.5, high
(playback, phase 3).

## 49. Length, up to 32,768 bars and Endless (phase 2)

**Why:** Mikey, 2026-10-13: far longer loops than today's 32 bars, the
app's picker to 32,768 bars plus Endless; no cap in the core (charter,
promise 11). The design's Length picker already exists (Painter 6).

**What:**
- **Pinned length**, any whole number from 1 bar up, and **Endless**.
  The generator works a **section at a time**: memory and cost are the
  same for 4 bars, 32,768 or Endless, and any section is reachable
  straight from the seed and its number, without generating what comes
  before (so a long piece can later be jumped into).
- **What fills a long piece, until long-form composing exists:**
  sections drawn the way "Let the loop wander" draws its next pass,
  holding the request's other picks (key, tempo, style, voices, vibes).
  Writing a piece with an arc (themes that return, a symphony's
  journey, or long slow wandering) is later composing work by ear, its
  own item; this item only makes sure it needs no rework. Conservative,
  "for Mikey's ears".
- A short pinned length (1, 2, 3 bars) plays as a loop of that length,
  honored even where today's profile tables never draw it.
- Length left to chance: today's draws, unchanged.
- The Length picker's play time ("bars · ≈ 45-55 s", "≈ 3-4 hours")
  comes from the request's tempo range, from one function the app and
  the core share, so the two never disagree.
- Remove Painter 6's clamped length plumbing (bars beyond today's
  tables now come from the core).

**Yardsticks:**
- Nothing picked: unchanged, as above.
- Pinned length: the rendered cycle is exactly that many bars (1, 2, 3,
  7, 24, 32, 1,024 and 32,768, every profile).
- Section *n* rendered straight from the seed equals section *n* reached
  by rendering through, sample for sample (hash), for *n* up to 1,000
  and for the last section of a 32,768-bar piece.
- Memory after `Core::new` and cost per section flat from 4 bars to
  Endless (the core rules: no allocation after `Core::new`).
- Endless plays for a 24-hour render without drift or growth (in CI,
  the command-line renderer at speed).

## 50. Key and scale (phase 2)

**Why:** line three of the design: "any key", or a pinned home note and
scale (major, minor, dorian, pentatonic and the rest).

**What:**
- Key (home note) and scale, each pinned, ranged (a set: "D or G";
  "any minor-ish mode") or chance. Follow 48's request pattern.
- **Vibes and scale:** today's vibes pull the mode (and cinder and
  shatter borrow bright modes when glad). A pinned scale wins; the vibes
  then lean everything else. A pinned scale against a profile's
  character finds its in-between ("no pair is forbidden").
- Profiles with fixed modes (wayfare's minor v, cinder's added F over
  E: Mikey's decisions stand) keep those features where the pinned
  scale allows them; say per profile what a pin changes.
- The app's line: values shown ("D dorian"); untappable until the
  picker is designed.

**Yardsticks:**
- Nothing picked: unchanged.
- Pinned key and scale: the loop's pitch classes stay in the scale
  (borrowed chords the profile is known for excepted, listed) and its
  tonic is the pinned key, measured from the render across 200 seeds
  and every profile.
- Refusals on full and lite for pinned scales no worse than the
  catalog.

## 51. Style (profile) (phase 2)

**Why:** line four: "any style", or one or several of Driftloom's
profiles (cinder, tide, wayfare, shrine and the rest).

**What:**
- Style pinned to one profile, or a set ("tide or shrine"); chance as
  today. Each profile keeps its own character, instruments and drums;
  vibes bend it within that character, as today.
- A pinned style with other pins that pull against it (tempo outside
  its band, a scale it doesn't use) follows 48 and 50: honor the pins,
  find the in-between, "can't do that" only if truly impossible.
- The app's line: the profile names; untappable until the picker is
  designed.

**Yardsticks:**
- Nothing picked: unchanged (the balance lock guards what plays on
  chance).
- Pinned style: every loop is that profile; a set draws only from the
  set, each member reached across 500 seeds.
- Per profile, `stats.mjs` figures for pinned-style loops match the
  same profile's loops drawn on chance, within stated tolerances (a
  pin picks a profile; it doesn't change it).

## 52. Voices, and which layers play (phase 2)

**Why:** line five: "any voices", or chosen instruments per layer
(melody, chords, bass, texture, drums), and layers left out (fewer
layers is welcome variety).

**What:**
- Per layer: a voice pinned or a set, or chance; and the layer on, off
  or chance. Follow 48's pattern.
- **The rarities stay rare:** the choir (about 1 loop in 30) is not
  forced by picking melody and chords voices; it can be asked for only
  if the voice list offers it by name. Conservative: not offered until
  Mikey says so.
- A voice pinned onto a profile that never uses it plays at its
  measured level (each voice already sits at its layer's median) and
  passes the refusals; say which combinations needed care.
- All layers off is the one obvious "can't do that".
- The app's line: values shown ("kalimba, marimba"); untappable until
  the picker is designed.

**Yardsticks:**
- Nothing picked: unchanged.
- Pinned voices and layers: what sounds is what was asked, per layer,
  measured from the render across 200 seeds and every profile.
- Levels: pinned-voice loops land on the catalog's level spread
  (`measure.mjs --profile`), no layer louder than its median allows.
- Refusals on full and lite no worse than the catalog.

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
