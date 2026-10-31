# 85a: a fast tempo pin and lite's refusals (hands, 2026-10-31)

Item 85a (#302: the request, weighted vibes and tempo) passes every check
but one, the standing rule "refusals on full and lite for the loops the item
touches: melody at full quality stays near zero, and lite no worse than the
catalog". A tempo pinned near the top of the range fails the lite half, so
the PR is not merged. I need a decision.

## What was measured

`node tools/measure.mjs --refusals` (new `--request` option), melody's
per-loop refusal rate, corpus seed 1:

| loops | quality | mean | loops losing any |
|---|---|---|---|
| catalog, 40 | full | 0.7% | 2.9% |
| `bpm=180`, 40 | full | 0.4% | 5.7% |
| `bpm=40`, 40 | full | 0.0% | 0% |
| `cozy=0.6,zany=0.6,bpm=100-140`, 40 | full | 0.2% | 2.9% |
| catalog, 40 | lite | 2.2% | 8.6% |
| `bpm=40`, 40 | lite | 1.3% | 8.6% |
| `cozy=0.6,zany=0.6,bpm=100-140`, 40 | lite | 1.5% | 8.6% |
| catalog, 100 | lite | 1.4% | 8.0% |
| **`bpm=180`, 100** | **lite** | **4.8%** | **22.7%** |

Full quality holds everywhere. Lite holds for slow, middling and weighted
requests, and fails for a fast pin.

## Why

The pin works as built: with style left to chance it leans the mix toward
profiles whose band holds the tempo, so at 180 bpm 66 of 100 loops are led
by shatter (band 140-178). The refusals are nearly all shatter's, and
shatter refuses that much in lite on its own:

| shatter-led loops, 40, lite | mean | loops losing any |
|---|---|---|
| catalog (its own tempos) | 6.2% | 25.7% |
| `bpm=180` | 7.7% | 21.9% |

So the pin barely moves shatter. The failure comes from the mix: a fast pin
asks for the profile whose lite melody already loses notes. Any request that
mostly asks for shatter (a style pin in item 91, fast vibes) would fail the
same way.

## What I need decided

1. **Accept it for 85a.** Read the rule per profile: a pin may not make a
   profile's lite refusals worse than that profile's own, and for shatter it
   does not. Lite's shatter refusals get their own item. This is what I'd
   suggest.
2. **Fix shatter's lite melody refusals first** (the voice budget in lite,
   or shatter's density there), then merge 85a as it is.
3. **Lean less hard at the fast end** (`TEMPO_PREFER`, now 6 bpm), so fewer
   loops at 180 are shatter. This trades the refusal figure against "prefer
   profiles that hold the tempo" (item 48), and other profiles at 180 play
   well outside their bands.

Everything else is green: 3,060 old generation-golden cases unchanged and
360 request cases added (all targets local and Chromium), baseline "Nothing
moved", balance lock unchanged, pinned tempo measured within 0.19 bpm, the
range covered evenly, same samples native, wasm and C (#302's body has the
figures).
