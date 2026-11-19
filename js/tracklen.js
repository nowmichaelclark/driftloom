// Track length (session 30c, queue item 111; Mikey, 2026-11-18: "Track
// length should get it's own viewport ... Should be able to set track
// length to an exact second", and "we could include a fade in/fade out
// option for the track length viewport actually").
//
// How long the loop playing plays before the next, to the second, and how
// it comes in and goes out: the loop's own, as the Length slider's was (a
// saved loop keeps it, an album's track has its own, a new loop plays on
// and on). Kept on the spec as `track`: { secs, fadeIn, fadeOut }, `secs`
// null for on and on; a spec without one plays on and on, fade in off and
// fade out on (the conservative defaults: today's start, and a close).
//
// The core counts the time on its own clock from when it is set (or from
// the loop's first step), so a tempo change doesn't move it
// (core/src/player/track.rs, `Track::Seconds`); the fades are its
// (`FADE_IN`, `FADE_OUT`).
//
// The view is built like the Bars view (js/picks.js, design/next-picks.md):
// the time typed to the second (h:mm:ss, or m:ss under an hour), keys a
// second and a minute either way, a coarse slider for the big moves (to a
// whole day), an "on and on" key, the two fade keys, cancel and done. It
// opens from the Vibe panel's pace line, and from the export when nothing
// would end the export (js/export.js).

// The most a track is set to: a day (core/src/fades.rs, TRACK_MOST).
export const TRACK_MOST = 86400;
// The kind's bits beside the time (core/src/fades.rs): the core's 3 is a
// time, and a fade in and a fade out each add their own.
const KIND_SECONDS = 3;
const KIND_FADE_IN = 4;
const KIND_FADE_OUT = 8;
// What a loop with none of its own plays: on and on, no fade in, a fade
// out ready for when a time is set.
export const DEFAULT_TRACK = Object.freeze({ secs: null, fadeIn: false, fadeOut: true });
// The coarse slider's stops, in seconds: fine at the short end, where people
// set it, and reaching a whole day (the Length slider's reach).
export const STOPS = [60, 120, 300, 600, 900, 1200, 1800, 2700, 3600, 5400, 7200, 10800, 14400, 28800, 43200, 86400];

const isSecs = (n) => Number.isInteger(n) && n >= 1 && n <= TRACK_MOST;

// The track of `spec`, each part filled in.
export function trackOfSpec(spec) {
  const t = (spec && spec.track) || {};
  return {
    secs: isSecs(t.secs) ? t.secs : null,
    fadeIn: typeof t.fadeIn === 'boolean' ? t.fadeIn : DEFAULT_TRACK.fadeIn,
    fadeOut: typeof t.fadeOut === 'boolean' ? t.fadeOut : DEFAULT_TRACK.fadeOut,
  };
}

// Set `spec`'s track to `t` (any part left out kept), or take it away where
// it is the default, so a loop with none set keeps none.
export function setTrack(spec, t) {
  const next = { ...trackOfSpec(spec), ...t };
  if (!isSecs(next.secs)) next.secs = null;
  if (next.secs == null && next.fadeIn === DEFAULT_TRACK.fadeIn && next.fadeOut === DEFAULT_TRACK.fadeOut) delete spec.track;
  else spec.track = next;
  return spec;
}

// Whether the loop of `spec` ends by itself (a time set).
export const ends = (spec) => trackOfSpec(spec).secs != null;

// The track as the core takes it (`dl_play_track`, `Track::of`): [kind,
// count], 0 on and on (4 with its fade in), 3 a time of `count` seconds
// with its fades' bits.
export function coreTrack(spec) {
  const t = trackOfSpec(spec);
  if (t.secs == null) return [t.fadeIn ? KIND_FADE_IN : 0, 0];
  return [KIND_SECONDS | (t.fadeIn ? KIND_FADE_IN : 0) | (t.fadeOut ? KIND_FADE_OUT : 0), t.secs];
}

// The track of the core's [kind, count] (an album code's pace,
// `dl_code_pace`), or null for none it could be.
export function trackOfCore(kind, count) {
  const fadeIn = (kind & KIND_FADE_IN) !== 0;
  const fadeOut = (kind & KIND_FADE_OUT) !== 0;
  if ((kind & 3) === 0) return { secs: null, fadeIn, fadeOut: DEFAULT_TRACK.fadeOut };
  if ((kind & 3) === KIND_SECONDS && isSecs(count)) return { secs: count, fadeIn, fadeOut };
  return null;
}

// A time as the view shows it: h:mm:ss, or m:ss under an hour.
export function clock(secs) {
  const s = Math.max(0, Math.round(secs));
  const h = Math.floor(s / 3600);
  const m = Math.floor((s % 3600) / 60);
  const ss = String(s % 60).padStart(2, '0');
  return h ? `${h}:${String(m).padStart(2, '0')}:${ss}` : `${m}:${ss}`;
}

