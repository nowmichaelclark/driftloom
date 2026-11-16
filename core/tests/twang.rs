//! twang (queue item 101h; batch 6's #13): a synth in the spirit of a
//! tremolo baritone guitar, drawn per loop at load on a stream of its own
//! (`choices::twang_of`), picked by the player (`Sequencer::twang`), sounded
//! by its voice (`blocks::twang`). What it holds to (the batch's yardstick):
//! it goes only to loops that play a chord and have no kit busier than a
//! soft one, to about its share of them, a third each version, each leaned
//! by its own vibes; a pin plays a version or keeps it off, and nothing the
//! generator decides but twang moves; in the player, at each chord whose
//! lowest note changes (or the same after `regap` seconds) that note's pitch
//! class between E2 and D#3, its fifth and its tenth (`spread` beats apart:
//! together, as the approved clips played them), 82 to 400 Hz, six strings
//! at most, never in a full stop and let go there;
//! and the voice: the pick's share above 1 kHz falling to a third or less by
//! 0.3 s, the level swinging 30 to 55% at 4 to 6 Hz or two cycles a beat,
//! its level following the chords channel, and nothing under 60 Hz.

// Counts of loops are shares as floats; steps and pitches copied, not
// computed, are compared exactly; the generator, its render and a pick are
// `g`, `r` and `t` as in the other tests.
#![allow(clippy::cast_precision_loss, clippy::float_cmp, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::many_single_char_names)]

use dlcore::block::Event;
use dlcore::block::event::LET_GO;
use dlcore::blocks::twang;
use dlcore::generator::Generator;
use dlcore::generator::choices::TWANG_KINDS;
use dlcore::generator::data::TWANG;
use dlcore::generator::request::{BAD_SETTING, Request};
use dlcore::generator::rng::Rng;
use dlcore::player::host::Host;
use dlcore::player::{Player, State};
use dlcore::{CHORDS, Core, EXTRA, Key, PAD, QUANTUM, TWANG as VOICE, Tap};

fn roomy(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new().stack_size(64 << 20).spawn(f).expect("a thread").join().expect("the test");
}

/// The seeds of the batch's corpus (corpus seed 20261009), its first `n`.
fn corpus(n: usize) -> Vec<u32> {
    let mut r = Rng::new(20_261_009);
    (0..n).map(|_| r.seed32()).collect()
}

fn ask(t: &str) -> Request {
    Request::parse(t).expect("a request")
}

/// Whether the loop `g` holds may play twang: a chord, and no kit busier
/// than `kitMost` hits a second over the kit's own cycle.
fn may(g: &Generator) -> bool {
    let (spec, r) = (g.spec(), g.render());
    let chord = r.chords.iter().any(|e| e.vel != 0.0 && !e.notes.is_empty());
    let cycle = spec.cycles.of(0).filter(|&c| c > 0.0).unwrap_or(spec.bars * spec.steps_per_bar);
    let hits = r.drums.iter().filter(|e| e.vel != 0.0).count() as f64;
    chord && hits <= TWANG.kit_most * cycle * 15.0 / spec.bpm + 1e-9
}

/// Of `n` corpus loops as `request` asks them: how many may play twang, and
/// how many of those play each version (soft, slow, bright).
fn versions(g: &mut Generator, request: &Request, n: usize) -> (usize, [usize; 3]) {
    let (mut of, mut on) = (0, [0; 3]);
    for seed in corpus(n) {
        g.load_with(seed, request);
        let v = g.settings().twang;
        if may(g) {
            of += 1;
            if v > 0 {
                on[usize::from(v) - 1] += 1;
            }
        } else {
            assert_eq!(v, 0, "seed {seed}: no chord or a busy kit, no twang");
        }
    }
    (of, on)
}

#[test]
fn it_goes_to_its_share_of_the_loops_it_may_play_in_a_third_each() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let (of, on) = versions(&mut g, &Request::new(), 2000);
        let all: usize = on.iter().sum();
        let f = all as f64 / of as f64;
        println!("twang on 2000 corpus loops: {on:?} (soft, slow, bright) of {of} that may play it ({f:.3}); {:.3} of all", all as f64 / 2000.0);
        assert!(of > 600 && (f - TWANG.share).abs() < 0.03, "{f:.3} of the loops, not near {}", TWANG.share);
        // A third each before the vibes; the corpus's own vibes lean them
        // (each version by its row), but none is starved.
        for &n in &on {
            assert!((0.12..=0.55).contains(&(n as f64 / all as f64)), "{on:?}: a version starved or taking all");
        }
    });
}

