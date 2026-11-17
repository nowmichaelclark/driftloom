# Session 17: long pieces: the core's jump, and the map for the page (106a)

**Changed by brain B-parallel (2026-11-17), after Mikey's Batch 6 Album:** a first PR of its own before the jump, twang soft only (his verdict; session 22 was changing `data/` when it was heard, so it waits for 22 and lands here, as brain B2 planned for the album's data lines).

**Changed by brain B-parallel (2026-11-17):** the chains (`README.md`, "From 17 on"): "Start after" was "session 15"; it now names B2, which was the last row before the chains.

**Changed by brain B2 (2026-11-17):** batch 6 gave what plays time and memory (an opening that waits, a width that opens, slow LFOs, strings and bells that ring on, a color's level read from the chords before it), so the jump's bullet now says what each must do at a jump, and the yardstick says what "bit for bit" is held to.

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** brain B2 (batch 6 and 84b checked). It may run alongside 20, 21, 22, 24 and 26 (the README's chains); 18 and 27 wait for it.
- **Queue item:** item 106.

Paste everything in the block as the session's first message.

````text
Hands session 17 on the road to Driftloom's 1.0 (Opus 5.5, high): long pieces: the core's jump, and the map for the page (106a).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 106); core/AGENTS.md; docs/CORE.md (promise 11); docs/LONGFORM.md; docs/PAINTER.md ("Long pieces"); core/src/player/mod.rs (load_at, change, position, the events); core/src/wasm.rs and core/include/dlcore.h; js/worklet.js and js/player.js.

Check your place: in docs/sessions/README.md every row above session 17 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

First, its own PR before the jump (Mikey's Batch 6 Album, 2026-11-17, docs/DECISIONS.md): twang soft only. He kept every other batch 6 sound; of twang he said "I only liked the soft version. The bright and slow sounded off to me", and "right now 2% or more is okay" for its share. So in data/choices.txt's [setting twang], `kinds` gives only soft, the share and everything else as they are, so every loop that drew twang plays the soft kind and no other loop moves (the vibes lean each kind's share, `kindVibes`: if leaving two kinds out would change which loops play twang at all, keep today's twang loops playing it, soft, and the rest as they are, and say how; say in the PR which goldens moved and that only twang loops did; the balance lock and the longform workflow as the rules say; he-rosk and tom-wen, the album's bright and slow loops, now soft). If the data can't say a kind at weight 0, remove bright and slow from `kinds` and their `kindVibes` rows, keeping the code for them. Merge it under the policy, then the jump.

The jump's work, one PR, core and its wasm, the worklet's messages and their tests; no screens (session 18's). What the screens will need from the core:

- A jump: play from a section, or from a point in a piece (a bar, or a time), reached straight from the seed and the section's number (promise 11), heard within a second, starting on a phrase line with a short fade, nothing late. As a command (Player, dl_play_*, the C header) and a worklet message, tested natively and in wasm.
- What plays after a jump is what playing would have reached (brain B2, 2026-11-17): batch 6 put time and memory into the sound. slowOpening's wait and fade and widePad's late opening count from the loop's start, so past them a jump lands with the floor in and the width open, never waiting or opening again; padWow's and wideSweep's LFOs at the phase playing would have; nothing rung before the jump carried across it (twang's six strings, the low bell, dropSpotlight's damping: let go in the fade, as at a new loop); colorTone's level read from the chords after the jump, as at a loop's start. Find any other such state by reading the blocks (`core/src/blocks/`), not from this list.
- Where it is: the bar playing of how many, the time played and the time left (none for Endless), as the player's position already says or as it needs to say for a long piece.
- The piece's map for the page: for a stretch of sections, each one's role, theme, key and energy (session 11's plan), read through js/gen.js's instance of the module in a form cheap enough to draw section markers for a 32,768-bar piece (ask for a window, or a coarse summary of the whole; say which and why).

Yardsticks: a jump to three hours into a 32,768-bar piece and into Endless starts within a second on the Moto-class figures (tools/autolite.mjs's slow case), with no late note and no underrun; the section reached by a jump is bit for bit the section reached by playing (hash both: the generator's notes and the blocks' settings bit for bit; the audio, once the jump's fade and the room's tail have passed, as playing renders it, or within a figure the PR states and explains, on loops that draw slowOpening, widePad's late kind, padWow, wideSweep, twang and lowBell); jumping back and forth twenty times leaves nothing ringing that shouldn't; nothing that plays without a jump moved (the goldens, seeds, the baseline); the new commands in the 94a table's style in the PR.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 17's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
