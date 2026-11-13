// An arc's themes, variations and keys as they are played (queue item 102d,
// session 14; docs/LONGFORM.md, "How an arc's themes are heard").
//
// Run with:  node tools/longform-themes.mjs [--seeds 40] [--long 6] [--one SEED REQUEST]
//
// Each corpus piece (corpus seed 20261009, `journey=arc`, 256 bars; `--long`
// of them at 1,024 too) is played by the player at the wander's amount 0
// (`dlrender --seed --calls`), and the notes it handed the core are read
// unit by unit beside the piece's map (tools/gen-lib.mjs's `pieceMap`):
//
//   key        every pitched layer's notes are in the unit's key: the share
//              of them outside its mode no higher than the loop's own
//              outside its home mode (a bass holding the home fifth before
//              a return apart)
//   second     where the map says theme B, the tune is another tune (under
//              0.9 of theme A's notes kept), or theme A in the unit's key
//              where the second theme's own loop has no tune (a theme
//              that rests stays resting, queue item 85f: counted, one in
//              five at most), never silent
//   return     theme A comes home changed: the share of the loop's own tune
//              kept in the return's first unit (the same step and pitch
//              class; a stretched return against the tune stretched, an
//              answered one against the half of each phrase it keeps) 0.35
//              at least, and the unit not a copy (the tune's notes, its
//              voice or the chords under it differ)
//   variations every variation is played somewhere in the corpus
//   clash      the tune's time a semitone over a sounding chord tone, as a
//              share of the tune's time: the corpus's mean over every unit
//              no higher than over theme A's own statements
//   octave     where the loop keeps its tune's centre five semitones from
//              its chords', so does every unit
//
// `--one` prints one piece unit by unit. Needs the tools' build
// (tools/build-core.sh --tools); builds dlrender.

import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { parts, pieceMap } from './gen-lib.mjs';
import { dlrender, readCalls } from './longform-played.mjs';
import { Rng } from '../js/rng.js';
import { SCALES } from '../js/theory.js';

const args = process.argv.slice(2);
const value = (name, def) => {
  const i = args.indexOf(name);
  return i >= 0 ? args[i + 1] : def;
};
const SEEDS = Number(value('--seeds', 40));
const LONG = Number(value('--long', 6));
const NOTES = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
const [MELODY, CHORDS, BASS, TEXTURE] = [0, 1, 2, 3];
const VARIATIONS = ['key', 'register', 'voice', 'stretched', 'reharmonized', 'answered', 'outline'];
const mod = (x, n) => ((x % n) + n) % n;
const pcsOf = (key) => new Set(SCALES[key.scale].steps.map((s) => mod(NOTES.indexOf(key.root) + s, 12)));
const mean = (v) => (v.length ? v.reduce((a, b) => a + b, 0) / v.length : NaN);

/** A piece played: its units, each with its notes by layer (steps counted from the unit's first). */
export function play(seed, request) {
  const p = parts(seed, request);
  const head = pieceMap(seed, request, 0, 1);
  const sections = head.form.sections;
  const map = pieceMap(seed, request, 0, sections);
  const spb = p.spec.stepsPerBar;
  const seconds = Math.ceil((p.spec.length ?? sections * p.spec.bars) * spb * (60 / p.spec.bpm / 4)) + 2;
  const file = path.join(os.tmpdir(), `themes-${seed}-${process.pid}.calls`);
  dlrender(['--seed', String(seed), '--request', request, '--seconds', String(seconds), '--rate', '22050', '--wander', '0', '--calls', file]);
  const { secs } = readCalls(file);
  fs.rmSync(file, { force: true });
  const units = [];
  let from = 0;
  for (const u of map.units) {
    if (u.phrase === 0) from = 0;
    const to = from + u.bars * spb;
    const s = secs.get(u.section);
    const layers = [0, 1, 2, 3].map((l) => (s ? s.layers[l].filter((n) => n.step >= from && n.step < to).map((n) => ({ ...n, step: n.step - from })) : []));
    units.push({ ...u, from, to, steps: to - from, layers });
    from = to;
  }
  // The loop as composed: its tune and its chords, by section step.
  const own = { melody: p.melody.filter((e) => e.vel > 0), chords: p.chords.filter((e) => e.vel > 0 && e.notes?.length) };
  // A tune that turns on a cycle of its own plays other bars each section:
  // nothing to hold a return against.
  const cycle = p.passes[0].cycles?.melody;
  const drifts = Boolean(cycle) && cycle !== p.spec.bars * spb;
  return { seed, request, spec: p.spec, form: map.form, units, spb, own, drifts };
}

