// An Ogg file for Opus audio (queue item 104c), written a page at a time.
//
// The export's small format: the browser's own Opus encoder (WebCodecs'
// AudioEncoder, js/export-opus.js) gives packets, and this puts them in an
// Ogg file (RFC 3533 for the container, RFC 7845 for Opus in it) with no
// library. Pure: no DOM, no Worker, no file; it takes packets and gives
// bytes, so Node tests it (test/ogg.test.mjs) and the Worker writes each
// page as it comes, straight to the file.
//
// The file is the OpusHead page, the OpusTags page, then audio pages, the
// last marked end of stream. A page holds whole packets (up to a second of
// audio or 255 segments, whichever first). Its granule position is the
// number of 48 kHz samples the decoder has made once it has decoded the
// page's last packet, counted from the first packet, so the pre-skip (the
// samples at the start that are the encoder's lead-in, not played) is in
// the count already: what plays is the granule less the pre-skip. The last
// page's granule is where the audio really ends (`finish(total)`: the
// pre-skip and the samples the file holds), which may be short of the
// packets' span (the encoder pads its last frame): a player cuts the rest.
// That is how a file's length is exact to the sample, and how a player seeks
// and shows the true length.

// Opus always counts its samples at 48 kHz in a granule, whatever the rate
// the audio was made at.
export const GRANULE_RATE = 48000;
// A page's most segments, and the audio a page holds before it is closed.
const SEGMENTS_MAX = 255;
const PAGE_SAMPLES = GRANULE_RATE;

// Ogg's CRC-32: polynomial 0x04C11DB7, not reflected, no initial or final
// xor, over the whole page with its checksum field zero.
const TABLE = (() => {
  const t = new Uint32Array(256);
  for (let i = 0; i < 256; i++) {
    let r = i << 24;
    for (let k = 0; k < 8; k++) r = r & 0x80000000 ? (r << 1) ^ 0x04c11db7 : r << 1;
    t[i] = r >>> 0;
  }
  return t;
})();

export function crc32(bytes, crc = 0) {
  let c = crc >>> 0;
  for (let i = 0; i < bytes.length; i++) c = ((c << 8) ^ TABLE[((c >>> 24) ^ bytes[i]) & 0xff]) >>> 0;
  return c >>> 0;
}

// The samples (at 48 kHz) one Opus packet holds, from its first byte and,
// for a packet of frames of differing codes, its second (RFC 6716 section
// 3.1). Null for a packet that is not one.
export function packetSamples(packet) {
  if (!packet || packet.length < 1) return null;
  const toc = packet[0];
  const config = toc >> 3;
  // The frame's length in samples at 48 kHz, by configuration.
  let frame;
  if (config < 12) frame = [480, 960, 1920, 2880][config & 3]; // SILK: 10, 20, 40, 60 ms
  else if (config < 16) frame = [480, 960][config & 1]; // hybrid: 10, 20 ms
  else frame = [120, 240, 480, 960][config & 3]; // CELT: 2.5, 5, 10, 20 ms
  const code = toc & 3;
  let frames;
  if (code === 0) frames = 1;
  else if (code === 1 || code === 2) frames = 2;
  else {
    if (packet.length < 2) return null;
    frames = packet[1] & 0x3f;
    if (frames < 1) return null;
  }
  const samples = frame * frames;
  // 120 ms is the most a packet may hold.
  return samples <= 5760 ? samples : null;
}

const encoder = new TextEncoder();

function head({ channels, preSkip, rate }) {
  const b = new Uint8Array(19);
  b.set(encoder.encode('OpusHead'), 0);
  const v = new DataView(b.buffer);
  b[8] = 1; // version
  b[9] = channels;
  v.setUint16(10, preSkip, true);
  v.setUint32(12, rate, true); // the rate the audio was made at, for information
  v.setInt16(16, 0, true); // output gain
  b[18] = 0; // channel mapping family 0: mono or stereo
  return b;
}

function tags(vendor, comments) {
  const v = encoder.encode(vendor);
  const c = comments.map((s) => encoder.encode(s));
  const b = new Uint8Array(8 + 4 + v.length + 4 + c.reduce((n, x) => n + 4 + x.length, 0));
  const dv = new DataView(b.buffer);
  b.set(encoder.encode('OpusTags'), 0);
  let at = 8;
  dv.setUint32(at, v.length, true);
  b.set(v, at + 4);
  at += 4 + v.length;
  dv.setUint32(at, c.length, true);
  at += 4;
  for (const x of c) {
    dv.setUint32(at, x.length, true);
    b.set(x, at + 4);
    at += 4 + x.length;
  }
  return b;
}

