// The wander as it is played (queue item 102b; docs/LONGFORM.md, "The
// wander"): what tools/longform.mjs checks on the plan, checked again on what
// the player plays, read from its calls (`dlrender --seed --calls`) and its
// samples (`dlrender --seed --wav`), beside the piece's map (the tools'
// build, tools/gen-lib.mjs's `pieceMap`). tools/longform.mjs runs it.
//
// A section is played at the wander's amount 0 (the slider all the way
// left: the plan still plays, nothing breathes), so where nothing moved a
// section's notes are the section before's, note for note; what differs is
// what moved, read by its kind:
//
//   layers    a layer's notes gone or back, or its gain in the mix moving
//   voice     the same notes, another voice (or kind of bass)
//   register  the chords' notes an octave away, nothing else
//   length    the chords' notes the same but their lengths
//   density   the tune's notes a few fewer or more, the rest the same
//   key       every pitched layer moved the same few semitones
//   warmth    the tone's lowpass gliding; space: the room's
//
// and every check is the plan's, held to what was heard: at most one thing
// moves a section; every move the map draws is heard in its unit, and
// nothing moves where the map draws nothing (a glide may run on over the
// data's `glide` seconds after its move); a move that can't glide starts
// on its unit's first step (a phrase or a section line), never inside one.
// Then, at the app's own amount (1), the samples: no click at a line (the
// largest step between two samples within a moment of each move, against
// the largest the piece takes anywhere else, as `dlrender --changes`
// measures a join, core/examples/dlrender/change.rs), and the loudness over
// twenty minutes (no jump past 2 LU between neighboring stretches of 20 s
// or more, and the whole within the catalog's spread).

import { execFileSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { pieceMap } from './gen-lib.mjs';
import { kWeighting } from './meter.mjs';

const ROOT = path.join(path.dirname(fileURLToPath(import.meta.url)), '..');
const DLRENDER = path.join(ROOT, 'core', 'target', 'release', 'examples', 'dlrender');
const CH = ['melody', 'chords', 'bass', 'texture', 'drums'];
const THINGS = ['key', 'layers', 'voice', 'warmth', 'space', 'register', 'length', 'density'];
const GLIDING = ['layers', 'warmth', 'space', 'density'];
/** How long a move glides, as data/longform.txt has it ([longform wander] glide). */
const GLIDE = Number(fs.readFileSync(path.join(ROOT, 'data', 'longform.txt'), 'utf8').match(/^glide = (\S+)/m)?.[1] ?? 24);
/** The catalog's spread of loudness: test/core-baseline.json's loops, from the tenth to the ninetieth. */
const SPREAD = (() => {
  const b = JSON.parse(fs.readFileSync(path.join(ROOT, 'test', 'core-baseline.json'), 'utf8'));
  const v = Object.values(b.loops.full).map((l) => l.lufs).sort((x, y) => x - y);
  return v[Math.floor(0.9 * (v.length - 1))] - v[Math.floor(0.1 * (v.length - 1))];
})();

/** The release dlrender, built once. */
export function dlrender(args) {
  if (!fs.existsSync(DLRENDER) || process.env.LONGFORM_BUILD !== 'no') {
    if (!dlrender.built) {
      execFileSync('cargo', ['build', '--release', '--locked', '--example', 'dlrender'], { cwd: path.join(ROOT, 'core'), stdio: 'ignore' });
      dlrender.built = true;
    }
  }
  return execFileSync(DLRENDER, args, { encoding: 'utf8', maxBuffer: 1 << 26 });
}

const num = (x) => (x.startsWith('x') ? Buffer.from(x.slice(1).padStart(16, '0'), 'hex').readDoubleBE(0) : Number(x));

/** A call list: each section's notes by layer (the loop's own; glass and stabs, keyed past 256, apart) and the mix's moves. */
function readCalls(file) {
  const secs = new Map();
  const params = [];
  let key = null;
  for (const line of fs.readFileSync(file, 'utf8').split('\n')) {
    const p = line.split(' ');
    if (p[0] === 'key') key = { pass: Number(p[2]), step: Number(p[3]), ord: Number(p[4]) };
    else if (p[0] === 'note' && key && key.ord < 256) {
      if (!secs.has(key.pass)) secs.set(key.pass, { start: Infinity, steps: 0, layers: CH.map(() => []) });
      const s = secs.get(key.pass);
      const t = num(p[3]);
      s.layers[Number(p[2])].push({ step: key.step, kind: `${p[1]}.${p[7]}`, midi: num(p[4]), dur: num(p[5]), t });
      s.steps = Math.max(s.steps, key.step + 1);
      if (key.step === 0) s.start = Math.min(s.start, t);
    } else if (p[0] === 'param') params.push({ id: Number(p[1]), v: num(p[3]), t: num(p[4]) });
  }
  return { secs, params };
}

const bag = (notes, keys) => {
  const m = new Map();
  for (const n of notes) {
    const k = keys.map((x) => (x === 'midi' ? Math.round(n.midi * 100) : x === 'dur' ? Math.round(n.dur * 1000) : n[x])).join('|');
    m.set(k, (m.get(k) ?? 0) + 1);
  }
  return m;
};
const same = (a, b) => a.size === b.size && [...a].every(([k, v]) => b.get(k) === v);
const within = (a, b) => [...a].every(([k, v]) => (b.get(k) ?? 0) >= v);
/** The first step at which a layer's notes differ between two sections. */
const firstDiff = (a, b) => {
  const all = ['step', 'kind', 'midi', 'dur'];
  const [x, y] = [bag(a, all), bag(b, all)];
  const steps = [...x, ...y].filter(([k, v]) => (x.get(k) ?? 0) !== (y.get(k) ?? 0) || v === 0).map(([k]) => Number(k.split('|')[0]));
  return steps.length ? Math.min(...steps) : Infinity;
};

/** What moved in `layer` from notes `a` to notes `b`; null for nothing. */
function moved(layer, a, b) {
  const all = ['step', 'kind', 'midi', 'dur'];
  if (same(bag(a, all), bag(b, all))) return null;
  if (!a.length || !b.length) return 'layers';
  // Another voice plays the same notes, at its own length and with its own
  // parts (a harp's second string): the same steps and pitches.
  const notes = (x) => new Set(x.map((n) => `${n.step}|${Math.round(n.midi * 100)}`));
  const [na, nb] = [notes(a), notes(b)];
  const kinds = (x) => new Set(x.map((n) => n.kind)).size;
  if (na.size === nb.size && [...na].every((k) => nb.has(k)) && (kinds(a) !== kinds(b) || a.some((n, i) => n.kind !== b[i]?.kind))) return 'voice';
  if (same(bag(a, ['step', 'kind', 'midi']), bag(b, ['step', 'kind', 'midi']))) return layer === 'chords' ? 'length' : 'other';
  if (same(bag(a, ['step', 'kind', 'dur']), bag(b, ['step', 'kind', 'dur']))) {
    const by = new Set(a.map((n, i) => Math.round(b[i].midi - n.midi)));
    return layer === 'chords' && [...by].every((d) => d === 12 || d === -12) ? 'register' : 'key';
  }
  const [x, y] = [bag(a, all), bag(b, all)];
  if (layer === 'melody' && (within(x, y) || within(y, x))) return 'density';
  return 'other';
}

/**
 * Play `request` of `seed` for `seconds` at the wander's amount 0 and read
 * what moved, section by section, against its map: the problems found and
 * the moves heard.
 */
export function playedChecks(seed, request, seconds) {
  const file = path.join(os.tmpdir(), `longform-${seed}-${process.pid}.calls`);
  dlrender(['--seed', String(seed), '--request', request, '--seconds', String(seconds), '--rate', '22050', '--wander', '0', '--calls', file]);
  const { secs, params } = readCalls(file);
  fs.rmSync(file, { force: true });
  const ns = [...secs.keys()].sort((x, y) => x - y);
  const last = ns.at(-1);
  const map = pieceMap(seed, request, 0, last + 1);
  const { form, units } = map;
  // The things a turn moves, by section and by the unit's first step there.
  const unitSteps = form.unitBars * (secs.get(ns[1])?.steps ?? 0) / form.sectionBars;
  const moves = [];
  for (const u of units) {
    const thing = u.changed.filter((c) => THINGS.includes(c))[0];
    if (u.role === 'turning' && thing) moves.push({ section: u.section, phrase: u.phrase, thing, voice: u.voice ?? units[units.indexOf(u) - 1]?.voice });
  }
  const sectionSeconds = form.sectionBars / form.unitBars * form.unitSeconds;
  // The tone's moves: the lowpass (12) for the warmth, the room (11) for
  // the space; a layer's gain (0 to 4) for its level.
  const which = (id) => (id <= 4 ? 'layers' : id === 12 ? 'warmth' : id === 11 ? 'space' : null);
  const problems = [];
  const heard = [];
  const budget = [];
  // Its first few sections hold the loop's late floor (the kit and the bass
  // sitting out after a load), its last is cut off by the render's end, and
  // a close is a role of its own.
  const closes = new Set(units.filter((u) => u.closes).map((u) => u.section));
  // A round comes round from its close, everything gliding home at once.
  const seam = (n) => form.sections && n % form.sections !== n && (n % form.sections) * sectionSeconds <= GLIDE + form.unitSeconds + 0.01;
  // A layer with a cycle of its own plays other notes each section: read
  // by its gain alone (it differs in most sections, the plan's or not).
  const differs = CH.map((l, i) => ns.slice(4, -1).filter((n) => secs.get(n - 1) && moved(l, secs.get(n - 1).layers[i], secs.get(n).layers[i])).length);
  const cycling = CH.map((_, i) => differs[i] > 0.4 * ns.length);
  for (const n of ns.slice(4, -1)) {
    const a = secs.get(n - 1), b = secs.get(n);
    if (!a || closes.has(n) || closes.has(n - 1) || seam(n)) continue;
    const end = secs.get(n + 1)?.start ?? Infinity;
    const things = new Map();
    CH.forEach((l, i) => {
      const m = !cycling[i] && moved(l, a.layers[i], b.layers[i]);
      if (m) things.set(m, Math.min(things.get(m) ?? Infinity, firstDiff(a.layers[i], b.layers[i])));
    });
    for (const p of params) {
      const t = which(p.id);
      if (t && p.t >= b.start - 0.3 && p.t < end - 0.3 && !things.has(t)) things.set(t, 0);
    }
    if (!things.size) continue;
    // A voice moved changes the voice budget's room: a layer it held back
    // may come in, or one go (the budget prefers the tune). That is the
    // voice's move, counted apart.
    if (moves.some((m) => (m.section === n || m.section === n - 1) && m.thing === 'voice')) {
      for (const t of ['layers', 'other']) {
        if (things.delete(t)) budget.push(n);
      }
      if (!things.size) continue;
    }
    heard.push({ section: n, things: [...things.keys()] });
    const where = `${seed} "${request}" section ${n}`;
    if (things.size > 1) problems.push({ check: 'at most one thing moves a section', where: `${where}: ${[...things.keys()]}` });
    for (const [thing, step] of things) {
      if (thing === 'other') {
        problems.push({ check: 'every move heard is one of the wander\'s things', where });
        continue;
      }
      // The move this is: drawn in this section, or one gliding on from
      // within the glide's seconds (and a unit) before.
      const move = moves.find((m) => m.thing === thing && m.section <= n && (m.section === n || (GLIDING.includes(thing) && (n - m.section) * sectionSeconds <= GLIDE + form.unitSeconds + 0.01)));
      if (!move) problems.push({ check: 'nothing moves where the map draws nothing', where: `${where}: ${thing}` });
      else if (!GLIDING.includes(thing) && step !== Infinity && (step < Math.round(move.phrase * unitSteps) || step >= Math.round((move.phrase + 1) * unitSteps))) {
        problems.push({ check: 'a move that can\'t glide starts on its line', where: `${where}: ${thing} at step ${step}` });
      }
    }
  }
  // Every move the map draws is heard where it is drawn (a glide may take
  // its seconds to be heard in the notes: the tune thinning by one note).
  for (const m of moves) {
    const reach = GLIDING.includes(m.thing) ? Math.ceil((GLIDE + form.unitSeconds) / sectionSeconds) : 0;
    if (m.section <= 4 || m.section + reach >= last - 1 || closes.has(m.section) || seam(m.section)) continue;
    const h = heard.find((x) => x.section >= m.section && x.section <= m.section + reach && x.things.includes(m.thing));
    // A layer with a cycle of its own is not read note by note.
    const layer = { voice: m.voice, register: 'chords', length: 'chords', density: 'melody' }[m.thing];
    if (!h && !(layer && cycling[CH.indexOf(layer)])) problems.push({ check: 'every move the map draws is heard', where: `${seed} "${request}" section ${m.section}: ${m.thing}` });
  }
  return { problems, heard, moves: moves.filter((m) => m.section > 4 && m.section < last), form, cycling: CH.filter((_, i) => cycling[i]), budget };
}

/** A WAV's samples (dlrender writes 32-bit floats, one channel). */
function readWav(file) {
  const b = fs.readFileSync(file);
  let at = 12;
  let rate = 0;
  while (at + 8 <= b.length) {
    const id = b.toString('ascii', at, at + 4);
    const size = b.readUInt32LE(at + 4);
    if (id === 'fmt ') rate = b.readUInt32LE(at + 12);
    if (id === 'data') return { rate, data: new Float32Array(b.buffer.slice(b.byteOffset + at + 8, b.byteOffset + at + 8 + size)) };
    at += 8 + size;
  }
  throw new Error(`${file}: no data`);
}

const stepMax = (x, from, to) => {
  let m = 0;
  for (let i = Math.max(1, from); i < Math.min(x.length, to); i++) m = Math.max(m, Math.abs(x[i] - x[i - 1]));
  return m;
};

/**
 * Play `request` of `seed` for `seconds` at the app's amount (1), at 44.1
 * kHz, and measure its lines and its loudness: each move's join (the
 * largest step between two samples from just before its line to a moment
 * after) against the piece's own largest anywhere else, and the loudness
 * of each stretch of 20 s or more.
 */
export function soundChecks(seed, request, seconds, lines) {
  const file = path.join(os.tmpdir(), `longform-${seed}-${process.pid}.wav`);
  dlrender(['--seed', String(seed), '--request', request, '--seconds', String(seconds), '--rate', '44100', '--wander', '1', '--wav', file]);
  const { rate, data } = readWav(file);
  fs.rmSync(file, { force: true });
  const { form } = pieceMap(seed, request, 0, 1);
  const sectionSeconds = form.sectionBars / form.unitBars * form.unitSeconds;
  const start = 0.05;
  // The joins: each move's line, from 20 ms before it to 300 ms after.
  const windows = lines.map((l) => {
    const t = start + l.section * sectionSeconds + l.phrase * form.unitSeconds;
    return [Math.round((t - 0.02) * rate), Math.round((t + 0.3) * rate)];
  }).filter(([, to]) => to < data.length);
  let own = 0;
  let from = 0;
  for (const [a, b] of [...windows].sort((x, y) => x[0] - y[0])) {
    own = Math.max(own, stepMax(data, from, a));
    from = Math.max(from, b);
  }
  own = Math.max(own, stepMax(data, from, data.length));
  const joins = windows.map(([a, b]) => stepMax(data, a, b));
  // The loudness, a stretch at a time (whole sections, 20 s at least).
  const per = Math.max(1, Math.ceil(20 / sectionSeconds));
  const stretch = Math.round(per * sectionSeconds * rate);
  const [s, h] = kWeighting(rate);
  const lufs = [];
  let x1 = 0, x2 = 0, y1 = 0, y2 = 0, z1 = 0, z2 = 0;
  for (let at = Math.round(start * rate) + Math.round(30 * rate); at + stretch <= data.length; at += stretch) {
    let sum = 0;
    for (let i = at; i < at + stretch; i++) {
      const x = data[i];
      const y = s.b[0] * x + s.b[1] * x1 + s.b[2] * x2 - s.a[0] * y1 - s.a[1] * y2;
      const z = h.b[0] * y + h.b[1] * y1 + h.b[2] * y2 - h.a[0] * z1 - h.a[1] * z2;
      x2 = x1; x1 = x; y2 = y1; y1 = y; z2 = z1; z1 = z;
      sum += z * z;
    }
    lufs.push(-0.691 + 10 * Math.log10(sum / stretch + 1e-20));
  }
  const jumps = lufs.slice(1).map((l, i) => Math.abs(l - lufs[i]));
  return { own, joins, lufs, jump: Math.max(0, ...jumps), range: Math.max(...lufs) - Math.min(...lufs), spread: SPREAD, stretch: per * sectionSeconds };
}

/** The loudness of the loop of `seed` as itself (no length asked: no plan), the same way: its own breathing's jumps. */
export function loopLoudness(seed, seconds, stretchSeconds) {
  const file = path.join(os.tmpdir(), `longform-own-${seed}-${process.pid}.wav`);
  dlrender(['--seed', String(seed), '--seconds', String(seconds), '--rate', '44100', '--wander', '1', '--wav', file]);
  const { rate, data } = readWav(file);
  fs.rmSync(file, { force: true });
  const [s, h] = kWeighting(rate);
  const stretch = Math.round(stretchSeconds * rate);
  const lufs = [];
  let x1 = 0, x2 = 0, y1 = 0, y2 = 0, z1 = 0, z2 = 0;
  for (let at = Math.round(30.05 * rate); at + stretch <= data.length; at += stretch) {
    let sum = 0;
    for (let i = at; i < at + stretch; i++) {
      const x = data[i];
      const y = s.b[0] * x + s.b[1] * x1 + s.b[2] * x2 - s.a[0] * y1 - s.a[1] * y2;
      const z = h.b[0] * y + h.b[1] * y1 + h.b[2] * y2 - h.a[0] * z1 - h.a[1] * z2;
      x2 = x1; x1 = x; y2 = y1; y1 = y; z2 = z1; z1 = z;
      sum += z * z;
    }
    lufs.push(-0.691 + 10 * Math.log10(sum / stretch + 1e-20));
  }
  return { jump: Math.max(0, ...lufs.slice(1).map((l, i) => Math.abs(l - lufs[i]))), range: Math.max(...lufs) - Math.min(...lufs) };
}
