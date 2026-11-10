import { newSpec as drawSpec, render, unavailable as noGenerator } from './gen.js';
import { RequestError, vibeSelection, rerollLayer, cloneSpec, LAYERS, STEPS_PER_BAR } from './request.js';
import { Output } from './output.js';
import { loadCore, fetchCore, coreUnsupported, newContext, DROP_CAUSES } from './core.js';
import * as quality from './quality.js';
import { picksAsk, PlayerEngine } from './player.js';
import { patternToMidi } from './midi.js';
import { MediaBridge } from './media.js';
import * as share from './share.js';

// Build stamp. Shown in Diagnostics so that after a deploy you can confirm
// in one glance which version you are actually running, rather than
// guessing whether a change landed. Bump it with CACHE in sw.js.
const BUILD = 'v152';

// A test's slow device (queue item 87b): `?burn=1200` adds that many
// microseconds of work to every block the core renders, so the measuring and
// the rule that picks "lighter on processor" can be tried on a fast machine
// (tools/perf.mjs --burn). Nothing else reads it.
const BURN = Number(new URLSearchParams(location.search).get('burn')) || 0;

// The app plays the core's player (queue item 94b): the core decides every
// note on the audio thread, and js/player.js sends it commands and draws its
// events. There is nothing else to play on (queue item 97): a browser whose
// core will not run is told so in a plain line, in the info card's place
// (Mikey, 2026-11-07).
const CANT_PLAY = "This browser can't play Driftloom; try a recent Chrome, Firefox or Safari";
// And where the browser could run the core but the core did not arrive (a
// first visit with no connection, a server error): not the browser's fault,
// so not the line above.
const NOT_LOADED = "Driftloom couldn't load. Check the connection and reload.";

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
// when the slider moves; a time shorter than one pass (a long loop's pass
// can be hours) as bars, and the track ends at the first phrase line after
// them (queue item 88b; `trackLength`). The stops are curated: fine at the short end, where
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
  output: null,
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
  picks: { tempo: null, length: null, key: null, scale: null, style: null, voices: null, layers: null },
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
  // Lighter on processor chosen by the app (queue item 87b, js/quality.js):
  // what the worklet says about how busy the audio thread is, whether the
  // loop played through the last report, whether the first guess (four
  // cores or fewer) started it on lite, and the switch under way, if one is.
  busy: new quality.Busy(),
  busyPlaying: false,
  liteGuess: false,
  switching: null,
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
  // The picks as sent to the core (queue item 95): each sending numbered,
  // what it held (the vibes, the picks, the request read, and whether only
  // the vibes could be sent), the last sent, and the one the core says it
  // holds, which Now Playing's pills and the panel's line show.
  picksTag: 0,
  sentPicks: new Map(),
  lastPicks: null,
  held: null,
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
// Until it does Play waits for it; a later output (the quality toggle)
// takes it from the start.
function prepareAudio() {
  if (state.ctx) return;
  const Ctx = window.AudioContext || window.webkitAudioContext;
  state.ctx = newContext(Ctx, { latencyHint: 'playback' });
  state.ctx.addEventListener('statechange', () => awayEvent(`audio ${state.ctx.state}`));
  const ctx = state.ctx;
  const why = coreUnsupported();
  if (why) {
    cantPlay(why);
    return;
  }
  loadCore(ctx).then((core) => {
    if (state.ctx !== ctx) return;
    state.core = core;
    state.coreAt = Math.round(performance.now());
    if (state.output) state.output.attachCore(core);
    // The player, waiting for it (Play came first), is given the loop now.
    if (state.engine && state.engine.coreArrived) state.engine.coreArrived();
  }).catch((err) => {
    if (state.ctx !== ctx) return;
    cantPlay(String((err && err.message) || err));
  });
}

