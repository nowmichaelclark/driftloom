# For Mikey

What the long hands run toward 1.0 (2026-11-10 on) left for Mikey: his ears,
his phone, his accounts, or a decision. One line each: what, which PR, what
to listen for or decide. The hands add lines as they go and never wait on
them; an answered line is removed (or moved to `docs/DECISIONS.md` by the
brain). Empty or answered at 1.0 (queue item 108).

## The listening album

Not built yet: it gathers batch 6 (101), 84b's movers and 102's long
pieces, as album codes for the app. The codes and what to listen for go
here.

## Ears

(none yet)

## Phone and tablet

- **The app with the JavaScript gone (item 97, v152):** on the Moto and the
  tablet, play a few loops, Next a few times, a long loop and the screen
  off; Diagnostics should read `late: 0  missed: 0  dropped: 0`. Trouble
  means a revert of that PR.
- **The too-slow line never shows on the tablet (item 100, v153):** play ten
  minutes on the tablet on lite; the card should stay, and Diagnostics
  should read `slow: no`. If "This browser is too slow to play Driftloom
  smoothly." shows there, tell the hands the `slow:` line.
- **The friend's Pixel 9 on GrapheneOS (item 100):** with JIT off for the
  site the line should show within about a minute of playing; send its
  `slow:` and `busiest:` lines either way.

## Accounts

(none yet)

## Decisions

- **A new plain line (item 97):** when the core's module doesn't arrive (no
  connection on a first visit), the card's place says "Driftloom couldn't
  load. Check the connection and reload." Keep, or reword?
- **Where the too-slow line sits (item 100):** it takes the card's place,
  as the can't-play line does, so the loop's name and key are hidden while
  it shows (until a reload). Say if it should sit above the card instead.
