# 87: rendering ahead, and the phone's figures

For the brain, from the hands' item 87. Everything in the item is built and
measured on the web host; one yardstick needs Mikey's phone, so the PR waits
for his figures (brain, 2026-11-02: "the Moto G Power (2022) figures can only
come from Mikey's phone").

## What waits

**The worst block at full quality, mono and stereo, on the Moto G Power
(2022), under promise 1's 25%.** Mikey runs the speed test on the PR's
preview (`speed.html`, also linked from Diagnostics): one tap, about ten
minutes, then Copy. It plays vei-koan, koth-loar, theith-yaing and taing-doan
(the baseline's busiest four at full) silently, full and lite, mono and
stereo, in a worker, as the worklet plays them with `?engine=player`; each
block timed, each block's least over the runs (three for vei-koan at full,
45 s; two elsewhere), at the phone's own rate. It reports each loop's worst
block as a % of its deadline and when in the loop it fell, the blocks' share
of real time, and the render-ahead speed (times real time), with a summary
line for the yardstick and lite's stereo against mono.

- **Under 25%:** the hands merge, and the brain proposes promise 2's speed
  target for the phone from the render-ahead figures.
- **Over 25%:** the hands stop here, as the yardstick says, and this note
  takes the figures and by how much; the fix would be a lower full ceiling
  on such phones, for Mikey's ears.

## What the web host measured

See the PR body (Node and headless Chromium, per loop, lite and full, mono
and stereo).

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
