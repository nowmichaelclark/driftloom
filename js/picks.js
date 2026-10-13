// Next picks (Painter session 6, design/next-picks.md): what new loops lean
// toward. Three places: Now Playing shows the picked values as pills (only
// outside an album; in an album the album is what comes next), the New
// loops panel lists the five lines, and the Length picker sets bars.
//
// The five lines are moods, tempo and length (sharing a line), key, style
// and voices. Moods are today's vibes (state.vibes). Length and tempo are
// state.picks, saved in the prefs, and reach today's generator through
// `applyPicks`, clamped to what it can play. Key, style and voices have no
// picker yet and always read "any".

import { el } from './ui.js';

export const MIN_BARS = 1;
export const MAX_BARS = 32768;
// The doubling slider: one stop per power of two, 1 to 32,768.
const STOPS = 15;
// Today's generator plays loops of 1 to 32 bars and tempos of 32 to 190
// bpm. A longer length is played as passes of one loop; the core brings
// the full range (docs/BRAIN.md, "Two tracks").
const GEN_MAX_BARS = 32;
const GEN_BPM = [32, 190];

const NB = ' ';
const LINES = [
  { id: 'mood', any: 'any mood', name: 'Any mood' },
  { id: 'pace', any: 'any tempo · any length', name: 'Any tempo and length' },
  { id: 'key', any: 'any key', name: 'Any key' },
  { id: 'style', any: 'any style', name: 'Any style' },
  { id: 'voices', any: 'any voices', name: 'Any voices' },
];

let api = null;
// The Length picker works on a draft: done keeps it, cancel drops it.
let draft = null;   // { bars, endless }
let before = null;  // what length was when the picker opened

// ------------------------------------------------------------- words

export function cleanPicks(p) {
  const out = { tempo: null, length: null };
  if (p && Array.isArray(p.tempo) && p.tempo.length === 2) {
    const lo = Math.round(Number(p.tempo[0])), hi = Math.round(Number(p.tempo[1]));
    if (lo > 0 && hi >= lo) out.tempo = [lo, hi];
  }
  if (p && p.length === 'endless') out.length = 'endless';
  else if (p && Number.isFinite(Number(p.length)) && p.length !== null) out.length = clampBars(p.length);
  return out;
}

export function clampBars(n) {
  return Math.max(MIN_BARS, Math.min(MAX_BARS, Math.round(Number(n)) || MIN_BARS));
}

const group = (n) => n.toLocaleString('en-US');

// The first three moods, then an ellipsis.
export function moodsText(moods) {
  if (!moods.length) return '';
  return moods.slice(0, 3).join(', ') + (moods.length > 3 ? '…' : '');
}

export function tempoText(t) {
  return t ? (t[0] === t[1] ? `${t[0]}${NB}bpm` : `${t[0]}–${t[1]}${NB}bpm`) : '';
}

export function lengthText(len) {
  if (len === 'endless') return 'endless';
  if (len == null) return '';
  return `${group(len)}${NB}${len === 1 ? 'bar' : 'bars'}`;
}

// For screen readers: "70 to 85 bpm" rather than a dash.
const spoken = (t) => t.replace('–', ' to ').replace(/ /g, ' ').replace('…', ', and more');

// ---------------------------------------------------- today's generator

// Bends a freshly drawn loop toward the picks, within what the generator
// can play. Tempo is clamped into the picked range. A length of up to 32
// bars is the loop's own length, played once; a longer one keeps the loop
// as drawn and plays it enough times round to last that long. Endless and
// "any" leave it looping until skipped, as today.
export function applyPicks(spec, picks) {
  if (!spec || !picks) return spec;
  const t = picks.tempo;
  if (t) {
    const lo = Math.max(GEN_BPM[0], Math.min(GEN_BPM[1], t[0]));
    const hi = Math.max(lo, Math.min(GEN_BPM[1], t[1]));
    spec.bpm = Math.max(lo, Math.min(hi, spec.bpm));
  }
  const len = picks.length;
  if (typeof len === 'number') {
    if (len <= GEN_MAX_BARS) {
      spec.bars = len;
      spec.playFor = 1;
    } else {
      spec.playFor = Math.max(1, Math.round(len / spec.bars));
    }
  }
  return spec;
}

