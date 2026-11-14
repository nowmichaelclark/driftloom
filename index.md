# The long pieces: no join a hole (102g, session 16b)

6 clips, 2:30 in all. Stereo Opus, 96 kbps, the app's wander amount (1), 44.1 kHz. Made by `tools/longform-clips.mjs`.

| Clip | Piece | Request | From | Seconds | Listen for |
|---|---|---|---|---|---|
| ta-muin-endless-205-now.opus | ta-muin, Endless, arcs | `1337654778 bars=endless,journey=arc` | 1:50 | 25 | "then alone" into 2:05, now (v169): nothing written: the loop plays on across the line |
| ta-muin-endless-205-v168.opus | ta-muin, Endless, arcs | `1337654778 bars=endless,journey=arc` | 1:50 | 25 | "then alone" into 2:05, as at v168: the tune alone for the last bar, the mix 20 to 40 dB under the loop there: a hole |
| ta-muin-endless-926-now.opus | ta-muin, Endless, arcs | `1337654778 bars=endless,journey=arc` | 9:11 | 25 | "then alone" into 9:26, now (v169): nothing written: the loop plays on across the line |
| ta-muin-endless-926-v168.opus | ta-muin, Endless, arcs | `1337654778 bars=endless,journey=arc` | 9:11 | 25 | "then alone" into 9:26, as at v168: the tune alone for the last bar, the mix 20 to 40 dB under the loop there: a hole |
| yi-yoar-endless-614-now.opus | yi-yoar, Endless, arcs | `819045617 bars=endless,journey=arc` | 5:59 | 25 | "then floor" into 6:14, now (v169): nothing written: the loop plays on across the line |
| yi-yoar-endless-614-v168.opus | yi-yoar, Endless, arcs | `819045617 bars=endless,journey=arc` | 5:59 | 25 | "then floor" into 6:14, as at v168: the kit and the bass out for the last half bar, the mix 20 to 40 dB under the loop there: a hole |

## Measured (wander 0, 44.1 kHz, each quarter-second against the loop as itself)

| Place | v168 (dB under the loop, from the join's start) | Now (v169) |
|---|---|---|
| ta-muin, "then alone" into 2:05 (124.0 to 125.75 s) | 26 21 27 23 32 30 38: 2.4 beats 25 dB under | 1 1 1 1 1 1 1: none |
| ta-muin, "then alone" into 9:26 (564.5 to 566.25 s) | 22 22 26 19 30 26 35 32: 2.4 beats | 1 1 1 1 1 1 1 1: none |
| yi-yoar, "then floor" into 6:14 (373.25 to 374.5 s) | 29 34 35 38: 1.6 beats | 3 0 1 1: none (0.07 beats, a frame) |

The same at the app's wander amount (1): none at all three.
