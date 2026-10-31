# The queue: finished items

The full text of every item that is done, closed or stopped by design, as it
stood in `docs/QUEUE.md` when this file was made (2026-10-26): the briefs,
their yardsticks, and the brain's checks. Nothing here is live; the live
queue is `docs/QUEUE.md`. The notes the hands left on these items are in
`docs/archive/notes.md`; what shipped is in `docs/DONE.md` and
`docs/archive/done.md`.

## Index

- [0. Finish #59 (fiddle and accordion, take two)](#0-finish-59-fiddle-and-accordion-take-two)
- [1. tide, part 2](#1-tide-part-2)
- [2. Soften the harsh leads](#2-soften-the-harsh-leads)
- [3. Voices, pair 2: nylon and pan flute](#3-voices-pair-2-nylon-and-pan-flute)
- [4. cinder, the fiery 6/8](#4-cinder-the-fiery-68)
- [5. wayfare, the rolling train](#5-wayfare-the-rolling-train)
- [6. Pan flute: static when notes overlap (a bug)](#6-pan-flute-static-when-notes-overlap-a-bug)
- [7. Nylon: a cleaner strum](#7-nylon-a-cleaner-strum)
- [8. wayfare's drums and chug: playful, not tough](#8-wayfares-drums-and-chug-playful-not-tough)
- [9. wayfare's drums: bring the life back](#9-wayfares-drums-bring-the-life-back)
- [10. A performance harness and baseline (measure only)](#10-a-performance-harness-and-baseline-measure-only)
- [11. Sound polish: measured fixes](#11-sound-polish-measured-fixes)
- [12. Composition depth, first pass](#12-composition-depth-first-pass)
- [13. Rust engine, step 2 -- stopped at its gate (brain, 2026-09-26)](#13-rust-engine-step-2----stopped-at-its-gate-brain-2026-09-26)
  - [(original brief, not to be built now)](#original-brief-not-to-be-built-now)
- [14. The performance wins the baseline found](#14-the-performance-wins-the-baseline-found)
- [15. The temple bell as a chord voice: lift it](#15-the-temple-bell-as-a-chord-voice-lift-it)
- [16. The mood labels: make "happy" sound happy](#16-the-mood-labels-make-happy-sound-happy)
- [17. Composition depth, second pass](#17-composition-depth-second-pass)
- [18. Housekeeping: comments and README against the code](#18-housekeeping-comments-and-readme-against-the-code)
- [19. A word-ranking test (diagnostics, roadmap 19)](#19-a-word-ranking-test-diagnostics-roadmap-19)
- [20. Ranking test: one word per map (pool v2)](#20-ranking-test-one-word-per-map-pool-v2)
- [21. Rust core, step 1: the pipeline and one voice (kalimba)](#21-rust-core-step-1-the-pipeline-and-one-voice-kalimba)
- [22. Rust core, step 2: fiddle and pad, and the performance A/B](#22-rust-core-step-2-fiddle-and-pad-and-the-performance-ab)
- [Rust core, steps 3-8: every voice (items 23-28)](#rust-core-steps-3-8-every-voice-items-23-28)
- [23. Rust core, step 3: the FM voices, sine and tubular (no new DSP)](#23-rust-core-step-3-the-fm-voices-sine-and-tubular-no-new-dsp)
- [24. Rust core, step 4: the wave-table voices](#24-rust-core-step-4-the-wave-table-voices)
- [25. Rust core, step 5: accordion and nylon, the shared bodies](#25-rust-core-step-5-accordion-and-nylon-the-shared-bodies)
- [26. Rust core, step 6: bandpass, highpass and the noise source](#26-rust-core-step-6-bandpass-highpass-and-the-noise-source)
- [26b. The one-frame click in the noise voices (before item 27)](#26b-the-one-frame-click-in-the-noise-voices-before-item-27)
- [27. Rust core, step 7: bass, textures and drums (five channels)](#27-rust-core-step-7-bass-textures-and-drums-five-channels)
- [27c. Three small fixes from the brain's check of 26b and 27 (before item 28)](#27c-three-small-fixes-from-the-brains-check-of-26b-and-27-before-item-28)
- [28. Rust core, step 8: the sung voices (vowel, hum, choir)](#28-rust-core-step-8-the-sung-voices-vowel-hum-choir)
- [Rust core, steps 9-11: the effects and the master chain (items 29-31)](#rust-core-steps-9-11-the-effects-and-the-master-chain-items-29-31)
- [29. Rust core, step 9: channels, sends, duck, echo and reverb](#29-rust-core-step-9-channels-sends-duck-echo-and-reverb)
- [30. Rust core, step 10: wobble, saturator, tone and highpass](#30-rust-core-step-10-wobble-saturator-tone-and-highpass)
- [31. Rust core, step 11: the compressors and the whole mix in the core](#31-rust-core-step-11-the-compressors-and-the-whole-mix-in-the-core)
- [31b. Hardening the core path (from the brain's check of items 30-31)](#31b-hardening-the-core-path-from-the-brains-check-of-items-30-31)
- [32. The core by default (the flag's last step)](#32-the-core-by-default-the-flags-last-step)
- [33. More words on the ranking page (/rank)](#33-more-words-on-the-ranking-page-rank)
- [47. Urgent: glitches when minimized or locked (Pixel 9, GrapheneOS, v92)](#47-urgent-glitches-when-minimized-or-locked-pixel-9-grapheneos-v92)
- [The core, phase 1: foundations (items 42-46)](#the-core-phase-1-foundations-items-42-46)
- [42. Golden renders on every target](#42-golden-renders-on-every-target)
- [43. The core's own math, and no standard library](#43-the-cores-own-math-and-no-standard-library)
- [44. The randomness moves into the core](#44-the-randomness-moves-into-the-core)
- [45. Any sample rate, any block size, any channel layout](#45-any-sample-rate-any-block-size-any-channel-layout)
- [46. The building-block interface](#46-the-building-block-interface)
- [46b. The interface, finished for the batch: modulation, stereo, channel events](#46b-the-interface-finished-for-the-batch-modulation-stereo-channel-events)
- [46c. The channel seams the batch still needs (before item 61)](#46c-the-channel-seams-the-batch-still-needs-before-item-61)
- [Before phase 2: the sound batch (items 54-73, then 74)](#before-phase-2-the-sound-batch-items-54-73-then-74)
- [54. The per-loop choices (no sound change)](#54-the-per-loop-choices-no-sound-change)
- [55. The frame drum's solo peak (and a voice ceiling in the kit)](#55-the-frame-drums-solo-peak-and-a-voice-ceiling-in-the-kit)
- [56. roundBass](#56-roundbass)
- [57. longChords](#57-longchords)
- [58. floorDrop](#58-floordrop)
- [59. glass](#59-glass)
- [60. dubStab](#60-dubstab)
- [61. The chapel organ](#61-the-chapel-organ)
- [62. The wash](#62-the-wash)
- [63. The arc](#63-the-arc)
- [64. The stereo render](#64-the-stereo-render)
- [65. The approved stereo baseline, as space recipes](#65-the-approved-stereo-baseline-as-space-recipes)
- [66. The Mono switch, and stereo by default](#66-the-mono-switch-and-stereo-by-default)
- [67. tunedKick](#67-tunedkick)
- [68. subDuck](#68-subduck)
- [69. lateFloor](#69-latefloor)
- [70. floorDrop shapes](#70-floordrop-shapes)
- [71. pedalBass](#71-pedalbass)
- [72. padSweep](#72-padsweep)
- [73. echoTime](#73-echotime)
- [41. Urgent: the Pixel 10a still glitches on v91 -- what the field report shows](#41-urgent-the-pixel-10a-still-glitches-on-v91----what-the-field-report-shows)
- [40. Urgent: glitches with the screen on, worst on Library and Save/Share](#40-urgent-glitches-with-the-screen-on-worst-on-library-and-saveshare)
- [39. Emergency: music skips and glitches when the phone is locked](#39-emergency-music-skips-and-glitches-when-the-phone-is-locked)
- [38. Vibes: pick the words, hear them in every new loop (top priority)](#38-vibes-pick-the-words-hear-them-in-every-new-loop-top-priority)
  - [38a. The vibes in the generator (data, draw, share code)](#38a-the-vibes-in-the-generator-data-draw-share-code)
  - [38b. The Vibes button and view](#38b-the-vibes-button-and-view)
- [34. The ten dials, measured: tools/dials.mjs (measure only)](#34-the-ten-dials-measured-toolsdialsmjs-measure-only)
- [35. Fitting the words, and building word albums (measure only)](#35-fitting-the-words-and-building-word-albums-measure-only)
- [36. Two mode decisions: reflective may be neutral; cinder and shatter may be bright](#36-two-mode-decisions-reflective-may-be-neutral-cinder-and-shatter-may-be-bright)
- [76. The first loop after the app starts loses its first notes (app track)](#76-the-first-loop-after-the-app-starts-loses-its-first-notes-app-track)
- [76b. The vibes to 88](#76b-the-vibes-to-88-glacial-twenty-more-four-gone)
- [76c. The bass level](#76c-the-bass-level-from-session-26s-album)
- [76d. lateFloor: the second arrival drawn](#76d-latefloor-the-second-arrival-drawn-not-fixed)
- [74. Clippy pedantic](#74-clippy-pedantic-in-the-files-46-left-before-item-79)
- [Before 77 (phase 2)](#before-77-phase-2)
- [77. Generation made exact, and its goldens](#77-generation-made-exact-and-its-goldens-javascript-no-sound-change)
- [78. The generation data as plain text](#78-the-generation-data-as-plain-text-read-by-both)
- [79. The Rust generator, 1](#79-the-rust-generator-1-the-spec-and-the-harmony)
- [80. The Rust generator, 2](#80-the-rust-generator-2-the-layers)
- [81. The Rust generator, 3](#81-the-rust-generator-3-the-choices-the-load-and-the-wander)
- [82. The player](#82-the-player-a-loop-from-a-seed-in-the-core)

## Old status notes (until 2026-10-25)

The status paragraphs the queue carried at the top until 2026-10-25, as written.

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

*The standing rules for core items that were here moved to `core/AGENTS.md` (2026-10-26).*

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

*The rules for the batch moved to `core/AGENTS.md` (2026-10-26); the one about the album loops stays here, done.*

- **The album loops, re-checked** (brain, 2026-10-20): at the end of group
  (c), re-check every Done line's album loops against `main` (adding choices
  reshuffles which loops draw what) and update the lines whose loop no
  longer plays its sound, before Session 26. **Done** (hands, 2026-10-26,
  against `main` after item 73): every album loop from #241 on checked
  through the real engine (wander on); eleven lines updated, each saying
  which loop no longer plays its sound, and cinder #12's frame drum moved
  from 0:03.4 to 0:03.5. The loops kept still play it at the times given
  (dust #69 60 → 66 refused and undertow #45's texture -0.78 dB
  with their choices forced, as before).

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
  shapes, pedalBass, subDuck, padSweep). Not counted: roundBass,
  longChords, the single floorDrop, the stereo baseline (see the rules
  above) and echoTime, drawn for every loop on the sketch's stream (the
  brain, 2026-10-26; item 73).
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

## 76. The first loop after the app starts loses its first notes (app track)

**App track.** Hands: **Sonnet 5.5, high**. Stay out of `core/`: the fix
belongs in the page's scheduling (`js/engine.js`, `js/synth.js`,
`js/core.js`). If it truly needs a core change, stop and leave a note.

**What the field shows** (Mikey's Moto, Diagnostics on v116, 2026-10-25):
the first loop after the app starts loses its first notes. Six notes were
late at 3.8 s into the page's time, every one "inside the buffer, left out"
(item 41's rule: a note more than 30 ms late is left out, not bunched). They
had been posted 77-82 ms ahead of their start; the render-ahead measured
right after was 133 ms. Later loops were fine.

**What the brain reads in it (unproven; check before relying on it):** the
planner places a note no closer than the render-ahead plus a margin, but
at a cold start there is no render-ahead yet. `Synth.renderAhead` is null
until a note has reached the core, or at best the context's `baseLatency`
(a callback's worth, tens of milliseconds), and `Engine._soonest` then falls
back to its own lead (0.08 s for the loop's start), which matches the 77-82
ms. The core was already rendering 133 ms past the clock, so the first
notes landed inside what it had rendered. Item 41 plans from a
measurement and item 47 assumes a deep buffer when hidden; a cold start
visible has neither.

**What:** until the render-ahead has been measured (no note has reached the
core yet this session), plan every note with a safe margin instead of the
short lead: **the device's reported output latency** (`AudioContext.outputLatency`,
where the browser gives one, else `baseLatency`), **and never less than
250 ms**. Once it is measured, plan from the measurement as item 41 left it.
- It covers every note decided before the first measurement: the first
  loop, and whatever is decided near the clock right after the start (a
  new loop, Next, a tempo change in the first seconds).
- The cost is that sound starts up to a quarter of a second after Play on
  the first loop; nobody can see that. Take the more conservative option
  wherever the choice is open and write it in the PR, "for Mikey's ears".
- The JavaScript synth is unchanged (it has no render-ahead).
- Diagnostics' log of late notes says whether a note was planned from the
  margin or from a measurement, so the next report shows which.

**Yardsticks:**
- **No late notes at a cold start on a phone in Diagnostics** (the item's
  measure): app started from nothing, first loop played to the end, the
  copied Diagnostics report `late: 0` for it, on full and on lite.
- In the harness, first reproduce the failure on `main` (a core that
  renders further ahead than 80 ms from the first note, as
  `node tools/views.mjs --buffer ... --delay ...` models a buffer), then show
  zero late notes in the first loop with the margin, across several seeds.
- Visible behavior after the first measurement unchanged: the latency of a
  mute or re-roll as item 41 measured it, and item 47's hidden planning.
- Steps for the tester in the PR (what to do, what to copy), so his next
  Diagnostics answers whether anything is left.
- Bump `BUILD` and `CACHE` together to the next version; the usual tests
  and checks.

**Done** (#277, v124, 2026-10-27): the note is `docs/archive/notes/76-cold-start.md`.

## 76b. The vibes to 88: glacial, twenty more, four gone

**Done** (#280, v125, 2026-10-29). Glacial's arc drawn outside the choices (`ARC.glacialChance` 0.6; 72% of glacial-alone loops); the note is `docs/archive/notes/76b-vibes.md`.

**Generator, before phase 2.** Hands: **Opus 5.5, high** (twenty-one
recipes tuned by measurement). Mikey's decision (2026-10-29): 71 vibes
become 88. It goes in now, while the JavaScript generator is the only one:
from item 77 on, a change like this is made twice.

**Read first:** `docs/VIBES.md`, "The vibes to 88" (why each word, and the
brain's draft recipes, every name already checked against the test).

**What:**
- **Twenty-one new vibes** in `js/vibes.js`, from the drafts: glacial,
  dewy, dusky, moonlit, rainy, whimsical, dreamy, chill, groovy, pulsing,
  cruising, droning, cosmic, bubbly, earthy, soaring, jazzy, breezy,
  rippling, playful, mellow. Tune any draft that misses its yardstick below
  and say what moved.
- **Four gone:** soft, energetic, silly, comfy. **Old codes don't need to
  keep working** (Mikey, 2026-10-29: no bloat for compatibility): the four
  leave `js/vibes.js`, `VIBE_IDS` is rewritten in alphabetical order (88
  ids), and every table that names them changes (`ARC_LIVELY`, `SUBDUCK`,
  `LATEFLOOR`, `FLOORDROP` and the rest in `js/choices.js`; the tests;
  `docs/MOODS.md` and `/rank`'s pool if it names them). Codes made before
  this item may play different vibes or none; that's fine.
- **Two folds:** **lively** takes energetic's drive (its tempo up to
  energetic's 0.85, drums 2.5, hats 2, shatter 3.4 among its profiles), so
  the top of the energy range doesn't get softer; **restful** takes the
  bedtime role sleepy would have had (a lullaby lean: tempo 0.15, music box
  and hum in the tune, drums 0.15).
- **Pairs that must sound apart** (today's recipes nearly match): **foggy**
  darker and denser (more space, moogpad and swell,
  the deep arc, few rests), **misty** lighter and dewier (higher lift,
  celeste and flute, drops and wind, the dip arc); **experimental** the
  unusual sounds (prepared piano, vowel, saw, wide wobble, shatter and
  clockwork) and **strange** the odd scales (keep its modes; ordinary
  voices); **airy** still and open (low energy, long notes, rests), **breezy**
  moving (its tempo and swing); **mellow** drumless and gentle, **chill**
  with a relaxed beat (drums in most loops).
- **Glacial's arc** (`ARC` in `js/choices.js`): glacial leans toward the arc
  (x2.5) and toward deep (so that, with glacial alone, deep is about 3 in 4
  of the loops that draw the arc), with a period of its own, **5-8 minutes**
  (Mikey), and **8-12 minutes when glacial is paired with a glacial-adjacent
  vibe**: still, serene, meditative, spacious, floating, frosty, foggy,
  misty, hushed or droning (the brain's list, "for Mikey's ears"), a named
  setting like every `ARC` variable.
- **The new vibes in the choice tables,** as the calm and lively ones are:
  the arc's calm list (toward, deep) gains droning, dreamy, moonlit, dusky,
  cosmic and rainy; its lively list (dip) gains playful, bubbly, breezy,
  whimsical, soaring and groovy; the echo's one beat leans toward glacial,
  droning, dreamy and cosmic, half a beat toward bubbly and playful; the wash
  toward glacial, rainy, breezy and droning; padSweep toward glacial, dreamy
  and cosmic; subDuck toward groovy, pulsing and chill; lateFloor toward
  cruising and pulsing, away from glacial; tunedKick's long boom toward
  pulsing and groovy, away from glacial. Multipliers as the batch's table
  uses them (toward x1.5-2, away x0.4).
- `docs/VIBES.md`: what the tuning changed, per vibe. The app's Vibes list
  shows 88 (no design change).

**Yardsticks:**
- Loops without vibes render exactly as before (core baseline, balance lock,
  `test/generator.test.mjs`'s "no vibes is today's loop").
- **Each new vibe, alone, over 300 loops** (the brain's figures for the
  drafts are in `docs/VIBES.md`): its first-named profiles lead; its tempo on
  the side the recipe says (glacial, droning, moonlit, restful among the
  slowest tenth of the vibes; cruising, lively, enthusiastic among the
  fastest); drums in under 15% of glacial, droning, moonlit, mellow and
  restful loops and in over 70% of chill, groovy, pulsing and cruising.
  Report a table: vibe, median bpm, drums share, top three profiles, top
  three tunes.
- **The pairs apart:** foggy and misty, experimental and strange, airy and
  breezy, mellow and chill each differ in at least two of: their leading
  profile, median tempo by 8 bpm or more, drums share by 20 points or more,
  their top tune voice.
- **Glacial's arc:** alone, the arc in at least half its loops, deep about
  3 in 4 of those, every period 5-8 minutes; with one adjacent vibe, 8-12.
- Refusals on full and lite for each new vibe's loops no worse than the
  catalog.
- The test's count reads 88; tests three times; bump `BUILD` and `CACHE`.

**Not now** (the ears' later work): grooves, arpeggios, a rain texture and
extended chords that groovy, pulsing, cruising, cosmic, rainy and jazzy will
want once the ears have heard their records. Today's recipes lean on what
exists.

## 76c. The bass level, from Session 26's album

**Done** (#282, v126, 2026-10-30). The hand kit's frame drum counts as a kick (noan-nou is a hand-kit loop); `rhodesbass` is not lifted (the old constant never reached it).

**Core and host, before phase 2.** Hands: **Sonnet 5.5, high**. From Mikey's
picks (2026-10-29, `docs/ALBUMS.md`, "Session 26"): roundBass at +3 dB on
dust #7 and noan-nou (both with a kit), today's level on dust #42 (no kit;
the bass carries the low end alone, and +3 there lifted the whole loop 2.4
LU).

**What:** the bass layer **+3 dB in loops that play a kick, today's level
in loops without one**, as a per-loop setting the host sends at load (the
core's `LIFT_DB` constant becomes that setting, every bass voice alike),
not a compile-time number. A loop whose kick only arrives late (lateFloor)
or drops out (floorDrop) keeps its loop's setting throughout. Named, with
its two values, so a vibe or phase 2's request can move it later. The
fallback leaves its bass as it is.

**Yardsticks:** on dust #7 and noan-nou the bass layer +3.0 dB by ear
(K-weighted) against `main`, on dust #42 unchanged to the bit; over the
baseline's 40, loops without a kick unchanged, loops with one moved by the
bass alone (the other layers' taps the same); refusals unchanged; the core
baseline updated, saying what moved; bump `BUILD` and `CACHE`.

## 76d. lateFloor: the second arrival drawn, not fixed

**Done** (#283, v127, 2026-10-30). The hands' calls, for Mikey's ears: the second arrival is capped inside the first pass when the first arrival is, else within one loop's length; the gap weights and the x1.6 / x0.5 vibe leans are theirs.

**Generator, before phase 2.** Hands: **Sonnet 5.5, high**. From Mikey's
verdict on undertow #45 (2026-10-29): keep it, "with the caveat that the
bass can come in later too in relation to the kit. It doesn't need a set
come in time though. I like when things are weighted or determined by vibe
or somewhat random."

**What** (`LATEFLOOR` in `js/choices.js`):
- When the kit and the bass arrive apart (`kitFirst`, `bassFirst`), the
  gap between them is **drawn per loop** instead of today's fixed two bars:
  1 to 8 bars, rounded to a bar, weighted toward the short end (the
  conservative choice, "for Mikey's ears"), still capped so the second
  arrival comes inside the loop's first pass.
- **Vibe leans** on the gap and on the wait itself: calm, vast vibes
  (glacial, droning, meditative, serene, still, spacious, floating, dreamy,
  hypnotic) lean long; lively ones (lively, enthusiastic, bouncy, skipping,
  groovy, pulsing, cruising) lean short. Every variable named, with its
  range, as `LATEFLOOR`'s are.
- Drawn on lateFloor's own stream, after its present draws, so a loop that
  draws `together` waits exactly as before.

**Yardsticks:** loops that don't draw lateFloor, or draw `together`, render
as before; over the corpus's lateFloor loops, the gaps cover 1-8 bars with
the short end commonest (report the spread), never past the first pass;
the vibe leans move the medians the right way (report them); refusals equal
or fewer; tests three times; bump `BUILD` and `CACHE`.

## 74. Clippy pedantic in the files 46 left (before item 79)

**Done** (#284, v128, 2026-10-30).

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

## Before 77 (phase 2)

**The brain's check (Session 28, 2026-10-31), items 77-82 together:** passes.
Run on `main` at v132 (d5b15e6), not taken from the reports: the JavaScript
tests (the generator's three times) and `cargo test --locked --release` (153
tests); the generation goldens in Node, 3,060 of 3,060; the balance lock;
`tools/build-core.sh --check` for both modules. The player against the app's
engine in Chromium on cases the hands never ran: see the log
(`docs/archive/brain-log.md`). Item 82's note on when notes reach the core
(`docs/archive/notes/82-hand-over.md`) answered in the queue's "For phase 3".

Item 74 is done (#284): pedantic clippy is zero in `core/` and CI holds it
there, so the generator lands clean. Item 75 (the reverb, rate-true) stays
low priority, for any session with usage to spare after 82. **Vibes and
generator changes from here on follow the phase's first rule** (one source
of truth at a time, above): after item 78 the data is one file both
generators read.

## 77. Generation made exact, and its goldens (JavaScript, no sound change)

**Why:** the port is proven note for note (above), which needs the
JavaScript's decisions to be reproducible anywhere.

**What:**
- `js/dlmath.js`: the core's math (`core/src/math.rs`: pow, exp, log,
  log2, log10, sin, cos, sqrt where used) mirrored in JavaScript, checked
  bit for bit against the wasm's own over a million inputs per function
  (export them from the core for the test if they aren't). The generator,
  `js/choices.js`, `js/theory.js`, `js/characters.js`, `js/picks.js`,
  `js/vibes.js` and the engine's load-time decisions (`stabOf`, `organOf`,
  `washOf`, `spaceOf`, the echo time, the kick's level fit) use it; nothing
  on those paths calls `Math.pow` and the rest. Say which loops move (a
  last bit flipping a comparison); expected: a handful in 10,000. Core
  baseline and balance lock updated on purpose if anything moves.
- **The generation goldens:** `tools/gen-golden.mjs` writes, for each case,
  one line: the seed, the vibes, and FNV-1a 64 hashes of (a) the spec, (b)
  each layer's events after `render`, (c) the choices and every per-loop
  draw the engine makes at load (arc, kick, subDuck, lateFloor, padSweep,
  echo, space, wash, stab, organ), (d) each of the first eight wandered
  passes' events (drift), in a canonical text form documented in the file.
  Cases: 2,000 corpus seeds with no vibes, 1,000 with 1, 3 or 5 vibes, the
  21 golden lists' seeds, and the album loops of `docs/DONE.md`. Kept in
  `core/golden/generation.hashes`; CI checks the JavaScript against it.
- No change to what anyone hears beyond the handful above.

**Yardsticks:** the mirror equal to the core on every input tested; the
generation goldens stable across three runs and across Node and Chromium;
the list of loops that moved (with what moved) in the PR; tests three
times.

## 78. The generation data as plain text, read by both

**Why:** promise 5 (profiles, vibes, moods and voices as data, a plain-ASCII
text form people can read and edit) and the two-tracks rule: if the data is
one file both generators read, vibe tuning never has to wait for the port.

**What:**
- One text format (propose it in the PR before converting: readable, plain
  ASCII, comments allowed, no dependency to parse it in Rust or JavaScript),
  and today's tables converted into it: the characters (profiles), vibes,
  moods, the voice table (layers, costs, levels) and the choice tables of
  `js/choices.js` (`ARC`, `KICK`, `SUBDUCK`, `LATEFLOOR`, `FLOORDROP`,
  `PEDAL`, `PADSWEEP`, `ECHO`, the change counts).
- The JavaScript reads it (fetched at start in the app, read from disk in
  Node); the tables in code go. `js/vibes.js` stays the brain's to edit in
  its new form (`AGENTS.md`'s rule moves with it).
- A compact binary form for the core is item 79's business; this item
  fixes the text.

**Yardsticks:** generation goldens, core baseline and balance lock
unchanged; a test that every entry parses and every name it uses exists;
`docs/VIBES.md` says how to edit a vibe now.

## 79. The Rust generator, 1: the spec and the harmony

**What:** in `core/` (its own module tree, `gen/`), from the seed and the
data: `newSpec` (the vibes' draw, drive, the profile mix, the feel, bars,
tempo and gait, key and mode) and `render`'s harmony and form (progression,
sections, rests, gaps, silent bars). The data compiled from item 78's text
into the core (a build step that writes a Rust table, or a small binary the
core reads at `Core::new`: say which and why). A host call returns the spec
(`dl_spec`, plain ASCII, for tests and the app's Now Playing).

**Yardsticks:** generation goldens (a) and the harmony part of (b) equal on
every case and every target; no allocation after `Core::new`; memory in
bytes reported.

## 80. The Rust generator, 2: the layers

**What:** the tracks: drums, bass (with item 56's register and item 71's
pedal), chords (with longChords, and `separateRegisters` exactly as today:
the octave fix is item 84, separate on purpose), melody (motifs, cells,
development), texture, the choir. The voice choice per layer, the entry
schedules, the repairs and gaps.

**Yardsticks:** generation goldens (b) equal on every case and target;
`stats.mjs`' corpus figures computed from the Rust tracks equal the
JavaScript's (a tool that reads both).

## 81. The Rust generator, 3: the choices, the load and the wander

**What:** item 54's per-loop choices and every choice's own draws (the arc's
shape, the kick, subDuck, lateFloor, the floor drop's shapes, the pedal,
padSweep, echoTime), the per-loop decisions the engine makes at load (stab,
organ, wash, space, kick level), and `drift` (each wandered pass, the floor
drop, item 67's kick swap).

**Yardsticks:** generation goldens (c) and (d) equal on every case and
target. With this, every decision the JavaScript makes before scheduling has
a Rust twin, proven.

## 82. The player: a loop from a seed, in the core

**Why:** Broken Friends' gate: a friend's theme, from a seed, with no
JavaScript. And the end of phase 2's port.

**What:**
- The sequencer: what `Engine._scheduleStep`, `_organ`, `_glass`,
  `_dubStab`, the floor drops and the late floor, and `Synth`'s note calls
  do between events and the core's notes (timing, swing, articulation
  values from `js/draw.js`'s streams, sends, choke, retune, damp, the
  per-loop settings and routes), in the core, sample-accurate.
- A host call to play a loop from a seed (and vibes): `Core::load(seed,
  request)` then render; the loop wanders pass after pass as the app's does
  with the wander on (the amount a setting). Mute per layer, tempo and
  volume as the app has them.
- `dlrender --seed N [--vibes ...] [--seconds S] [--stereo]` renders it
  natively; the C header (promise 4) gets the calls.
- The app does **not** switch to it yet (phase 3, item 53 and the rest);
  `?engine=player` plays it in the app for testing.
- Tell Broken Friends: a note in `docs/notes/82-for-broken-friends.md`
  with the calls the game uses, what changed since the pin at f654fdb, and
  the hashes to refresh.

**Yardsticks:**
- **The baseline's 40 loops, full and lite, mono and stereo, played from
  their seeds natively, equal the browser's renders bit for bit** (the
  same out hashes `tools/baseline.mjs` keeps), wander on, 45 s; where one
  can't be (a float order in the page's timing), say exactly where and by
  how much, and the brain decides.
- The goldens: every target renders the same samples from a seed.
- Cost: generating a loop spread so no block passes 25% of its deadline on
  the slowest target (promise 1); generation's share of a minute's render
  reported. No allocation after `Core::new`; memory in bytes.
- A seed-to-WAV run of the corpus's first 80 loops in CI (hashes kept).