// ------------------------------------------------------------ the pills

function pill(text, kind, opts = {}) {
  const p = document.createElement('span');
  p.className = `pk-pill pk-${kind}${opts.dashed ? ' dashed' : ''}`;
  p.textContent = text;
  return p;
}

// After layout: a pill wider than its row may wrap between words rather
// than run out of the panel (only very long moods at large text).
function fitPills(host) {
  for (const p of host.querySelectorAll('.pk-pill')) {
    p.classList.remove('wrap');
    if (p.scrollWidth > p.clientWidth + 1) p.classList.add('wrap');
  }
}

// Now Playing, outside an album: the picked values on up to two rows, all
// one button that opens New loops. Only picked values show; with nothing
// picked, one quiet dashed "anything".
export function renderNowPicks() {
  const { state } = api;
  const b = el('nowPicks');
  const moods = state.vibes;
  const { tempo, length } = state.picks;
  b.textContent = '';
  const spokenParts = [];
  if (moods.length) {
    const row = document.createElement('span');
    row.className = 'pk-row';
    row.append(pill(moodsText(moods), 'mood'));
    b.append(row);
    spokenParts.push(spoken(moodsText(moods)));
  }
  if (tempo || length != null) {
    const row = document.createElement('span');
    row.className = 'pk-row';
    if (tempo) { row.append(pill(tempoText(tempo), 'tempo')); spokenParts.push(spoken(tempoText(tempo))); }
    if (length != null) { row.append(pill(lengthText(length), 'length')); spokenParts.push(spoken(lengthText(length))); }
    b.append(row);
  }
  if (!spokenParts.length) {
    const row = document.createElement('span');
    row.className = 'pk-row';
    row.append(pill('anything', 'none', { dashed: true }));
    b.append(row);
    spokenParts.push('anything');
  }
  b.setAttribute('aria-label', `New loops: ${spokenParts.join(', ')}`);
  fitPills(b);
}

// ------------------------------------------------------ New loops panel

function lineButton(cls, onClick) {
  const b = document.createElement('button');
  b.type = 'button';
  b.className = `pk-val ${cls || ''}`.trim();
  b.addEventListener('click', onClick);
  return b;
}

function plain(text) {
  const s = document.createElement('span');
  s.className = 'pk-any';
  s.textContent = text;
  return s;
}

export function renderPanel() {
  const { state } = api;
  const list = el('picksList');
  list.textContent = '';
  const moods = state.vibes;
  const { tempo, length } = state.picks;
  const isAny = { mood: !moods.length, pace: !tempo && length == null, key: true, style: true, voices: true };

  for (const line of LINES) {
    const row = document.createElement('div');
    row.className = `pk-line pk-line-${line.id}`;
    const vals = document.createElement('div');
    vals.className = 'pk-vals';

    if (line.id === 'mood') {
      // Moods open today's Vibes list, picked or not.
      const b = lineButton('fill', () => api.openVibes());
      if (moods.length) {
        b.append(pill(moodsText(moods), 'mood'));
        b.setAttribute('aria-label', `Moods: ${spoken(moodsText(moods))}`);
      } else {
        b.append(plain('any mood'));
        b.setAttribute('aria-label', 'Moods: any');
      }
      vals.append(b);
    } else if (line.id === 'pace') {
      // Tempo has no picker yet, so it is shown, not tapped. Length opens
      // the Length picker.
      if (isAny.pace) {
        vals.append(plain('any tempo'));
        const sep = document.createElement('span');
        sep.className = 'pk-any pk-sep';
        sep.setAttribute('aria-hidden', 'true');
        sep.textContent = '·';
        vals.append(sep);
        const b = lineButton('', () => api.openLength());
        b.append(plain('any length'));
        b.setAttribute('aria-label', 'Length: any');
        vals.append(b);
      } else {
        const t = document.createElement('span');
        t.className = 'pk-val static';
        t.append(tempo ? pill(tempoText(tempo), 'tempo') : pill('any tempo', 'tempo', { dashed: true }));
        vals.append(t);
        const b = lineButton('', () => api.openLength());
        b.append(length != null ? pill(lengthText(length), 'length') : pill('any length', 'length', { dashed: true }));
        b.setAttribute('aria-label', `Length: ${length != null ? spoken(lengthText(length)) : 'any'}`);
        vals.append(b);
      }
    } else {
      vals.append(plain(line.any));
    }

    // The any key: lit while the whole line is left to chance; pressing an
    // unlit one clears the line (on line two, tempo and length both).
    const any = document.createElement('button');
    any.type = 'button';
    any.className = `vkey word pk-anykey${isAny[line.id] ? ' armed' : ''}`;
    any.textContent = 'any';
    any.setAttribute('aria-label', line.name);
    any.setAttribute('aria-pressed', isAny[line.id] ? 'true' : 'false');
    any.addEventListener('click', () => {
      if (any.getAttribute('aria-pressed') === 'true') return;
      api.clearLine(line.id);
    });
    row.append(vals, any);
    list.append(row);
  }
  fitPanel();
}