// A time in words, for screen readers and the line under it: "1 hour 23
// minutes 45 seconds".
export function spokenTime(secs) {
  const s = Math.max(0, Math.round(secs));
  const parts = [[Math.floor(s / 3600), 'hour'], [Math.floor((s % 3600) / 60), 'minute'], [s % 60, 'second']];
  return parts.filter(([n]) => n).map(([n, w]) => `${n} ${w}${n === 1 ? '' : 's'}`).join(' ') || '0 seconds';
}

// What is typed, as seconds (or null for nothing to read): h:mm:ss, m:ss or
// :ss; digits alone fill from the right, as a timer's keys do ("12345" is
// 1:23:45, "90" a minute and a half). Clamped to a second to a day.
export function readClock(text) {
  const t = String(text || '').trim();
  if (!t) return null;
  let h = 0, m = 0, s = 0;
  if (t.includes(':')) {
    const parts = t.split(':').map((p) => p.trim());
    if (parts.length > 3 || parts.some((p) => !/^\d*$/.test(p))) return null;
    const n = parts.map((p) => (p ? parseInt(p, 10) : 0));
    while (n.length < 3) n.unshift(0);
    [h, m, s] = n;
  } else {
    const d = t.replace(/\D/g, '');
    if (!d) return null;
    const x = d.slice(-6).padStart(6, '0');
    h = parseInt(x.slice(0, 2), 10);
    m = parseInt(x.slice(2, 4), 10);
    s = parseInt(x.slice(4), 10);
  }
  const secs = h * 3600 + m * 60 + s;
  if (!Number.isFinite(secs)) return null;
  return Math.max(1, Math.min(TRACK_MOST, secs));
}

// A loop saved before session 30c: its length was whole passes
// (`playFor`) or bars to a phrase line (`playBars`), counted at its tempo.
// Now the time they came to, to the second, with the defaults' fades (a
// close, as the slider's had; no fade in, as it started). True if it moved.
export function fromOldLength(spec) {
  if (!spec || typeof spec !== 'object' || !('playFor' in spec || 'playBars' in spec)) return false;
  const { playFor, playBars } = spec;
  delete spec.playFor;
  delete spec.playBars;
  const bar = (spec.stepsPerBar || 16) * (60 / (spec.bpm > 0 ? spec.bpm : 80) / 4);
  const passBars = spec.length === 'endless' ? Infinity : typeof spec.length === 'number' ? spec.length : spec.bars;
  let secs = null;
  if (Number.isInteger(playFor) && playFor >= 1) secs = playFor * passBars * bar;
  else if (Number.isInteger(playBars) && playBars >= 1) secs = playBars * bar;
  if (secs != null && Number.isFinite(secs)) setTrack(spec, { secs: Math.max(1, Math.min(TRACK_MOST, Math.round(secs))) });
  return true;
}

// ------------------------------------------------------------- the view

const el = (id) => document.getElementById(id);
// What the view holds while it is open: { secs, on, fadeIn, fadeOut }, on
// meaning on and on; and what it tells the app (`wireTrack`).
let draft = null;
let api = null;

// The slider's stop nearest `secs`.
function stopOf(secs) {
  let best = 0;
  for (let i = 1; i < STOPS.length; i++) if (Math.abs(STOPS[i] - secs) < Math.abs(STOPS[best] - secs)) best = i;
  return best;
}

// The view opened on `spec`'s track (none: on and on), from `from` (the
// view done and cancel go back to, and 'export' to export once a time is
// set).
export function openTrack(spec, from = 'vibe') {
  const t = trackOfSpec(spec);
  draft = { secs: t.secs || 300, on: t.secs == null, fadeIn: t.fadeIn, fadeOut: t.fadeOut, from };
  renderTrack();
}

export function renderTrack() {
  if (!draft) return;
  const { secs, on, fadeIn, fadeOut } = draft;
  const view = el('viewTrack');
  view.classList.toggle('endless', on);
  const num = el('trkNum');
  if (document.activeElement !== num || on) num.value = on ? 'on and on' : clock(secs);
  num.setAttribute('aria-label', on ? 'Track length: on and on' : `Track length: ${spokenTime(secs)}`);
  el('trkWords').textContent = on ? 'plays until you move on' : spokenTime(secs);
  el('trkSecDown').disabled = !on && secs <= 1;
  el('trkMinDown').disabled = !on && secs <= 1;
  el('trkSecUp').disabled = !on && secs >= TRACK_MOST;
  el('trkMinUp').disabled = !on && secs >= TRACK_MOST;
  const slide = el('trkSlide');
  const s = stopOf(secs);
  slide.value = String(s);
  const last = STOPS.length - 1;
  const w = slide.clientWidth || 280;
  slide.style.setProperty('--tile', `${((w - 24) / last).toFixed(3)}px`);
  slide.style.setProperty('--fill', on ? '0px' : `${(12 + (w - 24) * (s / last)).toFixed(2)}px`);
  slide.setAttribute('aria-valuetext', on ? 'on and on' : spokenTime(STOPS[s]));
  for (const tick of el('trkTicks').children) tick.classList.toggle('on', !on && STOPS[s] === secs && Number(tick.dataset.stop) === s);
  const onKey = el('trkOn');
  onKey.setAttribute('aria-pressed', on ? 'true' : 'false');
  onKey.classList.toggle('armed', on);
  const fin = el('trkFadeIn');
  fin.setAttribute('aria-pressed', fadeIn ? 'true' : 'false');
  fin.classList.toggle('armed', fadeIn);
  // The fade out is a time's: dimmed while it plays on.
  const fout = el('trkFadeOut');
  fout.setAttribute('aria-pressed', fadeOut ? 'true' : 'false');
  fout.classList.toggle('armed', fadeOut);
  fout.disabled = on;
  fout.parentElement.classList.toggle('off-out', on);
  fitNumber();
}

