# Done: before #287

One entry for each PR that merged before #287 (phase 2's first item, 2026-10-30),
as they stood in the queue's Done list, then in `docs/DONE.md`: the PR number,
the version, the preview URL, and what the ears should listen for. The preview
URLs from the old account depend on that account. Newer entries: `docs/DONE.md`.

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
  longer. For the album: cinder #12 at 0:03.5 and tide #52 at 0:00.
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
  (tei-har), where they play at full level. For the album: thaw #82 from 0:02.8
  and thaw #58 from 0:07.5 (re-checked 2026-10-26: vapor #30 no longer draws
  it).
- #253, dubStab (item 60, v109),
  https://9d274c1b-driftloom.nowmichaelclark.workers.dev: in busy loops
  (drums and seven or more notes a second; about one loop in four), on the
  offbeat after beat one or three, by a density drawn per loop, a soft stab of
  the chord sounding now (G3-C5) on a core voice of its own: a triangle under
  two saws 6 cents apart, a 22 ms attack, a lowpass closing toward 700 Hz, a
  short tail, and a darker, lighter throw into the echo. Only over a groove,
  never in a silent bar, and it never costs another part a note. Listen for a
  warm dub chord that answers the beat and dissolves into the echo without
  taking over the loop. For the album: bloom #37 from 0:00.3 (busy) and
  cinder #12 from 0:20.9 (sparse) (re-checked 2026-10-26: wayfare #23 no
  longer draws it).
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
  that change without a gap. For the album: haven #56 from 0:00 and dust #49
  from 0:00 (re-checked 2026-10-26: haven #38 and glade #21 no longer draw
  it).
- #255, the wash (item 62, v111),
  https://a12a0cdb-driftloom.nowmichaelclark.workers.dev: in loops that draw it
  (about two in five), a slow swell of air every 17-26 s on a channel of its
  own: two soft noises through lowpasses that open from 450 Hz to about 2.5 kHz
  and close again, tinted by two resonances on the loop's key (its root or
  fifth), like wind across a bottle. Its level follows the other five layers,
  so it is heard under a beat, stays behind a sparse loop and fades with the
  loop into a rest. Listen for air that breathes behind the music and never
  hisses or sits in front. For the album: vapor #30 at 0:15.7 and undertow #45
  at 0:22.4 (each at its first swell's peak; re-checked 2026-10-26: shrine #0
  and grove #47 no longer draw it).
- #256, the arc (item 63, v112),
  https://1ae42640-driftloom.nowmichaelclark.workers.dev: in loops that draw it
  (about one in three), a second lowpass after the master's tone breathes from
  300 Hz up to 9 kHz and back over a period drawn 2-5 minutes, lingering on the
  dark side, and the echo's feedback rises a little as it opens; a loop starts
  at a bright point of its swing (the prototype started closed: for Mikey's
  ears). The group's balance lock re-baselined. Listen for the whole mix
  breathing dark and bright over minutes, and for the instruments always
  coming back. For the album: bloom #66, bright from 0:00, darkest at 1:33.6,
  and dust #42, darkest at 1:26.6 (both the deep swing; re-checked 2026-10-26:
  haven #38 and undertow #6 no longer draw the arc).
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
  For the album: bloom #79 (deep), bright from 0:00, darkest at 1:44.8; dust
  #40 (dip), open until 0:39.5, darkest at 1:03.5 (884 Hz), open from 1:27.5
  (re-checked 2026-10-26: vapor #4 and wayfare #25 no longer draw the arc, and
  Mikey's loop, seed 102356715, now draws the organ, the wash and padSweep
  instead).
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
  don't click. For the album: dust #32 from 0:00 (a long boom on B1) and
  undertow #59 from 0:15.6, when its kit comes in (short, on A1; it draws
  lateFloor too) (re-checked 2026-10-26: undertow #6 and glade #63 no longer
  draw it).
