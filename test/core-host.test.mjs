// Tests for the core's main-thread side (js/core.js) that need no browser:
// the real module, with a stub for the AudioWorkletNode it builds.
// Run with:  node test/core-host.test.mjs
//
// What they pin down (queue item 32): which engine a URL asks for, why a
// browser cannot run the core, and that a disposed host hears nothing more
// from its node (a processor error queued just before a rebuild must not
// reach the synth that is gone).

let failures = 0;
function check(label, condition, detail = '') {
  console.log(condition ? `  pass  ${label}` : `  FAIL  ${label} ${detail}`);
  if (!condition) failures++;
}

class FakeNode {
  constructor(ctx, name, options) {
    this.options = options;
    this.sent = [];
    this.port = { postMessage: (m) => this.sent.push(m), onmessage: null };
    this.onprocessorerror = null;
    this.disconnected = false;
  }
  disconnect() { this.disconnected = true; }
}
globalThis.AudioWorkletNode = FakeNode;

const { CoreHost, coreUnsupported, newContext, CORE_MAX_RATE, CORE_FALLBACK_RATE } = await import('../js/core.js');

console.log('\nThe engine a URL asks for');
for (const [search, want] of [['', 'rust'], ['?engine=rust', 'rust'], ['?engine=js', 'js'], ['?engine=nonsense', 'rust'], ['?x=1&engine=js', 'js']]) {
  // ENGINE is read once, as the module loads: a fresh copy for each URL.
  globalThis.location = { search };
  const { ENGINE } = await import(`../js/core.js?${encodeURIComponent(search)}`);
  check(`'${search}' asks for ${want}`, ENGINE === want, ENGINE);
}
delete globalThis.location;

console.log('\nWhy a browser cannot run the core');
check('with both, it can', coreUnsupported() === null, String(coreUnsupported()));
{
  const real = globalThis.AudioWorkletNode;
  delete globalThis.AudioWorkletNode;
  check('without AudioWorklet it says so', coreUnsupported() === 'no AudioWorklet in this browser', String(coreUnsupported()));
  globalThis.AudioWorkletNode = real;
  const wasm = globalThis.WebAssembly;
  Object.defineProperty(globalThis, 'WebAssembly', { value: undefined, configurable: true });
  check('without WebAssembly it says so', coreUnsupported() === 'no WebAssembly in this browser', String(coreUnsupported()));
  Object.defineProperty(globalThis, 'WebAssembly', { value: wasm, configurable: true });
}

console.log('\nA disposed host');
{
  const host = new CoreHost({}, { bytes: new Uint8Array(4) }, [{}, {}, {}, {}, {}]);
  let heard = 0;
  host.onfail = () => { heard++; };
  host.node.onprocessorerror({ message: 'unreachable' });
  check('hears a processor error while alive', heard === 1 && host.failed === 'unreachable', `${heard} ${host.failed}`);

  const next = new CoreHost({}, { bytes: new Uint8Array(4) }, [{}, {}, {}, {}, {}]);
  let late = 0;
  next.onfail = () => { late++; };
  next.dispose();
  check('its node is told to dispose and cut', next.node.sent.some((m) => m.type === 'dispose') && next.node.disconnected);
  check('its processor-error handler is cleared', next.node.onprocessorerror === null);
  check('and so is its failure callback', next.onfail === null);
  // The error that was already queued arrives anyway, by whatever path.
  next._fail('late');
  check('a failure after dispose reaches nobody', late === 0);
}

console.log('\nA context faster than the core holds (queue item 45)');
{
  // A browser whose default rate is `rate`; `refuse` makes it turn down an explicit rate.
  const made = [];
  const browser = (rate, refuse = false) => class {
    constructor(options = {}) {
      if (options.sampleRate && refuse) throw new Error('rate not supported');
      this.sampleRate = options.sampleRate ?? rate;
      this.options = options;
      this.closed = false;
      made.push(this);
    }
    close() { this.closed = true; }
  };
  for (const rate of [44100, 48000, 96000]) {
    made.length = 0;
    const ctx = newContext(browser(rate), { latencyHint: 'playback' });
    check(`${rate} Hz is left as it is`, ctx.sampleRate === rate && made.length === 1);
  }
  made.length = 0;
  const ctx = newContext(browser(192000), { latencyHint: 'playback' });
  check(`192 kHz is asked to come up at ${CORE_FALLBACK_RATE} Hz instead, with the other options`,
    ctx.sampleRate === CORE_FALLBACK_RATE && ctx.options.latencyHint === 'playback' && made.length === 2);
  check('and the fast one is closed', made[0].closed && !made[1].closed);
  made.length = 0;
  const stuck = newContext(browser(192000, true));
  check('a browser that will not do that keeps the first (the synth falls back to JavaScript)', stuck.sampleRate === 192000 && !stuck.closed);
  check(`the limit is the web build's, ${CORE_MAX_RATE} Hz`, CORE_MAX_RATE === 96000);
}

// The space recipe's settings as js/pins.js copies them (queue item 85b):
// the core's, by number, name and range (core/src/space.rs, SETTINGS).
{
  const { SPACE_SETTINGS } = await import('../js/pins.js');
  const fs = await import('node:fs');
  const src = fs.readFileSync(new URL('../core/src/space.rs', import.meta.url), 'utf8');
  const body = src.slice(src.indexOf('pub const SETTINGS'), src.indexOf('];', src.indexOf('pub const SETTINGS')));
  const core = [...body.matchAll(/s\("([a-z-]+)", (-?[\d.]+), (-?[\d.]+)\)/g)].map((m) => [m[1], Number(m[2]), Number(m[3])]);
  check('the space settings js/pins.js copies are the core\'s', core.length === 32 && JSON.stringify(core) === JSON.stringify(SPACE_SETTINGS), JSON.stringify(core));
}

console.log(failures === 0 ? '\nAll good.\n' : `\n${failures} failing.\n`);
process.exit(failures === 0 ? 0 : 1);
