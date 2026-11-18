// What the Android app does for the page that a browser does itself (session
// 28, queue item 107d): the app puts an object in the page, `DriftloomApp`
// (android/app/src/main/kotlin/app/loops/AppBridge.kt), a second bridge beside
// the core's (js/native.js). Plain JSON text both ways:
//
//   to the app    media { playing, name, subtitle }   what is playing, for
//                                  the service, the notification and the
//                                  lock screen
//                 cover { data }   the loop's own cover (a base64 PNG)
//                 export { on, name, percent }   a file is being made
//                 save { id, name, mime, need, seek }   Android's own picker
//                                  for a new file; save-write, save-header,
//                                  save-close, save-abort { id, data } (base64);
//                                  save-view, save-share
//   from the app  action { action }  play, pause, next or previous, from the
//                                  lock screen, the notification, a headset,
//                                  a call (the page does what its own button
//                                  does)
//                 reply { id, ok, ... }  to each request that has an id
//
// In a browser there is no such object and none of this runs: the page uses
// the browser's media session and its own downloads. This file imports
// nothing (ui.js reads it), and loads the cover's drawing only when asked.

// The app's bridge, or null in a browser (and in an app whose WebView is too
// old to carry it).
export function appBridge() {
  const b = globalThis.DriftloomApp;
  return b && typeof b.postMessage === 'function' ? b : null;
}

// One link a page: the object's end of the conversation.
let link = null;
export function appLink() {
  if (link) return link;
  const bridge = appBridge();
  if (!bridge) return null;
  link = new AppLink(bridge);
  return link;
}

// The bytes as base64, a block at a time (a binary string of a megabyte at
// once would be refused by some engines).
export function toBase64(bytes) {
  let text = '';
  for (let i = 0; i < bytes.length; i += 0x8000) {
    text += String.fromCharCode.apply(null, bytes.subarray(i, i + 0x8000));
  }
  return btoa(text);
}

class AppLink {
  constructor(bridge) {
    this.bridge = bridge;
    this.next = 0;
    this.waiting = new Map();
    // What the lock screen's buttons do: set by the page (js/media.js).
    this.handlers = {};
    this.coverFor = '';
    this.holding = false;
    this.free = null;
    const hear = (e) => this._hear(e && e.data !== undefined ? e.data : e);
    if (typeof bridge.addEventListener === 'function') bridge.addEventListener('message', hear);
    else bridge.onmessage = hear;
    // The first message is what gives the app its way back to the page.
    this.send({ type: 'hello' });
  }

  // `type` is first in the text: the app reads a file's bytes on another
  // thread by what the text starts with.
  send(m) {
    try {
      this.bridge.postMessage(JSON.stringify(m));
    } catch { /* the app is going */ }
  }

  // A request the app answers: its reply, or null if it never does.
  request(m) {
    return new Promise((resolve) => {
      const id = ++this.next;
      this.waiting.set(id, resolve);
      this.send({ type: m.type, id, ...m });
    });
  }

  _hear(text) {
    let m;
    try {
      m = typeof text === 'string' ? JSON.parse(text) : text;
    } catch {
      return;
    }
    if (!m) return;
    if (m.type === 'reply') {
      const resolve = this.waiting.get(m.id);
      this.waiting.delete(m.id);
      if (resolve) resolve(m);
    } else if (m.type === 'action') {
      const fn = { play: 'onPlay', pause: 'onPause', next: 'onNext', previous: 'onPrev' }[m.action];
      // In the logs of a test build (adb logcat), so a button that never reached the page can be told from one
      // the page ignored.
      console.info(`DriftloomApp action ${m.action}`);
      if (fn && this.handlers[fn]) this.handlers[fn]();
    }
  }

  // What is playing. While it plays the page holds a Web Lock: Chromium
  // does not put a page that holds one on the once-a-minute timer it gives
  // pages hidden for long, and the page is hidden with the screen off.
  media(playing, name, subtitle) {
    this.send({ type: 'media', playing: !!playing, name: name || '', subtitle: subtitle || '' });
    if (playing) this._hold();
    else this._release();
  }

  _hold() {
    if (this.holding || !navigator.locks) return;
    this.holding = true;
    try {
      // Granted a task later; released by Pause (or at once, if Pause came first).
      navigator.locks.request('driftloom-playing', () => (this.holding ? new Promise((resolve) => { this.free = resolve; }) : undefined));
    } catch {
      this.holding = false;
    }
  }

  _release() {
    this.holding = false;
    const free = this.free;
    this.free = null;
    if (free) free();
  }

  // The loop's own cover for the lock screen and the notification: the one
  // the card draws (320 px, kept by js/covers.js, so no new drawing), as a
  // PNG. Sent once for each loop.
  async cover(spec) {
    if (!spec) return;
    const key = JSON.stringify(spec);
    if (key === this.coverFor) return;
    this.coverFor = key;
    try {
      const { coverImage } = await import('./covers.js');
      const image = await coverImage(spec, 320);
      if (this.coverFor !== key) return;
      const canvas = document.createElement('canvas');
      canvas.width = 320;
      canvas.height = 320;
      canvas.getContext('2d').drawImage(image, 0, 0, 320, 320);
      const blob = await new Promise((resolve) => canvas.toBlob(resolve, 'image/png'));
      if (!blob || this.coverFor !== key) return;
      this.send({ type: 'cover', data: toBase64(new Uint8Array(await blob.arrayBuffer())) });
    } catch {
      // The notification keeps the last cover, or none.
      if (this.coverFor === key) this.coverFor = '';
    }
  }

  // A file being made: for the notification's bar and the wake lock.
  exporting(on, name, percent) {
    this.send({ type: 'export', on: !!on, name: name || '', percent: Math.max(0, Math.min(100, Math.round(percent || 0))) });
  }

  // A whole Blob saved through Android's picker: for the small files (a MIDI
  // file, the backup). Resolves to the name it was saved as, or null (and
  // says why in `why`) if the listener closed the picker or the save failed.
  async saveBlob(blob, name, mime) {
    const opened = await this.request({ type: 'save', name, mime: mime || blob.type || 'application/octet-stream', need: blob.size, seek: false });
    if (!opened || !opened.ok) return { name: null, why: opened && opened.cancelled ? 'cancelled' : (opened && opened.why) || 'failed' };
    const CHUNK = 1 << 20;
    try {
      for (let at = 0; at < blob.size; at += CHUNK) {
        const bytes = new Uint8Array(await blob.slice(at, at + CHUNK).arrayBuffer());
        const r = await this.request({ type: 'save-write', data: toBase64(bytes) });
        if (!r || !r.ok) throw new Error((r && r.why) || 'write');
      }
      const done = await this.request({ type: 'save-close' });
      if (!done || !done.ok) throw new Error('close');
    } catch (err) {
      await this.request({ type: 'save-abort' });
      return { name: null, why: String((err && err.message) || err) };
    }
    return { name: opened.name };
  }
}
