import { newSpec as drawSpec, RequestError, vibeSelection, rerollLayer, cloneSpec, render, choirOf, LAYERS, STEPS_PER_BAR } from './generator.js';
import { Synth } from './synth.js';
import { ENGINE, loadCore, fetchCore, coreUnsupported, newContext, DROP_CAUSES, coreLog } from './core.js';
import { Engine } from './engine.js';
import { patternToMidi } from './midi.js';
import { MediaBridge } from './media.js';
import * as share from './share.js';

// Build stamp. Shown in Diagnostics so that after a deploy you can confirm
// in one glance which version you are actually running, rather than
// guessing whether a change landed. Bump it with CACHE in sw.js.
const BUILD = 'v135';

// Reported in Diagnostics. Declared here rather than beside the registration
// at the foot of the file so it is initialised before anything can read it.
let swState = 'unsupported';
// Every change in the service worker's state, with when it came (queue
// item 41): a field report said `installing` while the music played, and
// one word cannot tell a stuck install from a finished one.
const swEvents = [];
function swEvent(what) {
  if (swEvents.length >= 12) swEvents.splice(1, 1);
  swEvents.push(`${what} at ${(performance.now() / 1000).toFixed(1)} s`);
}

// The GPU the browser draws with, as WebGL names it, and whether that is
// a real one (queue item 41). Asked once, on a context let go at once.
let gpuInfo = null;
function gpuReport() {
  if (gpuInfo) return gpuInfo;
  gpuInfo = { renderer: '-', accelerated: '-' };
  try {
    const gl = document.createElement('canvas').getContext('webgl');
    if (!gl) {
      gpuInfo = { renderer: 'no WebGL', accelerated: 'no' };
      return gpuInfo;
    }
    const ext = gl.getExtension('WEBGL_debug_renderer_info');
    const renderer = String(gl.getParameter(ext ? ext.UNMASKED_RENDERER_WEBGL : gl.RENDERER));
    const soft = /swiftshader|llvmpipe|software|softpipe|microsoft basic/i.test(renderer);
    gpuInfo = { renderer, accelerated: soft ? 'no (software)' : 'yes' };
    const lose = gl.getExtension('WEBGL_lose_context');
    if (lose) lose.loseContext();
  } catch { /* not in this browser */ }
  return gpuInfo;
}
// Track length is chosen as a time, which is how anyone thinks of it, and
// stored as passes (a track ends on a whole loop), worked out at this tempo
// when the slider moves. The stops are curated: fine at the short end, where
// people actually set it, and reaching a whole day at the top.
const TRACK_TIMES = [
  0, 60, 120, 180, 300, 600, 900, 1200, 1800, 2700,
  3600, 5400, 7200, 10800, 14400, 21600, 28800, 43200, 86400,
];
import { coverImage, paint } from './covers.js';
import * as store from './storage.js';
import * as ui from './ui.js';
import * as library from './library.js';
import { VIBE_IDS } from './vibes.js';
import * as picks from './picks.js';

const state = {
  ctx: null,
  synth: null,
  engine: null,
  spec: null,
  pattern: null,
  currentId: null,
  bar: -1,
  gridSteps: 16,
  playlist: null,
  // Which viewport is up ('now', 'sounds', 'layers', 'library' or 'save');
  // the last of the first three, where leaving Library or Save goes back
  // to; and the last of Sound and Layers, which the switch offers.
  view: 'now',
  lastView: 'now',
  lastEdit: 'layers',
  // The listener's vibes (queue item 38b), in VIBE_IDS order. Every new
  // loop leans toward them; saved in the prefs.
  vibes: [],
  // The other next picks (design/next-picks.md): a tempo range and a
  // length in bars (or 'endless'); null is "any". Saved in the prefs.
  picks: { tempo: null, length: null },
  // The album most recently left, so Now Playing can offer the way back.
  lastAlbum: null,
  // Now Playing's last audible position, in steps, held while paused.
  nowSteps: 0,
  // The spec the cover was last drawn for, so the card is not redrawn when
  // only its words change.
  coverFor: null,
  passCount: 0,
  frameHandle: null,
  lite: false,
  // The Mono switch (queue item 66): the listener's, kept in the prefs and
  // never in a share code or a backup. Off plays the stereo render.
  mono: false,
  media: null,
  // The Rust core for this context, once loaded (js/core.js), and when
  // (ms since the page began).
  core: null,
  coreAt: null,
  coreError: null,        // why it will not come, if it will not
  // The last time the page was hidden, for Diagnostics (queue item 39).
  away: null,
  history: [],            // array of specs (max 5)
  historyIndex: -1,       // current position in history
};
const HISTORY_LIMIT = 5;

let lights = null;
let cells = null;

// --------------------------------------------------------------- audio

// The context, and the core loading into it. Made as the page loads, not on
// the first tap: a context starts suspended until a gesture resumes it, and
// the core needs only the context to be added to, so by the time Play is
// pressed it has almost always arrived and the first note plays on it.
// Until it does the synth plays its voices in JS, and counts them; a later
// synth (the quality toggle) takes it from the start.
function prepareAudio() {
  if (state.ctx) return;
  const Ctx = window.AudioContext || window.webkitAudioContext;
  state.ctx = newContext(Ctx, { latencyHint: 'playback' });
  state.ctx.addEventListener('statechange', () => awayEvent(`audio ${state.ctx.state}`));
  if (ENGINE !== 'rust') return;
  const ctx = state.ctx;
  const why = coreUnsupported();
  if (why) {
    state.coreError = why;
    return;
  }
  loadCore(ctx).then((core) => {
    if (state.ctx !== ctx) return;
    state.core = core;
    state.coreAt = Math.round(performance.now());
    if (state.synth) state.synth.attachCore(core);
  }).catch((err) => {
    if (state.ctx !== ctx) return;
    // Everything plays in JS, and Diagnostics says why. A later synth (the
    // quality toggle) starts with the same answer.
    state.coreError = String((err && err.message) || err);
    if (state.synth) state.synth.coreFailed(err);
  });
}

function ensureAudio() {
  prepareAudio();
  if (state.ctx.state === 'suspended') state.ctx.resume();
  if (!state.synth) buildAudio();
}

function buildAudio() {
  const wasPlaying = state.engine ? state.engine.playing : false;
  if (state.engine) state.engine.stop();
  // Tear the previous graph down before replacing it, or each rebuild
  // leaves a whole synth wired to the speakers and another keepalive
  // element in the DOM.
  if (state.media) state.media.dispose();
  if (state.synth) state.synth.dispose();
  // A core that has failed (loading, or while playing) stays failed for the
  // session: the new synth is not handed it, and says why.
  state.synth = new Synth(state.ctx, state.lite ? 'lite' : 'full', {
    engine: ENGINE,
    layout: state.mono ? 1 : 2,
    core: state.coreError ? null : state.core,
    coreError: state.coreError,
    onCoreError: (message) => { state.coreError = message; },
  });
  // Route the mix through a media element so the phone gives us lock-screen
  // controls and stops treating us as an idle tab.
  state.media = new MediaBridge(state.ctx);
  state.media.onchange = awayEvent;
  state.media.attach(state.synth.output);
  state.media.setHandlers({
    onPlay: () => { if (!state.engine.playing) togglePlay(); },
    onPause: () => { if (state.engine.playing) togglePlay(); },
    onNext: () => goForward(),
    onPrev: () => goBack(),
  });
  state.engine = new Engine(state.ctx, state.synth);
  state.engine.onStep = onStep;
  state.engine.onLoop = onLoop;
  state.engine.onTrackEnd = onTrackEnd;
  // The worst render-ahead this device has seen (queue item 47), planned
  // from while hidden.
  state.engine.deviceAhead = Number(store.getPrefs().renderAheadWorst) || 0;
  state.engine.setFrozen(pageFrozen);
  state.engine.visualOffset = (parseInt(ui.el('playheadSync').value, 10) || 0) / 1000;
  const wander = parseFloat(ui.el('driftAmount').value);
  state.engine.driftOn = wander > 0;
  state.engine.driftAmount = wander > 0 ? wander : 1;
  state.synth.setVolume(parseFloat(ui.el('volume').value));
  if (state.spec) {
    // The same loop, resumed under the new engine (queue item 69: its
    // late floor doesn't wait again).
    state.pattern = state.engine.load(state.spec, { resume: true });
    if (wasPlaying) {
      // The new bridge has to take over the media session too, otherwise
      // the lock-screen controls stay attached to the graph we just binned.
      state.media.start();
      state.engine.start();
      syncMediaMetadata();
      state.media.setPlaybackState(true);
      if (!state.frameHandle) state.frameHandle = requestAnimationFrame(frameLoop);
    }
  }
}

// -------------------------------------------------------------- history