#[test]
fn each_version_is_leaned_by_its_own_vibes_never_to_none() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let mut of = |vibe: &str| {
            let (n, on) = versions(&mut g, &ask(vibe), 1500);
            on.map(|x| x as f64 / n as f64)
        };
        let (wistful, meditative, cruising, zany) = (of("wistful"), of("meditative"), of("cruising"), of("zany"));
        let third = TWANG.share / 3.0;
        println!("soft/slow/bright: wistful {wistful:.3?}, meditative {meditative:.3?}, cruising {cruising:.3?}, zany {zany:.3?}");
        assert!(wistful[0] > third * 1.15 && wistful[0] > wistful[1] && wistful[0] > wistful[2], "wistful leans to soft");
        assert!(meditative[1] > third * 1.15 && meditative[1] > meditative[0] && meditative[1] > meditative[2], "meditative leans to slow");
        assert!(cruising[2] > third * 1.15 && cruising[2] > cruising[1], "cruising leans to bright");
        assert!(zany.iter().all(|&x| x < third) && zany.iter().sum::<f64>() > 0.0, "zany away from all three, never none");
    });
}

#[test]
fn a_pin_plays_its_version_or_keeps_it_off_and_nothing_else_moves() {
    roomy(|| {
        assert_eq!(Request::parse("twang.kind=loud").err(), Some(BAD_SETTING));
        let mut g = Box::new(Generator::new());
        let mut a = Box::new(Generator::new());
        let (mut busy, mut played) = (0, 0);
        for seed in corpus(200) {
            for (i, &(kind, _)) in TWANG_KINDS.iter().enumerate() {
                g.load_with(seed, &ask(&format!("twang.kind={kind}")));
                a.load_with(seed, &ask("twang.kind=today"));
                let (on, off) = (g.render(), a.render());
                let m = may(&g);
                busy += usize::from(!m && i == 1);
                played += usize::from(m && i == 1);
                assert_eq!(usize::from(g.settings().twang), if m { i } else { 0 }, "seed {seed}: {kind}");
                assert_eq!(a.settings().twang, 0, "seed {seed}: today's plays none");
                assert_eq!(on.choices.iter().map(|w| w.get()).collect::<Vec<_>>(), off.choices.iter().map(|w| w.get()).collect::<Vec<_>>(), "seed {seed}");
                assert!(on.chords.iter().zip(off.chords.iter()).all(|(x, y)| x.step == y.step && x.vel == y.vel && x.notes[..] == y.notes[..]), "seed {seed}: a chord moved");
                assert!(on.melody.iter().zip(off.melody.iter()).all(|(x, y)| x.step == y.step && x.midi == y.midi), "seed {seed}: the tune moved");
                assert!(on.bass.iter().zip(off.bass.iter()).all(|(x, y)| x.step == y.step && x.midi == y.midi), "seed {seed}: the bass moved");
                assert!(on.drums.iter().zip(off.drums.iter()).all(|(x, y)| x.step == y.step && x.vel == y.vel), "seed {seed}: the kit moved");
                assert_eq!(g.settings().tuning, a.settings().tuning, "seed {seed}");
                assert_eq!(g.settings().low_bell, a.settings().low_bell, "seed {seed}");
            }
        }
        println!("pinned soft: {played} loops play it, {busy} keep it off (no chord, or a kit busier than a soft one)");
        assert!(played > 60 && busy > 60, "{played} played, {busy} kept off");
    });
}

/// A host that renders nothing and writes down every twang pick (time,
/// midi, parts, its extra values) and every let-go to the chords channel
/// (time, the picks it reaches up to, its fall).
#[derive(Default)]
struct Heard {
    frame: u64,
    picks: Vec<(f64, f64, u32, [f64; 2])>,
    let_go: Vec<(f64, f64, f64)>,
}

