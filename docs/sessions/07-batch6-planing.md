# Session 07: batch 6, harmony: planingSevenths (101f)

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** session 06.
- **Queue item:** item 101.

Paste everything in the block as the session's first message.

````text
Hands session 07 on the road to Driftloom's 1.0 (Opus 5.5, high): batch 6, harmony: planingSevenths (101f).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 101); core/AGENTS.md (all of it: the rules for sound items); docs/ears/batch-6.md (Verdicts, this session's candidates in full, "What the levers did", and the [JUDGMENT] table: the vibe leans come from there, never 0); data/choices.txt; core/src/generator/plan.rs (the progression's draw); core/src/generator/melody.rs (how the tune reads the chord it sits over); core/src/generator/layers.rs (the bass).

Check your place: in docs/sessions/README.md every row above session 07 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

Rules for every batch 6 sound (Mikey, 2026-10-18 and 2026-11-08/09): today's behavior stays one option of every choice, never replaced; every version Mikey approved goes in as an option weighted by the vibes; each is drawn per loop from the loop's seed on a stream of its own, so nothing else moves; the sketched levels are what was approved. The sketches are in tools/ears/sketches/ears13/ and core/sketch/ (the levers of item 99): build the real thing in the core, where "What the levers did" says a lever fell short finish it and say how. Measure each as its section's "Measured" and "Yardstick" lines do, on the same corpus loops (corpus seed 20261009), build against sketch: level by ear within 1.5 LU, its tone figures, refused notes no more than the sketch's, VOICE_COST from render timing for a new voice, the coffee-shop probe (2-5 kHz share at or below the flute's) for anything in the melody's register, refusals on full and lite. The generation goldens, the core baseline and seeds.hashes move on purpose: update each and say in the PR what moved and why. The balance lock is re-baselined once, in session 09; until then say in each PR which locked figures moved and by how much. Add the new choices and pins to what the page reads (js/choices.js, js/pins.js) only where a request should be able to pin them.

The work, one candidate, one PR. The hardest of the batch: the lever did the chords and left the tune clashing (up to 47% of its time a semitone over a chord tone).

- planingSevenths (#10): an option of the progression's per-loop draw (today's first): every chord a major seventh, the roots a whole step or a minor third apart, the shape sliding rather than a progression in a key. Build the lever's other half: the tune follows the planed chord (its tones, the ninth, and the chord's own Lydian or Ionian scale), and so does anything else that reads the key (the bass follows the roots already; check the pedal, glass, the tuned kick, colorTone from session 06). Leaned toward the dreamy and floating vibes.

Yardstick: 80% or more of the chord time on major sevenths; the roots moving by a whole step or a minor third (count where the cycle turns back to the tonic honestly, as the batch did); the tune's time a semitone over a chord tone no higher than the same loop's today; notes stolen no more than the sketch's (four-note chords take more of the budget: say what it costs on lite); the bass's loudest window under 60 Hz within the low guard. If the tune can't be made to follow without rewriting how the melody is generated, stop and leave docs/notes/101f-planing.md with what you measured and the choices; don't ship the clash.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 07's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
