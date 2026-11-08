/* dlcore.h: the Driftloom core, from C (docs/CORE.md, promise 4).
 *
 * Today it holds the player (queue item 82): a loop from its seed, made and
 * played inside the core with its own sequencer, as the app plays it, and
 * wandering pass after pass. The same seed and vibes make the same samples,
 * bit for bit, on every target and in the app (`?engine=player`).
 *
 * Build the library from core/ with
 *
 *     cargo rustc --release --lib --features capi --crate-type staticlib
 *
 * and link target/release/libdlcore.a; nothing else is needed (the core
 * carries its own math). examples/c/seed.c is a whole host: a seed to a WAV.
 *
 * The core never allocates. The host gives each player its memory
 * (dl_player_size() bytes, aligned to dl_player_align()) and keeps it for
 * the player's life; nothing is kept anywhere else, so a host may play as
 * many players as it gives memory for. A player is used from one thread at
 * a time: the audio thread's, typically, calling dl_player_render.
 *
 * A host's day:
 *
 *     void *memory = aligned_alloc(dl_player_align(), size);  // size: dl_player_size(), rounded up
 *     dl_player *p = dl_player_start(memory, size, 48000, 2, 1);
 *     dl_player_vibe(p, "peaceful");
 *     dl_player_load(p, 1234);       // made over the next few blocks
 *     dl_player_render(p, out, 512); // again and again
 *
 * Every call takes a null player and does nothing with it. Text the core
 * gives is plain ASCII.
 *
 * The player's calls take some 60 KB of the calling thread's stack at most
 * (56 to 60 KB measured in WebAssembly, a loop being made inside a render).
 * Emscripten's default stack is 64 KB, so a web build keeps its own big
 * buffers off the stack or sets a bigger one (-sSTACK_SIZE).
 */
#ifndef DLCORE_H
#define DLCORE_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* A core and the player on it, in the host's memory. */
typedef struct dl_player dl_player;

/* The layers, for dl_player_mute. */
enum { DL_DRUMS = 0, DL_BASS = 1, DL_CHORDS = 2, DL_MELODY = 3, DL_TEXTURE = 4 };

/* The moments a change waits for, for dl_player_change (queue item 86): as
   soon as the next loop is made, or the loop playing's next beat, bar line,
   phrase (four bars) or pass, or the track's end (queue item 94a,
   dl_player_track: the old loop rings out at a pass's end, and fades over
   the crossfade at a phrase line inside one; it waits while no length is
   set). And the crossfade's length by default, in seconds. */
enum { DL_NOW = 0, DL_BEAT = 1, DL_BAR = 2, DL_PHRASE = 3, DL_PASS = 4, DL_END = 5 };
#define DL_CROSSFADE 1.0

/* Where a player is, from dl_player_state. */
enum { DL_IDLE = 0, DL_LOADING = 1, DL_WANDERING = 2, DL_PLAYING = 3 };

/* The bytes a player takes, and the alignment they need: fixed, about 19 MB
   on a 64-bit target (the core's 17.4 MB, most of it room for its voices and
   delay lines at up to 96 kHz, and the player's 1.5 MB). */
size_t dl_player_size(void);
size_t dl_player_align(void);

/* Start a player in `memory` (`bytes` long): a core at `rate` Hz (22050 to
   96000), rendering `channels` channels (1 mono, 2 stereo), the full mix
   (`full` not 0) or the lighter one (0). Null if the memory is too small or
   misaligned, or the rate or layout is out of range. */
dl_player *dl_player_start(void *memory, size_t bytes, double rate, uint32_t channels, int full);

/* Add the vibe called `id` ("glacial", as data/vibes.txt names it) to what
   the next load asks for. 1 if the core has it, else 0. */
int dl_player_vibe(dl_player *player, const char *id);

/* The request (promise 11; queue item 85): what the next load asks for,
   beside the seed. Each of these returns NULL if it can be met, or else a
   short plain ASCII line saying why not ("tempo must be 32 to 190 bpm"),
   static and never freed, and the request is left as it was.

   Add the vibe called `id` at `weight`: the share of the loop it steers.
   Shares adding up to more than 1 are scaled to 1, less is a lighter touch,
   0 leaves it out; a vibe added with dl_player_vibe then counts 1. Past
   five vibes the five heaviest steer. */
