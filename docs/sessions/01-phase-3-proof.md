# Session 01: phase 3's proof, and the tools made steady

**Changed by brain B1 (2026-11-11):** Mikey's two shorter plain lines go in
the app change of part 1; part 3 explains the two loops the brain's control
renders found different after a silent bar.

- **Model:** Sonnet 5.5. **Effort:** high.
- **Start after:** brain session B1 (or alongside it).
- **Queue item:** item 98.

Paste everything in the block as the session's first message.

````text
Hands session 01 on the road to Driftloom's 1.0 (Sonnet 5.5, high): phase 3's proof, and the tools made steady.

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 98); docs/CORE.md (promise 1); docs/notes/for-mikey.md.

Check your place: in docs/sessions/README.md every row above session 01 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

The work, two PRs:

1. The tools made steady (a tools-only PR, no version bump). (a) tools/rows.mjs plays a randomly drawn first loop, and two of its rows fail about one run in four on a quiet one ("sound: something plays", "mutes: all five layers off is silence, on again is sound": a tail still ringing reads as not silent). Make its rows independent of the draw: start from a fixed share code (tools/stalls.mjs has a set), and have the mutes row wait for the tail to fall rather than read once. Run it ten times in a row: ten passes. (b) Diagnostics still prints the old page scheduler's lines as constants (lateTicks: 0 and the like, js/main.js) because tools/perf.mjs, tools/stalls.mjs, tools/views.mjs and tools/autolite.mjs read them. Remove the constants and what reads them; that part is an app change, so it takes the next version (BUILD and CACHE). Keep what the core itself counts (late, missed, dropped, fill-ins). (c) In the same app change, Mikey's wording (docs/DECISIONS.md, 2026-11-11): the module that never arrives says "Please refresh.", and the too-slow line says "This browser is too slow." (js/main.js's NOT_LOADED and TOO_SLOW; rows.mjs's plain-line rows and for-mikey.md's lines follow).

2. Item 98's hands part. Run promise 1's stall test on the shipped app (tools/stalls.mjs: the main thread blocked for 2 s and for 5 s, the tab hidden for a minute, frozen; on shatter, the 32,768-bar piece and Endless; 4x and 6x CPU; full and lite): zero late or dropped notes, zero underruns. Put the table in the PR. If it is not clean, find why and fix it, or stop with a note (docs/notes/98-stall.md). Then write the testers' steps in plain words, one paste each, as docs/notes/98-testers.md: one for Mikey's Moto, one for his tablet, one for the friends' Pixels on GrapheneOS (with and without JIT allowed for the site, since item 100's line should show without it). Each: open the site, what to play (a few Nexts, a long loop from the Bars picker, Endless, the screen off for five minutes), then More, Advanced, Diagnostics, and exactly which lines to copy back (engine, busiest, slow, cores). Add one line to for-mikey.md pointing at the file.

3. A question from the brain's control (docs/ALBUMS.md, "Phase 3 Control"): of 30 corpus loops (corpus 20261009, the first 30) rendered in the v149 app (its Engine and Synth on the core, in Chromium) and on today's player, 28 null at -82 dB or lower, but tui-doask (seed 176315227) and soath-wa (2094475530) differ after their silent bars (from 31.1 s and 27.3 s; 25 to 50 dB under the music, fading over about ten seconds; loudness within 0.08 dB). Find what differs (a voice's phase, a noise state or a held param carried across bars where every layer rests are the brain's unproven guesses), say which side is right, and fix it only if today's is the wrong one. No PR needed if nothing changes: a paragraph in item 98's PR.

Yardsticks: rows.mjs ten of ten; the stall table clean; the two loops explained; the gate green.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 01's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
