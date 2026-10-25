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

// The tuned kick's settings (queue item 67; Ears 8's tunedKick, approved by
// Mikey 2026-10-18; docs/ears/batch-5.md): every variable of it, named, with
// its range, in this one place, as ARC's are. `kickOf` draws a loop's from
// them and works out its level at load; the core plays it
// (core/src/blocks/kick.rs).
//
// The kick and the soft kick keep today's sweep and land on the key's note
// between G1 and D2, where the records' kicks sit; a key of D# to F# takes
// its fifth. Short is today's shape falling at `short` dB a second; long is
// today's punch (213 dB a second) down to a tail that rings on at `long`.
export const KICK = {
  // Short and long, half and half; long leans toward booming and the rest of
  // batch 5's table, away from the hushed and the soothing.
  options: [['today', 1], ['short', 1], ['long', 1]],
  optionVibes: {
    long: { booming: 2.5, hypnotic: 2, steady: 1.5, cool: 1.5, hushed: 0.4, tiptoeing: 0.4, twinkling: 0.4, serene: 0.5, soothing: 0.5 },
  },
  // The note: the lowest and highest, as MIDI notes (G1, D2).
  note: [31, 38],
  // The falls, in dB a second.
  short: [180, 260],
  long: [45, 80],
  // Level by ear: each kick's K-weighted energy against today's kick's
  // (`loud`, 1: no louder), and a long boom's tail as loud as fits within
  // `tailLoud` of today's by ear (1.26: +1 dB) and `band` of today's kick's
  // own energy under 60 Hz and at 60-120 Hz (2: +3 dB).
  loud: 1,
  tailLoud: 1.26,
  band: 2,
  // A wandering loop that plays a tuned kick may change its kick at the top
  // of a pass ("Let the loop wander"; Mikey, 2026-10-18), to today's or the
  // other tuned one, half and half: `swap` is how often a pass does (a taste
  // call, for Mikey's ears). The pass's first kick chokes the last one
  // still ringing, as every tuned kick's next hit does.
  swap: 0.25,
};

// subDuck's settings (queue item 68; Ears 8's subDuck, approved by Mikey
// 2026-10-18): the bass channel alone dips at each kick and comes back, so
// the kick and the sub take turns. Every variable, named, with its range;
// `subDuckOf` draws a loop's; the core plays it (core/src/blocks/sub_duck.rs).
export const SUBDUCK = {
  // How often a loop draws it, and batch 5's vibe table for it.
  weight: 1,
  vibes: { bouncy: 2, energetic: 2, lively: 1.5, skipping: 1.5, steady: 1.5, restful: 0.4, serene: 0.4, soothing: 0.4, hushed: 0.4 },
  // The dip at a kick of velocity 0.8 or more, in dB (never deeper than 9:
  // the records' 20 dB "gates" are a bass playing off the beat).
  depth: [5, 9],
  // How long it takes to come back, in seconds, and never longer than this
  // share of a beat.
  recover: [0.15, 0.25],
  beat: 0.8,
};

// The choices, in a fixed order (the draw walks them in it). Each is
//
//   { id, options: [['today', w], [option, w], ...], weight,
//     eligible: (ctx) => bool, vibes: { vibe: lean, ... },
//     optionVibes: { option: { vibe: lean, ... }, ... } }
//
// `weight` is how often the choice is picked among those a loop is eligible
// for, before the vibes lean it (1 if missing); `options` past the first
// are what it plays when picked, by their weights, each leaned by its
// `optionVibes`. `ctx` is what the generator knows of the loop when it draws
// (see `choicesOf`).
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
  // tunedKick (queue item 67; Ears 8, approved 2026-10-18): the kick on the
  // key's note, short as today or a long boom (KICK, above; `kickOf`). Any
  // loop with a kick.
  { id: 'tunedKick', options: KICK.options, optionVibes: KICK.optionVibes, eligible: (ctx) => hasKick(ctx) },
  // subDuck (queue item 68; Ears 8, approved 2026-10-18): the bass alone dips
  // at each kick (SUBDUCK, above; `subDuckOf`). Only on a loop with a kick
  // and a bass line (Mikey's rule; the sketch's test).
  { id: 'subDuck', options: [['today', 1], ['on', 1]], weight: SUBDUCK.weight, vibes: SUBDUCK.vibes, eligible: (ctx) => hasKick(ctx) && hasBass(ctx) },
];

// Whether the loop plays a bass line.
function hasBass(ctx) {
  return !!ctx.tracks && ctx.tracks.bass.some((e) => e.vel);
}

// Whether the loop plays a kick (or a soft kick).
function hasKick(ctx) {
  return !!ctx.tracks && ctx.tracks.drums.some((e) => e.vel && (e.inst === 'kick' || e.inst === 'softkick'));
}

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
    out[c.id] = others.length === 1 ? others[0][0] : r.weighted(others.map(([o, w]) => [o, w * leanOf(c.optionVibes && c.optionVibes[o], vibes)]));
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

// The tuned kick's own stream (the sketch's salt), and the stream a
// wandering loop's kick swaps are drawn from.
const KICK_SALT = 0x7c1d;
const KICK_SWAP_SALT = 0x6b5a;

