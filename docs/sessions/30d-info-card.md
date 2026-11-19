# Session 30d: the info card: clean lines, the rest a tap away, a round playhead (112)

## Left (2026-11-18: stopped partway at Mikey's usage limit; start from here)

Branch `claude/project-thread-g37m5i`, draft PR. Built and checked in the
branch, **not yet merged, BUILD/CACHE not yet bumped** (main was v190; 30a's
#442 takes v191; take the next number after main's when you finish).

**Done**
- `tools/card.mjs`: a check of the card at 320 / 360 / 412 / tablet /
  landscape phone / landscape tablet, text 100 / 130 / 200%, for loops, a
  long piece, Endless, long, Thai and emoji names, an album on the card and
  playing (`--only <case>`, `--shots <dir>`, `--verbose`). Before the work it
  failed (lines cut, over the card's edge, the name cut at 200%). It also
  walks the taps: every page whole, the pages together every fact, the last
  back to the first, and the screen-reader list complete. The "busy loop"
  cases pass (18 views); the other cases are **not run yet**.
- `js/ui.js`: the details are facts of atoms with a rank; `layoutDetails`
  measures them, keeps what fits as whole lines (never wrapped or ellipsised),
  leaves the rest to further pages (`paginate`, `compose`), `moreDetails()`
  turns the page, `fitCard()` refits (resize observer, fonts, main.js's two
  calls). Order of what is kept longest: key, tempo, bars, meter, first style,
  album playing, first feeling, second style, second feeling, the shape word
  (airy, drifting), third style, third feeling, pass count. Tail facts share
  a line only when the card is too short for the loop's own four.
- `index.html`: `#loopDetail` (the lines drawn, `aria-hidden`), `#loopFacts`
  (every fact, for a screen reader), `#metaMore` (a button over the details
  with `#metaDots`, a dot a page); `js/main.js` wires its click.
- `css/style.css`: the playhead is round dots (5px, beats 7px, the lit one 14px
  with the ring and halo); the old per-width `.meta` steps removed; the
  `.meta-wrap` / `.meta-more` / `.meta-dots` rules.

**Left**
1. Run `node tools/card.mjs` whole and fix what fails (the album cards,
   Thai and emoji names, 320 px at 200%; a fact wider than the column on its
   own is on no page: see if any such exists).
2. Look at the round playhead in every theme, light and dark, forced colors
   and reduced motion (the light themes' glint offset in `i.on::before` is a
   first guess); 4/4, 6/8, 5/4, 7/8. Beats are still every fourth step
   (`buildPlayhead`): note in the PR that 6/8's dotted beats are Mikey's call.
3. Check `moveCursor` still touches only two dots a step; `tools/rows.mjs` ten
   passes (it reads `#loopDetail`'s text for the bpm and `#playhead`'s `.on`).
   Add a row or two for the tap (to the rest and back) if card.mjs alone is
   not enough for CI.
4. `tools/views.mjs`: no long task over 100 ms from the card.
5. Screenshots before and after, light and dark, in the PR; the order of what
   is left off first in the PR and in `docs/notes/for-mikey.md` (the card to
   look at on his phone and tablet).
6. Bump BUILD and CACHE; merge main's latest in (never rebase); CI; merge;
   `docs/DONE.md`, the queue's status line, tick this row in
   `docs/sessions/README.md` (keep every cell's `|`).

Written 2026-11-18 at Mikey's request (his list of seven for 1.0, items 3
and 4: the card's detail text on every display, and its playhead round).

- **Model:** Sonnet 5.5. **Effort:** high.
- **Start after:** session 23 (it changes what a name may hold, which the
  card shows). It may run alongside 21, 24, 28, 29, 30a and 30c; never
  beside 30b (both change the playheads' look in `js/ui.js` and
  `css/style.css`).
- **Queue item:** item 112.

Paste everything in the block as the session's first message.

````text
Hands session 30d on the road to Driftloom's 1.0 (Sonnet 5.5, high): the info card: clean lines, the rest a tap away, a round playhead (item 112).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 112); docs/PAINTER.md (the rules any design keeps); index.html (the info card: #card, #loopDetail, #playhead); js/ui.js (renderReadout, readoutLines, overflowing, renderAlbumCard, fitName, buildPlayhead, the layers' dots and moveCursor); css/style.css (.meta and its fit classes, .playhead, the themes' halo recipe, forced colors, reduced motion); js/main.js (where the card is drawn and the playhead moved); tools/views.mjs, tools/rows.mjs.

Check your place: in docs/sessions/README.md every row in session 30d's After column should be ticked, and every hands session running now named in its Alongside. If not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

Mikey (2026-11-18), two things on the card:

- "The detail text in the info card at the top doesn't always display properly on different sized displays. It's okay if some info gets left off; what's important is that the info that is showing is clean. If there is info that is left off, clicking the info card should switch to show the remaining info."
- "The playhead in the info card at the top; it would be nice for that to be circular rather than rectangular. It can appear similarly to the layers viewport playheads, but it's a bit bigger obviously and stands out a bit more."

The work, one PR, app only (BUILD and CACHE):

1. Find where the card's lines break today first: every view's card (a loop, a long piece, Endless, an album on the card, an album playing, a long name in Thai or with emoji since session 23) at widths from 320 px to a tablet's, landscape, and the browser's text at 200% and Android's largest font scale. Screenshots of the bad cases in the PR.
2. Every line shown is whole: no text cut off, clipped, overlapping, wrapped mid-fact or squeezed into tiny type. What doesn't fit is left off whole, in a fixed order of what matters least (write the order in the PR; the conservative order keeps the key, the tempo and the bars). renderReadout's folding may stay where it helps, but a fact is never shown broken.
3. When anything is left off, tapping the card's details shows the rest in the same place (tapping again goes back; a small quiet mark, not words, says there is more). Tapping the name still renames. A screen reader always hears every fact, whatever is shown.
4. The card's playhead becomes round: a dot a step, as the layers' dots are, with the same halo when lit, a bit bigger and standing out a bit more; the beats still marked; right for 4/4, 6/8, 5/4 and 7/8; in every theme, forced colors and reduced motion; only the two dots that change are touched each step, as today.

Yardsticks: the screenshots after, for every case in step 1, light and dark; a check in tools/ (or tools/rows.mjs's rows) that no visible line of the card overflows or is clipped at the widths and text sizes of step 1; tapping to the rest and back; tools/views.mjs: no long task over 100 ms from the card; tools/rows.mjs ten passes in a row; the gate. In for-mikey.md: the card to look at on his phone and tablet, and the order of what is left off first.

When you finish: add the merged PR's line to docs/DONE.md, keep the queue's status line current, tick session 30d's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR number), what to check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
