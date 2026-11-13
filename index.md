# The long pieces: what changed (102f, session 16)

Replaces session 15's set (heard and kept by Mikey, 2026-11-14). Only what session 16 changed: the two holes brain B3 measured on v164, each excerpt now (main at v165) and as at v164 (88c98be), not blind and not level-matched: listen for a chord, or a breath with the chords and bass in it, where there was near silence. Rendered by `node tools/longform-clips.mjs OUTDIR --fixes DLRENDER_AT_V164`. Delete this branch after his check.

6 clips, 2:40 in all. Stereo Opus, 96 kbps, the app's wander amount (1), 44.1 kHz. Made by `tools/longform-clips.mjs`.

| Clip | Piece | Request | From | Seconds | Listen for |
|---|---|---|---|---|---|
| fei-no-256-pivot-now.opus | fei-no, 256 bars, one arc | `2158762586 bars=256,journey=arc` | 5:15 | 25 | the key's line into the peak (5:27), now (v165): a chord both keys share struck at 5:25 and held across it |
| fei-no-256-pivot-v164.opus | fei-no, 256 bars, one arc | `2158762586 bars=256,journey=arc` | 5:15 | 25 | the key's line into the peak (5:27), as at v164: the chord struck at nothing: a bar near silence before the line |
| fei-no-256-into-close-now.opus | fei-no, 256 bars, one arc | `2158762586 bars=256,journey=arc` | 7:55 | 30 | the stretched return into the close (8:10), now (v165): a breath, half a bar from 8:09, the chords and the bass to it |
| fei-no-256-into-close-v164.opus | fei-no, 256 bars, one arc | `2158762586 bars=256,journey=arc` | 7:55 | 30 | the stretched return into the close (8:10), as at v164: "the tune alone" over a tune that, stretched, plays nothing there: three seconds near silence |
| loa-yer-1024-pivot-now.opus | loa-yer, 1,024 bars, adventurous (the vibe draws the arc) | `2975389383 bars=1024,adventurous=0.8` | 3:44 | 25 | the key's line into theme B (3:56), now (v165): a chord both keys share struck at 3:54 and held across it |
| loa-yer-1024-pivot-v164.opus | loa-yer, 1,024 bars, adventurous (the vibe draws the arc) | `2975389383 bars=1024,adventurous=0.8` | 3:44 | 25 | the key's line into theme B (3:56), as at v164: the chord struck at nothing: the kit and the bass play on, the chords missing for a bar |
