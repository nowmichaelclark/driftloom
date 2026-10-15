"""Step 7 of the album card: draft the card from the measurements.

Usage: python3 -I card.py WORKDIR/<album> OUT.md

Reads WORKDIR/<album>/pick.json and, for each picked track, events.json,
techniques.json, theory.json and space.json. Writes the plan's album card
(hook, sounds, techniques, theory, space, vibes, wholesome take, For later) with
numbers and timestamps filled in. Anything that needs ears or taste is a line
marked **[JUDGMENT]** for a person or a later session. Nothing here invents a
musical opinion: each sentence is a number, a timestamp, or a threshold read
off a number, and each threshold comes from the calibration table.

American English. No lyrics or quotes are ever written.
"""
import os, re, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np

from cardlib import load_json, mmss, NOTE_NAMES, hz_to_midi, note_name

J = "**[JUDGMENT]**"


def slug(s):
    return re.sub(r"[^A-Za-z0-9]+", "_", s).strip("_")[:60]


def short(track):
    return re.sub(r"^\d+\s+", "", track)


def f0(v, nd=1):
    return "n/a" if v is None else f"{v:.{nd}f}"


def pc(v):
    return "n/a" if v is None else f"{100 * v:.0f}%"


def load_track(work, track):
    d = os.path.join(work, slug(track))
    out = {"track": track, "dir": d}
    for k in ("events", "techniques", "theory", "space"):
        p = os.path.join(d, k + ".json")
        out[k] = load_json(p) if os.path.exists(p) else None
    return out


# --- sounds ---------------------------------------------------------------------------------
def sound_candidates(tracks, per_track=2, total=8):
    """Best groups first, at most `per_track` per track in the first pass so one track
    does not fill the list; the rest go to For later."""
    cands = []
    for t in tracks:
        ev = t["events"]
        if not ev:
            continue
        allw = sum(g["weight"] for s in ev["stems"].values() for g in s["groups"]) or 1.0
        for stem, s in ev["stems"].items():
            for g in s["groups"][:2]:
                share = g["weight"] / allw
                if g["n"] >= 6 and share >= 0.02:
                    cands.append({"track": t["track"], "stem": stem, "group": g, "share": share,
                                  "silhouette": s["silhouette"], "groups_in_stem": len(s["groups"])})
    cands.sort(key=lambda c: -c["share"])
    chosen, count = [], {}
    for c in cands:
        if count.get(c["track"], 0) < per_track and len(chosen) < total:
            chosen.append(c)
            count[c["track"]] = count.get(c["track"], 0) + 1
    rest = [c for c in cands if c not in chosen]
    return chosen, rest


def fp_row(g, k, nd=1, scale=1.0):
    v = g["fingerprint"].get(k) or {}
    if v.get("med") is None:
        return "n/a"
    return f"{v['med'] * scale:.{nd}f} ({v['p10'] * scale:.{nd}f} to {v['p90'] * scale:.{nd}f})"


def sound_block(i, c):
    g = c["group"]
    lines = [f"#### S{i}. {short(c['track'])}: {c['stem']} group `{g['id']}` ({g['n']} notes, {pc(c['share'])} of the track's note energy)"]
    lines.append(f"- In synth terms: {g['description']}.")
    lines.append("- Hear it at: " + ", ".join(f"{e['at']} ({e['note'] or '?'}, {e['dur_s']} s, {e['solo_db']:+.0f} dB alone)" for e in g["examples"]) + ".")
    lines.append("- Fingerprint, median (10th to 90th percentile):")
    lines.append(f"  - register {fp_row(g, 'f0_hz', 0)} Hz; attack {fp_row(g, 'attack_ms', 0)} ms; decay {fp_row(g, 'decay_db_s', 0)} dB/s")
    lines.append(f"  - brightness at the start {fp_row(g, 'bright_start_hz', 0)} Hz; slope over the note {fp_row(g, 'bright_slope_oct', 2)} octaves")
    lines.append(f"  - harmonic share {fp_row(g, 'harm_share', 2)}; partials off whole multiples {fp_row(g, 'inharm_cents', 0)} cents")
    lines.append(f"  - vibrato {fp_row(g, 'vib_cents', 0)} cents; detune beating {fp_row(g, 'beat_hz', 1)} Hz")
    lines.append(f"  - level against the mix {fp_row(g, 'vs_mix_db', 1)} dB; pan {fp_row(g, 'pan', 2)}; left-right correlation {fp_row(g, 'lr_corr', 2)}")
    if c["silhouette"] < 0.25 or c["groups_in_stem"] == 1:
        lines.append(f"- Group quality: weak (silhouette {c['silhouette']:.2f}, {c['groups_in_stem']} group{'s' if c['groups_in_stem'] != 1 else ''} in the stem). "
                     "The notes in this stem do not fall into clearly separate instruments; treat this as one blended texture and listen before trusting the split.")
    lines.append(f"- Building blocks it would need: {J} name the closest existing Driftloom block, or the one missing.")
    lines.append(f"- Yardstick: the percentile ranges above, measured on the {c['stem']} stem (Demucs), not on the raw mix.")
    return "\n".join(lines)