/** The share of a unit's notes (the given layers) outside the key `key`. */
function outside(unit, key, layers) {
  const pcs = pcsOf(key);
  const notes = layers.flatMap((l) => unit.layers[l]);
  return notes.length ? notes.filter((n) => !pcs.has(mod(Math.round(n.midi), 12))).length / notes.length : 0;
}

/** The tune's time a semitone over a sounding chord tone, and the tune's time, in steps. */
function clash(unit) {
  let [over, all] = [0, 0];
  for (const m of unit.layers[MELODY]) {
    all += m.dur;
    let worst = 0;
    for (const c of unit.layers[CHORDS]) {
      if (mod(Math.round(m.midi - c.midi), 12) !== 1) continue;
      worst = Math.max(worst, Math.min(m.t + m.dur, c.t + c.dur) - Math.max(m.t, c.t));
    }
    over += Math.max(0, worst);
  }
  return [over, all];
}

const centre = (notes) => mean(notes.map((n) => n.midi));
const tuneSet = (notes, f = (n) => n.step) => new Set(notes.map((n) => `${f(n)}|${mod(Math.round(n.midi), 12)}`));
/** The share of `a`'s notes (step and pitch class) that `b` plays too. */
const kept = (a, b) => {
  if (!a.size) return NaN;
  let n = 0;
  for (const k of a) if (b.has(k)) n += 1;
  return n / a.size;
};
const bag = (unit) => unit.layers.map((l) => l.map((n) => `${n.step}|${n.kind}|${Math.round(n.midi * 100)}|${Math.round(n.dur * 1000)}`).sort().join(' ')).join('/');

