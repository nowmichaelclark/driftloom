// "Lighter on processor" chosen by the app itself on slow devices (queue
// item 87b; Mikey, 2026-11-02: yes, and the listener can always switch it
// back). The worklet measures how busy the audio thread is where the music
// plays (js/worklet.js, BUSY_WINDOW); this module keeps those measurements,
// holds them to the rule, and hands one quality over to the other at a
// boundary without a cut. Pure apart from `handOver`, so test/quality.test.mjs
// runs it in Node.
//
// The rule, its first values the conservative choice, "for Mikey's ears":
// - On full: if the busiest quarter-second over the last OVER_FOR seconds of
//   playing passes OVER of its deadline, switch to lite at the next pass; if
//   a quarter-second took longer than the audio it made (the render falling
//   behind for its own cost), at the next bar.
// - On lite chosen by the first guess (four cores or fewer): if the busiest
//   quarter-second stays under UNDER for UNDER_FOR seconds of playing, move
//   up to full, at the next pass. Lite costs about 0.6 of full (item 87, on
//   the Moto G Power), so that leaves full well under OVER.
// - Once the app has moved, it never moves again (no back and forth); and a
//   listener's own choice is never changed.

// The share of a block's deadline past which full is too much, over the
// seconds of playing it is read across.
export const OVER = 0.5;
export const OVER_FOR = 20;
// The share under which lite may move up to full, over its seconds.
export const UNDER = 0.15;
export const UNDER_FOR = 60;
// The seconds of a new synth's playing left out at its start: the browser
// compiles the core's code again, faster, once it has run a while, and the
// first blocks read slower than the device is.
export const WARM = 5;
// How long the old quality fades out before the boundary, in seconds (for
// Mikey's ears).
export const FADE = 0.5;
// The least time from the decision to the start of the fade, in seconds.
export const PREP = 0.5;

// What the worklet has said, while playing, as one record a report:
// { at (seconds, the page's clock), seconds (of audio), max, behind }.
// Anything else (paused, a new synth's first seconds, the page asleep) ends
// the run: only an unbroken run of playing counts.
export class Busy {
  constructor() {
    this.runs = [];
    this.played = 0;
  }

  // One report: `playing` says whether the loop played throughout it.
  add(report, playing, at = 0) {
    if (!playing) {
      this.reset();
      return;
    }
    // A report with no window closed in it (the node's first) says nothing.
    if (!report || !(report.seconds > 0)) return;
    this.played += report.seconds;
    if (this.played <= WARM) return;
    this.runs.push({ at, seconds: report.seconds, max: report.max, behind: report.behind || 0 });
    // A minute and a half is more than any rule reads.
    let total = 0;
    for (let i = this.runs.length - 1; i >= 0; i--) {
      total += this.runs[i].seconds;
      if (total > UNDER_FOR + 30) {
        this.runs.splice(0, i);
        break;
      }
    }
  }

  reset() {
    this.runs = [];
    this.played = 0;
  }

  // The busiest quarter-second over the last `seconds` of unbroken playing,
  // or null if there have not been that many.
  busiest(seconds) {
    let total = 0;
    let max = 0;
    for (let i = this.runs.length - 1; i >= 0; i--) {
      total += this.runs[i].seconds;
      max = Math.max(max, this.runs[i].max);
      if (total >= seconds) return max;
    }
    return null;
  }

  // Whether the last report saw the render fall behind.
  behind() {
    const last = this.runs[this.runs.length - 1];
    return !!last && last.behind > 0;
  }
}

const pct = (x) => `${Math.round(x * 100)}%`;

// What to do now, or null. `quality` is the synth's ('full' or 'lite');
// `yours` the listener's own choice (true, false, or undefined for none);
// `auto` the app's last move, if it has made one ({ lite, why }); `guess`
// whether the first guess started it on lite. Returns { lite, why, at }:
// switch to lite (true) or full (false), at the next 'pass' or 'bar'.
export function decide({ quality, yours, auto, guess, busy }) {
  if (yours !== undefined && yours !== null) return null;
  if (auto) return null;
  if (quality === 'full') {
    if (busy.behind()) return { lite: true, why: 'fell behind at full', at: 'bar' };
    const b = busy.busiest(OVER_FOR);
    if (b !== null && b > OVER) return { lite: true, why: `busiest ${pct(b)} at full`, at: 'pass' };
    return null;
  }
  if (quality === 'lite' && guess) {
    const b = busy.busiest(UNDER_FOR);
    if (b !== null && b < UNDER) return { lite: false, why: `busiest ${pct(b)} on lite`, at: 'pass' };
  }
  return null;
}

// Where "lighter on processor" comes from, for Diagnostics and the switch's
// line: the listener's choice, the app's move (and why), or the first guess.
export function source({ yours, auto, cores }) {
  if (yours !== undefined && yours !== null) return 'yours';
  if (auto) return `auto (${auto.why})`;
  return `guess (${cores} cores)`;
}

// The first boundary of `unit` steps (a pass: the loop's length; a bar)
// that falls at or after `after` seconds, on `engine`'s clock: the time of a
// step whose number from the loop's top is a multiple of `unit`.
export function boundary(engine, unit, after) {
  const dur = engine.stepDur;
  if (!(unit >= 1) || !(dur > 0)) return null;
  let step = engine.step;
  let t = engine.nextStepTime;
  // The next step to decide, then forward to the boundary; or, if the
  // decided steps run past `after`, back to the first boundary after it.
  const toNext = (unit - (step % unit)) % unit;
  t += toNext * dur;
  step += toNext;
  while (t - unit * dur >= after) t -= unit * dur;
  while (t < after) t += unit * dur;
  return t;
}

// The old quality's part of a switch (queue item 87b): `old` ({ engine,
// synth }) decides nothing from `at` on, drops what it handed over for `at`
// or later, and fades out over `fade` seconds to it. The caller then lets it
// go and starts the new quality, which takes the loop from its top as soon as
// it can: the new synth is built only once the old one is silent and gone,
// because building it (its worklet compiles the core) works on the very
// audio thread the old one plays on, and two cores at once is what a slow
// device can least afford. So the switch is a fade, a breath and the loop
// again, never a cut.
export function handOff(old, at, fade = FADE) {
  old.engine.until = at;
  old.synth.cancelFrom(at);
  old.synth.fadeOut(Math.max(old.synth.ctx.currentTime, at - fade), at);
}

// Call `fn` once the context's clock reaches `at`: an alarm on the audio
// clock (a silent source's end), which a hidden page's timers would delay.
export function alarm(ctx, at, fn) {
  const src = ctx.createConstantSource();
  const mute = ctx.createGain();
  mute.gain.value = 0;
  src.connect(mute);
  mute.connect(ctx.destination);
  src.onended = () => {
    try { src.disconnect(); mute.disconnect(); } catch { /* already gone */ }
    fn();
  };
  src.start();
  src.stop(Math.max(ctx.currentTime, at));
  return src;
}