# --- techniques ----------------------------------------------------------------------------
def technique_lines(t):
    tech = t["techniques"]
    if not tech:
        return []
    L = []
    nm = short(t["track"])
    sec = tech["sections"]
    b = [x["t"] for x in sec["boundaries"]]
    if b:
        ln = sec["lengths_s"]
        L.append(f"- **{nm} sections:** {len(b)} boundaries at " + ", ".join(mmss(x) for x in b[:12]) +
                 f"; section lengths median {np.median(ln):.0f} s (range {min(ln):.0f} to {max(ln):.0f}). Reliable for sections of 10 s or more.")
    else:
        L.append(f"- **{nm} sections:** no boundary found; one continuous piece.")
    act = tech["stem_activity"]
    parts = []
    for s, a in act.items():
        ent = ", ".join(mmss(x) for x in a["enters"][:5])
        lv = ", ".join(mmss(x) for x in a["leaves"][:5])
        parts.append(f"{s} plays {pc(a['active_share'])} of the time (in: {ent or 'from the start'}; out: {lv or 'never'})")
    L.append(f"- **{nm} stem entries and exits:** " + "; ".join(parts) + ".")
    fd = tech.get("floor_drops") or {}
    if fd.get("drops") is not None:
        d = fd["drops"]
        L.append(f"- **{nm} floor drops (kick and bass out 2 s or more while the rest plays):** {fd['drops_per_min']:.2f} a minute"
                 + (": " + ", ".join(f"{mmss(x['t0'])} for {x['len_s']:.1f} s" for x in d[:8]) if d else "") + ".")
    elif fd.get("note"):
        L.append(f"- **{nm} floor drops:** {fd['note']}.")
    sw = []
    for who, arr in tech["sweeps"].items():
        for s in arr:
            if abs(s["octaves"]) >= 0.7:
                sw.append(f"{who} {s['dir']} {abs(s['octaves']):.1f} oct {mmss(s['t0'])} to {mmss(s['t1'])}")
    if sw:
        L.append(f"- **{nm} brightness sweeps (0.7 octave or more):** " + "; ".join(sw[:8]) + f"{' (' + str(len(sw) - 8) + ' more in techniques.json)' if len(sw) > 8 else ''}.")
    else:
        L.append(f"- **{nm} brightness sweeps:** none of 0.7 octave or more.")
    ech = []
    for who, e in tech["echoes"].items():
        for d in e.get("delays", []):
            if d.get("reliable"):
                ech.append(f"{who}: {1000 * d['delay_s']:.0f} ms (+{d['bump_db']:.1f} dB, {d['later_repeats_seen']} later repeats)")
    L.append(f"- **{nm} echo times:** " + ("; ".join(ech) + "." if ech else "none found reliably (needs 6 or more notes that ring alone for 1.5 s)."))
    pump = []
    for who, p in tech["pumping"].items():
        if p.get("pump_db", 0) >= 1.5:
            pump.append(f"{who} dips {p['pump_db']:.1f} dB at the kick (peak {p['lag_ms']:.0f} ms after, kick every {p['period_s']:.2f} s)")
    L.append(f"- **{nm} pumping locked to the kick:** " + ("; ".join(pump) + ". The tool reads about 70% of the true depth, so this is a floor." if pump else "under 1.5 dB; none found."))
    tp = []
    for who, tp_ in tech["tape"].items():
        if tp_.get("wobble_peak_cents", 0) >= 4:
            tp.append(f"{who} wobbles about {tp_['wobble_peak_cents']:.0f} cents at {tp_['wobble_hz']:.1f} Hz")
        if tp_.get("speed_move_p95_cents_per_4s", 0) >= 6:
            tp.append(f"{who} speed moves up to {tp_['speed_move_p95_cents_per_4s']:.0f} cents in 4 s")
    L.append(f"- **{nm} tape wobble and speed:** " + ("; ".join(tp) + "." if tp else "nothing at 4 cents or more in the held partials that could be tracked."))
    mv = []
    for who, m in tech["stereo_motion"].items():
        if who in ("mix",):
            continue
        if m.get("pan_range_p5_p95", 0) >= 0.25 or m.get("pan_cycle_strength", 0) >= 6:
            s = f"{who} pan range {m['pan_range_p5_p95']:.2f}"
            if m.get("pan_cycle_strength", 0) >= 6:
                s += f", cycling every {1 / m['pan_cycle_hz']:.0f} s"
            mv.append(s)
    L.append(f"- **{nm} stereo movement:** " + ("; ".join(mv) + "." if mv else "parts stay near where they started (pan range under 0.25 and no clear cycle)."))
    if tech.get("builds"):
        L.append(f"- **{nm} builds:** " + "; ".join(f"{mmss(x['t0'])} to {mmss(x['t1'])}" for x in tech["builds"][:5]) + ".")
    return L