function pushHistory(spec) {
  // If we're not at the end, discard forward history
  if (state.historyIndex < state.history.length - 1) {
    state.history = state.history.slice(0, state.historyIndex + 1);
  }
  state.history.push(JSON.parse(JSON.stringify(spec)));
  if (state.history.length > HISTORY_LIMIT) state.history.shift();
  state.historyIndex = state.history.length - 1;
  updateHistoryButtons();
}

function goBack() {
  if (state.playlist && state.playlist.ids.length) {
    advancePlaylist(-1);
    return;
  }
  if (state.historyIndex > 0) {
    state.historyIndex--;
    const spec = cloneSpec(state.history[state.historyIndex]);
    loadSpec(spec, { keepPosition: false, pushToHistory: false });
    if (!state.engine.playing) togglePlay();
    ui.toast(spec.name);
  }
}

function goForward() {
  // While an album is playing the skip buttons belong to the album, which is
  // what anyone would expect them to do.
  if (state.playlist && state.playlist.ids.length) {
    advancePlaylist(1);
    return;
  }
  // At the end of the history, skipping forward makes something new. A skip
  // button that does nothing is worse than no skip button, and on a headset
  // there is no other way to ask for a fresh loop.
  if (state.historyIndex >= state.history.length - 1) {
    newLoop();
    return;
  }
  state.historyIndex++;
  const spec = cloneSpec(state.history[state.historyIndex]);
  loadSpec(spec, { keepPosition: false, pushToHistory: false });
  if (!state.engine.playing) togglePlay();
  ui.toast(spec.name);
}

function resetHistory(spec) {
  state.history = [JSON.parse(JSON.stringify(spec))];
  state.historyIndex = 0;
  updateHistoryButtons();
}

function updateHistoryButtons() {
  const prev = ui.el('prevBtn');
  if (prev) prev.disabled = state.historyIndex <= 0;
  // Next is never disabled: past the end of the history it makes a new loop.
}

// -------------------------------------------------------------- loading

function loadSpec(spec, { keepPosition = false, id = null, pushToHistory = false } = {}) {
  ensureAudio();
  // A bar of 6/8 is twelve cells and 5/4 is twenty, so the grid has to be
  // rebuilt whenever the metre changes. Without this the cursor indexes a
  // sixteen-cell grid against twelve steps of music and drifts away from it.
  const spb = spec.stepsPerBar || 16;
  if (spb !== state.gridSteps) {
    state.gridSteps = spb;
    lights = ui.buildPlayhead(spb);
    cells = ui.buildLayers({ onReroll: reroll, onMute: toggleMute }, spb);
    ui.resetCursor();
  }
  state.spec = spec;
  state.currentId = id;
  state.pattern = state.engine.load(spec, { keepPosition });
  state.bar = -1;
  state.passCount = 0;
  if (!keepPosition) state.nowSteps = 0;
  ui.renderGrids(cells, state.pattern, 0, spec.mutes);
  syncToneInputs(spec);
  refreshCard();
  savedChanged();
  if (pushToHistory) pushHistory(spec);
  updateHistoryButtons();
  syncMediaMetadata();
}

// ------------------------------------------------------- while hidden

// A field report for the screen off (queue item 39), because nobody here
// has the phones it goes wrong on. Begun each time the page is hidden and
// kept as it stood when it is shown again, so a tester can lock the phone
// for a couple of minutes, unlock, and copy it from Diagnostics: how long
// it was hidden, the longest the clock went without a tick, the notes that
// reached the core late, the scheduler's late ticks, the times the audio
// device had to fill in (where the browser counts them: a starved audio
// thread, not a starved page), and every change in the audio context's
// state and in what the phone was told is playing.
function coreLate() {
  return state.synth && state.synth.core ? state.synth.core.late : null;
}

// The core's late and dropped notes by kind (queue item 41), for the views'
// report; null without the core.
function coreCounts() {
  const c = state.synth && state.synth.core;
  if (!c) return null;
  return { late: c.late, stall: c.lateStall, buffer: c.lateBuffer, missed: c.missed, dropped: c.dropped, by: c.droppedBy.slice() };
}

function fillIns() {
  try {
    const s = state.ctx && state.ctx.playoutStats;
    return s && typeof s.fallbackFramesEvents === 'number' ? s.fallbackFramesEvents : null;
  } catch {
    return null;
  }
}

function awayBegin() {
  if (state.away && state.away.to === null) return;
  state.away = {
    from: performance.now(), to: null, late: coreLate(), lateTo: null, fills: fillIns(), fillsTo: null, events: [],
    audio: state.ctx ? state.ctx.state : '-',
    session: 'mediaSession' in navigator ? navigator.mediaSession.playbackState : '-',
  };
}

function awayEnd() {
  const a = state.away;
  if (!a || a.to !== null) return;
  a.to = performance.now();
  a.lateTo = coreLate();
  a.fillsTo = fillIns();
  // Every hidden spell, not only the last (queue item 47).
  awayAll.spells++;
  awayAll.ms += a.to - a.from;
  if (a.late != null && a.lateTo != null) awayAll.late += Math.max(0, a.lateTo - a.late);
}

const awayAll = { spells: 0, ms: 0, late: 0 };

function awayEvent(what) {
  const a = state.away;
  if (!a || a.to !== null || a.events.length >= 20) return;
  a.events.push(`${what} at ${((performance.now() - a.from) / 1000).toFixed(1)} s`);
}

function awayReport() {
  const a = state.away;
  if (!a) return ['while hidden: not hidden yet'];
  const to = a.to ?? performance.now();
  const s = state.engine && state.engine.spell;
  const late = a.late != null && (a.lateTo ?? coreLate()) != null ? Math.max(0, (a.lateTo ?? coreLate()) - a.late) : '-';
  const fills = a.fills != null && (a.fillsTo ?? fillIns()) != null ? Math.max(0, (a.fillsTo ?? fillIns()) - a.fills) : '-';
  const events = a.events.filter((e) => /^audio /.test(e));
  const media = a.events.filter((e) => !/^audio /.test(e));
  return [
    `while hidden (every time): ${awayAll.spells} times, ${(awayAll.ms / 1000).toFixed(1)} s, late notes ${awayAll.late}`,
    'while hidden (the last time):',
    `hiddenFor: ${((to - a.from) / 1000).toFixed(1)} s`,
    `hiddenWorstGapMs: ${s ? s.worstGapMs : '-'}`,
    `hiddenLateNotes: ${late}`,
    `hiddenLateTicks: ${s ? s.lateTicks : '-'}`,
    `hiddenWorstLateMs: ${s ? s.worstLateMs : '-'}`,
    `hiddenFillIns: ${fills}`,
    `hiddenAudio: ${a.audio}${events.length ? `; ${events.join('; ')}` : ' throughout'}`,
    `hiddenMedia: ${a.session}${media.length ? `; ${media.join('; ')}` : ' throughout'}`,
  ];
}

// ------------------------------------------------- jumps and late notes

// When the page was hidden, shown, frozen and put away (queue item 47), on
// its own clock, so the log of the render-ahead's jumps and late notes can
// say where each fell: hidden or visible, and whether within a second of a
// change. The tester's late notes fell in neither the views' report nor the
// last hidden spell's.
const pageEvents = [[0, 'opened']];
let pageFrozen = false;

function pageEvent(what) {
  pageEvents.push([performance.now(), what]);
  if (pageEvents.length > 400) pageEvents.splice(1, 1);
}

// The worst render-ahead seen, kept for this device: the next session
// plans for it from the moment the page is hidden.
function rememberAhead(reach = 0) {
  const worst = Math.max(reach, state.synth ? state.synth.worstAhead || 0 : 0);
  const kept = Number(store.getPrefs().renderAheadWorst) || 0;
  if (worst > kept + 0.01) store.setPrefs({ renderAheadWorst: Math.round(worst * 1000) / 1000 });
}
coreLog.onjump = rememberAhead;

const HIDE_WORDS = { hidden: 'hiding', shown: 'showing', frozen: 'freezing', resumed: 'resuming', pagehide: 'pagehide', pageshow: 'pageshow' };

function whereAt(t) {
  let hidden = false;
  let near = null;
  for (const [at, what] of pageEvents) {
    if (what === 'opened') continue;
    // Frozen or put away is hidden too; resumed, it is as it was.
    if (at <= t && what === 'shown') hidden = false;
    else if (at <= t && (what === 'hidden' || what === 'frozen' || what === 'pagehide')) hidden = true;
    if (Math.abs(at - t) <= 1000 && (!near || Math.abs(at - t) < Math.abs(near[0] - t))) near = [at, what];
  }
  const by = near ? `, ${(Math.abs(t - near[0]) / 1000).toFixed(1)} s ${t >= near[0] ? 'after' : 'before'} ${HIDE_WORDS[near[1]]}` : '';
  return `${hidden ? 'hidden' : 'visible'}${by}`;
}

