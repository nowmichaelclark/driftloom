# Brain session B3: long pieces, checked, heard, and turned into a brief

**Changed by brain B1 (2026-11-11):** the listening set is rendered excerpts (session 15), so this session builds their page.

- **Model:** Opus 5.5. **Effort:** high. In the claude.ai Driftloom Project.
- **Start after:** hands session 15 is merged.

Paste everything in the block as the session's first message.

````text
Brain session B3 for Driftloom's road to 1.0 (Opus 5.5, high): long pieces, checked, heard, and turned into a brief.

Start as docs/BRAIN.md says ("Start here"). Then read docs/sessions/README.md (the plan the hands are working through: the order, who does what, and what is ticked), docs/QUEUE.md (status and "The road to 1.0"), docs/notes/for-mikey.md, docs/DONE.md's lines since your last session, and docs/DECISIONS.md's newest lines.

You are overseeing a run of hands sessions that Mikey starts one at a time from the prompts in docs/sessions/. Your job is what the brain's always is: verify what merged against what was asked, with your own measurements where a claim matters; keep the queue, the decisions and the plan true; build the listening albums; turn what Mikey says into decisions and into briefs; and never write app or core code. Merge only docs-only changes.

This is the most important checkpoint on the road: Mikey wants the composing for the longer lengths done really well.

1. Check sessions 11 to 15 (item 102) against docs/sessions/long-pieces-brief.md, docs/LONGFORM.md and docs/CORE.md's promise 11: read piece maps yourself (dlgen --map) at 64, 256, 1,024 and 32,768 bars and Endless; run tools/longform.mjs; confirm a section's cost is flat, a jump lands where playing would, loops of up to 32 bars are untouched, the balance lock holds. Then listen as far as you can: render the listening set's excerpts and measure them (energy over time, where themes enter, the joins), and say where the measures and the plan disagree.
2. Judge it as music, plainly: does an arc feel like it goes somewhere and comes home; is a return recognizable and still new; does a wander stay alive over twenty minutes without anything abrupt; is anything a copy of a real form rather than a shape of its own. Write what you think is weakest.
3. Put the listening set in front of Mikey (docs/notes/for-mikey.md has it; add it to docs/ALBUMS.md): build a page from the excerpts on the long-clips branch (an artifact with the audio and a saved verdict per piece and per blind pair, as the Phase 2 Blind Album was; the blind key in its private data, never in the repo before he answers), in what order to listen, and three or four plain questions that will tell the hands what to change (too much, too little, too soon, the same). When he answers, in this session or a later one, write docs/notes/102-mikey.md: his words first, then each as something the hands can act on, in order of importance. That file is session 16's brief; session 16 must not start without it. If his notes ask for new composing rather than tuning, write a new queue item and a new session prompt in docs/sessions/ instead of stretching session 16.

Always, before you end: record Mikey's answers in docs/DECISIONS.md (one dated line each) and remove the answered lines from docs/notes/for-mikey.md; fix any prompt in docs/sessions/ that what you learned makes wrong (say what you changed and why at the top of that file); keep docs/BRAIN.md's State current and the docs small; and tell Mikey, in a few plain lines, what is verified, what he should listen to or decide next, and which session to start next.
````
