# Session 22: codes, version 2, in the core (105a)

**Changed by brain B1 (2026-11-11):** no app name in the frozen format (the app is renamed at 1.0: docs/DECISIONS.md, 2026-11-11); the wander's salt (item 92) added to what a code carries.

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** session 21 (everything 1.0's codes must carry is in by then: batch 6, 84b, the journeys). Session 16 should be merged first if Mikey's notes have arrived; it need not wait for them.
- **Queue item:** item 105.

Paste everything in the block as the session's first message.

````text
Hands session 22 on the road to Driftloom's 1.0 (Opus 5.5, high): codes, version 2, in the core (105a).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 105); core/AGENTS.md; docs/CORE.md (promise 8, and promise 11's "codes carry only what was pinned"); docs/PAINTER.md ("Share codes, version 2"); docs/DECISIONS.md ("Share codes, requests and exports", and 2026-11-10's two lines on codes); js/share.js (version 1, to be replaced); core/src/generator/request.rs (everything a request can say).

Check your place: in docs/sessions/README.md every row above session 22 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

The work, one PR, core and tools; the app keeps version 1 until session 23. Design it once, carefully: from 1.0 on these codes never change (Mikey, 2026-11-10).

- The code: a version, the seed, and only what was pinned or changed beside it (vibes and their weights, tempo, bars or Endless, the journey, key, mode, style, voices, layers, pins, a theme, and a code's edits: layers re-rolled, the tone, the tempo, mutes; and the wander's salt where a play in the slider's upper range carries one, item 92), nothing else. Plain ASCII, the 64 link-safe characters (A-Z a-z 0-9 - _). No app name in a code, not even a two-letter prefix like today's DL: the app's name may change, and codes can't after 1.0; a code says what it is by its own version character. A loop with nothing pinned: under about 20 characters (aim for promise 8's 10 to 16).
- Room to grow: everything is named by a stable id, never by its place in a list; ids and lengths are variable-width; so new instruments, profiles, vibes, styles, picks and later a modder's recipe fit without a new format.
- Additions leave old codes as they sound: every instrument, profile, vibe, option and choice records the version it joined (in data/), and a code's draws see only what existed at its version: each pool is filtered by version before the draw, so a newer member can't shift an older code's picks. A deliberate improvement to an existing sound still changes old codes; that is allowed and goes in release notes.
- A code that names something this build doesn't have, or a newer version, is refused in plain words ("needs a newer version of Driftloom"), never played wrong.
- An album: a list of loop codes, and an album drawn from one seed (how many tracks, the picks they share) as one short code.
- Encode and decode in the core (Rust, tested, exported for the web and in the C header), so the app, a game and the tools share one implementation.

Yardsticks (the queue's): a loop with nothing pinned has a code under about 20 characters; every pick, length, journey and edit round-trips (a test walks the request's whole list, as the pins' test does, and fuzzes a few thousand random requests); every generation golden's case written as a version 2 code and read back generates its same hashes; a test adds a dummy instrument, profile and vibe at a later version and shows every golden renders bit for bit the same while a new code can draw them; a code from a "newer" version, and one naming an unknown id, are refused plainly; corrupt and truncated codes never panic (fuzzed). Write the format down in core/README.md, small and exact.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 22's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