# --- theory -------------------------------------------------------------------------------
def theory_lines(t, ears1=None):
    th = t["theory"]
    if not th:
        return []
    nm = short(t["track"])
    L = []
    m = th["meter"]
    if m.get("bpm"):
        half, dbl = m["bpm_if_half_or_double"]
        bar = f"{m['beats_per_bar']} beats a bar" if m.get("bar_score", 0) >= 0.3 else f"bar length unclear (best guess {m.get('beats_per_bar')}, score {m.get('bar_score', 0):.2f})"
        sw = m.get("eighth_off_phase")
        swing = "no clear off-beat onsets" if sw is None else (
            "straight eighths" if abs(sw - 0.5) < 0.04 else f"swung: off-beat at {sw:.2f} of the beat" + (" (triplet feel)" if abs(sw - 0.667) < 0.05 else ""))
        cr_all = sorted({r["ratio"] for r in m["cross_rhythm"]["ratios_found"]})
        cr = [] if (len(cr_all) > 2 or m.get("bar_score", 0) < 0.3) else m["cross_rhythm"]["ratios_found"]
        cr_note = ("; many unrelated periodicities found (" + ", ".join(cr_all) + "), which means irregular timing, not a cross-rhythm claim") if (cr_all and not cr) else ""
        L.append(f"- **{nm} meter:** about {m['bpm']:.0f} BPM (could be {half:.0f} or {dbl:.0f}; the tool cannot tell the beat level), {bar}; {swing}"
                 + (f"; possible cross-rhythm {', '.join(sorted({r['ratio'] for r in cr}))} (weak evidence, check by ear)" if cr else "; no cross-rhythm claim") + cr_note + ".")
    k = th["key"]
    unsure = k["collection_margin"] < 0.02
    L.append(f"- **{nm} key and mode:** {k['tonic']} {k['mode']}" + (" (low confidence: the key signature margin is "
             f"{k['collection_margin']:.3f}, so read this as a guess)" if unsure else f" (key signature margin {k['collection_margin']:.3f})")
             + (f"; fits {k['pentatonic_reading']}" if k.get("pentatonic_fits") else "") +
             "; pitch-class weights " + ", ".join(f"{n} {w:.2f}" for n, w in sorted(k['pitch_class_weights'].items(), key=lambda kv: -kv[1])[:5]) + ".")
    c = th["chords"]
    voc = ", ".join(f"{v['chord']} {pc(v['share'])}" for v in c["vocabulary"][:4])
    e1 = f"; Ears 1's 2-s counter said {ears1:.0f}" if ears1 is not None else ""
    L.append(f"- **{nm} chords:** {c['changes_per_min']:.0f} changes a minute ({c['settled_chords_per_min']:.0f} a minute are chords that last 1.5 s or more); "
             f"median chord {c['chord_len_s']['median']:.1f} s (10th to 90th percentile {c['chord_len_s']['p10']:.1f} to {c['chord_len_s']['p90']:.1f} s); "
             f"{c['distinct']} distinct chords; most time on {voc}. The old counter in this same run said {c['old_counter_per_min']:.0f}{e1}. "
             + (f" {pc(c['change_on_beat_share'])} of the changes land on a beat (chance would be {pc(c['change_on_beat_chance'])}), which is the check available on real music." if c.get("change_on_beat_share") is not None else "")
             + " Counts are calibrated on clean synthetic chords (exact down to a chord every 0.5 s); chord names on dense or noisy music are a guess.")
    b = th["bass"]
    if b.get("reading"):
        top = ", ".join(f"{n} {w:.2f}" for n, w in sorted(b["pc_hist"].items(), key=lambda kv: -kv[1])[:4])
        L.append(f"- **{nm} bass:** {b['reading']}; {b['notes_per_s']:.2f} notes a second; most time on {top}; "
                 f"steps {pc(b['step_share'])}, thirds {pc(b['third_share'])}, fourths and fifths {pc(b['fourth_fifth_share'])}, octaves {pc(b['octave_share'])}, repeats {pc(b['repeat_share'])}.")
    ld = th["lead"]
    if ld.get("median_abs_interval") is not None:
        cs = ld.get("contour_share", {})
        L.append(f"- **{nm} top line (from the 'other' stem; read by ear before trusting):** {ld['notes']} notes, range {ld['range_semitones_p10_p90']:.0f} semitones (10th to 90th percentile); "
                 f"steps {pc(ld['step_share'])}, thirds {pc(ld['third_share'])}, fourths and fifths {pc(ld['fourth_fifth_share'])}, leaps {pc(ld['leap_share'])}, repeats {pc(ld['repeat_share'])}; "
                 f"median interval {ld['median_abs_interval']:.0f} semitones; {pc(ld['up_share'])} of moves go up; phrases of about {ld.get('phrase_len_notes_median', 0):.0f} notes, "
                 + ", ".join(f"{k} {pc(v)}" for k, v in cs.items()) + ". Reliable only when the line sits clearly above its accompaniment.")
    f = th["form"]
    if f.get("bars"):
        lg = ", ".join(f"{r['bars']} bars ({r['similarity']:.2f})" for r in f["repeat_lags_bars"][:3])
        L.append(f"- **{nm} repetition:** {f['bars']} bars of {f['bar_s']:.1f} s; strongest repeat distances {lg or 'none'}; {pc(f['near_duplicate_bar_share'])} of bars near-duplicate an earlier bar."
                 + (" Bar length is uncertain, so these lags are weak evidence." if (m.get("bar_score", 0) < 0.3) else ""))
    return L