const char *dl_player_weight(dl_player *player, const char *id, double weight);

/* Ask for a tempo, `lo` to `hi` bpm (whole bpm; equal to pin it), from 32 to
   190. The loop's tempo is drawn across it from the seed. */
const char *dl_player_ask_tempo(dl_player *player, double lo, double hi);

/* Pin a named setting (queue item 85b) by its number: the choices from 0
   in data/choices.txt's order, then the settings from 16 (arc.shape,
   arc.period, arc.from, arc.floor, kick.note, kick.short, kick.long,
   subDuck.depth, subDuck.recover, lateFloor.order, lateFloor.seconds,
   lateFloor.gap, padSweep.lo, padSweep.oct, padSweep.period, padSweep.q,
   echo.beats), then the space recipe's from 48 (core/src/space.rs). A word
   (a choice's option, an arc's shape, lateFloor's order) is its place in
   its list. Each is held to what the loop could draw ("a setting outside
   its range"). */
const char *dl_player_pin(dl_player *player, uint32_t number, double value);

/* Bend the loop of `seed` (queue item 85d): the loop the next load makes
   keeps that loop's profile mix, key and mode, metre and tune, and the vibes,
   the tempo, the pins and the load's own seed bend the rest. Load the
   theme's own seed with nothing else asked and it is the theme's loop: a
   friend keeps its tune as the day changes. */
void dl_player_theme(dl_player *player, uint32_t seed);

/* Ask for a piece `bars` bars long, any whole number from 1 up, or Endless
   with `bars` INFINITY (queue item 88). Up to 32 bars it is the loop's own
   length; past that the loop as drawn plays section after section, each
   drawn as a wandering pass is, the last of a round cut short at a bar line
   where the loop's length doesn't divide the piece's. A piece comes round
   at its end, as a loop does ("a bad length" for anything else). */
const char *dl_player_length(dl_player *player, double bars);

/* Ask for the home notes in `keys`, a bit each with C the lowest (so D or G
   is (1 << 2) | (1 << 7)), or for no key in particular with 0 (queue item 89).
   One note is a pinned key; a set is chosen from, the loop's seed's own where
   that is in it ("no such key" for a bit past B). */
const char *dl_player_keys(dl_player *player, uint32_t keys);

/* Ask for the modes in `scales`, a bit each by its place in the core's list
   (dorian, aeolian, ionian, mixolydian, lydian, phrygian, harmonicMinor,
   minorPent, majorPent, kumoi, hirajoshi, insen, wholeTone, lydianDominant,
   phrygianDominant, dorianSharp4, yo, ritusen, akebono), or for no mode in
   particular with 0 (queue item 89). One mode is a pinned scale ("no such
   scale" for a bit past the last). */
const char *dl_player_scales(dl_player *player, uint32_t scales);

/* Ask for the profiles in `styles`, a bit each by its place in the frozen
   order (dust, glade, thaw, haven, bloom, vapor, halcyon, clockwork, shatter,
   undertow, grove, hollow, shrine, tide, cinder, wayfare), or for no style in
   particular with 0 (queue item 90). One profile is a pinned style; a set is
   chosen from ("no such style" for a bit past the last). */
const char *dl_player_styles(dl_player *player, uint32_t styles);

/* Ask for the voices in `voices` of layer `layer` (0 to 4: drums, bass,
   chords, melody, texture), a bit each by its place in the layer's list, or
   for none in particular with 0 (queue item 91). The drums' voice is the kit,
   the texture's its kind. A voice asked of the melody or the chords is the
   voice they play: the choir is not forced by the others. ("no such layer",
   "no such voice"). */
const char *dl_player_voices(dl_player *player, uint32_t layer, uint32_t voices);

/* Ask for layer `layer` (0 to 4) to be left to chance (`state` 0), always
   played (1) or left out (2), muted as the listener's mute would be (queue
   item 91). "all layers off" for the fifth left out: the one request that
   cannot be met. */
const char *dl_player_layer(dl_player *player, uint32_t layer, uint32_t state);

