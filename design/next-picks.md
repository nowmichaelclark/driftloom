# Next picks: the chosen design

Chosen by Mikey on 2026-10-13, after four rounds and 70 options (Painter
session 5). This is the brief for building it into the app. The canvas
("Winner · next picks (locked)" page) and the three renders in
`reference/next_picks_*.png` show it; where this text and a render
differ, this text wins.

Three places: **Now Playing** shows what new loops lean toward, the
**New loops** panel lists the picks, and the **Length** picker sets bars.
Built from option 67/69 (Now Playing), 63 (the panel) and 69 (the picker,
plus a cancel key).

## The picks and their colors

Five lines, like Layers. Tempo and length share line two.

| Line | Shows | Color (dark themes) | Color (Dawn, Ember, Tidal, Olive) |
|---|---|---|---|
| 1 | moods | `#ffb347` | `#c77a12` |
| 2 | tempo, then length | `#6fd6c8`, `#b8a2f2` | `#1f8f84`, `#7c5cd6` |
| 3 | key and scale | `#f39ac0` | `#c94c86` |
| 4 | style (profile) | `#9fd38a` | `#4f9a3a` |
| 5 | voices | `#8fc3f0` | `#3a7fc4` |

Colors are decoration only: they outline pills and color the length
slider; text stays `--text-primary` / `--text-secondary`. Define them as
theme tokens (for example `--pk-mood`), with the darker set in the four
light themes.

Values carry their own meaning, so lines have no labels: "merry,
tender", "70–85 bpm", "16 bars", "any key", "any style", "any voices".

**Moods past three end in an ellipsis:** two moods read "merry, tender";
four or more read "merry, tender, dreamy…" (the first three, then "…").
The same rule applies on Now Playing and in the panel.

## The pill

A picked value is an outline pill: 30px tall in the panel (26px on Now
Playing), radius half the height, 12px side padding (10px on Now
Playing), a 1.25px inset ring in the pick's color at 80%, text 600
weight, 0.8125rem (0.75rem on Now Playing), `--text-primary`, tabular
digits, never wrapping.

