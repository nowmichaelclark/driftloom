// Covers, as pictures to paste in (queue item 41).
//
// Every cover the page shows -- the playing loop's on the card, an album's
// on the card and in the lists -- comes from here, drawn by
// js/cover-worker.js on a thread of its own, so no cover ever holds the
// main thread (and the music's clock) for the hundreds of ms it costs on a
// slow phone. What comes back is an ImageBitmap: the page draws it into
// its canvas, and never reads a pixel back.
//
// A browser without module workers or OffscreenCanvas (Safari before 16.4)
// draws on the main thread as before, a task of its own per cover, on a
// canvas made for reading.
//
// Kept, the last few dozen, by what they show and their size: going back to
// a cover costs nothing.

import { drawCover } from './cover.js';

const KEEP = 48;
const kept = new Map();
let worker = null;
let workerFailed = typeof Worker === 'undefined' || typeof OffscreenCanvas === 'undefined';
let nextId = 0;
const waiting = new Map();

function startWorker() {
  if (worker || workerFailed) return worker;
  try {
    worker = new Worker(new URL('./cover-worker.js', import.meta.url), { type: 'module' });
  } catch {
    workerFailed = true;
    return null;
  }
  worker.onmessage = (e) => {
    const { id, bitmap, error } = e.data;
    const w = waiting.get(id);
    if (!w) return;
    waiting.delete(id);
    if (bitmap) w.resolve(bitmap);
    else w.reject(new Error(error || 'no cover'));
  };
  // The worker could not load (no module workers, say): everything waiting
  // and everything after is drawn on the main thread.
  worker.onerror = (e) => {
    if (e && e.preventDefault) e.preventDefault();
    workerFailed = true;
    worker = null;
    const all = [...waiting.values()];
    waiting.clear();
    for (const w of all) w.fallback();
  };
  return worker;
}

// On the main thread, a task of its own, when there is no worker.
function drawHere(spec, size) {
  return new Promise((resolve, reject) => {
    setTimeout(() => {
      try {
        const canvas = document.createElement('canvas');
        drawCover(canvas, spec, size);
        resolve(canvas);
      } catch (err) {
        reject(err);
      }
    }, 0);
  });
}

function request(spec, size) {
  const w = startWorker();
  if (!w) return drawHere(spec, size);
  return new Promise((resolve, reject) => {
    const id = ++nextId;
    waiting.set(id, {
      resolve,
      reject,
      fallback: () => drawHere(spec, size).then(resolve, reject),
    });
    w.postMessage({ id, spec, size });
  });
}

// The cover of `spec` at `size` px: a promise of something to drawImage
// (an ImageBitmap, or a canvas where there is no worker).
export function coverImage(spec, size = 320) {
  const key = `${size}|${JSON.stringify(spec)}`;
  let p = kept.get(key);
  if (p) {
    // Most recently used last.
    kept.delete(key);
    kept.set(key, p);
    return p;
  }
  p = request(spec, size);
  // A cover that failed is not kept: asked again, it is drawn again.
  p.catch(() => { if (kept.get(key) === p) kept.delete(key); });
  kept.set(key, p);
  while (kept.size > KEEP) kept.delete(kept.keys().next().value);
  return p;
}

// The picture into `canvas`, scaled to fill it. Drawing a bitmap in reads
// nothing back.
export function paint(canvas, image) {
  canvas.getContext('2d').drawImage(image, 0, 0, canvas.width, canvas.height);
}
