# Session 05: batch 6, the kit: tripletKit and dottedPulse (101d)

**Left** (brain B3, 2026-11-14): tripletKit merged (#391, v167). dottedPulse
is #392 (v168, branch `claude/batch-6-tripletkit-dottedpulse-9o4sqv`),
green in CI and mergeable, with its Done line, for-Mikey lines and this
row's tick inside it; the session stopped while waiting on its second full
`tools/longform.mjs` run. Left: merge `main` into the branch if it moved;
run the full `tools/longform.mjs` once more: it must keep every check
`main` keeps, apart from the pass-block timing check on undertow #1 lite if
it flips (the same flip shows on `main`: say so in the PR); then merge #392
under the policy and stop. The holes it found on ta-muin and yi-yoar are
session 16b's, not this session's.

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** session 04.
- **Queue item:** item 101.

**Changed by brain B3 (2026-11-14):** its yardsticks now name the long
pieces' run (`tools/longform.mjs`, full), since these kits change the notes
an arc's fills and floor outs are made from; the rule is in the README's
"Rules for every session". Why: Mikey kept the long pieces as they are.

Paste everything in the block as the session's first message.

````text
Hands session 05 on the road to Driftloom's 1.0 (Opus 5.5, high): batch 6, the kit: tripletKit and dottedPulse (101d).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 101); core/AGENTS.md (all of it: the rules for sound items); docs/ears/batch-6.md (Verdicts, this session's candidates in full, "What the levers did", and the [JUDGMENT] table: the vibe leans come from there, never 0); data/choices.txt; core/src/generator/layers.rs (gen_drums, travel_groove: how a hit's step and micro place it); core/src/generator/wander.rs (what a pass does to the kit).

Check your place: in docs/sessions/README.md every row above session 05 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

Rules for every batch 6 sound (Mikey, 2026-10-18 and 2026-11-08/09): today's behavior stays one option of every choice, never replaced; every version Mikey approved goes in as an option weighted by the vibes; each is drawn per loop from the loop's seed on a stream of its own, so nothing else moves; the sketched levels are what was approved. The sketches are in tools/ears/sketches/ears13/ and core/sketch/ (the levers of item 99): build the real thing in the core, where "What the levers did" says a lever fell short finish it and say how. Measure each as its section's "Measured" and "Yardstick" lines do, on the same corpus loops (corpus seed 20261009), build against sketch: level by ear within 1.5 LU, its tone figures, refused notes no more than the sketch's, VOICE_COST from render timing for a new voice, the coffee-shop probe (2-5 kHz share at or below the flute's) for anything in the melody's register, refusals on full and lite. The generation goldens, the core baseline and seeds.hashes move on purpose: update each and say in the PR what moved and why. The balance lock is re-baselined once, in session 09; until then say in each PR which locked figures moved and by how much. Add the new choices and pins to what the page reads (js/choices.js, js/pins.js) only where a request should be able to pin them.

The work, two candidates, one PR each. Both levers fell short in the sketch, so this session finishes them: read "What the levers did" closely.

- tripletKit (#1): a kit in threes. Not the lever's re-timing: write the triplet hats itself (three a beat at the loop's hat velocities and density), move the eighth to two thirds of the beat (the shuffle's place) rather than dropping it, keep the kick on the beat, and leave 6/8 loops alone (their thirds are the eighths they already play). An option of a seeded per-loop choice, today's kit first. Yardstick: on the player's own note list, 60% or more of the non-kick hits on the beat's thirds (within a twelfth of a beat), under 5% on the straight eighth, the kick on the beat; the hats' share above 2 kHz no higher than the loop's own kit; refused notes no more than its straight kit. Soft and quiet, as the records play it.
- dottedPulse (#8), two versions: "pulse", a soft part every three eighths (the sketch's rim at velocity 0.2 read 38 to 43 LU under the loop, too quiet to hear as a lilt: give it a level of its own, or a tuned tick, measured so it sits about where the kit's ghost hits do, and mark the level for Mikey's ears); and "kick332", the 3-3-2 kick written whole (all three hits), not the loop's kicks moved. Watch the low guard: moved kicks raised dust #69's 20-60 Hz past 1 dB in the sketch. Yardstick: the drum stem's strongest repeat at 1.5 beats (pulse: of the part itself) or a 3-3-2 over two beats; the beat still clear.

Yardsticks: as above; the wander still does to these kits what it does to today's (floor drops, opened hats); refusals on full and lite; the full node tools/longform.mjs run keeps every check it keeps on main (a fill is the loop's own snare or rim, a floor out takes its kit and bass: with a triplet kit or a written 3-3-2 kick they must still land as joins, not as holes or extra hits).

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 05's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
