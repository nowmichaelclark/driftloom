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
//   theme     the tune's notes another tune's (the piece's second theme,
//             session 14), or none where that theme rests
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
import { newSpec, pieceMap } from './gen-lib.mjs';
import { SCALES } from '../js/theory.js';

const SCALE_STEPS = Object.fromEntries(Object.entries(SCALES).map(([id, s]) => [id, s.steps]));
import { kWeighting, fft } from './meter.mjs';

const ROOT = path.join(path.dirname(fileURLToPath(import.meta.url)), '..');
const DLRENDER = path.join(ROOT, 'core', 'target', 'release', 'examples', 'dlrender');
const CH = ['melody', 'chords', 'bass', 'texture', 'drums'];
const THINGS = ['theme', 'key', 'layers', 'voice', 'warmth', 'space', 'register', 'length', 'density'];
const GLIDING = ['layers', 'warmth', 'space', 'density'];
/** How long a move glides, as data/longform.txt has it ([longform wander] glide). */
const GLIDE = Number(fs.readFileSync(path.join(ROOT, 'data', 'longform.txt'), 'utf8').match(/^glide = (\S+)/m)?.[1] ?? 24);
/** How long a finite arc's landing lasts at least, in seconds ([longform energy] land). */
// A join's length (session 15, `[longform joins]`): at most `bars` bars and
// `share` of its unit; a breath `breath` of that.
const JOIN = Object.fromEntries(['bars', 'share', 'breath'].map((k) => [k, Number(fs.readFileSync(path.join(ROOT, 'data', 'longform.txt'), 'utf8').split('[longform joins]')[1].match(new RegExp(`^${k} = (\\S+)`, 'm'))[1])]));
const LAND = Number(fs.readFileSync(path.join(ROOT, 'data', 'longform.txt'), 'utf8').match(/^land = (\S+)/m)?.[1] ?? 4);
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
export function readCalls(file) {
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
      s.layers[Number(p[2])].push({ step: key.step, kind: `${p[1]}.${p[7]}`, tone: p[8], midi: num(p[4]), dur: num(p[5]), t });
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
  // (A section's steps from the loop's spec: its last notes may end before
  // its last step.)
  const spb = newSpec(seed, request).stepsPerBar;
  const unitSteps = form.unitBars * spb;
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
  // A join's last bar (session 15: a breath, a pivot's chord and its bass,
  // the floor out) is the line's, not a move: the steps of either
  // section's joins are left out of both before they are compared.
  const barSteps = spb;
  const tails = (n) => units.filter((u) => u.section === n && !['none', 'glide', 'prepared'].includes(u.join)).map((u) => {
    const end = Math.round((u.phrase + 1) * unitSteps);
    const len = Math.max(1, Math.floor(Math.min(JOIN.bars * barSteps, JOIN.share * u.bars * barSteps)));
    return [end - (u.join === 'breath' ? Math.max(1, Math.floor(len * JOIN.breath)) : len), end];
  });
  const strip = (notes, cut) => notes.filter((x) => !cut.some(([f, t]) => x.step >= f && x.step < t));
  const pair = (n, i) => {
    const cut = [...tails(n - 1), ...tails(n)];
    return [strip(secs.get(n - 1).layers[i], cut), strip(secs.get(n).layers[i], cut)];
  };
  const differs = CH.map((l, i) => ns.slice(4, -1).filter((n) => secs.get(n - 1) && moved(l, ...pair(n, i))).length);
  const cycling = CH.map((_, i) => differs[i] > 0.4 * ns.length);
  for (const n of ns.slice(4, -1)) {
    const a = secs.get(n - 1), b = secs.get(n);
    if (!a || closes.has(n) || closes.has(n - 1) || seam(n)) continue;
    const end = secs.get(n + 1)?.start ?? Infinity;
    const things = new Map();
    CH.forEach((l, i) => {
      const [la, lb] = pair(n, i);
      let m = !cycling[i] && moved(l, la, lb);
      // The tune handed to the second theme, or home again (session 14):
      // other notes, more or fewer, or none (a theme that rests), on the
      // move's line; a move inside a section is heard in the next too.
      if (m && l === 'melody' && moves.some((x) => x.thing === 'theme' && (x.section === n || (x.section === n - 1 && x.phrase > 0)))) m = 'theme';
      if (m) things.set(m, Math.min(things.get(m) ?? Infinity, firstDiff(la, lb)));
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
      const move = moves.find((m) => m.thing === thing && m.section <= n && (m.section === n || (thing === 'theme' && m.section === n - 1 && m.phrase > 0) || (GLIDING.includes(thing) && (n - m.section) * sectionSeconds <= GLIDE + form.unitSeconds + 0.01)));
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
    const layer = { voice: m.voice, register: 'chords', length: 'chords', density: 'melody', theme: 'melody' }[m.thing];
    if (!h && !(layer && cycling[CH.indexOf(layer)])) problems.push({ check: 'every move the map draws is heard', where: `${seed} "${request}" section ${m.section}: ${m.thing}` });
  }
  return { problems, heard, moves: moves.filter((m) => m.section > 4 && m.section < last), form, cycling: CH.filter((_, i) => cycling[i]), budget };
}

/** A WAV's samples (dlrender writes 32-bit floats, one channel). */
export function readWav(file) {
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

/**
 * A stretch's brightness, `data` from `a` to `b`: the spectral centroid of
 * its magnitude spectrum from 250 Hz to 16 kHz, in Hz (the usual measure of
 * brightness, MPEG-7's and librosa's, on the magnitude, not the power, so
 * the highs a filter takes away and an octave doubling adds both show;
 * above the bass, whose level the loudness reads, so a bass coming in or
 * going does not read as the rest growing darker or brighter).
 */
function brightness(data, rate, a, b) {
  const N = 2048;
  const re = new Float64Array(N);
  const im = new Float64Array(N);
  let num = 0, den = 0;
  for (let at = a; at + N <= b; at += N) {
    for (let i = 0; i < N; i++) {
      re[i] = data[at + i] * (0.5 - 0.5 * Math.cos(2 * Math.PI * i / N));
      im[i] = 0;
    }
    fft(re, im);
    for (let i = 1; i < N / 2; i++) {
      const f = i * rate / N;
      if (f < 250 || f > 16000) continue;
      const m = Math.sqrt(re[i] * re[i] + im[i] * im[i]);
      num += f * m;
      den += m;
    }
  }
  return den > 0 ? num / den : 0;
}

/** K-weighted loudness of each stretch `[a, b)` of `data` (in samples), the filter run over the whole once. */
function lufsOf(data, rate, stretches) {
  const [s, h] = kWeighting(rate);
  const z = new Float32Array(data.length);
  let x1 = 0, x2 = 0, y1 = 0, y2 = 0, z1 = 0, z2 = 0;
  for (let i = 0; i < data.length; i++) {
    const x = data[i];
    const y = s.b[0] * x + s.b[1] * x1 + s.b[2] * x2 - s.a[0] * y1 - s.a[1] * y2;
    const w = h.b[0] * y + h.b[1] * y1 + h.b[2] * y2 - h.a[0] * z1 - h.a[1] * z2;
    x2 = x1; x1 = x; y2 = y1; y1 = y; z2 = z1; z1 = w;
    z[i] = w;
  }
  return stretches.map(([a, b]) => {
    let sum = 0;
    for (let i = a; i < b; i++) sum += z[i] * z[i];
    return -0.691 + 10 * Math.log10(sum / Math.max(1, b - a) + 1e-20);
  });
}

/**
 * An arc as heard (queue item 102c): play `request` of `seed` for its
 * whole round and `extra` seconds of the next (at the wander's amount
 * `wander`: 1 the app's, 0 the plan alone with no breathing; 22.05 kHz), and read, unit by unit, its loudness by ear and its
 * brightness (the spectrum's centroid) beside the map's energy and role;
 * and from the player's calls, how the round ends: the last chord sounded
 * before its end (its notes' pitch classes, and the home key's root), and
 * what sounds over the landing (the RMS of each 50 ms from the landing's
 * start to the round's end, and the largest step between two samples
 * there, against the piece's own largest).
 */
export function arcChecks(seed, request, { extra = 20, rate = 44100, wander = 1 } = {}) {
  const head = pieceMap(seed, request, 0, 1).form;
  const sections = head.sections ?? 0;
  const { form, units } = pieceMap(seed, request, 0, Math.max(1, sections));
  const sectionSeconds = form.sectionBars / form.unitBars * form.unitSeconds;
  const barSeconds = form.unitSeconds / form.unitBars;
  const roundSeconds = units.reduce((s, u) => s + u.bars * barSeconds, 0);
  const seconds = Math.ceil(roundSeconds + extra);
  const wavFile = path.join(os.tmpdir(), `longform-arc-${seed}-${process.pid}.wav`);
  const callFile = path.join(os.tmpdir(), `longform-arc-${seed}-${process.pid}.calls`);
  dlrender(['--seed', String(seed), '--request', request, '--seconds', String(seconds), '--rate', String(rate), '--wander', String(wander), '--wav', wavFile, '--calls', callFile]);
  const { data } = readWav(wavFile);
  const calls = fs.readFileSync(callFile, 'utf8');
  fs.rmSync(wavFile, { force: true });
  fs.rmSync(callFile, { force: true });
  const start = 0.05;
  // Each unit's samples.
  let t = start;
  const spans = units.map((u) => {
    const a = Math.round(t * rate);
    t += u.bars * barSeconds;
    return [a, Math.min(data.length, Math.round(t * rate))];
  });
  // The landing (the round's last bars) is not the close's body: a unit's
  // loudness and brightness stop where it starts.
  const landAt = Math.round((start + roundSeconds - Math.max(1, Math.ceil(LAND / barSeconds)) * barSeconds) * rate);
  for (const sp of spans) sp[1] = Math.min(sp[1], landAt);
  // A unit all landing (or under a second before it) is left out.
  const body = spans.map(([a, b]) => b - a >= rate);
  const lufs = lufsOf(data, rate, spans.map(([a, b]) => [a, Math.max(a + 1, b)]));
  const bright = spans.map(([a, b]) => brightness(data, rate, a, Math.max(a + 1, b)));
  // The round's end, and the landing before it.
  const end = Math.round((start + roundSeconds) * rate);
  const landBars = Math.max(1, Math.ceil(LAND / barSeconds));
  const land = Math.round((start + roundSeconds - landBars * barSeconds) * rate);
  const win = Math.round(0.05 * rate);
  const rms = [];
  for (let a = land; a + win <= end; a += win) {
    let s = 0;
    for (let i = a; i < a + win; i++) s += data[i] * data[i];
    rms.push(10 * Math.log10(s / win + 1e-20));
  }
  // The close's body: the unit before the landing begins.
  const own = stepMax(data, Math.round(start * rate), land);
  const landStep = stepMax(data, land - win, end);
  const after = lufsOf(data, rate, [[end, Math.min(data.length, end + Math.round(8 * rate))]])[0];
  // The last chord sounded before the round's end: the chords channel's
  // (1) notes that start in the landing's first 0.4 s (a roll's three),
  // and any that start after them before the end (none: it plays alone).
  const landT = start + roundSeconds - landBars * barSeconds - 0.001;
  const endT = start + roundSeconds;
  const last = [];
  let lastT = Infinity;
  let later = 0;
  for (const line of calls.split('\n')) {
    const p = line.split(' ');
    // (Struck notes only: one at velocity 0 is silent, session 15.)
    if (p[0] !== 'note' || p[2] !== '1' || !(num(p[6]) > 0)) continue;
    const at = num(p[3]);
    if (at >= landT && at < landT + 0.4) {
      last.push(Math.round(num(p[4])));
      lastT = Math.min(lastT, at);
    } else if (at >= landT + 0.4 && at < endT - 1e-6) {
      later += 1;
    }
  }
  // The loop as itself (no length asked: no plan), at the same amount, on
  // the same spans: its own unit-to-unit jumps, which the arc is held to.
  const rest = request.split(',').filter((x) => !/^(bars|journey)=/.test(x)).join(',');
  dlrender(['--seed', String(seed), ...(rest ? ['--request', rest] : []), '--seconds', String(seconds), '--rate', String(rate), '--wander', String(wander), '--wav', wavFile]);
  const loop = readWav(wavFile).data;
  fs.rmSync(wavFile, { force: true });
  const whole = spans.map(([a, b], i) => b - a >= 0.99 * units[i].bars * barSeconds * rate);
  const ownLufs = lufsOf(loop, rate, spans.map(([a, b]) => [a, Math.max(a + 1, b)]));
  const ownBright = spans.map(([a, b]) => brightness(loop, rate, a, Math.max(a + 1, b)));
  const jumps = (list) => Math.max(0, ...list.slice(1).map((l, i) => (whole[i] && whole[i + 1] ? Math.abs(l - list[i]) : 0)));
  const ownJump = jumps(ownLufs);
  const jump = jumps(lufs);
  // The arc's own step between neighbors: what the plan adds to the loop's
  // own section-to-section change.
  const step = jumps(lufs.map((l, i) => l - ownLufs[i]));
  // The same between neighbors that both play the loop's own notes (theme
  // A whole, at home): there the loop as itself is the arc's reference
  // note for note. A unit in another key, a fragment (half of each phrase
  // a rest) or another tune plays other notes than the loop does in those
  // bars (session 14), so its step is what the notes are, not only what
  // the arc adds to them.
  const plain = units.map((u) => u.theme === 'A' && !u.fragment && !u.calls && u.key.relation === 'home' && !['answered', 'stretched', 'reharmonized'].includes(u.variation));
  const stepOwn = Math.max(0, ...lufs.slice(1).map((l, i) => (whole[i] && whole[i + 1] && plain[i] && plain[i + 1] ? Math.abs(l - ownLufs[i + 1] - (lufs[i] - ownLufs[i])) : 0)));
  // The units that sound the loop's own notes in its own voice and register
  // (session 14): a return an octave up or in another voice is brighter or
  // darker than the loop there by what the variation is, not by the arc's
  // energy, so the brightness order is reported on these beside all units.
  // (A lift's or a peak's register is session 13's double: its own.)
  const ownTone = units.map((u) => u.theme === 'A' && !u.fragment && !u.calls && u.key.relation === 'home' && (['plain', 'outline'].includes(u.variation) || (u.variation === 'register' && ['lift', 'peak'].includes(u.role))));
  const keep = (_, i) => body[i];
  // The arc's own shape: each unit against the loop as itself there.
  const lift = lufs.map((l, i) => l - ownLufs[i]);
  // (Brightness as a ratio, in cents of the centroid: 1200 an octave.)
  const brighter = bright.map((b, i) => 1200 * Math.log2(Math.max(1, b) / Math.max(1, ownBright[i])));
  return { form, units: units.filter(keep), lufs: lufs.filter(keep), bright: bright.filter(keep), lift: lift.filter(keep), brighter: brighter.filter(keep), ownTone: ownTone.filter(keep), rms, own, landStep, after, last, lastT, later, endT, sectionSeconds, jump, ownJump, step, stepOwn };
}

/** Spearman's rank correlation of two lists. */
function spearman(x, y) {
  const rank = (v) => {
    const order = v.map((x, i) => [x, i]).sort((a, b) => a[0] - b[0]);
    const r = new Array(v.length);
    for (let i = 0; i < order.length;) {
      let j = i;
      while (j + 1 < order.length && order[j + 1][0] === order[i][0]) j += 1;
      for (let k = i; k <= j; k++) r[order[k][1]] = (i + j) / 2;
      i = j + 1;
    }
    return r;
  };
  const [a, b] = [rank(x), rank(y)];
  const m = (v) => v.reduce((s, z) => s + z, 0) / v.length;
  const [ma, mb] = [m(a), m(b)];
  let num = 0, da = 0, db = 0;
  for (let i = 0; i < a.length; i++) {
    num += (a[i] - ma) * (b[i] - mb);
    da += (a[i] - ma) ** 2;
    db += (b[i] - mb) ** 2;
  }
  return da && db ? num / Math.sqrt(da * db) : 0;
}

/**
 * What `arcChecks` measured, judged (queue item 102c's yardsticks): the
 * loudness and the brightness rise from the opening to the peak and fall
 * from it to the close, in the planned order (each above the opening's and
 * the close's at the peak, and following the plan's energy role by role:
 * Spearman's rank correlation 0.5 or more between each role's measure and
 * its energy, the unit by unit figure beside it); the peak's loudness over the
 * opening's; the largest jump between two neighboring units (against the
 * loop's own); the last chord the tonic's; and a ring-out, not a cut. The
 * rise and fall and the peak over the opening are judged on the arc's own
 * shape: each unit against the loop as itself on the same bars, so a loop
 * whose own form is loud in one section and quiet in the next is not
 * mistaken for the plan (the figures as heard are given beside them).
 */
export function arcVerdict(r) {
  const { units, lufs, bright } = r;
  const power = (xs) => 10 * Math.log10(xs.reduce((s, x) => s + 10 ** (x / 10), 0) / xs.length);
  const mean = (xs) => xs.reduce((s, x) => s + x, 0) / xs.length;
  const of = (roles, list, avg) => {
    const xs = units.map((u, i) => (roles.includes(u.role) ? list[i] : null)).filter((x) => x != null);
    return xs.length ? avg(xs) : NaN;
  };
  const peakRoles = ['peak', 'lift'];
  const energy = units.map((u) => u.energy);
  const at = (list, avg) => ({ opening: of(['opening'], list, avg), peak: of(peakRoles, list, avg), close: of(['close'], list, avg) });
  // As heard, and the arc's own shape (each unit against the loop as itself).
  const loud = at(lufs, power);
  const tilt = at(bright, mean);
  const lift = at(r.lift, mean);
  const brighter = at(r.brighter, mean);
  // The planned order: the roles' (each role's units together, every arc
  // of the piece's), its measure against its energy; unit by unit beside
  // it, where a theme's 0.45 and 0.47 are asked to differ too.
  const roles = [...new Set(units.map((u) => u.role))];
  const byRole = (list) => roles.map((role) => mean(units.map((u, i) => (u.role === role ? list[i] : null)).filter((x) => x != null)));
  const roleEnergy = byRole(energy);
  const loudOrder = spearman(roleEnergy, byRole(r.lift));
  const brightOrder = spearman(roleEnergy, byRole(r.brighter));
  const unitOrder = [spearman(energy, r.lift), spearman(energy, r.brighter)];
  const rises = (m, order) => m.peak > m.opening && m.peak > m.close && order >= 0.5;
  // The same on the units in the loop's own voice and register alone
  // (reported, not held; null where the peak or an end has none).
  const tone = r.ownTone ?? units.map(() => true);
  const toneOf = (pick) => { const xs = units.map((u, i) => (tone[i] && pick(u) ? r.brighter[i] : null)).filter((x) => x != null); return xs.length ? mean(xs) : NaN; };
  const toneRoles = roles.filter((role) => units.some((u, i) => tone[i] && u.role === role));
  const toneAt = { opening: toneOf((u) => u.role === 'opening'), peak: toneOf((u) => peakRoles.includes(u.role)), close: toneOf((u) => u.role === 'close') };
  const toneOrder = spearman(toneRoles.map((role) => mean(units.filter((u, i) => tone[i] && u.role === role).map((u) => u.energy))), toneRoles.map((role) => toneOf((u) => u.role === role)));
  const brightOwn = [toneAt.opening, toneAt.peak, toneAt.close].some(Number.isNaN) ? null : rises(toneAt, toneOrder);
  const pc = (n) => ((n % 12) + 12) % 12;
  const NOTES = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
  // The tonic's triad: the mode's third and fifth on the root (or their
  // nearest where it has none), every one of them in the last chord, its
  // lowest note one of them (a loop may voice its tonic with a seventh or
  // a ninth beside it).
  const root = NOTES.indexOf(r.form.home.root);
  const steps = SCALE_STEPS[r.form.home.scale] ?? [0, 2, 4, 5, 7, 9, 11];
  const third = steps.includes(4) ? 4 : steps.includes(3) ? 3 : steps[1];
  const fifth = steps.includes(7) ? 7 : steps.includes(8) ? 8 : 6;
  const triad = [0, third, fifth].map((x) => (root + x) % 12);
  const pcs = new Set(r.last.map(pc));
  const low = r.last.length ? pc(Math.min(...r.last)) : -1;
  const tonic = r.later === 0 && r.last.length > 0 && (pcs.size < 3 ? pcs.has(root) : triad.every((t) => pcs.has(t)) && triad.includes(low));
  // A ring-out, not a cut: the landing's last 50 ms at least 15 dB under
  // its loudest, falling with no step of more than 12 dB between
  // neighboring 50 ms while it can be heard (above -60 dB: a cut falls 20
  // dB and more in one).
  const top = Math.max(...r.rms);
  const audible = (x) => Math.max(x, -60);
  const falls = r.rms.slice(1).map((x, i) => audible(r.rms[i]) - audible(x));
  const rings = r.rms.length > 2 && top - r.rms.at(-1) >= 15 && Math.max(...falls) <= 12;
  // No step past 2 LU between neighboring units in what the arc adds (the
  // loop's own form moves as it always did, and is reported beside).
  // (Session 14: 2 LU where both units play the loop's own notes; 3 LU
  // where one plays other notes, a fragment, another key or another tune.)
  const jumps = r.stepOwn <= 2 && r.step <= 3;
  const heard = (m, list) => m.peak > m.opening && m.peak > m.close && spearman(roleEnergy, byRole(list)) >= 0.5;
  return {
    loud, tilt, lift, brighter, loudOrder, brightOrder, unitOrder,
    loudRises: rises(lift, loudOrder), brightRises: rises(brighter, brightOrder), brightOwn,
    // The arc's own job (session 15): darker at the opening and the close
    // than at the peak, the roles between apart (a return an octave up, a
    // lift in another voice, a release in outline read brighter or darker
    // on purpose since session 14).
    brightEnds: brighter.peak > brighter.opening && brighter.peak > brighter.close,
    loudHeard: heard(loud, lufs), brightHeard: heard(tilt, bright),
    over: lift.peak - lift.opening, overHeard: loud.peak - loud.opening,
    jump: r.jump, ownJump: r.ownJump, step: r.step, stepOwn: r.stepOwn, jumps, tonic, rings, last: r.last,
  };
}

/**
 * An arc's joins (queue item 102c): `request` of `seed` and the same seed
 * as a plain loop, each played `seconds` at the wander's amount 0 (the
 * plan with no breathing, so the two play the same notes but for the
 * plan's), at 44.1 kHz; at every unit's line, from 20 ms before it to 300
 * ms after, the largest step between two samples in each. An arc has a
 * line at every unit, on its downbeats and their kicks, so a line is held
 * to the loop's own step in the same window, not to the piece elsewhere:
 * a click the plan makes is a step the loop as itself does not take.
 */
export function arcJoins(seed, request, seconds) {
  const one = (args) => {
    const file = path.join(os.tmpdir(), `longform-joins-${seed}-${process.pid}.wav`);
    dlrender(['--seed', String(seed), ...args, '--seconds', String(seconds), '--rate', '44100', '--wander', '0', '--wav', file]);
    const { rate, data } = readWav(file);
    fs.rmSync(file, { force: true });
    return { rate, data };
  };
  const piece = one(['--request', request]);
  const rest = request.split(',').filter((x) => !/^(bars|journey)=/.test(x)).join(',');
  const loop = one(rest ? ['--request', rest] : []);
  const { form, units } = pieceMap(seed, request, 0, 2000);
  const bar = form.unitSeconds / form.unitBars;
  const start = 0.05;
  const rate = piece.rate;
  let t = start;
  const joins = [];
  for (const u of units) {
    if (t > 30 && t + 0.3 < seconds) {
      const [a, b] = [Math.round((t - 0.02) * rate), Math.round((t + 0.3) * rate)];
      joins.push({ t, piece: stepMax(piece.data, a, b), loop: stepMax(loop.data, a, b) });
    }
    // Where a join begins inside the unit (session 15): `[longform joins]`'s
    // length before the line, a breath's share of it, the last beat for a
    // fill; the same window, held the same way.
    const len = Math.min(JOIN.bars, JOIN.share * u.bars) * bar;
    const from = { fill: Math.min(0.25 * bar, len), breath: len * JOIN.breath }[u.join] ?? len;
    const at = t + u.bars * bar - from;
    if (!['none', 'glide', 'prepared'].includes(u.join) && at > 30 && at + 0.3 < seconds) {
      const [a, b] = [Math.round((at - 0.02) * rate), Math.round((at + 0.3) * rate)];
      joins.push({ t: at, join: u.join, piece: stepMax(piece.data, a, b), loop: stepMax(loop.data, a, b) });
    }
    t += u.bars * bar;
  }
  return joins;
}

/**
 * An ending that lands (session 15; the brief's "Endings that land"):
 * `request` of `seed` played with the length slider set to `bars` bars
 * (`dlrender --track`: at the track's end the core's own next loop follows,
 * as in the app), at the wander's amount `wander`. Read: the stop (the
 * first phrase line `bars` bars on, as `track::phrase_line` finds it), the
 * chord struck at the landing's start (the chords channel's notes in its
 * first 0.4 s) against the home key's tonic triad, every note after those
 * and before the stop (none: the landing alone), any fade of the mix from
 * the landing on (none: it rings out, never cut), and the ring itself, from
 * the landing to the stop in 50 ms windows: no fall of more than 12 dB
 * between neighbors while above -60 dB (a cut's way).
 */
export function endingChecks(seed, request, bars, { wander = 1, rate = 44100 } = {}) {
  const { form } = pieceMap(seed, request, 0, 1);
  const barSeconds = form.unitSeconds / form.unitBars;
  const sectionBars = form.sectionBars;
  const phrase = sectionBars % 4 === 0 ? 4 : sectionBars % 2 === 0 ? 2 : sectionBars;
  // The first phrase line at or after the bars asked, in the section.
  const k = Math.floor(bars / sectionBars);
  const into = bars - k * sectionBars;
  const stopBars = k * sectionBars + Math.min(Math.ceil(into / phrase) * phrase, sectionBars);
  const landBars = Math.max(1, Math.ceil(LAND / barSeconds));
  const seconds = Math.ceil(0.05 + stopBars * barSeconds + 4);
  const wavFile = path.join(os.tmpdir(), `longform-end-${seed}-${process.pid}.wav`);
  const callFile = path.join(os.tmpdir(), `longform-end-${seed}-${process.pid}.calls`);
  dlrender(['--seed', String(seed), '--request', request, '--track', `bars=${bars}`, '--seconds', String(seconds), '--rate', String(rate), '--wander', String(wander), '--wav', wavFile, '--calls', callFile]);
  const { data } = readWav(wavFile);
  const calls = fs.readFileSync(callFile, 'utf8');
  fs.rmSync(wavFile, { force: true });
  fs.rmSync(callFile, { force: true });
  const notes = [];
  const fades = [];
  for (const line of calls.split('\n')) {
    const p = line.split(' ');
    if (p[0] === 'note') notes.push({ ch: Number(p[2]), t: num(p[3]), midi: num(p[4]), vel: num(p[6]) });
    else if (p[0] === 'fade') fades.push(num(p[1]));
  }
  // The core's clock: the first note's time is the piece's first step.
  const t0 = Math.min(...notes.map((n) => n.t));
  const stopT = t0 + stopBars * barSeconds;
  const landT = stopT - landBars * barSeconds;
  // (Struck: a note at velocity 0 is silent.)
  const chord = notes.filter((n) => n.ch === 1 && n.vel > 0 && n.t >= landT - 0.02 && n.t < landT + 0.4).map((n) => Math.round(n.midi));
  // (The next loop's first notes may fall a few milliseconds early: their jitter.)
  const later = notes.filter((n) => n.vel > 0 && n.t >= landT + 0.4 && n.t < stopT - 0.02).length;
  const faded = fades.filter((t) => t >= landT - 0.02 && t < stopT + 1).length;
  const pc = (x) => ((x % 12) + 12) % 12;
  const NOTES = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
  const root = NOTES.indexOf(form.home.root);
  const steps = SCALE_STEPS[form.home.scale] ?? [0, 2, 4, 5, 7, 9, 11];
  const third = steps.includes(4) ? 4 : steps.includes(3) ? 3 : steps[1];
  const fifth = steps.includes(7) ? 7 : steps.includes(8) ? 8 : 6;
  const triad = [0, third, fifth].map((x) => (root + x) % 12);
  const pcs = new Set(chord.map(pc));
  const tonic = chord.length > 0 && (pcs.size < 3 ? pcs.has(root) : triad.every((t) => pcs.has(t)) && triad.includes(pc(Math.min(...chord))));
  const win = Math.round(0.05 * rate);
  const rms = [];
  for (let a = Math.round(landT * rate); a + win <= Math.min(data.length, Math.round(stopT * rate)); a += win) {
    let s = 0;
    for (let i = a; i < a + win; i++) s += data[i] * data[i];
    rms.push(10 * Math.log10(s / win + 1e-20));
  }
  const audible = (x) => Math.max(x, -60);
  const falls = rms.slice(1).map((x, i) => audible(rms[i]) - audible(x));
  const fall = Math.max(0, ...falls);
  const rings = rms.length > 2 && fall <= 12 && faded === 0;
  return { seed, request, bars, stopBars, stopT, chord, tonic, later, alone: later === 0, faded, rings, fall, landed: chord.length > 0 };
}
