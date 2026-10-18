"""List tracks that look like the same audio on different albums.

Usage: python3 -I twins.py WORKDIR [--csv docs/ears/data.csv] [--artist TEXT]
                           [--seconds 3] [--lu 0.6]

Reads every WORKDIR/*/pick.json (the picker's mix-level rows) and, with --csv,
the earlier piles' rows (docs/ears/data.csv; --artist keeps only albums whose
name contains TEXT, so a different artist with a similar length does not show).
Two tracks on different albums are a probable twin when their lengths are
within --seconds (default 3), loudness within --lu (default 0.6 LU), centroid
within 5% and sub share within 0.03. Run it at the start of a session on the
new albums' rows, and again at the end: a twin gets one deep pass, in the card
that runs first. A probable twin is a lead, not a proof; listen to both.
"""
import csv, glob, json, os, sys


def rows_from_workdir(workdir):
    out = []
    for p in sorted(glob.glob(os.path.join(workdir, "*", "pick.json"))):
        d = json.load(open(p))
        for r in d.get("rows", []):
            if "error" in r or "skip" in r or not r.get("seconds"):
                continue
            out.append((d.get("album", os.path.basename(os.path.dirname(p))), r["track"], r))
    return out


def rows_from_csv(path, artist=None):
    out = []
    for r in csv.DictReader(open(path)):
        try:
            row = {k: float(r[k]) for k in ("seconds", "lufs", "centroid_hz", "sub_lt120")}
        except (ValueError, KeyError):
            continue
        album = r.get("album", "")
        if artist and artist.lower() not in album.lower() and artist.lower() not in r.get("source", "").lower():
            continue
        out.append((album, r.get("track", ""), row))
    return out


def main():
    a = sys.argv[1:]
    csv_path = artist = None
    secs, lu = 3.0, 0.6
    if "--csv" in a:
        i = a.index("--csv"); csv_path = a[i + 1]; del a[i:i + 2]
    if "--artist" in a:
        i = a.index("--artist"); artist = a[i + 1]; del a[i:i + 2]
    if "--seconds" in a:
        i = a.index("--seconds"); secs = float(a[i + 1]); del a[i:i + 2]
    if "--lu" in a:
        i = a.index("--lu"); lu = float(a[i + 1]); del a[i:i + 2]
    rows = rows_from_workdir(a[0])
    if csv_path:
        rows += rows_from_csv(csv_path, artist)
    found = 0
    for i, (al1, t1, r1) in enumerate(rows):
        for al2, t2, r2 in rows[i + 1:]:
            if al1 == al2:
                continue
            try:
                if (abs(r1["seconds"] - r2["seconds"]) <= secs and abs(r1["lufs"] - r2["lufs"]) <= lu
                        and abs(r1["centroid_hz"] - r2["centroid_hz"]) <= 0.05 * max(r1["centroid_hz"], 1)
                        and abs(r1["sub_lt120"] - r2["sub_lt120"]) <= 0.03):
                    found += 1
                    print(f"{al1}: {t1} ({r1['seconds']:.1f} s, {r1['lufs']:.1f} LUFS)  ~  "
                          f"{al2}: {t2} ({r2['seconds']:.1f} s, {r2['lufs']:.1f} LUFS)")
            except (KeyError, TypeError):
                continue
    print(f"{found} probable twin pair(s) across {len(rows)} tracks")


if __name__ == "__main__":
    main()
