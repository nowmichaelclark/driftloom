"""Calibration of space.py: a known correlation, a part panned to a known place, bands with
different widths, a width that changes in time, opposite polarity."""
import numpy as np
import scipy.signal as sps

from calsig import SR, RNG, string_note, place, pan_stereo, pan_expected
import space as SP


def band_noise(lo, hi, dur, seed):
    r = np.random.default_rng(seed)
    sos = sps.butter(4, [lo, hi], "bandpass", fs=SR, output="sos")
    return sps.sosfilt(sos, r.standard_normal(int(dur * SR)))


def pair_with_corr(rho, lo, hi, dur=20, seed=0):
    """L and R share a common part; the rest is independent. corr = a^2/(a^2+b^2)."""
    c = band_noise(lo, hi, dur, seed)
    n1 = band_noise(lo, hi, dur, seed + 100)
    n2 = band_noise(lo, hi, dur, seed + 200)
    a, b = np.sqrt(rho), np.sqrt(1 - rho)
    return np.stack([a * c + b * n1, a * c + b * n2])


def cal_width(row):
    for rho in (1.0, 0.9, 0.7, 0.5, 0.2, 0.0):
        st = pair_with_corr(rho, 200, 4000)
        o, _ = SP.analyze_pair(st)
        a2, b2 = rho, 1 - rho
        truth_w = 10 * np.log10(max(b2 / 2, 1e-9) / (a2 + b2 / 2)) if rho < 1 else -60.0
        m = o["all"]["width_db"]
        row("space", f"width_db at left-right correlation {rho}", round(truth_w, 2), max(m, -60.0), 1.0 if rho < 1 else 12.0, "dB")
        row("space", f"corr at left-right correlation {rho}", rho, o["all"]["corr"], 0.05, "")
    st = pair_with_corr(0.0, 200, 4000)
    st = np.stack([st[0], -st[0]])
    o, _ = SP.analyze_pair(st)
    row("space", "opposite polarity (R = -L): corr", -1.0, o["all"]["corr"], 0.02, "")
    row("space", "opposite polarity: mono loss above 20 dB", 1, int(o["all"]["mono_loss_db"] > 20), 0, "", f"{o['all']['mono_loss_db']:.0f} dB")
    row("space", "unrelated channels: mono loss 3 dB", 3.0, SP.analyze_pair(pair_with_corr(0.0, 200, 4000))[0]["all"]["mono_loss_db"], 0.4, "dB")


def cal_bands(row):
    # low mid band wide (independent), presence band mono, air band half-correlated
    y = pair_with_corr(0.0, 150, 450, seed=1) * 1.0
    y = y + np.stack([band_noise(2300, 3800, 20, 7)] * 2) * 1.0
    y = y + pair_with_corr(0.5, 6000, 10000, seed=3) * 1.0
    o, _ = SP.analyze_pair(y)
    row("space", "band width: 150-450 Hz independent -> 0 dB", 0.0, o["low_120_500"]["width_db"], 1.0, "dB",
        "side/mid of unrelated channels is 0 dB")
    row("space", "band width: 2-5 kHz mono -> below -25 dB", 1, int(o["pres_2k_5k"]["width_db"] < -25), 0, "", f"{o['pres_2k_5k']['width_db']:.1f} dB")
    exp = 10 * np.log10((0.5 / 2) / (0.5 + 0.5 / 2))
    row("space", "band width: 5-11 kHz half-correlated", round(exp, 2), o["air_gt5k"]["width_db"], 1.5, "dB")


def cal_leak(row):
    # a loud wide band next to a quiet mono band: the quiet band's reading is contaminated
    wide = pair_with_corr(0.0, 5200, 10000, seed=1) * 10.0
    mono = np.stack([band_noise(2100, 4800, 20, 7)] * 2)
    o, _ = SP.analyze_pair(wide + mono)
    row("space", "quiet mono band next to a band 20 dB louder and wide (skirt leakage)", -40.0, o["pres_2k_5k"]["width_db"], "limit", "dB",
        f"reads {o['pres_2k_5k']['width_db']:.0f} dB instead of mono; the table's share_of_stem says when a band is too quiet to trust")


def cal_pan(row):
    base = place([string_note(262, 2.0)] * 6, [0.5 + 2.2 * i for i in range(6)], 15.0) * 0.4
    for p in (-1.0, -0.5, 0.0, 0.3, 0.8, 1.0):
        st = pan_stereo(base, p)
        o, _ = SP.analyze_pair(st)
        row("space", f"pan {p:+.1f} (constant power): energy balance", pan_expected(p) if False else (np.sin((p + 1) * np.pi / 4) ** 2 - np.cos((p + 1) * np.pi / 4) ** 2),
            o["all"]["pan"], 0.03, "")


def cal_time(row):
    # first half mono, second half wide
    a = pair_with_corr(1.0, 200, 4000, dur=20, seed=5)
    b = pair_with_corr(0.0, 200, 4000, dur=20, seed=6)
    st = np.concatenate([a, b], axis=1)
    f, t = None, None
    o, (f, t, L, R) = SP.analyze_pair(st)
    w = SP.width_over_time(L, R, f, t)
    row("space", "width over time: p10 reads the mono half (below -30 dB)", -60.0, w["p10"], (-300, -30), "dB", f"p10 {w['p10']:.0f}")
    row("space", "width over time: p90 reads the wide half (unrelated = 0 dB)", 0.0, w["p90"], 1.0, "dB")


TESTS = {"space_leak": cal_leak, "space_width": cal_width, "space_bands": cal_bands, "space_pan": cal_pan, "space_time": cal_time}
