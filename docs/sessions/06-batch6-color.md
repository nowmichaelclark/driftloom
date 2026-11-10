# Session 06: batch 6, harmony: colorTone (101e)

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** session 05.
- **Queue item:** item 101.

Paste everything in the block as the session's first message.

````text
Hands session 06 on the road to Driftloom's 1.0 (Opus 5.5, high): batch 6, harmony: colorTone (101e).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 101); core/AGENTS.md (all of it: the rules for sound items); docs/ears/batch-6.md (Verdicts, this session's candidates in full, "What the levers did", and the [JUDGMENT] table: the vibe leans come from there, never 0); data/choices.txt; core/src/generator/plan.rs (the progression, where the ninth is added now and then, long chords); core/src/generator/render.rs (the register work: chords_octave).

Check your place: in docs/sessions/README.md every row above session 06 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

Rules for every batch 6 sound (Mikey, 2026-10-18 and 2026-11-08/09): today's behavior stays one option of every choice, never replaced; every version Mikey approved goes in as an option weighted by the vibes; each is drawn per loop from the loop's seed on a stream of its own, so nothing else moves; the sketched levels are what was approved. The sketches are in tools/ears/sketches/ears13/ and core/sketch/ (the levers of item 99): build the real thing in the core, where "What the levers did" says a lever fell short finish it and say how. Measure each as its section's "Measured" and "Yardstick" lines do, on the same corpus loops (corpus seed 20261009), build against sketch: level by ear within 1.5 LU, its tone figures, refused notes no more than the sketch's, VOICE_COST from render timing for a new voice, the coffee-shop probe (2-5 kHz share at or below the flute's) for anything in the melody's register, refusals on full and lite. The generation goldens, the core baseline and seeds.hashes move on purpose: update each and say in the PR what moved and why. The balance lock is re-baselined once, in session 09; until then say in each PR which locked figures moved and by how much. Add the new choices and pins to what the page reads (js/choices.js, js/pins.js) only where a request should be able to pin them.

The work, one candidate, one PR:

- colorTone (#5): a held 13th over major chords and an 11th over minor ones, on a share of the chords. Not the sketch's block listening through a silent organ: the generator knows its chords, so add the color where the ninth is added, in the generator, as an option of a seeded per-loop choice (today's chords first), leaned toward the jazzy vibes. That removes the sketch's two limits (arpeggiated keys, one held chord). How it sounds is the sketch's: voiced just above the chord's top note (C6 at most), soft, swelling in and letting go; decide whether it is a note of the chord's own voice or a small voice of its own by which measures closer to the sketch, and say so.

Yardstick: on the chord layer, one color held over 25 to 50% of the chord time, the 13 over major chords and the 11 over minor; no color the chord's mode lacks; 14 to 22 LU under the loop; the 500 Hz to 2 kHz band no more than 1 dB up. Guards: never a flat ninth, a flat thirteenth over a minor chord or a natural 11 over a major one; chords with no third, a flat fifth, or the color already in them get none. The chords' octave rule (item 84) must not flip: run node tools/octave.mjs.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 06's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
