//! What a host asks of the next loop, beside its seed (promise 11's
//! request; queue item 85): `readRequest` and `parseRequest` in
//! `js/generator.js`, read the same way.
//!
//! One shape for everyone: the app's picks, a developer's code and a game's
//! events build the same request, and whatever it leaves out is drawn from
//! the seed as ever (a request of nothing is the loop the seed makes). It
//! holds the vibes, with or without weights (a game steers by a friend's
//! mood and the time of day; the core never knows what a friend is), the
//! tempo, pinned or a range, a theme (a seed whose loop this one bends: a
//! friend keeps its tune as the day changes) and the pins (`Pins`). Items
//! 86 to 91 grow it.
//!
//! **Bends, never breaks.** A request is checked as it is built: a vibe the
//! core does not have, a bad weight or a tempo outside `TEMPO_LIMIT` is
//! refused there, with a short plain ASCII line (the "can't do that" a host
//! shows), so a request the generator is handed can always be met.
//!
//! **As text** (`parse`): items split by commas, each a vibe (`glacial`),
//! a vibe and its weight (`glacial=0.7`), the tempo (`bpm=77`,
//! `bpm=70-85`), the theme (`theme=1234`) or a pin (`organ=on`,
//! `arc.period=240`; `Pins`); empty or `-` for nothing. The generation goldens, `dlrender --request` and the
//! wasm hosts write it so.

use super::data::{self, ARC, CHOICES, ECHO, KICK, LATEFLOOR, PADSWEEP, SUBDUCK, VIBE_COUNT};
use super::js;
use crate::space;

/// The tempos the core plays, in bpm: what a share code's tempo byte holds
/// (`TEMPO_LIMIT`).
pub const TEMPO_LIMIT: [f64; 2] = [32.0, 190.0];

/// The line for a vibe the core does not have.
pub const NO_SUCH_VIBE: &str = "no such vibe";
/// The line for a weight that is not a number of 0 or more.
pub const BAD_WEIGHT: &str = "a bad weight";
/// The line for a tempo that is not a number.
pub const BAD_TEMPO: &str = "a bad tempo";
/// The line for a theme that is not a seed (a whole number from 0 to
/// 4294967295).
pub const BAD_THEME: &str = "a bad theme";
/// The line for a tempo outside `TEMPO_LIMIT`.
pub const TEMPO_OUT: &str = "tempo must be 32 to 190 bpm";

/// A request: the vibes asked, each with its weight or none, the tempo
/// asked, and the pins. Plain data, `Copy`, about two kilobytes: a host
/// keeps one and hands it to each load.
#[derive(Clone, Copy)]
pub struct Request {
    /// Whether each vibe (by its number, `data::VIBES`) is asked.
    asked: [bool; VIBE_COUNT],
    /// Each asked vibe's weight; NaN where it was asked without one.
    weights: [f64; VIBE_COUNT],
    /// The tempo asked, [low, high] in whole bpm (equal when pinned).
    tempo: Option<[f64; 2]>,
    /// The theme (queue item 85d): the seed whose loop this one bends.
    theme: Option<u32>,
    /// The named settings fixed (queue item 85b).
    pins: Pins,
}

impl Request {
    /// Nothing asked for: the loop the seed makes.
    #[must_use]
    pub const fn new() -> Self {
        Request { asked: [false; VIBE_COUNT], weights: [f64::NAN; VIBE_COUNT], tempo: None, theme: None, pins: Pins::NONE }
    }

