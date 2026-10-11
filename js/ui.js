import { LAYERS, LAYER_LABELS, STEPS_PER_BAR } from './generator.js';
import { CHARACTERS, resolveKey } from './characters.js';
import { NOTE_NAMES, SCALES } from './theory.js';
import { moodWord, pointWord } from './moods.js';

export const el = (id) => document.getElementById(id);

// ------------------------------------------------------------- readout

// The grid is rebuilt whenever the metre changes, because a bar of 6/8 is
// twelve cells, not sixteen with four dead ones on the end.
export function buildPlayhead(steps = STEPS_PER_BAR) {
  const host = el('playhead');
  host.innerHTML = '';
  host.style.gridTemplateColumns = `repeat(${steps}, 1fr)`;
  for (let i = 0; i < steps; i++) {
    const light = document.createElement('i');
    if (i % 4 === 0) light.className = 'beat';
    host.appendChild(light);
  }
  return host.children;
}

const METER = { 12: '6/8', 16: '4/4', 20: '5/4', 14: '7/8' };

// Both the profile blend and the feeling are mixtures, so show them as
// mixtures. Percentages are more honest than one adjective standing in for a
// loop that is 60% one thing and 30% another.
function asMixParts(weights, nameOf, max = 3) {
  return Object.entries(weights)
    .filter(([, w]) => w >= 0.05)
    .sort((a, b) => b[1] - a[1])
    .slice(0, max)
    .map(([k, w]) => `${Math.round(w * 100)}% ${nameOf(k)}`);
}

function mixParts(spec) {
  if (!spec.mix) {
    const c = CHARACTERS[resolveKey(spec.character || 'dust')];
    return [c ? c.label : 'Dust'];
  }
  const named = {};
  for (const [k, w] of Object.entries(spec.mix)) {
    const key = resolveKey(k);
    named[key] = (named[key] || 0) + w;
  }
  const parts = asMixParts(named, (k) => CHARACTERS[k].label);
  return parts.length ? parts : ['Dust'];
}

function feelParts(spec) {
  if (spec.feelMix && Object.keys(spec.feelMix).length) {
    return asMixParts(spec.feelMix, moodWord);
  }
  // Saves from before feeling became a mixture.
  return [pointWord(spec.feel || { lift: spec.mood ?? 0.5, energy: 0.5, warmth: 0.6 })];
}

// One fact per line, as the card lays them out: the blend, the feeling,
// then key, tempo, meter and form. `extra` adds the album being played and
// the pass count when a track length is set; both are what someone glancing
// at the card would otherwise have to go looking for.
export function renderReadout(spec, pattern, extra = {}) {
  setName(spec.name);
  const meter = METER[spec.stepsPerBar || 16] || `${spec.stepsPerBar}/16`;
  const shape = [];
  if (pattern && pattern.form) shape.push('airy');
  if (spec.cycles) shape.push('drifting');
  const parts = {
    mix: mixParts(spec),
    feel: feelParts(spec),
    key: `${NOTE_NAMES[spec.root]} ${SCALES[spec.scale].label}`,
    bpm: `${spec.bpm} bpm`,
    meter: `${meter} · ${spec.bars} bars`,
    shape: shape.join(' · '),
    album: extra.album || '',
    pass: extra.pass || '',
  };
  // The card is as tall as its art and no taller, so a loop with a lot to
  // say says it more compactly instead of pushing the viewport down. Each
  // level folds a little more together; the first that fits wins.
  for (let level = 0; level <= 5; level++) {
    setLines(readoutLines(parts, level));
    if (level === 5) el('loopDetail').classList.add('tight');
    if (!overflowing()) break;
  }
  const bpm = el('bpmVal');
  if (bpm) bpm.textContent = spec.bpm;
}

// Folding never joins two long facts onto one line, since the column is
// narrow and a cut-off tempo is worse than a missing third feeling. In
// order: the lesser feelings go, then the third style, then the form word
// joins the tempo, then the second style, then the type gets smaller.
function readoutLines(p, level) {
  const lines = [];
  const feel = level >= 1 ? p.feel.slice(0, 1) : p.feel;
  const mix = level >= 4 ? p.mix.slice(0, 1) : level >= 2 ? p.mix.slice(0, 2) : p.mix;
  for (const m of mix) lines.push([m, 'mix']);
  for (const f of feel) lines.push([f, 'feel']);
  lines.push([p.key, 'key']);
  if (p.shape && level >= 3) {
    lines.push([`${p.bpm} · ${p.shape}`]);
  } else {
    lines.push([p.bpm]);
  }
  lines.push([p.meter]);
  if (p.shape && level < 3) lines.push([p.shape]);
  if (p.album) lines.push([p.album, 'album-line']);
  if (p.pass) lines.push([p.pass, 'faint']);
  return lines;
}

