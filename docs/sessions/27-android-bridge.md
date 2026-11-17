# Session 27: the Android app: the page talks to the native core (107c)

**Changed by brain B-parallel (2026-11-17), after 26:** the emulator's smoke job failed twice on unrelated PRs (#411: 90 s waiting for the WebView's inspector; #412: the page reloading in its first moment), so this session fixes `android/smoke.mjs`'s races first; it drives the same WebView.

**Changed by brain B-parallel (2026-11-17):** the chains (`README.md`, "From 17 on"): "Start after" was "session 26": it also waits for 17, since it carries 17's jump over the bridge (94a's table gains the jump) and both change `js/worklet.js`, `js/player.js` and `js/core.js`.

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** sessions 26 and 17. It may run alongside 18, 21, 22, 23 and 24; never beside 20 (both change "lighter on processor" in `js/main.js`).
- **Queue item:** item 107.

Paste everything in the block as the session's first message.

````text
Hands session 27 on the road to Driftloom's 1.0 (Opus 5.5, high): the Android app: the page talks to the native core (107c).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 107); docs/CORE.md ("the platforms"; promises 1, 2 and 4); docs/DECISIONS.md ("Platforms and games"); docs/PAINTER.md ("Platforms"); docs/archive/queue-items.md (item 94a: every control and display against its command or event); js/core.js, js/worklet.js, js/player.js, js/output.js, js/gen.js (the page's side of the worklet's messages); tools/rows.mjs; sessions 25 and 26's PRs.

Check your place: in docs/sessions/README.md every row above session 27 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

First, as part of this work: the emulator smoke job (`android/smoke.mjs`) flaked twice on PRs that didn't touch it (#411: it timed out after 90 s waiting for the WebView to open its inspector; #412: the page reloaded in its first moment, a race in its first connect). Make it wait for the page as it is, not as it was a moment ago, and pass ten runs in a row in CI before relying on it.

The work, one PR (BUILD and CACHE if the page changes): in the Android app the page's screens drive the native core by the same commands and events as the worklet (94a), and the WebView plays no audio of its own.

- A host on the page for Android (chosen when the app's bridge is present; the website is unchanged): everything js/core.js's CoreHost sends the worklet goes over the bridge to the native player instead, and everything the worklet posts back (events with their grids, counts, the cost and busy reports, probes, the picks' replies) comes back the same shape, batched a few times a second as the worklet does. Keep the page's code one path with two transports, not a fork.
- The page still reads a loop's record and notes from its own instance of the module (js/gen.js) in the WebView: that is not audio and stays as it is.
- The quality switch, the Mono switch, the length slider's hand-over, Next and Previous, re-rolls, mutes, tempo, tone, wander and its salt, the picks, a jump in a long piece: every row of 94a's table.

Yardsticks: tools/rows.mjs's rows run against the app on the CI emulator (through the WebView's debugging port, or as an instrumented test that drives the page): every row that makes sense there passes; Diagnostics in the app says the native core, with its late, missed and dropped counts and the stream's underruns; with the page's main thread blocked for 5 s the music does not stop (promise 1: the native thread needs nothing from the page); item 100's too-slow line can't show in the app. In for-mikey.md: the APK to install on the Moto and the tablet and what to do (a few Nexts, a long piece, the pickers), and Diagnostics' lines to send back.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 27's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
