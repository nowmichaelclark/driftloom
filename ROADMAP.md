# Roadmap

Open work, ordered. Everything here has a size and a test that would prove it
done. Decisions already made are at the bottom so they stop being reopened.

What has shipped or been closed is in `docs/archive/roadmap-shipped.md`: items
1-4, 9 (v2), 10-14 and 21, and the full original of 16. The numbers are kept,
so "roadmap 17" still means item 17.

Sizes: **S** under an hour, **M** a session, **L** more than a session.

---

## 5. Real-hardware budget calibration — **S**

Nothing is stopping this except that it needs a phone, and the phone is not
mine. The totals (260 / 170 / 140) are reasoned, not measured. Diagnostics
already reports `lateTicks` and `worstLateMs`.

**Done when** a twenty-minute run on the target phone, on the densest profile
mix available, reports zero late ticks — or the totals are lowered until it
does, and the number that worked is written down here.

**This got more urgent, not less.** A melody-starvation bug (fixed in v33)
turned out to be the budget refusing the tune on 28.6% of loops, and part
of the cause was a weight that was simply wrong: `keys` was priced at 25,
the same as a piano, when it is *half* of a piano -- one FM voice against
two. If one weight was that far out, others may be. The tool to check is
now in the repo: `node tools/measure.mjs --voice` for tone and the
marginal-render-time method for cost, both documented in the README.

The same fix also found that `SOFT_BUDGET` did nothing on lite, because
`Math.min(170, 140)` is 140. So two of the three numbers in this item's
title were not doing what the title claims, and neither failure was
visible from reading them. **Calibrating the totals on real hardware is
worth less than it looks until the weights they are spent on are checked**,
and this item should probably be done in that order: weights first, then
totals.

## 6. Cover art floor — **M**

"Occasionally amateurish" is not testable. The specific failure is the sparse
compositions (`drift`, `orb`, `aperture`) where the gradient ground carries
most of the frame and the result reads as a background rather than a picture.

**Done when** no cover in a 100-cover contact sheet has more than 70% of its
pixels within one palette stop of the ground color.

## 7. Interface: three standalone tickets — **S each**

The old "revisit when the feature set stops moving" had no trigger, which
meant never. These stand on their own:

- **7a.** Reorder tracks within an album.
- **7b.** Duplicate a loop before editing, so a saved version survives.
- **7c.** Search or filter the saved list once it passes ~30 entries.

**Done when** each works from the album or saved panel without a page reload.

## 8. Album code length — **S to decide, M to build**

The real complaint is not size, it is that a 1300-character code is not
tappable the way a link is. Compression does not fix that; it makes an
untappable thing slightly shorter.

**Decide first:** if the goal is tappability, the answer is a file export or a
QR code, not deflate. **Ship compression only if it clears 35%** — below that
it adds an async path and a fallback for no felt benefit. Album payloads are
mostly high-entropy seeds, so measure before building.

**Done when** either a measurement below 35% is recorded here and the item is
closed, or codes are 35%+ shorter with a working no-compression fallback.

## 9. Articulation v3: breath before phrase entries — **M** — *not started*

v2 (slid attacks) shipped; its entry, and the shared preamble, are in
`docs/archive/roadmap-shipped.md`.

A wind player inhales before a phrase, and the sound of that breath before
the first note is a large part of why a recording sounds like a person in a
room. The voices already have continuous breath noise *under* the note;
this is a separate, louder, unpitched event placed *before* an entry.

`annotatePrev` is half of what this needs already: a phrase entry is
exactly a note with no `prev`, which is the same test the scoop uses. What
it additionally needs is somewhere to put the breath, and that is the
problem -- it has to sound *before* the note, so either the engine
schedules it early (the note's own start time minus the breath length,
which can fall in the past at the top of a loop) or the composer leaves
room for it.

**Done when** a phrase-initial wind note is preceded by an audible breath
in 40-70% of entries, the breath never delays or clips the note it
precedes, and no breath is scheduled at a negative time. The last of those
is the one that will bite.

## 15. Ringing bells in fast blends starve the tune — **S** — *low priority*

