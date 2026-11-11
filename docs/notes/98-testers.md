# 98: the testers' steps

Item 98's hands part (session 01). One block per device: Mikey copies the
block and sends it as it is (or follows it himself). Each takes about
fifteen minutes, most of it waiting. What comes back is a few lines of
Diagnostics, pasted into a message to the brain or the hands. The goal:
`late: 0  missed: 0  dropped: 0` after a real listening session, and no
too-slow line where there should be none.

Every block uses the site's own screens, so nothing needs installing:
`https://driftloom.nowmichaelclark.workers.dev/`. Headphones or the
speaker, as you like; the sound does not matter for this, only the lines
at the end.

## Mikey's Moto (stays at full quality)

```text
Driftloom test: the Moto

1. Open https://driftloom.nowmichaelclark.workers.dev/ in Chrome, and
   reload it once so you have the newest version.
2. Tap Play. Let it play for a minute.
3. Tap Next (the right-hand arrow) six or seven times, a few seconds
   apart.
4. Make a long loop: tap "vibe", then the "any bars" line, type 4096 (or
   drag the slider to 4k), tap "done", then tap Next. Let it play for
   three minutes.
5. Make it Endless: tap "vibe", "bars", the infinity key, "done", then
   Next. Let it play for three minutes.
6. Lock the phone (the power key) with the music playing. Leave it for
   five minutes. Unlock it and see that it is still playing.
7. Tap Library at the bottom, then More, then Advanced, then scroll to
   Diagnostics. Tap "refresh".
8. Copy these lines from it and send them back: build, cores, busiest,
   slow, engine. (The "copy" key under it copies everything; that is fine
   too.)

What I hope to see: "engine: ... late: 0  missed: 0  dropped: 0" and
"slow: no". If anything stuttered or paused, say when.
```

## Mikey's tablet (the weakest device; it may move itself to lite)

```text
Driftloom test: the tablet

1. Open https://driftloom.nowmichaelclark.workers.dev/ in the browser you
   use on it, and reload once so you have the newest version.
2. Tap Play. Let it play for a minute.
3. Tap Next six or seven times, a few seconds apart.
4. Make a long loop: tap "vibe", then the "any bars" line, type 4096 (or
   drag the slider to 4k), tap "done", then tap Next. Let it play for five
   minutes.
5. Make it Endless: tap "vibe", "bars", the infinity key, "done", then
   Next. Let it play for five minutes.
6. Turn the screen off with the music playing and leave it for five
   minutes. Turn it on and see that it is still playing.
7. Tap Library, then More, then Advanced, then scroll to Diagnostics. Tap
   "refresh".
8. Copy these lines and send them back: build, cores, busiest, slow,
   engine.

What I hope to see: "engine: ... late: 0  missed: 0  dropped: 0" and
"slow: no". It is fine if the app has moved itself to lite ("lite: true,
auto" in the cores line); that is not trouble. If the card says "This
browser is too slow." tell me, and send the same lines.
```

## The friends' Pixels on GrapheneOS (twice: JIT allowed, then blocked)

```text
Driftloom test: the Pixel, GrapheneOS

You will do this twice: once with JavaScript JIT allowed for the site
(the normal setting) and once with it blocked. The second time Driftloom
should say so by itself: "This browser is too slow." in the place where
the loop's name is. That is expected, not a fault.

The setting: in Vanadium, open the site, tap the icon at the left of the
address bar, then Site settings, then "JavaScript JIT" (it may read
"JavaScript just-in-time (JIT)"): Allow for the first run, Block for the
second. Reload the page after changing it.

For each run:
1. Open https://driftloom.nowmichaelclark.workers.dev/ and reload.
2. Tap Play. Let it play for two minutes. (With JIT blocked, watch for
   the line within the first minute or two.)
3. Tap Next six or seven times, a few seconds apart.
4. Make a long loop: tap "vibe", then the "any bars" line, type 4096 (or
   drag the slider to 4k), tap "done", then tap Next. Let it play for
   three minutes.
5. Make it Endless: tap "vibe", "bars", the infinity key, "done", then
   Next. Let it play for three minutes.
6. Turn the screen off with the music playing, leave it for five minutes,
   and turn it on again.
7. Tap Library, then More, then Advanced, then scroll to Diagnostics. Tap
   "refresh".
8. Copy these lines and send them back, saying which run it was (JIT
   allowed, or JIT blocked): build, cores, busiest, slow, engine.

What I hope to see: with JIT allowed, "late: 0  missed: 0  dropped: 0"
and "slow: no". With JIT blocked, the line on the card, and
"slow: lite past the line" in Diagnostics, with the busiest line well
over 50%.
```
