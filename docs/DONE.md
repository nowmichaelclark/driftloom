# Done

One line for each PR that merged, newest last: the PR number, the version, the
commit preview URL, and what the ears should listen for. Add yours when your PR
merges (`docs/QUEUE.md`, standing rules). Entries before #331 are in
`docs/archive/done.md`.

- #331, the player's module at 636 KB (item 93, v149, no app change), preview
  URL: <https://f7cc7064-driftloom.nowmichaelclark.workers.dev> (commit
  e5635c1): nothing should sound different (the baseline and the 80 seeds are
  bit-identical); `js/dlplayer.wasm` is 636,125 bytes, down from 1,938,596,
  and loads only with `?engine=player` (try it: Diagnostics, late and dropped
  notes). The rest is for the hands: a new player is all zero bytes
  (`tests/zeros.rs`), and one module can replace the two at 94b.
- #337, everything the app does as a command or an event (item 94a, v149, no
  app change), preview URL: <https://33d5bb10-driftloom.nowmichaelclark.workers.dev>
  (commit 0727c51): nothing should sound different (the baseline, the 80
  seeds and the generation goldens' 4,884 cases are unchanged). Nothing to
  hear yet: the new commands (a re-roll in place, the tone, the length
  slider's end, a code's edits) and events (step, pass, phrase, section) are
  for 94b. For Mikey's ears in 94b: a re-roll on the player comes in up to a
  quarter second later than today's. The PR's table maps each control and
  display to its command.
- #341, the sketch levers for Ears 13b (item 99, v149, no app change), preview
  URL: <https://40e97708-driftloom.nowmichaelclark.workers.dev> (commit
  174499e): nothing should sound different (both wasm modules are byte for
  byte, and the 80 seeds and the 5,124 generation cases are unchanged).
  Nothing to hear yet: the sketch build has seven new levers (tripletKit,
  dottedPulse, planingSevenths, slowOpening's wait and fade, tuningOffset,
  widePad as `--space`, dropSpotlight as `--route`, and a retune at every
  chord), all off until a clip pulls them, for an Ears 13b session
  (`core/sketch/README.md`).
- #346, the picks through the core's commands (item 95, v151), preview URL:
  <https://8218f17b-driftloom.nowmichaelclark.workers.dev> (commit 4b6bf50):
  nothing should sound different (the baseline, the 80 seeds and the 5,124
  generation cases are unchanged). The core holds the New loops panel's
  picks and makes every new loop itself, at Next and where the length
  slider's time runs out, so a hand-over plays with the screen off. Listen
  for: Next and a length's end feel as before; a new loop's card comes up
  about a tenth of a second after the tap.
- #354, the fast gate (item 103, no version: CI only), commit 94e34b8: CI on
  a PR is now `tools/gate.sh`'s parts side by side, 10 minutes (it was 70,
  behind the PowerPC renders); every target's golden renders, Endless for 24
  hours and the other slow checks run nightly (`nightly.yml`) and hold no
  merge until 1.0. Listen for nothing. No album loop.
- #355, the checks on the player (item 96, no version: tools only), commit
  ca8fe56: `tools/measure.mjs` and `tools/baseline.mjs` play loops from
  their seeds on `js/dlplayer.wasm` in Node, no browser; the old path and
  the player agree to 0.005 dB on the same 40 loops. Listen for nothing. No
  album loop.
- #356, the JavaScript retires (item 97, v152), commit c9af54e: one engine
  and one generator, the core's; the page's JavaScript 19,538 lines to
  9,166; a browser that can't run the core gets a plain line. Listen for
  nothing: it should sound as v151 did. No album loop.
- #357, the too-slow line (item 100, v153), commit 30a23b9: on lite, when
  the render runs behind real time for two seconds in twenty (or the device
  fills in that often), the card's place says the browser is too slow to
  play Driftloom smoothly; Play goes on. Listen for nothing. No album loop.
