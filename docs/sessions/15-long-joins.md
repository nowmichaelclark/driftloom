# Session 15: long pieces: the joins, Endless, the measures, and the listening set (102e)

**Changed by the brain after session 13 (2026-11-12):** the breath that stepped 2.7 LU (Mikey merged #377 with it), to fix here.

**Changed by brain B1 (2026-11-11):** endings that land when the length
slider stops a piece before its close; the listening set as rendered
excerpts on a throwaway branch (the app can't jump inside a piece or take a
journey until sessions 17 and 18, so album codes can't reach "three
quarters in"), shorter, with a blind before-and-after pair. Brain B3 builds
the page from the clips.

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** session 14.
- **Queue item:** item 102.

Paste everything in the block as the session's first message.

````text
Hands session 15 on the road to Driftloom's 1.0 (Opus 5.5, high): long pieces: the joins, Endless, the measures, and the listening set (102e).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 102); core/AGENTS.md; docs/CORE.md (promise 11, all of it); docs/sessions/long-pieces-brief.md (all of it); docs/LONGFORM.md; sessions 12 to 14's PRs; docs/ALBUMS.md and js/share.js (how an album code is made today); tools/wordalbum.mjs.

Check your place: in docs/sessions/README.md every row above session 15 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

This is the work Mikey cares about most (2026-11-10: the composing for the longer lengths done really well; both journeys, an arc like a symphony's and long slow wandering, chosen by the vibes and somewhat at random, pinnable by a request). Take the time it needs. The rules that can't bend are in the brief: promise 11 (any section straight from the seed and its number; a section's cost never grows with its number; nothing allocates after Core::new), loops of up to 32 bars exactly as today, Endless never ends, changes at phrase lines, wholesome and calm, music computers are good at and never copies of real music, the journey plays at any wander setting (the slider is only its breathing), and what was asked (pins, mutes, a code's edits) is never moved. Listen with the tools you have: render excerpts (tools/clips.mjs, dlrender) and measure them; say plainly what you could measure and what only Mikey's ears can judge.

The work, two PRs:

1. The joins. Where two sections meet and something changed (a theme, a key, the layers that play), the join is written, not left to chance: a fill into the line; the floor out for the last bar and back on the line; a held chord across the line that both keys share; a bar of the tune alone; a breath. Chosen by what changes and by the energy either side. Then go through Endless and the long finite lengths end to end: Endless's arcs in turn with their breaths, a wander that runs for hours, a round coming round after a close, a jump straight to a far section (promise 11) sounding as if it had been reached by playing. Then the breath between arcs: #377 merged with one step of 2.7 LU between sections (the yardstick was 2), where tui-doask's 50 s sections make a breath one unit long and the tune rests in it (docs/notes/102c-arc-brightness.md). Keep the tune at its thinnest through a breath rather than resting it, or glide the rest across the unit, so every arc piece in tools/longform.mjs steps 2 LU or less between sections. Also note for Mikey's ears where batch 6's slowOpening and the arc's opening hold the kit back together (gluith-vuisk, rong-wuisk): whether that start runs too long thin. Then endings that land (the brief's "Endings that land"): when the length slider will stop a piece or Endless before its close, the phrase before the stop plays the close (thinned, on the tonic, ringing out) in place of today's fade; with the slider off nothing changes; for Mikey's ears. Fix what you find.

2. The measures and the listening set. Run everything tools/longform.mjs checks, at 64, 256, 1,024 and 32,768 bars and Endless, a few hundred seeds, and put the table in the PR, with what failed and how often. The generation goldens' long-length cases, written again, with the count that moved. Render and time: a section's cost at section 1, 1,000 and 10^9; the nightly's 24 hours of Endless (dlrender --endless 24) run by hand once (workflow_dispatch) and green. Then the listening set for Mikey, at the top of docs/notes/for-mikey.md, from docs/LONGFORM.md's "Pieces to hear" and the corpus: about eight pieces (64, 256 and 1,024 bars and Endless; arcs and wanders; several styles and vibes), chosen for being clear examples (say how you chose; don't pick only the best), each as rendered excerpts of about 40 s where something happens (the start, B's entry, the peak, the return, a slider-made close, the close; a wander's moves; for Endless, its first twenty minutes), plus one 64-bar piece whole: under about 30 minutes in all, each excerpt with one line on what to listen for. And one or two pairs for a blind test: the same seed and length rendered before item 102 (from the commit before session 12) and now, level-matched, the files named honestly (before, after); brain B3 makes them blind on its page, the key in the page's private data, so no key is ever in the repo. The clips, stereo Opus, go on a throwaway branch long-clips with only the clips and an index (the Ears' way, docs/DECISIONS.md 2026-11-08); brain B3 builds the page. Also the pieces as seeds and requests, for after sessions 17 and 18 and item 105, when the app can play them whole.

Yardsticks: every join kind appears in the corpus and none clicks; no join is left unwritten where the plan says something changed; the whole gate; promise 1's stall test on an arc of 32,768 bars and on Endless; a slider-made close on twenty pieces lands on the tonic and rings out, never cut; docs/LONGFORM.md brought up to what was built. Update item 102 in the queue to what was built, in the queue's style.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 15's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
