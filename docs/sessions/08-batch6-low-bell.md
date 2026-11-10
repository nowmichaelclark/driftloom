# Session 08: batch 6, a new instrument: lowBell, and dropSpotlight (101g)

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** session 07.
- **Queue item:** item 101.

Paste everything in the block as the session's first message.

````text
Hands session 08 on the road to Driftloom's 1.0 (Opus 5.5, high): batch 6, a new instrument: lowBell, and dropSpotlight (101g).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 101); core/AGENTS.md (all of it: the rules for sound items); docs/ears/batch-6.md (Verdicts, this session's candidates in full, "What the levers did", and the [JUDGMENT] table: the vibe leans come from there, never 0); data/choices.txt; core/README.md (how to add a block); core/src/blocks/tubular.rs and temple_bell.rs (the nearest voices); core/src/generator/wander.rs (floor drops); the sketches lowBell and dropSpotlight.

Check your place: in docs/sessions/README.md every row above session 08 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

Rules for every batch 6 sound (Mikey, 2026-10-18 and 2026-11-08/09): today's behavior stays one option of every choice, never replaced; every version Mikey approved goes in as an option weighted by the vibes; each is drawn per loop from the loop's seed on a stream of its own, so nothing else moves; the sketched levels are what was approved. The sketches are in tools/ears/sketches/ears13/ and core/sketch/ (the levers of item 99): build the real thing in the core, where "What the levers did" says a lever fell short finish it and say how. Measure each as its section's "Measured" and "Yardstick" lines do, on the same corpus loops (corpus seed 20261009), build against sketch: level by ear within 1.5 LU, its tone figures, refused notes no more than the sketch's, VOICE_COST from render timing for a new voice, the coffee-shop probe (2-5 kHz share at or below the flute's) for anything in the melody's register, refusals on full and lite. The generation goldens, the core baseline and seeds.hashes move on purpose: update each and say in the PR what moved and why. The balance lock is re-baselined once, in session 09; until then say in each PR which locked figures moved and by how much. Add the new choices and pins to what the page reads (js/choices.js, js/pins.js) only where a request should be able to pin them.

The work, two candidates, one PR each:

- lowBell (#2): a new voice of the core, one file and one line, probed by the kit with its stored render: a struck low bell on the chord's root, between D2 and C#3 (so its fundamental stays above the 60 Hz guard), partials about 60 to 68 cents off whole multiples, the sketch's falls; and its swelling kind (0.4 s attack). The generator plays it (it knows the chords, so no silent organ): at each chord whose root changes, or the same root after 4 s. A seeded per-loop choice like glass's (today's none first), the struck and swelling kinds drawn per loop, each with its own vibe leans. Yardstick: a note on each chord change at the root's pitch class, 65 to 165 Hz; partials 50 to 90 cents off; falling 8 to 25 dB/s; 14 to 22 LU under the loop (thaw #71's sparse pad read only 3 dB under at its loudest: for Mikey's ears, say what you did); energy under 60 Hz within 1 dB of the loop without it; centered; never in a full stop; its cost.
- dropSpotlight (#11): with floorDrop's shapes and lowBell, in a loop that plays the bell: the bell steps forward in a share of the floor drops, struck or swelling. The generator knows when a drop starts, so it strikes on the drop's first beat (the sketch had to hear the floor go quiet and came in 1.5 to 2 s late). Yardstick: all of its energy inside the drops (plus its ring); every drop of 3 s or more caught at its start; 15 to 22 LU under the loop; the floor's return damps it.

Yardsticks: as above, plus VOICE_COST measured, the voice budget's refusals on full and lite, and peaks (no voice over -1 dBFS alone at the kit's probe).

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 08's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
