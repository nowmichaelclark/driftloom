// The audio export's render (queue item 104b): a loop or a piece made into
// a FLAC file by its own instance of the core, a chunk at a time.
//
// Shared by the export's Worker (js/export-worker.js), which writes each
// chunk's bytes straight to the file, and by tools/export.mjs, which checks
// in Node that the file is the core's render (it decodes and nulls against
// a straight render of the same loop) and that memory stays flat. It touches
// nothing but the instance it is given: no DOM, no file, no timer.
//
// What is exported is what plays: the record's seed, request and edits as
// the page asks the core that plays them (`askOf`, the same calls in the
// same order as js/worklet.js's `ask`), its mutes, the wander and the play's
// salt, and the track length (session 30c: a time to the second, with its
// fade in and fade out, as the Track length view set it; or a finite piece
// whole). The track is played to its end as the app plays it, then the core
// stops deciding and what it has started rings out (the tail) before the
// file ends: with the fade out, from silence.
//
// Always at full quality (the six-comb mix), whatever "lighter on processor"
// says: the export is not in real time, so it can afford the better mix (for
// Mikey's ears). Stereo unless the Mono switch is on.
//
// Two formats (queue item 104b and 104c), the job's `format`: 'flac' (the
// default, lossless: 44.1 kHz, 16-bit, the core's own FLAC writer; `step`
// gives the file's next bytes) and 'opus' (the small one: the core renders at
// 48 kHz, the rate Opus works at, so nothing is resampled, and `pcm` gives
// each chunk's samples for the browser's encoder, js/export-opus.js). The
// track, its end and its tail are the same in both.

// The lossless file's format: CD's rate and depth, which every player reads.
export const RATE = 44100;
export const BITS = 16;
// The small format's rate: what Opus takes, and what a browser's own
// encoder is given.
export const OPUS_RATE = 48000;
// Frames a chunk: the most `dl_play_render` makes in stereo in one call.
// Well under the player's lookahead (a quarter of a second), so the track's
// end is always heard of before a chunk renders past it.
export const CHUNK = 2048;
// The tail: what was started rings out until it has been quiet (under QUIET,
// about -72 dBFS) for QUIET_FOR seconds, and for at least TAIL_MIN; a tail
// still sounding at TAIL_MAX - FADE fades to nothing over FADE (a drone held
// past the end). For Mikey's ears.
export const TAIL_MIN = 1;
export const TAIL_MAX = 12;
export const FADE = 2;
export const QUIET = 10 ** (-72 / 20);
export const QUIET_FOR = 0.5;
// A track that somehow never ends is ended anyway this far past what was
// expected (with the tail's fade): a file never grows without bound.
const OVERRUN = 1.25;
const OVERRUN_MIN = 60;

// The player's event kinds (`Kind::ALL`): the track's end is 6.
const END = 6;
const EVENTS_MAX = 256;

const text = (x, at, length) => String.fromCharCode(...new Uint8Array(x.memory.buffer, at, length));

// Hand the core what a loop asks for (js/worklet.js's `ask`, call for call).
// Returns how many parts it refused.
export function ask(x, a) {
  let refused = 0;
  const take = (ok) => { if (!ok) refused++; };
  if (!a) return 0;
  for (const [i, w] of a.vibes || []) take(w == null ? x.dl_play_vibe(i) : x.dl_play_weight(i, w));
  if (a.tempo) take(x.dl_play_ask_tempo(a.tempo[0], a.tempo[1]));
  for (const [i, v] of a.pins || []) take(x.dl_play_pin(i, v));
  if (a.theme != null) x.dl_play_theme(a.theme >>> 0);
  if (a.length != null) take(x.dl_play_length(a.length === 'endless' ? Infinity : a.length));
  if (a.keys) take(x.dl_play_keys(a.keys));
  if (a.scales) take(x.dl_play_scales(a.scales));
  if (a.styles) take(x.dl_play_styles(a.styles));
  (a.voices || []).forEach((mask, layer) => { if (mask) take(x.dl_play_voices(layer, mask)); });
  (a.layers || []).forEach((state, layer) => { if (state) take(x.dl_play_layer(layer, state)); });
  for (const [layer, seed, rolled] of a.seeds || []) take(x.dl_play_seed(layer, seed >>> 0, rolled ? 1 : 0));
  (a.tone || []).forEach((v, which) => take(x.dl_play_edit_tone(which, v)));
  if (a.bpm != null) take(x.dl_play_edit_bpm(a.bpm));
  return refused;
}

