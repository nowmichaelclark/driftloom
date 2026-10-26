# Roadmap: shipped and closed

The roadmap items that shipped or were closed, as they stood in `ROADMAP.md`
before 2026-10-26: 1, 2, 3 (taken up by composition depth), 4, 9 (v2; v3 is still
ahead), 10, 11 (closed), 12, 13, 14, 16 (the original entry; the core's phases are
in `docs/CORE.md`) and 21 (the shipped part). What is still ahead is `ROADMAP.md`.

## 1. Commit the measurement harnesses — **S** — *shipped, both halves*

Four items below say "measure first" and the tools to do it were not in the
repo. They had been written inline in throwaway scripts against a local
server and never cleaned up, which is the only reason they were missing;
they were not low quality, they were the things that found the envelope
clicks, the saturator, the voice-budget miscount and the melody bug. Both
now live here.

**`tools/stats.mjs` — generation.** Draws a corpus through `newSpec()` and
`render()` and reports profile, meter, voice and rhythmic-cell distributions,
melodic span, note count, duration, velocity and the share of melody notes
landing off the beat, either overall or bucketed by `feel.lift`. It is what found the vestigial ternary
that had kept every melody note in the app's history on an even step.

**`tools/measure.mjs` — audio.** Drives the real `Engine` and `Synth` against
an `OfflineAudioContext` in headless Chromium and reports per-loop peak, RMS
and full-scale sample count, corpus peak and RMS spread, crest factor, and
the dry level of each layer tapped at its channel gain, so the balance
between layers is measured rather than read off the gain table. Web Audio
does not exist in Node and a reimplementation of the graph would only
measure the reimplementation, so the tool needs Playwright and a Chromium
build; that is a dependency of the tool, not of the app.

It is what established that the peak range the README used to claim was
never true, and what item 11 is waiting on.

**Not built:** the per-voice probe the original acceptance line described
(`--voice templebell` printing ring time and cut-off dB). Nothing has needed
it yet. It is a small addition to the same harness whenever something does.


## 2. Articulation, v1: velocity by phrase position — **S** — *shipped*

Four features, and shipping all four at once is why this stalled twice.
Splitting it is what moved it. v1 was two things and they landed apart.

**The omission half shipped in #7**, though not where this item expected it.
Notes omitted mid-phrase at a rate that follows energy now happens *in the
motif*: which notes the figure leaves out is decided once, when the figure is
built, so every restatement is missing the same notes. Rolling it per bar had
been quietly undoing the motif -- dropping a note does not merely remove it,
it fuses the two intervals either side into a third that was never in the
figure -- and bars sharing the commonest rhythm ran at 0.55 because of it.
Deciding it once took that to 0.85. Density still follows energy: 2.5 notes a
bar in the low-lift bucket against 3.8 in the high, a 50% difference.

**The velocity half is this change.** Stronger on the first and last note of a
phrase, weaker through the middle, with the depth following energy so a
hushed loop arrives even and a quickened one breathes. The rhythmic cells'
own note-level accent stays underneath it: the accent says which note of the
figure is leaned on, the phrase arc says where in the phrase the leaning
happens. Slid attacks and breath-before-entry remain v2 and v3 and are still
out of scope.

**The old criterion was "velocity variance within a phrase at least 3x what
it is today", and it stopped meaning anything.** It was written when the
melody had almost no velocity variation at all. The rhythmic cells then
added a per-note accent, and within-bar velocity now measures sd 0.0906 over
9,126 bars, mean 0.533, across 478 distinct values. Tripling *that* is a
recipe for jitter, not for phrasing. Variance cannot tell the two apart:
phrasing is the part of the variation that is **systematic**, the same shape
every phrase, and a spread figure counts the random part just as happily.

**Done when** the notes at a phrase's edges average at least **0.10 of
velocity above the notes in its middle**, measured across the corpus by
`tools/stats.mjs` as "edges above middle", **and** that gap is at least
**1.4x** larger in the high-lift bucket than in the low one.

The two numbers, and why those:

- **0.10** has to be audible and has to be unmistakably deliberate. Mean
  velocity is 0.533, so a gap of 0.10 is a ratio of about 1.21 between the
  edges of a phrase and its middle -- roughly 1.7 dB, which reads as
  intentional dynamics where anything under about 1 dB reads as nothing. It
  also has to clear what the cells produce by accident: most cells accent
  their own first note and a phrase begins on one, which was already worth
  0.044 before any of this. Double that accident is a figure no amount of
  luck reaches.
- **1.4x** is the shaping following the feeling rather than being applied
  flat. Before this change the gap was 0.041 in the low bucket against 0.045
  in the high -- a ratio of 1.1, which is to say none. The threshold sits
  below what the change actually achieves (1.52 to 1.62 across five corpus
  seeds) by enough that an ordinary corpus cannot fail it.

Measured on 4000 loops: edges 0.641 against 0.509 mid, a gap of **0.133**,
and **1.62x** between the buckets (0.097 low, 0.157 high). The within-phrase
range roughly doubles, 0.223 to 0.360, but that is a consequence rather than
the test.


