# Session 24b: an album's code carries how each track plays (105d)

Written by brain B-parallel (2026-11-18), from Mikey's answer: "Yes, an album's code should carry track lengths and all necessary info." Version 2 left the length slider out of a code (#412: the listener's, not the loop's), so since #427 a shared album plays each track until the listener's own slider moves on; 26 tracks of the old listening albums lost their pacing (`docs/ALBUMS.md`). The format freezes at 1.0, so this lands before brain B4 checks it.

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** session 24 (both change the codes' app side). It may run alongside 26 to 29 and 30b.
- **Queue item:** item 105.

Paste everything in the block as the session's first message.

````text
Hands session 24b on the road to Driftloom's 1.0 (Opus 5.5, high): an album's code carries how each track plays (105d).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 105); core/AGENTS.md; core/README.md ("Share codes, version 2"); docs/DECISIONS.md ("Share codes, requests and exports": 2026-11-17 and 2026-11-18); PRs #412, #425, #427 and session 24's; js/share.js, js/storage.js, js/library.js, js/main.js (albums, the length slider, how a track ends); docs/ALBUMS.md.

Check your place: in docs/sessions/README.md every row above session 24b should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

Mikey (2026-11-18): "Yes, an album's code should carry track lengths and all necessary info." Version 2 left the length slider out (#412), so a shared album now plays each track until the listener's own slider moves on.

The work, one PR (core and app, BUILD and CACHE; two if cleaner apart):

- An album's code carries, for each track, how long it plays before the next (the length slider's setting as the album's maker had it: passes or bars, a time, or until moved on), and anything else an album needs to play as its maker heard it. Read the code to find what else that is (the order, a track's own name, the album's name are in already; look at what a Library album keeps that its code drops) and list what you add and what you leave out, and why.
- In the core's format (core/src/code.rs, core/README.md): an optional field, costing nothing when the album leaves a track to the listener's slider, short when it doesn't (a setting shared by every track written once). Held to Mikey's line of 2026-11-17: nothing in a code only to keep an old code's sound, no code rot. A single loop's code: decide, by reading how the app plays a shared loop, whether it should carry the slider too, and say why (the conservative choice: it carries it only if a loop sounds wrong without it).
- In the app: sharing an album writes each track's setting; opening one plays each track for its own length, then the next; the Library's albums keep it.
- docs/ALBUMS.md's albums still out: their codes again with their pacing, where the old ones had it.

Yardsticks: every album setting round-trips (a test walks every slider value and the "until moved on" case, and fuzzes a few thousand albums); an album with no pacing set has the same code as before this PR; a 12-track album where every track plays 2 passes stays under about 20 characters longer than without; an album opened in a fresh profile moves on track to track at the lengths its maker set (tools/rows.mjs gains the row; run in CI's export workflow); corrupt codes never panic (fuzzed); the gate.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 24b's row in docs/sessions/README.md (keep every cell's `|`), and end with a plain summary for Mikey: what merged (PR numbers), what to check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