function aheadLog() {
  const list = coreLog.entries;
  const lines = [`late notes and render-ahead jumps (since the page began; ${coreLog.total}${coreLog.total > list.length ? `, the last ${list.length}` : ''}):`];
  if (!list.length) lines.push('  none');
  for (const e of list) {
    const when = `${(e.at / 1000).toFixed(1)} s, ${whereAt(e.at)}`;
    if (e.kind === 'jump') lines.push(`  ${when}: render-ahead ${e.reachMs} ms (was ${e.wasMs})`);
    else {
      lines.push(`  ${when}: late note, ${e.lateMs} ms behind the core, posted ${e.spareMs} ms ahead`
        + `${e.inside ? ' (inside the buffer)' : ' (stall)'}${e.missed ? ', left out' : ''}; decided by ${e.why || '-'}`);
    }
  }
  const visits = pageEvents.filter(([, w]) => w !== 'opened').slice(-12)
    .map(([at, w]) => `${w} ${(at / 1000).toFixed(1)}`);
  lines.push(`page events (s): ${visits.length ? visits.join(', ') : 'none'}`);
  return lines;
}

// ------------------------------------------------------- per view

// A field report for the screen on (queue item 40): since Diagnostics was
// last copied, for each view, how long it was on screen and what happened
// meanwhile -- the main thread's long tasks (the page doing something for
// more than 50 ms at a go), the longest the clock went without a tick, the
// notes that reached the core late, the scheduler's late ticks (music
// skipped) and the audio device's fill-ins (a starved audio thread). Hidden
// time is the "while hidden" report's, not any view's.
const views = { since: performance.now(), stats: {}, open: null, longTasks: false };

function viewStats(view) {
  if (!views.stats[view]) {
    views.stats[view] = {
      ms: 0, longTasks: 0, longestTask: 0, worstGapMs: 0, lateNotes: 0, lateTicks: 0, fillIns: null,
      // Queue item 41: the late notes by why, and the dropped ones by cause.
      lateStall: 0, lateBuffer: 0, missed: 0, dropped: 0, droppedBy: DROP_CAUSES.map(() => 0),
    };
  }
  return views.stats[view];
}

// What happened since the last call goes to the view that was on screen.
// `hiding`: the page has just been hidden, and was on screen until now.
function viewFlush(hiding = false) {
  const now = performance.now();
  const late = coreLate();
  const counts = coreCounts();
  const fills = fillIns();
  const ticks = state.engine ? state.engine.lateTicksEver : 0;
  const gap = state.engine ? state.engine.takeGap() : 0;
  const o = views.open;
  if (o && (hiding || !document.hidden)) {
    const v = viewStats(o.view);
    v.ms += now - o.at;
    v.worstGapMs = Math.max(v.worstGapMs, gap);
    if (late != null && o.late != null) v.lateNotes += Math.max(0, late - o.late);
    v.lateTicks += Math.max(0, ticks - o.ticks);
    if (fills != null && o.fills != null) v.fillIns = (v.fillIns || 0) + Math.max(0, fills - o.fills);
    // A synth built again meanwhile starts its counts from zero: only what
    // went up is counted.
    const a = o.counts;
    if (counts && a) {
      v.lateStall += Math.max(0, counts.stall - a.stall);
      v.lateBuffer += Math.max(0, counts.buffer - a.buffer);
      v.missed += Math.max(0, counts.missed - a.missed);
      v.dropped += Math.max(0, counts.dropped - a.dropped);
      counts.by.forEach((n, i) => { v.droppedBy[i] += Math.max(0, n - (a.by[i] || 0)); });
    }
  }
  views.open = document.hidden ? null : { view: state.view, at: now, late, fills, ticks, counts };
}

function watchLongTasks() {
  try {
    if (!PerformanceObserver.supportedEntryTypes.includes('longtask')) return;
    new PerformanceObserver((list) => {
      if (document.hidden || !views.open) return;
      const v = viewStats(views.open.view);
      for (const t of list.getEntries()) {
        v.longTasks++;
        v.longestTask = Math.max(v.longestTask, Math.round(t.duration));
      }
    }).observe({ type: 'longtask' });
    views.longTasks = true;
  } catch { /* not in this browser */ }
}

function viewsReport() {
  viewFlush();
  const mins = ((performance.now() - views.since) / 60000).toFixed(1);
  const lines = [`views (since the last copy, ${mins} min; on screen only):`];
  const names = Object.keys(views.stats);
  if (!names.length) lines.push('  nothing yet');
  for (const name of names) {
    const v = views.stats[name];
    const long = views.longTasks ? `long tasks ${v.longTasks}${v.longTasks ? ` (longest ${v.longestTask} ms)` : ''}` : 'long tasks -';
    const why = v.lateNotes ? ` (stall ${v.lateStall}, inside the buffer ${v.lateBuffer}; left out ${v.missed})` : '';
    const dropped = v.dropped
      ? ` (${DROP_CAUSES.map((c, i) => [c, v.droppedBy[i]]).filter(([, n]) => n).map(([c, n]) => `${c} ${n}`).join(', ')})` : '';
    lines.push(`  ${name}: ${clock(v.ms / 1000)} on screen, ${long}, longest tick gap ${v.worstGapMs} ms, `
      + `late notes ${v.lateNotes}${why}, dropped ${v.dropped}${dropped}, late ticks ${v.lateTicks}, fill-ins ${v.fillIns ?? '-'}`);
  }
  return lines;
}

function viewsReset() {
  views.stats = {};
  views.since = performance.now();
  viewFlush();
}

// The lock screen and the headset notification read from here. Called on
// every load and again on Play, because the very first loop is put into the
// engine while the audio graph is being built, before any load happens.
function syncMediaMetadata() {
  if (!state.media || !state.spec) return;
  state.media.setMetadata(state.spec.name, `${state.spec.bpm} bpm · Driftloom`);
}

// Passes are the honest unit -- a track ends on a whole loop -- but nobody
// thinks in passes, so show the time it comes to at this tempo as well.
// The time a length actually comes to (whole passes at this tempo), kept
// short: it sits beside the slider and must not change width much.
function describeLength(passes, spec) {
  if (!passes) return 'off';
  if (!spec) return `${passes}×`;
  return ui.formatTime(passes * ui.passSeconds(spec));
}

// The playing loop's cover on the card. Drawn off the main thread
// (js/covers.js, queue item 41) and kept there, so neither a new loop nor
// going back to one -- leaving an album, changing the Library's tab, a code
// for the loop that is playing -- holds the page; the card keeps the last
// cover until the new one is in.
function drawCoverFor(spec) {
  const canvas = ui.el('cover');
  if (!canvas || !spec) return;
  coverImage(spec, 320).then((image) => {
    // Gone by then: another loop, or an album on the card instead.
    if (state.spec !== spec || library.shownAlbum()) {
      if (state.coverFor === spec && state.spec !== spec) state.coverFor = null;
      return;
    }
    canvas.width = 320;
    canvas.height = 320;
    paint(canvas, image);
  }).catch((err) => {
    console.warn('Could not draw cover', err);
    if (state.coverFor === spec) state.coverFor = null;
  });
}

// ---------------------------------------------------------------- card

// The card shows the album open in the library, or else the loop that is
// playing. Everything that changes either one ends up here.
function refreshCard() {
  const albumId = library.shownAlbum();
  const album = albumId && store.loadAlbums().find((a) => a.id === albumId);
  const playhead = ui.el('playhead');
  if (album) {
    const byId = new Map(store.loadAll().map((e) => [e.id, e]));
    const specs = album.ids.map((id) => byId.get(id)).filter(Boolean).map((e) => e.spec);
    const canvas = ui.el('cover');
    library.albumArt(album, specs, 320).then((image) => {
      if (library.shownAlbum() !== albumId) return;
      canvas.width = 320;
      canvas.height = 320;
      paint(canvas, image);
    }).catch((err) => console.warn('Could not draw album art', err));
    state.coverFor = null;
    const pl = state.playlist;
    const playing = pl && pl.albumId === album.id
      ? `playing ${pl.index + 1}/${pl.ids.length}` : '';
    ui.renderAlbumCard(album.title, specs, { playing });
    // Hidden rather than removed, so the card keeps its height and the
    // viewport below does not jump.
    playhead.style.visibility = 'hidden';
    return;
  }
  playhead.style.visibility = '';
  if (!state.spec) return;
  if (state.coverFor !== state.spec) {
    drawCoverFor(state.spec);
    state.coverFor = state.spec;
  }
  renderLoopWords();
}

// Time left and album position live in Now Playing; the card keeps only
// which album is playing, as a reminder of where Next goes.
function renderLoopWords() {
  const pl = state.playlist;
  ui.renderReadout(state.spec, state.pattern, {
    album: pl && pl.ids.length ? `${pl.title} ${pl.index + 1}/${pl.ids.length}` : '',
  });
}

// Anything saved, deleted or renamed: the lists, and whether the loop on
// screen counts as saved.
function savedChanged() {
  library.refresh();
  // The albums' art, ready for the Library before it is opened.
  library.warmArt();
  updateSaveButton();
  if (state.view === 'now') renderNow();
}

// ------------------------------------------------------------------ save

