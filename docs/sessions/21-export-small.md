# Session 21: audio export: the small format (104c)

- **Model:** Sonnet 5.5. **Effort:** high.
- **Start after:** session 20.
- **Queue item:** item 104.

Paste everything in the block as the session's first message.

````text
Hands session 21 on the road to Driftloom's 1.0 (Sonnet 5.5, high): audio export: the small format (104c).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 104); docs/CORE.md (phase 7); docs/PAINTER.md ("Exporting audio"); session 20's PR and its export code.

Check your place: in docs/sessions/README.md every row above session 21 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

The work, one PR, app only (BUILD and CACHE): the Small choice in the export sheet. Opus through the browser's own encoder (WebCodecs' AudioEncoder), written into an Ogg file the page muxes itself (a small, tested Ogg writer: pages, the CRC, the Opus headers, granule positions and pre-skip right, so players seek and show the true length); no library, no server. Fed in chunks from the same Worker render as the lossless path, straight to the file, memory flat. Where the browser has no such encoder, the choice is hidden (the queue's rule), never a fallback that could fail on a weak phone. If the browser offers AAC and not Opus, leave Small hidden there and say so in the PR: one format, done properly.

Yardsticks: the file plays in Firefox, Chromium, VLC and ffmpeg/ffprobe (duration right to the sample, no warnings); its size against the lossless file for the same code (the PR's table; about 70 MB for 2.5 hours mono was the charter's figure); a three-hour export with flat memory and nothing late in what plays; Cancel and a full disk as in session 20; the sheet shows Small only where it works (checked in Chromium and, with the encoder taken away, hidden); tools/rows.mjs's export rows cover both formats.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 21's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