## 3. Melodic range — **M** — *largely taken up by composition depth (item 21)*

*Update 2026-09-28:* the average-of-14 target was reframed with Mikey as a
**spread** -- some tunes narrow, some roaming -- and delivered through
composition depth (queue items 12 and 17): developing loops now span up
to 20-22 semitones while simple ones stay as they were. What follows is
the original item.


Melodies average 9.6 semitones. **Target: 14, roughly an octave and a half**,
which is an ordinary range for a tune and clearly wider than now.

Widening the contour arc changed the figure by nothing at all, so the arc is
not the constraint and the cause is unknown. Three suspects, in order:
`nearestChordTone` pulling phrase edges inward, the degree random walk rarely
approaching its own -5..10 limits, and the octave wrapping folding wide leaps
back into the window.

**Done when** the average is 14 semitones or more and distinct-shapes-per-bar
stays above 0.9, so range is not bought with repetition.


## 4. Keys and melody register — **S** — *shipped*

Both layers currently land in the same narrow band by accident. The decision,
not a coin flip:

**Different voices → force a minimum separation** of 5 semitones between
layer centroids, moving keys down rather than melody up.

**Done when** no loop has two different pitched voices whose centroids are
within 5 semitones.

**Shipped.** `separateRegisters()` runs in `render()` once both layers exist
and moves the chord *events* by whole octaves; the slots the bass and melody
read their notes from are untouched, so the harmony is unchanged and only
its voicing moves. Measured over 4000 loops: different-voice loops inside
five semitones went from 99.4% to **0%**, and the mean centroid gap from 1.4
to 9.8 semitones.

This item originally carried a second clause: when both layers drew the
same voice, force them into unison, measured as 81% of such loops sitting
within 2 semitones. **That clause has been removed, and the 81% should not
be chased.** It was measuring an artefact. The old per-note octave fold
crushed the melody into a near-fixed band that happened to sit a roughly
constant distance from the keys, and the alignment was a side effect of
that crushing rather than of anything this item did. Item 9's anchor work
gave the line real register freedom, the accident went with it, and the
figure fell to 42%. Restoring it would mean undoing that work. Same-voice
loops still sit within six semitones, which is one register by any reading,
and that is all this item ever needed from them.

What the clause was reaching for is a different thing entirely, and it is
now item 12.


## 9. Articulation v2 and v3 — **M each** — *v2 shipped; v3 (breath before phrase entries) open*

Only after item 2 ships. Most of what makes a whistle sound human is
articulation, so these subsume the old "human whistle" item rather than
sitting beside it. They had one shared done-when between them, which is
no use to either; they have their own now.

### v2, slid attacks — *shipped*

The wind voices -- `ocarina`, `flute`, `whistle` -- had vibrato and breath
and still did not sound played, because nothing ever arrived at a pitch.
Every note simply began on one. A player reaches the note.

**Where it lives** is the split vowel drift established. Which notes get a
slide is articulation, decided in `_slide` with `Math.random` beside the
scoop, the jitter and the vibrato. Where the line came from is not
something the synth can know -- it sees one note at a time -- so the
composer writes it down: `annotatePrev` walks the finished melody and
records the previous sounding pitch on each event. That pass is derived and
draws nothing, so `tools/stats.mjs` is byte-identical at four corpus seeds
and 4,000 codes written on `main` decode and render the same music on both
trees, re-encoding to the same string.

**The shape.** From the previous note when the two are joined -- within
120ms, which over the corpus is 63.4% of notes -- and no wider than a
fourth, because a glide across a big leap is a siren. Anything else takes a
scoop from a tone and a half below, which is what a player reaching a note
after a breath actually does. The reach is `dur * 0.22` clamped to
30-90ms, so it lands well inside the first quarter of the note, and notes
under 180ms -- the shortest quarter of the corpus -- do not slide at all.

**The rate is 0.22 of eligible notes, which measures 18.1% of wind melody
notes overall**: 9.8% from the previous note, 8.3% scooped. About one note
in five and a half, which is a player leaning into some of them rather than
an effect applied to the line. It holds across the three voices, 16.8% to
19.1%.

**It is not tied to lift or energy, and does not need to be.** The material
already carries that relationship: a driven loop has shorter notes and
fewer joined pairs, so the share of its notes a slide can touch falls from
96.8% at low energy to 67.5% at high, and 88.2% to 71.2% across lift.
Consulting the feeling as well would count the same thing twice.

**Cost: nothing.** It is parameter automation on an oscillator that already
exists, lasting 30-90ms. With *every* eligible note sliding -- an upper
bound the 0.22 rate never approaches -- marginal render time moves -3.2%,
+2.0% and +0.6% for the three voices, which is noise either side of zero.
`VOICE_COST` is unchanged.

**Found by ear** with `node tools/stats.mjs --voice-codes ocarina`, which
exists because the three wind voices are 12.4% of melody draws between them
and any single one is nearer 4%.

**Done when** a slide lands on 15-25% of wind melody notes, no slide
crosses more than a fourth, every slide is on pitch before the note's
quarter point, `stats.mjs` is byte-identical and existing codes render
unchanged. All five hold.


