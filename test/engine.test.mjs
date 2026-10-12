// Tests for the scheduler's deep queue on the core (queue item 40) that need
// no browser: the real engine and generator, a stub synth that writes down
// every call, and a clock moved by hand.
// Run with:  node test/engine.test.mjs
//
// What they pin down: visible, the core is handed two seconds ahead and the
// JavaScript synth 0.3; a change that must be heard at once (mute, unmute,
// re-roll, tempo, wander off) drops what the core holds from 60 ms ahead
// and decides again from there, so nothing before that is handed over
// twice and nothing after it is missing; and the JavaScript synth is not
// touched by any of it.

import { Engine } from '../js/engine.js';
import { newSpec, rerollLayer } from '../js/generator.js';

// The engine draws its jitter from Math.random: seeded, so a run is a run.
let seed = 20261012;
Math.random = () => {
  seed = (seed + 0x6d2b79f5) | 0;
  let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
  t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
  return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
};

let failures = 0;
function check(label, condition, detail = '') {
  console.log(condition ? `  pass  ${label}` : `  FAIL  ${label} ${detail}`);
  if (!condition) failures++;
}

// Where each call carries its time, by the synth's own signatures.
const TIME_AT = { drum: 1, bass: 1, pad: 1, voice: 2, texture: 2, damp: 1, duck: 0 };

function stubSynth(queuesAhead, renderAhead = null) {
  const calls = [];
  const target = {
    queuesAhead,
    renderAhead,
    channels: { drums: { gain: 'drums' }, bass: { gain: 'bass' }, chords: { gain: 'chords' }, melody: { gain: 'melody' }, texture: { gain: 'texture' } },
  };
  const synth = new Proxy(target, {
    get(t, prop) {
      if (prop in t) return t[prop];
      return (...args) => {
        const at = TIME_AT[prop];
        // A voice's channel is its fifth argument (the melody's or the chords').
        const layer = prop === 'voice' ? args[5] : prop === 'drum' ? 'drums' : prop;
        calls.push({ name: prop, time: at == null ? args[0] : args[at], layer, args });
      };
    },
  });
  return { synth, calls };
}

function rig(queuesAhead, seed = 4242, renderAhead = null) {
  const ctx = { currentTime: 0, state: 'running', sampleRate: 48000 };
  const { synth, calls } = stubSynth(queuesAhead, renderAhead);
  const engine = new Engine(ctx, synth);
  const spec = newSpec(seed);
  spec.mutes = { drums: false, bass: false, chords: false, melody: false, texture: false };
  engine.load(spec);
  engine.start();
  engine.clock.stop();
  // The clock by hand: a tick every 50 ms, as visible.
  const run = (to) => {
    while (ctx.currentTime < to - 1e-9) {
      ctx.currentTime = +(ctx.currentTime + 0.05).toFixed(6);
      engine._tick();
    }
  };
  return { ctx, engine, spec, calls, run };
}

const notes = (calls) => calls.filter((c) => TIME_AT[c.name] != null && c.name !== 'duck' && c.name !== 'damp');
const between = (list, a, b) => list.filter((c) => c.time >= a && c.time < b);

console.log('\nHow far ahead, visible');
{
  const core = rig(true);
  core.run(1);
  check('the core is handed two seconds ahead', core.engine.lookahead === 2 && core.engine.nextStepTime > 2.9,
    `lookahead ${core.engine.lookahead}, next step ${core.engine.nextStepTime}`);
  const js = rig(false);
  js.run(1);
  check('the JavaScript synth keeps 0.3 s', js.engine.lookahead === 0.3 && js.engine.nextStepTime < 1.5,
    `lookahead ${js.engine.lookahead}, next step ${js.engine.nextStepTime}`);
  check('and there is nothing to decide again', js.engine.redecide() === false
    && !js.calls.some((c) => c.name === 'cancelFrom'));
}

