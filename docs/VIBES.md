# Vibes

The app's word for what `docs/MOODS.md` calls mood words (Mikey,
2026-10-11). A listener picks vibes on the Now Playing screen; every new
loop leans toward them. This file holds the **recipes**: what each vibe
leans on. The hands copy the block below into `js/vibes.js` once (queue
item 38a); after that `js/vibes.js` is the source and this file keeps the
reasoning and the history.

## How a recipe works (queue item 38a builds it)

A recipe names only what the vibe cares about. Every field is optional.

- **Targets** (0 to 1), pulled toward, never jumped to:
  - `feel: { lift, energy, warmth }` -- the generator's three feeling dials
    (the eight moods' points in `js/moods.js` are on the same scale).
  - `tempo` -- where in the loop's tempo band (0 its slow end, 1 its fast
    end).
  - `tone: { warmth, space, wobble }` -- the Sound panel's three.
  - `swing` -- where in the loop's swing band.
- **Leans** (multipliers on the generator's own weights; 1 is no change,
  2 twice as likely, 0.3 a third as likely): `profiles`, `modes`,
  `melody`, `chords`, `bass` (bass voices), `bassStyle`, `texture`, and
  the single numbers `six8` (6/8 against 4/4), `long` (8 bars and up
  against 2-4), `drums` (chance of a kit), `hats` (hat density), `rests`
  (bars of rest in the tune) and `bigChords` (four notes and up against
  three).

Per new loop: up to five of the selected vibes, weights that share 100%
(each at least 5%), and a strength between 0.5 and 0.9. A target moves
the draw part-way, by strength x the summed weights of the vibes that set
it, toward their weighted mean target. A lean multiplies by
`value ^ (strength x weight)`, so a vibe at 5% barely moves it and a lone
vibe at full strength moves it all the way. Words bend a profile; the
profile's own ranges still clamp the feel and tone.

## As built (queue item 38a, v86)

`js/vibes.js` is the source from v86 on; the block below is the copy it
started from. Where the brief left a reading open, the hands took these
(the brain may change any of them):

- **A lean multiplies only what a pool already holds.** `melody: { harp: 2 }`
  on a loop whose profiles draw no harp does nothing: a vibe tilts a
  profile's own choices and never brings in a voice, mode or texture it
  lacks. The `profiles` lean is the way to reach a profile's voices.
- **`drums` leans the odds** of a kit (yes against no), so 2 makes a kit
  twice as likely against none and never more than certain. `hats` and
  `rests` multiply the profile's hat density and `restBar`, which the
  generator already scales and clamps.
- **`six8` leans every 12-step metre**, 6/8 and the waltz (3/4) alike, in
  the step-count pool and in a lead profile's gaits.
- **`tone` targets** pull the drawn value and then clamp to the blend's own
  range for that dial, so a vibe cannot take a profile's sound outside
  what it allows. `feel` targets pull before the clamp to the profiles'
  feel ranges (as the brief says), `tempo` and `swing` the draw's place
  in its band.
- **Targets with weights:** strength x the summed weights of the vibes
  that set the target, capped at 1, toward their weighted mean.
- **Share codes:** a loop with vibes is written as format 4; a loop
  without is still written as format 3, so no code in circulation, no
  saved album and no `/rank` answer changes its string.
- **The tuning tool:** `node tools/stats.mjs --vibes a,b,c` (one
  selection against none) and `--vibe-each` (a line per vibe);
  `measure.mjs --vibe-mix` draws its corpus with 1, 3, 5 or 10 random
  vibes for the loudness and refusal checks.

## The first recipes (brain, Session 16, 2026-10-11)

Drafts. Sources: the pilot recipes in `docs/MOODS.md`, Mikey's 181
rankings (what the energy and lift words went with), and his verdicts on
the two pilot word albums:
- **Heartwarming** fit: haven in lydian with a music box (yes,
  definitely), halcyon with hum, dust with a stab mostly, and glade in
  dorian with ocarina (the decoy -- so heartwarming is not about major
  chords). Not: undertow at 127 bpm, shrine's temple bell (almost, "but
  it's really not"), shatter.
- **Joyful** fit: wayfare at 137 bpm (yes), glade's harp at 111 and
  bloom's whistle at 118 (kind of). Not: grove's kalimba at 79, undertow's
  stab at 131, tide's fiddle at 97 only a bit. So: quick, a bright lead
  that sings or whistles, not a stab.

