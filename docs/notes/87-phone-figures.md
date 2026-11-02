# 87: rendering ahead, and the phone's figures

For the brain, from the hands' item 87. Everything in the item is built and
measured on the web host; one yardstick needs Mikey's phone, so the PR waits
for his figures (brain, 2026-11-02: "the Moto G Power (2022) figures can only
come from Mikey's phone").

## What waits

**The worst block at full quality, mono and stereo, on the Moto G Power
(2022), under promise 1's 25%.** Mikey runs the speed test on the PR's
preview (`speed.html`, also linked from Diagnostics): one tap, 10 to 15
minutes, then Copy. It plays vei-koan, koth-loar, theith-yaing and taing-doan
(the baseline's busiest four at full) silently, full and lite, mono and
stereo, in a worker, as the worklet plays them with `?engine=player`; each
block timed, each block's least over the runs (three at full, two on lite;
vei-koan 45 s at full and 30 s on lite, the others 20 s), at the phone's own
rate. It reports each loop's worst
block as a % of its deadline and when in the loop it fell, the blocks' share
of real time, and the render-ahead speed (times real time), with a summary
line for the yardstick and lite's stereo against mono.

- **Under 25%:** the hands merge, and the brain proposes promise 2's speed
  target for the phone from the render-ahead figures.
- **Over 25%:** the hands stop here, as the yardstick says, and this note
  takes the figures and by how much; the fix would be a lower full ceiling
  on such phones, for Mikey's ears.

## What the web host measured

The player in `js/dlplayer.wasm` on the hands' cloud machine (4 cores),
44.1 kHz, `tools/ahead.mjs` (Node 22, 45 s, each block's least over three
runs; render-ahead the best of three). Worst block as a % of its 2.9 ms
deadline, and rendering ahead in times real time:

| loop | full mono | full stereo | lite mono | lite stereo |
|---|---|---|---|---|
| vei-koan | 26.6%, 5.2x | 28.3%, 4.9x | 14.7%, 10.0x | 17.5%, 8.6x |
| koth-loar | 21.2%, 9.0x | 21.6%, 7.7x | 16.3%, 15.4x | 15.9%, 14.0x |
| theith-yaing | 20.6%, 8.5x | 24.4%, 7.4x | 11.7%, 16.7x | 15.5%, 14.8x |
| taing-doan | 18.8%, 12.9x | 23.9%, 10.1x | 7.7%, 26.7x | 8.6%, 22.2x |

Lite, stereo costs 1.09-1.20x mono rendering ahead. Headless Chromium
(`--chromium`, the phone's page itself, cross-origin isolated, 5 us clock)
reads the same within its noise: vei-koan full 27.2% mono, 26.9% stereo,
4.8-5.4x; lite 8.8-24.6x.

**The machine drifts:** the same check (`player-wasm.mjs --costs`, vei-koan,
full, mono) read 20.3% this morning and 26.3% this afternoon, on the same
build; item 85e's note read 19.1%. So the cloud figures are good to about a
quarter. **What to expect from the phone:** the busiest loop at full already
sits near 25% on a desktop-class core, and the Moto G Power's (Cortex-A53
class) are several times slower, so its figure will very likely be over. If
so, the decision is the one the yardstick names: a lower full ceiling on
such phones (or lite there), for Mikey's ears.

**The estimate** (`tools/ahead.mjs --estimate`, the baseline's first 20
loops, four passes each, a pass a chunk): estimate over the time taken, a
pass at a time, median 0.98-1.00 and 10-90% within 0.70-1.29 once scaled by
the last pass (as a host uses it); 0.90-1.04 median (10-90% 0.71-1.51) for a
first pass, before the voices are measured. The units are the budget's (a
note holds its cost through its tail), so the estimate is rough; it is
meant for spreading a chunk across frames with room to spare, and the first
pass is best made where there is no deadline (`ready`, a loading screen).

## How the timing could mislead

- A phone's browser coarsens its clock to 100 us unless the page is
  cross-origin isolated; `_headers` isolates `speed.html` (the report says
  `isolated yes` and the clock's step). If it says `no`, each figure is good
  to about 0.1 ms.
- The first run of a loop compiles the module's hot code again (the browser's
  faster tier), so the test warms up for a few seconds first and keeps each
  block's least over the runs; a pause the phone took for something else is
  not read as the core's.
- The test runs in a worker, not on the audio thread, which a phone runs at
  a higher priority; the worker's figures are the core's work, the
  yardstick's subject, not the thread's scheduling.
