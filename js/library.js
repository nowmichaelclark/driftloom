// The Library and Save viewports.
//
// Library: albums (Favorite Loops and Imported Loops built in, first), every
// loop, and More (backups, the processor switch, diagnostics). Save: which
// albums the playing loop is in, and sharing and exporting.
//
// A saved loop always lives in at least one album, so saving a loop IS
// ticking an album, and taking it out of its last album deletes it
// (storage.js has the rules).
//
// Lists scroll (design v17): the Library's lists and the Advanced page are
// the only things in the app that do. Everything else holds still.
//
// Opening an album turns the card over to show the album; leaving the album
// turns it back to whatever is playing.

import { drawCover, albumCoverSpec } from './cover.js';
import { NOTE_NAMES, SCALES } from './theory.js';
import * as store from './storage.js';
import * as ui from './ui.js';

const el = ui.el;

const lib = {
  app: null,
  tab: 'albums',
  albumId: null,
  mode: null,          // within an album: null (tracks), 'pick' or 'move'
  moving: null,        // the loop being moved
  advanced: false,     // More's Advanced page is open
  saveTab: 'albums',
  scroll: {},          // remembered scroll position per list
  armed: null,         // { key, until }: a destructive tap waiting for its second
};

// ---------------------------------------------------------------- setup

export function initLibrary(app) {
  lib.app = app;
  for (const b of document.querySelectorAll('[data-tab]')) {
    b.addEventListener('click', () => setTab(b.dataset.tab));
  }
  tabKeys(document.querySelectorAll('[data-tab]'));
  for (const b of document.querySelectorAll('[data-stab]')) {
    b.addEventListener('click', () => { lib.saveTab = b.dataset.stab; renderSave(); });
  }
  tabKeys(document.querySelectorAll('[data-stab]'));
  el('goAdvanced').addEventListener('click', () => {
    lib.advanced = true;
    render();
    el('libAdvanced').querySelector('.page-title').focus({ preventScroll: true });
  });
  el('advBack').addEventListener('click', () => {
    lib.advanced = false;
    render();
    el('goAdvanced').focus({ preventScroll: true });
  });
}

// Arrow keys move along a tab row, as tabs do everywhere.
function tabKeys(tabs) {
  const list = [...tabs];
  list.forEach((t, i) => t.addEventListener('keydown', (e) => {
    const d = { ArrowRight: 1, ArrowLeft: -1 }[e.key];
    if (!d) return;
    e.preventDefault();
    const n = list[(i + d + list.length) % list.length];
    n.focus();
    n.click();
  }));
}

function setTabState(tabs, current) {
  for (const b of tabs) {
    const on = (b.dataset.tab || b.dataset.stab) === current;
    b.setAttribute('aria-selected', String(on));
    b.tabIndex = on ? 0 : -1;
  }
}

export function isVisible() {
  return !el('viewLibrary').hidden;
}

function saveVisible() {
  return !el('viewSave').hidden;
}

// The album the card should show, or null for the playing loop.
export function shownAlbum() {
  return isVisible() && lib.tab === 'albums' ? lib.albumId : null;
}

export function setTab(tab) {
  // Tapping Albums while inside an album goes back up to the list; coming
  // to it from another tab returns to the album you were in. The same for
  // More and its Advanced page.
  if (tab === 'albums' && lib.tab === 'albums') {
    lib.albumId = null;
    lib.mode = null;
  }
  if (tab === 'more' && lib.tab === 'more') lib.advanced = false;
  lib.tab = tab;
  render();
  lib.app.cardChanged();
}

export function openAlbum(id) {
  lib.tab = 'albums';
  lib.albumId = id;
  lib.mode = null;
  render();
  lib.app.cardChanged();
}

export function closeAlbum() {
  lib.albumId = null;
  lib.mode = null;
  render();
  lib.app.cardChanged();
}

// Called whenever something the lists show may have changed. Cheap when
// neither view is on screen: it does nothing. And a list that would come
// out the same -- a new loop playing that is in none of them, say, every
// time one starts -- is left as it is rather than built again (queue item
// 40). A tap on the lists themselves always builds them.
export function refresh() {
  if (isVisible() && built.library !== shownKey()) render();
  if (saveVisible() && built.save !== shownKey()) renderSave();
}

