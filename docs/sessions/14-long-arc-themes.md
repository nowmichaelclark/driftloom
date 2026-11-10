# Session 14: long pieces: themes that return changed, and related keys (102d)

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** session 13.
- **Queue item:** item 102.

Paste everything in the block as the session's first message.

````text
Hands session 14 on the road to Driftloom's 1.0 (Opus 5.5, high): long pieces: themes that return changed, and related keys (102d).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 102); core/AGENTS.md; docs/CORE.md (promise 11, all of it); docs/sessions/long-pieces-brief.md (all of it); docs/LONGFORM.md; core/src/generator/melody.rs (the tune, its motifs, answerFigure and varyFigure, development); core/src/generator/request.rs and spec.rs (the theme, item 85d and 85f); core/tests/theme.rs.

Check your place: in docs/sessions/README.md every row above session 14 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

This is the work Mikey cares about most (2026-11-10: the composing for the longer lengths done really well; both journeys, an arc like a symphony's and long slow wandering, chosen by the vibes and somewhat at random, pinnable by a request). Take the time it needs. The rules that can't bend are in the brief: promise 11 (any section straight from the seed and its number; a section's cost never grows with its number; nothing allocates after Core::new), loops of up to 32 bars exactly as today, Endless never ends, changes at phrase lines, wholesome and calm, music computers are good at and never copies of real music. Listen with the tools you have: render excerpts (tools/clips.mjs, dlrender) and measure them; say plainly what you could measure and what only Mikey's ears can judge.

The work, one PR (two if the keys are cleaner apart: the themes first): the arc becomes a piece with ideas.

- A second theme: theme B from a seed derived from the piece's, through the theme machinery the request already has (another tune bent into this loop's style; a theme that rests stays resting, 85f). Some pieces keep one theme only (simple loops stay simple: Mikey's favorite in the Depth album was a simple 24-bar loop); how often is in docs/LONGFORM.md, or for you to settle conservatively.
- Themes that return, changed: the plan's variations, each a pure function of the theme and a drawn number, built and tested one at a time: in the related key; in another register; handed to another voice of the profile; stretched to twice its length over held chords; over the relative key's chords; answered (the first half kept, the second drawn anew); thinned to its outline. The development plays fragments of A and B. The return is A with one variation: known at once, and still new.
- Related keys: from the loop's mode (the fifth above or below, the relative, the parallel mode where the profile allows it). The bass, the chords, the tune, the tuned kick and anything else that reads the key move together, and only at a section line.

A variation must not make several loops to compare, and must not walk earlier sections: check the cost.

Yardsticks (tools/longform.mjs, on what is played): theme B is heard in the role the plan gives it, in its key; A returns after B, in the home key, and is neither a copy of its first statement nor unrecognizable (a measure of your own, said in the PR: for example the share of the tune's rhythm and contour kept, held between two bounds); every variation appears in the corpus and each is tested alone; no key outside the related set; no clash added (the tune's time a semitone over a chord tone no higher than the loop's own); the octave rule holds in every key (tools/octave.mjs's check, run on sections); a section's cost flat, a theme-B section within 10% of a theme-A one; refusals on full and lite no worse. For Mikey, in for-mikey.md: the same three arc pieces as session 13 plus two more, with the times where B enters, where the development is, and where A returns, and which variation each return uses.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 14's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
