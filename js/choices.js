// The per-loop choices (queue item 54): the one place that decides which of
// the sound batch's changes a loop gets.
//
// Mikey's rules for every choice (2026-10-18; docs/QUEUE.md, "Before phase
// 2: the sound batch"):
//
// - today's behavior stays one option of every choice, never replaced;
// - each choice is drawn per loop, from the loop's seed on a stream of its
//   own, so no other draw moves and a loop replays the same way;
// - most loops get one or two of the changes.
//
// So a loop first draws how many changes it gets (CHANGE_COUNTS), then which
// of the choices it is eligible for (weighted by the vibes, without
// repeats), then each chosen one's option other than today's. Every other
// choice stays today. A choice the loop is not eligible for is never drawn,
// so a loop eligible for fewer changes than it drew gets all it can.
//
// Each later item of the batch adds its choice to CHOICES: an id, its
// options (today's first), an eligibility test and the vibes it leans
// toward or away from. Nothing here goes in the share code: the code's seed
// gives the draws, and "Let the loop wander" never redraws them (item 67's
// kick swap is the one exception). Pins come with phase 2's requests.

import { Rng } from './rng.js';

const CHOICE_SALT = 0x43484f49;

// How many changes a loop gets: none 15%, one 45%, two 30%, three 10%. A
// taste call, for Mikey's ears: most loops get one or two (his rule), a few
// stay as they are, and three is rare enough that a loop never feels like a
// demo of the batch.
export const CHANGE_COUNTS = [[0, 15], [1, 45], [2, 30], [3, 10]];

// A choice's lean for one vibe: toward x1.5 to x3, away x0.2 to x0.5, never
// 0 (docs/ears/batch-5.md, "The [JUDGMENT] lines"). A vibe not named keeps x1.
const LEAN_MIN = 0.2;
const LEAN_MAX = 3;

// The arc's settings (queue items 63 and 63b; Mikey, 2026-10-21 and 22):
// every variable of the arc, named, with the range a loop draws it from, in
// this one place, so phase 2's requests can pin them (docs/CORE.md, promises
// 9 and 11) and a later slider or a game host can set them. `arcOf` draws a
// loop's from them; the core plays it (core/src/blocks/arc.rs).
//
// The two shapes: `deep` is the prototype's swing, 300 Hz to 9 kHz and back,
// lingering dark (under 700 Hz for about half of every swing); `dip` is open
// at 9 kHz but for one smooth dip, over `dip` of the swing, to `floor`.
// Calm, vast vibes lean deep and long; bright, lively ones lean dip and short.
const ARC_CALM = ['hazy', 'foggy', 'misty', 'frosty', 'still', 'meditative', 'spacious', 'floating', 'hypnotic', 'mysterious', 'reflective', 'wondrous'];
const ARC_LIVELY = ['bouncy', 'lively', 'energetic', 'skipping', 'zany'];
const ARC_BRIGHT = ['sunny', 'bright', 'happy', 'joyful', 'merry', ...ARC_LIVELY];
// The vibe leans below: toward x2 and away x0.4, the middle of the rule's
// ranges (Mikey, 2026-10-24).
const leans = (ids, x) => Object.fromEntries(ids.map((id) => [id, x]));

export const ARC = {
  // How often a loop draws the arc: its weight among the choices a loop is
  // eligible for (item 54), and the vibes that lean it on or away.
  weight: 1,
  vibes: { ...leans(ARC_CALM, 2), ...leans(ARC_LIVELY, 0.4) },
  // Which shape: deep about 1 in 3, dip about 2 in 3, each leaned by vibes.
  shapes: [['deep', 1], ['dip', 2]],
  shapeVibes: { deep: leans(ARC_CALM, 2), dip: leans(ARC_BRIGHT, 2) },
  // The swing's period, in seconds: whichever of the calm and bright lists
  // a loop's vibes weigh more picks its range; neither, the rest's.
  period: { calm: [180, 300], bright: [90, 150], rest: [120, 300] },
  calm: ARC_CALM,
  bright: ARC_BRIGHT,
  // Where a loop starts on its swing (0 the darkest point, 0.5 the
  // brightest): deep, a bright point (the cutoff 4.9 to 9 kHz; the
  // prototype started every loop closed, at 300 Hz: a taste call, for
  // Mikey's ears); dip, anywhere in the open part.
  from: { deep: [0.4, 0.6] },
  // The dip's floor, in Hz.
  floor: [800, 1500],
  // The share of the swing the dip takes (open at 9 kHz for the rest). Kept
  // at 35% for every period (Mikey, 2026-10-24): within twice its floor for
  // 15-45 s of a 2-5 minute swing, 11-22 s of a bright loop's short one.
  dip: 0.35,
};

