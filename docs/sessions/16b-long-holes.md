# Session 16b: long pieces, no join a hole, on any piece (102g)

Written by brain B3 (2026-11-14) after session 05 found two more holes,
outside the corpus that session 16's yardstick plays; B3 measured them on
v167 against the loop before item 102 (dfe78c3).

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** session 05 (done, #392). Runs before 06.
- **Queue item:** item 102 (102g).

Paste everything in the block as the session's first message.

````text
Hands session 16b on the road to Driftloom's 1.0 (Opus 5.5, high): long pieces, no join a hole, on any piece (102g).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 102, 102f and 102g); core/AGENTS.md; docs/LONGFORM.md (the joins); docs/notes/102-mikey.md (he kept every join as it is: change only the ones that are holes); core/src/generator/longform.rs (Form::join_of) and heard.rs (how a join is played); tools/longform.mjs and tools/longform-played.mjs (HOLE_DB, arcJoins: the yardstick and the pieces it plays).

Check your place: in docs/sessions/README.md every row above session 16b should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

What is wrong. Session 16's yardstick (no join as played 25 dB under the loop as itself, where the join made the quiet) passes its corpus, but a join can still be a hole where the layers it keeps carry little of the loop: "the tune alone" where the tune is quiet or sparse, a floor out where the kit and the bass are most of the mix. Session 05 found three on Endless arcs, the same since v164 (docs/notes/for-mikey.md, under dottedPulse), and brain B3 measured them, quarter-second windows against dfe78c3's render of the same moment:
- ta-muin (seed 1337654778, bars=endless,journey=arc), "then alone" into 2:05: 124.0 to 125.75 s, 21 to 38 dB under (the mix near -45 to -56 dBFS where the loop is near -20); and into 9:26: 564.5 to 566.25 s, 19 to 35 dB under.
- yi-yoar (seed 819045617, same request), "then floor" out of section 15 into 6:14: 373.5 to 374.25 s, 33 to 40 dB under.
The yardstick missed them because it plays few pieces: the finite arcs of its corpus and two Endless arcs.

The work, one PR:
1. A join that would be a hole is not written: where what the join keeps would leave the mix far under the loop there, the join falls back to a lighter one or to a glide (nothing written), decided when the unit is made, from what the unit plays (its notes and velocities, the layers' weights), never from audio, and at no cost that grows with the section's number. Choose the measure that predicts the played depth; show it against renders. Every join that is not a hole stays exactly as it is (Mikey kept them): name in the PR how many joins over the corpus changed and which kinds.
2. Widen the yardstick so it would have caught these: every join of at least 200 pieces, Endless arcs among them (the first 30 minutes) and 1,024-bar arcs, as played. If that is too slow for the gate, put the wide run in the nightly and keep a smaller set (these three pieces and their times among it) in tools/longform.mjs.
3. Before and after, at the times above, as measures and as a short listening set on the long-clips branch (replacing what is there), not blind, listed in docs/notes/for-mikey.md.

Yardsticks: the three places at most 25 dB under the loop, for no more than half a beat; the wide run: no hole anywhere; tools/longform.mjs's checks all still pass; loops of up to 32 bars as before (bit for bit); the generation goldens moved only where a join changed (say which cases); a section's cost as before (dlgen --section-cost at 1, 1,000 and 10^9).

When you finish: add the merged PR's line to docs/DONE.md, keep the queue's status line current, tick session 16b's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR number), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
