"""Calibration of theory.py: chords (with the old counter alongside), key and mode,
tempo / bar length / swing / cross-rhythm, bass type, lead line, repetition.

Every signal is made from nothing with numpy; the answer is known by construction.
"""
import numpy as np
import scipy.signal as sps

from calsig import SR, RNG, partials, env_ad, string_note, place, fade, t_
from cardlib import NOTE_NAMES, midi_to_hz
import theory as TH


# --- helpers ---------------------------------------------------------------------------
def tone(midi, dur, nh=4, amp=0.1, attack=0.01, decay_db_s=0.0):
    f = midi_to_hz(midi)
    ks = np.arange(1, nh + 1)
    y = partials(f, ks, 1.0 / ks, dur) * amp
    y = y * env_ad(dur, attack, decay_db_s)
    return fade(y, min(0.03, dur / 4))


def chord_sig(roots_types, L, nh=4):
    """Concatenate chords (root pc, type) each lasting L seconds; voiced around C3-C4."""
    segs = []
    for root, ty in roots_types:
        iv = TH.CHORD_TYPES[ty]
        base = 48 + root                       # root in C3..B3
        y = np.zeros(int(L * SR))
        for i, d in enumerate(iv):
            y += tone(base + d, L, nh, 0.08)
        y += tone(base - 12, L, nh, 0.08)       # bass root an octave down
        segs.append(y)
    return np.concatenate(segs)


def reverb(y, rt60=1.5, wet=0.5, seed=1):
    r = np.random.default_rng(seed)
    n = int(rt60 * SR)
    ir = r.standard_normal(n) * 10 ** (-60 * np.arange(n) / SR / rt60 / 20)
    ir[0] = 0
    w = sps.fftconvolve(y, ir)[: len(y)]
    w *= np.sqrt((y ** 2).mean() / max((w ** 2).mean(), 1e-12))
    return (1 - wet) * y + wet * w


PROG = [(0, "maj"), (9, "min"), (5, "maj"), (7, "maj")]       # C Am F G


def name_of(root, ty):
    return NOTE_NAMES[root] + ("" if ty == "maj" else ty)


# --- chords --------------------------------------------------------------------------------
def cal_chords(row):
    for L in (4.0, 2.0, 1.0, 0.5, 0.35):
        n = int(round(48 / L))
        seq = [PROG[i % 4] for i in range(n)]
        y = chord_sig(seq, L)
        truth = (n - 1) / (n * L / 60)
        for tag, sig in (("dry", y), ("reverb 1.5 s", reverb(y))):
            c = TH.chords(sig)
            old = TH.chord_changes_old(sig)
            # graded where the tool is reliable: dry down to 0.5 s, with a 1.5-s reverb down to 1 s;
            # beyond that the row is a limit (reported, not graded)
            beyond = L < 0.5 or (L < 1.0 and tag != "dry")
            tol = "limit" if beyond else max(2.0, 0.25 * truth)
            row("chords", f"changes/min, a chord every {L} s, {tag}", round(truth, 1), c["changes_per_min"], tol, "changes/min",
                f"old 2-s counter read {old:.1f}" + ("; past the reliable range" if beyond else ""))
            if tag == "dry":
                # label accuracy: sample each true chord's middle
                hit = 0
                for i, (r, ty) in enumerate(seq):
                    mid = (i + 0.5) * L
                    seg = [g for g in c["segments"] if g["t0"] <= mid < g["t1"]]
                    hit += bool(seg and seg[0]["chord"] == name_of(r, ty))
                row("chords", f"chord labels right, a chord every {L} s (dry)", 1.0, hit / n, "limit" if L < 0.5 else 0.2, "share")
    # richer vocabulary: seventh chords, sus
    seq = [(2, "min7"), (7, "7"), (0, "maj7"), (0, "maj7"), (9, "min7"), (2, "7"), (7, "maj7"), (7, "sus4")] * 3
    y = chord_sig(seq, 3.0)
    c = TH.chords(y)
    hit = 0
    for i, (r, ty) in enumerate(seq):
        mid = (i + 0.5) * 3.0
        seg = [g for g in c["segments"] if g["t0"] <= mid < g["t1"]]
        hit += bool(seg and seg[0]["chord"] == name_of(r, ty))
    row("chords", "labels right on sevenths and sus (3 s each)", 1.0, hit / len(seq), 0.45, "share",
        "sevenths read as the triad inside them are not wrong in kind; exact-name score is a floor")
    # no change at all
    y = chord_sig([(0, "maj")], 60.0)
    c = TH.chords(y)
    row("chords", "one held chord for 60 s", 0, c["changes"], 1, "changes")


