# Driftloom UI mockups: start here

This folder is the **UI mockup / design spec** for the one-screen phone app. It is not part of the app: `.assetsignore` keeps it off the live site. The detailed record of every decision lives inside the HTML file; this note tells you where to find it.

## What this is

One self-contained HTML file showing the same phone screen in nine color themes, plus a spec sheet and handover notes. Since v17 each phone also has the Library and Now Playing views (open them with the phone's own keys, or the gallery's View control). Open it in any browser; no build step.

- **Current file:** `driftloom_v17_library.html`.
- **State:** the 12-session platform-compliance roadmap is complete (WCAG 2.2 AA, iOS HIG, Material 3). v15 was a design pass from Mikey; v16 (Painter session 2) reworked the light themes; v17 (Painter session 3) added the Library and Now Playing views, Olive's wine-and-rust notes and a glint on the light-theme playhead. The design is **not locked yet**; Mikey reviews each version by eye.
- **In the app (Painter session 4, 2026-10-11):** v17 is ported into `index.html`, `css/style.css` (the spec's stylesheet lifted section by section), `js/ui.js` and `js/library.js`, on top of PR #153's logic. Deliberate departures: no LEDs or latched look on the bottom keys (they only change icons: Play, and the Sound / Layers switch), and Row 3 is two keys as in #153. Not yet re-measured with the test kit.
- **Relation to PR #153:** #153 (the first one-screen UI, Painter session 1) stays open on purpose until this new UI is finished and brought into the app.

## Where things are in the HTML

1. **The HTML comment at the very top** is the main handover. Read it in this order:
   - the **v17 · LIBRARY + NOW PLAYING** block (newest work, including the independent review);
   - the **v16 · LIGHT THEMES** block;
   - the **v15 · DESIGN PASS** block;
   - **WHAT SESSIONS 6–12 CHANGED**, including the Session 12 independent review;
   - the v13 / Session 5 block (text sizing);
   - earlier sessions;
   - **WHAT NOT TO DO** (hard rules) and **EDITING NOTES** (where each CSS and script block lives).
2. **Visible "handover" sections** on the page: one per session, in plain English, plus the roadmap.
3. **Spec tables:** every measurement a theme must honor.
4. **Gallery controls** above the phones: which view every phone shows (Layers, Albums, an album, Loops, More, Advanced, Now Playing), text size 100–200%, insets (iPhone, Android, or this device), touch-target outlines, simulate reduced motion, and a live audit line that re-measures all nine phones.

## Rules that must not break (short list; the full list is in the file)

- **Locked skeleton.** Zones are Info 205px, Dynamic ~319px and Buttons 240px, in a 780px content area. Nothing pops up. The zones never change height with text size.
- **Scrolling in exactly two places (v17).** Both inside the Dynamic zone: the Library's lists (Albums, an album, Loops) and the Advanced page. Nothing else scrolls; More and Now Playing fit at every text size.
- **Two settings, no more (v17).** Theme and Lighter on processor. Everything else is one behavior for everybody. The gallery's review controls are not part of the app.
- **Numbers stay with their words.** Use U+00A0 no-break spaces ("47%&nbsp;Glade").
- **Nothing in the Info card is ever dropped.** Lines that don't fit take turns in shared slots, with no animation, and the turning holds still while paused, hovered, focused, and for the rest of a loop once the user turns it by hand (the WCAG 2.2.2 mechanism).
- **Text sizes.** All text is 11px or larger and uses rem (the status bar excepted). The track name never goes below 28px except as a last resort, and is never cut.
- **Every control is a native `<button>`** with a name. On/off is a `role="switch"`. Play swaps its name between Play and Pause and has no `aria-pressed`.
- **Contrast is measured from pixels.** Text 4.5:1. Notes, beat cells, the lit playhead cell and focus rings 3:1. Off-beat playhead cells and empty-slot dots are deliberately quiet and treated as decorative.
- **Light-theme notes (v16):** keep their chroma; never darken them toward brown or gray to pass. Alternate light rows (~3.6:1) with deep rows (~5–6:1) and re-run the color-blind check after any change. Olive keeps to warm hues (since v17 its swatch's warm neighbors, wine and rust, are in); no cool hue.
- **Views (v17):** names are capitalized, actions are lowercase words on keys; "where you are" is a key held down with its LED lit; destructive keys ask twice by re-wording themselves (and their accessible name), never with a popup; no word is broken mid-word.
- **Reduced motion means nothing moves.** Every mark drawn as a background color needs a forced-colors rule.
- **Touch targets.** roll and on/off are drawn at 48×48. No invisible hit pads.

## How to verify (the test kit)

`test-kit/` holds the Playwright scripts behind every number in the handover.

```
cd design/test-kit
npm install
npx playwright install chromium
./run_all.sh ../driftloom_v17_library.html
python3 cvd_check.py ../driftloom_v17_library.html out/nontext_allon.json
```

Compare with `expected_output_v17.txt`. `sheet.mjs` makes contact sheets of any view, theme and size. Gotchas:

- **Screenshots far down the page come back wrong** at deviceScaleFactor 2 (more than ~8000 CSS px down). The scripts hide the handover and spec sections first (`SHALLOW=1`); for Nocturne, Abyss and Olive the lit-cell numbers are only valid from the `HIDEFIRST=1` run.
- **Load real fonts.** The scripts swap in local Inter and Space Grotesk via @fontsource and block every other network request.
- **Pre-installed Chromium:** if the sandbox already has a Chromium build, pin `playwright` to the matching version rather than downloading browsers.
- **Only Chromium has been tested.** Safari, Firefox and a real screen reader wait until the design is locked.

## Reference images (`reference/`)

- `v17_library_nine_themes.png`: the Library's Albums tab in all nine themes.
- `v17_views_light.png`, `v17_views_dark.png`: every new view state (Albums, an album, Loops, More, Advanced, Now Playing) in a light and a dark theme.
- `v17_text_200.png`: the new views at 200% text, plain and with the WCAG text-spacing override.
- `v17_glint_and_olive.png`: the light-theme glint up close, and Olive's notes before and after.
- `v17_forced_colours.png`: the new views in Windows dark and light contrast themes.
- `v16_all_nine_themes_100pct.png`, `v16_colorblind_check.png`, `v16_forced_colours_light.png`: the Layers view references from v16 (still accurate for Layers apart from Olive's notes and the new LEDs on the bottom keys).

## How Mikey likes to work

- **American English** in chat, docs and code comments.
- **One focused step per session.** Verify it against the standard, update the handover in the HTML, and bump the version (v16 → v17 …).
- **When proposing the next session, give four things:** new or continuing session, which model, what effort, and the session label (e.g. "Painter 3 · Now Playing view").
- **Mikey judges the design by eye.** Show changes as screenshots or contact sheets, and measure every compliance claim.
- **Use an independent reviewer.** An agent that hasn't seen the work catches over-claims.

## Open items

1. **Mikey reviews v17 by eye**, including one decision: should the Layers key show all the time (latched on Layers) so Row 3 never shifts under the thumb?
2. **The Play / Pause glyph is under 3:1 in four themes** (Dawn 2.05, Tidal 2.71, Olive 2.78, Night Light 2.88). Found in v17; it predates v16.
3. **Still unbuilt:** the Sound and Save views, the "add loops" and "move" pickers, naming a new album in place, the Info card showing the open album.
4. **Inert keys:** Previous, Next, Save/Share, Sound, and every key in the views that would change data.
5. **After the design is locked:** test Safari, Firefox, VoiceOver and TalkBack, then bring the UI into the app (superseding PR #153).