// What the lists show, as one string: what is stored, where the lists
// stand, and what is playing.
const built = { library: null, save: null };
function shownKey() {
  const st = lib.app.state;
  const pl = st.playlist;
  const spec = st.spec;
  return JSON.stringify([
    store.loadAll().map((e) => [e.id, e.spec.name, e.spec.root, e.spec.scale, e.spec.bpm]), store.loadAlbums(),
    lib.tab, lib.albumId, lib.mode, lib.moving, lib.advanced, lib.saveTab, lib.armed && lib.armed.key,
    st.currentId, pl ? [pl.albumId, pl.index, pl.ids.length] : null,
    spec ? spec.name : null, spec ? lib.app.isSaved() && lib.app.isChanged() : null,
  ]);
}

export function showShare() {
  lib.saveTab = 'share';
  renderSave();
}

// --------------------------------------------------------------- pieces

const NB = ' ';

function make(tag, cls, text) {
  const n = document.createElement(tag);
  if (cls) n.className = cls;
  if (text != null) n.textContent = text;
  return n;
}

// A list that scrolls, keeping its place across re-renders (rows are
// rebuilt on every change). The hairline at an edge shows only while
// there is more to see past it.
function scroller(key, ...children) {
  const sc = make('div', 'scroll');
  sc.append(...children);
  sc.dataset.key = key;
  sc.addEventListener('scroll', () => { lib.scroll[key] = sc.scrollTop; edges(sc); }, { passive: true });
  return sc;
}

function edges(sc) {
  sc.classList.toggle('more-up', sc.scrollTop > 1);
  sc.classList.toggle('more-down', sc.scrollTop + sc.clientHeight < sc.scrollHeight - 1);
}

// Put a pane's new contents in place, then restore its scroll, or bring
// the row marked current into view the first time.
function mount(host, nodes) {
  host.innerHTML = '';
  host.append(...nodes);
  for (const sc of host.querySelectorAll('.scroll')) {
    const key = sc.dataset.key;
    if (lib.scroll[key] != null) sc.scrollTop = lib.scroll[key];
    else {
      const cur = sc.querySelector('.lrow.current');
      if (cur) sc.scrollTop = Math.max(0, cur.offsetTop - sc.clientHeight / 3);
    }
    edges(sc);
  }
}

function lrows(label, rows) {
  const l = make('div', 'lrows');
  l.setAttribute('role', 'list');
  l.setAttribute('aria-label', label);
  l.append(...rows);
  return l;
}

function row(title, meta, onOpen, { art = null, current = false, cls = '', num = null } = {}) {
  const r = make('div', 'lrow' + (current ? ' current' : '') + (cls ? ` ${cls}` : ''));
  r.setAttribute('role', 'listitem');
  const main = make('button', 'lrow-main');
  main.type = 'button';
  if (num != null) {
    const n = make('span', 'trk', String(num));
    n.setAttribute('aria-hidden', 'true');
    main.appendChild(n);
  }
  if (art) main.appendChild(art);
  const text = make('span', 'lrow-text');
  const t = make('span', 'lrow-title');
  if (current) t.insertAdjacentHTML('beforeend', ui.icon('nowMark').replace('<svg ', '<svg class="now-mark" '));
  const name = make('span');
  if (num != null) name.appendChild(make('span', 'vh', `${num}. `));
  name.appendChild(document.createTextNode(title));
  t.appendChild(name);
  if (current) t.appendChild(make('span', 'vh', ', now playing'));
  const m = make('span', 'lrow-meta', meta);
  text.append(t, m);
  main.appendChild(text);
  main.addEventListener('click', onOpen);
  r.appendChild(main);
  return r;
}

function key(label, onClick, { word = false, aria = '' } = {}) {
  const b = make('button', 'vkey' + (word ? ' word' : ''), label);
  b.type = 'button';
  if (aria) b.setAttribute('aria-label', aria);
  b.addEventListener('click', onClick);
  return b;
}

function backKey(label, onBack, aria) {
  const b = make('button', 'vkey back');
  b.type = 'button';
  b.innerHTML = ui.icon('left');
  b.appendChild(document.createTextNode(label));
  if (aria) b.setAttribute('aria-label', aria);
  b.addEventListener('click', onBack);
  return b;
}

function pageHead(back, title, hero = false) {
  const h = make('div', 'page-head');
  const t = make('h3', 'page-title' + (hero ? ' hero-title' : ''), title);
  t.tabIndex = -1;
  h.append(back, t);
  return h;
}

function page(...nodes) {
  const p = make('div', 'page');
  p.append(...nodes);
  return p;
}