```js
export const VIBES = {
  adventurous: { feel: { lift: 0.75, energy: 0.8 }, tempo: 0.7, profiles: { wayfare: 3, tide: 2, glade: 1.5, cinder: 1.5, haven: 0.3, vapor: 0.3 }, melody: { fiddle: 2, whistle: 1.5, panflute: 1.5 }, drums: 1.5, long: 1.5 },
  airy: { feel: { energy: 0.35 }, tone: { space: 0.75, warmth: 0.4 }, profiles: { thaw: 2, vapor: 1.5, glade: 1.5, halcyon: 0.5 }, melody: { flute: 2.5, sine: 1.5, panflute: 1.5, saw: 0.3, stab: 0.3 }, texture: { wind: 3, swell: 1.5 }, rests: 1.8, drums: 0.6 },
  booming: { feel: { energy: 0.65 }, tone: { space: 0.7 }, profiles: { undertow: 2.5, halcyon: 2, shatter: 1.5 }, bass: { sub: 3, moogbass: 2 }, bassStyle: { pulse: 1.5, dub: 2 }, chords: { tubular: 2, templebell: 1.5 }, drums: 2 },
  bouncy: { feel: { energy: 0.7 }, tempo: 0.65, profiles: { bloom: 2.5, grove: 2, dust: 1.5, wayfare: 1.5, vapor: 0.3, haven: 0.3 }, melody: { marimba: 2, kalimba: 2, pluck: 2, whistle: 1.5 }, bassStyle: { walk: 2, pulse: 1.5, held: 0.5 }, drums: 2, hats: 1.5, rests: 0.6 },
  bright: { feel: { lift: 0.9 }, tone: { warmth: 0.35 }, modes: { lydian: 2, ionian: 2, majorPent: 1.5 }, melody: { musicbox: 2, celeste: 2, bell: 1.5, whistle: 1.5 }, texture: { bells: 2, chime: 1.5 } },
  cheeky: { feel: { lift: 0.75, energy: 0.6 }, profiles: { bloom: 2.5, clockwork: 2, grove: 1.5 }, melody: { moog: 2, prepared: 1.5, whistle: 1.5, pluck: 1.5 }, swing: 0.7, rests: 1.3 },
  chiming: { profiles: { shrine: 2, clockwork: 1.5, thaw: 1.5 }, melody: { bell: 2.5, celeste: 2, tubular: 2, musicbox: 1.5, templebell: 1.5 }, chords: { tubular: 2, templebell: 1.5 }, texture: { chime: 4, bells: 2.5 } },
  comforting: { feel: { lift: 0.54, energy: 0.36, warmth: 0.86 }, tone: { warmth: 0.75, wobble: 0.35 }, profiles: { halcyon: 2, haven: 2, dust: 1.5, shatter: 0.3 }, melody: { rhodes: 2, keys: 1.5, hum: 1.5 }, chords: { rhodes: 2, softpad: 1.5 }, bass: { round: 2 } },
  comfy: { feel: { energy: 0.3, warmth: 0.8 }, tone: { warmth: 0.75, space: 0.3 }, profiles: { haven: 2, dust: 2, halcyon: 1.5, shatter: 0.3, cinder: 0.5 }, chords: { rhodes: 2, softpad: 1.5 }, bass: { round: 2, rhodesbass: 1.5 }, bassStyle: { held: 1.5 } },
  cool: { feel: { warmth: 0.3 }, tone: { warmth: 0.25, space: 0.55 }, profiles: { thaw: 2, undertow: 1.5, clockwork: 1.5 }, melody: { sine: 1.5, piano: 1.5, analoglead: 1.5 }, chords: { pad: 1.5, analogpad: 1.5 } },
  cozy: { feel: { lift: 0.6, energy: 0.3, warmth: 0.85 }, tempo: 0.3, tone: { warmth: 0.8, space: 0.25 }, profiles: { haven: 2.5, dust: 2, grove: 1.5, halcyon: 1.5, shatter: 0.2, cinder: 0.4 }, melody: { rhodes: 2, kalimba: 1.5, keys: 1.5 }, chords: { rhodes: 2, nylon: 1.5 }, drums: 0.6, hats: 0.6 },
  crisp: { tone: { warmth: 0.3, wobble: 0.05 }, swing: 0.2, profiles: { grove: 2, glade: 1.5, clockwork: 1.5, wayfare: 1.3 }, melody: { pluck: 2, marimba: 2, kalimba: 1.5, harp: 1.5, softpad: 0.4 }, chords: { nylon: 2, harp: 1.5, softpad: 0.5 }, bass: { pluckbass: 2 }, bigChords: 0.6 },
  enthusiastic: { feel: { lift: 0.88, energy: 0.9, warmth: 0.66 }, tempo: 0.8, profiles: { wayfare: 2, bloom: 2, cinder: 1.5, tide: 1.5, haven: 0.3, vapor: 0.3 }, melody: { whistle: 2, fiddle: 2, analoglead: 1.5 }, drums: 2, hats: 1.5, rests: 0.5 },
  energetic: { feel: { energy: 0.92 }, tempo: 0.85, profiles: { shatter: 2, cinder: 2, undertow: 1.5, wayfare: 1.5, haven: 0.2, vapor: 0.2, thaw: 0.3 }, drums: 2.5, hats: 2, rests: 0.4 },
  experimental: { profiles: { clockwork: 2.5, vapor: 2, shatter: 1.5, hollow: 1.5 }, modes: { wholeTone: 2.5, dorianSharp4: 2, hirajoshi: 1.5 }, melody: { prepared: 2, saw: 1.5, vowel: 1.5 }, tone: { wobble: 0.6 } },
  floating: { feel: { energy: 0.2 }, tempo: 0.2, tone: { space: 0.8 }, profiles: { vapor: 2.5, haven: 2, hollow: 1.5, shatter: 0.2, cinder: 0.3 }, melody: { sine: 2, softpad: 2, flute: 1.5, hum: 1.5 }, chords: { softpad: 2, pad: 1.5, moogpad: 1.5 }, bassStyle: { held: 2, sparse: 2, pulse: 0.5, walk: 0.4 }, texture: { swell: 2.5, wind: 1.5 }, drums: 0.3, long: 1.5 },
  flowing: { feel: { energy: 0.5 }, swing: 0.5, profiles: { tide: 2, glade: 1.5, halcyon: 1.5, vapor: 1.3 }, melody: { flute: 2, harp: 2, fiddle: 1.5 }, chords: { harp: 2 }, bassStyle: { walk: 1.5, held: 1.3 }, rests: 0.6, long: 1.3 },
  foggy: { feel: { energy: 0.25, lift: 0.45 }, tempo: 0.3, tone: { warmth: 0.6, space: 0.75, wobble: 0.5 }, profiles: { vapor: 2, dust: 1.5, thaw: 1.5, hollow: 1.5 }, chords: { softpad: 2, moogpad: 1.5, pad: 1.5 }, texture: { swell: 2, wind: 2 }, drums: 0.5 },
  frosty: { feel: { warmth: 0.15 }, tone: { warmth: 0.15, space: 0.65 }, profiles: { thaw: 3, clockwork: 1.5, shrine: 1.3, halcyon: 0.5, cinder: 0.4 }, melody: { celeste: 2, musicbox: 2, piano: 1.5, sine: 1.5 }, texture: { bells: 2, chime: 2 } },
  golden: { feel: { lift: 0.7, warmth: 0.95 }, tone: { warmth: 0.85, wobble: 0.4 }, swing: 0.55, modes: { ionian: 1.8, lydian: 1.8, mixolydian: 1.4 }, profiles: { halcyon: 2, dust: 2, haven: 1.5, shatter: 0.3 }, melody: { rhodes: 2, keys: 2, ocarina: 1.5 }, chords: { rhodes: 2, pad: 1.5 }, bigChords: 1.8 },
  goofy: { feel: { lift: 0.8, energy: 0.6 }, profiles: { bloom: 3, clockwork: 1.5 }, melody: { moog: 2.5, whistle: 1.5, stab: 1.5 }, bassStyle: { walk: 1.5 }, bass: { moogbass: 1.5 } },
  happy: { feel: { lift: 0.82, energy: 0.52, warmth: 0.78 }, modes: { ionian: 2, majorPent: 1.5, mixolydian: 1.3 }, profiles: { grove: 1.5, glade: 1.5, bloom: 1.5, wayfare: 1.5, thaw: 0.5 }, melody: { whistle: 1.5, kalimba: 1.5, ocarina: 1.5 } },
  hazy: { feel: { energy: 0.35 }, tone: { warmth: 0.65, space: 0.6, wobble: 0.6 }, profiles: { dust: 2.5, vapor: 1.5, halcyon: 1.5 }, chords: { softpad: 1.5, moogpad: 1.5 }, texture: { swell: 1.5, wind: 1.5 } },
  heartwarming: { feel: { lift: 0.65, energy: 0.35, warmth: 0.85 }, tempo: 0.35, profiles: { haven: 2, halcyon: 2, glade: 1.5, dust: 1.5, hollow: 1.5, shrine: 0.6, undertow: 0.4, shatter: 0.2, cinder: 0.4 }, modes: { majorPent: 0.6 }, melody: { musicbox: 2, hum: 2, ocarina: 2, rhodes: 1.5, stab: 0.3, templebell: 0.4, saw: 0.4 } },
  hushed: { feel: { energy: 0.12 }, tone: { space: 0.4 }, profiles: { haven: 2.5, thaw: 2, vapor: 1.5, shrine: 1.5, shatter: 0.1, cinder: 0.2, undertow: 0.3 }, melody: { piano: 1.5, sine: 1.5, hum: 1.5 }, drums: 0.2, rests: 1.8 },
  hypnotic: { profiles: { undertow: 3, vapor: 2, clockwork: 1.5 }, bassStyle: { pulse: 2 }, rests: 0.5, long: 0.7 },
  intimate: { feel: { energy: 0.25, warmth: 0.75 }, tone: { space: 0.15 }, profiles: { haven: 2.5, hollow: 1.5, clockwork: 1.5, shatter: 0.2, undertow: 0.4 }, melody: { rhodes: 2, piano: 1.5, hum: 1.5 }, chords: { nylon: 1.5, rhodes: 1.5 }, bigChords: 0.6, drums: 0.4 },
  intricate: { feel: { energy: 0.6 }, profiles: { clockwork: 2.5, grove: 1.5, vapor: 1.5, cinder: 1.3 }, melody: { harp: 2, prepared: 2, marimba: 1.5, celeste: 1.5 }, chords: { harp: 1.5 }, hats: 1.5, rests: 0.6, bigChords: 1.4 },
  joyful: { feel: { lift: 0.92, energy: 0.72, warmth: 0.72 }, tempo: 0.75, modes: { ionian: 2, lydian: 1.5, majorPent: 1.5 }, profiles: { wayfare: 2, bloom: 2, glade: 2, tide: 1.5, grove: 0.6, undertow: 0.4, haven: 0.3, vapor: 0.3, thaw: 0.4 }, melody: { whistle: 2.5, harp: 2, fiddle: 1.5, ocarina: 1.5, analoglead: 1.5, stab: 0.3 }, drums: 1.5 },
  lively: { feel: { energy: 0.8 }, tempo: 0.7, profiles: { wayfare: 2, tide: 2, grove: 1.5, cinder: 1.5, vapor: 0.3, haven: 0.3 }, drums: 1.8, hats: 1.5, rests: 0.6 },
  meditative: { feel: { energy: 0.1 }, tempo: 0.15, tone: { space: 0.7 }, profiles: { shrine: 2.5, hollow: 2, vapor: 2, haven: 1.5, shatter: 0.1, cinder: 0.2, wayfare: 0.3 }, melody: { templebell: 2, hum: 2, tubular: 1.5 }, bassStyle: { held: 2 }, drums: 0.2, rests: 1.6, long: 1.4 },
  merry: { feel: { lift: 0.85, energy: 0.7, warmth: 0.7 }, six8: 1.8, profiles: { tide: 2.5, glade: 2, wayfare: 1.5, cinder: 1.3 }, modes: { ionian: 1.8, mixolydian: 1.8 }, melody: { fiddle: 2, whistle: 2, accordion: 1.5, marimba: 1.5 }, drums: 1.5 },
  minimal: { profiles: { undertow: 2, thaw: 2, vapor: 1.5, haven: 1.5 }, melody: { sine: 1.5, piano: 1.5 }, texture: { none: 2 }, bigChords: 0.5, rests: 1.5, hats: 0.6 },
  mischievous: { feel: { lift: 0.65, energy: 0.55 }, profiles: { clockwork: 2.5, bloom: 1.5, grove: 1.3 }, modes: { dorian: 1.5, lydianDominant: 1.8, wholeTone: 1.3 }, melody: { prepared: 2, pluck: 1.5, musicbox: 1.5 }, swing: 0.7, rests: 1.4 },
  misty: { feel: { energy: 0.3, warmth: 0.5 }, tone: { space: 0.75, wobble: 0.4, warmth: 0.5 }, profiles: { vapor: 2, thaw: 1.5, dust: 1.5, hollow: 1.5 }, chords: { softpad: 2, pad: 1.5 }, texture: { swell: 2, wind: 2 } },
  mysterious: { feel: { lift: 0.35, energy: 0.35 }, profiles: { hollow: 2, vapor: 2, clockwork: 1.5, shrine: 1.5, bloom: 0.4 }, modes: { hirajoshi: 2, kumoi: 1.5, dorianSharp4: 1.5, wholeTone: 1.5 }, melody: { vowel: 1.5, sine: 1.5, celeste: 1.5 }, tone: { space: 0.65 } },
  nimble: { feel: { energy: 0.75 }, tempo: 0.75, profiles: { grove: 2, tide: 2, glade: 1.5, bloom: 1.5, vapor: 0.3, haven: 0.3 }, melody: { marimba: 2, harp: 2, fiddle: 1.5, kalimba: 1.5, whistle: 1.5 }, rests: 0.6 },
  peaceful: { feel: { lift: 0.58, energy: 0.28, warmth: 0.6 }, tempo: 0.35, profiles: { haven: 1.5, grove: 1.5, tide: 1.3, shrine: 1.3, shatter: 0.2, cinder: 0.3 }, drums: 0.5, hats: 0.6 },
  plucky: { feel: { energy: 0.6 }, profiles: { grove: 2, glade: 1.5, dust: 1.5, cinder: 1.3 }, melody: { pluck: 2.5, harp: 2, kalimba: 1.5 }, chords: { nylon: 2, harp: 1.5 }, bass: { pluckbass: 2 }, swing: 0.6 },
  quirky: { feel: { lift: 0.75 }, profiles: { clockwork: 2, bloom: 2, grove: 1.3 }, modes: { lydian: 1.8, wholeTone: 1.5, lydianDominant: 1.5 }, melody: { musicbox: 1.5, prepared: 1.5, moog: 1.5 }, bass: { moogbass: 1.5 } },
  reflective: { feel: { lift: 0.4, energy: 0.22, warmth: 0.46 }, tempo: 0.3, profiles: { thaw: 1.5, haven: 1.5, hollow: 1.3, vapor: 1.3, bloom: 0.5, shatter: 0.4 }, melody: { piano: 1.5, rhodes: 1.5 }, rests: 1.3 },
  refreshing: { feel: { lift: 0.72, energy: 0.62, warmth: 0.42 }, tone: { warmth: 0.35, space: 0.5 }, profiles: { glade: 2, grove: 1.5, tide: 1.5, bloom: 1.3 }, melody: { flute: 1.5, whistle: 1.5, marimba: 1.5 }, texture: { waves: 2, drops: 1.5, wind: 1.5 } },
  restful: { feel: { energy: 0.15, warmth: 0.7 }, tempo: 0.2, profiles: { haven: 2.5, vapor: 2, hollow: 1.5, shatter: 0.1, cinder: 0.2, undertow: 0.3 }, bassStyle: { held: 2 }, drums: 0.3, rests: 1.4, long: 1.3 },
  reverent: { feel: { energy: 0.15 }, tempo: 0.2, tone: { space: 0.8 }, profiles: { shrine: 2.5, hollow: 2.5, haven: 1.3, shatter: 0.1, bloom: 0.3 }, melody: { choir: 2, hum: 2, templebell: 1.5, tubular: 1.5 }, chords: { choir: 2, hum: 1.5, vowel: 1.5 }, drums: 0.2, long: 1.4 },
  serene: { feel: { lift: 0.65, energy: 0.1 }, tempo: 0.15, tone: { space: 0.85 }, profiles: { vapor: 2, haven: 2, shrine: 1.5, thaw: 1.3, shatter: 0.1, cinder: 0.2 }, melody: { sine: 1.5, softpad: 1.5, flute: 1.5 }, bassStyle: { held: 2.5 }, texture: { swell: 2 }, drums: 0.2, rests: 1.3, long: 1.5 },
  silly: { feel: { lift: 0.85, energy: 0.65 }, profiles: { bloom: 3, grove: 1.5 }, melody: { moog: 2.5, whistle: 2, stab: 1.3 }, bassStyle: { walk: 1.5, pulse: 1.3 } },
  skipping: { feel: { energy: 0.7 }, six8: 2.5, swing: 0.7, profiles: { tide: 2.5, glade: 2, cinder: 1.5, wayfare: 1.3 }, melody: { whistle: 1.5, fiddle: 1.5, ocarina: 1.5 }, drums: 1.5 },
  soft: { feel: { energy: 0.3 }, tone: { warmth: 0.65, wobble: 0.2 }, profiles: { haven: 1.5, vapor: 1.5, hollow: 1.3, shatter: 0.3 }, melody: { softpad: 1.5, sine: 1.5, rhodes: 1.5, saw: 0.3, stab: 0.4 }, chords: { softpad: 2, rhodes: 1.5 }, hats: 0.6 },
  soothing: { feel: { lift: 0.62, energy: 0.18, warmth: 0.82 }, tempo: 0.25, profiles: { haven: 2, vapor: 1.5, hollow: 1.5, halcyon: 1.3, shatter: 0.1, cinder: 0.3 }, chords: { softpad: 2, rhodes: 1.5 }, bassStyle: { held: 2 }, drums: 0.3, long: 1.3 },
  sour: { feel: { warmth: 0.4 }, tone: { warmth: 0.35 }, modes: { dorian: 1.8, lydian: 1.5, phrygianDominant: 1.3, harmonicMinor: 1.2 }, profiles: { cinder: 1.5, dust: 1.3, glade: 1.3 }, bigChords: 1.5 },
  spacious: { tone: { space: 0.85 }, profiles: { thaw: 2, vapor: 2, shrine: 1.5, halcyon: 1.3 }, texture: { swell: 1.5, wind: 1.5 }, rests: 1.5, long: 1.3 },
  spicy: { feel: { energy: 0.75, warmth: 0.6 }, profiles: { cinder: 3, wayfare: 1.3 }, modes: { phrygianDominant: 2, harmonicMinor: 1.5 }, melody: { fiddle: 1.5, accordion: 1.5 }, chords: { nylon: 2 }, drums: 1.5 },
  starry: { feel: { energy: 0.25, lift: 0.7 }, tone: { space: 0.8 }, profiles: { vapor: 2, thaw: 2, shrine: 1.5, haven: 1.3 }, melody: { celeste: 2.5, musicbox: 2, bell: 1.5 }, texture: { bells: 2.5, chime: 2 }, drums: 0.4 },
  steady: { profiles: { undertow: 2, wayfare: 2, dust: 1.3 }, swing: 0.2, bassStyle: { pulse: 2 }, drums: 1.5 },
  still: { feel: { energy: 0.05 }, tempo: 0.1, profiles: { vapor: 2.5, shrine: 2, thaw: 2, haven: 1.5, shatter: 0.1, cinder: 0.1, wayfare: 0.2 }, bassStyle: { held: 2.5, sparse: 1.5 }, drums: 0.1, rests: 1.8, long: 1.4 },
  strange: { profiles: { clockwork: 2, vapor: 2, hollow: 1.5, shatter: 1.3 }, modes: { wholeTone: 2.5, hirajoshi: 1.8, dorianSharp4: 1.8, insen: 1.5 }, melody: { prepared: 1.5, vowel: 1.5, saw: 1.3 }, tone: { wobble: 0.6 } },
  sunny: { feel: { lift: 0.9, warmth: 0.8 }, modes: { ionian: 2, majorPent: 1.8, lydian: 1.5 }, profiles: { grove: 2, glade: 1.5, wayfare: 1.5, bloom: 1.5, thaw: 0.4, vapor: 0.5 }, melody: { kalimba: 2, whistle: 1.5, marimba: 1.5, ocarina: 1.5 } },
  swaying: { feel: { energy: 0.45 }, six8: 2.5, swing: 0.7, profiles: { tide: 3, cinder: 1.3, halcyon: 1.3 }, bassStyle: { held: 1.5, walk: 1.3 }, chords: { accordion: 1.5, nylon: 1.5 } },
  sweet: { feel: { lift: 0.8, warmth: 0.75 }, modes: { ionian: 1.8, majorPent: 1.8, lydian: 1.3, phrygian: 0.4, harmonicMinor: 0.5 }, profiles: { grove: 1.5, glade: 1.5, haven: 1.5, shatter: 0.3 }, melody: { musicbox: 2, kalimba: 1.5, celeste: 1.5, ocarina: 1.5 }, bigChords: 0.7 },
  tangy: { modes: { dorian: 1.8, mixolydian: 1.5, lydian: 1.5 }, profiles: { glade: 1.5, grove: 1.5, bloom: 1.3 }, melody: { pluck: 1.5, marimba: 1.5, whistle: 1.3 }, bigChords: 1.4 },
  tart: { tone: { warmth: 0.3 }, modes: { dorian: 2, dorianSharp4: 1.5, lydian: 1.3, phrygian: 1.2 }, profiles: { clockwork: 1.3, glade: 1.3 }, melody: { pluck: 1.5, prepared: 1.3 }, bigChords: 1.6 },
  tender: { feel: { lift: 0.6, energy: 0.2, warmth: 0.85 }, tempo: 0.25, tone: { space: 0.25, warmth: 0.75 }, modes: { ionian: 1.3, lydian: 1.3 }, profiles: { haven: 2.5, glade: 1.5, hollow: 1.3, shatter: 0.1, undertow: 0.3, cinder: 0.3 }, melody: { rhodes: 2, flute: 1.5, harp: 1.5, musicbox: 1.5 }, chords: { softpad: 1.5, rhodes: 1.5, nylon: 1.3 }, bigChords: 0.6, drums: 0.4, rests: 1.3 },
  tiptoeing: { feel: { energy: 0.3 }, tempo: 0.35, profiles: { clockwork: 2, thaw: 1.5, grove: 1.3 }, melody: { pluck: 1.5, prepared: 1.5, musicbox: 1.5, celeste: 1.5 }, bassStyle: { sparse: 2, walk: 1.3 }, drums: 0.5, rests: 1.5 },
  triumphant: { feel: { lift: 0.9, energy: 0.75 }, modes: { ionian: 2, lydian: 1.5, mixolydian: 1.3 }, profiles: { wayfare: 2, halcyon: 1.5, bloom: 1.5, tide: 1.3 }, melody: { analoglead: 1.5, whistle: 1.5, fiddle: 1.5 }, bigChords: 1.8, drums: 1.5, long: 1.3 },
  twinkling: { tone: { space: 0.6 }, profiles: { clockwork: 1.5, thaw: 1.5, haven: 1.3, grove: 1.3 }, melody: { celeste: 2.5, musicbox: 2.5, bell: 2, kalimba: 1.5 }, texture: { bells: 3, chime: 2.5 }, rests: 1.2 },
  velvety: { feel: { energy: 0.3, warmth: 0.85 }, tone: { warmth: 0.85, wobble: 0.1 }, profiles: { halcyon: 2, haven: 1.5, vapor: 1.5, hollow: 1.3, shatter: 0.2 }, melody: { softpad: 2, rhodes: 2, hum: 1.3, saw: 0.3, pluck: 0.4 }, chords: { softpad: 2, analogpad: 2, moogpad: 1.5, choir: 1.3 }, bass: { sub: 1.5, round: 1.5 }, bigChords: 1.6 },
  warm: { feel: { warmth: 0.9 }, tone: { warmth: 0.85 }, profiles: { halcyon: 2, dust: 1.5, haven: 1.5, thaw: 0.4 }, melody: { rhodes: 1.5, keys: 1.5, hum: 1.3 }, chords: { rhodes: 1.5, analogpad: 1.5 }, bass: { round: 1.5 } },
  wistful: { feel: { lift: 0.45, energy: 0.3 }, profiles: { thaw: 1.5, dust: 1.5, halcyon: 1.3, haven: 1.3 }, modes: { dorian: 1.5, mixolydian: 1.3, aeolian: 1.2 }, melody: { piano: 1.5, rhodes: 1.5, flute: 1.3 } },
  wondrous: { feel: { lift: 0.75, warmth: 0.75, energy: 0.4 }, tone: { space: 0.75 }, modes: { lydian: 2.5, lydianDominant: 1.5 }, profiles: { glade: 1.5, shrine: 1.5, hollow: 1.5, haven: 1.3 }, melody: { celeste: 1.5, harp: 1.5, choir: 1.5, ocarina: 1.5 }, texture: { swell: 1.5, bells: 1.5 } },
  zany: { feel: { lift: 0.8, energy: 0.85 }, tempo: 0.85, profiles: { bloom: 2.5, shatter: 2, clockwork: 1.5 }, modes: { wholeTone: 1.5, lydianDominant: 1.5 }, melody: { moog: 2, prepared: 1.5, saw: 1.3 }, drums: 1.5, hats: 1.5 },
  zesty: { feel: { lift: 0.75, energy: 0.7 }, tone: { warmth: 0.35 }, modes: { lydian: 1.5, mixolydian: 1.5, dorian: 1.3 }, profiles: { bloom: 1.5, grove: 1.5, cinder: 1.5, wayfare: 1.3 }, melody: { whistle: 1.5, pluck: 1.5, marimba: 1.5 }, hats: 1.5 },
};
```

