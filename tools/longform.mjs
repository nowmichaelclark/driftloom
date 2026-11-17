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
//   node tools/longform.mjs --endings 20    that many pieces stopped by the
//                                           length slider (20)
//   node tools/longform.mjs --plan          the plan's checks alone
//   node tools/longform.mjs --check         exit 1 if any check fails (with
//                                           --against: any check main kept)
//   node tools/longform.mjs --map SEED REQUEST [--from N] [--count N]
//                                           a piece's map as JSON lines
//                                           (dlgen --map prints it for people)
//
// In shards (the `longform` workflow, .github/workflows/longform.yml): the
// run is a list of pieces of work (a seed's plan, a piece played, a block
// timed), each shard plays its share and writes what it found, and the
// merge makes the same table one run makes, the checks in the same order.
//
//   node tools/longform.mjs --shard 3/8 --out s3.json
//                                           the third of eight shares, to a
//                                           file (the same options on every
//                                           shard: they are checked)
//   node tools/longform.mjs --merge s1.json s2.json ...
//                                           the table from every shard
//   node tools/longform.mjs --table FILE    also write the table as JSON
//                                           (the nightly keeps main's)
//   node tools/longform.mjs --against FILE  compare with a table so kept:
//                                           "kept every check main kept",
//                                           or what it lost
//   node tools/longform.mjs --times         the slowest pieces of work, to
//                                           weigh the shares by
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
import { Rng } from '../js/rng.js';

const args = process.argv.slice(2);
const flag = (name) => args.includes(name);
const value = (name, fallback) => (args.includes(name) ? args[args.indexOf(name) + 1] : fallback);

