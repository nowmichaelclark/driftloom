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

## The Android app, screen off (item 107d, v188, PR #431)

Install the universal APK from the PR's `driftloom-debug-apk` artifact over
the last one (same key). On the **Moto** and on the **tablet**:

- **Ten minutes with the screen off.** Play a loop, set Track length to one
  minute (Sounds), press Home, switch the screen off, put the phone down for
  ten minutes. Listen for: no gap, no crackle, the loops changing on the
  minute; the notification and the lock screen show the loop's **own cover**
  and its name, changing with each loop. Then in Diagnostics (screen on):
  `engine:` with `late: 0 missed: 0 dropped: 0` and its underruns, and
  `while hidden:`; send me those two lines and the phone's name.
- **The controls.** From the lock screen and the notification: Pause, Play,
  Next, Previous; from a headset's or Bluetooth's buttons too. Pull the
  headphones (or switch the Bluetooth off) while it plays: it should pause.
  Call the phone (or play a video in another app): it pauses, and returns when
  the other sound ends. A short sound from another app (a message tone) should
  only lower it for a moment.
- **A long piece and an export with the screen off.** Start a long piece
  (a Journey, a few minutes) and an export (Save, Share, export audio: pick a
  place in the system's picker, a 30-minute length, **lossless**), then switch
  the screen off. Both should finish: the export says "Saved where you chose"
  and the file plays. Try **small** too where the choice shows, and a MIDI
  export and a backup (they use the same picker now), and "Restore" (it opens
  Android's file picker).
- **The notification question** (Android 13 and later): the first Play asks
  whether Driftloom may show notifications. Say no once: the music and the
  lock screen's controls should work all the same (the notification itself is
  what is missing).
- **Battery.** Nothing runs when it is paused (no wake lock, no stream). While
  playing with the screen off the app costs what a music player does: one
  audio thread and one wake lock. The emulator's figures are in the PR and
  say only that; a phone's battery is its own: play for an hour with the
  screen off and send me what Settings, Battery says Driftloom used.
- Not tested without a device (the emulator can't say): that another app's short
  sound ducks the music (the system does it for this stream; tell me if it
  doesn't), and Bluetooth buttons.
## Links, an album from one seed, the plain lines (item 105c, v187)

- **Approve the link's shape.** A link is the site's address with the code
  after `#c=`: `https://driftloom.nowmichaelclark.workers.dev/#c=GRiSNBAxx`.
  The fragment never reaches a server (so no code in its logs or a
  referrer), and the page's own request has none, so the installed app's
  cached page opens a link offline (a `?c=` query would be a different
  request, not in the cache). It opens the loop (in Imported Loops) or the
  album (in the Library, open at its first track) **ready, not playing**
  (a browser won't let a page make sound before a tap). Save, Share has
  **share link** (the phone's share sheet where there is one, else copied)
  and an album's page has **link**. A pasted link works as its code.
- **Approve the preview.** Discord, Messages and Slack show one preview for
  every link: the title "Driftloom", the description, and the app icon
  (512 px, `icons/icon-512.png`) as a small card. A Painter may draw a
  wider picture. Paste the site's address into Discord to see it (it shows
  after this deploys; Discord caches a preview for a while).
- **Album from one seed:** Save, Share, "Album from one seed": a Tracks
  slider (2 to 48), each track asked the picks now set; 12 tracks is a
  14-character code. Its album has no name (it opens as "Untitled").
- **The plain lines** (a toast each): "That code needs a newer Driftloom.",
  "That code needs an instrument this device doesn't have." (wired to the
  core's refusal; it can't show yet, since the core says "needs a newer
  version" for any id it lacks, and B4 settles playing something close
  instead), the older-code line, and "That link's code can't be read."
  The name comes from the page's one constant. At the rename (31a) change
  `APP_NAME`/`APP_URL` in `js/main.js`, index.html's preview tags and the
  manifest together: `test/preview.test.mjs` fails if they disagree.

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
  site). Links in `docs/ALBUMS.md`; since v184 (session 23) two codes of the
  same six loops: the version 2 one there for **Now**, the version 1 one in
  `docs/archive/notes/98-phase3-control-v1.md` for **Before**. On the Moto (it stays at full),
  headphones, the same volume: each loop about 45 s in Before, then in Now;
  one word a loop. The brain's renders now match on all 30 loops it tried
  (-82 dB or lower). Two loops, tui-doask (track 1, from about 0:31) and
  soath-wa (track 4, from about 0:27), used to come back narrow in stereo
  for about ten seconds after a long rest; v155 fixed that, so there Now
  may sound the same as Before or wider after the rest.
- **No join a hole on soath-wa (v185, #430, session 23):** corpus piece
  #27 (seed 2094475530) as a 1,024-bar arc, at about 38:50 and 45:35, and
  as an Endless arc at about 9:20: where the bass dropped out before the
  line and left a fading pad alone (28 dB under the music), the bass and
  the pad now breathe together for half a beat. Nothing else moved.
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

- **Codes, version 2 (item 105b, session 23, #427, v186):** on the phone,
  rename a loop in Thai with an emoji (the box takes it; an invisible
  character it won't), copy its code (Save, Share, copy its code) and paste
  it in the tablet: the same loop, the name with it. Old codes from chats
  (DL1-, DLA1-) now get "from an older Driftloom" and don't open; your
  Library is untouched (and any album in it shares with a new short code).
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

- **The new name (2026-11-18: the domain is bought, on Cloudflare):** attach
  it to the Worker when you're ready (Workers, driftloom, Settings, Domains),
  and confirm the Android release id (the brain proposes the reverse of the
  domain plus `.app`). Session 31a needs both.

## Decisions

- **The journey's word "arc" (item 106b, #419):** the other words are
  approved (2026-11-18); you weren't sure how a listener reads "arc". Pick
  a word, or keep it.

(A shared loop's name, the export sheet's words and the export's ending
were decided 2026-11-17, `docs/DECISIONS.md`, "Share codes, requests and
exports". The long pieces' first picks were all kept, 2026-11-14, `docs/DECISIONS.md`.
How many changes a loop gets, and twang's picks, were decided 2026-11-17 and
built in v175: see twang's entry above.)