# --- space ---------------------------------------------------------------------------------
def space_lines(t):
    sp = t["space"]
    if not sp:
        return []
    nm = short(t["track"])
    L = []
    mx = sp["mix"]
    bands = sp["bands"]
    L.append(f"- **{nm} mix width by band** (width dB; 0 = unrelated channels, below -20 = near mono): " +
             "; ".join(f"{b.split('_')[0] if b.startswith('sub') else b} {mx[b]['width_db']:.0f} (corr {mx[b]['corr']:.2f})" for b in bands if mx.get(b)) +
             f"; over time (2-s windows) 10th/50th/90th percentile {sp['mix_width_over_time']['p10']:.0f}/{sp['mix_width_over_time']['p50']:.0f}/{sp['mix_width_over_time']['p90']:.0f} dB.")
    rows = []
    for s, v in sp["stems"].items():
        a = v["all"]
        if not a:
            continue
        widest = max((b for b in bands if v.get(b) and v[b].get("share_of_stem", 0) >= 0.1), key=lambda b: v[b]["width_db"], default=None)
        rows.append(f"{s}: {pc(v['share_of_mix_power'])} of the power, width {a['width_db']:.0f} dB, pan {a['pan']:+.2f}, mono loss {a['mono_loss_db']:.1f} dB"
                    + (f", widest in {widest} ({v[widest]['width_db']:.0f} dB)" if widest else ""))
    L.append(f"- **{nm} by part:** " + "; ".join(rows) + ". Bands holding under 10% of a part's energy are left out of 'widest' (they are too quiet to trust).")
    return L