// A saved loop lives in at least one album; saving is putting it in one.
function isSaved() {
  return !!state.currentId && store.loadAll().some((e) => e.id === state.currentId);
}

function isChanged() {
  if (!isSaved() || !state.spec) return false;
  const stored = store.loadAll().find((e) => e.id === state.currentId);
  return !!stored && !store.sameSpec(stored.spec, state.spec);
}

// Save the playing loop to an album, saving it first if it is not saved
// anywhere yet.
function saveIn(albumId) {
  if (!state.spec) return;
  if (!isSaved()) {
    const entry = store.save(state.spec);
    if (!entry) { ui.toast('Could not save it - this browser is refusing to store data'); return; }
    state.currentId = entry.id;
  }
  if (!store.addToAlbum(albumId, state.currentId)) { ui.toast('Could not change that album'); return; }
  const album = store.loadAlbums().find((a) => a.id === albumId);
  ui.toast(`Saved to ${album ? album.title : 'the album'}`);
  savedChanged();
  refreshCard();
}

function unsaveFrom(albumId) {
  if (!isSaved()) return;
  const name = state.spec.name;
  const res = store.removeFromAlbum(albumId, state.currentId);
  if (!res.ok) { ui.toast('Could not change that album'); return; }
  if (res.deleted) {
    state.currentId = null;
    ui.toast(`Deleted ${name}`);
  } else {
    ui.toast('Taken out of that album');
  }
  savedChanged();
  refreshCard();
}

function updateSaved() {
  if (!isSaved()) return;
  if (!store.replaceSpec(state.currentId, state.spec)) {
    ui.toast('Could not save - this browser is refusing to store data');
    return;
  }
  ui.toast(`Updated ${state.spec.name}`);
  savedChanged();
}

// The changed loop becomes a loop of its own, in Favorite Loops; the one it
// came from stays as it was.
function saveAsNew() {
  if (!state.spec) return;
  const entry = store.save(state.spec);
  if (!entry) { ui.toast('Could not save - this browser is refusing to store data'); return; }
  state.currentId = entry.id;
  store.addToAlbum(store.FAVORITES, entry.id);
  ui.toast(`Saved ${state.spec.name} as a new loop in Favorite Loops`);
  savedChanged();
}

// S on a keyboard: save to Favorite Loops, or save the changes to a saved loop.
function saveQuick() {
  if (!state.spec) return;
  if (!isSaved()) saveIn(store.FAVORITES);
  else if (isChanged()) updateSaved();
  else ui.toast(`${state.spec.name} is saved`);
}

// The bookmark is filled once the loop lives in an album, and becomes an
// outline with a dot when the saved loop has been changed since.
function updateSaveButton() {
  const kept = isSaved();
  // The key's icon stays put (only Play and the Sound / Layers switch
  // change theirs); its name says whether this loop is saved.
  const b = ui.el('saveBtn');
  ui.setIcon(b, 'saveShare');
  b.setAttribute('aria-label', kept ? (isChanged() ? 'Saved, changed since: save or share' : 'Saved: save or share') : 'Save or share');
}

// ------------------------------------------------------------ viewport

// Now Playing, Sound and Layers are where you stay; Library and Save are
// places you visit, whose buttons light while you are there. Pressing a lit
// button again (or the switch) goes back to where you were.
const BASE_VIEWS = ['now', 'sounds', 'layers'];

function setView(view) {
  viewFlush();
  state.view = view;
  if (views.open) views.open.view = view;
  const visiting = !BASE_VIEWS.includes(view);
  if (!visiting) state.lastView = view;
  if (view === 'sounds' || view === 'layers') state.lastEdit = view;
  for (const v of document.querySelectorAll('.view')) v.hidden = v.dataset.view !== view;
  // The switch shows where it goes, not where you are: from Sound it offers
  // Layers, from Layers it offers Sound, and from anywhere else it offers
  // whichever of the two you used last.
  const target = view === 'sounds' ? 'layers' : view === 'layers' ? 'sounds' : state.lastEdit;
  const vb = ui.el('viewBtn');
  ui.setIcon(vb, target);
  vb.setAttribute('aria-label', target === 'sounds' ? 'Show sound controls' : 'Show layers');
  vb.title = target === 'sounds' ? 'Sound' : 'Layers';
  ui.setColumnsLive(view === 'layers');
  const shownPattern = (state.engine && state.engine.live) || state.pattern;
  if (view === 'layers' && state.spec && shownPattern) ui.renderGrids(cells, shownPattern, state.bar > 0 ? state.bar : 0, state.spec.mutes || {});
  if (view === 'library') library.render();
  if (view === 'save') library.renderSave();
  if (view === 'now') renderNow();
  if (view === 'vibes') renderVibes();
  if (view === 'vibe') picks.renderPanel();
  if (view === 'length') picks.renderLength();
  refreshCard();
  store.setPrefs({ view: state.lastView, edit: state.lastEdit });
}

// ----------------------------------------------------------------- vibes

// Queue item 38b. The Vibe panel's mood line opens this view in its
// place; Close goes back. No limit on how many: past five, each new loop
// picks five of them (generator.js, `newSpec`).
function renderVibes() {
  const list = ui.el('vibesList');
  if (!list.children.length) {
    for (const id of VIBE_IDS) {
      const b = document.createElement('button');
      b.type = 'button';
      b.className = 'vkey vb-key';
      b.dataset.vibe = id;
      const led = document.createElement('span');
      led.className = 'led';
      led.setAttribute('aria-hidden', 'true');
      const word = document.createElement('span');
      word.className = 'vb-word';
      word.textContent = id;
      b.append(led, word);
      b.addEventListener('click', () => toggleVibe(id));
      list.appendChild(b);
    }
  }
  const on = new Set(state.vibes);
  for (const b of list.children) b.setAttribute('aria-pressed', on.has(b.dataset.vibe) ? 'true' : 'false');
  const n = state.vibes.length;
  ui.el('vibesCount').textContent = n ? `${n} for new loops` : 'none: new loops as usual';
  ui.el('vibesClear').disabled = !n;
  updateVibesKey();
}

function setVibes(list) {
  state.vibes = vibeSelection(list);
  store.setPrefs({ vibes: state.vibes });
  if (state.view === 'vibes') renderVibes();
  else updateVibesKey();
  // The picks on Now Playing and the panel show the moods; nothing else
  // changes until the next new loop.
  if (state.view === 'now') renderNow();
  if (state.view === 'vibe') picks.renderPanel();
}

// ----------------------------------------------------------- next picks

function setPicks(next) {
  state.picks = picks.cleanPicks({ ...state.picks, ...next });
  store.setPrefs({ picks: state.picks });
  if (state.view === 'now') renderNow();
  if (state.view === 'vibe') picks.renderPanel();
}

// An unlit any key puts its line back to chance. Line two holds tempo and
// length, so it clears both.
function clearPickLine(line) {
  if (line === 'mood') setVibes([]);
  else if (line === 'pace') setPicks({ tempo: null, length: null });
}

function toggleVibe(id) {
  setVibes(state.vibes.includes(id) ? state.vibes.filter((v) => v !== id) : [...state.vibes, id]);
}

// The vibe key (Mikey, 2026-10-14): one word, "vibe", that opens the Vibe
// panel; the panel and the pills say what is picked.
function updateVibesKey() {
  const b = ui.el('vibesBtn');
  b.textContent = 'vibe';
  b.setAttribute('aria-label', 'Vibe');
}

// ----------------------------------------------------------- now playing

// Big and plain: how long is left (or that it is endless), how far through,
// where in an album, and what Next will do. The one control sits at the
// bottom of the view, a thumb's reach from the buttons.
function renderNow() {
  if (state.view !== 'now' || !state.spec) return;
  updateNowClock();

  const pl = state.playlist;
  const albumRow = ui.el('nowAlbum');
  albumRow.hidden = !(pl && pl.ids.length);
  if (pl && pl.ids.length) {
    ui.el('nowAlbumName').textContent = pl.title;
    const n = pl.ids.length;
    const dots = ui.el('nowDots');
    dots.hidden = n > 12;
    dots.innerHTML = n <= 12
      ? Array.from({ length: n }, (_, i) => (i === pl.index ? '<i class="on"></i>' : '<i></i>')).join('')
      : '';
    ui.el('nowOf').textContent = `${pl.index + 1}\u00a0of\u00a0${n}`;
  }

  // In an album the album is what comes next, so Now Playing says which
  // track; outside one it shows what new loops lean toward instead.
  const inAlbum = !!(pl && pl.ids.length);
  const next = ui.el('nowNext');
  next.hidden = !inAlbum;
  ui.el('nowPicks').hidden = inAlbum;
  if (inAlbum) {
    next.textContent = 'Next: ';
    const b = document.createElement('b');
    b.textContent = nextName();
    next.appendChild(b);
  } else {
    picks.renderNowPicks();
  }

  const action = ui.el('nowAction');
  if (pl && pl.ids.length) {
    action.hidden = false;
    action.textContent = 'leave album';
    action.onclick = leaveAlbum;
  } else if (state.lastAlbum) {
    const album = store.loadAlbums().find((a) => a.id === state.lastAlbum.albumId);
    action.hidden = !album;
    if (album) {
      action.textContent = `back to ${album.title}`;
      action.onclick = () => playAlbum(album.id, state.lastAlbum.index);
    }
  } else {
    action.hidden = true;
  }
}

