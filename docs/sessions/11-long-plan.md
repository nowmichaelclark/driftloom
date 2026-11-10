# Session 11: long pieces: the plan and its map (102a)

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** session 10 (brain B2 may run alongside).
- **Queue item:** item 102.

Paste everything in the block as the session's first message.

````text
Hands session 11 on the road to Driftloom's 1.0 (Opus 5.5, high): long pieces: the plan and its map (102a).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 102); core/AGENTS.md; docs/CORE.md (promise 11, all of it); docs/sessions/long-pieces-brief.md (all of it); core/src/generator/piece.rs, wander.rs and mod.rs (Generator::section); core/src/generator/request.rs (how a request item and a pin are added; the theme); core/src/player/mod.rs (sections, events); core/examples/dlgen/main.rs; core/tests/length.rs.

Check your place: in docs/sessions/README.md every row above session 11 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

This is the work Mikey cares about most (2026-11-10: the composing for the longer lengths done really well; both journeys, an arc like a symphony's and long slow wandering, chosen by the vibes and somewhat at random, pinnable by a request). Take the time it needs. The rules that can't bend are in the brief: promise 11 (any section straight from the seed and its number; a section's cost never grows with its number; nothing allocates after Core::new), loops of up to 32 bars exactly as today, Endless never ends, changes at phrase lines, wholesome and calm, music computers are good at and never copies of real music. Listen with the tools you have: render excerpts (tools/clips.mjs, dlrender) and measure them; say plainly what you could measure and what only Mikey's ears can judge.

The work, one PR, and nothing in it sounds different yet:

1. Test the brief's starting design against the code, and write the design that stands as docs/LONGFORM.md (small: what a plan is, the arc's form, the wander's model, the journey's choice, the joins, the cost, what is left open). Change what doesn't hold and say why. Sessions 12 to 16 build from that file.
2. Build the plan as code: a pure function from the seed, the request and a section's number to what that section is (journey, role, theme and variation, key, tempo, energy, what changed from the section before), in closed form, with its own seeded streams, its numbers in data/ (a new file or a block of an existing one; build.rs compiles data/*.txt in). The journey's draw, leaned by the vibes; the request's journey=arc and journey=wander. The plan is computed and reported, and not yet heard: sections still play as today, so every golden, the baseline and the balance lock hold bit for bit.
3. dlgen --map SEED REQUEST: one line a section (and --from, --count for a far place in a long piece or Endless), and the same through the tools' build so Node tools can read it.
4. The corpus checks, as a Rust test and a tool (tools/longform.mjs): over a few hundred seeds at 64, 256, 1,024 and 32,768 bars and Endless, the map's promises: an arc's themes return (A after B, changed, in the home key); its energy rises to a peak and falls; a finite piece's last section is a close on the tonic; a wander moves at most one thing a section, holds as long as drawn; no key outside the related set; the same answer whether a section is reached straight or through; and a section's plan costs the same at section 1 and section 10^9 (timed).

Yardsticks: all of the above; tests/length.rs and tests/zeros.rs pass; the gate with nothing moved; a few maps pasted into the PR (64, 256 and 1,024 bars, Endless's first 60 sections and 60 from far in, an arc and a wander of each) so Mikey and the brain can read what a piece will be before it is heard. Put the open choices you settled, and the ones you left, in for-mikey.md, one line each.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 11's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
