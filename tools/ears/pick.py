"""Step 1 of the album card: choose the tracks for the deep pass.

Usage: python3 -I pick.py ALBUM_DIR OUT.json [--workers 2] [--keep-wav DIR]

Every track gets analyze.py's mix-level numbers (tracks cut into "- partNN"
pieces by driftloom-ears.sh are joined first). From those:

  * the most typical track = the one with the smallest average distance to
    the others (a medoid);
  * the two or three least like the rest = the ones farthest from the rest
    (3 when the album has 12 or more usable tracks, otherwise 2), chosen so
    two near-twins are not both picked;
  * an EP (8 usable tracks or fewer) = every track.

Distance is over robust z-scores of: the five energy bands, tail (dB a
second), pulse, width (side/mid dB) and chord changes a minute. Tracks under
45 s are listed but never picked for the deep pass (too short for sections
or chords); they still feed "For later" from their mix numbers.
"""
import json, os, re, subprocess, sys, tempfile
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np

FEATURES = ["sub_lt120", "low_120_500", "mid_500_2k", "pres_2k_5k", "air_gt5k",
            "tail_db_per_s", "pulse", "width_db", "chord_changes_per_min"]
MIN_DEEP_S = 45.0
EP_MAX = 8


def group_tracks(album_dir, wav_dir):
    """{track name: file path}; 'X - part00/part01' pieces are joined into one wav."""
    album_dir = os.path.abspath(album_dir)
    wav_dir = os.path.abspath(wav_dir)
    files = sorted(f for f in os.listdir(album_dir) if f.lower().endswith((".opus", ".mp3", ".flac", ".wav", ".m4a", ".ogg")))
    groups = {}
    for f in files:
        m = re.match(r"(.*) - part(\d+)\.[A-Za-z0-9]+$", f)
        key = m.group(1) if m else os.path.splitext(f)[0]
        groups.setdefault(key, []).append((int(m.group(2)) if m else 0, f))
    out = {}
    for key, parts in groups.items():
        parts.sort()
        if len(parts) == 1:
            out[key] = os.path.join(album_dir, parts[0][1])
            continue
        os.makedirs(wav_dir, exist_ok=True)
        lst = os.path.join(wav_dir, re.sub(r"\W+", "_", key) + ".txt")
        with open(lst, "w") as fh:
            for _, f in parts:
                fh.write("file '%s'\n" % os.path.join(album_dir, f).replace("'", "'\\''"))
        wav = os.path.join(wav_dir, re.sub(r"\W+", "_", key) + ".wav")
        subprocess.run(["ffmpeg", "-nostdin", "-v", "error", "-y", "-f", "concat", "-safe", "0", "-i", lst,
                        "-ar", "44100", "-ac", "2", wav], check=True)
        out[key] = wav
    return out


def _one(args):
    name, path = args
    import analyze
    row = analyze.analyze(path)
    row["track"] = name
    return row


def find_twins(rows):
    """Pairs of tracks that are probably the same audio: length within 3 s, loudness within 0.3 LU,
    centroid within 2% and the sub share within 0.02. Returns [(first, later)]."""
    ok = [r for r in rows if "error" not in r and "skip" not in r and r.get("seconds")]
    pairs = []
    for i, a in enumerate(ok):
        for b in ok[i + 1:]:
            try:
                if (abs(a["seconds"] - b["seconds"]) <= 3 and abs(a["lufs"] - b["lufs"]) <= 0.3
                        and abs(a["centroid_hz"] - b["centroid_hz"]) <= 0.02 * max(a["centroid_hz"], 1)
                        and abs(a["sub_lt120"] - b["sub_lt120"]) <= 0.02):
                    pairs.append((a["track"], b["track"]))
            except (KeyError, TypeError):
                continue
    return pairs


def choose(rows, n_far=None):
    """rows: analyze rows. Returns the pick report (no audio needed)."""
    twins = find_twins(rows)
    later = {b for _, b in twins}
    ok = [r for r in rows if "error" not in r and "skip" not in r and r.get("seconds", 0) >= MIN_DEEP_S
          and r["track"] not in later]
    report = {"usable": [r["track"] for r in ok],
              "duplicates": [list(p) for p in twins],
              "too_short_or_silent": [r["track"] for r in rows if r not in ok and r["track"] not in later]}
    if not ok:
        report["picks"] = []
        return report
    if len(ok) <= EP_MAX:
        report["rule"] = f"EP or short record ({len(ok)} usable tracks): every track"
        report["typical"] = None
        report["unlike"] = []
        report["picks"] = [r["track"] for r in ok]
        return report
    X = np.array([[r.get(k) if r.get(k) is not None else np.nan for k in FEATURES] for r in ok], float)
    med = np.nanmedian(X, axis=0)
    X = np.where(np.isnan(X), med, X)
    mad = np.median(np.abs(X - np.median(X, axis=0)), axis=0) * 1.4826
    sd = np.where(mad < 1e-6, X.std(axis=0) + 1e-6, mad)
    Z = (X - np.median(X, axis=0)) / sd
    D = np.linalg.norm(Z[:, None, :] - Z[None, :, :], axis=2)
    mean_d = D.sum(axis=1) / (len(ok) - 1)
    typical = int(np.argmin(mean_d))
    n_far = n_far or (3 if len(ok) >= 12 else 2)
    picked = [typical]
    twin = 0.5 * float(np.median(D[np.triu_indices(len(ok), 1)]))
    far = []
    for i in np.argsort(-mean_d):
        i = int(i)
        if i in picked:
            continue
        if min(D[i, j] for j in picked) < twin:
            continue
        picked.append(i)
        far.append(i)
        if len(far) == n_far:
            break
    report.update({
        "rule": f"{len(ok)} usable tracks: the most typical plus the {len(far)} least like the rest",
        "typical": ok[typical]["track"],
        "unlike": [ok[i]["track"] for i in far],
        "picks": [ok[i]["track"] for i in picked],
        "distance_mean": {ok[i]["track"]: round(float(mean_d[i]), 2) for i in range(len(ok))},
        "twin_limit": round(twin, 2),
        "features": FEATURES,
        "z": {ok[i]["track"]: [round(float(v), 2) for v in Z[i]] for i in range(len(ok))},
    })
    return report


def main():
    a = sys.argv[1:]
    workers = 2
    wav_dir = None
    if "--workers" in a:
        i = a.index("--workers"); workers = int(a[i + 1]); del a[i:i + 2]
    if "--keep-wav" in a:
        i = a.index("--keep-wav"); wav_dir = a[i + 1]; del a[i:i + 2]
    album_dir, out = a[0], a[1]
    wav_dir = wav_dir or tempfile.mkdtemp(prefix="pick_")
    tracks = group_tracks(album_dir, wav_dir)
    jobs = sorted(tracks.items())
    if workers > 1:
        from multiprocessing import Pool
        with Pool(workers) as p:
            rows = p.map(_one, jobs, chunksize=1)
    else:
        rows = [_one(j) for j in jobs]
    rep = choose(rows)
    rep["album"] = os.path.basename(album_dir.rstrip("/"))
    rep["paths"] = {k: v for k, v in tracks.items()}
    rep["rows"] = rows
    with open(out, "w") as f:
        json.dump(rep, f, indent=1, default=lambda o: float(o) if isinstance(o, np.floating) else int(o))
    print("typical:", rep.get("typical"), "| unlike:", rep.get("unlike"), "| picks:", rep["picks"])


if __name__ == "__main__":
    main()