## 10. Vowel movement within a note — **M** — *shipped*

Vowels are chosen per phrase and shared across a chord. The remaining step was
movement *within* a long note -- "ah" opening into "oh" across a sustained bar
-- which is the difference between a formant filter and something sung.

**Shipped in the synth, not the composer**, and that is the whole of why it
was cheap. Contribution rule 1 is about voices, but its reason is general:
anything drawn from the composer's stream renumbers every decision after it,
and every share code in circulation renders as different music. A vowel's
*destination* is not a compositional decision in any case -- it is
articulation, the same category as the scoop, the jitter and the vibrato,
which have always been per-note and drawn from `Math.random`. It is also the
only place that can answer the question: the composer knows a note's length
in steps, and whether a vowel has time to travel is a question about seconds.

The move is one rung along the open/close axis and never across it -- a to
o or e, o to u or a, u to o, e to a. F1 is the openness formant (a 800,
o 450, e 400, u 325), so neighboring rungs glide and the ear hears one
vowel changing shape. A jump across the ladder ("eh" straight into "oo") is
two vowels in succession, which is a word, and words are in *Decided
against*. A hum has no vowel to go to and opens instead: the same closed
tract relaxing.

**Length decides both halves.** `held` is how far a note is into "long" --
zero below 0.5s, one from 2.0s up -- and it is both the probability of
moving at all and, through `0.34 + held * 0.66`, how far along the way the
note actually gets. Over 4000 loops, 26.4% of formant-voice notes move, at a
mean depth of 0.81; the 41% of them under 0.55s never move at all. That is
the "short notes stay put" clause, measured rather than asserted.

**The filters move; there is no second bank.** Crossfading into a second set
of peaking filters is the obvious build and it is wrong: two banks summed
have different phase responses, so the sum combs and it sounds like a
flanger rather than a mouth. Sliding the resonances themselves means every
instant in between is a real vowel shape, and it adds no nodes at all.

**Cost: 5 units on top of the voice**, against 22 for `vowel`, 16 for `hum`
and 34 for `choir`, and it is passed rather than merely declared, which is
contribution rule 4. Measured the same way the rest of the table was: the
marginal render time of a note through the real `Synth` in an
`OfflineAudioContext`, one build with the decision pinned off against the
same build with it pinned on. Movement runs 27-29% of a vowel, 19-23% of a
hum and 13-16% of a choir, which through each voice's own weight is 4.7 and
5.0 units on two clean passes. One constant rather than three, because the
extra is not nodes -- none are added -- but the same three biquads
recomputing coefficients while a parameter is in motion, which is the same
work in every case. The README has the caveats on the conversion.

