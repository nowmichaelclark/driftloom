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
// touched by any of it. And (queue item 47) hidden or frozen, the core is
// planned for a deep output buffer at once: Next, Play and the catch-up after
// a freeze start past it, the old loop playing on until the new one starts;
// visible changes keep their 60 ms. And (queue item 94b) the app's default
// engine, js/player.js, hands the core's player the loop, the controls and
// the track, and draws the core's events; it decides no notes.

import { Engine, UNREPEATABLE_FROM } from '../js/engine.js';
import { VIBE_IDS } from '../js/vibes.js';
import { PlayerEngine, picksAsk, MOMENT, KIND, CHANGE_FADE, END_FADE } from '../js/player.js';
import { boundary } from '../js/quality.js';
import { newSpec, rerollLayer, vibesOf, LAYERS } from '../js/generator.js';
import { echoTimeOf } from '../js/choices.js';

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

// `measured`: a note has reached the core (queue item 76); false is a cold start.
function stubSynth(queuesAhead, renderAhead = null, worstAhead = 0, measured = true) {
  const calls = [];
  const target = {
    queuesAhead,
    renderAhead,
    aheadMeasured: measured,
    worstAhead,
    channels: { drums: { gain: 'drums' }, bass: { gain: 'bass' }, chords: { gain: 'chords' }, melody: { gain: 'melody' }, texture: { gain: 'texture' } },
  };
  const synth = new Proxy(target, {
    get(t, prop) {
      if (prop in t) return t[prop];
      return (...args) => {
        const at = TIME_AT[prop];
        // A voice's channel is its fifth argument (the melody's or the chords').
        const layer = prop === 'voice' ? args[5] : prop === 'drum' ? 'drums' : prop;
        calls.push({ name: prop, time: at == null ? args[0] : args[at], layer, args, why: t.why, key: t.key ? { ...t.key } : null });
      };
    },
  });
  return { synth, calls };
}

