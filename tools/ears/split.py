"""Split a track into stems: drums, bass, other, vocals.

Usage: python3 -I split.py IN OUT_DIR [--mode auto|demucs|hpss] [--seconds N]

auto (default): Demucs htdemucs on the CPU when it is installed and the
weights load, otherwise harmonic/percussive separation plus band splits.
Writes OUT_DIR/{drums,bass,other,vocals}.wav (44.1 kHz stereo) and
OUT_DIR/split.json with the mode used and the time taken, so a card says
which separator its stems came from.

Fallback (hpss), described honestly: percussive part -> "drums" (this
includes any transient, not only drums); harmonic part below 200 Hz ->
"bass"; harmonic part above 200 Hz -> "other"; "vocals" is left silent
(no separation of voices without a model). Per-stem measures taken from the
fallback are about bands and transients, not about instruments.
"""
import json, os, shutil, subprocess, sys, tempfile, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))  # python3 -I drops the script folder
import numpy as np
import soundfile as sf

from cardlib import decode


def demucs_split(src, out_dir, seconds=None):
    tmp = tempfile.mkdtemp(prefix="demucs_")
    wav = os.path.join(tmp, "in.wav")
    cmd = ["ffmpeg", "-nostdin", "-v", "error", "-y", "-i", src, "-ar", "44100", "-ac", "2"]
    if seconds:
        cmd += ["-t", str(seconds)]
    subprocess.run(cmd + [wav], check=True)
    t0 = time.time()
    subprocess.run([sys.executable, "-m", "demucs", "-n", "htdemucs", "-d", "cpu",
                    "-o", os.path.join(tmp, "out"), wav], check=True,
                   stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    took = time.time() - t0
    src_dir = os.path.join(tmp, "out", "htdemucs", "in")
    os.makedirs(out_dir, exist_ok=True)
    for s in ("drums", "bass", "other", "vocals"):
        shutil.move(os.path.join(src_dir, s + ".wav"), os.path.join(out_dir, s + ".wav"))
    n = sf.info(os.path.join(out_dir, "drums.wav")).frames
    shutil.rmtree(tmp, ignore_errors=True)
    return took, n / 44100


def hpss_split(src, out_dir, seconds=None):
    import librosa
    from scipy.signal import butter, sosfiltfilt
    sr = 44100
    t0 = time.time()
    x = decode(src, sr, 2, dur=seconds)
    os.makedirs(out_dir, exist_ok=True)
    # HPSS on the mid channel gives a mask; apply it to both channels.
    S = [librosa.stft(c, n_fft=2048, hop_length=512) for c in x]
    mid = np.abs(S[0] + S[1]) / 2
    H, P = librosa.decompose.hpss(mid, margin=2.0, kernel_size=(31, 31))
    eps = 1e-9
    mh = H ** 2 / (H ** 2 + P ** 2 + eps)
    perc = [librosa.istft(s * (1 - mh), hop_length=512, length=x.shape[1]) for s in S]
    harm = [librosa.istft(s * mh, hop_length=512, length=x.shape[1]) for s in S]
    lo = butter(4, 200, "lowpass", fs=sr, output="sos")
    hi = butter(4, 200, "highpass", fs=sr, output="sos")
    bass = np.stack([sosfiltfilt(lo, c) for c in harm])
    other = np.stack([sosfiltfilt(hi, c) for c in harm])
    stems = {"drums": np.stack(perc), "bass": bass, "other": other,
             "vocals": np.zeros_like(bass)}
    for k, v in stems.items():
        sf.write(os.path.join(out_dir, k + ".wav"), v.T.astype(np.float32), sr, subtype="PCM_16")
    return time.time() - t0, x.shape[1] / sr


def split(src, out_dir, mode="auto", seconds=None):
    used, note = None, ""
    if mode in ("auto", "demucs"):
        try:
            import demucs  # noqa: F401
            took, dur = demucs_split(src, out_dir, seconds)
            used = "demucs-htdemucs"
        except Exception as e:  # install or weights missing, or it crashed
            if mode == "demucs":
                raise
            note = f"demucs failed ({type(e).__name__}); fell back to hpss"
    if used is None:
        took, dur = hpss_split(src, out_dir, seconds)
        used = "hpss+bands"
    info = {"mode": used, "audio_seconds": round(dur, 1), "wall_seconds": round(took, 1),
            "speed_x_realtime": round(dur / max(took, 1e-9), 2), "note": note, "source": os.path.basename(src)}
    with open(os.path.join(out_dir, "split.json"), "w") as f:
        json.dump(info, f, indent=1)
    return info


if __name__ == "__main__":
    a = sys.argv[1:]
    mode, secs = "auto", None
    if "--mode" in a:
        i = a.index("--mode"); mode = a[i + 1]; del a[i:i + 2]
    if "--seconds" in a:
        i = a.index("--seconds"); secs = float(a[i + 1]); del a[i:i + 2]
    print(split(a[0], a[1], mode, secs))
