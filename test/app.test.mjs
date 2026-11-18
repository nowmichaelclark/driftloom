// Tests for what the Android app does for the page (session 28, js/app.js)
// that need no browser and no device: a stub in place of the app's
// `DriftloomApp`, the real media bridge and file saving on it.
// Run with:  node test/app.test.mjs
//
// What they pin down: what is playing goes to the app as `media` (the loop's
// name and its tempo, whether it plays), with the cover once for each loop;
// the lock screen's actions come back as the page's own Play, Pause, Next and
// Previous; a small file (a MIDI file, the backup) goes through the app's
// picker in megabyte pieces and is closed, or aborted when a piece fails;
// and without the app's object nothing here runs at all.

let failures = 0;
function check(label, condition, detail = '') {
  console.log(condition ? `  pass  ${label}` : `  FAIL  ${label} ${detail}`);
  if (!condition) failures++;
}

// Without the app: no link, and the media bridge keeps to the browser's ways.
const none = await import('../js/app.js');
check('no object, no link', none.appBridge() === null && none.appLink() === null);

class App extends EventTarget {
  constructor() {
    super();
    this.sent = [];
    // What the next `save` is answered with, and which write (if any) fails.
    this.open = { ok: true, name: 'a loop.mid', free: 1e9, seekable: true };
    this.failWrite = 0;
  }
  postMessage(text) {
    const m = JSON.parse(text);
    this.sent.push(m);
    if (m.id === undefined) return;
    let r = { ok: true };
    if (m.type === 'save') r = this.open;
    if (m.type === 'save-write' && this.failWrite && this.sent.filter((x) => x.type === 'save-write').length === this.failWrite) r = { ok: false, why: 'space' };
    queueMicrotask(() => this.say({ type: 'reply', id: m.id, ...r }));
  }
  say(m) {
    const e = new Event('message');
    e.data = JSON.stringify(m);
    this.dispatchEvent(e);
  }
}
const app = new App();
globalThis.DriftloomApp = app;
const { appLink, toBase64 } = await import('../js/app.js');
const { MediaBridge } = await import('../js/media.js');

const link = appLink();
check('found, and one link a page', link && appLink() === link);
check('hello first', app.sent[0] && app.sent[0].type === 'hello');
check('base64 of a megabyte matches Buffer', toBase64(new Uint8Array(1 << 20).fill(7)) === Buffer.from(new Uint8Array(1 << 20).fill(7)).toString('base64'));

console.log('\nWhat is playing, for the lock screen');
const ctx = { native: true };
const media = new MediaBridge(ctx);
media.attach(null);
check('native mode, the app\'s end', media.mode === 'native' && media.app === link);
const played = [];
media.setHandlers({ onPlay: () => played.push('play'), onPause: () => played.push('pause'), onNext: () => played.push('next'), onPrev: () => played.push('previous') });
media.setMetadata('Soft Tide', '72 bpm · Driftloom', { seed: 1 });
check('a loop named while paused tells nothing yet', !app.sent.some((m) => m.type === 'media'));
await media.start();
const last = () => app.sent.filter((m) => m.type === 'media').pop();
check('Play tells the app: playing, with the name', last() && last().playing === true && last().name === 'Soft Tide' && last().subtitle === '72 bpm · Driftloom', JSON.stringify(last()));
media.setMetadata('Grey Lantern', '64 bpm · Driftloom', { seed: 2 });
check('a new loop while playing is told at once', last().name === 'Grey Lantern');
media.setPlaybackState(false);
check('Pause tells the app', last().playing === false);
check('the cover is asked for once for each loop (nothing to draw here, so none sent)', !app.sent.some((m) => m.type === 'cover'));

console.log('\nThe buttons come back');
for (const action of ['play', 'pause', 'next', 'previous']) app.say({ type: 'action', action });
check('the page\'s own handlers run, in order', played.join() === 'play,pause,next,previous', played.join());
app.say({ type: 'action', action: 'nonsense' });
check('an unknown action is nothing', played.length === 4);

console.log('\nA small file through the app\'s picker');
globalThis.Blob = globalThis.Blob || (await import('node:buffer')).Blob;
const small = new Blob([new Uint8Array(2.5 * (1 << 20)).fill(1)], { type: 'audio/midi' });
const before = app.sent.length;
const done = await link.saveBlob(small, 'a loop.mid');
const mine = app.sent.slice(before);
check('the picker first, with the size and the type', mine[0].type === 'save' && mine[0].need === small.size && mine[0].mime === 'audio/midi' && mine[0].seek === false, JSON.stringify(mine[0]));
check('three pieces and a close, in order', mine.map((m) => m.type).join() === 'save,save-write,save-write,save-write,save-close', mine.map((m) => m.type).join());
check('the type comes first in each (the app reads a piece on its own thread by it)', mine.every((m) => JSON.stringify(m).startsWith(`{"type":"${m.type}"`)));
check('saved under the name the app gave', done.name === 'a loop.mid');

app.open = { ok: false, cancelled: true };
const cancelled = await link.saveBlob(small, 'x.mid');
check('a closed picker is a cancel, and nothing is written', cancelled.name === null && cancelled.why === 'cancelled');

app.open = { ok: true, name: 'b.mid', seekable: true };
app.failWrite = app.sent.filter((x) => x.type === 'save-write').length + 2;
const mark = app.sent.length;
const failed = await link.saveBlob(small, 'b.mid');
check('a piece that fails aborts the file', failed.name === null && app.sent.slice(mark).pop().type === 'save-abort', app.sent.slice(mark).map((m) => m.type).join());

console.log(failures ? `\n${failures} failed` : '\nAll good.');
process.exit(failures ? 1 : 0);
