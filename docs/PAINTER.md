# What there is to paint

For Painter sessions (Mikey's design work with other chatbots, brought
into the app afterward). Written by the brain, 2026-10-13, from the core
charter (`docs/CORE.md`). Everything here is something the app does not
paint yet. Ideas welcome for all of it; only the first group will be
used soon. The rest waits for its phase of the core work.

## The rules any design keeps

- **One screen:** an info card on top, a switching viewport in the
  middle, mostly icon buttons below. Previous / Play / Next are always
  fixed in place.
- **Scrolling in exactly two places,** both in the middle viewport: the
  Library's lists and the Advanced page. Nothing else scrolls.
- **Bottom buttons have no lights or LEDs;** they only change icon (for
  now only the bottom-right Sound / Layers switch does).
- **Only two settings exist:** the theme and "lighter on processor".
  Everything new below is a pick, a sheet or a message, not a setting.
- **The music is stereo** (coming soon), with a **Mono** switch in the
  settings for anyone who prefers it (Mikey, 2026-10-15).
- **Calm and wholesome:** the music is cozy (a nice coffee shop, Animal
  Crossing, Ocarina of Time's whimsy); the look should feel the same.
- **Plain characters only** in anything the app shows as a code or name
  (letters, digits, `-` and `_`); no emoji in codes.
- It must run smoothly on old Android phones and weak chips.

## 1. Next-loop picks (soonest: can be designed now)

**Designed (Mikey chose it, 2026-10-13):** `design/next-picks.md`, from
Painter 5 -- the New loops panel, the picks on Now Playing and the
Length picker. Still to design: the tempo, key and scale, style and
voices pickers (with the range control), and the "can't do that" line.

Today a listener can pick **vibes** (mood words such as merry, tender or
floating) that each new loop leans toward. Soon they can also pin down
the music itself. Developers and games will ask for the same things in
code ("a peaceful, zany loop at 77 bpm, 16 bars, when the sun rises").

- **The "Next loop" panel.** One place for everything the listener asks
  of the next loop: the vibes plus the new picks below. It could be the
  Vibes view grown up, or a sibling to it. It shapes only loops still to
  come, never the one playing.
- **Bars (length).** How long the loop is before it repeats or
  wanders. Today loops are 2 to 32 bars; soon anything up to **32,768
  bars** (about 27 hours at 80 bpm), or **Endless**. Needs quick presets
  (4, 8, 16, 32...), a way to type any number, and an Endless choice.
  Very long pieces can be a long journey (think a symphony) or slow
  wandering.
- **Tempo.** Beats per minute: slow and drifting to bouncy.
- **Meter and rhythm feel.** 4/4, 3/4 (waltz), 6/8 (a lilting folk
  feel), and feels such as straight or swung.
- **Scale and key.** The set of notes the loop uses (major, minor,
  dorian, pentatonic and so on) and its home note.
- **Profile.** Driftloom's styles (cinder, tide, wayfare, shrine and
  the rest), each with its own character, instruments and drums.
- **Instruments, per layer.** Which voice plays the melody (kalimba,
  music box, marimba, fiddle, ocarina...), the chords, the bass, the
  texture, the drums.
- **Which layers play.** Melody, chords, bass, texture, drums: any can
  be left out (fewer layers is welcome variety).
- **Three states for every pick.** Each choice can be **pinned** (exactly
  this), given a **range** (anywhere from 70 to 85 bpm) or **left to
  chance** (Driftloom decides, as today). The design needs one clear,
  consistent way to show and switch these three states on every pick.
- **A range control.** Setting the low and high ends of a range.
- **Clear all picks.** Back to "everything left to chance", like Vibes'
  Clear All.
- **"Can't do that" message.** When picks truly can't be met together,
  a short, friendly line saying so (most combinations simply find an
  in-between, as vibes do).

## 2. Long pieces (after the generator moves to the core)

- **Where you are.** For a long piece: bar 812 of 3,072, time played
  and time left; something calm that fits the info card.
- **Moving within a piece.** Jumping or scrubbing to any point of a long
  piece (the engine can start anywhere instantly).
- **Section markers.** Later, when long pieces have sections (themes
  that return, an arc), showing them along the way.
- **Endless.** How an endless piece shows on Now Playing and the info
  card, where there is no "time left".
- **Long pieces in the Library.** How a loop's length or duration
  reads in a list, from 4 bars to 27 hours or Endless.

## 3. Exporting audio

Saving a loop as an audio file to keep or share. Safety comes first:
an export must never crash or break anything, even on an old phone.

- **The export sheet.** Choose the format each time: **Lossless** (full
  quality, larger) or **Small** (much smaller, for sharing).
- **Length for Endless pieces.** How much of an endless piece to export.
- **Not enough space.** A clear message before anything starts.
- **Progress.** Calm progress with Cancel; a note that it keeps going
  if the app goes to the background.
- **Done and failed.** Open or share the file when done; a plain,
  kind message if it failed.

## 4. Share codes, version 2

Share codes are how loops and albums travel: everything is in the code,
no server. They are about to become much shorter (a 24-hour, 12-track
album can fit in under 20 characters).

- **Showing a code.** The new short code, easy to copy.
- **Share as a link.** Sharing a link rather than the raw code.
- **Link preview image.** The picture Discord and other sites show
  when the link is posted.
- **Album from one seed.** Making a whole album from a single seed:
  how many tracks, and the picks they share.
- **"Needs a newer version" / "needs an instrument" messages.** When a
  code needs a newer Driftloom, or an instrument someone made that this
  device doesn't have.
- **Custom instrument badge.** A small mark that a code carries a
  home-made instrument inside it.

## 5. New instruments

Instruments are becoming small recipes anyone can make, including mods.

- **Icons for new instruments.** roundBass (a round, soft bass), glass
  (glassy bell tones), floorDrop (a soft low drum hit), arc (a gentle
  rising tone), and more as they are approved.
- **Modded or recipe instrument badge.** In Layers and Sounds, a quiet
  mark that an instrument is not one of Driftloom's own.

## 6. Stereo (soon: stereo by default, with a mono switch)

- **The Mono switch.** A setting that makes all of Driftloom mono
  (exactly today's sound), for people who prefer it or listen on one
  speaker. Stereo is the default.
- **Mono or stereo indicator,** if wanted: which one is playing.
- **Space controls,** if they are ever shown to listeners: how wide the
  whole mix is, where each instrument sits left to right, and how it
  moves.
## 7. The Advanced page

- **Core version.** Which version of the engine is running (share codes
  depend on it).
- **Health for testers.** Readouts such as "late notes: 0" that testers
  report back.

## 8. Platforms

- **Android app.** An adaptive app icon (with the monochrome version
  Android uses for themed icons), a splash screen, and the artwork shown
  on the lock screen and in the media notification.
- **F-Droid listing** (later Google Play too). Icon, feature graphic and
  screenshots.
- **itch.io desktop release.** A cover image, a page banner,
  screenshots, and window icons for Windows and Linux.

## 9. Games and developers

Driftloom's engine is going into three games (Luanti, OpenRCT2, Stardew
Valley), where the music follows time of day, weather and place.

- **Mod icons and screenshots** for each game's mod sites (Luanti's
  ContentDB, OpenRCT2's and Stardew's mod pages).
- **An in-game "now playing" toast.** Optional, small, out of the way.
- **A name and logo for the engine itself**, separate from the app, and
  simple diagrams for the developer guide.
