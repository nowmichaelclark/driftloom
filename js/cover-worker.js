// Covers drawn off the main thread (queue item 41).
//
// A cover is a few hundred ms of arithmetic on a slow phone -- noise fields,
// masks, auto-levels and grain over every pixel -- and on the main thread
// that held the clock back while the music needed it: the Pixel 10a's
// Library and Save / Share stalled the page for 1.3-1.8 s. Here it runs on
// a thread of its own, on an OffscreenCanvas made for reading
// (willReadFrequently: a CPU canvas, so no pixel is read back from a GPU),
// and the finished picture goes to the page as an ImageBitmap, moved, not
// copied. The page only pastes it in.
//
// Messages: { id, spec, size } in; { id, bitmap } or { id, error } out, in
// the order they came.

import { drawCover } from './cover.js';

self.onmessage = (e) => {
  const { id, spec, size } = e.data;
  try {
    const canvas = new OffscreenCanvas(size, size);
    drawCover(canvas, spec, size);
    const bitmap = canvas.transferToImageBitmap();
    self.postMessage({ id, bitmap }, [bitmap]);
  } catch (err) {
    self.postMessage({ id, error: String((err && err.message) || err) });
  }
};