# --- for later -----------------------------------------------------------------------------------
EXTRA = [("tail_db_per_s", "tail (dB a second)"), ("pulse", "pulse strength"), ("width_db", "width (side/mid dB)"),
         ("sweep_oct", "brightness sweep (octaves)"), ("wobble_cents", "wobble (cents)"), ("bends_per_min", "pitch bends a minute"),
         ("chord_changes_per_min", "chord changes a minute (2-s counter)"), ("repeat", "repetition")]


def for_later(rep, picked):
    rows = [r for r in rep["rows"] if "error" not in r and "skip" not in r]
    L = []
    items = []
    if len(rows) < 4:
        return L
    stats = {}
    for k, _ in EXTRA:
        v = np.array([r.get(k) if r.get(k) is not None else np.nan for r in rows], float)
        med = np.nanmedian(v)
        mad = np.nanmedian(np.abs(v - med)) * 1.4826
        sd = max(mad, 0.5 * np.nanstd(v), 1e-9)
        stats[k] = (med, sd)
    for r in rows:
        if r["track"] in picked:
            continue
        hits = []
        for k, lab in EXTRA:
            if r.get(k) is None:
                continue
            z = (r[k] - stats[k][0]) / stats[k][1]
            if abs(z) >= 2.5:
                hits.append((abs(z), f"{lab} {r[k]:.2f} ({'high' if z > 0 else 'low'}, album median {stats[k][0]:.2f})"))
        hits.sort(reverse=True)
        short_note = " Too short for the deep pass (under 45 s)." if r.get("seconds", 999) < 45 else ""
        if hits or short_note:
            score = max([h[0] for h in hits], default=0)
            items.append((score, r, hits, short_note))
    items.sort(key=lambda x: -x[0])
    for score, r, hits, short_note in items[:8]:
        if True:
            L.append(f"- **{short(r['track'])}** ({mmss(r.get('seconds', 0))}, {r.get('tempo', 0):.0f} BPM by the mix-level count): "
                     + ("stands out for " + "; ".join(h for _, h in hits[:3]) + "." if hits else "no outstanding number.") + short_note +
                     f" {J} listen and decide if it is worth a deep pass; why it waits is not measurable.")
    return L


# --- the album card ------------------------------------------------------------------------------
def write_album(work, out_path):
    rep = load_json(os.path.join(work, "pick.json"))
    album = rep["album"]
    tracks = [load_track(work, p) for p in rep["picks"]]
    tracks = [t for t in tracks if t["events"]]
    # Ears 1's chord number for each picked track, from the mix-level rows
    e1 = {r["track"]: r.get("chord_changes_per_min") for r in rep["rows"]}
    md = []
    md.append(f"## {album} (draft card from Ears 3 tools, {time.strftime('%Y-%m-%d')})")
    md.append("")
    md.append(f"How it was made: {rep.get('rule', '')}. Deep pass on: " + "; ".join(short(t["track"]) for t in tracks) +
              f". Typical track: {short(rep['typical']) if rep.get('typical') else 'none (EP)'}. Least like the rest: "
              + (", ".join(short(x) for x in rep.get("unlike", [])) or "n/a") + ".")
    md.append(f"Stems from Demucs htdemucs (drums, bass, other, vocals). Lines marked {J} need a person or a later session; every other line is a measurement or a threshold read off one.")
    md.append("")
    md.append("### The hook")
    md.append(f"- Numbers that repeat across the deep-pass tracks: " + hook_numbers(tracks) + ".")
    md.append(f"- {J} The one thing that makes a listener say \"that's them\", in one line, after listening at the timestamps under Sounds.")
    md.append("")
    md.append("### Sounds")
    md.append("Groups are ranked by share of the track's note energy, at most two per track. Examples are the clearest notes (loud against everything else playing).")
    cands, rest = sound_candidates(tracks)
    if not cands:
        md.append("- No note group big enough to report.")
    for i, c in enumerate(cands, 1):
        md.append(sound_block(i, c))
        md.append("")
    md.append("### Techniques")
    for t in tracks:
        md += technique_lines(t)
    md.append(f"- {J} Pick the 1 to 3 moves above that define the record and name them (a filter arc, a floor drop, tape speed changes, chopped loops, a build).")
    md.append("")
    md.append("### Theory")
    for t in tracks:
        md += theory_lines(t, e1.get(t["track"]))
    md.append(f"- {J} Pick 1 or 2 of the readings above and write each as something the generator could draw (a spread, not a rule).")
    md.append("")
    md.append("### Space")
    for t in tracks:
        md += space_lines(t)
    md.append("")
    md.append("### Vibes")
    md.append(f"- {J} For each sound and technique: vibes that lean toward it (x1.5 to x3), ones that lean away (x0.2 to x0.5, never 0), any new vibe or profile. "
              "Numbers to start from: " + vibe_hints(tracks) + ".")
    md.append("")
    md.append("### Wholesome take")
    md.append(f"- {J} What to take, what to leave. Measured flags to consider: " + wholesome_flags(tracks) + ".")
    md.append("")
    md.append("### For later")
    for c in rest[:8]:
        g = c["group"]
        md.append(f"- {short(c['track'])}, {c['stem']} group `{g['id']}` ({g['n']} notes): {g['description']}; hear it at "
                  + ", ".join(e["at"] for e in g["examples"][:2]) + f". {J} why it waits.")
    md += for_later(rep, [t["track"] for t in tracks])
    md.append("")
    with open(out_path, "w") as f:
        f.write("\n".join(md))
    return out_path