    /// The same, with pin number `number` (`Pins`) at `value` (a word by its
    /// place in the pin's list).
    ///
    /// # Errors
    /// `NO_SUCH_SETTING`, `BAD_SETTING` or `SETTING_OUT` (`Pins::with`).
    pub fn with_pin(mut self, number: usize, value: f64) -> Result<Self, &'static str> {
        self.pins = self.pins.with(number, value)?;
        Ok(self)
    }

    /// The pins asked.
    #[must_use]
    pub fn pins(&self) -> &Pins {
        &self.pins
    }

    /// The same, bending the loop of `seed` (queue item 85d): the loop made
    /// keeps that loop's profile mix, its key and mode, its metre and its
    /// tune, and the vibes, the tempo, the pins and the load's own seed bend
    /// the rest. The load's seed the theme's own and nothing else asked is
    /// the theme's loop.
    #[must_use]
    pub fn with_theme(mut self, seed: u32) -> Self {
        self.theme = Some(seed);
        self
    }

    /// The theme asked.
    #[must_use]
    pub fn theme(&self) -> Option<u32> {
        self.theme
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
    /// written); a vibe or a pin named twice keeps the last.
    ///
    /// # Errors
    /// `NO_SUCH_VIBE`, `BAD_WEIGHT`, `BAD_TEMPO`, `TEMPO_OUT`,
    /// `NO_SUCH_SETTING`, `BAD_SETTING` or `SETTING_OUT`, for the first item
    /// that is one.
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
            } else if name == "theme" {
                let seed = value.and_then(whole).filter(|&x| x <= f64::from(u32::MAX)).ok_or(BAD_THEME)?;
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // a whole number, checked in range
                let seed = seed as u32;
                out = out.with_theme(seed);
            } else if is_pin_name(name) {
                let value = value.ok_or(BAD_SETTING)?;
                out.pins = out.pins.with_text(name, value)?;
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

// ---------------------------------------------------------------- the pins

/// The line for a dotted name that is no setting.
pub const NO_SUCH_SETTING: &str = "no such setting";
/// The line for a pin's value that is no number, or no word it takes.
pub const BAD_SETTING: &str = "a bad setting";
/// The line for a pin's value outside what the loop could draw.
pub const SETTING_OUT: &str = "a setting outside its range";

/// Room for the choices' pins: pin numbers 0 to 15 (`CHOICE_PINS` in
/// `js/pins.js`), the choices in `data::CHOICES`' order.
pub const CHOICE_PINS: usize = 16;
/// The settings' first number (`SETTING_PINS`).
pub const SETTING_PINS: usize = 16;
/// The space recipe's first number (`SPACE_PINS`), its settings in
/// `space::SETTINGS`' order.
pub const SPACE_PINS: usize = 48;
/// How many pin numbers there are room for.
pub const PINS_MAX: usize = SPACE_PINS + space::n::COUNT;
const _: () = assert!(PINS_MAX <= 128, "the pins' bits are a u128");

/// The settings' pins, by number from `SETTING_PINS` (`js/pins.js`, the
/// same order; append only).
pub const SETTINGS: [&str; 17] = [
    "arc.shape",
    "arc.period",
    "arc.from",
    "arc.floor",
    "kick.note",
    "kick.short",
    "kick.long",
    "subDuck.depth",
    "subDuck.recover",
    "lateFloor.order",
    "lateFloor.seconds",
    "lateFloor.gap",
    "padSweep.lo",
    "padSweep.oct",
    "padSweep.period",
    "padSweep.q",
    "echo.beats",
];

/// What a pin takes.
enum Allowed {
    /// One of these words, sent by its place.
    Words(data::Pool),
    /// A number in this range, a whole one if `true`.
    Range([f64; 2], bool),
    /// One of these numbers.
    Values(data::NumPool),
}

/// The request's pins (queue item 85b; `js/pins.js`, whose doc says what a
/// pin is and does): the named settings a host fixes for the loops it asks
/// for, by number, each the value it is pinned to (a word by its place in
/// the pin's list), and which are pinned (a bit each: nothing pinned is all
/// zeros, so a core built in a static stores none of it). **A pin chooses within what the loop could
/// draw:** each is held to the data's range or list. **A pin shapes, it
/// doesn't draw:** every draw is made as ever and a pin then overrules one
/// value (`render::Render`'s choices, `load::Load`'s settings).
#[derive(Clone, Copy)]
pub struct Pins {
    values: [f64; PINS_MAX],
    pinned: u128,
}

impl Pins {
    /// Nothing pinned.
    pub const NONE: Pins = Pins { values: [0.0; PINS_MAX], pinned: 0 };

    /// Pin `number`'s name ("organ", "arc.period"), or `None`; a space
    /// setting's is its name in `space::SETTINGS`, "space." before it.
    #[must_use]
    pub fn name(number: usize) -> Option<(&'static str, &'static str)> {
        if number < CHOICE_PINS {
            CHOICES.get(number).map(|c| ("", c.id))
        } else if number < SPACE_PINS {
            SETTINGS.get(number - SETTING_PINS).map(|&n| ("", n))
        } else {
            space::SETTINGS.get(number - SPACE_PINS).map(|s| ("space.", s.name))
        }
    }

    /// The number of the pin called `name`, or `None`.
    #[must_use]
    pub fn number(name: &str) -> Option<usize> {
        if let Some(rest) = name.strip_prefix("space.") {
            return space::SETTINGS.iter().position(|s| s.name == rest).map(|i| SPACE_PINS + i);
        }
        CHOICES
            .iter()
            .position(|c| c.id == name)
            .or_else(|| SETTINGS.iter().position(|&n| n == name).map(|i| SETTING_PINS + i))
    }

    /// What pin `number` takes.
    fn allowed(number: usize) -> Option<Allowed> {
        if number < CHOICE_PINS {
            return CHOICES.get(number).map(|c| Allowed::Words(c.options));
        }
        if number >= SPACE_PINS {
            return space::SETTINGS.get(number - SPACE_PINS).map(|s| Allowed::Range([s.lo, s.hi], false));
        }
        let p = &ARC.period;
        let periods = [p.calm, p.bright, p.rest, p.glacial, p.glacial_near];
        let period = [
            periods.iter().fold(f64::INFINITY, |m, r| js::min(m, r[0])),
            periods.iter().fold(f64::NEG_INFINITY, |m, r| js::max(m, r[1])),
        ];
        Some(match *SETTINGS.get(number - SETTING_PINS)? {
            "arc.shape" => Allowed::Words(ARC.shapes),
            "arc.period" => Allowed::Range(period, false),
            "arc.from" => Allowed::Range([0.0, 1.0], false),
            "arc.floor" => Allowed::Range(ARC.floor, false),
            "kick.note" => Allowed::Range(KICK.note, true),
            "kick.short" => Allowed::Range(KICK.short, false),
            "kick.long" => Allowed::Range(KICK.long, false),
            "subDuck.depth" => Allowed::Range(SUBDUCK.depth, false),
            "subDuck.recover" => Allowed::Range(SUBDUCK.recover, false),
            "lateFloor.order" => Allowed::Words(LATEFLOOR.orders),
            "lateFloor.seconds" => Allowed::Range(LATEFLOOR.seconds, false),
            "lateFloor.gap" => Allowed::Range([1.0, LATEFLOOR.max_gap], true),
            "padSweep.lo" => Allowed::Range(PADSWEEP.lo, false),
            "padSweep.oct" => Allowed::Range(PADSWEEP.oct, false),
            "padSweep.period" => Allowed::Range(PADSWEEP.period, false),
            "padSweep.q" => Allowed::Range(PADSWEEP.q, false),
            "echo.beats" => Allowed::Values(ECHO.beats),
            _ => return None,
        })
    }

    /// The same, with pin `number` at `value` (a word by its place).
    ///
    /// # Errors
    /// `NO_SUCH_SETTING` for a number that is no pin, `BAD_SETTING` for a
    /// value that is no number (or no word's place), `SETTING_OUT` for one
    /// outside what the loop could draw.
    pub fn with(mut self, number: usize, value: f64) -> Result<Self, &'static str> {
        let allowed = Self::allowed(number).ok_or(NO_SUCH_SETTING)?;
        if !value.is_finite() {
            return Err(BAD_SETTING);
        }
        let ok = match allowed {
            Allowed::Words(w) => {
                if value < 0.0 || js::floor(value) != value || value >= js::num(w.len()) {
                    return Err(BAD_SETTING);
                }
                true
            }
            Allowed::Range([lo, hi], whole) => value >= lo && value <= hi && (!whole || js::floor(value) == value),
            Allowed::Values(v) => v.iter().any(|&(x, _)| x == value),
        };
        if !ok {
            return Err(SETTING_OUT);
        }
        self.values[number] = value;
        self.pinned |= 1 << number;
        Ok(self)
    }

    /// The same, with the pin called `name` at the value `text` says (a
    /// word, or a number as `js/pins.js` reads one).
    ///
    /// # Errors
    /// As `with`.
    pub fn with_text(self, name: &str, text: &str) -> Result<Self, &'static str> {
        let number = Self::number(name).ok_or(NO_SUCH_SETTING)?;
        let value = match Self::allowed(number) {
            Some(Allowed::Words(w)) => w.iter().position(|&(x, _)| x == text).map(js::num).ok_or(BAD_SETTING)?,
            _ => number_text(text).ok_or(BAD_SETTING)?,
        };
        self.with(number, value)
    }

    /// Whether anything is pinned.
    #[must_use]
    pub fn any(&self) -> bool {
        self.pinned != 0
    }

    /// Pin `number`'s value, or `None`.
    #[must_use]
    pub fn at(&self, number: usize) -> Option<f64> {
        (number < PINS_MAX && self.pinned & (1 << number) != 0).then(|| self.values[number])
    }

    /// The setting called `name`'s pinned number (a word's place), or
    /// `None`.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<f64> {
        Self::number(name).and_then(|n| self.at(n))
    }

    /// The word the pin called `name` is pinned to, or `None`.
    #[must_use]
    pub fn word(&self, name: &str) -> Option<&'static str> {
        let n = Self::number(name)?;
        let i = self.at(n)?;
        match Self::allowed(n)? {
            Allowed::Words(w) => w.get(js::index(i)).map(|x| x.0),
            _ => None,
        }
    }

    /// The option choice number `index` (`data::CHOICES`) is pinned to, by
    /// its place, or `None`.
    #[must_use]
    pub fn choice(&self, index: usize) -> Option<usize> {
        if index >= CHOICE_PINS {
            return None;
        }
        self.at(index).map(js::index)
    }

    /// The space recipe's setting `which` (`space::n`) pinned, or `None`.
    #[must_use]
    pub fn space(&self, which: usize) -> Option<f64> {
        self.at(SPACE_PINS + which)
    }

    /// The canonical text (`spec.pins` in `tools/gen-golden.mjs`): an object
    /// of the pins by name, sorted, each its word or its number.
    pub fn write<S: super::canon::Sink>(&self, w: &mut super::canon::Writer<S>) {
        let mut order = [0u8; PINS_MAX];
        let mut n = 0;
        for i in 0..PINS_MAX {
            if self.at(i).is_some() {
                order[n] = u8::try_from(i).unwrap_or(u8::MAX);
                n += 1;
            }
        }
        let full = |i: u8| Self::name(usize::from(i)).unwrap_or(("", ""));
        order[..n].sort_unstable_by(|&a, &b| {
            let (pa, na) = full(a);
            let (pb, nb) = full(b);
            pa.bytes().chain(na.bytes()).cmp(pb.bytes().chain(nb.bytes()))
        });
        w.obj();
        for &i in &order[..n] {
            let (prefix, name) = full(i);
            let mut key = [0u8; 64];
            let len = prefix.len() + name.len();
            if len > key.len() {
                continue;
            }
            key[..prefix.len()].copy_from_slice(prefix.as_bytes());
            key[prefix.len()..len].copy_from_slice(name.as_bytes());
            w.key_any(core::str::from_utf8(&key[..len]).unwrap_or(""));
            let number = usize::from(i);
            match Self::allowed(number) {
                Some(Allowed::Words(words)) => w.str(self.at(number).and_then(|x| words.get(js::index(x))).map_or("", |x| x.0)),
                _ => w.num(self.values[number]),
            }
        }
        w.end_obj();
    }
}

impl Default for Pins {
    fn default() -> Self {
        Self::NONE
    }
}

/// Whether `name` is a pin's (a choice's id, or a dotted setting's) rather
/// than a vibe's (`isPinName`).
#[must_use]
pub fn is_pin_name(name: &str) -> bool {
    name.contains('.') || CHOICES.iter().any(|c| c.id == name)
}

/// A pin's number as text (`/^-?\d+(\.\d+)?(e[-+]?\d+)?$/` and `Number`):
/// both languages read such text to the same nearest double.
fn number_text(s: &str) -> Option<f64> {
    let digits = |p: &str| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit());
    let body = s.strip_prefix('-').unwrap_or(s);
    let (mantissa, exp) = match body.split_once('e') {
        Some((m, e)) => (m, Some(e)),
        None => (body, None),
    };
    let (int, frac) = match mantissa.split_once('.') {
        Some((i, f)) => (i, Some(f)),
        None => (mantissa, None),
    };
    let exp_ok = exp.is_none_or(|e| digits(e.strip_prefix(['-', '+']).unwrap_or(e)));
    (digits(int) && frac.is_none_or(digits) && exp_ok).then(|| s.parse().ok()).flatten()
}
