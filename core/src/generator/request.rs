//! What a host asks of the next loop, beside its seed (promise 11's
//! request; queue item 85): `readRequest` and `parseRequest` in
//! `js/generator.js`, read the same way.
//!
//! One shape for everyone: the app's picks, a developer's code and a game's
//! events build the same request, and whatever it leaves out is drawn from
//! the seed as ever (a request of nothing is the loop the seed makes). For
//! now it holds the vibes, with or without weights (a game steers by a
//! friend's mood and the time of day; the core never knows what a friend
//! is), and the tempo, pinned or a range. Items 86 to 91 grow it.
//!
//! **Bends, never breaks.** A request is checked as it is built: a vibe the
//! core does not have, a bad weight or a tempo outside `TEMPO_LIMIT` is
//! refused there, with a short plain ASCII line (the "can't do that" a host
//! shows), so a request the generator is handed can always be met.
//!
//! **As text** (`parse`): items split by commas, each a vibe (`glacial`),
//! a vibe and its weight (`glacial=0.7`) or the tempo (`bpm=77`,
//! `bpm=70-85`); empty or `-` for nothing. The generation goldens,
//! `dlrender --request` and the wasm hosts write it so.

use super::data::{self, VIBE_COUNT};
use super::js;

/// The tempos the core plays, in bpm: what a share code's tempo byte holds
/// (`TEMPO_LIMIT`).
pub const TEMPO_LIMIT: [f64; 2] = [32.0, 190.0];

/// The line for a vibe the core does not have.
pub const NO_SUCH_VIBE: &str = "no such vibe";
/// The line for a weight that is not a number of 0 or more.
pub const BAD_WEIGHT: &str = "a bad weight";
/// The line for a tempo that is not a number.
pub const BAD_TEMPO: &str = "a bad tempo";
/// The line for a tempo outside `TEMPO_LIMIT`.
pub const TEMPO_OUT: &str = "tempo must be 32 to 190 bpm";

/// A request: the vibes asked, each with its weight or none, and the tempo
/// asked. Plain data, `Copy`, a few hundred bytes: a host keeps one and
/// hands it to each load.
#[derive(Clone, Copy)]
pub struct Request {
    /// Whether each vibe (by its number, `data::VIBES`) is asked.
    asked: [bool; VIBE_COUNT],
    /// Each asked vibe's weight; NaN where it was asked without one.
    weights: [f64; VIBE_COUNT],
    /// The tempo asked, [low, high] in whole bpm (equal when pinned).
    tempo: Option<[f64; 2]>,
}

impl Request {
    /// Nothing asked for: the loop the seed makes.
    #[must_use]
    pub const fn new() -> Self {
        Request { asked: [false; VIBE_COUNT], weights: [f64::NAN; VIBE_COUNT], tempo: None }
    }

    /// The same, with the vibe numbered `index` (`data::VIBES`) asked
    /// without a weight, as a listener picks it (a number the core does not
    /// have is left out, as the app's selection leaves out names).
    #[must_use]
    pub fn with(mut self, index: usize) -> Self {
        if index < VIBE_COUNT {
            self.asked[index] = true;
            self.weights[index] = f64::NAN;
        }
        self
    }

    /// The same, with the vibe called `id` ("glacial", as `data/vibes.txt`
    /// names it); `None` for a vibe the core does not have.
    #[must_use]
    pub fn with_vibe(self, id: &str) -> Option<Self> {
        data::vibe_index(id).map(|i| self.with(i))
    }

