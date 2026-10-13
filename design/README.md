# Driftloom design spec: start here

This folder is the **design spec** for the one-screen phone app. It is not part of the app: `.assetsignore` keeps it off the live site.

## What this is

- **Current file:** `driftloom_v18.html`. One phone, exactly as the app draws it, at 340 × 780. Use it like the app: the bottom keys move between views, **Library › More** picks the theme, Play starts the playhead. Review controls beside the phone switch the view, text size (100 / 130 / 150 / 200%), insets (iPhone, Android, this device), touch-target outlines and reduced motion, and an audit line re-measures the phone after every change.
- **It is baked from the app (since v18, Painter session 5).** `test-kit/bake.mjs` runs the real app in headless Chromium, seeds a library, plays an album, visits every view at each text size and copies what the app drew. The phone's stylesheet is `css/style.css` itself. So the spec stays in step with the app as long as it is baked again after every change to the look.
- **Until v17 it went the other way:** the spec was drawn by hand (nine phones, one per theme) and the app followed it. v17 and its full handover (sessions 1–12, v13–v17) are kept unchanged in `archive/driftloom_v17_library.html`.

## How to change it

| To change | Edit | Then |
|---|---|---|
| How the phone looks | the app: `css/style.css`, `index.html`, `js/ui.js`, `js/library.js` | bake |
| The page around the phone (review controls, notes, tables) | `test-kit/spec.template.html` | bake |
| What the bake visits or seeds | `test-kit/bake.mjs` | bake |

```
cd design/test-kit
npm install            # Playwright; if the sandbox already has a Chromium, pin playwright to its version
node bake.mjs          # writes ../driftloom_v18.html
node sheet18.mjs ../driftloom_v18.html out/sheet.png --places layers --themes all
```

The bake prints the sample loop, the zone heights it measured and a check that the Layers notes it read from the generator match what the app drew. A design idea can still be tried by hand in a copy of the baked file; once it is agreed, it goes into the app and the spec is baked again.

## What is live in the file and what is baked

- **Live** (the page's script, following `js/main.js` and `js/library.js`): moving between views, Library tabs and albums, More and Advanced, Save and Share, the theme swatches, Play / Pause, the playhead and halo column at the loop's tempo, the Layers rows bar by bar (the loop's first pass; Wander's later changes are not shown), layer on/off, picking vibes, ticking albums, the sliders, the Now Playing clock.
- **Not live:** anything that makes music or changes data (Previous, Next, roll, new album, rename, export…). Those show "Not in the spec: try it in the app".
- **Baked:** every view's markup and card at each text size. The app fits the Info card in script, so each text size is its own capture; the rest follows the stylesheet live.
- **What the bake changes in the app's markup:** each canvas becomes a picture of what it drew, inputs keep their values as attributes, the moving marks (lit cell, halo column, clock) are cleared for the page's script to put back, and scripts are left out.
- **Three changes the bake makes to the stylesheet** (noted at the top of the file): fonts inlined; the short-screen rule (`max-height: 700px`) switched off, since the spec phone is never that short; the reduced-motion rules copied under `html.sim-reduced-motion` for the review switch.
- **For scripts:** `window.spec` has `show(place)`, `setTheme(cls)`, `setSize(pct)`, `setPlaying(bool)`, `audit()` and the lists of places, themes and sizes. The address bar takes the same: `driftloom_v18.html#view=library:loops&theme=t-dawn&size=150&paused&insets=android`.

## What changed since v17

- One phone instead of nine; the theme is picked inside it.
- Bottom keys: no LEDs or latched look; Row 3 is two keys (Now Playing, and the Sound / Layers switch).
- No drawn status-bar gap: the Info zone is 174px and the Dynamic zone 350px (v17: 205 / 319); Buttons stay 240px.
- The Info card has real cover art and the app's readout, which folds lesser lines away when full (v17's lines took turns instead).
- New: the Vibes key and view, Sound, Save and Share.
- Fixed in the app (build v93): two reduced-motion blocks in `css/style.css` damaged when v17 was brought in. Because of them no key sank when pressed (for everybody), and under reduced motion the glint, transitions and panel-key drops kept going.

## Reference images (`reference/`)

- `v18_nine_themes_layers.png`: Layers in all nine themes.
- `v18_every_view_nocturne.png`: every place in Nocturne.
- `v18_text_sizes_now.png`: Now Playing in Dawn at 100, 130, 150 and 200%.
- The `v16_*` and `v17_*` images belong to the archived v17 file.

## How Mikey likes to work

- **American English** in chat, docs and code comments.
- **One focused step per session.** Verify it, update the handover, and bump the version.
- **When proposing the next session, give four things:** new or continuing session, which model, what effort, and the session label (e.g. "Painter 6 · Measure the app").
- **Mikey judges the design by eye.** Show changes as screenshots or contact sheets, and measure every compliance claim.
- **Use an independent reviewer.** An agent that hasn't seen the work catches over-claims.
- **Don't theme the design on the name.** The app may not stay "Driftloom", so no loom, thread or weaving motifs in the UI (2026-10-13).
- **No decorative art on Now Playing.** The cover lives on the Info card; Now Playing stays text and controls.
- **Favor plain over metaphor.** Of 20 Next-picks directions (café menu, dice, radio, magnets…), Mikey chose the quietest: one small line on Now Playing that opens a plain list. Themed metaphors read as gimmicks.
- **Exact values, not just presets.** People want to pick exactly how many bars, not choose from five sizes; presets can sit beside an exact control.
- **Next picks, settled so far (round three, 2026-10-13):** the New loops panel has five lines, like Layers, with Tempo and Length sharing one line. Labels are optional where the value says what it is ("16 bars", "70–85 bpm", "any key"). The panel's key reads "close", not "‹ Now". Loop length runs from 1 to 32,768 bars, or endless, and gets its own picker view. Colored backdrops behind values are liked; colored dots, "+3 any" and pencil marks are not needed.
- **Round four (2026-10-13):** Now Playing shows the new-loop picks only outside an album. While an album plays, the album is what comes next, so the picks stay hidden. More than three moods end in an ellipsis ("merry, tender, dreamy…"). The length picker sits centered rather than top-heavy and has no explainer text ("tap the number to type", "plays until you skip").

## Open items

1. **Measure the app against the standard.** Port the v17 test kit (`test-kit/`) to the v18 file: one phone, a theme at a time, through `window.spec`. Since the file is the app's own drawing, that measures the app. Until then the v17 numbers describe the v17 drawings, not the app.
2. **Decide: the Info card drops lines when it is full.** v17's rule was that nothing in it is ever dropped (lines took turns, the WCAG 2.2.2 mechanism). The app folds the lesser feelings and styles away.
3. **The Info card can cut off its last lines.** Its text column is as tall as the art (118px). With the sample loop the readout fits at 100%, but from 130% the meter and album lines are hidden, even after folding as far as it goes. Seen in the app itself; the spec's audit line flags it.
4. **The Play / Pause glyph is under 3:1** in Dawn, Tidal, Olive and Night Light (found in v17).
5. **After the design is locked:** Safari, Firefox, VoiceOver and TalkBack.
