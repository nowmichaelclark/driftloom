"""Step 6 of the album card: width, by band and by stem.

Usage: python3 -I space.py STEM_DIR MIX_FILE OUT.json

For the mix and for each stem, in five bands (sub <120 Hz, low 120-500, mid 500-2k,
presence 2-5k, air >5k):

  width_db   side/mid power in dB. 0 dB = the two channels are unrelated (as wide as
             stereo gets without opposite polarity); -20 dB or lower = close to mono;
             above +3 dB = opposite polarity parts that vanish in mono.
  corr       left-right correlation (+1 mono, 0 unrelated, -1 opposite)
  pan        energy balance, -1 all left to +1 all right
  mono_loss  dB lost when summed to mono (0 for a mono part, 3 for unrelated channels; above 3 = parts that cancel)

and over time (2-s windows): the 10th, 50th, 90th percentile of the mix width, so a part
that is wide only in some places shows. Calibrated in cal_space.py: a known correlation,
a part panned to a known place, bands with different widths, a width that changes.
Reliable only where the band holds enough energy (the table says so: share_of_stem).
"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
import scipy.signal as sps

from cardlib import SR, load_stems, decode, save_json

BANDS = {"sub_lt120": (20, 120), "low_120_500": (120, 500), "mid_500_2k": (500, 2000),
         "pres_2k_5k": (2000, 5000), "air_gt5k": (5000, 11000)}
NFFT = 2048


def _stft(x):
    f, t, Z = sps.stft(x, fs=SR, nperseg=NFFT, noverlap=NFFT // 2, padded=False, boundary=None)
    return f, t, Z


def band_stats(L, R, f, lo, hi, cols=None):
    sel = (f >= lo) & (f < hi)
    l, r = L[sel], R[sel]
    if cols is not None:
        l, r = l[:, cols], r[:, cols]
    pl, pr = (np.abs(l) ** 2).sum(), (np.abs(r) ** 2).sum()
    cross = np.real((l * np.conj(r)).sum())
    pm = (np.abs(l + r) ** 2).sum() / 4
    ps = (np.abs(l - r) ** 2).sum() / 4
    tot = pl + pr
    if tot <= 1e-18:
        return None
    return {"width_db": float(10 * np.log10(max(ps, 1e-18) / max(pm, 1e-18))),
            "corr": float(cross / max(np.sqrt(pl * pr), 1e-18)),
            "pan": float((pr - pl) / tot),
            "mono_loss_db": float(10 * np.log10(max(tot / 2, 1e-18) / max(pm, 1e-18))),   # 0 mono, 3 unrelated, more = cancels
            "power": float(tot)}


def analyze_pair(st):
    """st: (2, n). Returns {band: stats} plus the whole-range stats."""
    f, t, L = _stft(st[0])
    _, _, R = _stft(st[1])
    out = {}
    all_p = 0.0
    for name, (lo, hi) in BANDS.items():
        s = band_stats(L, R, f, lo, hi)
        out[name] = s
        all_p += s["power"] if s else 0.0
    out["all"] = band_stats(L, R, f, 20, 11000)
    for name in BANDS:
        if out[name]:
            out[name]["share_of_stem"] = float(out[name]["power"] / max(all_p, 1e-18))
    return out, (f, t, L, R)


def width_over_time(L, R, f, t, win_s=2.0):
    hop_s = t[1] - t[0]
    k = max(1, int(win_s / hop_s))
    vals = []
    for i in range(0, L.shape[1] - k + 1, k):
        s = band_stats(L, R, f, 120, 11000, cols=slice(i, i + k))
        if s:
            vals.append(s["width_db"])
    if not vals:
        return {}
    v = np.array(vals)
    return {"windows": len(v), "p10": float(np.percentile(v, 10)), "p50": float(np.percentile(v, 50)),
            "p90": float(np.percentile(v, 90))}


def run(stem_dir, mix_file, out_json):
    stems = load_stems(stem_dir)
    mix = sum(stems.values())
    res = {"bands": list(BANDS)}
    r, (f, t, L, R) = analyze_pair(mix)
    res["mix"] = r
    res["mix_width_over_time"] = width_over_time(L, R, f, t)
    res["stems"] = {}
    total_p = sum(float((s ** 2).sum()) for s in stems.values())
    for name, st in stems.items():
        sr_, _ = analyze_pair(st)
        sr_["share_of_mix_power"] = float((st ** 2).sum() / max(total_p, 1e-18))
        res["stems"][name] = sr_
    save_json(out_json, res)
    return res


if __name__ == "__main__":
    run(sys.argv[1], sys.argv[2], sys.argv[3])