// A loop's tuned kick, drawn from KICK on a stream of its own: { midi, hz,
// short, long }, `short` and `long` each { dbps, lift, tail, softLift,
// softTail } (the kick's and the soft kick's level by ear; a tail of 1 is
// none), so a wandering loop can swap between them. `root` is the loop's key.
// `pins` may fix the note (a MIDI note) or either fall.
export function kickOf(seed, root, pins = {}) {
  const r = new Rng(((seed ^ KICK_SALT) >>> 0) || 1);
  const shortRate = r.range(KICK.short[0], KICK.short[1]);
  const longRate = r.range(KICK.long[0], KICK.long[1]);
  // The key's note between G1 and D2; D# to F# take their fifth (A# to C#).
  const pc = ((root % 12) + 12) % 12;
  const use = pc >= 3 && pc <= 6 ? (pc + 7) % 12 : pc;
  const midi = pins.midi ?? 24 + use + (use < 7 ? 12 : 0);
  const hz = 440 * Math.pow(2, (midi - 69) / 12);
  const fits = (kind, dbps) => {
    const hard = kickFit(hz, kind, dbps, 128, 44, 0.09, 0.004);
    const soft = kickFit(hz, kind, dbps, 96, 46, 0.14, 0.009);
    return { dbps, lift: hard.lift, tail: hard.tail, softLift: soft.lift, softTail: soft.tail };
  };
  return { midi, hz, short: fits('short', pins.short ?? shortRate), long: fits('long', pins.long ?? longRate) };
}

// The kick a wandering loop plays on pass `pass`: the loop's own (`kind`) on
// the first pass and most others; now and then (KICK.swap) one of the other
// two, drawn from the pass alone, so any pass can be reached without playing
// the ones before it.
export function kickKindOf(seed, kind, pass) {
  if (!pass) return kind;
  const r = new Rng((((seed ^ KICK_SWAP_SALT) >>> 0) + Math.imul(pass, 0x9e3779b1)) >>> 0 || 1);
  if (!r.chance(KICK.swap)) return kind;
  const others = ['today', 'short', 'long'].filter((k) => k !== kind);
  return others[r.chance(0.5) ? 0 : 1];
}

// The tuned kick's level by ear, as the sketch worked it out at load: the
// lift that keeps the kick (or, for a long boom, its punch) at today's
// loudness by ear (K-weighting's 38 Hz high-pass) and its own energy under 60
// Hz and at 60-120 Hz within KICK.band of today's kick's; then, for a long
// boom, the tail's level, as loud as fits within KICK.tailLoud by ear and the
// same band guard. `f0`, `today`, `sweep` and `att` are today's start, end,
// sweep time and attack.
function kickFit(hz, kind, dbps, f0, today, sweep, att) {
  const H2 = (f) => Math.pow(f, 4) / (Math.pow(f * f - 38 * 38, 2) + Math.pow(f * 38 / 0.5, 2));
  const SUB = (f) => (f < 60 ? 1 : 0);
  const LOW = (f) => (f >= 60 && f < 120 ? 1 : 0);
  const PUNCH = 213;
  const energy = (f1, tail, rate, w) => {
    const t1 = att + -20 * Math.log10(tail) / PUNCH;
    const end = t1 + (80 + 20 * Math.log10(tail)) / rate;
    let e = 0;
    for (let t = 0; t < end; t += 0.0005) {
      const f = t < sweep ? f0 * Math.pow(f1 / f0, t / sweep) : f1;
      const a = t < att ? t / att : t < t1 ? Math.pow(10, -PUNCH * (t - att) / 20) : tail * Math.pow(10, -rate * (t - t1) / 20);
      e += a * a * w(f);
    }
    return e;
  };
  const ref = [H2, SUB, LOW].map((w) => energy(today, 1, PUNCH, w));
  const ok = (lift, tail, rate, loud) => [[H2, loud], [SUB, KICK.band], [LOW, KICK.band]]
    .every(([w, room], i) => lift * lift * energy(hz, tail, rate, w) <= room * ref[i] * 1.0001);
  let lo = 0;
  let hi = 1.5;
  for (let i = 0; i < 24; i++) {
    const m = (lo + hi) / 2;
    if (ok(m, 1, kind === 'short' ? dbps : PUNCH, KICK.loud)) lo = m;
    else hi = m;
  }
  const lift = lo;
  if (kind === 'short') return { lift, tail: 1 };
  let tlo = 0.001;
  let thi = 1;
  for (let i = 0; i < 24; i++) {
    const m = Math.sqrt(tlo * thi);
    if (ok(lift, m, dbps, KICK.tailLoud)) tlo = m;
    else thi = m;
  }
  return { lift, tail: tlo };
}

// subDuck's own stream (the sketch's salt).
const SUBDUCK_SALT = 0x5dc4;

// A loop's subDuck, drawn from SUBDUCK on a stream of its own: { depth,
// recover }. The sketch drew whether a loop ducked first, on the same
// stream; that draw is spent here too, so a loop gets the depth and recovery
// the sketch drew it. `bpm` caps the recovery at SUBDUCK.beat of a beat.
// `pins` may fix either.
export function subDuckOf(seed, bpm, pins = {}) {
  const r = new Rng(((seed ^ SUBDUCK_SALT) >>> 0) || 1);
  r.f();
  const depth = r.range(SUBDUCK.depth[0], SUBDUCK.depth[1]);
  const recover = Math.min(r.range(SUBDUCK.recover[0], SUBDUCK.recover[1]), SUBDUCK.beat * 60 / bpm);
  return { depth: pins.depth ?? depth, recover: pins.recover ?? recover };
}
