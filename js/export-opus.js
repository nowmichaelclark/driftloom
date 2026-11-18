// The small format's encoder (queue item 104c): Opus through the browser's
// own encoder (WebCodecs' AudioEncoder, in the export's Worker), the packets
// written into an Ogg file as they come (js/ogg.js). No library, no server.
//
// Where the browser has no such encoder, `opusWorks` says so and the sheet
// hides the choice: there is no other way to make the small file (a
// fallback encoder in script could fail on a weak phone, and an export must
// never break anything). A browser that offers AAC and not Opus gets the
// choice hidden too: one format, done properly.
//
// The render is js/export-render.js's, in its 'opus' format: the core at
// 48 kHz, a chunk of samples at a time. Each chunk goes into the encoder
// (which queues and encodes off this thread's work), and the packets that
// have come out go through the Ogg writer into bytes for the file, so memory
// stays the same for a minute or three hours: the encoder's queue is held to
// a few chunks (`settle`), and nothing else accumulates.
//
// The file's length is exact to the sample: the encoder is given silence
// after the last sample, as much as its lead-in and no more than a frame
// beyond (so what it has made covers the audio and not a packet too much,
// which Ogg Opus's end trimming allows), and the file's last granule
// position says where the audio ends, so a player cuts the rest. (Measured
// in Chromium: its encoder gives only whole frames, dropping a partial last
// one, so the silence also brings the total to a whole number of frames.)

import { OggOpus, GRANULE_RATE } from './ogg.js';
import { OPUS_RATE, CHUNK } from './export-render.js';

// The bits a second, by lanes: 64 kbit/s mono (the charter's figure: about
// 70 MB for 2.5 hours) and 96 kbit/s stereo, at a constant rate: measured,
// Chromium's variable mode lands near the target but Firefox's runs a fifth
// to a third over it, and a constant rate makes the file's size the same
// in both, so the size the sheet shows and the space it checks are true.
// Opus at these rates is well past what these quiet loops need to sound the
// same (for Mikey's ears).
export const BITRATE = { 1: 64000, 2: 96000 };
// What libopus's lead-in is at 48 kHz when the browser gives no OpusHead
// (6.5 ms): the samples a player drops from the start.
const PRE_SKIP = 312;
// The frame the encoder works in, in samples (20 ms).
const FRAME = 960;
// Chunks the encoder may hold unencoded before the export waits for it.
const QUEUE_MAX = 16;

const configs = (lanes) => [
  // What is asked: music, the best the encoder can do (the export is not in
  // real time), 20 ms frames, plain Opus packets.
  {
    codec: 'opus', sampleRate: OPUS_RATE, numberOfChannels: lanes, bitrate: BITRATE[lanes], bitrateMode: 'constant',
    opus: { application: 'audio', signal: 'music', complexity: 10, frameDuration: 20000, format: 'opus' },
  },
  // A browser that doesn't take the Opus options still encodes Opus.
  { codec: 'opus', sampleRate: OPUS_RATE, numberOfChannels: lanes, bitrate: BITRATE[lanes] },
];

// The configuration this browser will encode `lanes` with, or null.
export async function opusConfig(lanes) {
  if (typeof AudioEncoder === 'undefined' || typeof AudioData === 'undefined' || typeof AudioEncoder.isConfigSupported !== 'function') return null;
  for (const config of configs(lanes)) {
    try {
      const r = await AudioEncoder.isConfigSupported(config);
      if (r && r.supported) return config;
    } catch { /* not this one */ }
  }
  return null;
}

// Whether the small format works here: mono and stereo both.
export async function opusWorks() {
  try {
    return !!((await opusConfig(1)) && (await opusConfig(2)));
  } catch {
    return false;
  }
}

// The pre-skip in an OpusHead the encoder gave (WebCodecs' `description`),
// or null if it gave none.
export function preSkipOf(description) {
  if (!description) return null;
  const b = description instanceof ArrayBuffer ? new Uint8Array(description) : new Uint8Array(description.buffer, description.byteOffset, description.byteLength);
  if (b.length < 19 || String.fromCharCode(...b.subarray(0, 8)) !== 'OpusHead') return null;
  return b[10] | (b[11] << 8);
}

const join = (parts) => {
  if (parts.length === 1) return parts[0];
  const out = new Uint8Array(parts.reduce((n, p) => n + p.length, 0));
  let at = 0;
  for (const p of parts) {
    out.set(p, at);
    at += p.length;
  }
  return out;
};

