/* A loop from its seed to a WAV, through dlcore.h alone (docs/CORE.md,
 * promise 4; queue item 82): the tiny C host CI builds and holds to
 * `dlrender --seed`, line for line and byte for byte.
 *
 *     cc -O2 -std=c11 -Iinclude examples/c/seed.c target/release/libdlcore.a -o seed
 *     ./seed SEED [--seconds S] [--rate HZ] [--stereo] [--lite]
 *                 [--wander A] [--vibes ID,...] [--wav OUT.wav] [--ahead]
 *                 [--flac OUT.flac [--bits 16|24]]
 *
 * It prints the line dlrender prints: the loop's name, how it was played and
 * the hashes of its samples (FNV-1a over the float words, a channel each, and
 * over their bytes, interleaved), and writes the WAV dlrender writes (32-bit
 * float, from a little-endian host). With --ahead it renders as a host with
 * no audio thread does (queue item 87): a pass at a time, each in one call
 * (dl_player_pass_frames), its cost estimated first (dl_player_estimate,
 * written to stderr); the line and the WAV are the same. With --flac it also
 * writes the render as FLAC through the core's own encoder (queue item 104a,
 * dl_flac_*), 16 or 24 bits (--bits, 24 by default), its header patched at
 * the end: the same bytes as `dlrender --flac`. */

#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "dlcore.h"

#define CHUNK 512

static void put32(FILE *f, uint32_t x) {
  unsigned char b[4] = { x & 255, (x >> 8) & 255, (x >> 16) & 255, x >> 24 };
  fwrite(b, 1, 4, f);
}

static void put16(FILE *f, uint32_t x) {
  unsigned char b[2] = { x & 255, (x >> 8) & 255 };
  fwrite(b, 1, 2, f);
}

static uint32_t fnv(uint32_t h, float x) {
  uint32_t w;
  memcpy(&w, &x, 4);
  return (h ^ w) * 0x01000193u;
}

static uint64_t fnv64(uint64_t h, float x) {
  uint32_t w;
  memcpy(&w, &x, 4);
  for (int i = 0; i < 4; i++) h = (h ^ ((w >> (8 * i)) & 255)) * 0x100000001b3ull;
  return h;
}