Found by Claude Code while measuring `cinder` (#66), 2026-09-25. When a
fast loop (around 136-146 bpm) blends in enough `shrine` to draw its
temple bell, the bells ring long (about 1.4 s) at a cost of 12 each, and a
dense tune of them fills the voice budget. One `cinder` loop in 183 lost
48.7% of its melody at full quality. The same case already exists on
`main` in `shatter` and `undertow` blends, so it is rare, not new.

Mikey: worth fixing, not soon. Options when it comes up: shorten the
bell's ring (and so its reservation) at fast tempos, reserve budget for
the melody the way `SOFT_BUDGET` does for pads, or keep the long-ringing
bells out of fast blends.

**Done when** no loop in a 200-loop corpus loses more than 5% of its
melody at full quality to this case, with refusals re-measured on full
and lite.

**A related budget fix, same priority** (raised in #67). Every bass voice
except `fifths` reserves budget for 0.8 s past its note but stops sounding
at 0.4 s, so bass holds room it doesn't use in every loop. `wayfare`'s
chug already reserves only while it sounds (#67); doing the same for all
bass notes would free room for the tune everywhere, lite especially. It
changes how existing loops sound (fewer refusals), so it gets a refusal
run on full and lite before and after.

Also measured (#74, #75): on lite, tide and cinder loops lose more melody
notes than the catalog (mean 2.9% and 3.5% against 1.5%), the same on
the old code. The fixes above are the likely remedy; re-measure both
profiles when this item is done.

## 16. A compiled audio engine (Rust) — **L** — *the core is the engine and plays by default; what is ahead is the phases in `docs/CORE.md`*

The synth, the effects and the master chain shipped (queue items 21-32), and the core is
Driftloom's own engine now (`docs/DECISIONS.md`, "The engine and stereo"). What is still
ahead is the charter's phases: the generator in Rust, the page off the audio path, the
platforms and the export (`docs/CORE.md`). The original entry -- the timing, the feasibility
probes, the language comparison -- is in `docs/archive/roadmap-shipped.md`.

## 17. Export a loop as audio — **M**

Rendering a loop offline was considered for playback and rejected
(2026-09-26): "Let the loop wander" is on by default and changes the loop
every pass, and re-rolling a layer must be instant. As an export it's
right. Download a loop as an audio file, including a wandering loop
rendered for as many passes as the user asks ("bake a track that wanders
for 100 loops"). Offline rendering has no deadline, so the export has no
refusals even on a slow phone. MIDI export already exists beside it.

## 18. Adaptive music for games — **vision**

Mikey: could the engine go into a videogame, where "the player's stats and
which room they're in and what is going on or significant events could
change what kind of music is generated"? The games he has in mind are chill ones --
Animal Crossing, a point-and-click adventure, something turn-based -- not
action: music that follows the time of day, the weather, the place, the
season, the turn, rather than combat. The generator is already
parameterised in the right way: a profile blend, feel (energy, lift,
warmth), mood, layer mutes and drift. A game would map its state onto
those, with changes landing on bar or phrase boundaries so the music
turns rather than jumps, and short stingers for events. It needs the
compiled core (item 16) to run outside a browser, and an API: set the
parameters, get bar and phrase callbacks, trigger stingers. Not planned;
recorded so item 16 is designed with it in mind.

**Now a real game (Mikey, 2026-10-24):** Broken Friends, his Godot
creature-care game, is where this happens: a theme per creature, music
by time of day and by what is happening at camp. `docs/CORE.md`,
promise 10.

## 19. Words that steer the music — **L, a design project, worth doing well**

From Mikey's mood vocabulary (`docs/MOODS.md`, 178 words). Agreed
2026-09-27:

- **Labels of one to five words**, usually two or three of different
  kinds (heart, light, texture and taste, motion, sound, strangeness),
  each true of what the loop does. Some loops are **one word, fully
  embraced**: everything about the loop leans into that word.
- **Words steer, not just describe.** A one-word loop needs generating
  *toward* its word, so every word gets a recipe in musical terms: which
  dials it moves (lift, energy, warmth, tempo, swing, density, register,
  brightness, space, harmony color, instrument families, rhythm feel)
  and by how much.
- **Words bend a profile, they don't replace it.** The profile is the
  base; a word pushes it within its character. How each word meets each
  profile is to be defined together, by ear. Mikey's example: what would
  "sour" and "floating" sound like on `cinder`? One reading: cinder's
  flamenco core, slowed and suspended (long notes, sparse drums, more
  space, a higher line), with tart harmony (the added b9, close
  clusters, a thinner and brighter tone). Some pairings may turn out
  not to fit; finding that out is part of the work.

The process:
1. A pilot set of about a dozen words across the kinds, chosen with
   Mikey, each with a written recipe in `docs/MOODS.md`.
2. Build the recipes as data the generator reads, so tuning a word is an
   edit, not a code change.
3. Listening albums per word across several profiles: "does this track
   fit the word?", and "which word fits this track?" blind.
4. Tune, then widen to more words.

Where it stands (2026-10-11): the ranking test (`/rank`, pool 3, 71
words) is collecting Mikey's answers; queue items 34-35 measure each
loop on the ten dials and fit each word's region from them, so the
recipes can come from his ears rather than the brain's drafts.

This is generator work, so it happens before the generator moves to Rust
(item 16's timing); the vocabulary will be part of what the Rust
generator carries.

## 20. `tools/listen.mjs`: albums in one command — **S**

Every listening album so far was assembled by hand in the brain's
container: filter a corpus, shuffle, encode, round-trip, and work out the
key. A tool that does it -- by voice, profile, kit, meter, length,
development or mood, blind or labeled, with the key derivable -- would
make albums quick for any brain session and remove the chance of a
mistyped code.

## 21. Composition depth — **M** — *two passes shipped; tuning by ear*

Shipped as queue items 12 (#92) and 17 (#108) (the entry is in
`docs/archive/roadmap-shipped.md`). Open:
- "Depth two" is out for listening (`docs/ALBUMS.md`).
- Three in four 32-bar loops are micro-loops that never develop; making
  them develop would be a new kind of form. Mikey to decide.
- The share is his to tune as he listens.

---

## Decided against

**More of the FM bell family.** There were already five FM bell voices (bell,
chime, musicbox, celeste, harp) differing mainly in ratio and decay, and a
sixth would have been a sixth setting of the same instrument.

This was wrongly used to defer temple and church bells too, which are a
different object rather than another setting: struck metal is inharmonic, and
a bowl's character is *beating* between partials a few cents apart, which a
single FM voice cannot produce however inharmonic its ratio. `templebell` and
`tubular` now exist in the `shrine` profile.

**Vocaloid-style sung words.** Words would make a loop be *about* something,
which fights the use case. A wordless voice also never sounds dated or
foreign; a synthesised word always does.

**Speech-synthesis dependencies** (Klattsch, Pink Trombone, Qlatt). They
optimize for intelligibility, which is the opposite of the goal, and each
brings a build step or a worklet, breaking "works offline from a folder".

---

## Contribution rules

Not roadmap items; the things that are not obvious from reading the code.

Run `node test/generator.test.mjs` before and after any change, a few times,
since it uses random seeds. For anything that touches generation, also run
`node tools/stats.mjs` before and after and compare the two: the tests prove
nothing is broken, the statistics say whether the change did what it claimed.

1. **A voice added to an existing pool re-renders every code that draws
   from it.** Voices are drawn at render time from the blended pool, so one
   added entry shifts that draw and every random decision after it.
   While the app is in testing (Mikey, 2026-09-23) that is allowed: old
   codes are not protected. It still has to pass the balance lock, and the
   PR should say so, since an A/B album across it compares different
   loops. `grove`, `hollow` and `shrine` were added as new profiles when
   codes were still protected.
2. **`PROFILE_IDS`, `SCALE_IDS` and `MOOD_IDS` in `share.js` are append-only.**
   Reordering them silently rewrites codes already written down. Breaking
   old codes is allowed during testing, but reordering gains nothing.
3. **Generation is quantised to a 1/255 grid** (`q8` in `generator.js`) so
   share codes are lossless. Weights feed weighted random picks and a
   rounding difference of 0.004 selects a different scale.
4. **A declared cost that is not passed is worse than no cost.** `VOICE_COST`
   only applies where the call site passes it; several weights sat declared
   and ignored for a while, charging the default instead.
5. **A new voice arrives measured, not guessed.** Its `VOICE_COST` weight and
   its loudness beside the voices it will sit next to are measured when it is
   added, not estimated and corrected in a later project. `keys` was priced
   at 25 -- one FM voice charged like `piano`'s two -- and quietly ate the
   melody on real phones until #19; the same kind of guess about level is
   what item 13 is now unpicking. Both get cheaper the earlier they are taken.

