# For Mikey

What the long hands run toward 1.0 (2026-11-10 on) left for Mikey: his ears,
his phone, his accounts, or a decision. One line each: what, which PR, what
to listen for or decide. The hands add lines as they go and never wait on
them; an answered line is removed (or moved to `docs/DECISIONS.md` by the
brain). Empty or answered at 1.0 (queue item 108).

## The long pieces: the listening set (102e, session 15)

**31 clips, 23 minutes in all**, on the throwaway branch `long-clips`
(stereo Opus, its `index.md` has every clip's piece, place and one line on
what to listen for). Brain B3 builds the page from it. The pairs are named
before and after there, and B3's page makes them blind. Made by
`tools/longform-clips.mjs` from `main` at v164 (#381); each excerpt's place
is read from the piece's map, so the set can be made again after session 16.

**How they were chosen:** the pieces sessions 12 to 14 wrote into
`docs/LONGFORM.md`'s "Pieces to hear", one clear example of each thing
built, picked by what their maps show, not by how they sound. Beside them, from the corpus:
- an Endless arc (brum-yei: the stall test's piece);
- a piece whose journey a vibe chose (loa-yer with adventurous, teth-hu
  with still), with no journey asked;
- one piece likely to sound weakest: gluith-vuisk, where batch 6's
  slowOpening and the arc's opening hold the kit back together.

Lengths: 64, 256 and 1,024 bars and Endless; six styles; arcs and wanders.

| Piece (seed, request) | Clips: what happens |
|---|---|
| tha-vol, `2728346932 bars=64,journey=arc` | **whole** (1:34): the small form end to end, every join kind of an arc but the pivot, the landing |
| brum-yei, `1151594917 bars=256,journey=arc` | the start; B's entry over a pivot chord (1:10); the peak and the floor out after it (2:38); the return prepared on the home fifth (3:13); the breath between its two movements; the close and the landing |
| fei-no, `2158762586 bars=256,journey=arc` | B in the relative key (2:43); A home stretched (7:05); **a close the length slider made** at bar 120 (4:05, `track bars=120`) |
| moan-teil, `2060199680 bars=1024,journey=arc` | the interlude's one move; the finale looking back (29:17); the close after 34 minutes |
| loa-yer, `2975389383 bars=1024,adventurous=0.8` | an arc the vibe chose: the start; the first peak (7:23) |
| gluith-vuisk, `1068550449 bars=64,journey=arc` | its first 90 s: the kit out for 74 s, the bass for 100 s: too long thin? |
| glor-ti, `7 bars=endless,journey=wander` | Endless's first twenty minutes as its first five moves (3:20 to 19:13), each alone; **a close the slider made** at bar 100 (2:59) |
| mu-meth, `1086007177 bars=endless,journey=wander` | the wander's rare key move over a pivot chord (11:49) |
| teth-hu, `706141505 bars=endless,still=0.8` | a wander the vibe chose, at 40 bpm: the start; its first move |
| brum-yei, `1151594917 bars=endless,journey=arc` | Endless's first arc ending in a breath, not a close; the gentle movement after it |
| **two blind pairs** | brum-yei 256 bars from 0:55, and glor-ti Endless from 5:20, 45 s each: before item 102 (`dfe78c3`, the loop's wandered sections) and now, level-matched to the quieter (within 0.6 dB): **is composed better than before?** |

**What only your ears can judge** (I could measure the rest; the measures
are in #381 and the second PR): whether each join is welcome or too much; whether
a build is felt; whether B belongs and A's return is known at once; whether
the breath, the slider's close and the landing feel like endings; whether
the wander's moves are slow enough and still heard; and the pairs.

**After sessions 17, 18 and item 105** the app will play these whole: the
seeds and requests above are the pieces (the journey and the vibe in the
request; the slider's close is the length slider's `track`).

## What to start next

The rest of the road is cut into sessions, each with a prompt to paste:
`docs/sessions/README.md` has the order, the model and effort for each, and
the points where the next step is yours.

## The listening album

- **Phase 3 Control (brain B1; fixed at v155):** six loops, one album code,
  in the app before phase 3 (**Before**) and today's (**Now**, the live
  site). Code and links in `docs/ALBUMS.md`. On the Moto (it stays at full),
  headphones, the same volume: each loop about 45 s in Before, then in Now;
  one word a loop. The brain's renders now match on all 30 loops it tried
  (-82 dB or lower). Two loops, tui-doask (track 1, from about 0:31) and
  soath-wa (track 4, from about 0:27), used to come back narrow in stereo
  for about ten seconds after a long rest; v155 fixed that, so there Now
  may sound the same as Before or wider after the rest.
- **Batch 6's far ends (101a, session 02, v156):** deepDuck, wideSweep (wide
  and fast) and slowOpening now play in the app, each an option of its
  approved choice. To hear them before the album: deepDuck on loa-yer (corpus
  20261009 #23) and toam-woar (#67), any time after 0:20; wide on brai-fei
  (#46) and sol-sham (#77) from 0:15; fast on li-roun (#96) and broa-glou
  (#292) from 0:10; slowOpening on rong-wuisk (#6) from 0:45 (the kit at 0:49,
  the bass at 0:53, full by about 1:01) and gluith-vuisk (#0) from 1:10.
  Three taste calls for your ears (the PR's "For Mikey's ears"): each new
  option weighs the same as the approved one, so about 1 loop in 12 now
  opens with 40 to 90 s of no kit or bass; the calm vibes lean them away
  (x0.3 to x0.4) rather than ruling them out; and deepDuck keeps the
  sketch's line (about 12 dB down at 50 ms, 10 at 100 ms), not 12 to 16 dB
  held to 150 ms. Each is one line of `data/choices.txt` to change.
- **Batch 6's pitch (101b, session 03, v157 and v159):** tuningOffset and
  padWow now play in the app. To hear them before the album: tuningOffset on
  wen-glui (corpus 20261009 #50, 32.5 cents flat) and kask-hoan (#43, 28.3
  sharp) from 0:00; padWow on soath-wa (#27) from 0:00 and sheith-sith
  (#47) from 0:14. Taste calls for your ears (the PRs' "For Mikey's ears"),
  each one line of `data/choices.txt`: tuningOffset goes to 15% of loops
  before the vibes (about 17% after; a quarter of the hazy ones), 15 to 35
  cents either way, not the batch's 40; padWow to 15% of the loops with
  chords (about 16%), 12 to 14.5 cents at 0.4 to 0.6 Hz around the sketch's
  14 at 0.5 (the records' median is nearer 20). On a few loops where the
  chords are doubled, padWow makes the mix 0.2 to 0.35 LU quieter or
  louder, as the sketch did.
- **The wander heard (102b, session 12, v161):** nothing to do now (the app
  can't take a journey or jump yet; the excerpts are in the listening set
  above). The
  three wander pieces and the minutes where each thing moves are in
  `docs/LONGFORM.md`, "Pieces to hear". What only your ears can judge: is
  each move slow enough and still heard (a 24-s glide; the tone moves at
  most a quarter of the Sound panel), is the octave or voice change at a
  line ever too sudden, does a layer's rest feel like a breath, and is a
  key change a fourth away welcome once in a while.
- **The arc's themes and movements (102d, session 14, v163):** merged as it
  is (2026-11-13); since session 15 the arc's brightness is judged on its
  opening, its peak and its close (`docs/notes/102d-arc-brightness.md`
  says why). Five arc pieces with the
  minutes where the second theme enters, where the development is, where
  theme A returns and which variation each return uses are in
  `docs/LONGFORM.md`, "Pieces to hear" (the fourth, moan-teil, 34 minutes,
  is five movements of four kinds with a finale that looks back). What
  only your ears can judge, each a line of `data/longform.txt`: does theme
  B sound like it belongs to the piece (it is another seed's tune in this
  loop's voice and chords); is a return known at once and still new (an
  octave away, another voice, stretched, over the relative key's chords,
  answered); does the bass holding one note for
  a phrase before the return feel like coming home; are a gentle
  movement, a light one and an interlude different enough from a full arc,
  and their lengths right (0.7, 0.4 and 0.5 of it). **Two choices made for
  you to overrule:** about 1 second theme in 12 comes from a loop whose
  tune rests, and is never given a tune (85f): that movement keeps theme A
  in the new key instead (played as a silence of the tune it left a hole
  of 4 LU); and the parallel key (a minor loop's major) stays, though the
  tune sits a semitone over a chord tone a little more there (4.5% of its
  time against about 4% for the loop itself). **And one thing the measures
  show that ears must judge:** a unit that plays other notes than the loop
  (a fragment with half of each phrase resting, another key) can sit up to
  2.9 LU from its neighbor, where the energy alone steps under 2.
- **The arc heard (102c, session 13, v162):** nothing to do now either
  (the excerpts are in the listening set above). Three arc pieces with the minutes of
  their opening, peak, release and close are in `docs/LONGFORM.md`, "Pieces
  to hear". Listen for: the build noticed only once it has passed (the
  loudness rises 1 to 2 LU by ear, the filter opens, the hats and the
  backbeat come in, the tune is doubled an octave up toward the peak, softly);
  the close thinning to the chords and the tune; the landing, the tonic
  chord and the tune on the tonic alone, ringing out, then the opening
  again. What only your ears can judge: is the arc strong enough to be felt
  at all (the conservative end, on purpose), is the opening's darker filter
  welcome or muffled, and is the landing a real ending.
- **The joins and endings that land (102e, session 15, v164):** where two
  units meet and something changes, the line is written: a chord both keys
  share held across a key's line (the bass on its root), a fill of the
  kit's backbeat into a rise, the floor (kit and bass) out before the fall
  from the peak, the tune alone where a theme changes, or a breath; a
  wander's voice, register or length move gives
  only that layer a breath. A breath between arcs now thins the tune across it
  instead of resting it. And when the length slider will stop a
  long piece or Endless before its close, the last phrases thin to a close
  and land on the tonic chord, ringing out, where a fade was. The
  excerpts are in the listening set above. What only your ears can judge:
  is each kind welcome, and how often (pivots come at every key change, so
  through a development every 30 s or so); is a fill gentle enough (four
  soft hits of the loop's own snare or rim, 0.35 to 0.65 of its loudest);
  is the floor out or the tune alone too much of a hole, or too little to
  notice (each is a bar at most, and at most a sixteenth of its unit: a
  beat in a four-bar phrase); and does
  a slider-made close feel like an ending or like being hurried out.
- **slowOpening and the arc's opening together (102e, for your ears):** in
  gluith-vuisk (corpus #0, seed 1068550449) the loop's own slow opening
  keeps the kit out for 74 s (full by 81 s) and the bass for 100 s, while
  the arc's opening ends at 15 to 30 s: so as an arc its first minute and
  a quarter is the chords and the tune alone, through the opening and the
  theme, and the kit arrives just before the lift (at 64 bars, 89 s). In
  rong-wuisk (#6, seed 3372926893) the kit comes at 49 s (full by 60 s),
  four seconds before the 64-bar lift. Whether that start runs too long
  thin is yours; one way out is for an arc to shorten slowOpening's wait
  (a line of `data/choices.txt` or the plan), not done.
- Batch 6's and 84b's: brain session B2. The long pieces': brain session
  B3. B4 makes them again in the new codes.

## Ears

(none yet)

## Phone and tablet

- **The testers' steps (item 98, session 01):** `docs/notes/98-testers.md` has
  one paste for your Moto, one for your tablet and one for the friends'
  Pixels on GrapheneOS (JIT allowed, then blocked). Each ends with the five
  Diagnostics lines to copy back: build, cores, busiest, slow, engine. The
  stall test on the shipped app is clean (the PR's table); the phones are the
  rest of promise 1.
- **The app with the JavaScript gone (item 97, v152):** on the Moto and the
  tablet, play a few loops, Next a few times, a long loop and the screen
  off; Diagnostics should read `late: 0  missed: 0  dropped: 0`. Trouble
  means a revert of that PR.
- **The too-slow line never shows on the tablet (item 100, v153):** play ten
  minutes on the tablet on lite; the card should stay, and Diagnostics
  should read `slow: no`. If "This browser is too slow." shows there, tell the hands the `slow:` line.
- **The friend's Pixel 9 on GrapheneOS (item 100):** with JIT off for the
  site the line should show within about a minute of playing; send its
  `slow:` and `busiest:` lines either way.

## Accounts

- **The new name's domain (2026-11-11):** when you're ready, buy it; the
  brain's advice is in its reply of 2026-11-11 (Cloudflare Registrar, kept on
  the same Cloudflare Workers hosting). Then confirm the Android release id
  (the brain proposes the reverse of the domain plus `.app`). Session 31a
  needs both; nothing before it does.

## Decisions

- **The arc's brightness (102c, #377; you merged it as it is, 2026-11-12,
  so this is now a listening question, not a decision): brightness rises and falls on
  87% of pieces (9 in 10 asked), and one 1,024-bar piece steps 2.7 LU (2
  asked). Four loops miss: tui-doask and so-rou (nothing above about 1 kHz
  to brighten), gluith-vuisk and rong-wuisk (their own slow opening hides
  the arc's). `docs/notes/102c-arc-brightness.md` says why and what could
  be tried. When you hear them, say whether they rise and fall to you.
  Since session 15 the yardstick is the arc's own job, darker at the
  opening and the close than at the peak: 91% of 47 pieces (the misses the
  same loops); over every role 77%, reported.

**Long pieces: the plan (102a, session 11, v160).** Nothing sounds
different yet; a piece can be read before it is heard (`docs/LONGFORM.md`
says how; the PR has maps). Each line is a first pick, one line of
`data/longform.txt` to change, for your ears once sessions 12 to 15 play
them. Settled for now:

- Which journey: arc and wander even; Endless leans to the wander (about
  37% arcs); your vibes lean it (still, glacial, meditative to the wander;
  adventurous, lively, soaring to the arc); `journey=arc` or
  `journey=wander` pins it.
- An arc lasts 4 to 15 minutes; a long piece holds several, as movements,
  and only the last closes (the others end in a breath).
- A piece under about 4 minutes (most 64-bar ones) plays a small arc: one
  theme, a lift, a short release, the return, a close.
- A second theme in 7 arc pieces of 10; the rest stay with one tune.
- The peak falls 60% to 72% of the way through an arc.
- Keys an arc may visit: a fifth up or down, the relative, and the parallel
  only to brighten a minor loop; never a very dark mode; always home for
  the return and the close.
- The wander: a hold of 1 to 5 minutes (about 3 on average), then exactly
  one thing moves; a layer that rests comes back a hold or two later.
- The wander may visit a related key (15% of its half-hour epochs) and hand
  the tune to a second theme (20%); set either to 0 to turn it off.
- The tempo is held for the whole piece.

**The wander heard (102b, session 12, v161), first picks** (each one line
of `data/longform.txt`, `[longform wander]`):

- Everything a piece moves goes out from the loop's own over about 25
  minutes, one thing a hold, and comes home over the next 25.
- The warmth and the space swing to the other side of the loop's own each
  time, at least 0.4 of the way, at most a quarter of the Sound panel
  (`apart`, `reach`); the tune thins to 55% to 82% of its notes and comes
  back (`density`).
- A glide (a layer resting or coming back, the tone, the tune thinning)
  takes 24 s (`glide`); a voice, the chords' octave or length, or a key
  change on a phrase or section line.
- The chords' lengths: 0.6 or 1.5 of their own (`lengths`).
- A wander visits only a key a fifth away (the whole loop moved), never the
  relative or the parallel (those change the chords: the arc's).
- The wander's theme turn is off until session 14 makes theme B (`theme =
  0`; it was 0.2).
- A voice moved can let the voice budget play a layer it held back (seen
  twice in eight pieces): the chords coming in when the tune's voice gets
  lighter. Yours to judge.
- A finite piece played round again plays the same plan again.

Left open, for later sessions or for you:

- A slower middle movement, or easing into the close: not built; a sketch
  to offer.
- How strong a build sounds (session 13) and how each changed return
  sounds (session 14).
- Moving the tempo slider mid-piece stretches the plan's spans; it does not
  re-plan.

**The arc heard (102c, session 13, v162), first picks** (each one line of
`data/longform.txt`, `[longform energy]`):

- The loop as written is the arc's fullest, near its peak; the rest of the
  arc holds it back a little. The kit and the bass sit at 0.85 of their
  level at an opening (`level.*`), the chords and the tune at 0.85 at the
  lowest (`swell`): the peak ends up about 1 to 2 LU over the opening.
- The kit's hats and shakers come in with the energy, and the backbeat
  from half its velocity (`kit`, `shimmer`, `backbeat`, `beat`): the
  brief's "kick out" made openings brighter and far quieter, so it was
  turned around.
- The tune thins under the theme's energy (`thin`); the chords sound longer
  in an opening and a close (`held`); the tune is doubled an octave up from
  the build, at most 0.3 of its velocity (`double`).
- Brightness is the loop's own slow filter, aimed: up to 0.6 of its range
  darker in an opening and a close, its swing half as deep at the peak
  (`aim`, `dark`, `open`); the space opens with it, a little (`space`).
- A finite arc lands on its last 4 s and a bar at least (`land`): the
  tonic chord and the tune on the tonic, alone, ringing out.


**The joins (102e, session 15, v164), first picks** (each one line of
`data/longform.txt`, `[longform joins]`):

- A fill where the energy rises by 0.04 or more across a line where a theme
  changes, 0.08 where only a role does (`step`); the floor out where it
  falls as far from the peak or the lift.
- A join lasts a bar, at most a sixteenth of its unit (`bars`, `share`); a
  breath half of that (`breath`); a fill the last beat, rising from 0.35
  to 0.65 of the instrument's loudest (`fill`); a pivot's chord held a beat
  past the line (`hold`).
- Chosen in the code, not the data (`Form::join_of`): a key's line a pivot
  (else the floor out, or a breath); a theme's a fill rising, the floor out
  falling from the height, else the tune alone (a breath where the tune is
  silent); every
  wander's theme a breath of the tune; the return prepared where it is.
- A slider-made close lasts 10 s in whole phrases (`[longform arc] close`)
  and lands as a finite arc does (`[longform energy] land`).
