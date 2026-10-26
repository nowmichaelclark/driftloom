# The queue

Written by the brain for Claude Code, so building can carry on while Mikey
has no time to listen (2026-09-25). Work top to bottom. Mikey's ears come
later, in listening albums the brain builds from the Done list.

**Status, 2026-10-12:** items 0-33 are done (13 stopped at its gate by
design): the Rust core is Driftloom's engine and plays by default; the
JavaScript synth is the fallback (and `?engine=js`). The gate for a change
to the core is now its own baseline, `node tools/baseline.mjs` (see the
standing rules). Items 39 (#180), 40 (#182) and 41 (#187, the Pixel 10a
on v91) are merged, each waiting on a tester's report (Diagnostics'
"while hidden" and "views" sections; on v92 also `gpu`, the render-ahead
and the late and dropped splits). 41's 1.3-1.8 s stalls were not
reproduced here (Notes from Claude Code, below). 38, 34, 35 and 36 are
done. **Next: the core, phase 1 (items 42-46)**, from the charter
`docs/CORE.md` (approved by Mikey, 2026-10-13); if the tester's v92
report calls for a fix, it comes first. 37 stays on hold. A new
Claude Code session starts here: read the standing rules and the merge
policy, then take the next item.

**Status, 2026-10-19 (brain, Session 24):** the core's phase 1 is done
(items 42-47 merged; 46 checked by the brain). **Next: item 46b** (the
interface finished for the batch: modulation, stereo shape, channel
events), **then the sound batch, items 54-73** in three groups
(instruments, stereo in the core, the mixing logic), then the brain's
check and Mikey's album (Session 26), then phase 2. All of it sits under
item 46, below. Item 74 (clippy) is low priority.

**Next picks (2026-10-13):** Mikey chose the design
(`design/next-picks.md`); Painter 6 builds the screens on the app track.
Wiring each pick for real is items 48-53, inside phases 2 and 3; see
"Next picks, wired for real" below.

**Two tracks (2026-10-13):** app work (fixes, redesigns, vibe tuning,
new controls) goes on alongside the core phases, as separate PRs; see
`docs/BRAIN.md`, "Two tracks". Core-track items here don't wait for it,
and app-track sessions stay out of `core/`.

**Stereo across the board, with a mono switch (Mikey, 2026-10-15;
replaces the earlier mono lines):** stereo is coming, after the
instrument batch and before phase 2: items 64-66 (`docs/CORE.md`, promise
9), on the stereo shape item 46b decides. Until those items, build
nothing toward stereo beyond item 45's channel-layout seam and 46b's
shape.

