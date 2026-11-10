# Session 23: codes, version 2, in the app (105b)

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** session 22.
- **Queue item:** item 105.

Paste everything in the block as the session's first message.

````text
Hands session 23 on the road to Driftloom's 1.0 (Opus 5.5, high): codes, version 2, in the app (105b).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 105); docs/PAINTER.md ("Share codes, version 2"); docs/DECISIONS.md (2026-10-12 and 2026-10-29: old codes may break, no compatibility bloat); session 22's PR and core/README.md's format; js/share.js, js/storage.js, js/library.js, js/main.js (share, paste, albums, the Library, backups); docs/ALBUMS.md; tools that read codes (tools/dials.mjs, tools/wordalbum.mjs, tools/stalls.mjs, tools/rows.mjs, tools/ranks.mjs, rank.html).

Check your place: in docs/sessions/README.md every row above session 23 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

The work, one PR (BUILD and CACHE): the app speaks version 2 and only version 2. js/share.js's version 1 goes (no shims: Mikey, 2026-10-29), replaced by the core's encode and decode through js/gen.js's instance.

- Showing, copying and pasting a loop's code and an album's; the Library's saved loops and albums. Saved loops are kept as their records today (js/storage.js): decide, by reading the code, whether they convert on first load to what version 2 can say (seed, request, edits) or are re-saved lazily, so a tester's Library survives the change even though old codes pasted from elsewhere stop working; say what is lost, if anything.
- A pasted version 1 code gets one plain line (it is from an older Driftloom and can't be opened), not an error.
- Every tool, test and doc that holds a version 1 code moves to version 2 (the stall tool's fixed loops, rows.mjs's fixed code from session 01, the dials' tests, rank.html, docs/ALBUMS.md's open albums if any are still out: ask the brain's note if unsure and leave a line in for-mikey.md).

Yardsticks: every app control that makes or reads a code works on version 2 (tools/rows.mjs: share, paste, an album, the Library after a reload, a backup and restore); a Library saved by the build before this one opens with its loops playing as they did (a scripted check: save twenty loops on the old build's storage format, load on the new); the "older Driftloom" line shows for a version 1 code; no version 1 code left in the repo outside docs/archive (grep for "DL1-" and "DLA1-"); the gate.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 23's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