def hook_numbers(tracks):
    bits = []
    bpm = [t["theory"]["meter"].get("bpm") for t in tracks if t["theory"] and t["theory"]["meter"].get("bpm")]
    if bpm:
        bits.append(f"tempo {min(bpm):.0f}" + (f" to {max(bpm):.0f}" if round(min(bpm)) != round(max(bpm)) else "") + " BPM (octave-uncertain)")
    keys = [f"{t['theory']['key']['tonic']} {t['theory']['key']['mode'].split(' ')[0]}" for t in tracks if t["theory"]]
    if keys:
        bits.append("keys " + ", ".join(dict.fromkeys(keys)))
    w = [t["space"]["mix"]["all"]["width_db"] for t in tracks if t["space"]]
    if w:
        bits.append(f"mix width {min(w):.0f} to {max(w):.0f} dB")
    ch = [t["theory"]["chords"]["changes_per_min"] for t in tracks if t["theory"]]
    if ch:
        bits.append(f"chord changes {min(ch):.0f} to {max(ch):.0f} a minute")
    return "; ".join(bits) or "none"


def vibe_hints(tracks):
    bits = []
    for t in tracks:
        sp, th = t["space"], t["theory"]
        if not (sp and th):
            continue
        mx = sp["mix"]
        hi = (mx["pres_2k_5k"]["power"] + mx["air_gt5k"]["power"]) / max(mx["all"]["power"], 1e-12)
        bits.append(f"{short(t['track'])}: {th['meter']['bpm']:.0f} BPM, {th['chords']['settled_chords_per_min']:.0f} settled chords a minute, "
                    f"{pc(hi)} of energy above 2 kHz, width {mx['all']['width_db']:.0f} dB")
    return "; ".join(bits) or "none"


def wholesome_flags(tracks):
    bits = []
    for t in tracks:
        sp = t["space"]
        if not sp:
            continue
        mx = sp["mix"]
        hi = (mx["pres_2k_5k"]["power"] + mx["air_gt5k"]["power"]) / max(mx["all"]["power"], 1e-12)
        flag = []
        if hi > 0.25:
            flag.append(f"{pc(hi)} of energy above 2 kHz (bright or harsh)")
        if mx["all"]["mono_loss_db"] > 3.5:
            flag.append(f"mono loss {mx['all']['mono_loss_db']:.1f} dB (opposite-polarity parts)")
        if flag:
            bits.append(f"{short(t['track'])}: " + ", ".join(flag))
    return "; ".join(bits) or "no brightness or polarity flags from the numbers; listening still decides"


if __name__ == "__main__":
    write_album(sys.argv[1], sys.argv[2])
