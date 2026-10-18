# Raw ears measurements

`pile-2-first-half-raw.tar.gz` (3.5 MB): every album.py output from Ears 6 except audio and stems. Six albums (I Care Because You Do, Ventolin, Donkey Rhubarb, Hangable Auto Bulb, GAK, Analogue Bubblebath 4), 128 files, one folder per album:

- `pick.json`: the picker's mix-level row for every track (all tracks, not just the deep pass) and the picks
- `<track>/events.json`, `techniques.json`, `theory.json`, `space.json`: the full per-track measurements the cards summarize (note groups with every fingerprint number, all sweeps, all chord and bass readings)
- `stems/<track>/split.json`: which separator ran and how long it took (the stems themselves are not kept; MDX-Net stems, about 6 GB, rebuild with `tools/ears/split.py`)
- `progress.log`, `card.md`: stage timings and the unedited draft card

Unpack with `tar xzf pile-2-first-half-raw.tar.gz -C SOMEWHERE`. Later piles can add their own archive here. The cards in `docs/ears/cards-pile-*.md` stay the readable record.

`pile-2-second-half-raw.tar.gz`: the same for Ears 7 (Demucs htdemucs stems, not MDX-Net). Five albums (Bradley's Beat, Bradley's Robot, Analogue Bubblebath 3, Caustic Window Compilation, Analogue Bubblebath 5), one folder per album, same layout as above (`pick.json`, `<track>/{events,techniques,theory,space,stem_check}.json`, `stems/<track>/split.json`, `progress.log`, `card.md`). `stem_check.json` is new since Ears 6b: each track's stem shares, band shares, strongest low lines, drum-onset grid and the summed-stems floor-drop runs. `split.json` here says `demucs-htdemucs` and the seconds it took. No audio, stems or joined wavs. The cards are in `docs/ears/cards-pile-2.md` (second half). Its `ears7-session/` folder (added 2026-10-18, when `~/ears7` was deleted) holds the album.py chain script and log, the working card drafts and the 20 s separator test. Unpack with `tar xzf pile-2-second-half-raw.tar.gz -C SOMEWHERE`.

`ears8-synthesis-raw.tar.gz` (29 KB): the first synthesis's measurements (Ears 8, 2026-10-18), behind `docs/ears/batch-5.md`. No audio: the render2.mjs index of every final render (sketch draws, floor drops, refused and asked notes per voice), `sketch_check.py`, `low_guard.py` and `stereo_guard.py` outputs, `kick_check.py` on Ears 7's record stems, the clip plan and timestamps, the corpus rates of each sketch, and every job list. Since 2026-10-18 (58 KB) it also holds `work/`: the session's helper scripts, render logs, selection and null checks and the clip timestamps, kept when `~/ears8` was deleted. Its `README.txt` lists the files.