## The vibes to 88 (brain, Session 26, 2026-10-29; queue item 76b)

**Decided by Mikey (2026-10-29):** the vibes go from 71 to 88. Twenty-one
join: glacial (designed with him on 2026-10-26) and twenty the brain proposed
from the gaps in the list and the records the ears still have to hear. Four
leave: **soft** (it fit 18 of 23 loops in the ranking test, so it named
Driftloom rather than a loop; mellow and chill take its place), **energetic**
(enthusiastic and lively stay; lively takes its drive), **silly** (goofy
stays) and **comfy** (cozy stays). He turned down *nostalgic* ("too many
meanings") and *sleepy* (restful and soothing stay; restful takes the
bedtime role). Old codes are not kept working: the four leave the list
outright, with no retired slots.

**Why each one** (the gaps it fills):
- **Time of day**, for Broken Friends' music that follows the hours: dewy
  (early morning), dusky (the warm dim hour before night), moonlit (cool,
  silvery night). Sunny and starry were the only two before.
- **Weather:** rainy (cozy, not gloomy; the commonest lo-fi weather).
- **What Driftloom is about:** whimsical (the Ocarina of Time sense: catchy,
  childlike magic, where goofy and quirky are jokes), dreamy (Boards of
  Canada, Ulrich Schnauss, Bowery Electric), chill (relaxed *with* a beat;
  soothing and restful are mostly drumless), mellow (soft's gentle side,
  drumless).