**The engine, from here (Mikey, 2026-10-10):** the Rust core is
Driftloom's own engine, not a copy of Chromium. Matching Chromium was
how the port was proven; from item 31b on, the core may do better where
better is possible (judged by measurement and Mikey's ears), and
nulling against JavaScript is a safety net until the core is the
default, not a goal. See `docs/BRAIN.md`, "The engine".

**The words, from here (brain, Session 16, 2026-10-11):** item 38
first (Vibes in the app, Mikey's top priority), then items 34-37.
34 and 35 are tools only (no sound changes): the dials, the fit, and a
word-album builder; 36 is a small generator change from two decisions
Mikey made; 37 lets him answer word albums on `/rank`. Do them in order. Mikey is ranking on `/rank` meanwhile: nothing in
these items may change what an existing song code renders, because every
ranking is tied to its code.

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
  full quality stays near zero, and lite no worse than the catalog.
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
- **Stop and leave a note** at the bottom of this file, rather than push
  on, if a check can't be met or the item turns out to need a decision.
- **When a PR is merged,** add one line to Done below: the PR number, the
  commit preview URL, and what the ears should listen for.

## 0. Finish #59 (fiddle and accordion, take two)

Finish its after-runs, fill in the PR body, and merge it before starting 1.

## 1. `tide`, part 2

- **3/4 melody rhythms.** In 3/4 the melody still uses the 6/8 cells.
  Give 3/4 its own cells (a waltz tune: long notes on the beat, lilting
  pickups), chosen as the 6/8 cells are.
- **3/4 gaps.** Silent gaps in 3/4 still count 3-step beats; count the
  waltz's 4-step beats.
- **Ornaments.** Cuts and turns, the quick grace notes of folk fiddle and
  whistle. The generator only marks which notes get one (a profile field
  such as `ornament: 0.4`, a salted draw); the engine plays the grace at
  sub-step timing, 30-60 ms before the note, never on the step grid.
  Only tide asks for them for now.
- **Harmony in thirds or sixths**, on held notes only (long notes and
  phrase ends), never doubling every melody note, costed like any voice.
  A profile field; tide only.
- **A waves texture:** slow swells of filtered noise, the sea without
  anything literal. Add it to tide's textures.
- **A hand-drum kit** beside tape, brush and machine: a frame drum
  (bodhran-like), with shaker and jingle. Add it to tide's kits.
  Calibrate to the drums layer as it stands.

## 2. Soften the harsh leads

Saw, moog and analoglead are at the right level (#48, #53) but Mikey
called them "harsh", "sharp" and "gross" in the blind lead audition. Do
what take two did for the fiddle: a softer shaped source instead of a raw
sawtooth, fixed tone rather than a bright per-note sweep, keeping each
voice's character (moog may keep a gentle filter movement). Tone probe:
each at or below the flute's 2-5 kHz share. Re-level to the melody
median at 0.4 s, re-measure `VOICE_COST`.

## 3. Voices, pair 2: nylon and pan flute

- **Nylon guitar.** Plucked, warm, with body resonance. As a chord voice
  it strums: 15-30 ms across the strings, down on the beat and up off it,
  the downstroke accented. Today's chords spread 11 ms in one fixed
  order; the strum applies to nylon only.
- **Pan flute**, the Spirit Tracks instrument: a flute variant with more
  breath, a chiff on the attack, a small pitch dip into the note, no
  vibrato on short notes.
- Into tide: chords nylon 2.5; melody pan flute 1.5. Keep every existing
  entry. (They also go into items 4 and 5.)

## 4. `cinder`, the fiery 6/8

The island-and-volcano music: fast, rhythmic, Spanish-tinged.

- 6/8 mostly (about 120-150 bpm), some 4/4; drums in about 90% of loops,
  the hand kit leading.
- Scales: phrygianDominant, harmonicMinor, aeolian, dorian, phrygian.
- The signature progression is the Andalusian cadence (in A: Am-G-F-E,
  ending on a *major* V). Degrees alone give the wrong chord qualities in
  these modes, as #57 found, so filter or override per mode to get it
  right.
- Chords: nylon (strummed) leading, marimba, accordion. Melody: fiddle,
  whistle, marimba, accordion, pan flute. Bass: pulse and walk.
- `level` solved by measurement; appended to the end of `PROFILE_IDS`.

## 5. `wayfare`, the rolling train

The traveling music: steady, bright, moving.

- Mostly 4/4 (about 104-138 bpm), some 6/8. A **chug**: a steady
  eighth-note pulse in the bass (and brushes) under the tune, like wheels
  on rails; a new bass style, drawn only by profiles that ask for it.
- Scales: mixolydian, ionian, dorian, lydian, majorPent.
- Progressions: I-bVII-IV-I and I-IV-V-IV, filtered per mode.
- Melody: pan flute leading, then fiddle, whistle, ocarina. Chords:
  accordion, nylon, harp. Drums in about 85% of loops: brush, tape, hand.
- `level` solved by measurement; appended to the end of `PROFILE_IDS`.

## 6. Pan flute: static when notes overlap (a bug)

Mikey, review album track 11 (a tide loop, pan flute leading): "some sort
of glitch ... almost sounds like it's clipping; there's this static noise
that happens when the pan flute notes overlap. The pan flute itself
sounds good." So the tone stays; the glitch goes.

Find the cause by rendering, not guessing: overlapping pan flute notes
through the real Synth, looking for clipping at any node, sample-to-sample
jumps (clicks), and noise that builds as notes stack. Suspects, unproven:
the chiff spike (a 4 ms exponential ramp up to `PANFLUTE_CHIFF`), every
note's breath starting the same shared noise buffer at offset 0 (so
overlapping notes play the same noise a few milliseconds apart, which
combs), or the breath's bandpass stacking. Fix whatever it is, keep the
tone probe and level where they are, and add a check to the tests or
`measure.mjs` that would have caught it. Other voices that share the
noise buffer the same way should be checked for the same fault and named
in the PR, fixed only if they have it.

## 7. Nylon: a cleaner strum

Mikey, track 10 (tide, nylon strumming): "sounds smeared, not bad sounding
though either." Keep the sound; tighten it. Likely levers: a narrower
spread across the strings than 15-30 ms, the previous strum's strings
damped when the next chord starts (a guitarist's strings don't ring on
into the next chord), and less ring from the shared body resonance. Take
the conservative end, write the choices down "for Mikey's ears", and
re-check tone, level and cost.

## 8. `wayfare`'s drums and chug: playful, not tough

Mikey, review album tracks 16-19 (all `wayfare`): the melodies, chords
and the minor v are nice, but the drums "kill it" on all four, though
"not at all terrible". The chug "leans a bit toward that machine gun
vibe or hip-hop ... too heavy or tough rather than playful". On the
others: "the kick on the drums is a bit too loud, or not the right sound
shape or tone -- more tribal sounding? Not sure."

What the brain found: `wayfare` draws the catalog's lo-fi drum patterns,
so two of the four tracks played a syncopated boom-bap kick (steps 0, 6
and 11 in the bar) with claps; one played the tape kit; the hand-kit track
put the same kind of pattern on the frame drum. The kits aren't the
problem (tide's hand-kit jig got "love it"); the grooves are.

Do, for `wayfare` only:
- **Its own drum grooves:** traveling music, light and steady. The kick
  (or frame drum) on 1 and 3, never syncopated boom-bap; brush, rim or tap
  on 2 and 4; no claps; hats, shaker or jingles light on the eighths. The
  same shapes on every kit it draws.
- **A softer kick in `wayfare`**, lower and rounder. Yardstick: the drums'
  level over the music (as in #31) should sit near tide's hand-kit jig
  loops, which Mikey loved, not near the catalog's lo-fi loops.
- **A playful chug:** the beat accented and the offbeats ghosted well
  down ("chug-a" rather than an even rattle), shorter bass notes, and the
  brush's eighths lighter to match. Keep it a train, not a march.
- Melody, chords and everything outside `wayfare` untouched; existing
  non-`wayfare` loops must render identically. The usual level, refusal
  and balance-lock checks. Take the gentle option where unsure, and say
  "for Mikey's ears".

## 9. `wayfare`'s drums: bring the life back

Mikey, before/after on the four wayfare tracks: "the drums sound better
in v53, but ... the composition is less dynamic and interesting than it
was in v52 for the drums." Keep what #79 fixed (no boom-bap, no claps,
the soft round kick, the lighter chug); give back the variety it took.

Measured by the brain, 150 wayfare-led loops each (tide-led for the
yardstick, which Mikey loved):

| | wayfare v52 | wayfare v53 | tide v53 |
|---|---|---|---|
| bars that differ from each other (share) | 0.95 | **0.32** | 0.99 |
| instruments per loop | 4.8 | **3.0** | 4.7 |
| velocity spread (sd) | 0.177 | **0.150** | 0.193 |
| hits per bar | 11.4 | 11.1 | 7.3 |

So v53 plays nearly the same bar over and over, on fewer instruments, with
flatter dynamics. The item 8 brief asked for grooves that were "steady"
with "the same shapes on every kit", and took it too literally.

Do, for `wayfare` only:
- Several groove variants per loop, not one, with bars that vary as the
  catalog's do: the kick stays anchored on 1 and 3, but pickups, an
  occasional extra kick into a beat, ghost notes, open hats or shaker
  accents, and a fill or roll at phrase ends are all welcome. Still never
  boom-bap and never claps.
- Back to about 4-5 instruments per loop (percussion color: rim, shaker,
  jingles, taps, whatever each kit has).
- Dynamics: accents and ghosts, and swells across the phrase, so the
  velocity spread returns to v52's or tide's.
- Yardsticks, reported in the table above's form: bars that differ at or
  above 0.9, instruments near 4.5, velocity spread at or above 0.17, and
  drums over the music still near tide's hand-kit jigs.
- The chug stays as #79 made it unless the variation needs it to breathe.
- Everything outside `wayfare` untouched. The usual checks.

## 10. A performance harness and baseline (measure only)

Mikey, 2026-09-25: even at full quality the app should be very light and
fast -- "something that would make even DHH go 'wow, I can't believe how
smooth this is even while I have a hundred other apps open'". Overhauls
are on the table, but numbers come first. This item changes nothing in
`js/`.

Build `tools/perf.mjs`: the real app in headless Chromium, playing loops
live (not offline), under CPU throttling through the DevTools protocol (at
least 1x, 4x and 6x), full and lite. Per run, report:
- audio health: the engine's late ticks, and any glitch signal the browser
  exposes;
- main-thread cost: script, layout and paint time per second of playback
  (`Performance.getMetrics`), and long tasks;
- audio-graph churn: Web Audio nodes created per second (instrument the
  context in the harness, not the app);
- memory: JS heap over a few minutes of playback, and whether it climbs;
- time from pressing play to first sound; page weight on first load;
- the same with the tab hidden, which should cost next to nothing.
Use a fixed set of loops that includes the heaviest profiles and long
loops. Commit a baseline report (for example `docs/perf-baseline.md`) and
say where the time goes. Do not optimize anything yet.

Two more asks, because this baseline is what a future compiled audio
engine (roadmap item 16) has to beat:
- Keep the harness engine-agnostic: it drives the app from outside and
  reads its figures from the page, so the same tool can measure a
  replacement engine later.
- Split the cost as far as the browser lets you: note scheduling and
  generation in JavaScript, audio-graph construction (node creation and
  connection), audio rendering, and the UI. The point is to know how
  much a compiled engine could actually save, and where.
Play with "Let the loop wander" on, the default, since that's how most
people will listen.

## 11. Sound polish: measured fixes

Each is small and measurable; they change how existing loops sound, which
is the point.
- **Sung notes that jump out.** In `vowel`, `choir` and `hum`, a note whose
  fundamental lands on a formant peak comes out 10-15 LU louder than its
  neighbors (#24's probe: note-to-note spreads of 16-21 LU against about
  1-2 for most voices). Tame the jumps without flattening the voices'
  character; report the per-note spread before and after.
- **Temple bell as a chord voice** is still about 2.5 LU under the chords
  layer, and Mikey heard it as buried. Bring the chord use to the layer
  median at 0.4 s notes; the melody use stays.
- **5/4 chords.** 5/4 still uses the 4/4 chord table, so its fifth beat is
  always empty. Give 5/4 its own table, in the spirit of the 6/8 rules
  (#34, #38): chords move on the beats and hold.
- **Arpeggios run past the bar**, because each note keeps the full length.
  Clip them to their slot.

## 12. Composition depth, first pass

Mikey: "wider melodies and just wider compositions in general would be
really nice. It can sometimes feel like the app will stick with
compositions that are too elementary even when there are many bars to
fill." Also: "it should still be completely possible for the very simple
melodies and compositions to fill a 24-bar song. ... I really like the
music the app already makes, we only need some compositions to explore
adding more depth. We can play with what that balance should be."

Measured by the brain (3000 loops, main at v54):

| loop length | loops | distinct melody bars | bars with melody | distinct chord bars | melodic span |
|---|---|---|---|---|---|
| 4 bars | 36% | 2.9 | 3.6 | 3.5 | 8.8 |
| 8 bars | 28% | 4.2 | 5.7 | 4.9 | 10.3 |
| 16 bars | 16% | 6.0 | 9.9 | 6.5 | 11.4 |
| 24 bars | 2% | 7.9 | 13.7 | 7.4 | 15.8 |
| 32 bars | 1% | 5.1 | 10.8 | 5.3 | 7.8 |

A 32-bar loop carries five distinct melody bars and a narrower tune than
an 8-bar one. Long loops mostly repeat a short idea.

Do:
- **Development in some loops, not all.** A share of loops, weighted
  toward the long ones, develop: a contrasting section (a B phrase that
  answers the A, derived from its motif, often in a different register or
  rhythm), a return that varies rather than repeats (A'), chord movement
  that spans the form rather than cycling one short progression, and a
  shape across the whole loop.
- **Range as a spread.** Some tunes stay narrow, some roam: widen the
  distribution, not every tune.
- **One knob for the balance**, easy to turn later (for example a global
  `DEPTH` share with per-profile weights), so Mikey can "play with what
  that balance should be". Start conservative, around a third of loops of
  8 bars or more.
- **Simple loops survive.** Loops that don't develop must stay exactly as
  simple as today, and any length can still be filled by a simple idea.
- Yardsticks, in the table above's form, reported separately for loops
  that develop and loops that don't. The balance lock will move; that is
  the point. Report every figure that moves and re-baseline deliberately.
- For Mikey's ears afterwards: the brain builds an album of developing
  long loops beside simple ones.

## 13. Rust engine, step 2 -- stopped at its gate (brain, 2026-09-26)

Item 10's baseline answers the gate: building the graph costs 1.2-4.2 ms
a second on the main thread, 2-5% of it. The big costs are elsewhere and
fixable in plain JavaScript and CSS (item 14). Don't build the prototype.
The Rust port stays on the roadmap (item 16) as a portability project --
the phone app's core, a native build, games -- to be taken up after
composition depth settles, not as the performance fix. The original
brief follows for the record.

### (original brief, not to be built now)

Agreed with Mikey (2026-09-26): the synth moves to Rust sooner, the
generator later. This is the first real step. Start it only after items
10 and 11 are merged.

**The gate.** Read item 10's baseline first. If building and tearing down
the audio graph is a small share of the cost, stop and leave a note here
saying so, with the numbers: a Rust engine wouldn't pay for itself.

Otherwise:
- An AudioWorklet engine with its DSP in Rust compiled to WebAssembly,
  a preallocated pool of voices, and no per-note objects or garbage.
- Three voices, chosen to span the kinds: `kalimba` (simple and struck),
  `fiddle` (a shaped wave through fixed filters), and `pad` as a chord
  voice (the heaviest). Everything else stays on the JavaScript engine,
  and both engines play together through the same channels, reverb and
  master chain.
- Behind a URL flag (for example `?engine=rust`), off by default, so
  Mikey can play the same loops both ways on his phone.
- **Proven, not assumed:** each Rust voice against its JavaScript
  original with `measure.mjs` -- tone probe, level at both note lengths,
  and `--endings` -- within tight tolerances. Then item 10's harness,
  both engines, same loops, same throttling: that decides whether this
  goes further.
- **The build:** the repo deploys plain files today. Keep that: commit
  the compiled `.wasm` with a reproducible build script, and add a check
  that the committed file matches its source.
- The generator, the voice budget's rules, share codes and every other
  voice are untouched.

Report the equivalence table, the before/after performance figures, the
file size added to the app, and anything that made the port harder than
expected.

## 14. The performance wins the baseline found

From `docs/perf-baseline.md` (item 10). Re-measure every change with
`tools/perf.mjs` against that baseline, same loops, same settings, and
report before and after. None of these should change how anything sounds;
prove it with `measure.mjs --voice all`, `--endings` and the corpus
loudness where the audio path is touched.

1. **Paint, the main thread's biggest cost** (57-67 ms/s of 71-84 at 1x,
   90% at 6x). The whole 765 x 3555 page repaints about 116 times a
   second because it is one paint layer and the cursor, the playhead
   lights (a 90 ms color transition and a glowing `box-shadow`) and the
   grid cells change on it every step. Put what moves on its own
   compositor layers, animate only `transform` and `opacity`, drop the
   glow's `box-shadow` transition for something that doesn't repaint,
   and touch only the cells that change. Keep the look. Target: paint
   under 10 ms/s at 1x.
2. **Disconnect every voice when it ends.** Finished voices keep
   rendering until garbage collection finds them: 40-45% of the audio
   thread at 1x, about 70% when the main thread is slow. Disconnect each
   voice's nodes on its end. Expected: the audio thread from 54-95 to
   about 49-53 ms/s on the baseline loops.
3. **The hidden-tab excess.** Hidden, the audio thread costs 20-100% more,
   likely because voices are built and wired up to 3 s before they
   sound. Prove the cause, then build or connect voices closer to their
   start without risking dropouts when timers throttle.
4. **The graph that always runs** (reverb combs, delays, the bus: 28-37
   ms/s, even with every layer muted). Find what costs what; look for
   savings that leave the sound identical, and propose (don't make) any
   that would change it.
5. **Page weight.** The service worker precaches both `keepalive.flac` and
   `keepalive.wav` (334 KB of 458 KB gzipped) though only one plays, and
   every script is fetched twice on a first visit. Precache what's used.

6. **Drop the playhead lights' fade** (Mikey, 2026-09-27: "drop it for
   speed"). #96 kept the 90 ms fade and put the choice to him; he chose
   speed. Remove it, confirm paint under 10 ms/s at 1x.

One PR per part, in this order, each merged under the policy.

## 15. The temple bell as a chord voice: lift it

Item 11's note: at 0.4 s notes the chord temple bell already sits above
the chords median, at 1.6 s it sits 1.9 LU under, and the bell rings 6.5 s
whatever the note length. Mikey heard it as buried, so the long-note
reading decides: lift the chord use about 1.9 LU, melody use unchanged.

## 16. The mood labels: make "happy" sound happy

Mikey: "sometimes I will get a track that is '100%' happy and it certainly
doesn't sound that way. It's not a bad track or a negative sounding
track, but it doesn't feel like it is '100%' happy." And the constraint
stands: never negative or depressing -- Animal Crossing and a nice coffee
shop wouldn't go there. The eight moods already keep to that.

What the brain found (20,000 fresh loops on v55): 1,028 are labeled 90%+
happy. Their final `lift` averages 0.80, right on the happy region (0.82),
so the feel numbers are fine. But **42% of them are in minor-ish modes**:
dorian 16%, aeolian 9%, then harmonic minor, phrygian dominant, kumoi,
hirajoshi, insen and more. The scale comes from the profile's pool and the
mood barely steers it, so a "100% happy" loop in aeolian or phrygian
dominant sounds wistful or fiery, not happy. And the "100%" reads as
intensity when it means "all of its mood comes from the happy region".

Do:
- **Measure first:** for each mood, how its loops come out on the cues a
  listener hears as bright or dark -- mode (major-ish vs minor-ish), share
  of major chords, tempo, register, melodic direction. Report the table.
- **Let the mood steer the music.** High `lift` should weight bright modes
  and major harmony, and the other cues, clearly enough that the label is
  honest; low `lift` leans inward (reflective, peaceful), never sad. Keep
  every profile's character: steer within its scale pool where you can,
  and say where a profile's pool can't honour a mood.
- **The labels: not yet.** Mikey (2026-09-27): the mood words are
  "something we need to work through and maybe have some test albums to
  decide. We might need more mood words." So leave the words and the
  display as they are for now. Do make the vocabulary easy to change:
  the moods, their feel targets and their display words as data in one
  place, so adding, merging or renaming moods later is a small edit. The
  brain will run listening albums (Mikey describing tracks in his own
  words) to find the vocabulary; a later item changes the labels.
- The balance lock will move (more major modes, perhaps); report and
  re-baseline deliberately.
- Afterwards the brain builds a blind album to check the steering is
  audible (bright moods sounding brighter), separate from the naming.

## 17. Composition depth, second pass

Mikey heard the blind "Depth" album (8 long loops, 5 developing, 3
simple), rating each "goes somewhere", "loops" or "too busy":

| loop | kind | bars | heard |
|---|---|---|---|
| rul-glun (glade) | developing | 32 | goes somewhere |
| yam-vai (hollow) | developing | 24 | goes somewhere |
| neing-sho (wayfare) | developing | 16 | goes somewhere |
| shuing-soun (thaw) | developing | 16 | goes somewhere at the end, mostly loops |
| loal-lor (shrine) | developing | 16 | kind of; circles one idea |
| yain-hain (haven) | simple | 24 | goes somewhere -- "wonderful" |
| thoum-yeim (tide) | simple | 16 | loops |
| keing-shai (bloom) | simple | 16 | loops |

Nothing was "too busy". At 24 and 32 bars development is clearly heard and
liked. At 16 bars it's weak: the contrast comes late (A8 B4 A'4 puts B at
bar 9) and two of three still read as looping, matching the numbers
(distinct melody bars 7.3 developing vs 6.7 simple). And his favorite
was a simple 24-bar loop, so simple long loops stay.

Do:
- **Stronger development at 16 bars:** the contrast earlier and clearer --
  four-bar phrases (A A' B A'', each A varied, not repeated), a B that
  moves register, rhythm and harmony together. Measure the 16-bar
  developing loops against the 24-bar ones and aim for a comparable lift.
- **More development at 24 and 32 bars:** raise their share to about two
  thirds, per length, through the `DEPTH` weights. Leave 8 and 16 bars'
  share as it is until the 16-bar form has been heard again.
- Simple loops untouched. The usual checks; the balance lock will move.
- Afterwards the brain sends a second blind album.

## 18. Housekeeping: comments and README against the code

The brain audited the documents at the end of its long session (README,
roadmap, `docs/*.md` updated in the same PR as this item). The code's own
comments it can't edit. Known stale ones:
- `js/characters.js`'s header says profiles "would be ten boxes" and
  "ten points"; there are sixteen.
- `js/generator.js` (around line 1124) says phrase position "is roadmap
  item 2 and is still absent"; item 2 shipped (phrase velocity, #11).

Do a pass over `js/`, `tools/` and `test/`:
- Fix comments that are stale: counts, "not yet" notes about things that
  shipped, rules that changed (new voices may now join existing pools;
  old share codes are not protected while testing; the hands merge under
  the queue's policy).
- Check the README's claims against the code where they are checkable
  (flags, file names, figures quoted as current), and fix what's wrong.
- Comments and docs only: no behavior change. Prove it: tests three
  times, `stats.mjs` output byte-identical, `--check` unchanged. No
  version bump needed.

List what you changed in the PR, and anything you found but weren't sure
about as a note here.

## 19. A word-ranking test (diagnostics, roadmap 19)

Mikey (2026-09-28): describing tracks in his own words is too abstract.
He wants to hear a loop, see four or five words, and put them in order
from most like the track to least like it -- "not an over-engineered test
with nice UI, in fact simple code and fast would be preferable", so that
"a hundred plus songs" is easy. His answers become the data that places
each word on the maps (`docs/MOODS.md`, maps and clouds).

Build a separate page, `rank.html`, not linked from the app's main screen
(Mikey opens it by URL). It uses the app's own engine and synth, so it
always tests what the app plays. Plain HTML, legible on a phone, nothing
more.

- **A trial:** a fresh loop from `newSpec`, seeds counting up from a
  fixed start so a run is reproducible; it starts on its own and loops.
  Don't show the loop's mood label or profile (they'd bias the answer).
- **Five words** from the pool, shuffled, as big buttons. Mikey taps them
  in order, most like the track first, and **stops when the rest don't
  fit**: untapped words mean "doesn't fit" (no taps at all is allowed).
  Buttons: Undo, Next (records, moves on), Skip (a loop he can't judge or
  that sounds broken; recorded as a skip).
- **The pool is data** at the top of the page's script, so the brain can
  change it. Each trial draws five different words, balanced so every
  word is shown about equally often. Pool v1 (all from his list): merry,
  tender, serene, golden, twinkling, crisp, velvety, sour, bouncy,
  floating, swaying, quirky, happy, joyful, peaceful, reflective, lively,
  cozy, airy, misty, intimate, spacious.
- **A consistency check:** about one trial in twenty replays an earlier
  loop with the same five words in a new order, unmarked.
- **Record per trial:** trial number, build, the loop's song code
  (`encodeSong`), the words in the order shown, the taps in order, skip,
  and seconds from start to Next. Saved in the browser as he goes, so he
  can stop and pick up another day; a counter shows how many are done.
- **Export:** one button that copies everything as JSON lines and also
  downloads a `.jsonl`. Clear sits behind a confirm.
- **A tool for the brain:** `tools/ranks.mjs <file.jsonl>` prints per
  word how often it was shown, tapped first, tapped at all, never tapped,
  and the repeat trials' agreement. Measuring each loop's cloud on the
  dials is a later item.
- **Nothing in the app changes:** `index.html`, the engine and share
  codes untouched; the balance lock doesn't move. Say in the PR whether
  the service worker caches `rank.html` and why.

## 20. Ranking test: one word per map (pool v2)

From the first 20 trials (`docs/MOODS.md`, ranking results): no word fit
8 times, partly because five random words can bunch on one corner (one
trial offered tender, golden, misty, serene and peaceful together), and
fast, tense loops had few words to reach for. So each trial now shows
**one word from each of the five maps**, and the pool grows to cover both
ends of every map. All words are from Mikey's list.

- The pool becomes five groups, data at the top of the script:
  - color (warmth x lift): golden, frosty, tender, happy, joyful,
    reflective, merry, peaceful
  - shimmer (edge x height): twinkling, crisp, velvety, booming, soft
  - air (room x ground): floating, airy, intimate, cozy, spacious,
    serene, misty, steady
  - motion (energy x bounce): bouncy, swaying, lively, energetic, still,
    flowing
  - flavor (tang x oddness): sour, sweet, spicy, quirky, zany
- Each trial draws one word per group, least-shown-first within the group
  (as now), and shows the five shuffled. Group names are never shown.
- Record the pool version on each trial (`"pool": 2`; trials without it
  are pool 1) and keep the saved answers: v1 trials stay valid data.
- New loops continue from where the saved run left off; the seeds and
  hidden repeats work as before.
- `tools/ranks.mjs` reports per pool version, and per group.
- Nothing else in the app changes; the balance lock doesn't move.

## 21. Rust core, step 1: the pipeline and one voice (kalimba)

Roadmap 16, timing 2026-10-03: the synth moves to a Rust core now. Its
case is **portability**, not speed: the same core will later run the
phone app, a native build and games. This item proves the whole path end
to end with one voice, so the next voices are routine. Read roadmap 16
first, including "Feasibility": the brain's probe built, ran in an
AudioWorklet and nulled to -55.6 dB against Web Audio, but only once
note starts were sample-accurate.

**Intent.** With the flag on, a kalimba note comes out of Rust and sounds
the same as today; with it off, nothing in the app changes at all. Every
other voice, the effects and the master chain stay in JavaScript and
work exactly as now. The JavaScript synth stays the reference.

**The crate.**
- `core/`: a Rust library crate with **no dependencies** (crates.io has
  been flaky behind the sandbox's proxy, and the core needs none). Use
  `std`: its math (`sin`, `exp`, `powf`, `tanh`) compiles to WebAssembly
  with zero imports.
- **Host-agnostic:** the DSP knows nothing about browsers -- a voice
  pool, `process` into a block, events in. The WebAssembly exports live
  in their own module behind `cfg(target_arch = "wasm32")`. Native
  `cargo test` covers the core.
- **Preallocated:** a fixed pool of voices, no allocation after init, no
  panics reachable from `process`. Note what happens when the pool is
  full (it shouldn't be: the JS budget still decides who plays).
- **The building blocks, built to be reused by every later voice:** a
  parameter type that follows Web Audio's automation rules exactly
  (`setValueAtTime`, linear and exponential ramps, `setTargetAtTime`,
  as the spec defines them, including where a ramp starts), a sine
  oscillator, and FM by phase modulation the way an oscillator's
  frequency input does it. Kalimba is `fm()` (sine carrier, sine
  modulator, the decaying index) plus a sine body an octave down; port
  that, nothing more.

**The build -- the committed file.**
- `core/rust-toolchain.toml` pins an **exact** stable version (the newest
  the sandbox can fetch, e.g. `1.9x.0`, never `stable`) with the
  `wasm32-unknown-unknown` target. Release profile: `opt-level = 3`,
  `lto`, one codegen unit, `panic = "abort"`, `strip`; remap the source
  path so the output doesn't depend on where the repo sits.
- `tools/build-core.sh` builds and copies the result to `js/dlcore.wasm`
  (commit it). `--check` rebuilds and fails if the committed file
  differs by a single byte.
- `.github/workflows/core.yml`: on every PR and push to `main`, run
  `tools/build-core.sh --check`, `cargo test` in `core/`, and
  `node test/generator.test.mjs`. This becomes the repo's first CI beyond
  Cloudflare's build. **If the push of the workflow file is refused**
  (Claude Code may lack permission to change workflows), leave the
  file's full contents in a note at the bottom of this file and carry
  on; Mikey adds it on GitHub's website.
- `.gitignore`: `core/target/`. `.assetsignore`: `core`, `.github` (the
  source must not ship as site assets; `js/dlcore.wasm` must).
- If the sandbox and GitHub Actions produce different bytes from the same
  source, stop and leave a note: the whole decision rests on this.

**The host -- the worklet.**
- `js/worklet.js`, an `AudioWorkletProcessor`. The main thread fetches
  `js/dlcore.wasm` once and passes the **bytes** in `processorOptions`;
  the worklet compiles them with `new WebAssembly.Module` (sync, small
  module). Bytes rather than a compiled module, because iOS Safari is
  the target and support for passing a module across is less certain.
- No `SharedArrayBuffer`, no threads, nothing that needs special headers.
- **Events by `port.postMessage`**, each with its absolute start time in
  context seconds. The worklet queues them and starts each **on its exact
  sample** within the block (block-boundary starts wrecked the probe's
  match). A note that arrives late starts at once; count late notes for
  Diagnostics.
- **Outputs into the existing channels:** the node has one output per
  channel it serves (melody and chords for now), each connected into
  that channel's input in the synth, so echo, reverb, ducking, mutes,
  `silence()` and the master chain apply exactly as to a JS note. If a
  note's destination isn't one of those, it plays on the JS engine
  (count those too).
- Stop, re-roll, "let the loop wander", the hidden tab and `dispose()`:
  find what happens to an already-scheduled JS kalimba note in each case
  and make the Rust note do the same. Add a clear-all message for
  disposal.
- `recordingContext` and `OfflineAudioContext` (the measure harness) must
  work too: the probe ran there.

**The split -- what stays in JavaScript.**
- Scheduling, the voice budget (`_budget` and `_release` with kalimba's
  same costs, so refusals don't move), and **every `Math.random` draw,
  in the same order and number as today's kalimba code**. The values a
  note needs travel with its event. That keeps seeded renders comparable
  and keeps every other voice's draws where they were. Kalimba draws none
  today as far as the brain can see; check, and keep the principle for
  the next voices.

**The flag.** `?engine=rust` turns it on; off by default. Diagnostics
shows `engine: js` or `engine: rust` (plus late and fallback counts when
on). Precache `js/dlcore.wasm` and `js/worklet.js` in the service worker
so the flag works offline; say the bytes added.

**Proven, not assumed.** `measure.mjs --engine js|rust` (default js).
Report, both engines, kalimba as a melody voice and as a chord voice:
- `--voice kalimba` level at `--note 0.4` and `--note 1.6`: within
  **0.25 dB**.
- The tone probe's bands, including the 2-5 kHz share: within 0.5 points.
- `--endings`: no flags the JS voice doesn't have.
- **A null test** (new option): the same seeded notes through both
  engines, subtracted. Kalimba is all sines, so expect well below
  -40 dB; report the figure and explain anything above.
- **Nothing else moves:** with the flag off, `measure.mjs` output on a
  fixed set of loops (include some kalimba loops) is byte-identical to
  `main`. With it on, loops without kalimba are byte-identical too.
- Refusals on full and lite unchanged; the balance lock untouched (the
  generator isn't touched); tests three times.

**Report in the PR:** the equivalence table, the null figures, the
`.wasm` size and the bytes added to the page, the late and fallback
counts over a few loops, and anything that made the port harder than
expected. Since the sandbox can't open workers.dev, list the commit
preview URL with `?engine=rust` for the brain and Mikey.

## 22. Rust core, step 2: fiddle and pad, and the performance A/B

Start only after item 21 is merged. Same crate, host, flag, split and
proofs as item 21; this item adds the two voices that cover the hard
parts, and measures both engines.

- **Fiddle:** the custom `bowed` wave (Web Audio's `PeriodicWave` is
  band-limited: build a band-limited equivalent from the same harmonics,
  as Chrome does, and say how), vibrato on detune with its random rate
  and depth, slurs and slides (`_slur`, `_slide`, including its
  `SLIDE_CHANCE` draw), and the fixed body filters (`_body('fiddle')`,
  Web Audio's biquad formulas). Port the body into Rust too, per channel,
  so the voice is whole for the native and game hosts.
- **Pad:** both paths, `pad()` and `pad` as a chord voice: two detuned
  band-limited triangles per note through a lowpass whose frequency
  ramps.
- **Random draws:** in the same order and number as today, drawn in JS
  and sent with the note (item 21's rule). Fiddle has several; get the
  order right or every later draw in the loop shifts.
- **Proofs:** item 21's table for fiddle (melody and chords) and pad, at
  both note lengths, endings, and the null test. Band-limited waves may
  not null as deep as sines; report the figure and what's left in the
  residual (aliasing, phase, the wavetable). Nothing else moves, as in
  item 21.
- **Performance:** `perf.mjs --ab`, flag off against flag on, on the same
  loops (pick some with all three ported voices), under the usual
  throttling. Audio health must not get worse; report main-thread and
  audio-thread cost, graph churn and memory. This is information for the
  roadmap, not a gate: the port's case is portability.

**Then stop.** After the merge, the brain verifies, builds a short blind
X/Y album (the same loops with and without the flag, plus a control
track), and Mikey listens on his phone. What comes next -- the rest of
the voices, behind the same flag -- is written after he has heard it.

## Rust core, steps 3-8: every voice (items 23-28)

Written 2026-10-08 (brain, Session 5). Items 21-22 proved the path:
kalimba, fiddle and pad play from Rust behind `?engine=rust`, null
against JavaScript, and Mikey heard no difference (Firefox, desktop).
These six items bring **every remaining voice** over, behind the same
flag, so the synth runs whole on the core. The effects and the master
chain stay in Web Audio (see "After item 28"). The plan rests on a
read-only survey of `synth.js` (brain, 2026-10-08): which voices share
which building blocks, and every `Math.random` draw. Its findings are
summarized in each item; check them against the code as you go -- the
survey is a map, not a proof.

**Standing rules for items 23-28**, on top of the usual ones and item
21's crate, host, flag and split:

- **The JavaScript synth stays the reference.** With the flag off,
  nothing changes. Every `Math.random` draw stays in JavaScript, in the
  same order and number, including draws made before a budget refusal;
  the values travel with the note. The budget (`_budget`, `_release`,
  costs, timing) stays in JavaScript, unchanged, per part where a voice
  is billed in parts (kalimba's `parts` mask is the pattern).
- **A voice is whole in Rust.** Its constants, waves and bodies live in
  the core, not only in the message, so a native or game host gets the
  same voice. A note that can't go to the core (its channel isn't served
  yet, or the core hasn't loaded) plays in JavaScript, as now.
- **Widen the note message** when a voice needs more than four extra
  values (fm's five options, tubular's five detunes): a fixed, larger
  array, still no allocation after init.
- **The proofs, for every voice the item ports**, on melody and chords
  where the voice plays both: item 21's table (level at `--note 0.4` and
  `1.6` within 0.25 dB; tone bands within 0.5 points; `--endings` no new
  flags), the **null test**, and **nothing else moves** (flag off:
  `measure.mjs` byte-identical to `main` on a fixed set that includes the
  item's voices; flag on: loops without them byte-identical too).
  Refusals on full and lite unchanged, the balance lock untouched, core
  tests and generator tests three times, `tools/build-core.sh --check`.
  Where a voice nulls shallower than -60 dB, say what the residual is.
- **Memory and size.** Report the `.wasm` size and the core's memory
  (static and at peak) after each item. Each band-limited wave is about
  590 KB of tables at Chromium's layout, and this plan adds about ten
  (roughly 6 MB, zeros in the `.wasm`, real memory once built). The
  phone is the target: if the core's memory would pass 16 MB, stop and
  leave a note with the options rather than choosing.
- **Report** as item 21 did, plus the commit preview URL with
  `?engine=rust`, and the Diagnostics `fallback` count over a few loops
  (it falls as items land; say which voices still fall back).
- **Self-merge** stays as the merge policy says. If Claude Code's safety
  check refuses the merge, stop and tell Mikey; he merges.

## 23. Rust core, step 3: the FM voices, sine and tubular (no new DSP)

The core's `Fm` is `fm()` exactly, and eight voices are nothing but
`fm()` calls: keys (a third of all loops, 11% of all notes), bell,
celeste, musicbox, rhodes, marimba, harp (two calls) and piano (two
calls). **Hook `fm()` itself**, as `pad()` is hooked: when the core is
on and the note's channel is one it serves, the note goes to the core
with its five options; each `fm()` call is still budgeted on its own,
so a refused second strike stays refused. That covers every caller on
melody and chords at once. `fm()` calls on bass (rhodesbass) and
texture (bell, chime) keep playing in JavaScript until item 27.

Also: **sine** (one sine, a linear swell, `_release2`) and **tubular**
(five sines, five detune draws sent with the note). Moogpad already
plays through `pad()`; confirm it does.

Survey notes: no voice in this item draws, except tubular's five.
`keys` and `bell` reach `fm()` through `pluck()`'s first branches; the
rest through `voice()`.

## 24. Rust core, step 4: the wave-table voices

Seven voices need only new wave tables, built the way the core already
builds `triangle` and `bowed`:
- Chromium's **built-in sawtooth and square**: softpad (three saws),
  analogpad (three saws), and the square beep (`pluck()`'s default
  branch: `pluck` and any name without its own case), with its constant
  5.4 Hz vibrato on detune.
- The leads' custom waves, `_makeWave(1.5 / 1.8 / 1.7)`: saw, moog,
  analoglead.
- **Whistle** (built-in triangle, which the core has): its pitch scoop
  comes from `_slide`, whose draw happens only when `dur >= 0.18`;
  JavaScript draws and sends whether it slid, from where, and the reach.
  Moog and whistle share one case with an automated lowpass and a
  vibrato depth that ramps in.

Stab waits for the bandpass (item 26). Report the memory per table and
in total (standing rule).

## 25. Rust core, step 5: accordion and nylon, the shared bodies

**First, a harness fix (brain, 2026-10-08).** `measure.mjs --voice
<the 18 voices of items 23-24> --engine rust --note 0.4` runs Chromium
out of memory: the renderer was killed by the kernel at 5.7 GB, after
which the tool waits forever with no output (seen twice). The same probe
on `--engine js`, and on one voice with `--engine rust`, completes; the
`--null` run over the same voices completes too. Suspect: each offline
render builds a core (7.1 MB since item 24) whose memory is never
released. Find where; make the harness release it, and fail loudly when
the page crashes instead of hanging. Then say whether the app can leak
the same way (the synth rebuilt on a quality change, the recording
context, export): `dispose()` sets `alive` false so `process` returns
false, but check it happens.

The core has one body (the fiddle's, per channel). Generalize it to a
body per kind and channel, as `_body()` does in JavaScript, with the
`BODIES` table: nylon and accordion (peaking and lowpass, kinds the core
has).
- **Accordion:** the `reed` wave (`_makeWave(2.0, 0.6)`), two reeds at
  -1.25 and +1.25 cents, the second at 0.35 and started `rand / f`
  seconds late (one draw, after the budget, sent with the note); its
  level differs on chords.
- **Nylon:** `nylonMellow` and `nylonBright` (`_makePluck`), two gains
  with their own decays, and **`damp`**: a strummed chord damps the
  previous strum's strings that are still ringing (`damp()` and
  `_strung`). In the core that is a message that adds a
  `setTargetAtTime(0.0001, time, 0.08)` to notes already queued or
  playing on that channel, in the order JavaScript pushed them (mellow,
  then bright). Find what Chromium does when an event is inserted into
  a timeline that is already rendering, and match it; prove it with a
  null test on strummed nylon chords, not only single notes.

## 26. Rust core, step 6: bandpass, highpass and the noise source

The one big new primitive. Every breath, strike and drum uses
`this.noise`, a two-second buffer filled from `Math.random` in the
Synth's constructor, so Rust cannot make it: **JavaScript hands its
samples to the core once per Synth** (about 384 KB at 48 kHz; size the
buffer for the highest rate the app accepts, and say what that is).
- **Biquad kinds:** bandpass (its Q is linear, unlike the lowpass's dB)
  and highpass, from Chromium's formulas, including its edge cases.
- **A buffer source** that plays the noise as Chromium's
  `AudioBufferSourceNode` does: `start(when, offset, duration)` with the
  duration in buffer time, a constant `playbackRate`, looped and
  unlooped, sub-frame starts, Chromium's interpolation. Read Chromium's
  source rather than guessing. Two users: `_noiseSource` (unlooped, rate
  and offset drawn) and the winds' and singers' looped breath (rate
  drawn, offset 0).
- Prove the source first on the simplest case before any voice: noise
  -> highpass -> gain (the hat's graph, rendered on melody for the test).
- **Voices:** stab (saws, bandpass), ocarina and flute (sine and
  triangle, slide, vibrato, looped breath through a bandpass), panflute
  (the `pipe` wave, a vibrato drawn only when `dur >= 0.5`, breath and
  chiff), prepared's knock (three draws, only after its 3-unit budget)
  and the temple bell (eight sine partials with eight detune draws, then
  the strike's two). Draw orders are in the survey notes below; check
  them in the code.

Survey notes on draws: ocarina and flute, the slide draw (`dur >= 0.18`)
then the breath rate; panflute, the vibrato rate (`dur >= 0.5`) then the
breath rate; templebell, eight detunes (partial by partial, -4 then +4
cents) then rate and offset; prepared, rate, offset, then the knock's
bandpass frequency.

## 26b. The one-frame click in the noise voices (before item 27)

Found by the hands in item 26 (#137) and traced by the brain from the
code (2026-10-08). **A sound fix, in both engines at once.**

- **What happens.** A noise buffer source started at `t` can start one
  frame before its note's gain takes its first value: Chromium rounds a
  buffer source's start to 1/1024 of a frame, but compares automation
  times in frames (`time * rate <= frame`). When `t * rate` lands just
  above a whole frame (by less than about 1/2048), that one frame of
  noise passes through a gain still at its default of **1**: a single
  loud sample (a -33 dBFS click was seen). #137 made the core reproduce
  it, since JavaScript is the reference.
- **How often.** Notes on the bare step grid (bass, texture, melody
  and chord roots when the loop has no choir) land on whole frames
  whenever `rate * 15 / bpm` is whole (bpm 120 at 44.1 kHz on every
  other step, about 24 tempos in 40-190 at 44.1 kHz and 19 at 48 kHz),
  and about half of those carry the tiny positive error. Jittered notes
  (drums, swung steps, strums) hit it about 1 time in 2000. The voices
  exposed are mostly quiet-onset ones, where a tick stands out.
- **Exposed gains** (brain's reading; check each in the code): ocarina
  and flute's air gain, panflute's air gain, prepared's knock, the
  temple bell's strike and its outer gain, the kick's click, snare and
  clap, hat/open hat/shaker, the frame and tap slap, the jingle's outer
  gain, pluckbass's click, the sung voices' breath and their `amp`,
  and the textures drop, wind and waves. Oscillator-fed gains are not
  exposed (oscillators start on the plain ceiling, and their waves start
  at zero); confirm.
- **The fix.** Give each exposed gain its first event's value as its
  intrinsic value when it is made (`g.gain.value = x` before
  `setValueAtTime(x, t)`), so the early frame plays at `x`, not 1. Make
  the matching change in the core (its `reset_at(UNITY, ...)` for those
  gains), so both engines lose the click together and keep nulling.
  Only the clicking frames should change.
- **Proof.** A reproduction first, on `main`: a temple bell and a hat
  struck where `t * rate` sits just above a whole frame (11.3 s at
  44.1 kHz, or a grid note at an integral tempo) render a loud first
  sample in JavaScript; after the fix they don't, on both engines. The
  hat test in `--null` starts on fractional frames and never hits the
  window, so add starts that do. Then: every sample outside the clicking
  frames unchanged (flag off, a corpus of loops, compared with `main`
  within the known summing noise), nulls for the ported voices as deep
  as before, refusals and the balance lock untouched. Count how many
  notes in a few hundred loops landed in the window before the fix.
  Version bump: it changes what the app plays, if only by a sample.
- For Mikey's ears: nothing to listen for but the absence of rare ticks.

## 27. Rust core, step 7: bass, textures and drums (five channels)

Three more outputs, so the core serves all five channels: **bass,
texture, drums** next to melody and chords, each connected into its
channel's gain as the first two are (ducking, sends and mutes stay in
Web Audio). Split it **27a** (bass and textures) and **27b** (drums) if
that is cleaner. Start from 26b's fixed JavaScript, so the drums and
textures are ported without the click.
- **Bass:** sub and moogbass (sawtooth through an automated resonant
  lowpass, from item 24), round and pluckbass (triangle, lowpass;
  pluckbass's click from item 26), fifths (two sines), rhodesbass
  (`fm()`, from item 23). Glide, chug's shorter release, and each
  voice's trim. Only pluckbass draws (its click's two).
- **Textures:** swell (sines, one budget check per note; the first
  refusal drops the rest of the call), bell and chime (`fm()`), drop
  (noise, bandpass at a drawn frequency), wind (looped noise, a bandpass
  whose frequency a 0.07-0.13 Hz sine moves at audio rate) and waves
  (looped noise, a drawn rate and offset, two lowpasses in series).
- **Drums:** kick and softkick, snare and clap, hat, open hat and shaker,
  rim, and the hand kit (frame, tap, jingle, open jingle). They are the
  most notes per second of anything in the app, so this item also runs
  **`perf.mjs --ab`** (flag off against on, loops with full kits, the
  usual throttling): audio health must not get worse; report audio- and
  main-thread cost, graph churn and memory, as item 22 did.

## 27c. Three small fixes from the brain's check of 26b and 27 (before item 28)

Found 2026-10-09 by the brain's review and reruns. One PR, a version
bump (the first changes what the app plays).

- **26b's setter and late notes (JavaScript, flag off and on).** The
  Web Audio `value` setter is `setValueAtTime(value, currentTime)`. For
  a note built after its own start time (the engine plays notes up to
  about 0.25 s late, e.g. after a hidden tab or a stall), that event
  lands *after* the note's envelope events: the gain jumps back to its
  first value when the envelope has already moved on (most are 0.0001,
  which would cut the note; the kick's click would hold at `v * 0.28`).
  Offline renders never build a note late, so 26b's proofs couldn't see
  it. Reproduce first (a note built behind `currentTime`, in an offline
  render through `suspend()`, before and after 26b), then fix every
  setter 26b added: e.g. `setValueAtTime(x, Math.min(time,
  ctx.currentTime))`, which is 26b's fix when on time and a no-op when
  late. Check the core does what the fixed JavaScript does for a late
  note (it starts late notes at once).
- **The jingle's draws on a fallback (flag on).** `_coreDrum` draws the
  jingle's five zil detunes before `_coreNoiseChannel` can refuse; on a
  fallback the JavaScript jingle draws them again, 12 draws where the
  rule is 7. Check the channel first, or hand the JS path the drawn
  values. Check the other drums for the same pattern.
- **Pan flute grace notes (flag on).** `--null --voice panflute` on v75:
  the 0.1 s and "repeated" rows show a worst sample of -68.5 dBFS (worst
  note -80.8 dB); in the brain's check of item 26 (v72) the same rows
  were -125 dBFS. Something in 26b or 27a moved it, likely the air
  gain's new first value or the frame-based event compare. Find which,
  make it null again, and say why the other voices didn't move.

## 28. Rust core, step 8: the sung voices (vowel, hum, choir)

The hardest voices, last. Each singer's detune is a ramp, then a jitter
of many small ramps (two draws per step, about one step per 0.13 s),
**plus** a vibrato oscillator summed into the same detune; three peaking
formant filters whose frequency, Q and gain all ramp when the vowel
drifts; a lowpass; and a breath through a bandpass.
- **Param capacity:** `MAX_EVENTS` is 8 and a long sung note needs
  dozens. Measure the longest note a sung voice gets across the corpus
  and size it from that, with a margin; still no allocation after init.
- **The draw order is intricate** and partly before the budget: whether
  it drifts and where to (both drawn before the budget check), then per
  singer the choir spread, the scoop's start and end, the jitter steps,
  the vibrato's three, and last the breath rate. Read `voice()`'s sung
  case closely; a single misplaced draw shifts every later note.
- The `glottal` wave, `_formantTrim` (computed in JavaScript and sent,
  or ported: say which), choir's three singers into one tract.

Start from 27c's fixed code.

**After item 28, stop.** Every voice plays from Rust behind the flag.
The brain verifies, Mikey listens to one labeled album across the new
voices, and then he decides what is next: making the core the default,
moving the effects and master chain into Rust (needed for a native or
game host, not for the phone's web app), and the generator's port after
the words.

## Rust core, steps 9-11: the effects and the master chain (items 29-31)

Written 2026-10-09 (brain, Session 5), at Mikey's word: after item 28
every voice plays from the core; now **the rest of the sound path moves
too**, so the core makes the whole mix and a native or game host needs
nothing from Web Audio but an output. Same flag (`?engine=rust`), same
standing rules as items 23-28 (JavaScript is the reference; flag off,
nothing changes; every check, every item), plus these:

- **The graph, as it is today** (`_build()` in `synth.js`; read it, this
  is a map): five channel gains (drums .82, bass .62, chords .44, melody
  .58, texture .50), each with a reverb send and an echo send taken from
  the channel gain itself; drums go straight to `preBus`, the other four
  through `pumpBus` (the kick's duck). Reverb: `reverbIn` -> six combs on
  full, three on lite (each a delay with a lowpass in its feedback, the
  sum taken *before* the lowpass) -> `combSum` -> `reverbOut` ->
  `preDelay` -> `tails` -> `preBus`. Echo: a delay with a lowpass and a
  feedback gain, out through `tails`. Master: `preBus` -> `wobble` (a
  delay whose time two always-running sine LFOs move, wow and flutter) ->
  `sat` (a WaveShaper, a `tanh` curve, 2x oversampling on full, none on
  lite) -> `tone` lowpass -> `hp` highpass -> `comp` (a
  DynamicsCompressor) -> `master` gain -> `kill` gain -> `ceiling` (a
  second DynamicsCompressor, the limiter) -> the destination. Runtime
  changes: `setTone`, `fadeTails`/`restoreTails`, `duck`,
  `setEchoTime`, `setMute`, `silence`/`unsilence`, `setVolume`,
  `setCharacterLevel`. `setTone` replaces the saturator's curve outright.
- **Notes that still play in JavaScript must keep reaching the effects.**
  A fallback note (an unserved channel, a note before the core loads, a
  sung note too long for its detune) plays through Web Audio nodes; once
  the effects live in the core, those nodes need a way in. Give the
  worklet node one input per channel, and connect each channel's JS path
  into it, so a fallback note gets the same sends, duck and master as a
  core note. If the core fails to load, the whole JavaScript graph plays,
  as now.
- **Cycles.** The echo and the combs are feedback loops; Web Audio clamps
  a delay inside a cycle to at least one render quantum (128 frames).
  Match what Chromium does, not what the spec suggests, and prove it.
- **Proofs move from notes to the mix.** Each item adds a `--null` stage
  for what it ports: the same input through the JavaScript stage and the
  core's, subtracted, with signals that exercise it (impulses, swept
  sines, noise, real loops; level steps for the compressors; every
  runtime change at an awkward time). Whole loops: the mix against
  JavaScript, next to the JS-against-JS floor (about -86 dB). Where a
  stage can't null to the floor, say what the residual is and how loud,
  in dBFS and against the signal, before going on.
- **Performance** each item: `perf.mjs --ab`, flag off against on, on
  loops with full kits and long reverb, the usual throttling, on full
  and lite. The native Web Audio effects are fast C++; the core must not
  make audio health worse (late ticks, fill-ins). If it does, stop and
  leave a note with the figures rather than optimizing past the brief.
- **Stereo.** Today the graph is mono end to end, as far as the brain can
  see; confirm it, and keep it so.

## 29. Rust core, step 9: channels, sends, duck, echo and reverb

The mixing stage, up to `preBus`. The core sums its own voices into the
five channels as now, plus the worklet's five inputs (the JS fallback
notes); applies channel gains and mutes, the two sends, the duck on
`pumpBus`, the echo (delay, lowpass, feedback, `setEchoTime`), the
reverb (the combs on full and lite, `combSum`, `reverbOut`, `preDelay`)
and `tails` (`fadeTails`/`restoreTails`), and outputs `preBus` to one
node that feeds the JavaScript master chain (`wobble` onward), which
stays as it is for now. **DelayNode** as Chromium renders it: its
interpolation, its smoothing of `delayTime` changes, and the cycle
clamp. `setTone`'s reverb and comb changes go to the core as messages,
with the same `setTargetAtTime` curves.

## 30. Rust core, step 10: wobble, saturator, tone and highpass

The first half of the master chain. **Wobble:** a delay whose time is
0.014 s plus two sine LFOs (wow 0.32 Hz, flutter 6.3 Hz) at audio rate,
the LFOs started when the synth is built (find their phase on a real
context, where the synth is built mid-stream). **Saturator:** the
WaveShaper's curve lookup as Chromium does it (its index mapping and
interpolation), with `oversample` 'none' on lite and '2x' on full:
Chromium's up- and downsampling filters, read from its source, not
approximated. `setTone` swaps the curve outright: match when the new
curve takes effect. **Tone** (lowpass, `setTone` moves it) and **hp**
(highpass, 38 Hz): the core has both kinds. The core then outputs the
signal before `comp`.

## 31. Rust core, step 11: the compressors and the whole mix in the core

The last of the graph. **`comp` and `ceiling`:** Chromium's
DynamicsCompressor (its kernel: the knee curve, attack, the adaptive
release, its lookahead pre-delay, its makeup gain, its metering),
ported from Chromium's source (write it, and the stages before it, so
that a stereo output is a later change, not a rewrite: the ears reports
put stereo first among the next sound items; Chromium's compressor links
its channels' detection), then `master` (`setVolume`,
`setCharacterLevel`) and `kill` (`silence`, `unsilence`). The worklet's
output goes straight to the destination. Performance A/Bs from here on
alternate which build runs first (the sandbox VM freezes now and then,
and slot order showed in item 30's first A/Bs); trace any long quantum
(`perf.mjs --trace-dir/--trace-over`) before blaming code. Prove both
compressors with
level steps, bursts, and loops at the catalog's loudest (the loudest
loops reach about 0.85 peak; find a few that push `ceiling`), and the
whole app path: play, stop, re-roll, "let the loop wander", the hidden
tab, a quality change (which rebuilds the synth), and `dispose()`. With
the flag on, Web Audio then carries only the fallback notes and the
output.

**After item 31, stop.** The brain verifies; Mikey hears one more labeled
album (the mix, on his phone); then he decides about making the core the
default, and the generator's port (which waits on the words, his
2026-10-03 decision, unless he changes it).

## 31b. Hardening the core path (from the brain's check of items 30-31)

Found 2026-10-10 by the brain's review; none is a bug seen in a render,
all are paths a phone could take. One PR, flag on only (flag off must
not change), the usual checks.

- **A core that fails while playing.** `core.js` sets no
  `onprocessorerror`, and the worklet only reports `failed` from its
  constructor; with `panic = "abort"`, a trap in `process` would leave
  the node silent for good, the JavaScript chain already cut, and
  Diagnostics reading healthy. On a processor error, fall back to the
  full JavaScript graph (`_mixInJs` already does this for a load
  failure) and show `rust (failed: ...)`. Prove it with a test-only
  trap.
- **A core that fails to load** (`loadCore` rejects, or the node's
  constructor throws): JavaScript plays correctly, but Diagnostics shows
  `rust (loading)` forever. Show `failed`.
- **#150's frame count can only move forward.** If Chromium ever called
  `process()` more often than real time rather than repeating a stale
  `currentFrame`, `next` would drift ahead and never come back. Trust
  `currentFrame` again once `next` is more than a block or two ahead.
- **The LFO catch-up** (`master.rs`, `while lfo_block < block`) runs in
  one `process()` call from `_lfoStart` to the first block the core
  renders: on a slow phone that loads the core 30 s in, about 1.3
  million steps in one block. Compute the LFOs' phase at that block
  directly instead of stepping to it, and null against the stepped
  version.
- **The handover re-glides** the tone and the wobble depths (replayed
  with `setTargetAtTime` from the core's defaults). Set the core's
  starting values at attach instantly, then glide only real changes.
- **Warm the core's compressors** (Mikey, 2026-10-10: the engine is ours
  now, not a copy of Chromium; BRAIN.md, "The engine"). Start both
  detectors as if the music had been playing, so first Play, a rebuild
  and the core's arrival don't dip the first notes by up to 10 dB. The
  JavaScript engine keeps its cold start; the nulls will differ in the
  first second, and that's expected (say by how much, and that after it
  they meet the floor again).

## 32. The core by default (the flag's last step)

Mikey heard "Rust, all voices" on his phone (2026-10-11): "it sounded
great!", and the brain verified item 31b. The Rust core becomes
Driftloom's engine. One PR (split into a/b if cleaner), next version.

- **The core plays by default.** No flag needed; `?engine=js` asks for
  the JavaScript synth (for measurement and comparison), and old
  `?engine=rust` links keep working. The JavaScript synth stays, only as
  the fallback: a browser without AudioWorklet or WebAssembly, a core
  that fails to load, a core that traps (31b's paths). Diagnostics says
  which engine plays and why: `rust ...`, `js (fallback: <reason>)`, or
  `js (asked)`.
- **The first note.** Load the core as early as the browser allows, so
  that first Play almost always starts on the core. Measure, on the test
  machine and with a slowed CPU, how long the core takes to be ready
  after the page loads and after the first tap. If Play comes first, the
  JavaScript synth plays and hands over as now; add a short wait before
  first Play only if the measurements show it is short and it removes a
  handover in most starts. Report the figures either way.
- **The regression guard.** Nulling against JavaScript stops being the
  gate for core changes. In its place: a committed **core baseline** of
  the core's render fingerprints (31b's) for a fixed set of loops, full
  and lite, beside the measured figures that matter (level per voice and
  layer, cost, the tone probe, refusals), and a check that fails on any
  difference. A PR that changes the sound on purpose updates the
  baseline in the same PR and says what moved and why. Put the check in
  CI if headless Chromium runs there cheaply; otherwise make it a
  standing rule every item runs. Update the standing rules to match.
- **The tools default to the core** (`measure.mjs`, `perf.mjs`,
  `core-paths.mjs`), with `--engine js` for the JavaScript synth. The
  fallback is what a phone without the core hears, so the JavaScript
  synth keeps passing its own checks (refusals, `--clicks`,
  `--endings`); `--null` stays available as a tool.
- **Follow-ups from the brain's check of 31b:**
  - `CoreHost.dispose()` leaves `onprocessorerror` and `onfail` set: a
    processor error queued just before a rebuild could run `_mixInJs` on
    the disposed synth and reconnect its chain. Clear both on dispose.
  - After a core traps, stay on JavaScript for the rest of the session:
    a quality toggle should not rebuild a core that failed, and
    Diagnostics keeps saying why.
  - The frame count snaps back to `currentFrame` after three stale
    frames, which hands the core a block that goes backward
    (`self.next = end`, `lib.rs`). Trace what the core's voices and
    timelines do with that; make it ride out cleanly if they don't.
  - The flutter's catch-up error is understated in `osc.rs` and
    `NOTES.md` (the brain measured about 7e-12 table samples after
    1,000 blocks and 5.4e-9 after 1.24 million, for a start between
    frames). Correct the figures; the tolerance is fine.
  - `measure.mjs` says its figures repeat to a thousandth of a dB, but
    the JavaScript-vs-JavaScript floor moved by up to 2.8 dB between two
    identical runs (the core's renders were identical). Find why, or fix
    the claim.
- **Performance.** A/B the new default against v82 with `?engine=rust`
  (should be the same) and against v82 without it (the JavaScript synth,
  what testers hear today): audio render, main-thread time, fill-ins,
  full and lite, counterbalanced. 31b added an unexplained +3% render on
  full: take one look with the trace tools and report; don't chase it
  past the brief.
- **Mono stays** (Mikey, 2026-10-11): one output channel.
- **Docs:** the README intro, the `core.js` header, `NOTES.md`, and the
  Done line.

## 33. More words on the ranking page (`/rank`)

The words (roadmap 19) wait on Mikey's rankings, and he wants to rank
hundreds of loops in one go so the next brain session has data to fit.
A small item: data in `rank.html`, no generator or synth change.

- **Add the candidate words Mikey names** in the session's opening
  message, from `docs/MOODS.md`, "Candidate words". Add only the ones he
  names; if he names none, stop and ask. Each goes in the group the
  brain assigned (one group per map, as `GROUPS` explains):
  - color: bright, sunny, sunlit, moonlit, starry, twilight, dusky,
    hopeful, magical, enchanted, wondrous, triumphant, wistful,
    nostalgic, bittersweet
  - shimmer: tinkling, chiming, hazy
  - air: hushed, snug, homey, sleepy, drowsy, lulling, tranquil,
    reverent, meditative
  - motion: lilting, jaunty, sprightly, nimble, skipping, tiptoeing,
    hypnotic, adventurous
  - flavor: tangy, zesty, tart, mischievous, cheeky, plucky, mysterious
- **Bump `POOL_VERSION` to 3**, so the saved answers say which pool a
  trial came from. Saved answers from pools 1 and 2 must still load,
  export and count toward the balance (a new word is shown more often
  until it catches up; check that this still holds).
- **Report the balance:** with the final pool, how many trials until
  every word has been shown at least 5 times, and at least 10 (simulate
  from Mikey's answers so far being empty, and from 20 pool-1 answers).
  A big group spreads its words thin, so say which group is slowest.
- **Keep the trial as it is** (five words, one per group, the hidden
  repeats). Don't change the loops (`START_SEED`, `newSpec`).
- **Checks:** the page loads and plays a trial (core and `?engine=js`),
  Export writes the new words and the pool, the tests pass. Bump BUILD
  and CACHE. Add the new words to `docs/MOODS.md` under "Ranking
  results" as pool 3.

## 47. Urgent: glitches when minimized or locked (Pixel 9, GrapheneOS, v92)

**App track, emergency** (Mikey, 2026-10-13). Hands: **Opus 5.5, high**.
Stay out of `core/` (the core track's item 43 is in progress there): the
fix belongs in the page's scheduling (`js/engine.js`, `js/synth.js`,
`js/core.js`, `js/worklet.js`). If it truly needs a core change, stop
and leave a note.

The first tester's phone (Pixel 9, GrapheneOS, installed app, Chrome
155) still glitches when the app is minimized and when the screen is
locked. His Diagnostics on v92, 3.9 minutes, the figures that matter:

- `engine: rust late: 10 (stall 0, inside the buffer 10; left out 10)`
- `renderAheadMs: 21 (measured 6)`, but `worstRenderAheadMs: 1413` and
  `leastSpareMs: -1333`
- `outputLatency: 0.272`, `measuredLatencyMs: 600`, `baseLatency: 0.0213`
- `lookahead: 2`, `redecides: 0`, `lateTicks: 0`, `recentStallMs: 51`
- While hidden (the last time only): 15.5 s, worst tick gap 252 ms,
  **0 late notes**, audio and media running throughout.
- Every view on screen: 0 late notes, 0 dropped.

**What the brain reads in it (unproven; check each before relying on
it):**
1. **No stall, all ten late notes "inside the buffer".** The page was on
   time; the core had already rendered past the notes' start times. The
   render-ahead is usually 6-21 ms on this phone but reached **1.4 s** at
   least once. The likely moment: Android moves a backgrounded or
   screen-off app's audio to a deep, power-saving output buffer, so the
   worklet suddenly renders over a second ahead in bursts.
2. **The planner adapts only after it measures.** `engine.ahead` is the
   latest measured render-ahead, not the worst seen, so right after the
   jump, notes are still planned with tens of milliseconds to spare, land
   inside the new buffer and are left out (item 41's rule): missing notes,
   which is the glitch. Notes decided close to now are the ones at risk:
   look at what decides notes near the clock (a new loop or pass starting,
   "Let the loop wander", Next/Previous, media keys) rather than the
   steady lookahead.
3. **The hidden report can't see it.** It keeps only the last hidden
   spell, and the late notes were not in it; they may fall at the moment
   of hiding or showing, which neither section covers.

**What:**
- **Find the moment first, by evidence.** Add to Diagnostics a short log
  of the render-ahead's jumps and of each late note: when (time since the
  page began), whether hidden, visible or within a second of a change,
  and what decided it (lookahead tick, new loop/pass, user action). If
  the harness can model a deep buffer that appears on hide (as
  `tools/views.mjs --buffer` models a fixed one), reproduce it there.
- **Plan for the deep buffer before it bites:** what the evidence calls
  for, likely both of these:
  - remember the **worst** render-ahead seen this session (and per
    device across sessions, in storage), and plan every note, including
    the ones decided near the clock, at least that far ahead plus the
    margin;
  - when the page is hidden (and on `pagehide` / `freeze`), assume a deep
    buffer at once rather than waiting to measure it.
  The cost is a little latency on a mute, re-roll or tempo change while
  hidden, which nobody can see; keep visible responsiveness as item 41
  left it.
- Keep item 41's rule (a note more than 30 ms late is left out, not
  bunched).

**Yardsticks:**
- In the harness, with a deep buffer that appears on hide and leaves on
  show: zero late notes across hide, lock-like freeze, show, new loops,
  passes and Next/Previous while hidden.
- Visible behavior unchanged on an ordinary phone: the latency of a mute
  or re-roll as item 41 measured it.
- Steps for the tester in the PR (what to do, what to copy), so his next
  Diagnostics answer whether anything is left.
- Bump `BUILD` and `CACHE`; the usual tests and checks.

## The core, phase 1: foundations (items 42-46)

**Read `docs/CORE.md` first** (the charter, approved by Mikey on
2026-10-13). It is the reason for every item below: ten promises, each
with a measure. Phase 1 lays the ground the rest stands on: the same code
makes **the same samples, bit for bit, on every device**; the core runs
without the standard library; it takes any sample rate and block size;
and its building blocks sit behind one small interface anyone can extend.

Nothing in phase 1 is meant to change how Driftloom sounds to a
listener. Items 43 and 44 change samples (by rounding, and by moving the
random draws), so they update the core baseline and say what moved; the
brain builds a control album from them for Mikey's ears ("does it still
sound the same?"). The other items must leave `tools/baseline.mjs`
passing unchanged.

**Standing rules for core items** (in addition to the rules above; from
the charter's promises 2 and 6, held on every PR from here on):

- Nothing allocates, locks or frees after `Core::new`; nothing reachable
  from rendering can panic. Say how you checked.
- Every public item the PR touches is documented, with the reason, not
  only the what. `cargo clippy -- -W clippy::pedantic` is clean in the
  files the PR touches (allow a lint locally, with a one-line reason,
  only where following it would make the code worse).
- No dependencies in `core/` (dev-only tools in CI are fine).
- Everything the core emits (names, errors, logs) is plain ASCII.
- Costs: the core baseline's render costs may not rise by more than 5%
  on any loop without a note saying why it is worth it.
- The JavaScript synth is the fallback until the end of phase 3: leave it
  working, untouched unless an item says otherwise.
- The core is no longer a copy of Chromium. Where a doc comment says a
  block "follows Chromium to the sample", leave it if still true, but
  new work is judged by the charter's measures, not by nulling.

Hands, one fresh Claude Code session per model (Mikey's tokens):
**42 Sonnet 5.5, high** (Session 19: a clear spec, tooling and CI);
**43 Opus 5.5, high** (Session 20: numeric code, accuracy, denormals);
**44 and 45 Sonnet 5.5, high** (Session 21: clear specs with strict
yardsticks; if a yardstick won't pass, stop and leave a note rather than
push on); then a brain check (Session 22); **46 Opus 5.5, high**
(Session 23: an interface design); then the brain's phase review
(Session 24).
**Then, before phase 2 (Mikey, 2026-10-15):** an instrument batch
written by Session 24 (new sounds from the ears, core-only, on item 46's
interface), built by Session 25 (Opus 5.5, high), checked and heard in
Session 26; phase 2 starts with Session 27. New profiles come after
phase 2, as data. See `docs/BRAIN.md`, "The plan to phase 2".
Items in order; each a PR (a/b splits welcome).

**Brain check, Session 22 (2026-10-18): items 42-45 pass their
yardsticks** (`docs/BRAIN.md`, the log); Mikey heard the control album:
no difference. **Next: item 46.** For 46, one thing from the check: the files
it moves carry old pedantic clippy warnings; clean the ones in code the
refactor rewrites, and say how many are left in the PR, rather than
leaving the standing rule unmet without a word.

**Brain check, Session 24 (2026-10-19): item 46 (#238) passes its
yardsticks, so phase 1 is done.** Run by the brain on `main` (7f0e6af), not
taken from the report: `build-core.sh --check`; `cargo test --release` (the
kit with its cost check); goldens `--strict` on x86-64 and wasm32,
`--strict --flush`, `--blocks all` on both, `--rates` on both: every hash
equal, and #238 touched no hash file and not `test/core-baseline.json`;
`node tools/baseline.mjs`: nothing moved; instruction counts (cachegrind)
against #237 on busy-cinder -0.37%, 09-shrine -0.05%, 15-wayfare -0.11%,
07-dust +0.34%; no panic, trap or allocation in the release library's
assembly; "one file plus one line" checked independently (a throwaway
block added in a scratch copy: the kit found it, asked for its stored
render, then passed, and flagged a 2% gain change as a sound change).
Pedantic clippy: 405 warnings left, in files 46 didn't rewrite (item 74,
low priority). **Next: item 46b, then the sound batch (items 54-73),
below item 46.**

## 42. Golden renders on every target

**Why:** promise 3 says the same code makes the same samples on every
device. Before making that true, measure it: a set of renders whose
hashes CI checks on several kinds of machine.

**What:**
- **A note-list file** (plain text or a small documented binary; your
  call, say why): everything the core needs to render a loop with no
  JavaScript (the notes with their draws, the parameters, the sample
  rate, the length). A tool exports note lists from a fixed set of codes
  through today's generator and host: about 12 loops covering every
  voice, every channel and the master chain, plus one long busy loop.
  Commit them under `core/golden/`.
- **A native test host:** a small Rust program in `core/` (an example or
  a binary target, no dependencies) that reads a note list, renders it
  through `Core` and writes a WAV and a hash (a simple documented hash of
  the raw samples; no crates). This is also the seed of the WAV export
  (roadmap 17) and of every later native host.
- **Golden hashes** in `core/golden/`, one per note list, and a check
  (`cargo test` or a script) that renders them all and compares.
- **CI on several targets**, each rendering the goldens and comparing:
  `wasm32-unknown-unknown` (run in Node, through the same exports the
  worklet uses), `x86_64-unknown-linux-gnu`, and under QEMU user mode
  `aarch64-unknown-linux-gnu`, `armv7-unknown-linux-gnueabihf` and a
  32-bit big-endian target (`powerpc-unknown-linux-gnu`, standing in for
  a GameCube-style port). Keep the job's wall time sensible; say what it
  costs.

**Yardsticks:**
- On each target, rendering twice gives the same hash.
- **Report, don't fix yet:** which targets match the wasm hashes and
  which don't, and for the ones that don't, the largest sample
  difference and where it comes from (expected: the platforms' own
  `sin`, `exp`, `powf`... item 43 removes them). Make cross-target
  equality a CI failure only from item 43 on; until then CI checks each
  target against its own golden.
- The app is untouched; `tools/baseline.mjs` passes unchanged.

## 43. The core's own math, and no standard library

**Why:** platforms compute `sin`, `exp`, `powf`, `tanh`, `log10`... each
their own way, so the same code can't make the same samples everywhere.
The charter's answer (Mikey: yes, bit-identical everywhere): the core
carries its own math. The same change lets the core build with
`#![no_std]`, which is what lets it run on a console, an old handheld or
a microcontroller.

**What:**
- A `math` module: every transcendental the core uses (today: `sin`,
  `cos`, `exp`, `exp2`, `powf`, `log10`, `log2`, `tanh`; check with a
  grep, there may be more) implemented in plain Rust from `+ - * /`,
  comparisons and bit manipulation only, so IEEE rounding makes them
  identical on every target. `sqrt` is exact in IEEE and may stay as the
  hardware op if it stays identical in the goldens.
  - Accuracy: say what each function achieves (max error in ulps over
    its range, measured against a high-precision reference in a test),
    and where audio allows a cheaper version, say so and why. The aim is
    not "as fast as possible" or "as exact as possible" but well
    reasoned: exact where errors accumulate (phase, frequency), cheap
    where nobody can hear it.
  - No fused multiply-add anywhere (Rust doesn't fuse unless asked;
    keep it that way and write it down).
- **Denormals**, handled explicitly: decide how the core keeps
  recursive filters, envelopes and reverbs out of subnormal numbers
  (some ARM and old hardware flush them to zero, others are very slow on
  them), so results and costs match everywhere. Say what you chose.
- `#![no_std]` for the library, with `alloc` only if `Core::new` still
  needs it (allocation at construction only; document it). The wasm
  build and the native test host keep working. Note in the PR what a
  host on a machine with no allocator would need (for later; don't
  build it).
- From this item on, **cross-target equality is a CI failure**: every
  target's hashes equal the wasm hashes.

**Yardsticks:**
- All goldens identical on all five targets.
- The sound barely moves: report each golden's largest difference and
  its residual against the old render (expected far below hearing, on
  the order of -100 dB or lower; if any is not, stop and leave a note).
  Update the core baseline in the same PR and say what moved.
- Costs within the 5% rule; the `.wasm` size reported (159 KB before).

## 44. The randomness moves into the core

**Why:** promise 3. Today dozens of small articulation draws (strum
spread, timing jitter, slides, the ensemble spread, the noise buffer,
bell detunes, reed delays, source rates and offsets) come from
`Math.random` in `js/synth.js` and `js/engine.js` and travel with each
note. So the same code never makes the same audio twice, and a native
or game host would have to invent them.

**What:**
- One small seeded generator in the core (documented, deterministic,
  cheap; no crates). Each draw comes from the loop's seed, the note's
  identity (where it sits in the loop and which pass of the loop it is,
  so "Let the loop wander" still varies pass to pass, reproducibly) and
  a fixed salt per kind of draw, so adding a new kind of draw later
  never shifts the others.
- The noise buffer: generated in the core from a fixed seed, not
  handed over from JavaScript.
- The host hands the core the seeds; the notes stop carrying draws.
  Update the note-list format and the goldens.
- The JavaScript synth keeps its own `Math.random` draws (it is the
  fallback and retires at the end of phase 3); don't spend work on it.
- List every draw you moved in the PR, with where it came from and its
  range, so the brain can check none was missed or changed in range.

**Yardsticks:**
- Playing the same code twice in the app (core path) gives identical
  audio: add this to `tools/baseline.mjs` (render twice, compare).
- Each moved draw keeps its range and distribution: a test draws many
  and compares mean, spread and range against the old formula.
- Goldens identical across all targets.
- Update the core baseline; the brain builds the control album from the
  PR's list.

## 45. Any sample rate, any block size, any channel layout

**Why:** promise 4. A game or an operating system asks for its own rate
(22,050 to 192,000 Hz) and its own block size (an audio callback may ask
for 441 or 1,024 frames, or a different number every time). Today the
core runs on Web Audio's 128-frame quantum. And promise 9: the core
should render to whatever channel layout the host asks for, so stereo
can come later without rebuilding it.

**What:**
- The host asks for any number of frames per call; the core may still
  work internally in fixed blocks (128 is fine) and buffer between them.
  Note start times stay sample-accurate whatever the host's block size.
- Every rate-dependent piece (filter coefficients, delay lengths,
  envelopes, the reverb, LFOs, oversampling) derives from the rate given
  at `Core::new`; no hidden 44.1 or 48 kHz constants. Grep and say what
  you found.
- **Output layout** chosen by the host at `Core::new`: mono is the only
  layout implemented; asking for another returns a clear "not supported
  yet" (plain ASCII), not a panic. Shape the internals so a stereo
  layout can be added later as a new render, not as a mono mix spread
  across two channels (promise 9: the mono render pays nothing for
  stereo existing). Build nothing toward stereo beyond that seam.
- The web host is unchanged in behavior.

**Yardsticks:**
- **Block-size independence:** each golden rendered with host blocks of
  1, 64, 128, 441, 1024 frames, and a random size every call, gives
  identical hashes.
- **Rate:** goldens at 48 kHz unchanged; new goldens at 22,050, 44,100,
  96,000 and 192,000 Hz, each checked for level (within 0.2 dB of 48
  kHz), tone (the 2-5 kHz share within the tone probe's tolerance) and
  no aliasing worse than at 48 kHz. Say how you measured.
- `tools/baseline.mjs` passes unchanged.

**Decisions on #213's stop (Mikey, 2026-10-14):**
1. **Tone tolerance:** at 44,100 and 22,050 Hz the 2-5 kHz share may
   differ from 48 kHz by up to **1.5 points** (0.5 stays at 96 and 192
   kHz). No per-rate trims. Put `tools/rates.mjs` in CI with that
   tolerance. Mikey's ears decide in the control album if anything is
   heard.
2. **The noise's color: make it rate-true** (the version written for
   #213, exactly 1 at 48 kHz so the goldens don't move). Run
   `node tools/baseline.mjs --update` in the same PR and say in the body
   what moved at 44.1 kHz and by how much.
3. **192 kHz stays a feature (`wide`):** on for native builds, off for
   the web build (which keeps its 96 kHz limit and 14.75 MB). If a
   browser's context runs above 96 kHz, the web host asks for a 48 kHz
   context instead (or falls back as today); say which in the PR.
4. **CI cost:** the PowerPC job renders the plain goldens only; block
   sizes and rates run on x86-64, wasm32, aarch64 and armv7.
Then finish item 45 and merge when CI is green.

## 46. The building-block interface

**Why:** promise 5. Mikey (2026-10-13): developers, AI agents and he
himself should be able to add building blocks, or make fundamental
changes to one, easily, without understanding the whole core. This is
also the ground phase 5 (instruments as recipes) builds on.

**What:**
- Read the core and propose the interface first, in the PR body before
  the code: what a block is (an oscillator, a filter, an envelope, the
  noise, the formants, the reed, the bell body, the compressor...), what
  it takes in, puts out and is set by, how its state is held, how the
  core finds it (one registry list). Fast: no dynamic dispatch on the
  per-sample path unless measured to cost nothing; no allocation. Keep
  it small; two or three concepts, not ten.
- Move today's blocks behind it. Voices become compositions of blocks
  where that is natural; where a voice's insides don't split cleanly,
  say so rather than force it (phase 5 decides the recipe format).
- **A test kit every block gets for free:** level, cost, tone (the 2-5
  kHz share), determinism (twice, same hash) and a null test against a
  stored render of the block's previous version, so a change is either
  proven identical or reported as a sound change for an A/B.
- **Prove "one file plus one line":** add a trivial demonstration block
  in the tests (not in the app) that needs exactly one new file and one
  line in the registry, and gets the kit's tests automatically.
- `core/README.md`, first version: what the core is, the host calls,
  how to add a building block (the walk-through the demonstration block
  proves). Short; for developers and AI agents.
- Update the header of `lib.rs`: the core is Driftloom's own engine;
  the JavaScript synth is a fallback, no longer the reference.

**Yardsticks:**
- Goldens and `tools/baseline.mjs` unchanged, byte for byte: this is a
  refactor.
- Costs within the 5% rule (dispatch must not cost).
- The demonstration block: one file, one line, the kit runs on it.
- Leave a note for the brain: which of today's blocks felt awkward
  behind the interface, and what a block catalogue (charter, promise 5)
  should consider.

## 46b. The interface, finished for the batch: modulation, stereo, channel events

**Hands: Session 25-0, Opus 5.5, high** (an interface design, like 46).
Written by the brain, Session 24 (2026-10-19), at Mikey's request.

**Why:** item 46's notes for the brain ("Notes from Claude Code", item 46)
name what doesn't fit the interface yet, and the sound batch below needs
three of those things eight times over: padSweep and the arc are slow
oscillators moving a cutoff, glass is FM, the stereo render needs to know
what a block is when there are two channels, the organ is retuned, held
and let go chord by chord, the tuned kick chokes its own last hit, and
nylon's damp is the one place the core still names a voice. Deciding each
once, here, keeps the batch from inventing it eight ways. Like 46, a
refactor: nothing anyone can hear changes.

**What** -- propose all three in the PR body before the code, as 46 did;
keep the interface small (46's three concepts may grow by one or two, not
by ten):

1. **A modulation input.** A way to drive a block's setting with a
   signal, as Web Audio connects a node into an `AudioParam`.
   - A block names its inputs (`"freq"`, `"cutoff"`, `"gain"`...), so a
     recipe (phase 5) can wire them by name; nothing positional.
   - Say what rate a signal drives a setting at: every frame (FM needs
     it), once a block (enough for a slow sweep, and cheaper), or the
     block's choice. A setting with nothing wired reads its value or its
     `Param` exactly as today.
   - A control source is a block like any other (an LFO is today's sine
     `TableOsc`); the envelope stays a `Param` (data), as 46 found.
   - Static dispatch, no allocation, no `dyn` on the per-sample path.
   - Use it inside today's voices only where the samples stay identical:
     at least FM's modulator into the carrier's frequency and one
     vibrato, or say why not. List the places left as they are and why.
2. **The stereo signal shape.** What a block takes and gives when the
   render is stereo, chosen now so items 64-65 build the stereo render on
   it. Your design (a second signal, a stereo variant of the trait, a
   channel count in the type...), held to promise 9:
   - **The mono render pays nothing:** no per-sample branch, mono goldens
     and their costs unchanged.
   - A mono block in a stereo render runs once, not once a side; width is
     made by processors that take mono and give stereo (placement,
     spread) or take stereo and give stereo (the room, the echo, the width
     ceiling, the master).
   - Build the type and the trait shape only. No stereo render yet, and
     `Core::start` still refuses two channels.
3. **Channel events, and a channel's own blocks.** An event sent to a
   channel at a time (`Core::event(channel, time, kind, value)` or your
   shape), which reaches every voice sounding there through an optional
   `Voice` hook (default: ignore it), and reaches the blocks a channel
   holds for the whole loop.
   - **Kinds to start with:** damp (nylon's), choke (a voice whose next
     hit cuts its last: the tuned kick, item 67), release, and retune (a
     new chord for a held instrument: the organ, item 61, where notes two
     chords share keep sounding).
   - **Nylon's damp through it,** bit for bit: `Core::damp` no longer
     reaches into `Held::Nylon`, and `Core::note` no longer reads a strum
     from nylon's extra value. After this no voice is named outside its
     file and its registry line (the numbers mirrored in `js/core.js`
     aside). The web host's call changes in the same PR (it's ours).
   - **A channel's own blocks:** today's per-channel bodies, generalized
     just enough that a channel can hold a processor (padSweep's lowpass,
     subDuck's gain) or a source that lives for the loop (the organ, the
     wash), set by events, outside the voice pool and its budget. Move the
     bodies onto it if they stay bit-identical; otherwise propose the
     shape and leave building it to item 61.
   - The channel count stays 5 here; the wash's channel (item 62) adds
     the sixth.
- **The test kit grows with it:**
  - a probe can wire a modulation input (a block played with an LFO on a
    setting), so modulated blocks are checked like the rest;
  - a stereo block gets **the ears' stereo guards for free:** folded to
    mono against its mono input (loudness within 0.5 LU, worst octave
    within 1 dB), side under mid above 200 Hz and below 120 Hz (-20 dB or
    lower), correlation (median 0.7 or more, 5th percentile 0.3 or more)
    and left/right balance (within 2 dB). These are the limits in
    `docs/ears/batch-4.md` ("Guards") that Mikey's approved baseline met.
- **Two demonstrations, each one new file plus one line** (test only,
  like the tremolo): a modulated block (the tremolo redone as an LFO
  wired into a gain's input is enough), and a stereo one (a balance:
  mono in, stereo out), each through the kit with no test written for it.
- `core/README.md`: how to wire a modulation input, how a block goes
  stereo, how a voice answers a channel event.

**Yardsticks:**
- A refactor: goldens on every mode (`--strict`, `--flush`,
  `--blocks all`, `--rates`), `tools/baseline.mjs` and `core/golden/blocks.txt`
  unchanged byte for byte (new test-only lines aside).
- Costs: instruction counts within 1% of `main` on every golden (the 5%
  rule is the hard limit; this one is a refactor).
- The demonstrations: one file and one line each, checked by a test as 46
  checks the tremolo.
- Nylon's damp: a test that a damped strum renders as before, and a grep
  test that no voice's type is named outside its file and registry line.
- **A note for the brain:** which of the batch's items (below) should
  use which part of this, and what is still awkward for phase 5 (46's
  notes 5-7: memory for big voices, named extra values, several numbers
  per type).

## 46c. The channel seams the batch still needs (before item 61)

**Why** (brain, 2026-10-19, from 46b's own notes for the brain): 46b
landed clean (#241, v101), but four batch items assume things it did not
build, and its note says so:
- **Channel blocks take no per-loop settings, modulation or events**
  (note 1). Item 61's organ registration, 62's wash, 63's arc period and
  echo feedback, 68's duck depth and recovery and 72's sweep period are
  all per-loop data on a channel's own block.
- **Each note hears each kind of event once** (note 2). Item 61's organ
  is retuned at every chord, many times while held.
- **No cross-channel signals** (note 4). Item 68's subDuck is the kick's
  signal driving the bass channel's gain; 62's wash follows other
  layers.
Built inside those items, each would grow its own path. One small item
first, so the batch has one way of doing each. Hands: **the start of
Session 25a (Opus 5.5, high)**, before item 54, or at the latest before
item 61.

**What:**
- **A host path into a channel's own blocks:** per-loop settings set by
  the host (as `Core::param` reaches the mix today), and the same
  modulation inputs voice blocks have, so a channel block can carry its
  own `Lfo` with a per-loop rate and depth.
- **Each event kind declares its reach:** once (today's damp, exactly as
  now) or every time (a retune or bend heard by a held note as often as
  it is sent). A **retune** event for held voices: a note two chords
  share keeps sounding untouched; only changed notes are let go and
  spoken again (item 61's rule).
- **Cross-channel signals:** one channel's signal (or a chosen tap of it)
  can drive another channel's block setting. Decide the order and say it
  in the PR: render the source channel first within the block, or one
  block's delay; deterministic either way, the same on every target and
  block size.
- **Three test-only demonstrations,** each one file plus one line, through
  the kit with no test written for it: a channel block whose rate is set
  per loop, a held voice retuned many times, and a gain on one channel
  ducked by another's signal.
- `core/README.md`: the three, in the walk-through.
- Note 6 (the oscillator's start rule changing when any signal is
  connected) goes in the README as a caution; no change.

**Yardsticks:** as 46b. A refactor: goldens on every mode,
`tools/baseline.mjs` and `core/golden/blocks.txt` unchanged byte for byte
(new test-only lines aside); costs not above `main` by more than 1% on any
golden (cheaper is fine; 46b came in at -3.6% to +0.46%); no allocation,
`dyn` or panic on the render path; the mono render pays nothing.

## Before phase 2: the sound batch (items 54-73, then 74)

**Read first:** `docs/ears/batch-4.md` (dubStab, the organ, the wash,
longChords, the stereo baseline: recipes, levels, guards),
`docs/ears/batch-5.md` (the mixing logic: tunedKick, lateFloor, floorDrop shapes,
pedalBass, subDuck, padSweep, echoTime), `docs/ears/prototypes.md`
(roundBass, glass, floorDrop, arc), and `docs/EARS.md`. **The sketches are
the reference:** `tools/ears/sketches/ears4-sketch.patch` (and
`space-sketch.patch` inside it) and `ears8-sketch.patch` (applied after
it), offline JavaScript Mikey heard and approved. The prototypes' branch
(`ears/prototypes`, #152) holds roundBass, glass, floorDrop and arc.
**Mikey approved all of it** (2026-10-10, 10-16, 10-18): build what he
heard, in the core, then he hears it in the app (Session 26's album).

Three groups, in this order, a fresh hands session for each (or when
usage runs out), one PR per item: **(a) instruments, items 54-63,
Session 25a; (b) stereo in the core, 64-66, Session 25b; (c) the mixing
logic, 67-73, Session 25c.** All Opus 5.5, high. Item 74 (clippy) is low
priority, for any session with usage to spare. Every item stands on 46b.

**Rules for the batch** (with the standing rules and the core rules
above):

- **Where it is decided, where it sounds** (brain, 2026-10-18): *when* a
  sound plays and *which loops* get it is the JavaScript side's (the
  generator, or the engine at load where the sketch decides it), allowed
  until phase 2 starts and then ported with everything else, as part of
  what the port must match. *What* plays is the core's, built on 46b's
  interface: each new sound a block or a voice in `core/src/blocks/`, one
  file and one line, probed by the kit, with its stored render.
- **Core only.** The JavaScript synth (the fallback) builds nothing new:
  for a new voice number it plays the nearest existing voice, or leaves
  the part out where nothing fits; each item says which. `js/core.js`
  mirrors the new numbers.
- **Mikey's rules for every choice** (2026-10-18): today's behavior stays
  one option in every choice, never replaced; each choice is drawn per
  loop, seeded from the loop's code on a stream of its own (so no other
  draw moves and a loop replays the same way); **most loops get one or
  two of the changes** (item 54 sets how). Exceptions, as approved:
  roundBass is every loop's bass, longChords keeps its own approved
  spread (today's lengths 56%), and floorDrop replaces full stops as
  approved; the stereo baseline is every loop's.
- **Measured against the ears.** Each new sound is measured the way its
  report measured the sketch, on the report's corpus loops (corpus seed
  20261009; `tools/ears/`: `render2.mjs`, `sketch_check.py`, `warmth.py`,
  `low_guard.py`, `kick_check.py`, `stereo_guard.py`), core build against
  sketch: level by ear (its loudest 3 s against the loop, K-weighted)
  within 1.5 LU of the sketch's figure, its own tone figure (centroid,
  attack, band shares) where the report gives one, and refused notes no
  more than the sketch's (none, in every report). Plus the standing
  rules: `VOICE_COST` from render timing, the tone probe's 2-5 kHz share
  at or below the flute's for anything that plays in the melody's
  register, refusals on full and lite. Say where the core's version must
  differ from the sketch, and why.
- **Peaks:** from item 55 on, no voice peaks over -1 dBFS alone at the
  kit's probe (the kick's neighborhood today).
- **Baselines:** each item that changes sound updates the core baseline
  and says what moved; **the balance lock is re-baselined once per group**
  (`stats.mjs --check`, then `--n 10000`), on purpose, at the group's
  last item, saying what moved.
- **For the album:** each Done line names two corpus loops that draw the
  item's sound, with a time, so Session 26 can build a labeled album
  (keep / tune / drop) without searching.
- **Stop and leave a note** where a recipe can't be met in the core as
  heard, rather than build something Mikey didn't hear.
- **The album loops, re-checked** (brain, 2026-10-20): at the end of group
  (c), re-check every Done line's album loops against `main` (adding choices
  reshuffles which loops draw what) and update the lines whose loop no
  longer plays its sound, before Session 26.

**Group (a): instruments (Session 25a).**

## 54. The per-loop choices (no sound change)

**Why:** Mikey's rule, "most loops get one or two of the changes", needs
one place that decides how many and which, so every later item only adds
an option.

**What** (JavaScript, beside the generator's draws):
- A list of choices, each with its options and weights; today is always
  one option. Each choice has an eligibility test (subDuck needs a kick
  and a sub; stutter and stagger a bass line; padSweep a sustained chord
  voice; lateFloor something that plays from the first second...) and
  vibe weights (toward x1.5 to x3, away x0.2 to x0.5, never 0; the table
  in `docs/ears/batch-5.md`, "The [JUDGMENT] lines").
- Per loop, on a stream of its own: draw **how many** changes (none 15%,
  one 45%, two 30%, three 10%: a taste call, written "for Mikey's ears";
  one constant), then **which** eligible choices (weighted, without
  repeats), then each one's non-today option. The rest stay today.
- What counts as a change: the new parts (glass, dubStab, the organ, the
  wash, the arc) and Ears 8's ideas (tunedKick, lateFloor, floorDrop
  shapes, pedalBass, subDuck, padSweep, echoTime). Not counted: roundBass,
  longChords, the single floorDrop and the stereo baseline (see the rules
  above).
- `stats.mjs` reports, over 10,000 loops, the share of loops by number of
  changes and by choice; a test checks the counts' shares within two
  points of the weights and that every choice is reachable.
- Choices hold for the loop; "Let the loop wander" doesn't redraw them
  (item 67's kick swap is the one exception). Nothing goes in the share
  code: the code's seed gives them, and pins come with phase 2's
  requests.

**Yardsticks:** with the list empty, every loop renders as before (core
baseline, balance lock and `test/generator.test.mjs` unchanged); the
shares test passes.

## 55. The frame drum's solo peak (and a voice ceiling in the kit)

**Why:** item 46's kit showed the frame drum peaking at **+1.0 dBFS on its
own** at velocity 0.8 (the kick -1.1, the soft kick -1.2), before the mix
and the master's ceiling. A game or a host that plays a voice alone, and
the batch's new voices, shouldn't rely on the master to catch it.

**What:**
- Bring the frame drum's solo peak to -1 dBFS or under at the kit's probe
  by its shape (its strike's transient), not by level: its K-weighted
  loudness alone within 0.3 dB of today, and its layer's level in the
  baseline loops within 0.3 dB. The conservative change, "for Mikey's
  ears".
- The kit's ceiling for voices becomes -1 dBFS at the probe (other blocks
  keep 46's +6 dBFS); any voice over it fails, with its name.
- Check every voice and drum kind against it; report any other over it
  (none expected but the frame).

**Yardsticks:** the kit passes with the new ceiling; the frame's
loudness as above; baseline updated (only loops with a frame drum move;
say which and by how much); a sound change: in Session 26's album.

## 56. roundBass

**Reference:** `docs/ears/prototypes.md` and the `ears/prototypes` branch.
- **Generator:** the bass register 36-57 instead of 28-52, never the extra
  octave down. Every loop (approved as the bass, not a choice).
- **Core:** soft 2nd, 3rd and 4th harmonics under a 900 Hz lowpass on
  every bass voice but `rhodesbass` (an FM voice), so the bass reads on a
  phone. A block the bass voices share, not five copies.
- **Fallback:** the JavaScript synth gets the register (the generator's)
  but not the harmonics.
- **Measured:** on dust: 40-60 Hz 18% → about 6% of the energy, 120-250
  Hz 10% → about 27% (the prototype's); the bass layer's level within 1
  dB of today by ear; no refusals added.

## 57. longChords

**Reference:** `docs/ears/batch-4.md` (rounds 3 and 4), the sketch's
`longChords: 'on'`. Generator only (no new sound):
- Drawn per loop from the chords' own stream: today's lengths 56%, twice
  today 16%, about 8 s 10%, about 16 s 9%, the whole loop 9%; calm loops
  (energy under 0.35, or floating, meditative, hushed, misty, foggy,
  hypnotic) take the long ones 1.8 times as often, driving loops (energy
  over 0.65) half as often.
- Held chords trimmed: twice today -0.7 dB, about 8 s -1.3 dB, 16 s and
  the whole loop -2 dB. Repeated pad chords tied into one note.
- **The chords keep the octave today's progression gets** (round 4: 0 of
  87 held loops move; the flip in `separateRegisters` is what Mikey heard
  as loud and rough). Check it: the 200-loop scan, 0 flips.
- **Yardsticks:** the round 4 figures on grove #47, dust #51, thaw #2;
  refusals equal or fewer (round 4: fewer).

## 58. floorDrop

**Reference:** `docs/ears/prototypes.md`; Ears 8 ported it as
`floorDrop: 'on'` (`ears8-sketch.patch`). Generator only:
- drift's "the whole thing stops" becomes the kick and the bass out for
  one or two bars while everything else plays on, at 0.35 a pass instead
  of 0.1; rests no longer fade the tails. Loops of 4 bars or more.
- **Yardsticks:** grove: floor drops about 0.6 a minute, full stops 0;
  never a full stop; refusals as the Ears 8 report gives for the
  approved recipe (undertow's rise from 272 is the recipe's own; say if
  the core's differs).

## 59. glass

**Reference:** `docs/ears/prototypes.md`; the prototypes' branch.
- **Generator** (a choice in item 54's list): on a chord's first beat (55%
  of the time), two or three of its notes ring out in E5-G6, a dotted
  eighth apart, into the texture channel.
- **Core:** a soft FM bell, ratio 3.5 (on 46b's modulation input), its
  level from its chord's velocity, scaled by chords a bar as the sketch did
  (Mikey, 2026-10-20); only chord events of two or more distinct notes ring
  out, since the sketch rings two or three of a chord's notes and a one-note
  event can't give what Mikey approved (Mikey, 2026-10-20); a probe in the
  kit.
- **Fallback:** the FM bell voice the JavaScript synth already has.
- **Measured:** thaw: 1-2 kHz 2.8% → about 6% (the prototype's rise of
  about 3.4 points: today's thaw starts lower, so the rise is the
  yardstick); on vapor (under one chord a bar) it adds no more than about
  1.3 LU; the tone probe's 2-5 kHz share at or below the flute's (it plays
  high).

## 60. dubStab

**Reference:** `docs/ears/batch-4.md`, round 4 (approved).
- **Generator** (a choice in item 54's list): busy loops only (drums and
  7 or more notes a second); density drawn rare 34, sparse 36, some 20,
  busy 14 (chances per bar on the offbeat after beats one and three, as
  the sketch); only over a groove (drums or bass in the bar and four notes
  among the other layers), never in a rest or full stop; voiced G3-C5.
- **Core:** a voice of its own, not today's `stab` (two saws through a
  bandpass): a triangle under two saws 6 cents either side, a 22 ms
  attack, a lowpass closing from 2.4-3.4 kHz toward 700 Hz, the lows
  trimmed under 320 Hz, a tail of 0.24-0.34 s, a darker, lighter throw
  into the echo, half round 3's reverb.
- **The budget:** it asks whether there is room (its cost plus 20 units
  under the full ceiling) and holds nothing, so it never costs another
  note.
- **Fallback:** today's `stab`, at the same level.
- **Measured:** centroid near 285-302 Hz and attack near 22 ms
  (`warmth.py`; "Quadrant Dub I" reads 377 Hz and 12 ms); its loudest 3 s
  against the loop: wayfare #23 about -10 LU, grove #62 -13.5, cinder #12
  -14.

## 61. The chapel organ

**Reference:** `docs/ears/batch-4.md`, rounds 5-7: one organ, the mix
Mikey chose (`organVariant: 'chapelOrgan'`: 55% chapel, 25% round 5's
pipe, 10% distant cathedral, 10% celeste).
- **Generator** (a choice in item 54's list): when it plays it holds the
  chords; it moves at most once a bar and holds through a passing chord
  (one that keeps the lowest note and two of the notes); it lets go in a
  bar the loop leaves silent and after a bar or so where the chords rest
  for more than two bars.
- **Core:** an instrument a channel holds for the loop (46b's channel
  blocks), **outside the voice budget, priced as fixed overhead**:
  - four voices, retuned at each chord through 46b's retune event; a note
    two chords share keeps sounding, untouched; only changed notes are
    let go and spoken again;
  - the stops mixed as pipe loudness (the chapel's 8' and 4' flutes
    leading, a little 16', 2 2/3', 2' and mixture); speech about 0.14 s
    with a chiff about half round 5's; a celeste rank about 3.7 cents
    sharp at about 0.36; release about 0.47 s; a 3.75 dB dip at 800 Hz;
    a darker tone (1.4-2.5 kHz), a slow level shimmer (no pitch-bending
    chorus), no drive; about 0.53 of it into the room;
  - the loop's soft registration (the 25% from round 5's pipe), so loops
    still differ.
- **Fallback:** none fits (the JavaScript synth leaves it out).
- **Measured:** its loudest 3 s against the loop: haven #38 about -14 LU,
  glade #21 -10.7, grove #62 -11.1 (round 7); refused notes of other parts
  unchanged (it's outside the budget); its fixed cost reported (render
  timing, full and lite).

## 62. The wash

**Reference:** `docs/ears/batch-4.md`, round 5 (approved).
- **Generator** (a choice in item 54's list): a swell every 17-26 s
  (drawn per loop), aimed 17-21 dB under the loop.
- **Core:**
  - **a channel of its own** (the sixth), with the texture's level and
    sends; `js/core.js` and the mix grow by one channel;
  - two decorrelated noise loops through a pair of gentle lowpasses
    sweeping 350 Hz up to about 2.5 kHz and back, slow rise and fall;
  - two soft resonances tuned to the loop's key (its root or fifth in
    600-1200 Hz, and an octave and a fifth above), like wind across a
    bottle;
  - **a level follower:** a slow envelope (about 1.4 s) of the five other
    layers' sum sets the wash's level, so it is heard under a beat and
    stays behind a sparse loop.
- **Fallback:** none fits (left out).
- **Measured:** centroid about 0.9 kHz; its loudest 3 s against the loop:
  grove #62 (beat) about -16.9 LU, vapor #4 -13, haven #43 -15.9; plays
  through a full stop only as the sketch does (say what it does).

## 63. The arc

**Reference:** `docs/ears/prototypes.md` and its branch.
- **Generator** (a choice in item 54's list): the arc's period drawn 2-5
  minutes (the prototype's 120 s was for the demo).
- **Core:** a second lowpass after the master's tone filter, breathing
  300 Hz → 9 kHz → 300 Hz, lingering on the dark side; the echo's
  feedback rises a little as it opens (46b's modulation input).
- **Starts open, at a drawn point of its swing that is bright** (a taste
  call, "for Mikey's ears"): the prototype started every loop closed at
  300 Hz, the second suspect when sounds went missing on its preview.
  Say what you chose.
- **Measured:** haven: -15 dB above 500 Hz at the closed points, unchanged
  when open; refusals unchanged (it is a filter); the group's balance
  lock re-baselined here, saying what moved.
- **Amended by item 63b (brain, from Mikey's report, 2026-10-21/22; built
  in v113):** the swing above is now one of **two shapes**, drawn per loop
  on the arc's own stream:
  - **deep:** the swing above, exactly (about 1 loop in 3). Its cutoff sits
    under 700 Hz for about half of every swing, which kept Mikey's loop
    muffled long enough that he thought the instruments were gone; once he
    knew it returns he liked it, so it stays, as the rarer shape.
  - **dip:** mostly open (about 2 in 3): at 9 kHz for 65% of each swing,
    with one smooth dip over the other 35% (a raised cosine in log
    frequency) to a floor drawn per loop from 800 Hz to 1.5 kHz; a loop
    starts in the open part; the echo's lift follows how open it is (the
    same cutoff lifts it as the deep swing's does).
  - **Vibe weights** (item 54's rule): the arc itself leans toward hazy,
    foggy, misty, frosty, still, meditative, spacious, floating, hypnotic,
    mysterious, reflective and wondrous, and away from bouncy, lively,
    energetic, skipping and zany; deep leans toward the same calm list, dip
    toward sunny, bright, happy, joyful, merry, bouncy, lively, energetic,
    skipping and zany (toward x2, away x0.4); the period is 3-5 minutes for
    the calm list, 1.5-2.5 for the bright list, 2-5 for the rest.
  - **Named settings:** shape, period, floor, start, the dip's share and
    how often a loop draws the arc are `ARC` in `js/choices.js`, each with
    its range, so phase 2's requests can pin them (`arcOf` takes pins
    already) and a later slider or a game host can set them (Mikey,
    2026-10-22); the core takes them through `ARC_SHAPE`, `ARC_FLOOR` and
    `ARC_DIP` beside `ARC_FROM` and `ARC_PERIOD`.
  - **Yardsticks:** dip below 2 kHz for no more than about 15% of a swing,
    within twice the floor for about 15-40 s, never under the floor; deep
    identical to today's for the same period and start; loops without the
    arc bit for bit as before (6-minute renders). Mikey kept the dip at 35%
    of every swing (2026-10-24), so its time within twice the floor runs
    11-45 s at the ranges' corners (Notes from Claude Code, item 63b).

**For Session 26 (Mikey's idea, 2026-10-22):** a **"glacial"** vibe, very
slow and vast, to be designed with him there: it leans the deep arc and the
longest periods.

**Group (b): stereo in the core (Session 25b).**

## 64. The stereo render

**Why:** promise 9 (`docs/CORE.md`): stereo across the board, a mono
switch, two first-class renders.

**What:**
- `Core::start` accepts two channels. The stereo render is a render of
  its own on 46b's stereo shape, not a mono mix spread across two: **the
  mono render stays bit-identical and pays nothing** (mono goldens, mono
  baseline and mono costs unchanged).
- The mix's mid is today's mono mix: before the master, (L + R) / 2 of the
  stereo render equals the mono render's mix, exactly where the
  arithmetic allows (say where it can't).
- The master in stereo: the compressors **linked** (one gain for both
  sides, from the mono detector) so they act as in mono; the saturator per
  side (its fold residual measured; the ears read -20 to -41 dB against
  mono, from the saturator and compressors).
- **The width ceiling** (a block; `docs/ears/batch-3.md`, "Stereo"): the
  side turned down wherever it rises above its target against the mid,
  above 200 Hz, over 2 s windows (down in about 0.5 s, back over about 3
  s); the mid untouched, so the fold doesn't change.
- Goldens: a stereo set beside the mono set, on every target and mode.
- The web host renders stereo through the worklet's two outputs;
  everything still centered here (L = R), the spaces come in 65.

**Yardsticks:** mono goldens, baseline and costs unchanged; the stereo
goldens equal on every target; with no space recipe, L = R and each
equals the mono output to the stated limit; cost of the stereo render
reported (target: under 1.5 times mono on the baseline loops).

## 65. The approved stereo baseline, as space recipes

**Reference:** `docs/ears/batch-4.md`, round 4: the blend of round 3's
**room** (weight 0.64) and **ensemble** (weight 0.5), **steady** (no
breath, no drift: moving stereo is a later, experimental option, in a
session of its own). Built as data: a space recipe gives ranges, the core
draws within them from the loop's seed (promise 9), and every variable
is a named setting.
- **Room:** every dry part centered; the reverb's side made from the
  reverb through four delay allpasses (5-23 ms) above 180 Hz; no echo
  width.
- **Ensemble:** chords and texture widened symmetrically (a slowly
  swaying 9-15 ms copy, above 200 Hz, as side only; 1.6 times round 2's
  spread, then the whole matched to the target width); the light kit
  (hats, shakers, jingles) placed 0.08-0.15 to one side, as one; a light
  texture (chimes, bells, drops; glass) placed by 0.1-0.25; placement a
  balance above 150 Hz, so low notes stay centered by construction.
- **Never placed:** the melody, chords and pads (widened only), the bass,
  the kick and the other heavy drums. The organ rides the chords' spread
  and the room; the wash's two noises go out as its own width, within the
  ceiling.
- **Width by loop:** every width scaled by how busy the loop is (0.25 under
  half a note a second, up to 1 at 5.5) and by 0.75 when the reverb's space
  is over 0.7; ceilings -7 dB on busy loops, -9 sparse, -12 very sparse.
  What the core needs to know about the loop (how busy, how wet) the host
  tells it at load.
- **Target:** side about 18 dB under mid above 200 Hz (median), as
  approved.

**Yardsticks** (`stereo_guard.py`, on the report's loops and the
baseline's 40, against the mono render): fold within ±0.5 LU and every
octave within ±1 dB; side 20 dB or more under mid below 120 Hz;
correlation median 0.7 or more, 5th percentile 0.3 or more; balance within
±2 dB; bounce and travel near round 4's (1.2 dB, 0.5 dB); refused notes as
in mono; cost under 1.5 times mono, lite included. The new instruments
(54-63) checked inside it.

## 66. The Mono switch, and stereo by default

- **Mono** joins the settings (Mikey's exception to "theme and lighter on
  processor are the only settings"; `docs/CORE.md`, promise 9), off by
  default: the app plays the stereo render; on, it plays the mono render
  (exactly today's balance). Kept with the other settings; a Painter
  session can restyle it later.
- Saved like the other settings; Diagnostics shows the layout.
- Share codes and exports don't carry it (it is the listener's).
- **Yardsticks:** switching while a loop plays doesn't click or restart
  it; the mono render is bit-identical to mono before item 64; the app's
  tests pass; a phone's render cost in stereo reported in Diagnostics
  (promise 9: stereo must not double the cost on a phone). The group's
  balance lock re-baselined here, saying what moved (expected: nothing in
  mono). **Session 26 runs the blind stereo-against-mono A/B.**

**Group (c): the mixing logic (Session 25c).**

**Reference for all of them:** `docs/ears/batch-5.md` (each candidate's
recipe, yardstick and guards) and `ears8-sketch.patch`; Mikey's rules in
"How they go in". Each is a choice in item 54's list, today one option.

## 67. tunedKick

- **Core:** a kick that lands on a target note, a fall rate and an
  optional tail level, as one voice per loop that each hit retriggers
  (46b's choke), priced as one short hit (held 0.36 s in the budget, as
  the soft kick; today's kick holds 0.45 s). Today's kick and soft kick,
  the same sweep, landing on the key's note between G1 and D2 (49-73 Hz;
  a key of D# to F# takes its fifth). Level matched by ear at load
  (K-weighted energy against today's kick); the long boom's tail as loud
  as fits within +1 dB by ear and +3 dB of today's kick's own energy under
  60 Hz and at 60-120 Hz. Centered always.
- **Generator:** today, short (falling 180-260 dB/s) or long (today's
  punch, then a tail 7-17 dB down ringing at 45-80 dB/s); booming x2.5
  toward long, and the rest of the vibe table.
- **A wandering loop may swap its kick** at a section boundary between
  today's F1 kick, the short tuned kick and the long boom, with a
  crossfade or a wait for the old kick to ring out (Mikey's rule).
- **Yardstick:** batch-5's, by `kick_check.py kick` on the drum stem:
  85% or more of kicks on the drawn note; falls within 20% of the drawn
  family; the loop within ±0.5 LU of today (+1 for a boom above B1); the
  kick's low bands within 3 dB of today's; refusals no more than today.

## 68. subDuck

- **Core:** a duck on the bass channel alone (46b's channel block), keyed
  by the kick: 5-9 dB (scaled by the kick's velocity), back in 0.15-0.25
  s; depth and recovery per-loop data.
- **Generator:** only on a loop with a kick and a sub.
- **Yardstick:** the bass stem 5-9 dB down 10-50 ms after each kick
  against the same loop without it, back within 1 dB by 0.25 s; other
  layers untouched; never deeper than 9 dB.

## 69. lateFloor

Generator only. After a load (never a reroll or a resumed loop), the kit
and the bass sit out 8-30 s, rounded to two bars, then enter together
(40%), the kit first and the bass two bars later (35%), or the bass first
(25%). Only on loops with drums or bass and something else that plays
from the first second. **Yardstick:** batch-5's (absent for the drawn
time within one step, present after; other layers' notes and levels
within 0.5 dB; refusals equal or fewer).

## 70. floorDrop shapes

Generator only, on item 58's floor drop: each drop draws a shape, single
(40), stagger (25: the kick back on time, the bass one or two bars
later), stutter (20: three or four one-bar drops, two at fast tempos, one
every two or four bars), long (15: 10-20 s, or the whole pass in a loop
of 8 bars or fewer). Drawn **at the end of drift's draws**, so no other
variation moves; stagger and stutter only with a bass line; the style may
follow the bar length (Mikey's rule: say how you did it). **Yardstick:**
batch-5's; rate per pass unchanged; never a full stop; refusals no more
than the single drop's.

## 71. pedalBass

Generator only: a bass style `pedal` that can take over a held, pulse,
dub or walking bass: the tonic in a rhythm drawn per loop (held, half
bars, beats, or off the beat between the kicks), stepping in 10-25% of
its notes to one neighbor drawn per loop (a whole tone up, the tone
below, the flat second where the mode has one, or the fifth); the loop's
bass voice, the octave nearest its own bass (in roundBass's register).
**Drift must spend the original bass's draws** (or drift's bass draws
move to a stream of their own), or every later variation shifts: check
0 passes differ. **Yardstick:** batch-5's; the low-end shares within 3
points of the loop's own bass in the new register.

## 72. padSweep

- **Core:** a lowpass with a slow LFO on the chord channel (46b's
  modulation input and channel block): closed at 450-800 Hz, swinging 2
  to 3.5 octaves up and back every 12-28 s, Q 0.7-1.2; per-loop data.
  Hand it over across a load as the sketch couldn't check.
- **Generator:** sustained chord voices only.
- **Yardstick:** the chord layer's brightness swinging two octaves or more
  at the drawn period; the other layers and the refused notes unchanged.

## 73. echoTime

- **Generator:** the echo's time drawn per loop: three quarters of a beat
  (today, 45%), one beat (25%), half a beat (15%), two thirds (15%); one
  beat toward spacious and floating, half a beat toward nimble and
  skipping.
- **Core:** the echo time a per-loop parameter (check what `setEchoTime`
  already sends; build only what is missing).
- **Yardstick:** the echo time as drawn, within 4 ms. The group's
  balance lock re-baselined here, saying what moved.

## 74. Clippy pedantic in the files 46 left (low priority)

**Hands:** Sonnet 5.5, high, any session with usage to spare; it may go
between items of the batch, never inside one. **Why:** the core rule
("clean in the files the PR touches") can't be met in eleven files that
carry 405 old pedantic warnings (#238's table: `wave.rs` 69, `timeline.rs`
68, `blocks/compressor.rs` 56, `resample.rs` 41, `param.rs` 38,
`master.rs` 38, `blocks/osc.rs` 31, `blocks/noise_source.rs` 22, `mix.rs`
16, `delay.rs` 16, `blocks/filter.rs` 14). Fix them, or allow a lint
locally with a one-line reason where following it would make the code
worse; one PR, or one per file if that is cleaner. **Yardsticks:** a
refactor: goldens on every mode, baseline and `blocks.txt` unchanged
byte for byte; costs within 1%; `cargo clippy --all-targets -- -W
clippy::pedantic` reports zero, and CI gains that check so it stays
zero.

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

## Next picks, wired for real (items 48-53)

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
rules above):
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

## 48. The request, end to end, with tempo (phase 2)

**Why:** promise 11. One request for everyone: the app's panel, a
developer's code and a game's events build the same thing. Tempo is
the simplest pick to prove it with.

**What:**
- The Rust generator takes a **request**: for each choice, pinned,
  ranged or chance. Start with vibes (as today's selection) and tempo;
  leave room for the rest without building them (48's design is what
  49-52 follow, so keep it small and documented).
- **Tempo:** pinned ("77 bpm") or a range ("70-85 bpm", the design's
  example). A range draws from the seed inside it. Profiles with gaits
  draw tempo and meter together today: a pinned or ranged tempo picks
  the gait that fits and bends the tempo inside it. When style is left
  to chance, prefer profiles whose band holds the tempo; when style is
  pinned too and its band doesn't, play the profile at the asked tempo
  (conservative: honor the pin; "for Mikey's ears"). The core's limit
  is the core's to set; the app's picker range is the Painter's.
- The app builds the request from the panel and sends it with each new
  loop; today's vibes selection becomes part of it. Remove Painter 6's
  clamped tempo plumbing.
- Share codes: a loop made from a request reproduces from its code.
  (Short codes that carry only the pins are phase 6.)
- The "can't do that" line, for whatever request truly can't be met.

**Yardsticks:**
- Nothing picked: goldens, baseline and balance lock unchanged.
- Pinned tempo: measured bpm within 0.5 of the pin on every target,
  across 200 seeds and every profile.
- Ranged tempo: every loop inside the range; over 500 seeds, the draws
  cover the range (no clumping at one end; report the spread).
- Same request and seed, same samples on every target.
- A test that walks the request's choices (it grows with 49-52).

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

## 41. Urgent: the Pixel 10a still glitches on v91 -- what the field report shows

Item 40 fixed what the harness could find; the tester's phone (Pixel 10a,
GrapheneOS, installed app, Chrome 155) still glitches. His Diagnostics on
v91, 1.5 minutes across views (Mikey has the full text; the figures that
matter):

| view | on screen | long tasks (longest) | longest tick gap | late notes |
|---|---|---|---|---|
| library | 0:50 | 8 (1338 ms) | 1366 ms | 728 |
| save | 0:19 | 10 (1788 ms) | 1792 ms | 181 |
| layers | 0:18 | 3 (632 ms) | 83 ms | 0 |
| sounds | 0:02 | 0 | 67 ms | 78 |
| now | 0:01 | 0 | 106 ms | 0 |

Also: `late: 1007`, `dropped: 31`, `lookahead: 2`, `redecides: 0`,
`outputLatency: 0.448`, `measuredLatencyMs: 600`, `baseLatency: 0.0213`,
`sw: installing`, `fill-ins: -` in every view.

**What the brain reads in it (Session 16; check each before relying on
it):**
1. **Late notes without any stall.** In `sounds` the longest tick gap was
   67 ms and 78 notes still arrived late. So something besides the main
   thread delays notes on this phone. The likely one: its audio output
   runs on a large, bursty buffer (output latency 448 ms, measured 600 ms),
   so the worklet takes its messages and renders in big chunks, and a note
   handed over less than a chunk ahead is already in the past when it
   lands. `REDECIDE_LEAD` (60 ms, used when a Sound slider, mute or re-roll
   cancels and decides again) is far inside that. The core plays a late
   note at once (`lib.rs`, `Taken::Late`), so late notes come out bunched:
   the glitch.
2. **Stalls still beat the 2 s queue.** Library and Save / Share still
   block the main thread for 1.3-1.8 s on this phone (the harness, at 6x
   CPU throttle, saw 188 ms at most). Add the phone's ~0.6 s of buffer and
   a 2 s queue is not enough. **Not software graphics:** the tester's
   `chrome://gpu` (screenshot from Mikey) shows Canvas, Compositing,
   Rasterization and WebGL all hardware accelerated, Vulkan on, but
   **Multiple Raster Threads: Disabled** and Skia Graphite off. The brain's
   new lead: `cover.js` reads pixels back with `getImageData` and writes
   them with `putImageData`. On a **GPU-backed** canvas each readback
   flushes the GPU and copies the image back to the CPU, which on some
   drivers (this is a new GPU, Imagination in the Tensor G5) costs tens to
   hundreds of ms per call, and covers do it per draw. A desktop harness
   with a fast GPU (or none) never sees that cost. Raster work on one
   thread also makes large blurred panels slower to repaint.
3. **`dropped: 31`:** the core's queue or sung-draws arena was full when
   notes arrived, or every voice was busy. Find which, here.
4. **`sw: installing`** while playing: check a service-worker update isn't
   stuck (a precache fetch failing on the phone) or competing for the page.

**Do:**
1. **Reproduce this phone, not a fast desktop.** Run the harness with a
   large audio buffer (Chromium's `--audio-buffer-size`, about 16384 frames)
   at 4x and 6x CPU throttle. For the stalls, find every pixel readback and
   GPU flush on the main thread (`getImageData`, `putImageData`,
   `toDataURL`, `toBlob`, `drawImage` from a canvas) and time them; also
   try `--disable-features=CanvasOopRasterization` and a GPU-backed canvas
   with readbacks (headless Chromium may need `--use-angle=swiftshader`
   and `--enable-gpu-rasterization` to keep the canvas GPU-backed), so the
   readback cost shows up. Software graphics (`--disable-gpu`) as a second
   stress, not as a model of this phone. Per view: long tasks
   and what they are (a trace), tick gaps, late and dropped notes, and how
   far ahead of its time each note reached the core. Report the table
   before any fix; if it doesn't show what the field report shows, say so
   and stop to leave a note.
2. **Never hand the core a note it can't play on time.** Measure the
   device's real render-ahead on the page (the core can report the time it
   has rendered up to, or how early each note arrived) and plan from that,
   not from `ctx.currentTime`: the re-decide lead becomes render-ahead plus
   a margin, and the queue depth covers the longest recent stall plus
   render-ahead, growing toward the hidden 8 s when the page stalls. A
   change on this phone may then take about 0.6 s to be heard; that is its
   own output delay, and nothing does better.
3. **A late note must not bunch.** If one still arrives late, the core
   drops it when it is more than a few tens of ms late (a sustained note
   may join partway instead, if that sounds cleaner): measure both, choose,
   and write it down for Mikey's ears. Count late-dropped apart from
   queue-full.
4. **Take the heavy work off the main thread, for good.** Draw covers in a
   Worker with `OffscreenCanvas` (all of them, the playing loop's
   included), so no cover ever blocks the page, and **never read pixels
   back from a GPU canvas**: compute the cover's pixels in a typed array and
   write them once, on a context made with `willReadFrequently: true` (a
   CPU canvas) wherever a readback is unavoidable; hand finished covers to
   the page as `ImageBitmap`s. Find what else is long in
   Library, Save / Share and Layers under software graphics and fix it.
   Where blur is the cost, give the blurred panels a cheaper equivalent when
   the GPU isn't doing the work (or always, if it can't be told apart): the
   same colors, a still pre-blurred background, no live blur. Any visible
   change is noted for a Painter session.
5. **Diagnostics:** add the GPU's renderer string (WebGL
   `UNMASKED_RENDERER`, if available) and whether the page thinks graphics
   are accelerated; the measured render-ahead; per view, late notes split
   into "stall" and "arrived inside the buffer", and dropped notes split by
   cause; and the service worker's state over time.
6. **Checks:** the table after the fix, with the large buffer and software
   graphics at 4x and 6x: no late notes, no bunching, nothing dropped for a
   full queue, in every view. `perf.mjs` against the baseline; the core
   baseline (a drop rule for late notes changes nothing on time); refusals,
   `--clicks`, `--endings`; tests three times. Bump `BUILD` and `CACHE`.
   In the PR, plain steps for the tester (as in #182).

## 40. Urgent: glitches with the screen on, worst on Library and Save/Share

A second tester (Pixel 10a, GrapheneOS, installed app) hears the music
glitch or clip, most often while he is on the **Library** (albums) or
**Save / Share** view. Screen on, page visible: not item 39's case. Both
testers use GrapheneOS; neither can send Diagnostics for about a day, so
fix what the harness can find before they test again. Mikey: a music app
must not glitch at all.

**The brain's lead (from the code; not tested):** visible, the scheduler
still hands notes to the core only `LOOKAHEAD_VISIBLE` (0.3 s) ahead.
Library and Save / Share are the heaviest views on the main thread (cover
art drawn per row, lists built, Design v17's blurs and glass over moving
content), so a long task there (scrolling, opening an album, building the
list, pasting a code) can hold the tick past 0.3 s and the notes arrive
late. A second possibility: the views' drawing and compositing starve the
audio thread itself (dropouts, heard as clicks or clipping) on a phone
with fewer fast cores free. The two need different fixes; the harness
tells them apart.

**Do:**
1. **Reproduce first,** visible, at 4x and 6x CPU throttle, full and
   lite, on the core: play a busy loop and a sung loop while the harness
   opens each view in turn (Now Playing, Vibes, Library with about 30
   saved loops and 5 albums, an album open, Save / Share), scrolls the
   long ones, opens an album, pastes a code, and switches views quickly.
   Per view, record the main thread's long tasks (count, longest, what
   they were), the longest tick gap, late notes on the core, late ticks,
   and the audio device's fill-ins (item 39's counter). Report the table
   before any fix. The `stalls.mjs` harness from #180 is the start.
2. **Fix the late-notes path for good:** with the core, keep a deep queue
   visible too (about 2 s, the hands' call from the table), and make every
   change that must be heard at once -- mute, re-roll, tempo, the Sound
   sliders if they reach queued notes, wander, a vibe loop starting --
   cancel what is queued and re-decide (item 39's `cancelAhead`). Each such
   change must still be heard within about 100 ms, as now: measure it. The
   JavaScript fallback keeps 0.3 s.
3. **Cut the views' main-thread cost** where the table points: covers drawn
   once and cached, not redrawn per scroll or per open; lists built once
   and updated, not rebuilt; nothing animating in views that are hidden;
   blur or glass effects that repaint every frame over moving content
   replaced by an equivalent that doesn't (keep the look; anything that
   changes it visibly is noted for a Painter session, for Mikey's eyes).
4. **If the audio thread itself drops out** with nothing late on the main
   thread, say so plainly with the numbers, and what would help (lite
   quality, a larger output buffer); stop and leave a note rather than
   guess.
5. **Field report, any view:** Diagnostics' report (item 39) covers the
   visible page too: since the last copy, per view, the time spent there,
   long tasks, the longest tick gap, late notes and dropouts. **copy**
   copies it all.
6. **Checks:** the table after the fix (no late notes in any view at 4x
   and 6x; dropouts reported before and after); `perf.mjs` against the
   baseline (no regression; record any win); the core baseline (no sound
   change); refusals, `--clicks`, `--endings`; tests three times. Bump
   `BUILD` and `CACHE`. In the PR, plain steps for the testers: play a
   busy loop, sit on Library and on Save / Share for a few minutes,
   scroll, open an album, then copy Diagnostics and send it.

## 39. Emergency: music skips and glitches when the phone is locked

**Before anything else.** A tester (Pixel 9, GrapheneOS, Driftloom
installed with Chrome's "Install app") hears the music skip and glitch as
soon as he locks his phone. Playing with the screen off is the main way
people will use this app.

**The brain's lead (Session 16, from the code; not tested):** when the page
is hidden, `engine.js` decides 3 s ahead but holds each note in `pending`
and hands it over only `buildAhead` before it sounds (about four tick gaps,
at least 0.3 s). That window was made for the JavaScript synth, where a
built voice costs the audio thread from the moment it is wired
(`docs/perf-baseline.md`, "a hidden tab costs the audio thread more"). Since item 32 the **core**
plays, and the window still applies: a note reaches the worklet a few
tenths of a second before it is due. If a locked phone delays the clock
worker's ticks by more than that (a throttled worker, a CPU parked on a
slow core, the renderer briefly frozen), notes arrive late and the core
drops or slurs them: skips and glitches. The perf baseline that showed "no
late ticks hidden" was measured on desktop Chromium before the core was
the default.

**Do:**
1. **Reproduce first, on the core and on `?engine=js`.** In `perf.mjs`
   (or a small harness beside it), hidden, with the CPU throttled (4x,
   6x), stall the clock worker's ticks on purpose -- gaps of 0.5, 1, 2 and
   5 s, now and then and back to back -- through a test-only hook that
   doesn't ship (as `core-paths.mjs` injects its traps). Count what the
   listener would hear: the core's `late` notes, the engine's
   `lateTicks` and `worstLateMs`, and the audio itself (dropouts, notes
   that never sound). Also, if Chromium allows it, freeze the page for a
   few seconds (`Page.setWebLifecycleState`). Report the table before any
   fix. If the lead is wrong, say what the harness shows instead and stop
   to leave a note if the cause isn't clear.
2. **Fix, the smallest change that clears it.** Candidates, in order:
   - On the core, hand every decided note to the worklet as soon as it is
     decided, hidden or not (no `pending` window): queued events cost the
     core nothing until they sound -- check that, and that its queue has
     room for several seconds of the busiest loops (shatter, cinder).
   - If stalls longer than the 3 s lookahead are plausible on a locked
     phone, widen the hidden lookahead on the core (say 8-10 s), and make
     sure **Next, Previous and Pause from the lock screen or headphones
     still act at once**: notes already handed to the core for the old
     loop must be cancelled, not played out.
   - Check the worklet's own headroom hidden at 4x and 6x on full and
     lite: if render quanta run long (underruns), say so; that is a
     different fix.
   The JavaScript fallback keeps its window (its reason still holds).
3. **A field report, because nobody here owns a Pixel with GrapheneOS.**
   Diagnostics gets a short "while hidden" section that resets each time
   the page is shown: how long it was hidden, the longest gap between
   clock ticks, late notes on the core, late ticks, the AudioContext's
   state changes, and whether the media session stayed `playing`. And a
   **Copy diagnostics** key that copies the whole Diagnostics text, so a
   tester can lock the phone for two minutes, unlock, copy and send it.
4. **Checks:** the harness table after the fix (no late notes at the
   stall lengths the fix claims to cover); `perf.mjs` hidden and visible
   against `docs/perf-baseline.md` (cost must not regress); the core
   baseline passes (no sound change); refusals, `--clicks` and
   `--endings` on the core; tests three times. Bump `BUILD` and `CACHE`.
   In the PR, say in plain words what a tester should do to confirm it on
   his phone (the installed app updates itself from `main`).

## 38. Vibes: pick the words, hear them in every new loop (top priority)

**Do this before items 34-37.** Mikey (2026-10-11): "I want to
incorporate this into the app immediately." The mood words become
**vibes** in the app: a listener picks some, and every new loop leans
toward them. It is a real feature, and the tuning tool for the words: he
picks vibes, listens, and sends the brain the loops that fit well or
don't. Two PRs, in order, in one hands session.

### 38a. The vibes in the generator (data, draw, share code)

- **`js/vibes.js`:** copy the `VIBES` block from `docs/VIBES.md` as it is
  (71 recipes, every pool-3 word), plus `VIBE_IDS`, the frozen order for
  share codes: alphabetical today, append-only after. Data only, no logic
  in that file: the brain edits it from now on (see "The words" in
  `docs/BRAIN.md`), so a test must check every recipe's fields and names
  (profiles, modes, voices, textures, bass styles: the brain's draft
  passes such a check already) and fail on any unknown one.
- **The per-loop draw:** `newSpec(seed, vibes)`, `vibes` the listener's
  selection. **With none selected, the spec is exactly today's**: no extra
  draws from the loop's stream; a test proves it on 1,000 seeds, and the
  balance lock, the core baseline and `/rank` don't move. With a
  selection, from a separate stream salted from the seed:
  - up to five vibes: all of them if five or fewer, else five at random,
    a new pick each loop;
  - their weights: one vibe is 100%; two to five share 100% with each at
    least 5% (a Dirichlet draw over the rest), so the fifth may be 5% one
    loop and 70% the next;
  - a strength, uniform 0.5 to 0.9.
  The spec stores `vibes` (name -> weight) and `vibeStrength`, on the q8
  grid like the rest.
- **Applying them** as `docs/VIBES.md` says ("How a recipe works"):
  targets pull a draw part-way (strength x the summed weights of the
  vibes that set it, toward their weighted mean); leans multiply weights
  by `value ^ (strength x weight)`. Where each lands:
  - in `newSpec`: `profiles` on the profile pool; `feel` on the loop's
    feel after the mood mixture and before the clamp to the profiles'
    ranges; `tempo` on the tempo draw's place in its band; `modes` on the
    scale weights after `moodWeighted`; `six8` on the 12-step metres (and
    gaits); `long` on bar counts of 8 and up; `swing` on its band; `tone`
    on the three tone draws (clamped to the blend's ranges).
  - in `characterOf(spec)`, so a code renders the same everywhere:
    `melody`, `chords`, `bass`, `bassStyle` and `texture` on those pools;
    `drums` on the kit's chance; `hats` on hat density; `rests` on
    `restBar`; `bigChords` on chord sizes of four and up.
  - Transform weights and positions only; never add draws to the loop's
    own stream, so a vibe loop's other draws stay tied to its seed.
  - Vibes that pull against each other average for now; Mikey's shapes
    over time (the helium balloon, `docs/MOODS.md`) come later.
- **Share codes:** `FORMAT` 4 appends the vibes (count; id and weight
  each; strength). Codes of formats 1-3 read with no vibes; a vibe loop's
  code round-trips to an identical render (test). Albums carry them as is.
- **A tuning tool for the brain:** `stats.mjs --vibes a,b,c` draws the
  corpus with that selection and reports, against no vibes: profile
  shares, mode bands (dark, inward, neutral, bright, as item 36), tempo
  median and spread, drums on, 6/8 share, bar counts, the top melody
  voices, and the realized weights. `stats.mjs --vibe-each`: one line per
  vibe on its own (the same figures), so the brain can tune all 71
  without ears first.
- **Checks:** refusals on full and lite for 40 loops drawn with random
  selections of 1, 3, 5 and 10 vibes; their loudness spread against the
  catalog's (`measure.mjs`; vibes may lean toward louder profiles: report,
  don't fix); render cost unchanged. Tests three times. Bump `BUILD` and
  `CACHE`.

### 38b. The Vibes button and view

Mikey's sketch (two phone screenshots; he attaches them to the hands
session): on the **Now Playing** view, a **Vibes** key at the top left of
the view's panel -- where the red box is, left of an album's title row
("Joyful?" and its dots when an album plays).

- Tapping **Vibes** opens the **Vibes view** in the same panel (the
  dynamic zone). The key becomes **Close** (same place); **Clear All**
  sits at the top right and deselects every vibe. Close returns to Now
  Playing.
- The view: every vibe in `VIBE_IDS` order, as buttons in **two columns**
  (the yellow rectangles), tap to select or deselect, the selected ones
  clearly marked in every theme. **This view scrolls** (Mikey: views
  usually don't, this one does because there are many vibes); its header
  row (Close, Clear All) stays put.
- **No limit on how many** are selected: past five, each new loop picks
  five (38a). The selection is saved in the prefs and survives a reload.
- **When it applies:** to every new loop drawn after the change (Next,
  the next loop when one ends, anything else that calls `newSpec`; name
  them in the PR). Not to the loop playing, an album's tracks or a saved
  loop.
- **The readout:** a loop with vibes shows them in the details, in the
  same style the mood line has today ("100% enthusiastic"), weights
  included, in place of the mood line; a loop without vibes shows the mood
  line as now. (The weights are what the loop was steered by, not yet
  measured from the sound; the honest measured share comes with the dials,
  items 34-35. For Mikey's ears.)
- Design v17's tokens and the nine themes; buttons sized for a thumb;
  works at phone width. Keep it plain: a Painter session can refine it
  later (Mikey). Screenshots in the PR on a phone-width viewport in two
  themes, the view closed and open. Bump `BUILD` and `CACHE`.

## 34. The ten dials, measured: `tools/dials.mjs` (measure only)

Mikey's rankings say which words fit which loops; to turn that into word
regions on the maps, every ranked loop needs a position on the ten dials
(`docs/MOODS.md`, "Dials" and "Maps and clouds"). Item 19 left this as
"a later item"; this is it. A tool, no app change: `BUILD` and `CACHE` stay
as they are; the balance lock and the core baseline don't move.

- **Input:** a `/rank` export (`.jsonl`; measure each distinct `code`
  once) or song codes on the command line. Decode with `decodeSong`, then
  `render(spec)` as `stats.mjs` does, so the dials are measured on the
  pattern as heard: after entry schedules and gaps, with no wander (`/rank`
  plays the loop as composed, `driftOn = false`).
- **The ten dials, 0 to 1,** each from the loop's own notes, and each
  documented in a comment at its function: what it counts and why that is
  the dial. Starting points; where the brain is vague, the hands' call:

  | dial | low end -- high end | measure from |
  |---|---|---|
  | lift | inward -- bright | the mode's `SCALE_BRIGHTNESS`, and the share of chord time on chords with a major third |
  | energy | still -- lively | tempo, sounding onsets per second over all layers, drum hits per bar |
  | warmth | cool -- warm | the spec's tone warmth and where its voices sit in the spectrum (from figures already measured per voice, or a spectral centroid with `--audio`, below) |
  | room | close -- wide | tone space, chord spread (lowest to highest note), the tune's span |
  | ground | floating -- grounded | share of beats with a kick or bass onset; notes tied over the beat pull it down |
  | edge | soft -- crisp | the voices' attacks (struck and plucked against sustained), median note length, the 2-5 kHz share with `--audio` |
  | height | low -- high | sounding-weighted median pitch of melody and texture |
  | bounce | flowing -- springy | dotted and syncopated onsets, short notes, melodic leaps of a fourth or more |
  | tang | sweet -- tart | chord time with added seconds, a b9 or a semitone cluster, against plain triads |
  | oddness | familiar -- strange | the mode (whole-tone, hirajoshi, phrygian: far from ionian), tritone and augmented steps in the tune, how rare the loop's pairing of voices is in the catalog |

- **Scaled by the catalog:** map each dial's raw value to its percentile
  in a fixed corpus (2,000 loops at `stats.mjs`'s default seed, written to
  a small JSON in `test/` so the scale doesn't move between runs), so 0.5
  is the catalog median. Print the raw values too. When a later generator
  change moves the corpus, regenerating the scale is a deliberate step
  (`--write-scale`), like the balance lock's.
- **A cloud, not only a point:** per bar wherever the dial means something
  bar by bar (lift, energy, ground, height, bounce and tang at least), each
  bar weighted by how much sounds in it; the loop's point is the weighted
  mean. Per layer is welcome for height and edge if it is cheap. Say which
  dials stay loop-wide and why.
- **Audio is optional** (`--audio`): the score dials need no rendering. If
  warmth and edge read better from audio (through `measure.mjs`'s pipeline,
  on the core), add it behind the flag and report which reads better, and
  what it costs.
- **Output:** a table (a row a loop, ten columns) and `--json` with the raw
  values, the scaled values and the per-bar clouds, keyed by code.
- **Yardsticks, reported in the PR:**
  - Deterministic: the same code twice, the same numbers.
  - Fast: the time for 181 codes and for 500 (score dials only).
  - **Ten different dials:** over the 2,000-loop corpus, the 10x10
    correlation table. Flag any pair above 0.7; if two dials measure the
    same thing, say so and propose the fix rather than shipping twins.
  - **Face validity:** each dial's mean by profile (shatter and cinder high
    on energy, haven and vapor low, and so on), and lift against
    `feel.lift` (it should rise with it). Report anything that surprises.
  - **Checked against Mikey's ears:** his 181-trial export (2026-10-11,
    v85, seeds 19000-19173; he attaches it to the hands session). The
    brain's scratch check found the energy words separating cleanly on
    tempo, notes per second and drums (energetic, bouncy, lively,
    enthusiastic, adventurous and nimble up; cozy, tiptoeing, meditative,
    peaceful, soothing and foggy down), and happy and sunny against
    reflective on mode brightness. The energy and lift dials must show the
    same, at least as clearly; report the fits' and doesn't-fits' means
    for those words. Without the export, regenerate the codes as `/rank`
    does (`newSpec(19000 + n)` on v85, `encodeSong`) and check the first
    against
    `DL1-0C07C-1R60G-60789-DFZDM-QV19C-NRNCG-FKYW6-B9P0G-ENS6B-KR001-53G0G-8YC3G-R0G3Y-W10HC-XZDMW-6WC00-00000-04CK0`;
    the taps need the export.
- Tests in `test/`: a still loop low on energy and a fast one high,
  percentiles inside 0..1, determinism. Tests three times.

## 35. Fitting the words, and building word albums (measure only)

With the dials (item 34), Mikey's answers can place the words, and the
brain can guess loops for a word. Votes, not verdicts (his words: "I might
have selected different options on a different day"), so the fit is soft
and says how sure it is. Tools, no app change. Since Mikey's 181 trials
(`docs/MOODS.md`), the test is moving to **word albums**: one word, about
eight loops, "which of these don't fit?" (item 37). The fit feeds the
guesses, and the albums' answers feed the fit.

**(a) `node tools/ranks.mjs <file.jsonl> --fit`** measures each distinct
code with item 34 (or reads `--dials <file.json>`) and, per word:
- **fits** against **doesn't fit**, from the five-word trials (tapped
  against shown but untapped; a tap weighs less the later it came: 1, 0.8,
  0.6, 0.5, 0.4, tunable, say what was chosen) and from word-album answers
  (item 37: fits / doesn't fit; "not sure" left out);
- per dial: the weighted means of both, their standardized difference, and
  an 80% bootstrap interval (trials resampled 1,000 times, seeded);
- the word's **region**, kept lean: only the dials whose interval stays
  clear of zero, and **at most three** unless the data clearly carries
  more; a center and a spread on each; every other dial "doesn't care".
  Fewer than 8 answers: "too few", no region.
- **Words that fit almost everything or almost nothing** (90%+ or 10%-
  of the times shown, with 8 or more answers) are listed apart: no
  region, because they don't tell loops apart (steady, cool, wistful and
  zany fit nearly every loop in his 181; heartwarming and joyful almost
  none).
- **The honest check, against the right baselines:** leave one trial out
  at a time, fit from the rest, order that trial's five words by how
  likely each is to fit the loop, and count how often the top word is
  his first tap. Report it next to (1) chance, 20%; (2) **base rates
  alone** -- each word's overall fit rate, ignoring the loop, which got
  38% on the 181 trials, while the brain's scratch fit on spec features
  got 32%; and (3) his own repeats' agreement on the first word (4 of 7).
  A fit that doesn't beat base rates says so in its first line.
- Repeats: keep the agreement report; add, per word, how often its fits /
  doesn't-fit answer flipped.
- Output: a readable report by group, region dials first, and `--json`
  for the brain. Pools 1-3 and albums all count; say how many of each.

**(b) `node tools/wordalbum.mjs`** builds a word album for the brain:
- `--word <w>` and a **target**: the word's fitted region from (a), or
  dial ranges written by the brain (`--target "warmth>0.7,lift 0.55-0.8,
  energy<0.5"`), so the brain can guess where the data is thin.
- Draws a fresh corpus (`--n 3000`, seeded), measures it with item 34,
  and picks `--guesses 6` loops closest to the target and `--decoys 2`
  far from it on its main dials, each from a different profile where it
  can, 4 bars or longer, `playFor` set to about 40 seconds; shuffled.
- Prints the album code (`encodeAlbum`, title "<Word>?"), the key (track,
  guess or decoy, profile, mode, tempo, the target dials' values) and
  the `/rank` link for item 37.
- `--seed` makes it reproducible; the key is derivable by re-running it,
  so nobody writes it into `docs/ALBUMS.md`.

- Run (a) on the 181-trial export and put the report in the PR. Tests:
  in a synthetic export where one word is always tapped on loops high on
  one dial, that word gets a region on that dial and nothing else; a word
  tapped at random gets none; `wordalbum.mjs` with a target picks loops
  inside it and decoys outside. Tests three times.

## 36. Two mode decisions: reflective may be neutral; cinder and shatter may be bright

**Reflective: skipped (Mikey, 2026-10-11).** "Reflective already reaches
the in-between modes about half the time, which is what I wanted." The
measurement (Notes from Claude Code, below) found reflective loops 53.8% in
the neutral band already, so only the cinder and shatter half is built.

Two decisions from Mikey (2026-10-11) on what #107 (item 16) did to the
mode draw. A generator change for **new loops only**: a share code writes
its mode down, so every existing code, and every `/rank` trial, renders
exactly as before.

- **Reflective:** "reflective leaning brighter since #107 is okay, but
  reflective can also be neutral." Below `INWARD_FROM` the inward pull
  centers on 0.4 and all but rules out every mode above about 0.6, so a
  reflective loop is inward (dorian, kumoi, minor pentatonic, insen) and
  rarely in the middle of the axis. Widen it so reflective loops also reach
  the **neutral** modes -- the middle of `SCALE_BRIGHTNESS`, about 0.44 to
  0.75 (whole-tone, dorian, akebono, kumoi, ritusen, mixolydian) -- while
  the darkest (phrygian, harmonic minor, phrygian dominant) stay pulled
  away as now and the glad end (yo and up, 0.82 and over) stays rare. How
  (the pull's center, its width, or both) is the hands' call.
  **Measure first, then after:** for 10,000 fresh loops whose top mood is
  reflective, the share of each mode and of four bands: dark (0.2 and
  under), inward (0.21-0.43), neutral (0.44-0.75), bright (0.76 and up).
  Aim: neutral up to roughly a third or more; inward still the largest or
  close to it; dark no higher than today; bright under about 10%. The
  other seven moods' tables must not move beyond noise (check that only
  reflective reaches below `INWARD_FROM` in practice).
- **Cinder and shatter:** "cinder and shatter may take a bright mode when a
  word calls for one." Their pools hold no bright mode, so a happy cinder
  or shatter loop is never bright. Today the words that call for
  brightness are the glad moods (`feel.lift` above `GLAD_FROM`: joyful,
  happy, enthusiastic, refreshing); later the word recipes call the same
  hook. Give a profile an optional list of **borrowed bright modes**,
  weight 0 unless the glad tilt is on and growing with it as the tilt
  does:
  - **cinder:** ionian and mixolydian -- flamenco's own major forms (the
    alegrías, literally "joys") and the village fiesta `docs/MOODS.md`
    imagines for *merry* on cinder;
  - **shatter:** lydian and mixolydian -- bright and still restless.

  Cinder's progressions spell chords for its minor-ish modes (the
  Andalusian cadence's major V; the added F over the E chord, which
  stays). In a borrowed major mode, check what each progression becomes:
  keep the Andalusian moves and the added F in the modes they belong to,
  and give a bright cinder loop progressions on a major tonic (I-IV-V,
  I-V), the 6/8 strum and hand kit unchanged. Whatever is chosen, write it
  down **for Mikey's ears**. No other profile changes.
  Report: the share of glad cinder and shatter loops (top mood joyful,
  happy or enthusiastic) in a bright mode, before and after; and the same
  over all cinder and shatter loops (it should stay small: brightness only
  where a word calls for it).
- **Share codes:** a cinder loop in ionian and a shatter loop in lydian
  encode and decode (every mode is in `SCALES`, so they should).
- **Checks:** the balance lock will move a little: report which figures
  and by how much, re-baseline at `--n 10000`, and say so. The core
  baseline: if any of its loops re-draw, `--update` and say which and why;
  if they are fixed codes, it passes unchanged. If item 34's dial scale
  (`test/`) moves, regenerate it and say so. Refusals on full and lite for
  40 new glad cinder and shatter loops and 40 new reflective ones. Tests
  three times. Bump `BUILD` and `CACHE`.
- **For the album:** list in the PR 6 new reflective loops in neutral
  modes and 6 glad cinder and shatter loops in bright modes (seeds and
  codes), so the brain can build the listening album from them directly.

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
  **Where it is written down (brain, Session 24, 2026-10-19):** to be
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

## Notes from Claude Code

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

## Done

- #59, fiddle and accordion take two (v44),
  https://e2c802cd-driftloom.kruu-mikey-thaiculture.workers.dev: a rounder,
  softer fiddle and accordion with no rasp or buzz; short notes speak at
  once and joined fiddle notes slur; vibrato only on long fiddle notes; the
  accordion's shimmer slow and gentle, never dropping out on a held note.
- #62, tide part 2a (v45),
  https://23353f47-driftloom.kruu-mikey-thaiculture.workers.dev: waltz
  tunes leaning on the beat with pickups into the bar; quick grace notes
  (cuts and turns) on some of tide's beat notes; in some loops a second
  line in thirds or sixths under the held notes. Listen for graces that
  sound played rather than glitchy, and a harmony that stays underneath.
- #63, tide part 2b (v46),
  https://778e624e-driftloom.kruu-mikey-thaiculture.workers.dev: on about
  a quarter of tide's drum loops a frame drum and tambourine play the kit's
  part (round low skin, soft tipper taps, zils above the harsh band); and
  waves as the air, slow swells of filtered sea. Listen for a hand kit that
  sits where the kit did, and waves that stay a background.
- #64, the harsh leads softened (v47),
  https://c5c031fd-driftloom.kruu-mikey-thaiculture.workers.dev: saw, moog
  and analoglead without the rasp or the wah on every note -- each still
  itself (the saw a pluck, the moog a little movement, the analoglead its
  detuned shimmer). Listen for leads that no longer grate, and whether the
  moog now reads as too dull.
- #65, nylon guitar and pan flute (v48),
  https://ce292354-driftloom.kruu-mikey-thaiculture.workers.dev: a nylon
  guitar strumming tide's chords (down on the beat, up off it, the upstroke
  lighter) and a breathy pan flute among tide's tunes. Listen for a strum
  that sounds played rather than smeared, and a pan flute whose breath is
  air, not hiss.
- #66, cinder, the fiery 6/8 (v49),
  https://d7e09d62-driftloom.kruu-mikey-thaiculture.workers.dev: a new
  profile, fast and Spanish-tinged -- a driving 6/8 on frame drum and zils,
  a nylon guitar strumming the Andalusian cadence (Am-G-F-E, ending on a
  major E) with the tune taking G# over that E. Listen for the fall landing
  bright rather than clashing, and whether the b9 (F) the E chord sometimes
  takes should go.
- #67, wayfare, the rolling train (v50),
  https://6d5ae00b-driftloom.kruu-mikey-thaiculture.workers.dev: a new
  profile for traveling -- a walking 4/4, pan flute and fiddle over
  accordion, nylon and harp, and in half its loops a chug, a short bass
  note on every eighth with the brushes swishing along. Listen for wheels
  on rails rather than a machine gun, and whether I-IV-v-IV's minor v in
  mixolydian and dorian should go.
- #74, pan flute without the static (v51),
  https://14260a7b-driftloom.kruu-mikey-thaiculture.workers.dev: the
  static was tide's grace notes, which held for 0.4 s under the note they
  led into and were cut off with a click; now they flick into the note and
  let go (pan flute, and the flute, ocarina and analoglead, which had the
  same fault). Listen for clean graces in tide's pan flute and ocarina
  tunes, and a pan flute that otherwise sounds exactly as before.
- #75, a cleaner nylon strum (v52),
  https://178f4bb4-driftloom.kruu-mikey-thaiculture.workers.dev: the strum
  crosses the strings in 10-20 ms (was 15-30), and a strum still held when
  the next one starts lets go under it rather than ringing on. Listen for
  each strum landing as one gesture, and a new chord no longer sitting on
  the old one's strings (clearest in 6/8); the guitar's tone is unchanged.
- #79, wayfare's drums and chug (v53),
  https://d90d26df-driftloom.kruu-mikey-thaiculture.workers.dev: wayfare
  plays grooves of its own -- the kick on 1 and 3, a rim or soft snare on 2
  and 4, light eighths, no boom-bap and no claps -- with a softer, rounder
  kick on tape and brush, the drums a little under where tide's jigs sit;
  and the chug leans on the beat with the eighths between ghosted, short
  notes, the brushes lighter with it. Listen for a light, steady traveling
  beat under the same tunes, a kick that thumps rather than punches, and a
  chug that bounces ("chug-a") like wheels on rails rather than a machine
  gun -- and whether the chug's bass, 4.4 LU lighter, is now too faint.
- #82, wayfare's drums with their life back (v54),
  https://edcf8d59-driftloom.kruu-mikey-thaiculture.workers.dev: the same
  soft wayfare kit as #79 -- kick on 1 and 3, a rim or soft snare on 2 and
  4, no boom-bap, no claps, the chug as it was -- but no longer one bar
  over and over: four-bar phrases that swell toward a fill or a soft
  roll, an answer bar every so often with a pickup kick and an open hat,
  ghost notes before the backbeat, and a shaker or hat coloring the
  offbeats. Listen for drums that move and breathe across the phrase
  again, as in v52, while still sounding like v53's gentle kit; and
  whether the fills or the pickup kick ever tip it back toward tough.
- #88, the performance harness and baseline (item 10; `js/` untouched,
  still v54), https://15986051-driftloom.kruu-mikey-thaiculture.workers.dev:
  nothing to listen for, it measures and changes no sound. What it found
  is in `docs/perf-baseline.md`: no dropouts in any run; the main thread
  goes mostly on repainting the whole page every frame; finished voices
  keep the audio thread busy until the garbage collector finds them
  (40-70% of its work); a hidden tab costs the audio thread more, not less.
- #89, sound polish (item 11, v55),
  https://8de059b7-driftloom.kruu-mikey-thaiculture.workers.dev: sung notes
  (vowel, hum, choir) no longer leap out when a note lands on a formant --
  the loud ones come down, nothing is lifted, the vowels' color is the
  same; 5/4 chords fill the bar, felt 3+2, instead of stopping after four
  beats; keys arpeggios stop with their chord instead of ringing into the
  next one. Listen for hollow and choir tunes that stay even, thaw's 5/4
  loops, and arpeggios that no longer smear. The temple bell is unchanged
  (see Notes).
- #92, composition depth, first pass (item 12, v56),
  https://b8b54753-driftloom.kruu-mikey-thaiculture.workers.dev: about a
  third of the loops of 8 bars or more develop -- a statement, a departure
  that answers it with its own motif turned (new rhythm, upside down, or
  slowed), usually higher, over its own chords, and a return that
  restates the motif another way. Every other loop plays exactly as
  before. Listen for long loops that go somewhere and come home, a middle
  that sounds like an answer rather than a new tune, and whether a third
  is too many or too few (`DEPTH` in `js/generator.js` is the knob).
- #96, paint (item 14.1, v57),
  https://762d584a-driftloom.kruu-mikey-thaiculture.workers.dev: nothing
  to hear; the playhead lights and the layer cursors now move without
  repainting the page (paint 33 -> 0.7 ms/s, the main thread 60 -> 43 ms/s
  at normal speed, 338 -> 223 at 6x). Look for the same playhead and
  cursors; and whether a light that switched off instantly would do, which
  would save another ~12 ms/s (the 90 ms glow-out is kept for now).
- #100, finished voices let go (item 14.2, v58),
  https://c83c1eb5-driftloom.kruu-mikey-thaiculture.workers.dev: nothing
  to hear -- every note of every voice renders identically with and
  without it (`measure.mjs --retire`). The audio thread does 25% less
  work at normal speed and 44% less when the phone is busy, and no longer
  grows with load. Measured A/B on one machine (`perf.mjs --ab`), since a
  container restart moved the session to another mid-item.
- #102, hidden tab (item 14.3, v59),
  https://f18c1275-driftloom.kruu-mikey-thaiculture.workers.dev: nothing
  to hear; with the screen off or the app in the background, each voice is
  now built just before its note instead of up to 3 s early, and the
  audio thread does about 31% less work there. Listen for music that
  carries on with the phone locked exactly as before -- no gaps, no late
  notes -- including after a long time in the background.
- #103, the graph that always runs (item 14.4; `js/` untouched),
  https://6ddfb7ef-driftloom.kruu-mikey-thaiculture.workers.dev: nothing
  to hear. What the always-on reverb, dynamics, tape wobble and channels
  cost, part by part (in `docs/perf-baseline.md`); no saving leaves the
  sound identical. Four that would change it are proposed for Mikey's
  ears: four reverb combs instead of six, no saturator oversampling on
  full, one dynamics stage instead of two, a coarser wobble.
- #104, page weight (item 14.5, v60),
  https://01076bc2-driftloom.kruu-mikey-thaiculture.workers.dev: nothing
  to hear; a first visit now sends half what it did (233 KB gzipped, was
  467), the unused WAV keepalive no longer precached. Look for the lock
  screen controls still appearing when playing, and the app still working
  offline after one visit.
- #105, the playhead without its fade (item 14.6, v61),
  https://c251be28-driftloom.kruu-mikey-thaiculture.workers.dev: nothing
  to hear; the playhead's lights now switch on and off with the beat
  instead of glowing out over 90 ms, and the main thread does a quarter
  less work (61 to 46 ms a second at 1x, paint 15 to 10). Look for the
  playhead still reading clearly on full and on lite.
- #106, the temple bell as a chord voice (item 15, v62),
  https://ca65ea28-driftloom.kruu-mikey-thaiculture.workers.dev: listen
  for a loop whose keys are the temple bell -- its chords 1.9 dB up, now
  level with the other chord voices on held chords (was 1.7 LU under).
  On short chords it now sits 2.7 LU above them, since it rings 6.5 s
  whatever the note: the long-note reading decided. The bell as a melody
  is unchanged.
- #107, the mood steers the mode (item 16, v63),
  https://270efce4-driftloom.kruu-mikey-thaiculture.workers.dev: new loops
  whose mood is joyful, happy or enthusiastic now come out in bright modes
  (lydian, ionian, major pentatonic, mixolydian ...) about 80% of the time,
  was under 60%; "90%+ happy" in a minor-third mode went from 38% to 18%.
  Reflective loops lean inward -- dorian, kumoi, minor pentatonic -- away
  from both the bright and the darkest modes (for Mikey's ears: is that
  inward, not sad?). Soothing, peaceful and comforting as they were.
  Cinder's and shatter's pools have no bright mode, so a happy cinder or
  shatter loop still is not bright. Loops that already exist are
  unchanged (a code keeps its mode); the labels and display too. The mood
  words, feel targets and code numbers now live in `js/moods.js`.
- #108, composition depth, second pass (item 17, v64),
  https://0ee8da3d-driftloom.kruu-mikey-thaiculture.workers.dev: a 16-bar
  loop that develops is now four four-bar phrases -- A, A turned, B, A
  turned again. Listen for the tune moving at bar 5 (same rhythm, its end
  going somewhere else over a new cadence), B at bar 9 in a new register
  and rhythm over its own chords, and a return that is A but not A again.
  B's tune is always heard now (an airy loop used to rest it away in one
  developing loop in five). About two thirds of 24-bar loops develop (was
  57%). Simple loops are untouched, note for note. 32 bars: see the note
  above.
- #111, housekeeping: comments and README against the code (item 18; no
  version bump), https://11e02ab0-driftloom.kruu-mikey-thaiculture.workers.dev:
  nothing to hear -- comments and docs only, and `stats.mjs` renders the
  same corpus byte for byte. Stale counts, "not yet" notes about things
  that shipped and quoted figures were corrected in the code's comments
  and the README (the list is in the PR). One thing for Mikey's ears came
  out of it: short loops still take short gaps, a whole bar of silence in
  about one 2- or 4-bar loop in thirteen (see the note above).
- #117, the word-ranking test (item 19, v65),
  https://51ca22bb-driftloom.kruu-mikey-thaiculture.workers.dev/rank.html:
  nothing changes in the app. Open `/rank.html`, tap Start, and rank the
  five words for each loop, most like it first, stopping when the rest
  don't fit. Check that the loop starts on its own after Next, that the
  buttons are easy to hit on the phone, and that Export downloads the file
  (`node tools/ranks.mjs <file>` reads it). Drift is off on this page so
  each recorded code is exactly what was heard.
- #122, the ranking test's second pool (item 20, v66),
  https://6e6a5116-driftloom.kruu-mikey-thaiculture.workers.dev/rank.html:
  nothing changes in the app. Each loop now offers one word from each of
  the five maps (color, shimmer, air, motion, flavor), so the five can't
  bunch on one corner and fast, tense loops get words to reach for. Saved
  v1 answers stay and the run carries on from the next seed; new trials
  are marked `"pool": 2`, and `tools/ranks.mjs` reports per pool and per
  group. Check that a fresh trial shows a spread of words, and that your
  earlier answers are still counted.
- #126, the Rust core's first step (item 21, v67),
  https://09d9e12a-driftloom.kruu-mikey-thaiculture.workers.dev/?engine=rust:
  nothing should sound different. Without the flag the app is unchanged;
  with `?engine=rust` the kalimba comes out of Rust (it nulls against the
  JavaScript kalimba to about -100 dB). Check on the phone that
  Diagnostics reads `engine: rust  late: 0  fallback: 0` while a grove
  loop plays, and that the kalimba is there. If Diagnostics says
  `rust (failed: ...)`, the phone's browser cannot run the module (it
  needs Safari 15 or later) and everything played in JavaScript.
- #127, the Rust core's second step: fiddle and pad (item 22, v68),
  https://767f07a1-driftloom.kruu-mikey-thaiculture.workers.dev/?engine=rust:
  nothing should sound different. With `?engine=rust` the fiddle and the
  pad come out of Rust as well as the kalimba (they null against the
  JavaScript voices to about -105 and -117 dB). Play a tide loop (the
  fiddle) and one with the pad, and check that Diagnostics reads
  `engine: rust  late: 0`. `fallback` may read 1 or 2 just after a fresh
  load: the first chord played in JavaScript while the core was still
  arriving. If Diagnostics says `rust (failed: ...)`, the phone's browser
  cannot run the module (it needs Safari 15 or later).
- #133, the Rust core's third step: the FM voices, sine and tubular (item 23, v69),
  https://23327644-driftloom.nowmichaelclark.workers.dev/?engine=rust:
  nothing should sound different. With `?engine=rust`, keys, bell, celeste,
  music box, Rhodes, marimba, harp, piano, sine and the tubular bell (and
  prepared piano's two strikes; its knock is still JavaScript) come out of
  Rust; they null against the JavaScript voices to between -88 and -121 dB.
  Play a loop with keys or a bell and one with tubular bells, and check
  Diagnostics reads `engine: rust  late: 0`; `fallback` still counts bass,
  texture and drum notes, which move in later items.
- #134, the Rust core's fourth step: the wave-table voices (item 24, v70),
  https://20650621-driftloom.nowmichaelclark.workers.dev/?engine=rust:
  nothing should sound different. With `?engine=rust`, softpad, analogpad,
  analoglead, the saw pluck, the square beep, the moog and the whistle come
  out of Rust, on five new wave tables (they null against the JavaScript
  voices between -106 and -119 dB). Play a loop with a lead (the whistle's
  slides are the thing to listen for) and one with a pad, and check
  Diagnostics reads `engine: rust  late: 0`. The core's memory is now about
  7 MB, up from 4.
- #136, the Rust core's fifth step: accordion and nylon (item 25, v71),
  https://58895670-driftloom.nowmichaelclark.workers.dev/?engine=rust:
  nothing should sound different. With `?engine=rust` the accordion and the
  nylon guitar come out of Rust, through their bodies (accordion -115 to
  -117 dB, nylon about -100 dB against the JavaScript voices). Play a loop
  with the accordion (glade, tide) and one with strummed nylon chords
  (cinder, wayfare): each strum should still stop the last chord's strings.
  Also in it: the measure harness no longer runs out of memory on
  `--engine rust`. Core memory about 9 MB.
- #137, the Rust core's sixth step: bandpass, highpass and the noise
  source (item 26, v72),
  https://b2222e78-driftloom.nowmichaelclark.workers.dev/?engine=rust:
  nothing should sound different. With `?engine=rust` the stab, ocarina,
  flute, pan flute, prepared piano's knock and temple bell come out of Rust,
  breath and strikes included (-99 to -126 dB against the JavaScript
  voices). Play a loop with an ocarina or flute and one with temple bells,
  and check Diagnostics reads `engine: rust  late: 0`; `fallback` now counts
  only bass, texture, drum and sung notes. Core memory about 11 MB.
- #139, the one-frame click (item 26b, v73),
  https://64580854-driftloom.nowmichaelclark.workers.dev/?engine=rust:
  nothing to listen for but the absence of rare ticks. A noise voice
  started a hair after a whole frame (about 1.4% of noise starts, and one
  loop in four had one) let one loud sample through; it is gone in both
  engines (`measure.mjs --clicks`: 26 voices clicked, none do now). Hats,
  snares, claps, the bells and the breath voices are where it was. Nothing
  else changes, apart from the tick.
- #140, the Rust core's seventh step, first half: bass and textures (item 27a,
  v74), https://0f217058-driftloom.nowmichaelclark.workers.dev/?engine=rust:
  nothing should sound different. With `?engine=rust` the bass voices (sub,
  round, fifths, pluck, moog; the Rhodes bass through the FM voice) and the
  swell, drop, wind and waves textures come out of Rust (-94 to -147 dB
  against the JavaScript voices), and the core now serves all five channels.
  Play a loop with a bass line and one with the wind or the sea (tide's waves)
  and check Diagnostics reads `engine: rust  late: 0  fallback: 0`. Drum
  notes still play in JavaScript until the next step. Also in it: a core bug
  at frame-exact note starts (a loop at 11.3 s, 44.1 kHz nulled at -38 dB)
  fixed for every voice. Core memory 10.4 MB.
- #141, the Rust core's seventh step, second half: the drums (item 27b, v75),
  https://d282b713-driftloom.nowmichaelclark.workers.dev/?engine=rust:
  nothing should sound different. With `?engine=rust` every drum comes out of
  Rust (kick, soft kick, snare, clap, hats, shaker, rim, and tide's frame,
  tap and jingle; -117 to -148 dB against the JavaScript drums), so the core
  plays all five channels. Play tide's hand kit and a house or wayfare loop,
  and check Diagnostics reads `engine: rust  late: 0  fallback: 0`. On the
  drum loops measured (`perf.mjs --ab`) the audio thread runs 4-5% busier,
  the main thread 9-13% lighter, and no Web Audio nodes are made at all (110
  a second before); no late ticks or device fill-ins either way. Only the
  sung voices still play in JavaScript. Core memory 10.4 MB.
- #143, three small fixes before the sung voices (item 27c, v76),
  https://e6fb9fa3-driftloom.nowmichaelclark.workers.dev/?engine=rust:
  nothing to listen for. With `?engine=rust`, a note that starts or stops a
  hair past a whole sample (one time in about two thousand) now plays
  exactly as JavaScript plays it; before, the core could play its breath or
  noise a sample late for the whole note (the pan flute's -68.5 dBFS in the
  brain's check; snares and hats too, when they landed there). Also: the
  first-value fix from 26b no longer uses the `value` setter, which a
  browser could apply after a late note's envelope (Chromium doesn't), and
  a jingle the core can't take no longer draws its zils twice.
- #144, the Rust core's eighth and last step: the sung voices (item 28, v77),
  https://d03e5106-driftloom.nowmichaelclark.workers.dev/?engine=rust:
  nothing should sound different. With `?engine=rust` the vowel, the hum and
  the choir come out of Rust (-96 to -99 dB against the JavaScript voices,
  6-second notes with a drifting vowel included), so every voice does now.
  Play a `hollow` loop and check Diagnostics reads `engine: rust  late: 0
  fallback: 0`; listen for the scoop into each note, the vowel opening on
  long ones, and the breath at the start. Core memory 12.4 MB.
- #147, the Rust core's ninth step: the mix (item 29, v78),
  https://8b136266-driftloom.nowmichaelclark.workers.dev/?engine=rust:
  nothing should sound different. With `?engine=rust` the core mixes: the
  channel gains and mutes, the reverb and echo sends, the kick's duck, the
  echo, the reverb's combs and pre-delay, and the tails' fades, into the
  master chain at `preBus` (a mix-stage null at -143 to -175 dB; whole
  loops at the JS-against-JS floor). Play a roomy loop (haven, tide),
  re-roll a few times, mute and unmute a layer, and check Diagnostics reads
  `engine: rust  late: 0  fallback: 0`. Core memory 14.4 MB.
- #150, the worklet's block count (v79),
  https://db3590ea-driftloom.nowmichaelclark.workers.dev/?engine=rust:
  nothing to listen for. Chromium sometimes hands the core's worklet the last
  block's frame number while it renders the next; the core then played that
  block's envelopes 128 frames late (about one note in fifty on item 30's
  build, none seen on item 29's). The worklet now keeps its own count.
- #148, the Rust core's tenth step: wobble, saturator, tone and highpass
  (item 30, v80),
  https://10e757b3-driftloom.nowmichaelclark.workers.dev/?engine=rust:
  nothing should sound different. With `?engine=rust` the core runs the
  first half of the master chain after its mix: the tape wobble (its 14 ms
  delay and the wow and flutter LFOs, phased from when the synth started
  them), the saturator (Chromium's curve lookup, and on full its 2x
  oversampling, the downsampler an FFT as Chromium's), the tone lowpass and
  the 38 Hz highpass; its output goes to the bus compressor. Mix stage -77
  to -85 dB against a JS-vs-JS floor of -78 to -97 dB (the wobble's
  last-bit LFO differences; -116 to -136 dB with it held still); whole
  loops at the floor. Performance, counterbalanced against item 29's core,
  120 runs a side: device fill-in runs 4 against 5; audio render +4% (full
  +7%, lite -3%). The long render quanta seen along the way are the
  sandbox VM's, in both builds (Notes from Claude Code). Play a warm,
  wobbly loop and check Diagnostics reads `engine: rust  late: 0
  fallback: 0`. `.wasm` 149,440 bytes.
- #156, the Rust core's eleventh step: the compressors and the whole mix in
  the core (item 31, v81),
  https://6a4f8d1d-driftloom.nowmichaelclark.workers.dev/?engine=rust:
  nothing should sound different. With `?engine=rust` the core runs the
  whole master chain -- after the wobble, saturator, tone and highpass, the
  bus compressor and the ceiling (Chromium's `DynamicsCompressorNode`,
  ported), the volume and the stop/start fade -- and its output goes
  straight to the speakers; Web Audio carries only fallback notes. Whole
  loops null at the JS-vs-JS floor (-85 to -87 dB), the loudest at full
  volume too, with the ceiling limiting up to 1.4 dB; with the same signal
  into both chains, the core sits at the floor Chromium's own compressor
  gives a last-bit change. Audio render 7-11% lighter than item 30's core.
  Play a loud loop with the volume up, stop and start a few times, change
  the quality, and check Diagnostics reads `engine: rust  late: 0
  fallback: 0`. For his ears: both engines' compressors start cold, so the
  first notes after a Play (JS) or after the core arrives (Rust) can dip for
  a few tenths of a second, as Chromium's always have. `.wasm` 157,544
  bytes; core memory 14.50 MB.
- #161, hardening the core path (item 31b, v82),
  https://b26772a9-driftloom.nowmichaelclark.workers.dev/?engine=rust:
  with `?engine=rust`, the first notes after a Play, a quality change or the
  core's arrival no longer dip (listen to the first half second of a loud
  loop; it used to sag by up to 9 dB, 14 at the ceiling, for about a
  quarter second). Otherwise nothing should sound different. A core that
  fails while playing (a trap in `process`) now hands everything to the
  JavaScript synth and Diagnostics reads `engine: rust (failed: ...)`; a
  core that never loads reads `failed`, not `loading`. The wobble's LFOs
  catch up to a late core in one step, the handover sets the tone, wobble,
  reverb, echo and mutes instantly, and the worklet's frame count finds the
  clock again if it ever runs ahead. Proofs: `tools/core-paths.mjs` (the
  real app, the core made to fail five ways; 13 of its checks fail on v81),
  `test/worklet.test.mjs` (in CI), 79 core tests. With the harness's `cold`
  start the core's renders are bit for bit v81's (6 loops, 36 stage
  variants); warm and cold differ only until 0.25-0.28 s. Against
  JavaScript, whose compressors still start cold, the first second differs
  by -19.5 to -22.7 dB (its dip) and after it meets the floor (-85 to -87
  dB against -86 to -89). Balance lock and refusals unchanged; flag off
  moves only by the harness's own run-to-run noise. Performance, A/B
  against v81 on the core, 15 pairs: audio render +3% on full (about +1.5
  ms/s), +1% on lite; fill-in runs 1 of 15 against 0, the VM's usual (the
  main thread, which this cannot touch, was +3% too). `.wasm` 158,942
  bytes; core memory 14.50 MB.
- #164, the core by default (item 32, v83),
  https://claude-upbeat-volta-x491oa-driftloom.nowmichaelclark.workers.dev/:
  nothing should sound different; this makes the Rust core the engine for
  everyone, no flag. Open it, press Play on a loud loop, and check
  Diagnostics reads `engine: rust  late: 0  fallback: 0` and `core: loaded
  ... ms after the page began`; `?engine=js` reads `js (asked)` and plays the
  JavaScript synth (it is the fallback, and has the cold compressors' dip at
  the start that the core does not). The core loads as the page loads, so the
  first Play is on the core (0 notes in JavaScript in 15 first taps at 1x, 4x
  and 6x; v82 played 4-8). The gate for core changes is now
  `node tools/baseline.mjs` against `test/core-baseline.json`, in CI. Fixed on
  the way: a block handed to the core twice made the pads, leads and basses
  spike 20-40 times their level (never seen in play); time now only goes
  forward. `.wasm` 158,959 bytes (+17); core memory unchanged.
- #168, more words on `/rank` (v85),
  https://f8df3b08-driftloom.nowmichaelclark.workers.dev/rank.html: pool 3, 39
  more words (71 in all), one group per map. Listen for nothing; rank as before. Check that new words
  turn up often at first and that Export writes `"pool": 3`.
- #173, vibes in the generator (item 38a, v86),
  https://29ea8635-driftloom.nowmichaelclark.workers.dev: 71 recipes in
  `js/vibes.js`, `newSpec(seed, vibes)`, share format 4 for vibe loops
  (plain loops stay format 3), `stats.mjs --vibes` / `--vibe-each`. Listen
  for nothing yet (no button until 38b); every existing loop and code is
  unchanged.
- #174, the Vibes key and view (item 38b, v87),
  https://11c2216a-driftloom.nowmichaelclark.workers.dev: "vibes" at the top
  left of Now Playing opens every vibe in two columns; picked ones steer
  every new loop (Next, a loop ending, the first loop on opening), and a
  vibe loop's details show its vibes and weights. Listen for: pick a few,
  press Next a few times, and say which loops fit the vibes and which don't.
- #175, the ten dials (item 34), tools only:
  https://6cf10684-driftloom.nowmichaelclark.workers.dev (the app as v87).
  `node tools/dials.mjs <export|codes>`, scaled to the catalog
  (`test/dial-scale.json`), with `--yardsticks` and `--ears`. Nothing to
  hear; the energy dial separates Mikey's energy words more clearly than
  tempo or notes per second alone.
- #176, fitting the words and word albums (item 35), tools only:
  https://f1ab99d4-driftloom.nowmichaelclark.workers.dev (the app as v87).
  `node tools/ranks.mjs <export> --fit` and `node tools/wordalbum.mjs`. On
  the 181 trials the fit does not beat base rates yet (32% against 39%);
  word albums are the way to firm the regions up. Nothing to hear.
- #177, cinder and shatter may borrow bright modes when glad (item 36, v88;
  reflective skipped by Mikey's decision),
  https://75121fdf-driftloom.nowmichaelclark.workers.dev: a glad loop led by
  cinder (ionian, mixolydian) or shatter (lydian, mixolydian) sometimes
  borrows a bright mode; every existing code plays as before. Listen for: does
  a bright cinder loop still sound like cinder (the strum, the hand kit, the
  6/8) with a glad face, and is bright shatter still restless? Six loops for
  the album are in the PR.
- #180, the music with the screen off (item 39, v89),
  https://83d25e19-driftloom.nowmichaelclark.workers.dev: the core holds 8 s
  of notes while the page is hidden; Pause, Next and Previous from the lock
  screen or headphones act at once. Listen for: lock the phone on a busy
  loop for a few minutes; nothing skips or glitches, and Pause from the lock
  screen stops it straight away.
- #182, glitches with the screen on (item 40, v90),
  https://220a1cd7-driftloom.nowmichaelclark.workers.dev: visible, the core
  holds 2 s of notes, and mute, re-roll, tempo and wander cancel and decide
  again (heard 60-130 ms after the key); opening the Library no longer
  draws every album's art at once (1-2 s on a slow phone, which skipped
  the music). Listen for: play a busy loop, sit on Library and on Save /
  Share, scroll, open albums, paste a code: nothing skips; and a mute, a
  re-roll or a tempo change still lands at once.
- #187, the Pixel 10a on v91 (item 41, v92),
  https://8e345412-driftloom.nowmichaelclark.workers.dev: notes are never
  handed to the core later than it can play them, on a phone whose
  output runs far behind the clock; a note that still arrives more than
  30 ms late is left out instead of piling onto the next; every cover is
  drawn off the page. Listen for: on a phone with a long output delay, a
  mute, re-roll or tempo change lands about half a second after the key
  and nothing skips or bunches; on an ordinary one, as before. Nothing in
  the sound itself changed.
- #204, golden renders on every target (core phase 1, item 42), no version
  (the app is untouched): 21 note lists in `core/golden/`, a native test host
  (`cargo run --release --example dlrender`), and `tools/golden.sh`, run by a
  `golden` CI job on wasm32, x86-64, aarch64, armv7 and big-endian PowerPC.
  Each target renders the same twice; the four glibc targets agree on all 21
  lists, and wasm32 matches them on 5 (its `libm` differs by 1-3 ulp in every
  transcendental; the output differs by -84 to -145 dB). Chromium also
  flushes subnormals in the worklet, which Node does not (for item 43).
  Nothing to listen for.
- #208, the core's own math, no standard library, no subnormals (core phase
  1, item 43, v93), https://62b75066-driftloom.nowmichaelclark.workers.dev:
  the core carries its own `sin`, `exp`, `pow` and the rest (fdlibm, the code
  the wasm build already ran), builds with `no_std`, and stops filter and
  channel tails at a -301 dB floor so no subnormal is ever made. All 21
  goldens now hash the same on wasm32, x86-64, aarch64, armv7 and PowerPC (CI
  `--strict`, and with flush-to-zero on x86-64). The app's sound moved only in
  tails under -225 dB; the core baseline was updated. Listen for: nothing; a
  control album should sound the same as before.
- #210, glitches when minimized or locked (item 47, v94),
  https://410d5478-driftloom.nowmichaelclark.workers.dev: hidden or frozen, the
  page plans for a deep output buffer at once (1.5 s, or the worst this
  session or this device has seen, up to 4 s), so Next, Previous and Play
  from the lock screen and the catch-up after a freeze no longer land inside
  it and get left out; hidden, Next keeps the old loop playing until the new
  one starts. Diagnostics logs each late note with what decided it, and each
  jump in the render-ahead. Listen for: lock the phone and press Next,
  Previous, Pause and Play from the lock screen straight away: each lands
  about two seconds later, with no skip or hole; on screen, a mute or re-roll
  still lands at once. Nothing in the sound itself changed.
- #211, the randomness moves into the core (core phase 1, item 44, v95),
  https://5f3b317d-driftloom.nowmichaelclark.workers.dev: a draw is a pure
  function of the loop's seed, the pass, the note's place in its layer and the
  kind of draw, so the same code plays the same audio every time (the baseline
  renders its 40 loops again with another `Math.random` and all 40 are
  bit-identical); the core makes its own noise; "Let the loop wander" is drawn
  from the loop's seed and the pass. Different draws from the same
  distributions: 37 of 40 loops changed in detail (at most 0.16 LU louder or
  quieter). Listen for: nothing; a control album should sound the same as
  before.
- #213, any sample rate, any block size, any channel layout (core phase 1,
  item 45, v98), https://b663e0c6-driftloom.nowmichaelclark.workers.dev: the
  core takes 22,050 Hz up to its build's highest rate (192 kHz native, 96 kHz
  in the web build) and renders any number of frames a call with the same
  bits; mono is the only layout and anything else is refused in plain words;
  the noise's colour is the same at every rate (exactly as before at 48 kHz);
  a browser context faster than 96 kHz is asked to run at 48 kHz instead. At
  44.1 kHz 36 of 40 baseline loops change in detail, by at most 0.03 LU.
  Listen for: nothing; the control album should sound the same, a 44.1 kHz
  phone's hats, breaths and textures a hair different at most.
- #238, the building-block interface (core phase 1, item 46, v100),
  https://e76eec03-driftloom.nowmichaelclark.workers.dev: every voice, the
  bodies and the oscillator, filter, noise source and compressor sit behind one
  interface (`core/src/block.rs`) and one registry list
  (`core/src/blocks/mod.rs`), and each block gets a test kit (level, tone,
  cost, determinism, a null test against `core/golden/blocks.txt`). A new
  block is one file plus one line; `core/README.md` shows how. It is a
  refactor: the goldens and the baseline are bit for bit unchanged, and the
  instruction counts are within 0.4% of before. Listen for: nothing; it
  should sound exactly the same.
- #241, modulation and stereo seams (core item 46b, v101),
  https://37664061-driftloom.nowmichaelclark.workers.dev: a signal can now drive
  a block's setting (`Block::MODS`, `render_mod`; FM, every vibrato and every
  filter sweep run on it), every block has a stereo shape the mono render never
  pays for, a host can send a damp, release or bend to a channel's held notes
  (nylon's damp is one), and the bodies are a channel's own blocks in the
  registry. The kit probes every input and guards stereo; two test-only
  demonstration blocks are one file plus one line each. A refactor: goldens,
  baseline and stored block renders bit for bit unchanged, instruction counts
  -3.6% to +0.5%. Listen for: nothing; it should sound exactly the same.
- #244, the channel seams (core item 46c, v102),
  https://60366274-driftloom.nowmichaelclark.workers.dev: a channel's own
  blocks take per-loop settings (`Core::set`), hear every event sent to their
  channel when its time comes, and take signals routed from other channels
  (`Core::route`, one block late); each event kind says how far it reaches
  (damp and release once, bend and the new retune every time), and the retune
  rule keeps the notes two chords share. Three test-only demonstrations (a
  swell, a chorale, a duck) are one file plus one line each. A refactor:
  goldens, baseline and stored block renders bit for bit unchanged,
  instruction counts +0.02% to +0.10%. Listen for: nothing; it should sound
  exactly the same.
- #245, the per-loop choices (item 54, v103),
  https://f9a244e1-driftloom.nowmichaelclark.workers.dev: `js/choices.js` draws,
  once a loop and on a stream of its own, how many of the sound batch's changes
  it gets (none 15%, one 45%, two 30%, three 10%) and which; the list is empty
  until the batch's items add to it, so nothing changes yet. Listen for:
  nothing.
- #246, the frame drum's solo peak (item 55, v104),
  https://05aa78b0-driftloom.nowmichaelclark.workers.dev: the frame drum's
  pitch rests at its own first frequency instead of 440 Hz, so it loses a click
  it only had by chance (the same hit alone peaked anywhere from -1.1 to +1.2
  dBFS by where it fell in its block, now -1.0 to -1.1), and its skin rings
  0.40 s instead of 0.32 to keep its loudness; the kit fails any voice over -1
  dBFS alone. Listen for the frame drum losing that click and ringing a little
  longer. For the album: cinder #12 at 0:03.4 and tide #52 at 0:00.
- #248, roundBass (item 56, v105),
  https://bbdb00f6-driftloom.nowmichaelclark.workers.dev: the bass sits an
  octave higher at the bottom (36-57, never the extra octave down), and every
  bass voice but rhodesbass adds soft 2nd, 3rd and 4th harmonics under a
  900 Hz lowpass, so it reads on a phone; the layer keeps today's level by ear
  (Mikey's choice; `LIFT_DB` in `core/src/blocks/bass.rs` lifts it). A note's
  level no longer depends on where it lands in its block. Listen for a bass
  you can follow on a phone speaker without it getting louder or brighter
  than a bass should, and for the album, the same loops at today's level,
  +3 dB and +5.5 dB. For the album: dust #7 at 0:00 and dust #42 at 0:00 (the
  biggest move: 40-60 Hz 58% to 0.1% of the mix).
- #250, longChords (item 57, v106),
  https://8a9bf291-driftloom.nowmichaelclark.workers.dev: each loop draws how
  long its chords last (today's 56%, twice today 16%, about 8 s 10%, about 16 s
  9%, the whole loop 9%; calm loops lean long, driving ones away), held chords
  trimmed up to 2 dB, repeated pads tied into one note, and every held chord in
  the octave today's progression gives it. Listen for long chords that float
  rather than drone or get loud, and for no octave jumps. For the album: grove
  #47 at 0:00 (pads held twice as long) and dust #51 at 0:00 (one chord the
  whole loop).
- #251, floorDrop (item 58, v107),
  https://a355cd22-driftloom.nowmichaelclark.workers.dev: with "Let the loop
  wander" on, the loop no longer stops now and then; instead the kick and the
  bass step out for a bar or two while everything else plays on, about 3.5
  times as often as the old stops, and nothing fades the tails. Listen for a
  floor that drops and comes back without the loop ever going silent. For the
  album (wander on): wayfare #23 at 0:33.8 (two bars) and undertow #45 at
  0:03.8 (one bar).
- #252, glass (item 59, v108),
  https://2af52b47-driftloom.nowmichaelclark.workers.dev: in about two loops of
  three (the only choice in item 54's list so far), on a chord's first beat
  now and then, two or three of its notes ring out high (E5-G6) as a soft FM
  bell, a dotted eighth apart, at the chord's own velocity and quieter in a
  loop under one chord a bar; never from a one-note chord, and it never costs
  another part a note. Listen for bells that sparkle over the chords without
  standing in front of them, and for shatter's quiet prepared-piano loops
  (tei-har), where they play at full level. For the album: thaw #82 from 0:00
  and vapor #30 from 0:08.9.
- #253, dubStab (item 60, v109),
  https://9d274c1b-driftloom.nowmichaelclark.workers.dev: in busy loops
  (drums and seven or more notes a second; about one loop in four), on the
  offbeat after beat one or three, by a density drawn per loop, a soft stab of
  the chord sounding now (G3-C5) on a core voice of its own: a triangle under
  two saws 6 cents apart, a 22 ms attack, a lowpass closing toward 700 Hz, a
  short tail, and a darker, lighter throw into the echo. Only over a groove,
  never in a silent bar, and it never costs another part a note. Listen for a
  warm dub chord that answers the beat and dissolves into the echo without
  taking over the loop. For the album: wayfare #23 from 0:10.6 (busy) and
  cinder #12 from 0:07.2 (sparse).
- #254, the chapel organ (item 61, v110),
  https://7a353b2b-driftloom.nowmichaelclark.workers.dev: in loops that draw it
  (about three in five), the chords are held on a soft pipe organ for the
  whole loop, outside the voice budget: the round 7 mix (55% chapel, 25%
  round 5's pipe, 10% distant cathedral, 10% celeste), flutes leading, a
  breath of chiff on each new note, a celeste a few cents sharp, a dip at 800
  Hz, a slow level shimmer and some of it in the room. A note two chords share
  sounds on; it moves at most once a bar, holds through passing chords, and
  lets go at a long rest, a silent bar, Pause or a muted chords layer. Listen
  for an organ that surrounds the tune without covering it, and for chords
  that change without a gap. For the album: haven #38 from 0:00 and glade #21
  from 0:00.6.
- #255, the wash (item 62, v111),
  https://a12a0cdb-driftloom.nowmichaelclark.workers.dev: in loops that draw it
  (about two in five), a slow swell of air every 17-26 s on a channel of its
  own: two soft noises through lowpasses that open from 450 Hz to about 2.5 kHz
  and close again, tinted by two resonances on the loop's key (its root or
  fifth), like wind across a bottle. Its level follows the other five layers,
  so it is heard under a beat, stays behind a sparse loop and fades with the
  loop into a rest. Listen for air that breathes behind the music and never
  hisses or sits in front. For the album: shrine #0 at 0:20.3 and grove #47 at
  0:23.4 (each at its first swell's peak).
- #256, the arc (item 63, v112),
  https://1ae42640-driftloom.nowmichaelclark.workers.dev: in loops that draw it
  (about one in three), a second lowpass after the master's tone breathes from
  300 Hz up to 9 kHz and back over a period drawn 2-5 minutes, lingering on the
  dark side, and the echo's feedback rises a little as it opens; a loop starts
  at a bright point of its swing (the prototype started closed: for Mikey's
  ears). The group's balance lock re-baselined. Listen for the whole mix
  breathing dark and bright over minutes, and for the instruments always
  coming back. For the album: haven #38, bright from 0:00, darkest at 1:05.5,
  and undertow #6, darkest at 1:31.9.
- #260, the arc's two shapes (item 63b, v113),
  https://7702bdc6-driftloom.nowmichaelclark.workers.dev: a loop that draws the
  arc now draws one of two shapes: deep (about one in three), item 63's swing
  exactly, 300 Hz to 9 kHz and back, lingering dark; or dip (about two in
  three), open at 9 kHz for 65% of each swing with one smooth dip over the
  other 35% to a floor of 800 Hz to 1.5 kHz, starting anywhere in the open
  part. Calm, vast vibes lean deep and 3-5 minute swings; bright, lively ones
  lean dip and 1.5-2.5 minutes. Every variable is a named setting (`ARC` in
  `js/choices.js`). Listen for the dip as a gentle dimming that comes and goes,
  never long enough to lose the instruments, and for the deep swing as before.
  For the album: vapor #4 (deep), bright from 0:00, darkest at 1:16.4; wayfare
  #25 (dip), open until 0:06, darkest at 0:44.2 (1.24 kHz), open from 1:22.
  Mikey's loop (seed 102356715) draws the dip, darkest at 2:05.5.
- #261, CI on Ubuntu 24.04 (no version),
  https://776de890-driftloom.nowmichaelclark.workers.dev: the core job's runner
  pinned, since Playwright 1.56.1 can't install Chromium on ubuntu-latest's
  26.04 (the baseline check failed on #260 for it). Nothing to listen for.
- #262, the stereo render (item 64, v114),
  https://107b774b-driftloom.nowmichaelclark.workers.dev: the core renders
  stereo as a render of its own (`?stereo` plays it): the master on two lanes,
  compressors keyed by the mid, the width ceiling after the limiter. Nothing
  is placed yet, so each side is the mono render bit for bit; mono itself is
  unchanged. Listen for nothing: `?stereo` should sound exactly like the plain
  page.
- #263, the space recipes (item 65, v115),
  https://51edc910-driftloom.nowmichaelclark.workers.dev: the stereo render
  gets its space (`?stereo` on v115; the default from v116): a room around
  the mix, the chords, texture and melody spread wide by a swaying copy, the
  light kit and glass placed a little to either side, the wash's noises wide
  above 200 Hz; the bass, kick and everything low stay in the middle, and
  the mix folded to mono is the mono render. Each loop draws its widths from
  its seed, narrower when busy, wider when wet. Listen for space without
  anything sitting on one side, and for nothing lost on a phone speaker. For
  the album: glade #57, from 0:00 (the room and the spreads), and vapor #30,
  from 0:00 (the wash's width).
- #264, the Mono switch, and stereo by default (item 66, v116),
  https://41916f9a-driftloom.nowmichaelclark.workers.dev: the app plays the
  stereo render by default; **Mono** (Advanced, under "Sound") plays the mono
  render, exactly today's balance, and switching while a loop plays fades
  over a quarter second without a restart. Kept with the settings, never in a
  share code. Diagnostics shows the layout and what the render costs this
  device in each. Listen for: a switch that never clicks or restarts the loop,
  and (Session 26) the blind stereo-against-mono A/B; on Mikey's phone, the
  `render:` line in Diagnostics after a minute each way.
- #266, tunedKick (item 67, v117),
  https://16235f1f-driftloom.nowmichaelclark.workers.dev: in loops with a kick
  that draw it (about one in seven), the kick and the soft kick keep today's
  sweep but land on the key's note between G1 and D2 (a key of D# to F# on its
  fifth): short, falling as fast as today's, or a long boom, today's punch
  then a tail ringing on 7-17 dB down, both as loud by ear as today's kick.
  One kick at a time: the next hit chokes a boom still ringing. Booming vibes
  lean long. With "Let the loop wander" on, a pass now and then swaps the kick
  (today's, short, long). Listen for a kick that sits with the bass instead of
  under it, a boom that thumps without getting louder, and for swaps that
  don't click. For the album: undertow #6 from 0:00 (a long boom on C2) and
  glade #63 from 0:00 (short, on C#2).
- #267, subDuck (item 68, v118),
  https://c54ff1f8-driftloom.nowmichaelclark.workers.dev: in loops with a kick
  and a bass that draw it (about one in eight), the bass alone dips 5-9 dB at
  each kick, less for a soft one, and comes back over 0.15-0.25 s, so the kick
  and the sub take turns while the pads and the tune stay where they were.
  Listen on speakers or headphones with some low end for a bass that breathes
  with the kick without pumping, and for nothing else moving. For the album:
  halcyon #18 from 0:00 (7.6 dB) and halcyon #68 from 0:00 (6.6 dB).
- #268, lateFloor (item 69, v119),
  https://8e44cc27-driftloom.nowmichaelclark.workers.dev: in loops that draw it
  (about one in five), after a load the kit and the bass sit out 8-30 s while
  the pads, tune or texture play alone, then arrive together, the kit first
  with the bass two bars later, or the bass first. Never on a reroll, a
  resumed loop, or a loop with nothing else playing in its first second. A few
  notes the budget used to turn away now play in the intro. Listen for an
  opening that sets the scene and a beat that arrives rather than starts, and
  for no silence at the start. For the album: undertow #45 with lateFloor
  forced on (the report's loop, Mikey's ask: the kit at 0:29.8, the bass at
  0:33.5; its texture is 0.8 dB lighter from the budget's ripple), and
  clockwork #33 from 0:00 (the kit at 0:07.8, the bass at 0:15.5).
- #269, floorDrop shapes (item 70, v120),
  https://29f8bbc6-driftloom.nowmichaelclark.workers.dev: with "Let the loop
  wander" on, in loops that draw them (about one in six), each floor drop takes
  a shape: single as before, stagger (the kick back on time, the bass a bar or
  two later), stutter (two to four short drops a couple of bars apart) or long
  (a dozen seconds, or the whole pass of a short loop); the drops come as often
  and where they did. Listen for a floor that leaves and comes back in more
  than one way, never a full stop. For the album, both ends: vapor #4, the
  whole pass dropped for 34.3 s from 1:08.6 (a slow 8-bar loop), and halcyon
  #18, four 2.7 s stutter drops from 1:30.7 (131 bpm).
- #270, pedalBass (item 71, v121),
  https://3ba6d476-driftloom.nowmichaelclark.workers.dev: in loops that draw it
  (about one in eight), a held, pulse, dub or walking bass holds the key's
  tonic under whatever the chords do, held, in half bars, on the beats or off
  them, now and then stepping to one neighbor (a whole tone up, the tone
  below, the flat second or the fifth) between downbeats. Listen for a floor
  that stays put while the harmony moves over it. For the album: cinder #5
  from 0:00 (a beat pedal a whole tone up, drawn on its own); with pedalBass
  forced on, dust #69 from 1:02 (its busiest moment, 60 → 66 refused: do the
  missing chord and bell notes matter?), undertow #8 from 0:00 (the heavier
  low end) and clockwork #65 from 0:00 (the lighter one).
