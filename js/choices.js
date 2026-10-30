// The per-loop choices (queue item 54): the one place that decides which of
// the sound batch's changes a loop gets.
//
// Mikey's rules for every choice (2026-10-18; core/AGENTS.md, "Rules for
// sound items"):
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
// Every number the choices decide by is data (queue item 78):
// data/choices.txt, with the comments that say where each came from, read
// here into the same tables the code has always used (CHOICES, ARC, KICK, and
// so on). What stays in code is the draws and each choice's eligibility test.
// Each later item of the batch adds its choice to the file (an id, its
// options with today's first, and the vibes it leans toward or away from) and
// its eligibility test to ELIGIBLE below. Nothing here goes in the share
// code: the code's seed gives the draws, and "Let the loop wander" never
// redraws them (item 67's kick swap is the one exception). Pins come with
// phase 2's requests.

import { Rng } from './rng.js';
import { log10, pow, sqrt } from './dlmath.js';
import { blockOf, blocksOf, readBlock } from './data.js';

const CHOICE_SALT = 0x43484f49;

const WHERE = 'data/choices.txt';
const setting = (name, schema) => readBlock(blockOf('choices', 'setting', name), schema, WHERE);
const ALL = readBlock(blockOf('choices', 'choices', 'all'), { changes: 'numpool', lean: 'range' }, WHERE);

// How many changes a loop gets: none 15%, one 45%, two 30%, three 10% (a
// taste call, for Mikey's ears; the file says why).
export const CHANGE_COUNTS = ALL.changes;

// A choice's lean for one vibe is held between these (the file says why). A
// vibe not named keeps x1.
const [LEAN_MIN, LEAN_MAX] = ALL.lean;

// The data of every choice: { options, weight?, vibes?, optionVibes? }.
const CHOICE_DATA = Object.fromEntries(blocksOf('choices', 'choice').map((b) => [b.name, readBlock(b, {
  options: 'pool', weight: 'num', vibes: 'table', 'optionVibes.*': 'table',
}, WHERE)]));
const choiceData = (id) => CHOICE_DATA[id];

// The arc's settings (queue items 63 and 63b): every variable of the arc,
// named, with the range a loop draws it from, in this one place, so phase 2's
// requests can pin them (docs/CORE.md, promises 9 and 11) and a later slider
// or a game host can set them. `arcOf` draws a loop's from them; the core
// plays it (core/src/blocks/arc.rs). The file says what each is.
export const ARC = {
  weight: choiceData('arc').weight,
  vibes: choiceData('arc').vibes,
  ...setting('arc', {
    shapes: 'pool', 'shapeVibes.*': 'table', 'period.*': 'range', calm: 'words', bright: 'words', glacialNear: 'words',
    glacialChance: 'num', 'from.*': 'range', floor: 'range', dip: 'num',
  }),
};

// The tuned kick's settings (queue item 67): every variable of it, named,
// with its range, as ARC's are. `kickOf` draws a loop's from them and works
// out its level at load; the core plays it (core/src/blocks/kick.rs).
export const KICK = {
  options: choiceData('tunedKick').options,
  optionVibes: choiceData('tunedKick').optionVibes,
  ...setting('kick', {
    note: 'range', short: 'range', long: 'range', loud: 'num', tailLoud: 'num', band: 'num', swap: 'num',
  }),
};

// subDuck's settings (queue item 68): the bass channel alone dips at each
// kick and comes back, so the kick and the sub take turns. `subDuckOf` draws
// a loop's; the core plays it (core/src/blocks/sub_duck.rs).
export const SUBDUCK = {
  weight: choiceData('subDuck').weight,
  vibes: choiceData('subDuck').vibes,
  ...setting('subduck', { depth: 'range', recover: 'range', beat: 'num' }),
};

// lateFloor's settings (queue item 69): after a load the kit and the bass sit
// out a while, then arrive. `lateFloorOf` draws a loop's; the engine plays it
// (js/engine.js).
export const LATEFLOOR = {
  weight: choiceData('lateFloor').weight,
  vibes: choiceData('lateFloor').vibes,
  ...setting('latefloor', {
    seconds: 'range', round: 'num', first: 'num', orders: 'pool', gap: 'numpool', maxGap: 'num', gapLong: 'num', longVibes: 'table',
  }),
};

