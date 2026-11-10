# Session 12: long pieces: the wandering journey (102b)

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** session 11.
- **Queue item:** item 102.

Paste everything in the block as the session's first message.

````text
Hands session 12 on the road to Driftloom's 1.0 (Opus 5.5, high): long pieces: the wandering journey (102b).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 102); core/AGENTS.md; docs/CORE.md (promise 11, all of it); docs/sessions/long-pieces-brief.md (all of it); docs/LONGFORM.md (the design that stands); core/src/generator/wander.rs; core/src/player/sequencer.rs and mod.rs (how a pass and a section are played, what changes at a phrase line).

Check your place: in docs/sessions/README.md every row above session 12 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

This is the work Mikey cares about most (2026-11-10: the composing for the longer lengths done really well; both journeys, an arc like a symphony's and long slow wandering, chosen by the vibes and somewhat at random, pinnable by a request). Take the time it needs. The rules that can't bend are in the brief: promise 11 (any section straight from the seed and its number; a section's cost never grows with its number; nothing allocates after Core::new), loops of up to 32 bars exactly as today, Endless never ends, changes at phrase lines, wholesome and calm, music computers are good at and never copies of real music. Listen with the tools you have: render excerpts (tools/clips.mjs, dlrender) and measure them; say plainly what you could measure and what only Mikey's ears can judge.

The work, one PR: the wander is heard. A piece whose journey is the wander plays its plan: long held states, exactly one thing moving at each hold's end (which layers play, a layer's voice within the profile's pools, the tone, the chords' register and length, the tune's density, and rarely the key or the theme, as docs/LONGFORM.md settled), a glide where a thing can glide and a phrase line where it can't, nothing abrupt. Inside a hold the loop still wanders as today. A finite wander closes softly; Endless goes on.

Build it as marks and levels on the loop's own events and the mix, the way a wandered pass is, so a section still costs one loop's making. Anything a move needs that the core can't yet do at a section line without a click or a gap (a voice changing, a layer fading in over a section) is this session's to build, small, with a test.

Yardsticks: tools/longform.mjs's wander checks now read what is played (from the player's note list, dlrender --calls), not only the map: at most one thing moves a section; holds as drawn; no move mid-phrase; nothing clicks at a line (the measure tool's clicks check on twenty section lines); loudness across a 20-minute render within the catalog's spread (no jump past 2 LU between neighboring sections); a section's cost flat from the first to the ten-thousandth (timed) and the baseline's costs within 5%; promise 1's stall test on a wander of 32,768 bars. Loops of up to 32 bars and every arc piece: bit for bit as before. The long-length goldens move: say which cases. For Mikey, in for-mikey.md: three wander pieces as requests he can play (seed, vibes, bars=256 and Endless, journey=wander), each with the times where something moves, and what to listen for.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 12's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