// The fit guard: at large text, very long picks can make the five lines
// taller than the panel. Step the text cap down (1.5x, 1.3x, 1.15x, 1x)
// until they fit, never below the 100% size; then take the lines'
// padding away.
const CAPS = [1.5, 1.3, 1.15, 1];
function fitPanel() {
  const view = el('viewPicks');
  const list = el('picksList');
  const fits = () => list.scrollHeight <= list.clientHeight + 0.5;
  view.classList.remove('pk-tight');
  for (const cap of CAPS) {
    view.style.setProperty('--pk-cap', String(cap));
    fitPills(list);
    if (fits()) return;
  }
  // Still too tall at 1x (the widest moods with both pace values): the
  // lines lose their padding.
  view.classList.add('pk-tight');
  fitPills(list);
}

// --------------------------------------------------------- Length picker

function stopOf(bars) { return Math.log2(bars); }

// Seconds of play for `bars` bars, at each end of the picked tempo range
// (or the playing loop's tempo), in the playing loop's metre.
function playTime(bars) {
  const { state } = api;
  const spec = state.spec;
  const spb = (spec && spec.stepsPerBar) || 16;
  const t = state.picks.tempo || [spec ? spec.bpm : 80, spec ? spec.bpm : 80];
  const secs = (bpm) => bars * spb * (60 / bpm / 4);
  return [secs(t[1]), secs(t[0])];
}

function roughly([a, b]) {
  // One unit for both ends, picked by the longer one.
  let unit, div, step;
  if (b < 90) { unit = 's'; div = 1; step = 5; }
  else if (b < 90 * 60) { unit = 'min'; div = 60; step = 1; }
  else if (b < 48 * 3600) { unit = 'hours'; div = 3600; step = 1; }
  else { unit = 'days'; div = 86400; step = 1; }
  const r = (x) => {
    const v = x / div;
    if (unit === 's' && v < 10) return Math.max(1, Math.round(v));
    return Math.max(step, Math.round(v / step) * step);
  };
  const lo = r(a), hi = r(b);
  if (unit === 'hours' && hi === 1) unit = 'hour';
  if (unit === 'days' && hi === 1) unit = 'day';
  return lo === hi ? `≈${NB}${lo}${NB}${unit}` : `≈${NB}${lo}–${hi}${NB}${unit}`;
}

function setDraftBars(n) {
  draft.bars = clampBars(n);
  renderLength();
}

export function openLength() {
  const { state } = api;
  before = state.picks.length;
  const start = typeof before === 'number' ? before
    : before === 'endless' ? (state.spec ? state.spec.bars : 16)
      : (state.spec ? state.spec.bars : 16);
  draft = { bars: clampBars(start), endless: before === 'endless' };
  renderLength();
}