// Start an instance as the export plays: `job` is
//   { seed, ask, mutes: [5 booleans], wander, salt, layout: 1 | 2,
//     track: [kind, count], format: 'flac' | 'opus' }
// (the core's kinds, `Track::of`: 1 whole passes, 2 bars, 3 seconds, with
// the fades' bits; never on and on)
// in the order the page starts the core that plays (js/player.js's
// `_begin`: the wander and its salt, the loop, then the mutes and the
// track), and the loop made at once (`dl_play_ready`: an export has no
// deadline). Returns { refused, seconds }: the parts of the request the core
// refused, and how long the track plays, as the core counts it.
// The rate a job is rendered at.
export const rateOf = (job) => (job.format === 'opus' ? OPUS_RATE : RATE);

export function begin(x, job) {
  const rate = rateOf(job);
  x.dl_init(rate);
  if (!x.dl_quality(1)) throw new Error(`no mix at ${rate} Hz`);
  if (job.layout === 2 && !x.dl_layout(2)) throw new Error('no stereo render');
  x.dl_play_attach(1);
  x.dl_play_wander(job.wander || 0);
  x.dl_play_wander_salt((job.salt || 0) >>> 0);
  const refused = ask(x, job.ask);
  x.dl_play_load(job.seed >>> 0, 0);
  (job.mutes || []).forEach((m, i) => x.dl_play_mute(i, m ? 1 : 0));
  const [kind, count] = job.track;
  if (!((kind & 3) >= 1) || !x.dl_play_track(kind >>> 0, count >>> 0)) throw new Error(`no such track: ${job.track}`);
  x.dl_play_ready();
  // A time's track ends on its second on the core's clock; a count of
  // passes or bars on its step.
  const at = x.dl_play_track_at();
  const end = x.dl_play_track_end();
  const about = text(x, x.dl_play_about_at(), x.dl_play_describe());
  const bpm = Number((/^bpm=(.*)$/m.exec(about) || [])[1]);
  const seconds = at >= 0 ? at : end >= 0 && bpm > 0 ? end * (60 / bpm / 4) : null;
  return { refused, seconds };
}

// The export's render, a chunk at a time. In 'flac' it goes into the core's
// FLAC encoder: the bytes each step gives are the file's next bytes, written
// by the caller before the next step (they sit in the instance's memory). In
// 'opus' `pcm()` gives the chunk's samples instead, for the browser's
// encoder. Memory is the instance's own and a few views: the same for a
// minute or three hours.
export class Render {
  constructor(x, job) {
    this.x = x;
    this.format = job.format === 'opus' ? 'opus' : 'flac';
    this.rate = rateOf(job);
    const { refused, seconds } = begin(x, job);
    this.refused = refused;
    // How long the track plays, as the core counts it; the file is that and
    // its tail.
    this.seconds = seconds;
    this.lanes = x.dl_outputs();
    if (this.format === 'flac') {
      const head = x.dl_flac_start(RATE, this.lanes, BITS);
      if (head < 0) throw new Error('the FLAC writer refused the format');
      this.first = this.bytes(head);
    } else {
      this.first = null;
    }
    this.frames = 0;
    // The frame the track ends at (from its End event), when the core
    // stopped deciding, and how long the output has been quiet since.
    this.endFrame = null;
    this.endTime = null;
    this.stopFrame = null;
    this.quietFrames = 0;
    this.fadeFrom = null;
    this.finished = false;
    this.owed = false;
    this.lateEnd = 0;
    const most = Math.max(this.seconds == null ? 0 : this.seconds * OVERRUN, (this.seconds || 0) + OVERRUN_MIN);
    this.limit = Math.ceil(most * this.rate);
    this.views = null;
  }

  // The bytes the encoder's last call gave, as a view (valid until the next
  // call).
  bytes(n) {
    if (n < 0) throw new Error('the FLAC writer refused a chunk');
    return new Uint8Array(this.x.memory.buffer, this.x.dl_flac_out(), n);
  }

  // The frames the file will hold, as far as is known: the track and the
  // longest tail. For progress.
  get expectedFrames() {
    if (this.endFrame != null) return this.endFrame + TAIL_MAX * this.rate;
    return this.seconds == null ? null : Math.ceil((this.seconds + TAIL_MAX) * this.rate);
  }

  // The track's end, heard as the core decides it (an End event, up to the
  // player's lookahead before it sounds): notes from there on are dropped,
  // the core stops deciding, and what was started rings out.
  watch() {
    const x = this.x;
    const n = x.dl_play_events();
    if (!n || this.endFrame != null) return;
    const f = new Float64Array(x.memory.buffer, x.dl_play_event_at(), 4 * Math.min(n, EVENTS_MAX));
    for (let i = 0; i < Math.min(n, EVENTS_MAX); i++) {
      if (f[4 * i + 1] !== END) continue;
      this.stopAt(f[4 * i]);
      return;
    }
  }

  stopAt(time) {
    const x = this.x;
    x.dl_cancel_from(time);
    x.dl_play_stop();
    // The time as the core gave it (a straight render stops the same way),
    // and the frame it falls on.
    this.endTime = time;
    this.endFrame = Math.round(time * this.rate);
    // The frames rendered when it stopped (before the next chunk).
    this.stopFrame = this.frames;
    // Should never be: the end heard after it was rendered.
    if (this.endFrame < this.frames) this.lateEnd++;
  }