// The calls made by a change at `now`: everything after the cancel.
function change(r, now, act) {
  r.run(now);
  const before = r.calls.length;
  act();
  const cancel = r.calls.slice(before).find((c) => c.name === 'cancelFrom');
  return { cancel, from: cancel ? cancel.args[0] : null, after: r.calls.slice(before), kept: r.calls.slice(0, before) };
}

console.log('\nMute and unmute');
{
  const r = rig(true);
  const { from, after, kept } = change(r, 1.0, () => {
    r.spec.mutes.melody = true;
    r.engine.redecide();
  });
  check('the core is told to drop what it holds from 60 ms ahead', from != null && Math.abs(from - 1.06) < 1e-6, `from ${from}`);
  const fresh = notes(after);
  check('nothing decided again starts before that', fresh.every((c) => c.time >= from), `${fresh.filter((c) => c.time < from).length} early`);
  check('no melody from then on', !fresh.some((c) => c.layer === 'melody'));
  check('the queue is two seconds deep again', r.engine.nextStepTime > 3);
  // The other layers carry on as they were: the same notes either side of
  // the change, each matched to one within 20 ms (drums jitter by a few ms,
  // drawn again) and by what it plays.
  const what = (c) => `${c.name} ${JSON.stringify(c.name === 'voice' ? c.args[1] : c.args[0])}`;
  const unmatched = (xs, ys) => {
    const left = ys.slice();
    let miss = 0;
    for (const x of xs) {
      const i = left.findIndex((y) => what(y) === what(x) && Math.abs(y.time - x.time) < 0.02);
      if (i < 0) miss++;
      else left.splice(i, 1);
    }
    return miss;
  };
  // Each side's notes in the span, against the other's a little wider, so a
  // note jittered across an edge is not counted.
  const span = (list, pad = 0) => list.filter((c) => c.layer !== 'melody' && c.time >= from + 0.03 - pad && c.time < 2.8 + pad);
  const was = span(notes(kept));
  const now = span(fresh);
  const gone = unmatched(was, span(fresh, 0.02));
  const added = unmatched(now, span(notes(kept), 0.02));
  check(`the other layers' notes after the change are the same (${was.length})`,
    was.length > 10 && gone === 0 && added === 0, `${gone} gone, ${added} new`);
  // Every note from the change on is handed over once: kept up to `from`,
  // decided again from there.
  const handed = [...notes(kept).filter((c) => c.time < from), ...fresh];
  let doubled = 0;
  for (const [i, x] of handed.entries()) {
    if (handed.some((y, j) => j > i && what(y) === what(x) && Math.abs(y.time - x.time) < 0.02)) doubled++;
  }
  check('no note handed over twice', doubled === 0, `${doubled} doubled`);

  const back = change(r, 1.5, () => {
    r.spec.mutes.melody = false;
    r.engine.redecide();
  });
  const melody = notes(back.after).filter((c) => c.layer === 'melody').map((c) => c.time);
  check('unmuted, the melody is decided again from 60 ms ahead', back.from != null && melody.length > 0 && Math.min(...melody) >= back.from,
    `${melody.length} notes, first ${Math.min(...melody)}`);
  check('the visual queue holds each step once', new Set(r.engine.visualQueue.map((v) => v.time.toFixed(6))).size === r.engine.visualQueue.length);
}

console.log('\nRe-roll, tempo, wander');
{
  const r = rig(true, 777);
  const { from, after } = change(r, 2.0, () => {
    const next = rerollLayer(r.spec, 'drums');
    next.mutes = r.spec.mutes;
    r.spec = next;
    r.engine.load(next, { keepPosition: true });
  });
  check('a re-roll decides again from 60 ms ahead, in place', from != null && Math.abs(from - 2.06) < 1e-6
    && notes(after).every((c) => c.time >= from) && r.engine.nextStepTime > 4, `from ${from}`);

  const t = change(r, 3.0, () => {
    r.spec.bpm = Math.round(r.spec.bpm * 1.25);
    r.engine.redecide();
  });
  const steps = r.engine.visualQueue.filter((v) => v.time > t.from + 0.2).map((v) => v.time);
  const gap = steps[1] - steps[0];
  check('a new tempo is decided from 60 ms ahead at the new step', t.from != null && Math.abs(gap - 15 / r.spec.bpm) < 1e-6,
    `step ${gap}, want ${15 / r.spec.bpm}`);

  r.engine.driftOn = true;
  r.run(4);
  const w = change(r, 4.5, () => {
    r.engine.driftOn = false;
    r.engine.live = r.engine.base;
    r.engine.redecide(r.engine.base);
  });
  check('wander off goes back to the loop as composed from 60 ms ahead', w.from != null && r.engine.live === r.engine.base);
}