export function renderLength() {
  if (!draft) return;
  const num = el('lenNum');
  const slide = el('lenSlide');
  const { bars, endless } = draft;
  el('viewLength').classList.toggle('endless', endless);
  if (document.activeElement !== num || endless) num.value = endless ? '∞' : group(bars);
  num.disabled = endless;
  num.setAttribute('aria-label', endless ? 'Bars: endless' : 'Bars');
  const time = el('lenTime');
  time.textContent = endless ? 'endless' : `${bars === 1 ? 'bar' : 'bars'} · `;
  if (!endless) {
    const nw = document.createElement('span');
    nw.className = 'nw';
    nw.textContent = roughly(playTime(bars));
    time.append(nw);
  }
  el('lenDown').disabled = endless || bars <= MIN_BARS;
  el('lenUp').disabled = endless || bars >= MAX_BARS;
  slide.disabled = endless;
  const s = stopOf(bars);
  slide.value = String(s);
  const w = slide.clientWidth || 280;
  slide.style.setProperty('--tile', `${((w - 24) / STOPS).toFixed(3)}px`);
  slide.style.setProperty('--fill', `${(12 + (w - 24) * (s / STOPS)).toFixed(2)}px`);
  slide.setAttribute('aria-valuetext', endless ? 'endless' : lengthText(bars).replace(/ /g, ' '));
  for (const tick of el('lenTicks').children) {
    tick.classList.toggle('on', Math.abs(Number(tick.dataset.stop) - s) < 1e-9);
  }
  const sw = el('lenEndless');
  sw.setAttribute('aria-checked', endless ? 'true' : 'false');
  fitNumber();
}

// The number keeps its size until it would not fit between −1 and +1
// (five digits at large text), then steps down.
function fitNumber() {
  const num = el('lenNum');
  num.style.fontSize = '';
  for (let i = 0; i < 8 && num.scrollWidth > num.clientWidth + 1; i++) {
    num.style.fontSize = `${parseFloat(getComputedStyle(num).fontSize) * 0.9}px`;
  }
}

function commitTyped() {
  if (!draft) return;
  const num = el('lenNum');
  const digits = num.value.replace(/[^0-9]/g, '');
  if (digits) draft.bars = clampBars(digits);
  renderLength();
}

export function wirePicks(handlers) {
  api = handlers;
  el('nowPicks').addEventListener('click', () => api.setView('picks'));
  el('picksClose').addEventListener('click', () => api.setView('now'));

  el('lenCancel').addEventListener('click', () => {
    // Nothing was changed while the picker was open, so leaving it is
    // enough: the length is still what it was before, "any" included.
    draft = null;
    api.setView('picks');
  });
  el('lenDone').addEventListener('click', () => {
    if (document.activeElement === el('lenNum')) commitTyped();
    const value = draft.endless ? 'endless' : draft.bars;
    draft = null;
    api.setLength(value);
    api.setView('picks');
  });
  el('lenDown').addEventListener('click', () => setDraftBars(draft.bars - 1));
  el('lenUp').addEventListener('click', () => setDraftBars(draft.bars + 1));

  const num = el('lenNum');
  num.addEventListener('focus', () => {
    if (draft.endless) return;
    num.value = String(draft.bars);
    num.select();
  });
  num.addEventListener('input', () => {
    const digits = num.value.replace(/[^0-9]/g, '').slice(0, 5);
    if (digits !== num.value) num.value = digits;
  });
  num.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') { e.preventDefault(); num.blur(); }
    if (e.key === 'Escape') { e.preventDefault(); num.value = ''; num.blur(); }
  });
  num.addEventListener('blur', commitTyped);

  // The slider snaps to its sixteen stops; arrows step a stop at a time,
  // from a number between stops to the stop on that side.
  const slide = el('lenSlide');
  slide.addEventListener('input', () => setDraftBars(2 ** Math.round(Number(slide.value))));
  slide.addEventListener('keydown', (e) => {
    const up = e.key === 'ArrowRight' || e.key === 'ArrowUp' || e.key === 'PageUp';
    const down = e.key === 'ArrowLeft' || e.key === 'ArrowDown' || e.key === 'PageDown';
    const s = stopOf(draft.bars);
    let next = null;
    if (up) next = Number.isInteger(s) ? s + 1 : Math.ceil(s);
    else if (down) next = Number.isInteger(s) ? s - 1 : Math.floor(s);
    else if (e.key === 'Home') next = 0;
    else if (e.key === 'End') next = STOPS;
    if (next == null) return;
    e.preventDefault();
    setDraftBars(2 ** Math.max(0, Math.min(STOPS, next)));
  });

  el('lenEndless').addEventListener('click', () => {
    draft.endless = !draft.endless;
    renderLength();
  });
}
