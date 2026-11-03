// Next picks (Painter session 6, design/next-picks.md): what new loops lean
// toward. Three places: Now Playing shows the picked values as pills (only
// outside an album; in an album the album is what comes next), the Vibe
// panel lists the five lines (the vibe key and the pills open it; it was
// "New loops" in the brief, renamed by Mikey 2026-10-14), and the Length
// picker sets bars.
//
// The five lines are moods, tempo and length (sharing a line), key, style
// and voices. Moods are today's vibes (state.vibes). Length and tempo are
// state.picks, saved in the prefs, with the key and the mode (queue item
// 89). The moods, the tempo, the length and the key and mode reach the
// generator as its request (`requestFor`; queue items 85, 88 and 89: any
// length from 1 bar to 32,768, or Endless; one key or mode, or a set to
// choose from), the style (queue item 90: one profile or a set), and the
// voices of each layer and the layers on or off (queue item 91). None of
// the key, style and voices lines has a picker yet: they show what is picked
// but cannot be tapped until a Painter session designs their pickers.

import { el } from './ui.js';
import { log2, pow } from './dlmath.js';
import { readRequest, RequestError, pieceTime, LAYERS } from './generator.js';
import { NOTE_NAMES, SCALES, SCALE_ORDER } from './theory.js';
import { PROFILE_ORDER, VOICE_ORDER } from './characters.js';

export const MIN_BARS = 1;
export const MAX_BARS = 32768;
// The doubling slider: one stop per power of two, 1 to 32,768.
const STOPS = 15;

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
// mode is 'bars', 'endless' or 'any'; bars is the number shown when the
// mode is 'bars', and where −1, +1 and the slider start from otherwise.
let draft = null;   // { bars, mode }
let before = null;  // what length was when the picker opened

// ------------------------------------------------------------- words