console.log('\nJumps and stops');
{
  const r = rig(true, 99);
  r.run(1);
  r.engine.stop();
  check('stopped, nothing to decide again', r.engine.redecide() === false && r.engine.decided.length === 0);
  r.engine.start();
  r.engine.clock.stop();
  r.run(2);
  const n = r.engine.decided.length;
  check('playing, a second back and two ahead is kept to go back to', n > 0 && r.engine.decided[0].time >= r.ctx.currentTime - 1.0001,
    `${n} kept, first ${r.engine.decided[0] && r.engine.decided[0].time}`);
}

console.log('\nA phone that renders far ahead of its clock (queue item 41)');
{
  // The Pixel 10a: the core had rendered about 0.6 s past the page's clock
  // by the time a note reached it.
  const r = rig(true, 4242, 0.6);
  const first = Math.min(...notes(r.calls).map((c) => c.time));
  // Its margin: a quarter of it (0.15 s).
  check('Play: the first note is no closer than the render-ahead and its margin', first >= 0.75 - 0.03,
    `first at ${first.toFixed(3)}`);
  const { from, after } = change(r, 1.0, () => {
    r.spec.mutes.melody = true;
    r.engine.redecide();
  });
  check('a change is heard from the render-ahead and its margin, not 60 ms', from != null && Math.abs(from - 1.75) < 1e-6, `from ${from}`);
  check('nothing decided again starts inside the render-ahead', notes(after).every((c) => c.time >= from));
  check('the queue covers the render-ahead too', r.engine.lookahead >= 2 && r.engine.nextStepTime > 1.0 + 2 - 0.2,
    `lookahead ${r.engine.lookahead}, next ${r.engine.nextStepTime}`);
  // A stall: the queue deepens to cover it and the render-ahead.
  r.engine.recentStall = 1.8;
  check('after a 1.8 s stall the queue is deeper (stall + render-ahead + 0.5 s)', Math.abs(r.engine.lookahead - 2.9) < 1e-9,
    `lookahead ${r.engine.lookahead}`);
  r.engine.recentStall = 9;
  check('but never deeper than the hidden queue', r.engine.lookahead === 8);
  r.engine.recentStall = 0;
  // A tick that wakes with the next step inside the render-ahead skips to
  // the soonest it can still be on time, rather than handing over notes
  // that could only be late.
  const before = r.calls.length;
  r.engine.nextStepTime = r.ctx.currentTime + 0.3;
  r.engine._tick();
  const late = notes(r.calls.slice(before)).filter((c) => c.time < r.ctx.currentTime + 0.6);
  const soon = Math.min(...notes(r.calls.slice(before)).map((c) => c.time)) - r.ctx.currentTime;
  check('and it goes on from the soonest it can be on time', soon >= 0.75 - 0.03 && soon < 0.75 + 0.2, `first after ${soon}`);
  check('no note handed over inside the render-ahead after a late tick', late.length === 0, `${late.length} inside`);

  const js = rig(false, 4242, 0.6);
  const jsFirst = Math.min(...notes(js.calls).map((c) => c.time));
  check('the JavaScript synth ignores it: Play at 80 ms as before', Math.abs(jsFirst - 0.08) < 0.02 && js.engine.lookahead === 0.3,
    `first at ${jsFirst}`);
}

console.log(failures === 0 ? '\nAll good.\n' : `\n${failures} failing.\n`);
process.exit(failures === 0 ? 0 : 1);