/* Ask for what `request` says, as text, in place of what was asked since the
   last load: items split by commas, each a vibe ("glacial"), a vibe and its
   weight ("glacial=0.7"), the tempo ("bpm=77", "bpm=70-85"), the theme
   ("theme=1234"), the length ("bars=1024", "bars=endless"), the key ("key=D",
   "key=D|G"), the mode ("scale=dorian", "scale=dorian|aeolian"), the style ("style=tide",
   "style=tide|shrine"), a layer's voices ("voice.melody=kalimba|marimba"), a
   layer on or off ("layer.drums=off") or a pin
   ("organ=on", "tunedKick=long", "arc.period=240", "space.room=0.8"). */
const char *dl_player_ask(dl_player *player, const char *request);

/* Ask for the loop of `seed`, with what was asked since the last load. It
   is made a stage a block over the blocks rendered next (so no block's work
   spikes), then starts; a loop playing stops being decided at once, and
   what it has handed over plays out. */
void dl_player_load(dl_player *player, uint32_t seed);

/* Change to the loop of `seed`, with what was asked since the last load, at
   `moment` of the loop playing (DL_NOW to DL_END): the next loop is made in
   the blocks before the moment and takes its first step on it, and the old
   one's notes fall to nothing over `crossfade` seconds from there
   (DL_CROSSFADE), or ring out for a negative one. No reset, no second core.
   A moment nearer than the loop takes to make is passed for the next of its
   kind. Several changes may wait, one for each kind of moment, a later one
   in place of an earlier for the same kind or boundary; each starts at its
   own. With nothing playing it is a load. 1 if asked, 0 for no such moment. */
int dl_player_change(dl_player *player, uint32_t seed, uint32_t moment, double crossfade);

/* When the next change waiting starts its loop, in seconds on the core's
   clock (frames rendered over the rate); -1 if none is worked out yet (it
   is once the next block is rendered). */
double dl_player_next_change(dl_player *player);

/* Make what is left of the loop asked for, now: for a host with no deadline
   (an offline render, a loading screen). It starts on the next block. */
void dl_player_ready(dl_player *player);

/* Render the next `frames` frames: `out` holds a pointer for each channel,
   each to `frames` floats. Any number of frames a call; the samples are the
   same whatever the split. 0, or -1 for a null player or channel. */
int dl_player_render(dl_player *player, float *const *out, size_t frames);

/* Rendering ahead (a host with no audio thread, as a web build): render in
   chunks of any size with dl_player_render; each carries on exactly where
   the last stopped, the same samples as straight through. The frames from
   the last rendered to the end of the pass playing, the chunk for a pass at
   a time; -1 while no loop plays (dl_player_ready makes one at once). */
int64_t dl_player_pass_frames(dl_player *player);

/* How long rendering the next `frames` frames will take, from the loop's
   voices, in microseconds on the reference host (WebAssembly in Node on the
   machine the core is measured on). Scale it to this device: time a chunk,
   divide by its estimate, multiply the next estimate by that. -1 for a null
   player. */
double dl_player_estimate(dl_player *player, uint64_t frames);

/* Pause: stop deciding steps; the loop stays loaded. And go on from where it
   stood, on the next block. */
void dl_player_stop(dl_player *player);
void dl_player_resume(dl_player *player);

/* The app's controls. How far each pass wanders (0 plays the loop as
   composed, 1 is the app's default, up to 2.4), from the next pass. A layer
   muted (DL_DRUMS to DL_TEXTURE) or brought back, until brought back, across
   loads. The tempo in beats a minute, from the next step (asked while a loop
   is being made, it starts at it). The listener's volume, 0 to 1 (0.85). */
void dl_player_wander(dl_player *player, double amount);
/* The salt on the wander's stream, from the next pass (queue item 92): send
   a new one each time a loop is played and it wanders differently each
   time; 0, the default, is the wander every code has always had. The same
   seed, request and salt make the same samples. */
void dl_player_wander_salt(dl_player *player, uint32_t salt);
void dl_player_mute(dl_player *player, uint32_t layer, int muted);
void dl_player_tempo(dl_player *player, double bpm);
void dl_player_volume(dl_player *player, double volume);

/* Where the player is: DL_IDLE, DL_LOADING, DL_WANDERING (its first pass
   being drawn) or DL_PLAYING. */
uint32_t dl_player_state(dl_player *player);

/* The loop's name into `out` (`size` bytes, NUL-terminated, cut short if it
   must be); returns the name's whole length. */
size_t dl_player_name(dl_player *player, char *out, size_t size);