impl Host for Heard {
    fn frame(&self) -> u64 {
        self.frame
    }
    fn key(&mut self, _: Key) {}
    fn note(&mut self, voice: u32, channel: u32, time: f64, midi: f64, _: f64, _: f64, parts: u32, x: [f64; EXTRA]) {
        if voice == VOICE {
            assert_eq!(channel, CHORDS);
            self.picks.push((time, midi, parts, [x[twang::BEATS], x[twang::ZERO]]));
        }
    }
    fn event(&mut self, channel: u32, e: Event) {
        if channel == CHORDS && e.kind == LET_GO {
            self.let_go.push((e.time, e.value, e.notes[0]));
        }
    }
    fn damp(&mut self, _: u32, _: f64) {}
    fn set(&mut self, _: u32, _: usize, _: usize, _: f64, _: f64) {}
    fn route(&mut self, _: u32, _: usize, _: usize, _: Option<Tap>, _: f64) {}
    fn param(&mut self, _: u32, _: u32, _: f64, _: f64, _: f64) {}
    fn bass_lift(&mut self, _: f64) {}
    fn space_range(&mut self, _: usize, _: f64, _: f64) {}
    fn steal(&mut self, _: u32, _: Key, _: f64, _: f64) {}
    fn fade_all(&mut self, _: f64, _: f64) {}
}

fn play(seed: u32, text: &str, wander: f64, seconds: u64) -> Heard {
    let mut player = Box::new(Player::new());
    let mut host = Heard::default();
    player.attach(&mut host, 48_000.0, true);
    player.set_wander(wander);
    player.load(seed, &ask(text));
    player.ready(&mut host);
    assert_eq!(player.state(), State::Playing);
    while host.frame < seconds * 48_000 {
        player.decide(&mut host);
        host.frame += QUANTUM as u64;
    }
    host
}

const PINS: [&str; 3] = ["twang.kind=soft,tuning.cents=0", "twang.kind=slow,tuning.cents=0", "twang.kind=bright,tuning.cents=0"];

#[test]
fn it_picks_the_chord_s_lowest_note_its_fifth_and_its_tenth() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let (mut loops, mut roots, mut want_roots, mut later) = (0, 0, 0, 0);
        for (n, seed) in corpus(160).into_iter().enumerate() {
            let text = PINS[n % 3];
            g.load_with(seed, &ask(text));
            if g.settings().twang == 0 || g.render().form.is_some() {
                continue;
            }
            let (spec, r) = (g.spec(), g.render());
            let sd = 15.0 / spec.bpm;
            // The picks' spacing (`spread` beats; 0: together, as the
            // approved clips played them).
            let beat = 4.0 * sd * TWANG.spread;
            let cycle = spec.cycles.of(2).filter(|&c| c > 0.0).unwrap_or(spec.bars * spec.steps_per_bar);
            // As composed (no wander): every slot whose chords play.
            let heard = play(seed, text, 0.0, 30);
            let Some(&(t0, _, _, _)) = heard.picks.first() else { panic!("seed {seed}: no pick") };
            let first = r.harmony.slots.iter().find(|s| r.chords.iter().any(|e| e.vel != 0.0 && e.step >= s.start_step && e.step < s.start_step + s.length_steps)).map_or(0.0, |s| s.start_step);
            // Each root picked (time, its note, its tenth's offset), as the
            // chords come: where the root changes, or after `regap`.
            let mut want = Vec::new();
            let (mut last, mut at) = (-1, f64::NEG_INFINITY);
            let mut k = first;
            while (k - first) * sd < 29.0 {
                let s = k % cycle;
                if let Some(slot) = r.harmony.slots.iter().find(|x| x.start_step == s)
                    && r.chords.iter().any(|e| e.vel != 0.0 && !e.notes.is_empty() && e.step >= s && e.step < s + slot.length_steps)
                {
                    let t = (k - first) * sd + if (k as u64) % 2 == 1 { spec.swing * sd } else { 0.0 };
                    // The lowest note the chords sound in the slot, and what
                    // sits over it.
                    let notes: Vec<i32> = r.chords.iter().filter(|e| e.vel != 0.0 && e.step >= s && e.step < s + slot.length_steps).flat_map(|e| e.notes.iter().map(|&m| i32::from(m))).collect();
                    let lowest = notes.iter().copied().min().unwrap_or(0);
                    let p = lowest.rem_euclid(12);
                    if p != last || t - at >= TWANG.regap - 0.001 {
                        let has = |d: i32| notes.iter().any(|&m| (m - lowest).rem_euclid(12) == d);
                        let third = if has(4) { 4.0 } else if has(3) { 3.0 } else { 0.0 };
                        let low = 40.0 + f64::from((p - 4).rem_euclid(12));
                        want.push((t, low, third));
                        last = p;
                        at = t;
                    }
                }
                k += 1.0;
            }
            // The picks well inside what was listed (the player's clock from
            // its first pick).
            let end = 29.0 - 4.0 * (4.0 * sd);
            let got: Vec<(f64, f64)> = heard.picks.iter().map(|&(t, m, _, _)| (t - t0, m)).filter(|g| g.0 <= end).collect();
            want.retain(|w| w.0 <= end);
            for &(t, m) in &got {
                let hz = 440.0 * 2f64.powf((m - 69.0) / 12.0);
                assert!((82.0..=400.0).contains(&hz), "seed {seed}: picked at {m} ({hz:.1} Hz)");
                let of = want.iter().find(|w| {
                    [(0.0, w.1), (beat, w.1 + 7.0), (2.0 * beat, w.1 + 12.0 + w.2)].iter().any(|&(d, x)| (t - w.0 - d).abs() < 0.02 && (m - x).abs() < 1e-9)
                });
                assert!(of.is_some(), "seed {seed}: picked {m} at {t:.3}, no chord's root, fifth or tenth\n{want:?}");
                later += usize::from(of.is_some_and(|w| (m - w.1).abs() > 0.5));
            }
            let picked = want.iter().filter(|w| got.iter().any(|g| (g.0 - w.0).abs() < 0.02 && g.1 == w.1)).count();
            roots += picked;
            want_roots += want.len();
            for &(_, _, parts, [beats, _]) in &heard.picks {
                assert_eq!(parts, (n % 3) as u32, "seed {seed}: its version");
                assert!((beats - spec.bpm / 60.0).abs() < 1e-9, "seed {seed}: the loop's beats a second");
            }
            loops += 1;
        }
        println!("{loops} loops: {roots} roots picked of {want_roots} chords that call for one; {later} fifths and tenths, {} beats after", TWANG.spread);
        assert!(loops > 20 && roots > 150 && later > roots, "{loops} loops, {roots} roots, {later} later picks");
        assert!(roots as f64 > 0.95 * want_roots as f64, "the budget left out {} of {want_roots}", want_roots - roots);
    });
}

