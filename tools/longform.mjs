#!/usr/bin/env node
// Long pieces, composed (queue item 102; docs/LONGFORM.md): the piece map's
// promises, checked over a corpus on the core's own plan
// (core/src/generator/longform.rs, read through the tools' build:
// tools/build-core.sh --tools). The same checks as core/tests/longform.rs,
// as a table a person reads; sessions 12 to 15 add what is played to it.
// (That the plan is the same wherever the wander's slider stands is the
// Rust test's: it plays the player at 0, 1 and 2.4.)
//
//   node tools/longform.mjs                 the table, 240 seeds, and the
//                                           wander as played (a few minutes
//                                           more: tools/longform-played.mjs)
//   node tools/longform.mjs --seeds 60      fewer (or more) seeds
//   node tools/longform.mjs --played 8      that many wander seeds played (6)
//   node tools/longform.mjs --arcs 30       that many arc seeds played at 64
//                                           bars (30; 256 bars: 12, 1,024:
//                                           4, each two fifths as many again)
//   node tools/longform.mjs --plan          the plan's checks alone
//   node tools/longform.mjs --check         exit 1 if any check fails
//   node tools/longform.mjs --map SEED REQUEST [--from N] [--count N]
//                                           a piece's map as JSON lines
//                                           (dlgen --map prints it for people)
//
// Each seed is planned at 64, 256, 1,024 and 32,768 bars and Endless, as an
// arc and as a wander (journey=arc, journey=wander). Since session 12 the
// wander is heard: its checks are made again on what the player plays (the
// corpus's first pieces, rendered by `dlrender`, built first), with the
// joins' clicks, the loudness over twenty minutes and the cost of a
// section's making and of its block. Since session 13 the arc is heard
// too: corpus pieces at 64, 256 and 1,024 bars, rendered whole, measured
// unit by unit against the loop as itself (tools/longform-played.mjs's
// `arcChecks`, `arcVerdict`): the loudness and the brightness rise to the
// peak and fall, the peak's swell, the steps, the last chord the tonic's,
// the ring-out; then the joins, a section's cost and its block.

import { execFileSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { pieceMap } from './gen-lib.mjs';
import { arcChecks, arcJoins, arcVerdict, dlrender, loopLoudness, playedChecks, soundChecks } from './longform-played.mjs';
import { Rng } from '../js/rng.js';

const args = process.argv.slice(2);
const flag = (name) => args.includes(name);
const value = (name, fallback) => (args.includes(name) ? args[args.indexOf(name) + 1] : fallback);

if (flag('--map')) {
  const i = args.indexOf('--map');
  const map = pieceMap(Number(args[i + 1]), args[i + 2] ?? '-', Number(value('--from', 0)), Number(value('--count', 64)));
  if (!map) {
    console.error('longform: no long piece (ask a length past 32 bars)');
    process.exit(1);
  }
  console.log(JSON.stringify(map.form));
  for (const u of map.units) console.log(JSON.stringify(u));
  process.exit(0);
}

const SEEDS = Number(value('--seeds', 240));
const LENGTHS = ['bars=64', 'bars=256', 'bars=1024', 'bars=32768', 'bars=endless'];
const JOURNEYS = ['arc', 'wander'];
const THINGS = ['theme', 'key', 'layers', 'voice', 'warmth', 'space', 'register', 'length', 'density'];
// How many sections of a piece are read: the whole of a short one, the
// start of a long one, and for Endless a stretch from far in as well.
const MOST = 1500;
const FAR = 1e9;

const checks = new Map();
const check = (name, ok, where) => {
  const c = checks.get(name) ?? { name, seen: 0, failed: 0, first: null };
  c.seen += 1;
  if (!ok) {
    c.failed += 1;
    c.first ??= where;
  }
  checks.set(name, c);
};

const sameKey = (a, b) => a.root === b.root && a.scale === b.scale;
const things = (u) => u.changed.filter((c) => THINGS.includes(c));
// A unit without what changed from the one before: what it is.
const bare = (u) => JSON.stringify({ ...u, changed: null, section: null, phrase: null });

/** The movements' kinds (session 14), over a whole finite piece's units or a stretch of Endless's. */
function movementChecks(where, form, units, whole) {
  const kinds = [];
  for (const u of units) if (kinds.at(-1)?.part !== u.part) kinds.push({ part: u.part, kind: u.movement });
  if (form.cycle.length === 1) {
    check('a piece of one or two arcs, or with no room for kinds, is full arcs', kinds.every((k) => k.kind === 'full'), where);
    return;
  }
  check('no two neighboring movements are the same kind unless both are full', kinds.every((k, i) => i === 0 || k.kind !== kinds[i - 1].kind || k.kind === 'full'), where);
  if (!whole) return;
  check('the first movement and the last are full arcs', kinds[0].kind === 'full' && kinds.at(-1).kind === 'full' && kinds.length === form.arcs, where);
  if (form.arcs >= 4) check('four movements or more: every kind appears', ['full', 'gentle', 'light', 'interlude'].every((k) => kinds.some((x) => x.kind === k)), where);
  else check('three movements: no interlude', kinds.every((k) => k.kind !== 'interlude'), where);
}

function arcPlanChecks(where, form, units) {
  // Whole arcs only.
  for (let at = 0; at < units.length;) {
    let end = at;
    while (end < units.length && units[end].part === units[at].part) end += 1;
    const run = units.slice(at, end);
    at = end;
    // An interlude (session 14) is the wander's way for a span: theme A at
    // home, one thing moving at a time, the loop's own at both its ends.
    if (run[0].movement === 'interlude') {
      const started = at - run.length > 0 && units[at - run.length - 1].movement !== 'interlude';
      const whole = started && at < units.length;
      const own = (u) => !u.rest && !u.voice && u.warmth === 0 && u.space === 0 && u.register === 0 && u.length === 0 && u.density === 1;
      const moves = (u) => u.changed.filter((c) => !['role', 'theme', 'variation', 'key'].includes(c)).length;
      check('an interlude keeps theme A at home and moves one thing at a time', run.every((u) => u.theme === 'A' && sameKey(u.key, form.home) && moves(u) <= 1 && !u.closes), where);
      if (whole) check("an interlude is the loop's own at both its ends, and moves something between", own(run[0]) && own(run.at(-1)) && (form.turns === 0 || run.some((u) => !own(u))), where);
      continue;
    }
    if (run.length !== run[0].of) continue;
    const top = Math.max(...run.map((u) => u.energy));
    const first = run.findIndex((u) => u.energy === top);
    const last = run.findLastIndex((u) => u.energy === top);
    const rises = run.slice(0, first + 1).every((u, i, a) => i === 0 || a[i - 1].energy <= u.energy);
    const falls = run.slice(last).every((u, i, a) => i === 0 || a[i - 1].energy >= u.energy);
    check("an arc's energy rises to one peak and falls", rises && falls && run[0].energy < top && run.at(-1).energy < top, where);
    check("an arc's peak is its peak's (or the lift's), from a third to 0.85 of the way", run.slice(first, last + 1).every((u) => u.role === 'peak' || u.role === 'lift') && first / run.length > 0.3 && last / run.length < 0.85, where);
    const stated = run.findIndex((u) => u.role === 'theme');
    const back = run.findIndex((u) => u.role === 'return');
    const home = (u) => sameKey(u.key, form.home);
    check('theme A is stated plain, in the home key', stated >= 0 && run[stated].theme === 'A' && run[stated].variation === 'plain' && home(run[stated]), where);
    check('theme A returns after it, changed, in the home key', back > stated && run[back].theme === 'A' && !['plain', 'key'].includes(run[back].variation) && home(run[back]), where);
    if (form.second && !form.small && run[0].movement !== 'light') {
      const b = run.findIndex((u) => u.role === 'second' && u.theme === 'B');
      check('theme B comes between A and its return, in a related key', stated < b && b < back && (form.related.length === 0 || !home(run[b])), where);
    }
    const ends = run.at(-1).role;
    check('an arc ends in a close or a breath', ends === 'close' || ends === 'breath', where);
  }
}

function wanderChecks(where, form, units, fromStart) {
  let last = null;
  let one = true;
  let held = true;
  let turns = true;
  let never = true;
  units.forEach((u, x) => {
    if (fromStart && x === 0) return;
    const moved = things(u);
    if (moved.length > 1) one = false;
    if (u.closes) return;
    if (u.at > 0) {
      if (moved.length || u.role !== 'holding') held = false;
    } else if (x > 0 && !units[x - 1].closes) {
      if (!moved.length || u.role !== 'turning') turns = false;
      if (form.turns >= 4 && last === moved[0] && moved[0] !== 'layers') never = false;
      last = moved[0];
      holds.count += 1;
      holds.seconds += u.of * form.unitSeconds;
      holds.things[moved[0]] = (holds.things[moved[0]] ?? 0) + 1;
    }
  });
  check('a wander moves at most one thing a unit', one, where);
  check('a hold is held: nothing moves inside it', held, where);
  check("at a hold's end one thing moves", turns, where);
  check('never the thing that moved last (a layer back from its rest apart)', never, where);
}
const holds = { count: 0, seconds: 0, things: {} };

// The corpus's seeds: the corpus seed's own stream.
const rng = new Rng(20261009);
const seeds = Array.from({ length: SEEDS }, () => Math.floor(rng.f() * 4294967296));
const journeys = { arc: 0, wander: 0 };
const grains = { section: 0, phrase: 0 };
let pieces = 0;
const t0 = Date.now();

for (const seed of seeds) {
  for (const length of LENGTHS) {
    // The journey a piece draws when none is asked.
    journeys[pieceMap(seed, length, 0, 1).form.journey] += 1;
    for (const journey of JOURNEYS) {
      const request = `${length},journey=${journey}`;
      const where = `${seed} "${request}"`;
      const head = pieceMap(seed, request, 0, 1).form;
      const sections = head.sections;
      const count = sections == null ? MOST : Math.min(sections, MOST);
      const { form, units } = pieceMap(seed, request, 0, count);
      pieces += 1;
      grains[form.grain] += 1;
      check('the journey asked is the piece\'s', form.journey === journey && form.asked, where);

      const keys = [form.home, ...form.related];
      check('no key outside the related set', units.every((u) => keys.some((k) => sameKey(k, u.key))), where);
      check('the tempo is the loop\'s own throughout', units.every((u) => u.bpm === units[0].bpm), where);

      if (journey === 'arc') {
        arcPlanChecks(where, form, units);
        movementChecks(where, form, units, form.sections != null && units.at(-1).section === form.sections - 1);
      }
      else wanderChecks(where, form, units, true);

      if (sections != null) {
        // The piece's end, straight from its number.
        const tail = pieceMap(seed, request, Math.max(0, sections - 40), Math.min(sections, 40) + 1).units;
        const again = tail.filter((u) => u.section === sections)[0];
        const end = tail.filter((u) => u.section === sections - 1).at(-1);
        check('a finite piece\'s last section is a close on the tonic', end.role === 'close' && end.closes && sameKey(end.key, form.home) && end.theme === 'A', where);
        check('the round after it begins at the opening again', bare(again) === bare(units[0]), where);
        const closing = tail.filter((u) => u.section < sections && u.closes);
        check('a close lasts ten seconds at least', closing.reduce((s, u) => s + u.bars, 0) / form.unitBars * form.unitSeconds >= 9.99, where);
      } else {
        const far = pieceMap(seed, request, FAR, 400).units;
        check('Endless never closes', [...units, ...far].every((u) => !u.closes && u.role !== 'close'), where);
        check('no key outside the related set', far.every((u) => keys.some((k) => sameKey(k, u.key))), where);
        if (journey === 'wander') wanderChecks(where, form, far, false);
        else arcPlanChecks(where, form, far.slice(far.findIndex((u) => u.at === 0)));
      }

      // Straight or through: a section asked alone is the one in the run.
      for (const x of [Math.floor(units.length / 3), units.length - 1]) {
        const u = units[x];
        const alone = pieceMap(seed, request, u.section, 1).units.find((v) => v.phrase === u.phrase);
        check('a section is the same reached straight or through', JSON.stringify(alone) === JSON.stringify(u), where);
      }
    }
  }
}

// What was asked is never moved (a few seeds: the Rust test walks more).
for (const seed of seeds.slice(0, 20)) {
  for (const journey of JOURNEYS) {
    const map = (ask) => pieceMap(seed, `bars=endless,journey=${journey},${ask}`, 0, 1200);
    const where = `${seed} ${journey}`;
    let m = map('key=D');
    check('a key asked is the home key, and a wander never leaves it', m.form.home.root === 'D' && (journey === 'arc' || m.units.every((u) => sameKey(u.key, m.form.home))), where);
    m = map('scale=aeolian');
    check('a scale asked rules out the parallel mode', m.units.every((u) => u.key.relation !== 'parallel'), where);
    m = map('voice.melody=flute');
    check('a voice asked is never handed on', m.units.every((u) => u.variation !== 'voice' && u.voice !== 'melody'), where);
    m = map('layer.drums=on,layer.texture=off,edit.roll.bass=5');
    check('a layer asked or re-rolled never rests', m.units.every((u) => !['drums', 'texture', 'bass'].includes(u.rest)), where);
    m = map('edit.warmth=0.6,edit.space=0.2,bpm=77');
    check('a tone edited and a tempo asked are never moved', m.units.every((u) => u.warmth === 0 && u.space === 0 && u.bpm === 77), where);
  }
}

// An arc lasts about as long in minutes at 50 bpm as at 150 (its spans are
// set in time): the same seed asked both tempos.
for (const seed of seeds.slice(0, 40)) {
  const at = (bpm) => pieceMap(seed, `bars=endless,journey=arc,bpm=${bpm}`, 0, 1).form;
  const [slow, fast] = [at(50), at(150)];
  const minutes = (f) => f.arcUnits * f.unitSeconds / 60;
  const unit = Math.max(slow.unitSeconds, fast.unitSeconds) / 60;
  check('an arc lasts as long in minutes at 50 bpm as at 150', Math.abs(minutes(slow) - minutes(fast)) <= unit + 0.05 && minutes(slow) >= 3.5 && minutes(slow) <= 16, `${seed}: ${minutes(slow).toFixed(1)} and ${minutes(fast).toFixed(1)} minutes`);
}

// A section's cost, timed: the map of 200 sections from section 1, from
// 10^5 and from 10^9, the least of several runs each.
const timed = {};
for (const journey of JOURNEYS) {
  const request = `bars=endless,journey=${journey}`;
  const time = (from) => {
    let best = Infinity;
    for (let run = 0; run < 7; run++) {
      const t = process.hrtime.bigint();
      for (let k = 0; k < 10; k++) pieceMap(seeds[5], request, from, 200);
      best = Math.min(best, Number(process.hrtime.bigint() - t) / 1e6 / 2000);
    }
    return best;
  };
  timed[journey] = [time(1), time(1e5), time(1e9)];
  const [, on, far] = timed[journey];
  check("a section's plan costs the same at 10^9 as at 10^5", far < on * 1.5 + 0.002, `${journey}: ${timed[journey].map((t) => (t * 1000).toFixed(1)).join(', ')} us`);
}

const mean = holds.seconds / Math.max(1, holds.count);
check('holds as long as drawn: between one and five minutes on average', mean >= 90 && mean <= 270, `${mean.toFixed(0)} s`);
// The theme's turn is off until theme B is made (data/longform.txt).
const themeOff = /^theme = 0$/m.test(fs.readFileSync(new URL('../data/longform.txt', import.meta.url), 'utf8'));
check('every thing of the wander takes its turns', THINGS.every((t) => holds.things[t] > 0 || (t === 'theme' && themeOff)), JSON.stringify(holds.things));

// The wander as played: the corpus's first pieces at Endless and 1,024
// bars, 25 minutes each at the wander's amount 0, every move read from the
// notes and the mix; two of them at the app's amount for their joins and
// their loudness; then what a section costs.
const played = { pieces: 0, moves: 0, heard: 0, budget: 0, cycling: 0, joins: [], own: [], loud: [], costs: [], blocks: [] };
if (!flag('--plan')) {
  const PLAYED = Number(value('--played', 6));
  const lines = [];
  for (const seed of [7, ...seeds.slice(0, PLAYED - 1)]) {
    for (const length of ['bars=endless', 'bars=1024']) {
      const request = `${length},journey=wander`;
      const r = playedChecks(seed, request, 1500);
      played.pieces += 1;
      played.moves += r.moves.length;
      played.heard += r.heard.length;
      played.budget += r.budget.length;
      played.cycling += r.cycling.length;
      for (const name of ['at most one thing moves a section', "every move heard is one of the wander's things", 'nothing moves where the map draws nothing', "a move that can't glide starts on its line", 'every move the map draws is heard']) {
        const bad = r.problems.filter((p) => p.check === name);
        check(`played: ${name}`, bad.length === 0, bad[0]?.where);
      }
      if (length === 'bars=endless') lines.push({ seed, request, moves: r.moves });
    }
  }
  // Twenty lines at least, from as many pieces as that takes (two at least,
  // each over twenty minutes for its loudness).
  for (const { seed, request, moves } of lines) {
    if (played.joins.length >= 20 && played.loud.length >= 2) break;
    const r = soundChecks(seed, request, 1500, moves);
    const own = loopLoudness(seed, 1500, r.stretch);
    played.joins.push(...r.joins);
    played.own.push(r.own);
    played.loud.push({ seed, jump: r.jump, range: r.range, ownJump: own.jump });
    check("played: no click at a line (each join under the piece's own largest step)", r.joins.every((j) => j <= r.own), `${seed}: ${r.joins.map((j) => j.toFixed(4))} against ${r.own.toFixed(4)}`);
    check("played: no loudness jump past 2 LU (or the loop's own) between stretches", r.jump <= Math.max(2, own.jump + 0.25), `${seed}: ${r.jump.toFixed(2)} LU, the loop's own ${own.jump.toFixed(2)}`);
    check("played: twenty minutes' loudness within the catalog's spread", r.range <= r.spread, `${seed}: ${r.range.toFixed(2)} LU against ${r.spread.toFixed(2)}`);
  }
  // A section's making, timed natively (dlgen --section-cost): the same at
  // section 1, 10^4 and 10^9.
  const core = path.join(path.dirname(fileURLToPath(import.meta.url)), '..', 'core');
  execFileSync('cargo', ['build', '--release', '--locked', '--example', 'dlgen'], { cwd: core, stdio: 'ignore' });
  for (const seed of [7, ...seeds.slice(0, 3)]) {
    const line = execFileSync(path.join(core, 'target', 'release', 'examples', 'dlgen'), ['--section-cost', String(seed), 'bars=endless,journey=wander'], { encoding: 'utf8' });
    const us = [...line.matchAll(/=([\d.]+)us/g)].map((m) => Number(m[1]));
    played.costs.push(us);
    check("played: a section's making costs the same at 10^9 as at 1 and 10^4", us[2] <= 2 * Math.max(us[0], us[1]) + 2, `${seed}: ${us.join(', ')} us`);
  }
  // The worst block at a section line against the loop's own worst audio
  // block (dlrender --costs: a pass, its plan heard, is drawn in a block of
  // its own a section ahead, and a line's steps are decided in another),
  // full and lite.
  for (const seed of [7, ...seeds.slice(0, 3)]) {
    for (const lite of [false, true]) {
      const out = dlrender(['--seed', String(seed), '--request', 'bars=endless,journey=wander', '--seconds', '300', '--costs', ...(lite ? ['--lite'] : [])]);
      const m = out.match(/pass (\d+), steps (\d+), audio (\d+)/);
      if (!m) continue;
      const [pass, steps, audio] = m.slice(1).map(Number);
      played.blocks.push({ seed, lite, pass, steps, audio });
      check("played: no section line's block over the loop's own worst audio block", pass <= audio && steps <= audio, `${seed}${lite ? ' lite' : ''}: pass ${pass} us, steps ${steps} us, audio ${audio} us`);
    }
  }
}

// The arc as played (session 13): each piece rendered whole at the wander's
// amount 0 (the plan with no breathing, so what is measured is the arc's
// own shape: each unit against the loop as itself on the same bars), the
// first of them at the app's amount too for what is heard.
const arcs = { pieces: 0, verdicts: [], heard: [], joins: [], own: [], costs: [], blocks: [] };
if (!flag('--plan')) {
  const n = Number(value('--arcs', 30));
  const sets = [['bars=64', n], ['bars=256', Math.round(0.4 * n)], ['bars=1024', Math.max(1, Math.round(0.4 * 0.4 * n))]];
  for (const [length, count] of sets) {
    for (const seed of seeds.slice(0, count)) {
      const request = `${length},journey=arc`;
      const v = arcVerdict(arcChecks(seed, request, { wander: 0 }));
      arcs.pieces += 1;
      arcs.verdicts.push({ seed, length, ...v });
      const where = `${seed} "${request}"`;
      check('arc played: the peak no more than 4 LU over the opening', v.over <= 4, `${where}: ${v.over.toFixed(2)} LU`);
      check("arc played: no step past 2 LU between neighboring units of the loop's own notes, 3 LU between any (the arc's own)", v.jumps, `${where}: ${v.stepOwn.toFixed(2)} LU, ${v.step.toFixed(2)} LU`);
      check("arc played: a finite arc's last chord is the tonic's, alone", v.tonic, `${where}: ${v.last}`);
      check('arc played: it ends in a ring-out, not a cut', v.rings, where);
    }
  }
  const share = (key) => arcs.verdicts.filter((v) => v[key]).length / arcs.verdicts.length;
  check('arc played: the loudness rises to the peak and falls, in order (9 in 10)', share('loudRises') >= 0.9, `${(100 * share('loudRises')).toFixed(0)}%: ${arcs.verdicts.filter((v) => !v.loudRises).map((v) => v.seed).join(', ')}`);
  check('arc played: the brightness rises to the peak and falls, in order (9 in 10)', share('brightRises') >= 0.9, `${(100 * share('brightRises')).toFixed(0)}%: ${arcs.verdicts.filter((v) => !v.brightRises).map((v) => v.seed).join(', ')}`);
  // As heard at the app's amount, the loop's own form and breathing in it:
  // reported, not held.
  for (const seed of seeds.slice(0, Math.min(10, n))) {
    arcs.heard.push(arcVerdict(arcChecks(seed, 'bars=64,journey=arc', { wander: 1 })));
  }
  // The joins: every unit's line of two Endless arcs, twenty minutes each,
  // against the loop as itself in the same window (`arcJoins`: an arc has a
  // line on every unit's downbeat, so the piece elsewhere is no measure).
  for (const seed of [7, seeds[1]]) {
    const joins = arcJoins(seed, 'bars=endless,journey=arc', 1200);
    const worst = joins.reduce((w, j) => (j.piece / j.loop > w.piece / w.loop ? j : w), joins[0]);
    arcs.joins.push(...joins.map((j) => j.piece));
    arcs.own.push(Math.max(...joins.map((j) => j.loop)));
    arcs.ratios = [...(arcs.ratios ?? []), ...joins.map((j) => j.piece / j.loop)];
    check("arc played: no click at a line (each join within the loop's own step there, +25%)", joins.every((j) => j.piece <= 1.25 * j.loop + 0.002), `${seed} at ${worst.t.toFixed(1)} s: ${worst.piece.toFixed(4)} against ${worst.loop.toFixed(4)}`);
  }
  // A section's making and its block, as the wander's.
  const core = path.join(path.dirname(fileURLToPath(import.meta.url)), '..', 'core');
  execFileSync('cargo', ['build', '--release', '--locked', '--example', 'dlgen'], { cwd: core, stdio: 'ignore' });
  for (const seed of [7, ...seeds.slice(0, 3)]) {
    const line = execFileSync(path.join(core, 'target', 'release', 'examples', 'dlgen'), ['--section-cost', String(seed), 'bars=endless,journey=arc'], { encoding: 'utf8' });
    const us = [...line.matchAll(/=([\d.]+)us/g)].map((m) => Number(m[1]));
    arcs.costs.push(us);
    check("arc played: a section's making costs the same at 10^9 as at 1 and 10^4", us[2] <= 2 * Math.max(us[0], us[1]) + 2, `${seed}: ${us.join(', ')} us`);
    for (const lite of [false, true]) {
      for (const request of ['bars=endless,journey=arc', 'bars=256,journey=arc']) {
        const out = dlrender(['--seed', String(seed), '--request', request, '--seconds', '600', '--costs', ...(lite ? ['--lite'] : [])]);
        const m = out.match(/pass (\d+), steps (\d+), audio (\d+)/);
        if (!m) continue;
        const [pass, steps, audio] = m.slice(1).map(Number);
        arcs.blocks.push({ seed, lite, request, pass, steps, audio });
        check("arc played: no section line's block over the loop's own worst audio block", pass <= audio && steps <= audio, `${seed} ${request}${lite ? ' lite' : ''}: pass ${pass} us, steps ${steps} us, audio ${audio} us`);
      }
    }
  }
}

console.log(`longform: ${SEEDS} seeds x ${LENGTHS.length} lengths x 2 journeys = ${pieces} pieces, in ${((Date.now() - t0) / 1000).toFixed(0)} s`);
const drawn = journeys.arc + journeys.wander;
console.log(`  drawn with nothing asked: arc ${(100 * journeys.arc / drawn).toFixed(0)}%, wander ${(100 * journeys.wander / drawn).toFixed(0)}%; planned in sections ${grains.section}, in phrases ${grains.phrase}`);
console.log(`  a wander's holds: ${holds.count}, ${mean.toFixed(0)} s on average; moves: ${THINGS.map((t) => `${t} ${holds.things[t] ?? 0}`).join(', ')}`);
for (const journey of JOURNEYS) {
  console.log(`  a section's plan, ${journey}: ${timed[journey].map((t, i) => `${(t * 1000).toFixed(1)} us at ${['1', '10^5', '10^9'][i]}`).join(', ')} (with its text)`);
}
if (played.pieces) {
  console.log(`  played: ${played.pieces} wander pieces of 25 minutes: ${played.moves} moves drawn, heard in ${played.heard} sections (a glide over several); ${played.budget} sections where a voice moved changed what the voice budget let another layer play; ${played.cycling} layers with cycles of their own read by their gain alone`);
  console.log(`  joins: the largest step at ${played.joins.length} lines ${Math.max(...played.joins).toFixed(4)}; the pieces' own largest ${played.own.map((x) => x.toFixed(4)).join(', ')}`);
  for (const l of played.loud) console.log(`  loudness, ${l.seed}: the largest jump between 20 s stretches ${l.jump.toFixed(2)} LU (the loop as itself ${l.ownJump.toFixed(2)}), the range ${l.range.toFixed(2)} LU`);
  console.log(`  a section's making: ${played.costs.map((c) => c.join('/')).join(', ')} us at 1/10^4/10^9`);
  console.log(`  blocks: ${played.blocks.map((b) => `${b.seed}${b.lite ? ' lite' : ''} pass ${b.pass}, steps ${b.steps}, audio ${b.audio} us`).join('; ')}`);
}
if (arcs.pieces) {
  const v = arcs.verdicts;
  const pct = (f) => `${(100 * v.filter(f).length / v.length).toFixed(0)}%`;
  const med = (xs) => [...xs].sort((a, b) => a - b)[Math.floor(xs.length / 2)];
  console.log(`  arcs played: ${arcs.pieces} pieces (${['bars=64', 'bars=256', 'bars=1024'].map((l) => `${v.filter((x) => x.length === l).length} at ${l.slice(5)} bars`).join(', ')}), against the loop as itself:`);
  console.log(`    loudness rises and falls ${pct((x) => x.loudRises)}, brightness ${pct((x) => x.brightRises)}, both ${pct((x) => x.loudRises && x.brightRises)}; the peak over the opening ${Math.max(...v.map((x) => x.over)).toFixed(2)} LU at most (median ${med(v.map((x) => x.over)).toFixed(2)}); the largest step ${Math.max(...v.map((x) => x.step)).toFixed(2)} LU`);
  console.log(`    the opening / the peak / the close, median: ${['opening', 'peak', 'close'].map((r) => `${med(v.map((x) => x.lift[r])).toFixed(1)} LU ${med(v.map((x) => x.brighter[r])).toFixed(0)} cents`).join(' / ')} (loudness, brightness: the centroid over 250 Hz)`);
  console.log(`    the tonic's chord alone at the end ${pct((x) => x.tonic)}, a ring-out ${pct((x) => x.rings)}`);
  const h = arcs.heard;
  if (h.length) console.log(`  as heard at the app's amount (${h.length} at 64 bars, the loop's own form in it): loudness rises and falls ${(100 * h.filter((x) => x.loudHeard).length / h.length).toFixed(0)}%, brightness ${(100 * h.filter((x) => x.brightHeard).length / h.length).toFixed(0)}%; the peak over the opening ${Math.max(...h.map((x) => x.overHeard)).toFixed(2)} LU at most`);
  const ratios = [...(arcs.ratios ?? [])].sort((a, b) => a - b);
  console.log(`  arc joins: ${arcs.joins.length} lines, the largest step ${Math.max(...arcs.joins).toFixed(4)} (the loop as itself in the same windows ${arcs.own.map((x) => x.toFixed(4)).join(', ')}); a line's step against the loop's there: median ${ratios[Math.floor(ratios.length / 2)]?.toFixed(2)}, at most ${ratios.at(-1)?.toFixed(2)}`);
  console.log(`  an arc's section's making: ${arcs.costs.map((c) => c.join('/')).join(', ')} us at 1/10^4/10^9`);
  console.log(`  arc blocks: ${arcs.blocks.map((b) => `${b.seed}${b.lite ? ' lite' : ''} ${b.request.slice(5, b.request.indexOf(','))} pass ${b.pass}, steps ${b.steps}, audio ${b.audio} us`).join('; ')}`);
}
console.log('');
let failed = 0;
for (const c of checks.values()) {
  failed += c.failed ? 1 : 0;
  console.log(`  ${c.failed ? 'FAIL' : 'ok  '}  ${c.name.padEnd(72)} ${String(c.seen - c.failed).padStart(6)} of ${c.seen}${c.failed ? `   first: ${c.first}` : ''}`);
}
console.log('');
console.log(failed ? `longform: ${failed} checks failed` : 'longform: every check passed');
if (failed && flag('--check')) process.exit(1);
