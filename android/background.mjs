// The Android app with the screen off (session 28, queue item 107d), on an
// emulator: the music plays on in the background, the notification and the
// lock screen's controls work, a call pauses it and gives it back, nothing
// runs when it is paused, and an export finishes with the screen off, into a
// file chosen through Android's own picker. It drives the app's page through
// the WebView's DevTools port (android/cdp.mjs) and the system through adb
// (`dumpsys`, `cmd media_session`, `input`, `uiautomator`). What CI's
// `background` job runs (android/emulator-background.sh boots the emulator).
//
//   node android/background.mjs [--minutes 10] [--no-export] [--summary FILE]
//
// The emulator's figures are the emulator's, said to be only that: they say
// the app's service holds what it must and the audio thread keeps its time
// with the screen off; what a battery costs is a phone's to say.

import fs from 'node:fs';
import { AppPage, appId, adb, until } from './cdp.mjs';

const argv = process.argv.slice(2);
const opt = (name, fallback) => {
  const i = argv.indexOf(name);
  return i >= 0 && argv[i + 1] ? argv[i + 1] : fallback;
};
const MINUTES = Number(opt('--minutes', '10'));
const EXPORT = !argv.includes('--no-export');
const SUMMARY = opt('--summary', '');
const pkg = appId();
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const log = (...a) => console.log(...a);
const sh = (...a) => adb('shell', ...a);
const rows = [];
const check = (name, ok, detail = '') => {
  rows.push({ name, ok, detail });
  log(`${ok ? 'ok  ' : 'FAIL'} ${name}${detail ? `: ${detail}` : ''}`);
  return ok;
};
const facts = [];
const fact = (name, value) => {
  facts.push({ name, value });
  log(`     ${name}: ${value}`);
};

// ------------------------------------------------------------- the system

