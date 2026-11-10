# Session 10: the chords nearer the tune (84b)

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** session 09.
- **Queue item:** item 101's "84b" paragraph.

Paste everything in the block as the session's first message.

````text
Hands session 10 on the road to Driftloom's 1.0 (Opus 5.5, high): the chords nearer the tune (84b).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 101's "84b" paragraph); core/AGENTS.md; docs/ALBUMS.md (the Phase 2 Blind Album's table: which loops moved which way and what Mikey heard); core/src/generator/render.rs (chords_octave and the register work); tools/octave.mjs.

Check your place: in docs/sessions/README.md every row above session 10 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

The work, one PR. Item 84 reads the chords' octave from the progression, so a pin or a non-harmonic change can't flip it. Mikey preferred, wherever he heard a difference, the chords nearer the melody: where 84 lifted the chords toward the tune (a gap of about 17 semitones to about 5) the new side won 3 of 3; where it dropped them away (about 5 to 17) the old side won 2 of 3. Keep 84's rule and change its aim: the chords sit within about a fifth to an octave under the tune, never pushed down to about 17 semitones under it.

First measure: extend tools/octave.mjs (it already writes each loop's octave and the gap between the tune's and the chords' centres) to print the gap's distribution over 2,000 corpus loops, today. Then change the aim, and print it again: how many loops moved, which way, and the gap's spread before and after. The chords must not collide with the tune (the balance lock has the collision share: it must hold, or move for the better, said in the PR) and must not crowd the bass (the low guard). Where a choice is open (how near, and what happens when the tune is very low), take the conservative one and mark it for Mikey's ears.

Yardsticks: no loop left with the chords more than about an octave under the tune because of the octave drop; node tools/octave.mjs: no flips; the balance lock, said figure by figure; the generation goldens and the baseline updated on purpose, with the count of cases that moved; refusals on full and lite no worse. In the Done line, name six corpus loops that moved downward under item 84 and are nearer now (three by a lot, three by a little), with times, for Mikey's album; and add the line to for-mikey.md: a revert follows if he hears worse.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 10's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