// One page: its flags (1 continued, 2 first of the stream, 4 last), granule
// (a number: the sums here stay far under 2^53), serial, sequence number and
// the packets it holds, whole.
export function page(flags, granule, serial, sequence, packets) {
  const segments = [];
  let data = 0;
  for (const p of packets) {
    let n = p.length;
    for (; n >= 255; n -= 255) segments.push(255);
    segments.push(n);
    data += p.length;
  }
  if (segments.length > SEGMENTS_MAX) throw new Error('too many segments for one page');
  const out = new Uint8Array(27 + segments.length + data);
  const v = new DataView(out.buffer);
  out.set(encoder.encode('OggS'), 0);
  out[4] = 0; // version
  out[5] = flags;
  v.setUint32(6, granule % 2 ** 32, true);
  v.setUint32(10, Math.floor(granule / 2 ** 32), true);
  v.setUint32(14, serial >>> 0, true);
  v.setUint32(18, sequence >>> 0, true);
  // 22: the checksum, below
  out[26] = segments.length;
  out.set(segments, 27);
  let at = 27 + segments.length;
  for (const p of packets) {
    out.set(p, at);
    at += p.length;
  }
  v.setUint32(22, crc32(out), true);
  return out;
}

export class OggOpus {
  // { channels: 1 | 2, preSkip: samples at 48 kHz the decoder drops from the
  // start (the encoder's lead-in), rate: the rate of the audio made (for
  // information), serial, vendor, comments: ['TITLE=...'] }
  constructor({ channels, preSkip, rate = GRANULE_RATE, serial = 0, vendor = 'WebCodecs Opus', comments = [] }) {
    if (!(channels === 1 || channels === 2)) throw new Error('Opus in Ogg here is mono or stereo');
    if (!(preSkip >= 0 && preSkip <= 0xffff)) throw new Error('the pre-skip is out of range');
    this.preSkip = preSkip;
    this.serial = serial >>> 0;
    this.sequence = 0;
    // The samples of every packet given so far, the packets and segments of
    // the page in progress, and the audio it holds.
    this.samples = 0;
    this.packets = [];
    this.sizes = [];
    this.segments = 0;
    this.pending = 0;
    // Full pages not yet written. The newest is held back until another is
    // full or the file ends: the end may cut the audio short of the
    // packets' span, and a granule past the end must never be written.
    this.ready = [];
    this.held = null;
    this.finished = false;
    this.first = [
      page(2, 0, this.serial, this.sequence++, [head({ channels, preSkip, rate })]),
      page(0, 0, this.serial, this.sequence++, [tags(vendor, comments)]),
    ];
  }

  // The two header pages, as one run of bytes: write them first.
  header() {
    const [a, b] = this.first;
    const out = new Uint8Array(a.length + b.length);
    out.set(a, 0);
    out.set(b, a.length);
    return out;
  }

  // One packet from the encoder; `samples` (at 48 kHz) from its own header if
  // not given. Returns the pages now ready to write, in order (often none).
  add(packet, samples = packetSamples(packet)) {
    if (this.finished) throw new Error('the file is finished');
    if (!samples) throw new Error('not an Opus packet');
    const lace = Math.floor(packet.length / 255) + 1;
    if (this.packets.length && this.segments + lace > SEGMENTS_MAX) this.seal();
    this.packets.push(packet);
    this.sizes.push(samples);
    this.segments += lace;
    this.pending += samples;
    this.samples += samples;
    if (this.pending >= PAGE_SAMPLES) this.seal();
    return this.take(0, Infinity);
  }

  // The page in progress is full: it becomes the held one, and the one held
  // before it is ready.
  seal() {
    if (!this.packets.length) return;
    if (this.held) this.ready.push(this.held);
    this.held = { packets: this.packets, sizes: this.sizes, granule: this.samples };
    this.packets = [];
    this.sizes = [];
    this.segments = 0;
    this.pending = 0;
  }

  // The ready pages as bytes, their granules no further than `end`.
  take(flags, end) {
    const out = this.ready.map((p) => page(flags, Math.min(p.granule, end), this.serial, this.sequence++, p.packets));
    this.ready = [];
    return out;
  }

  // The end: `total` is the samples (at 48 kHz) the audio really holds, not
  // counting the pre-skip. The packets may span more (the encoder pads its
  // last frame, and a lead-in is flushed after the audio): packets that lie
  // wholly past the end are left out (a player may be asked to trim less
  // than one packet), and the last page's granule says where the audio ends.
  // Returns the last pages, the final one marked end of stream.
  finish(total) {
    if (this.finished) throw new Error('the file is finished');
    this.finished = true;
    this.seal();
    if (this.held) this.ready.push(this.held);
    this.held = null;
    const end = Math.min(this.preSkip + total, this.samples);
    const last = this.ready;
    this.ready = [];
    let span = this.samples;
    while (last.length) {
      const p = last[last.length - 1];
      while (p.packets.length && span - p.sizes[p.sizes.length - 1] >= end) {
        span -= p.sizes.pop();
        p.packets.pop();
      }
      if (p.packets.length) break;
      last.pop();
    }
    // No audio at all: an empty file is still a file.
    if (!last.length) return [page(4, 0, this.serial, this.sequence++, [])];
    return last.map((p, i) => page(i === last.length - 1 ? 4 : 0, Math.min(p.granule, end), this.serial, this.sequence++, p.packets));
  }
}