// floorDrop's shapes (queue item 70): a loop that draws them gives each of
// its floor drops a shape of its own. The generator plays them, at the end of
// drift's draws (js/generator.js, `drift`).
export const FLOORDROP = {
  weight: choiceData('floorDrop').weight,
  ...setting('floordrop', {
    shapes: 'pool', needBass: 'words', 'shapeVibes.*': 'table', staggerShort: 'num', stutterShort: 'num', stutter: 'range',
    longSeconds: 'num', longBars: 'range', wholeUnder: 'num',
  }),
};

// pedalBass's settings (queue item 71): the bass holds the key's tonic in a
// rhythm while the chords move, now and then stepping to one neighbor and
// back. The generator plays it (js/generator.js, `pedalBass`).
export const PEDAL = {
  weight: choiceData('pedalBass').weight,
  vibes: choiceData('pedalBass').vibes,
  ...setting('pedal', {
    styles: 'words', rhythms: 'pool', neighbors: 'numpool', 'neighborVibes.*': 'table', share: 'range', low: 'num', high: 'num',
  }),
};

// padSweep's settings (queue item 72): a lowpass on the chord layer swings
// open and shut while the rest stays as bright as it was. `padSweepOf` draws
// a loop's; the core plays it (core/src/blocks/pad_sweep.rs).
export const PADSWEEP = {
  weight: choiceData('padSweep').weight,
  vibes: choiceData('padSweep').vibes,
  ...setting('padsweep', { sustained: 'words', lo: 'range', oct: 'range', period: 'range', q: 'range' }),
};

// echoTime's settings (queue item 73): the echo's time drawn per loop, in
// beats. Drawn for every loop as the sketch drew it, outside the list of
// choices and not counted among a loop's changes; `echoTimeOf` draws a
// loop's.
export const ECHO = setting('echo', { beats: 'numpool', 'vibes.*': 'table' });

// The bass layer's level for a loop (queue item 76c), in dB over the level
// the bass had before roundBass's lift, with a kick and without; the host
// sends the loop's to the core at each load (`Core::bass_lift`). The
// JavaScript synth leaves its bass alone.
export const BASS_LIFT = setting('basslift', { kick: 'num', noKick: 'num' });

// The bass lift for a loop whose tracks are `tracks`: the kick's is for a
// loop that plays a kick, a soft kick or, in the hand kit, the frame drum
// that is its kick (`HAND_KIT` in js/generator.js; Mikey's +3 on noan-nou is
// a hand-kit loop) anywhere in it. A kick that only arrives late (lateFloor)
// or drops out (floorDrop) is still the loop's kick, so the setting holds
// throughout; a muted kit is still the loop's (the mutes are the listener's,
// and change while it plays).
export function bassLiftOf(tracks) {
  const kick = tracks.drums.some((e) => e.vel && (e.inst === 'kick' || e.inst === 'softkick' || e.inst === 'frame'));
  return kick ? BASS_LIFT.kick : BASS_LIFT.noKick;
}