**Done, and how it was checked.** One 3.0s "a" pinned to drift to "o",
rendered dry and band-integrated early (22% in) and late (90% in): the two
builds are identical in the early window to a tenth of a dB, and in the late
one the moving build puts 8.2 dB more into 350-600Hz (o's F1) and
11.8 dB less into 1000-1400Hz (a's F2). The note starts on the vowel it
was given and arrives somewhere else. `tools/stats.mjs` over 4000 loops is
byte-identical before and after, which is the other half of the claim: no
draw moved.


## 11. Loudness spread across the catalog — **closed, the spread is wanted; reopened narrower as 13**

Measured with `tools/measure.mjs` over thirty loops: peak level runs from
about 0.17 to about 0.85, roughly 14 dB, with nothing normalizing it. A
quiet loop is quiet because its profile is quiet -- the sparse, airy
palettes land at the bottom of that range and the busy ones at the top --
and the master chain deliberately does not pull them together.

**This is not filed as a fault.** Quiet tracks are wanted, and so is
dynamic range across the catalog; a loop machine whose every loop arrives
at the same level has had something taken away from it. Normalization is
not the obvious answer and is not being proposed here.

The open question is narrower: **is the spread wider than intended when you
listen to twenty in a row?** Reaching for the volume control between tracks
is a different experience from noticing that one piece is hushed, and 14 dB
of peak is a lot if the quiet ones are also quiet in the middle rather than
merely less peaky.

**What would settle it:** report RMS spread alongside peak spread from
`tools/measure.mjs` -- it already prints both -- and compare them. If RMS
spread is much narrower than peak spread, the quiet loops are simply less
peaky and the perceived range is smaller than the figure suggests, and this
closes. If the two track each other, the catalog really does span 14 dB of
loudness and the question becomes a real one worth answering.

**First reading, thirty loops:** peak spread 13.4 dB, RMS spread 13.2 dB.
They track almost exactly, so the quiet loops are quiet in the middle and
not merely less peaky, and the perceived range is about what the peak
figure says. That points at the question being a real one rather than an
artefact of crest factor -- but one corpus of thirty is a reading, not a
verdict, and what it cannot say is whether 13 dB is wider than *wanted*.
That part is a judgment about listening, not a measurement.

**Somebody has now sat through twenty in a row, and the answer is that the
spread is intended.** Quiet tracks are wanted and so is dynamic range
across the catalog; a loop machine whose every loop arrives at the same
level has had something taken away from it. Reaching for the volume
control between two tracks is the cost of that, and it is worth paying.

**So: no normalization, and nothing to build.** Not a limiter across the
master, not a loudness target, not a per-loop trim written into the spec.
The 13 dB stands. If this is ever reopened it should be reopened as a
different question -- something about the *order* loops arrive in, which is
a sequencing problem and not a gain one -- rather than as this one.

The measurement stays in `tools/measure.mjs` because it is worth knowing
when the figure moves. A future change that quietly narrowed the spread to
4 dB would be a regression, and this is the item that says so.

**Reopened, narrower, as item 13** (2026-09-22). The spread stays wanted.
Reaching for the volume every other track does not: the aim is loops that
are dynamic but sensible, still with no limiter, compressor or
normalization doing the work.


## 12. An occasional choir — **M** — *shipped*

Melody and keys currently draw the same voice in **11.1% of loops**, purely
because the two pools sometimes collide. Nothing makes those loops sound
like anything in particular; they are just loops where the register
separation had nothing to do. One in nine is also far too often for
something that is supposed to feel like an event.

The intent is the opposite of an accident: a doubling drawn **on purpose,
rarely**, and arranged so that it lands. Three parts, all of which have to
be true at once or it is just two layers playing the same thing:

- **Drawn deliberately, at roughly 1 loop in 30.** Not a consequence of the
  two pools matching -- an explicit draw that then sets both voices.
- **A voice that rewards doubling.** The sung and struck families are the
  candidates; a pad doubling a pad is inaudible as an effect.
- **The arrangement thins around it.** Doubling is only audible if something
  gets out of the way. Whatever else is competing in that register steps
  back for the duration.

**Done when** the deliberate case fires in 3 ± 1% of loops, the incidental
collision no longer forces unison on its own, and a listener who did not
know the feature existed can pick the choir loops out of twenty by ear.
That last part is the real test and it is not a number `stats.mjs` can
produce.

**Shipped, with the last clause still open** -- see the bottom of this item.

**The draw.** `choirOf(spec)` hangs a stream off the loop seed with its own
salt, the same pattern `feelForLayer`, `genGaps` and `genForm` use. It
therefore takes nothing from any layer's stream, which is what makes the
next paragraph possible, and it survives a per-layer re-roll because
`rerollLayer` replaces a layer seed and never the loop seed. It needed no
share-format change either: `writeSpec` has always written `spec.seed`, so
a code written down before this existed already says whether its loop
sings. Measured at **3.06%** over 400,000 draws; 2.9-3.3% on 4,000-loop
corpora.

**Nothing else moved.** A choir loop *relabels* the voice its draw
produced rather than replacing the draw, so every `r.` call in `genHarmony`
and `genMelody` still happens in the same order with the same results. A
choir loop is the loop that seed always made, sung. Checked against a
worktree of `main` over 4,000 loops at each of six corpus seeds: every loop
that is neither a choir nor one of the collisions below is byte-identical,
0 differences out of ~3,430 per seed.

**The accident is gone.** `separateRegisters`' unison branch read "the keys
drew the voice the melody drew", which fired on 11.1% of loops and forced
unison on pairs nobody had chosen. It now reads "this loop drew a choir".
`genMelody` asked the same question a second time, for its register
anchor, and that one is changed too -- leaving it would have had the melody
aim at the keys while the keys walked away from it. Collisions still
happen at about the same rate and are now simply separated like anything
else; `stats.mjs` prints the count so a silent return to acting on them
would show.

**The sound.** Melody and keys take one sung voice -- `vowel` or `hum`,
weighted 3:2 -- with a standing detune of 4-11 cents in opposite
directions, an ensemble slip of +/-8ms drawn per layer per step, and
starting vowels two rungs apart along `VOWEL_KEYS`. The air layer drops to
0.4 of its level for the loop, which is the only thing left competing in
that register once the keys stop being a pad.

The voice actually named `choir` is not in the pool. It is three detuned
singers per note at a cost of 34, and this loop pays on two layers at once,
one of them a chord: putting it in roughly doubles what the budget refuses,
melody notes from 8.9% to 19.7% and keys from 8.1% to 19.3%. Dropping one
melody note in five on the loops built to show off a doubled melody is the
opposite of the point.

The unison works: choir loops sit **2.66 semitones** apart at the centroid
against 11.75 for everything else, and 100% of them are inside six.

**Two things measured here turned out to contradict the reasoning that
produced them**, and both are worth writing down because the wrong version
is the intuitive one:

- *Two identical voices do not sum to +6dB here.* They measure +3.3dB --
  the incoherent sum -- because each note already draws its own scoop,
  jitter, vibrato rate and breath. The detune is still worth having, since
  a standing lean is a different thing from a zero-mean wobble, but it is
  not what prevents the doubling collapsing into one louder voice. Nothing
  was going to.
- *Thinning the air layer is not a budget lever.* It is charged against the
  same voice budget, so it looked as though stepping it back would buy the
  doubling headroom. Silencing it outright moves melody refusals from 9.1%
  to 8.7%. The depth is a mix decision and was chosen as one.

A third thing went the other way and is worth the same honesty: the
`choir` voice was excluded on reasoning, the exclusion was removed on the
strength of a probe that seemed to show the budget coping, and it was put
back when an independent probe measured the doubling of refusals above.
The first probe was wrong. Reasoning that a measurement appears to
overturn is worth re-measuring before it is thrown away.

**Cost: +2.3 units a note, 14%,** counted over the melody and keys of
12,000 loops with the weights read out of `synth.js`. The worst single
chord attack on a choir loop is 88 units at the 90th percentile against
125 for the catalog at large, and 110 at the maximum against 170 -- a
choir loop is nowhere near the heaviest thing this app makes. Choir loops
also *refuse fewer* notes overall than ordinary ones, because a sung chord
at 16-22 a note is cheaper than the pad or piano it replaced: keys
refusals fall from 17.5% to 8.1%, against melody refusals rising from 6.8%
to 8.9%.

**The listening test passed.** The acceptance line was a listener picking
the choirs out of twenty by ear, and no amount of the above substitutes
for it. `node tools/stats.mjs --seed 4127 --choir-quiz` produced the
quiz -- twenty share codes shuffled, five of them choirs, answers written
to a file rather than the screen.

**5 of 5 picked, 0 false picks.** Two of the five were not immediate: they
registered a moment after the loop started rather than on the first phrase,
which is about what a rare event should feel like. The other three were
obvious at once. Item 12 is done on its own terms, not merely on its
numbers.

Worth recording that the marker is *audible* and not just statistical: an
effect that only a spectrum reveals is not an event, and this one does not
need the spectrum.


## 13. Level, by ear — **M** — *the complaint solved; 13b-13d not needed for now*

*Outcome, 2026-09-28:* the volume reaches came from the melody leads, not
the drums: pluck and saw sat 4-6 dB hot, and trimming them at the source
(#48) took the reaches on the test album from five to none; five more
leads were trimmed the same way (#53). Since then Mikey reports nothing
too loud or too quiet, so the catalog-wide albums 13b-13d, including
"twenty in a row", are not needed for now. The ocarina over drums is on
watch (ease it back 2-3 dB if it comes up buried). The original item
follows.


Item 11 in the narrower form it asked to be reopened in. The spread across
the catalog is still wanted: a hushed loop stays hushed, a busy one stays
bigger, and nothing pulls them to one level. What is no longer accepted is
the cost item 11 wrote off as worth paying -- a listener reaching for the
volume every other track. **Dynamic, but sensible.**

**Not the answer:** a limiter or compressor doing the work, a loudness
target, normalization. The master already carries a gentle compressor (3:1
above -10 dBFS) and a ceiling at -3 dBFS. They stay the backstop they are
and are not to be leaned on harder. (Measured in #24: they do not "only
touch the loud end". Web Audio's compressor applies makeup gain, so the
chain lifts every loop by a flat +3.8 dB and takes back 0-0.9 dB, moving
the spread by about half a LU.)

**One behavior for everybody** (decided 2026-09-22). No Room/Car switch, no
range slider, nothing for the user to scroll past. That also rules out the
slow leveler: it is defensible as an opt-in for noisy places and nothing
else, because always on it is the compressor this item exists to avoid.
The reaching goes both ways about equally -- some loops too quiet, some
jumping out -- and all three listening patterns happen: one loop for an
hour, new loops back to back, saved albums in order. Ordering therefore
helps the album case only and cannot be the whole fix.

**The target.** Played back to back the catalog is one program, and the
spread between loops is that program's loudness range. Online and mobile
content generally sits around -16 to -14 LUFS with a loudness range of
6-8 LU; this catalog measures about 13 dB. The authored profile trims are
4.6 dB and fit inside that band with room to spare, which is the argument of
this whole item: the accidental spread is the entire problem, and removing
it lands the catalog in the band on its own, with no per-loop mechanism at
all. Two things to establish alongside it -- a house level taken from a loop
Mikey already likes, and whether the loudest loops are riding the master
compressor. (They are not -- see below.)

**Measured, #24** (`tools/measure.mjs`, seed 1; the brain reproduced the
first 12 loops to the decimal). Integrated loudness spans 15.1 LU across
100 loops, but the middle of the catalog -- 10th to 95th percentile, as
LRA takes a range -- is **8.9 LU**, so the band is close and the problem is
mostly the tails. The authored trims account for about 1.3 LU of it. The
single biggest factor is **drums**: loops with drums average -23.2 LUFS,
loops without -28.0. The loud-and-squeezed theory was wrong: the loud end
arrives loud before the chain sees it. RMS misled in both directions -- it
over-counts the bass and under-counts the drums.

The voice table splits in two. **Plain bugs**, where a voice's code does
not do what it says: `templebell` and `tubular` apply velocity twice, so
level goes with its square; `pad` reaches `pad()` directly and skips the
engine's 0.8 chord spread, measuring exactly 1.9 LU over `moogpad`, the
same sound; `rhodesbass` is trimmed twice. **Taste calls**, where the
number is a question for the ear: `ocarina` and `flute` sit 5-7 LU over the
melody layer from their sustained envelopes; the saw leads 4-9 LU over at
realistic note lengths (queued already, and possibly part of why they read
as harsh); `hum` trimmed 8-9 LU down to tame one fixed 280 Hz resonance;
single notes of `vowel`, `choir` and `hum` jumping 10-15 LU when a harmonic
lands on a formant peak; `swell` over the other texture voices and `wind`
well under. `stab` measures 12 LU quiet, but a 220 ms sound is the wrong
shape for a 400 ms meter and Mikey heard it as fine -- the ear wins there.

**Why there is something to find.** The per-profile `level` trims in
`characters.js` run from 0.62 to 1.05, about 4.6 dB. That is the spread
somebody chose. Item 11 measured about 13 dB. The rest comes from things
nobody set as a level decision -- density, which voices a loop draws, how
loud each voice is for a given velocity, and bugs like the one #20 fixed,
where every `keys` chord in the app's history played through the melody
bus. The aim is to keep the authored spread and find the accidental one.

**Albums first, fixes second.** Each question gets a listening album,
chosen with `tools/measure.mjs` numbers attached, answered blind in one word
per track with the key in a file. The tracks Mikey flags, matched against
their measurements, say where "sensible" actually sits -- a target from his
ears rather than a number anyone guessed.

- **13a. Keys, after #20.** `dust` loops with `keys` chords against `dust`
  loops with `pad` chords, then the other four profiles that draw `keys`.
  Same profile, different chord voice, so only the voice differs. One word
  for the chords: *buried / fine / forward*. If keys read as buried and pad
  does not, the fix is inside the keys voice, not the channel table.
  `keys` is the main chord voice of `dust`, the commonest profile, so this
  is the change most widely heard.
- **13b. Melody voices.** Does the tune sit in front whichever voice draws
  it? The same melody voice across similar loops, extremes picked by
  `measure.mjs --voice` and the per-layer levels. *lost / fine / loud*.
  After the sawtooth-lead fix, since softening those at the source moves
  their level too.
- **13c. Kinds of loop.** Sparse against busy, low energy against high,
  across profiles. The question is whether a quiet loop reads as a quiet
  *piece* or as a loop that has merely been turned down -- the first is
  the spread working, the second is the accident. *hushed / fine / loud*.
- **13d. Twenty in a row.** A shuffled run at the catalog's natural
  spread, straight through at one volume. Mikey marks every track where he
  would have reached for the volume, and which way. The measured loudness
  of those tracks, and the jump from the track before, turns "too much
  spread" into a number.

**Tooling.** `measure.mjs` reports peak and RMS. It wants a perceived
loudness figure as well -- K-weighted, as in ITU-R BS.1770, the basis of
LUFS -- because raw RMS over-counts bass and the albums should be chosen and
read on what the ear weights. `tools/listen.mjs` is what makes each album
one command.

**Candidate fixes, chosen by the albums rather than before them:**

- *Per-voice calibration.* A voice that is inherently louder or quieter than
  its family at the same velocity is trimmed at the source. This is
  calibration, the same kind of fix as the `keys` cost weight in #19, and
  not normalization.
- *Pull in the tails, leave the middle.* If only the extremes send a hand to
  the volume, narrow the extremes and touch nothing else. Anything per-loop
  is derived from the spec and draws no random numbers, so codes stay stable.
- *Order, not level.* If the reach comes from big jumps between neighbors
  rather than from any one track, smoothing the shuffle or playlist order
  fixes it without changing a single gain. Item 11 pointed here.

**Done when** a fresh twenty-in-a-row run, on a new seed, draws **at most
one** reach for the volume, **and** the corpus loudness spread has not
fallen below what 13d shows Mikey is comfortable with -- that number gets
written here once it exists -- 6-8 LU is the expectation, not the rule.
Crest factor and within-loop range must be unchanged: the macro spread is
what moves, and the phrase-level dynamics are not to be touched by anything
done here. Item 11's warning stands too: a change that passes the first half
by collapsing the spread toward 4 dB has failed the item.


## 14. Folk sounds: the 6/8 fix, new voices, new profiles — **L, in steps** — *shipped*

*Shipped 2026-09-24/25:* the 6/8 accompaniment (#34-#46), fiddle and
accordion (take two, #59), nylon with strumming and pan flute (#65),
`tide` (#57, #62, #63), `cinder` (#66) and `wayfare` (#67, then #79 and
#82 for its drums), all heard in the review album. The plan below is
kept as written.


The island and sea music of Wind Waker and Spirit Tracks: Celtic-folk
melody, waltz and jig meters, fiddle, whistle, accordion, guitar, hand
drums, drones. Mikey brought a set of DeepSeek suggestions to the brain;
the diagnosis (instruments, meter, ornament) stood, much of the code did
not. Decided 2026-09-23, in this order.

**14a. The 6/8 accompaniment.** In 12-step bars the chords borrow the 4/4
patterns in `CHORD_RHYTHMS`, filtered to `step < 12`. So `offbeat` hits step
6, the downbeat of the second dotted beat; `twoAndFour` becomes one hit on
a weak eighth; and `pad` appears to keep its 16-step hold and ring a third
of a bar into the next chord (to confirm). 55% of glade-led loops are in 12
steps, so the profile nearest this music plays its tunes over the wrong
meter most of the time. Fix: a 12-step pattern for every rhythm name, drawn
by the same name -- the choir's relabel-not-redraw trick. Codes may change
now (testing phase), so this is a preference for a clean A/B, not a rule. Existing 12-step loops will sound different,
which is the point, so it gets an A/B album. Check the bass for the same
fault.

**14b. Voices, one PR each, measured per contribution rule 5.** fiddle
(slow bow attack, bow noise, vibrato that arrives late); nylon, with
strumming (down and up, 15-30 ms across the strings, downstroke accented --
chords today spread 11 ms in one fixed order); accordion (two detuned
reeds, bellows swell); pan flute (a flute variant with more breath and
chiff). Steel pan optional. Existing pools may now take new voices --
fiddle into glade, say -- since old codes are no longer protected; each
such addition is Mikey's call and must pass the balance lock.

**14c. After the drum fix (item 13): new profiles, one at a time, appended
to the end of `PROFILE_IDS`.** First `tide`, the sailing waltz; then
`cinder`, the fiery 6/8; then `wayfare`, the rolling train. Names are
placeholders until Mikey picks, in the house style: about the sound, not
the source. Each brings what glade lacks:

- its own progression pool -- the I-bVII shuttle, i-bVII-bVI-bVII,
  IV-I endings -- where today every profile draws the same twelve;
- a drone bass style: an open fifth held under changing chords;
- ornaments -- cuts and turns at sub-step timing, 30-60 ms, played by the
  engine, with the generator only marking which notes get one;
- harmony in thirds or sixths on held notes only;
- a waves texture (slow filtered-noise swells);
- hand drums as a new kit beside tape, brush and machine, not a flag.

Every new generator feature draws only when the loop's profile asks for
it, from its own salted seed, where that is free -- it keeps A/B albums
honest. Old codes are not protected (testing phase).

**Not taken from the suggestions:** Nintendo names;
raising `MAX_BUDGET` (it is the low-end-phone guard, and #19 showed the fix
is measuring costs); guessed costs; grace notes on the step grid (a step is
about 150 ms at 100 bpm, a cut 30-60 ms, and one step early collides with
the note before); a harmony line copying every melody note (it doubles the
melody's cost, which is #19's starvation); new jig and waltz cells
(`cross12`, `six8` and `hemiola12` already are them).

**Done when** each step has been A/B'd by ear and the balance lock passes,
or its baseline has moved with Mikey's say-so.


## 16. A compiled audio engine (Rust) — **L** — *every voice in the core behind the flag (items 21-28); effects and master chain queued as items 29-31; generator after the words*

**Timing, 2026-10-03 (Mikey):** start now. "Settled" never fully arrives
on a project he'll keep growing. The synth ports first, now: the voices
are approved, no new instruments are planned, and the words work barely
touches it. The generator waits until the words' draws stop moving, so
changes aren't made twice. Word recipes, maps and pools live as data, so
most later tuning needs no code in either language.

**Feasibility (2026-10-03, brain, Session 3).** Can Rust be built where
the work happens, or is the compiled file committed?
- *The hands' sandbox* (Claude Code cloud): `rustc` and `cargo` come
  pre-installed, and the default network allowlist includes
  `static.rust-lang.org`, `rustup.rs` and crates.io, so adding the
  `wasm32-unknown-unknown` target or a pinned toolchain should work.
  crates.io has been flaky behind its proxy before, which is one more
  reason for **no dependencies**: plain Rust `std` compiles its math
  (`sin`, `exp`, `powf`, `tanh`) to WebAssembly with zero imports.
- *Cloudflare Workers Builds*: the build image has Node, Python, Go, Ruby,
  Bun and Hugo, no Rust. Rust can be installed in a build command (about a
  minute per build, reported elsewhere), but that turns a site with no
  build step into a pipeline, adds a minute to every preview, and adds a
  failure the hands can't retry (#51).
- *The brain's probe* (Rust 1.90.0, `wasm32-unknown-unknown`, no crates):
  a pool of sine voices through a Web-Audio-formula lowpass, 2.8 KB
  (7.5 KB with `std` math). Built in two different folders, the two files
  were **byte-identical**. Run in an AudioWorklet in headless Chromium
  (OfflineAudioContext, the harness's own setting), it nulled against the
  same notes built from Web Audio nodes to a **-55.6 dB** residual --
  but only once note starts were sample-accurate; starting notes at the
  128-sample block boundary left the residual at -0.1 dB.

Decision: **commit the built `.wasm`**, as item 13's brief proposed. The
deploy stays plain files, old commit previews keep working, and the
bytes the brain verifies are the bytes that ship. A pinned toolchain
(`rust-toolchain.toml`) makes the build reproducible; a build script
with a `--check` mode, and a check on every PR, prove the committed file
matches its source.

Mikey wants the app light and fast even at full quality ("DHH-wow" smooth
with a hundred other apps open), and is open to an overhaul. Today every
note builds its own little Web Audio graph (oscillators, filters, gains),
hundreds a second, plus the garbage they leave. The candidate: one
engine running inside an AudioWorklet, written in a compiled language
(Rust to WebAssembly, say) with a preallocated pool of voices, no
per-note objects and no garbage.

Staged, so nothing is a leap of faith:
1. The baseline (queue item 10) says how much of the cost is the graph
   and how much is JavaScript. If the graph isn't the problem, stop.
2. A prototype of a few voices in the worklet, measured against the same
   voices today with the same harness, and heard side by side. Queued
   2026-10-03 as items 21 (the pipeline and kalimba) and 22 (fiddle,
   pad, the performance A/B); both merged and verified 2026-10-05 (#126,
   #127): every ported note nulls at float rounding against Chromium's
   own voice, and the audio thread works 10-20% harder. Waiting on
   Mikey's phone check.
3. Voice-by-voice migration behind a flag, each voice proven equivalent
   with the existing `--voice` tone and level probes and `--endings`.
4. The generator ported last, so share codes stay deterministic across
   both engines.

**What the baseline found (2026-09-26, queue item 10):** building the
graph is 2-5% of the main thread. The main thread is mostly paint (a UI
problem), and most of the audio thread is finished voices never
disconnected plus the always-running effects -- both fixable in
JavaScript (queue item 14). So a compiled engine is not the performance
fix; its case is portability: the phone app's core, a native build,
games. Taken up after composition depth and the words settle (agreed
with Mikey, 2026-09-26; superseded 2026-10-03, see Timing above), in a
fresh brain chat. The prototype (queue item
13) stopped at its gate, as designed.

**Design requirement: a host-agnostic core.** The Rust core generates
samples and knows nothing about browsers: `process(buffer)` plus the
parameters. Thin hosts wrap it -- an AudioWorklet for the web app, a
native audio backend (such as `cpal`) for a desktop or mobile build, and
a game engine's audio for a game (Bevy natively or in the browser via
WebAssembly, Godot through its Rust bindings, Unity through a C plugin).
Browser support is broad: WebAssembly and AudioWorklet run in current
Chrome, Edge, Firefox and Safari, including iOS Safari 14.5 and later.
Avoid anything needing special headers (threads, SharedArrayBuffer),
keep the JavaScript engine as the fallback until the Rust one has been
heard on real phones, and test on Mikey's own phone early.

**Which language (considered 2026-09-27).** Rust stays the pick: no
garbage collector to cause audio hiccups, the best WebAssembly toolchain,
a native audio backend (`cpal`), a Rust game engine (Bevy) plus bindings
for Godot and a C ABI for Unity, and DSP crates to build on (`fundsp`).
The one real alternative is C++, the audio industry's language (JUCE,
FMOD, Wwise, every engine speaks it), at the cost of memory bugs that are
harder to catch in code maintained with AI help, and messier builds.
Faust, a language just for DSP that compiles to Rust, C++ and
WebAssembly, is worth a look for the voices alone. Not chosen:
AssemblyScript (web only), Zig (young ecosystem, few engine bindings),
C# (garbage collected, no good worklet story). Staying in JavaScript is
also legitimate if the web app is the only target; the reason to port
is portability, not speed.

**Timing -- superseded.** The first plan (below, 2026-09-26) moved the
synth soon after queue item 11 for speed. The baseline then showed speed
wasn't the reason, so the whole port waits until composition depth and
the words (item 19) settle, and happens in its own brain chat. Within
the port, the order stays: synth first, generator after. The original
paragraph, for the record: sooner for the synth, later for the
generator. The voices are close to settled --
Mikey has approved nearly all of them -- while the composition is about
to change a lot (queue item 12). So the synth moves to Rust once queue
item 11's polish lands, and voice design freezes during the port (any
voice change made meanwhile is made in the Rust version). The generator
stays in JavaScript while composition depth settles, then follows. The
JavaScript engine is the reference throughout: every ported voice is
proven against it with the probes before it replaces it.

**The deliverable** is the phone app, with a Rust core compiled to
WebAssembly inside the current web app: the same deploy, the same UI,
the audio and (later) the generator in Rust. A native app-store build and
game integration are separate projects the same core makes possible.

It is also the road to item 18: the same compiled core can build for the
web and for a game engine.


## 21. Composition depth — **M** — *two passes shipped; tuning by ear*

Mikey (2026-09-25): long loops too often stick with an elementary idea;
some compositions should explore more depth while simple ones stay.
Shipped as queue items 12 (#92) and 17 (#108): some loops develop -- a
contrasting B, varied returns, harmony spanning the form, a wider range --
while simple loops render exactly as before. `DEPTH` in
`js/generator.js` is the knob, with `DEPTH_SHARE` by length and
`DEPTH_WEIGHTS` by profile; `stats.mjs --depth` measures it and
`developmentOf(spec)` marks a developing loop.

Heard (the "Depth" album): development audible and liked at 24 and 32
bars, weak at 16 until the second pass (A A' B A'' in four-bar phrases).
His favorite was a simple 24-bar loop. Open:

(Its open points stay in `ROADMAP.md`.)