function nextName() {
  const pl = state.playlist;
  if (pl && pl.ids.length) {
    const id = pl.ids[(pl.index + 1) % pl.ids.length];
    const entry = store.loadAll().find((e) => e.id === id);
    return entry ? entry.spec.name : 'the next track';
  }
  if (state.historyIndex < state.history.length - 1) {
    return state.history[state.historyIndex + 1].name;
  }
  return 'something new';
}

// Where the listener is in this track, in steps, read off the audio clock.
// The scheduler's own counters (loopCount, step) run ahead of the sound by
// its lookahead, up to three seconds while the screen is off, so counting
// onLoop calls made the clock jump early at every pass. This takes the
// scheduler's position and walks it back by how far its next step is ahead
// of what is being heard, using the engine's measured output latency.
function audibleSteps() {
  const e = state.engine;
  if (!e || !e.playing || !e.live || !e.spec) return null;
  const heard = state.ctx.currentTime - (e.latency || 0);
  const ahead = (e.nextStepTime - heard) / e.stepDur;
  return Math.max(0, e.loopCount * e.live.totalSteps + e.step - ahead);
}

// The clock and the bar. Called every animation frame while Now Playing is
// up and playing, so the bar glides; the text is only rewritten when the
// second it shows changes. A set length counts down (shown with a minus, as
// players do); an endless loop counts up. Paused, it holds where it was.
function updateNowClock() {
  const e = state.engine;
  const steps = audibleSteps();
  if (steps != null) state.nowSteps = steps;
  const spec = state.spec;
  if (!spec || !e || !e.live) {
    setNowText('0:00');
    ui.el('nowFill').style.transform = 'scaleX(0)';
    return;
  }
  const total = e.live.totalSteps;
  const pos = state.nowSteps || 0;
  let text;
  let frac;
  if (spec.playFor) {
    const end = spec.playFor * total;
    text = `\u2212${clock(Math.max(0, end - pos) * e.stepDur)}`;
    frac = Math.min(1, pos / end);
  } else {
    text = clock(pos * e.stepDur);
    frac = (pos % total) / total;
  }
  setNowText(text);
  ui.el('nowFill').style.transform = `scaleX(${frac.toFixed(4)})`;
}

function setNowText(text) {
  const big = ui.el('nowBig');
  if (big.textContent !== text) big.textContent = text;
}

function clock(secs) {
  const s = Math.max(0, Math.round(secs));
  const h = Math.floor(s / 3600);
  const m = Math.floor((s % 3600) / 60);
  const r = s % 60;
  return h ? `${h}:${String(m).padStart(2, '0')}:${String(r).padStart(2, '0')}` : `${m}:${String(r).padStart(2, '0')}`;
}


// --------------------------------------------------------------- rename

// Tap the name on the card to rename whatever the card shows: the loop, or
// the album open in the library. It edits in place rather than in a pop-up.
function startRename() {
  const albumId = library.shownAlbum();
  if (!albumId && !state.spec) return;
  const h = ui.el('loopName');
  const input = ui.el('nameEdit');
  input.value = h.textContent;
  input.style.fontSize = h.style.fontSize;
  h.hidden = true;
  input.hidden = false;
  input.focus();
  input.select();

  let done = false;
  const finish = (commit) => {
    if (done) return;
    done = true;
    input.removeEventListener('keydown', onKey);
    input.removeEventListener('blur', onBlur);
    input.hidden = true;
    h.hidden = false;
    const name = input.value.trim().slice(0, store.NAME_MAX);
    if (!commit || !name || name === h.textContent) return;
    if (albumId) {
      if (!store.renameAlbum(albumId, name)) { ui.toast('Could not rename'); return; }
      library.refresh();
      refreshCard();
      return;
    }
    state.spec.name = name;
    if (state.historyIndex >= 0 && state.history[state.historyIndex]) {
      state.history[state.historyIndex].name = name;
    }
    if (state.currentId) store.rename(state.currentId, name);
    refreshCard();
    savedChanged();
    syncMediaMetadata();
  };
  const onKey = (e) => {
    if (e.key === 'Enter') { e.preventDefault(); finish(true); }
    else if (e.key === 'Escape') { e.preventDefault(); finish(false); }
  };
  const onBlur = () => finish(true);
  input.addEventListener('keydown', onKey);
  input.addEventListener('blur', onBlur);
}

// -------------------------------------------------------------- albums

// Out of album mode without stopping the music: Next makes new loops again.
// Out of album mode without stopping the music: Next makes new loops again,
// and Now Playing offers the way back.
function leaveAlbum() {
  const pl = state.playlist;
  if (pl) state.lastAlbum = { albumId: pl.albumId, index: Math.max(0, pl.index) };
  state.playlist = null;
  refreshCard();
  library.refresh();
  renderNow();
  ui.toast('Left the album. Next makes new loops');
}

function playAlbum(albumId, index = 0) {
  const album = store.loadAlbums().find((a) => a.id === albumId);
  if (!album) return;
  const saved = new Set(store.loadAll().map((e) => e.id));
  const present = album.ids.filter((id) => saved.has(id));
  if (!present.length) { ui.toast('That album is empty'); return; }
  state.playlist = { albumId: album.id, title: album.title, ids: present, index: index - 1 };
  advancePlaylist(1);
}

async function shareAlbum(albumId) {
  const album = store.loadAlbums().find((a) => a.id === albumId);
  if (!album) return;
  const byId = new Map(store.loadAll().map((e) => [e.id, e]));
  const specs = album.ids.map((id) => byId.get(id)).filter(Boolean).map((e) => e.spec);
  if (!specs.length) { ui.toast('That album is empty'); return; }
  await offerCode(share.encodeAlbum(album.title, specs), `${album.title} · ${specs.length} loops`);
}

// Put a code where it can be taken. The clipboard is the quick path; the
// box below is the one that still works when the clipboard is refused.
async function offerCode(code, label) {
  const out = ui.el('shareOut');
  out.hidden = false;
  out.textContent = `${label}\n\n${code}`;
  try {
    await navigator.clipboard.writeText(code);
    ui.toast('Code copied');
  } catch {
    // The box lives under Save > Share; take them to it.
    setView('save');
    library.showShare();
    ui.toast('Copy it from the box below');
  }
}

function syncToneInputs(spec) {
  ui.el('bpm').value = spec.bpm;
  const len = spec.playFor || 0;
  const secs = len * ui.passSeconds(spec);
  const idx = len ? TRACK_TIMES.reduce((best, v, i) =>
    (i && Math.abs(v - secs) < Math.abs(TRACK_TIMES[best] - secs) ? i : best), 1) : 0;
  ui.el('trackLen').value = idx;
  ui.el('lenVal').textContent = describeLength(len, spec);
  ui.el('warmth').value = spec.tone.warmth;
  ui.el('space').value = spec.tone.space;
  ui.el('wobble').value = spec.tone.wobble;
  ui.el('bpmVal').textContent = spec.bpm;
}

function openSaved(entry) {
  // One unreadable save should not take the app down with it.
  try {
    state.playlist = null;
    resetHistory(cloneSpec(entry.spec));
    loadSpec(cloneSpec(entry.spec), { id: entry.id, pushToHistory: false });
  } catch (err) {
    console.warn('Could not open saved loop', err);
    ui.toast('That saved loop could not be opened');
    return;
  }
  if (!state.engine.playing) togglePlay();
  ui.toast(`Loaded ${entry.spec.name}`);
}

// ------------------------------------------------------------ callbacks

// Repaint on animation frames, reading position off the audio clock. This
// is the only place the cursor moves.
function frameLoop() {
  if (!state.engine || !state.engine.playing) {
    state.frameHandle = null;
    return;
  }
  const step = state.engine.visualStep();
  if (step != null) onStep(step);
  if (state.view === 'now') updateNowClock();
  state.frameHandle = requestAnimationFrame(frameLoop);
}

function onStep(step) {
  if (!state.engine.playing) return;
  const spb = state.gridSteps;
  const bar = Math.floor(step / spb);
  if (bar !== state.bar) {
    state.bar = bar;
    // The layers' grid only while it is on screen; it catches up when
    // Layers is opened (setView).
    if (state.view === 'layers') ui.renderGrids(cells, state.engine.live, bar, state.spec.mutes);
  }
  ui.moveCursor(lights, cells, step % spb);
}

function onLoop(count) {
  state.passCount = count;
  if (state.view === 'now') renderNow();
}

