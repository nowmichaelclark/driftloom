// Track length's fades and ends, measured on renders (session 30c, queue
// item 111): the export's own render (js/export-render.js, what the app's
// export runs) on the app's module, at 48 kHz (its 'opus' samples, float).
//
//   node tools/track-fades.mjs            the fades and the ends, on corpus
//                                         loops, a piece and Endless
//   node tools/track-fades.mjs --long     also a track of 2:00:07 exported
//                                         whole: its length
//
// For each loop, a track of SECONDS seconds, rendered four ways (each fade on
// and off), the level read in 50 ms windows (RMS, dBFS, both lanes):
//
//   - a fade in rises from silence: its first 5 ms from the loop's first
//     step are under SILENT, and the level reaches the loop's own (within
//     3 dB of the same loop without the fade, a second after FADE_IN) by
//     FADE_IN, never above it on the way;
//   - a fade out reaches silence on the second: the last window before the
//     end is under SILENT, and the file after it holds nothing louder; the
//     level FADE_OUT before the end is the loop's own;
//   - without a fade out nothing is faded: every sample to the second is a
//     longer track's (to 0.1 s before it: the step on the second, whose
//     graces start ahead of it, is not played), and the end is where the core told it, to the sample,
//     what was sounding ringing out after it (the export's kept ending,
//     2026-11-17);
//   - without a fade in, the start is the same to the sample as a render
//     with none set.
//
// Exits 1 on any failure.

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { Render, OPUS_RATE, TAIL_MAX } from '../js/export-render.js';
import { decodeSong } from '../js/share.js';
import { askOf } from '../js/request.js';
import { LOOPS } from './stalls.mjs';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const MODULE = new WebAssembly.Module(fs.readFileSync(path.join(ROOT, 'js', 'dlplayer.wasm')));
const LONG = process.argv.includes('--long');
// The core's (core/src/player/track.rs).
const FADE_IN = 3;
const FADE_OUT = 8;
const START = 0.05;
const SECONDS = 45;
// Silence: under -60 dBFS RMS in a window.
const SILENT = -60;
const WINDOW = 0.05;
const RATE = OPUS_RATE;
let failures = 0;
const fail = (what) => {
  failures++;
  console.log(`  FAIL ${what}`);
};

const db = (x) => (x > 0 ? 20 * Math.log10(x) : -200);

// The whole render of `job`: its samples (mono, the lanes' mean square), the
// end the core told (seconds), and the file's length.
function render(job) {
  const x = new WebAssembly.Instance(MODULE, {}).exports;
  const r = new Render(x, { ...job, format: 'opus' });
  const sq = [];
  let c;
  while ((c = r.pcm())) {
    for (let i = 0; i < c.frames; i++) {
      let s = 0;
      for (const p of c.planes.slice(0, r.lanes)) s += p[i] * p[i];
      sq.push(s / r.lanes);
    }
  }
  return { sq, end: r.endTime, frames: r.frames, seconds: r.seconds };
}

// The level of the window from `t` seconds, dBFS.
function level(sq, t, w = WINDOW) {
  const a = Math.max(0, Math.round(t * RATE));
  const b = Math.min(sq.length, a + Math.round(w * RATE));
  if (b <= a) return -200;
  let s = 0;
  for (let i = a; i < b; i++) s += sq[i];
  return db(Math.sqrt(s / (b - a)));
}

// The loudest window between `a` and `b` seconds.
function loudest(sq, a, b) {
  let m = -200;
  for (let t = a; t + WINDOW <= b; t += WINDOW) m = Math.max(m, level(sq, t));
  return m;
}

const kind = (fin, fout) => 3 | (fin ? 4 : 0) | (fout ? 8 : 0);
const loop = (name) => decodeSong(LOOPS[name].code);

// A busy loop and a sung one, a long piece (the wander) and an Endless arc.
const cases = [
  ['a loop', loop('shatter')],
  ['a loop', loop('hollow')],
  ['a long piece', loop('piece')],
  ['Endless', loop('endless')],
];

