# Session 16: long pieces, tuned from Mikey's notes (102f)

**Changed by brain B1 (2026-11-11):** the second listening set rendered as
session 15's is (excerpts on a throwaway branch), for the same reason.

**Changed by brain B3 (2026-11-14):** reads `docs/notes/102-b3-measured.md`,
whose three bugs (a silent pivot, "the tune alone" over a silent varied tune,
a development that stutters with a join every unit) are its first PR, Mikey's
notes or not; without his notes it does that PR alone and stops, so it can run
before he has listened. Why: B3 measured them on v164; they are defects, not
taste, and his notes should not have to name them.

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** brain session B3 and Mikey's listening: this session needs his notes.
- **Queue item:** item 102.

Paste everything in the block as the session's first message.

````text
Hands session 16 on the road to Driftloom's 1.0 (Opus 5.5, high): long pieces, tuned from Mikey's notes (102f).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 102); core/AGENTS.md; docs/CORE.md (promise 11, all of it); docs/sessions/long-pieces-brief.md (all of it); docs/LONGFORM.md; docs/notes/102-b3-measured.md (what brain B3 measured: three bugs, and where the measures and the plan disagree); docs/notes/102-mikey.md (the brain writes it in session B3 or later from what Mikey said about the listening set: what he liked, what bothered him, what he wants more or less of).

Check your place: in docs/sessions/README.md every row above session 16 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

This is the work Mikey cares about most (2026-11-10: the composing for the longer lengths done really well; both journeys, an arc like a symphony's and long slow wandering, chosen by the vibes and somewhat at random, pinnable by a request). Take the time it needs. The rules that can't bend are in the brief: promise 11 (any section straight from the seed and its number; a section's cost never grows with its number; nothing allocates after Core::new), loops of up to 32 bars exactly as today, Endless never ends, changes at phrase lines, wholesome and calm, music computers are good at and never copies of real music, the journey plays at any wander setting (the slider is only its breathing), and what was asked (pins, mutes, a code's edits) is never moved. Listen with the tools you have: render excerpts (tools/clips.mjs, dlrender) and measure them; say plainly what you could measure and what only Mikey's ears can judge.

First PR, whether or not Mikey's notes are in: the three bugs in docs/notes/102-b3-measured.md. A pivot's chord takes its velocity as the landing's does (a chord the loop plays, else the chords' loudest), never a silent entry's; "the tune alone" reads the tune as the unit plays it (its variation) and is a breath or a glide where that is silent; inside a development, a fragment's trade (A's for B's, or back) glides, and a join stays where the role, the key or a stated theme changes. Add the yardstick that would have caught them: a join, as played, takes something out but never leaves the mix more than about 20 dB under the same moment of the loop (tools/longform.mjs, against a render with the plan's marks off or the loop as itself), and no development has more than two joins in a row. Name before and after times on fei-no (seed 2158762586, bars=256,journey=arc: 5:23 and 8:08), sheim-gling (1278568317, bars=1024,journey=arc: 7:13 to 9:40) and moan-teil (2060199680, bars=1024,journey=arc: 29:17). Merge it under the policy.

Then, if docs/notes/102-mikey.md is missing or empty, stop: the rest of this session exists to act on what Mikey heard, and guessing would waste it. Write "Left: Mikey's notes" at the top of this file, say so and end.

The work after it, one PR per change of substance: take his notes one at a time. For each: find it in a piece from the listening set (the time he named, or the nearest), find what in the plan or the playing makes it, change the smallest thing that answers it (a share, a range, a lean, a variation dropped or added, a join), and show before and after as a measure and as a new excerpt time for him. Where a note asks for something the design can't do without new composing (not a number), say so in for-mikey.md with what it would take, and leave it for the brain to queue.

Yardsticks: every note answered in the PR, one line each: changed (what, and where to hear it), not changed (why), or queued; tools/longform.mjs's checks still pass; loops of up to 32 bars as before; the long-length goldens written again; a second, shorter listening set with only what changed, rendered as session 15's was (excerpts on the long-clips branch, listed in for-mikey.md).

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 16's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