if (flag('--map')) {
  const { pieceMap } = await import('./gen-lib.mjs');
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

// What a shard writes and the merge reads: numbers that JSON can't hold
// (an empty list's largest, a ratio over zero) kept as text.
const toJson = (x) => JSON.stringify(x, (_, v) => (typeof v === 'number' && !Number.isFinite(v) ? { $num: String(v) } : v));
const fromJson = (text) => JSON.parse(text, (_, v) => (v && typeof v === 'object' && typeof v.$num === 'string' && Object.keys(v).length === 1 ? Number(v.$num) : v));

const MERGE = (() => {
  if (!flag('--merge')) return null;
  const rest = args.slice(args.indexOf('--merge') + 1);
  const end = rest.findIndex((a) => a.startsWith('--'));
  return end < 0 ? rest : rest.slice(0, end);
})();
const shardFiles = MERGE?.map((f) => fromJson(fs.readFileSync(f, 'utf8')));

// The options every shard of a run shares (a merge takes them from its
// shards).
const OPTIONS = shardFiles ? shardFiles[0].options : {
  seeds: Number(value('--seeds', 240)),
  plan: flag('--plan'),
  played: Number(value('--played', 6)),
  arcs: Number(value('--arcs', 30)),
  endings: Number(value('--endings', 20)),
};
const SEEDS = OPTIONS.seeds;
const LENGTHS = ['bars=64', 'bars=256', 'bars=1024', 'bars=32768', 'bars=endless'];
const JOURNEYS = ['arc', 'wander'];
const THINGS = ['theme', 'key', 'layers', 'voice', 'warmth', 'space', 'register', 'length', 'density'];
// How many sections of a piece are read: the whole of a short one, the
// start of a long one, and for Endless a stretch from far in as well.
const MOST = 1500;
const FAR = 1e9;

// Each check remembers where it was first made (the piece of work and the
// order within it) and where it first failed, so the shards' tables merge
// into the order and the first failures one run gives.
const checks = new Map();
let at = 0;
let seq = 0;
const check = (name, ok, where) => {
  let c = checks.get(name);
  if (!c) {
    c = { name, seen: 0, failed: 0, first: null, order: [at, seq], firstAt: null };
    checks.set(name, c);
  }
  seq += 1;
  c.seen += 1;
  if (!ok) {
    c.failed += 1;
    if (c.first == null && where != null) {
      c.first = where;
      c.firstAt = [at, seq];
    }
  }
};
const before = (a, b) => a[0] - b[0] || a[1] - b[1];

// The libraries that play (the tools' build, dlrender): loaded only to run
// work, so a merge needs neither.
let L = null;
let HOLE_DB = shardFiles?.[0].holeDb;
async function libs() {
  const gen = await import('./gen-lib.mjs');
  const played = await import('./longform-played.mjs');
  L = { ...gen, ...played };
  HOLE_DB = played.HOLE_DB;
}

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

function wanderChecks(where, form, units, fromStart, holds) {
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

// The corpus's seeds: the corpus seed's own stream.
const rng = new Rng(20261009);
const seeds = Array.from({ length: SEEDS }, () => Math.floor(rng.f() * 4294967296));
const CORE = path.join(path.dirname(fileURLToPath(import.meta.url)), '..', 'core');
const dlgen = (args) => {
  if (!dlgen.built) {
    execFileSync('cargo', ['build', '--release', '--locked', '--example', 'dlgen'], { cwd: CORE, stdio: 'ignore' });
    dlgen.built = true;
  }
  return execFileSync(path.join(CORE, 'target', 'release', 'examples', 'dlgen'), args, { encoding: 'utf8' });
};
// The holes session 05 found on Endless arcs (docs/notes/for-mikey.md,
// under dottedPulse), where each join began, in seconds as rendered.
const PLACES = [{ seed: 1337654778, name: 'ta-muin', at: [124.0, 564.5] }, { seed: 819045617, name: 'yi-yoar', at: [373.5] }];
const WANDERED = [7, ...seeds.slice(0, OPTIONS.played - 1)];
// A wander's moves as played, kept for the joins' sound checks (the same
// piece's in a shard that played it, else played again).
const moved = new Map();
const movesOf = (seed, request) => {
  if (!moved.has(`${seed} ${request}`)) moved.set(`${seed} ${request}`, L.playedChecks(seed, request, 1500).moves);
  return moved.get(`${seed} ${request}`);
};
const noDepths = ({ depths, ...v }) => v;

/**
 * The run as pieces of work, in the order one run makes them: `run` plays
 * (in a shard), `finish` reads what the pieces before it found (in the
 * merge). `cost` is about how many seconds a piece takes on CI's runner
 * (measured on PR #405's run: `--times` prints them), for the shares; `group` keeps pieces that share a render in one shard.
 */
function work() {
  const w = [];
  const add = (name, cost, step, group) => w.push({ name, cost, group, ...step });

  for (const seed of seeds) {
    add(`plan ${seed}`, 0.25, { run: () => {
      const r = { journeys: { arc: 0, wander: 0 }, grains: { section: 0, phrase: 0 }, pieces: 0, joins: {}, holds: { count: 0, seconds: 0, things: {} } };
      const { pieceMap } = L;
      for (const length of LENGTHS) {
        // The journey a piece draws when none is asked.
        r.journeys[pieceMap(seed, length, 0, 1).form.journey] += 1;
        for (const journey of JOURNEYS) {
          const request = `${length},journey=${journey}`;
          const where = `${seed} "${request}"`;
          const head = pieceMap(seed, request, 0, 1).form;
          const sections = head.sections;
          const count = sections == null ? MOST : Math.min(sections, MOST);
          const { form, units } = pieceMap(seed, request, 0, count);
          r.pieces += 1;
          r.grains[form.grain] += 1;
          check('the journey asked is the piece\'s', form.journey === journey && form.asked, where);

          // The joins (session 15): every line where a theme, a key or how
          // the tune is played changes (or a wander's line move) written, a
          // glide only for what glides; none after a close.
          for (let x = 1; x < units.length; x++) {
            const [a, b] = [units[x - 1], units[x]];
            r.joins[a.join] = (r.joins[a.join] ?? 0) + 1;
            if (a.closes) {
              check('a close lands: no join after it', a.join === 'none', where);
              continue;
            }
            if (b.changed.some((c) => ['theme', 'key', 'variation', 'voice', 'register', 'length'].includes(c))) {
              check('no line left unwritten where a theme, a key or a line move changes', !['none', 'glide'].includes(a.join), `${where} section ${a.section}.${a.phrase}: ${a.join} into ${b.changed.join(', ')}`);
            } else if (b.changed.length) {
              check('something changes at the line: a join or its glide', a.join !== 'none', `${where} section ${a.section}.${a.phrase}`);
            }
            if (b.changed.includes('key')) check('a key changes: a chord both keys share, the return prepared, the floor out or a breath', ['pivot', 'prepared', 'floor', 'breath'].includes(a.join), `${where} section ${a.section}.${a.phrase}: ${a.join}`);
          }
          const keys = [form.home, ...form.related];
          check('no key outside the related set', units.every((u) => keys.some((k) => sameKey(k, u.key))), where);
          check('the tempo is the loop\'s own throughout', units.every((u) => u.bpm === units[0].bpm), where);

          if (journey === 'arc') {
            arcPlanChecks(where, form, units);
            movementChecks(where, form, units, form.sections != null && units.at(-1).section === form.sections - 1);
          }
          else wanderChecks(where, form, units, true, r.holds);

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
            if (journey === 'wander') wanderChecks(where, form, far, false, r.holds);
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
      return r;
    } });
  }
  const plans = w.map((_, i) => i);

  // What was asked is never moved (a few seeds: the Rust test walks more).
  add('asked', 6, { run: () => {
    for (const seed of seeds.slice(0, 20)) {
      for (const journey of JOURNEYS) {
        const map = (ask) => L.pieceMap(seed, `bars=endless,journey=${journey},${ask}`, 0, 1200);
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
  } });

  // An arc lasts about as long in minutes at 50 bpm as at 150 (its spans are
  // set in time): the same seed asked both tempos.
  add('tempos', 0.1, { run: () => {
    for (const seed of seeds.slice(0, 40)) {
      const at = (bpm) => L.pieceMap(seed, `bars=endless,journey=arc,bpm=${bpm}`, 0, 1).form;
      const [slow, fast] = [at(50), at(150)];
      const minutes = (f) => f.arcUnits * f.unitSeconds / 60;
      const unit = Math.max(slow.unitSeconds, fast.unitSeconds) / 60;
      check('an arc lasts as long in minutes at 50 bpm as at 150', Math.abs(minutes(slow) - minutes(fast)) <= unit + 0.05 && minutes(slow) >= 3.5 && minutes(slow) <= 16, `${seed}: ${minutes(slow).toFixed(1)} and ${minutes(fast).toFixed(1)} minutes`);
    }
  } });

  // A section's cost, timed: the map of 200 sections from section 1, from
  // 10^5 and from 10^9, the least of several runs each.
  const timedAt = w.length;
  add('plan timed', 2.5, { run: () => {
    const timed = {};
    for (const journey of JOURNEYS) {
      const request = `bars=endless,journey=${journey}`;
      const time = (from) => {
        let best = Infinity;
        for (let run = 0; run < 7; run++) {
          const t = process.hrtime.bigint();
          for (let k = 0; k < 10; k++) L.pieceMap(seeds[5], request, from, 200);
          best = Math.min(best, Number(process.hrtime.bigint() - t) / 1e6 / 2000);
        }
        return best;
      };
      timed[journey] = [time(1), time(1e5), time(1e9)];
      const [, on, far] = timed[journey];
      check("a section's plan costs the same at 10^9 as at 10^5", far < on * 1.5 + 0.002, `${journey}: ${timed[journey].map((t) => (t * 1000).toFixed(1)).join(', ')} us`);
    }
    return timed;
  } });

  const planAt = w.length;
  add('plan, the whole corpus', 0, { finish: (R) => {
    const p = { journeys: { arc: 0, wander: 0 }, grains: { section: 0, phrase: 0 }, pieces: 0, joins: {}, holds: { count: 0, seconds: 0, things: {} }, timed: R[timedAt] };
    for (const i of plans) {
      const r = R[i];
      for (const k in r.journeys) p.journeys[k] += r.journeys[k];
      for (const k in r.grains) p.grains[k] += r.grains[k];
      p.pieces += r.pieces;
      for (const [k, n] of Object.entries(r.joins)) p.joins[k] = (p.joins[k] ?? 0) + n;
      p.holds.count += r.holds.count;
      p.holds.seconds += r.holds.seconds;
      for (const [k, n] of Object.entries(r.holds.things)) p.holds.things[k] = (p.holds.things[k] ?? 0) + n;
    }
    const JOINS = ['glide', 'fill', 'floor', 'pivot', 'alone', 'breath', 'prepared'];
    check('every join kind appears in the corpus', JOINS.every((k) => p.joins[k] > 0), JSON.stringify(p.joins));
    p.mean = p.holds.seconds / Math.max(1, p.holds.count);
    check('holds as long as drawn: between one and five minutes on average', p.mean >= 90 && p.mean <= 270, `${p.mean.toFixed(0)} s`);
    // The theme's turn is off until theme B is made (data/longform.txt).
    const themeOff = /^theme = 0$/m.test(fs.readFileSync(new URL('../data/longform.txt', import.meta.url), 'utf8'));
    check('every thing of the wander takes its turns', THINGS.every((t) => p.holds.things[t] > 0 || (t === 'theme' && themeOff)), JSON.stringify(p.holds.things));
    return p;
  } });

  const out = { planAt };
  if (OPTIONS.plan) return { w, out };

  // The wander as played: the corpus's first pieces at Endless and 1,024
  // bars, 25 minutes each at the wander's amount 0, every move read from the
  // notes and the mix; two of them at the app's amount for their joins and
  // their loudness; then what a section costs.
  out.wandered = [];
  for (const seed of WANDERED) {
    for (const length of ['bars=endless', 'bars=1024']) {
      const request = `${length},journey=wander`;
      out.wandered.push(w.length);
      // The first two Endless pieces share their render with the sound
      // checks.
      const group = length === 'bars=endless' && WANDERED.indexOf(seed) < 2 ? 'wander sound' : undefined;
      add(`played ${seed} ${request}`, 40, { run: () => {
        const r = L.playedChecks(seed, request, 1500);
        if (length === 'bars=endless') moved.set(`${seed} ${request}`, r.moves);
        for (const name of ['at most one thing moves a section', "every move heard is one of the wander's things", 'nothing moves where the map draws nothing', "a move that can't glide starts on its line", 'every move the map draws is heard']) {
          const bad = r.problems.filter((p) => p.check === name);
          check(`played: ${name}`, bad.length === 0, bad[0]?.where);
        }
        return { moves: r.moves.length, heard: r.heard.length, budget: r.budget.length, cycling: r.cycling.length };
      } }, group);
    }
  }
  // Twenty lines at least, from as many pieces as that takes (two at least,
  // each over twenty minutes for its loudness).
  out.sound = w.length;
  add('played, the joins and the loudness', 165, { run: () => {
    const s = { joins: [], own: [], loud: [] };
    for (const seed of WANDERED) {
      if (s.joins.length >= 20 && s.loud.length >= 2) break;
      const request = 'bars=endless,journey=wander';
      const r = L.soundChecks(seed, request, 1500, movesOf(seed, request));
      const own = L.loopLoudness(seed, 1500, r.stretch);
      s.joins.push(...r.joins);
      s.own.push(r.own);
      s.loud.push({ seed, jump: r.jump, range: r.range, ownJump: own.jump });
      check("played: no click at a line (each join under the piece's own largest step)", r.joins.every((j) => j <= r.own), `${seed}: ${r.joins.map((j) => j.toFixed(4))} against ${r.own.toFixed(4)}`);
      check("played: no loudness jump past 2 LU (or the loop's own) between stretches", r.jump <= Math.max(2, own.jump + 0.25), `${seed}: ${r.jump.toFixed(2)} LU, the loop's own ${own.jump.toFixed(2)}`);
      check("played: twenty minutes' loudness within the catalog's spread", r.range <= r.spread, `${seed}: ${r.range.toFixed(2)} LU against ${r.spread.toFixed(2)}`);
    }
    return s;
  } }, 'wander sound');
  // A section's making, timed natively (dlgen --section-cost): the same at
  // section 1, 10^4 and 10^9.
  out.wanderCosts = w.length;
  add('played, a section\'s making', 20, { run: () => {
    const costs = [];
    for (const seed of [7, ...seeds.slice(0, 3)]) {
      const line = dlgen(['--section-cost', String(seed), 'bars=endless,journey=wander']);
      const us = [...line.matchAll(/=([\d.]+)us/g)].map((m) => Number(m[1]));
      costs.push(us);
      check("played: a section's making costs the same at 10^9 as at 1 and 1,000", us[2] <= 2 * Math.max(us[0], us[1]) + 2, `${seed}: ${us.join(', ')} us`);
    }
    return costs;
  } });
  // The worst block at a section line against the loop's own worst audio
  // block (dlrender --costs: a pass, its plan heard, is drawn in a block of
  // its own a section ahead, and a line's steps are decided in another),
  // full and lite.
  out.wanderBlocks = [];
  for (const seed of [7, ...seeds.slice(0, 3)]) {
    out.wanderBlocks.push(w.length);
    add(`played, blocks ${seed}`, 35, { run: () => {
      const blocks = [];
      for (const lite of [false, true]) {
        const text = L.dlrender(['--seed', String(seed), '--request', 'bars=endless,journey=wander', '--seconds', '300', '--costs', ...(lite ? ['--lite'] : [])]);
        const m = text.match(/pass (\d+), steps (\d+), audio (\d+)/);
        if (!m) continue;
        const [pass, steps, audio] = m.slice(1).map(Number);
        blocks.push({ seed, lite, pass, steps, audio });
        check("played: no section line's block over the loop's own worst audio block", pass <= audio && steps <= audio, `${seed}${lite ? ' lite' : ''}: pass ${pass} us, steps ${steps} us, audio ${audio} us`);
      }
      return blocks;
    } });
  }

  // The arc as played (session 13): each piece rendered whole at the wander's
  // amount 0 (the plan with no breathing, so what is measured is the arc's
  // own shape: each unit against the loop as itself on the same bars), the
  // first of them at the app's amount too for what is heard.
  const n = OPTIONS.arcs;
  const sets = [['bars=64', n, 9], ['bars=256', Math.round(0.4 * n), 35], ['bars=1024', Math.max(1, Math.round(0.4 * 0.4 * n)), 140]];
  out.arcs = [];
  for (const [length, count, cost] of sets) {
    for (const seed of seeds.slice(0, count)) {
      out.arcs.push(w.length);
      add(`arc ${seed} ${length}`, cost, { run: () => {
        const request = `${length},journey=arc`;
        const v = L.arcVerdict(L.arcChecks(seed, request, { wander: 0 }));
        const where = `${seed} "${request}"`;
        check('arc played: the peak no more than 4 LU over the opening', v.over <= 4, `${where}: ${v.over.toFixed(2)} LU`);
        check("arc played: no step past 2 LU between neighboring units of the loop's own notes, 3 LU between any (the arc's own)", v.jumps, `${where}: ${v.stepOwn.toFixed(2)} LU, ${v.step.toFixed(2)} LU`);
        check("arc played: a finite arc's last chord is the tonic's, alone", v.tonic, `${where}: ${v.last}`);
        check('arc played: it ends in a ring-out, not a cut', v.rings, where);
        return { verdict: { seed, length, ...noDepths(v) }, depths: v.depths.map((d) => ({ ...d, where })) };
      } });
    }
  }
  add('arcs played, the whole set', 0, { finish: (R) => {
    const verdicts = out.arcs.map((i) => R[i].verdict);
    const share = (key) => verdicts.filter((v) => v[key]).length / verdicts.length;
    check('arc played: the loudness rises to the peak and falls, in order (9 in 10)', share('loudRises') >= 0.9, `${(100 * share('loudRises')).toFixed(0)}%: ${verdicts.filter((v) => !v.loudRises).map((v) => v.seed).join(', ')}`);
    // The brightness judged on the arc's own job (session 15): the opening and
    // the close darker than the peak. The order over every role is reported.
    check('arc played: darker at the opening and the close than at the peak (9 in 10)', share('brightEnds') >= 0.9, `${(100 * share('brightEnds')).toFixed(0)}%: ${verdicts.filter((v) => !v.brightEnds).map((v) => `${v.seed}@${v.length.slice(5)}`).join(', ')}`);
  } });
  // As heard at the app's amount, the loop's own form and breathing in it:
  // reported, not held.
  out.heard = [];
  for (const seed of seeds.slice(0, Math.min(10, n))) {
    out.heard.push(w.length);
    add(`arc heard ${seed}`, 9, { run: () => noDepths(L.arcVerdict(L.arcChecks(seed, 'bars=64,journey=arc', { wander: 1 }))) });
  }
  // The joins: every unit's line of two Endless arcs, twenty minutes each,
  // against the loop as itself in the same window (`arcJoins`: an arc has a
  // line on every unit's downbeat, so the piece elsewhere is no measure).
  out.arcJoins = [];
  for (const seed of [7, seeds[1]]) {
    out.arcJoins.push(w.length);
    add(`arc joins ${seed}`, 50, { run: () => {
      const joins = L.arcJoins(seed, 'bars=endless,journey=arc', 1200);
      const worst = joins.reduce((w, j) => (j.piece / j.loop > w.piece / w.loop ? j : w), joins[0]);
      check("arc played: no click at a line (each join within the loop's own step there, +25%)", joins.every((j) => j.piece <= 1.25 * j.loop + 0.002), `${seed} at ${worst.t.toFixed(1)} s: ${worst.piece.toFixed(4)} against ${worst.loop.toFixed(4)}`);
      return {
        joins: joins.map((j) => j.piece),
        own: Math.max(...joins.map((j) => j.loop)),
        ratios: joins.map((j) => j.piece / j.loop),
        depths: joins.depths.map((d) => ({ ...d, where: `${seed} "bars=endless,journey=arc"` })),
      };
    } });
  }
  // The places session 05 found (session 16b): three joins of two Endless
  // arcs the corpus run never played, each held to HOLE_DB under the loop
  // there for half a beat at most (the wide run is the nightly's,
  // tools/longform-holes.mjs).
  out.places = [];
  for (const { seed, name, at } of PLACES) {
    out.places.push(w.length);
    add(`places ${name}`, 22, { run: () => {
      const request = 'bars=endless,journey=arc';
      const joins = L.arcJoins(seed, request, Math.ceil(Math.max(...at) + 20));
      const places = [];
      for (const t of at) {
        const d = joins.depths.find((j) => Math.abs(j.t - t) < 1.5);
        places.push({ name, seed, t, d });
        check(`arc played: the places session 05 found ${HOLE_DB} dB under the loop for half a beat at most`, !d || d.under <= 0.5, `${name} (${seed}) at ${t} s: a ${d?.join} ${d?.under.toFixed(2)} beats ${HOLE_DB} dB under`);
      }
      return { places, depths: joins.depths.map((d) => ({ ...d, where: `${seed} "${request}"` })) };
    } });
  }
  // No join a hole (session 16): every join of every arc played, as played,
  // against the loop as itself in the same window (`joinDepths`).
  out.depthsAt = w.length;
  add('arc joins, the whole set', 0, { finish: (R) => {
    const depths = [...out.arcs, ...out.arcJoins, ...out.places].flatMap((i) => R[i].depths);
    const holes = depths.filter((d) => d.hole);
    check(`arc played: no join a hole (none ${HOLE_DB} dB under the loop there, made by the join)`, holes.length === 0, holes.map((d) => `${d.where} at ${d.t.toFixed(1)} s: a ${d.join} ${d.frames.toFixed(1)} dB under, from ${d.before.toFixed(1)}`).join('; '));
    return depths;
  } });
  // A section's making and its block, as the wander's.
  out.arcCosts = [];
  for (const seed of [7, ...seeds.slice(0, 3)]) {
    out.arcCosts.push(w.length);
    add(`arc costs ${seed}`, 125, { run: () => {
      const line = dlgen(['--section-cost', String(seed), 'bars=endless,journey=arc']);
      const us = [...line.matchAll(/=([\d.]+)us/g)].map((m) => Number(m[1]));
      check("arc played: a section's making costs the same at 10^9 as at 1 and 1,000", us[2] <= 2 * Math.max(us[0], us[1]) + 2, `${seed}: ${us.join(', ')} us`);
      const blocks = [];
      for (const lite of [false, true]) {
        for (const request of ['bars=endless,journey=arc', 'bars=256,journey=arc']) {
          const text = L.dlrender(['--seed', String(seed), '--request', request, '--seconds', '600', '--costs', ...(lite ? ['--lite'] : [])]);
          const m = text.match(/pass (\d+), steps (\d+), audio (\d+)/);
          if (!m) continue;
          const [pass, steps, audio] = m.slice(1).map(Number);
          blocks.push({ seed, lite, request, pass, steps, audio });
          check("arc played: no section line's block over the loop's own worst audio block", pass <= audio && steps <= audio, `${seed} ${request}${lite ? ' lite' : ''}: pass ${pass} us, steps ${steps} us, audio ${audio} us`);
        }
      }
      return { costs: us, blocks };
    } });
  }

  // Endings that land (session 15): twenty pieces the length slider stops
  // before their close (Endless arcs and wanders, and finite arcs part way),
  // each with the core's own next loop after it, as in the app: the last
  // chord struck the tonic's, nothing after it but its ring, never faded.
  out.endings = [];
  const asks = ['bars=endless,journey=arc', 'bars=endless,journey=wander', 'bars=1024,journey=arc', 'bars=256,journey=arc'];
  for (let i = 0; i < OPTIONS.endings; i++) {
    const seed = seeds[i];
    const request = asks[i % asks.length];
    out.endings.push(w.length);
    add(`ending ${seed} ${request}`, 4, { run: () => {
      // A stop somewhere in the second to the fifth minute, by the seed.
      const { form } = L.pieceMap(seed, request, 0, 1);
      const bar = form.unitSeconds / form.unitBars;
      const bars = Math.max(9, Math.round((60 + (seed % 180)) / bar));
      const e = L.endingChecks(seed, request, bars);
      const where = `${seed} "${request}" stopped at bar ${e.stopBars}`;
      check('a slider-made close lands on the tonic', e.tonic, `${where}: ${e.chord.join('/')}`);
      check('a slider-made close plays alone, then rings out (never faded or cut)', e.alone && e.rings, `${where}: ${e.later} notes after it, ${e.faded} fades, a fall of ${e.fall.toFixed(1)} dB`);
      return e;
    } });
  }
  return { w, out };
}

/** The shares: the pieces of work by group, the costliest first, each to the shard with the least so far. */
function shares(w, count) {
  const groups = new Map();
  w.forEach((x, i) => {
    if (x.finish) return;
    const key = x.group ?? `#${i}`;
    const g = groups.get(key) ?? { cost: 0, items: [], first: i };
    g.cost += x.cost;
    g.items.push(i);
    groups.set(key, g);
  });
  const load = Array(count).fill(0);
  const of = new Map();
  for (const g of [...groups.values()].sort((a, b) => b.cost - a.cost || a.first - b.first)) {
    const s = load.indexOf(Math.min(...load));
    load[s] += g.cost;
    for (const i of g.items) of.set(i, s);
  }
  return of;
}

/** The table and what it found, from every piece's results (`R`, by index). */
function report(w, out, R, elapsed) {
  const p = R[out.planAt];
  console.log(`longform: ${SEEDS} seeds x ${LENGTHS.length} lengths x 2 journeys = ${p.pieces} pieces, in ${elapsed}`);
  console.log(`  joins in the maps: ${Object.entries(p.joins).sort((a, b) => b[1] - a[1]).map(([k, c]) => `${k} ${c}`).join(', ')}`);
  const endings = (out.endings ?? []).map((i) => R[i]);
  if (endings.length) console.log(`  endings that land: ${endings.filter((e) => e.tonic && e.alone && e.rings).length} of ${endings.length} on the tonic, alone, ringing out (the largest fall in a ring ${Math.max(...endings.map((e) => e.fall)).toFixed(1)} dB)`);
  const drawn = p.journeys.arc + p.journeys.wander;
  console.log(`  drawn with nothing asked: arc ${(100 * p.journeys.arc / drawn).toFixed(0)}%, wander ${(100 * p.journeys.wander / drawn).toFixed(0)}%; planned in sections ${p.grains.section}, in phrases ${p.grains.phrase}`);
  console.log(`  a wander's holds: ${p.holds.count}, ${p.mean.toFixed(0)} s on average; moves: ${THINGS.map((t) => `${t} ${p.holds.things[t] ?? 0}`).join(', ')}`);
  for (const journey of JOURNEYS) {
    console.log(`  a section's plan, ${journey}: ${p.timed[journey].map((t, i) => `${(t * 1000).toFixed(1)} us at ${['1', '10^5', '10^9'][i]}`).join(', ')} (with its text)`);
  }
  if (out.wandered?.length) {
    const pieces = out.wandered.map((i) => R[i]);
    const sum = (k) => pieces.reduce((s, x) => s + x[k], 0);
    const s = R[out.sound];
    const costs = R[out.wanderCosts];
    const blocks = out.wanderBlocks.flatMap((i) => R[i]);
    console.log(`  played: ${pieces.length} wander pieces of 25 minutes: ${sum('moves')} moves drawn, heard in ${sum('heard')} sections (a glide over several); ${sum('budget')} sections where a voice moved changed what the voice budget let another layer play; ${sum('cycling')} layers with cycles of their own read by their gain alone`);
    console.log(`  joins: the largest step at ${s.joins.length} lines ${Math.max(...s.joins).toFixed(4)}; the pieces' own largest ${s.own.map((x) => x.toFixed(4)).join(', ')}`);
    for (const l of s.loud) console.log(`  loudness, ${l.seed}: the largest jump between 20 s stretches ${l.jump.toFixed(2)} LU (the loop as itself ${l.ownJump.toFixed(2)}), the range ${l.range.toFixed(2)} LU`);
    console.log(`  a section's making: ${costs.map((c) => c.join('/')).join(', ')} us at 1/1,000/10^9`);
    console.log(`  blocks: ${blocks.map((b) => `${b.seed}${b.lite ? ' lite' : ''} pass ${b.pass}, steps ${b.steps}, audio ${b.audio} us`).join('; ')}`);
  }
  if (out.arcs?.length) {
    const v = out.arcs.map((i) => R[i].verdict);
    const pct = (f) => `${(100 * v.filter(f).length / v.length).toFixed(0)}%`;
    const med = (xs) => [...xs].sort((a, b) => a - b)[Math.floor(xs.length / 2)];
    console.log(`  arcs played: ${v.length} pieces (${['bars=64', 'bars=256', 'bars=1024'].map((l) => `${v.filter((x) => x.length === l).length} at ${l.slice(5)} bars`).join(', ')}), against the loop as itself:`);
    console.log(`    loudness rises and falls ${pct((x) => x.loudRises)}; brightness darker at the opening and the close than at the peak ${pct((x) => x.brightEnds)}, in order over every role ${pct((x) => x.brightRises)} (reported); the peak over the opening ${Math.max(...v.map((x) => x.over)).toFixed(2)} LU at most (median ${med(v.map((x) => x.over)).toFixed(2)}); the largest step ${Math.max(...v.map((x) => x.step)).toFixed(2)} LU`);
    console.log(`    the opening / the peak / the close, median: ${['opening', 'peak', 'close'].map((r) => `${med(v.map((x) => x.lift[r])).toFixed(1)} LU ${med(v.map((x) => x.brighter[r])).toFixed(0)} cents`).join(' / ')} (loudness, brightness: the centroid over 250 Hz)`);
    console.log(`    the tonic's chord alone at the end ${pct((x) => x.tonic)}, a ring-out ${pct((x) => x.rings)}`);
    const h = out.heard.map((i) => R[i]);
    {
      const judged = v.filter((x) => x.brightOwn != null);
      console.log(`    brightness on the units in the loop's own voice and register alone (reported): ${judged.filter((x) => x.brightOwn).length} of ${judged.length} judged rise and fall; missing: ${judged.filter((x) => !x.brightOwn).map((x) => `${x.seed}@${x.length.slice(5)}`).join(', ')}`);
    }
    if (h.length) console.log(`  as heard at the app's amount (${h.length} at 64 bars, the loop's own form in it): loudness rises and falls ${(100 * h.filter((x) => x.loudHeard).length / h.length).toFixed(0)}%, brightness ${(100 * h.filter((x) => x.brightHeard).length / h.length).toFixed(0)}%; the peak over the opening ${Math.max(...h.map((x) => x.overHeard)).toFixed(2)} LU at most`);
    const aj = out.arcJoins.map((i) => R[i]);
    const joins = aj.flatMap((j) => j.joins);
    const ratios = aj.flatMap((j) => j.ratios).sort((a, b) => a - b);
    console.log(`  arc joins: ${joins.length} lines, the largest step ${Math.max(...joins).toFixed(4)} (the loop as itself in the same windows ${aj.map((j) => j.own.toFixed(4)).join(', ')}); a line's step against the loop's there: median ${ratios[Math.floor(ratios.length / 2)]?.toFixed(2)}, at most ${ratios.at(-1)?.toFixed(2)}`);
    {
      const depths = R[out.depthsAt];
      const kinds = [...new Set(depths.map((d) => d.join))].sort();
      console.log(`  arc joins against the loop as itself (session 16), dB under it, median / deepest: ${kinds.map((k) => { const xs = depths.filter((d) => d.join === k).map((d) => d.frames); return `${k} ${med(xs).toFixed(1)} / ${Math.max(...xs).toFixed(1)} (${xs.length})`; }).join(', ')}`);
      const quiet = depths.filter((d) => d.inherited);
      console.log(`    ${HOLE_DB} dB under, as the beat before already was (the unit's quiet, not the join's; reported): ${quiet.length ? quiet.map((d) => `${d.where} at ${d.t.toFixed(1)} s (a ${d.join}, ${d.frames.toFixed(1)} from ${d.before.toFixed(1)})`).join('; ') : 'none'}`);
    }
    for (const p of out.places.flatMap((i) => R[i].places)) console.log(`  ${p.name} (${p.seed}) at ${p.t} s: ${p.d ? `a ${p.d.join} at ${p.d.t.toFixed(2)} s, ${p.d.frames.toFixed(1)} dB under the loop (median frame), ${p.d.under.toFixed(2)} beats ${HOLE_DB} dB under` : 'no join written there (a glide)'}`);
    const ac = out.arcCosts.map((i) => R[i]);
    console.log(`  an arc's section's making: ${ac.map((c) => c.costs.join('/')).join(', ')} us at 1/1,000/10^9`);
    console.log(`  arc blocks: ${ac.flatMap((c) => c.blocks).map((b) => `${b.seed}${b.lite ? ' lite' : ''} ${b.request.slice(5, b.request.indexOf(','))} pass ${b.pass}, steps ${b.steps}, audio ${b.audio} us`).join('; ')}`);
  }
  console.log('');
  const table = [...checks.values()].sort((a, b) => before(a.order, b.order));
  let failed = 0;
  for (const c of table) {
    failed += c.failed ? 1 : 0;
    console.log(`  ${c.failed ? 'FAIL' : 'ok  '}  ${c.name.padEnd(72)} ${String(c.seen - c.failed).padStart(6)} of ${c.seen}${c.failed ? `   first: ${c.first}` : ''}`);
  }
  console.log('');
  console.log(failed ? `longform: ${failed} checks failed` : 'longform: every check passed');
  return { table, failed };
}

/** The table against one kept (main's): every check it kept, kept here. */
function against(table, kept) {
  const here = new Map(table.map((c) => [c.name, c]));
  const lost = [];
  const notRun = [];
  const still = [];
  for (const c of kept.checks) {
    const h = here.get(c.name);
    if (!h) {
      if (!c.failed) notRun.push(c);
    } else if (!c.failed && h.failed) lost.push(h);
    else if (c.failed && h.failed) still.push([c, h]);
  }
  const fixed = kept.checks.filter((c) => c.failed && here.get(c.name) && !here.get(c.name).failed);
  const fresh = table.filter((c) => !kept.checks.some((k) => k.name === c.name));
  const from = kept.commit ? `main at ${kept.commit.slice(0, 7)}${kept.date ? `, ${kept.date}` : ''}` : 'the table kept';
  console.log('');
  if (JSON.stringify(kept.options) !== JSON.stringify(OPTIONS)) console.log(`longform: ${from} was run with other options (${JSON.stringify(kept.options)}); the comparison is loose`);
  for (const c of lost) console.log(`  LOST  ${c.name}: ${c.failed} of ${c.seen} fail here, none on ${from}   first: ${c.first}`);
  for (const c of notRun) console.log(`  GONE  ${c.name}: kept on ${from}, not run here`);
  for (const [c, h] of still) console.log(`  also on main  ${c.name}: ${h.failed} of ${h.seen} fail here, ${c.failed} of ${c.seen} on ${from}`);
  for (const c of fixed) console.log(`  fixed  ${c.name}: ${c.failed} of ${c.seen} failed on ${from}, none here`);
  for (const c of fresh) console.log(`  new  ${c.name}: ${c.failed ? `${c.failed} of ${c.seen} fail` : `${c.seen} of ${c.seen}`}`);
  console.log(lost.length ? `longform: ${lost.length} checks ${from} kept fail here` : `longform: kept every check ${from} kept${notRun.length ? ` (${notRun.length} not run here)` : ''}`);
  return lost.length;
}

const { w, out } = work();
const R = [];
const t0 = Date.now();
const SHARD = value('--shard', null);
let elapsed;

if (SHARD) {
  // A shard: its share of the work, to a file.
  const [shard, count] = SHARD.split('/').map(Number);
  if (!(shard >= 1 && shard <= count)) throw new Error(`longform: --shard ${SHARD}: a shard is i/n, 1 <= i <= n`);
  const of = shares(w, count);
  await libs();
  const items = {};
  for (let i = 0; i < w.length; i++) {
    if (of.get(i) !== shard - 1) continue;
    at = i;
    seq = 0;
    const t = Date.now();
    const data = w[i].run();
    items[i] = { data: data ?? null, ms: Date.now() - t };
    if (!flag('--quiet')) console.error(`longform ${SHARD}: ${w[i].name}, ${((Date.now() - t) / 1000).toFixed(1)} s`);
  }
  const file = value('--out', `longform-${shard}-of-${count}.json`);
  fs.writeFileSync(file, toJson({ options: OPTIONS, holeDb: HOLE_DB, shard, count, ms: Date.now() - t0, items, checks: [...checks.values()] }));
  console.log(`longform: shard ${SHARD}, ${Object.keys(items).length} pieces of work in ${((Date.now() - t0) / 1000).toFixed(0)} s, to ${file}`);
  process.exit(0);
}

const ms = {};
if (shardFiles) {
  // The merge: every shard's results and checks, then the pieces that read
  // them all.
  const count = shardFiles[0].count;
  const got = shardFiles.map((s) => s.shard).sort((a, b) => a - b);
  if (got.join() !== Array.from({ length: count }, (_, i) => i + 1).join()) throw new Error(`longform: --merge needs shards 1 to ${count} once each, got ${got.join(', ')}`);
  for (const s of shardFiles) {
    if (JSON.stringify(s.options) !== JSON.stringify(OPTIONS) || s.count !== count) throw new Error(`longform: shard ${s.shard} was run with other options`);
    for (const [i, { data, ms: t }] of Object.entries(s.items)) {
      R[i] = data;
      ms[i] = t;
    }
    for (const c of s.checks) {
      const m = checks.get(c.name);
      if (!m) checks.set(c.name, { ...c });
      else {
        m.seen += c.seen;
        m.failed += c.failed;
        if (before(c.order, m.order) < 0) m.order = c.order;
        if (c.firstAt && (!m.firstAt || before(c.firstAt, m.firstAt) < 0)) {
          m.first = c.first;
          m.firstAt = c.firstAt;
        }
      }
    }
  }
  const missing = w.findIndex((x, i) => x.run && !(i in R));
  if (missing >= 0) throw new Error(`longform: no shard ran ${w[missing].name}`);
  for (let i = 0; i < w.length; i++) {
    if (!w[i].finish) continue;
    at = i;
    seq = 0;
    R[i] = w[i].finish(R);
  }
  const longest = Math.max(...shardFiles.map((s) => s.ms));
  elapsed = `${count} shards, the longest ${(longest / 1000).toFixed(0)} s`;
} else {
  // One run: every piece, in order.
  await libs();
  for (let i = 0; i < w.length; i++) {
    at = i;
    seq = 0;
    const t = Date.now();
    R[i] = w[i].run ? w[i].run() : w[i].finish(R);
    ms[i] = Date.now() - t;
  }
  elapsed = `${((Date.now() - t0) / 1000).toFixed(0)} s`;
}

const { table, failed } = report(w, out, R, elapsed);
if (flag('--times')) {
  const slow = Object.entries(ms).sort((a, b) => b[1] - a[1]).slice(0, 25);
  console.log('\nlongform: the slowest pieces of work (seconds, and the cost the shares weigh them by)');
  for (const [i, t] of slow) console.log(`  ${(t / 1000).toFixed(1).padStart(7)}  ${String(w[i].cost).padStart(4)}  ${w[i].name}`);
  const total = Object.values(ms).reduce((s, t) => s + t, 0);
  console.log(`  ${(total / 1000).toFixed(0)} s in all`);
}
const TABLE = value('--table', null);
if (TABLE) {
  // The commit the table is of (CI names a PR's head, not its merge commit).
  const commit = process.env.LONGFORM_COMMIT || (() => { try { return execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim(); } catch { return null; } })();
  fs.writeFileSync(TABLE, `${JSON.stringify({ commit, date: new Date().toISOString().slice(0, 10), options: OPTIONS, checks: table.map(({ name, seen, failed, first }) => ({ name, seen, failed, first })) }, null, 1)}\n`);
}
const AGAINST = value('--against', null);
if (AGAINST) {
  const lost = against(table, JSON.parse(fs.readFileSync(AGAINST, 'utf8')));
  if (lost && flag('--check')) process.exit(1);
} else if (failed && flag('--check')) process.exit(1);
