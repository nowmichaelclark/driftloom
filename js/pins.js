// The request's pins (queue item 85b): the named settings a host may fix for
// the loops it asks for, beside the vibes and the tempo (js/generator.js,
// `readRequest`). The game's and the ears' levers: any of item 54's choices
// to one of its options, and the settings each choice draws by (the arc, the
// tuned kick, subDuck, lateFloor, padSweep), the echo's time and the space
// recipe's 32 (core/src/space.rs). `core/src/generator/request.rs` reads the
// same table, in the same order.
//
// **A pin chooses within what the loop could draw.** Every setting is held
// to the range (or the list) the data draws it from, so a pinned loop is one
// the seed could have made; a value outside it is refused with a short plain
// line. Widening past the data is the sketch build's `--set`, not a request.
//
// **A pin shapes, it doesn't draw.** The loop's own draws are all made as
// they always are, on their own streams, and a pin then overrules one value,
// so nothing else moves. A choice pinned to today's option is today's, even
// where the loop drew a change (and `arc=today` keeps a glacial loop's arc
// away too); pinned to a change, the loop plays it where its eligibility
// test allows (Mikey's rules for each: subDuck needs a kick and a bass line,
// lateFloor something else from the first second), and today's where not: a
// request bends the music, never breaks it. A setting pinned shapes its
// choice when the loop plays it; it does not turn the choice on.
//
// As text: `organ=on`, `tunedKick=long`, `arc.shape=deep`, `arc.period=240`,
// `echo.beats=1`, `space.room=0.8` (numbers as JavaScript writes them).

import { ARC, CHOICES, ECHO, KICK, LATEFLOOR, PADSWEEP, SUBDUCK } from './choices.js';

export class RequestError extends Error {}

// The space recipe's settings, by number (core/src/space.rs, `SETTINGS`):
// each name and the range a loop draws it from (one number where the two are
// equal). test/core-host.test.mjs holds this copy to the core's.
export const SPACE_SETTINGS = [
  ['room-weight', 0.64, 0.64], ['ensemble-weight', 0.5, 0.5], ['room', 0.7, 0.9],
  ['spread-chords', 0.3, 0.5], ['spread-texture', 0.35, 0.6], ['spread-light-texture', 0.15, 0.3],
  ['spread-melody', 0.0, 0.12], ['spread-scale', 1.6, 1.6], ['spread-max', 0.8, 0.8],
  ['spread-delay', 0.012, 0.012], ['spread-sway', 0.003, 0.003], ['spread-rate', 0.35, 0.35],
  ['spread-rate-step', 0.07, 0.07], ['kit', 0.08, 0.15], ['kit-soft', 0.7, 0.7],
  ['texture-place', 0.1, 0.25], ['room-cutoff', 180, 180], ['spread-cutoff', 200, 200],
  ['place-cutoff', 150, 150], ['busy-from', 0.5, 0.5], ['busy-span', 5, 5],
  ['width-least', 0.25, 0.25], ['wet-from', 0.7, 0.7], ['wet-scale', 0.75, 0.75],
  ['ceiling-busy', -7, -7], ['ceiling-sparse', -9, -9], ['ceiling-very-sparse', -12, -12],
  ['dense', 0.5, 0.5], ['sparse', 0.15, 0.15], ['wash', 1, 1],
  ['wash-follows-width', 1, 1], ['wash-cutoff', 200, 200],
];

const periods = Object.values(ARC.period);
const ids = (pool) => pool.map(([id]) => id);

// Every pin, in its frozen order (append only: a share code writes them by
// name, the core's hosts by number): the choices in CHOICES' order, then the
// settings, then the space recipe's. Each { name, number, words } (the
// words it takes, by number) or { name, number, range, whole } or { name,
// number, values }. The numbers come in blocks, so a choice or a setting
// added later moves none already given: the choices from 0 (room for
// CHOICE_PINS), the settings from SETTING_PINS, the space's from SPACE_PINS.
export const CHOICE_PINS = 16;
export const SETTING_PINS = 16;
export const SPACE_PINS = 48;
const SETTINGS = [
  { name: 'arc.shape', words: ids(ARC.shapes) },
  { name: 'arc.period', range: [Math.min(...periods.map((p) => p[0])), Math.max(...periods.map((p) => p[1]))] },
  { name: 'arc.from', range: [0, 1] },
  { name: 'arc.floor', range: ARC.floor },
  { name: 'kick.note', range: KICK.note, whole: true },
  { name: 'kick.short', range: KICK.short },
  { name: 'kick.long', range: KICK.long },
  { name: 'subDuck.depth', range: SUBDUCK.depth },
  { name: 'subDuck.recover', range: SUBDUCK.recover },
  { name: 'lateFloor.order', words: ids(LATEFLOOR.orders) },
  { name: 'lateFloor.seconds', range: LATEFLOOR.seconds },
  { name: 'lateFloor.gap', range: [1, LATEFLOOR.maxGap], whole: true },
  { name: 'padSweep.lo', range: PADSWEEP.lo },
  { name: 'padSweep.oct', range: PADSWEEP.oct },
  { name: 'padSweep.period', range: PADSWEEP.period },
  { name: 'padSweep.q', range: PADSWEEP.q },
  { name: 'echo.beats', values: ECHO.beats.map(([b]) => b) },
];
if (CHOICES.length > CHOICE_PINS || SETTING_PINS + SETTINGS.length > SPACE_PINS) throw new Error('js/pins.js: a block of pins is full');
export const PINS = [
  ...CHOICES.map((c, i) => ({ name: c.id, number: i, words: ids(c.options) })),
  ...SETTINGS.map((p, i) => ({ ...p, number: SETTING_PINS + i })),
  ...SPACE_SETTINGS.map(([name, lo, hi], i) => ({ name: `space.${name}`, number: SPACE_PINS + i, range: [lo, hi] })),
];

