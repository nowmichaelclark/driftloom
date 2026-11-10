# Session 02: batch 6, the far ends of three sounds we have (101a)

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** session 01.
- **Queue item:** item 101.

Paste everything in the block as the session's first message.

````text
Hands session 02 on the road to Driftloom's 1.0 (Opus 5.5, high): batch 6, the far ends of three sounds we have (101a).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 101); core/AGENTS.md (all of it: the rules for sound items); docs/ears/batch-6.md (Verdicts, this session's candidates in full, "What the levers did", and the [JUDGMENT] table: the vibe leans come from there, never 0); data/choices.txt ([setting subduck], [setting padsweep], [setting latefloor]); core/src/generator/choices.rs; core/src/blocks/sub_duck.rs and pad_sweep.rs; core/src/player/sequencer.rs (lateFloor).

Check your place: in docs/sessions/README.md every row above session 02 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

Rules for every batch 6 sound (Mikey, 2026-10-18 and 2026-11-08/09): today's behavior stays one option of every choice, never replaced; every version Mikey approved goes in as an option weighted by the vibes; each is drawn per loop from the loop's seed on a stream of its own, so nothing else moves; the sketched levels are what was approved. The sketches are in tools/ears/sketches/ears13/ and core/sketch/ (the levers of item 99): build the real thing in the core, where "What the levers did" says a lever fell short finish it and say how. Measure each as its section's "Measured" and "Yardstick" lines do, on the same corpus loops (corpus seed 20261009), build against sketch: level by ear within 1.5 LU, its tone figures, refused notes no more than the sketch's, VOICE_COST from render timing for a new voice, the coffee-shop probe (2-5 kHz share at or below the flute's) for anything in the melody's register, refusals on full and lite. The generation goldens, the core baseline and seeds.hashes move on purpose: update each and say in the PR what moved and why. The balance lock is re-baselined once, in session 09; until then say in each PR which locked figures moved and by how much. Add the new choices and pins to what the page reads (js/choices.js, js/pins.js) only where a request should be able to pin them.

The work, three candidates, one PR each (or one PR if they share files so much that three would conflict; say which you chose):

- deepDuck (#6): a second option of subDuck's choice, "deep", with its own depth and recovery ranges in data/choices.txt (the sketch: 14 dB, 0.6 s; the yardstick: 12 to 16 dB down 50 to 150 ms after each kick, back within 1 dB by 0.5 to 0.7 s). Guards: only with a kick and a bass; 16 dB at most; the recovery never longer than the kick spacing; kept from the calm vibes.
- wideSweep (#7): two more options of padSweep, "wide" (closing near 380 Hz, 3.5 to 4.5 octaves, a 14 to 20 s swing) and "fast" (2 to 3 octaves every 5 to 8 s), each with its ranges. Guards: sustained chord voices only; fast only on loops with a kit; never under the calm vibes.
- slowOpening (#9): lateFloor's long end: the wait up to 40 to 90 s, leaned long by the calm vibes, and a fade (the floor rising to its level over a drawn 4 to 20 s rather than arriving at once). Guards: lateFloor's own (something plays from the first second; not on a re-roll); the fade never over 20 s. A loop that draws today's lateFloor must play exactly as today.

Yardsticks: each candidate's own, measured and put in its PR as a table; a request can pin each (the pins' test walks the list); the standing checks.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 02's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