const rows = [];
for (const [what, spec0] of cases) {
  const spec = { ...spec0 };
  const job = (fin, fout, secs = SECONDS) => ({ seed: spec.seed >>> 0, ask: askOf(spec), mutes: [], wander: 1, salt: 0, layout: 2, track: [kind(fin, fout), secs] });
  const plain = render(job(false, false));
  // The same loop to a longer time, nothing faded: what plays to the second.
  const longer = render(job(false, false, SECONDS + 15));
  const name = `${what} ${spec.name || spec.seed}`;
  const end = START + SECONDS;
  // The loop's own level: its loudest seconds from 5 s in to 10 s before
  // its end.
  const own = loudest(plain.sq, 5, end - 10);
  for (const [fin, fout] of [[false, false], [true, false], [false, true], [true, true]]) {
    const r = fin || fout ? render(job(fin, fout)) : plain;
    const tag = `${name}, fade in ${fin ? 'on' : 'off'}, fade out ${fout ? 'on' : 'off'}`;
    if (Math.abs(r.end - end) > 0.5 / RATE) fail(`${tag}: ended at ${r.end} for ${end}`);
    const first = level(r.sq, START, 0.005);
    const atIn = loudest(r.sq, START + FADE_IN, START + FADE_IN + 1);
    const plainIn = loudest(plain.sq, START + FADE_IN, START + FADE_IN + 1);
    const lastWin = level(r.sq, end - WINDOW);
    const after = loudest(r.sq, end, r.frames / RATE);
    const before = loudest(r.sq, end - FADE_OUT - 1, end - FADE_OUT);
    // Halfway through each fade, as heard.
    const midIn = level(r.sq, START + FADE_IN / 2, 0.25);
    const midOut = level(r.sq, end - FADE_OUT / 2, 0.25);
    if (fin) {
      if (!(first < SILENT)) fail(`${tag}: its first window at ${first.toFixed(1)} dB, not silence`);
      if (!(atIn > plainIn - 3)) fail(`${tag}: ${atIn.toFixed(1)} dB after its fade in against ${plainIn.toFixed(1)}`);
      // Rising: no window in the fade louder than where it arrives.
      if (loudest(r.sq, START, START + FADE_IN - 0.25) > atIn + 1) fail(`${tag}: louder in its fade in than after it`);
    } else {
      let same = true;
      for (let i = 0; i < Math.round((START + 1) * RATE); i++) if (r.sq[i] !== plain.sq[i]) { same = false; break; }
      if (!same) fail(`${tag}: its start moved`);
    }
    if (fout) {
      if (!(lastWin < SILENT)) fail(`${tag}: its last window at ${lastWin.toFixed(1)} dB, not silence`);
      if (!(after < SILENT)) fail(`${tag}: ${after.toFixed(1)} dB after its end`);
      if (!(before > own - 6)) fail(`${tag}: ${before.toFixed(1)} dB as its fade out begins, the loop's ${own.toFixed(1)}`);
    } else {
      // Nothing faded to the second (with no fade in: from the start; with
      // one, from when it has risen).
      // To a tenth of a second before it: the step on the second is not
      // played, and its graces (up to 0.06 s ahead of it) and the bus
      // compressor's look-ahead reach just before it in the longer one.
      const from = Math.round((fin ? START + FADE_IN + 0.01 : 0) * RATE);
      const to = Math.round((end - 0.1) * RATE);
      let moved = -1;
      for (let i = from; i < to; i++) if (r.sq[i] !== longer.sq[i]) { moved = i; break; }
      if (moved >= 0) fail(`${tag}: a sample at ${(moved / RATE).toFixed(3)} s is not the longer track's`);
      if (!(after > SILENT)) fail(`${tag}: nothing rings after the end (${after.toFixed(1)} dB)`);
    }
    const tail = r.frames / RATE - end;
    if (tail < 0 || tail > TAIL_MAX + 0.1) fail(`${tag}: a tail of ${tail.toFixed(2)} s`);
    rows.push({ tag, first, midIn, atIn, before, midOut, lastWin, after, tail, own });
  }
}

const f = (x) => (x < -150 ? 'silence' : `${x.toFixed(1)} dB`);
console.log(`track-fades: ${SECONDS} s tracks, fade in ${FADE_IN} s, fade out ${FADE_OUT} s, 50 ms windows (silence under ${SILENT} dBFS)`);
for (const r of rows) {
  console.log(`  ${r.tag}`);
  console.log(`    first 5 ms ${f(r.first)}, half the fade in ${f(r.midIn)}, after it ${f(r.atIn)}; the loop ${f(r.own)}`);
  console.log(`    as the fade out begins ${f(r.before)}, halfway ${f(r.midOut)}, last window ${f(r.lastWin)}, after the end ${f(r.after)}; tail ${r.tail.toFixed(2)} s`);
}

if (LONG) {
  // 2:00:07, with the fade out (the defaults): that long, and the kept tail.
  const spec = loop('shatter');
  const x = new WebAssembly.Instance(MODULE, {}).exports;
  const r = new Render(x, { seed: spec.seed >>> 0, ask: askOf(spec), mutes: [], wander: 1, salt: 0, layout: 2, track: [kind(false, true), 7207], format: 'opus' });
  const t0 = Date.now();
  while (r.pcm()) { /* the file's samples */ }
  const secs = r.frames / RATE;
  const tail = secs - r.endTime;
  console.log(`  2:00:07 exported: ${secs.toFixed(3)} s, its end at ${r.endTime.toFixed(6)} s (the loop's first step at ${START} s, so ${(r.endTime - START).toFixed(6)} s of track), a tail of ${tail.toFixed(3)} s; ${((Date.now() - t0) / 1000).toFixed(0)} s to render`);
  if (Math.abs(r.endTime - START - 7207) > 0.5 / RATE) fail(`2:00:07 ended at ${r.endTime - START}`);
  if (tail < 0 || tail > TAIL_MAX + 0.1) fail(`2:00:07's tail ${tail}`);
}

console.log(failures ? `track-fades: ${failures} failed` : 'track-fades: ok');
process.exit(failures ? 1 : 0);