const power = () => sh('dumpsys', 'power');
const asleep = () => /mWakefulness=Asleep/.test(power());
// Held now: the app's lock in dumpsys's list of the locks held (a lock let go is not in it).
const wakeLocks = () => {
  const o = power();
  const at = o.indexOf('Wake Locks: size=');
  if (at < 0) return o.split('\n').filter((l) => l.includes('Driftloom:playing'));
  const rest = o.slice(at).split('\n');
  const end = rest.findIndex((l, i) => i > 0 && !/^\s/.test(l));
  return rest.slice(1, end < 0 ? undefined : end).filter((l) => l.includes('Driftloom:playing'));
};
const wakeLockHeld = () => wakeLocks().length > 0;
const screenOff = async () => {
  sh('input', 'keyevent', 'KEYCODE_SLEEP');
  await until('the screen to go off', () => asleep(), 20000, 500);
};
const screenOn = async () => {
  sh('input', 'keyevent', 'KEYCODE_WAKEUP');
  await until('the screen to come on', () => !asleep(), 20000, 500);
  try { sh('wm', 'dismiss-keyguard'); } catch { /* none */ }
};
const service = () => {
  const o = sh('dumpsys', 'activity', 'services', pkg);
  const running = /PlaybackService/.test(o);
  const foreground = /isForeground=true/.test(o);
  return { running, foreground, types: (/foregroundServiceType=0x([0-9a-f]+)/.exec(o) || /types=0x([0-9a-f]+)/.exec(o) || [])[1] };
};
// The media session's block, as dumpsys writes it: from the line that names
// the app to the next session's.
const session = () => {
  const o = sh('dumpsys', 'media_session');
  const at = o.indexOf(`${pkg}/Driftloom`) >= 0 ? o.indexOf(`${pkg}/Driftloom`) : o.indexOf(pkg);
  return at < 0 ? '' : o.slice(at, at + 3000);
};
const sessionState = () => {
  const m = /PlaybackState \{state=(\w+)/.exec(session());
  if (!m) return null;
  return /^\d+$/.test(m[1]) ? Number(m[1]) : ({ PLAYING: 3, PAUSED: 2, STOPPED: 1 }[m[1]] ?? null);
};
const notification = () => {
  const o = sh('dumpsys', 'notification', '--noredact');
  const records = o.split(/\n\s*NotificationRecord\(/).filter((r) => r.includes(`pkg=${pkg}`));
  return records.join('\n');
};
const focusHeld = () => sh('dumpsys', 'audio').includes(`pack: ${pkg}`);
const logcat = (tag) => sh('logcat', '-d', '-s', `${tag}:I`).split('\n');
const heartbeats = () => logcat('DriftloomService').filter((l) => l.includes('heartbeat')).map((l) => ({
  under: Number((/underruns=(\d+)/.exec(l) || [])[1]),
  late: Number((/late=(\d+)/.exec(l) || [])[1]),
  missed: Number((/missed=(\d+)/.exec(l) || [])[1]),
  dropped: Number((/dropped=(\d+)/.exec(l) || [])[1]),
  frames: Number((/frames=(\d+)/.exec(l) || [])[1]),
  rate: Number((/rate=(\d+)/.exec(l) || [])[1]),
  line: l,
}));

// ------------------------------------------------------------ the picker

// The system's file picker (DocumentsUI) on the screen: its nodes, from the
// UI dump.
function nodes() {
  sh('uiautomator', 'dump', '/sdcard/ui.xml');
  const xml = sh('cat', '/sdcard/ui.xml');
  const out = [];
  for (const m of xml.matchAll(/<node [^>]*>/g)) {
    const a = (k) => ((new RegExp(`${k}="([^"]*)"`)).exec(m[0]) || [])[1] || '';
    const b = /bounds="\[(\d+),(\d+)\]\[(\d+),(\d+)\]"/.exec(m[0]);
    if (!b) continue;
    out.push({
      text: a('text'), id: a('resource-id'), desc: a('content-desc'), enabled: a('enabled') === 'true', clickable: a('clickable') === 'true',
      x: (Number(b[1]) + Number(b[3])) / 2, y: (Number(b[2]) + Number(b[4])) / 2,
    });
  }
  return { xml, list: out };
}
const tap = (n) => sh('input', 'tap', String(Math.round(n.x)), String(Math.round(n.y)));

// Choose a place and save in it. The AOSP picker opens in some folder; if
// Save is not offered there, Downloads is.
async function pickerSave() {
  await until('the system picker', () => nodes().xml.includes('com.android.documentsui'), 45000, 1500);
  let last = null;
  for (let i = 0; i < 20; i++) {
    const { xml, list } = nodes();
    last = xml;
    const save = list.find((n) => /^save$/i.test(n.text) || /container_save|button1/.test(n.id) && /save/i.test(n.text));
    if (save && save.enabled) {
      tap(save);
      return true;
    }
    // Not here: the roots (the menu), then Downloads.
    const downloads = list.find((n) => /^downloads$/i.test(n.text));
    if (downloads) tap(downloads);
    else {
      const menu = list.find((n) => /roots|navigate up|open navigation/i.test(n.desc));
      if (menu) tap(menu);
    }
    await sleep(1500);
  }
  log('the picker never offered Save; its last screen:');
  log((last || '').slice(0, 4000));
  return false;
}

// ----------------------------------------------------------------- the page

const page = await AppPage.open({ pkg, ms: 120000, log });
const ev = (fn, arg, gesture = false, ms = 20000) => page.evaluate(fn, arg, { gesture, ms });
// A question to a page that may not answer (a hidden page's timers are slowed, a frozen one's stop): an
// answer, or null after a few seconds; never a test that waits for it.
const ask = (fn, arg) => ev(fn, arg, false, 8000).catch(() => null);
const playingNow = () => ask(() => document.getElementById('playBtn').getAttribute('aria-label') === 'Pause');
const loopName = () => ask(() => document.getElementById('loopName').textContent.trim());
const diag = () => ev("document.getElementById('diagRefresh').click(); document.getElementById('diagOut').textContent");
const line = (t, key) => (t.split('\n').find((l) => l.startsWith(`${key}:`)) || '').trim();
const waitFor = (what, fn, ms = 20000) => until(what, fn, ms, 500);
const dispatch = (key) => sh('cmd', 'media_session', 'dispatch', key);
// What the app and the page said about the controls, for a failed check.
const evidence = (what) => {
  log(`  evidence (${what}):`);
  const lines = sh('logcat', '-d').split('\n').filter((l) => /DriftloomService|DriftloomApp|DriftloomStream|MediaSession|MediaButton/.test(l)).slice(-25);
  for (const l of lines) log(`    ${l.slice(0, 260)}`);
};
// How the page runs, as it says itself without a timer of its own to wait for: what it thinks of its
// visibility, and how long since its own 50 ms interval last ran (a page whose timers are slowed shows it).
const hiddenPage = () => ask(() => ({
  visible: document.visibilityState,
  hidden: document.hidden,
  lastTickAgoMs: window.__tick ? Math.round(performance.now() - window.__tick) : null,
  locks: !!navigator.locks,
}));

try {
  await waitFor('the page to be built', () => ev("document.readyState === 'complete' && !!document.getElementById('playBtn') && !!document.getElementById('diagOut')"), 90000);
  sh('logcat', '-c');
  await ev(() => { window.__tick = performance.now(); setInterval(() => { window.__tick = performance.now(); }, 50); });
  const bridged = await ev("typeof window.DriftloomApp === 'object' && !!window.DriftloomApp");
  check('the app\'s second bridge (saving, the lock screen) is in the page', bridged);

  // ------------------------------------- the notification, refused first
  // Android 13 and later asks at the first Play, with the app in front. The
  // listener says no: the music, the service and the session go on as they
  // do (the notification itself is the one thing that doesn't show).
  log('\n-- the first Play asks about notifications; the answer is no --');
  const sdk = Number(sh('getprop', 'ro.build.version.sdk'));
  await ev("document.getElementById('playBtn').click()", undefined, true);
  if (sdk >= 33) {
    const asked = await until('the permission dialog', () => nodes().xml.includes('permissioncontroller'), 30000, 1500).then(() => true, () => false);
    check('the first Play asks about notifications', asked);
    if (asked) {
      const no = nodes().list.find((n) => /^don.t allow$/i.test(n.text.replace(/\u2019/g, "'")));
      if (no) tap(no);
      await sleep(1500);
    }
  }
  await waitFor('the music to play', () => playingNow(), 60000);
  await sleep(2000);
  check('refused: the music plays and the service holds on', service().foreground && sessionState() === 3, JSON.stringify(service()));
  sh('input', 'keyevent', 'KEYCODE_HOME');
  await sleep(1500);
  await screenOff();
  await sleep(20000);
  check('refused: with the screen off it plays on', service().foreground && sessionState() === 3 && wakeLockHeld() && asleep(), JSON.stringify(service()));
  dispatch('pause');
  await waitFor('Pause', async () => !(await playingNow()), 20000).catch(() => null);
  check('refused: the session\'s Pause still works', !(await playingNow()));
  await screenOn();
  sh('am', 'start', '-n', `${pkg}/app.loops.MainActivity`);
  await sleep(2000);
  // Allowed from here on: the notification itself is read below.
  sh('pm', 'grant', pkg, 'android.permission.POST_NOTIFICATIONS');
  sh('logcat', '-c');

  // One minute for each track, so the core hands the music from loop to loop
  // by itself while the page is hidden.
  await ev(() => {
    const r = document.getElementById('trackLen');
    r.value = '1';
    r.dispatchEvent(new Event('input', { bubbles: true }));
  });

  // ---------------------------------------------------------------- play
  log('\n-- Play, then the app leaves the screen --');
  await ev("document.getElementById('playBtn').click()", undefined, true);
  await waitFor('the music to play', () => playingNow(), 60000);
  await sleep(3000);
  const first = await loopName();
  check('Play starts the foreground service', service().running && service().foreground, JSON.stringify(service()));
  check('the media session plays', sessionState() === 3, `state ${sessionState()}`);
  const note = notification();
  check('the notification has the loop\'s name and Previous, Play or Pause, Next', note.includes(first) && /Previous/.test(note) && /Next/.test(note) && /Pause/.test(note), note.slice(0, 600));
  check('the audio focus is held', focusHeld());
  check('the wake lock is held while it plays', wakeLockHeld());
  fact('cover in the session', /METADATA_KEY_ALBUM_ART|bitmap/i.test(session()) || /albumArt|bitmap/i.test(session()) ? 'yes' : 'not shown by dumpsys (read the screenshot)');

  sh('input', 'keyevent', 'KEYCODE_HOME');
  await sleep(2000);
  await screenOff();
  check('the screen is off and the app is in the background', asleep());

  // -------------------------------------------------------- ten minutes
  log(`\n-- ${MINUTES} minutes with the screen off --`);
  const t0 = Date.now();
  const startBeats = heartbeats().length;
  await sleep(MINUTES * 60000);
  const elapsed = (Date.now() - t0) / 1000;
  const beats = heartbeats();
  const mine = beats.slice(startBeats);
  check(`the service's heartbeat came every minute (${mine.length} in ${MINUTES})`, mine.length >= MINUTES - 1, `${mine.length}`);
  const a = mine[0];
  const z = mine[mine.length - 1];
  if (a && z) {
    const secs = (z.frames - a.frames) / z.rate;
    const span = (mine.length - 1) * 60;
    check('the audio thread kept real time: its frames over the minutes are those minutes', Math.abs(secs - span) <= 3, `${secs.toFixed(1)} s of sound in ${span} s`);
    check('no underrun with the screen off', z.under === a.under, `underruns ${a.under} -> ${z.under}`);
    check('nothing late, missed or dropped', mine.every((b) => b.late === 0 && b.missed === 0 && b.dropped === 0), z.line.replace(/^.*heartbeat /, ''));
    fact('the callback', z.line.replace(/^.*heartbeat /, ''));
  }
  check('still playing: the service, the session and the wake lock', service().foreground && sessionState() === 3 && wakeLockHeld() && asleep(), JSON.stringify(service()));
  fact('screen off for', `${Math.round(elapsed)} s`);

  // What the app cost, as the system counts it (the emulator's, said to be only that).
  const cpu = sh('dumpsys', 'cpuinfo').split('\n').filter((l) => /driftloom|sandboxed|webview|chromium/i.test(l)).map((l) => l.trim()).slice(0, 6);
  fact('CPU with the screen off (dumpsys cpuinfo, the last minutes)', cpu.join(' ; ') || 'not listed');
  fact('the page while the screen is off', JSON.stringify(await hiddenPage()));
  fact('the page\'s timer', JSON.stringify(await ask(() => new Promise((resolve) => { const t = performance.now(); setTimeout(() => resolve(Math.round(performance.now() - t)), 100); }))));

  // The page was hidden the whole time: its loops moved on by the core's tracks.
  const gone = await ev(() => document.getElementById('playBtn').getAttribute('aria-label'));
  check('the page still plays (its button says Pause)', gone === 'Pause', gone);
  const t = await diag();
  check('the page\'s player has nothing late across the tracks', /late: 0\s+missed: 0\s+dropped: 0/.test(line(t, 'engine')), line(t, 'engine'));

  // ------------------------------------------------ the lock screen's keys
  log('\n-- the lock screen\'s controls, screen off --');
  fact('the page before the first button', JSON.stringify(await hiddenPage()));
  const before = await loopName();
  dispatch('next');
  await waitFor('Next to change the loop', async () => { const n = await loopName(); return n && n !== before; }, 40000).catch(() => null);
  const after = await loopName();
  if (!check('Next (the session\'s) goes to a new loop', after !== before, `${before} -> ${after}`)) evidence('Next');
  dispatch('previous');
  await sleep(3000);
  check('Previous (the session\'s) is heard by the page and the music plays on', await playingNow(), `${after} -> ${await loopName()}`);

  dispatch('pause');
  await waitFor('Pause to pause the page', async () => !(await playingNow()), 20000).catch(() => null);
  if (!check('Pause pauses the page', !(await playingNow()))) evidence('Pause');
  await waitFor('the session to say paused', () => sessionState() === 2, 15000).catch(() => null);
  check('the session says paused', sessionState() === 2, `state ${sessionState()}`);
  await sleep(11000);
  check('paused: no wake lock, no audio focus', !wakeLockHeld() && !focusHeld(), `wake ${wakeLocks().join(' | ').trim()}, focus ${focusHeld()}`);
  check('paused: the service is not foreground; the notification stays (Play)', !service().foreground && /Play/.test(notification()), JSON.stringify(service()));
  check('paused: the stream is stopped (nothing runs)', logcat('DriftloomStream').some((l) => l.includes('stream stopped')), logcat('DriftloomStream').slice(-3).join('\n'));

  dispatch('play');
  await waitFor('Play to play', () => playingNow(), 20000).catch(() => null);
  check('Play plays the page again', await playingNow());
  await sleep(3000);
  check('playing again: the stream is back, and the lock is held', logcat('DriftloomStream').some((l) => l.includes('stream started')) && wakeLockHeld() && service().foreground, JSON.stringify(service()));

  // ------------------------------------------------------------- a call
  log('\n-- a call (simulated: another app takes the audio focus) --');
  const call = (what) => sh('am', 'broadcast', '-n', `${pkg}/app.loops.CallSimulator`, '-a', `app.loops.debug.${what}`);
  call('CALL_START');
  await waitFor('the call to pause the music', async () => !(await playingNow()), 20000).catch(() => null);
  check('a call pauses the music', !(await playingNow()));
  await sleep(2000);
  call('CALL_END');
  await waitFor('the music to return', () => playingNow(), 20000).catch(() => null);
  check('when the call ends, the music returns', await playingNow());
  call('CALL_START');
  await waitFor('the second call to pause', async () => !(await playingNow()), 20000).catch(() => null);
  dispatch('pause');
  await sleep(1000);
  call('CALL_END');
  await sleep(4000);
  check('a music paused by the listener before the call ends stays paused', !(await playingNow()));
  dispatch('play');
  await waitFor('Play again', () => playingNow(), 20000).catch(() => null);

  // ------------------------------------------------------ the screen on
  await screenOn();
  sh('am', 'start', '-n', `${pkg}/app.loops.MainActivity`);
  await sleep(3000);
  const t2 = await diag();
  check('on again: the page is playing and nothing was late', await playingNow() && /late: 0\s+missed: 0\s+dropped: 0/.test(line(t2, 'engine')), line(t2, 'engine'));
  fact('Diagnostics, the stream', line(t2, 'stream'));
  fact('Diagnostics, while hidden', line(t2, 'hiddenMedia') || line(t2, 'hidden'));

  // Pause for the export's own run: nothing playing, only the file.
  await ev("document.getElementById('playBtn').click()", undefined, true);
  await waitFor('the pause', async () => !(await playingNow()), 20000);

  // ---------------------------------------------------------- the export
  if (EXPORT) {
    log('\n-- an export into a file chosen in Android\'s picker, finishing with the screen off --');
    const exportOne = async (format, length) => {
      await ev(() => {
        if (document.getElementById('viewSave').hidden) document.getElementById('saveBtn').click();
        document.querySelector('[data-stab="share"]').click();
      });
      await sleep(1500);
      if (format === 'small') {
        const shown = await ev(() => !document.querySelector('[data-format="small"]').hidden);
        if (!shown) {
          fact('Small', 'hidden: this WebView has no Opus encoder');
          return null;
        }
      }
      const missing = await ev(([f, l]) => {
        const pick = (sel) => { const e = document.querySelector(sel); if (e) e.click(); return e ? '' : sel; };
        return [pick(`[data-format="${f}"]`), pick(`#exportLengths [data-length="${l}"]`)].filter(Boolean).join(', ')
          || ((document.getElementById('exportAudio').disabled && `exportAudio is disabled: ${document.getElementById('exportAbout').textContent}`) || '');
      }, [format, String(length)]);
      if (missing) {
        const sheet = await ev(() => ({
          about: document.getElementById('exportAbout').textContent,
          choose: !document.getElementById('exportChoose').hidden,
          keys: [...document.querySelectorAll('#exportLengths .vkey')].map((b) => b.dataset.length).join(' '),
          worker: typeof Worker,
        }));
        check(`${format}: the export sheet has what the run clicks`, false, `${missing}; ${JSON.stringify(sheet)}`);
        // Why the Worker would not start: what the page's own loader serves, and what Workers say.
        const probe = (label, make) => ev(([text]) => new Promise((resolve) => {
          try {
            const w = (new Function('return ' + text))()();
            w.onerror = (e) => resolve(`error: ${e.message} ${e.filename}:${e.lineno}`);
            w.onmessage = (e) => resolve(`message: ${JSON.stringify(e.data).slice(0, 80)}`);
            w.postMessage({ type: 'hello' });
            setTimeout(() => resolve('no answer in 10 s'), 10000);
          } catch (e) { resolve(`threw: ${e}`); }
        }), [make], false, 30000).then((r) => fact(label, r), (e) => fact(label, `no answer: ${e.message}`));
        await probe('the export Worker (module)', "() => new Worker(new URL('js/export-worker.js', location.href), { type: 'module' })");
        await probe('the cover Worker (module)', "() => new Worker(new URL('js/cover-worker.js', location.href), { type: 'module' })");
        await probe('a classic Worker of the same file', "() => new Worker(new URL('js/rng.js', location.href))");
        await probe('a Worker from a Blob', "() => new Worker(URL.createObjectURL(new Blob(['onmessage = () => postMessage(\\'blob ok\\')'])))");
        await probe('a module Worker from a Blob importing a file by its address', "() => new Worker(URL.createObjectURL(new Blob([`import * as r from '${location.origin}/js/rng.js'; onmessage = () => postMessage('imported ' + Object.keys(r).length)`], { type: 'text/javascript' })), { type: 'module' })");
        await probe('a module Worker from a Blob with no import', "() => new Worker(URL.createObjectURL(new Blob(['onmessage = () => postMessage(\\'module blob ok\\')'], { type: 'text/javascript' })), { type: 'module' })");
        await probe('a module Worker of a file with no import (js/rng.js)', "() => new Worker(new URL('js/rng.js', location.href), { type: 'module' })");
        await probe('a classic Worker that fetches a file', "() => new Worker(URL.createObjectURL(new Blob([`onmessage = async () => { try { const r = await fetch('${location.origin}/js/rng.js'); postMessage('fetch ' + r.status + ' ' + r.headers.get('content-type')); } catch (e) { postMessage('fetch failed ' + e); } }`])))");
        await probe('a classic Worker that imports by dynamic import()', "() => new Worker(URL.createObjectURL(new Blob([`onmessage = async () => { try { const m = await import('${location.origin}/js/rng.js'); postMessage('import ok ' + Object.keys(m).length); } catch (e) { postMessage('import failed ' + e); } }`])))");
        fact('the WebView', String(await ask(() => navigator.userAgent)));
        for (const m of ['export-render', 'export-opus', 'ogg', 'app', 'request', 'player']) {
          fact(`import of js/${m}.js in the page`, String(await ask((n) => import(new URL(`js/${n}.js`, location.href).href).then(() => 'ok', (e) => `failed: ${e}`), m)));
        }
        log(`  the page's problems: ${page.problems.slice(-5).join(' | ')}`);
        return null;
      }
      await ev("document.getElementById('exportAudio').click()", undefined, true);
      const picked = await pickerSave();
      if (!check(`${format}: the picker opened and took Save`, picked)) return null;
      await waitFor('the export to run', () => ev("!document.getElementById('exportRun').hidden"), 60000).catch(() => null);
      sh('input', 'keyevent', 'KEYCODE_HOME');
      await sleep(1500);
      await screenOff();
      await sleep(8000);
      const n = notification();
      check(`${format}: with the screen off, the notification says it is saving and the service holds on`, /Saving/.test(n) && service().foreground && wakeLockHeld(), `${service().foreground} ${wakeLockHeld()} ${n.slice(0, 300)}`);
      const t0 = Date.now();
      await waitFor('the export to finish', () => ev("!document.getElementById('exportDone').hidden || !document.getElementById('exportFailed').hidden"), 20 * 60000);
      const took = (Date.now() - t0) / 1000;
      const sheet = await ev(() => ({
        done: !document.getElementById('exportDone').hidden,
        saved: document.getElementById('exportSaved').textContent,
        why: document.getElementById('exportWhy').textContent,
      }));
      check(`${format}: the export finished with the screen off`, sheet.done && asleep(), `${sheet.saved || sheet.why} (${Math.round(took)} s after the first check)`);
      await sleep(2000);
      check(`${format}: when it is done, nothing is held`, !wakeLockHeld() && !service().foreground, `wake ${wakeLockHeld()}, ${JSON.stringify(service())}`);
      await screenOn();
      sh('am', 'start', '-n', `${pkg}/app.loops.MainActivity`);
      await sleep(2000);
      const name = (/chose: (.+?), \d/.exec(sheet.saved) || [])[1];
      if (name) {
        const found = sh('find', '/storage/emulated/0/', '-name', name, '-size', '+1k').split('\n').filter(Boolean)[0];
        const head = found ? sh('head', '-c', '4', `'${found}'`) : '';
        const bytes = found ? Number(sh('stat', '-c', '%s', `'${found}'`)) : 0;
        check(`${format}: the file is where the picker put it, with the right header`, !!found && (format === 'small' ? head.startsWith('OggS') : head.startsWith('fLaC')), `${found} ${bytes} bytes`);
        fact(`${format} file`, `${found}, ${bytes} bytes`);
      } else {
        check(`${format}: the page says where it saved`, false, sheet.saved);
      }
      await ev("document.getElementById('exportDoneBtn').click()");
      return true;
    };
    await exportOne('lossless', 1800);
    await exportOne('small', 600);
  }
} catch (e) {
  check('the run went to its end', false, e.message);
}

log('\n---- the page\'s problems ----');
const uncaught = page.problems.filter((p) => p.startsWith('uncaught'));
log(page.problems.join('\n') || 'none');
check('no uncaught error in the page', !uncaught.length, uncaught.join('; '));
try { page.close(); } catch { /* gone */ }

const failed = rows.filter((r) => !r.ok).length;
if (SUMMARY) {
  const text = [
    `### The app with the screen off (emulator, ${MINUTES} minutes)`,
    '',
    '| check | result |',
    '|---|---|',
    ...rows.map((r) => `| ${r.name.replace(/\|/g, '/')} | ${r.ok ? 'ok' : `**FAIL** ${String(r.detail).replace(/\|/g, '/').replace(/\n/g, ' ').slice(0, 200)}`} |`),
    '',
    ...facts.map((f) => `- ${f.name}: ${String(f.value).replace(/\n/g, ' ')}`),
    '',
    'Said only of the emulator: its figures say the service holds what it must and the audio thread keeps time with the screen off; a phone\'s battery is the phone\'s to say.',
    '',
  ].join('\n');
  fs.appendFileSync(SUMMARY, text);
}
log(failed ? `\n${failed} check(s) failed` : '\nthe app plays on with the screen off, and saves');
process.exit(failed ? 1 : 0);