function overflowing() {
  const info = el('loopDetail').parentElement;
  return info.scrollHeight > info.clientHeight + 1;
}

// The card turned over to show an album: its art is drawn by the caller,
// this fills in the words.
export function renderAlbumCard(title, specs, extra = {}) {
  setName(title);
  const lines = [[`${specs.length} ${specs.length === 1 ? 'track' : 'tracks'}`, 'mix']];
  if (specs.length) {
    // The album's blend is the average of its loops' blends.
    const named = {};
    for (const spec of specs) {
      const mix = spec.mix || { [spec.character || 'dust']: 1 };
      for (const [k, w] of Object.entries(mix)) {
        const key = resolveKey(k);
        named[key] = (named[key] || 0) + w / specs.length;
      }
    }
    for (const part of asMixParts(named, (k) => (CHARACTERS[k] ? CHARACTERS[k].label : k), 2)) {
      lines.push([part, 'feel']);
    }
    const bpms = specs.map((s) => s.bpm);
    const lo = Math.min(...bpms);
    const hi = Math.max(...bpms);
    lines.push([lo === hi ? `${lo} bpm` : `${lo}-${hi} bpm`]);
    const secs = specs.reduce((t, s) => t + passSeconds(s), 0);
    lines.push([`~${formatTime(secs)} once through`]);
  }
  if (extra.playing) lines.push([extra.playing, 'album-line']);
  setLines(lines);
  if (overflowing()) el('loopDetail').classList.add('tight');
}

export function passSeconds(spec) {
  return spec.bars * (spec.stepsPerBar || 16) * (60 / spec.bpm / 4);
}

export function formatTime(secs) {
  if (secs < 90) return `${Math.round(secs)}s`;
  if (secs < 5400) return `${Math.round(secs / 60)}m`;
  if (secs < 86400) {
    const h = Math.floor(secs / 3600);
    const m = Math.round((secs % 3600) / 60);
    return m ? `${h}h ${m}m` : `${h}h`;
  }
  const d = Math.floor(secs / 86400);
  const h = Math.round((secs % 86400) / 3600);
  return h ? `${d}d ${h}h` : `${d}d`;
}

// The name is as big as fits on one line of the card, down to a floor;
// only a name too long even at the floor wraps. Breaking "mun-dain" at its
// hyphen to keep a size reads worse than a slightly smaller name.
export function setName(name) {
  const h = el('loopName');
  h.textContent = name;
  fitName();
}

export function fitName() {
  const h = el('loopName');
  if (h.hidden || !h.clientWidth) return;
  h.classList.remove('wrap', 'break');
  h.style.fontSize = '';
  const max = parseFloat(getComputedStyle(h).fontSize) || 32;
  let size = max;
  const set = (px) => { size = px; h.style.fontSize = `${px}px`; };
  // One line, as large as fits, down to a size that still reads as a title.
  const oneLine = () => h.scrollWidth <= h.clientWidth + 1;
  while (!oneLine() && size > NAME_ONE_LINE_MIN) set(size - 1);
  if (oneLine()) return;
  // Otherwise two lines, as large as lets both fit. Names are capped at
  // NAME_MAX characters, so this always lands well above the floor; the
  // CSS clamps to two lines in case an old, longer name does not.
  h.classList.add('wrap');
  set(max);
  const twoLines = () => h.scrollWidth <= h.clientWidth + 1
    && h.scrollHeight <= Math.ceil(size * 1.1 * 2) + 2;
  while (!twoLines() && size > NAME_TWO_LINE_MIN) set(size - 1);
  // One unbroken word too long even at the floor: only then split it.
  h.classList.toggle('break', !twoLines());
}

const NAME_ONE_LINE_MIN = 21;
const NAME_TWO_LINE_MIN = 14;

// Refit when the card changes width (rotation, a resized window, fonts
// arriving late), not just when the name changes.
if (typeof ResizeObserver !== 'undefined') {
  let lastWidth = 0;
  const ro = new ResizeObserver((entries) => {
    const w = entries[0].contentRect.width;
    if (Math.abs(w - lastWidth) < 1) return;
    lastWidth = w;
    fitName();
  });
  const watch = () => {
    const host = el('loopName');
    if (host && host.parentElement) ro.observe(host.parentElement);
  };
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', watch);
  else watch();
}

function setLines(lines) {
  const host = el('loopDetail');
  host.classList.remove('tight');
  host.innerHTML = '';
  for (const [text, cls] of lines) {
    const li = document.createElement('li');
    li.textContent = text;
    if (cls) li.className = cls;
    host.appendChild(li);
  }
}

// -------------------------------------------------------------- layers

