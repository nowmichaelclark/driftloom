// The speed test's worker (queue item 87, speed.html): runs js/speed.js off
// the page's thread, so drawing the page never lands in a timed block. It
// is handed the player's module bytes and a plan (which loops, which ways,
// how long, how many runs) and posts each result as it has it.

import { measure, line, tick, warm } from './speed.js';

const now = () => performance.now();

self.onmessage = async (e) => {
  const { bytes, rate, plan } = e.data;
  try {
    const module = await WebAssembly.compile(bytes);
    self.postMessage({ type: 'tick', us: tick(now) });
    self.postMessage({ type: 'warm' });
    warm(module, { rate, now });
    for (let i = 0; i < plan.length; i++) {
      const p = plan[i];
      self.postMessage({ type: 'start', i, n: plan.length, loop: p.loop, way: p.way, seconds: p.seconds, runs: p.runs });
      const r = measure(module, { seed: p.loop.seed, way: p.way, rate, seconds: p.seconds, runs: p.runs, now });
      self.postMessage({ type: 'result', i, loop: p.loop, way: p.way, r, text: line(p.loop, p.way, r) });
    }
    self.postMessage({ type: 'done' });
  } catch (err) {
    self.postMessage({ type: 'error', message: String(err && err.message || err) });
  }
};