- #267, subDuck (item 68, v118),
  https://c54ff1f8-driftloom.nowmichaelclark.workers.dev: in loops with a kick
  and a bass that draw it (about one in eight), the bass alone dips 5-9 dB at
  each kick, less for a soft one, and comes back over 0.15-0.25 s, so the kick
  and the sub take turns while the pads and the tune stay where they were.
  Listen on speakers or headphones with some low end for a bass that breathes
  with the kick without pumping, and for nothing else moving. For the album:
  grove #31 from 0:00 (8.9 dB) and glade #67 from 0:00 (5.4 dB) (re-checked
  2026-10-26: halcyon #18 and halcyon #68 no longer draw it).
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
  undertow #6 from 0:00 (the kit at 0:13.4, the bass at 0:17.8) (re-checked
  2026-10-26: clockwork #33 no longer draws it).
- #269, floorDrop shapes (item 70, v120),
  https://29f8bbc6-driftloom.nowmichaelclark.workers.dev: with "Let the loop
  wander" on, in loops that draw them (about one in six), each floor drop takes
  a shape: single as before, stagger (the kick back on time, the bass a bar or
  two later), stutter (two to four short drops a couple of bars apart) or long
  (a dozen seconds, or the whole pass of a short loop); the drops come as often
  and where they did. Listen for a floor that leaves and comes back in more
  than one way, never a full stop. For the album, both ends: vapor #4, the
  whole pass dropped for 34.3 s from 1:08.6 (a slow 8-bar loop), and cinder
  #93, two 2.8 s stutter drops from 1:55.4 (128 bpm) (re-checked 2026-10-26:
  halcyon #18 no longer draws the shapes).
- #270, pedalBass (item 71, v121),
  https://3ba6d476-driftloom.nowmichaelclark.workers.dev: in loops that draw it
  (about one in eight), a held, pulse, dub or walking bass holds the key's
  tonic under whatever the chords do, held, in half bars, on the beats or off
  them, now and then stepping to one neighbor (a whole tone up, the tone
  below, the flat second or the fifth) between downbeats. Listen for a floor
  that stays put while the harmony moves over it. For the album: clockwork
  #64 from 0:00 (a beat pedal, the fifth on about a fifth of its notes, drawn
  on its own); undertow #8 from 0:00 (the heavier low end; it now draws the
  pedal on its own, half bars a whole tone up, as forced before); with
  pedalBass forced on, dust #69 from 1:02 (its busiest moment, 60 → 66
  refused: do the missing chord and bell notes matter?) and clockwork #65
  from 0:00 (the lighter one) (re-checked 2026-10-26: cinder #5 no longer
  draws it).
- #271, padSweep (item 72, v122),
  https://0231e9a4-driftloom.nowmichaelclark.workers.dev: in loops whose chords
  are a pad, an organ, a choir or another held voice and that draw it (about
  one in twenty), a lowpass on the chord layer alone swings from closed at
  450-800 Hz up 2-3.5 octaves and back once every 12-28 s, while the rest of
  the loop stays as bright as it was. Listen for pads that breathe dark and
  bright, slowly, and for nothing else moving. For the album: dust #77 from
  0:00 (a hum pad, 3.4 octaves every 23.7 s, drawn on its own) and thaw #71
  with padSweep forced on, from 0:00 (the pad is most of what plays).
- #272, echoTime (item 73, v123),
  https://75ca9b80-driftloom.nowmichaelclark.workers.dev: every loop draws its
  echo's time, in beats: today's dotted eighth (45%), one beat (25%), half a
  beat (15%) or two thirds (15%), one beat leaning spacious and floating, half
  a beat nimble and skipping. Not counted among a loop's changes, so nothing
  else moves; the tempo slider keeps the loop's own time. Listen for echoes
  that fall where the music breathes rather than always on the same off-beat.
  For the album: haven #38 from 0:00 (one beat, 779 ms) and thaw #10 from 0:00
  (half a beat, 448 ms).