const BY_NAME = new Map(PINS.map((p, i) => [p.name, i]));

export const NO_SUCH_SETTING = 'no such setting';
export const BAD_SETTING = 'a bad setting';
export const SETTING_OUT = 'a setting outside its range';

// Whether `name` is a pin's (a choice's id, or a dotted setting's) rather
// than a vibe's.
export function isPinName(name) {
  return BY_NAME.has(name) || name.includes('.');
}

const NUMBER_TEXT = /^-?\d+(\.\d+)?(e[-+]?\d+)?$/;

// The value pin `name` takes from `value` (text, a number or a word), or a
// refusal.
export function pinValue(name, value) {
  const i = BY_NAME.get(name);
  if (i == null) throw new RequestError(NO_SUCH_SETTING);
  const p = PINS[i];
  if (p.words) {
    if (typeof value !== 'string' || !p.words.includes(value)) throw new RequestError(BAD_SETTING);
    return value;
  }
  let x = value;
  if (typeof x === 'string') {
    if (!NUMBER_TEXT.test(x)) throw new RequestError(BAD_SETTING);
    x = Number(x);
  }
  if (typeof x !== 'number' || !Number.isFinite(x)) throw new RequestError(BAD_SETTING);
  if (p.values) {
    if (!p.values.includes(x)) throw new RequestError(SETTING_OUT);
    return x;
  }
  if (x < p.range[0] || x > p.range[1] || (p.whole && !Number.isInteger(x))) throw new RequestError(SETTING_OUT);
  return x;
}

// Pins read from an object ({ name: value }) into their canonical form: the
// same, in the table's order, every value checked; null for none.
export function readPins(input) {
  if (input == null) return null;
  if (typeof input !== 'object' || Array.isArray(input)) throw new RequestError(BAD_SETTING);
  const out = {};
  for (const [name, value] of Object.entries(input)) out[name] = pinValue(name, value);
  return framedPins(out);
}

// Pins in the table's order, or null for none.
export function framedPins(pins) {
  if (!pins) return null;
  const names = PINS.map((p) => p.name).filter((n) => n in pins);
  return names.length ? Object.fromEntries(names.map((n) => [n, pins[n]])) : null;
}

// The pins as text items ("organ=on", "arc.period=240"), in the table's order.
export function pinItems(pins) {
  const framed = framedPins(pins);
  return framed ? Object.entries(framed).map(([n, v]) => `${n}=${v}`) : [];
}

// A pin as the core's hosts send it (`dl_play_pin`): its number, and its
// value as a number (a word by its place in the pin's list).
export function pinNumbers(pins) {
  const framed = framedPins(pins);
  if (!framed) return [];
  return Object.entries(framed).map(([n, v]) => {
    const p = PINS[BY_NAME.get(n)];
    return [p.number, p.words ? p.words.indexOf(v) : v];
  });
}

// One group's settings from a spec's pins ({ shape, period } for 'arc'), for
// the draws' `pins` arguments; {} for none.
export function pinsFor(spec, group) {
  const out = {};
  const pins = spec && spec.pins;
  if (!pins) return out;
  const head = `${group}.`;
  for (const [n, v] of Object.entries(pins)) if (n.startsWith(head)) out[n.slice(head.length)] = v;
  return out;
}

// The option a spec's pins fix for choice `id`, or undefined.
export function choicePin(spec, id) {
  return spec && spec.pins ? spec.pins[id] : undefined;
}

// The space recipe's pins, as [setting number, value] pairs.
export function spacePins(spec) {
  const s = pinsFor(spec, 'space');
  return SPACE_SETTINGS.map(([name], i) => [i, s[name]]).filter(([, v]) => v !== undefined);
}
