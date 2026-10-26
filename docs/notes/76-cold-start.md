# 76. The first loop after the app starts (v124)

**Built.** On the core, until a note has reached it (no render-ahead
measured this session), the engine plans every note no closer than the
device's output latency (`AudioContext.outputLatency`, else `baseLatency`),
and never less than 250 ms (`COLD_MARGIN`, `Engine.coldMargin` in
`js/engine.js`). It applies wherever the engine picks a start: Play, a new
loop or Next (`_cut`), a re-roll or mute (`_rewindPoint`). Once a note has
reached the core (`Synth.aheadMeasured`, new, in `js/synth.js`), planning is
item 41's, unchanged. The JavaScript synth never counts as cold. Nothing in
`core/` moved.

**The reading was right.** On `main` the unmeasured case fell back to the
80 ms lead (`Synth.renderAhead` is the context's `baseLatency` at best, a
callback's worth, before a note arrives).

**Diagnostics.** The log of late notes ends each `decided by` in `[margin]`
(planned from the cold margin) or `[measured]`. The report has a
`plannedFrom` line beside `renderAheadMs`: "the cold margin, N ms" until
the first note reaches the core, then "the measured render-ahead".

**Measured** (`node tools/coldstart.mjs`, new: the real app in headless
Chromium started from nothing for each run, Play pressed, 12 s played,
Diagnostics' `late` read; `--buffer 0.37` and `--delay <ms>` model a phone
as `views.mjs` does):
- On `main`, `--buffer 0.37`, 2 fresh starts each of full and lite: at
  `--delay 150`, late notes in 2 of 4 runs, and at `--delay 300` in 4 of 4,
  at 1.3-1.5 s, up to two per run, mostly left out, "decided by play, new
  loop" or "catch-up", posted 131-236 ms ahead against a core 17-84 ms
  past them.
- With the margin, 4 fresh starts each of full and lite at `--delay` 150,
  200 and 300 (all with `--buffer 0.37`): `late: 0` in all 24. Without a
  buffer or delay: `late: 0` in 6 of 6.
- `test/engine.test.mjs` (item 76): the cold start's first note is no closer
  than 250 ms, the output latency when it is larger, the callback when there
  is no output latency, a new loop just after Play too; measured, Play is
  at 80 ms and a change is heard from 60 ms as before; the JavaScript synth
  is at 80 ms. The test rig's core is "measured" by default, so every older
  check still tests the planning after the first measurement.
- Tests three times, `stats.mjs --check`, `baseline.mjs`, `build-core.sh
  --check`: unchanged (no core, generator or sound change).

**Limits.** The margin is a floor, not a measurement: a core that renders
more than about 250 ms (or the device's output latency) past the clock
before its first note would still lose the first notes. The harness's
`--delay 300` stayed clean only because the modeled output latency there is
over 500 ms. If the next phone report still shows a late note
`[margin]`, the margin is the thing to raise.

**For Mikey's ears:** sound now starts up to about a quarter of a second
after Play on the first loop after the app starts (and a Next or re-roll in
the first moments, before a note has reached the core). Conservative, as the
item says.

**For the tester (Mikey's Moto).** Close the app completely, open it, press
Play, let the first loop play to the end, open Diagnostics and Copy. The
report should say `late: 0` in the engine line, and under "late notes and
render-ahead jumps" no `late note` line; a `plannedFrom` line shows the
cold margin until a note has reached the core. Do the same on lite. If a
late note is listed, copy the report as it is: its `[margin]` or
`[measured]` says which planning missed it.
