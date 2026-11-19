# Done

One line for each PR that merged, newest last: the PR number, the version, the
commit preview URL, and what the ears should listen for. Add yours when your PR
merges (`docs/QUEUE.md`, standing rules). Entries before #357 are in
`docs/archive/done.md`.

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
  at 6:14, the same where the kit and the bass dropped out. The clips are on
  `long-clips`.
- #396, colorTone (item 101e, session 06, v170), preview URL: the branch's
  preview (no hash kept): about 1 loop in 7 that can take it (nearly a quarter
  of the jazzy ones, 1 in 11 of the minimal ones) holds a soft color over some of
  its chords, a 13th over the major ones and an 11th over the minor ones,
  just above the chord (C6 at most), swelling in and letting go; about 43% of
  the chord time where it plays. Two loops (corpus 20261009) that draw it:
  **luir-wo** (#97, 76 BPM) from 0:05; **broan-mei** (#163, 109 BPM, a kit in
  threes too) from 0:02. A loop without it plays as before, its wander too.
- #397, planingSevenths (item 101f, session 07, v171), preview URL: the
  branch's preview (no hash kept): about 1 loop in 7 (1 in 5 of the dreamy and
  floating ones) slides major seventh chords a whole step or a minor third at
  a time (Ears 13b's chords), and the tune and the bass now follow each chord
  in its own Lydian (its major pentatonic in a five-note mode): no rub against
  the chords. Two loops (corpus 20261009) that draw it: **muith-sei** (#62, 89
  BPM) from 0:01; **broan-bro** (#39, 122 BPM) from 0:06. The balance lock was
  re-baselined (bars quoting it audibly 0.6364 to 0.6215); a loop that does
  not plane plays as before.
- #398, lowBell (item 101g, session 08, v172), preview URL: the branch's
  preview (no hash kept): about 1 loop in 7 that plays a chord strikes a low
  bell (D2 to C#3) on the chord's root at each chord whose root changes, half
  of them struck and half swelling in over 0.4 s, at the sketch's level (14 to
  24 LU under the loop). Two loops (corpus 20261009) that draw it:
  **weith-yo** (#49, struck) from 0:00; **shu-san** (#16, swelling) from 0:00
  (the PR named four-hoask and muith-sei, which since #399 strike it in their
  floor drops alone). A loop without it plays as before, its wander too.
- #399, dropSpotlight (item 101g, session 08, v173), preview URL: the branch's
  preview (no hash kept): in about half of the loops that play the low bell
  and whose floor can drop, the bell rings only where the kick and the bass
  have both left for 2 s or more, struck on the drop's first beat (alone
  there where the chords have gone too) and at each new root, a little
  louder, damped as the floor comes back; never in a full stop. Two loops
  (corpus 20261009) that draw it: **sei-sheim** (#173, struck) from 0:13;
  **wu-hail** (#197, swelling) from 0:08. Every other loop plays as before.
- #400, twang (item 101h, session 09, v174), preview URL: the branch's
  preview (no hash kept): on about 1 in 7 of the loops that play a chord and
  have no kit or a soft one, a low, round, plucked synth under one slow
  tremolo picks each chord whose lowest note changes (or the same after 3 s):
  that note between E2 and D#3 with its fifth and its tenth, struck
  together as the approved clips struck them (`spread = 0`; 1 picks them a
  beat apart: Mikey's call). soft trembles at 4.5 Hz, slow twice a beat and
  rounder, bright at 5.5 Hz with a brighter pick; let go in a full stop. Two
  loops (corpus 20261009) that draw it: **he-rosk** (#33, bright) from 0:00;
  **maisk-feim** (#151, soft) from 0:00 (and tom-wen, #172, slow, from
  0:00). Every other loop plays as before.
- #401, batch 6's locks (item 101h, session 09; no app change), preview URL:
  the branch's preview (no hash kept): nothing should sound different. The
  balance lock re-baselined once for the batch at 10,000 loops (it came out
  byte for byte as session 07 left it: of the 22 locked figures only 07's
  planingSevenths moved any, from the pre-batch file); every batch 6
  candidate's two album loops checked to still draw it; how many changes a
  loop now gets with all of the batch in (52% one or two, 43% three or
  more) left for Mikey with three measured ways back
  (`docs/archive/notes/101h-change-counts.md`). The batch's table (each candidate,
  its options, its share, its two loops) is in the PR.
- #402, Mikey's two calls for batch 6 (item 101h, 2026-11-17, v175), preview
  URL: the branch's preview (no hash kept). **twang** picks a chord's three
  notes a beat apart, low and slow (`spread = 1`), where v174 struck them
  together: he-rosk (#33, bright), maisk-feim (#151, soft) and tom-wen
  (#172, slow) from 0:00. **How many changes a loop gets:** batch 6's nine
  draws count against the changes a loop drew and the list fills what they
  leave (way A), so 78.5% of loops get one or two (52% at v174) and 16.6%
  three or more (43%); 56% of loops' choices move, so their codes play
  differently. Five album loops gave their far end to the batch's later
  sounds; the far ends' loops now (corpus 20261009): deepDuck **vor-the**
  (#136) from 0:10 and **toam-woar** (#67) after 0:20; fast **meil-broa**
  (#264) and **moal-doang** (#812) from 0:00; slowOpening **tui-doask** (#2)
  from 1:14 and **sir-houl** (#105) from 0:48. Every other candidate's two
  loops still draw it.
- #403, the keepalive without the hiss (2026-11-17, v176), preview URL: the
  branch's preview (no hash kept). Mikey heard a hiss behind every loop: the
  media session's hidden keepalive was noise at -62 dBFS up to 4 kHz, played
  under everything (the core's renders of his six loops have no floor: their
  quietest tenth falls to -79 to -141 dBFS above 5 kHz). It is now a 20 Hz
  sine at the same power (-60 dB), 300 cycles in its 15 s, written by
  `tools/keepalive.py`. Any loop, a quiet one best, on headphones: no hiss;
  and the lock-screen and Bluetooth controls still come up (his phone's
  check).
- #404, the chords nearer the tune (item 84b, session 10, v177), preview URL:
  the branch's preview (no hash kept). Where item 84's whole octave down left
  the chords 12 to 17 semitones under the tune, they now go down only as far
  as a fifth under it: a few of each chord's top notes an octave down, the
  same chord in another inversion (2 semitones deeper with held chords). 1,003
  of 2,000 corpus loops move, every one nearer; the median gap 12.6 to 9.2,
  loops over 12.5 942 to 100; inside the arcs the same (theme A at home 13.4
  to 8.6). The chords read up to 2.5 LU louder and the loops wider in stereo,
  both from sitting higher. Item 84's downward movers, now nearer (corpus
  20261009): by a lot **shu-san** (#16, nylon) 17.0 to 8.0 from 0:00,
  **glai-thel** (#1843, keys) 18.0 to 8.5 from 0:01, **youn-ti** (#1956,
  prepared piano) 18.3 to 9.3 from 0:00; by a little **theim-rem** (#68,
  keys) 18.0 to 13.2, **fum-doun** (#1982, accordion) 17.3 to 13.9, **fo-roan**
  (#1803, pad) 18.0 to 14.6, each from 0:00. A revert follows if Mikey hears
  worse.
- #405, the checks a sound change needs, in minutes and in CI (2026-11-17;
  Mikey's yes in session 10 to the rule). The core's tests optimized
  (`[profile.test]` opt-level 3, debug assertions and overflow checks on):
  `tools/gate.sh core` 6:55 to 1:22 locally, CI's `core` job 9:33 to 2:14
  and `sketch` 14:10 to 1:54; the app's .wasm, the goldens, the baseline
  and the seeds unmoved. The full `tools/longform.mjs` in CI: eight shards
  (`--shard`, `--merge`), about 7 minutes, on PRs that touch the
  generator, the player or `data/`; its table the one-process run's, check
  for check. Main's table kept by the nightly and every push to main (the
  `longform-table` artifact), and a PR's run held to it (`--against`:
  "kept every check main kept"). The rule: the gate and the long pieces
  read from CI, never run side by side at home.
- #411, the core native on Android's own audio (item 107b, session 26, no
  app change; `android/`, its workflow and docs): nothing changes on the site.
  The APK carries the core built for each ABI from the same source
  (`android/native`, AAudio straight from Rust, a lock-free queue, JNI), and
  debug builds get a second icon, **Native audio test**. On CI's emulator a
  seed played five minutes with 0 underruns and 0 late, missed or dropped
  notes; the golden loop and a 48 kHz stereo render hash the same as
  `js/dlplayer.wasm`'s; the busiest callback took 2.09 ms, 10.5% of a
  960-frame block's deadline (the emulator's, no fast path). APKs: arm64-v8a
  3.4 MB, armeabi-v7a 3.1 MB, universal 6.2 MB. What to try on the Moto and
  the tablet: `docs/notes/for-mikey.md`.
- #412, codes, version 2, in the core (item 105a, session 22, v178),
  preview URL: the branch's preview (no hash kept). Nothing to hear: the app
  still writes version 1 codes until session 23, and nothing that plays
  moved (every generation golden as it was). New: `core/src/code.rs` reads
  and writes loops and albums as short codes of `A-Z a-z 0-9 - _` (a plain
  loop 9 characters, a 12-track album from one seed with two vibes 19),
  every name by its id in `data/codes.txt` with the version it joined, and
  a code's loop drawn at its version so later additions never move it
  (`tests/dummy.rs`). The format is `core/README.md`, "Share codes, version
  2", frozen by `core/golden/share-codes.txt`; `node tools/codes.mjs` makes and
  opens codes and albums from the command line. The app's module grew 10 KB.
- #414, twang soft only (Mikey's Batch 6 Album, session 17, v179), preview
  URL: <https://claude-s17-twang-soft-driftloom.nowmichaelclark.workers.dev>
  (commit 416462c): every loop that drew twang plays the soft version; the
  draw is kept whole, so the same loops play twang and no other loop moved
  (239 generation goldens, each a slow or bright twang loop, only its load
  part; the baseline's dith-shuisk and tei-har). Listen to he-rosk (corpus
  20261009 #33, was bright) and tom-wen (#172, was slow): the same picks, in
  the soft voice.
- #417, the jump within a long piece, where it is, and the map for the page
  (item 106a, session 17, v180), preview URL:
  <https://claude-s17-jump-driftloom.nowmichaelclark.workers.dev>: nothing
  should sound different (the generation goldens, the core's render tests and
  the baseline hold). Nothing on screen yet (session 18): the core jumps to a
  section, a bar or a time, heard about 0.06 s after asked three hours into a
  32,768-bar piece or Endless, no late note, playing's notes bit for bit; it
  says where it is (the bar of how many, the time played and left); and the
  page reads the piece's map (`pieceMarks`: a whole piece's movements in about
  10 ms). The PR's table has the commands and the measures
  (`dlrender --jumps`).
- #416, audio export in the app, lossless, safe first (item 104b, session
  20, v181); preview URL: the branch's preview (no hash kept). Nothing that
  plays moved. Save, the Share tab, Audio: a loop or a piece exported as a
  FLAC by its own instance of the core in a Worker (`js/export-worker.js`,
  `js/export-render.js`), written as it is made (the save picker, else the
  site's private file system and a download, else memory to 10 minutes),
  free space checked first, Cancel leaving no file, "lighter on processor"
  making it gentle. Check on the Moto and the tablet (`for-mikey.md`): a
  30-minute export while the music plays, nothing stuttering. Every file
  nulls against a straight render (`tools/export.mjs`, in the gate);
  `tools/rows.mjs` has the export's rows and `tools/export-long.mjs` the
  three-hour run (the `export` workflow). The app's module grew 23 KB.
- #420, the Android app plays on the native core (item 107c, session 27,
  v182): the page's player posts the worklet's very messages over the app's
  bridge (`js/native.js`, `DriftloomNative`; `android/native/src/bridge.rs`)
  and hears the worklet's messages back, so every control and display of
  94a's table drives the native core; the WebView plays no audio of its own
  (no keepalive). Nothing that plays moved, and the website is unchanged.
  On the CI emulator: `tools/rows.mjs --app` (every row that makes sense
  there, the length slider's minute included), the page's main thread
  blocked 5 s with the music on through it (no underrun, nothing late), the
  too-slow line unable to show; the smoke job waits for the page as it is
  (`android/cdp.mjs`) and passed ten fresh emulators at once. Diagnostics in
  the app: `engine: native (...) underruns` and a `stream:` line. Check on
  the Moto and the tablet (`for-mikey.md`).
- #419, long pieces on screen (item 106b, session 18, v183), preview URL:
  <https://claude-suspicious-raman-f88179-driftloom.nowmichaelclark.workers.dev>:
  nothing that plays moved (the module grew 124 bytes: `dl_play_journey`, the
  one core addition, so the page can ask a journey). Now Playing for a piece
  past 32 bars or Endless: the line of its sections with the map's markers
  (a second theme, the peak, the return, the close, a wander's turns), bar 812
  of 3,072, the time played and left (Endless: played), a tap or drag on the
  line to jump and a key each way to the section before or after. The Bars
  picker has the journey (arc, wander) beside the ∞ and any, shown as a pill
  beside the bars, sent as the request's journey; the Library's lists end in
  each loop's length. Listen to a jump into the middle of a piece: it should
  come in on a phrase line within a blink, nothing ringing from before.
- #424, audio export: the Small format (item 104c, session 21, v184): the
  export sheet's Small choice makes an Opus file (`.opus`, Ogg) through the
  browser's own encoder (WebCodecs `AudioEncoder`, in the export's Worker),
  muxed by a small tested Ogg writer (`js/ogg.js`, `test/ogg.test.mjs`: the
  CRC, the headers, granule positions and pre-skip, packets past the end
  dropped, so players show and seek the true length, to the sample); hidden
  where the browser has no Opus encoder (a browser with AAC and not Opus
  too). 64 kbit/s mono and 96 kbit/s stereo, constant: about 2.2 (mono) and
  2.9 (stereo) times smaller than the median lossless file; 2.5 hours mono
  is about 73 MB. Also Mikey's answers on the sheet: "Closing the app stops
  it", and a loop with no name of its own saves under its generated name.
  Nothing that plays moved (the lossless file's checks are unchanged). Check
  in `for-mikey.md`.
- #425, a loop's own name travels with its code (item 105b's core part,
  session 23, no app change), preview URL:
  <https://claude-s23-code-names-driftloom.nowmichaelclark.workers.dev>:
  nothing to hear. Codes version 2's tag 30 is now a name of its own for a
  loop, an album's track or an album, as its UTF-8 bytes (1 to 60) packed
  eight bits a byte into the 64 characters, written only for a renamed loop.
  Thai, other scripts and emoji pass; controls, invisible and direction
  characters, piled-up marks (more than four) and stray joiners or tags are
  refused ("a name a code can't carry"), by `core/src/code/name.rs` on
  `classes.rs` (Unicode 17.0, `tools/name-table.mjs`). `code::request_text`
  (`dl_code_request`) writes a code's request as text, for the app's
  records. Golden codes: a name column, the two titled albums moved, new
  Thai and emoji lines.
- #430, the nightly green again (session 23, v185), preview URL:
  <https://claude-s23-nightly-driftloom.nowmichaelclark.workers.dev>: the
  seed renders' hashes for #414's four twang loops (dith-shuisk, tei-har,
  nong-lu, yon-reil), and the holes on corpus piece #27 (soath-wa, seed
  2094475530), from #402 (bisected in CI): it lost its organ, and three
  floor joins kept a 9-second pad alone, which the join's estimate read
  at its 2 to 4 s loudness. `[longform loudness]` gains a measured 4 to 8
  s span; those three joins become breaths, and no other join of the
  holes corpus's 23,016 moves. Listen: soath-wa as a 1,024-bar arc
  (`bars=1024,journey=arc`) at 38:50 and 45:35, and as an Endless arc at
  9:20: the bass and the pad both breathe for half a beat before the line,
  where the pad was left alone, nearly silent.
- #427, the app speaks share codes version 2, only version 2 (item 105b,
  session 23, v186), preview URL:
  <https://claude-s23-codes-app-driftloom.nowmichaelclark.workers.dev>:
  nothing that plays moved. Copy a loop's code: a plain loop is 9
  characters (was about 80), two vibes 13; paste it back and it plays the
  same loop. Rename a loop in Thai with an emoji and share it: the name
  comes with it. Paste an old `DL1-` code: "That code is from an older
  Driftloom and can't be opened." The Library opens as it was (nothing
  converts; `test/library.test.mjs` holds twenty loops saved by v183 to the
  notes v183 played). A shared code doesn't carry the length slider, so an
  album's pacing no longer travels. The module grew 124 KB (48 KB gzipped):
  the codes.
- #431, the Android app with the screen off, the notification and saving a
  file (item 107d, session 28, v188), the APK from the PR's
  `driftloom-debug-apk` artifact (no preview URL: the web page is unchanged
  but for the app's own calls): the web sounds as it did. In the app: Play,
  Home, switch the screen off: the music plays on, the notification and the
  lock screen show the loop's own cover and name and change with each loop,
  and Previous, Pause or Play, Next work (a headset's too); pulling the
  headphones or a call pauses it, and it returns when the call ends. Save,
  Share, export audio now opens Android's own picker and writes the file as it
  is made; MIDI and the backup and Restore use it too. Details and what to
  send back are in `docs/notes/for-mikey.md`.
- #433, links, an album from one seed and the plain lines (item 105c, v187): a
  link is the address with `#c=` and the code (no server sees it; the cached
  page opens it offline), opening a loop or an album ready, not playing; share
  link (share sheet, else copied); a static preview (title, description, the
  app icon); an album of one seed from Save, Share (12 tracks, 14
  characters); the newer, instrument, older and damaged-link lines. Check the
  preview in Discord and a link on a phone.
- #438, an album's code carries how long each track plays (item 105d,
  session 24b, v189): each track's length slider setting (passes, or bars to
  a phrase line) travels with its album, the one most tracks share written
  once (two characters for twelve tracks of two passes), nothing where every
  track plays on (the code as before); opening it plays each track its
  length, then the next. A loop's own code and an album from one seed carry
  none. `docs/ALBUMS.md`'s six old albums that set lengths have them again.
  Share an album with lengths set and open its link elsewhere.
- #437, the Android app on F-Droid (item 107e, no app change: still v189):
  F-Droid's listing (`fastlane/metadata/android/en-US/`: title, short and
  full description, the changelog for 1.0, icon, feature graphic, six phone
  screenshots taken from the app on an emulator), the launcher icon drawn as
  vectors from the web icon's orbit with a monochrome layer, the build recipe
  as a template filled from `gradle.properties` (`android/fdroid/render.mjs`),
  built from a clean clone by F-Droid's own tooling in CI, the two-builds
  comparison and the licenses and anti-features check (`android/LICENSES.md`,
  `docs/notes/107-reproducible.md`), the release APK on an emulator, the tag's
  release workflow, and Mikey's steps (signing, the submission after 31) in
  `docs/notes/for-mikey.md`. Nothing sounds different. Check the six
  screenshots look like the app, and read the signing choice.
- #440, the layers show what is heard (item 110, session 30b, v190): the
  core reports what each layer sounded at each step (a Heard event: its
  loudness and the share of its level it was heard at, on the web and in
  the Android app), and the Layers view draws that: a note the budget left
  out is not drawn, nor a layer a long piece rests (a muted one shows its
  pattern dimmed, as before), a long piece's quieter layers and a kit
  coming in are drawn dimmer and brighten as they rise. The grid of each
  pass is now as its plan has it heard. Android measures its own output
  latency for the playhead.
  `tools/heard.mjs` measures the drawing against what each channel sounds:
  on every case, drawn-but-not-heard and heard-but-not-drawn went to 0. Watch
  Layers with a long piece on the phone and in the app.
- #442, Previous works in an album (item 109, session 30a, v191), preview URL:
  the branch's preview (no hash kept): in an album the Previous key was off
  wherever the history was at its start (an album's tracks don't enter it);
  it is on now, from the first track round to the last, and from the screen,
  the keyboard, a headset, the lock screen and Android's notification it goes
  to the track before, which plays its own length again; a deleted track is
  gone round the way Previous was going. Outside an album, the history's.
  `tools/rows.mjs --only previous`: 12 rows. Listen for nothing; press
  Previous in an album on the phone and in the Android app. Previous a few
  seconds in still goes to the track before, not the start: for Mikey's ears.
- #448, Track length, its own view, to the second (item 111, session 30c,
  v192), preview URL: the branch's preview (no hash kept): the pace line's
  third key opens Track length (h:mm:ss typed or by a timer's digits, a
  second and a minute either way, a coarse slider to a day, on and on, fade
  in, fade out); the core counts a time on its own clock (a tempo change
  doesn't move it), plays nothing from its second, and starts the next loop
  on it; fade in 3 s up from silence, fade out 8 s to silence on the second
  (notes and a track gain), without it the track stops and rings out. The
  Sound view's Length slider and the export's How much are gone: an export
  is the track length (2:00:07 exports 7207.000 s of track and its tail);
  saved loops convert; an album code carries the time and fades. Listen to
  a 0:45 track with each fade on and off: are 3 s and 8 s right?
