# Session 03: batch 6, pitch: tuningOffset and padWow (101b)

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** session 02.
- **Queue item:** item 101.

Paste everything in the block as the session's first message.

````text
Hands session 03 on the road to Driftloom's 1.0 (Opus 5.5, high): batch 6, pitch: tuningOffset and padWow (101b).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 101); core/AGENTS.md (all of it: the rules for sound items); docs/ears/batch-6.md (Verdicts, this session's candidates in full, "What the levers did", and the [JUDGMENT] table: the vibe leans come from there, never 0); data/choices.txt; core/src/master.rs (the wobble); core/src/blocks/ (how a channel's own block is one file and one line); core/sketch/ and tools/ears/sketches/ears13/padWow.

Check your place: in docs/sessions/README.md every row above session 03 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

Rules for every batch 6 sound (Mikey, 2026-10-18 and 2026-11-08/09): today's behavior stays one option of every choice, never replaced; every version Mikey approved goes in as an option weighted by the vibes; each is drawn per loop from the loop's seed on a stream of its own, so nothing else moves; the sketched levels are what was approved. The sketches are in tools/ears/sketches/ears13/ and core/sketch/ (the levers of item 99): build the real thing in the core, where "What the levers did" says a lever fell short finish it and say how. Measure each as its section's "Measured" and "Yardstick" lines do, on the same corpus loops (corpus seed 20261009), build against sketch: level by ear within 1.5 LU, its tone figures, refused notes no more than the sketch's, VOICE_COST from render timing for a new voice, the coffee-shop probe (2-5 kHz share at or below the flute's) for anything in the melody's register, refusals on full and lite. The generation goldens, the core baseline and seeds.hashes move on purpose: update each and say in the PR what moved and why. The balance lock is re-baselined once, in session 09; until then say in each PR which locked figures moved and by how much. Add the new choices and pins to what the page reads (js/choices.js, js/pins.js) only where a request should be able to pin them.

The work, two candidates, one PR each:

- tuningOffset (#12): a per-loop draw at load, beside echoTime's (outside the list of choices, as echoTime is): the whole loop a little off A440, within plus or minus 40 cents, weighted toward the hazy vibes and none for most loops (take the conservative share and mark it for Mikey's ears). Read where the core turns a note into Hz, so every pitched voice moves together; the tuned kick and the noises keep their pitch as in the sketch, unless measuring says a tuned kick against a detuned bass beats audibly, in which case move it too and say so. A host must be able to hold a loop at A440 (a pin: a game's own sounds may need it).
- padWow (#3): a slow wow on the chord layer only, as the sketch's block (a 3 to 5 ms delay moved by two sines: 14 cents at 0.5 Hz and a third of that at 0.185 Hz), as a channel's own block on the chords, an option of a seeded per-loop choice (today's none first), leaned toward the hazy vibes. Guards: never on the kick or the bass; no faster than 1.2 Hz; 20 cents at most. Yardstick: the chords' pitch moving 10 to 20 cents at 0.3 to 1 Hz, the drums' and bass's unchanged, the loop within 0.2 LU, its cost on the kit.

Yardsticks: as each section says; bit-identity on every target is the nightly's, but run tools/golden.sh --strict on wasm32 and natively yourself.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 03's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
