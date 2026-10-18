"""One command per album: pick tracks, split, measure, write the draft card.

Usage:
  python3 -I album.py ALBUM_DIR WORKDIR [--stages pick,split,events,techniques,theory,space,card]
                      [--only "track name"] [--workers 2] [--no-vocals]

Everything is cached in WORKDIR/<album>/, so a stage is skipped when its
output exists (delete the file to redo it). Long: run it in the background
(nohup) and poll WORKDIR/<album>/progress.log; a shell command stops after
10 minutes.

  WORKDIR/<album>/pick.json                  step 1 (pick.py)
  WORKDIR/<album>/stems/<track>/*.wav        step 2 (split.py)
  WORKDIR/<album>/<track>/events.json        step 3 (events.py)
  WORKDIR/<album>/<track>/techniques.json    step 4 (techniques.py)
  WORKDIR/<album>/<track>/theory.json        step 5 (theory.py)
  WORKDIR/<album>/<track>/space.json         step 6 (space.py)
  WORKDIR/<album>/<track>/stem_check.json    step 7 (check_stems.py: void flags, drum grid)
  WORKDIR/<album>/card.md                    step 8 (card.py)

--no-vocals skips the MDX-Net vocals model on instrumental records (only matters
when Demucs is unavailable).
"""
import os, re, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from cardlib import load_json, save_json

ALL = ["pick", "split", "events", "techniques", "theory", "space", "check", "card"]


def slug(s):
    return re.sub(r"[^A-Za-z0-9]+", "_", s).strip("_")[:60]


def log(work, msg):
    line = time.strftime("%H:%M:%S ") + msg
    print(line, flush=True)
    with open(os.path.join(work, "progress.log"), "a") as f:
        f.write(line + "\n")


def main():
    a = sys.argv[1:]
    stages, only, workers = ALL, None, 2
    skip_vocals = "--no-vocals" in a
    if skip_vocals:
        a.remove("--no-vocals")
    if "--stages" in a:
        i = a.index("--stages"); stages = a[i + 1].split(","); del a[i:i + 2]
    if "--only" in a:
        i = a.index("--only"); only = a[i + 1]; del a[i:i + 2]
    if "--workers" in a:
        i = a.index("--workers"); workers = int(a[i + 1]); del a[i:i + 2]
    album_dir, workdir = a[0], a[1]
    album = os.path.basename(album_dir.rstrip("/"))
    work = os.path.join(os.path.abspath(workdir), slug(album))
    os.makedirs(work, exist_ok=True)

    pick_path = os.path.join(work, "pick.json")
    if "pick" in stages and not os.path.exists(pick_path):
        import pick
        log(work, "pick: analyzing every track")
        t0 = time.time()
        tracks = pick.group_tracks(album_dir, os.path.join(work, "joined"))
        jobs = sorted(tracks.items())
        if workers > 1:
            from multiprocessing import Pool
            with Pool(workers) as p:
                rows = p.map(pick._one, jobs, chunksize=1)
        else:
            rows = [pick._one(j) for j in jobs]
        rep = pick.choose(rows)
        rep["album"], rep["paths"], rep["rows"] = album, tracks, rows
        save_json(pick_path, rep)
        log(work, f"pick: done in {time.time() - t0:.0f} s; typical {rep.get('typical')}; unlike {rep.get('unlike')}")
    rep = load_json(pick_path)
    picks = [p for p in rep["picks"] if only is None or only in p]

    for track in picks:
        src = rep["paths"][track]
        tdir = os.path.join(work, slug(track))
        sdir = os.path.join(work, "stems", slug(track))
        os.makedirs(tdir, exist_ok=True)
        if "split" in stages and not os.path.exists(os.path.join(sdir, "split.json")):
            import split
            log(work, f"split: {track}")
            info = split.split(src, sdir, skip_vocals=skip_vocals)
            log(work, f"split: {track} -> {info['mode']}, {info['audio_seconds']} s of audio in {info['wall_seconds']} s")
        if not os.path.exists(os.path.join(sdir, "drums.wav")):
            continue
        if "events" in stages and not os.path.exists(os.path.join(tdir, "events.json")):
            import events
            log(work, f"events: {track}")
            t0 = time.time()
            events.run(sdir, os.path.join(tdir, "events.json"))
            log(work, f"events: done in {time.time() - t0:.0f} s")
        if "techniques" in stages and not os.path.exists(os.path.join(tdir, "techniques.json")):
            import techniques
            log(work, f"techniques: {track}")
            t0 = time.time()
            techniques.run(sdir, src, os.path.join(tdir, "techniques.json"))
            log(work, f"techniques: done in {time.time() - t0:.0f} s")
        if "theory" in stages and not os.path.exists(os.path.join(tdir, "theory.json")):
            import theory
            log(work, f"theory: {track}")
            t0 = time.time()
            theory.run(sdir, src, os.path.join(tdir, "theory.json"))
            log(work, f"theory: done in {time.time() - t0:.0f} s")
        if "space" in stages and not os.path.exists(os.path.join(tdir, "space.json")):
            import space
            log(work, f"space: {track}")
            t0 = time.time()
            space.run(sdir, src, os.path.join(tdir, "space.json"))
            log(work, f"space: done in {time.time() - t0:.0f} s")
        if "check" in stages and not os.path.exists(os.path.join(tdir, "stem_check.json")):
            import check_stems
            r = check_stems.report(sdir)
            r.pop("runs", None)
            save_json(os.path.join(tdir, "stem_check.json"), r)
            log(work, f"check: {track}: " + ("; ".join(r["flags"]) or "no flags"))
    if "card" in stages:
        import card
        card.write_album(work, os.path.join(work, "card.md"))
        log(work, "card: written")


if __name__ == "__main__":
    main()