function foot(...keys) {
  const f = make('div', 'page-foot');
  f.append(...keys);
  return f;
}

// Destructive taps ask twice. The first arms `key` (and the caller shows
// what will happen); a second tap on the same thing within a few seconds
// returns true. Survives re-renders, since rows are rebuilt on every change.
function armed(k) {
  const now = Date.now();
  if (lib.armed && lib.armed.key === k && lib.armed.until > now) {
    lib.armed = null;
    return true;
  }
  lib.armed = { key: k, until: now + 3500 };
  setTimeout(() => {
    if (lib.armed && lib.armed.key === k && lib.armed.until <= Date.now()) {
      lib.armed = null;
      refresh();
    }
  }, 3600);
  return false;
}

function isArmed(k) {
  return !!lib.armed && lib.armed.key === k && lib.armed.until > Date.now();
}

// An armed key re-words itself, and its accessible name says what a
// second press does (v17: never a pop-up).
function confirmKey(k, label, armedLabel, armedAria, onConfirm, word = true) {
  const on = isArmed(k);
  const b = key(on ? armedLabel : label, () => {
    if (armed(k)) onConfirm();
    else refresh();
  }, { word, aria: on ? armedAria : '' });
  if (on) b.classList.add('armed');
  return b;
}

function mark(on, warn = false) {
  const m = make('span', 'pick' + (warn ? ' warn' : ''));
  m.setAttribute('aria-hidden', 'true');
  m.innerHTML = warn ? ui.icon('close') : on ? ui.icon('check') : '';
  return m;
}

// Album art is drawn once per album contents and copied after that, so
// re-rendering a list does not redraw noise fields every time.
//
// And in a list it is drawn one album at a time, a task each, after the
// list is up: a Library's worth at once held the main thread for a second
// and more on a slow phone, past what the music had queued (queue item
// 40). A row's art comes in a moment after the row the first time; `warmArt`
// draws them ahead, while nothing else is happening, so mostly it is there.
const artCache = new Map();
const artWaiting = new Map();
let artTimer = null;

function artKey(album, size) {
  return `${album.title}|${album.ids.join(',')}|${size}`;
}

function drawArt(album, specs, size) {
  const src = document.createElement('canvas');
  try {
    drawCover(src, albumCoverSpec(album.title, specs.length ? specs : [
      { seed: 1, feel: { lift: 0.5, energy: 0.3, warmth: 0.6 }, mix: { dust: 1 } },
    ]), size);
  } catch (err) {
    console.warn('Could not draw album art', err);
  }
  if (artCache.size > 40) artCache.clear();
  artCache.set(artKey(album, size), src);
  return src;
}

// At once: the card's art.
export function albumArt(album, specs, size = 96) {
  return artCache.get(artKey(album, size)) || drawArt(album, specs, size);
}

function paintArt(canvas, src) {
  canvas.getContext('2d').drawImage(src, 0, 0, canvas.width, canvas.height);
}

// Into `canvas` now if it is drawn, otherwise when its turn comes.
function artInto(canvas, album, specs, size = 96) {
  const k = artKey(album, size);
  const src = artCache.get(k);
  if (src) {
    if (canvas) paintArt(canvas, src);
    return;
  }
  let w = artWaiting.get(k);
  if (!w) {
    w = { album, specs, size, canvases: [] };
    artWaiting.set(k, w);
  }
  if (canvas) w.canvases.push(canvas);
  artNext();
}

function artNext() {
  if (artTimer || !artWaiting.size) return;
  // Hidden, nothing is looking: the next time the page is shown.
  if (document.hidden) {
    artTimer = true;
    document.addEventListener('visibilitychange', () => { artTimer = null; artNext(); }, { once: true });
    return;
  }
  artTimer = setTimeout(() => {
    artTimer = null;
    if (!artWaiting.size) return;
    if (document.hidden) { artNext(); return; }
    const [k, w] = artWaiting.entries().next().value;
    artWaiting.delete(k);
    const src = artCache.get(k) || drawArt(w.album, w.specs, w.size);
    for (const c of w.canvases) if (c.isConnected) paintArt(c, src);
    artNext();
  }, 0);
}

// Every album's art for the lists, drawn ahead, one a task.
export function warmArt() {
  const byId = loopsById();
  for (const album of listedAlbums(store.IMPORTED)) {
    artInto(null, album, album.ids.map((id) => byId.get(id)).filter(Boolean).map((e) => e.spec));
  }
}

