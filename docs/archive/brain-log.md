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
- **The core baseline:** `node tools/baseline.mjs` run here (result in the next commit); green in CI on `main`.
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