/** The checks of one piece: the problems found, and what was counted. */
export function themeChecks(piece) {
  const { units, form, spb, own, drifts } = piece;
  const problems = [];
  const seen = { variations: {}, rests: 0, seconds: 0, returns: [], clash: {}, likeA: 0, prepared: 0, calls: 0 };
  const home = form.home;
  const all = [MELODY, CHORDS, BASS, TEXTURE];
  const statements = units.filter((u) => u.role === 'theme' && u.variation === 'plain');
  const ownOut = Math.max(0, ...units.filter((u) => u.key.relation === 'home' && u.variation !== 'reharmonized').map((u) => outside(u, home, all)));
  // (A voice is its number and its first setting: two of a family share the number.)
  const kinds = new Set(statements.flatMap((u) => u.layers[MELODY].map((n) => `${n.kind}/${n.tone}`)));
  const phrase = (form.sectionBars % 4 === 0 ? 4 : form.sectionBars % 2 === 0 ? 2 : form.sectionBars) * spb;
  // The loop's own tune in a unit's steps (counted from the unit's first), and its gap over the chords there.
  const tuneIn = (u) => own.melody.filter((e) => e.step >= u.from && e.step < u.to).map((e) => ({ ...e, step: e.step - u.from }));
  const gapIn = (u) => {
    const c = own.chords.filter((e) => e.step >= u.from && e.step < u.to).flatMap((e) => e.notes);
    const m = tuneIn(u);
    return m.length >= 3 && c.length ? mean(m.map((e) => e.midi)) - mean(c) : NaN;
  };
  const seconds = new Map();
  const add = (label, [over, total]) => {
    const c = (seen.clash[label] ??= [0, 0]);
    c[0] += over;
    c[1] += total;
  };
  for (const [x, u] of units.entries()) {
    const where = `${u.section}.${u.phrase} ${u.role}`;
    if (u.journey !== 'arc') continue;
    // (Where the plan moved a pitch: at home the loop is its own notes.)
    const pitched = u.key.relation !== 'home' || ['register', 'reharmonized', 'key'].includes(u.variation);
    const out = outside(u, u.key, u.prepares ? [MELODY, CHORDS, TEXTURE] : all);
    if (pitched && out > ownOut + 0.05) problems.push(`${where}: ${(out * 100).toFixed(0)}% of its notes outside ${u.key.root} ${u.key.scale} (the loop's own ${(ownOut * 100).toFixed(0)}%)`);
    if (u.prepares) seen.prepared += 1;
    if (u.calls) seen.calls += 1;
    const b = u.theme === 'B' || u.variation === 'answered';
    add(u.role === 'theme' ? 'theme A stated' : u.variation === 'reharmonized' ? 'reharmonized' : u.key.relation === 'parallel' ? 'the parallel key' : b ? 'another tune (B, an answer)' : 'theme A elsewhere', clash(u));
    const a = tuneIn(u);
    const tune = u.layers[MELODY];
    // The octave rule: theme A's units against the loop's own in the same
    // bars; a second theme's over all its statement.
    if (pitched && !drifts && !b && !u.fragment && tune.length >= 3 && u.layers[CHORDS].length && Math.abs(gapIn(u)) >= 5) {
      const gap = centre(tune) - centre(u.layers[CHORDS]);
      if (Math.abs(gap) < 4) problems.push(`${where}: the tune's centre ${gap.toFixed(1)} from the chords' (the loop's own ${gapIn(u).toFixed(1)} in these bars)`);
    }
    if (u.theme === 'B' && !u.fragment && u.role === 'second') {
      const s = seconds.get(u.part) ?? { tune: [], chords: [], like: [], where };
      s.tune.push(...tune.map((n) => n.midi));
      s.chords.push(...u.layers[CHORDS].map((n) => n.midi));
      // Against theme A, moved any whole way.
      if (a.length >= 4 && !drifts) s.like.push(Math.max(...Array.from({ length: 12 }, (_, d) => kept(tuneSet(a.map((n) => ({ ...n, midi: n.midi + d }))), tuneSet(tune)))));
      seconds.set(u.part, s);
    }
    if (u.role === 'return' && units[x - 1]?.role !== 'return') {
      const stretched = a.filter((n) => mod(n.step + u.from, phrase) < phrase / 2).map((n) => ({ ...n, step: n.step + mod(n.step + u.from, phrase) }));
      // (Answered: its first halves are what is kept.)
      const firsts = a.filter((n) => mod(n.step + u.from, phrase) < phrase / 2);
      const share = kept(tuneSet(u.variation === 'stretched' ? stretched : u.variation === 'answered' ? firsts : a), tuneSet(tune));
      const set = (notes) => notes.map((n) => `${n.step}|${Math.round(n.midi)}`).sort().join(' ');
      const chordsOwn = own.chords.filter((e) => e.step >= u.from && e.step < u.to).flatMap((e) => e.notes.map((midi) => ({ step: e.step - u.from, midi })));
      const copy = set(a) === set(tune) && set(chordsOwn) === set(u.layers[CHORDS]) && tune.every((n) => kinds.has(`${n.kind}/${n.tone}`));
      if (a.length >= 4 && tune.length && !drifts && (u.variation !== 'answered' || firsts.length >= 2)) {
        seen.returns.push({ variation: u.variation, kept: share, copy });
        if (!(share >= 0.35)) problems.push(`${where}: the return (${u.variation}) keeps ${(share * 100).toFixed(0)}% of theme A's notes`);
        if (copy) problems.push(`${where}: the return (${u.variation}) is a copy of theme A as stated`);
      }
      if (u.key.relation !== 'home') problems.push(`${where}: the return is not in the home key`);
    }
    if (u.variation !== 'plain') seen.variations[u.variation] = (seen.variations[u.variation] ?? 0) + 1;
  }
  const ownGap = own.melody.length && own.chords.length ? mean(own.melody.map((e) => e.midi)) - mean(own.chords.flatMap((e) => e.notes)) : NaN;
  for (const s of seconds.values()) {
    seen.seconds += 1;
    if (!s.tune.length) {
      seen.rests += 1;
      continue;
    }
    if (s.like.length && mean(s.like) >= 0.9) seen.likeA += 1;
    const gap = mean(s.tune) - mean(s.chords);
    if (Math.abs(ownGap) >= 5 && s.tune.length >= 4 && Math.abs(gap) < 4) problems.push(`${s.where}: the second theme's centre ${gap.toFixed(1)} from the chords' (the loop's own ${ownGap.toFixed(1)})`);
  }
  return { problems, seen };
}