function artFor(album, byId, big = false) {
  const specs = album.ids.map((id) => byId.get(id)).filter(Boolean).map((e) => e.spec);
  const art = document.createElement('canvas');
  art.className = 'art' + (big ? ' big' : '');
  art.setAttribute('aria-hidden', 'true');
  art.width = 96;
  art.height = 96;
  artInto(art, album, specs);
  return art;
}

function loopsById() {
  return new Map(store.loadAll().map((e) => [e.id, e]));
}

function keyOf(spec) {
  return `${NOTE_NAMES[spec.root]}${NB}${SCALES[spec.scale].label} · ${spec.bpm}${NB}bpm`;
}

function count(n, word) {
  return `${n}${NB}${word}${n === 1 ? '' : 's'}`;
}

function empty(text) {
  return make('p', 'empty', text);
}

// Albums worth listing: Imported Loops only when it has something in it.
function listedAlbums(alsoShow = null) {
  return store.loadAlbums().filter((a) => a.id !== store.IMPORTED || a.ids.length || a.id === alsoShow);
}

function newAlbumThen(after) {
  const title = prompt('Name this album', 'Untitled');
  if (!title || !title.trim()) return;
  const album = store.createAlbum(title.trim().slice(0, store.NAME_MAX));
  if (!album) { ui.toast('Could not create the album'); return; }
  after(album);
}

// ------------------------------------------------------------- library

export function render() {
  built.library = shownKey();
  setTabState(document.querySelectorAll('[data-tab]'), lib.tab);
  const more = lib.tab === 'more';
  el('libMore').hidden = !more || lib.advanced;
  el('libAdvanced').hidden = !more || !lib.advanced;
  el('libPane').hidden = more;
  if (more) return;
  if (lib.tab === 'albums') {
    const album = lib.albumId && store.loadAlbums().find((a) => a.id === lib.albumId);
    if (lib.albumId && !album) { lib.albumId = null; lib.mode = null; }
    if (!album) renderAlbums();
    else if (lib.mode === 'pick') renderPicker(album);
    else if (lib.mode === 'move' && lib.moving) renderMove(album);
    else renderAlbum(album);
  } else {
    renderLoops();
  }
}

function renderAlbums() {
  const byId = loopsById();
  const playing = lib.app.state.playlist;
  const actions = make('div', 'lib-actions');
  actions.append(
    key('new album', () => newAlbumThen((a) => { ui.toast(`Created ${a.title}`); render(); })),
    key('paste a code', () => lib.app.pasteCode()),
  );
  const rows = listedAlbums().map((album) => {
    const n = album.ids.filter((id) => byId.has(id)).length;
    const isPlaying = !!playing && playing.albumId === album.id;
    const meta = count(n, 'loop') + (isPlaying ? ` · playing ${playing.index + 1}${NB}of${NB}${playing.ids.length}` : '');
    const r = row(album.title, meta, () => openAlbum(album.id),
      { art: artFor(album, byId), current: isPlaying, cls: 'album' });
    r.appendChild(key('play', () => lib.app.playAlbum(album.id, 0), { word: true, aria: `Play ${album.title}` }));
    return r;
  });
  mount(el('libPane'), [page(scroller('albums', actions,
    rows.length ? lrows('Albums', rows) : empty('No albums yet.')))]);
}