- **Groove**, for the ears' coming records (Basic Channel, The Field,
  Monoton, LFO, Neu!, Harmonia): groovy (a loose, swung bass and kit),
  pulsing (a steady electronic throb), cruising (motorik, the open road).
- **Sound worlds from the records:** droning (Eno, Steve Roach, Kankyo
  Ongaku; glacial's neighbor), cosmic (arpeggiated synth space: Pauline Anna
  Strom, Iasos, Solar Fields), bubbly (the Analogue Bubblebath side of
  Aphex), earthy (wood, hand drums, folk warmth), soaring (Beethoven 9, Jaga
  Jazzist), jazzy (Miles Davis, Jaga Jazzist).
- **Motion and air:** breezy (moving air, where airy stays still and open),
  rippling (water and arpeggios), playful (broad and light, under whimsical
  and cheeky).

**Waiting on the ears:** groovy, pulsing, cruising, cosmic, rainy and jazzy
will get much better once the ears hear their records (grooves, arpeggios, a
rain texture, extended chords). Their first recipes lean on what exists.

**The drafts** (every name checked against `test/generator.test.mjs`; the
hands tune them to the yardsticks in item 76b and record what moved):

```js
  glacial: { feel: { energy: 0.05, warmth: 0.3, lift: 0.5 }, tempo: 0.05, tone: { space: 0.9, warmth: 0.3 }, profiles: { vapor: 15.6, thaw: 8, shrine: 3.4, hollow: 3.4, shatter: 0.05, cinder: 0.05, wayfare: 0.05, bloom: 0.05 }, chords: { softpad: 5.7, pad: 5.7, moogpad: 2.8 }, bassStyle: { held: 9.9, sparse: 2.8, pulse: 0.2, walk: 0.1 }, texture: { swell: 9.9, wind: 5.7 }, drums: 0.1, rests: 1.6, long: 1.8 },
  dewy: { feel: { lift: 0.68, energy: 0.3, warmth: 0.45 }, tempo: 0.3, tone: { space: 0.6, warmth: 0.4 }, profiles: { glade: 8, thaw: 8, haven: 3.4, cinder: 0.05, shatter: 0.05 }, melody: { flute: 5.7, celeste: 2.8, harp: 2.8, kalimba: 2.8 }, texture: { drops: 9.9, wind: 2.8 }, drums: 0.4, rests: 1.3 },
  dusky: { feel: { lift: 0.45, energy: 0.28, warmth: 0.75 }, tempo: 0.3, tone: { warmth: 0.75, space: 0.6, wobble: 0.3 }, modes: { dorian: 1.5, mixolydian: 1.5, aeolian: 1.3 }, profiles: { halcyon: 8, dust: 8, haven: 3.4, hollow: 3.4, bloom: 0.1 }, melody: { rhodes: 5.7, keys: 2.8, hum: 2.8 }, chords: { rhodes: 5.7, analogpad: 2.8 }, texture: { swell: 2.8 }, drums: 0.6 },
  moonlit: { feel: { lift: 0.55, energy: 0.18, warmth: 0.3 }, tempo: 0.2, tone: { space: 0.8, warmth: 0.3 }, profiles: { thaw: 8, vapor: 8, shrine: 3.4, haven: 3.4, cinder: 0.05, wayfare: 0.05 }, melody: { piano: 5.7, celeste: 2.8, sine: 2.8, musicbox: 2.8 }, chords: { softpad: 5.7, pad: 2.8 }, texture: { bells: 2.8, swell: 2.8 }, drums: 0.3, rests: 1.4 },
  rainy: { feel: { lift: 0.45, energy: 0.3, warmth: 0.6 }, tone: { warmth: 0.6, space: 0.5, wobble: 0.3 }, profiles: { dust: 8, haven: 8, thaw: 3.4, halcyon: 3.4 }, melody: { piano: 5.7, rhodes: 5.7, keys: 2.8 }, chords: { rhodes: 5.7, softpad: 2.8 }, texture: { drops: 20, waves: 2.8 }, drums: 0.7, hats: 0.6 },
  whimsical: { feel: { lift: 0.8, energy: 0.5, warmth: 0.7 }, modes: { lydian: 1.8, majorPent: 1.5, ionian: 1.3 }, profiles: { glade: 8, clockwork: 8, grove: 3.4, tide: 3.4, undertow: 0.1 }, melody: { ocarina: 9.9, musicbox: 5.7, kalimba: 5.7, celeste: 2.8, whistle: 2.8 }, chords: { harp: 2.8 }, texture: { bells: 2.8, chime: 2.8 }, rests: 1.2 },
  dreamy: { feel: { lift: 0.6, energy: 0.3, warmth: 0.65 }, tempo: 0.3, tone: { space: 0.8, wobble: 0.55, warmth: 0.6 }, modes: { lydian: 1.8, majorPent: 1.3 }, profiles: { dust: 8, vapor: 8, halcyon: 3.4, hollow: 3.4 }, melody: { softpad: 5.7, rhodes: 2.8, celeste: 2.8, sine: 2.8 }, chords: { softpad: 5.7, analogpad: 2.8, moogpad: 2.8 }, texture: { swell: 5.7 }, long: 1.3 },
  chill: { feel: { energy: 0.4, warmth: 0.7 }, tempo: 0.4, tone: { warmth: 0.7, wobble: 0.35 }, swing: 0.6, profiles: { dust: 15.6, halcyon: 8, haven: 3.4, grove: 3.4, shatter: 0.05, cinder: 0.1 }, melody: { rhodes: 5.7, keys: 5.7, piano: 2.8 }, chords: { rhodes: 5.7 }, bass: { round: 2.8, sub: 2.8 }, bassStyle: { pulse: 2.8, walk: 1.9 }, drums: 1.4 },
  groovy: { feel: { energy: 0.6, warmth: 0.7 }, swing: 0.7, profiles: { grove: 8, dust: 8, bloom: 3.4, undertow: 3.4, vapor: 0.05, haven: 0.05 }, melody: { rhodes: 2.8, moog: 2.8, keys: 2.8 }, bass: { moogbass: 5.7, round: 2.8 }, bassStyle: { walk: 5.7, dub: 2.8, held: 0.2 }, drums: 2, hats: 1.5 },
  pulsing: { feel: { energy: 0.55 }, swing: 0.15, profiles: { undertow: 15.6, clockwork: 3.4, vapor: 3.4 }, chords: { analogpad: 2.8, moogpad: 2.8 }, bass: { sub: 5.7, moogbass: 2.8 }, bassStyle: { pulse: 9.9 }, drums: 1.8, hats: 1.3, rests: 0.5, long: 0.8 },
  cruising: { feel: { lift: 0.7, energy: 0.6 }, tempo: 0.6, swing: 0.2, profiles: { wayfare: 15.6, undertow: 3.4, tide: 3.4, vapor: 0.05, haven: 0.05 }, melody: { analoglead: 2.8, whistle: 2.8, flute: 2.8 }, bassStyle: { pulse: 5.7 }, drums: 1.8, rests: 0.5, long: 1.3 },
  droning: { feel: { energy: 0.08 }, tempo: 0.1, tone: { space: 0.75 }, profiles: { shrine: 15.6, hollow: 8, vapor: 8, shatter: 0.05, cinder: 0.05, bloom: 0.05 }, melody: { hum: 5.7, sine: 2.8 }, chords: { hum: 5.7, softpad: 5.7, analogpad: 2.8, accordion: 2.8 }, bassStyle: { held: 20 }, drums: 0.1, rests: 1.4, long: 1.8 },
  cosmic: { feel: { lift: 0.65, energy: 0.4 }, tone: { space: 0.85 }, modes: { lydian: 2, dorianSharp4: 1.3, wholeTone: 1.2 }, profiles: { vapor: 8, clockwork: 8, hollow: 3.4, undertow: 3.4 }, melody: { celeste: 2.8, analoglead: 2.8, sine: 2.8, vowel: 1.9 }, chords: { analogpad: 5.7, moogpad: 5.7 }, texture: { swell: 5.7, bells: 2.8 }, drums: 0.6 },
  bubbly: { feel: { lift: 0.8, energy: 0.65 }, tone: { warmth: 0.45 }, profiles: { bloom: 15.6, clockwork: 8, grove: 3.4, vapor: 0.1, haven: 0.1 }, melody: { pluck: 5.7, marimba: 5.7, kalimba: 2.8, moog: 2.8 }, bass: { moogbass: 2.8 }, texture: { drops: 9.9 }, hats: 1.5, rests: 0.7 },
  earthy: { feel: { energy: 0.45, warmth: 0.75 }, tone: { warmth: 0.7, space: 0.35 }, profiles: { tide: 8, grove: 8, glade: 3.4, cinder: 3.4, vapor: 0.1 }, melody: { marimba: 5.7, kalimba: 5.7, fiddle: 2.8, flute: 2.8 }, chords: { nylon: 5.7, accordion: 2.8 }, bass: { pluckbass: 2.8, round: 2.8 }, drums: 1.3 },
  soaring: { feel: { lift: 0.85, energy: 0.6 }, tone: { space: 0.75 }, modes: { ionian: 1.8, lydian: 1.8 }, profiles: { wayfare: 8, glade: 8, halcyon: 3.4, tide: 3.4 }, melody: { flute: 5.7, whistle: 5.7, choir: 2.8, fiddle: 2.8 }, chords: { pad: 2.8, choir: 1.9 }, bigChords: 1.6, long: 1.4 },
  jazzy: { feel: { energy: 0.45, warmth: 0.75 }, swing: 0.75, modes: { dorian: 1.8, mixolydian: 1.8, lydianDominant: 1.3 }, profiles: { halcyon: 8, dust: 8, haven: 3.4 }, melody: { rhodes: 5.7, piano: 5.7, keys: 2.8 }, chords: { rhodes: 9.9 }, bass: { rhodesbass: 2.8, round: 2.8 }, bassStyle: { walk: 9.9 }, bigChords: 1.8 },
  breezy: { feel: { lift: 0.75, energy: 0.5, warmth: 0.5 }, tempo: 0.55, tone: { space: 0.6, warmth: 0.4 }, swing: 0.5, profiles: { glade: 8, tide: 8, grove: 3.4, thaw: 3.4 }, melody: { flute: 5.7, whistle: 2.8, harp: 2.8 }, chords: { nylon: 2.8, harp: 2.8 }, texture: { wind: 9.9 }, rests: 0.8 },
  rippling: { feel: { lift: 0.65, energy: 0.45 }, tone: { space: 0.65 }, profiles: { clockwork: 8, thaw: 8, tide: 3.4, glade: 3.4 }, melody: { harp: 9.9, celeste: 5.7, marimba: 2.8, piano: 2.8 }, chords: { harp: 5.7 }, texture: { waves: 5.7, drops: 2.8 }, rests: 0.7 },
  playful: { feel: { lift: 0.8, energy: 0.62 }, swing: 0.6, profiles: { bloom: 8, grove: 8, glade: 3.4, clockwork: 3.4, vapor: 0.1 }, melody: { kalimba: 5.7, marimba: 5.7, whistle: 2.8, pluck: 2.8, ocarina: 2.8 }, drums: 1.3, rests: 0.9 },
  mellow: { feel: { energy: 0.25, warmth: 0.75 }, tempo: 0.3, tone: { warmth: 0.75, space: 0.45, wobble: 0.2 }, profiles: { haven: 8, halcyon: 8, dust: 3.4, hollow: 2.2, shatter: 0.05, cinder: 0.05 }, melody: { rhodes: 5.7, softpad: 2.8, piano: 2.8, hum: 2.8 }, chords: { softpad: 5.7, rhodes: 2.8 }, drums: 0.2 },
```