// The core will not play here (no WebAssembly or no AudioWorklet, its
// module did not load, or it failed while playing): the music stops, the
// info card's place says so in a plain line, and Diagnostics says why. The
// answer holds for the session.
function cantPlay(why) {
  if (state.coreError) return;
  state.coreError = why;
  if (state.engine && state.engine.playing) {
    state.engine.stop();
    state.media.stop();
    state.media.setPlaybackState(false);
    setPlayButton(false);
  }
  ui.plainLine(CANT_PLAY);
}

function ensureAudio() {
  prepareAudio();
  if (state.ctx.state === 'suspended') state.ctx.resume();
  if (!state.output) buildAudio();
}

// An output, its media bridge and an engine on it, at the quality the app is
// on, wired to the app's controls: what buildAudio installs, and what a
// quality switch builds beside the one playing (queue item 87b).
function makeAudio() {
  // A core that has failed (loading, or while playing) stays failed for the
  // session: the new output is not handed it, and says why.
  const output = new Output(state.ctx, state.lite ? 'lite' : 'full', {
    layout: state.mono ? 1 : 2,
    core: state.coreError ? null : state.core,
    coreError: state.coreError,
    onCoreError: (message) => cantPlay(message),
    burn: BURN,
  });
  output.onBusy = (busy) => onBusy(output, busy);
  // Route the mix through a media element so the phone gives us lock-screen
  // controls and stops treating us as an idle tab.
  const media = new MediaBridge(state.ctx);
  media.onchange = awayEvent;
  media.attach(output.output);
  media.setHandlers({
    onPlay: () => { if (!state.engine.playing) togglePlay(); },
    onPause: () => { if (state.engine.playing) togglePlay(); },
    onNext: () => goForward(),
    onPrev: () => goBack(),
  });
  const engine = new PlayerEngine(state.ctx, output);
  engine.onStep = onStep;
  engine.onLoop = onLoop;
  engine.onTrackEnd = onTrackEnd;
  // The core's own loops and its answer to the picks (queue item 95).
  engine.onOwn = onOwn;
  engine.onPicks = onPicksHeld;
  if (state.lastPicks) engine.setPicks(state.lastPicks.ask, state.lastPicks.tag);
  engine.visualOffset = (parseInt(ui.el('playheadSync').value, 10) || 0) / 1000;
  const wander = parseFloat(ui.el('driftAmount').value);
  engine.driftOn = wander > 0;
  engine.driftAmount = wander > 0 ? wander : 1;
  output.setVolume(parseFloat(ui.el('volume').value));
  return { output, media, engine };
}

// Let an old output, its bridge and its engine go.
function dropAudio({ output, media, engine }) {
  if (engine) engine.stop();
  // Tear the graph down, or each rebuild leaves a whole output wired to the
  // speakers and another keepalive element in the DOM.
  if (media) media.dispose();
  if (output) output.dispose();
}