function renderAlbum(album) {
  const byId = loopsById();
  const present = album.ids.filter((id) => byId.has(id));
  const playing = lib.app.state.playlist;
  const here = !!playing && playing.albumId === album.id;

  const hero = make('div', 'hero');
  const meta = make('p', 'hero-meta', count(present.length, 'loop')
    + (here ? ` · playing ${playing.index + 1}${NB}of${NB}${playing.ids.length}` : ''));
  const keys = make('div', 'hero-keys');
  keys.append(
    // While this album is playing the same key takes you out of it, so
    // Next goes back to making new loops.
    here ? key('stop album', () => lib.app.leaveAlbum())
      : key('play album', () => lib.app.playAlbum(album.id, 0)),
    key('add loops', () => { lib.mode = 'pick'; render(); }),
    key('share', () => lib.app.shareAlbum(album.id)),
  );
  hero.append(artFor(album, byId, true), meta, keys);

  const rows = present.map((id, i) => {
    const spec = byId.get(id).spec;
    const isCurrent = here && playing.index === i;
    const r = row(spec.name, keyOf(spec), () => lib.app.playAlbum(album.id, i),
      { current: isCurrent, cls: 'track', num: i + 1 });
    const rk = make('span', 'row-keys');
    rk.appendChild(key('move', () => {
      lib.mode = 'move';
      lib.moving = id;
      render();
    }, { word: true, aria: `Move ${spec.name} to another album` }));
    const last = store.albumsOf(id).length <= 1;
    rk.appendChild(confirmKey(`rm:${album.id}:${id}`, 'remove', last ? 'delete?' : 'sure?',
      last ? `delete? Press again to delete ${spec.name}` : `sure? Press again to remove ${spec.name} from ${album.title}`,
      () => {
        const res = store.removeFromAlbum(album.id, id);
        if (!res.ok) { ui.toast('Could not change that album'); return; }
        if (res.deleted && lib.app.state.currentId === id) lib.app.state.currentId = null;
        ui.toast(res.deleted ? `Deleted ${spec.name}` : 'Removed from album');
        lib.app.savedChanged();
        lib.app.cardChanged();
      }));
    r.appendChild(rk);
    return r;
  });

  const parts = [hero, rows.length ? lrows(`Loops in ${album.title}`, rows)
    : empty('Nothing in here yet. Tap add loops to pick from your loops.')];
  if (!store.isBuiltin(album.id)) {
    const k = `del-album:${album.id}`;
    const going = store.onlyHere(album.id).length;
    const ask = going ? `delete it and ${count(going, 'loop')} only here?` : 'delete album?';
    const end = make('div', 'end-keys');
    end.appendChild(confirmKey(k, 'delete album', ask, `${ask} Press again to delete ${album.title}`, () => {
      const pl = lib.app.state.playlist;
      if (pl && pl.albumId === album.id) lib.app.state.playlist = null;
      store.removeAlbum(album.id);
      ui.toast('Album deleted');
      lib.app.savedChanged();
      closeAlbum();
    }, false));
    parts.push(end);
  }
  mount(el('libPane'), [page(
    pageHead(backKey('Albums', closeAlbum, 'Back to Albums'), album.title, true),
    scroller(`album:${album.id}`, ...parts),
  )]);
}

// Move one track: tap the album it should go to.
function renderMove(album) {
  const byId = loopsById();
  const entry = byId.get(lib.moving);
  if (!entry) { lib.mode = null; render(); return; }
  const done = () => { lib.mode = null; lib.moving = null; render(); };
  const go = (target) => {
    if (!store.moveToAlbum(entry.id, album.id, target.id)) { ui.toast('Could not move it'); return; }
    ui.toast(`Moved to ${target.title}`);
    lib.app.savedChanged();
    lib.app.cardChanged();
    done();
  };
  // Imported Loops is where codes land, not somewhere to file things.
  const targets = store.loadAlbums().filter((a) => a.id !== album.id && a.id !== store.IMPORTED);
  const rows = targets.map((target) => {
    const has = target.ids.includes(entry.id);
    return row(target.title, has ? 'already has it' : count(target.ids.length, 'loop'),
      () => go(target), { art: artFor(target, byId) });
  });
  mount(el('libPane'), [page(
    pageHead(backKey(album.title, done, `Back to ${album.title}`), `Move ${entry.spec.name} to…`),
    scroller(`move:${album.id}`, rows.length ? lrows('Albums', rows) : empty('No other albums yet. Make one with new album.')),
    foot(key('new album', () => newAlbumThen(go)), key('cancel', done)),
  )]);
}

// Pick loops into the album without playing them: tap a row to put it in,
// tap again to take it out (twice, if that would delete it).
function renderPicker(album) {
  const done = () => { lib.mode = null; render(); };
  const loops = store.loadAll();
  const fresh = store.loadAlbums().find((a) => a.id === album.id) || album;
  const rows = loops.map((entry) => {
    const inAlbum = fresh.ids.includes(entry.id);
    const last = inAlbum && store.albumsOf(entry.id).length <= 1;
    const k = `pick:${album.id}:${entry.id}`;
    const warn = last && isArmed(k);
    const r = row(entry.spec.name, warn ? 'only here: tap again to delete' : keyOf(entry.spec), () => {
      if (!inAlbum) {
        store.addToAlbum(album.id, entry.id);
      } else if (last && !armed(k)) {
        render();
        return;
      } else {
        const res = store.removeFromAlbum(album.id, entry.id);
        if (res.deleted) {
          if (lib.app.state.currentId === entry.id) lib.app.state.currentId = null;
          ui.toast(`Deleted ${entry.spec.name}`);
        }
      }
      lib.app.savedChanged();
      lib.app.cardChanged();
    }, { current: entry.id === lib.app.state.currentId });
    if (inAlbum) r.classList.add('in');
    r.querySelector('.lrow-main').setAttribute('aria-pressed', String(inAlbum));
    r.querySelector('.lrow-main').appendChild(mark(inAlbum, warn));
    return r;
  });
  const keys = [];
  const s = lib.app.state.spec;
  if (s && !lib.app.isSaved()) {
    keys.push(key(`add ${s.name}`, () => { lib.app.saveIn(album.id); render(); }));
  }
  keys.push(key('done', done));
  mount(el('libPane'), [page(
    pageHead(backKey(album.title, done, `Back to ${album.title}`), 'Add loops'),
    scroller(`pick:${album.id}`, rows.length ? lrows('Your loops', rows) : empty('No loops yet. Save one you like and it will show up here.')),
    foot(...keys),
  )]);
}