// The choices, in a fixed order (the draw walks them in it). Each is
//
//   { id, options: [['today', w], [option, w], ...], weight,
//     eligible: (ctx) => bool, vibes: { vibe: lean, ... } }
//
// `weight` is how often the choice is picked among those a loop is eligible
// for, before the vibes lean it (1 if missing); `options` past the first
// are what it plays when picked, by their weights. `ctx` is what the
// generator knows of the loop when it draws (see `choicesOf`).
export const CHOICES = [
  // glass (queue item 59; the prototypes, approved by Mikey 2026-10-10): on
  // a chord's first beat, now and then, two or three of its notes ring out
  // high as a soft FM bell. For any loop with a chord of two notes or more
  // (a one-note "chord" never rings out). No vibe leans: the batch's table
  // has none for the instruments (for Mikey's ears).
  { id: 'glass', options: [['today', 1], ['on', 1]], eligible: (ctx) => hasChords(ctx) },
  // dubStab (queue item 60; Ears 4, round 4, approved 2026-10-16): a soft
  // chord stab on the offbeat over a busy groove, thrown into the echo.
  // Busy loops only: drums, and seven or more notes a second.
  { id: 'dubStab', options: [['today', 1], ['on', 1]], eligible: (ctx) => playsChords(ctx) && busy(ctx) },
  // The chapel organ (queue item 61; Ears 4, round 7, approved 2026-10-16):
  // a pipe organ the loop holds its chords on, outside the voice budget.
  // For any loop that plays a chord. No vibe leans (for Mikey's ears).
  { id: 'organ', options: [['today', 1], ['on', 1]], eligible: (ctx) => playsChords(ctx) },
  // The wash (queue item 62; Ears 4, round 5, approved 2026-10-16): a slow
  // swell of air every 17-26 s, under whatever the loop plays. Any loop.
  { id: 'wash', options: [['today', 1], ['on', 1]], eligible: () => true },
  // The arc (queue items 63 and 63b; the prototypes, approved 2026-10-10):
  // the whole mix breathing dark and bright over minutes, in one of two
  // shapes (ARC, above; `arcOf`). Any loop.
  { id: 'arc', options: [['today', 1], ['on', 1]], weight: ARC.weight, vibes: ARC.vibes, eligible: () => true },
];

// How many notes a second the loop plays, every layer, as heard.
export function notesPerSecond(spec, tracks) {
  const spb = spec.stepsPerBar || 16;
  const seconds = spec.bars * spb * (60 / spec.bpm / 4);
  let n = 0;
  for (const layer of Object.keys(tracks)) n += tracks[layer].filter((e) => e.vel).length;
  return n / seconds;
}

// A busy groove (dubStab's test): drums, and seven or more notes a second.
function busy(ctx) {
  return !!ctx.tracks && ctx.tracks.drums.some((e) => e.vel) && notesPerSecond(ctx.spec, ctx.tracks) >= 7;
}

// Whether the loop plays a chord with notes (dubStab stabs whatever sounds).
function playsChords(ctx) {
  return !!ctx.tracks && ctx.tracks.chords.some((e) => e.vel && e.notes && e.notes.length);
}

// Whether the loop plays a chord of two notes or more.
function hasChords(ctx) {
  return !!ctx.tracks && ctx.tracks.chords.some((e) => e.vel && e.notes && new Set(e.notes).size >= 2);
}