// A track that has run its length hands over: to the next loop in the album
// if one is playing, otherwise onward through the history.
function onTrackEnd() {
  // Out of the scheduler's call stack before touching the graph.
  setTimeout(() => {
    if (state.playlist && state.playlist.ids.length) advancePlaylist(1);
    else goForward();
  }, 0);
}

function advancePlaylist(step) {
  const pl = state.playlist;
  if (!pl || !pl.ids.length) return;
  pl.index = (pl.index + step + pl.ids.length) % pl.ids.length;
  const entry = store.loadAll().find((e) => e.id === pl.ids[pl.index]);
  if (!entry) {
    // A loop was deleted out from under the album; drop it and carry on.
    pl.ids.splice(pl.index, 1);
    if (!pl.ids.length) { state.playlist = null; return; }
    pl.index %= pl.ids.length;
    advancePlaylist(0);
    return;
  }
  loadSpec(cloneSpec(entry.spec), { id: entry.id, pushToHistory: false });
  if (!state.engine.playing) togglePlay();
  ui.toast(`${pl.title} · ${pl.index + 1}/${pl.ids.length}`);
}

// ------------------------------------------------------------- actions

// Every new loop is drawn here, so every new loop leans toward the vibes:
// Next at the end of the history (also a loop ending, and the N key), Play
// before any loop exists, and the first loop when the app opens. Not the
// loop playing, an album's tracks, a saved loop, history or a re-roll.
// The picks are its request (queue item 85); one the generator cannot meet
// (the panel says so) is left out, the moods kept.
function newSpec() {
  let spec;
  try {
    spec = drawSpec(undefined, picks.requestFor(state));
  } catch (e) {
    if (!(e instanceof RequestError)) throw e;
    spec = drawSpec(undefined, state.vibes);
  }
  return picks.applyPicks(spec, state.picks);
}

function togglePlay() {
  ensureAudio();
  if (!state.engine.spec) {
    const s = state.spec || newSpec();
    resetHistory(s);
    loadSpec(s, { pushToHistory: false });
  }
  if (state.engine.playing) {
    state.engine.stop();
    state.media.stop();
    state.media.setPlaybackState(false);
    setPlayButton(false);
    ui.moveCursor(lights, cells, -1);
    ui.resetCursor();
  } else {
    state.media.start();
    state.engine.start();
    if (!state.frameHandle) state.frameHandle = requestAnimationFrame(frameLoop);
    syncMediaMetadata();
    state.media.setPlaybackState(true);
    setPlayButton(true);
  }
}

function setPlayButton(playing) {
  renderNow();
  const b = ui.el('playBtn');
  ui.el('app').classList.toggle('playing', playing);
  b.setAttribute('aria-label', playing ? 'Pause' : 'Play');
  b.title = playing ? 'Pause' : 'Play';
  ui.setIcon(b, playing ? 'pause' : 'play');
}

function newLoop() {
  // Making something new is leaving the album.
  state.playlist = null;
  // Save the current loop to history before creating a new one (if it exists)
  if (state.spec) {
    // Avoid duplicate if the current spec is already the last in history
    const last = state.history[state.history.length - 1];
    if (!last || JSON.stringify(last) !== JSON.stringify(state.spec)) {
      pushHistory(state.spec);
    }
  }
  const s = newSpec();
  // Push the new spec to history (this truncates any forward history)
  pushHistory(s);
  // Load the new spec without pushing it again
  loadSpec(s, { pushToHistory: false });
  state.engine.reset();
  if (!state.engine.playing) togglePlay();
  ui.toast(`New loop: ${s.name}`);
  updateHistoryButtons();
}

function reroll(layer) {
  if (!state.spec) return newLoop();
  const next = rerollLayer(state.spec, layer);
  next.name = state.spec.name; // a re-roll is a revision, not a new piece
  // And because it is a revision it REPLACES the current history entry
  // instead of adding one. Otherwise Previous walks back through your own
  // rolls of the same loop rather than reaching the loop before it.
  // It keeps its place among the saved loops: Save then offers to update the
  // saved one or save this as a new loop, rather than silently forgetting it.
  loadSpec(next, { keepPosition: true, id: state.currentId, pushToHistory: false });
  if (state.historyIndex >= 0 && state.history[state.historyIndex]) {
    state.history[state.historyIndex] = cloneSpec(next);
  }
  ui.toast(`Re-rolled ${layer}`);
}

function toggleMute(layer) {
  if (!state.spec) return;
  // Saves predating the mutes field would otherwise throw here.
  if (!state.spec.mutes) {
    state.spec.mutes = { drums: false, bass: false, chords: false, melody: false, texture: false };
  }
  state.spec.mutes[layer] = !state.spec.mutes[layer];
  state.synth.setMute(layer, state.spec.mutes[layer]);
  // The channel goes quiet at once either way; back on, its notes are
  // decided again from now, not after what the core holds (queue item 40).
  state.engine.redecide();
  ui.renderGrids(cells, state.engine.live || state.pattern, Math.max(0, state.bar), state.spec.mutes);
}

function exportMidi(spec = state.spec) {
  if (!spec) return;
  const blob = patternToMidi(render(spec), { repeats: 4 });
  ui.download(blob, `${spec.name}-${spec.bpm}bpm.mid`);
  ui.toast('MIDI exported');
}

// Paste a song or album code: from the clipboard if the browser allows it,
// otherwise from a box.
async function pasteCode() {
  let text = '';
  try {
    text = await navigator.clipboard.readText();
  } catch {
    text = '';
  }
  if (!share.codeKind(text)) {
    text = prompt('Paste a Driftloom code') || '';
  }
  const kind = share.codeKind(text);
  if (!kind) {
    if (text.trim()) ui.toast('That does not look like a Driftloom code');
    return;
  }
  try {
    if (kind === 'song') {
      // A loop code lands in Imported Loops, unless this exact loop is saved
      // already, in which case it plays from where it lives.
      const spec = share.decodeSong(text);
      const same = store.findSame(spec);
      state.playlist = null;
      if (same) {
        loadSpec(cloneSpec(same.spec), { id: same.id });
        const home = store.albumsOf(same.id)[0];
        ui.toast(`Already saved${home ? ` in ${home.title}` : ''}`);
      } else {
        const ids = store.addSpecs([spec]);
        if (ids) store.addToAlbum(store.IMPORTED, ids[0]);
        loadSpec(spec, { id: ids ? ids[0] : null });
        ui.toast(ids ? `Added ${spec.name} to Imported Loops` : `Playing ${spec.name} (could not save it)`);
      }
      if (!state.engine.playing) togglePlay();
    } else {
      const { title, specs } = share.decodeAlbum(text);
      const ids = store.addSpecs(specs);
      if (!ids) {
        ui.toast('Could not save those loops');
        return;
      }
      const album = store.createAlbum(title);
      if (album) store.setAlbumIds(album.id, ids);
      savedChanged();
      ui.toast(`Added ${title} · ${specs.length} loops`);
    }
  } catch (err) {
    ui.toast(err.message || 'That code could not be read');
  }
}

// --------------------------------------------------------------- themes

// The nine themes of design v17. The theme is one of the three settings
// (the others are Lighter on processor and Mono); it is kept in prefs and
// applied before first paint by the script at the top of index.html.
const THEMES = [
  ['t-dawn', 'dawn', 'Dawn', '#f0dccc'], ['t-ember', 'ember', 'Ember', '#c86a80'],
  ['t-tidal', 'tidal', 'Tidal', '#78b8c4'], ['t-dark', 'dark', 'Dark', '#121218'],
  ['t-amoled', 'amoled', 'AMOLED', '#000000'], ['t-nightlight', 'nightlight', 'Night Light', '#1a0e04'],
  ['t-nocturne', 'nocturne', 'Nocturne', '#160830'], ['t-abyss', 'abyss', 'Abyss', '#0e1e24'],
  ['t-olive', 'olive', 'Olive', '#9c9642'],
];
const DEFAULT_THEME = 't-nocturne';

function currentTheme() {
  const t = store.getPrefs().theme;
  return THEMES.some((x) => x[0] === t) ? t : DEFAULT_THEME;
}

function applyTheme(cls, save = true) {
  const app = ui.el('app');
  for (const t of THEMES) app.classList.remove(t[0]);
  app.classList.add(cls);
  const th = THEMES.find((t) => t[0] === cls);
  const meta = document.querySelector('meta[name="theme-color"]');
  if (meta && th) meta.content = th[3];
  if (save) store.setPrefs({ theme: cls });
  ui.el('themeName').textContent = th ? th[2] : '';
  for (const sw of document.querySelectorAll('#themes .sw')) {
    const on = sw.dataset.theme === cls;
    sw.setAttribute('aria-checked', String(on));
    sw.tabIndex = on ? 0 : -1;
  }
  requestAnimationFrame(() => ui.fitName());
}

