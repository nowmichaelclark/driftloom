# Session 28: the Android app: the screen off, the notification, saving an export (107d)

- **Model:** Sonnet 5.5. **Effort:** high.
- **Start after:** session 27.
- **Queue item:** item 107.

Paste everything in the block as the session's first message.

````text
Hands session 28 on the road to Driftloom's 1.0 (Sonnet 5.5, high): the Android app: the screen off, the notification, saving an export (107d).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 107); docs/CORE.md ("the platforms"; promises 1, 2 and 4); docs/DECISIONS.md ("Platforms and games"); docs/PAINTER.md ("Platforms"); js/media.js (the page's media session today); session 20's export code; sessions 26 and 27's PRs.

Check your place: in docs/sessions/README.md every row above session 28 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

The work, one PR (two if the export is cleaner apart):

- Playing with the screen off and the app in the background: a foreground service of the media-playback kind with its notification, a MediaSession (play, pause, next, previous; the loop's name and the app's artwork on the lock screen and in the notification), audio focus handled properly (pause on a call, duck or pause as Android asks, stop when headphones are pulled), no wake lock held when nothing plays. The native player needs nothing from the WebView to keep playing, so the service owns the audio.
- Battery: nothing running when paused; say what the app costs while playing with the screen off (the emulator's figures, said to be only that).
- Export in the app: session 20's and 21's exports work in the WebView or go native where they must (the file saved through Android's own picker, the Storage Access Framework, written as it is made; free space checked with the platform's call). Small is shown only if the WebView has the encoder.
- Permissions: only what this needs (the foreground service's, notifications on Android 13 and later, asked at the moment of first Play in the background, with the app still working if refused).

Yardsticks (the queue's): with the screen off for ten minutes on the emulator, the music plays on with nothing late and no underrun; the notification's and the lock screen's controls work; a call (simulated) pauses and returns; a long piece and an export finish with the screen off; the app passes Android's lint with no errors. In for-mikey.md: the same on the Moto and the tablet, ten minutes each with the screen off, a long piece and an export, and what to send back.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 28's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
