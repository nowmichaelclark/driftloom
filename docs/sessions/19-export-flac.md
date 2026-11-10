# Session 19: audio export: the FLAC writer in the core (104a)

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** session 15 (it may run alongside 16 to 18: it adds a module nothing else touches until it merges).
- **Queue item:** item 104.

Paste everything in the block as the session's first message.

````text
Hands session 19 on the road to Driftloom's 1.0 (Opus 5.5, high): audio export: the FLAC writer in the core (104a).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 104); core/AGENTS.md; docs/CORE.md (phase 7, "Exports, safe first"; promises 2, 4 and 6); core/examples/dlrender/ (how a WAV is written today); core/README.md.

Check your place: in docs/sessions/README.md every row above session 19 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

The work, one PR, core only: a FLAC encoder the core carries itself (no dependencies, no_std, nothing allocated after start, clippy pedantic clean, every public item documented with its reason). Streaming: the host hands it blocks of samples (mono or stereo, 16 or 24 bits, the core's sample rates) and takes bytes out as they are made, so memory is the same for a minute or three hours; the stream header is finished at the end without holding the audio (write STREAMINFO with what is known and say how the host patches the total samples and the MD5 at the end, or streams without them: choose and say why). A correct, simple encoder is the aim (fixed predictors and Rice coding are enough: about half a WAV's size); a slow, clever one is not. Bit-exact on every target, like everything else.

Its exports for the web (dl_flac_* in core/src/wasm.rs) and the C header, behind a feature if it should stay out of the player's module until session 20 (remember what a line added above shipped code does to js/dlplayer.wasm: core/AGENTS.md); dlrender --flac writes a file from a seed or a note list.

Yardsticks: for the golden lists and twenty corpus loops, mono and stereo, 16 and 24 bits, 44.1 and 48 kHz: the file decodes with the reference decoder (flac or ffmpeg, in CI's runner, dev-only) and nulls sample for sample against the WAV of the same render, and its MD5 is right; the size against the WAV (the PR's table); a three-hour render's memory flat (the encoder's state is a fixed size: say how many bytes); its speed (times real time, natively and in wasm); the same bytes on wasm32 and natively; cargo test and clippy with and without the feature; js/dlplayer.wasm byte for byte unless you meant it to move.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 19's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
