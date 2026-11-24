import { newSpec as drawSpec, render, unavailable as noGenerator } from './gen.js';
import { RequestError, vibeSelection, rerollLayer, cloneSpec, LAYERS, LAYER_LABELS, STEPS_PER_BAR } from './request.js';
import { Output } from './output.js';
import { loadCore, fetchCore, coreUnsupported, newContext, DROP_CAUSES } from './core.js';
import { nativeBridge, NativeContext } from './native.js';
import * as quality from './quality.js';
import { JUMP, picksAsk, PlayerEngine } from './player.js';
import { patternToMidi } from './midi.js';
import { AudioExport } from './export.js';
import { MediaBridge } from './media.js';
import { appBridge, appLink } from './app.js';
import { watchViewport } from './viewport.js';
import { shortcutOf, ownKeys, keeps, keyList, keyNames, onMac, REPEATS, VIEW_KEYS } from './shortcuts.js';
import * as share from './share.js';

// Build stamp. Shown in Diagnostics so that after a deploy you can confirm
// in one glance which version you are actually running, rather than
// guessing whether a change landed. Bump it with CACHE in sw.js.
const BUILD = 'v216';

// A test's slow device (queue item 87b): `?burn=1200` adds that many
// microseconds of work to every block the core renders, so the measuring and
// the rule that picks "lighter on processor" can be tried on a fast machine
// (tools/perf.mjs --burn). Nothing else reads it.
const BURN = Number(new URLSearchParams(location.search).get('burn')) || 0;

// The app plays the core's player (queue item 94b): the core decides every
// note on the audio thread, and js/player.js sends it commands and draws its
// events. There is nothing else to play on (queue item 97): a browser whose
// core will not run is told so in a plain line, in the info card's place
// (Mikey, 2026-11-07; shorter in session 30e, the same meaning).
const CANT_PLAY = "Can't play here. Try recent Chrome, Firefox or Safari.";
// And where the browser could run the core but the core did not arrive (a
// first visit with no connection, a server error): not the browser's fault,
// so not the line above.
const NOT_LOADED = 'Please refresh.';
// And one that plays it, but too slowly even on lite (queue item 100; Mikey,
// 2026-11-10): the line, and where the Driftloom app is. The link is empty
// until the Android app has a page (F-Droid, queue item 107); the second
// sentence shows only when it is set.
const TOO_SLOW = 'Browser too slow.';
// The app's name, where the page needs it (the app is renamed at 1.0:
// session 31a changes it here). Since session 30e no popup names it.
const APP_NAME = 'Driftloom';
// The app's address, where a link to a loop has none of the page's own (the
// Android app's files are inside it). Renamed with the app; index.html's
// preview tags carry the same address and name (test/preview.test.mjs holds
// them, the manifest and these two together).
const APP_URL = 'https://driftloom.nowmichaelclark.workers.dev/';
// Each a popup of two or three words, and what a screen reader hears for it.
const OLD_CODE = ['Code too old', `That code is from an older ${APP_NAME} and can't be opened.`];
const NEWER_CODE = ['Needs update', `That code needs a newer ${APP_NAME}.`];
const NOT_A_CODE = ['Bad code', `That doesn't look like a ${APP_NAME} code.`];
const NOT_A_BACKUP = ['Bad backup', `That doesn't look like a ${APP_NAME} backup.`];
// The core's refusal for a code that names an instrument this device does
// not have (it can't say so yet: today it says "needs a newer version" for
// any id it lacks, and session B4 settles whether it plays something close
// instead). share.codeKind answers 'instrument' for such a refusal.
const INSTRUMENT_CODE = ['Missing instrument', "That code needs an instrument this device doesn't have."];
// A link whose code is damaged (cut short, or a character changed).
const BAD_LINK = ['Bad link', "That link's code can't be read."];
// Storage refused (a private window, or the site's storage full).
const CANT_SAVE = ['Storage blocked', "Can't save: this browser is refusing to store data."];
const GET_APP = 'Get app.';
const APP_LINK = '';

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
import { coverImage, coverFile, paint } from './covers.js';
import * as store from './storage.js';
import * as ui from './ui.js';
import * as library from './library.js';
import { VIBE_IDS } from './vibes.js';
import * as picks from './picks.js';
import * as pickers from './pickers.js';
import * as tracklen from './tracklen.js';
import * as piece from './piece.js';

const state = {
  ctx: null,
  output: null,
  engine: null,
  spec: null,
  pattern: null,
  currentId: null,
  bar: -1,
  gridSteps: 16,
  // What the layers last drew from (the engine's rowsVersion).
  rowsDrawn: -1,
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
  picks: { tempo: null, length: null, key: null, scale: null, style: null, voices: null, layers: null, journey: null },
  // The album most recently left, so Now Playing can offer the way back.
  lastAlbum: null,
  // Now Playing's last audible position, in steps, held while paused.
  nowSteps: 0,
  // The rail pressed (session 30h): { id, f } while a finger or the pointer
  // is down on it, and the step the keys last asked for (and when).
  railDrag: null,
  railAsked: null,
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
  // Set once lite is past the line (queue item 100): { at, why }.
  slow: null,
  liteGuess: false,
  switching: null,
  // The Mono switch (queue item 66): the listener's, kept in the prefs and
  // never in a share code or a backup. Off plays the stereo render.
  mono: false,
  // The letter keys switch (session 30n, item 122): kept as Mono is.
  letterKeys: true,
  // The volume M brings back.
  unmuteTo: null,
  media: null,
  // The audio export's sheet and its Worker (queue item 104b, js/export.js).
  audioExport: null,
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
  // In the Android app the core plays natively (session 27, js/native.js):
  // its context is a stand-in on the core's clock, and the WebView makes no
  // audio of its own.
  const bridge = nativeBridge();
  const Ctx = window.AudioContext || window.webkitAudioContext;
  state.ctx = bridge ? new NativeContext(bridge) : newContext(Ctx, { latencyHint: 'playback' });
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
  // A jump held while paused (session 30h): the place drawn when it is said.
  engine.onPlace = () => { if (state.view === 'now') renderNow(); };
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
  // An audio export running (queue item 104b): the report is the export's
  // (a busy audio thread slows it), and never moves the app to lite: the
  // export's load is not the device's.
  if (state.audioExport && state.audioExport.watchBusy(report)) {
    state.busyPlaying = false;
    return;
  }
  state.busy.add(report, playing && state.busyPlaying, performance.now() / 1000, fillIns());
  state.busyPlaying = playing;
  if (!output.core || output.core.failed) return;
  // Too slow even on lite (queue item 100): said once, in the card's place.
  // Play goes on.
  if (!state.slow && !state.coreError) {
    const s = quality.stuck({ quality: output.quality, busy: state.busy, native: !!state.ctx.native });
    if (s) {
      state.slow = { at: performance.now(), why: s.why };
      ui.plainLine(TOO_SLOW, APP_LINK ? { text: GET_APP, href: APP_LINK } : null);
      awayEvent(`too slow even on lite (${s.why})`);
    }
  }
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
  switchQuality(at);
}

// The output rebuilt at the quality now chosen (`state.lite`), the loop
// playing handed over without a cut: the old one fades out and is let go
// once it is silent, and the new one takes the loop from its top. The app's
// own switch waits for the next pass (or bar, `at`); the listener's ('now',
// session 30g, step 4) fades at once, briefly: letting the old output go
// at once cut the sound mid-note (a step in the samples; android/native's
// `a_switch_by_hand_or_by_itself_goes_over_without_a_step`). A switch
// already under way builds at the quality chosen when it ends.
function switchQuality(at) {
  const engine = state.engine;
  if (!engine || !engine.playing || !state.spec) {
    // Nothing playing: the next output is simply built at the new quality.
    if (state.output) buildAudio();
    return;
  }
  if (state.switching) return;
  const now = state.ctx.currentTime;
  let when;
  let fade = quality.FADE;
  if (at === 'now') {
    fade = quality.HAND_FADE;
    when = now + quality.HAND_PREP + fade;
  } else {
    const base = engine.live || engine.base;
    const unit = at === 'bar' ? (state.spec.stepsPerBar || 16) : base.totalSteps;
    when = quality.boundary(engine, unit, now + quality.PREP + quality.FADE);
  }
  if (when === null) {
    buildAudio();
    return;
  }
  const old = { output: state.output, media: state.media, engine };
  quality.handOff(old, when, fade);
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
  // The switch's description reads it, hidden or not: empty where it is yours.
  hint.textContent = '';
  if (!auto) return;
  hint.textContent = 'auto';
  const why = document.createElement('span');
  why.className = 'vh';
  why.textContent = prefs.liteAuto.lite ? ': turned on by itself; this device was working hard at full quality.' : ': turned off by itself; this device has room to spare.';
  hint.append(why);
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
    sayLoop(spec.name);
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
  sayLoop(spec.name);
}