#[test]
fn six_strings_ring_at_most_and_one_tremolo_moves_them() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let (mut loops, mut taken) = (0, 0);
        for (n, seed) in corpus(120).into_iter().enumerate() {
            let text = PINS[n % 3];
            g.load_with(seed, &ask(text));
            if g.settings().twang == 0 {
                continue;
            }
            let heard = play(seed, text, 1.0, 40);
            let zero = heard.picks.first().map_or(0.0, |p| p.3[1]);
            for (i, &(t, _, _, [_, z])) in heard.picks.iter().enumerate() {
                assert_eq!(z, zero, "seed {seed}: one tremolo for the loop");
                // The strings still ringing as this one is picked: picked
                // before, not let go by now, and within their ring.
                let ringing = heard.picks[..i].iter().filter(|p| t < p.0 + twang::ring() && !heard.let_go.iter().any(|&(at, upto, _)| at <= t && p.0 <= upto)).count();
                assert!(ringing <= 6, "seed {seed}: {ringing} strings ring as one is picked at {t:.2}");
            }
            taken += heard.let_go.iter().filter(|l| l.2 < 0.1).count();
            loops += 1;
        }
        println!("{loops} loops, {taken} strings taken by a seventh");
        assert!(loops > 10 && taken > 50, "{loops} loops, {taken} taken");
    });
}