// The lean of a choice for a loop's vibes, each vibe's raised to its
// strength times its weight, as the generator leans its pools (vibeLean).
function lean(choice, vibes) {
  return leanOf(choice.vibes, vibes);
}

// The same for any table of leans ({ vibe: lean }).
function leanOf(table, vibes) {
  if (!vibes || !table) return 1;
  let m = 1;
  for (const [id, w] of Object.entries(vibes.vibes)) {
    const x = table[id];
    if (typeof x !== 'number') continue;
    m *= Math.pow(Math.min(LEAN_MAX, Math.max(LEAN_MIN, x)), vibes.strength * w);
  }
  return m;
}

// The choices for a loop: every choice's option, `today` for those it did
// not draw. `seed` is the loop's; `vibes` its vibes ({ vibes, strength }, or
// null); `ctx` what the eligibility tests read. `list` is CHOICES unless a
// test hands its own.
export function drawChoices(seed, ctx, vibes = null, list = CHOICES) {
  const out = {};
  for (const c of list) out[c.id] = c.options[0][0];
  if (!list.length) return out;
  const r = new Rng(((seed ^ CHOICE_SALT) >>> 0) || 1);
  const count = r.weighted(CHANGE_COUNTS);
  let pool = list.filter((c) => c.eligible(ctx)).map((c) => [c, (c.weight ?? 1) * lean(c, vibes)]);
  for (let k = 0; k < count && pool.length; k++) {
    const c = r.weighted(pool);
    pool = pool.filter(([x]) => x !== c);
    const others = c.options.slice(1);
    out[c.id] = others.length === 1 ? others[0][0] : r.weighted(others);
  }
  return out;
}

// How many of a loop's choices are not today.
export function changesIn(choices, list = CHOICES) {
  return list.filter((c) => choices[c.id] !== c.options[0][0]).length;
}

// The arc's own stream (the salt item 63 drew its period and start from).
const ARC_SALT = 0x0a4c;

// A loop's arc, drawn from ARC on a stream of its own: { shape, period,
// from, floor, dip }. `vibes` is the loop's ({ vibes, strength }, or null);
// `pins` may fix any of the five, as phase 2's requests will. Every draw is
// made whatever is pinned, in the same order, so a pin moves nothing else.
// The period and the start are drawn first, as item 63 drew them, so a loop
// with no vibes that draws the deep swing plays exactly the swing it did.
export function arcOf(seed, vibes = null, pins = {}) {
  const r = new Rng(((seed ^ ARC_SALT) >>> 0) || 1);
  const [lo, hi] = ARC.period[arcPace(vibes)];
  const period = r.range(lo, hi);
  const at = r.f();
  const shapes = ARC.shapes.map(([id, w]) => [id, w * leanOf(ARC.shapeVibes[id], vibes)]);
  const drawn = r.weighted(shapes);
  const shape = pins.shape ?? drawn;
  const floor = r.range(ARC.floor[0], ARC.floor[1]);
  const dip = pins.dip ?? ARC.dip;
  // Deep: a bright point, as item 63 drew it (Rng.range's arithmetic, so the
  // same bits). Dip: anywhere in the open part, outside the dip's share
  // around the darkest point.
  const [a, b] = shape === 'deep' ? ARC.from.deep : [dip / 2, 1 - dip / 2];
  const from = a + at * (b - a);
  return { shape, period: pins.period ?? period, from: pins.from ?? from, floor: pins.floor ?? floor, dip };
}

// Which period range a loop's vibes pick: 'calm', 'bright' or 'rest'.
function arcPace(vibes) {
  if (!vibes) return 'rest';
  let calm = 0;
  let bright = 0;
  for (const [id, w] of Object.entries(vibes.vibes)) {
    if (ARC.calm.includes(id)) calm += w;
    if (ARC.bright.includes(id)) bright += w;
  }
  return calm > bright ? 'calm' : bright > calm ? 'bright' : 'rest';
}
