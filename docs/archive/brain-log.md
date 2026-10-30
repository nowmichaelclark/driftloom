# The brain's log and history

Everything the brain wrote down as it went, newest first (the log), and the
one-line history of the early PRs, as they stood in `docs/BRAIN.md` before
2026-10-26. A brain session ends by adding its own entry at the top of the
log. Nothing here is live; the live state is `docs/BRAIN.md`, "Start here".

## The log
**Session 26 (brain, Opus 5.5, 2026-10-26): the sound batch checked, the
albums, phase 2.** What the brain ran itself on `main` at v123 (a7eb64d), not
taken from the reports:
- **Goldens:** `build-core.sh --check` (the `.wasm` matches its source,
  246,992 bytes); `cargo test --release` (139 tests); x86-64 and wasm32, mono
  and stereo, `--strict` (x86-64 also `--flush`), `--blocks all` and
  `--rates`: all equal; `node tools/rates.mjs` passes. Every target's hash
  file, mono and stereo, equals wasm32's line for line, and CI on `main` is
  green on all five legs.
- **JavaScript:** the generator's tests three times and the other eight
  suites pass; the balance lock: all 22 figures within tolerance.
- **The core baseline:** `node tools/baseline.mjs` here: "Nothing moved" (loops 1280 figures, voices 245, refusals 26, costs 55, stereo 480, repeat 2), so the committed baseline is what `main` plays, mono and stereo, full and lite.
- **Item 72's big baseline moves** (shil-doask +7.6 LU, reisk-foa +4.6,
  their-roask +3.2): the reshuffle, not a bug. Every baseline loop's choices
  drawn at item 71 and at 72: those three drew lateFloor at 71 and not at 72
  (padSweep joining the list moves item 54's weighted draw), so at 71 their
  kit or bass sat out most of the baseline's 20 s; ri-houng gained lateFloor
  (-2.6 LU). Exact: the 29 loops whose choices didn't change are bit for bit
  unchanged, and the 11 that moved are exactly those whose choices changed.
- **Costs** (instructions, cachegrind, the native host; before = #244, the
  last item before the batch): the same old note lists on `main`'s core
  +1.4% (09-shrine) to +6.1% (07-dust, lite), mostly roundBass's harmonics
  on every bass note (item 56's note: 40-70% a bass note) plus the stereo
  lane's bookkeeping; today's lists, with the batch's sounds, +4.7%
  (15-wayfare), +6.3% (09-shrine), +18.0% (07-dust, which now draws more).
  Stereo against mono on today's lists 1.25-1.34 times, under 1.5.
- **The stereo guards** on six loops (glade #57, vapor #30, haven #56, thaw
  #82, grove #31, wayfare #23; 30 s, the brain's renders through the app's
  engine): fold 0.00 LU; side under mid below 120 Hz -28.6 dB or lower;
  correlation median 0.98-1.00, 5th percentile 0.93 or more; balance within
  0.12 dB. All pass.
- **Mikey's decisions along the way** (the hands' notes, items 55-73): read,
  each recorded in the archived notes and the items; nothing undone or left
  hanging. The open taste calls are in the album.
**The albums** (`docs/ALBUMS.md`, "Session 26"): three in-app codes (new
sounds, mixing ideas, flagged cases; each track's length set by `playFor`)
and four sets of clips (the bass at 0/+3/+5.5 dB, a blind stereo/mono A/B
with a null, the forced cases, and two moments deep into loops) on one
private page with a verdict database. The clips came from a scratch
renderer through the app's own engine on the core
(`docs/archive/session26-clips.md`), which reproduced the hands' lateFloor
figure for undertow #45. **With Mikey:** phase 2 takes the sequencer; the
port is proven note for note; glacial is vast and cool, its arc 5-8 minutes
(8-12 with a glacial-adjacent vibe). **Phase 2 written** (#275): items 77-92
in Broken Friends' order, 48-52 folded in, 74 before 79, 75 after 82, and
76b (glacial) before the port.

**Brain, 2026-10-25: what Broken Friends needs from the core** (read
from the game's repo, `plan/systems/audio-music`, decisions 0016, 0037,
0038, spike 2; for Session 26 to order phase 2's items around):
- **Phase 2 is the game's gate.** The extension plays golden note lists
  bit for bit (Linux 22-32x real time, browser about 19x), but new music
  waits for the generator in Rust. What the game will ask of it: a loop
  from a seed or a stored code (each friend's theme), steered by the
  friend, its mood, the time of day and camp events (promise 11's
  requests).
- **Music changes at moments, not on a timer** (decision 0038): a new
  room, a friend made well, a mood, surgery. So the core needs a cheap
  change of loop at a musical boundary, with a crossfade. Today a new
  list resets a core (about 15 ms on a laptop, more on a phone), too slow
  for the audio thread, so the game keeps two cores and switches. A
  loop change without a full reset, and a crossfade in the core, belong
  in phase 2 or 3 (the app's Next would use them too).
- **The browser has no audio thread** in Godot's web engine; music fed
  frame by frame crackled on Mikey's phone, so the game renders each
  loop whole and plays it as one sound. With the Rust generator (and
  wandering or endless loops) it will need loops rendered ahead pass by
  pass, quickly: render speed stays a gate (stereo costs 1.2-1.4 times
  mono).
- **Target phone: a Moto G Power (2022)** and weaker; the game uses the
  lite mix on phones and in the browser. Worth adding to promise 2's
  measured devices.
- **The game pins the core** (at f654fdb, before stereo and the space
  recipes); its six golden hashes need refreshing at each repin, and the
  host calls it uses changed with items 64-66 (layout, space). Its web
  build needs Emscripten with panic=abort, which the core's no-panic rule
  already serves; an Emscripten target in the core's CI is a phase 4
  thought.

**Broken Friends session (Opus 5.5, 2026-10-24), not a brain session.**
Mikey made Broken Friends the one game that proves the core (promise 10
rewritten; phases 4 and 7; "Answered"; Painter section 9). That
session is building the Godot extension in the game's repo
(`rust/driftloom-godot/`), checked against the golden hashes, and
testing Godot's web build with extensions; its results will come back
here as notes, not as queue items.

**Brain, 2026-10-19 (Session 22, continued), after item 46b.** #241
read (its report and notes; the figures not rerun): the stereo shape is a
second signal, the mono render untouched (same `.wasm` bytes without the
stereo code), demonstrations test-only, the baseline green in CI. Its
notes showed four batch items assuming seams 46b did not build (per-loop
settings and modulation on channel blocks, repeated retunes, sidechains):
queued as item 46c, first in Session 25a.

**Session 24 (brain, Opus 5.5, 2026-10-19), phase 1 review and the sound
batch.** Item 46 (#238) checked against its yardsticks, run here, not
taken from the report: `build-core.sh --check`; `cargo test --release`
(101 library tests with the kit and its cost check, and the golden, draw
and math tests); goldens `--strict` on x86-64 and wasm32, `--strict
--flush`, `--blocks all` on both, `--rates` on both: all equal, and #238
touched no hash file; `node tools/baseline.mjs`: "Nothing moved";
cachegrind against #237: busy-cinder -0.37%, 09-shrine -0.05%,
15-wayfare -0.11%, 07-dust +0.34% (the report's figures); the release
library's assembly: no panic, trap or allocation; `js/core.js`: comments
only; pedantic clippy 405 left, as reported. "One file plus one line"
checked independently: a throwaway processor added in a scratch copy
(one file, one registry line) was found by the kit, asked for its stored
render, passed once stored, and a 2% gain change was flagged as a sound
change. **Phase 1 is done.** Then, at Mikey's request: **item 46b**
(Session 25-0, Opus 5.5, high): a modulation input, the stereo signal
shape and channel events (with a channel's own blocks), a refactor like
46, so the batch builds on one way of doing each; **the sound batch** as
items 54-73 under three groups: (a) the per-loop choices (54), the frame
drum's solo peak with a -1 dBFS voice ceiling in the kit (55), roundBass,
longChords, floorDrop, glass, dubStab, the chapel organ, the wash, the
arc (56-63); (b) the stereo render, the approved baseline as space
recipes, the Mono switch (64-66); (c) tunedKick, subDuck, lateFloor,
floorDrop shapes, pedalBass, padSweep, echoTime (67-73); item 74, the
clippy cleanup, low priority. The wander note carried into the charter's
phase 2 and "Known issues". **Brain's calls, written in the queue:** the
per-loop choices go in one framework first (54), so every later item
only adds an option; the new parts count toward Mikey's "one or two
changes a loop"; the core's stereo master links its compressors (one
gain from the mono detector) so stereo folds like mono; the arc starts
bright. Phase 2's items wait for Session 26. Docs only.

**Brain, 2026-10-19 (Session 22, continued), after item 46.** Read
#238's report and notes (not its figures; Session 24 checks those).
Nothing to redo. Advised, and Mikey agreed: item 46b (modulation input
and the stereo signal shape, bit-identical) before the sound batch, so
the new instruments aren't built twice; a clippy cleanup later; the
frame drum's solo peak into group (a). Written into "Start here".

**Session 22 (brain, Opus 5.5, 2026-10-18), phase 1 check.** Items
42-45 (#204, #208, #211, #213) checked against their yardsticks; all
pass. What the brain ran itself, not taken from the reports:
- **Goldens:** native x86-64, `tools/golden.sh --strict`, `--strict
  --flush`, `--rates`, `--blocks all`: 21 of 21 equal to wasm32's, the
  same with flush-to-zero, the 12 renders at 22.05-192 kHz match, the
  speaker output the same at every block size. The five targets' hash
  files are line for line identical, and CI on main (run 39123172239)
  is green on all five legs with `--strict`.
- **The moved draws (#211):** every `Math.random` in `js/synth.js` and
  `js/engine.js` before the PR accounted for (moved into the core, moved
  to `js/draw.js`, or left in the JavaScript synth, which is the
  fallback); each slot the core fills matches what the page sent before,
  and the formulas and ranges match (the sung voices' too). Checked that
  no two notes on a channel share a key where a draw is made (21
  goldens: none; the only shared keys are swell textures, which draw
  nothing), so a chord's notes don't move in lockstep.
- **Costs, which #213 did not report:** instruction counts (native,
  cachegrind) for busy-cinder, 09-shrine (choir) and 15-wayfare at items
  43, 44 and 45: within +-0.25% step to step, except the choir 1.1%
  cheaper at 44 (no arena). #208's three loops over 5% (+5.0 to +7.4%
  in wasm instructions) were reported with the reason (the silence
  floors, code layout); accepted.
- Not rerun: `tools/baseline.mjs` (no Chromium in this container; CI
  runs it and it is green on main).
**Notes for later (Session 24):** existing files carry hundreds of
pedantic clippy warnings (lib.rs 81, wave.rs 69, timeline.rs 68, #211),
so the "clean in every file touched" rule can't be met; a cleanup item
belongs before or with 46's refactor. 45's other rates were hashed on
three goldens only (13-dust, 16-grove, 18-shatter), which is enough
for now. **For Mikey, small:** since 44, "Let the loop wander" is
drawn from the loop's code, so a saved loop wanders the same way every
time it is played. **Mikey pushed back (2026-10-18):** at the top of the
wander slider a loop should wander in an experimental, non-repeatable
way; across the slider's whole upper range (his decision, the
boundary tuned by ear as we go). Not a fix now; filed under "Known
issues" in `docs/QUEUE.md`, for phase 2's generator port.
**Session 22 closed** (2026-10-18), with #237 for Mikey to merge.
**The control album, "Phase 1 check"** (`docs/ALBUMS.md`): blind A/B,
v92 (PR #204's preview) against v99 (PR #236's, which is main), nine
corpus loops covering every kind of moved draw, one pass each, and a
null track that is bit-identical on both. **Heard the same evening:
"everything sounds the same on both versions"** -- phase 1 changed
nothing he can hear, as intended. Next: Session 23 (hands, Opus 5.5,
high), item 46.

**Brain, 2026-10-13 evening (Opus 5.5), next picks.** Recorded Mikey's
choice of the next-picks design (Painter 5) under "The core"; queued
the real wiring as items 48-53 inside phases 2 and 3 (request and
tempo; length to 32,768 and Endless, section by section; key and
scale; style; voices and layers; the picks as core commands). Carried
the brief's two open questions to Mikey. Docs only.

**Session 18 (brain, Opus 5.5, 2026-10-13), the core.** Drafted the
core charter, `docs/CORE.md` (PR #196, draft): promises each with a
measure, phases, questions for Mikey. His answers (2026-10-13) added
promise 9 (stereo possible, mono default) and open building blocks;
then bit-identical everywhere (yes) and three games (promise 10).
**Approved by Mikey (2026-10-13).** Phase 1 queued as items 42-46
(golden renders on five targets; the core's own math and `no_std`;
randomness into the core; any rate, block size and channel layout; the
building-block interface). Hands: Session 19, Opus 5.5, high.

**Session 16 (brain, 2026-10-11 to 10-12), the words, then the glitches.**
Its State bullets, newest first, kept here as written:

- **2026-10-12, late: item 41 (#187, v92)** -- notes now planned from the
  phone's measured render-ahead, late notes past 30 ms dropped rather than
  bunched, covers drawn in a Worker. The 1.3-1.8 s stalls on the Pixel
  10a were **not reproduced** (the covers were already on CPU canvases;
  the brain's readback lead was wrong); the tester's next Diagnostics will
  say more. **Next: a new brain chat for the core** (see "The core");
  before it, one more fix if that report calls for one.
- **2026-10-12, evening: queue item 41 (urgent).** The Pixel 10a
  tester's Diagnostics on v91: late notes even with no stall (the Sound
  view: 67 ms gap, 78 late), and Library / Save stalls of 1.3-1.8 s
  against a 2 s queue on a phone with ~0.6 s of output buffer. Brain's
  reading: notes handed over inside the device's render-ahead, and
  software graphics making blur and canvas covers slow; the harness
  modelled neither. Asked Mikey for the tester's `chrome://gpu`.
  Hands: Opus 5.5, high.
- **2026-10-12, afternoon:** items 39 (#180, v89) and 40 (#182, v90)
  merged; both wait on the GrapheneOS testers' Diagnostics (the "while
  hidden" and "views" sections; steps in the PRs). Item 40's cause was the
  page's own work (Library drew every cover at once, 1-2 s on a slow
  phone) against a 0.3 s queue; the core now gets 2 s visible, 8 s hidden.
  The brain strengthened the vibes' profile and instrument leans (#183,
  v91; `docs/VIBES.md`, tuning log). Nothing queued for the hands; 37 on
  hold. Next: Mikey picks vibes and sends loops; the testers' reports.
- **Urgent (2026-10-12): queue item 40**, glitches with the screen on,
  worst on Library and Save / Share (second tester, Pixel 10a,
  GrapheneOS). Lead: visible, the core still gets notes only 0.3 s ahead,
  and those views are the heaviest on the main thread; or the audio
  thread itself starves. Item 39 merged (#180, v89): the hidden lookahead
  on the core is 8 s and Pause / Next / Previous cancel queued notes;
  waiting on the first tester's Diagnostics. Hands: Opus 5.5, high.
- **Emergency (2026-10-12): queue item 39**, the music skips when a
  tester's phone is locked (Pixel 9, GrapheneOS, installed app). The
  brain's lead from the code: hidden, the core still gets each note only
  a few tenths of a second ahead (the JavaScript synth's build window).
  Hands: Opus 5.5, high. Mikey heard item 36's tracks (#177): "everything
  is sounding great."
- **Next: Vibes (queue item 38), Mikey's top priority.** The words go
  into the app as vibes (see "The words"); recipes for all 71 drafted in
  `docs/VIBES.md`. Hands: **Session 17, Opus 5.5, high**, 38a then 38b;
  Mikey attaches his two screenshots. Then items 34-36 (Sonnet 5.5 can
  take 36; 34-35 Opus); 37 is on hold.
- **The pilot word albums worked** (`docs/MOODS.md`): about half of the
  brain's guesses fit words that fit almost no random loop. Their
  verdicts went into the heartwarming and joyful recipes.
- **For the next words brain** (after 38 merges): read "Getting
  oriented", "What Mikey has decided", this State, ROADMAP.md item 19,
  `docs/MOODS.md` and `docs/VIBES.md`; skip QUEUE.md's history. Check 38
  against its brief; run `stats.mjs --vibe-each` and fix any vibe that
  does nothing or the wrong thing; then tune from the loops Mikey sends
  (codes carry their vibes), editing `js/vibes.js` directly. Item 36's
  album from its PR's list when it lands.


`main` at **v68**, deployed at the new address (see Session 5, below).
No open PRs. Nothing is queued. Item 20
(#122, v66) and items 21-22 (#126 v67, #127 v68, Session 4) are merged
and brain-verified (below).

As of 2026-09-28: queue items 0-17 done (13
stopped at its gate, by design); item 18, the housekeeping pass over
code comments and the README, merged as #111 (brain check: tests pass,
`--check` holds, `stats.mjs` output byte-identical). Nothing is queued. The
brain's document audit (2026-09-28) rewrote this file, updated the
roadmap's statuses (items 3, 9, 13, 14, 16, new 20 and 21) and the
README (profiles, rough edges, `--retire`, `--profile`, a project
documents list).

Since the last full State: the performance wins (#96 paint, #100 voices
let go, #102 hidden-tab build window, #103 always-on chain costed, #104
page weight, #105 fade dropped), the temple bell as a chord voice lifted
1.9 LU (#106), moods steering the mode with the vocabulary as data in
`js/moods.js` (#107: loops 90%+ happy in minor-ish modes 38% -> about
20%), and composition depth's second pass (#108: 16-bar developing loops
8.8 distinct melody bars against 6.7 simple). Brain check on v64: tests
and `--check` pass.

**Open decisions for Mikey:**
- **32-bar micro-loops** (three in four 32-bar loops) never develop, by
  design; developing them would be a new kind of form. Say if wanted.
- **Reflective leans brighter** since #107; is that right?
- **Cinder and shatter can't sound bright**: no bright modes in their
  pools. Part of roadmap 19.
- The ocarina over drums: ease back 2-3 dB if it comes up buried again.
- **Short loops' gaps** (from #111): `genGaps` draws at every length, so
  about a third of 2- and 4-bar loops take a gap and 7-8% a whole bar of
  silence (half a 2-bar loop). Keep, keep only beat-long ones, or none?

**Out for listening:** `docs/ALBUMS.md` -- "Depth two" (goes somewhere /
loops / too busy). "Moods" was retired unheard (2026-09-28).

**Sessions (Mikey, 2026-10-03).** Every session gets a label, one
counter for brain and hands: Session 1 is the words brain chat
(2026-09-28 to 10-03). For each next session, tell Mikey: brain or a new
hands session, which model, which effort, and its label. Defaults: brain
chats on Opus 5.5; hands on Sonnet 5.5 at medium effort for items with a
clear spec, Opus 5.5 at high for hard or judgment-heavy ones (DSP
parity, first prototypes). Next labels: Session 2, hands, queue item 20;
Session 3, brain, the Rust port.

**Words session (Session 1, 2026-09-28 to 10-03, closed):** State checked against GitHub;
item 18 (#111) verified. Decided this session (details in `docs/MOODS.md`):
lively joins (178 words); tender/soothing and serene/peaceful told apart;
heart words will replace the eight moods; percentages stay only if honest;
ten dials, paired as option B into five maps; a loop measured as a cloud,
with each word's region fitted from Mikey's own answers. **American
English** for the project (docs converted; identifiers left alone).
Free description and most/least albums were too slow and abstract, so the
words get a **ranking test** instead: queue item 19 (`rank.html`), for the
hands: merged as #117 (v65), brain-verified (tests pass, `--check`
holds, app untouched but the stamps; live page matches the repo). It
lives at https://driftloom.nowmichaelclark.workers.dev/rank
(`/rank.html` redirects there). The Moods album is retired; Depth two is
still out. Next: Mikey does a first batch of rankings, exports, and the
brain reads them with `tools/ranks.mjs`. Candidate words later (he
likes about half).

**Rust session (Session 3, brain, 2026-10-03, in progress):** State
checked against GitHub (drift fixed here: v65, the Rust timing line,
the retired album). Feasibility settled -- see roadmap 16, "Feasibility":
the hands' sandbox can build Rust, Cloudflare's build image can't without
installing it on every build, and the decision is to **commit the built
`.wasm`**, built by a pinned toolchain, with a check that it
matches its source. Mikey said go: queue items 21 (the pipeline and
kalimba) and 22 (fiddle and pad, the performance A/B) are written. Next
label: **Session 4, hands, Opus 5.5 at high effort**, items 21-22. After
21 merges, the brain checks the `?engine=rust` preview and sends Mikey a
few kalimba loops to try on his phone; after 22, a blind X/Y album.

**Rust core, items 21-22 verified (brain, 2026-10-05).** On a third
machine, the pinned toolchain (1.99.0) rebuilt `js/dlcore.wasm`
byte for byte (41,832 bytes); 32 core tests, the generator tests three
times and `--check` (22 of 22) pass. `measure.mjs --null` reproduces the
hands' figures: notes null at -95 to -119 dB, core layers in whole loops
at -105 to -123 dB, and the mix sits at Chromium's own JS-against-JS
floor (about -86 dB), with no late notes or fallbacks. The live site
serves the same `.wasm` (`application/wasm`) and 404s the crate's
source. Performance, from the hands: audio thread about 10-20% busier
with Rust, main thread the same or lighter, 4x fewer Web Audio nodes.

**The album is labeled, not blind (a change from the plan):** the engine
is a URL flag Mikey can see, and Diagnostics names it, so a blind X/Y
isn't possible without a hidden toggle. On Chromium a blind test would
find nothing anyway (the residual is float rounding). The open question
is his phone: on iPhone, the JS voices are rendered by WebKit while the
core copies Chromium, so the two engines could differ audibly there --
the case for a hidden toggle and a real blind test. Asked Mikey which
phone and browser he listens on.

**Rust check heard (2026-10-07):** Mikey played the album with and
without `?engine=rust` in **Firefox on desktop (Fedora)** and heard no
difference. That is a cross-engine result: Firefox renders the JS voices
with its own Web Audio, while the core copies Chromium's, so the two
engines agreeing there is good evidence the port holds outside Chrome.
iPhone/WebKit is still unheard; a hidden-toggle blind test stays optional.

**Account move (planned 2026-10-07, done 2026-10-08):** Mikey is moving Driftloom from
the Kruu-Mikey GitHub and Claude accounts to his nowmichaelclark ones.
Nothing is queued and no PR is open, so it's a clean point to move.
After the move, the repo URL, the brain's token and possibly the live
URL change; the next brain fixes the links (README, this file, the
Done lines) once Mikey confirms the new addresses. Old preview links in
QUEUE.md's Done list keep working only if the Cloudflare account stays.

**Session 5 (brain, 2026-10-08), the first on the new account.** State
checked against GitHub and the live site: the repo arrived with its
history (#129 the last merge, no open PRs, Workers Builds green on
main); the live site serves v68 and `/rank` works. The old site's saved
songs and ranking answers stay behind, and Mikey is fine with that. The
handoff file from the old account's memory is in the Project's
knowledge; what it held that this file lacked (the variety goal) is now
under the decisions. This session, as a docs PR: links moved to the new
repo and site (README, this file; the Done list's old preview links are
history and stay), the commit identity switched, and the README trimmed
to a front page -- what Driftloom is, the live link, how to use and run
it, Profiles, the documents list -- with the engineering sections moved
unchanged into `docs/NOTES.md` (one relative link fixed so it still
resolves). The intro now names the Rust core behind `?engine=rust`, and
the British spellings left in the README and NOTES are American now.
Its PR (#130) was the first build on the new Cloudflare account, and
the preview failed. The new Cloudflare account builds PR branches as
**Previews** (beta, "Previews Base" in Settings -> Build), whose command
is `npx wrangler preview` and which needs a `previews` block in
`wrangler.jsonc` (added in this PR; config, not code, so the brain made
it). `npx wrangler versions upload` doesn't work there: the preview
build runs as a different Worker and rejects the config's name. Each
Preview deploy gets a **Deployment URL** that "never changes"
(`<deployment-id>-driftloom.nowmichaelclark.workers.dev`): use it as the
commit preview for A/Bs. The branch's Preview URL moves with each push.
Limits: 100 deployments per Preview, 100 Previews per Worker on the free
plan, oldest pruned first. Production stays `npx wrangler deploy`.
**Each Preview keeps its own copy of the build settings**, taken from
Previews Base when its branch first builds: fixing Base doesn't fix a
Preview that already exists (a typo'd command broke `brain-rust-6`'s
until Mikey fixed it in that Preview's own settings). When a branch's
preview fails and Base looks right, check that branch's Preview.

**Next (Mikey, 2026-10-08): the Rust port first.** The brain wrote
queue items 23-28 (every remaining voice, behind `?engine=rust`), from a
read-only survey of `synth.js` done by two Sonnet subagents and
spot-checked by the brain. Mikey's note: Sonnet 5.5 and Haiku 5.5 (new
that day) are available, and the brain chooses models and effort. The
sessions:
- **Session 6:** hands, **Sonnet 5.5, high effort**: items 23-24 (the FM
  voices, sine, tubular; the wave-table voices). Existing building
  blocks and Chromium's own wave formulas; high effort because parity is
  exact.
- **Session 7:** hands, **Opus 5.5, high effort**: items 25-26 (bodies
  and nylon's damp; bandpass, highpass and the noise source). Chromium
  behavior to read and match: judgment-heavy.
- **Session 8:** hands, **Sonnet 5.5, high effort**: item 27 (bass,
  textures, drums; five channels; the performance A/B). Routine by then,
  but many voices.
- **Session 9:** hands, **Opus 5.5, high effort**: item 28 (the sung
  voices).
- **Brain checks between sessions.** The brain verifies each merged item
  from GitHub, as before, and hands the mechanical reruns (the `.wasm`
  byte check, tests three times, `--check`, the null and equivalence
  runs) to **Haiku 5.5** subagents, reviewing their figures itself; a
  Sonnet subagent reads diffs against the briefs. Haiku starts on these
  low-risk reruns until it has a track record.
- **Listening is batched:** one labeled album after item 28 across the
  new voices, unless a null comes out shallow enough to be worth
  hearing sooner.
**Session 6 (hands, Sonnet 5.5, high) merged items 23 (#133, v69) and
24 (#134, v70); brain-verified 2026-10-08.** A Sonnet subagent read
both diffs against the briefs: no bugs; flag-off paths, draw order and
count (tubular's five, whistle's slide), budget costs and fallbacks
unchanged; voice constants live in Rust; the 0.2 s filter tail (a fix
the hands found: the core freed the voice before the resonant lowpass
had rung out) only on filtered voices. A Haiku subagent reran: generator
tests three times pass, `--check` holds (22 of 22), and `--null` over
all 18 ported voices reproduces the hands' figures (single notes -85 to
-121 dB worst note; loops at the JS-against-JS floor, late 0). The
`.wasm` byte check and `cargo test` could not run here: **this
container's proxy refuses `static.rust-lang.org`** (policy, not a
glitch), so the pinned 1.99.0 toolchain can't be installed; GitHub
Actions' `core` job ran both, green on #133 and #134, and that is the
independent machine for now. Found while verifying: the Rust per-voice
probe over many voices runs Chromium out of memory and hangs -- a
harness fix put in front of item 25. Haiku 5.5's first job: accurate
and candid (it reported the toolchain failure and the stall plainly),
but slow (about 100 minutes, mostly the stall); fine for reruns.
Wasm 55,229 bytes; core memory 7.08 MB (the hands' figure).

**Session 7 (hands, Opus 5.5, high) merged items 25 (#136, v71) and
26 (#137, v72); brain-verified 2026-10-08.** Harness fix first: Chromium
never frees an offline context that loaded a worklet module (about
1.6 MB a render), so `measure.mjs` now splits Rust and `--null` runs
across fresh page loads and exits non-zero when the renderer crashes;
the app can't leak this way (it never renders offline; 200 cores built
and disposed on one live context stayed flat). A Sonnet subagent read
both diffs: no bugs; flag-off paths, draw orders (accordion, the winds,
panflute, temple bell, prepared's knock) and budgets unchanged; nylon's
damp handled ahead, live and late; Chromium's "a setTarget starts from
the last rendered value" rule (`reset_at`) applied only to the new
voices, so earlier ones render as before. A Haiku subagent reran:
generator tests three times, `--check` 22 of 22, `--null` over the new
voices (noise source -146 dB; voices -99 to -131 dB; loops at the floor,
late 0), and the per-voice probe through the core, which now finishes.
CI's `core` job (byte check, core tests) green on both. Wasm 81,796
bytes; core memory 10.9 MB. Haiku flagged its two probe runs (Rust and
JS) as byte-identical as suspicious; the brain's reading: expected,
since the tool prints levels to 0.1 LU and the engines differ by about
-100 dB.

**The one-frame click** (found by the hands in #137, traced by the
brain's Sonnet subagent from the code): a real defect in the
JavaScript synth, on `main` today. A noise note on the bare grid at
certain tempos plays one sample through a gain still at 1. Queued as
**item 26b**, a fix in both engines, before the drums are ported.

**Session 8 (hands, Sonnet 5.5, high) merged 26b (#139, v73), 27a
(#140, v74) and 27b (#141, v75); brain-verified 2026-10-09.** 26b: a
`--clicks` probe found 26 voices clicking on `main` and none after, on
both engines; 1.4% of noise starts landed in the window (215 of 15,340
in 200 loops). 27a/27b: bass, textures and drums play from the core,
which serves all five channels; #140 also made every core parameter
compare event times in frames (a loop at 11.3 s nulled at -38 dB
before). Performance A/B, four loops with full kits: late ticks 0 both
sides, audio thread 4-5% busier, main thread 9-13% lighter, node
creation 110/s -> 0. Wasm 106,458 bytes; core memory 10.4 MB. Only the
sung voices remain in JavaScript (not counted as fallbacks in
Diagnostics). Brain's check: a Sonnet subagent read all three diffs
(26b touched exactly the noise-fed gains, nothing else; flag-off paths,
draws, budgets, ducking and the drums channel unchanged); a Haiku
subagent reran tests three times, `--check`, `--clicks` on both engines
(no clicks) and `--null` over every new voice (-95 to -147 dB; loops at
the floor, late 0, fallback 0) and over the earlier voices. Three small
problems found, queued as **item 27c**: 26b's `value` setter misbehaves
for notes built late (offline proofs can't see it); the jingle draws
its zils twice on a fallback; pan flute grace notes regressed to a
-68.5 dBFS worst sample (was -125 at v72).

**Session 9 (hands, Opus 5.5, high) merged 27c (#143, v76) and 28
(#144, v77); brain-verified 2026-10-09. Every voice now plays from the
Rust core behind `?engine=rust`.** 27c: the -68.5 dBFS pan flute was a
core bug since item 21, not 26b or 27a: Chromium times sources to
1/1024 of a frame, the core used whole frames, so notes starting or
stopping a hair past a frame (about 1 in 2000) came out wrong in every
voice; fixed, and `--null` now starts one note in five in that window.
The 26b setter did not cut late notes in Chromium (it moves a late
note's past events to the present), but the safer form went in anyway,
for WebKit. The jingle draws 7 on a fallback. 28: vowel, hum and choir;
each singer's detune holds 13.5 s (longest sung note in 30,000 loops:
10.2 s); the formant trim is computed in JavaScript and sent (a native
host needs it ported, about 40 lines). Wasm 121,209 bytes; core memory
12.44 MB. Brain's check: a Sonnet subagent read both diffs (no bugs; the
source-timing fix goes through one `schedule` for every voice; every
`_core*` call site checks the channel before drawing; the sung draws
match in order and count, and a note over 13.5 s falls back before
drawing); a Haiku subagent reran tests three times, `--check`, `--null`
over every voice (worst note -84 dB or deeper, worst sample -89 dBFS;
12 loops late 0, fallback 0) and `--clicks` on both engines (none).
Small, unverified, for later: the core's sung step count isn't clamped
to the draws it got (only a corrupt message could exploit it); a sung
note's draws (up to about 7 KB) are cloned per message, where a
transferred `Float64Array` would avoid garbage on the audio thread; no
test plays a 13.5 s three-singer note.

**Out for listening: "Rust, all voices"** (`docs/ALBUMS.md`), ten
loops, each played with and without the flag.

**Mikey, 2026-10-09: move the rest to Rust now** (the album waits for
his evening). The brain wrote queue items 29-31: the channels, sends,
duck, echo and reverb (29); wobble, saturator with Chromium's 2x
oversampling, tone and highpass (30); both DynamicsCompressors, master
and kill, the core's output straight to the destination (31). JS
fallback notes reach the core's effects through one worklet input per
channel. The generator's port still waits on the words (his 2026-10-03
decision), unless he says otherwise. Sessions: **Session 10, hands,
Opus 5.5, high: items 29-30; Session 11, hands, Opus 5.5, high: item
31** (Chromium's compressor kernel and oversampling filters are the
judgment-heavy parts). The brain verifies between them, as before.

**Session 10 (2026-10-09): item 29 merged (#147, v78); item 30 (#148)
done but left unmerged under the block's performance rule.** 29: the
core mixes (channels, sends, duck, echo, reverb, tails); mix stage
nulls at -143 to -175 dB, loops at the floor; flag off byte-identical;
main thread 8-12% lighter, no fill-ins. 30: wobble, saturator with
Chromium's 2x oversampling, tone, highpass; nulls at -77 to -86 dB
before the compressor (JS against JS -78 to -104 there: last-bit LFO
differences moving the wobble's read by a fraction of a sample), loops
at the floor. The blocker: device fill-ins in 3 of 40 runs on item 30's
core, none in 40 on item 29's core or JavaScript. **Brain's call:** 3
against 0 in 40 runs each is within chance (one-sided p about 0.12), and
the runs predate the fix for the LFOs' start-up catch-up, so not yet a
finding either way. Session 10 continues: (1) the worklet fix
(Chromium sometimes hands `process()` the previous block's
`currentFrame`; the core then renders that block 128 frames late; `main`
is exposed too) lands now in its own PR; (2) the 256-tap downsampler
becomes an FFT convolution, as Chromium does it (cheaper, and closer to
Chromium's rounding); (3) a longer interleaved A/B, item 29's core
against item 30's, with each fill-in's time and the worst `process()`
block; merge if item 30's fill-in rate is no worse, else stop with the
figures. Mikey also plays the #148 preview on his phone.

**The rerun (Session 10, 2026-10-09 evening).** The worklet fix landed
on its own (#150, v79). The downsampler is now an FFT convolution
(half the cost; re-null unchanged). The interleaved A/B, 60 runs a side:
fill-ins in 5 runs on item 30's core against 1 on item 29's; pooled
with the first A/B, 8 of 100 against 1 of 100 (p about 0.017). Item
30's runs also have far more long render quanta (90th percentile 28 ms
against 11; 7 runs over 20 ms against 1), four of its fill-in runs had a
38-71 ms quantum, and it happens on lite too, where item 30 costs *less*
than item 29 on average. So: real, and not compute cost. Something
stalls the audio thread now and then. **Brain's call:** find the stall
before deciding anything (suspects: garbage collection in the worklet's
scope, a message or allocation item 30 added, nodes of the old JS chain
still pulling or running after it's cut off, `memory.grow`); use runs
with a quantum over 20 ms as the measure (seven times commoner than
fill-ins, so a bisect needs far fewer runs). #148 stays open.

**The stall hunt (Session 10, 2026-10-10).** Not item 30's code. Traces
(V8 GC, wasm, per-node Web Audio) put every quantum over 15 ms in one of
two kinds in *both* builds: off the CPU (15-322 ms of wall time for
0.5-20 ms of CPU), or charged as CPU wherever the thread happened to be
(item 29 inside native DelayNode and gains, item 30 inside its script,
once inside Chromium's topology check). During one, a separate sampler
process went 115 ms without a tick: the whole VM stopped. The core's
`process()` takes 0.15-0.4 ms; no allocator, fixed memory, GCs under
0.5 ms, the old JS chain doesn't render once cut off, page faults and
context switches the same in both builds. **Slot order mattered:** with
item 30 run first, item 29 had more long quanta; the earlier A/Bs put
item 30 second every time, so their "8 of 100 against 1" was partly the
slot, and the brain read it too strongly. Counterbalanced, all 120 runs
a side: fill-ins 5 (item 29) against 4 (item 30); quanta over 20 ms 3
against 8 (p about 0.11). **Brain's call (2026-10-10): merge #148.**
The stalls are the test machine's, item 30 adds nothing that could
cause one, and fill-ins are even; the flag is testers-only and the merge
revertible. The phone is the real test (the "Rust, all voices" album,
then the item-31 album). **For future A/Bs:** counterbalance the slot
order, and use `perf.mjs --trace-dir/--trace-over` to look inside any
long quantum before blaming code; the sandbox VM freezes now and then.

**Session 10 merged #148 (item 30, v80); Session 11 merged item 31
(#156, v81). Brain-verified 2026-10-10: with `?engine=rust` the whole
sound path runs in the core** -- voices, mix, wobble, saturator, both
compressors (ported from Chromium 141's DynamicsCompressorKernel),
master and kill -- and the worklet goes straight to the speakers; Web
Audio carries only fallback notes. Live audio nodes 62 instead of about
1,280; audio thread lighter than item 30's; fill-ins even against item
30 (counterbalanced). Stereo-ready (per-channel state, linked
detection), still mono. Wasm 157,544 bytes; core memory 14.50 MB. A
Sonnet subagent read #150, #148 and #156: no bugs; flag off unchanged;
the JS chain cut on attach, fallback notes reach the core through its
inputs, a load failure restores the JS graph; every runtime method
carries the same automation. A Haiku subagent reran: tests three times,
`--check`, the full `--null` (loops -85 to -87 dB at the JS floor, late
and fallback 0, the ceiling pressed in the loud set; mix and chain
stages -77 to -94 dB beside their own JS floors), `--clicks` and
`--endings` on the core (clean). Hardening the review found, none seen
in a render, queued as **item 31b**: a core that traps while playing
goes silent instead of falling back; Diagnostics after a load failure;
#150's forward-only frame count; the LFO catch-up in one block; the
handover re-glide.

**Open for Mikey: the compressors' cold start.** Chromium's compressors
start with an empty detector, so the first tenth of a second of music
after first Play (and after every rebuild, e.g. a quality change) can
dip by up to about 10 dB for a few tenths of a second. The JavaScript
app has always done this; the core copies it, and with the flag on it
happens when the core arrives (a core arriving mid-music dips again). A
warm start is one line in the core; in JavaScript it needs a short,
inaudible signal into both compressors before the music. *Settled
(Mikey, 2026-10-10, "The engine"):* warm the core's compressors, in item
31b; the JavaScript engine keeps its cold start.

**Listening:** the "Rust, all voices" album now covers the whole mix.

**Next:** Session 12, hands, **Sonnet 5.5, high**: item 31b. After
Mikey's listening: making the core the default (the flag's last step),
then the ears reports' sound items (stereo first), and the generator's
port when the words settle.

**Ears reports** (`docs/ears/batch-1.md`, `batch-2.md`, `tools.md`): a parallel sounds-and-ideas brain chat
measured 32 albums against Driftloom (stereo, too much 60-120 Hz and
too little 1-5 kHz, chords changing too often, loops that circle,
short tails, full stops where records drop the floor). Its candidate
items 1 (stereo), 2 (bass and presence) and 5 (tails, echo throws)
touch the effects chain, so they come after item 31. Not started: the word rankings, Depth two, the open decisions,
the iPhone check.

Not started, for Mikey's own time: the word rankings, Depth two, the
open decisions above, the iPhone check.

**Session 12 (hands, Sonnet 5.5, high) merged item 31b (#161, v82).
Brain-verified 2026-10-11 (Session 13).** A Sonnet subagent read the
diff against the brief: all six parts do what it asks; flag off
unchanged (every new path behind the flag); no test hook ships (the
trap is injected by `tools/core-paths.mjs`); v82 in both stamps. The
warm start plays each compressor on silence until it rests (at most 195
blocks, at 96 kHz), bit-identical at every sample rate to a long cold
run. A Haiku subagent reran: worklet and generator tests, the balance
lock (22 of 22), `core-paths.mjs` twice (29 of 29), `--clicks` and
`--endings` on the core (clean), and `--null --voice kalimba` twice
(core renders identical across runs; late and fallback 0; first second
-15.8 to -22.7 dB against JavaScript's cold start, as expected; after
it -83.6 to -87.3 dB, at JavaScript's own floor). Cargo tests and the
`.wasm` byte check can't run here (the pinned toolchain is behind the
proxy); CI's `core` job ran both, green. **Found, folded into item 32:**
`dispose()` leaves the failure handlers set (a narrow window where a
disposed synth reconnects its JS chain); a quality toggle after a trap
rebuilds the core; a block that goes backward into the core after three
stale frames is untraced; the flutter's catch-up error is about 1000x
the stated figure (still harmless); the JS-vs-JS floor moved up to
2.8 dB between identical runs, against `measure.mjs`'s stated
repeatability; and 31b's +3% render on full is unexplained.

**Mikey decided: mono only** (see "The engine"), and closed #152.

**Next:** Session 14, hands, **Sonnet 5.5, high**: item 32.

**Later on 2026-10-11:** item 32 merged (#164, v83) and the Design v17
UI (#166, v84); neither brain-verified yet. Mikey chose the words next,
with ranking in bulk: item 33 widens `/rank`'s pool first.

**Session 16 (brain, Opus 5.5, 2026-10-11), the words, roadmap 19 only.**
Item 33 merged (#168, v85; not brain-verified, by Mikey's choice: no
verifying other merged items this session). Read Mikey's first 41 pool-3
trials with `ranks.mjs` (findings in `docs/MOODS.md`): nothing fit 8% of
the time (was 40%); he reaches for air and motion words, seldom the heart
words; 2-5 shows a word, far too few to fit. The fit needs every ranked
loop placed on the ten dials, which nothing measures yet, so the queue
gets the tools first. **Mikey decided:** reflective may also be neutral;
cinder and shatter may take a bright mode when a word calls for one.
Queued items 34 (dials), 35 (the fit) and 36 (the two mode decisions).
Then Mikey ranked 181 trials in all: nothing fit 3%; steady, cool,
wistful and zany fit nearly every loop, heartwarming and joyful almost
none; the energy words follow tempo and drums clearly, but a scratch fit
on spec features (32% top word) did not beat base rates (38%). Mikey
proposed **word albums** instead of more five-word trials; the brain sent
two pilots and queued 37 (answering them on `/rank`), with 35 gaining the
album builder and an honest check against base rates.
Mikey heard both pilots the same evening: about half the guesses fit
(details in `docs/MOODS.md`). He then asked for the words in the app as
**vibes**, his top priority, with a sketch: queued as item 38 ahead of
34-37 (37 on hold), and the brain drafted all 71 recipes in
`docs/VIBES.md` (names checked against the generator's).

## Start here before Session 28

The bullets of `docs/BRAIN.md`, "Start here", as they stood on 2026-10-30,
moved whole at Session 28's archive pass.

- **Session 26 (brain, Opus 5.5, 2026-10-26): done; Mikey's verdicts
  pending.** The sound batch (46b-73) checked: it passes (the log,
  `docs/archive/brain-log.md`). Item 72's big baseline moves are the
  reshuffle: shil-doask, reisk-foa and their-roask drew lateFloor at item 71
  and not at 72, and every loop whose choices didn't change is bit for bit
  unchanged. **The albums** (`docs/ALBUMS.md`, "Session 26"): three in-app
  codes and four sets of clips on one private page whose verdicts save to its
  database. **Read them back** with `ArtifactData` (`list`, collection
  `verdicts`, the page's URL in `docs/ALBUMS.md`); each verdict becomes a
  queue item ahead of 77 (tune and drop) or nothing (keep); the bass pick
  sets `LIFT_DB`; the blind A/B's key is in `docs/ALBUMS.md`. **Phase 2
  written** (items 77-92, #275), with Mikey's three decisions of the day:
  phase 2 takes the sequencer, the port is proven note for note, glacial
  (76b). **Next:** the hands take 76, 76b, 74, then 77. The brain's next
  check: after item 82 (the player), or sooner if the verdicts call for one.
  Done lines #241-#274 stay live until the verdicts are in, then go to the
  archive.
- **Since Session 26 (brain, 2026-10-29):** item 76 merged (#277, v124;
  archived with its note). Mikey took the vibes to 88: queue item 76b now
  holds glacial and twenty more, four dropped (`docs/VIBES.md`, "The vibes
  to 88"; `docs/DECISIONS.md`). And no compatibility bloat for old codes.
  **The album was heard the same day** (`docs/ALBUMS.md`): everything kept;
  two tunes queued as 76c (the bass level) and 76d (lateFloor's second
  arrival). Done lines #241-#274 archived.
- **2026-10-30:** 76b, 76c, 76d and 74 merged (#280-#285, v125-v128) and
  archived. **Next: phase 2 from item 77** (hands, Opus 5.5, high). The
  brain's next check: after item 82, or sooner if a note comes in.
- **Since 2026-10-26 (hands, docs restructure and item 76).** All three
  groups of the sound batch are merged (items 54-73, #245-#272, v103-v123;
  group (b) is items 64-66, group (c) 67-73), and the album loops in the
  Done lines were re-checked against `main` after item 73 (#273). `main` is
  at v123. **Item 76** is queued from Mikey's Moto Diagnostics (v116,
  2026-10-25): the first loop after the app starts loses its first notes
  (app track, Sonnet 5.5, high). The docs were split by role: this file
  keeps the orientation and this "Start here"; the decisions are
  `docs/DECISIONS.md`; the log, History, finished items, old notes and Done
  lines are in `docs/archive/`; `docs/README.md` is the map. **For Session
  26:** end the session with the archive pass (see "The docs stay small").
- **The plan to phase 2 (Mikey, 2026-10-15).** He wants more
  instruments, profiles and vibes before the Broken Friends step, with ears
  sessions to find them. Timed so nothing is built twice:
  - **Now, alongside everything:** ears sessions (records he loves ->
    candidate sounds as offline clips; his verdicts, including on
    dubStab, organ, wash and longChords). Vibes may be added until phase
    2 starts (data in `data/vibes.txt`).
  - **Session 22** (brain, Opus, 2026-10-18): **done.** Items 42-45
    verified, 47's PR read; the control album "Phase 1 check" heard by
    Mikey the same day: no difference. See the log
    (`docs/archive/brain-log.md`); one known issue for phase 2
    (wander across the slider's upper range should not repeat; `docs/QUEUE.md`,
    "Known issues").
  - **Session 23** (hands, Opus, high): item 46, the building-block
    interface. **Done** (#238, v100).
  - **Session 24** (brain, Opus 5.5, 2026-10-19): **done.** Item 46
    checked by the brain (passes; phase 1 is done); **item 46b** queued
    (modulation input, stereo signal shape, channel events and a
    channel's own blocks; bit-identical); **the sound batch** written as
    items 54-73, with item 74 (clippy) low priority; the wander note
    carried into the charter's phase 2. See the log
    (`docs/archive/brain-log.md`). As planned before it: write **the sound
    batch** into the queue, three groups, from `docs/ears/`
    (batch-4 and batch-5 and Mikey's verdicts):
    (a) **instruments**: dubStab, longChords, wash, the chapel organ,
    roundBass, glass, floorDrop, arc; built in the core on 46's
    interface, core-only (the JavaScript-synth fallback substitutes an
    existing voice), each measured for level, cost and tone against its
    ears reference;
    (b) **stereo in the core**: the approved stereo baseline, the mono
    switch, the first space recipes (charter promise 9);
    (c) **the mixing logic** (Ears 8, batch 5): tunedKick, lateFloor,
    floorDrop shapes, pedalBass, subDuck, padSweep, echoTime, under
    Mikey's rules (today's behavior stays one option in every choice;
    seeded per-loop choices; most loops get one or two changes; subDuck
    only with a kick and a sub; stagger and stutter only with a bass
    line; drop style may follow the bar length; a wandering loop may
    swap the kick at section boundaries with a crossfade or a wait).
    **Where it lands (brain, 2026-10-18, answering Ears 8's question):**
    the choices go in today's JavaScript generator, which is allowed
    until phase 2 starts and then ported with everything else (they
    become part of what the port must match); what they play (the tuned
    kick and boom, the bass-only duck, the chord-layer sweep, the echo
    time as a parameter) goes in the core only, after item 46, so
    nothing collides with 46. One builder session for both halves.
    Then phase 2's items (ordering 48-53 among them). The balance lock
    is re-baselined on purpose, once per group, and says what moved.
  - **Since Session 24 (2026-10-19 to 22), for the next brain:** 46b
    split into 46b and 46c; items 54-63 merged (#245-#256, v103-v112),
    so group (a) is done and the balance lock is re-baselined. Decided
    on the way: glass follows its chord's velocity times chords a bar,
    and rings only on chords of two or more notes (Mikey); a 0.4 dB rate
    tolerance for golden lists holding a steady tone through the reverb,
    with **item 75** (the reverb rate-true for steady tones, low
    priority) and a rule to re-check every Done line's album loops at
    the end of group (c) (brain). **The arc (Mikey, 2026-10-21):** a
    loop of his sank under 700 Hz for most of a 3-minute swing and he
    thought the instruments were gone; once he knew it returns he liked
    it. So **item 63b**: two shapes drawn per loop, "deep" (today's,
    about 1 in 3) and "dip" (mostly open, one gentle dip to 800 Hz-1.5
    kHz, about 2 in 3), with vibe weights (calm and vast vibes lean deep
    and long; bright and lively lean dip, short and rarer). **And Mikey
    (2026-10-22):** the arc's variables (shape, period, floor, start,
    how often) should become controls for listeners and game developers
    eventually; for now the generator weights them. So 63b keeps them as
    named, ranged settings (promise 9: every variable a named setting),
    and phase 2's requests (promise 11) can pin them; sliders in the app
    are his call later (a Painter design), game hosts reach them through
    the host API (phases 4 and 7). **For Session 26:** a "glacial" vibe
    (Mikey's idea: very slow and vast; the deep arc, the longest swings),
    designed with him there. **Two accounts (Mikey, 2026-10-22):** this
    Claude account ran low for the week, so the hands sessions (63b,
    group b, group c) and Ears 9 may run on his other account. The repo
    carries the state: only one brain edits these files at a time, and a
    hands session that hits a taste call takes the conservative option,
    writes it down "for Mikey's ears", and goes on (decisions batch up
    for Session 26), unless the item says to stop.
  - **Session 25-0** (hands, Opus 5.5, high): item 46b. **Done (#241,
    v101, 2026-10-19)**: every yardstick met; the stereo shape is a second
    signal beside the mono one, and the web `.wasm` rebuilt without the
    stereo code is the same bytes, so mono pays nothing.
  - **Item 46c** (brain, 2026-10-19, from 46b's notes): channel blocks
    with per-loop settings and modulation, events that reach a held note
    every time (the organ's retune), cross-channel signals (subDuck).
    Items 61, 62, 63, 68 and 72 depend on it. Session 25a takes it first.
  - **Session 25a / 25b / 25c** (hands, Opus 5.5, high): the three
    groups in that order (a: items 54-63; b: 64-66; c: 67-73), one PR per
    item; a new session per group (or when usage runs out), same brief.
    Item 74 whenever a session has usage to spare (Sonnet is enough).
  - **Session 26** (brain, Opus; **done 2026-10-26**, see the first bullet): check all three groups (the hands' notes
    on items 55-73 are in `docs/archive/notes.md`; their Done lines in
    `docs/DONE.md`); a labeled
    album (keep / tune / drop) of the new sounds and mixing, and a blind
    stereo-vs-mono A/B; **the bass level** (item 56, #248, Mikey chose
    today's level for now, 2026-10-20): the same few loops with the bass
    at today's level, +3 dB and +5.5 dB (the prototype's), labeled, his
    pick sets the per-voice trims; in the same album, item 56's wander
    no longer folding the bass's passing tones down; new vibes with Mikey, last before the port.
    **Then write phase 2's items** (not written yet): the generator's
    port, ordering 48-53 among them, with the chords' octave fix and the
    wander's upper range that must not repeat (both in the charter's
    phase 2 and in "Known issues"), and item 54's per-loop choices and
    the batch's generator choices as part of what the port must match.
  - **Open for Mikey (Session 24's taste calls, in the queue "for his
    ears"):** the new parts (glass, dubStab, the organ, the wash, the
    arc) count toward "most loops get one or two changes" (item 54: none
    15%, one 45%, two 30%, three 10%), so each is in fewer loops than in
    the ears' clips; the arc starts each loop bright, not closed (item
    63). His answer moves a weight, not the design.
  - **Ears cadence (brain's advice, 2026-10-18):** after Ears 9, pause
    new piles until the batch is built and heard in the app. The
    approved backlog is large, and sketches can sound different in the
    app than as clips (the first ears session's lesson); hearing it in
    the app first makes later syntheses sharper and saves tokens.
    **Agreed by Mikey, 2026-10-18:** Ears 10 on wait until Session 26's
    album has been heard.
  - **Session 27** (hands): the first item of phase 2.
  - **New profiles wait until phase 2 lands**, then go in as data
    (before phase 3 and Broken Friends). Phase order in the charter unchanged.
- **Next picks (2026-10-13):** Mikey chose the design
  (`design/next-picks.md`, from Painter 5, PR #205); see "Next picks,
  the design" under "The core". Painter 6 (app track) is building the
  screens now. Real wiring queued as items 48-52 (phase 2) and 53
  (phase 3), for the brain to order among phase 2's own items when it
  writes them (Session 26, after the sound batch; moved from Session 24). **Waiting on Mikey:** the two open questions (the way in
  during an album; setting only length or only tempo to "any"). **Next
  Painter work:** the tempo, key and scale, style and voices pickers,
  and the "can't do that" line; long-piece display (`docs/PAINTER.md`,
  section 2) before item 53.
- **Where it is:** `main` at v100 (2026-10-19; phase 1's items 42-47
  merged and checked, Painter 6's next-picks screens). The Rust core plays by default (the
  JavaScript synth is the fallback, `?engine=js`); Vibes are in the app
  (items 38a-b, v86-87; recipes in `data/vibes.txt`, tuned once, v91); the
  dials, the word fit and the word-album builder are tools (items 34-35);
  cinder and shatter borrow bright modes when glad (item 36).
- **Glitches on GrapheneOS phones (items 39-41, v89-v92):** the core now
  gets notes 2-8 s ahead, planned from the phone's measured render-ahead;
  late notes past 30 ms are dropped, not bunched; covers are drawn in a
  Worker. **Open:** the Pixel 10a's 1.3-1.8 s main-thread stalls were never
  reproduced; the tester's next Diagnostics (v92: `gpu`, render-ahead,
  late and dropped by cause) will say whether anything is left. If it
  calls for a fix, that comes before the core work.
- **Item 45 stopped for decisions (#213, draft); decided by Mikey
  2026-10-14** (written under item 45 in the queue): tone tolerance 1.5
  points at 44.1/22.05 kHz; the noise made rate-true (baseline update);
  192 kHz native-only; PowerPC CI runs plain goldens only. Items 42-44
  merged (44 is #211).
- **Emergency (2026-10-13): queue item 47**, glitches when minimized
  or locked (Pixel 9, GrapheneOS, v92). All 10 late notes "inside the
  buffer", none from stalls; render-ahead usually ~20 ms but 1.4 s at
  worst. Brain's lead: Android deepens the output buffer in the
  background and the planner only adapts after measuring. App track,
  runs alongside the core track (outside `core/`). Hands: Opus 5.5, high.
- **The core charter is approved (Session 18, 2026-10-13):
  `docs/CORE.md`.** Phase 1 is queued: items 42-46 in `docs/QUEUE.md`.
  Hands, by item: 42 Sonnet (Session 19), 43 Opus (20), 44-45 Sonnet
  (21), 46 Opus (23), all high. **Brain checks, to save tokens, at
  two points rather than every PR:** Session 22, after 45 (goldens
  equal on all five targets, the moved draws' list and ranges, the
  baseline, rates and block sizes; then the control album for Mikey);
  Session 24, after 46 (phase review, then phase 2's items). Each a
  new brain chat, Opus 5.5. Session 18 closed after this. Before phase 5: the block
  catalogue for Mikey (charter, promise 5).
- **Next: the core** -- a new brain chat. Read **"The core: aims and design"**
  in `docs/DECISIONS.md` first: the number one priority, shoot for the moon,
  embeddable anywhere (game engines, mods, any OS or device, iOS and
  macOS kept easy), notes decided and played away from the page so the
  website never glitches from screen work, instruments as Rust building
  blocks plus data recipes, short plain-ASCII share codes, anything may
  break. **Start by drafting the core charter for Mikey's approval**: what
  the core promises, how each promise is measured, and the phases. The
  platforms (F-Droid, itch.io, later Google Play) are later phases of
  their own.
- **Not brain-verified** (Mikey chose to save tokens): #164 and #166, and
  items 38-41's PRs (the hands' reports are in them).
- **From the ears, for the generator port (Ears 4, 2026-10-16; Mikey
  asked that the brain hear of it): the chords' octave can flip.**
  `separateRegisters` (`js/generator.js`) moves the keys down an octave
  when the mean pitch of the *sounding* chord notes is within five
  semitones of the melody's mean. That mean depends on which chord
  events survive the form, rests and gaps, so a change that doesn't
  touch the harmony can still push it across the line: holding chords
  longer (the ears' longChords sketch) flipped the octave in 11 of 87
  held loops (200 scanned), and Mikey heard those loops' chords as too
  loud and rough (grove #47's pads jumped from 104 Hz to 208 Hz at the
  bottom). Pins and requests (promise 11: length, tempo, voices) are
  exactly that kind of change. **Wanted in phase 2, not after it:**
  decide the octave from the progression itself (its chords and voicing
  before form, rests and gaps) rather than from the events, so pinning or
  varying anything else never moves it; and a corpus check that varies
  one non-harmonic choice per loop and counts octave flips (0 expected).
  Today's loops render as they always have (the flip needs a change to
  happen); the fix itself will move the octave in the few loops where the
  two ways of deciding disagree, which the phase's blind album covers; the ears' sketch fix and numbers: `docs/ears/batch-4.md`,
  "Round 4", and `docs/EARS.md`. Written into the charter's phase 2.
- **Waiting, not urgent:** item 37 (on hold; Vibes cover it); the
  ears-approved sounds -- roundBass, glass, floorDrop, arc (2026-10-10);
  **dubStab and longChords approved 2026-10-16** (as in Ears 4's round 4,
  `tools/ears/sketches/ears4-sketch.patch`), and **stereo's baseline
  approved** (the blend of room and ensemble, steady; moving stereo kept
  for a later, experimental option); **the wash and the chapel organ
  approved the same day** (Ears 4's rounds 5 and 7; every recipe, with
  levels, costs and checks, in `docs/ears/batch-4.md`, the sketch in
  `tools/ears/sketches/ears4-sketch.patch`; the organ belongs outside
  the voice budget as fixed overhead, the stab is a budget probe that
  holds nothing) -- under the charter they may become the first
  instruments built as recipes; vibe tuning from the loops Mikey sends
  (`data/vibes.txt`, the brain's to edit).

## History

One line per PR; details are in the PRs, `ROADMAP.md` and the queue's
Done list.

- **#1-#15 (to v32):** rhythm cells and `stats.mjs`; Cloudflare deploy,
  build stamp, offline fix; melody forward in the mix; `measure.mjs`;
  motif-level omission, stepwise walk, register separation; audible
  contour; anchor by transposition; phrase velocity; vowel drift; the
  choir; the glottal source (v31); slid attacks (v32).
- **#16-#18:** this file.
- **#19 (v33):** the melody starvation fix -- `keys` priced 25 -> 12, the
  soft cap made a reserve that works on lite. Lite still lost >5% of the
  melody on 12% of loops then.
- **#20 (v34):** `keys` chords had played through the melody bus since the
  app began; moved to the chords bus.
- **#21-#25:** roadmap item 13; the loudness yardstick in `measure.mjs`.
- **#26 (v35):** three level bugs -- bells took velocity twice, pad skipped
  the chord spread, rhodesbass trimmed twice.
- **#30, #31:** the balance lock; the punch figures (no metric matched the
  ear, so levels went to listening).
- **#34, #38, #40, #46 (v36-v39):** 6/8 -- chords made strict, then the
  rhythm section restored to the cross-rhythm he preferred.
- **#42:** the drum knob, a listening test, closed unmerged.
- **#48, #53 (v40, v42):** the loud leads trimmed at the source (pluck,
  saw; then ocarina, flute, whistle, moog, analoglead).
- **#51, #59 (v41, v44):** fiddle and accordion, take one (disliked) and
  take two.
- **#57, #62, #63 (v43, v45, v46):** `tide` -- profile, progressions per
  mode, drone, 3/4; waltz melodies, grace notes, harmony; waves and the
  hand kit.
- **#64, #65 (v47, v48):** the harsh leads softened; nylon with strumming,
  pan flute.
- **#66, #67 (v49, v50):** `cinder` (the Andalusian cadence spelled per
  mode), `wayfare` (the chug, with its budget fix).
- **#74, #75 (v51, v52):** grace notes shorter than the attack cut off with
  a click (pan flute, flute, ocarina, analoglead) and `--endings`; the
  nylon strum tightened.
- **#79, #82 (v53, v54):** wayfare's own drum grooves; then their variety
  back.
- **#88, #89 (v55):** the performance harness and baseline; sound polish
  (sung notes on formant peaks, 5/4 chords, arpeggios clipped).
- **#92 (v56):** composition depth, first pass.
- **#96-#108 (v57-v64):** performance wins, the temple bell, the moods,
  composition depth's second pass.
- Docs PRs from the brain are the rest of the numbers in between.