    /// The same, with the vibe numbered `index` asked at `weight`: the share
    /// of the loop it steers. Shares adding up to more than 1 are scaled to
    /// add up to 1, less is a lighter touch, 0 leaves the vibe out; once any
    /// vibe has a weight, one asked without counts 1. Past five vibes the
    /// five heaviest steer.
    ///
    /// # Errors
    /// `NO_SUCH_VIBE` for a number the core does not have, `BAD_WEIGHT` for
    /// a weight that is not a finite number of 0 or more.
    pub fn with_weight(mut self, index: usize, weight: f64) -> Result<Self, &'static str> {
        if index >= VIBE_COUNT {
            return Err(NO_SUCH_VIBE);
        }
        if !weight.is_finite() || weight < 0.0 {
            return Err(BAD_WEIGHT);
        }
        self.asked[index] = true;
        self.weights[index] = weight;
        Ok(self)
    }

    /// The same, with the tempo asked: `lo` to `hi` bpm, each rounded to a
    /// whole bpm (the low first, whichever way they come); pinned when they
    /// are equal. The loop's tempo is drawn across the range from its seed,
    /// and its profile leans toward one whose band holds it.
    ///
    /// # Errors
    /// `BAD_TEMPO` for a tempo that is not a finite number, `TEMPO_OUT` for
    /// one outside `TEMPO_LIMIT`.
    pub fn with_tempo(mut self, lo: f64, hi: f64) -> Result<Self, &'static str> {
        if !lo.is_finite() || !hi.is_finite() {
            return Err(BAD_TEMPO);
        }
        let (mut a, mut b) = (js::round(lo), js::round(hi));
        if a > b {
            core::mem::swap(&mut a, &mut b);
        }
        if a < TEMPO_LIMIT[0] || b > TEMPO_LIMIT[1] {
            return Err(TEMPO_OUT);
        }
        self.tempo = Some([a, b]);
        Ok(self)
    }

    /// A request read from its text (the module's doc says how it is
    /// written); a vibe named twice keeps the last.
    ///
    /// # Errors
    /// `NO_SUCH_VIBE`, `BAD_WEIGHT`, `BAD_TEMPO` or `TEMPO_OUT`, for the
    /// first item that is one.
    pub fn parse(text: &str) -> Result<Self, &'static str> {
        let text = text.trim();
        let mut out = Request::new();
        if text.is_empty() || text == "-" {
            return Ok(out);
        }
        for item in text.split(',') {
            let (name, value) = match item.split_once('=') {
                Some((n, v)) => (n, Some(v)),
                None => (item, None),
            };
            if name == "bpm" {
                let value = value.ok_or(BAD_TEMPO)?;
                let (lo, hi) = value.split_once('-').unwrap_or((value, value));
                let (Some(lo), Some(hi)) = (whole(lo), whole(hi)) else { return Err(BAD_TEMPO) };
                out = out.with_tempo(lo, hi)?;
            } else {
                let index = data::vibe_index(name).ok_or(NO_SUCH_VIBE)?;
                out = match value {
                    None => out.with(index),
                    Some(v) => out.with_weight(index, decimal(v).ok_or(BAD_WEIGHT)?)?,
                };
            }
        }
        Ok(out)
    }

    /// Whether the vibe numbered `index` is asked, and its weight (`None`
    /// where it was asked without one).
    #[must_use]
    pub fn vibe(&self, index: usize) -> Option<Option<f64>> {
        match self.asked.get(index) {
            Some(true) => Some(Some(self.weights[index]).filter(|w| !w.is_nan())),
            _ => None,
        }
    }

    /// Whether any vibe is asked with a weight.
    #[must_use]
    pub fn weighted(&self) -> bool {
        (0..VIBE_COUNT).any(|i| self.asked[i] && !self.weights[i].is_nan())
    }

    /// The tempo asked, [low, high] in whole bpm.
    #[must_use]
    pub fn tempo(&self) -> Option<[f64; 2]> {
        self.tempo
    }
}

impl Default for Request {
    fn default() -> Self {
        Self::new()
    }
}

/// Digits only, as a number (the JavaScript's `/^\d+$/` and `Number`).
fn whole(s: &str) -> Option<f64> {
    (!s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())).then(|| s.parse().ok()).flatten()
}

/// Digits, then a point and digits or not (`/^\d+(\.\d+)?$/`), as a number:
/// both languages read such text to the same nearest double.
fn decimal(s: &str) -> Option<f64> {
    let (int, frac) = s.split_once('.').unwrap_or((s, "0"));
    let digits = |p: &str| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit());
    (digits(int) && digits(frac)).then(|| s.parse().ok()).flatten()
}

/// How far a band `[lo, hi]` sits from a tempo asked `[a, b]`, in bpm: 0
/// where they overlap (`tempoGap`).
#[must_use]
pub fn tempo_gap(band: [f64; 2], t: [f64; 2]) -> f64 {
    if t[0] > band[1] {
        t[0] - band[1]
    } else if band[0] > t[1] {
        band[0] - t[1]
    } else {
        0.0
    }
}

/// A profile whose band misses the asked tempo by this many bpm keeps half
/// its weight in the mix's draw (`TEMPO_PREFER`; for Mikey's ears).
const TEMPO_PREFER: f64 = 6.0;

/// The lean on a profile's weight in the mix's draw for a tempo asked
/// (`tempoLean`): 1 where its band holds it, falling away with the gap but
/// never to nothing, so no profile is ruled out.
#[must_use]
pub fn tempo_lean(t: Option<[f64; 2]>, band: [f64; 2]) -> f64 {
    let Some(t) = t else { return 1.0 };
    let d = tempo_gap(band, t) / TEMPO_PREFER;
    1.0 / (1.0 + d * d)
}