- #360, session 01: the tools made steady, the stall test on the shipped app,
  the testers' steps (item 98's hands part, v154), preview URL: the branch's
  preview (no hash kept): nothing should sound different. `tools/rows.mjs`
  starts from a fixed loop (ten runs, ten passes); Diagnostics no longer prints
  the old scheduler's lines; the module that never arrives says "Please
  refresh." and the too-slow line says "This browser is too slow." The stall
  table (84 rows, full and lite, the main thread blocked, the page frozen,
  hidden a minute) reads zero late, left-out and dropped notes. The phones:
  `docs/notes/98-testers.md`.
- #361, the width ceiling after a rest (the control's two loops, v155), preview
  URL: the branch's preview (no hash kept): after a long rest the stereo
  picture comes back at once, not narrow for ten seconds. Two loops (corpus
  20261009) that draw it: **tui-doask at 31.1 s** and **soath-wa at 27.3 s**
  (25 to 50 dB under the music, loudness the same); the baseline's ri-houng
  (full, stereo) moved too.
- #363, batch 6's far ends (item 101a, session 02, v156), preview URL: the
  branch's preview (no hash kept): three approved sounds, each an option of
  the choice it extends, drawn on a stream of its own. **deepDuck**, the
  bass dipping about 12 dB at each kick and back in 0.55 s: **loa-yer**
  (corpus 20261009 #23) and **toam-woar** (#67), any time after 0:20.
  **wideSweep wide**, the pads' filter riding about four octaves over 15 to
  19 s: **brai-fei** (#46) and **sol-sham** (#77) from 0:15; **fast**, two or
  three octaves every 5 to 8 s, only with a kit: **li-roun** (#96) and
  **broa-glou** (#292) from 0:10. **slowOpening**, the kit and the bass
  waiting 40 to 90 s and fading up over 4 to 20: **rong-wuisk** (#6) from
  0:45 (the kit at 0:49, the bass at 0:53) and **gluith-vuisk** (#0) from
  1:10. A loop that draws the approved kinds plays as before.
- #366, the nightly's stereo hashes after #361 (no app change), preview URL:
  the branch's preview (no hash kept): nothing should sound different. #361
  had moved three stereo goldens (06-undertow, 15-wayfare, 18-shatter) on
  every target without updating `core/golden/*.stereo.hashes`; they are
  regenerated on x86_64 and wasm32 (identical) and copied to the others.
- #367, tuningOffset (item 101b, session 03, v157), preview URL: the branch's
  preview (no hash kept): about 1 loop in 6 (a quarter of the hazy ones, 1
  in 12 of the crisp ones) is tuned 15 to 35 cents off A440 as a whole,
  every pitched voice together; the kit, the tuned kick and the noises keep
  their pitch. Two loops (corpus 20261009) that draw it: **wen-glui** (#50,
  32.5 cents flat) and **kask-hoan** (#43, 28.3 sharp), from 0:00; against
  a loop at A440 just before, the whole loop sits a little lower or higher,
  no voice out of tune with another. `tuning.cents=0` holds a loop at A440.
- #368, the nightly's Emscripten step (v158, no sound change), preview URL:
  the branch's preview (no hash kept): nothing should sound different (the
  goldens, the seeds and the baseline are unchanged). Since #363 the
  player's calls took about 70 KB of a C host's stack, past Emscripten's
  64 KB default, and the nightly's emcc host crashed on its second loop; the
  generator's stages are now functions of their own and take 45 to 47 KB.
- #369, padWow (item 101b, session 03, v159), preview URL: the branch's
  preview (no hash kept): about 1 loop in 6 with chords (a quarter of the
  hazy ones) has a slow, tape-like wow on the chords alone, 12 to 14.5 cents
  at 0.4 to 0.6 Hz with a slower third under it, never past 20 cents; the
  kit and the bass stay steady. Two loops (corpus 20261009) that draw it:
  **soath-wa** (#27, 13.0 cents at 0.58 Hz, the organ's held chords) from
  0:00 to 0:19, and **sheith-sith** (#47, 13.4 cents at 0.54 Hz, a pad) from
  0:14. A loop without it plays as before.
- #373, long pieces: the plan and its map (item 102a, session 11, v160),
  preview URL: the branch's preview (no hash kept): nothing should sound
  different (the goldens, the balance lock and the baseline are unchanged;
  the module grew 16 bytes). Nothing to hear yet. To read a piece before it
  is heard: in `core/`, `cargo run --release --example dlgen -- --map 12
  bars=256,journey=arc` (and `journey=wander`, `bars=endless --from 1000000
  --count 60`); the PR has eleven maps. The design that stands is
  `docs/LONGFORM.md`; the checks are `core/tests/longform.rs` and `node
  tools/longform.mjs`; the first picks waiting for his ears are in
  `docs/notes/for-mikey.md`, "Decisions".
- #375, long pieces: the wander heard (item 102b, session 12, v161),
  preview URL: the branch's preview (no hash kept): nothing in the app
  changes yet (it can't take a journey; loops of up to 32 bars and every arc
  piece render bit for bit as before, 144 renders against main). A piece
  whose journey is the wander now plays its plan: about every three minutes
  one thing moves, slowly (a layer fades out or back over 24 s, the tone
  warms or the room changes, the tune thins), or on a line (a voice, the
  chords an octave up or down, their lengths, rarely the whole loop a fourth
  away). The longest stretch with nothing moving is 300 s (was 587 s). To
  hear before session 15's excerpts: in `core/`, `cargo run --release
  --example dlrender -- --seed 7 --request bars=endless,journey=wander
  --seconds 1560 --wav glor-ti.wav`; the moves' minutes are in
  `docs/LONGFORM.md`, "Pieces to hear".
- #377, long pieces: the arc heard (item 102c, session 13, v162), preview
  URL: the branch's preview (no hash kept): nothing in the app changes yet
  (it can't take a journey; loops of up to 32 bars and every wander render
  bit for bit as before, 144 renders against main). A piece whose journey
  is the arc now rises and falls: a thin, slightly dark opening (the kit
  and the bass held back, no hats, the tune sparse, the loop's own filter
  low), the build heard only once it has passed (hats and backbeat in, the
  filter opening, the tune doubled an octave up toward the peak, about 1 to
  2 LU louder by ear), the release, the return, and a close that thins to
  the chords and the tune and lands on the tonic chord, ringing out. To
  hear: tha-vol (corpus 20261009 #18, seed 2728346932, `bars=64,journey=arc`)
  from 0:32 (the lift) and 1:16 (the close, the landing at 1:23); brum-yei
  (#17, seed 1151594917, `bars=256,journey=arc`) from 2:20 (the peak at
  2:38) and 9:00 (the close). In `core/`: `cargo run --release --example
  dlrender -- --seed 2728346932 --request bars=64,journey=arc --seconds 100
  --wav tha-vol.wav`; the times are in `docs/LONGFORM.md`, "Pieces to hear".
- #379, long pieces: themes that return changed, related keys, movements
  of kinds (item 102d, session 14, v163), preview URL: the branch's preview
  (no hash kept): nothing in the app changes yet (it can't take a journey;
  loops of up to 32 bars play bit for bit as before, the core baseline
  unmoved). An arc is now a piece with ideas: a second theme (another
  seed's tune bent into the loop, in the related key), a development
  trading fragments (now and then a call and response in one voice), theme
  A home again with one variation (an octave away, another voice,
  stretched, over the relative key's chords, answered), the bass holding
  the home fifth for a phrase before it, movements of four kinds in a
  piece of three or more, a finale that recalls the earlier movements'
  second themes, and in a wander the tune now and then handed to the
  second theme. To hear: brum-yei (corpus 20261009 #17, seed 1151594917,
  `bars=256,journey=arc`) from 1:00 (B enters 1:10 in F#, the call and
  response 1:45, the return 3:13) and from 6:40 (the look back, the
  answered return 8:13); fei-no (#42, seed 2158762586,
  `bars=256,journey=arc`) from 2:35 (B in the relative key 2:43) and 7:00
  (the stretched return 7:05). In `core/`: `cargo run --release --example
  dlrender -- --seed 1151594917 --request bars=256,journey=arc --seconds
  570 --wav brum-yei.wav`; every time is in `docs/LONGFORM.md`, "Pieces to
  hear".
- #381, long pieces: the joins, endings that land, the breath between arcs
  (item 102e, session 15, v164), preview URL: the branch's preview (no hash
  kept): nothing in the app changes yet (it can't take a journey; loops of
  up to 32 bars play bit for bit as before, the core baseline unmoved;
  Endless and long pieces stopped by the length slider now land). Where two
  units of a long piece meet and something changes, the line is written: a
  chord both keys share held across a key's line (the bass on its root), a
  fill of the kit's backbeat into a rise, the floor out after the peak, the
  tune alone or a breath where a theme changes, a wander's line move a
  breath of its layer. The breath between arcs thins the tune across it
  instead of resting it. When the length slider will stop a piece or
  Endless before its close, its last phrases thin to a close and land on
  the tonic, ringing out, where a fade was. A landing struck at velocity 0
  since 102c fixed. To hear: brum-yei (corpus 20261009 #17, seed
  1151594917, `bars=256,journey=arc`) from 0:50 (a pivot into theme B at
  1:10) and 2:23 (the peak, the floor out at 2:56); fei-no (seed
  2158762586, `bars=256,journey=arc`, the slider at 120 bars) from 3:35, the
  close it makes landing at 4:05. In `core/`: `cargo run --release --example
  dlrender -- --seed 2158762586 --request bars=256,journey=arc --track
  bars=120 --seconds 250 --stereo --wav fei-no.wav`; every clip is on the
  branch `long-clips` (`docs/notes/for-mikey.md`, the listening set).
- #382, long pieces: the measures and the listening set (item 102e, session
  15), docs and a tool (no app change): everything `tools/longform.mjs`
  checks, over 240 seeds at 64, 256, 1,024 and 32,768 bars and Endless, in
  the PR (one check of 24 short once, as on `main`); the nightly run by hand
  on #381; and the listening set for brain B3's page: 31 clips, 23
  minutes, on the throwaway branch `long-clips` (the pieces, how they were
  chosen and what to listen for at the top of `docs/notes/for-mikey.md`;
  two blind pairs, before item 102 and now). To hear: tha-vol (seed
  2728346932, `bars=64,journey=arc`), the whole small form in 1:34; glor-ti
  (seed 7, `bars=endless,journey=wander`) from 3:05, the wander's first
  move. `node tools/longform-clips.mjs OUTDIR --before DLRENDER` makes the
  set again.
- #385, long pieces: the two holes fixed, and a yardstick against holes
  (item 102f, session 16, v165), preview URL: the branch's preview (no hash
  kept): nothing in the app changes yet (it can't take a journey; loops of up
  to 32 bars play bit for bit as before, the core baseline unmoved). Nothing
  retuned (Mikey kept everything). A pivot is struck at the loop's first
  sounding chord's velocity, never a silent entry chord's 0; "the tune
  alone" only over a tune that sounds there as the unit plays it, else a
  breath; `tools/longform.mjs` holds every arc's join from a hole, and
  `core/tests/joins.rs` every pivot struck. To hear (corpus 20261009):
  fei-no (#42, `bars=256,journey=arc`) from 5:15, a chord at the key's line
  into the peak (5:25) where v164 had near silence, and from 7:55, a breath
  into the close (8:09) where v164 had three seconds near silence; loa-yer
  (#23, `bars=1024,adventurous=0.8`) from 3:44, the chord both keys share at
  3:54. Six clips, now and v164, on `long-clips` (`docs/notes/for-mikey.md`).
- #387, widePad (item 101c, session 04, v166), preview URL: the branch's
  preview (no hash kept): about 1 loop in 6 with chords (a quarter of the
  spacious or floating ones, 1 in 20 of the intimate ones) has its chord
  layer much wider in stereo, on the side only: the Mono switch and a
  phone's speaker hear exactly what they did, and the bass and the kick stay
  centered. Half of them are wide from the start, half open from today's
  width to the wide one over 60 to 120 s. Two loops (corpus 20261009) that
  draw it: **loa-yer** (#23, wide from the start, a busy loop) from 0:10,
  with headphones, against the Mono switch; **broan-bro** (#39, the late
  kind) from 0:00 to 1:26 as it opens, then held wide. A loop without it
  plays as before, in stereo and mono.
- #390, the FLAC writer in the core (item 104a, session 19; core only, no
  version: the app and `js/dlplayer.wasm` are unchanged), preview URL: none
  needed (nothing in the app moves). The lossless export's encoder,
  `core/src/flac/`: streaming, a fixed 122,576 bytes for a minute or three
  hours, the same bytes natively, in wasm and from C. Nothing to listen for
  yet: session 20 puts it in the app. To check: `dlrender --seed 1234
  --stereo --flac out.flac` writes a file any player opens; `node
  tools/flac.mjs` holds 328 renders to `flac` (about a fifth of a WAV at 16
  bits, a third at 24).
- #391, tripletKit (item 101d, session 05, v167), preview URL: the branch's
  preview (no hash kept): about 1 loop in 7 with a kit (none in 6/8; a fifth of
  the swaying ones, 1 in 11 of the still ones) plays it in threes, the 12/8
  shuffle: the hats three a beat, the eighth moved to two thirds of the beat,
  the kick on the beat, no brighter than its own kit. Two loops (corpus
  20261009) that draw it: **dun-roar** (#45, a busy machine kit at 129) from
  0:00; **thisk-doam** (#76, a slow tape kit at 55) from 0:00. A loop without
  it plays as before, its wander too.
- #389, the Android project and its CI build (item 107a, session 25, no app
  change; the only file outside `android/` and the workflow is
  `.assetsignore`): nothing to hear (the page plays in the WebView as in a
  browser). The APK is 2.0 MB (debug), builds from a clean checkout in about
  1.5 minutes, and on an Android 15 emulator in CI the page loads, Play is
  pressed and Diagnostics reads `rust (the core's player)  late: 0  missed: 0
  dropped: 0  player: playing`. Where to get it and what to check:
  `docs/notes/for-mikey.md`.
- #392, dottedPulse (item 101d, session 05, v168), preview URL: the
  branch's preview (no hash kept): about 1 loop in 7 with a 4/4 kit (nearly a
  quarter of the groovy ones, 1 in 12 of the still ones) plays a lilt against the bar,
  half and half: `pulse`, a soft rim every three eighths under the beat, and
  `kick332`, the kick on a 3-3-2 of the bar's eighths, written whole. Two
  loops (corpus 20261009) that draw it: **ta-muin** (#121, the pulse) from
  0:16, as the kit comes in; **yi-yoar** (#51, the 3-3-2) from 0:00. A loop
  without it plays as before, its wander too.
- #395, long pieces: no join a hole, on any piece (item 102g, session 16b,
  v169), preview URL: the branch's preview (no hash kept): nothing in the
  app changes yet (it can't take a journey; loops of up to 32 bars play bit
  for bit as before, the core baseline unmoved). A tune alone, a floor out
  or a breath that its notes say would leave the mix 12 dB under falls back
  to a breath, or to nothing written; every other join is as Mikey kept it.
  To hear, as Endless arcs (`bars=endless,journey=arc`): ta-muin
  (1337654778) at 2:05 and 9:26, the loop now playing on across the line
  where the tune alone left two seconds near silence; yi-yoar (819045617)
  at 6:14, a short breath where the kit and the bass dropped out. The clips
  are on `long-clips`.