**What the brain measured on the drafts** (300 loops each, the vibe alone;
no vibes: median 95 bpm): glacial 60 bpm, thaw and vapor lead; droning 65
bpm, shrine leads; moonlit 68; cruising 121, wayfare leads; pulsing 106,
undertow leads; bubbly bloom; jazzy and chill dust. Two things to fix in
tuning: the calm vibes still draw a kit in 20-35% of loops, and **foggy and
misty draw the same profiles in the same shares** (dust 15%, vapor 11%,
thaw 11%), which item 76b separates.

**As built (queue item 76b, v125).** `js/vibes.js` holds 88; `VIBE_IDS`
is rewritten in alphabetical order, so codes made before v125 may play
other vibes or none (Mikey: no bloat for compatibility). What the hands
moved from the drafts and from `main`, to meet the item's yardsticks:

- **Drums, the calm five** (under 15% of loops): the lean is on the odds of
  a kit and raised to the loop's strength, so a draft's 0.1 still let a kit
  into 20-41% of loops. glacial 0.1 -> 0.01, droning 0.1 -> 0.01, moonlit
  0.3 -> 0.01, mellow 0.2 -> 0.005, restful 0.15 (the item's fold) -> 0.02.
- **Drums, the beat four** (over 70%): chill 1.4 -> 2.8, pulsing 1.8 -> 3.5,
  cruising 1.8 -> 2.2 (margin; it passed); groovy as drafted.
- **moonlit** tempo 0.2 -> 0.15 (into the slowest tenth with room).
- **rippling** clockwork 8 -> 15.6, glade 3.4 -> 2.2 (thaw and clockwork were
  not leading).
- **mellow** tempo 0.3 -> 0.2, dust 3.4 -> 1 (dust led it, as it leads chill).
- **The folds:** lively tempo 0.7 -> 0.85, drums 1.8 -> 2.5, hats 1.5 -> 2,
  shatter 3.4; restful tempo 0.2 -> 0.15, melody music box and hum 5.7,
  drums 0.3 -> 0.02.
- **foggy** (darker, denser): lift 0.45 -> 0.3, energy 0.25 -> 0.22, tempo
  0.3 -> 0.2, space 0.75 -> 0.9; vapor 15.6 and hollow 8 lead (dust 1, thaw
  0.3); moogpad 9.9; swell 15.6, wind 2.8; rests 0.6; the deep arc (it was
  already on the arc's calm list).
- **misty** (lighter, dewier): lift 0.68, tempo 0.5, space 0.65, wobble 0.3;
  thaw and glade lead (dust 0.3); celeste and flute 5.7; drops 9.9, wind
  5.7; the dip arc (still on the calm list for the arc and its period, but
  dip x2 in place of deep).
- **experimental** (the unusual sounds): no modes lean; saw, prepared and
  vowel (below), wobble 0.85; clockwork and shatter lead.
- **strange** (the odd scales): its modes kept; prepared, vowel and saw
  x0.3, no wobble target; hollow 15.6 and vapor lead (clockwork 0.3).
- **airy** (still, open): energy 0.25, tempo 0.25, long 1.3, drums 0.3, thaw
  15.6 and vapor 8 lead.
- **Refusals at lite** (150 loops each, against the catalog's 1.3% of the
  tune on the same seeds): the expensive tune voices (softpad, piano,
  vowel, choir) ate the budget in the worst loops. glacial melody sine 5.7,
  softpad 0.2 (2.3% -> 1.1%); pulsing softpad 0.05, vowel 0.2, choir 0.3,
  hats 1.3 -> 1 (2.1% -> 1.0%); rippling piano 2.8 -> 0.3, bigChords 0.7
  (1.5% -> 0.8%); breezy flute and whistle 5.7, harp 2.8 -> 1, piano 0.3,
  vowel 0.2, rests 0.8 -> 1, and thaw out of its profiles (wayfare 2.2,
  vapor and shatter 0.1 in) (2.0% -> 1.2%); mellow softpad 2.8 -> 0.3
  (1.8% -> 1.3%); experimental saw 9.9 first, prepared 5.7, vowel 2.8,
  bigChords 0.6 (3.0% -> 2.2%; main's experimental was 2.8%).
- Everything else is the draft as written.

Each new vibe alone, 300 loops (`stats.mjs`'s corpus seed 1; drums and
tunes rendered):

| vibe | median bpm | drums | top profiles | top tunes |
|---|---|---|---|---|
| breezy | 97 | 67% | glade 24%, tide 20%, grove 12% | whistle 24%, fiddle 11%, ocarina 8% |
| bubbly | 98.5 | 65% | bloom 27%, grove 11%, clockwork 11% | moog 17%, whistle 13%, pluck 8% |
| chill | 79.5 | 74% | dust 36%, halcyon 12%, haven 8% | keys 23%, pluck 10%, rhodes 6% |
| cosmic | 85 | 48% | clockwork 17%, undertow 12%, vapor 9% | sine 13%, prepared 9%, fiddle 9% |
| cruising | 121 | 88% | wayfare 27%, undertow 13%, tide 11% | whistle 21%, fiddle 11%, ocarina 9% |
| dewy | 83.5 | 41% | glade 27%, thaw 14%, haven 9% | flute 9%, musicbox 9%, fiddle 8% |
| dreamy | 78 | 64% | dust 30%, vapor 10%, halcyon 8% | sine 14%, pluck 9%, fiddle 7% |
| droning | 68 | 9% | shrine 16%, hollow 11%, glade 10% | sine 16%, fiddle 8%, hum 8% |
| dusky | 79 | 58% | dust 29%, halcyon 17%, haven 7% | keys 16%, sine 10%, pluck 9% |
| earthy | 96 | 66% | tide 20%, grove 19%, glade 14% | fiddle 17%, kalimba 11%, marimba 11% |
| glacial | 57 | 6% | vapor 19%, thaw 16%, glade 11% | sine 30%, piano 9%, fiddle 8% |
| groovy | 92 | 78% | dust 27%, grove 11%, undertow 10% | keys 11%, sine 8%, pluck 8% |
| jazzy | 88 | 67% | dust 29%, halcyon 15%, haven 7% | keys 17%, pluck 9%, fiddle 7% |
| mellow | 84 | 8% | halcyon 22%, haven 14%, glade 10% | rhodes 12%, sine 10%, analoglead 9% |
| moonlit | 69 | 9% | thaw 16%, vapor 12%, glade 10% | piano 20%, sine 10%, musicbox 10% |
| playful | 92.5 | 64% | bloom 19%, grove 17%, glade 15% | whistle 19%, kalimba 10%, marimba 8% |
| pulsing | 99 | 77% | undertow 27%, dust 8%, glade 7% | sine 12%, fiddle 9%, bell 9% |
| rainy | 82 | 55% | dust 28%, haven 12%, thaw 8% | keys 13%, rhodes 9%, pluck 9% |
| rippling | 84 | 51% | clockwork 22%, tide 13%, thaw 12% | celeste 12%, fiddle 10%, musicbox 10% |
| soaring | 105 | 67% | glade 24%, tide 13%, wayfare 13% | whistle 19%, fiddle 17%, flute 8% |
| whimsical | 89 | 60% | glade 24%, clockwork 15%, tide 13% | ocarina 22%, musicbox 16%, whistle 14% |

Slowest tenth of the 88: glacial 57, still 59, meditative 62, restful 67,
droning 68, moonlit 69, floating 70, serene 70.5, hushed 71. Fastest:
triumphant 109, joyful 110, booming 111, zany 113, spicy 115.5,
enthusiastic 118, cruising 121, adventurous 122.5, lively 124.5.

The pairs (2 of 4 needed: lead profile, 8 bpm, 20 points of drums, top
tune): foggy/misty vapor/glade, 82/92 bpm, tune sine/music box (3);
experimental/strange clockwork/hollow, 91.5/83.5 bpm, prepared/sine (3);
airy/breezy thaw/glade, 80.5/97 bpm, 40/67% drums, sine/whistle (4);
mellow/chill halcyon/dust, 8/74% drums, rhodes/keys (3). Checked on corpus
seeds 1-3; each pair kept 2 or more on every seed.

Glacial's arc: deep 72% of its arcs, 5-8 minutes alone and 8-12 with an
adjacent vibe, as asked; but the arc in only 31% of its loops, not half
(`docs/notes/76b-vibes.md`). For Mikey's ears: every recipe above, and
glacial's adjacent list.

## Tuning log

**v91 (brain, Session 16, 2026-10-12): profile and instrument leans
strengthened.** `stats.mjs --vibe-each` on v90 showed the targets
working (tempo, drums, mode brightness) but the leans on profiles and
voices hardly moving anything: with a lone vibe, glade still led most
vibes and fiddle most tunes, as with no vibes at all. A lean is applied as
`value ^ (strength x weight)`, and a strength of 0.5-0.9 shrank the
drafts' 1.5-3 to barely more than 1. So every `profiles` value was cubed
and every `melody`, `chords`, `bass`, `bassStyle` and `texture` value
raised to the 2.5th power (kept between 0.05 and 20); the targets, modes
and the single-number leans are as drafted. `js/vibes.js` holds the
numbers now; the block above is the first draft, kept for the record.
After: each vibe on its own leads with its own profiles and voices --
floating vapor and sine, frosty thaw and music box, cozy dust and rhodes,
tender haven and rhodes, joyful a whistle a third of the time, zany bloom
and moog, reverent hollow and hum, spicy cinder. Still mild: reflective,
peaceful, sour, tart, soft and mysterious (no strong profile of their
own); for Mikey's ears. Five vibes at once still average out, as built.
Loops without vibes are unchanged (balance lock, tests).
