// Making the phone treat Driftloom as a music player.
//
// Web Audio on its own gets no lock-screen controls, no notification, and
// no claim on the Bluetooth transport buttons.
//
// The obvious fix — pipe the mix through an <audio> element using a
// MediaStreamAudioDestinationNode — does not work on Chrome for Android.
// Stream-backed elements are classed as *communications* audio, the same
// category as a WebRTC call, and communications audio is deliberately
// excluded from media notifications. It also adds a resampling stage that
// glitches under CPU pressure. It was tried; it produced sound and nothing
// else.
//
// What does work is playing a real encoded file, and it has to clear two
// separate bars that are easy to miss:
//
//   Length. Chrome treats anything under about five seconds as a sound
//   effect rather than content, and sound effects never get transport
//   controls. A two second loop was granted a session and still showed no
//   notification. This file is fifteen seconds.
//
//   Level. A stream Chrome judges silent loses the session, so this is not
//   digital silence. It was noise at roughly -62 dBFS RMS until v176, and
//   Mikey heard it as a hiss behind every loop (2026-11-17): noise up to
//   4 kHz, under the quiet passages. Now it is a 20 Hz sine at the same
//   power (-60 dB, peak -57 dBFS; tools/keepalive.py writes it): the
//   detector measures power, not pitch, so it clears the threshold as the
//   noise did, and 20 Hz that quiet is far under hearing at any volume.
//   Fifteen seconds is exactly 300 cycles, so the loop wraps without a
//   click. That is also why it stays lossless — an MP3 encoder would
//   discard a signal this quiet and hand back actual silence.
//
// The music itself goes straight to the speakers, untouched.
//
// In the Android app (session 28) none of this is the browser's: the core
// plays natively and a foreground service of the app's own holds the audio
// focus, the notification and a MediaSession, so what the page tells the
// phone (what is playing, the loop's name and its own cover, whether it
// plays) goes to the app (js/app.js), and the lock screen's, the
// notification's and a headset's buttons come back as the page's own Play,
// Pause, Next and Previous.

import { appLink } from './app.js';

const KEEPALIVE = [
  ['audio/keepalive.flac', 'audio/flac'],
  ['audio/keepalive.wav', 'audio/wav'],
];

export class MediaBridge {
  constructor(ctx) {
    this.ctx = ctx;
    this.source = null;
    this.audio = null;
    this.mode = 'pending';
    this.lastError = '';
    this.handlersSet = 0;
    // The Android app's end (js/app.js), when this is the app.
    this.app = null;
    // Told of every change in what the phone is told is playing: the
    // session's state as the app sets it, and the keepalive element stopping
    // or starting (Diagnostics' "while hidden", queue item 39).
    this.onchange = null;
  }

  _changed(what) {
    if (this.onchange) this.onchange(what);
  }

  attach(sourceNode) {
    // The synth already reaches the speakers on its own. Nothing is
    // re-routed here any more; re-routing was the mistake.
    this.source = sourceNode;
    // The Android app (session 27): the core plays natively, and the WebView
    // plays no audio of its own, the keepalive included. The lock screen's
    // controls there are the app's own service's (session 28).
    if (this.ctx && this.ctx.native) {
      this.mode = 'native';
      this.app = appLink();
      this.title = '';
      this.line = '';
      this.playing = false;
      return;
    }

    try {
      this.audio = document.createElement('audio');
      // FLAC first because it is a third of the size; the WAV is there in
      // case a browser will not take it.
      for (const [src, type] of KEEPALIVE) {
        const source = document.createElement('source');
        source.src = src;
        source.type = type;
        this.audio.appendChild(source);
      }
      this.audio.loop = true;
      this.audio.preload = 'auto';
      // Quiet content, not a quiet element. A muted or zero-volume element
      // forfeits the media session.
      this.audio.volume = 1;
      this.audio.setAttribute('playsinline', '');
      this.audio.style.display = 'none';
      // 'playing' comes again each time the loop wraps; only the one after
      // a pause is news.
      let paused = false;
      this.audio.addEventListener('pause', () => { paused = true; this._changed('keepalive paused'); });
      this.audio.addEventListener('playing', () => {
        if (paused) this._changed('keepalive playing');
        paused = false;
      });
      document.body.appendChild(this.audio);
      this.audio.load();
      this.mode = 'ready';
    } catch (err) {
      this.mode = 'unavailable';
      this.lastError = String(err);
    }
  }