#[test]
fn never_in_a_full_stop_and_let_go_there() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let mut stops = 0;
        for seed in corpus(600) {
            g.load_with(seed, &ask("twang.kind=soft"));
            let r = g.render();
            if g.settings().twang == 0 || r.quiet.is_empty() {
                continue;
            }
            let spec = g.spec();
            let bar = spec.steps_per_bar * 15.0 / spec.bpm;
            let total = spec.bars * bar;
            let quiet: Vec<f64> = r.quiet.iter().map(|&b| b as f64 * bar).collect();
            let heard = play(seed, "twang.kind=soft", 1.0, 40);
            // The loop's first step, the player's `START` on.
            let start = 0.05;
            let into = |t: f64| ((t - start) % total + total) % total;
            for &(t, _, _, _) in &heard.picks {
                assert!(!quiet.iter().any(|&q| into(t) >= q - 1e-6 && into(t) < q + bar - 1e-6), "seed {seed}: a pick at {t:.2} s in a full stop");
            }
            // Each full stop that comes while a string may ring lets them go.
            for &q in &quiet {
                let mut at = start + q;
                while at < 40.0 - bar {
                    let last = heard.picks.iter().map(|b| b.0).filter(|&t| t < at).fold(f64::NEG_INFINITY, f64::max);
                    if last + twang::ring() > at {
                        assert!(heard.let_go.iter().any(|&(t, upto, _)| t > last && t < at + 0.05 && upto >= last), "seed {seed}: nothing let go by the full stop at {at:.2} s");
                        stops += 1;
                    }
                    at += total;
                }
            }
        }
        println!("{stops} full stops let the strings go");
        assert!(stops > 3, "{stops} full stops");
    });
}

/// One pick alone through the core (`parts` its version, at 60 BPM, the
/// tremolo's phase 0 at the start; a pad played on the chords channel first
/// when `under`, so the core stamps a louder level), and the pick's samples
/// from 1 s.
fn pick_alone(midi: f64, parts: u32, under: bool) -> Vec<f32> {
    let run = |pick: bool| {
        let mut core = Box::new(Core::new());
        core.start(48_000.0, 1).expect("a core");
        if under {
            for (k, n) in [55.0, 59.0, 62.0].into_iter().enumerate() {
                let mut x = [f64::NAN; EXTRA];
                x[0] = 3.0;
                core.note(PAD, CHORDS, 0.01 * k as f64, n, 8.0, 0.8, 0, x);
            }
        }
        if pick {
            let mut x = [f64::NAN; EXTRA];
            x[twang::BEATS] = 1.0;
            x[twang::ZERO] = 0.0;
            core.note(VOICE, CHORDS, 1.0, midi, twang::ring(), 1.0, parts, x);
        }
        let mut out = Vec::new();
        let mut block = vec![0.0f32; QUANTUM];
        for _ in 0..(7 * 48_000 / QUANTUM) {
            core.render(&mut [&mut block]).expect("a block");
            out.extend_from_slice(&block);
        }
        out
    };
    let (with, without) = (run(true), run(false));
    with.iter().zip(&without).skip(48_000).map(|(a, b)| a - b).collect()
}

fn db(x: &[f32]) -> f64 {
    let p = x.iter().map(|&v| f64::from(v) * f64::from(v)).sum::<f64>() / x.len() as f64;
    10.0 * p.log10()
}

/// The share of `x`'s energy between `lo` and `hi` Hz (48 kHz): the band's
/// bins of a plain DFT against all of them (Parseval's sum, the half
/// spectrum's).
fn share(x: &[f32], lo: f64, hi: f64) -> f64 {
    let n = x.len();
    let bin = 48_000.0 / n as f64;
    let all = x.iter().map(|&v| f64::from(v) * f64::from(v)).sum::<f64>() * n as f64 / 2.0;
    let mut band = 0.0;
    let mut k = ((lo / bin).ceil() as usize).max(1);
    while (k as f64) * bin < hi && k < n / 2 {
        let (mut re, mut im) = (0.0, 0.0);
        for (i, &v) in x.iter().enumerate() {
            let w = std::f64::consts::TAU * ((k * i) % n) as f64 / n as f64;
            re += f64::from(v) * w.cos();
            im -= f64::from(v) * w.sin();
        }
        band += re * re + im * im;
        k += 1;
    }
    band / all
}