// Each layer shows one bar at sixteenth resolution, as a row of dots (the
// v17 design): a quiet guide dot on every step, a note where one sounds, a
// bigger note for an accent. The playhead's column gets a halo on every
// layer that is on. Showing the whole loop at once would squash a four-bar
// pattern into slivers on a phone, so the rows follow the playhead bar by
// bar instead.
let columns = [];

export function buildLayers(handlers, steps = STEPS_PER_BAR) {
  const host = el('layerList');
  host.innerHTML = '';
  const cells = {};
  columns = Array.from({ length: steps }, () => []);

  for (const layer of LAYERS) {
    const label = LAYER_LABELS[layer];
    const row = document.createElement('div');
    row.className = 'layer';
    row.dataset.layer = layer;

    const name = document.createElement('div');
    name.className = 'lbl';
    name.id = `lbl-${layer}`;
    name.textContent = label;

    const grid = document.createElement('div');
    grid.className = 'pattern';
    grid.setAttribute('role', 'img');
    grid.setAttribute('aria-label', `${label} pattern`);
    grid.style.gridTemplateColumns = `repeat(${steps}, 1fr)`;
    const dots = [];
    for (let i = 0; i < steps; i++) {
      const d = document.createElement('span');
      grid.appendChild(d);
      dots.push(d);
      columns[i].push(d);
    }
    cells[layer] = dots;

    const reroll = document.createElement('button');
    reroll.type = 'button';
    reroll.className = 'btn-roll';
    reroll.textContent = 'roll';
    reroll.setAttribute('aria-label', `Roll ${label}`);
    reroll.addEventListener('click', () => handlers.onReroll(layer));

    const mute = document.createElement('button');
    mute.type = 'button';
    mute.className = 'btn-on';
    mute.setAttribute('role', 'switch');
    mute.setAttribute('aria-checked', 'true');
    mute.setAttribute('aria-labelledby', name.id);
    mute.innerHTML = '<span aria-hidden="true">on</span>';
    mute.addEventListener('click', () => handlers.onMute(layer));

    row.append(name, grid, reroll, mute);
    host.appendChild(row);
  }
  lastCursor = -1;
  return cells;
}

function stepsForBar(pattern, layer, bar) {
  const spb = pattern.stepsPerBar || STEPS_PER_BAR;
  const out = new Array(spb).fill(0);
  // With polymeter a layer wraps on its own cycle, so ask where it actually
  // is rather than assuming it shares the pattern's bar lines.
  const cycle = (pattern.cycles && pattern.cycles[layer]) || pattern.totalSteps;
  const start = (bar * spb) % cycle;
  for (const e of pattern.tracks[layer]) {
    if (!e.vel) continue;
    const idx = ((e.step - start) % cycle + cycle) % cycle;
    if (idx >= spb) continue;
    out[idx] = Math.max(out[idx], e.vel);
  }
  return out;
}

export function renderGrids(cells, pattern, bar, mutes) {
  for (const layer of LAYERS) {
    const vels = stepsForBar(pattern, layer, bar);
    const dots = cells[layer];
    let notes = 0;
    for (let i = 0; i < dots.length; i++) {
      const v = vels[i];
      if (v) notes++;
      const on = !!v;
      const big = v > 0.55;
      // Only the dots that change. Writing a class, even the same one,
      // marks the element for style and paint.
      if (dots[i].classList.contains('on') !== on) dots[i].classList.toggle('on', on);
      if (dots[i].classList.contains('big') !== big) dots[i].classList.toggle('big', big);
    }
    const row = document.querySelector(`.layer[data-layer="${layer}"]`);
    const off = !!mutes[layer];
    row.classList.toggle('dim', off);
    const sw = row.querySelector('.btn-on');
    const want = off ? 'false' : 'true';
    if (sw.getAttribute('aria-checked') !== want) {
      sw.setAttribute('aria-checked', want);
      sw.firstChild.textContent = off ? 'off' : 'on';
    }
    const pat = row.querySelector('.pattern');
    pat.setAttribute('aria-label', `${LAYER_LABELS[layer]} pattern: ${notes} ${notes === 1 ? 'note' : 'notes'} in ${dots.length} steps`);
  }
}

let lastCursor = -1;

export function resetCursor() {
  lastCursor = -1;
}

// Only the column being left and the column being entered change: the lit
// cell on the card, and the halo on each layer's dot in that column. The
// lit colour and glow are pseudo-elements and shadows switched by class,
// with no fade, so a step is a handful of class writes.
export function moveCursor(lights, cells, index) {
  if (index === lastCursor) return;
  if (lastCursor >= 0) {
    if (lights[lastCursor]) lights[lastCursor].classList.remove('on');
    if (columns[lastCursor]) for (const d of columns[lastCursor]) d.classList.remove('at');
  }
  if (index >= 0) {
    if (lights[index]) lights[index].classList.add('on');
    if (columns[index]) for (const d of columns[index]) d.classList.add('at');
  }
  lastCursor = index;
}