export function cleanPicks(p) {
  const out = { tempo: null, length: null, key: null, scale: null, style: null, voices: null, layers: null };
  if (p && Array.isArray(p.tempo) && p.tempo.length === 2) {
    const lo = Math.round(Number(p.tempo[0])), hi = Math.round(Number(p.tempo[1]));
    if (lo > 0 && hi >= lo) out.tempo = [lo, hi];
  }
  if (p && p.length === 'endless') out.length = 'endless';
  else if (p && Number.isFinite(Number(p.length)) && p.length !== null) out.length = clampBars(p.length);
  // The key (home notes, 0 is C) and the mode (names), a set each; what the
  // generator would not take is left out.
  if (p && Array.isArray(p.key)) {
    const keys = [...new Set(p.key)].filter((k) => Number.isInteger(k) && k >= 0 && k <= 11).sort((a, b) => a - b);
    if (keys.length) out.key = keys;
  }
  if (p && Array.isArray(p.scale)) {
    const scales = SCALE_ORDER.filter((id) => p.scale.includes(id));
    if (scales.length) out.scale = scales;
  }
  if (p && Array.isArray(p.style)) {
    const styles = PROFILE_ORDER.filter((id) => p.style.includes(id));
    if (styles.length) out.style = styles;
  }
  // The voices, { layer: names } in each layer's frozen order, and the
  // layers, { layer: 'on' | 'off' } (not all five off).
  if (p && p.voices && typeof p.voices === 'object') {
    const voices = {};
    for (const layer of LAYERS) {
      const asked = Array.isArray(p.voices[layer]) ? VOICE_ORDER[layer].filter((v) => p.voices[layer].includes(v)) : [];
      if (asked.length) voices[layer] = asked;
    }
    if (Object.keys(voices).length) out.voices = voices;
  }
  if (p && p.layers && typeof p.layers === 'object') {
    const layers = {};
    for (const layer of LAYERS) if (p.layers[layer] === 'on' || p.layers[layer] === 'off') layers[layer] = p.layers[layer];
    if (Object.keys(layers).length && !LAYERS.every((l) => layers[l] === 'off')) out.layers = layers;
  }
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

// The key line's words: "D dorian", "D or G", "dorian or aeolian", or ''.
export function keyText(key, scale) {
  const parts = [];
  if (key && key.length) parts.push(key.map((k) => NOTE_NAMES[k]).join(' or '));
  if (scale && scale.length) parts.push(scale.map((id) => SCALES[id].label.toLowerCase()).join(' or '));
  return parts.join(' ');
}

// The style line's words: "tide", "tide or shrine", or ''.
export function styleText(style) {
  return style && style.length ? style.join(' or ') : '';
}

// The voices line's words: the voices asked ("kalimba, marimba"), then the
// layers left out ("no drums") and the layers always played ("melody
// always"), or ''.
export function voicesText(voices, layers) {
  const parts = [];
  if (voices) for (const layer of LAYERS) for (const v of voices[layer] || []) if (!parts.includes(v)) parts.push(v);
  if (layers) {
    for (const layer of LAYERS) if (layers[layer] === 'off') parts.push(`no ${layer}`);
    for (const layer of LAYERS) if (layers[layer] === 'on') parts.push(`${layer} always`);
  }
  return parts.join(', ');
}

export function lengthText(len) {
  if (len === 'endless') return 'endless';
  if (len == null) return '';
  return `${group(len)}${NB}${len === 1 ? 'bar' : 'bars'}`;
}

// For screen readers: "70 to 85 bpm" rather than a dash.
const spoken = (t) => t.replace('–', ' to ').replace(/ /g, ' ').replace('…', ', and more');

// ---------------------------------------------------- the request

// What the next loop is asked for (the generator's request): the moods, the
// tempo and the length picked. The same shape a game or a developer sends.
export function requestFor(state) {
  const { tempo, length, key, scale, style, voices, layers } = state.picks;
  return { vibes: state.vibes.slice(), tempo, length, key, scale, style, voices, layers };
}

// The "can't do that" line for the picks (a short plain line, its look a
// Painter question), or null when the request can be met.
export function pickProblem(state) {
  try {
    readRequest(requestFor(state));
    return null;
  } catch (e) {
    if (e instanceof RequestError) return e.message;
    throw e;
  }
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
// one button that opens the Vibe panel. Only picked values show; with nothing
// picked, one quiet dashed "anything".
export function renderNowPicks() {
  const { state } = api;
  const b = el('nowPicks');
  const moods = state.vibes;
  const { tempo, length, key, scale, style, voices, layers } = state.picks;
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
  if (key || scale) {
    const row = document.createElement('span');
    row.className = 'pk-row';
    row.append(pill(keyText(key, scale), 'key'));
    b.append(row);
    spokenParts.push(spoken(keyText(key, scale)));
  }
  if (style) {
    const row = document.createElement('span');
    row.className = 'pk-row';
    row.append(pill(styleText(style), 'style'));
    b.append(row);
    spokenParts.push(spoken(styleText(style)));
  }
  if (voices || layers) {
    const row = document.createElement('span');
    row.className = 'pk-row';
    row.append(pill(voicesText(voices, layers), 'voices'));
    b.append(row);
    spokenParts.push(spoken(voicesText(voices, layers)));
  }
  if (!spokenParts.length) {
    const row = document.createElement('span');
    row.className = 'pk-row';
    row.append(pill('anything', 'none', { dashed: true }));
    b.append(row);
    spokenParts.push('anything');
  }
  b.setAttribute('aria-label', `Vibe: ${spokenParts.join(', ')}`);
  fitPills(b);
}

// ------------------------------------------------------------ Vibe panel

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
  const { tempo, length, key, scale, style, voices, layers } = state.picks;
  const isAny = { mood: !moods.length, pace: !tempo && length == null, key: !key && !scale, style: !style, voices: !voices && !layers };

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
      const cant = pickProblem(state);
      if (cant) {
        const note = document.createElement('span');
        note.className = 'pk-any pk-cant';
        note.setAttribute('role', 'status');
        note.textContent = `can't do that: ${cant}`;
        vals.append(note);
      }
    } else if (line.id === 'key' && !isAny.key) {
      // The key and the mode have no picker yet, so they are shown, not
      // tapped: "D dorian".
      const t = document.createElement('span');
      t.className = 'pk-val static';
      t.append(pill(keyText(key, scale), 'key'));
      vals.append(t);
    } else if (line.id === 'voices' && !isAny.voices) {
      // The voices have no picker yet either, so they are shown, not tapped.
      const t = document.createElement('span');
      t.className = 'pk-val static';
      t.append(pill(voicesText(voices, layers), 'voices'));
      vals.append(t);
    } else if (line.id === 'style' && !isAny.style) {
      // The style has no picker yet either, so it is shown, not tapped.
      const t = document.createElement('span');
      t.className = 'pk-val static';
      t.append(pill(styleText(style), 'style'));
      vals.append(t);
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
  const view = el('viewVibe');
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

function stopOf(bars) { return log2(bars); }

// Seconds of play for `bars` bars, at each end of the picked tempo range
// (or the playing loop's tempo), in the playing loop's metre: the
// generator's `pieceTime`, the core's `piece::time`, so the two never
// disagree.
function playTime(bars) {
  const { state } = api;
  const spec = state.spec;
  const spb = (spec && spec.stepsPerBar) || 16;
  const t = state.picks.tempo || [spec ? spec.bpm : 80, spec ? spec.bpm : 80];
  return pieceTime(bars, spb, t);
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

// −1, +1, the slider and typing all pick a number of bars, whatever the
// mode was.
function setDraftBars(n) {
  draft.bars = clampBars(n);
  draft.mode = 'bars';
  renderLength();
}

export function openLength() {
  const { state } = api;
  before = state.picks.length;
  // From endless or any, −1, +1 and the slider start at the playing
  // loop's length.
  const start = typeof before === 'number' ? before : (state.spec ? state.spec.bars : 16);
  const mode = before === 'endless' ? 'endless' : before == null ? 'any' : 'bars';
  draft = { bars: clampBars(start), mode };
  renderLength();
}

export function renderLength() {
  if (!draft) return;
  const num = el('lenNum');
  const slide = el('lenSlide');
  const { bars, mode } = draft;
  const endless = mode === 'endless';
  const any = mode === 'any';
  const view = el('viewLength');
  view.classList.toggle('endless', endless);
  view.classList.toggle('any', any);
  // Endless shows ∞ and "endless"; any shows "any" alone, and typing a
  // number from there picks it.
  if (document.activeElement !== num || endless) num.value = endless ? '∞' : any ? 'any' : group(bars);
  num.disabled = endless;
  num.setAttribute('aria-label', endless ? 'Bars: endless' : any ? 'Bars: any' : 'Bars');
  const time = el('lenTime');
  time.textContent = endless ? 'endless' : any ? '\u00a0' : `${bars === 1 ? 'bar' : 'bars'} · `;
  if (mode === 'bars') {
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
  slide.style.setProperty('--fill', any ? '0px' : `${(12 + (w - 24) * (s / STOPS)).toFixed(2)}px`);
  slide.setAttribute('aria-valuetext', endless ? 'endless' : any ? 'any' : lengthText(bars).replace(/ /g, ' '));
  for (const tick of el('lenTicks').children) {
    tick.classList.toggle('on', !any && Math.abs(Number(tick.dataset.stop) - s) < 1e-9);
  }
  el('lenEndless').setAttribute('aria-pressed', endless ? 'true' : 'false');
  const anyKey = el('lenAny');
  anyKey.setAttribute('aria-pressed', any ? 'true' : 'false');
  anyKey.classList.toggle('armed', any);
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
  if (digits) { draft.bars = clampBars(digits); draft.mode = 'bars'; }
  renderLength();
}

export function wirePicks(handlers) {
  api = handlers;
  el('nowPicks').addEventListener('click', () => api.setView('vibe'));
  el('picksClose').addEventListener('click', () => api.setView('now'));

  el('lenCancel').addEventListener('click', () => {
    // Nothing was changed while the picker was open, so leaving it is
    // enough: the length is still what it was before, "any" included.
    draft = null;
    api.setView('vibe');
  });
  el('lenDone').addEventListener('click', () => {
    if (document.activeElement === el('lenNum')) commitTyped();
    const value = draft.mode === 'endless' ? 'endless' : draft.mode === 'any' ? null : draft.bars;
    draft = null;
    api.setLength(value);
    api.setView('vibe');
  });
  el('lenDown').addEventListener('click', () => setDraftBars(draft.bars - 1));
  el('lenUp').addEventListener('click', () => setDraftBars(draft.bars + 1));

  const num = el('lenNum');
  num.addEventListener('focus', () => {
    if (draft.mode === 'endless') return;
    // From any, the field starts empty (leaving it empty keeps any).
    num.value = draft.mode === 'any' ? '' : String(draft.bars);
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
  slide.addEventListener('input', () => setDraftBars(pow(2, Math.round(Number(slide.value)))));
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
    setDraftBars(pow(2, Math.max(0, Math.min(STOPS, next))));
  });

  // The ∞ key lights while endless is on; pressed again, it goes back to
  // a number of bars. The any key lights while length is left to chance;
  // pressing it lit does nothing, like the panel's any keys.
  el('lenEndless').addEventListener('click', () => {
    draft.mode = draft.mode === 'endless' ? 'bars' : 'endless';
    renderLength();
  });
  el('lenAny').addEventListener('click', () => {
    if (draft.mode === 'any') return;
    draft.mode = 'any';
    renderLength();
  });
}