// The choices, in a fixed order (the draw walks them in it): the file's
// [choice id] blocks, each
//
//   { id, options: [['today', w], [option, w], ...], weight,
//     eligible: (ctx) => bool, vibes: { vibe: lean, ... },
//     optionVibes: { option: { vibe: lean, ... }, ... } }
//
// `weight` is how often the choice is picked among those a loop is eligible
// for, before the vibes lean it (1 if missing); `options` past the first
// are what it plays when picked, by their weights, each leaned by its
// `optionVibes`. `ctx` is what the generator knows of the loop when it draws
// (see `choicesOf`). The eligibility tests, by id, are what the file can't
// say; they are the same ones the choices were written with, and a choice
// without one (or a test without a choice) is an error.
const ELIGIBLE = {
  // For any loop with a chord of two notes or more (a one-note "chord" never
  // rings out).
  glass: (ctx) => hasChords(ctx),
  // Busy loops only: drums, and seven or more notes a second.
  dubStab: (ctx) => playsChords(ctx) && busy(ctx),
  // For any loop that plays a chord.
  organ: (ctx) => playsChords(ctx),
  // Any loop.
  wash: () => true,
  arc: () => true,
  // Any loop with a kick.
  tunedKick: (ctx) => hasKick(ctx),
  // Only on a loop with a kick and a bass line (Mikey's rule; the sketch's
  // test).
  subDuck: (ctx) => hasKick(ctx) && hasBass(ctx),
  // Only on a loop with drums or a bass and something else that plays from
  // its first second, so it never starts with nothing.
  lateFloor: (ctx) => floorAndMore(ctx),
  // Only where a floor can drop: a loop of 4 bars or more, with drums or a
  // bass.
  floorDrop: (ctx) => !!ctx.tracks && ctx.spec.bars >= 4 && (ctx.tracks.drums.some((e) => e.vel) || ctx.tracks.bass.some((e) => e.vel)),
  // Only over a held, pulse, dub or walking bass.
  pedalBass: (ctx) => !!ctx.bass && PEDAL.styles.includes(ctx.bass.style) && !!ctx.tracks && ctx.tracks.bass.some((e) => e.vel),
  // Only where the chords are a sustained voice.
  padSweep: (ctx) => !!ctx.tracks && ctx.tracks.chords.some((e) => e.vel && PADSWEEP.sustained.includes(e.voice)),
};

export const CHOICES = Object.keys(CHOICE_DATA).map((id) => {
  if (!ELIGIBLE[id]) throw new Error(`${WHERE}: [choice ${id}] has no eligibility test in js/choices.js`);
  return { id, ...CHOICE_DATA[id], eligible: ELIGIBLE[id] };
});
for (const id of Object.keys(ELIGIBLE)) {
  if (!CHOICE_DATA[id]) throw new Error(`js/choices.js: eligibility test for '${id}', which ${WHERE} lacks`);
}

// Whether the loop has a floor to hold back (drums or a bass) and something
// else that plays from its first second (chords, a tune or a texture), as
// the tracks hold them after their entry schedules. The sketch's test was
// anything else at all, which let 3.6% of the loops that draw it open in
// silence for a second or more, until their pads or tune came in (Mikey,
// 2026-10-25: the first second, as the queue reads).
function floorAndMore(ctx) {
  if (!ctx.tracks) return false;
  const step = 60 / ctx.spec.bpm / 4;
  const plays = (layer) => ctx.tracks[layer].some((e) => e.vel);
  const first = (layer) => ctx.tracks[layer].some((e) => e.vel && e.step * step < LATEFLOOR.first);
  return (plays('drums') || plays('bass')) && (first('chords') || first('melody') || first('texture'));
}

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
    m *= pow(Math.min(LEAN_MAX, Math.max(LEAN_MIN, x)), vibes.strength * w);
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
  const { period, at, drawn, floor } = arcDraws(seed, vibes);
  const shape = pins.shape ?? drawn;
  const dip = pins.dip ?? ARC.dip;
  // Deep: a bright point, as item 63 drew it (Rng.range's arithmetic, so the
  // same bits). Dip: anywhere in the open part, outside the dip's share
  // around the darkest point.
  const [a, b] = shape === 'deep' ? ARC.from.deep : [dip / 2, 1 - dip / 2];
  const from = a + at * (b - a);
  return { shape, period: pins.period ?? period, from: pins.from ?? from, floor: pins.floor ?? floor, dip };
}

// The arc's draws, in their order on its stream, and the stream, left after
// them.
function arcDraws(seed, vibes) {
  const r = new Rng(((seed ^ ARC_SALT) >>> 0) || 1);
  const [lo, hi] = ARC.period[arcPace(vibes)];
  const period = r.range(lo, hi);
  const at = r.f();
  const shapes = ARC.shapes.map(([id, w]) => [id, w * leanOf(ARC.shapeVibes[id], vibes)]);
  const drawn = r.weighted(shapes);
  const floor = r.range(ARC.floor[0], ARC.floor[1]);
  return { period, at, drawn, floor, r };
}

