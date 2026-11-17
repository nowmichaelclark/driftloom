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

- **The first Android APK (item 107a, session 25, #389):** the shell only: the
  page in a WebView, playing as in a browser (the native audio is session 26,
  so a stutter there is no news). To try it on the Moto: open the PR's checks
  on GitHub (signed in), the `android` run, and under Artifacts download
  `driftloom-debug-apk` (a zip; the Files app opens it), then tap the `.apk`
  (Android asks once to let that app install apps). Each later build installs
  over this one (they share a throwaway test key, `android/test-signing/`),
  so your loops stay. Check: it starts, Play plays, rotating the phone keeps
  the music going; Diagnostics' `sw: failed` and `mediaSession: false` are
  expected for now (sessions 27 and 28). A launcher icon is a placeholder.
- **The core native on Android's own audio (item 107b, session 26, #411):**
  no page yet, a debug screen. Get the APK as for 107a, from #411's `android`
  run: `driftloom-debug-apk` holds `driftloom-universal-debug.apk` (fits both
  devices; it installs over 107a's). A second icon appears, **Native audio
  test**. Open it, headphones on, press **Play the test**, and let it play
  five minutes (the screen stays on). Send back the two numbers at the top
  for each device: **Underruns** (should be 0) and **Busiest block** (its %
  and ms). Also listen: the golden loop (reisk-waith) should sound as on the
  site, with no clicks. Optional: **Check the samples** should say "same
  samples as the web's"; if it says DIFFERS, send that line. Next and Pause
  work too; the stream's line at the bottom says whether the phone gave the
  low-latency path (performance mode 12) and a real-time thread (SCHED_FIFO).

## Accounts

- **The new name's domain (2026-11-11):** when you're ready, buy it; the
  brain's advice is in its reply of 2026-11-11 (Cloudflare Registrar, kept on
  the same Cloudflare Workers hosting). Then confirm the Android release id
  (the brain proposes the reverse of the domain plus `.app`). Session 31a
  needs both; nothing before it does.

## Decisions

- **A shared loop's name (codes, version 2, session 22, #412):** the new
  codes carry what a loop sounds like, not the name you gave it, nor the
  length slider's setting (version 1's codes carried both). So a friend who
  opens your code sees its own generated name. Say if you want a renamed
  loop's name to travel with its code; a field can still be added before
  1.0 (the brain's B4 looks at the format closely before it freezes).

(The long pieces' first picks were all kept, 2026-11-14, `docs/DECISIONS.md`.
How many changes a loop gets, and twang's picks, were decided 2026-11-17 and
built in v175: see twang's entry above.)
