# Session 20: audio export in the app: lossless, safe first (104b)

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** sessions 18 and 19.
- **Queue item:** item 104.

Paste everything in the block as the session's first message.

````text
Hands session 20 on the road to Driftloom's 1.0 (Opus 5.5, high): audio export in the app: lossless, safe first (104b).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 104); core/AGENTS.md; docs/CORE.md (phase 7, "Exports, safe first"); docs/PAINTER.md ("Exporting audio"); js/gen.js (a second instance of the module on the page, as a pattern); js/main.js (the MIDI export, and "lighter on processor"); tools/autolite.mjs.

Check your place: in docs/sessions/README.md every row above session 20 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

The work, one PR (two if the core's part is cleaner apart), BUILD and CACHE. Safety first (Mikey, 2026-10-13): an export must never crash or break anything, whatever the device.

- The render: a separate instance of the core in a Worker (never the one that plays, never the audio thread), rendering the loop or the piece in small chunks through session 19's FLAC writer straight to the file, so memory is flat for a minute or three hours. What is exported is what plays: the same seed, request and edits, the wander as the code has it, a long piece whole, and for Endless a length the sheet asks for.
- The file: written as it is made, where the browser can (the File System Access API, or the origin's private file system and a download at the end); where it can't, say so before starting and offer what fits in memory safely, with the limit stated. Free space checked first (navigator.storage.estimate, or the platform's own), with a plain message if there isn't enough.
- The sheet (plain, in the app's style): Lossless now; the Small choice hidden until session 21 builds it. Progress with Cancel; a note that it keeps going in the background, and it does (a Worker survives the tab being hidden; say what each browser does and what happens when the page is closed). Done: open or share the file. Failed: a plain, kind message.
- Speed from "lighter on processor": on, gentle (the Worker yields so the music playing never stutters and the device stays cool); off, as fast as is safe. Measure both.

Yardsticks (the queue's): a three-hour lossless export on the Moto-class figures in tools/autolite.mjs's slow case finishes with flat memory (sampled through the run) and nothing late or underrun in what plays meanwhile; the file decodes and nulls against a straight render of the same code; Cancel leaves no partial file and nothing broken; a full disk (simulated) ends with the plain message and the app still playing; tools/rows.mjs gains the export's rows. In for-mikey.md: an export to try on the Moto and the tablet, and the sheet's wording to approve.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 20's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
