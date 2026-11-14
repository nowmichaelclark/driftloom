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
- **widePad (101c, session 04, v166, #387):** the chord layer much wider in
  stereo, on the side only (Mono and a phone's speaker hear exactly what they
  did), in two kinds, half and half: wide from the start, and opening from
  today's width to the wide one over 60 to 120 s. To hear it before the album,
  on headphones: loa-yer (corpus 20261009 #23) from 0:10, flipping Mono to
  hear what it adds; broan-bro (#39) from 0:00 to 1:26 as it opens. Taste
  calls for your ears, each one line of `data/choices.txt` or
  `core/src/space.rs`: 15% of the loops with chords before the vibes (16%
  after, padWow's share); the opening kind takes the choice's vibes (the batch
  gave it no row of its own); on the widest loops the chords go past the
  batch's -4 dB (grove #62 at -1.8), as the sketch you approved did.
- **tripletKit (101d, session 05, v167, #391):** a kit in threes, the
  small hours' shuffle: about 1 loop in 7 with a kit (none in 6/8) plays its
  hats three a beat, the eighth moved to two thirds of the beat, the kick on
  the beat. To hear it before the album: dun-roar (corpus 20261009 #45) from
  0:00, a busy machine kit at 129; thisk-doam (#76) from 0:00, a slow tape kit
  at 55. Taste calls for your ears: 15% of the loops that may have it before
  the vibes (swaying and skipping lean toward it, still and glacial away);
  the hats written at the loop's own density and velocities, then the hats off
  the beat turned down so the kit is no brighter than it was; a kick off the
  beat goes to the nearer beat (a third of dust #69's would otherwise have
  merged); and in a long piece the fill into a line still plays four
  sixteenths over the last beat, not thirds. Each is a line of
  `data/choices.txt` or `core/src/generator/kit.rs`.
- **dottedPulse (101d, session 05, v168, #392):** a lilt against the bar,
  in two versions, half and half, on about 1 loop in 7 with a 4/4 kit (never
  beside a kit in threes): `pulse`, a soft rim every three eighths, and
  `kick332`, the kick on a 3-3-2 written whole. To hear them before the
  album: the pulse on ta-muin (corpus 20261009 #121) from 0:16, as the kit
  comes in; the 3-3-2 on yi-yoar (#51) from 0:00 and la-glo (#73) from 0:00.
  Taste calls for your ears: the pulse's level, rim velocity 0.5 (the sketch's
  0.2 was too quiet to hear as a lilt; 0.5 sits where the kit's ghost rims
  do, 31 to 40 LU under the loop, a little under its off-beat hats), one
  number in `data/choices.txt`; the 3-3-2 keeps each bar's kick within 1 dB
  of the loop's own there (the downbeat as it was, the other two softer where
  needed) and plays only where the loop's own kick or bass plays, so it never
  fills a gap's silence; 15% of the loops that may have it before the vibes.
  The holes found while measuring it (ta-muin and yi-yoar as Endless arcs)
  are fixed in 102g, below.
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
- Batch 6's and 84b's: brain session B2. B4 makes the albums again in the new codes.

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

## Accounts

- **The new name's domain (2026-11-11):** when you're ready, buy it; the
  brain's advice is in its reply of 2026-11-11 (Cloudflare Registrar, kept on
  the same Cloudflare Workers hosting). Then confirm the Android release id
  (the brain proposes the reverse of the domain plus `.app`). Session 31a
  needs both; nothing before it does.

## Decisions

(none open: the long pieces' first picks were all kept, 2026-11-14, `docs/DECISIONS.md`)
