/* Queue item 94a through dlcore.h alone: a layer re-rolled in its place, the
 * track's end and the change that waits for it, the events, the grid and the
 * loop's description, as a C host (a game) calls them. It checks what it
 * sees and exits 1 if anything is wrong; CI runs it (tools/seeds.sh --c).
 *
 *     cc -O2 -std=c11 -Iinclude examples/c/report.c target/release/libdlcore.a -lm -o report
 *     ./report */

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "dlcore.h"

#define RATE 44100.0
#define CHUNK 512

static int failures = 0;

static void check(const char *what, int ok) {
  if (!ok) {
    fprintf(stderr, "report: FAIL %s\n", what);
    failures++;
  }
}

int main(void) {
  size_t size = (dl_player_size() + dl_player_align() - 1) / dl_player_align() * dl_player_align();
  void *memory = aligned_alloc(dl_player_align(), size);
  dl_player *p = dl_player_start(memory, size, RATE, 1, 1);
  check("a player", p != NULL);
  if (!p) return 1;
  dl_player_load(p, 1063107040u);
  dl_player_ready(p);

  char about[4096];
  size_t n = dl_player_describe(p, about, sizeof about);
  check("a description", n > 0 && n < sizeof about && strstr(about, "seed=1063107040\n") != NULL);
  double grid[640];
  check("a grid", dl_player_grid(p, DL_CHORDS, grid, 640) > 0 && dl_player_grid(p, DL_CHORDS, grid, 4) == 0);

  dl_note note;
  size_t chords = dl_player_notes(p, DL_CHORDS);
  check("the notes", chords > 0 && dl_player_note(p, DL_CHORDS, 0, &note) == 1 && note.count > 0 && note.notes[0] > 20 &&
                     dl_player_note(p, DL_CHORDS, chords, &note) == 0);
  check("two passes", dl_player_track(p, DL_PASSES, 2) == 1 && dl_player_track(p, 9, 1) == 0);
  int64_t end = dl_player_track_end(p);
  check("an end", end > 0);
  check("a change for the end", dl_player_change(p, 663542962u, DL_END, DL_CROSSFADE) == 1);
  check("a seed edit", dl_player_seed(p, DL_DRUMS, 4242u, 1) == NULL && dl_player_seed(p, 7, 1, 0) != NULL);

  float buf[CHUNK];
  float *out[1] = { buf };
  dl_event e;
  uint64_t next_step = 0;
  int revised = 0, ended = 0, began = 0, steps_ok = 1;
  for (int chunk = 0; chunk < (int)(RATE * 60 / CHUNK) && !began; chunk++) {
    if (chunk == 200) {
      dl_player_seed(p, DL_DRUMS, 4242u, 1);
      dl_player_revise(p, 1063107040u);
    }
    check("a render", dl_player_render(p, out, CHUNK) == 0);
    while (dl_player_event(p, &e)) {
      if (e.kind == DL_STEP) {
        if (e.step != next_step) steps_ok = 0;
        next_step = e.step + 1;
      }
      if (e.kind == DL_REVISED) revised = e.step > 0;
      if (e.kind == DL_TRACK_END) ended = (int64_t)e.step == end;
      if (e.kind == DL_LOOP && e.number == 663542962u) began = ended;
    }
  }
  check("every step, once, in order", steps_ok);
  check("the re-roll kept its place", revised);
  check("the end, where it said", ended);
  check("the next loop on the end", began);
  check("none lost", dl_player_events_lost(p) == 0);
  check("no trouble", dl_player_trouble(p) == 0);
  dl_player_tone(p, 0.1, 0.2, 0.3);
  n = dl_player_describe(p, about, sizeof about);
  check("the tone told", strstr(about, "tone=0.1|0.2|0.3\n") != NULL);
  check("cut short, NUL-ended, its whole length", dl_player_describe(p, about, 8) == n && strlen(about) == 7);
  free(memory);
  if (!failures) printf("report: the calls and events of queue item 94a, as a C host sees them: all well\n");
  return failures ? 1 : 0;
}
