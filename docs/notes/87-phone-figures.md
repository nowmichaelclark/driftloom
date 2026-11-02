# 87: rendering ahead, and the phone's figures

**Stopped at the yardstick (hands, 2026-11-02).** Everything in item 87 is
built and every other yardstick holds. Mikey ran the speed test on the Moto
G Power (2022): the worst block at full quality is **27.9% of its deadline
in stereo and 25.5% in mono** (theith-yaing), over promise 1's 25% by 2.9
and 0.5 points. The other three loops at full are 17.3-21.4%; lite is
10.1-17.1%. As the item says, the PR stops here for a decision (below); it
is not merged.

## Mikey's figures

Both devices: Chrome 156, Android 10, 48 kHz (a block's deadline 2,667 us),
cross-origin isolated, a 5 us clock. Worst block as a % of the deadline, and
rendering ahead in times real time.

**Moto G Power (2022)** (8 cores, 8 GB; 2.8 minutes):

| loop | full mono | full stereo | lite mono | lite stereo |
|---|---|---|---|---|
| vei-koan | 17.3%, 6.9x | 19.3%, 5.9x | 11.1%, 12.0x | 12.4%, 10.5x |
| koth-loar | 18.9%, 8.6x | 21.4%, 7.1x | 14.6%, 13.1x | 17.1%, 11.1x |
| theith-yaing | **25.5%**, 7.4x | **27.9%**, 6.3x | 15.2%, 14.3x | 16.3%, 12.2x |
| taing-doan | 18.0%, 9.4x | 20.2%, 7.8x | 10.1%, 20.2x | 10.9%, 16.4x |

**Mikey's older Lenovo tablet** (8 cores, 4 GB; 15.2 minutes), a weaker
device than the Moto, for the "and weaker" in the item's Why:

| loop | full mono | full stereo | lite mono | lite stereo |
|---|---|---|---|---|
| vei-koan | 93.9%, 1.2x | 108.9%, 1.0x | 55.5%, 2.1x | 61.1%, 2.0x |
| koth-loar | 90.7%, 1.7x | 104.1%, 1.4x | 68.1%, 2.6x | 74.3%, 2.3x |
| theith-yaing | 119.1%, 1.4x | 132.2%, 1.2x | 70.3%, 2.7x | 76.3%, 2.4x |
| taing-doan | 89.1%, 1.7x | 103.3%, 1.4x | 47.3%, 3.7x | 54.0%, 3.1x |

Lite, stereo costs 1.14-1.23x mono rendering ahead on the Moto, 1.08-1.20x
on the tablet (the item's guess was 1.2-1.4x).

**Where the worst block is:** theith-yaing at 7.7 s, the same block on both
devices and in every way. It is the core's own audio, not the player's work
(`dlrender --costs`: deciding the steps 93 us at most, the pass's draw 10
us, the block's audio 633 us natively at 48 kHz stereo), and not one odd
block: every block from 7.68 to 7.92 s costs about twice the loop's median
(wasm, Node). A held five-note chord, the texture and the melody sound
together there. So it is the full ceiling's own cost (260 units), and
nothing in the player can spread it.

## What needs deciding (brain, for Mikey's ears)

1. **A lower full ceiling on such phones** (the fix the item names): the
   worst block needs about 10% less work at theith-yaing's busiest moment.
   The hands would measure which ceiling does that and what it gives back
   in refusals, on the corpus, before anyone listens.
2. **Lite on such phones**, as the Moto's figures allow: 17.1% at worst,
   10.5-16.4x real time in stereo rendering ahead.
3. **Accept it for the Moto**: one loop of four, 2.9 points over in
   stereo; the others are under 21.5%.

For the game's render-ahead (the item's Why): on the Moto, lite stereo
renders a pass 10.5-16.4x faster than it plays, so a pass is well inside its
playing time. On the tablet it is 2.0-3.1x: inside, with little room. The
speed target for promise 2 is the brain's to propose; these are its inputs.
Once decided, the hands finish here (the Done line, the merge) or take the
fix as its own item.

## How the test was run

Mikey ran the speed test on the PR's preview (`speed.html`, also linked
from Diagnostics): one tap, then Copy. It plays vei-koan, koth-loar, theith-yaing and taing-doan
(the baseline's busiest four at full) silently, full and lite, mono and
stereo, in a worker, as the worklet plays them with `?engine=player`; each
block timed, each block's least over the runs (three at full, two on lite;
vei-koan 45 s at full and 30 s on lite, the others 20 s), at the phone's own
rate. It reports each loop's worst
block as a % of its deadline and when in the loop it fell, the blocks' share
of real time, and the render-ahead speed (times real time), with a summary
line for the yardstick and lite's stereo against mono.

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
quarter. The Moto's figures came out close to the cloud machine's.

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
