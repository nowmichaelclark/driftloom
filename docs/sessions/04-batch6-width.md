# Session 04: batch 6, width: widePad (101c)

**Order changed 2026-11-12 (Mikey):** this follows the long pieces (sessions 11 to 15) now. The long pieces' checks (`tools/longform.mjs`, the piece-map goldens, `core/tests/length.rs`) are part of the gate: keep them green, and say in the PR whether the new sound changes any piece's map (it should change only what a section plays, never the plan). The same holds for 05 to 10.

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** session 15 (brain B3 and its listening row may still be open).
- **Queue item:** item 101.

Paste everything in the block as the session's first message.

````text
Hands session 04 on the road to Driftloom's 1.0 (Opus 5.5, high): batch 6, width: widePad (101c).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 101); core/AGENTS.md (all of it: the rules for sound items); docs/ears/batch-6.md (Verdicts, this session's candidates in full, "What the levers did", and the [JUDGMENT] table: the vibe leans come from there, never 0); data/choices.txt; docs/CORE.md (promise 9: the stereo guards); core/src/space.rs; core/src/blocks/width.rs; tools/ears/stereo_guard.py.

Check your place: in docs/sessions/README.md every row above session 04 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

Rules for every batch 6 sound (Mikey, 2026-10-18 and 2026-11-08/09): today's behavior stays one option of every choice, never replaced; every version Mikey approved goes in as an option weighted by the vibes; each is drawn per loop from the loop's seed on a stream of its own, so nothing else moves; the sketched levels are what was approved. The sketches are in tools/ears/sketches/ears13/ and core/sketch/ (the levers of item 99): build the real thing in the core, where "What the levers did" says a lever fell short finish it and say how. Measure each as its section's "Measured" and "Yardstick" lines do, on the same corpus loops (corpus seed 20261009), build against sketch: level by ear within 1.5 LU, its tone figures, refused notes no more than the sketch's, VOICE_COST from render timing for a new voice, the coffee-shop probe (2-5 kHz share at or below the flute's) for anything in the melody's register, refusals on full and lite. The generation goldens, the core baseline and seeds.hashes move on purpose: update each and say in the PR what moved and why. The balance lock is re-baselined once, in session 09; until then say in each PR which locked figures moved and by how much. Add the new choices and pins to what the page reads (js/choices.js, js/pins.js) only where a request should be able to pin them.

The work, one candidate, one PR:

- widePad (#4): the chord layer much wider on the side only, so the mono render and the fold to mono stay exactly as they are. As the 13b sketch found: spread-chords alone stops at the width ceiling, so the option raises the ceiling for the chords (about -4, -5 and -7 dB for busy, sparse and very sparse loops) and uses a 250 Hz highpass on the side. A space option drawn per loop by the vibes, beside the baseline. Its late-opening kind (the width growing from the baseline to the wide value over 60 to 120 s) needs a space setting that moves over the loop: build that as the option's second version if it stays small; if it does not, stop at the first version and leave docs/notes/101c-late-width.md saying what it needs.

Yardsticks (the batch's): the chord stem's side/mid above 250 Hz at -8 to -4 dB; the fold within 0.1 LU of the mono render and every octave within 0.5 dB; side under mid below 120 Hz at -30 dB or lower; correlation over 0.4 at the 5th percentile; no bounce added beyond the sketch's; the mono render bit for bit as before; stereo cost on the baseline within 5%. Guards: all the stereo guards; never on the bass or the kick; sparse loops at the narrow end.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 04's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
