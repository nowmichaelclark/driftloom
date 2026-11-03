# The queue

Written by the brain for Claude Code (the hands). Work top to bottom. Only
what is live is here: the status, the standing rules, the open items, the
known issues and the merge policy. Finished items, the old notes and the
Done list are in `docs/archive/` and `docs/DONE.md`; `docs/README.md` is the
map of all the docs. Mikey's ears come later, in listening albums the brain
builds from the Done list.

**Status, 2026-11-03:** `main` is at v144. Done: items 0-36 and 38-47 (13
stopped at its gate by design), the core's phase 1 (42-46c), the sound batch
(54-73), 74 and 76-76d, and phase 2 so far: **the port, items 77-82**
(#287-#296; checked by the brain's Session 28), **83** (ears on the core),
**84** (the chords' octave), **85** (the request: 85a #302, 85c #303, 85b
#304, 85d #305; checked by the brain's Session 29), **86** (#308, a change of loop at a musical moment), **85e** (#309, the steal at full quality), **85f** (#310, a theme's rest kept) and **87** (#311, rendering ahead); 86-87 checked by the brain's Session 29; **87b** (#313, the app picks lite by itself on slow devices) **86b** (#315, a late `now` change on the grid) and **88** (#318, length up to 32,768 bars and Endless), **89** (#319, key and scale), **90** (#320, style) and **91** (#321, voices and layers). **Next: 92.** Item
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

## 87b. The app picks "lighter on processor" by itself on slow devices

**Hands:** Opus 5.5, high. Before 88 (Mikey, 2026-11-02: yes, the app may
switch to lite by itself on slow devices, the listener can still switch it
back; `docs/DECISIONS.md`).

**Why:** item 87's speed test on Mikey's older Lenovo tablet (8 cores, 4 GB)
put the worst block at full quality at 89-132% of its deadline, and lite at
47-76% (`docs/archive/notes/87-phone-figures.md`). It plays without a stutter
for him today, as far as he hears, thanks to the app's large output buffer
(`latencyHint: 'playback'`) and because the test ran in a worker, which a
phone may put on a slow core, where the worklet gets a fast one at a higher
priority. So the worker's figures are pessimistic, and the decision should
read the real audio thread. Today the app guesses instead: lite when the
device has 4 cores or fewer (`js/main.js`, `prefs.lite ?? cores <= 4`), so
an 8-core tablet starts on full.

**What:**
- **Measure where it plays:** the worklet times its blocks on the audio
  thread (it already reports rendering cost, item 66) and keeps the busiest
  blocks over a sliding window, plus the late notes it already counts by
  cause (a stall of the page is not the device's speed: count only what the
  render's own cost explains).
- **The rule** (first values the conservative choice, "for Mikey's ears", both
  named settings): on full, if the busiest blocks over the last 20 s pass
  **50%** of the deadline (a high percentile, not one block), or the render
  falls behind once for its own cost, switch to lite. From the measured
  ratio of lite to full on the Moto (about 0.6), the app may also move a
  device it started on lite by guess up to full, once, when lite's busiest
  blocks stay under **15%** for a minute; never back and forth (once moved,
  the auto choice holds until the listener chooses).
- **The listener's choice always wins:** switching "lighter on processor"
  by hand stores it as theirs, and the app never changes it again. The auto
  choice is stored apart from theirs, per device, and remembered.
- **The switch glides, never cuts:** today the lite switch rebuilds the
  audio (`buildAudio()`). For the automatic one, wait for the next loop or
  the next pass, and hand over as a loop change does (a short fade), or say
  why it can't yet; in phase 3 it moves onto `Player::change`. A switch for
  falling behind may go at the next bar.
- **Say so, quietly:** Diagnostics shows the choice and why
  (`lite: auto (busiest 63% at full)`), and the switch's line on the
  Advanced page shows that it was set automatically, in words the Painter
  docs allow; no pop-up.
- The 4-core guess stays only as the first guess before anything is
  measured.

**Yardsticks:** under `tools/perf.mjs`'s CPU throttle, a throttle that puts
full's busiest blocks over the line switches to lite within the window, with
no click and no late note at the switch (rendered and measured); with no
throttle, full stays full over ten minutes and many loops; a listener's
choice is never overridden (both ways); the choice survives a reload; the
rule can't flap (a throttle near the line, an hour: at most one change); no
sound changes in either quality (the core baseline and the goldens
unchanged); the cost of the measuring itself under 1% of a block. Then Mikey
on his devices: the Moto stays on full, the tablet moves to lite, and
Diagnostics says why on each.

**Built (hands, 2026-11-02):** the worklet measures how busy its audio
thread is a quarter-second at a time (Chromium's worklet has no
`performance.now()`, so a block's time is whole milliseconds, exact on average
over a window), and `js/quality.js` holds the reports to the rule: on full,
the busiest quarter-second over 20 s of playing past 50%, or one taking longer
than the audio it made, moves to lite; lite from the four-core guess, under
15% for a minute, moves to full; once moved, never again; the listener's
choice is never touched (`prefs.lite`, the app's own `prefs.liteAuto`). The
switch waits for the next pass (a bar, for a render falling behind): the old
quality fades out over 0.5 s to it and is let go, then the new one takes the
loop from its top, a breath later. Not a crossfade: building the new worklet
compiles the core on the audio thread the old one plays on, and two cores at
once is what a slow device can least afford (for Mikey's ears).
Diagnostics: `lite: true, auto (busiest 63% at full)` and the busiest figures;
the switch's line says when the app chose. `tools/switch.mjs` renders the
switch offline (the join's step against the loop's own, late notes);
`tools/autolite.mjs` runs the app live with `?burn=` (a slower audio thread:
`perf.mjs`'s CPU throttle slows only the page's).

## 86b. A "now" change asked while the next pass is being made

**Hands:** Sonnet 5.5, high. After 87b, before 88. Small (brain, Session 29,
2026-11-02).

**Why:** the brain's own run of item 86's harness on another corpus
(`dlrender --changes 300 --from-corpus 20261101`) found one change that
missed its moment, where the hands' 1,000 on corpus 1 found none: change
270, `Now`, old loop shoung-roun (180 bpm, 2 bars, a pass about 2.7 s),
asked at 2.28 s, while the player was making the old loop's next pass; the
new loop began at 2.601 s, 31.2 of the old loop's steps from its start, off
its step grid. The join was clean (0.049 against the loops' own 0.065).
Suspect, unproven: `Player::change`'s branch for a change asked while a loop
is being made (`making`), which starts the new loop where the pass being
made would have started, or a time taken before the old loop's swing or its
tempo is applied.

**What:** find the cause by rendering; a `now` change lands on the old
loop's next step from any state, as the other 299 did, or the item says why
it can't and the harness's check says what it holds instead. Add the case to
`core/tests/change.rs`.

**Yardsticks:** the brain's run above with 0 missed; corpus 1's 1,000, full,
lite, ring-out and 48 kHz still 0 missed; the core baseline and every golden
unchanged.

**Built (hands, 2026-11-02):** not the `making` branch: the new loop took
longer to make than the old one had decided ahead. Change 270's new loop
has a tuned kick, whose fit takes 94 blocks here (0.27 s, past the 0.25 s
lookahead), so when it was made (2.5513 s) the old loop's next step (2.5500
s) had passed by 1.3 ms, and `begin` fell back to "now plus `START`", off
the grid. Now it takes the first of the old loop's steps still to come
(2.633 s, step 31). Only a load that outlasts the lookahead changes (from
playing: a `now` change, or `load`); `core/tests/change.rs` asks it across
2.20-2.36 s, and fails without the fix.

## 88. Length, up to 32,768 bars and Endless (was 49)

49's text (below) is the brief, built on 82's player and 87's chunks. Plus:
Endless and long pieces render ahead as 87 says.

**Built (hands, 2026-11-03):** the request's length (`bars=N`, any whole
number from 1 to 2^53 - 1, or `bars=endless`; `Request::with_length`,
`dl_player_length`, `readRequest`'s `length`), in both generators, the
player, the page's engine, the C header and wasm, share code format 8 (a
number in as few bytes as it needs). Up to 32 bars (the longest any table
draws) it is the loop's own length, the table's draw made and overruled.
Past that, and Endless, the loop keeps its own length and plays section
after section, section `n` its wandered pass `n` (`generator/piece.rs`,
`pieceOf`): memory and a section's cost flat, any section straight from
its number. **For Mikey's ears:** a round's last section is cut short at a
bar line where the loop's length doesn't divide the piece's (100 bars of a
16-bar loop: six sections and four bars); a piece comes round with new
sections, as a loop's next pass wanders (the app plays it once round, then
the next loop, as before); sections wander as the slider says (off: the
loop repeated); layers with their own cycles keep drifting across sections;
a piece's first section is section 0 (a load's first pass otherwise wanders
at the last loop's count). The Length picker's play time is `pieceTime`,
the core's `piece::time`; Now Playing keeps today's clock (a countdown to
the piece's end). Section `n` is held for its notes as the generator draws
them (the pass's hash), not its samples: a section's audio carries what
rang into it from the one before, which a jump can't have.

## 89-91. Key and scale (was 50), style (was 51), voices and layers (was 52)

50, 51 and 52's texts (below) are the briefs, in that order, each on 85's
request. Sonnet 5.5, high, each.

**89 built (hands, 2026-11-03):** the request's key and mode, each a note or
mode or a set to choose from (`key=D`, `key=D|G`, `scale=dorian|aeolian`;
`Request::with_keys`, `with_scales`, `dl_player_keys`, `dl_player_scales`,
`readRequest`'s `key` and `scale`), in both generators, the player, the
page's engine, the C header and wasm, and share code format 9 (two numbers,
each as few bytes as it needs; a loop asked for neither is the format it
was). The draws are made as ever and the request then overrules two values:
one the seed already drew inside the set stands, else one of the set from a
stream of the seed's own (keys alike; modes weighted by the profile mix and
vibes, a mode the profiles never draw at a floor, `ASKED_FLOOR` 0.25, **for
Mikey's ears**: so no pair is forbidden and the vibes still lean what they
can). A key and mode asked overrule a theme's. The app's key line shows what
is asked ("D dorian") and clears with its "any" key; there is no picker, so
it cannot be tapped. What a pin changes, per profile: nothing is forbidden;
a profile's progressions written for other modes fall back to the
seven-note shapes, and cinder's added F over E and wayfare's minor v play
where the mode allows them (`plainTriad`, `spelledSteps`). Out of the mode,
as the catalog's own loops are: the bass's fifth and pickups (a fixed
interval from a chord's root; 1.2% of its notes pinned, the catalog's 0.85%),
cinder's spelled chords and the melody over them, and the organ's and
stab's fixed intervals on the chords channel (0.9% of its notes pinned, the
same seeds on their own modes 0.7%).

**90 built (hands, 2026-11-03):** the request's style, a profile or a set
(`style=tide`, `style=tide|shrine`; `Request::with_styles`,
`dl_player_styles`, `readRequest`'s `style`), in both generators, the player,
the page's engine, the C header and wasm, and share code format 10 (one
number). The draws are made as ever and the request then overrules the
profile mix: a mix already inside the set stands, else each profile outside
it is replaced by one of the set (a stream of the seed's own, weighted as the
mix's draw weighed the profiles), so a pinned style is that profile whole and
a set keeps the loop's blend across its members. A style asked overrules a
theme's mix (its key, mode, metre and tune hold); a tempo asked is honored
(a profile with gaits plays its nearest gait at it). Profiles keep their own
character; vibes bend them as ever. The app's style line shows what is asked
("tide or shrine") and clears with its "any" key; no picker, so it cannot be
tapped. **For Mikey's ears:** the substitution's weights (a set of profiles
far from the seed's own lands where the mix's draw would have), and a pinned
style onto a tempo outside its band.

**91 built (hands, 2026-11-03):** the request's voices and layers: for each
layer a voice or a set (`voice.melody=kalimba|marimba`; the layer's pools'
names less the choir's, which is not offered: the rarities stay rare), and
each layer on, off or left to chance (`layer.drums=off`);
`Request::with_voices`, `with_layer`, `dl_player_voices`, `dl_player_layer`,
`readRequest`'s `voices` and `layers`, in both generators, the player, the
page's engine, the C header and wasm, and share code format 11. Every draw is
made as ever and the request overrules one value: a voice inside the set
stands, else one of the set from the layer's seed, leaning to the profiles'
pool (`VOICE_FLOOR` 0.25, **for Mikey's ears**); a voice asked of the melody
or the chords makes the loop not a choir and the arpeggio that voice. A layer
on is not left out by its own chances (drums, melody, bass style, texture) and
plays throughout (its entry schedule all in, after the rest placed by hand);
a layer off is the spec's mute, the player's sequencer mutes it at the load.
All five off is the one "can't do that". The app's voices line shows what is
asked ("kalimba, marimba, no drums") and clears with its "any" key; no
picker. Known and not mine: a melody asked on is silent in 4 loops of 3,000
(undertow's one-bar micro-loop cycles; a forced re-roll is too); drums asked
on wait for the lateFloor choice's second arrival where the loop has it.

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

**From items 88-91 (brain, 2026-11-03):** the same listening, so Mikey
listens once: 88's lengths (100 bars, whose last section stops at a bar
line; 1,024 bars and Endless, wander on and off; 1-, 3- and 7-bar loops),
and two or three undertow loops with the chords' stab, beside a loop with
`voice.chords=stab` pinned on another profile. **Decided (brain,
2026-11-03; `docs/archive/notes/91-voice-levels.md`): the quiet voices stay
offered as built** (the catalog's own voices at their own levels; no picker
can ask for them yet, and `VOICE_ORDER` is append-only). If Mikey hears the
stab's chords as too quiet in undertow's own loops, the fix is the voice's
level in `data/voices.txt` for every loop (a sound change, for his ears),
not a separate level for pinned use; the same question for tubular, nylon,
templebell and wind. Also accepted: 91's lite refusals for a pinned piano
(a lower mean than the catalog's; 3 of 222 loops losing a note against 1 of
224 is within the noise of so few).

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
