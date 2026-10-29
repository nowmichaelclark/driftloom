# Done

One line for each PR that merged, newest last: the PR number, the version, the
commit preview URL, and what the ears should listen for. Add yours when your PR
merges (`docs/QUEUE.md`, standing rules). Entries before #277 are in
`docs/archive/done.md`.

- #277, the cold start (item 76, v124), preview URL: the Workers Builds check
  on #277 (its link carries no hash I can read from the session): until a note
  has reached the core, the first notes are planned no closer than the
  device's output latency and never less than 250 ms, so the first loop after
  the app starts keeps its first notes (the Moto lost six). Listen for sound
  that starts up to a quarter of a second after Play on the first loop after
  the app starts (and after a Next in its first moments), and for no missing
  first notes. Tester: close the app, open it, Play, let the first loop end,
  copy Diagnostics, on full and on lite: `late: 0`; any late note says
  `[margin]` or `[measured]`. No album loop (nothing in the music changed).
- #280, the vibes to 88 (item 76b, v125), preview URL:
  <https://19b7a2e6-driftloom.nowmichaelclark.workers.dev> (commit 1fa8bf2):
  twenty-one new vibes, soft, energetic, silly and comfy gone, codes made
  before v125 may play other vibes. Listen to each new vibe alone, above all
  glacial (the arc in most of its loops, deep, 5-8 minute swings; 8-12 with
  still, droning or another of its neighbors), droning, moonlit and mellow
  (no kit) against chill, groovy, pulsing and cruising (a beat), and the four
  pairs side by side: foggy and misty, experimental and strange, airy and
  breezy, mellow and chill. Lively should drive as hard as energetic did,
  restful should sound like a lullaby. Loops without vibes are unchanged.
- #282, the bass level (item 76c, v126), preview URL:
  <https://6fee2058-driftloom.nowmichaelclark.workers.dev> (commit 3383d71):
  the bass layer is +3 dB (by ear, K-weighted) in loops that play a kick (a
  soft kick or the hand kit's frame drum count), today's level in loops
  without one; rhodesbass (an FM voice) stays as it was. Listen to dust #7
  and noan-nou (corpus seed 20261009 and the baseline's seed 1, 0:20 on) for
  the fuller low end, and to dust #42 (no kit) for no change at all. A kit
  or bass that comes in late (lateFloor) or drops out (floorDrop) keeps its
  loop's level throughout. Loops without a kick are unchanged to the bit.