function one(seed, request) {
  const piece = play(seed, request);
  const { problems, seen } = themeChecks(piece);
  const sd = 60 / piece.spec.bpm / 4;
  let t = 0;
  for (const u of piece.units) {
    const [over, all] = clash(u);
    const gap = u.layers[MELODY].length && u.layers[CHORDS].length ? (centre(u.layers[MELODY]) - centre(u.layers[CHORDS])).toFixed(1) : '-';
    const clock = `${Math.floor(t / 60)}:${String(Math.floor(t % 60)).padStart(2, '0')}`;
    console.log(
      `${String(u.section).padStart(4)}.${u.phrase} ${clock.padStart(6)} ${u.role.padEnd(11)} ${(u.theme + (u.fragment ? ' frag' : '') + ' ' + u.variation).padEnd(18)} ${(u.key.root + ' ' + u.key.scale).padEnd(14)} tune ${String(u.layers[MELODY].length).padStart(3)} chords ${String(u.layers[CHORDS].length).padStart(3)} bass ${String(u.layers[BASS].length).padStart(3)} out ${(outside(u, u.key, [0, 1, 2, 3]) * 100).toFixed(0).padStart(3)}% clash ${(all ? (over / all) * 100 : 0).toFixed(0).padStart(3)}% gap ${gap}${u.calls ? ' call' : ''}${u.prepares ? ' pedal' : ''} voices ${[...new Set(u.layers[MELODY].map((n) => n.kind))].join(',')}`,
    );
    t += u.steps * sd;
  }
  console.log(JSON.stringify(seen));
  for (const p of problems) console.log(`  ! ${p}`);
}

function corpus() {
  const rng = new Rng(20261009);
  const seeds = Array.from({ length: SEEDS }, () => rng.seed32());
  const total = { pieces: 0, problems: 0, variations: {}, seconds: 0, rests: 0, likeA: 0, returns: {}, copies: 0, clash: {}, prepared: 0, calls: 0 };
  const failing = [];
  const run = (seed, request) => {
    const { problems, seen } = themeChecks(play(seed, request));
    total.pieces += 1;
    if (problems.length) failing.push({ seed, request, problems });
    total.problems += problems.length;
    for (const [k, n] of Object.entries(seen.variations)) total.variations[k] = (total.variations[k] ?? 0) + n;
    for (const k of ['seconds', 'rests', 'likeA', 'prepared', 'calls']) total[k] += seen[k];
    for (const r of seen.returns) (total.returns[r.variation] ??= []).push(r.kept);
    for (const [k, v] of Object.entries(seen.clash)) v.forEach((n, i) => ((total.clash[k] ??= [0, 0])[i] += n));
  };
  for (const [i, seed] of seeds.entries()) {
    run(seed, 'bars=256,journey=arc');
    if (i < LONG) run(seed, 'bars=1024,journey=arc');
  }
  const share = ([over, all]) => (all ? over / all : 0);
  console.log(`${total.pieces} arc pieces played (${SEEDS} at 256 bars, ${Math.min(LONG, SEEDS)} at 1,024)`);
  console.log(`second themes stated: ${total.seconds}; theme A in the key, where the second theme rests (85f): ${total.likeA}; silent: ${total.rests}`);
  console.log(`returns, the share of theme A's notes kept: ${Object.entries(total.returns).map(([k, v]) => `${k} ${v.length} (${Math.min(...v).toFixed(2)} to ${Math.max(...v).toFixed(2)}, mean ${mean(v.filter((x) => !Number.isNaN(x))).toFixed(2)})`).join('; ')}`);
  console.log(`variations played (units): ${VARIATIONS.map((v) => `${v} ${total.variations[v] ?? 0}`).join(', ')}; returns prepared ${total.prepared}; call-and-response units ${total.calls}`);
  const rest = Object.entries(total.clash).filter(([k]) => k !== 'theme A stated').reduce((a, [, v]) => [a[0] + v[0], a[1] + v[1]], [0, 0]);
  console.log(`clash (the tune a semitone over a sounding chord tone, share of its time): ${Object.entries(total.clash).map(([k, v]) => `${k} ${(share(v) * 100).toFixed(2)}%`).join('; ')}; every unit but the statements ${(share(rest) * 100).toFixed(2)}%`);
  let bad = total.problems;
  for (const f of failing.slice(0, 12)) {
    console.log(`  seed ${f.seed} ${f.request}:`);
    for (const p of f.problems.slice(0, 4)) console.log(`    ${p}`);
  }
  const missing = VARIATIONS.filter((v) => !total.variations[v]);
  if (missing.length) {
    console.log(`  never played: ${missing.join(', ')}`);
    bad += 1;
  }
  if (share(rest) > share(total.clash['theme A stated'] ?? [0, 0]) + 0.0005) {
    console.log('  the units clash more than theme A alone does');
    bad += 1;
  }
  if (total.likeA * 5 > total.seconds || total.rests) {
    console.log('  too many second themes are theme A again, or one is silent');
    bad += 1;
  }
  console.log(bad ? `FAILED: ${total.problems} problems in ${failing.length} pieces` : 'ok');
  return bad ? 1 : 0;
}

if (import.meta.url === `file://${process.argv[1]}`) {
  if (args[0] === '--one') one(Number(args[1]), args[2]);
  else process.exit(corpus());
}
