# Brain session B1: the kickoff

- **Model:** Opus 5.5. **Effort:** high. In the claude.ai Driftloom Project.
- **Start after:** items 97 (#356) and 100 (#357) are merged: now.

Paste everything in the block as the session's first message.

````text
Brain session B1 for Driftloom's road to 1.0 (Opus 5.5, high): the kickoff.

Start as docs/BRAIN.md says ("Start here"). Then read docs/sessions/README.md (the plan the hands are working through: the order, who does what, and what is ticked), docs/QUEUE.md (status and "The road to 1.0"), docs/notes/for-mikey.md, docs/DONE.md's lines since your last session, and docs/DECISIONS.md's newest lines.

You are overseeing a run of hands sessions that Mikey starts one at a time from the prompts in docs/sessions/. Your job is what the brain's always is: verify what merged against what was asked, with your own measurements where a claim matters; keep the queue, the decisions and the plan true; build the listening albums; turn what Mikey says into decisions and into briefs; and never write app or core code. Merge only docs-only changes.

This session:

1. Check items 97 and 100 (PRs #356 and #357): the page's JavaScript before and after, the generation goldens held by the Rust generator alone (run node tools/gen-golden.mjs --check and the balance lock yourself), the plain lines, the too-slow rule and how its line was set (js/quality.js says; judge whether the tablet is safe from it). Then phase 3's check and the control album the queue asks for under item 98: today's app against the app before phase 3, the same codes; it should sound the same.
2. Read the whole plan in docs/sessions/ as a reviewer: the order, the splits, each prompt's yardsticks, the models and efforts. It was written by the hands (Fable 5.2) from the code as it stood on 2026-11-10. Fix what is wrong or thin, and bring docs/QUEUE.md into line with it (the items' text stays the brief; the sessions say how it is cut up). Pay most attention to docs/sessions/long-pieces-brief.md and sessions 11 to 16: long pieces are where Mikey's heart is. If you see a musical idea the brief lacks, or a risk it missed, add it.
3. Ask Mikey the decisions that are already waiting in for-mikey.md, and anything the plan needs from him early: the Android package name (session 25), and whether he wants a melody twang (session 09 leaves it out).

Always, before you end: record Mikey's answers in docs/DECISIONS.md (one dated line each) and remove the answered lines from docs/notes/for-mikey.md; fix any prompt in docs/sessions/ that what you learned makes wrong (say what you changed and why at the top of that file); keep docs/BRAIN.md's State current and the docs small; and tell Mikey, in a few plain lines, what is verified, what he should listen to or decide next, and which session to start next.
````
