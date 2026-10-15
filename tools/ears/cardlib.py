"""Shared helpers for the album-card tools (Ears 3).

Everything here is small on purpose: decoding, stem loading, time strings,
a few signal helpers. The measures live in events.py, techniques.py,
theory.py and space.py; calibrate.py proves each one on test signals.
"""
import json, os, subprocess
import numpy as np

SR = 22050            # analysis rate for everything except space (stereo stems)
STEMS = ("drums", "bass", "other", "vocals")


def decode(path, sr=SR, channels=1, start=None, dur=None):
    """Decode any audio file to float32 with ffmpeg. channels=2 gives (2, n)."""
    cmd = ["ffmpeg", "-nostdin", "-v", "error"]
    if start:
        cmd += ["-ss", str(start)]
    if dur:
        cmd += ["-t", str(dur)]
    cmd += ["-i", path, "-f", "f32le", "-ac", str(channels), "-ar", str(sr), "-"]
    raw = subprocess.run(cmd, capture_output=True, check=True).stdout
    x = np.frombuffer(raw, dtype=np.float32)
    return x.reshape(-1, channels).T.copy() if channels > 1 else x.copy()


def load_stems(stem_dir, sr=SR):
    """Stereo stems as {name: (2, n) float32} at sr. Missing stems are skipped."""
    out = {}
    for s in STEMS:
        p = os.path.join(stem_dir, s + ".wav")
        if os.path.exists(p):
            out[s] = decode(p, sr, 2)
    return out


def mono(x):
    return x.mean(axis=0) if x.ndim == 2 else x


def db(x, floor=1e-12):
    return 10 * np.log10(np.maximum(x, floor))


def mmss(t):
    t = max(0.0, float(t))
    return f"{int(t // 60)}:{t % 60:04.1f}"


def rms_env(y, win, hop):
    """RMS envelope in dB, frame k covers y[k*hop : k*hop+win] (cumulative sums, no big arrays)."""
    n = max(0, (len(y) - win) // hop + 1)
    if n == 0:
        return np.array([-120.0])
    cs = np.concatenate([[0.0], np.cumsum(np.asarray(y, np.float64) ** 2)])
    k = hop * np.arange(n)
    return 10 * np.log10(np.maximum((cs[k + win] - cs[k]) / win, 1e-12))


def smooth(x, k):
    k = max(1, int(k))
    if k == 1 or len(x) < k:
        return np.asarray(x, float)
    pad = np.pad(np.asarray(x, float), (k // 2, k - 1 - k // 2), mode="edge")
    return np.convolve(pad, np.ones(k) / k, mode="valid")


def runs(mask, min_len=1):
    """[(start, end_exclusive)] of True runs at least min_len long."""
    out, s = [], None
    for i, m in enumerate(list(mask) + [False]):
        if m and s is None:
            s = i
        elif not m and s is not None:
            if i - s >= min_len:
                out.append((s, i))
            s = None
    return out


def to_py(o):
    """Make numpy results JSON-safe."""
    if isinstance(o, dict):
        return {str(k): to_py(v) for k, v in o.items()}
    if isinstance(o, (list, tuple)):
        return [to_py(v) for v in o]
    if isinstance(o, np.ndarray):
        return to_py(o.tolist())
    if isinstance(o, (np.floating, float)):
        return None if not np.isfinite(o) else round(float(o), 4)
    if isinstance(o, np.integer):
        return int(o)
    if isinstance(o, np.bool_):
        return bool(o)
    return o


def save_json(path, obj):
    with open(path, "w") as f:
        json.dump(to_py(obj), f, indent=1)


def load_json(path):
    with open(path) as f:
        return json.load(f)


# Pitch helpers ---------------------------------------------------------------
NOTE_NAMES = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"]


def hz_to_midi(f):
    return 69 + 12 * np.log2(np.maximum(f, 1e-9) / 440.0)


def midi_to_hz(m):
    return 440.0 * 2 ** ((np.asarray(m, float) - 69) / 12)


def note_name(midi):
    m = int(round(midi))
    return f"{NOTE_NAMES[m % 12]}{m // 12 - 1}"
