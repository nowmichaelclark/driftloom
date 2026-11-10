# Session 17: long pieces: the core's jump, and the map for the page (106a)

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** session 15 (it does not wait for session 16).
- **Queue item:** item 106.

Paste everything in the block as the session's first message.

````text
Hands session 17 on the road to Driftloom's 1.0 (Opus 5.5, high): long pieces: the core's jump, and the map for the page (106a).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 106); core/AGENTS.md; docs/CORE.md (promise 11); docs/LONGFORM.md; docs/PAINTER.md ("Long pieces"); core/src/player/mod.rs (load_at, change, position, the events); core/src/wasm.rs and core/include/dlcore.h; js/worklet.js and js/player.js.

Check your place: in docs/sessions/README.md every row above session 17 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

The work, one PR, core and its wasm, the worklet's messages and their tests; no screens (session 18's). What the screens will need from the core:

- A jump: play from a section, or from a point in a piece (a bar, or a time), reached straight from the seed and the section's number (promise 11), heard within a second, starting on a phrase line with a short fade, nothing late. As a command (Player, dl_play_*, the C header) and a worklet message, tested natively and in wasm.
- Where it is: the bar playing of how many, the time played and the time left (none for Endless), as the player's position already says or as it needs to say for a long piece.
- The piece's map for the page: for a stretch of sections, each one's role, theme, key and energy (session 11's plan), read through js/gen.js's instance of the module in a form cheap enough to draw section markers for a 32,768-bar piece (ask for a window, or a coarse summary of the whole; say which and why).

Yardsticks: a jump to three hours into a 32,768-bar piece and into Endless starts within a second on the Moto-class figures (tools/autolite.mjs's slow case), with no late note and no underrun; the section reached by a jump is bit for bit the section reached by playing (hash both); jumping back and forth twenty times leaves nothing ringing that shouldn't; nothing that plays without a jump moved (the goldens, seeds, the baseline); the new commands in the 94a table's style in the PR.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 17's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