# --- key and mode --------------------------------------------------------------------------------
def mode_notes(tonic, deg):
    major = [0, 2, 4, 5, 7, 9, 11]
    return [(tonic + major[(deg + i) % 7] - major[deg]) % 12 for i in range(7)]


def cal_key(row):
    rng = np.random.default_rng(3)
    names = TH.MODES
    right_m = right_t = n = 0
    miss = []
    for tonic in (0, 3, 6):
        for deg in range(7):
            sc = mode_notes(tonic, deg)
            # 48 notes, tonic and fifth favoured; a bass pedal on the tonic half the time
            w = np.array([3.0, 1.0, 1.4, 1.0, 1.8, 1.0, 1.0])
            w = w / w.sum()
            pcs = rng.choice(7, 48, p=w)
            y = np.concatenate([tone(60 + sc[p], 0.45, 4, 0.1) for p in pcs])
            ch, _ = TH.chroma_of(y)
            bass_pc = np.zeros(12); bass_pc[tonic] = 1.0
            k = TH.key_mode(ch, bass_pc)
            n += 1
            ok_t = k["tonic"] == NOTE_NAMES[tonic]
            ok_m = k["mode"] == names[deg]
            right_t += ok_t; right_m += ok_m
            if not (ok_t and ok_m):
                miss.append(f"{NOTE_NAMES[tonic]} {names[deg]} read as {k['tonic']} {k['mode']}")
    row("key", "tonic right, 21 cases (7 modes x 3 keys), bass on tonic", 1.0, right_t / n, 0.1, "share", "; ".join(miss[:4]))
    row("key", "mode right, 21 cases (7 modes x 3 keys), bass on tonic", 1.0, right_m / n, 0.15, "share", "; ".join(miss[:4]))
    # without a bass: tonic leans on weights alone
    right_t = right_m = n = 0
    miss = []
    for tonic in (0, 3, 6):
        for deg in range(7):
            sc = mode_notes(tonic, deg)
            w = np.array([3.0, 1.0, 1.4, 1.0, 1.8, 1.0, 1.0]); w = w / w.sum()
            pcs = rng.choice(7, 48, p=w)
            y = np.concatenate([tone(60 + sc[p], 0.45, 4, 0.1) for p in pcs])
            ch, _ = TH.chroma_of(y)
            k = TH.key_mode(ch, None)
            n += 1
            right_t += k["tonic"] == NOTE_NAMES[tonic]
            right_m += k["mode"] == names[deg]
    row("key", "tonic right, 21 cases, no bass (tonic seen only in the weights)", 1.0, right_t / n, 0.35, "share")
    row("key", "mode right, 21 cases, no bass", 1.0, right_m / n, 0.4, "share")
    # flat weights: no key at all. The tool must say the margin is small.
    y = np.concatenate([tone(60 + p, 0.4, 4, 0.1) for p in range(12) for _ in range(4)])
    ch, _ = TH.chroma_of(y)
    k = TH.key_mode(ch, None)
    row("key", "all 12 notes equal (no key): collection margin", 0.0, k["collection_margin"], (0, 0.08), "",
        "a small margin is the signal that the key is a guess")
    # pentatonic
    pen = [0, 2, 4, 7, 9]
    pcs = rng.choice(5, 60)
    y = np.concatenate([tone(60 + pen[p], 0.4, 4, 0.1) for p in pcs])
    ch, _ = TH.chroma_of(y)
    k = TH.key_mode(ch, None)
    row("key", "major pentatonic detected", 1, int(k["pentatonic_fits"]), 0, "", f"{k.get('pentatonic_reading')}; weights {k['pitch_class_weights']}")