// Whether a loop plays the arc (queue item 76b): when it drew it among its
// choices, or, with glacial among its vibes, with ARC.glacialChance times
// glacial's weight, drawn on the arc's own stream after arcOf's draws. Not
// one of the loop's changes, so its other choices and draws stay as they
// are; a loop without glacial plays the arc exactly when its choices say.
export function arcPlays(seed, choices, vibes = null) {
  if (choices && choices.arc === 'on') return true;
  const w = vibes && vibes.vibes.glacial;
  if (!w) return false;
  return arcDraws(seed, vibes).r.chance(ARC.glacialChance * w);
}

// Which period range a loop's vibes pick: 'calm' (or glacial's own),
// 'bright' or 'rest'.
function arcPace(vibes) {
  if (!vibes) return 'rest';
  let calm = 0;
  let bright = 0;
  for (const [id, w] of Object.entries(vibes.vibes)) {
    if (ARC.calm.includes(id)) calm += w;
    if (ARC.bright.includes(id)) bright += w;
  }
  if (calm > bright && vibes.vibes.glacial) return Object.keys(vibes.vibes).some((id) => ARC.glacialNear.includes(id)) ? 'glacialNear' : 'glacial';
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
  const hz = 440 * pow(2, (midi - 69) / 12);
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
  const H2 = (f) => pow(f, 4) / (pow(f * f - 38 * 38, 2) + pow(f * 38 / 0.5, 2));
  const SUB = (f) => (f < 60 ? 1 : 0);
  const LOW = (f) => (f >= 60 && f < 120 ? 1 : 0);
  const PUNCH = 213;
  const energy = (f1, tail, rate, w) => {
    const t1 = att + -20 * log10(tail) / PUNCH;
    const end = t1 + (80 + 20 * log10(tail)) / rate;
    let e = 0;
    for (let t = 0; t < end; t += 0.0005) {
      const f = t < sweep ? f0 * pow(f1 / f0, t / sweep) : f1;
      const a = t < att ? t / att : t < t1 ? pow(10, -PUNCH * (t - att) / 20) : tail * pow(10, -rate * (t - t1) / 20);
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
    const m = sqrt(tlo * thi);
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

// lateFloor's own stream (the sketch's salt).
const LATEFLOOR_SALT = 0x1a7e;

// A loop's lateFloor, drawn from LATEFLOOR on a stream of its own: { order,
// bars, gap, drums, bass }, the last two the steps from the load each sits
// out. The sketch drew whether a loop waits first, on the same stream; that
// draw is spent here too, so a loop waits as long as the sketch drew it. The
// gap between the two arrivals (queue item 76d) is the last draw on the
// stream, after the sketch's, so nothing before it moves. `spb` is the
// loop's steps a bar, `stepDur` a step's seconds, `vibes` its vibes ({
// vibes, strength }, or null), `loopBars` its length in bars (the second
// arrival stays inside the first pass, or within a loop's length of the first). `pins` may fix the order, the
// seconds or the gap.
export function lateFloorOf(seed, spb, stepDur, pins = {}, vibes = null, loopBars = Infinity) {
  const r = new Rng(((seed ^ LATEFLOOR_SALT) >>> 0) || 1);
  r.f();
  const lean = leanOf(LATEFLOOR.longVibes, vibes);
  const u = r.f();
  const [lo, hi] = LATEFLOOR.seconds;
  const secs = pins.seconds ?? lo + pow(u, 1 / lean) * (hi - lo);
  const drawn = r.weighted(LATEFLOOR.orders);
  const order = pins.order ?? drawn;
  const k = LATEFLOOR.round;
  const bars = Math.max(k, Math.round(secs / (spb * stepDur) / k) * k);
  const gaps = LATEFLOOR.gap.map(([g, w]) => [g, g >= LATEFLOOR.gapLong ? w * lean : w]);
  const gapDrawn = r.weighted(gaps);
  // Inside the loop's first pass when the first part is (up to its last
  // bar); past it, within one loop's length of the first.
  const room = bars < loopBars ? loopBars - bars : loopBars;
  const gap = Math.max(1, Math.min(pins.gap ?? gapDrawn, LATEFLOOR.maxGap, room));
  return {
    order, bars, gap,
    drums: (order === 'bassFirst' ? bars + gap : bars) * spb,
    bass: (order === 'kitFirst' ? bars + gap : bars) * spb,
  };
}

// The shape of one floor drop (FLOORDROP.shapes), drawn with `rng` (drift's,
// at the end of its draws) for a loop with or without a bass line, leaned by
// its vibes ({ vibes, strength }, or null).
export function dropShapeOf(rng, hasBass, vibes = null) {
  const shapes = FLOORDROP.shapes
    .filter(([id]) => hasBass || !FLOORDROP.needBass.includes(id))
    .map(([id, w]) => [id, w * leanOf(FLOORDROP.shapeVibes[id], vibes)]);
  return rng.weighted(shapes);
}

// pedalBass's own stream (the sketch's salt, on the bass's seed).
const PEDAL_SALT = 0x9eda1;

// A loop's pedal, drawn from PEDAL on a stream of its own: { rhythm, nb,
// share, rng }, `nb` the neighbor in semitones (0 for none in the mode), `rng`
// the stream, left where the per-note draws go on. The sketch drew whether a
// bass takes the pedal first, on the same stream; that draw is spent here too,
// so a loop gets the rhythm, neighbor and share the sketch drew it. `steps`
// is the loop's mode (semitones from its root); `vibes` its vibes.
export function pedalOf(bassSeed, steps, vibes = null) {
  const r = new Rng(((bassSeed ^ PEDAL_SALT) >>> 0) || 1);
  r.f();
  const rhythm = r.weighted(PEDAL.rhythms);
  const has = (st) => steps.includes(((st % 12) + 12) % 12);
  const nbs = PEDAL.neighbors.filter(([nb]) => has(nb)).map(([nb, w]) => [nb, w * leanOf(PEDAL.neighborVibes[nb], vibes)]);
  const nb = nbs.length ? r.weighted(nbs) : 0;
  const share = r.range(PEDAL.share[0], PEDAL.share[1]);
  return { rhythm, nb, share, rng: r };
}

// padSweep's own stream (the sketch's salt).
const PADSWEEP_SALT = 0x5e3e;

// A loop's padSweep, drawn from PADSWEEP on a stream of its own: { lo, oct,
// period, q }, in the sketch's order. The sketch drew whether a loop sweeps
// first, on the same stream; that draw is spent here too, so a loop sweeps as
// the sketch drew it. `pins` may fix any.
export function padSweepOf(seed, pins = {}) {
  const r = new Rng(((seed ^ PADSWEEP_SALT) >>> 0) || 1);
  r.f();
  const lo = r.range(PADSWEEP.lo[0], PADSWEEP.lo[1]);
  const oct = r.range(PADSWEEP.oct[0], PADSWEEP.oct[1]);
  const period = r.range(PADSWEEP.period[0], PADSWEEP.period[1]);
  const q = r.range(PADSWEEP.q[0], PADSWEEP.q[1]);
  return { lo: pins.lo ?? lo, oct: pins.oct ?? oct, period: pins.period ?? period, q: pins.q ?? q };
}

// echoTime's own stream (the sketch's salt).
const ECHO_SALT = 0xec40;

// A loop's echo time in beats, drawn from ECHO on a stream of its own, the
// sketch's one draw; `vibes` is the loop's ({ vibes, strength }, or null).
// With no vibes a loop draws the time the sketch drew it.
export function echoTimeOf(seed, vibes = null) {
  const r = new Rng(((seed ^ ECHO_SALT) >>> 0) || 1);
  return r.weighted(ECHO.beats.map(([b, w]) => [b, w * leanOf(ECHO.vibes[b], vibes)]));
}