// --------------------------------------------------------------- misc

let toastTimer = null;
export function toast(message) {
  const t = el('toast');
  t.textContent = message;
  t.classList.add('show');
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => t.classList.remove('show'), 2200);
}

export function download(blob, filename) {
  // Loop names are user-editable, and a slash or colon in one either breaks
  // the download or silently writes somewhere unexpected.
  const safe = filename
    .replace(/[\\/:*?"<>|\u0000-\u001f]/g, '-')
    .replace(/\s+/g, ' ')
    .replace(/^[.\s]+/, '')
    .slice(0, 120) || 'driftloom.dat';
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = safe;
  document.body.appendChild(a);
  a.click();
  a.remove();
  setTimeout(() => URL.revokeObjectURL(url), 4000);
}

// --------------------------------------------------------------- icons

// Line icons on a 24-unit grid, drawn in the button's text color so the
// theme decides how they look, not the icon. Anything marked `solid` is
// filled instead of stroked. The transport and view keys are the v17
// design's own glyphs.
const ICONS = {
  prev: '<polygon class="solid" points="19 20 9 12 19 4 19 20"/><line x1="5" y1="19" x2="5" y2="5"/>',
  next: '<polygon class="solid" points="5 4 15 12 5 20 5 4"/><line x1="19" y1="5" x2="19" y2="19"/>',
  play: '<polygon class="solid" stroke-width="1" points="7 4 20 12 7 20"/>',
  pause: '<rect class="solid" stroke-width="1" x="6.5" y="4" width="4" height="16" rx="1"/>'
    + '<rect class="solid" stroke-width="1" x="13.5" y="4" width="4" height="16" rx="1"/>',
  // Books on a shelf, one leaning.
  library: '<path d="M6 4v15"/><path d="M10.5 4v15"/><path d="M15 5.2 17.6 19"/><path d="M4 20h16"/>',
  // A bookmark with an arrow out of it: save, or send it on.
  saveShare: '<path d="M17.5 21 12 17 6.5 21V5a2 2 0 0 1 2-2h7a2 2 0 0 1 2 2z"/><path d="M12 15V9"/><path d="m9.2 11.8 2.8-2.8 2.8 2.8"/>',
  // A note whose flag is a wave: what is playing now.
  now: '<circle cx="6.5" cy="17" r="2.6"/><path d="M9.1 17V5.5"/><path d="M9.1 5.5c1.5-2 3-2 4.5 0s3 2 4.5 0"/>',
  nowMark: '<circle class="solid" cx="6.5" cy="17" r="2.6"/><path d="M9.1 17V5.5"/><path d="M9.1 5.5c1.5-2 3-2 4.5 0s3 2 4.5 0"/>',
  check: '<path d="m5 12.5 4.5 4.5L19 7.5"/>',
  close: '<path d="M18 6 6 18"/><path d="m6 6 12 12"/>',
  plus: '<path d="M12 5v14"/><path d="M5 12h14"/>',
  bookmark: '<path d="M18 21l-6-4.5L6 21V5a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2z"/>',
  // Three joined dots: the share mark phones already use.
  share: '<circle cx="18" cy="5" r="2.6"/><circle cx="6" cy="12" r="2.6"/><circle cx="18" cy="19" r="2.6"/>'
    + '<path d="m8.3 13.3 7.4 4.4"/><path d="m15.7 6.3-7.4 4.4"/>',
  // Three sliders: the sound controls.
  sounds: '<line x1="4" y1="6" x2="20" y2="6"/><line x1="4" y1="12" x2="20" y2="12"/><line x1="4" y1="18" x2="20" y2="18"/>'
    + '<circle class="solid" cx="9" cy="6" r="2"/><circle class="solid" cx="15" cy="12" r="2"/><circle class="solid" cx="7" cy="18" r="2"/>',
  layers: '<path d="M12 3.5 21 8l-9 4.5L3 8z"/><path d="m3 12.5 9 4.5 9-4.5"/><path d="m3 16.5 9 4.5 9-4.5"/>',
  left: '<path d="m14.5 6-6 6 6 6"/>',
  right: '<path d="m9.5 6 6 6-6 6"/>',
};

export function icon(name) {
  const body = ICONS[name].replace(/class="solid"/g, 'fill="currentColor"');
  return `<svg viewBox="0 0 24 24" aria-hidden="true" focusable="false" fill="none" stroke="currentColor"
    stroke-width="2" stroke-linecap="round" stroke-linejoin="round">${body}</svg>`;
}

// Only rewrites when the icon actually changes, since it is called on every
// load and every play/stop.
export function setIcon(button, name) {
  if (button.dataset.icon === name) return;
  button.dataset.icon = name;
  button.innerHTML = icon(name);
}