# --- meter ---------------------------------------------------------------------------------------
def drum_hit(kind, dur=0.3):
    t = t_(dur)
    if kind == "k":
        f = 50 + 70 * np.exp(-t / 0.03)
        return np.sin(2 * np.pi * np.cumsum(f) / SR) * np.exp(-t / 0.09)
    if kind == "h":
        r = np.random.default_rng(5)
        sos = sps.butter(2, 6000, "highpass", fs=SR, output="sos")
        return sps.sosfilt(sos, r.standard_normal(len(t))) * np.exp(-t / 0.02) * 0.6
    if kind == "s":
        r = np.random.default_rng(6)
        return r.standard_normal(len(t)) * np.exp(-t / 0.07) * 0.5
    raise ValueError(kind)


def drum_pattern(bpm, bpb, bars, swing=0.5, accent=(1.0, 0.55), seed=0, hats=True, extra=None):
    """Kick on every beat (bar-start kick louder), off-beat hats at `swing` of the beat."""
    per = 60.0 / bpm
    total = bars * bpb * per + 1.0
    y = np.zeros(int(total * SR))
    for b in range(bars * bpb):
        pos = b % bpb
        a = accent[0] if pos == 0 else accent[1]
        h = drum_hit("k")
        i = int(b * per * SR)
        y[i:i + len(h)] += a * h
        if hats:
            h2 = drum_hit("h")
            i2 = int((b + swing) * per * SR)
            y[i2:i2 + len(h2)] += 0.5 * h2
            # sixteenth ticks at swing between the beat and the off-beat
    return y


EXACT = []