/* Notes the core could not play as asked since the start (dropped, late or
   left out): 0, always, unless something is wrong. */
uint32_t dl_player_trouble(dl_player *player);

/* Queue item 94a: everything the app does, as a call or an event.
 *
 * A share code's edits, for the next load, change or re-roll (with the
 * vibes and the rest; dl_player_ask takes them as text too: "edit.roll.drums=
 * 1234", "edit.seed.bass=99", "edit.warmth=0.6", "edit.space=0.5",
 * "edit.wobble=0.4", "edit.bpm=88"): layer `layer`'s seed, re-rolled (always
 * played) with `rolled` not 0. NULL if it can be met, else why not. */
const char *dl_player_seed(dl_player *player, uint32_t layer, uint32_t seed, int rolled);

/* The loop playing made again with what was asked since the last load, in
   its place: the steps go on where they were (a layer re-rolled). With
   nothing playing it is a change now. */
void dl_player_revise(dl_player *player, uint32_t seed);

/* The tone from now: warmth, space and wobble, 0 to 1 each. */
void dl_player_tone(dl_player *player, double warmth, double space, double wobble);

/* How long the track plays, counted from now (or a new loop's first step),
   holding across loads: DL_ALWAYS (on and on), `count` whole passes
   (DL_PASSES) or `count` bars to the next phrase line (DL_BARS). 1 if it is
   a kind there is. And the step from the loop's load it ends at (-1 while
   it plays on); a change for DL_END starts its loop there. */
enum { DL_ALWAYS = 0, DL_PASSES = 1, DL_BARS = 2 };
int dl_player_track(dl_player *player, uint32_t kind, uint32_t count);
int64_t dl_player_track_end(dl_player *player);

/* What the player tells the host, each at the time it is heard (seconds on
   the core's clock), decided up to a lookahead before: take them after each
   render (dl_player_event, 1 while one was taken) and show each at its
   time. `number` is the step in its pass (DL_STEP, DL_PHRASE), the pass
   (DL_PASS; in a piece, its round), the section (DL_SECTION, a piece's), the
   seed (DL_LOOP: a new loop's first step; DL_REVISED: the loop made again
   in place), or 0 (DL_END: the track's end); `step` is from the loop's load.
   256 wait at most; dl_player_events_lost counts the ones lost. */
enum { DL_STEP = 0, DL_PASS_BEGINS = 1, DL_SECTION = 2, DL_PHRASE_LINE = 3, DL_LOOP = 4, DL_REVISED = 5, DL_TRACK_END = 6 };
typedef struct dl_event {
    double time;
    uint32_t kind;
    uint64_t number;
    uint64_t step;
} dl_event;
int dl_player_event(dl_player *player, dl_event *out);
uint32_t dl_player_events_lost(dl_player *player);

/* Layer `layer`'s grid, of the pass being decided (read it when a pass or a
   section begins, and show it from that event's time): the loudest note on
   each step of the layer's cycle, 0 for none, into `out` (`size` doubles;
   640 is always enough). Returns the cycle's steps, 0 if nothing is made or
   `out` is too short. */
size_t dl_player_grid(dl_player *player, uint32_t layer, double *out, size_t size);

/* The loop described, plain ASCII lines of name=value (seed, name, mix,
   moods, feel, vibes, root, scale, bpm, metre, bars, tone, layer_seeds and
   the rest: what a card, a cover and a share code show), into `out` (`size`
   bytes, NUL-terminated, cut short if it must be); returns its whole
   length. */
size_t dl_player_describe(dl_player *player, char *out, size_t size);

/* The loop as composed, note by note (a MIDI export's): how many notes layer
   `layer` has, and note `index` into `out` (1, or 0 past the last): its step
   from the loop's first, its length in steps, its velocity (0 for a note
   left out), a drum hit's offset off the step and instrument ("kick",
   "hat"...), and `count` pitches, MIDI note numbers. */
typedef struct dl_note {
    double step;
    double dur;
    double vel;
    double micro;
    uint32_t count;
    double notes[8];
    char drum[16];
} dl_note;
size_t dl_player_notes(dl_player *player, uint32_t layer);
int dl_player_note(dl_player *player, uint32_t layer, size_t index, dl_note *out);

#ifdef __cplusplus
}
#endif

#endif