function buildAudio() {
  const wasPlaying = state.engine ? state.engine.playing : false;
  // A quality switch under way is overtaken: the old output (the one
  // playing) goes now, at the quality already chosen.
  state.switching = null;
  // The play's wander salt (queue item 92) goes on under the new engine.
  const salt = state.engine ? state.engine.salt : null;
  dropAudio({ output: state.output, media: state.media, engine: state.engine });
  ({ output: state.output, media: state.media, engine: state.engine } = makeAudio());
  if (salt != null) state.engine.salt = salt;
  state.busy.reset();
  if (state.spec) {
    // The same loop, resumed under the new engine (queue item 69: its
    // late floor doesn't wait again).
    state.pattern = state.engine.load(state.spec, { resume: true });
    armNext();
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

// ------------------------------------------------- lighter on processor

// A report from the worklet of how busy the audio thread was (queue item
// 87b): kept while the loop played through it, and held to the rule.
function onBusy(output, report) {
  if (output !== state.output || state.switching) return;
  const playing = !!(state.engine && state.engine.playing);
  state.busy.add(report, playing && state.busyPlaying, performance.now() / 1000);
  state.busyPlaying = playing;
  if (!output.core || output.core.failed) return;
  const prefs = store.getPrefs();
  const d = quality.decide({ quality: output.quality, yours: prefs.lite, auto: prefs.liteAuto, guess: state.liteGuess, busy: state.busy });
  if (d) autoQuality(d);
}

// The app moves to lite (or, once, up to full): kept as its own choice,
// apart from the listener's, and heard at the next pass (or bar): the old
// quality fades out to it, and the new one takes the loop from its top
// (`quality.handOff` says why it is built only then). On the player as on
// the page's engine (queue item 94b; a crossfade is a later item): the new
// core starts the loop again, its lateFloor not waited out again.
function autoQuality({ lite, why, at }) {
  store.setPrefs({ liteAuto: { lite, why, at: new Date().toISOString() } });
  state.lite = lite;
  setLiteSwitch(lite);
  showLiteSource();
  awayEvent(`lighter on processor: ${lite ? 'on' : 'off'} by itself (${why})`);
  const engine = state.engine;
  if (!engine || !engine.playing || !state.spec) {
    // Nothing playing: the next output is simply built at the new quality.
    if (state.output) buildAudio();
    return;
  }
  const now = state.ctx.currentTime;
  const base = engine.live || engine.base;
  const unit = at === 'bar' ? (state.spec.stepsPerBar || 16) : base.totalSteps;
  const when = quality.boundary(engine, unit, now + quality.PREP + quality.FADE);
  if (when === null) {
    buildAudio();
    return;
  }
  const old = { output: state.output, media: state.media, engine };
  quality.handOff(old, when);
  const alarm = quality.alarm(state.ctx, when, () => {
    if (!state.switching || state.switching.old !== old) return;
    state.switching = null;
    // buildAudio lets the old one go and starts the loop again, from its
    // top, on the new quality.
    buildAudio();
  });
  state.switching = { old, alarm, at: when };
}

// The switch's line on the Advanced page, and Diagnostics: who chose.
function liteSource() {
  const prefs = store.getPrefs();
  return quality.source({ yours: prefs.lite, auto: prefs.liteAuto, cores: navigator.hardwareConcurrency || '?' });
}

function showLiteSource() {
  const prefs = store.getPrefs();
  const hint = ui.el('liteAuto');
  if (!hint) return;
  const auto = (prefs.lite === undefined || prefs.lite === null) && prefs.liteAuto;
  hint.hidden = !auto;
  if (auto) hint.textContent = prefs.liteAuto.lite ? 'Turned on by itself: this device was working hard at full quality.' : 'Turned off by itself: this device has room to spare.';
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
  armNext();
}

// The loop that follows when the track ends, where a length is set (queue
// item 94b): the album's next, the history's next, or a new loop drawn
// ahead. The core's player is armed with it, so it starts on the line with
// the page asleep; onTrackEnd then takes the same one.
function nextForEnd() {
  const spec = state.spec;
  if (!spec || !(spec.playFor || spec.playBars)) return null;
  const pl = state.playlist;
  if (pl && pl.ids.length) {
    const id = pl.ids[(pl.index + 1) % pl.ids.length];
    const entry = store.loadAll().find((e) => e.id === id);
    return entry ? cloneSpec(entry.spec) : null;
  }
  if (state.historyIndex < state.history.length - 1) return cloneSpec(state.history[state.historyIndex + 1]);
  // The next new loop is the core's own, with the picks it holds (queue
  // item 95): nothing of the page's is armed.
  return null;
}

function armNext() {
  if (state.engine) state.engine.arm(nextForEnd());
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
  return state.output && state.output.core ? state.output.core.late : null;
}

// The core's late and dropped notes by kind (queue item 41), for the views'
// report; null without the core.
function coreCounts() {
  const c = state.output && state.output.core;
  if (!c) return null;
  return { late: c.late, missed: c.missed, dropped: c.dropped, by: c.droppedBy.slice() };
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

function pageEvent(what) {
  pageEvents.push([performance.now(), what]);
  if (pageEvents.length > 400) pageEvents.splice(1, 1);
}

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

function pageLog() {
  const visits = pageEvents.filter(([, w]) => w !== 'opened').slice(-12)
    .map(([at, w]) => `${w} ${(at / 1000).toFixed(1)} (${whereAt(at)})`);
  return [`page events (s): ${visits.length ? visits.join(', ') : 'none'}`];
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
      missed: 0, dropped: 0, droppedBy: DROP_CAUSES.map(() => 0),
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
    // A output built again meanwhile starts its counts from zero: only what
    // went up is counted.
    const a = o.counts;
    if (counts && a) {
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
    const why = v.lateNotes ? ` (left out ${v.missed})` : '';
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
// thinks in passes, so show the time it comes to at this tempo instead:
// whole passes, or a length shorter than a pass counted in time (queue item
// 88b). Kept short: it sits beside the slider and must not change width much.
function describeLength(spec) {
  const secs = lengthSeconds(spec);
  return secs ? ui.formatTime(secs) : 'off';
}

// The time the track length set comes to at this tempo, or 0 for none.
function lengthSeconds(spec) {
  if (!spec) return 0;
  if (spec.playFor) return spec.playFor * ui.passSeconds(spec);
  if (spec.playBars) return spec.playBars * ui.barSeconds(spec);
  return 0;
}

// The slider's time, `secs` (0 for off), as the spec keeps it: whole passes
// where it is a pass or longer, as ever; shorter than a pass, the bars it
// comes to, rounded up (queue item 88b: the track ends at the first phrase
// line after them, its notes fading).
function trackLength(spec, secs) {
  spec.playFor = null;
  delete spec.playBars;
  if (!secs) return;
  const pass = ui.passSeconds(spec);
  if (secs >= pass) spec.playFor = Math.max(1, Math.round(secs / pass));
  else spec.playBars = Math.max(1, Math.ceil(secs / ui.barSeconds(spec) - 1e-9));
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
  sendPicks();
  armNext();
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
  sendPicks();
  armNext();
  if (state.view === 'now') renderNow();
  if (state.view === 'vibe') picks.renderPanel();
}

// The panel's request sent to the core as a command (queue item 95): it holds
// it for every loop it makes itself. Each sending is numbered, so a loop the
// core made says which picks it took (`onOwn`); a request the reader can't
// read goes as its vibes alone, as the page's own draw kept the moods of one
// it could not meet.
function sendPicks() {
  const request = picks.requestFor(state);
  const sent = picksAsk(request);
  const tag = ++state.picksTag;
  state.sentPicks.set(tag, {
    tag, vibes: state.vibes.slice(), picks: { ...state.picks }, request, vibesOnly: sent.vibesOnly, why: sent.why, met: null,
  });
  // Kept: the last few, and the one the core says it holds.
  for (const t of state.sentPicks.keys()) {
    if (state.sentPicks.size <= 32) break;
    if (!state.held || t !== state.held.tag) state.sentPicks.delete(t);
  }
  state.lastPicks = { ask: sent.ask, tag };
  if (state.engine && state.engine.setPicks) state.engine.setPicks(sent.ask, tag);
}

// The core's answer to the picks numbered `tag`: held whole (`met`), or only
// their vibes, and why not in its own words.
function onPicksHeld({ tag, met, why }) {
  const entry = state.sentPicks.get(tag);
  if (!entry) return;
  entry.met = met && !entry.vibesOnly;
  entry.coreWhy = why;
  state.held = entry;
  if (state.view === 'now') renderNow();
  if (state.view === 'vibe') picks.renderPanel();
}

// What the picks numbered `tag` asked of the core: the request it held.
function heldRequest(entry) {
  if (!entry) return state.vibes.slice();
  return entry.vibesOnly || entry.met === false ? entry.vibes.slice() : entry.request;
}

// A loop the core made itself has started (queue item 95): at Next, or where
// the length slider's time ran out with nothing of the page's armed. The
// page draws the same spec for its card, history and share codes from the
// seed and the picks the core took (the generation goldens hold the two
// generators to the same loop), and loads it: the engine sees the core has
// it and sends nothing.
function onOwn(seed, tag) {
  setTimeout(() => {
    let spec;
    try {
      spec = drawSpec(seed, heldRequest(state.sentPicks.get(tag)));
    } catch (e) {
      if (!(e instanceof RequestError)) throw e;
      spec = drawSpec(seed, state.vibes);
    }
    state.playlist = null;
    if (state.spec) {
      const last = state.history[state.history.length - 1];
      if (!last || JSON.stringify(last) !== JSON.stringify(state.spec)) pushHistory(state.spec);
    }
    pushHistory(spec);
    loadSpec(spec, { pushToHistory: false });
    ui.toast(`New loop: ${spec.name}`);
    updateHistoryButtons();
  }, 0);
}

// An unlit any key puts its line back to chance. Line two holds tempo and
// length, so it clears both.
function clearPickLine(line) {
  if (line === 'mood') setVibes([]);
  else if (line === 'pace') setPicks({ tempo: null, length: null });
  else if (line === 'key') setPicks({ key: null, scale: null });
  else if (line === 'style') setPicks({ style: null });
  else if (line === 'voices') setPicks({ voices: null, layers: null });
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
  return e ? e.audibleSteps() : null;
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
  const pos = state.nowSteps || 0;
  let text;
  let frac;
  // A set track length counts down to where it hands over (queue item 88b:
  // whole passes, or a phrase line inside one); a loop's own length, however
  // long, never ends it. Today's clock: the long-piece display waits for a
  // Painter session.
  const end = e.endStep();
  const round = e.roundSteps();
  if (end != null) {
    text = `\u2212${clock(Math.max(0, end - pos) * e.stepDur)}`;
    frac = Math.min(1, Math.max(0, (pos - e.countFrom) / Math.max(1, end - e.countFrom)));
  } else {
    const total = Number.isFinite(round) ? round : e.base.totalSteps;
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
  armNext();
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
  const secs = lengthSeconds(spec);
  const idx = secs ? TRACK_TIMES.reduce((best, v, i) =>
    (i && Math.abs(v - secs) < Math.abs(TRACK_TIMES[best] - secs) ? i : best), 1) : 0;
  ui.el('trackLen').value = idx;
  ui.el('lenVal').textContent = describeLength(spec);
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

// The loop the page opens on, and Play before any loop exists, lean toward
// the vibes: drawn here, since there is no core before a tap. Every other
// new loop is the core's own (`newLoop`). The picks are its request (queue
// item 85); one the generator cannot meet (the panel says so) is left out,
// the moods kept.
function newSpec() {
  let spec;
  try {
    spec = drawSpec(undefined, picks.requestFor(state));
  } catch (e) {
    if (!(e instanceof RequestError)) throw e;
    spec = drawSpec(undefined, state.vibes);
  }
  return spec;
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
    nowPlaying();
  }
}

// What the page does when it starts playing: the frames drawn, the phone
// told, the button.
function nowPlaying() {
  if (!state.frameHandle) state.frameHandle = requestAnimationFrame(frameLoop);
  syncMediaMetadata();
  state.media.setPlaybackState(true);
  setPlayButton(true);
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
  ensureAudio();
  // The core makes it (queue item 95), with the picks it holds; the page
  // catches up when it starts (`onOwn`), which keeps the history and says
  // its name.
  const was = state.engine.playing;
  if (!state.engine.spec && state.spec) {
    resetHistory(state.spec);
    loadSpec(state.spec, { pushToHistory: false });
  }
  state.engine.next();
  if (!was) {
    state.media.start();
    nowPlaying();
  }
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
  // The channel goes quiet at once either way; back on, its notes are
  // decided again from now (on the page's engine, not after what the core
  // holds: queue item 40).
  state.engine.setMute(layer, state.spec.mutes[layer]);
  ui.renderGrids(cells, state.engine.live || state.pattern, Math.max(0, state.bar), state.spec.mutes);
}

// The loop playing as its code carries it: with the wander's salt of this
// play when it wanders across the slider's upper range (queue item 92), so
// the code says which play it was shared from. Opening it draws a play's
// own salt, as any play does.
function sharedSpec() {
  const spec = { ...state.spec };
  delete spec.wanderSalt;
  const salt = state.engine ? state.engine.wanderSalt : 0;
  if (salt) spec.wanderSalt = salt;
  return spec;
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
    // Off, the loop as composed; on, as far as asked.
    if (state.engine) state.engine.setWander(v);
    ui.el('wanderRow').classList.toggle('off', !on);
    store.setPrefs(on ? { drift: true, driftAmount: v } : { drift: false });
  });
  ui.el('trackLen').addEventListener('input', (e) => {
    if (!state.spec) return;
    const secs = TRACK_TIMES[parseInt(e.target.value, 10)] || 0;
    trackLength(state.spec, secs);
    // Counted from what is heard now, the pass playing as the first; on the
    // player, with the loop that follows armed for its end.
    if (state.engine) state.engine.trackChanged(nextForEnd());
    ui.el('lenVal').textContent = describeLength(state.spec);
    state.passCount = 0;
    renderNow();
    updateSaveButton();
  });

  ui.el('bpm').addEventListener('input', (e) => {
    if (!state.spec) return;
    state.spec.bpm = parseInt(e.target.value, 10);
    ui.el('bpmVal').textContent = state.spec.bpm;
    // The same number of passes takes a different time at a new tempo.
    ui.el('lenVal').textContent = describeLength(state.spec);
    if (state.engine) state.engine.setTempo(state.spec.bpm);
  });

  for (const k of ['warmth', 'space', 'wobble']) {
    ui.el(k).addEventListener('input', (e) => {
      if (!state.spec) return;
      state.spec.tone[k] = parseFloat(e.target.value);
      if (state.engine) state.engine.setTone(state.spec.tone);
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
    if (state.output) state.output.setVolume(parseFloat(e.target.value));
    store.setPrefs({ volume: parseFloat(e.target.value) });
  });

  ui.el('liteMode').addEventListener('click', () => {
    state.lite = !state.lite;
    setLiteSwitch(state.lite);
    // The listener's choice: kept as theirs, and never changed by the app.
    store.setPrefs({ lite: state.lite });
    showLiteSource();
    if (state.output) buildAudio();
  });

  // Mono or stereo, while a loop plays: the core fades from one to the
  // other, nothing is rebuilt.
  ui.el('monoMode').addEventListener('click', () => {
    state.mono = !state.mono;
    setSwitch('monoMode', state.mono);
    store.setPrefs({ mono: state.mono });
    if (state.output) state.output.setLayout(state.mono ? 1 : 2);
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
    lines.push(`cores: ${navigator.hardwareConcurrency || '?'}  lite: ${state.lite}, ${liteSource()}`);
    // "Let the loop wander" (queue item 92): unrepeatable across the
    // slider's upper range, with this play's salt.
    if (state.engine) {
      const e = state.engine;
      const salt = e.wanderSalt;
      lines.push(`wander: ${e.driftOn ? e.driftAmount : 'off'}${salt ? `, unrepeatable (salt ${salt})` : ', repeatable'}`);
    }
    // How busy the audio thread has been (queue item 87b): the busiest
    // quarter-second over the rule's spans, while the loop played.
    {
      const b20 = state.busy.busiest(quality.OVER_FOR);
      const b60 = state.busy.busiest(quality.UNDER_FOR);
      const p = (x) => (x === null ? 'not yet' : `${Math.round(x * 100)}%`);
      lines.push(`busiest: ${p(b20)} over ${quality.OVER_FOR} s, ${p(b60)} over ${quality.UNDER_FOR} s (${state.output ? state.output.quality : '?'}; switches to lite past ${Math.round(quality.OVER * 100)}%)`);
    }
    // The core's player, with the notes it took late, left out or dropped;
    // or why there is none.
    lines.push(`engine: ${state.output ? state.output.engineReport() : state.coreError ? `none (${state.coreError})` : 'rust (loading)'}`);
    lines.push(`core: ${state.core ? `loaded ${state.coreAt} ms after the page began` : state.coreError ? 'not loaded' : 'loading'}`);
    const host = state.output && state.output.core && !state.output.core.failed ? state.output.core : null;
    lines.push(`layout: ${host ? (host.layout === 2 ? 'stereo' : 'mono') : '-'}  mono switch: ${state.mono ? 'on' : 'off'}`);
    // What the core's render costs this device, each layout apart (queue
    // item 66; docs/CORE.md, promise 9: stereo must not double it).
    if (host) lines.push(`render: ${host.costReport()}`);
    // One loop in thirty is a deliberate doubling (roadmap item 12), and
    // "is this one of them?" is otherwise only answerable by ear, which is
    // no use at all when the question is whether it fired.
    lines.push(`choir: ${state.pattern && state.pattern.choir ? 'yes' : 'no'}`);
    if (state.media) push(state.media.report());
    else lines.push('media: not built yet (press Play)');
    if (state.engine && state.engine.spec) push(state.engine.report());
    else lines.push('scheduler: not built yet (press Play)');
    lines.push(...awayReport());
    lines.push(...pageLog());
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
      await offerCode(share.encodeSong(sharedSpec()), state.spec.name);
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

  // Resume anything the system paused on the way out. (The core plays on
  // whatever the page does: nothing is planned for a hidden page.)
  document.addEventListener('visibilitychange', () => {
    pageEvent(document.hidden ? 'hidden' : 'shown');
    if (document.hidden) awayBegin();
    else awayEnd();
    viewFlush(document.hidden);
    if (state.output && state.ctx.state === 'suspended') state.ctx.resume();
    if (document.visibilityState === 'visible' && state.media && state.engine
        && state.engine.playing) {
      state.media.resume();
    }
  });

  // A page frozen or put away, for Diagnostics' page events.
  document.addEventListener('freeze', () => pageEvent('frozen'));
  document.addEventListener('resume', () => pageEvent('resumed'));
  window.addEventListener('pagehide', () => pageEvent('pagehide'));
  window.addEventListener('pageshow', (e) => { if (e.persisted) pageEvent('pageshow'); });

  watchLongTasks();
  const cores = navigator.hardwareConcurrency || 4;
  const prefs = store.getPrefs();
  state.vibes = vibeSelection(prefs.vibes);
  state.picks = picks.cleanPicks(prefs.picks);
  // The picks go to the core as it starts (and survive a reload as prefs).
  sendPicks();
  updateVibesKey();
  // The listener's choice; else the app's own, if it has moved (queue item
  // 87b); else the first guess, from the cores.
  state.liteGuess = (prefs.lite === undefined || prefs.lite === null) && !prefs.liteAuto && cores <= 4;
  state.lite = prefs.lite ?? (prefs.liteAuto ? prefs.liteAuto.lite : cores <= 4);
  setLiteSwitch(state.lite);
  showLiteSource();
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

if (noGenerator) {
  // No loop can be made or shown here (js/gen.js): the page is the line.
  state.coreError = noGenerator;
  ui.plainLine(coreUnsupported() ? CANT_PLAY : NOT_LOADED);
} else {
  wire();
  // The core is fetched and loaded ahead, so it is ready by the time Play is
  // pressed. A browser that refuses a context before a gesture still gets
  // the bytes now, and the context on the first tap.
  if (coreUnsupported()) {
    cantPlay(coreUnsupported());
  } else {
    fetchCore().catch(() => {});
    try { prepareAudio(); } catch { state.ctx = null; }
  }
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