# Session 13: long pieces: the arc's energy, an opening and a close (102c)

**Changed by brain B1 (2026-11-11):** the energy's brightness through the
arc block (one control, not a second filter: the brief's "Three ideas"); the
close fitted to a section cut short; the section line on the audio thread;
the pieces to `docs/LONGFORM.md`, not to Mikey now. Why: as for session 12.

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** session 12.
- **Queue item:** item 102.

Paste everything in the block as the session's first message.

````text
Hands session 13 on the road to Driftloom's 1.0 (Opus 5.5, high): long pieces: the arc's energy, an opening and a close (102c).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 102); core/AGENTS.md; docs/CORE.md (promise 11, all of it); docs/sessions/long-pieces-brief.md (all of it); docs/LONGFORM.md; what session 12 built (its PR and the wander's code); core/src/blocks/arc.rs (the slow filter); core/src/generator/plan.rs (entry schedules).

Check your place: in docs/sessions/README.md every row above session 13 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

This is the work Mikey cares about most (2026-11-10: the composing for the longer lengths done really well; both journeys, an arc like a symphony's and long slow wandering, chosen by the vibes and somewhat at random, pinnable by a request). Take the time it needs. The rules that can't bend are in the brief: promise 11 (any section straight from the seed and its number; a section's cost never grows with its number; nothing allocates after Core::new), loops of up to 32 bars exactly as today, Endless never ends, changes at phrase lines, wholesome and calm, music computers are good at and never copies of real music, the journey plays at any wander setting (the slider is only its breathing), and what was asked (pins, mutes, a code's edits) is never moved. Listen with the tools you have: render excerpts (tools/clips.mjs, dlrender) and measure them; say plainly what you could measure and what only Mikey's ears can judge.

The work, one PR: an arc piece is heard to rise and fall. With one theme still (the loop's own; the second theme and the keys are session 14's), each section plays its role's energy: an opening that is thin, a build, a peak near two thirds, a release, a return, and for a finite piece a close that thins to the chords and the tune's last phrase, lands on the tonic and rings out. Endless plays arc after arc with a breath between.

Energy is carried by density and brightness more than by level: which layers play, how many of a layer's notes sound, velocities, the kit's pieces, the chords' register and length, the tune up an octave or doubled at the peak, the arc's filter and the space opening with it. The arc's filter is the loop's own slow swing, which Mikey likes: make the energy its slow aim (its centre and depth), not a second filter beside it. Reuse what session 12 built for moving a thing at a section line. Keep it gentle: someone working in a coffee shop should notice a build only when it has passed.

Yardsticks: over the corpus (tools/longform.mjs), measured on renders: an arc's loudness by ear and its brightness rise to the peak and fall, in the planned order, on at least 9 pieces in 10; the peak no more than about 4 LU over the opening (take the conservative end; mark it for Mikey's ears) and no section-to-section jump past 2 LU; every finite piece's last sounding chord is the tonic's, and the piece ends in a ring-out, not a cut, a last section cut short included; the round after it starts at the opening; nothing clicks at a line; a section's cost flat, and no block at a section line over the loop's own worst (natively and on tools/autolite.mjs's slow case); the baseline's costs within 5%; loops of up to 32 bars and every wander: bit for bit as session 12 left them. For the listening set: three arc pieces in docs/LONGFORM.md's "Pieces to hear" (64, 256 and 1,024 bars, journey=arc), each with the times of the opening, the peak, the release and the close.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 13's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
