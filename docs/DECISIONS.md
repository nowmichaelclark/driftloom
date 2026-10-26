# What Mikey has decided

Stated decisions, not inferences: one line each, grouped by topic, in the words
they were written down in. The date leads the line. "by 2026-10-16" means the
decision was recorded here by that day and its own date was never written down
(this file's history starts then); the date of a decision that has one is
Mikey's. Where a line is a short form of a longer entry, "the full text" links
to it in `docs/archive/decisions-reasoning.md`.

**How this file is kept.** A brain session adds each new decision as one dated
line under its topic and never rewrites an old one; a decision that is replaced
stays, marked superseded, with the date. Long reasoning goes to
`docs/archive/decisions-reasoning.md` and the line links to it. The size limit
(40 KB) is checked in CI.

## The music

- 2026-10-20 -- **Best sound wins, not how it used to be (Mikey, 2026-10-20):** "I just want whatever is best / sounds the best for the core and app going forward. I'm not married at all to how things used to be or operate or function." Today's behavior is a reference to measure against, not a thing to protect; his ears decide. (Level changes still go to his ears level-matched or labeled, since louder wins a quick listen.)
- by 2026-10-16 -- **The current balance is right.** Preserve it as things are added. Meandering, ambient loops alongside tuneful ones; not every loop a triumphant melody. The balance lock (`stats.mjs --check`) guards it; a deliberate change re-baselines and says so.
- by 2026-10-16 -- Whimsy in the Ocarina of Time sense, sometimes: catchy, childlike, hummable. Inspiration also from Wind Waker and Spirit Tracks.
- by 2026-10-16 -- **The north star for tone: pleasant** -- nothing that would bother people in a coffee shop. Harsh, buzzy or toy-like timbres fail whatever their level. Respectable through an aux cable at a live venue or in a car.
- by 2026-10-16 -- **Moods stay wholesome, never negative or depressing** (Animal Crossing, a nice coffee shop). In the spirit of Buddhadasa and Ajahn Dhammarato: feelings and sensations are textures, neither good nor bad. See `docs/MOODS.md`.
- by 2026-10-16 -- **Dynamic but sensible:** quiet loops and a spread across the catalog are wanted, but the listener shouldn't need the volume every other track. Fixed at the source, never by a limiter, compressor or normalization on the mix. (Solved in practice by the lead trims, #48 and #53. "Twenty in a row" is not needed for now.)
- by 2026-10-16 -- **One behavior for everybody:** no user-facing modes, toggles or sliders to scroll through.
- by 2026-10-16 -- **Loops with fewer layers are wanted variety**, not a bug, even drums over a texture alone.
- by 2026-10-16 -- **About twice the variety it had in September 2026**: some loops wider and deeper, very simple ones still possible at any length.
- by 2026-10-16 -- **The choir is a rarity** (about 1 loop in 30) that should feel special.
- by 2026-10-16 -- Fine with retro, strange, and occasionally wrong if it opens things up.
- by 2026-10-16 -- **6/8:** chords move on the dotted beats and hold; figures between the beats are welcome; an entry on a weak eighth tied over the beat is not. In the rhythm section he likes the old cross-rhythm (hints of 3/4 and 2/4 against the 6/8); only the chords wanted to be strict.
- by 2026-10-16 -- **Cinder's added F over the E chord stays; wayfare's minor v stays.**
- by 2026-10-16 -- **Composition depth:** some loops develop, simple ones stay (his favorite in the Depth album was a simple 24-bar loop). The share is a knob to tune by ear.
- 2026-10-09 -- **New instruments are welcome again** (2026-10-09; the 2026-09-25 pause on new sounds was temporary). Mikey liked the ears' first five as clips (stereo, roundBass, glass, floorDrop, arc: "all the new stuff is sounding really great"); build the ones he approves, in the core, except stereo (dropped 2026-10-11: mono only). Inspiration only from wholesome, comforting music, not harsh or dark records. No new *profiles* still. Fiddle, accordion and drone may get more nuance later. [Stereo came back on 2026-10-15; see "The engine and stereo".]
- 2026-09-27 -- **The playhead fade is gone for speed** (2026-09-27).

## The words and vibes

- 2026-09-28 -- 178 mood words of his own, in `docs/MOODS.md` (lively added 2026-09-28; candidates for more listed there). Near-synonyms may differ ("a happy song isn't the same as a merry song").
- 2026-09-27 -- Labels of one to five words, mostly two or three of different kinds; some loops one word fully embraced. Words steer generation, bending a profile within its character; how each word meets each profile is defined with him by ear.
- by 2026-10-16 -- **No pair is forbidden:** words that pull apart find their in-between, a shape over time (bouncy + floating is a helium balloon).
- 2026-09-28 -- A pilot dozen is agreed: merry, tender, serene, golden, twinkling, crisp, velvety, sour, bouncy, floating, swaying, quirky. Draft recipes in `docs/MOODS.md`; he loves most of the approach (2026-09-28).
- 2026-09-28 -- **Heart words replace the eight moods** (2026-09-28), eventually; the eight become heart words with recipes. Tender is close, soothing settling; serene still and wide, peaceful content and gently moving.
- by 2026-10-16 -- **Percentages stay only if honest:** measured from the rendered loop.
- by 2026-10-16 -- **Ten dials, five maps (option B), loops as clouds**, word regions fitted from his rankings (`docs/MOODS.md`). Open: which candidate words join.
- by 2026-10-16 -- **Listening tests don't ask him to describe tracks freely** (too abstract). The word-ranking test (queue item 19) is the tool.
- 2026-10-11 -- **Pool 3** (2026-10-11): 39 of the candidate words joined `/rank` (71 in all; `docs/MOODS.md`). He ranks in bulk and exports.
- 2026-10-11 -- **Reflective may be neutral** (2026-10-11): leaning brighter since #107 is okay, and it can also sit in the middle of the mode axis.
- 2026-10-11 -- **Cinder and shatter may take a bright mode when a word calls for one** (2026-10-11). Both built in queue item 36.
- 2026-10-11 -- **Word albums** (2026-10-11, his idea after 181 rankings): one word at a time, "Is this playlist heartwarming? Which songs are not?" The brain guesses loops for a word and mixes in a couple it expects not to fit. The five-word ranking rests; its answers stay the base for the fit.
- 2026-10-11 -- **Vibes in the app** (2026-10-11, top priority): the words are called vibes; a **Vibes** key on Now Playing opens a scrolling two-column list with Close and Clear All; pick any number; each new loop leans toward up to five of them, weights shifting loop to loop (one vibe alone at 50% or more). All 71 pool-3 words. A real feature, and the tuning tool: he picks vibes and sends loops that fit well or don't. Queue item 38; recipes in `docs/VIBES.md`. A Painter session can refine the look.
- by 2026-10-16 -- The old branches stay: he likes looking back on them.

## The engine and stereo

- 2026-10-10 -- **Driftloom has its own Rust audio engine, and copying Chromium is no longer the goal.** The port used Chromium as the reference so that every step could be proven; now that the whole sound path runs in the core, the core is the thing. Aim for what audiophiles, game developers and programmers who value efficient, easy-to-run software would favor: an app and an engine people marvel at for its beauty and commitment to excellence. Do things better than Chromium where better is possible, and in the way that fits Driftloom's own aims.
- 2026-10-10 -- What that means in practice: Chromium-parity work stops unless it serves a purpose (it still does as a safety net until the core is the default). Improvements are judged by measurement (level, cost, tone, determinism, performance on a phone) and by Mikey's ears, not by nulling against Chromium. Changes to sounds he already likes still go to blind A/Bs; the balance lock and the refusal checks still hold.
- 2026-10-10 -- **Candidates for the engine (brain, unranked; not a decision).** A warm compressor start (done in 31b), smooth curve and parameter changes, better oversampling and a true-peak ceiling, a reverb designed for this music, an offline renderer for audio export (roadmap 17), a documented host API for native and game hosts, the generator in Rust when the words settle. The list with its reasons: [the full text](archive/decisions-reasoning.md#engine-candidates-brain-2026-10-10).
- 2026-10-15 -- **Stereo across the board, with a mono switch (Mikey, 2026-10-15)** -- replaces the two lines below. Stereo everywhere by default; an optional switch makes everything mono for listeners and devs who like that (a settings exception he chose). Ears sessions propose stereo treatments as clips first; the core's stereo render, the mono switch and the first space recipes come after the instrument batch, before phase 2 (`docs/CORE.md`, promise 9).
- 2026-10-13 -- *(Superseded 2026-10-15.)* **Stereo possible, mono the default (Mikey, 2026-10-13).** The app stays mono for now; stereo comes much later, experimental first, and the core must make it implementable without a rebuild. Carefully: "stereo holds its own set of infinite possibilities". [the full text](archive/decisions-reasoning.md#stereo-possible-mono-the-default-superseded-2026-10-15).
- 2026-10-11 -- *(Superseded 2026-10-13.)* **Mono only, for now (Mikey, 2026-10-11).** The ears put stereo first; Mikey decided against it: mono gives a controlled soundscape that sounds the same from system to system, where stereo can sound wildly different. The output stays one channel. [the full text](archive/decisions-reasoning.md#mono-only-for-now-superseded-2026-10-13).

## The core: aims and design

- 2026-10-12 -- **Now is the time to get the core right:** the part users never see but developers do -- good for battery, fast, written beautifully, immaculate. "There are still a lot of things I may want to do and change with the app", so the core must make change easy, not freeze the app.
- 2026-10-12 -- **The website must never glitch from screen work.** Notes are decided and played away from the page's main thread, whatever the page is doing.
- 2026-10-12 -- **The generator's Rust port no longer waits for the words.** "The words will never fully settle, and I imagine I will still want to add more sounds or profiles." Vibes, profiles and voices stay easy to change (data where possible).
- 2026-10-12 -- **Anything may break** (Mikey, 2026-10-12): share codes, saved loops, albums. "Breaking is how we learn. I'm playful and experimental." The app stays in testing for a long time; even the F-Droid release is an alpha or beta. So the port is proven step by step for the brain's own safety, not to protect old codes.
- 2026-10-12 -- **The number one priority: an excellent, beautifully made core** (Mikey, 2026-10-12). "Shoot for the moon." Take the time it needs. So good and so well written that developers could put it in anything: any game engine, any game mod, any operating system, any kind of device.
- 2026-10-12 -- **Both: hand-written Rust building blocks, and instruments as small data recipes.** A seed or share code is a recipe, so "same seed, same music" holds only with the same core version and the same data (brain). **Mikey likes doing both** (2026-10-12): hand-written Rust building blocks, and instruments as small data recipes (patches) that wire and set them -- like a synth's presets. The code should at least say what it needs (core version, packs with hashes) and degrade clearly when something is missing. The brain's reasoning (instruments as data, a few hundred bytes, named by a hash): [the full text](archive/decisions-reasoning.md#a-seed-or-share-code-is-a-recipe-brain-2026-10-12).
- 2026-10-13 -- **The building blocks are open to change too** (Mikey, 2026-10-13): developers, AI agents and Mikey can add blocks or make fundamental changes to them easily. He doesn't know yet which blocks could be added or are worth changing: the brain drafts a catalogue of ideas for him before phase 4.
- 2026-10-13 -- **The JavaScript synth retires at the end of phase 3** (Mikey, 2026-10-13); earlier only on its own branch, merged when the replacement is ready. **The charter's targets stand**, to be revisited: the brain proposes changes, or higher aims, as measurements come in.
- 2026-10-13 -- **Bit-identical everywhere** (Mikey, 2026-10-13): the same code makes exactly the same samples on every device; the core carries its own math.
- 2026-10-12 -- **A new brain chat for the core.** Before it: one more bug fix if the Pixel 10a tester's report on #187 calls for one.

## Share codes, requests and exports

- 2026-10-12 -- **Shorter share codes** (Mikey, 2026-10-12): an album fills half a phone screen in Discord; not tidy. Brain's proposal for the charter: with a deterministic, versioned core, a loop's code is the version, the seed and only what the listener changed (vibes, mutes, edits) -- about 10-16 characters; an album, a list of those (about 60 for eight loops, against over a thousand now). Shared as a link, so Discord shows a tidy preview. The levers for shorter still, and why the version replaces explicit parameters: [the full text](archive/decisions-reasoning.md#shorter-share-codes-mikey-2026-10-12).
- 2026-10-12 -- **No online server for albums** (Mikey, 2026-10-12): codes carry everything. He'd like them shorter still.
- 2026-10-12 -- **Decided (Mikey, 2026-10-12): codes never need reading aloud, but must show correctly on almost any device of the last 30 years** (a DS, a PSP, an old computer, "maybe some wild dev will want to port the core to something like GameCube"). So **plain ASCII only**: no emoji, no other Unicode. The same rule for everything the core emits (names, codes, errors). Brain's recommendation: the 64 link-safe characters (`A-Z a-z 0-9 - _`), 6 bits each.
- 2026-10-13 -- **Requests (Mikey, 2026-10-13):** in the app, listeners choose more for the next loop -- bars, tempo, rhythm, scale, etc. -- alongside the vibes; developers ask the same way ("a peaceful, zany loop at 77 bpm, 16 bars, when the sun rises"). Charter promise 11: one request, each choice pinned, ranged or left to chance; the generator port (phase 2) is built around it. Picks like the vibes, not settings (brain's reading; the settings rule stands).
- 2026-10-13 -- **Longer loops (Mikey, 2026-10-13):** listeners and developers can pick far more bars than today's 32, e.g. 3,072. Charter promise 11: any length up to endless; the generator works section by section, so length costs nothing; long-form composition is later work by ear. Both kinds wanted: a symphony-like journey (3,072 came from Beethoven's Ninth) and long slow wandering. Decided: no cap in the core; the app's picker to 32,768 bars plus Endless.
- 2026-10-13 -- **Next picks, the design (Mikey, 2026-10-13):** chosen after four rounds and 70 options (Painter session 5); the brief is `design/next-picks.md` (it wins over the renders). Painter 6 (app track) builds the screens: moods wired to vibes; length and tempo to today's generator only if cheap, clamped. The real wiring is queue items 48-53 (phases 2 and 3). **Open, for Mikey:** (1) the way into New loops during an album; (2) how to set only length, or only tempo, back to "any". The description of the design and the two questions in full: [the full text](archive/decisions-reasoning.md#next-picks-the-design-mikey-2026-10-13).
- 2026-10-13 -- **Exports, safe first (Mikey, 2026-10-13):** never crash or break anything on the user's end, whatever the hardware, old Android and weak chips included; fast only where safe. He suggested "fast" or "lossless" as a toggle in settings; decided, from the brain's proposal (charter phase 7): format chosen per export, speed from the existing "lighter on processor" setting, so no new setting.

## Platforms and games

- 2026-10-12 -- **A native Android app (F-Droid) is the likely later step**, from the same Rust core, with the web screens inside it; the website stays, on the same core. Chromium is not a requirement for the website (it plays in Firefox); the test tools use it.
- 2026-10-12 -- **Platforms, each its own phase, after the core:** the native Android app on F-Droid; later Google Play; an **itch.io** desktop release for people who run it locally. **No iOS app for now** (Apple's yearly developer fee), but the core must make iOS and macOS easy if that changes. His first multi-platform app: the brain and hands carry as much of it as they can.
- 2026-10-24 -- **Broken Friends is the one game that proves the core** (Mikey, 2026-10-24): his own Godot game is "the one and the only test game we're going to use. I don't want to make music mods for those other games anymore anyways. Maybe another time." It replaces the three games of 2026-10-13 (Luanti, OpenRCT2, Stardew Valley) and takes phase 4. In the game's own plans it is "Driftloom's most important proof game" (its decision 0016). `docs/CORE.md`, promise 10.
- by 2026-10-16 -- **Games:** chill ones -- Animal Crossing, point-and-click, turn-based -- where music follows time of day, weather, place, season, turn. Not action. Roadmap 18.

## How the project runs

- 2026-09-23 -- **Old share codes, saved loops and albums may change or break** while the app is in testing (2026-09-23). Keep draws stable where it's free, only because it keeps A/B albums comparing the same loops.
- by 2026-10-16 -- **Merging before an A/B is fine**, and the hands merge their own PRs under the queue's policy. Production is testers only; merges can be reverted.
- by 2026-10-16 -- **Performance: excellence.** Light and fast even at full quality, "DHH-wow" smooth with a hundred other apps open.
- by 2026-10-16 -- **Baking loops for playback: no.** "Let the loop wander" is on by default and changes the loop every pass, and re-rolls must be instant. Baking lives on as an audio export (roadmap 17).
- 2026-10-03 -- **Rust:** the synth, then the generator, move to a host-agnostic Rust core (WebAssembly in an AudioWorklet for the web), in its own brain chat. **(Superseded 2026-10-12: the generator no longer waits; see "The core: aims and design".)** **Timing (2026-10-03): the synth ports now; the generator waits until the words' draws stop moving.** Its case is portability, not speed (the performance baseline showed the fixable costs were elsewhere). The deliverable is the phone app; a native build and games are separate projects. Rust over C++ and the rest: roadmap 16.

## Working style

- by 2026-10-16 -- Briefs ready to paste; concise answers; instructions step by step.
- by 2026-10-16 -- His ears are the scarce resource: batch listening, keep albums short, let the tools judge level, cost and tone.
- by 2026-10-16 -- Voices he likes: struck and sung -- kalimba, musicbox, marimba, hum, vowel since #14; the fiddle and accordion since take two.