function rig(queuesAhead, seed = 4242, renderAhead = null, worstAhead = 0, measured = true, ctxExtra = {}) {
  const ctx = { currentTime: 0, state: 'running', sampleRate: 48000, ...ctxExtra };
  const { synth, calls } = stubSynth(queuesAhead, renderAhead, worstAhead, measured);
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

console.log('\nWhere each note sits in its loop (queue item 44)');
{
  const noise = Math.random;
  const r = rig(true, 777);
  r.run(0.5);
  // The core is handed whole passes ahead; play past the end of the first.
  while (r.engine.loopCount < 1) r.run(r.ctx.currentTime + 1);
  r.run(r.ctx.currentTime + 1);
  const made = notes(r.calls);
  check('every note is stamped with where it sits', made.length > 30 && made.every((c) => c.key && c.key.seed === (r.spec.seed >>> 0)),
    `${made.length} notes, ${made.filter((c) => !c.key).length} unstamped, seed ${made[0] && JSON.stringify(made[0].key)} against ${r.spec.seed}`);
  const places = made.map((c) => `${c.layer}:${c.key.pass}:${c.key.step}:${c.key.ord}`);
  check('and no two notes of a layer share a place', new Set(places).size === places.length);
  check('the pass rises as the loop comes round', made.some((c) => c.key.pass === 0) && made.some((c) => c.key.pass === 1));
  check('the synth is left with no key once a note is built', r.engine.synth.key == null);

  // Another Math.random changes nothing: the engine draws from the notes' places.
  const again = rig(true, 777);
  Math.random = () => { throw new Error('Math.random in the scheduler'); };
  again.run(r.ctx.currentTime);
  Math.random = noise;
  const key = (c) => `${c.name} ${c.layer} ${JSON.stringify(c.key)} ${c.time}`;
  const a = notes(r.calls).filter((c) => c.time < 2.5).map(key);
  const b = notes(again.calls).filter((c) => c.time < 2.5).map(key);
  const firstDiff = a.findIndex((x, i) => x !== b[i]);
  check('played again, the same notes at the same times (jitter, strum and ensemble included)', a.length > 10 && a.join('\n') === b.join('\n'),
    `${a.length} against ${b.length}; first difference ${a[firstDiff]} | ${b[firstDiff]}`);

  // A muted layer takes nothing from the others' places.
  const muted = rig(true, 777);
  muted.spec.mutes.melody = true;
  muted.run(2.5);
  const others = (calls) => notes(calls).filter((c) => c.layer !== 'melody' && c.time < 2.5).map(key);
  check('muting the melody leaves every other layer\'s places as they were', others(muted.calls).join('\n') === others(r.calls).join('\n'));
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

console.log('\nThe first loop after the app starts (queue item 76)');
{
  // Nothing has reached the core yet: no render-ahead to plan from. The
  // Moto's core had rendered 133 ms past the clock; the first notes were
  // posted 77-82 ms ahead.
  const cold = rig(true, 4242, null, 0, false, { baseLatency: 0.02 });
  const first = Math.min(...notes(cold.calls).map((c) => c.time));
  check('cold, the engine knows it', cold.engine.cold === true);
  check('cold, the first note is no closer than 250 ms', first >= 0.25 - 0.03, `first at ${first.toFixed(3)}`);
  check('cold, the core is told the notes were planned from the margin', cold.calls.length > 0
    && notes(cold.calls).every((c) => /\[margin\]$/.test(c.why)), [...new Set(notes(cold.calls).map((c) => c.why))].join(' / '));

  // The device's own output latency, where the browser gives one, when it
  // is more than 250 ms; else the render callback.
  const slow = rig(true, 4242, null, 0, false, { outputLatency: 0.45, baseLatency: 0.02 });
  const slowFirst = Math.min(...notes(slow.calls).map((c) => c.time));
  check('cold, the output latency when it is more than 250 ms', slowFirst >= 0.45 - 0.03 && Math.abs(slow.engine.coldMargin - 0.45) < 1e-9,
    `first at ${slowFirst.toFixed(3)}`);
  const noOut = rig(true, 4242, null, 0, false, { baseLatency: 0.4 });
  check('and the render callback when there is no output latency', Math.abs(noOut.engine.coldMargin - 0.4) < 1e-9);
  const none = rig(true, 4242, null, 0, false, {});
  check('and 250 ms when the browser gives neither', Math.abs(none.engine.coldMargin - 0.25) < 1e-9);

  // A change decided near the clock right after the start (Next, a tempo
  // change in the first seconds) waits for the margin too.
  const next = change(cold, 0.5, () => {
    const spec = newSpec(31337);
    spec.mutes = { drums: false, bass: false, chords: false, melody: false, texture: false };
    cold.engine.load(spec);
  });
  await null;
  const fresh = notes(cold.calls.slice(next.kept.length));
  check('cold, a new loop right after Play starts no closer than 250 ms either',
    fresh.length > 0 && Math.min(...fresh.map((c) => c.time)) >= 0.5 + 0.25 - 0.07, `first at ${Math.min(...fresh.map((c) => c.time)).toFixed(3)}`);

  // Once a note has reached the core, it is planned from the measurement
  // as ever: a mute or a re-roll is heard from 60 ms ahead, the loop starts
  // at 80 ms.
  const warm = rig(true, 4242, null, 0, true, { baseLatency: 0.02 });
  const warmFirst = Math.min(...notes(warm.calls).map((c) => c.time));
  check('measured, Play starts at 80 ms as before', warm.engine.cold === false && Math.abs(warmFirst - 0.08) < 0.02, `first at ${warmFirst.toFixed(3)}`);
  check('measured, the core is told so', notes(warm.calls).every((c) => /\[measured\]$/.test(c.why)));
  const heard = change(warm, 1.0, () => {
    warm.spec.mutes.melody = true;
    warm.engine.redecide();
  });
  check('measured, a change is still heard from 60 ms ahead', heard.from != null && Math.abs(heard.from - 1.06) < 1e-6, `from ${heard.from}`);

  // The measurement arriving mid-session ends the margin.
  cold.engine.synth.aheadMeasured = true;
  check('and a measurement ends the cold margin', cold.engine.cold === false);

  const js = rig(false, 4242, null, 0, false, { baseLatency: 0.02 });
  const jsFirst = Math.min(...notes(js.calls).map((c) => c.time));
  check('the JavaScript synth is unchanged: Play at 80 ms', js.engine.cold === false && Math.abs(jsFirst - 0.08) < 0.02, `first at ${jsFirst}`);
}

console.log('\nHidden, a deep buffer assumed at once (queue item 47)');
{
  // The Pixel 9: 6-21 ms ahead on screen, 1.4 s at least once with the
  // screen locked, before anything had measured it.
  globalThis.document = { hidden: false };
  const r = rig(true, 4242, null, 0.02);
  r.run(3);
  check('visible, the queue is two seconds deep as before', r.engine.lookahead === 2, `lookahead ${r.engine.lookahead}`);
  const vis = change(r, 3.0, () => {
    r.spec.mutes.melody = true;
    r.engine.redecide();
  });
  check('visible, a change is still heard from 60 ms ahead', vis.from != null && Math.abs(vis.from - 3.06) < 1e-6, `from ${vis.from}`);
  r.spec.mutes.melody = false;
  r.engine.redecide();

  // Hidden: the next tick neither counts late nor skips, and the queue
  // deepens to eight seconds.
  globalThis.document.hidden = true;
  const late0 = r.engine.lateTicks;
  const before = r.calls.length;
  r.run(3.5);
  check('hiding is not a late tick', r.engine.lateTicks === late0, `${r.engine.lateTicks - late0} late`);
  check('hidden, the queue is eight seconds deep', r.engine.lookahead === 8 && r.engine.nextStepTime > 3.5 + 7.5);
  const gapless = notes(r.calls.slice(before)).map((c) => c.time).sort((a, b) => a - b);
  check('and carries on from where the visible queue ended', gapless.length > 0 && gapless[0] < 5.6, `first at ${gapless[0]}`);
  // Assumed 1.5 s, and its margin (a quarter): 1.875 s.
  check('hidden, the render-ahead planned from is 1.5 s', Math.abs(r.engine.ahead - 1.5) < 1e-9, `${r.engine.ahead}`);

  // Next from the lock screen: the new loop starts past the deep buffer, and
  // the old one plays on until then.
  const n = change(r, 5.0, () => {
    const spec = newSpec(31337);
    spec.mutes = { drums: false, bass: false, chords: false, melody: false, texture: false };
    r.engine.load(spec);
  });
  await null;
  // Graces run up to 60 ms ahead of their step, and drums jitter by 4.
  const fresh = notes(r.calls.slice(n.kept.length));
  const firstNew = Math.min(...fresh.map((c) => c.time));
  check('Next: only what the old loop holds from the new start is dropped', n.from != null && Math.abs(n.from - (5 + 1.875)) < 1e-6
    && !n.after.some((c) => c.name === 'cancelAhead'), `from ${n.from}`);
  check('Next: no new note inside the deep buffer', firstNew >= 5 + 1.875 - 0.07, `first at ${firstNew}`);
  const told = [...new Set(fresh.filter((c) => c.time < 5 + 1.875 + 0.1).map((c) => c.why))];
  check('the core is told Next decided them', told.length === 1 && told[0] === 'jump, new loop [measured]', told.join(' / '));

  // Pause, then Play, from the lock screen.
  r.engine.stop();
  r.ctx.currentTime = 6.0;
  const p0 = r.calls.length;
  r.engine.start();
  r.engine.clock.stop();
  const firstPlay = Math.min(...notes(r.calls.slice(p0)).map((c) => c.time));
  check('Play while hidden: the first note is past the deep buffer', firstPlay >= 6 + 1.875 - 0.03, `first at ${firstPlay}`);

  // A freeze longer than the queue: the tick that wakes skips to past the
  // deep buffer, never into it.
  r.run(8);
  r.ctx.currentTime = 20;
  const f0 = r.calls.length;
  r.engine._tick();
  const thaw = notes(r.calls.slice(f0)).map((c) => c.time);
  check('after a freeze past the queue: a late tick, and no note inside the deep buffer',
    r.engine.lateTicks > late0 && Math.min(...thaw) >= 20 + 1.875 - 0.03, `first at ${Math.min(...thaw)}`);

  // Shown again: planned from the measurement, as before.
  globalThis.document.hidden = false;
  const back = change(r, 23.0, () => {
    r.spec.mutes.bass = true;
    r.engine.redecide();
  });
  check('shown again, a change is heard from 60 ms ahead', back.from != null && Math.abs(back.from - 23.06) < 1e-6, `from ${back.from}`);

  // Frozen while not yet hidden (`freeze`, `pagehide`): deep at once.
  r.engine.setFrozen(true);
  check('frozen, the deep buffer is assumed and the queue is the hidden one', r.engine.ahead === 1.5 && r.engine.lookahead === 8);
  r.engine.setFrozen(false);

  // A phone that has rendered 2.5 s ahead before (in storage): planned
  // from that, hidden, and the visible queue covers it.
  r.engine.deviceAhead = 2.5;
  check('this device\'s worst before: the visible queue covers it', Math.abs(r.engine.lookahead - 3.0) < 1e-9, `lookahead ${r.engine.lookahead}`);
  const v2 = change(r, 24.0, () => r.engine.redecide());
  check('and visible changes are not slowed by it', v2.from != null && Math.abs(v2.from - 24.06) < 1e-6, `from ${v2.from}`);
  globalThis.document.hidden = true;
  check('hidden, planned from it', r.engine.ahead === 2.5);
  r.engine.deviceAhead = 30;
  check('never more than four seconds', r.engine.ahead === 4);
  globalThis.document.hidden = false;
  r.engine.deviceAhead = 0;

  const js = rig(false, 4242, 0.6, 1.4);
  globalThis.document.hidden = true;
  check('the JavaScript synth: nothing assumed', js.engine.ahead === 0 && js.engine.lookahead === 3);
  globalThis.document.hidden = false;
  delete globalThis.document;
}

console.log('\nglass (queue item 59)');
{
  // A loop that draws glass, played 30 s: its bells are FM notes into the
  // texture channel, in E5-G6, yielding to every other part, at the chord's
  // velocity scaled by the loop's chords a bar.
  let r = null;
  for (let s = 1; s < 400 && !r; s++) {
    const x = rig(true, s * 7919);
    if (x.engine.base.choices.glass === 'on' && x.engine.glassScale > 0.5) r = x;
  }
  r.run(30);
  const bells = r.calls.filter((c) => c.name === 'fm' && c.args[4] && c.args[4].out === 'texture');
  const top = Math.max(...r.engine.base.tracks.chords.filter((e) => e.vel).map((e) => e.vel));
  check('a loop with glass rings bells', bells.length > 0, `${bells.length} bells`);
  check('in E5-G6, yielding', bells.every((c) => c.args[0] >= 76 && c.args[0] <= 91 && c.args[4].yields && c.args[4].ratio === 3.5));
  check('at most the chord\'s velocity, scaled', bells.every((c) => c.args[3] <= 0.62 * (top / 0.5) * r.engine.glassScale + 1e-9),
    `${Math.max(...bells.map((c) => c.args[3]))} against ${0.62 * (top / 0.5) * r.engine.glassScale}`);
  check('each bell carries a place of its own', bells.every((c) => c.key && c.key.ord >= 256));
  // The same loop with every chord one note: no bell rings.
  const one = rig(true, r.spec.seed);
  for (const e of one.engine.base.tracks.chords) if (e.notes && e.notes.length) e.notes = [e.notes[0]];
  one.engine.live = one.engine.base;
  // What start() handed over already was decided from the chords as they were.
  const from = one.calls.length;
  one.run(30);
  const later = one.calls.slice(from).filter((c) => c.name === 'fm' && c.args[4] && c.args[4].yields);
  check('a one-note chord never rings out', later.length === 0, `${later.length} bells`);
}

console.log('\ndubStab (queue item 60)');
{
  // A busy loop that draws dubStab, played 30 s: its stabs fall on the
  // offbeat after beat one or three, voice at most three of the chord's
  // notes folded into G3-C5, and each carries a place of its own.
  let r = null;
  for (let s = 1; s < 3000 && !r; s++) {
    const x = rig(true, s * 7919);
    if (x.engine.stab) r = x;
  }
  r.run(30);
  const stabs = r.calls.filter((c) => c.name === 'dubStab');
  const spb = r.engine.base.stepsPerBar || 16;
  check('a busy loop with dubStab stabs', stabs.length > 0, `${stabs.length} stabs, ${r.engine.stab.kind}`);
  check('on the offbeat after beat one or three', stabs.every((c) => [2, Math.floor(spb / 2) + 2].includes(c.key.step % spb)));
  check('at most three notes, in G3-C5', stabs.every((c) => c.args[0].length >= 1 && c.args[0].length <= 3 && c.args[0].every((m) => m >= 55 && m <= 72)));
  check('each with a place of its own', stabs.every((c) => c.key.ord === 256));
}

console.log('\ntunedKick (queue item 67)');
{
  // A loop that draws the tuned kick, played 30 s: every kick carries the
  // loop's kick and kind; wandering, a pass may swap it, as `kickKindOf` says.
  let r = null;
  for (let s = 1; s < 3000 && !r; s++) {
    const x = rig(true, s * 7919);
    if (x.engine.kick) r = x;
  }
  r.run(20);
  const kicks = r.calls.filter((c) => c.name === 'drum' && (c.args[0] === 'kick' || c.args[0] === 'softkick'));
  check('a loop with the tuned kick hands each kick its kind', kicks.length > 0 && kicks.every((c) => c.args[3] && c.args[3].kind === r.engine.kick.kind
    && c.args[3].kick === r.engine.kick.kick), `${kicks.length} kicks, ${r.engine.kick.kind}`);
  const others = r.calls.filter((c) => c.name === 'drum' && c.args[0] !== 'kick' && c.args[0] !== 'softkick');
  check('and no other drum', others.every((c) => !c.args[3]));
  const plain = (() => {
    for (let s = 1; s < 3000; s++) {
      const x = rig(true, s * 7919);
      if (!x.engine.kick && x.engine.base.tracks.drums.some((e) => e.inst === 'kick' && e.vel)) return x;
    }
    return null;
  })();
  plain.run(10);
  check('a loop without it plays today\'s kick, as before', plain.calls.filter((c) => c.name === 'drum').every((c) => c.args[3] == null));
  // Wandering: the kind of each pass's kicks is that pass's draw.
  const w = rig(true, r.spec.seed);
  w.engine.driftOn = true;
  w.run(240);
  const byPass = new Map();
  for (const c of w.calls.filter((c) => c.name === 'drum' && c.args[3])) byPass.set(c.args[3].kind, (byPass.get(c.args[3].kind) || 0) + 1);
  check('wandering, the kick swaps now and then', byPass.size > 1 && byPass.has(w.engine.kick.kind), JSON.stringify([...byPass]));
}

console.log('\nThe chapel organ (queue item 61)');
{
  // A loop that draws the organ, played 30 s: set up once at its first step,
  // retuned at its chords no more than once a bar, and each call handed over
  // once when a change decides the loop again.
  let seed = 0;
  // One whose chords move the organ three times or more in 30 s.
  for (let s = 1; s < 3000 && !seed; s++) {
    const x = rig(true, s * 104729);
    if (!x.engine.organ) continue;
    x.run(30);
    if (x.calls.filter((c) => c.name === 'organChord').length > 2) seed = s * 104729;
  }
  const r = rig(true, seed);
  r.run(30);
  const of = (calls, name) => calls.filter((c) => c.name === name);
  const setups = of(r.calls, 'organSetup');
  const lets = of(r.calls, 'organOff');
  check('set up once, with its 21 settings, at the first step, the last loop let go there', setups.length === 1 && setups[0].args[0].length === 21
    && setups[0].args[0].every(Number.isFinite) && lets.length > 0 && lets[0].args[0] === setups[0].args[1] && lets[0].args[1] === 0.15
    && setups[0].args[1] < 0.1, JSON.stringify(setups.map((c) => c.args[1])));
  const chords = of(r.calls, 'organChord');
  const bar = (r.engine.base.stepsPerBar || 16) * r.engine.stepDur;
  const gaps = chords.slice(1).map((c, i) => c.args[1] - chords[i].args[1]);
  check('retuned at its chords, at most once a bar', chords.length > 2 && gaps.every((g) => g > bar - 0.03),
    `${chords.length} chords, closest ${Math.min(...gaps).toFixed(3)} s, a bar ${bar.toFixed(3)} s`);
  check('the voice budget never hears of it', !r.calls.some((c) => c.name === '_budget'));

  // A change at 1 s: what is handed over again starts from the cancel, and
  // matches what the loop decided without the change.
  const plain = rig(true, seed);
  plain.run(3);
  const x = rig(true, seed);
  const { from, after, kept } = change(x, 1.0, () => {
    x.spec.mutes.melody = true;
    x.engine.redecide();
  });
  x.run(3);
  const shown = (c) => `${c.args[1].toFixed(6)} ${c.args[0].join(',')}`;
  const again = of(after, 'organChord');
  check('after a change, retuned only from then on', again.every((c) => c.args[1] >= from));
  // Kept up to the change, decided again from there (the core drops the
  // rest): the same chords at the same times as without it, none twice.
  const handed = [...of(kept, 'organChord').filter((c) => c.args[1] < from), ...of(x.calls.slice(kept.length), 'organChord')];
  const was = of(plain.calls, 'organChord');
  check('and the same chords at the same times as without it, none twice',
    handed.filter((c) => c.args[1] < 3).map(shown).join('|') === was.filter((c) => c.args[1] < 3).map(shown).join('|') && handed.length > 0,
    `${handed.map(shown).join(' / ')} against ${was.map(shown).join(' / ')}`);

  // Muting the chords lets it go; so does Pause.
  const m = change(x, 4.0, () => {
    x.spec.mutes.chords = true;
    x.engine.redecide();
  });
  check('muting the chords lets it go', of(m.after, 'organOff').some((c) => c.args[0] >= m.from) && !of(m.after, 'organChord').length);
  const n = r.calls.length;
  r.engine.stop();
  check('and Pause', r.engine.organHold.on === false && of(r.calls.slice(n), 'organOff').length === 1);

  // A loop without it never reaches the organ.
  let none = null;
  for (let s = 1; s < 50 && !none; s++) {
    const y = rig(true, s * 7);
    if (!y.engine.organ) none = y;
  }
  none.run(10);
  check('a loop without it makes no organ call', !none.calls.some((c) => c.name.startsWith('organ')));
}

console.log('\nThe wash (queue item 62)');
{
  // A loop that draws the wash: set up once, at its first step, as the
  // sketch drew it; Pause lets it go, and Play starts its swells again.
  let seed = 0;
  for (let s = 1; s < 3000 && !seed; s++) if (rig(true, s * 7907).engine.wash) seed = s * 7907;
  const r = rig(true, seed);
  r.run(20);
  const setups = r.calls.filter((c) => c.name === 'washSetup');
  const o = setups[0] && setups[0].args[0];
  check('set up once, at the first step', setups.length === 1 && setups[0].args[1] < 0.1, JSON.stringify(setups.map((c) => c.args[1])));
  check('a swell every 17-26 s, tuned in 600-1200 Hz', o && o.period >= 17 && o.period <= 26 && o.tint >= 600 && o.tint <= 1200
    && o.tintGain >= 4 && o.tintGain <= 7 && o.tintHigh >= 2 && o.tintHigh <= 4 && o.tintQ >= 3 && o.tintQ <= 5 && o.level > 0, JSON.stringify(o));
  const n = r.calls.length;
  r.engine.stop();
  check('Pause lets it go', r.calls.slice(n).some((c) => c.name === 'washSetup' && c.args[0] === null));
  r.engine.start();
  r.engine.clock.stop();
  r.run(r.ctx.currentTime + 1);
  const again = r.calls.slice(n).filter((c) => c.name === 'washSetup' && c.args[0]);
  check('and Play starts it again', again.length === 1 && JSON.stringify(again[0].args[0]) === JSON.stringify(o));
  let none = null;
  for (let s = 1; s < 50 && !none; s++) {
    const y = rig(true, s * 13);
    if (!y.engine.wash) none = y;
  }
  none.run(10);
  check('a loop without it makes no wash call', !none.calls.some((c) => c.name === 'washSetup'));
}

console.log('\nsubDuck (queue item 68)');
{
  // A loop that draws subDuck: its depth and recovery handed to the synth
  // once, at its first step; sent again with the loop on Play; a loop
  // without it makes no call.
  let seed = 0;
  for (let s = 1; s < 3000 && !seed; s++) if (rig(true, s * 7907).engine.subDuck) seed = s * 7907;
  const r = rig(true, seed);
  r.run(20);
  const setups = r.calls.filter((c) => c.name === 'subDuckSetup');
  const o = setups[0] && setups[0].args[0];
  check('set up once, at the first step', setups.length === 1 && setups[0].args[1] < 0.1, JSON.stringify(setups.map((c) => c.args[1])));
  check('5-9 dB, back in 0.15-0.25 s', o && o.depth >= 5 && o.depth <= 9 && o.recover >= 0.15 && o.recover <= 0.25, JSON.stringify(o));
  const n = r.calls.length;
  r.engine.stop();
  r.engine.start();
  r.engine.clock.stop();
  r.run(r.ctx.currentTime + 1);
  const again = r.calls.slice(n).filter((c) => c.name === 'subDuckSetup' && c.args[0]);
  check('and sent again on Play', again.length === 1 && JSON.stringify(again[0].args[0]) === JSON.stringify(o));
  let none = null;
  for (let s = 1; s < 50 && !none; s++) {
    const y = rig(true, s * 13);
    if (!y.engine.subDuck) none = y;
  }
  none.run(10);
  check('a loop without it makes no subDuck call', !none.calls.some((c) => c.name === 'subDuckSetup'));
}

console.log('\nlateFloor (queue item 69)');
{
  // A loop that draws lateFloor: no kit and no bass until its wait is over,
  // then both; the other layers from the first step. A reroll keeps the wait
  // it is in; a resumed loop (the engine rebuilt) doesn't wait again.
  let seed = 0;
  for (let s = 1; s < 3000 && !seed; s++) {
    const x = rig(true, s * 7907);
    if (x.engine.late && x.engine.base.tracks.drums.some((e) => e.vel) && x.engine.base.tracks.bass.some((e) => e.vel)) seed = s * 7907;
  }
  const r = rig(true, seed);
  const L = r.engine.late;
  const sd = r.engine.stepDur;
  r.run(L.bars * 16 * sd + 20);
  const first = (name) => Math.min(...r.calls.filter((c) => c.name === name).map((c) => c.args[1]));
  const at = (steps) => 0.05 + steps * sd;
  check('the kit arrives after its wait, within a step', Math.abs(first('drum') - at(L.drums)) < sd + 0.01, `${first('drum').toFixed(2)} against ${at(L.drums).toFixed(2)}, ${L.order}`);
  check('and the bass after its own', Math.abs(first('bass') - at(L.bass)) < sd + 0.01, `${first('bass').toFixed(2)} against ${at(L.bass).toFixed(2)}`);
  const others = notes(r.calls).filter((c) => c.name !== 'drum' && c.name !== 'bass').map((c) => c.time);
  check('something else plays from the start', others.length && Math.min(...others) < at(16), `${Math.min(...others)}`);
  // Rerolled during the wait: the wait goes on where it was.
  const w = rig(true, seed);
  w.run(2);
  w.engine.load(w.spec, { keepPosition: true });
  check('a reroll keeps the wait it is in', w.engine.late && w.engine.late.drums === L.drums);
  const re = rig(true, seed);
  re.engine.load(re.spec, { resume: true });
  check('a resumed loop doesn\'t wait again', re.engine.late === null);
}

console.log('\nthe bass lift (queue item 76c)');
{
  // +3 dB in a loop with a kick, none in one without: handed to the synth
  // once at the first step, held through the loop (a late kit or a floor
  // drop doesn't change it), and sent again on Play.
  let withKick = null;
  let without = null;
  for (let s = 1; s < 400 && !(withKick && without); s++) {
    const x = rig(true, s * 31);
    const kick = x.engine.base.tracks.drums.some((e) => e.vel && (e.inst === 'kick' || e.inst === 'softkick' || e.inst === 'frame'));
    if (kick && !withKick) withKick = x;
    else if (!kick && !without) without = x;
  }
  withKick.run(10);
  const sent = withKick.calls.filter((c) => c.name === 'bassLiftSetup');
  check('a loop with a kick sends +3 dB once, at its first step', sent.length === 1 && sent[0].args[0] === 3, JSON.stringify(sent.map((c) => c.args)));
  const n = withKick.calls.length;
  withKick.engine.stop();
  withKick.engine.start();
  withKick.engine.clock.stop();
  withKick.run(withKick.ctx.currentTime + 1);
  check('and sends it again on Play', withKick.calls.slice(n).filter((c) => c.name === 'bassLiftSetup' && c.args[0] === 3).length === 1);
  without.run(10);
  const none = without.calls.filter((c) => c.name === 'bassLiftSetup');
  check('a loop without one sends today\'s level, 0', none.length === 1 && none[0].args[0] === 0, JSON.stringify(none.map((c) => c.args)));
}

console.log('\npadSweep (queue item 72)');
{
  // A loop that draws padSweep: its sweep handed to the synth once, at its
  // first step, and again on Play; a loop without it makes no call.
  let seed = 0;
  for (let s = 1; s < 3000 && !seed; s++) if (rig(true, s * 7907).engine.padSweep) seed = s * 7907;
  const r = rig(true, seed);
  r.run(20);
  const setups = r.calls.filter((c) => c.name === 'padSweepSetup');
  const o = setups[0] && setups[0].args[0];
  check('set up once, at the first step', setups.length === 1 && setups[0].args[1] < 0.1, JSON.stringify(setups.map((c) => c.args[1])));
  check('closed at 450-800 Hz, 2-3.5 octaves, every 12-28 s', o && o.lo >= 450 && o.lo <= 800 && o.oct >= 2 && o.oct <= 3.5 && o.period >= 12 && o.period <= 28, JSON.stringify(o));
  const n = r.calls.length;
  r.engine.stop();
  r.engine.start();
  r.engine.clock.stop();
  r.run(r.ctx.currentTime + 1);
  check('and sent again on Play', r.calls.slice(n).filter((c) => c.name === 'padSweepSetup' && c.args[0]).length === 1);
  let none = null;
  for (let s = 1; s < 50 && !none; s++) {
    const y = rig(true, s * 13);
    if (!y.engine.padSweep) none = y;
  }
  none.run(10);
  check('a loop without it makes no padSweep call', !none.calls.some((c) => c.name === 'padSweepSetup'));
}

console.log('\nechoTime (queue item 73)');
{
  // The echo's time at load: the loop's own draw, in beats, at its tempo.
  const times = new Set();
  for (const seed of [3, 41, 77, 1234, 9001, 31337]) {
    const r = rig(true, seed);
    const beats = echoTimeOf(r.spec.seed, vibesOf(r.spec));
    const set = r.calls.filter((c) => c.name === 'setEchoTime');
    const last = set[set.length - 1];
    check(`loop ${seed}: the echo at ${beats.toFixed(3)} of a beat`, r.engine.echoBeats === beats && last && Math.abs(last.args[0] - (60 / r.spec.bpm) * beats) < 1e-9,
      JSON.stringify(set.map((c) => c.args[0])));
    times.add(beats);
  }
  check('not every loop at the dotted eighth', times.size > 1, [...times].join(' '));
}

console.log('\nThe arc (queue item 63)');
{
  // A loop that draws the arc: started once, at its first step, over 2-5
  // minutes, at a bright point of its swing; a loop without it never starts
  // one.
  let seed = 0;
  for (let s = 1; s < 3000 && !seed; s++) if (rig(true, s * 6151).engine.arc) seed = s * 6151;
  const r = rig(true, seed);
  r.run(10);
  const arcs = r.calls.filter((c) => c.name === 'arcSetup');
  const o = arcs[0] && arcs[0].args[0];
  check('started once, at the first step', arcs.length === 1 && arcs[0].args[1] < 0.1, JSON.stringify(arcs.map((c) => c.args[1])));
  const bright = o && (o.shape === 'deep' ? o.from >= 0.4 && o.from <= 0.6 : o.shape === 'dip' && o.from >= o.dip / 2 && o.from <= 1 - o.dip / 2);
  check('over 2-5 minutes, from a bright point of its shape', o && o.period >= 120 && o.period <= 300 && bright, JSON.stringify(o));
  let none = null;
  for (let s = 1; s < 50 && !none; s++) {
    const y = rig(true, s * 17);
    if (!y.engine.arc) none = y;
  }
  none.run(10);
  check('a loop without it starts none', !none.calls.some((c) => c.name === 'arcSetup'));
}

console.log('\nThe space (queue item 65)');
{
  // Every loop tells the core what its space needs, once, at its first
  // step: its seed, how busy and how wet it is, and whether its texture is
  // light.
  const r = rig(true, 4242);
  r.run(10);
  const sent = r.calls.filter((c) => c.name === 'spaceSetup');
  const o = sent[0] && sent[0].args[0];
  check('sent once, at the first step', sent.length === 1 && sent[0].args[1] < 0.1, JSON.stringify(sent.map((c) => c.args[1])));
  check('the loop\'s seed, how busy and wet it is, whether its texture is light',
    o && o.seed === (r.spec.seed >>> 0) && o.busy > 0 && o.wet === (r.spec.tone.space || 0) && typeof o.light === 'boolean', JSON.stringify(o));
  let light = null;
  for (let s = 1; s < 400 && !light; s++) {
    const y = rig(true, s * 31);
    if (['bells', 'chime', 'drops'].includes(y.engine.base.meta.textureKind)) light = y;
  }
  check('a light texture (bells, chimes, drops) says so', light && light.engine.space.light === true);
}

console.log('\nThe core\'s player (queue item 94b)');
{
  // The core's player (queue item 94b, js/player.js): the app's engine by
  // default. It waits for a core still on its way, then hands it the loop
  // (its seed, request and a code's edits), the controls and the track; a
  // new loop is a change with a crossfade, a re-roll a revision in place;
  // Pause stops the core and Play goes on where it stood; the cursor, the
  // grid, the passes and the track's end come from the core's events, and
  // the loop armed for the track's end is taken without a second change.
  const ctx = { currentTime: 0, state: 'running', sampleRate: 48000 };
  const posts = [];
  const core = {
    player: true, failed: null, onevents: null, playerAt: null, eventsLost: 0, refused: 0,
  };
  for (const m of ['playLoad', 'playChange', 'playRevise', 'playTrack', 'playArm', 'playTone', 'playStop', 'playResume', 'playWander', 'playMute', 'playTempo', 'playVolume', 'cancel']) {
    core[m] = (...args) => posts.push([m, ...args]);
  }
  const synthCalls = [];
  const synth = { core: null, userVolume: 0.7, unsilence: () => synthCalls.push('unsilence'), silence: () => synthCalls.push('silence') };
  const engine = new PlayerEngine(ctx, synth);
  engine.driftOn = true;
  engine.driftAmount = 1;
  const spec = newSpec(8282, 'key=D|G,scale=dorian|aeolian,style=tide|shrine,voice.melody=kalimba|marimba,voice.bass=sub,layer.texture=on,layer.drums=off');
  spec.mutes = { drums: false, bass: true, chords: false, melody: false, texture: false };
  engine.load(spec);
  engine.start();
  check('player: before the core, nothing is sent and Play is heard as the core arrives', posts.length === 0 && synthCalls[0] === 'unsilence');
  synth.core = core;
  engine.coreArrived();
  const load = posts.find((p) => p[0] === 'playLoad');
  check('player: as the core arrives, it is handed the loop of the spec\'s seed, as soon as it is made', !!load && load[1] === spec.seed >>> 0 && load[3] === 0 && load[4] === false);
  const ask = load && load[2];
  check('player: the key and the mode asked go with it, a bit each (queue item 89)', !!ask && ask.keys === ((1 << 2) | (1 << 7)) && ask.scales === 0b11, JSON.stringify(ask));
  check('player: and the style asked, a bit by place in the profiles\' frozen order (queue item 90)', !!ask && ask.styles === ((1 << 12) | (1 << 13)));
  check('player: and the voices and layers, a mask and a state for each layer in the layers\' order (queue item 91)', !!ask
    && JSON.stringify(ask.voices) === JSON.stringify([0, 1 << 0, 0, (1 << 19) | (1 << 20), 0]) && JSON.stringify(ask.layers) === JSON.stringify([2, 0, 0, 0, 1]));
  check('player: and a code\'s edits, whole: every layer\'s seed, the tone and the tempo (queue item 94a)', !!ask
    && ask.seeds.length === 5 && ask.seeds.every(([i, s, rolled]) => s === spec.layerSeeds[LAYERS[i]] >>> 0 && rolled === false)
    && JSON.stringify(ask.tone) === JSON.stringify([spec.tone.warmth, spec.tone.space, spec.tone.wobble]) && ask.bpm === spec.bpm);
  const wanderAt = posts.findIndex((p) => p[0] === 'playWander');
  check('player: the wander goes before the loop, its first pass drawn with it', wanderAt >= 0 && wanderAt < posts.indexOf(load));
  check('player: the volume, the layers\' mutes and the track go with it', posts.some((p) => p[0] === 'playVolume' && p[1] === 0.7)
    && posts.filter((p) => p[0] === 'playMute').length === 5 && posts.some((p) => p[0] === 'playMute' && p[1] === 1 && p[2] === true)
    && posts.some((p) => p[0] === 'playTrack' && p[1] === 0));
  // The controls.
  posts.length = 0;
  engine.setMute('drums', true);
  engine.setTempo(91);
  engine.setTone({ warmth: 0.2, space: 0.3, wobble: 0.4 });
  engine.setWander(2.4);
  const salted = posts.find((p) => p[0] === 'playWander');
  engine.setWander(0);
  check('player: a mute, the tempo and the tone are passed on', posts.some((p) => p[0] === 'playMute' && p[1] === 0 && p[2] === true)
    && posts.some((p) => p[0] === 'playTempo' && p[1] === 91) && posts.some((p) => p[0] === 'playTone' && p[1].warmth === 0.2));
  check('player: the wander, salted in its upper range, and off at 0 (queue item 92)', !!salted && salted[1] === 2.4 && salted[2] === engine.salt && salted[2] !== 0
    && posts.some((p) => p[0] === 'playWander' && p[1] === 0 && p[2] === 0) && engine.driftOn === false);
  engine.setWander(1);
  // A re-roll: made again in its place, the play's salt kept.
  posts.length = 0;
  const salt = engine.salt;
  const rolled = rerollLayer(spec, 'melody');
  engine.load(rolled, { keepPosition: true });
  const revise = posts.find((p) => p[0] === 'playRevise');
  check('player: a re-roll is a revision in place, its layer re-rolled, the salt kept', !!revise && revise[1] === spec.seed >>> 0
    && revise[2].seeds[3][1] === rolled.layerSeeds.melody >>> 0 && revise[2].seeds[3][2] === true && engine.salt === salt && !posts.some((p) => p[0] === 'playChange'));
  // A new loop while playing: a change now, with a crossfade.
  posts.length = 0;
  const other = newSpec(9393);
  other.mutes = { drums: false, bass: false, chords: false, melody: false, texture: false };
  engine.load(other);
  const change = posts.find((p) => p[0] === 'playChange');
  check('player: Next is a change now, the old loop crossfading over the core\'s second', !!change && change[1] === other.seed >>> 0 && change[3] === MOMENT.now && change[4] === CHANGE_FADE);
  check('player: and a new play draws its own salt', engine.salt !== salt && engine.salt !== 0);
  // The events: the cursor, the grid of each pass, the track's end.
  const sd = engine.stepDur;
  const grid = LAYERS.map((l, i) => (i === 0 ? [0.9, 0, 0.4, 0] : []));
  core.onevents({
    events: [1, KIND.loop, other.seed, 0, 1, KIND.pass, 0, 0, 1, KIND.step, 0, 0, 1 + sd, KIND.step, 1, 1, 1 + 2 * sd, KIND.step, 2, 2],
    grids: [{ at: 0, grid }], trackEnd: -1,
  });
  ctx.currentTime = 1 + sd + 0.001;
  const lit = engine.visualStep();
  check('player: the cursor follows the core\'s steps, at their time', lit === 1 && engine.visualQueue.length === 1, `${lit}`);
  check('player: the grid is the core\'s, from its pass\'s time', engine.live.tracks.drums.length === 2 && engine.live.tracks.drums[1].step === 2 && engine.live.cycles.drums === 4);
  check('player: where it stands, for the quality switch\'s boundary', engine.step === 3 && Math.abs(engine.nextStepTime - (1 + 3 * sd)) < 1e-9);
  const at = boundary(engine, 16, ctx.currentTime);
  check('player: the next pass\'s boundary is its line', Math.abs(at - (1 + 16 * sd)) < 1e-9, `${at}`);
  // The length slider: the loop that follows armed for the track's end, in
  // the same message.
  posts.length = 0;
  other.playFor = 2;
  const next = newSpec(9494);
  engine.trackChanged(next);
  const track = posts.find((p) => p[0] === 'playTrack');
  check('player: the track and the loop armed for its end go together', !!track && track[1] === 1 && track[2] === 2 && track[3].seed === next.seed >>> 0 && track[3].fade === END_FADE);
  core.onevents({ events: [], trackEnd: 96 });
  check('player: the track\'s end, as the core counts it', engine.endStep() === 96);
  // Pause, then Play: the core stops and goes on where it stood; the armed
  // loop goes again (Pause dropped it), the track's count is not restarted.
  posts.length = 0;
  engine.stop();
  check('player: Pause stops the core, drops what has not started, and silences', posts.some((p) => p[0] === 'playStop') && posts.some((p) => p[0] === 'cancel') && synthCalls.includes('silence'));
  posts.length = 0;
  engine.start();
  check('player: Play goes on where it stood, the armed loop sent again, the track not restarted', posts.some((p) => p[0] === 'playResume')
    && !posts.some((p) => p[0] === 'playLoad') && posts.some((p) => p[0] === 'playArm' && p[1].seed === next.seed >>> 0) && !posts.some((p) => p[0] === 'playTrack'));
  // The track ends: the core has started the armed loop; the page catches up
  // without a second change.
  let ended = 0;
  engine.onTrackEnd = () => ended++;
  core.onevents({ events: [5, KIND.end, 0, 96, 5, KIND.loop, next.seed, 0], trackEnd: -1 });
  posts.length = 0;
  next.mutes = { drums: false, bass: false, chords: false, melody: false, texture: false };
  engine.load(next);
  check('player: at the track\'s end the page is told, and takes the loop the core took', ended === 1 && !posts.some((p) => p[0] === 'playChange' || p[0] === 'playLoad'));
  posts.length = 0;
  engine.load(newSpec(9595));
  check('player: a loop after that is a change again', posts.some((p) => p[0] === 'playChange'));
  // The quality switch's new engine: the loop from its top, resumed.
  posts.length = 0;
  const again = new PlayerEngine(ctx, synth);
  again.load(next, { resume: true });
  again.start();
  const resumed = posts.find((p) => p[0] === 'playLoad');
  check('player: a loop resumed on a new core is a load from its top, its lateFloor not waited again', !!resumed && resumed[1] === next.seed >>> 0 && resumed[4] === true);
}

console.log('\nThe picks through the core (queue item 95)');
{
  // The PlayerEngine sends the picks as a command, once a core, with a seed
  // for the core's own loops; Next asks the core for one of its own, and the
  // page hears which (onOwn) and catches up without a second change; the
  // track's end with nothing armed is the core's own, so the page's
  // onTrackEnd is not called; an arm of null drops the one waiting.
  const ctx = { currentTime: 0, state: 'running', sampleRate: 48000 };
  const posts = [];
  const core = { player: true, failed: null, onevents: null, onpicks: null, playerAt: null, eventsLost: 0, refused: 0 };
  for (const m of ['playLoad', 'playChange', 'playRevise', 'playTrack', 'playArm', 'playTone', 'playStop', 'playResume', 'playWander', 'playMute', 'playTempo', 'playVolume', 'cancel', 'playPicks', 'playDraws', 'playNext']) {
    core[m] = (...args) => posts.push([m, ...args]);
  }
  const synth = { core: null, userVolume: 0.7, unsilence: () => {}, silence: () => {} };
  const engine = new PlayerEngine(ctx, synth);
  const sent = picksAsk({ vibes: ['cozy'], tempo: [77, 77], length: 12 });
  engine.setPicks(sent.ask, 1);
  check('picks: none sent before a core', posts.length === 0 && engine.ownLoops);
  const spec = newSpec(5151);
  engine.load(spec);
  engine.start();
  synth.core = core;
  engine.coreArrived();
  const draws = posts.findIndex((p) => p[0] === 'playDraws');
  const sentPicks = posts.findIndex((p) => p[0] === 'playPicks');
  check('picks: as the core arrives, a seed to draw from and the picks, before the loop', draws >= 0 && posts[draws][1] !== 0 && sentPicks > draws
    && posts[sentPicks][1] === sent.ask && posts[sentPicks][2] === 1 && sentPicks < posts.findIndex((p) => p[0] === 'playLoad'));
  check('picks: with the vibes a listener\'s pick, no weight, and the tempo and the bars asked', JSON.stringify(sent.ask.vibes) === JSON.stringify([[VIBE_IDS.indexOf('cozy'), null]])
    && JSON.stringify(sent.ask.tempo) === '[77,77]' && sent.ask.length === 12);
  let heard = null;
  engine.onPicks = (m) => { heard = m; };
  core.onpicks({ tag: 1, met: true, why: null });
  check('picks: the core\'s answer reaches the page', heard && heard.tag === 1 && heard.met);
  posts.length = 0;
  engine.setPicks(picksAsk({ vibes: ['glacial'] }).ask, 2);
  check('picks: a pick made while it plays is sent at once, and the loop playing is left as it is', posts.length === 1 && posts[0][0] === 'playPicks' && posts[0][2] === 2);
  // Next: the core's own.
  posts.length = 0;
  engine.next();
  const next = posts.find((p) => p[0] === 'playNext');
  check('picks: Next asks the core for a loop of its own, crossfading', !!next && next[1] === MOMENT.now && next[2] === CHANGE_FADE && !posts.some((p) => p[0] === 'playChange'));
  let owned = null;
  engine.onOwn = (seed, tag) => { owned = [seed, tag]; };
  core.onevents({ events: [2, KIND.own, 2, 0, 2, KIND.loop, 31337, 0], trackEnd: -1 });
  check('picks: the page hears the core\'s own loop, its seed and the picks it took', owned && owned[0] === 31337 && owned[1] === 2);
  posts.length = 0;
  const salt = engine.salt;
  const caught = newSpec(31337, ['glacial']);
  engine.load(caught);
  check('picks: the page catches up without a second change, the play\'s salt kept', !posts.some((p) => p[0] === 'playChange' || p[0] === 'playLoad') && engine.salt === salt);
  // The track's end with nothing armed: the core's own, not the page's.
  let ended = 0;
  engine.onTrackEnd = () => ended++;
  posts.length = 0;
  caught.playFor = 1;
  engine.trackChanged(null);
  const track = posts.find((p) => p[0] === 'playTrack');
  check('picks: a length with nothing of the page\'s to follow arms nothing (the core\'s own follows)', !!track && track[1] === 1 && track[3] === null);
  core.onevents({ events: [9, KIND.end, 0, 64, 9, KIND.own, 2, 0, 9, KIND.loop, 4242, 0], trackEnd: -1 });
  check('picks: and at its end the page is not sent forward itself: the core\'s own loop is told', ended === 0 && owned[0] === 4242);
  // An armed loop goes with its own track; an arm of null drops it.
  posts.length = 0;
  const armed = newSpec(6161);
  armed.playFor = 3;
  engine.arm(armed);
  engine.arm(null);
  const arms = posts.filter((p) => p[0] === 'playArm');
  check('picks: an armed loop goes with the track it plays, and null drops it', arms.length === 2 && JSON.stringify(arms[0][1].track) === '[1,3]' && arms[1][1] === null);
  // Next from Pause starts playing, the core's own loop first.
  engine.stop();
  posts.length = 0;
  engine.next();
  check('picks: Next from Pause plays the core\'s own loop, not the old one', engine.playing && posts.some((p) => p[0] === 'playNext') && !posts.some((p) => p[0] === 'playResume' || p[0] === 'playLoad'));
  // A new core (the quality switch, a reload) is given the picks again.
  posts.length = 0;
  const again = new PlayerEngine(ctx, synth);
  again.setPicks(sent.ask, 3);
  again.load(spec, { resume: true });
  again.start();
  check('picks: a new engine gives its core the picks again', posts.some((p) => p[0] === 'playPicks' && p[2] === 3));
}

console.log('\nLength (queue items 88 and 88b)');
{
  // A piece plays its sections, a round's last cut short, and comes round
  // to its first section; its first section is section 0. A loop's own
  // length never ends it: only the length slider does (queue item 88b).
  const tops = (spec, passes, { drift = true, onEnd = null } = {}) => {
    const ctx = { currentTime: 0, state: 'running', sampleRate: 48000 };
    const { synth, calls } = stubSynth(false);
    const engine = new Engine(ctx, synth);
    engine.driftOn = drift;
    // A load after a loop that played three passes: today's first pass is
    // drawn at 3, a piece's at its own 0.
    engine.loopCount = 3;
    engine.load(spec);
    const first = JSON.stringify(engine.live);
    let ended = null;
    let fade = null;
    engine.onTrackEnd = () => { if (ended == null) { ended = engine.absStep; fade = engine.endFade; } if (onEnd) onEnd(engine); };
    const starts = [0];
    const lives = [first];
    let n = 0;
    while (engine.loopCount < passes && n++ < 1e7 && !engine.handover) {
      const before = engine.loopCount;
      engine._advance();
      if (engine.loopCount !== before) { starts.push(engine.absStep); lives.push(JSON.stringify(engine.live)); }
    }
    engine.clock.dispose();
    return { starts, lives, ended, fade, calls, first, zero: JSON.stringify(engine._wander(0)), three: JSON.stringify(engine._wander(3)), engine };
  };
  // A loop of 16 or 24 bars, which 100 doesn't divide.
  let at = 7100;
  while (![16, 24].includes(newSpec(at, 'bars=100').bars)) at++;
  const spec = newSpec(at, 'bars=100');
  const spb = spec.stepsPerBar;
  const sections = Math.ceil(100 / spec.bars);
  const { starts, ended, first, zero, three } = tops(spec, 2 * sections + 1);
  const lens = starts.slice(1).map((x, i) => (x - starts[i]) / spb);
  const want = [];
  for (let i = 0; i < 2 * sections + 1; i++) want.push(i % sections === sections - 1 ? 100 % spec.bars : spec.bars);
  check(`100 bars of a ${spec.bars}-bar loop: its sections, the last cut short, then round again`, JSON.stringify(lens) === JSON.stringify(want), JSON.stringify(lens));
  check('with the slider off it plays on past its 100 bars', ended === null && starts[sections] === 100 * spb && starts[2 * sections] === 200 * spb, String(ended));
  check('a piece\'s first section is section 0, not the last loop\'s count', first === zero && first !== three);
  const still = tops(spec, 2 * sections + 1, { drift: false });
  check('wander off: pass 2\'s section 0 is pass 1\'s', still.lives[sections] === still.lives[0] && still.lives[0] === JSON.stringify(still.engine.base));
  const plain = newSpec(7100);
  const t = tops(plain, 3);
  check('a loop with no length keeps today\'s first pass and plays on', t.first === t.three && t.ended === null);
  const short = tops(newSpec(7101, 'bars=3'), 4);
  check('3 bars: passes of 3 bars, never handed over with the slider off', short.starts.slice(1).every((x, i) => x - short.starts[i] === 3 * short.engine.spec.stepsPerBar) && short.starts.length === 5 && short.ended === null);
  const s14 = newSpec(7103, 'bars=14');
  const fourteen = tops(s14, 3);
  check('14 bars: a cycle of 14 bars, three passes and on with the slider off', s14.bars === 14 && fourteen.ended === null
    && fourteen.starts.slice(1).every((x, i) => x - fourteen.starts[i] === 14 * s14.stepsPerBar), JSON.stringify(fourteen.starts));
  const endless = tops(newSpec(7102, 'bars=endless'), 6);
  check('endless: whole sections, never handed over', endless.ended === null
    && endless.starts.slice(1).every((x, i) => x - endless.starts[i] === endless.engine.spec.bars * endless.engine.spec.stepsPerBar));

  // The slider: whole passes hand over at a pass's end, with no fade.
  const two = { ...s14, playFor: 2 };
  const twice = tops(two, 9);
  check('14 bars, the slider at two passes: handed over at 28 bars, no fade', twice.ended === 28 * s14.stepsPerBar && twice.fade === null, `${twice.ended} ${twice.fade}`);
  const round2 = tops({ ...spec, playFor: 2 }, 9 * sections);
  check('100 bars, the slider at two passes: handed over at 200 bars, no fade', round2.ended === 200 * spb && round2.fade === null, String(round2.ended));
  // Shorter than a pass: past the bars, at the first phrase line, fading.
  let lat = 7200;
  while (![16, 24, 32].includes(newSpec(lat, 'bars=1024').bars)) lat++;
  const long = newSpec(lat, 'bars=1024');
  const bars5 = Math.ceil(300 / ((long.stepsPerBar || 16) * (60 / long.bpm / 4)));
  const mid = tops({ ...long, playBars: bars5 }, 1e6);
  const lspb = long.stepsPerBar;
  const line = Math.ceil(bars5 / 4) * 4 * lspb;
  check(`1,024 bars (${long.bars}-bar sections), ${bars5} bars set: handed over at the first four-bar line past them, fading there`,
    mid.ended === line && mid.ended >= bars5 * lspb && mid.fade != null && Math.abs(mid.fade - line * mid.engine.stepDur) < 1e-6,
    `${mid.ended} against ${line}, fade ${mid.fade}`);
  const ahead = tops({ ...long, playBars: bars5 }, 0);
  check('the clock counts down to that line', ahead.engine.endStep() === line, String(ahead.engine.endStep()));
  const fiveBar = newSpec(7300, 'bars=endless');
  fiveBar.bars = 5; // sections of five bars: no four- or two-bar lines
  const odd = tops({ ...fiveBar, playBars: 7 }, 1e6);
  check('Endless of five-bar sections, 7 bars set: handed over at the section\'s end, 10 bars', odd.ended === 10 * fiveBar.stepsPerBar && odd.fade != null, String(odd.ended));

  // Decided on the core: nothing of the old loop at or after the line, and
  // its notes let go there before the next loop's are handed over.
  const r = rig(true, lat);
  r.engine.load({ ...long, mutes: r.spec.mutes, playBars: 8 });
  let endAt = null;
  r.engine.onTrackEnd = () => { endAt = r.engine.nextStepTime; };
  r.run(40);
  const notes = r.calls.filter((c) => ['drum', 'bass', 'pad', 'voice', 'texture'].includes(c.name));
  const fades = r.calls.filter((c) => c.name === 'fadeNotes');
  check('on the core: no note decided at or past the line', endAt != null && notes.every((c) => c.time < endAt + 0.2) && notes.filter((c) => c.time >= endAt - 1e-6).length === 0,
    `end ${endAt}, last ${Math.max(...notes.map((c) => c.time))}`);
  check('and every note let go from the line over a second', fades.length === 1 && Math.abs(fades[0].args[0] - endAt) < 1e-9 && Math.abs(fades[0].args[1] - endAt - 1) < 1e-9
    && r.calls.indexOf(fades[0]) > r.calls.indexOf(notes[notes.length - 1]), JSON.stringify(fades.map((c) => c.args)));
  r.engine.clock.dispose();

  // Moved while playing: counted from the step heard, the pass playing the
  // first; a loop's wander count is left alone.
  const e = tops({ ...s14, playFor: 1 }, 0).engine;
  let endedAt = null;
  e.onTrackEnd = () => { endedAt ??= e.absStep; };
  for (let i = 0; i < 20 * s14.stepsPerBar; i++) e._advance();
  check('before the slider moved, one pass ended it at 14 bars', endedAt === 14 * s14.stepsPerBar, String(endedAt));
  e.handover = false;
  e.restartCount();
  const was = e.loopCount;
  check('moved: one pass ends it at the end of the pass playing', e.endStep() === 28 * s14.stepsPerBar && e.loopCount === was, String(e.endStep()));
}

console.log('\n"Let the loop wander", unrepeatable in its upper range (queue item 92)');
{
  const passes = (amount, seed = 7400) => {
    const ctx = { currentTime: 0, state: 'running', sampleRate: 48000 };
    const { synth } = stubSynth(false);
    const engine = new Engine(ctx, synth);
    engine.driftOn = true;
    engine.driftAmount = amount;
    engine.load(newSpec(seed));
    const out = [];
    for (let n = 0; n <= 8; n++) out.push(JSON.stringify(engine._wander(n)));
    engine.clock.dispose();
    return { out, engine };
  };
  const a = passes(2.2);
  const b = passes(2.2);
  check('in the upper range, the same code played twice wanders differently (passes 2-8)', [2, 3, 4, 5, 6, 7, 8].every((n) => a.out[n] !== b.out[n]) && a.engine.wanderSalt !== 0);
  const lo1 = passes(1.9);
  const lo2 = passes(1.9);
  check('below it, identically, bit for bit, on the stream it always had', lo1.out.join() === lo2.out.join() && lo1.engine.wanderSalt === 0);
  check(`the range starts at ${UNREPEATABLE_FROM}`, UNREPEATABLE_FROM === 2.0);
  // The default amount draws what the goldens hold: no salt.
  const d = passes(1);
  d.engine.salt = 12345;
  check('at the default amount the salt is not heard', d.engine.wanderSalt === 0 && JSON.stringify(d.engine._wander(3)) === d.out[3]);
  // A re-roll keeps its play's salt; a new loop draws another.
  const e = a.engine;
  const salt = e.salt;
  e.load(rerollLayer(e.spec, 'melody'), { keepPosition: true });
  const kept = e.salt === salt;
  e.load(newSpec(7401));
  check('a re-roll keeps the play\'s salt, a new loop draws its own', kept && e.salt !== salt && e.salt !== 0);
}

console.log(failures === 0 ? '\nAll good.\n' : `\n${failures} failing.\n`);
process.exit(failures === 0 ? 0 : 1);