#[test]
fn the_pick_falls_to_a_rounder_tail() {
    roomy(|| {
        for (name, parts, midi) in [("soft", 0, 45.0), ("slow", twang::SLOW, 40.0), ("bright", twang::BRIGHT, 47.0)] {
            let x = pick_alone(midi, parts, false);
            let w = |a: f64| &x[(a * 48_000.0) as usize..((a + 0.05) * 48_000.0) as usize];
            let (pick, tail) = (1.0 - share(w(0.0), 0.0, 1000.0), 1.0 - share(w(0.3), 0.0, 1000.0));
            let low = share(&x[..48_000], 0.0, 60.0);
            println!("{name}: above 1 kHz {:.1}% in the first 50 ms, {:.1}% at 0.3 s; under 60 Hz {:.2}%", 100.0 * pick, 100.0 * tail, 100.0 * low);
            assert!(tail <= pick / 3.0, "{name}: {tail:.3} at 0.3 s against {pick:.3}");
            assert!(low < 0.01, "{name}: {low:.4} under 60 Hz");
            let peak = x.iter().fold(0.0f32, |m, &v| m.max(v.abs()));
            assert!(20.0 * f64::from(peak).log10() < -1.0, "{name}: its peak alone");
        }
    });
}

#[test]
fn its_tremolo_swings_30_to_55_percent_at_its_rate() {
    roomy(|| {
        // The level in 20 ms frames (a tenth of a cycle of the quickest
        // tremolo) over 0.3 to 1.9 s, each frame against the mean of the
        // cycle around it, which takes out the string's fall; its depth each
        // cycle's dip against its top. A low note (midi 28), so the two
        // saws' own slow beat (4 cents apart: 0.1 Hz there, 0.25 Hz at 110
        // Hz, where its null lands 2 s in) stays out of the reading: the
        // tremolo is the voice's, the same at every pitch.
        for (name, parts, hz, asked) in [("soft", 0, 4.5f64, 0.35f64), ("slow", twang::SLOW, 2.0, 0.55), ("bright", twang::BRIGHT, 5.5, 0.45)] {
            let x = pick_alone(28.0, parts, false);
            let frame = 960;
            let level: Vec<f64> = x[(0.3 * 48_000.0) as usize..(1.9 * 48_000.0) as usize].chunks(frame).map(db).map(|d| 10f64.powf(d / 20.0)).collect();
            let per = (50.0 / hz).round() as usize;
            let half = per / 2;
            let norm: Vec<f64> = (half..level.len() - half).map(|i| level[i] / (level[i - half..i - half + per].iter().sum::<f64>() / per as f64)).collect();
            let mut swings = Vec::new();
            let mut i = 0;
            while i + per <= norm.len() {
                let c = &norm[i..i + per];
                let (lo, hi) = (c.iter().copied().fold(f64::INFINITY, f64::min), c.iter().copied().fold(0.0, f64::max));
                swings.push(1.0 - lo / hi);
                i += per;
            }
            let depth = swings.iter().sum::<f64>() / swings.len() as f64;
            // Its rate: the strongest of the level's cycles from 1 to 8 Hz.
            let mean = norm.iter().sum::<f64>() / norm.len() as f64;
            let best = (10..=80)
                .map(|k| {
                    let f = f64::from(k) / 10.0;
                    let (mut re, mut im) = (0.0, 0.0);
                    for (i, &l) in norm.iter().enumerate() {
                        let w = std::f64::consts::TAU * f * i as f64 * 0.02;
                        re += (l - mean) * w.cos();
                        im += (l - mean) * w.sin();
                    }
                    (re * re + im * im, f)
                })
                .fold((0.0, 0.0), |a, b| if b.0 > a.0 { b } else { a })
                .1;
            println!("{name}: swings {:.0}% at {best:.1} Hz (asked {hz} Hz)", 100.0 * depth);
            // What each version asks is inside the yardstick's 30 to 55%,
            // and the reading is what it asks.
            assert!((0.30..=0.55).contains(&asked) && (depth - asked).abs() < 0.05, "{name}: {depth:.2} deep, asked {asked}");
            assert!((best - hz).abs() <= 0.15, "{name}: {best} Hz");
        }
    });
}

#[test]
fn its_level_follows_the_chords_channel() {
    roomy(|| {
        let alone = db(&pick_alone(45.0, 0, false)[..48_000]);
        let over = db(&pick_alone(45.0, 0, true)[..48_000]);
        println!("a pick alone {alone:.1} dB, over a held chord {over:.1} dB");
        assert!(over > alone + 1.0, "louder over a louder chord layer");
        // Never past the most: 0.16 against the floor's 0.06.
        assert!(over - alone < 20.0 * (0.16f64 / 0.06).log10() + 0.1);
    });
}
