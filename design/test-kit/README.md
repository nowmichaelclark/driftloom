# Driftloom test kit

Playwright scripts for the design spec.

## v18: baking the spec, and contact sheets

    npm install
    node bake.mjs                                   # writes ../driftloom_v18.html from the app
    node sheet18.mjs ../driftloom_v18.html out/s.png --places all --themes t-dawn --sizes 100

| Script | What it does |
|---|---|
| bake.mjs | Runs the app (repo root) in headless Chromium at 340 × 780, seeds 30 loops and five albums, plays an album, visits every view at 100 / 130 / 150 / 200% and writes the spec from `spec.template.html` |
| spec.template.html | The page around the phone: review controls, notes, the script that makes the keys work |
| sheet18.mjs | Contact sheets of the v18 file through its `window.spec` (`--places`, `--themes`, `--sizes`, `--paused`, `--insets android`) |

## v17: the compliance measurements

The scripts below measure the **v17** file, which has nine phones; they do not run on v18 yet (porting them is the next open item). Run them against the archived file:

    npx playwright install chromium
    ./run_all.sh ../archive/driftloom_v17_library.html     # writes images to out/

Compare the output with `expected_output_v17.txt` (v16: `expected_output_v16.txt`, v15: `expected_output_v15.txt`).

Color-blind check for the light themes' notes (after run_all.sh has written out/nontext_allon.json):

    python3 cvd_check.py ../archive/driftloom_v17_library.html out/nontext_allon.json

| Script | What it checks |
|---|---|
| cvd_check.py | Light-theme notes: neighboring-row OKLab distance under protanopia, deuteranopia, tritanopia (Machado 2009) and grayscale, plus each note's contrast under each |
| ui.mjs | The gallery's own audit at 100, 130, 150 and 200% |
| modes.mjs | Which info-line alternation mode applies at each text size |
| s11fine.mjs | 51 text sizes × plain / user text spacing × 6-, 8- and 12-character names, letting only the page's observer react |
| nontext.mjs | Note, beat-cell and lit-cell contrast against the real composited panel (ALLON=1 turns every layer on; SHALLOW=1 and HIDEFIRST=1 keep elements near the top of the page). Since v17 the lamp is measured with the glint hidden; GLINT=1 shows it |
| words.mjs | on / off / roll word contrast against the actual button fill |
| ring.mjs | Focus-ring contrast before vs after focus, for every control in every theme |
| a11y.mjs | Tab order, Enter / Space operation, ARIA snapshot |
| motion.mjs | Press transforms with no preference, with reduce, and with the gallery switch |
| rot.mjs | Info-line turning: 4 s pace, hover / focus / pause holds, tap takeover, resume on next loop |
| fc.mjs | Forced-colours render (dark or light) and probes |
| zoom.mjs | Reflow at 1280, 640 and 320 CSS px |
| views.mjs | v17: every Library / Now Playing state × 51 text sizes × plain / user spacing, through the page's own audit (nothing outside the panel except in a .scroll, fixed pages never overflow or overlap, 48 × 48 keys, no text under 11px or too wide, no word broken mid-word) |
| views_a11y.mjs | v17: text contrast of every new kind of text (pixels), non-text marks (LEDs, now-playing glyph, swatch edges and ring, progress), focus rings on every new control, keyboard and ARIA. Argument `text`, `marks`, `rings` or `keys` runs one part |
| icons.mjs | Buttons-zone icon contrast (icon pixels vs the key under them), every key, every theme; second argument `lib:albums` measures with Library latched |
| glint.mjs | v17 light-theme glint: twinkles only while playing, gone when paused, still under reduced motion, gone in forced colours, and how much of the lamp it covers |
| sheet.mjs | Contact sheets: phones in chosen views, sizes and modes (CROP=dyn, SPACING=1, FC=dark/light, RM=1, PAUSED=1) |
| shot9iso.mjs, pages.mjs, layers.mjs | Screenshot helpers for visual diffs and contact sheets |

Gotchas:

- At deviceScaleFactor 2, element screenshots more than about 8000 CSS px down the page are unreliable. Keep SHALLOW=1.
- The scripts load local fonts through @fontsource. Without real fonts, the measurements change.

- Select keys by class (.btn-lib, .btn-now, .btn-sound, .btn-layers), not position: since v17 Row 3's last child is the hidden Layers key.
- If the sandbox ships an older pre-installed Chromium, pin `playwright` to the matching version instead of downloading browsers (this run used playwright 1.56.1 with Chromium 141).