A value left to chance is either plain secondary text with the line's
**any** key lit (the whole line is "any"), or, on line two when only one
of tempo and length is picked, a dashed pill ("any tempo" / "any
length"): 1.5px dashed border in the pick's color at 60%, text 500
weight, `--text-secondary`.

## Now Playing (outside an album only)

- **In an album the picks are hidden.** The album is what comes next, so
  Now Playing stays as it is today: album row, "Next: …", "leave album".
- **Outside an album:** no album row and **no "Next: …" line** (Mikey,
  2026-10-13: "Next: something new" goes). Below the clock and bar sit
  the picked values as outline pills, centered, on up to two rows:
  moods on the first, tempo and length on the second. Only picked values
  show; nothing says "+3 any". A row with nothing picked is left out
  (with only length picked, one row: "4,096 bars").
- **Nothing picked at all:** one quiet dashed pill, "anything", in
  `--rule`, so there is still something to tap. (Painter's call; Mikey
  may change it.)
- The pills are **one button** (min 44px tall) that opens New loops.
  Its accessible name says what it does, for example "New loops: merry,
  tender, 70 to 85 bpm, 16 bars".
- The bottom key stays as the app has it: "back to Harbor" after
  leaving an album, hidden otherwise.
- The clock keeps its full size; there is room now the album row is
  gone.
- The Vibes key (top left today) stays for now; see the open questions.

## New loops panel

- **Head:** title "New loops" on the left, a **close** key on the right
  (not "‹ Now"). Same `page-head` as the app's other views.
- **Five lines**, each at least 48px tall with a `--rule` line under it.
  Each line is two controls:
  - the values, as one button that fills the line (left aligned; on line
    two, tempo and length are two buttons side by side). Tapping a value
    opens its picker: moods open the Vibes list (as today), length opens
    the Length picker below. Tempo, key, style and voices pickers are
    not designed yet; until they are, those values can't be tapped.
  - an **any** key on the right: `vkey word`, at least 52 × 40px, text
    "any". **Lit** (`.vkey.armed`) when the whole line is left to chance;
    tapping a lit key does nothing, tapping an unlit one clears that line
    back to chance. On line two it clears both tempo and length.
- Nothing scrolls (the app's rule: only the Library and Advanced scroll).
- Clear all / reset is not on this panel in the chosen design.

## Length picker

Opened from the length value in the panel. A view of its own in the
middle viewport.

- **Head:** **cancel** on the left, "Length" centered, **done** on the
  right (done in 700 weight).
  - **cancel** goes back to the panel and restores the length chosen
    before the picker opened, whether that was a number, Endless or
    "any".
  - **done** keeps what is shown and goes back to the panel.
- **The body is vertically centered** in the viewport (not top-heavy),
  three groups 22px apart:
  1. **−1 · number · +1.** The number is 3rem Space Grotesk, 600,
     `-0.03em`, tabular, with a 2px underline in the length color; tap
     it to type (numeric keyboard; no hint text says so). Under it, in
     small secondary text: "bars · ≈ 45–55 s" (the play time at the
     tempo range; "≈ 3–4 hours" for long lengths). −1 and +1 are `vkey`s,
     at least 48px. Range 1 to 32,768.
  2. **The doubling slider.** 16 stops, one per power of two from 1 to
     32,768 (1, 2, 4 … 16k, 32k); labels under 1, 16, 256, 4k and 32k,
     the current one in `--text-primary`. A 4px track, filled in the
     length color up to the thumb; the thumb is a 24px white circle with
     a 3px ring in the length color. Dragging snaps to the stops; a
     number between stops (say 24) puts the thumb between them.
  3. **Endless:** "∞ Endless" and a switch (on = length color). No
     subtitle. When on, the number shows "∞" with "endless" under it,
     and −1, +1 and the slider dim to 40% and don't respond.
- **No explainer text anywhere** ("tap the number to type", "plays until
  you skip" and similar are out).
- Opened while length is "any": it starts at the playing loop's length;
  cancel returns to "any".

## Decided (Mikey, 2026-10-14)

Both open questions are answered, and the panel is renamed. Built in
build v99, spec v20. Where this section and the text above differ, this
section wins.

- **The way in during an album:** the picks on Now Playing stay hidden
  in an album; to see them, leave the album first. The top-left key now
  reads **"vibe"** (no count) and opens the panel, in an album and out
  of one.
- **The panel is called Vibe** (it was "New loops"). No title on screen,
  only the close key on the right ("cleaner this way"); screen readers
  still hear "Vibe". Its mood line opens the vibe words, whose close
  comes back to the panel.
- **Only tempo, or only length, back to "any":** each gets its own
  **any** key inside its own picker. In the Length picker the Endless
  label and switch are replaced by one **∞ key** that lights while
  endless is on, which leaves room for an any key beside it. With any
  lit the number reads "any" (the slider's thumb hollow); −1, +1, the
  slider or typing a number pick one. Turning ∞ off goes to a number of
  bars, not back to any. Tempo's any key comes with the tempo picker. Line two's any
  key in the panel still clears both.

## Open questions (as they were asked, now answered above)

1. **The way in during an album.** The pills are hidden in an album, so
   they can't open New loops there. Painter's suggestion: keep today's
   top-left key, renamed "new loops", in an album (where it is the only
   way in), and drop it outside an album (where the pills are the way
   in). Until Mikey decides, leave the Vibes key as it is.
2. **How does someone set only length (or only tempo) back to "any"?**
   Line two's any key clears both. The chosen design has no "any" inside
   the Length picker. Until Mikey decides, building the line-level key
   alone is enough; the dashed "any tempo" / "any length" pill exists for
   when tempo and length differ (for example, length picked from the
   picker while tempo is still "any").

## Building notes

- Today's generator draws bars from a profile table (2 to 32) and tempo
  from a band (32 to 190 bpm). Per the brain's rules (`docs/BRAIN.md`,
  "Two tracks"), wiring length and tempo to today's generator is fine if
  cheap, clamped to what it can play; the full range (32,768 bars,
  Endless) arrives with the core. Moods are today's vibes.
- Bump `BUILD` and `CACHE` together; bake the spec again afterward
  (`design/test-kit/bake.mjs`) so it shows the new views.