def cal_meter(row):
    EXACT.clear()
    pad = lambda n: 0.01 * RNG.standard_normal(n)
    # tempo
    for bpm in (72, 90, 100, 120, 140, 160):
        d = drum_pattern(bpm, 4, 20)
        m = TH.meter(d, d + pad(len(d)))
        got = m.get("bpm") or 0
        fold = min((got * f for f in (0.5, 1, 2)), key=lambda v: abs(v - bpm))
        octave = "exact" if abs(got - bpm) < 0.02 * bpm else f"{got / bpm:.1f}x (octave)"
        row("meter", f"tempo {bpm} BPM within a factor of 2 (4/4, kick + hats)", bpm, fold, max(1.0, 0.02 * bpm), "BPM",
            f"read {got:.1f} = {octave}")
        EXACT.append(abs(got - bpm) < 0.02 * bpm)
    row("meter", "tempo exactly right, not an octave off (6 tempos)", 1.0, float(np.mean(EXACT)), 0.35, "share",
        "the beat level is a judgment: hats on the off-beat make the doubled tempo as strong as the real one")
    # bar length: accent on beat 1
    for bpb in (3, 4, 5, 6, 7):
        d = drum_pattern(100, bpb, 40 // bpb * 2 + 8, hats=False)
        m = TH.meter(d, d + pad(len(d)))
        row("meter", f"beats per bar = {bpb} (accent on beat 1, 100 BPM)", bpb, m.get("beats_per_bar"), 0.5, "beats",
            f"scores {m.get('bar_scores')}")
    d = drum_pattern(100, 4, 24, accent=(0.8, 0.8), hats=False)
    m = TH.meter(d, d + pad(len(d)))
    row("meter", "NO accent: bar score stays low (the number it reads is a guess)", 0.0, m.get("bar_score", 0), (-1.0, 0.2), "",
        f"read {m.get('beats_per_bar')} beats; with an accent the score is 0.8 or more")
    # swing
    for sw in (0.5, 0.583, 0.667, 0.75):
        d = drum_pattern(100, 4, 24, swing=sw)
        m = TH.meter(d, d + pad(len(d)))
        row("meter", f"swing: off-beat at {sw} of the beat", sw, m.get("eighth_off_phase"), 0.03, "of beat",
            f"triplet share {m.get('triplet_share')}")
    # triplet vs straight: share
    d = drum_pattern(100, 4, 24, swing=0.5)
    m = TH.meter(d, d + pad(len(d)))
    row("meter", "triplet share on straight eighths", 0.0, m.get("triplet_share") or 0, (0, 0.2), "")
    d = drum_pattern(100, 4, 24, swing=2 / 3)
    m = TH.meter(d, d + pad(len(d)))
    row("meter", "triplet share on triplet swing", 1.0, m.get("triplet_share") or 0, (0.7, 1.0), "")
    # cross-rhythm: 4 against 3 over the same bar
    bpm = 100
    per = 60 / bpm
    bar = 4 * per
    y = np.zeros(int((16 * bar + 1) * SR))
    for b in range(16):
        for k in range(4):
            h = drum_hit("k"); i = int((b * bar + k * per) * SR); y[i:i + len(h)] += (1.0 if k == 0 else 0.7) * h
        for k in range(3):
            h = drum_hit("s"); i = int((b * bar + k * bar / 3) * SR); y[i:i + len(h)] += 0.9 * h
    m = TH.meter(y, y + pad(len(y)))
    names = [r["ratio"] for r in m["cross_rhythm"]["ratios_found"]]
    row("meter", "cross-rhythm 4 against 3 found", 1, int("4:3" in names or "3:2" in names), 0, "",
        f"ratios found {names}")
    fa = []
    for sw in (0.5, 0.583, 0.667):
        for bpb in (4, 3):
            d = drum_pattern(100, bpb, 20, swing=sw)
            m = TH.meter(d, d + pad(len(d)))
            fa.append(len(m["cross_rhythm"]["ratios_found"]))
    row("meter", "cross-rhythm false alarms on 6 plain / swung grooves", 0, sum(1 for v in fa if v), 1, "grooves",
        f"ratios found per groove {fa}")


# --- bass ---------------------------------------------------------------------------------------------
def bass_note(midi, dur, glide_to=None):
    t = t_(dur)
    m = np.full_like(t, float(midi)) if glide_to is None else midi + (glide_to - midi) * (t / dur)
    f = midi_to_hz(m)
    ph = 2 * np.pi * np.cumsum(f) / SR
    y = np.sin(ph) + 0.4 * np.sin(2 * ph) + 0.15 * np.sin(3 * ph)
    y *= np.minimum(1, t / 0.01) * np.exp(-t / 1.5) * 0.3
    return fade(y, 0.03)


def walk(n, seed=4, lo=0, hi=12):
    r = np.random.default_rng(seed)
    p, out = 5, []
    for _ in range(n):
        s = int(r.choice([-2, -1, 1, 2]))
        if not lo <= p + s <= hi:
            s = -s
        p += s
        out.append(p)
    return out


def cal_bass(row):
    A1 = 33
    def make(seq, dur, glide=None):
        notes = []; times = []
        t0 = 0.3
        for i, m in enumerate(seq):
            g = None if glide is None else m + glide[i]
            notes.append(bass_note(m, dur - 0.04, g)); times.append(t0 + i * dur)
        return place(notes, times, t0 + len(seq) * dur + 0.5)
    cases = [
        ("pedal (one note repeated)", make([A1] * 60, 0.5), "pedal"),
        ("ostinato (4-note figure)", make([A1, A1, A1 + 3, A1 + 7] * 15, 0.4), "ostinato"),
        ("walking (random stepwise walk)", make([A1 + v for v in walk(70)], 0.35), "walking"),
        ("repeating 8-note stepwise figure (an ostinato, not a walk)", make(list(np.array([0, 2, 3, 5, 7, 5, 3, 2] * 8) + A1), 0.35), "ostinato"),
        ("slides (3 semitones each)", make([A1, A1 + 3, A1, A1 + 5, A1, A1 + 3] * 6, 0.6, glide=[3, -3, 5, -5, 3, -3] * 6), "slides"),
    ]
    for label, y, key in cases:
        b = TH.bass_line(y)
        row("bass", f"{label}: reading", 1, int(key in b.get("reading", "")), 0, "", f"read: {b.get('reading')}, {b.get('notes')} notes")
    # pitch accuracy: a known 12-note bass sequence
    seq = [33, 36, 38, 40, 31, 33, 35, 38, 36, 33, 40, 28] * 3
    y = make(seq, 0.5)
    t, midi = TH.pitch_track(y)
    notes = TH.notes_from_track(t, midi)
    row("bass", "notes found (36 notes, 0.5 s each)", 36, len(notes), 4, "notes")
    errs = []
    for i, m in enumerate(seq):
        tt = 0.3 + i * 0.5
        near = [n for n in notes if abs(n["t"] - tt) < 0.15]
        if near:
            errs.append(near[0]["midi"] - m)
    row("bass", "pitch of each note (median abs error)", 0.0, float(np.median(np.abs(errs))) if errs else 99, 0.25, "semitones",
        f"{len(errs)} of {len(seq)} notes matched; sub-bass below 35 Hz is the weak end")
    # very low bass
    seq = [24, 26, 24, 28] * 6                                    # C1 = 32.7 Hz
    y = make(seq, 0.6)
    t, midi = TH.pitch_track(y)
    notes = TH.notes_from_track(t, midi)
    hit = sum(1 for i, m in enumerate(seq) if any(abs(n["t"] - (0.3 + i * 0.6)) < 0.2 and abs(n["midi"] - m) < 0.6 for n in notes))
    row("bass", "very low notes (C1-E1, 33-41 Hz) found with the right pitch", 1.0, hit / len(seq), 0.35, "share")


# --- lead ------------------------------------------------------------------------------------------------
def make_melody(seed, n=48, lo=67, hi=84):
    r = np.random.default_rng(seed)
    scale = [m for m in range(lo, hi + 1) if (m % 12) in (0, 2, 4, 5, 7, 9, 11)]
    i = len(scale) // 2
    seq = []
    for _ in range(n):
        step = r.choice([-3, -2, -1, 0, 1, 2, 3, 5, -5], p=[.07, .17, .17, .06, .17, .17, .07, .07, .05 + .0])
        i = int(np.clip(i + step, 0, len(scale) - 1))
        seq.append(scale[i])
    return seq


def melody_sig(seq, dur=0.4, gap=0.06):
    notes = [tone(m, dur, 6, 0.1, attack=0.015, decay_db_s=3) for m in seq]
    times = [0.3 + i * (dur + gap) for i in range(len(seq))]
    return place(notes, times, 0.3 + len(seq) * (dur + gap) + 0.5), times


def lead_score(seq, times, notes):
    got = 0
    perr = []
    for m, t in zip(seq, times):
        near = [n for n in notes if abs(n["t"] - t) < 0.12]
        if near and abs(near[0]["midi"] - m) < 0.6:
            got += 1
    return got / len(seq)


def cal_lead(row):
    seq = make_melody(11)
    mel, times = melody_sig(seq)
    d = np.diff(seq); ad = np.abs(d)
    truth = {"step": ((ad >= 1) & (ad <= 2)).mean(), "third": ((ad >= 3) & (ad <= 4)).mean(),
             "med": float(np.median(ad)), "up": (d > 0).sum() / max(1, (d != 0).sum())}
    pad_chords = np.zeros_like(mel)
    # a pad: C-Am-F-G in the octave below the melody
    prog = chord_sig([PROG[i % 4] for i in range(int(len(mel) / SR / 4) + 2)], 4.0)[: len(mel)]
    pad_chords = prog
    pad_chords = pad_chords * (np.sqrt((mel ** 2).mean()) / np.sqrt((pad_chords ** 2).mean()))
    for tag, sig in (("alone", mel), ("pad 12 dB below", mel + pad_chords * 10 ** (-12 / 20)), ("pad at equal level", mel + pad_chords)):
        notes, hop_s, _ = TH.lead_line(sig)
        s = TH.lead_summary(notes)
        hit = lead_score(seq, times, notes)
        row("lead", f"notes found with the right pitch, {tag}", 1.0, hit, {"alone": 0.2, "pad 12 dB below": 0.3, "pad at equal level": "limit"}[tag], "share",
            f"{len(notes)} notes found for {len(seq)}")
        if s.get("step_share") is not None:
            row("lead", f"step share (truth {truth['step']:.2f}), {tag}", truth["step"], s["step_share"],
                {"alone": 0.12, "pad 12 dB below": 0.2, "pad at equal level": "limit"}[tag], "share")
            row("lead", f"median interval (truth {truth['med']:.0f}), {tag}", truth["med"], s["median_abs_interval"],
                {"alone": 1.0, "pad 12 dB below": 1.5, "pad at equal level": "limit"}[tag], "semitones")
    # contour: arch, rising, falling phrases
    def phrase(shape):
        if shape == "arch":
            return [67, 69, 72, 76, 74, 71, 69]
        if shape == "rising":
            return [67, 69, 71, 72, 74, 76, 79]
        return [79, 77, 76, 74, 72, 71, 67]
    for shape in ("arch", "rising", "falling"):
        seq2 = []
        for i in range(8):
            seq2 += phrase(shape)
        notes_l, times_l = [], []
        t = 0.3
        for i, m in enumerate(seq2):
            notes_l.append(tone(m, 0.35, 6, 0.1, attack=0.015)); times_l.append(t)
            t += 0.4 + (0.7 if (i + 1) % 7 == 0 else 0)
        sig = place(notes_l, times_l, t + 0.5)
        notes, _, _ = TH.lead_line(sig)
        s = TH.lead_summary(notes)
        key = {"arch": "arch", "rising": "rising", "falling": "falling"}[shape]
        row("lead", f"phrase contour: {shape}", 1.0, s.get("contour_share", {}).get(key, 0.0), (0.7, 1.0), "share of phrases",
            f"{s.get('contour_share')}")


# --- form ----------------------------------------------------------------------------------------------------
def bar_block(i, bar_s):
    """A bar with its own chord and colour; i picks the identity."""
    r = np.random.default_rng(100 + i)
    root = int(r.integers(0, 12)); ty = list(TH.CHORD_TYPES)[int(r.integers(0, 4))]
    y = chord_sig([(root, ty)], bar_s, nh=int(2 + (i % 4)))
    sos = sps.butter(4, [300 + 400 * (i % 5), 2500 + 800 * (i % 7)], "bandpass", fs=SR, output="sos")
    y = y + sps.sosfilt(sos, r.standard_normal(len(y))) * 0.02
    return y


def cal_form(row):
    bar_s = 2.0
    # a 4-bar phrase repeated 4 times, then 4 new bars repeated twice
    P = [bar_block(i, bar_s) for i in range(4)]
    Q = [bar_block(10 + i, bar_s) for i in range(4)]
    y = np.concatenate(P * 4 + Q * 2)
    f = TH.form(y, bar_s)
    lags = [r["bars"] for r in f.get("repeat_lags_bars", [])]
    row("form", "repeat period of a 4-bar phrase (lags found include 4)", 1, int(4 in lags), 0, "", f"lags {lags}")
    row("form", "bars counted (24)", 24, f.get("bars", 0), 1, "bars")
    row("form", "near-duplicate bar share (truth 5/8 of bars repeat an earlier one)", 0.75, f.get("near_duplicate_bar_share", 0), 0.25, "share")
    # all different bars: no repetition
    y = np.concatenate([bar_block(50 + i, bar_s) for i in range(24)])
    f = TH.form(y, bar_s)
    row("form", "24 different bars: near-duplicate share (false alarm)", 0.0, f.get("near_duplicate_bar_share", 0), 0.1, "share")
    # AABA of 8 bars, bar length known
    A = [bar_block(70 + i, bar_s) for i in range(8)]
    B = [bar_block(90 + i, bar_s) for i in range(8)]
    y = np.concatenate(A + A + B + A)
    f = TH.form(y, bar_s)
    lags = [r["bars"] for r in f.get("repeat_lags_bars", [])]
    row("form", "AABA, 8-bar parts: lag 8 found", 1, int(8 in lags), 0, "", f"lags {lags}")
    # bar length a little wrong: repetition blurs. 1% should still work; 5% shows the limit.
    for pct, tol in ((1, 0), (5, "limit")):
        f = TH.form(y, bar_s * (1 + pct / 100))
        lags = [r["bars"] for r in f.get("repeat_lags_bars", [])]
        row("form", f"same, bar length {pct}% off: lag 8 still found", 1, int(8 in lags), tol, "", f"lags {lags}; bars drift out of step as the error adds up over a track")


TESTS = {"chords": cal_chords, "key": cal_key, "meter": cal_meter, "bass": cal_bass, "lead": cal_lead, "form": cal_form}