  // Render the next chunk, with the tail's rules, into `planes` (a view a
  // lane, valid until the next call). Returns how many frames of it are in
  // the file, or -1 once the file is finished. In 'flac' the planes are the
  // FLAC writer's input; in 'opus' they are the core's own output.
  chunk() {
    // The track's end heard in the last chunk's events (in 'opus', the
    // samples were the core's own buffer until now).
    if (this.owed) {
      this.owed = false;
      if (!this.finished) this.watch();
    }
    if (this.finished) return -1;
    const x = this.x;
    const rate = this.rate;
    // Past the limit with no end heard: end it here, with the tail's fade.
    if (this.endFrame == null && this.frames >= this.limit) this.stopAt(this.frames / rate);
    const at = x.dl_play_render(CHUNK);
    if (!at) throw new Error('the core would not render');
    const lanes = this.lanes;
    // Views made once: nothing allocates after the core starts, so its
    // memory never grows and the buffers never move.
    if (!this.views || this.views.at !== at) {
      const buffer = x.memory.buffer;
      const runs = [0, 1].map((c) => new Float32Array(buffer, at + c * CHUNK * 4, CHUNK));
      if (this.format === 'flac') {
        const input = x.dl_flac_input();
        this.views = { at, runs, input: [0, 1].map((c) => new Float32Array(buffer, input + c * 4096 * 4, 4096)) };
      } else {
        this.views = { at, runs, input: runs };
      }
    }
    const input = this.views.input;
    let peak = 0;
    for (let c = 0; c < lanes; c++) {
      const run = this.views.runs[c];
      const to = input[c];
      if (to === run) {
        for (let i = 0; i < CHUNK; i++) {
          const v = run[i];
          const a = v < 0 ? -v : v;
          if (a > peak) peak = a;
        }
        continue;
      }
      for (let i = 0; i < CHUNK; i++) {
        const v = run[i];
        to[i] = v;
        const a = v < 0 ? -v : v;
        if (a > peak) peak = a;
      }
    }
    let frames = CHUNK;
    // The tail.
    if (this.endFrame != null) {
      const from = this.frames;
      const since = from - this.endFrame;
      if (since >= 0) this.quietFrames = peak < QUIET ? this.quietFrames + CHUNK : 0;
      if (this.fadeFrom == null && since >= (TAIL_MAX - FADE) * rate) this.fadeFrom = from;
      const span = FADE * rate;
      if (this.fadeFrom != null) {
        for (let i = 0; i < CHUNK; i++) {
          const g = Math.max(0, 1 - (from + i - this.fadeFrom) / span);
          for (let c = 0; c < lanes; c++) input[c][i] *= g;
        }
        frames = Math.min(CHUNK, Math.max(0, this.fadeFrom + span - from));
      }
      const quietLongEnough = since + CHUNK >= TAIL_MIN * rate && this.quietFrames >= QUIET_FOR * rate;
      if (quietLongEnough || (this.fadeFrom != null && from + CHUNK >= this.fadeFrom + span)) this.finished = true;
    }
    this.planes = input;
    return frames;
  }

  // The chunk's frames are in the file: count them, and hear where the track
  // ends before the next chunk is rendered (events the chunk made).
  taken(frames) {
    this.frames += frames;
    if (this.format === 'flac') {
      if (!this.finished) this.watch();
    } else {
      this.owed = true;
    }
  }

  // 'flac': render the next chunk and encode it. Returns the bytes to append
  // (a view, possibly empty), or null once the file is finished (then `last`
  // holds the final bytes and `header()` the finished header).
  step() {
    const frames = this.chunk();
    if (frames < 0) return null;
    const out = this.bytes(this.x.dl_flac_write(frames));
    this.taken(frames);
    return out;
  }

  // 'opus': render the next chunk. Returns { planes, frames } (the samples
  // of each lane, valid until `pcm` is called again; `frames` of them are
  // the file's), or null once the file is finished.
  pcm() {
    const frames = this.chunk();
    if (frames < 0) return null;
    this.taken(frames);
    return { planes: this.planes, frames };
  }

  // The last frame's bytes, once `step` has returned its last.
  last() {
    return this.bytes(this.x.dl_flac_finish());
  }

  // The finished header (42 bytes: the length and the MD5), to write over
  // the file's first bytes.
  header() {
    return new Uint8Array(this.x.memory.buffer, this.x.dl_flac_header(), 42);
  }

  // The late, missed and dropped notes the core counted while rendering.
  counts() {
    const x = this.x;
    return { late: x.dl_late(), missed: x.dl_missed(), dropped: x.dl_dropped() };
  }
}