const nothing = new Uint8Array(0);

export class OpusMaker {
  // `render`: an export-render.js Render in 'opus'. `title`: the loop's name,
  // for the file's tags.
  constructor(render, { title = '' } = {}) {
    if (OPUS_RATE !== GRANULE_RATE) throw new Error('the render and the Ogg writer count at different rates');
    this.render = render;
    this.lanes = render.lanes;
    this.title = title;
    this.encoder = null;
    this.writer = null;
    this.ready = [];
    this.error = null;
    this.fed = 0;
    this.plane = new Float32Array(this.lanes * CHUNK);
    this.wake = null;
    // The file has no bytes before the audio's.
    this.first = null;
  }

  async open() {
    const config = await opusConfig(this.lanes);
    if (!config) throw new Error('this browser has no Opus encoder');
    this.encoder = new AudioEncoder({
      output: (chunk, meta) => {
        try {
          this.packet(chunk, meta);
        } catch (err) {
          this.error = err;
        }
      },
      error: (err) => { this.error = err; },
    });
    this.encoder.addEventListener('dequeue', () => {
      const wake = this.wake;
      this.wake = null;
      if (wake) wake();
    });
    this.encoder.configure(config);
  }

  // A packet from the encoder: the first one makes the file's headers.
  packet(chunk, meta) {
    if (!this.writer) {
      const given = preSkipOf(meta && meta.decoderConfig && meta.decoderConfig.description);
      const comments = this.title ? [`TITLE=${this.title}`] : [];
      this.writer = new OggOpus({ channels: this.lanes, preSkip: given == null ? PRE_SKIP : given, rate: OPUS_RATE, serial: crypto.getRandomValues(new Uint32Array(1))[0], comments });
      this.ready.push(this.writer.header());
    }
    const bytes = new Uint8Array(chunk.byteLength);
    chunk.copyTo(bytes);
    for (const p of this.writer.add(bytes)) this.ready.push(p);
  }

  // `frames` of each lane's samples to the encoder.
  feed(planes, frames) {
    const data = this.plane;
    for (let c = 0; c < this.lanes; c++) data.set(planes[c].subarray(0, frames), c * frames);
    const audio = new AudioData({
      format: 'f32-planar', sampleRate: OPUS_RATE, numberOfFrames: frames, numberOfChannels: this.lanes,
      timestamp: Math.round((this.fed * 1e6) / OPUS_RATE), data: data.subarray(0, this.lanes * frames),
    });
    this.encoder.encode(audio);
    audio.close();
    this.fed += frames;
  }

  // The bytes the packets so far have made (the pages now ready).
  drain() {
    if (this.error) throw this.error;
    if (!this.ready.length) return nothing;
    const out = join(this.ready);
    this.ready = [];
    return out;
  }

  // Render the next chunk into the encoder. Returns the bytes ready to
  // append (possibly none), or null once the audio is all given (then
  // `last` gives the rest).
  step() {
    if (this.error) throw this.error;
    const r = this.render.pcm();
    if (!r) return null;
    if (r.frames) this.feed(r.planes, r.frames);
    return this.drain();
  }

  // Between slices of rendering: let the encoder's outputs through, and wait
  // while it holds more than a few chunks.
  async settle() {
    while (this.encoder && this.encoder.encodeQueueSize > QUEUE_MAX && !this.error) {
      await new Promise((resolve) => {
        this.wake = resolve;
        setTimeout(resolve, 20);
      });
    }
    if (this.error) throw this.error;
  }

  // The audio is all given: the silence after it, the encoder's last
  // packets, and the Ogg file's end. Returns the last bytes.
  async last() {
    const pre = this.writer ? this.writer.preSkip : PRE_SKIP;
    const pad = pre + ((FRAME - ((this.fed + pre) % FRAME)) % FRAME);
    this.feed([new Float32Array(pad), new Float32Array(pad)], pad);
    await this.encoder.flush();
    if (this.error) throw this.error;
    if (!this.writer) throw new Error('the encoder gave no audio');
    for (const p of this.writer.finish(this.render.frames)) this.ready.push(p);
    return this.drain();
  }

  close() {
    try {
      if (this.encoder && this.encoder.state !== 'closed') this.encoder.close();
    } catch { /* closed already */ }
    this.encoder = null;
  }
}
