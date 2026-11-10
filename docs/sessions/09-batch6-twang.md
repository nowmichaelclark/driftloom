# Session 09: batch 6, a new instrument: twang; then the batch's locks (101h)

**Changed by brain B1 (2026-11-11):** the melody twang answered (after 1.0).

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** session 08.
- **Queue item:** item 101.

Paste everything in the block as the session's first message.

````text
Hands session 09 on the road to Driftloom's 1.0 (Opus 5.5, high): batch 6, a new instrument: twang; then the batch's locks (101h).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 101); core/AGENTS.md (all of it: the rules for sound items); docs/ears/batch-6.md (Verdicts, this session's candidates in full, "What the levers did", and the [JUDGMENT] table: the vibe leans come from there, never 0); data/choices.txt; core/src/blocks/nylon.rs (the nearest voice); the twang sketch; tools/stats.mjs (the balance lock).

Check your place: in docs/sessions/README.md every row above session 09 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

Rules for every batch 6 sound (Mikey, 2026-10-18 and 2026-11-08/09): today's behavior stays one option of every choice, never replaced; every version Mikey approved goes in as an option weighted by the vibes; each is drawn per loop from the loop's seed on a stream of its own, so nothing else moves; the sketched levels are what was approved. The sketches are in tools/ears/sketches/ears13/ and core/sketch/ (the levers of item 99): build the real thing in the core, where "What the levers did" says a lever fell short finish it and say how. Measure each as its section's "Measured" and "Yardstick" lines do, on the same corpus loops (corpus seed 20261009), build against sketch: level by ear within 1.5 LU, its tone figures, refused notes no more than the sketch's, VOICE_COST from render timing for a new voice, the coffee-shop probe (2-5 kHz share at or below the flute's) for anything in the melody's register, refusals on full and lite. The generation goldens, the core baseline and seeds.hashes move on purpose: update each and say in the PR what moved and why. The balance lock is re-baselined once, in session 09; until then say in each PR which locked figures moved and by how much. Add the new choices and pins to what the page reads (js/choices.js, js/pins.js) only where a request should be able to pin them.

The work, two PRs:

1. twang (#13): a new voice in the spirit of a tremolo baritone guitar (a synth in its spirit, never an imitation: docs/DECISIONS.md, 2026-11-08). As the sketch: each string two band-limited saws 4 cents apart through a two-pole lowpass whose cutoff falls from a multiple of the note over 0.12 s, a two-stage fall, a pitch settling from 8 cents sharp; it picks each chord slowly, a beat apart (the lowest note's pitch class between E2 and D#3, its fifth, its tenth); one tremolo moves everything that rings. Three versions, each an option with its own vibe leans: soft (4.5 Hz, 35% deep), slow (two cycles a beat, 55%, rounder), bright (5.5 Hz, 45%, a brighter pick). On the chord layer, played by the generator at each chord, drawn per loop like glass (today's none first). The melody twang is not part of this session: Mikey (2026-11-11) wants it after 1.0; it is in the queue's "Later". Yardstick: the batch's (notes on the root, fifth and tenth a beat apart, 82 to 400 Hz; the pick's share above 1 kHz falling to a third or less by 0.3 s; the level swinging 30 to 55% at 4 to 6 Hz or one or two cycles a beat; 14 to 19 LU under the loop; energy under 60 Hz within 1 dB). Guards: its register starts at E2; centered; not over a kit busier than a soft one; never in a full stop.

2. The batch's locks: with all fourteen in, check that most loops still get one or two of the changes (data/choices.txt's change counts; node tools/stats.mjs --choices): if the larger pool has moved that, propose the smallest change that restores it and mark it for Mikey's ears. Then re-baseline the balance lock once (stats.mjs --check, then --write-baseline at --n 10000), the dial scale if it moved, and say in the PR every locked figure that moved across sessions 02 to 09 and by how much. Check docs/DONE.md has two corpus loops with a time for every candidate; fill in any that are missing.

Yardsticks: as above; the whole gate; a table in the last PR of all fourteen candidates, each with its option names, its share of 10,000 loops, and its two album loops.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 09's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
