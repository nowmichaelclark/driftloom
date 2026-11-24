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
  Two exceptions. A computer zoomed to 400% (Mikey, 2026-11-22: "let it
  scroll at 400%, only those who have accessibility issues would zoom in
  that much I think"): under 280 px high and no finger, the page scrolls
  down, never across. And, for the browser's address bar alone (Mikey, 2026-11-22:
  "when viewing on chrome, I can't get the address bar at the top to go
  away; what would be the sensible way to do that without Fullscreen? I
  would thing the user could just be able to scroll down a bit"): in a
  touch browser's tab the document scrolls that bit; the app stays put and
  grows into the room. Not installed, not the Android app.
- **Bottom buttons have no lights or LEDs;** they only change icon (for
  now only the bottom-right Sound / Layers switch does).
- **Four settings exist:** the theme, "lighter on processor", Mono, and
  the letter keys' switch in Advanced (item 122, v211; Mikey, 2026-11-23,
  his exception to three). Everything new below is a pick, a sheet or a
  message, not a setting.
- **The music is stereo** (since v116), with a **Mono** switch in the
  settings for anyone who prefers it (Mikey, 2026-10-15). It sits in
  Advanced, under Sound, as a plain switch like Lighter on processor's:
  yours to restyle (queue item 66).
- **Calm and wholesome:** the music is cozy (a nice coffee shop, Animal
  Crossing, Ocarina of Time's whimsy); the look should feel the same.
- **Plain characters only** in anything the app shows as a code or name
  (letters, digits, `-` and `_`); no emoji in codes.
- It must run smoothly on old Android phones and weak chips.

## What the final pass settled (session 30f, item 114, v196 to v198)

- **Two layouts.** Upright: card, panel, keys. Under 520 px high and 480 or
  wider (a phone on its side; Mikey's review, 2026-11-22): panel at the
  left, card over keys at the right, the two filling the safe area, up to
  560 px a side. The reading order is the same both ways: card, panel,
  keys. A design is drawn for both, and at 360x640 and 320x568.
- **The panel asks its own height** (the container `middle`: 340, 300,
  240 px): a view closes its gaps by it, and never scrolls to fit.
- **Targets:** 44 px where there is room, never under 24.
- **Keys** (item 122, v211; Mikey, 2026-11-23: intuitive over accessible
  where they pull apart): as music players have them. Space and K play,
  the arrows next and previous (Shift a bar), Up and Down the volume, M
  mute, R and 1 to 5 the dice, a letter a view, Esc back, ? a popup that
  lists them. The letter keys' switch in Advanced turns every lone letter,
  digit and symbol off (WCAG 2.1.4). The map and its sources are at
  `js/shortcuts.js`'s top; a key's tooltip shows its key.
- **The bar is WCAG 2.2 AA** in every theme and in forced colors, held by
  `tools/a11y.mjs`: text 4.5:1 (3:1 large), marks and rings 3:1. A hue used
  for words has its own deeper token (`--mark-text`).
- **State is a shape as well as a color:** pressed is a ring, muted notes
  are rings, a soft note keeps its ring at full color. Off is one dimming,
  0.55, never doubled.
- **Focus:** a 2 px ring on everything, whole; a view opened puts the focus
  on its way out, closed on the key that opened it.
- **A popup is solid,** and so is a slider's knob (item 119). A screen
  that can do nothing shows no keys.
- **The card** (item 119, Mikey, 2026-11-23 and 24): the art on the right,
  the words on the left; the playhead's lit dot a diffused LED, no sparkle.
- **Text size is the root size** (rem, the pixel caps holding), in the
  browser and in the Android app.
- The baked spec (`design/driftloom_v20.html`) predates this: bake it again
  before designing on it.

## 1. Next-loop picks (soonest: can be designed now)

**Designed (Mikey chose it, 2026-10-13):** `design/next-picks.md`, from
Painter 5 -- the New loops panel, the picks on Now Playing and the
Length picker. **Built plain (session 30j, v205):** the tempo, key and
scale, style and voices pickers, the range control (two sliders) and clear
all, `design/next-picks.md`, "Tempo, key, style and voices"; the "can't do
that" line is the core's words, under the pace line and in each picker.

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

**Built plain (session 18, v183, `js/piece.js`, `design/` untouched):** on Now Playing the
rail becomes the piece's line (ticks for the map, a bead for you), under it
"bar 812/3,072 · peak" and the time played and left (a minus before it), with a key each way
for the section before or after; press the line to jump. The journey (arc,
wander) sits beside the Bars picker's ∞ and any. The Library's lists end in each
loop's length. The final pass (30f) kept it.

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

- **The Export view** (session 30e; behind Share's export audio). A key a
  format, named by it, and pressing it starts: **flac** (lossless),
  **opus** (much smaller, for sharing), **midi**.
- **Its length is the track's** (session 30c): the Track length view's time
  and fades; with none, a finite piece exports whole, and a loop that plays
  on (or Endless) opens the Track length view first.
- **Not enough space.** A clear message before anything starts.
- **Progress.** Calm progress with Cancel, and one line: closing the app
  stops it.
- **Done and failed.** Open or share the file when done; a plain,
  kind message if it failed.
- **An album whole** (item 121): with an album playing or open, keys
  **loop** and **album**, and for the album **zip** and **files**. One calm
  progress for the album ("2/4 · 37%"), its space checked first; a track
  that plays on asks how long once, for all. Files are named "NN name.ext".
- **Tagged** (item 121): every file says its name, the album's and its place
  in it, the app, the day, its code, and its cover as the picture.

## 4. Share codes, version 2

Share codes are how loops and albums travel: everything is in the code,
no server. They are about to become much shorter (a 24-hour, 12-track
album can fit in under 20 characters).

- **Showing a code.** The new short code, easy to copy.
- **Share as a link.** Sharing a link rather than the raw code.
- **Link preview image.** The picture Discord and other sites show
  when the link is posted.
- **Album from one seed.** Making a whole album from a single seed:
  how many tracks, and the picks they share; **save** puts it in the
  Library (item 121).
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

- **The icon** is drawn (item 120, Mikey's pick): `icons/icon.svg` is the
  one source, `node tools/icons.mjs` makes every size (the web app's, the
  adaptive icon's three layers and the status icon as vector drawables, the listing's icon and
  feature graphic) and checks the safe zones.
- **Android app.** A splash screen, and the artwork shown on the lock
  screen and in the media notification.
- **F-Droid listing** (later Google Play too). Icon, feature graphic and
  screenshots.
- **itch.io desktop release.** A cover image, a page banner,
  screenshots, and window icons for Windows and Linux.

## 9. Games and developers

Driftloom's engine is going into Broken Friends, Mikey's own game
(Mikey, 2026-10-24; it replaced the three mod games), where the music
follows each creature, the time of day and what is happening at camp.

- **An in-game "now playing" toast.** Optional, small, out of the way.
- **A name and logo for the engine itself**, separate from the app, and
  simple diagrams for the developer guide.