// Every loop, wherever it lives. Tap to play; changes happen in albums.
function renderLoops() {
  const loops = store.loadAll();
  const albums = store.loadAlbums();
  const currentId = lib.app.state.currentId;
  const rows = loops.map((entry) => {
    const homes = albums.filter((a) => a.ids.includes(entry.id));
    const where = homes.length ? homes[0].title + (homes.length > 1 ? `${NB}+${homes.length - 1}` : '') : '';
    return row(entry.spec.name, `${keyOf(entry.spec)}${where ? ` · ${where}` : ''}`, () => lib.app.openSaved(entry),
      { current: entry.id === currentId });
  });
  mount(el('libPane'), [page(scroller('loops',
    rows.length ? lrows('All loops', rows) : empty('No loops yet. Save one you like and it will show up here.')))]);
}

// ----------------------------------------------------------------- save

export function renderSave() {
  built.save = shownKey();
  setTabState(document.querySelectorAll('[data-stab]'), lib.saveTab);
  const share = lib.saveTab === 'share';
  el('saveShare').hidden = !share;
  el('savePane').hidden = share;
  if (share) return;
  const app = lib.app;
  const spec = app.state.spec;
  if (!spec) { mount(el('savePane'), [empty('Press play to make a loop first.')]); return; }
  const kept = app.isSaved();
  const id = kept ? app.state.currentId : null;

  // A saved loop that has been re-rolled or tweaked since: say so, and offer
  // both ways forward rather than guessing.
  const head = make('div', 'save-head');
  if (kept && app.isChanged()) {
    const homes = store.albumsOf(id).length;
    head.classList.add('changed');
    head.append(make('p', 'hint', 'Changed since you saved it.'),
      key(homes > 1 ? `update in ${homes} albums` : 'update it', () => app.updateSaved()),
      key('save as new', () => app.saveAsNew()));
  } else if (!kept) {
    head.append(make('p', 'hint', `Tick an album to save ${spec.name} to it.`));
  } else {
    head.append(make('p', 'hint', `${spec.name} is saved. Untick to take it out.`));
  }

  const byId = loopsById();
  const albums = listedAlbums(store.IMPORTED);
  const shown = albums.filter((a) => a.id !== store.IMPORTED || (id && a.ids.includes(id)));
  const rows = shown.map((album) => {
    const inAlbum = !!id && album.ids.includes(id);
    const last = inAlbum && store.albumsOf(id).length <= 1;
    const k = `save:${album.id}:${id}`;
    const warn = last && isArmed(k);
    const meta = warn ? 'only album: tap again to delete' : count(album.ids.filter((x) => byId.has(x)).length, 'loop');
    const r = row(album.title, meta, () => {
      if (!inAlbum) { app.saveIn(album.id); return; }
      if (last && !armed(k)) { renderSave(); return; }
      app.unsaveFrom(album.id);
    }, { art: artFor(album, byId) });
    if (inAlbum) r.classList.add('in');
    const main = r.querySelector('.lrow-main');
    main.setAttribute('role', 'checkbox');
    main.setAttribute('aria-checked', String(inAlbum));
    main.appendChild(mark(inAlbum, warn));
    return r;
  });

  mount(el('savePane'), [page(
    head,
    scroller('save', rows.length ? lrows('Albums', rows) : empty('No albums.')),
    foot(key('new album', () => newAlbumThen((a) => {
      app.saveIn(a.id);
      ui.toast(`Saved to ${a.title}`);
    }))),
  )]);
}
