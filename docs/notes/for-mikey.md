# For Mikey

What the long hands run toward 1.0 (2026-11-10 on) left for Mikey: his ears,
his phone, his accounts, or a decision. One line each: what, which PR, what
to listen for or decide. The hands add lines as they go and never wait on
them; an answered line is removed (or moved to `docs/DECISIONS.md` by the
brain). Empty or answered at 1.0 (queue item 108).

## What to start next

The rest of the road is cut into sessions, each with a prompt to paste:
`docs/sessions/README.md` has the order, the model and effort for each, and
the points where the next step is yours.

## Small audio export (item 104c, v184, PR #424)

- **Listen to a Small export.** Save, Share, Audio: pick **small**, a few
  minutes of a loop you know, and play the file (`.opus`) in the player you
  use, on your phone and in VLC. Compare with the lossless file of the same
  loop. It is Opus at a constant 96 kbit/s stereo (64 mono): meant to be the
  same to your ears; if you hear a difference (a soft top end, a shimmer in
  the reverb), say where, and the rate goes up (for Mikey's ears). Small
  shows only where the browser can encode Opus: tested in Chromium and
  Firefox on desktop; not tested on a phone or in Safari, so say whether the
  choice shows on the Moto, the tablet and an iPhone (where it doesn't,
  Lossless is the only one, by design).

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
- **The hiss (v176, #403):** the hiss you heard behind every loop was the
  page's hidden keepalive (the file that keeps the lock-screen controls), not
  the music: noise at -62 dBFS. It is now a 20 Hz tone at the same power, far
  under hearing. Check on your phone, a quiet loop on headphones: no hiss,
  and the lock-screen and Bluetooth controls still come up. If the controls
  are gone, say so: the old file is one revert away.
- **No join a hole, on any piece (102g, session 16b, v169, PR #395):** six
  clips, 2:30, on the branch `long-clips` (its `index.md`, which replaces
  session 16's set; that one is in the branch's history), not blind: each
  place now and as at v168, as Endless arcs (`bars=endless,journey=arc`).
  ta-muin (seed 1337654778) from 1:50, the line into 2:05, and from 9:11,
  the line into 9:26: at v168 the tune alone for the last bar, a stab's one
  note fading, the mix 19 to 38 dB under the loop for about two seconds;
  now nothing is written there and the loop plays on across the line.
  yi-yoar (seed 819045617) from 5:59, the line into 6:14: at v168 the kit and
  the bass out for half a bar, 29 to 38 dB under; now nothing written there
  either. Listen for: no gap at those lines, and
  nothing else different (every other join is as you kept it but where it
  would have been a hole: 704 of about 30,000 written joins over the
  corpus's maps, most of them breaths that become glides; the PR has the
  count by kind). Taste call for your ears: a hole falls back to a breath where that
  would not be one, else to nothing written, never to a new kind of join.
- **The long pieces, what changed (102f, session 16, v165):** six clips, 2:40,
  once on the branch `long-clips` (replaced by 102g's set; they are in the
  branch's history, commit cf3576c), not blind: each place now and
  as at v164. fei-no (seed 2158762586, `bars=256,journey=arc`) from 5:15, the
  key's line into the peak at 5:27: a chord now sounds from 5:25 where v164
  had a bar near silence; and from 7:55, the stretched return into the close
  at 8:10: a half-bar breath with the chords and bass in it where v164 had
  three seconds near silence. loa-yer (2975389383, `bars=1024,adventurous=0.8`)
  from 3:44, the key's line into theme B at 3:56: the chord both keys share
  now struck at 3:54 (in v164 it was struck at nothing, under the kit and the
  bass). Nothing else moved: every number, join and choice is as you kept it.
  One more quiet found while measuring, not a join's and not changed (it
  is the variation you kept, since v163): vath-sei (3104090029,
  `bars=64,journey=arc`) at 2:45, theme A home in another voice that dies away
  in about a tenth of a second where the loop's own voice holds the note, so
  0.65 s sits near silence in a two-bar loop where the tune plays alone. If
  it bothers you, tell the brain; it is a variation's choice to queue, not a
  number. Brain B3 (checked v165): fei-no's stretched return has the same
  kind of quiet just before its breath, 8:07 to 8:09, about 2 s some 30 dB
  under the loop, unchanged since v164 (its held chord dies away under a
  tune at half speed); you'll hear it in the second fei-no clip. And the
  chord at 5:25 now sounds but dies away within a bar (that voice's own
  decay), so the last second before the peak is still thin. Both are the
  variations' and the voice's, not the joins'; say if either bothers you.
- B4 makes the open albums again in the new codes (version 2).
- **The FLAC writer (104a, session 19, #390):** nothing to hear yet; the
  app gets export in session 20. One choice for your ears then: the core
  writes 16 or 24 bits with **no dither** (the conservative pick: the file
  nulls against the render, and at 24 bits it can't matter). If session 20's
  export offers 16 bits and a very quiet fade-out ever sounds grainy on
  headphones, say so and a seeded dither is a small change.

- **twang soft only (v179, #414, session 17):** as you asked on the Batch 6
  Album, every loop that played twang plays the soft one, and the same loops
  play it. he-rosk (corpus #33, was bright) and tom-wen (#172, was slow) are
  the album's two that changed: the same picks, a beat apart, in the soft
  voice. If either still sounds off, say which.

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
- **Audio export (item 104b, session 20, #416):** on the Moto and the
  tablet, in the browser: play a loop, then Save, the Share tab, Audio.
  Pick 30 min and **export audio**; let the music play on, turn the screen
  off for a few minutes, come back. It should finish while the music never
  stutters, and download a `.flac` (the Files app plays it; so does VLC).
  Then once with "lighter on processor" off. Send back, for each device:
  how long the export took (the progress line says), whether anything
  stuttered, and Diagnostics' `engine:` line (should read `late: 0  missed:
  0  dropped: 0`). Optional: Cancel one halfway; nothing should be saved.
- **Long pieces on screen (item 106b, session 18, #419, v183):** on the phone
  (and the tablet if you can), in the browser. Vibe, then the bars line, then
  set 3,072 bars; **arc** or **wander** is a key between the ∞ and any (it
  dims under 33 bars). Done, Next, Play: Now Playing has the piece's line
  where the rail was, with bar 812 of 3,072, the time played and left (for
  Endless, played only), and a key each way. Press the line anywhere (drag to
  see where, let go to jump) and press the keys. A jump should come in on a
  phrase line within a blink, the old place fading under it, never waiting
  for its opening or opening its width again; a long chord that was ringing
  before the jump isn't there after. Look too at the Library (an album's
  rows end in each loop's length: "4 bars · 7s", "3,072 bars · 2h 48m",
  "endless"), and the Bars picker at your phone's height: nothing should
  scroll. Screenshots at 360 px and a tablet's width, dark and light, are on
  the branch `s18-shots` (linked in the PR).

## Accounts

- **The new name's domain (2026-11-11):** when you're ready, buy it; the
  brain's advice is in its reply of 2026-11-11 (Cloudflare Registrar, kept on
  the same Cloudflare Workers hosting). Then confirm the Android release id
  (the brain proposes the reverse of the domain plus `.app`). Session 31a
  needs both; nothing before it does.

## Decisions

- **The long pieces' words (item 106b, session 18, #419):** approve or change
  them (`ROLE_WORDS` in `js/piece.js`; the line's second row is "1:12:03
  played · 2:10:00 left", Endless "1:12:03 played"). What the playhead is in:
  the opening, the theme, building, a second theme, developing, the peak,
  easing, the return, the close, a breath, a lift, holding, turning. The
  journey keys read "arc" and "wander"; the pills beside the bars say the
  same. A piece past a day shows hours ("32,768 bars · 30h") in the Library.
- **A journey is not in a share code yet (#419):** the pick is kept with a
  loop in the Library, but a version-1 code (what the app still writes) has
  no room for it; codes version 2 carry it (`core/src/code.rs`), so session 23
  writes `askJourney` (the spec's field) into them. Until then a pasted code
  of a long piece draws the journey its seed and vibes lean to.

(A shared loop's name, the export sheet's words and the export's ending
were decided 2026-11-17, `docs/DECISIONS.md`, "Share codes, requests and
exports". The long pieces' first picks were all kept, 2026-11-14, `docs/DECISIONS.md`.
How many changes a loop gets, and twang's picks, were decided 2026-11-17 and
built in v175: see twang's entry above.)
