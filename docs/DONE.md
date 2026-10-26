# Done

One line for each PR that merged, newest last: the PR number, the version, the
commit preview URL, and what the ears should listen for. Add yours when your PR
merges (`docs/QUEUE.md`, standing rules). Entries before #241 are in
`docs/archive/done.md`.

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