  // What the app is told: whether it plays, and what.
  _tell(playing) {
    this.playing = playing;
    if (this.app) this.app.media(playing, this.title, this.line);
  }

  async start() {
    if (this.app) {
      this._tell(true);
      return this.mode;
    }
    if (!this.audio) return this.mode;
    try {
      await this.audio.play();
      this.mode = 'session';
    } catch (err) {
      // Autoplay refused, or the file is missing. The music is unaffected;
      // only the lock-screen controls are lost.
      this.mode = 'no-session';
      this.lastError = String(err && err.name ? err.name : err);
    }
    return this.mode;
  }

  stop() {
    if (this.app) this._tell(false);
    if (this.audio) this.audio.pause();
  }

  async resume() {
    if (this.audio && this.audio.paused) {
      try {
        await this.audio.play();
      } catch { /* the next tap will get it */ }
    }
  }

  // ------------------------------------------------------------ session

  // The element is in the document, so dropping the reference is not enough
  // to stop it: every rebuild left another keepalive loop in the DOM.
  dispose() {
    this.onchange = null;
    if (!this.audio) return;
    try { this.audio.pause(); } catch { /* nothing playing */ }
    try { this.audio.removeAttribute('src'); this.audio.load(); } catch { /* fine */ }
    try { this.audio.remove(); } catch { /* already gone */ }
    this.audio = null;
    this.mode = 'disposed';
  }

  setHandlers({ onPlay, onPause, onNext, onPrev }) {
    if (this.app) {
      this.app.handlers = { onPlay, onPause, onNext, onPrev };
      this.handlersSet = 4;
      return;
    }
    if (!('mediaSession' in navigator)) return;
    const set = (action, fn) => {
      try {
        navigator.mediaSession.setActionHandler(action, fn);
        return true;
      } catch {
        return false; // this browser does not offer that action
      }
    };
    const ok = [
      set('play', () => onPlay && onPlay()),
      set('pause', () => onPause && onPause()),
      set('stop', () => onPause && onPause()),
      set('nexttrack', () => onNext && onNext()),
      set('previoustrack', () => onPrev && onPrev()),
    ];
    this.handlersSet = ok.filter(Boolean).length;
  }

  setMetadata(title, subtitle, spec) {
    if (this.app) {
      this.title = title;
      this.line = subtitle;
      // The loop's own cover goes along; a loop changing while it plays is told at once.
      this.app.cover(spec);
      if (this.playing) this._tell(true);
      return;
    }
    if (!('mediaSession' in navigator) || typeof MediaMetadata === 'undefined') return;
    try {
      navigator.mediaSession.metadata = new MediaMetadata({
        title,
        artist: subtitle,
        album: 'Driftloom',
        artwork: [
          { src: 'icons/icon-192.png', sizes: '192x192', type: 'image/png' },
          { src: 'icons/icon-512.png', sizes: '512x512', type: 'image/png' },
        ],
      });
    } catch { /* metadata is a nicety, not a requirement */ }
  }

  setPlaybackState(playing) {
    if (this.app) {
      this._tell(playing);
      this._changed(`session ${playing ? 'playing' : 'paused'}`);
      return;
    }
    if (!('mediaSession' in navigator)) return;
    try {
      navigator.mediaSession.playbackState = playing ? 'playing' : 'paused';
    } catch { /* older browser */ }
    this._changed(`session ${playing ? 'playing' : 'paused'}`);
  }

  report() {
    const ms = 'mediaSession' in navigator;
    return {
      keepalive: this.mode,
      keepaliveErr: this.lastError || '-',
      keepalivePlaying: this.audio ? !this.audio.paused : false,
      keepaliveTime: this.audio ? +this.audio.currentTime.toFixed(1) : 0,
      keepaliveDuration: this.audio && isFinite(this.audio.duration)
        ? +this.audio.duration.toFixed(1) : '?',
      keepaliveSrc: this.audio && this.audio.currentSrc
        ? this.audio.currentSrc.split('/').pop() : '(none)',
      keepaliveReady: this.audio ? this.audio.readyState : '-',
      keepaliveMediaErr: this.audio && this.audio.error
        ? this.audio.error.code : '-',
      mediaSession: ms,
      handlers: this.handlersSet,
      metadata: ms && navigator.mediaSession.metadata
        ? navigator.mediaSession.metadata.title : '(none)',
      playbackState: ms ? navigator.mediaSession.playbackState : '-',
    };
  }
}