- #274, the docs split by role (no version bump; docs and comments only,
  `main` redeploys): nothing to hear. `AGENTS.md`, `core/AGENTS.md` and
  `docs/README.md` are new; the queue, the brain's file, the roadmap and the
  decisions are small live docs, with the history in `docs/archive/`; queue
  item 76 (the first loop after the app starts) is queued.
- #277, the cold start (item 76, v124), preview URL: the Workers Builds check
  on #277 (its link carries no hash I can read from the session): until a note
  has reached the core, the first notes are planned no closer than the
  device's output latency and never less than 250 ms, so the first loop after
  the app starts keeps its first notes (the Moto lost six). Listen for sound
  that starts up to a quarter of a second after Play on the first loop after
  the app starts (and after a Next in its first moments), and for no missing
  first notes. Tester: close the app, open it, Play, let the first loop end,
  copy Diagnostics, on full and on lite: `late: 0`; any late note says
  `[margin]` or `[measured]`. No album loop (nothing in the music changed).
- #280, the vibes to 88 (item 76b, v125), preview URL:
  <https://19b7a2e6-driftloom.nowmichaelclark.workers.dev> (commit 1fa8bf2):
  twenty-one new vibes, soft, energetic, silly and comfy gone, codes made
  before v125 may play other vibes. Listen to each new vibe alone, above all
  glacial (the arc in most of its loops, deep, 5-8 minute swings; 8-12 with
  still, droning or another of its neighbors), droning, moonlit and mellow
  (no kit) against chill, groovy, pulsing and cruising (a beat), and the four
  pairs side by side: foggy and misty, experimental and strange, airy and
  breezy, mellow and chill. Lively should drive as hard as energetic did,
  restful should sound like a lullaby. Loops without vibes are unchanged.
- #282, the bass level (item 76c, v126), preview URL:
  <https://6fee2058-driftloom.nowmichaelclark.workers.dev> (commit 3383d71):
  the bass layer is +3 dB (by ear, K-weighted) in loops that play a kick (a
  soft kick or the hand kit's frame drum count), today's level in loops
  without one; rhodesbass (an FM voice) stays as it was. Listen to dust #7
  and noan-nou (corpus seed 20261009 and the baseline's seed 1, 0:20 on) for
  the fuller low end, and to dust #42 (no kit) for no change at all. A kit
  or bass that comes in late (lateFloor) or drops out (floorDrop) keeps its
  loop's level throughout. Loops without a kick are unchanged to the bit.
- #283, lateFloor's second arrival drawn (item 76d, v127), preview URL:
  <https://0f9be036-driftloom.nowmichaelclark.workers.dev> (commit 8d0490c):
  when the kit and the bass arrive apart, the gap between them is drawn per
  loop, 1-8 bars (the short end commonest, mean about 2.5 bars; it was always
  2), and it ends inside the loop's first pass when the first part does.
  Calm, vast vibes (glacial, droning, meditative, serene, still, spacious,
  floating, dreamy, hypnotic) lean the wait and the gap long; lively ones
  (lively, enthusiastic, bouncy, skipping, groovy, pulsing, cruising) lean
  them short. Listen to undertow #45 with lateFloor forced (kit 0:30, bass
  0:34 before) and to any lateFloor loop under glacial against lively: the
  second part should not always come two bars after the first. Loops
  without vibes, or that draw `together`, wait as before.
- #284, clippy pedantic at zero (item 74, v128), preview URL:
  <https://4c9485da-driftloom.nowmichaelclark.workers.dev> (commit 201d923):
  a refactor of the core's source in the eleven files item 46 left; CI now
  fails on any pedantic clippy warning. Nothing should sound different: the
  goldens on every target and mode, the core baseline and the sample-rate
  checks did not move, and native render time is the same to 0.05%. Listen for
  nothing. Tester: the app reports v128 (its `.wasm` has new bytes, so the
  service worker fetches it once). No album loop (nothing in the music
  changed).