function fitNumber() {
  const num = el('trkNum');
  num.style.fontSize = '';
  for (let i = 0; i < 8 && num.scrollWidth > num.clientWidth + 1; i++) {
    num.style.fontSize = `${parseFloat(getComputedStyle(num).fontSize) * 0.9}px`;
  }
}

// −1 s, +1 s, −1 min, +1 min, the slider and typing all set a time, from on
// and on too (it starts from the time the view last showed).
function setDraft(secs) {
  draft.secs = Math.max(1, Math.min(TRACK_MOST, Math.round(secs)));
  draft.on = false;
  renderTrack();
}

function commitTyped() {
  if (!draft) return;
  const secs = readClock(el('trkNum').value);
  if (secs != null) setDraft(secs);
  else renderTrack();
}

// What the view hands back on done: the track for the spec.
function result() {
  return { secs: draft.on ? null : draft.secs, fadeIn: draft.fadeIn, fadeOut: draft.fadeOut };
}

// `handlers`: { done(track, from), cancel(from) }.
export function wireTrack(handlers) {
  api = handlers;
  el('trkCancel').addEventListener('click', () => {
    const from = draft ? draft.from : 'vibe';
    draft = null;
    api.cancel(from);
  });
  el('trkDone').addEventListener('click', () => {
    if (!draft) return;
    if (document.activeElement === el('trkNum')) commitTyped();
    const t = result();
    const from = draft.from;
    draft = null;
    api.done(t, from);
  });
  el('trkSecDown').addEventListener('click', () => setDraft(draft.secs - 1));
  el('trkSecUp').addEventListener('click', () => setDraft(draft.secs + 1));
  el('trkMinDown').addEventListener('click', () => setDraft(draft.secs - 60));
  el('trkMinUp').addEventListener('click', () => setDraft(draft.secs + 60));
  const num = el('trkNum');
  num.addEventListener('focus', () => {
    num.value = draft.on ? '' : clock(draft.secs);
    num.select();
  });
  num.addEventListener('input', () => {
    const v = num.value.replace(/[^0-9:]/g, '').slice(0, 9);
    if (v !== num.value) num.value = v;
  });
  num.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') { e.preventDefault(); num.blur(); }
    if (e.key === 'Escape') { e.preventDefault(); num.value = ''; num.blur(); }
  });
  num.addEventListener('blur', commitTyped);
  // The slider snaps to its stops; arrows step a stop at a time, from a time
  // between stops to the stop on that side.
  const slide = el('trkSlide');
  slide.addEventListener('input', () => setDraft(STOPS[Math.round(Number(slide.value))]));
  slide.addEventListener('keydown', (e) => {
    const up = e.key === 'ArrowRight' || e.key === 'ArrowUp' || e.key === 'PageUp';
    const down = e.key === 'ArrowLeft' || e.key === 'ArrowDown' || e.key === 'PageDown';
    const last = STOPS.length - 1;
    let next = null;
    if (up) next = STOPS.findIndex((s) => s > draft.secs);
    else if (down) next = STOPS.findLastIndex((s) => s < draft.secs);
    else if (e.key === 'Home') next = 0;
    else if (e.key === 'End') next = last;
    if (next == null) return;
    e.preventDefault();
    if (next < 0) next = up ? last : 0;
    setDraft(STOPS[next]);
  });
  // On and on: lit while the loop plays on; pressed again, the time it
  // showed. The fades: each lit while on, pressed to turn it over.
  el('trkOn').addEventListener('click', () => {
    draft.on = !draft.on;
    renderTrack();
  });
  el('trkFadeIn').addEventListener('click', () => {
    draft.fadeIn = !draft.fadeIn;
    renderTrack();
  });
  el('trkFadeOut').addEventListener('click', () => {
    if (draft.on) return;
    draft.fadeOut = !draft.fadeOut;
    renderTrack();
  });
}
