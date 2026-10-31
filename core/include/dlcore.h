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

/* Ask for what `request` says, as text, in place of what was asked since the
   last load: items split by commas, each a vibe ("glacial"), a vibe and its
   weight ("glacial=0.7") or the tempo ("bpm=77", "bpm=70-85"). */
const char *dl_player_ask(dl_player *player, const char *request);

/* Ask for the loop of `seed`, with what was asked since the last load. It
   is made a stage a block over the blocks rendered next (so no block's work
   spikes), then starts; a loop playing stops being decided at once, and
   what it has handed over plays out. */
void dl_player_load(dl_player *player, uint32_t seed);

/* Make what is left of the loop asked for, now: for a host with no deadline
   (an offline render, a loading screen). It starts on the next block. */
void dl_player_ready(dl_player *player);

/* Render the next `frames` frames: `out` holds a pointer for each channel,
   each to `frames` floats. Any number of frames a call; the samples are the
   same whatever the split. 0, or -1 for a null player or channel. */
int dl_player_render(dl_player *player, float *const *out, size_t frames);

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

#ifdef __cplusplus
}
#endif

#endif
