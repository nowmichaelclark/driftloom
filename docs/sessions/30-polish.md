# Session 30: the known issues and the polish

- **Model:** Sonnet 5.5. **Effort:** high.
- **Start after:** sessions 24 and 29, and brain session B4.
- **Queue item:** the "Known issues" section and the road's "the known issues, the polish".

Paste everything in the block as the session's first message.

````text
Hands session 30 on the road to Driftloom's 1.0 (Sonnet 5.5, high): the known issues and the polish.

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, the "Known issues" section and the road's "the known issues, the polish"); docs/notes/for-mikey.md (every line: what Mikey has answered since, and what is still open); docs/DONE.md (what merged on the road); README.md.

Check your place: in docs/sessions/README.md every row above session 30 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

The work, small PRs, one per kind of thing. Don't start anything new: this session closes what the road left open.

1. for-mikey.md, line by line: for every line Mikey or the brain has answered (the brain's notes and docs/DECISIONS.md say), do what the answer asks (a wording, a level, a revert) or remove the line; for every line still open, check it is still true and still needed, and make it shorter and clearer. Nothing here is decided by you.
2. The queue's "Known issues" and "Also noted": for each, either it is done (move it to the archive with the brain's way of archiving), or it is not for 1.0 (say so in one line where it stands). Item 75 (the reverb, rate-true) stays low priority unless the brain's note for this session says otherwise.
3. A pass over the app as a listener would use it, with the tools: tools/rows.mjs ten times; tools/views.mjs on every view at 4x and 6x; tools/perf.mjs's baseline against docs/archive/perf-baseline.md's figures (say what moved since v54 and why it is fine or not); tools/autolite.mjs, all cases; the nightly green on every target (fix what is red). Fix small things you find; write up larger ones for the brain instead of fixing them.
4. The words: README.md as a newcomer reads it (what Driftloom is, how to run it, the app, the codes); a CHANGELOG.md begun, in plain words, from docs/DONE.md, grouped for listeners (what is new to hear, what is new to do), ready for session 31; the app's own About or version line if it has one.
5. The docs kept small: finished items out of docs/QUEUE.md into the archive as the brain does it, docs/README.md's map true, node tools/check-docs.mjs.

Yardsticks: the gate; the nightly green; rows ten of ten; for-mikey.md holding only what is truly still his.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 30's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