// The loop come to: the card names it, so it is only said, unless the card
// is showing an album.
function sayLoop(name, said = name) {
  if (library.shownAlbum()) ui.toast(name, said);
  else ui.announce(said);
}

function resetHistory(spec) {
  state.history = [JSON.parse(JSON.stringify(spec))];
  state.historyIndex = 0;
  updateHistoryButtons();
}

function updateHistoryButtons() {
  const prev = ui.el('prevBtn');
  // In an album Previous is the album's, whatever the history says (its
  // tracks don't enter the history): the track before, round from the first
  // to the last. Outside one it is the history's.
  const inAlbum = !!(state.playlist && state.playlist.ids.length);
  if (prev) prev.disabled = !inAlbum && state.historyIndex <= 0;
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
  if (!spec || !tracklen.ends(spec)) return null;
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
// it was hidden, the notes that reached the core late, the times the audio
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
  const late = a.late != null && (a.lateTo ?? coreLate()) != null ? Math.max(0, (a.lateTo ?? coreLate()) - a.late) : '-';
  const fills = a.fills != null && (a.fillsTo ?? fillIns()) != null ? Math.max(0, (a.fillsTo ?? fillIns()) - a.fills) : '-';
  const events = a.events.filter((e) => /^audio /.test(e));
  const media = a.events.filter((e) => !/^audio /.test(e));
  return [
    `while hidden (every time): ${awayAll.spells} times, ${(awayAll.ms / 1000).toFixed(1)} s, late notes ${awayAll.late}`,
    'while hidden (the last time):',
    `hiddenFor: ${((to - a.from) / 1000).toFixed(1)} s`,
    `hiddenLateNotes: ${late}`,
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
// more than 50 ms at a go), the notes that reached the core late, those the
// core left out or dropped, and the audio device's fill-ins (a starved audio thread). Hidden
// time is the "while hidden" report's, not any view's.
const views = { since: performance.now(), stats: {}, open: null, longTasks: false };

function viewStats(view) {
  if (!views.stats[view]) {
    views.stats[view] = {
      ms: 0, longTasks: 0, longestTask: 0, lateNotes: 0, fillIns: null,
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
  const o = views.open;
  if (o && (hiding || !document.hidden)) {
    const v = viewStats(o.view);
    v.ms += now - o.at;
    if (late != null && o.late != null) v.lateNotes += Math.max(0, late - o.late);
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
  views.open = document.hidden ? null : { view: state.view, at: now, late, fills, counts };
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
    lines.push(`  ${name}: ${clock(v.ms / 1000)} on screen, ${long}, `
      + `late notes ${v.lateNotes}${why}, dropped ${v.dropped}${dropped}, fill-ins ${v.fillIns ?? '-'}`);
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
  state.media.setMetadata(state.spec.name, `${state.spec.bpm} bpm`, state.spec);
}

// Track length (session 30c, queue item 111): its view, opened from the Vibe
// panel's pace line (`from` 'vibe') or from the export when nothing would
// end it ('export').
// From an album's export ('album', item 121) it opens on the first track
// that plays on, and its time goes to the album's export, not to a loop.
function openTrackView(from, spec = state.spec) {
  if (!spec) return;
  tracklen.openTrack(spec, from);
  setView('track');
}

// The Track length view's done: the loop playing plays this long, with these
// fades, as the Length slider's setting did (the loop's own: a saved loop
// keeps it, an album's tracks and the history's keep theirs, a new loop
// plays on). Counted from what is heard now, on the core, with the loop that
// follows armed for its end.
function setTrackLength(track) {
  if (!state.spec) return;
  tracklen.setTrack(state.spec, track);
  if (state.engine) state.engine.trackChanged(nextForEnd());
  state.passCount = 0;
  renderNow();
  refreshCard();
  updateSaveButton();
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
  // The track length set (session 30c), as a time.
  const t = tracklen.trackOfSpec(state.spec);
  ui.renderReadout(state.spec, state.pattern, {
    album: pl && pl.ids.length ? `${pl.title} ${pl.index + 1}/${pl.ids.length}` : '',
    length: t.secs != null ? `track ${tracklen.clock(t.secs)}` : '',
  });
}

// Anything saved, deleted or renamed: the lists, and whether the loop on
// screen counts as saved.
function savedChanged() {
  library.refresh();
  // The albums' art, ready for the Library before it is opened.
  library.warmArt();
  updateSaveButton();
  // An album deleted while it plays leaves Previous to the history.
  updateHistoryButtons();
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
    if (!entry) { ui.toast(...CANT_SAVE); return; }
    state.currentId = entry.id;
  }
  if (!store.addToAlbum(albumId, state.currentId)) { ui.toast("Can't change album"); return; }
  const album = store.loadAlbums().find((a) => a.id === albumId);
  if (album) ui.toast('Saved', `Saved to ${album.title}`);
  else ui.toast("Can't change album");
  savedChanged();
  refreshCard();
}

function unsaveFrom(albumId) {
  if (!isSaved()) return;
  const name = state.spec.name;
  const res = store.removeFromAlbum(albumId, state.currentId);
  if (!res.ok) { ui.toast("Can't change album"); return; }
  if (res.deleted) {
    state.currentId = null;
    ui.toast('Deleted', `Deleted ${name}`);
  } else {
    ui.toast('Removed', `Removed ${name} from that album`);
  }
  savedChanged();
  refreshCard();
}

function updateSaved() {
  if (!isSaved()) return;
  if (!store.replaceSpec(state.currentId, state.spec)) {
    ui.toast(...CANT_SAVE);
    return;
  }
  ui.toast('Updated', `Updated ${state.spec.name}`);
  savedChanged();
}

// The changed loop becomes a loop of its own, in Favorite Loops; the one it
// came from stays as it was.
function saveAsNew() {
  if (!state.spec) return;
  const entry = store.save(state.spec);
  if (!entry) { ui.toast(...CANT_SAVE); return; }
  state.currentId = entry.id;
  store.addToAlbum(store.FAVORITES, entry.id);
  ui.toast('Saved', `Saved ${state.spec.name} as a new loop in Favorite Loops`);
  savedChanged();
}

// Save on a keyboard: to Favorite Loops, or the changes to a saved loop.
function saveQuick() {
  if (!state.spec) return;
  if (!isSaved()) saveIn(store.FAVORITES);
  else if (isChanged()) updateSaved();
  else ui.toast('Already saved');
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

// The focus follows the view (WCAG 2.4.3): a child view opens on its way
// back out, and closed, the focus is on the key that opened it.
const FOCUS_IN = {
  vibe: '#picksClose', vibes: '#vibesClose', length: '#lenCancel', track: '#trkCancel',
  tempo: '#tmpCancel', key: '#keyCancel', style: '#styCancel', voices: '#voxCancel',
};
const FOCUS_BACK = {
  'vibe>now': '#vibesBtn',
  'vibes>vibe': '#picksList .pk-line-mood .pk-val',
  'length>vibe': '#picksList .pk-go-bars',
  'tempo>vibe': '#picksList .pk-go-tempo',
  'key>vibe': '#picksList .pk-go-key',
  'style>vibe': '#picksList .pk-go-style',
  'voices>vibe': '#picksList .pk-go-voices',
  'track>vibe': '#picksList .pk-track',
  'track>export': '#viewExport .page-title',
};

function focusView(from, view, had) {
  // Only for a focus that was in the view gone: never taken from the rest.
  if (!had || from === view) return;
  const sel = FOCUS_BACK[`${from}>${view}`] || FOCUS_IN[view];
  const n = sel && document.querySelector(sel);
  if (n && n.checkVisibility && !n.checkVisibility()) return;
  if (n) n.focus({ preventScroll: true });
}

function setView(view) {
  viewFlush();
  const from = state.view;
  const was = document.querySelector('.view:not([hidden])');
  const had = !!(was && document.activeElement && was.contains(document.activeElement));
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
  if (view === 'layers' && state.spec) drawLayers(state.bar > 0 ? state.bar : 0);
  if (view === 'library') library.render();
  if (view === 'save') library.renderSave();
  if (view === 'now') renderNow();
  if (view === 'vibes') renderVibes();
  if (view === 'vibe') picks.renderPanel();
  if (view === 'length') picks.renderLength();
  if (view === 'track') tracklen.renderTrack();
  if (pickers.PICKERS.includes(view)) pickers.renderPicker(view);
  refreshCard();
  if (view === 'sounds') saySliders();
  focusView(from, view, had);
  sayKeys();
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
  const count = ui.el('vibesCount');
  count.textContent = n ? `${n} picked` : 'any';
  const more = document.createElement('span');
  more.className = 'vh';
  more.textContent = n ? ', for new loops' : ': new loops as usual';
  count.append(more);
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
// the track length's time ran out with nothing of the page's armed. The
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
    // A new track made (Next, or the core's own at a track's end): the
    // album's way back goes until another album is left (session 30h).
    state.lastAlbum = null;
    if (state.spec) {
      const last = state.history[state.history.length - 1];
      if (!last || JSON.stringify(last) !== JSON.stringify(state.spec)) pushHistory(state.spec);
    }
    pushHistory(spec);
    loadSpec(spec, { pushToHistory: false });
    sayLoop(spec.name, `New loop: ${spec.name}`);
    updateHistoryButtons();
  }, 0);
}

// An unlit any key puts its line back to chance. Line two holds tempo and
// length, so it clears both.
function clearPickLine(line) {
  if (line === 'mood') setVibes([]);
  else if (line === 'pace') setPicks({ tempo: null, length: null, journey: null });
  else if (line === 'key') setPicks({ key: null, scale: null });
  else if (line === 'style') setPicks({ style: null });
  else if (line === 'voices') setPicks({ voices: null, layers: null });
}

// Clear all (session 30j): every pick back to chance, the moods with them,
// for loops still to come. The Track length is the loop playing's, not a
// pick: it stays.
function clearAllPicks() {
  state.vibes = [];
  state.picks = picks.cleanPicks(null);
  store.setPrefs({ vibes: state.vibes, picks: state.picks });
  sendPicks();
  armNext();
  updateVibesKey();
  if (state.view === 'now') renderNow();
  if (state.view === 'vibe') picks.renderPanel();
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
    ui.el('nowOf').innerHTML = `<span aria-hidden="true">${pl.index + 1}/${n}</span><span class="vh">${pl.index + 1} of ${n}</span>`;
  }

  // In an album the album is what comes next, so Now Playing says which
  // track; outside one it shows what new loops lean toward instead.
  const inAlbum = !!(pl && pl.ids.length);
  const next = ui.el('nowNext');
  next.hidden = !inAlbum;
  ui.el('nowPicks').hidden = inAlbum;
  if (inAlbum) {
    // An arrow for the word (Mikey, 2026-11-22), the word for a screen reader.
    next.innerHTML = `${ui.icon('arrowRight')}<span class="vh">Next: </span>`;
    const b = document.createElement('b');
    b.textContent = nextName();
    next.appendChild(b);
  } else {
    picks.renderNowPicks();
  }

  const action = ui.el('nowAction');
  if (pl && pl.ids.length) {
    action.hidden = false;
    action.textContent = 'leave';
    action.title = '';
    action.setAttribute('aria-label', 'leave album');
    action.onclick = leaveAlbum;
  } else if (state.lastAlbum) {
    const album = store.loadAlbums().find((a) => a.id === state.lastAlbum.albumId);
    action.hidden = !album;
    if (album) {
      action.innerHTML = ui.icon('arrowLeft');
      action.append(album.title);
      action.title = 'Back';
      action.setAttribute('aria-label', `Back to ${album.title}`);
      action.onclick = () => playAlbum(album.id, state.lastAlbum.index);
    }
  } else {
    action.hidden = true;
  }
  fitNow();
}

function nextName() {
  const pl = state.playlist;
  if (pl && pl.ids.length) {
    const id = pl.ids[(pl.index + 1) % pl.ids.length];
    const entry = store.loadAll().find((e) => e.id === id);
    return entry ? entry.spec.name : 'next track';
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
    sayRail(null, 0);
    piece.updatePiece();
    return;
  }
  const pos = state.nowSteps || 0;
  let text;
  let frac;
  // A set track length counts down to where it hands over (queue item 88b:
  // whole passes, or a phrase line inside one); a loop's own length, however
  // long, never ends it. Today's clock: the long-piece display waits for a
  // Painter session.
  const span = railSpan();
  const drag = state.railDrag;
  // Pressed (session 30h): the rail and the clock show where a let-go goes.
  const at = drag && span ? railStep(span, drag.f) : pos;
  if (span.track) {
    text = `\u2212${clock(Math.max(0, span.to - at) * e.stepDur)}`;
    frac = Math.min(1, Math.max(0, (at - span.from) / (span.to - span.from)));
  } else {
    text = clock(at * e.stepDur);
    frac = Math.min(1, Math.max(0, (at - span.from) / (span.to - span.from)));
  }
  setNowText(text);
  ui.el('nowFill').style.transform = `scaleX(${frac.toFixed(4)})`;
  sayRail(span, at);
  // A long piece's line and where you are (queue item 106b).
  piece.updatePiece();
}

// ------------------------------------------------ the rail (session 30h)

// What a short loop's rail spans (queue item 116), in steps from the load:
// the track, from where its count began to where it ends, where a Track
// length is set; else the round playing, the loop's pass (a short piece's
// whole length, where its bars picker set one). Null with nothing loaded.
function railSpan() {
  const e = state.engine;
  if (!state.spec || !e || !e.live) return null;
  const end = e.endStep();
  if (end != null) return { from: e.countFrom, to: Math.max(e.countFrom + 1, end), track: true };
  const round = e.roundSteps();
  const total = Number.isFinite(round) && round > 0 ? round : e.base.totalSteps;
  const pos = Math.max(0, state.nowSteps || 0);
  const from = Math.floor(pos / total) * total;
  return { from, to: from + total, track: false };
}

// The step at `f` (0 to 1) of the rail: its first at 0, its last at 1.
function railStep(span, f) {
  return Math.max(span.from, Math.min(span.to - 1, Math.floor(span.from + f * (span.to - span.from))));
}

// Go to step `step` of the loop (`Jump::Step`): playing, heard there a few
// blocks on; paused, the playhead goes there now and Play starts from it.
// The track's clock moves with it, its fades too (the core's).
function railSeek(step) {
  const e = state.engine;
  if (!e || !e.jumpTo(JUMP.step, step)) return false;
  state.railAsked = { step, at: performance.now() };
  if (!e.playing) {
    state.nowSteps = step;
    updateNowClock();
  }
  return true;
}

// The rail as a slider says it (a screen reader's; nothing on screen): the
// time into the track of its length, or the bar of the round. Only rewritten
// when it moves.
function sayRail(span, at) {
  const rail = ui.el('nowRail');
  let said;
  if (!span) {
    said = '';
  } else if (span.track) {
    const sd = state.engine.stepDur;
    const total = Math.round((span.to - span.from) * sd);
    const into = Math.min(total, Math.round(Math.max(0, at - span.from) * sd));
    said = `0|${total}|${into}|${clock(into)} of ${clock(total)}`;
  } else {
    const spb = (state.spec && state.spec.stepsPerBar) || 16;
    const bars = Math.max(1, Math.round((span.to - span.from) / spb));
    const bar = Math.min(bars, Math.floor(Math.max(0, at - span.from) / spb) + 1);
    said = `1|${bars}|${bar}|bar ${bar} of ${bars}`;
  }
  if (rail.dataset.said === said) return;
  rail.dataset.said = said;
  if (!said) {
    rail.setAttribute('aria-disabled', 'true');
    rail.setAttribute('aria-valuetext', 'nothing playing');
    return;
  }
  const [min, max, now, text] = said.split('|');
  rail.removeAttribute('aria-disabled');
  rail.setAttribute('aria-valuemin', min);
  rail.setAttribute('aria-valuemax', max);
  rail.setAttribute('aria-valuenow', now);
  rail.setAttribute('aria-valuetext', text);
}

// The rail's pointer, finger and keys (session 30h, as the long piece's
// line): press or drag shows where, let go goes there; the arrows a bar each
// way, Page Up and Down a phrase (four bars), Home its start, End its last
// bar.
function wireRail() {
  const rail = ui.el('nowRail');
  const frac = (ev) => {
    const r = rail.getBoundingClientRect();
    return r.width > 0 ? Math.max(0, Math.min(1, (ev.clientX - r.left) / r.width)) : 0;
  };
  rail.addEventListener('pointerdown', (ev) => {
    if (ev.button > 0 || !railSpan()) return;
    try { rail.setPointerCapture(ev.pointerId); } catch { /* a pointer that is gone: the move events still come */ }
    state.railDrag = { id: ev.pointerId, f: frac(ev) };
    updateNowClock();
    ev.preventDefault();
  });
  rail.addEventListener('pointermove', (ev) => {
    if (!state.railDrag || ev.pointerId !== state.railDrag.id) return;
    state.railDrag.f = frac(ev);
    updateNowClock();
  });
  const finish = (ev, go) => {
    const drag = state.railDrag;
    if (!drag || ev.pointerId !== drag.id) return;
    state.railDrag = null;
    const span = railSpan();
    if (go && span) railSeek(railStep(span, drag.f));
    updateNowClock();
  };
  rail.addEventListener('pointerup', (ev) => finish(ev, true));
  rail.addEventListener('pointercancel', (ev) => finish(ev, false));
  rail.addEventListener('keydown', (ev) => {
    if (ev.altKey || ev.ctrlKey || ev.metaKey) return;
    const k = { ArrowRight: 1, ArrowUp: 1, ArrowLeft: -1, ArrowDown: -1, PageUp: 4, PageDown: -4, Home: 'home', End: 'end' }[ev.key];
    if (k != null && railKey(k)) ev.preventDefault();
  });
}

// The rail's keys: `by` bars on (or back), or 'home' or 'end'. False with
// nothing to move.
function railKey(by) {
  const span = railSpan();
  if (!span) return false;
  const spb = state.spec.stepsPerBar || 16;
  // Keys held or pressed quickly go on from the last one asked, not from
  // what is heard yet (a jump is heard a few blocks on).
  const asked = state.railAsked && performance.now() - state.railAsked.at < 600 ? state.railAsked.step : null;
  const from = asked != null ? asked : Math.floor(state.nowSteps || 0);
  const to = by === 'home' ? span.from : by === 'end' ? span.to - spb : from + by * spb;
  railSeek(Math.max(span.from, Math.min(span.to - 1, to)));
  return true;
}

// The playhead a step on or back from anywhere (Shift with an arrow,
// js/shortcuts.js): a long piece's section, as its line's keys do, else a
// bar of the rail.
function seekStep(dir) {
  if (!state.spec || !state.engine || !state.engine.live) return;
  if (piece.stepPiece(dir)) return;
  // Away from Now Playing the rail's place isn't drawn: read it first.
  const steps = state.engine.playing ? audibleSteps() : null;
  if (steps != null) state.nowSteps = steps;
  railKey(dir);
}

// A short screen: the piece's line and words take less room before Now
// Playing would run over its panel (nothing scrolls).
function fitNow() {
  const view = ui.el('viewNow');
  const mid = view.querySelector('.np-mid');
  view.classList.remove('fit-tight');
  if (mid.scrollHeight > mid.clientHeight + 1) view.classList.add('fit-tight');
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

  // The box takes only what a share code can carry (session 23): a
  // character the code refuses (an invisible or a control one, a direction
  // override, a pile of marks) or a name past its room is not taken, and the
  // box keeps what it had. Spaces are tidied at the end, and an input
  // method's word is read once it is whole.
  let kept = input.value;
  const onInput = (e) => {
    if (e.isComposing) return;
    const tidy = input.value.normalize('NFC').replace(/\s+/g, ' ').trim();
    if (!tidy || share.fitName(tidy, store.NAME_MAX) === tidy) kept = input.value;
    else input.value = kept;
  };
  input.addEventListener('input', onInput);

  let done = false;
  const finish = (commit) => {
    if (done) return;
    done = true;
    input.removeEventListener('keydown', onKey);
    input.removeEventListener('blur', onBlur);
    input.removeEventListener('input', onInput);
    input.hidden = true;
    h.hidden = false;
    const name = share.fitName(input.value, store.NAME_MAX);
    if (!commit || !name || name === h.textContent) return;
    if (albumId) {
      if (!store.renameAlbum(albumId, name)) { ui.toast("Can't rename"); return; }
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
    if (e.key !== 'Enter' && e.key !== 'Escape') return;
    e.preventDefault();
    e.stopPropagation();
    finish(e.key === 'Enter');
    h.focus({ preventScroll: true });
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
  updateHistoryButtons();
  ui.announce('Left the album. Next makes new loops');
}

function playAlbum(albumId, index = 0, play = true) {
  const album = store.loadAlbums().find((a) => a.id === albumId);
  if (!album) return;
  const saved = new Set(store.loadAll().map((e) => e.id));
  const present = album.ids.filter((id) => saved.has(id));
  if (!present.length) { ui.toast('Empty', 'That album is empty'); return; }
  state.playlist = { albumId: album.id, title: album.title, ids: present, index: index - 1 };
  advancePlaylist(1, play);
}

async function shareAlbum(albumId, asLink = false) {
  const album = store.loadAlbums().find((a) => a.id === albumId);
  if (!album) return;
  const byId = new Map(store.loadAll().map((e) => [e.id, e]));
  const specs = album.ids.map((id) => byId.get(id)).filter(Boolean).map((e) => e.spec);
  if (!specs.length) { ui.toast('Empty', 'That album is empty'); return; }
  try {
    const code = share.encodeAlbum(album.title, specs);
    const label = album.title;
    if (asLink) await shareLink(code, label);
    else await offerCode(code, label);
  } catch (err) {
    console.warn('Could not build an album code', err);
    ui.toast("Can't make code");
  }
}

// Put a code where it can be taken. The clipboard is the quick path; the
// box below is the one that still works when the clipboard is refused.
async function offerCode(code, label, what = 'Code') {
  const out = ui.el('shareOut');
  try {
    await navigator.clipboard.writeText(code);
    out.hidden = true;
    ui.toast('Copied', `${what} copied`);
  } catch {
    // The box, only now (session 30e), holds the code alone, so all of it
    // selected is the code. It lives under Save > Share; take them to it.
    out.textContent = code;
    out.setAttribute('aria-label', `${what} to copy: ${label}`);
    out.hidden = false;
    setView('save');
    library.showShare();
    out.scrollIntoView({ block: 'nearest' });
    ui.toast('Copy from box', 'The clipboard is blocked: copy it from the box');
  }
}

// The address a link is made on: the page's own where it is served over
// http(s) (so a link follows the site, through the rename), else the app's.
function linkBase() {
  if (nativeBridge() || !location.protocol.startsWith('http')) return APP_URL;
  return (location.origin + location.pathname).replace(/index\.html$/, '');
}

// Share a code as a link: the system's share sheet where there is one, else
// copied (and the box, where the clipboard is refused).
async function shareLink(code, label) {
  const url = share.linkOf(linkBase(), code);
  if (navigator.share && !nativeBridge()) {
    try {
      await navigator.share({ title: label, url });
      return;
    } catch (err) {
      if (err && err.name === 'AbortError') return;
    }
  }
  await offerCode(url, label, 'Link');
}

// An album of one seed, as many tracks as the slider says, each asked the
// picks now set: its short code, as a link or as the code.
async function seedAlbum(asLink) {
  const count = Number(ui.el('seedCount').value) || 12;
  try {
    const code = share.encodeSeedAlbum('', count, picks.requestFor(state));
    const label = `${count} tracks`;
    if (asLink) await shareLink(code, label);
    else await offerCode(code, label);
  } catch (err) {
    console.warn('Could not make an album', err);
    ui.toast(err instanceof RequestError ? err.message : "Can't make album");
  }
}

// The album of one seed saved to the Library (item 121; Mikey, 2026-11-23:
// "Add 'save'"): named as a new album is, its tracks drawn from a fresh seed
// with the picks now set, and kept as its code plays them (the code made,
// then read back as a pasted one is), so the album played from the Library
// is the album its code would bring.
function seedAlbumSave() {
  const count = Number(ui.el('seedCount').value) || 12;
  const name = library.askAlbumName();
  if (!name) return;
  try {
    const { title, specs } = share.decodeAlbum(share.encodeSeedAlbum(name, count, picks.requestFor(state)));
    const album = keepAlbum(title, specs);
    if (album) ui.toast('Saved', `Saved ${title} to the Library, ${specs.length} loops`);
    else if (album === null) ui.toast("Can't save", "Can't save the album");
  } catch (err) {
    console.warn('Could not save the album', err);
    ui.toast(err instanceof RequestError ? err.message : "Can't make album");
  }
}

function restoredToast({ added, rejected }) {
  ui.toast(`Restored ${added}${rejected ? `, skipped ${rejected}` : ''}`,
    `Restored ${added} loop${added === 1 ? '' : 's'}${rejected ? `, skipped ${rejected} unreadable` : ''}`);
}

// A slider's value as it is said: a tempo, a share, a count.
function saySliders() {
  const of = (id) => Number(ui.el(id).value);
  const say = (id, text) => ui.el(id).setAttribute('aria-valuetext', text);
  say('bpm', `${of('bpm')} bpm`);
  for (const id of ['warmth', 'space', 'wobble', 'volume']) say(id, `${Math.round(of(id) * 100)}%`);
  const wander = of('driftAmount');
  say('driftAmount', wander > 0 ? `${wander.toFixed(2).replace(/\.?0+$/, '')} of 2.4` : 'off');
  say('seedCount', `${of('seedCount')} tracks`);
}

function syncToneInputs(spec) {
  ui.el('bpm').value = spec.bpm;
  ui.el('warmth').value = spec.tone.warmth;
  ui.el('space').value = spec.tone.space;
  ui.el('wobble').value = spec.tone.wobble;
  ui.el('bpmVal').textContent = spec.bpm;
  saySliders();
}

function openSaved(entry) {
  // One unreadable save should not take the app down with it.
  try {
    state.playlist = null;
    resetHistory(cloneSpec(entry.spec));
    loadSpec(cloneSpec(entry.spec), { id: entry.id, pushToHistory: false });
  } catch (err) {
    console.warn('Could not open saved loop', err);
    ui.toast("Can't open", 'That saved loop could not be opened');
    return;
  }
  if (!state.engine.playing) togglePlay();
  sayLoop(entry.spec.name, `Loaded ${entry.spec.name}`);
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
  // A step's sounds came in, or a pass's grid took over: the layers drawn
  // again (only the dots that moved change).
  if (state.view === 'layers' && state.bar >= 0 && state.engine.rowsVersion !== state.rowsDrawn) drawLayers(state.bar);
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
    if (state.view === 'layers') drawLayers(bar);
  }
  ui.moveCursor(lights, cells, step % spb);
}

// The layers' bar `bar` (session 30b): while playing, as heard (each step
// the core has decided as it sounded, the rest of the bar as the pass's plan
// will have it heard; js/heard.js); else the loop as composed.
function drawLayers(bar) {
  if (!state.spec) return;
  const engine = state.engine;
  const rows = engine && engine.playing ? engine.barRows(bar) : null;
  const mutes = state.spec.mutes || {};
  if (rows) ui.renderRows(cells, rows, mutes);
  else {
    const pattern = (engine && engine.live) || state.pattern;
    if (pattern) ui.renderGrids(cells, pattern, bar, mutes);
  }
  state.rowsDrawn = engine ? engine.rowsVersion : 0;
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

// One step through the album: 1 is Next, -1 is Previous (round from the last
// track to the first, and from the first to the last).
function advancePlaylist(step, play = true) {
  const pl = state.playlist;
  if (!pl || !pl.ids.length) return;
  pl.index = (pl.index + step + pl.ids.length) % pl.ids.length;
  const saved = new Map(store.loadAll().map((e) => [e.id, e]));
  let entry = saved.get(pl.ids[pl.index]);
  while (!entry) {
    // A loop was deleted out from under the album: drop it and carry on the
    // way the step was going (back, the track before it).
    pl.ids.splice(pl.index, 1);
    if (!pl.ids.length) {
      state.playlist = null;
      updateHistoryButtons();
      return;
    }
    pl.index = step < 0 ? (pl.index - 1 + pl.ids.length) % pl.ids.length : pl.index % pl.ids.length;
    entry = saved.get(pl.ids[pl.index]);
  }
  loadSpec(cloneSpec(entry.spec), { id: entry.id, pushToHistory: false });
  if (play && !state.engine.playing) togglePlay();
  ui.announce(`${pl.title}, ${pl.index + 1} of ${pl.ids.length}: ${entry.spec.name}`);
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
  sayKeys();
}

function newLoop() {
  // Making something new is leaving the album, and its way back goes
  // (session 30h).
  state.playlist = null;
  state.lastAlbum = null;
  if (state.view === 'now') renderNow();
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
  revise(rerollLayer(state.spec, layer));
  if (state.view === 'layers') ui.announce(`Re-rolled ${layer}`);
  else ui.toast(`Rolled ${layer}`, `Re-rolled ${layer}`);
}

// Every layer's die at once (the keyboard's R, js/shortcuts.js).
function rollAll() {
  if (!state.spec) return newLoop();
  revise(LAYERS.reduce((spec, layer) => rerollLayer(spec, layer), state.spec));
  ui.toast('Rolled all', 'Re-rolled every layer');
}

// A loop's layers made again in place.
function revise(next) {
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
  drawLayers(Math.max(0, state.bar));
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
  let code = '';
  try { code = share.encodeSong(spec === state.spec ? sharedSpec() : spec); } catch { code = ''; }
  const blob = patternToMidi(render(spec), { repeats: 4, code });
  ui.download(blob, `${spec.name}-${spec.bpm}bpm.mid`);
  // In the Android app its picker comes next, and says Saved itself.
  if (!appBridge()) ui.toast('Exported', 'MIDI exported');
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
  // A link pasted is its code.
  text = share.codeOfLink(text);
  if (!share.codeKind(text)) {
    text = share.codeOfLink(prompt('Paste code') || '');
  }
  openCode(text, { play: true });
}

// A link opened (the page's address has "#c=" and a code): the loop or the
// album ready to play, never started by itself (a browser does not let a
// page make sound before a tap). The address is cleared of it, so a reload
// does not bring the album in twice.
function openLink() {
  if (!location.hash.startsWith('#c=')) return;
  const code = share.codeOfLink(location.hash);
  try { history.replaceState(null, '', location.pathname + location.search); } catch { /* the code is read already */ }
  openCode(code, { play: false, link: true });
}

// An album's tracks into the Library as an album of its own (a code pasted
// or opened by its link, a seed album saved): the album, null if its list
// could not be written, or false (said) if the loops could not be saved.
function keepAlbum(title, specs) {
  const ids = store.addSpecs(specs);
  if (!ids) {
    ui.toast("Can't save", "Can't save those loops");
    return false;
  }
  const album = store.createAlbum(title);
  if (album) store.setAlbumIds(album.id, ids);
  savedChanged();
  return album;
}

// Take a song or album code: from a paste (`play`: the loop starts) or a
// link (ready, not playing).
function openCode(text, { play, link = false }) {
  const kind = share.codeKind(text);
  // A version 1 code (session 23: the app speaks version 2 only), or one
  // from a newer build, or one that needs what this device lacks: one plain
  // line each, not an error.
  if (kind === 'old') { ui.toast(...OLD_CODE); return; }
  if (kind === 'newer') { ui.toast(...NEWER_CODE); return; }
  if (kind === 'instrument') { ui.toast(...INSTRUMENT_CODE); return; }
  if (!kind) {
    if (link) ui.toast(...BAD_LINK);
    else if (text.trim()) ui.toast(...NOT_A_CODE);
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
        ui.toast('Already saved');
      } else {
        const ids = store.addSpecs([spec]);
        if (ids) store.addToAlbum(store.IMPORTED, ids[0]);
        loadSpec(spec, { id: ids ? ids[0] : null });
        if (ids) ui.toast('Imported', `Added ${spec.name} to Imported Loops`);
        else ui.toast('Opened, not saved');
      }
      if (play && !state.engine.playing) togglePlay();
    } else {
      const { title, specs } = share.decodeAlbum(text);
      const album = keepAlbum(title, specs);
      if (album === false) return;
      // From a link the album is open at its first track, ready to play. (Its
      // place is said as it opens: the import is said after, so it is heard.)
      if (link && album) playAlbum(album.id, 0, false);
      ui.toast('Imported', `Added ${title}, ${specs.length} loops`);
    }
  } catch (err) {
    console.warn('Could not open the code', err);
    if (err instanceof share.CodeError && err.message === 'needs a newer version') ui.toast(...NEWER_CODE);
    else ui.toast(...(link ? BAD_LINK : NOT_A_CODE));
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
  // What shows past the app as a browser's bar slides away.
  if (th) document.documentElement.style.setProperty('--page-back', th[3]);
  // The Android app's bars behind the page take the theme's color too.
  const link = appLink();
  if (link && th) link.theme(th[3]);
  if (save) store.setPrefs({ theme: cls });
  ui.el('themeName').textContent = th ? th[2] : '';
  for (const sw of document.querySelectorAll('#themes .sw')) {
    const on = sw.dataset.theme === cls;
    sw.setAttribute('aria-checked', String(on));
    sw.tabIndex = on ? 0 : -1;
  }
  requestAnimationFrame(() => ui.fitCard());
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

// The text's two steps, where Layers stacks and rows wrap their keys: the
// width there is, in ems of the text as it is now. Measured here, not by a
// container query in em: Chromium 141 leaves one at the size the page
// loaded with when the text size changes under it (the Android app's font
// size, a browser's setting; measured by tools/a11y.mjs).
function stepText() {
  const app = ui.el('app');
  const box = matchMedia('(min-width: 480px)').matches ? app.querySelector('.zone-dynamic') : app;
  const style = getComputedStyle(box);
  const room = box.clientWidth - parseFloat(style.paddingLeft) - parseFloat(style.paddingRight);
  if (!(room > 0)) return;
  const ems = room / parseFloat(getComputedStyle(app).fontSize);
  app.classList.toggle('text-large', ems <= 17.5);
  app.classList.toggle('text-larger', ems <= 15.5);
}

// A text size changed with no resize (a browser's setting): a probe a rem
// wide says so.
function watchTextSize(changed) {
  if (typeof ResizeObserver === 'undefined') return;
  const probe = document.createElement('i');
  probe.setAttribute('aria-hidden', 'true');
  probe.style.cssText = 'position:absolute;left:0;top:0;width:1rem;height:1rem;visibility:hidden;pointer-events:none';
  document.body.appendChild(probe);
  let was = probe.getBoundingClientRect().width;
  new ResizeObserver(() => {
    const now = probe.getBoundingClientRect().width;
    if (now === was) return;
    was = now;
    changed();
  }).observe(probe);
}

// ------------------------------------------------------------ the keyboard

// The keyboard as music players have it (session 30n, item 122): the map,
// what each key follows and what a key never does are js/shortcuts.js's.

// The volume a step (Up and Down, YouTube's 5%), and mute (M): the Sound
// view's slider moves with it, kept as it keeps it.
const VOLUME_STEP = 0.05;
function setVolume(v) {
  const r = ui.el('volume');
  r.value = String(Math.round(Math.max(0, Math.min(1, v)) * 100) / 100);
  r.dispatchEvent(new Event('input', { bubbles: true }));
  return Math.round(parseFloat(r.value) * 100);
}
function nudgeVolume(by) {
  const pct = setVolume(parseFloat(ui.el('volume').value) + by);
  ui.toast(pct ? `Volume ${pct}%` : 'Muted');
}
function toggleVolumeMute() {
  const now = parseFloat(ui.el('volume').value);
  if (now > 0) {
    state.unmuteTo = now;
    setVolume(0);
    ui.toast('Muted');
  } else {
    ui.toast(`Volume ${setVolume(state.unmuteTo || 0.85)}%`);
  }
}

// A layer on or off (Shift with its number): said where its row isn't seen.
function toggleLayer(i) {
  const layer = LAYERS[i];
  toggleMute(layer);
  if (!state.spec || state.view === 'layers') return;
  const label = LAYER_LABELS[layer];
  const said = `${label[0].toUpperCase()}${label.slice(1)} ${state.spec.mutes[layer] ? 'off' : 'on'}`;
  ui.toast(said);
}

// Where each view's letter goes: whether it is up, and how it opens (each
// as its key on screen opens it, so the focus rules are theirs).
function viewUp(name) {
  if (name === 'share') return state.view === 'save' && !ui.el('saveShare').hidden;
  if (name === 'advanced') return state.view === 'library' && !ui.el('libAdvanced').hidden;
  return state.view === name;
}
function openView(name) {
  if (name === 'length') picks.openLength();
  if (pickers.PICKERS.includes(name)) pickers.openPicker(name);
  if (name === 'track') openTrackView('vibe');
  else if (name === 'share') { setView('save'); library.showShare(); }
  else if (name === 'export') ui.el('goExport').click();
  else if (name === 'advanced') {
    if (state.view !== 'library') setView('library');
    library.setTab('more');
    ui.el('goAdvanced').click();
  } else setView(name);
}
function letterView(name) {
  // Its letter again goes back, as a lit key's press does; Now Playing,
  // Sound and Layers are where you stay.
  if (viewUp(name)) {
    if (!BASE_VIEWS.includes(name)) closeView();
    return;
  }
  openView(name);
}

// Esc: the view's own way out (close, cancel, back), or, for the Library
// and Save, back to where you were, the focus on their key. False where
// there is nothing to close.
const WAY_OUT = '.back, #picksClose, #vibesClose, #lenCancel, #trkCancel, .pv-cancel';
function closeView() {
  const view = document.querySelector('.view:not([hidden])');
  if (!view) return false;
  const out = [...view.querySelectorAll(WAY_OUT)].find((b) => !b.disabled && b.checkVisibility());
  if (out) {
    out.click();
    return true;
  }
  const key = { library: 'libraryBtn', save: 'saveBtn' }[state.view];
  if (!key) return false;
  const had = view.contains(document.activeElement);
  setView(state.lastView);
  if (had) ui.el(key).focus({ preventScroll: true });
  return true;
}

// The list of keys (?), a popup that says them whole: those that work now.
function showKeys() {
  const pop = ui.el('keysPop');
  if (pop.open) return;
  const list = ui.el('keysList');
  list.textContent = '';
  for (const row of keyList(onMac())) {
    if (row.letter && !state.letterKeys) continue;
    const li = document.createElement('li');
    if (row.view) li.className = 'kp-view';
    const keys = document.createElement('kbd');
    keys.textContent = row.keys;
    keys.setAttribute('aria-hidden', 'true');
    const said = document.createElement('span');
    said.className = 'vh';
    said.textContent = `${row.said}: `;
    const what = document.createElement('span');
    what.className = 'kp-what';
    what.textContent = row.what;
    li.append(keys, said, what);
    list.appendChild(li);
  }
  ui.el('keysOff').hidden = state.letterKeys;
  keysFrom = document.activeElement;
  pop.showModal();
}
let keysFrom = null;

// The letter keys switch (Advanced; WCAG 2.1.4), the fourth setting.
function setLetterKeys(on, save = true) {
  state.letterKeys = on;
  setSwitch('letterKeys', on);
  if (save) store.setPrefs({ letterKeys: on });
  sayKeys();
}

// What each key on screen says it has (`aria-keyshortcuts`), and its
// tooltip, as players show them ("Next (→)"): the letter ones while the
// letter keys are on. [said, shown, a letter].
function sayKeys() {
  const on = state.letterKeys !== false;
  const mac = onMac();
  const lettered = ui.el('viewBtn').title === 'Sound' ? 'S' : 'L';
  const KEYS = {
    playBtn: [['Space', 'Space'], ['K']], prevBtn: [['ArrowLeft', '\u2190']], nextBtn: [['ArrowRight', '\u2192']],
    saveBtn: [[mac ? 'Meta+S' : 'Control+S', mac ? 'Cmd+S' : 'Ctrl+S']],
    libraryBtn: [null, ['I']], nowBtn: [null, ['N']], viewBtn: [null, [lettered]], vibesBtn: [null, ['V']],
  };
  for (const [id, [plain, letter]] of Object.entries(KEYS)) {
    const b = ui.el(id);
    const use = [plain, on ? letter : null].filter(Boolean);
    if (use.length) b.setAttribute('aria-keyshortcuts', use.map((k) => k[0]).join(' '));
    else b.removeAttribute('aria-keyshortcuts');
    if (!b.title) continue;
    const base = b.title.replace(/ \(.*\)$/, '');
    b.title = use.length ? `${base} (${use[0][1] || use[0][0]})` : base;
  }
  ui.sayLayerKeys(on);
}

function wireKeys() {
  const mac = onMac();
  const DOES = {
    play: togglePlay,
    next: goForward,
    previous: goBack,
    seekAhead: () => seekStep(1),
    seekBack: () => seekStep(-1),
    louder: () => nudgeVolume(VOLUME_STEP),
    quieter: () => nudgeVolume(-VOLUME_STEP),
    mute: toggleVolumeMute,
    rollAll,
    keys: showKeys,
    save: saveQuick,
  };
  LAYERS.forEach((layer, i) => {
    DOES[`roll${i}`] = () => reroll(layer);
    DOES[`layer${i}`] = () => toggleLayer(i);
  });
  for (const [, view] of VIEW_KEYS) DOES[`view:${view}`] = () => letterView(view);
  // After the control with the focus: what it did itself is never done
  // again, and what it keeps is its own (js/shortcuts.js).
  const pop = ui.el('keysPop');
  document.addEventListener('keydown', (e) => {
    // The popup's keys are its own (Esc closes it), wherever the focus is.
    if (e.defaultPrevented || pop.open) return;
    const act = shortcutOf(e, mac, state.letterKeys);
    if (!act || keeps(ownKeys(e.target, byKeys), e)) return;
    if (act === 'close') {
      if (!e.repeat && closeView()) e.preventDefault();
      return;
    }
    e.preventDefault();
    if (!e.repeat || REPEATS.has(act)) DOES[act]();
  });
  ui.el('keysClose').addEventListener('click', () => pop.close());
  // A tap outside the list closes it too.
  pop.addEventListener('click', (e) => { if (e.target === pop) pop.close(); });
  pop.addEventListener('close', () => {
    if (keysFrom && keysFrom.isConnected && keysFrom.focus) keysFrom.focus({ preventScroll: true });
    keysFrom = null;
  });
  ui.el('keysBtn').addEventListener('click', showKeys);
  const hint = ui.el('letterHint');
  hint.textContent = keyNames(hint.textContent, mac);
  ui.el('letterKeys').addEventListener('click', () => setLetterKeys(!state.letterKeys));
}

// ---------------------------------------------------------------- boot

// Whether the focus is where the keyboard put it (not a tap).
let byKeys = true;

function wire() {
  lights = ui.buildPlayhead(state.gridSteps);
  cells = ui.buildLayers({ onReroll: reroll, onMute: toggleMute }, state.gridSteps);

  ui.setIcon(ui.el('prevBtn'), 'prev');
  ui.setIcon(ui.el('nextBtn'), 'next');
  ui.setIcon(ui.el('libraryBtn'), 'library');
  ui.setIcon(ui.el('nowBtn'), 'now');
  ui.setIcon(ui.el('saveBtn'), 'saveShare');
  for (const n of document.querySelectorAll('[data-ico]')) n.innerHTML = ui.icon(n.dataset.ico);
  stepText();
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
    shareAlbumLink: (id) => shareAlbum(id, true),
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
    openTrack: () => openTrackView('vibe'),
    setLength: (length, journey) => setPicks({ length, journey }),
    clearLine: clearPickLine,
    clearAll: clearAllPicks,
    openPicker: (kind) => { pickers.openPicker(kind); setView(kind); },
  });
  // The tempo, key, style and voices pickers (session 30j, queue item 118).
  pickers.wirePickers({ state, setView, setPicks });
  ui.el('vibesClear').addEventListener('click', () => {
    setVibes([]);
    // It goes dim with nothing picked: the focus to close beside it.
    ui.el('vibesClose').focus({ preventScroll: true });
  });
  // A short loop's rail moves the playhead (session 30h).
  wireRail();
  // A long piece on Now Playing (queue item 106b): its line, where you are,
  // and the jump (session 17's command).
  piece.wirePiece({
    jump: (kind, at) => !!(state.engine && state.engine.jumpTo && state.engine.jumpTo(kind, at)),
    spec: () => state.spec,
    place: () => (state.engine && state.engine.place ? state.engine.place() : null),
    playing: () => !!(state.engine && state.engine.playing),
    barSeconds: () => (state.spec ? ui.barSeconds(state.spec) : 3),
    fit: fitNow,
  });

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

  // The details of the card: a tap shows the rest of them (session 30d).
  ui.el('metaMore').addEventListener('click', () => ui.moreDetails());
  const nameEl = ui.el('loopName');
  nameEl.addEventListener('click', startRename);
  nameEl.addEventListener('keydown', (e) => {
    if (e.key === 'Enter' || (e.key === ' ' && byKeys)) {
      e.preventDefault();
      e.stopPropagation();
      startRename();
    }
  });

  ui.el('playBtn').addEventListener('click', togglePlay);
  ui.el('exportMidi').addEventListener('click', () => exportMidi());
  // Share's two views (session 30e): Export, and an album from one seed.
  // The focus goes in on the title and comes back on the key.
  for (const [key, view, id, back] of [['goExport', 'export', 'viewExport', 'exportBack'], ['goSeedAlbum', 'seed', 'viewSeed', 'seedBack']]) {
    ui.el(key).addEventListener('click', () => {
      setView(view);
      ui.el(id).querySelector('.page-title').focus({ preventScroll: true });
    });
    ui.el(back).addEventListener('click', () => {
      setView('save');
      library.showShare();
      ui.el(key).focus({ preventScroll: true });
    });
  }
  // Exporting audio (queue item 104b): the Export view, the loop
  // playing as the core plays it, "lighter on processor" its speed.
  state.audioExport = new AudioExport({
    spec: () => state.spec,
    engine: () => state.engine,
    mono: () => state.mono,
    lite: () => state.lite,
    // Nothing would end the export: how long, first (session 30c).
    openTrack: (from = 'export', spec = state.spec) => openTrackView(from, spec),
    // An album to export whole (item 121): the one playing, else the one
    // open in the Library.
    album: () => {
      const id = (state.playlist && state.playlist.albumId) || library.openedAlbum();
      const album = id ? store.loadAlbums().find((a) => a.id === id) : null;
      return album ? { id: album.id, title: album.title, ids: album.ids } : null;
    },
    specs: (ids) => {
      const byId = new Map(store.loadAll().map((e) => [e.id, e]));
      return ids.map((i) => byId.get(i)).filter(Boolean).map((e) => e.spec);
    },
    // A file's tags: the loop's code, the app's one name.
    code: (spec) => {
      try { return share.encodeSong(spec === state.spec ? sharedSpec() : spec); } catch { return ''; }
    },
    cover: (spec, size) => coverFile(spec, size, 'image/jpeg'),
    appName: APP_NAME,
  });
  tracklen.wireTrack({
    done: (track, from) => {
      // An album's export (item 121): the time for its tracks that play on,
      // the loops themselves left as they are.
      if (from === 'album') {
        setView('export');
        if (track.secs != null && state.audioExport) state.audioExport.startAlbum(track);
        return;
      }
      setTrackLength(track);
      setView(from === 'export' ? 'export' : 'vibe');
      // From the export, it goes on once something ends it.
      if (from === 'export' && state.audioExport && state.spec && tracklen.ends(state.spec)) state.audioExport.start();
    },
    cancel: (from) => setView(from === 'export' || from === 'album' ? 'export' : 'vibe'),
  });
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
  ui.el('bpm').addEventListener('input', (e) => {
    if (!state.spec) return;
    state.spec.bpm = parseInt(e.target.value, 10);
    ui.el('bpmVal').textContent = state.spec.bpm;
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

  for (const id of ['bpm', 'warmth', 'space', 'wobble', 'volume', 'driftAmount', 'seedCount']) {
    ui.el(id).addEventListener('input', saySliders);
  }

  ui.el('liteMode').addEventListener('click', () => {
    state.lite = !state.lite;
    setLiteSwitch(state.lite);
    // The listener's choice: kept as theirs, and never changed by the app.
    store.setPrefs({ lite: state.lite });
    showLiteSource();
    if (state.output) switchQuality('now');
    // An export running takes the new speed.
    if (state.audioExport) state.audioExport.setPace();
  });

  // Mono or stereo, while a loop plays: the core fades from one to the
  // other, nothing is rebuilt.
  ui.el('monoMode').addEventListener('click', () => {
    state.mono = !state.mono;
    setSwitch('monoMode', state.mono);
    store.setPrefs({ mono: state.mono });
    if (state.output) state.output.setLayout(state.mono ? 1 : 2);
    if (state.audioExport) state.audioExport.setPace();
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
    // The app's own last move, kept after the listener's choice overrides
    // it (session 30g: "yours" alone hid why it moved). "reset counters"
    // clears the line, not the move: the rule still never moves twice.
    {
      const { liteAuto: auto, liteAutoReset: reset } = store.getPrefs();
      const since = auto && !(reset && (!auto.at || auto.at <= reset));
      lines.push(since ? `lite by itself: last ${auto.lite ? 'on' : 'off'} at ${auto.at || '?'} (${auto.why})` : `lite by itself: never${reset ? ' since reset counters' : ''}`);
    }
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
      // Too slow even on lite (queue item 100): how long ago the line was
      // shown, and what tripped it.
      lines.push(state.slow ? `slow: lite past the line, ${Math.round((performance.now() - state.slow.at) / 1000)}s (${state.slow.why})` : 'slow: no');
    }
    // The core's player, with the notes it took late, left out or dropped;
    // or why there is none.
    lines.push(`engine: ${state.output ? state.output.engineReport() : state.coreError ? `none (${state.coreError})` : 'rust (loading)'}`);
    // The Android app's native core (session 27): its stream, as the core's
    // side says it.
    if (state.ctx && state.ctx.native) lines.push(`stream: ${state.ctx.streamReport()}`);
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
    else lines.push('player: not built yet (press Play)');
    lines.push(...awayReport());
    lines.push(...pageLog());
    lines.push(...viewsReport());
    ui.el('diagOut').textContent = lines.join('\n');
  };

  ui.el('diagRefresh').addEventListener('click', renderDiag);
  ui.el('goAdvanced').addEventListener('click', renderDiag);
  ui.el('diagReset').addEventListener('click', () => {
    if (state.engine) state.engine.clearMetrics();
    // The Android app's native counts (session 30g): the stream's, too.
    if (state.ctx && state.ctx.native) state.ctx.resetCounts();
    store.setPrefs({ liteAutoReset: new Date().toISOString() });
    renderDiag();
    ui.toast('Reset', 'Counters cleared');
  });
  ui.el('diagCopy').addEventListener('click', async () => {
    renderDiag();
    try {
      await navigator.clipboard.writeText(ui.el('diagOut').textContent);
      // The views' report starts again from here.
      viewsReset();
      ui.toast('Copied', 'Diagnostics copied');
    } catch {
      ui.toast('Copy from box', 'Select the report and copy it');
    }
  });

  ui.el('copySong').addEventListener('click', async () => {
    if (!state.spec) return;
    try {
      await offerCode(share.encodeSong(sharedSpec()), state.spec.name);
    } catch (err) {
      console.warn('Could not build a code', err);
      ui.toast("Can't make code");
    }
  });

  ui.el('pasteCode').addEventListener('click', pasteCode);
  ui.el('shareSong').addEventListener('click', async () => {
    if (!state.spec) return;
    try {
      await shareLink(share.encodeSong(sharedSpec()), state.spec.name);
    } catch (err) {
      console.warn('Could not build a link', err);
      ui.toast("Can't make link");
    }
  });
  ui.el('seedCount').addEventListener('input', () => { ui.el('seedCountVal').textContent = ui.el('seedCount').value; });
  ui.el('seedAlbumLink').addEventListener('click', () => seedAlbum(true));
  ui.el('seedAlbumCode').addEventListener('click', () => seedAlbum(false));
  ui.el('seedAlbumSave').addEventListener('click', seedAlbumSave);

  ui.el('copyBackup').addEventListener('click', async () => {
    try {
      await navigator.clipboard.writeText(await store.exportAll().text());
      ui.toast('Copied', 'Backup copied. Paste it somewhere safe.');
    } catch {
      ui.toast('Clipboard blocked');
    }
  });

  ui.el('pasteBackup').addEventListener('click', async () => {
    let text = '';
    try {
      text = await navigator.clipboard.readText();
    } catch {
      text = prompt('Paste backup') || '';
    }
    if (!text.trim()) return;
    try {
      const res = store.importAll(text);
      savedChanged();
      if (res.failed) ui.toast("Can't save", "Can't save the restored loops");
      else if (res.added) restoredToast(res);
      else if (res.rejected) ui.toast(`Skipped ${res.rejected}`, `Skipped ${res.rejected} unreadable loops`);
      else ui.toast('Nothing new');
    } catch {
      ui.toast(...NOT_A_BACKUP);
    }
  });
  ui.el('importFile').addEventListener('change', async (e) => {
    const file = e.target.files[0];
    if (!file) return;
    try {
      const res = store.importAll(await file.text());
      savedChanged();
      if (res.failed) ui.toast("Can't save", "Can't save the restored loops");
      else if (res.added) restoredToast(res);
      else if (res.rejected) ui.toast(`Skipped ${res.rejected}`, `Skipped ${res.rejected} unreadable loops`);
      else ui.toast('Nothing new');
    } catch {
      ui.toast("Can't read file");
    }
    e.target.value = '';
  });

  document.addEventListener('pointerdown', () => { byKeys = false; }, true);
  // (An arrow a control keeps moves the focus there; one the app takes doesn't.)
  document.addEventListener('keydown', (e) => { if (e.key === 'Tab' || (e.key.startsWith('Arrow') && keeps(ownKeys(e.target, true), e))) byKeys = true; }, true);
  wireKeys();
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
  setLetterKeys(prefs.letterKeys !== false, false);
  // Drift was a switch plus an amount; the amount was never kept, so an old
  // "on" comes back at the default distance.
  const wander = prefs.drift === false ? 0 : (prefs.driftAmount ?? 1);
  ui.el('driftAmount').value = wander;
  ui.el('wanderRow').classList.toggle('off', !(wander > 0));
  if (prefs.volume != null) ui.el('volume').value = prefs.volume;
  saySliders();
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

  // A turned phone or a resized window: what is measured is measured again
  // (the name, the card's lines, the sliders' stops, the panel's fit).
  const refit = () => {
    stepText();
    ui.fitCard();
    if (state.view === 'length') picks.renderLength();
    else if (state.view === 'track') tracklen.renderTrack();
    else if (state.view === 'vibe') picks.fitPanel();
    else if (pickers.PICKERS.includes(state.view)) pickers.refitPicker(state.view);
    else if (state.view === 'now') fitNow();
  };
  window.addEventListener('resize', refit);
  watchTextSize(refit);
}

// The safe area as it really is, and the browser's bar a scroll puts away
// (js/viewport.js): before anything is measured.
watchViewport({ app: !!nativeBridge() });

if (noGenerator) {
  // No loop can be made or shown here (js/gen.js): the page is the line.
  state.coreError = noGenerator;
  ui.el('app').classList.add('unplayable');
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
    // A link to a loop or an album (an address with "#c=" and a code), from
    // first load or opened over this page.
    window.addEventListener('hashchange', openLink);
    openLink();
  }
}

if (nativeBridge()) {
  // The Android app (session 27): its files are inside it, so no worker.
  swState = 'none (the app holds its own files)';
} else if ('serviceWorker' in navigator && location.protocol.startsWith('http')) {
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