function initThemes() {
  const host = ui.el('themes');
  host.innerHTML = '';
  for (const [cls, key, label] of THEMES) {
    const b = document.createElement('button');
    b.type = 'button';
    b.className = 'sw';
    b.setAttribute('role', 'radio');
    b.dataset.theme = cls;
    b.setAttribute('aria-label', label);
    b.innerHTML = `<span class="sw-art sw-${key}" aria-hidden="true"><span class="sw-card"><i></i><i></i><i></i><b></b></span></span>`
      + `<span class="sw-check" aria-hidden="true">${ui.icon('check')}</span>`;
    b.addEventListener('click', () => applyTheme(cls));
    b.addEventListener('keydown', (e) => {
      const dir = { ArrowRight: 1, ArrowDown: 1, ArrowLeft: -1, ArrowUp: -1 }[e.key];
      if (!dir) return;
      e.preventDefault();
      const i = THEMES.findIndex((t) => t[0] === cls);
      const next = THEMES[(i + dir + THEMES.length) % THEMES.length][0];
      applyTheme(next);
      host.querySelector(`[data-theme="${next}"]`).focus();
    });
    host.appendChild(b);
  }
  applyTheme(currentTheme(), false);
}

function setLiteSwitch(on) {
  setSwitch('liteMode', on);
}

function setSwitch(id, on) {
  const b = ui.el(id);
  b.setAttribute('aria-checked', String(on));
  b.firstElementChild.textContent = on ? 'on' : 'off';
}

// ---------------------------------------------------------------- boot