int main(int argc, char **argv) {
  double seconds = 45, rate = 44100, wander = 1;
  uint32_t channels = 1;
  int full = 1;
  int ahead = 0;
  const char *wav = NULL, *vibes = NULL, *request = NULL, *flac_path = NULL;
  uint32_t bits = 24;
  if (argc < 2) {
    fprintf(stderr, "usage: seed SEED [--seconds S] [--rate HZ] [--stereo] [--lite] [--wander A] [--vibes ID,...] [--request TEXT] [--wav OUT.wav] [--ahead] [--flac OUT.flac [--bits 16|24]]\n");
    return 2;
  }
  uint32_t seed = (uint32_t)strtoul(argv[1], NULL, 10);
  for (int i = 2; i < argc; i++) {
    if (!strcmp(argv[i], "--stereo")) channels = 2;
    else if (!strcmp(argv[i], "--lite")) full = 0;
    else if (!strcmp(argv[i], "--ahead")) ahead = 1;
    else if (i + 1 < argc && !strcmp(argv[i], "--seconds")) seconds = atof(argv[++i]);
    else if (i + 1 < argc && !strcmp(argv[i], "--rate")) rate = atof(argv[++i]);
    else if (i + 1 < argc && !strcmp(argv[i], "--wander")) wander = atof(argv[++i]);
    else if (i + 1 < argc && !strcmp(argv[i], "--vibes")) vibes = argv[++i];
    else if (i + 1 < argc && !strcmp(argv[i], "--request")) request = argv[++i];
    else if (i + 1 < argc && !strcmp(argv[i], "--wav")) wav = argv[++i];
    else if (i + 1 < argc && !strcmp(argv[i], "--flac")) flac_path = argv[++i];
    else if (i + 1 < argc && !strcmp(argv[i], "--bits")) bits = (uint32_t)strtoul(argv[++i], NULL, 10);
    else {
      fprintf(stderr, "seed: no such option: %s\n", argv[i]);
      return 2;
    }
  }

  /* The player's memory: the host's, kept for its life. */
  size_t align = dl_player_align();
  size_t size = (dl_player_size() + align - 1) / align * align;
  void *memory = aligned_alloc(align, size);
  dl_player *p = memory ? dl_player_start(memory, size, rate, channels, full) : NULL;
  if (!p) {
    fprintf(stderr, "seed: no player at %g Hz, %u channels\n", rate, channels);
    return 1;
  }
  dl_player_wander(p, wander);
  /* The request as text ("glacial=0.7,bpm=70-85"); the vibes of --vibes are
     added to it. */
  if (request) {
    const char *why = dl_player_ask(p, request);
    if (why) {
      fprintf(stderr, "seed: can't do that: %s\n", why);
      return 1;
    }
  }
  if (vibes) {
    char ids[256];
    snprintf(ids, sizeof ids, "%s", vibes);
    for (char *id = strtok(ids, ","); id; id = strtok(NULL, ",")) {
      if (!dl_player_vibe(p, id)) {
        fprintf(stderr, "seed: no vibe '%s'\n", id);
        return 1;
      }
    }
  }
  dl_player_load(p, seed);
  dl_player_ready(p);

  size_t frames = (size_t)ceil(seconds * rate);
  FILE *f = NULL;
  if (wav) {
    f = fopen(wav, "wb");
    if (!f) {
      perror(wav);
      return 1;
    }
    uint32_t data = (uint32_t)(frames * channels * 4);
    fwrite("RIFF", 1, 4, f);
    put32(f, 36 + data);
    fwrite("WAVEfmt ", 1, 8, f);
    put32(f, 16);
    put16(f, 3); /* IEEE float */
    put16(f, channels);
    put32(f, (uint32_t)rate);
    put32(f, (uint32_t)rate * 4 * channels);
    put16(f, 4 * channels);
    put16(f, 32);
    fwrite("data", 1, 4, f);
    put32(f, data);
  }
  /* The encoder's memory, the host's too: a fixed size, however long. */
  FILE *ff = NULL;
  dl_flac *enc = NULL;
  void *flac_memory = NULL;
  size_t len = 0;
  if (flac_path) {
    size_t falign = dl_flac_align();
    size_t fsize = (dl_flac_size() + falign - 1) / falign * falign;
    flac_memory = aligned_alloc(falign, fsize);
    enc = flac_memory ? dl_flac_start(flac_memory, fsize, (uint32_t)rate, channels, bits) : NULL;
    ff = fopen(flac_path, "wb");
    if (!enc || !ff) {
      fprintf(stderr, "seed: no FLAC encoder for %s at %g Hz, %u bits\n", flac_path, rate, bits);
      return 1;
    }
    const uint8_t *head = dl_flac_bytes(enc, &len);
    fwrite(head, 1, len, ff);
  }
  static float small[2][CHUNK], small_both[2 * CHUNK];
  float *left = small[0], *right = small[1], *both = small_both;
  size_t room = CHUNK;
  float *out[2] = { left, right };
  double estimated = 0;
  int chunks = 0;
  uint32_t h[2] = { 0x811c9dc5u, 0x811c9dc5u };
  uint64_t all = 0xcbf29ce484222325ull;
  for (size_t done = 0, n = 0; done < frames; done += n) {
    n = frames - done < CHUNK ? frames - done : CHUNK;
    if (ahead) {
      /* The pass playing, whole, in one call: buffers as big as a pass. */
      int64_t pass = dl_player_pass_frames(p);
      n = pass > 0 && (size_t)pass < frames - done ? (size_t)pass : frames - done;
      if (n > room) {
        room = n;
        if (left != small[0]) {
          free(left);
          free(right);
          free(both);
        }
        left = malloc(room * sizeof(float));
        right = malloc(room * sizeof(float));
        both = malloc(2 * room * sizeof(float));
        if (!left || !right || !both) {
          fprintf(stderr, "seed: no memory for a pass\n");
          return 1;
        }
        out[0] = left;
        out[1] = right;
      }
      estimated += dl_player_estimate(p, n);
      chunks++;
    }
    if (dl_player_render(p, out, n) != 0) {
      fprintf(stderr, "seed: the render failed\n");
      return 1;
    }
    for (size_t i = 0; i < n; i++) {
      h[0] = fnv(h[0], left[i]);
      all = fnv64(all, left[i]);
      if (channels == 2) {
        h[1] = fnv(h[1], right[i]);
        all = fnv64(all, right[i]);
      }
      both[2 * i] = left[i];
      both[2 * i + 1] = right[i];
    }
    if (f) fwrite(channels == 2 ? both : left, 4 * channels, n, f);
    for (size_t at = 0; enc && at < n; at += DL_FLAC_BLOCK) {
      size_t take = n - at < DL_FLAC_BLOCK ? n - at : DL_FLAC_BLOCK;
      const float *planes[2] = { left + at, right + at };
      if (dl_flac_write(enc, planes, take) != 0) {
        fprintf(stderr, "seed: the FLAC encoder refused a block\n");
        return 1;
      }
      const uint8_t *bytes = dl_flac_bytes(enc, &len);
      fwrite(bytes, 1, len, ff);
    }
  }
  if (enc) {
    /* The last frame, then the finished header over the first. */
    uint8_t head[DL_FLAC_HEADER];
    const uint8_t *bytes;
    if (dl_flac_finish(enc) != 0 || dl_flac_header(enc, head) != 0) {
      fprintf(stderr, "seed: the FLAC encoder could not finish\n");
      return 1;
    }
    bytes = dl_flac_bytes(enc, &len);
    fwrite(bytes, 1, len, ff);
    fseek(ff, 0, SEEK_SET);
    fwrite(head, 1, DL_FLAC_HEADER, ff);
    fclose(ff);
    free(flac_memory);
  }
  if (f) fclose(f);
  if (ahead) fprintf(stderr, "seed: rendered ahead in %d chunks, estimated %.0f ms on the reference host\n", chunks, estimated / 1000);
  if (dl_player_trouble(p)) {
    fprintf(stderr, "seed: the core could not play %u notes as asked\n", dl_player_trouble(p));
    return 1;
  }

  char name[64];
  dl_player_name(p, name, sizeof name);
  printf("seed=%u name=%s quality=%s layout=%s seconds=%g rate=%g wander=%g", seed, name, full ? "full" : "lite",
         channels == 2 ? "stereo" : "mono", seconds, rate, wander);
  if (vibes) printf(" vibes=%s", vibes);
  if (channels == 2) printf(" frames=%zu left=%08x right=%08x", frames, h[0], h[1]);
  else printf(" frames=%zu out=%08x", frames, h[0]);
  printf(" hash=%016llx\n", (unsigned long long)all);
  free(memory);
  return 0;
}
