"""Signal makers for the calibration tests (numpy only; nothing is read from disk)."""
import numpy as np
from cardlib import SR

RNG = np.random.default_rng(7)


def t_(dur, sr=SR):
    return np.arange(int(dur * sr)) / sr


def env_ad(dur, attack, decay_db_s, sr=SR):
    t = t_(dur, sr)
    a = np.clip(t / max(attack, 1e-4), 0, 1)
    return a * 10 ** (-decay_db_s * np.maximum(t - attack, 0) / 20)


def partials(f0, ratios, amps, dur, sr=SR, phase=None):
    t = t_(dur, sr)
    y = np.zeros_like(t)
    for i, (r, a) in enumerate(zip(ratios, amps)):
        if f0 * r < sr / 2 - 200:
            y += a * np.sin(2 * np.pi * f0 * r * t + (0 if phase is None else phase[i]))
    return y


def fade(y, secs=0.04, sr=SR):
    n = int(secs * sr)
    y = y.copy()
    y[-n:] *= np.linspace(1, 0, n)
    return y


def string_note(f0, dur, attack=0.01, decay_db_s=12):
    ks = np.arange(1, 11)
    return fade(partials(f0, ks, 1.0 / ks, dur) * env_ad(dur, attack, decay_db_s))


def bell_note(f0, dur, attack=0.003, decay_db_s=14):
    r = [1, 2.756, 5.404, 8.933, 13.34]
    a = [1, 0.6, 0.4, 0.25, 0.15]
    return fade(partials(f0, r, a, dur) * env_ad(dur, attack, decay_db_s))


def place(notes, times, total, sr=SR):
    y = np.zeros(int(total * sr))
    for n, t in zip(notes, times):
        i = int(t * sr)
        y[i:i + len(n)] += n[: len(y) - i]
    return y


def pan_stereo(y, p):
    """Constant-power pan, p in -1..1 -> (2, n)."""
    th = (p + 1) * np.pi / 4
    return np.stack([y * np.cos(th), y * np.sin(th)])


def pan_expected(p):
    th = (p + 1) * np.pi / 4
    c, s = np.cos(th), np.sin(th)
    return (s - c) / (s + c)