function wire() {
  lights = ui.buildPlayhead(state.gridSteps);
  cells = ui.buildLayers({ onReroll: reroll, onMute: toggleMute }, state.gridSteps);

  ui.setIcon(ui.el('prevBtn'), 'prev');
  ui.setIcon(ui.el('nextBtn'), 'next');
  ui.setIcon(ui.el('libraryBtn'), 'library');
  ui.setIcon(ui.el('nowBtn'), 'now');
  ui.setIcon(ui.el('saveBtn'), 'saveShare');
  for (const n of document.querySelectorAll('[data-ico]')) n.innerHTML = ui.icon(n.dataset.ico);
  initThemes();
  setPlayButton(false);
  store.ensureAlbums();
  updateSaveButton();

  library.initLibrary({
    state,
    cardChanged: refreshCard,
    savedChanged,
    playAlbum,
    leaveAlbum,
    isSaved,
    isChanged,
    saveIn,
    unsaveFrom,
    updateSaved,
    saveAsNew,
    shareAlbum,
    openSaved,
    pasteCode,
    exportMidi: (spec) => exportMidi(cloneSpec(spec)),
  });

  // The vibe key opens the Vibe panel; the vibe words (from its mood line)
  // close back to it.
  ui.el('vibesBtn').addEventListener('click', () => setView('vibe'));
  ui.el('vibesClose').addEventListener('click', () => setView('vibe'));
  picks.wirePicks({
    state,
    setView,
    openVibes: () => setView('vibes'),
    openLength: () => { picks.openLength(); setView('length'); },
    setLength: (length) => setPicks({ length }),
    clearLine: clearPickLine,
  });
  ui.el('vibesClear').addEventListener('click', () => setVibes([]));

  ui.el('libraryBtn').addEventListener('click', () => {
    setView(state.view === 'library' ? state.lastView : 'library');
  });
  ui.el('saveBtn').addEventListener('click', () => {
    setView(state.view === 'save' ? state.lastView : 'save');
  });
  // Now Playing is home: its button goes there, and from there back to
  // whichever of Sound and Layers you were using.
  ui.el('nowBtn').addEventListener('click', () => {
    setView(state.view === 'now' ? state.lastEdit : 'now');
  });
  ui.el('viewBtn').addEventListener('click', () => {
    if (state.view === 'sounds') setView('layers');
    else if (state.view === 'layers') setView('sounds');
    else setView(state.lastEdit);
  });

  const nameEl = ui.el('loopName');
  nameEl.addEventListener('click', startRename);
  nameEl.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') { e.preventDefault(); startRename(); }
  });

  ui.el('playBtn').addEventListener('click', togglePlay);
  ui.el('exportMidi').addEventListener('click', () => exportMidi());
  ui.el('prevBtn').addEventListener('click', goBack);
  ui.el('nextBtn').addEventListener('click', goForward);

  // One slider for drift: all the way left is off, anywhere else is how far
  // it strays. The label dims at off so the state reads at a glance.
  ui.el('driftAmount').addEventListener('input', (e) => {
    const v = parseFloat(e.target.value);
    const on = v > 0;
    if (state.engine) {
      state.engine.driftOn = on;
      if (on) state.engine.driftAmount = v;
      else if (state.engine.base) state.engine.live = state.engine.base;
      // Off, the loop as composed from now; on, the next pass in the queue
      // wanders as far as asked.
      state.engine.redecide(on ? null : state.engine.base);
    }
    ui.el('wanderRow').classList.toggle('off', !on);
    store.setPrefs(on ? { drift: true, driftAmount: v } : { drift: false });
  });
  ui.el('trackLen').addEventListener('input', (e) => {
    if (!state.spec) return;
    const secs = TRACK_TIMES[parseInt(e.target.value, 10)] || 0;
    const passes = secs ? Math.max(1, Math.round(secs / ui.passSeconds(state.spec))) : 0;
    state.spec.playFor = passes || null;
    if (state.engine) state.engine.loopCount = 0;
    ui.el('lenVal').textContent = describeLength(passes, state.spec);
    state.passCount = 0;
    renderNow();
    updateSaveButton();
  });

  ui.el('bpm').addEventListener('input', (e) => {
    if (!state.spec) return;
    state.spec.bpm = parseInt(e.target.value, 10);
    ui.el('bpmVal').textContent = state.spec.bpm;
    // The same number of passes takes a different time at a new tempo.
    ui.el('lenVal').textContent = describeLength(state.spec.playFor || 0, state.spec);
    // The loop's own echo time in beats (queue item 73), at the new tempo.
    if (state.synth) state.synth.setEchoTime((60 / state.spec.bpm) * (state.engine ? state.engine.echoBeats : 0.75));
    if (state.engine) state.engine.redecide();
  });

  for (const k of ['warmth', 'space', 'wobble']) {
    ui.el(k).addEventListener('input', (e) => {
      if (!state.spec) return;
      state.spec.tone[k] = parseFloat(e.target.value);
      if (state.synth) state.synth.setTone(state.spec.tone);
    });
  }
  for (const [id, d] of [['trimDown', -5], ['trimUp', 5]]) {
    ui.el(id).addEventListener('click', () => {
      const r = ui.el('playheadSync');
      r.value = String(Math.max(-200, Math.min(200, (parseInt(r.value, 10) || 0) + d)));
      r.dispatchEvent(new Event('input'));
    });
  }
  ui.el('playheadSync').addEventListener('input', (e) => {
    const ms = parseInt(e.target.value, 10);
    ui.el('syncVal').textContent = `${ms} ms`;
    if (state.engine) state.engine.visualOffset = ms / 1000;
    store.setPrefs({ playheadSync: ms });
  });

  ui.el('volume').addEventListener('input', (e) => {
    if (state.synth) state.synth.setVolume(parseFloat(e.target.value));
    store.setPrefs({ volume: parseFloat(e.target.value) });
  });

  ui.el('liteMode').addEventListener('click', () => {
    state.lite = !state.lite;
    setLiteSwitch(state.lite);
    store.setPrefs({ lite: state.lite });
    if (state.synth) buildAudio();
  });

  // Mono or stereo, while a loop plays: the core fades from one to the
  // other, nothing is rebuilt.
  ui.el('monoMode').addEventListener('click', () => {
    state.mono = !state.mono;
    setSwitch('monoMode', state.mono);
    store.setPrefs({ mono: state.mono });
    if (state.synth) state.synth.setLayout(state.mono ? 1 : 2);
  });

  ui.el('exportJson').addEventListener('click', () => {
    ui.download(store.exportAll(), 'driftloom-loops.json');
  });
  ui.el('importBtn').addEventListener('click', () => ui.el('importFile').click());

  const renderDiag = () => {
    const lines = [];
    const push = (o) => { for (const [k, v] of Object.entries(o)) lines.push(`${k}: ${v}`); };
    lines.push(`build: ${BUILD}`);
    lines.push(`ua: ${navigator.userAgent}`);
    lines.push(`standalone: ${window.matchMedia('(display-mode: standalone)').matches}`);
    // Registration failures are swallowed so they cannot break the app, which
    // means a dead service worker -- no offline mode -- is otherwise invisible.
    lines.push(`sw: ${swState}${swEvents.length ? ` (${swEvents.join('; ')})` : ''}`);
    const gpu = gpuReport();
    lines.push(`gpu: ${gpu.renderer}  accelerated: ${gpu.accelerated}`);
    lines.push(`cores: ${navigator.hardwareConcurrency || '?'}  lite: ${state.lite}`);
    // Which synth plays the voices: the Rust core (with the notes that
    // arrived late and the ones that fell back to JS), or JS and why.
    lines.push(`engine: ${state.synth ? state.synth.engineReport() : ENGINE === 'js' ? 'js (asked)' : state.coreError ? `js (fallback: ${state.coreError})` : 'rust (loading)'}`);
    if (ENGINE === 'rust') lines.push(`core: ${state.core ? `loaded ${state.coreAt} ms after the page began` : state.coreError ? 'not loaded' : 'loading'}`);
    const host = state.synth && state.synth.core && !state.synth.core.failed ? state.synth.core : null;
    lines.push(`layout: ${host ? (host.layout === 2 ? 'stereo' : 'mono') : 'mono (the JavaScript synth)'}  mono switch: ${state.mono ? 'on' : 'off'}`);
    // What the core's render costs this device, each layout apart (queue
    // item 66; docs/CORE.md, promise 9: stereo must not double it).
    if (host) lines.push(`render: ${host.costReport()}`);
    // One loop in thirty is a deliberate doubling (roadmap item 12), and
    // "is this one of them?" is otherwise only answerable by ear, which is
    // no use at all when the question is whether it fired.
    lines.push(`choir: ${state.spec && choirOf(state.spec) ? 'yes' : 'no'}`);
    if (state.media) push(state.media.report());
    else lines.push('media: not built yet (press Play)');
    if (state.engine && state.engine.spec) push(state.engine.report());
    else lines.push('scheduler: not built yet (press Play)');
    lines.push(...awayReport());
    lines.push(...aheadLog());
    lines.push(...viewsReport());
    ui.el('diagOut').textContent = lines.join('\n');
  };

  ui.el('diagRefresh').addEventListener('click', renderDiag);
  ui.el('goAdvanced').addEventListener('click', renderDiag);
  ui.el('diagReset').addEventListener('click', () => {
    if (state.engine) state.engine.clearMetrics();
    renderDiag();
    ui.toast('Counters cleared');
  });
  ui.el('diagCopy').addEventListener('click', async () => {
    renderDiag();
    try {
      await navigator.clipboard.writeText(ui.el('diagOut').textContent);
      // The views' report starts again from here.
      viewsReset();
      ui.toast('Diagnostics copied');
    } catch {
      ui.toast('Select the text above and copy it manually');
    }
  });

  ui.el('copySong').addEventListener('click', async () => {
    if (!state.spec) return;
    try {
      await offerCode(share.encodeSong(state.spec), state.spec.name);
    } catch (err) {
      console.warn('Could not build a code', err);
      ui.toast('Could not build a code for this loop');
    }
  });

  ui.el('pasteCode').addEventListener('click', pasteCode);

  ui.el('copyBackup').addEventListener('click', async () => {
    try {
      await navigator.clipboard.writeText(await store.exportAll().text());
      ui.toast('Backup copied. Paste it somewhere safe.');
    } catch {
      ui.toast('This browser will not let the page reach the clipboard');
    }
  });

  ui.el('pasteBackup').addEventListener('click', async () => {
    let text = '';
    try {
      text = await navigator.clipboard.readText();
    } catch {
      text = prompt('Paste your backup here') || '';
    }
    if (!text.trim()) return;
    try {
      const res = store.importAll(text);
      savedChanged();
      if (res.failed) ui.toast('Could not save the restored loops');
      else if (res.added) ui.toast(`Restored ${res.added} loops${res.rejected ? `, skipped ${res.rejected}` : ''}`);
      else ui.toast(res.rejected ? `Skipped ${res.rejected} unreadable loops` : 'Nothing new in that backup');
    } catch {
      ui.toast('That does not look like a Driftloom backup');
    }
  });
  ui.el('importFile').addEventListener('change', async (e) => {
    const file = e.target.files[0];
    if (!file) return;
    try {
      const res = store.importAll(await file.text());
      savedChanged();
      if (res.failed) ui.toast('Could not save the restored loops');
      else if (res.added) ui.toast(`Restored ${res.added} loops${res.rejected ? `, skipped ${res.rejected}` : ''}`);
      else ui.toast(res.rejected ? `Skipped ${res.rejected} unreadable loops` : 'Nothing new in that file');
    } catch {
      ui.toast('That file could not be read');
    }
    e.target.value = '';
  });

  document.addEventListener('keydown', (e) => {
    if (e.target.matches('input, textarea')) return;
    if (e.code === 'Space' || e.code === 'MediaPlayPause') {
      e.preventDefault();
      togglePlay();
    } else if (e.key === 'n') newLoop();
    else if (e.key === 's') saveQuick();
    else if (e.key >= '1' && e.key <= '5') reroll(LAYERS[parseInt(e.key, 10) - 1]);
    else if (e.code === 'MediaNextTrack') {
      e.preventDefault();
      goForward();
    } else if (e.code === 'MediaPreviousTrack') {
      e.preventDefault();
      goBack();
    }
  });

  // The scheduler queues further ahead while hidden, so it has to be told
  // when that changes. Resume anything the system paused on the way out.
  document.addEventListener('visibilitychange', () => {
    pageEvent(document.hidden ? 'hidden' : 'shown');
    rememberAhead();
    if (document.hidden) awayBegin();
    else awayEnd();
    viewFlush(document.hidden);
    if (state.synth && state.ctx.state === 'suspended') state.ctx.resume();
    if (state.engine) state.engine.retune();
    if (document.visibilityState === 'visible' && state.media && state.engine
        && state.engine.playing) {
      state.media.resume();
    }
  });

  // A page about to be frozen or put away plans for a deep buffer at once,
  // as hidden does (queue item 47): it may wake to one.
  const frozen = (on, what) => () => {
    pageEvent(what);
    pageFrozen = on;
    if (on) rememberAhead();
    if (state.engine) state.engine.setFrozen(on);
  };
  document.addEventListener('freeze', frozen(true, 'frozen'));
  document.addEventListener('resume', frozen(false, 'resumed'));
  window.addEventListener('pagehide', frozen(true, 'pagehide'));
  window.addEventListener('pageshow', (e) => { if (e.persisted) frozen(false, 'pageshow')(); });

  watchLongTasks();
  const cores = navigator.hardwareConcurrency || 4;
  const prefs = store.getPrefs();
  state.vibes = vibeSelection(prefs.vibes);
  state.picks = picks.cleanPicks(prefs.picks);
  updateVibesKey();
  state.lite = prefs.lite ?? cores <= 4;
  setLiteSwitch(state.lite);
  state.mono = prefs.mono === true;
  setSwitch('monoMode', state.mono);
  // Drift was a switch plus an amount; the amount was never kept, so an old
  // "on" comes back at the default distance.
  const wander = prefs.drift === false ? 0 : (prefs.driftAmount ?? 1);
  ui.el('driftAmount').value = wander;
  ui.el('wanderRow').classList.toggle('off', !(wander > 0));
  if (prefs.volume != null) ui.el('volume').value = prefs.volume;
  const sync = prefs.playheadSync ?? 0;
  ui.el('playheadSync').value = sync;
  ui.el('syncVal').textContent = `${sync} ms`;

  state.spec = newSpec();
  // Build the grid for the metre of the loop that is actually on screen,
  // not for an assumed 4/4.
  state.gridSteps = state.spec.stepsPerBar || 16;
  lights = ui.buildPlayhead(state.gridSteps);
  cells = ui.buildLayers({ onReroll: reroll, onMute: toggleMute }, state.gridSteps);
  ui.resetCursor();
  resetHistory(state.spec);
  syncToneInputs(state.spec);
  state.lastEdit = prefs.edit === 'sounds' ? 'sounds' : 'layers';
  // Now Playing is where a first visit starts.
  setView(BASE_VIEWS.includes(prefs.view) ? prefs.view : 'now');
  updateHistoryButtons();
  updateSaveButton();

  // The name's size depends on the card's width.
  window.addEventListener('resize', () => ui.fitName());
}

wire();
// The core is fetched and loaded ahead, so it is ready by the time Play is
// pressed. A browser that refuses a context before a gesture still gets the
// bytes now, and the context on the first tap.
if (ENGINE === 'rust' && !coreUnsupported()) {
  fetchCore().catch(() => {});
  try { prepareAudio(); } catch { state.ctx = null; }
}

if ('serviceWorker' in navigator && location.protocol.startsWith('http')) {
  swState = 'registering';
  navigator.serviceWorker.register('sw.js').then((reg) => {
    // Followed as it moves: the state shown is the newest worker's, and
    // each step it takes is kept with its time.
    const watched = new WeakSet();
    const track = () => {
      const w = reg.installing || reg.waiting || reg.active;
      swState = w ? w.state : 'registered';
      if (w && !watched.has(w)) {
        watched.add(w);
        swEvent(w.state);
        w.addEventListener('statechange', () => { swEvent(w.state); track(); });
      }
    };
    track();
    reg.addEventListener('updatefound', () => { swEvent('update found'); track(); });
  }).catch((err) => { swState = `failed: ${err && err.message ? err.message : err}`; });
}