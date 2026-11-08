//! The levers a clip pulls (queue item 83): `dlrender --seed` with choices
//! forced, a constant moved, the sketch block on, a setting pinned, the head
//! of the clip trimmed. `tools/clips.mjs` drives them from a job list.
//!
//!   --force CHOICE=OPTION    a choice every loop is made with, whatever it
//!                            drew (`lateFloor=on`; `CHOICE=today` the
//!                            first option, the way it plays without)
//!   --set NAME=VALUE         a named constant moved (`bass.LIFT_DB=3`, the
//!                            table in `src/tuning.rs`)
//!   --sketch                 the sketch block (`core/sketch/block.rs`) on,
//!                            in the layer `core/sketch/where.txt` names
//!   --pin BLOCK.SETTING=V    a channel block's setting pinned to V whenever
//!                            the loop sets it (`sub_duck.depth=0.5`)
//!   --space NAME=V|LO:HI     a space setting (`src/space.rs`, `spread-chords`)
//!                            pinned to V, or drawn from LO to HI, whatever
//!                            its range is: past it, for a width the approved
//!                            baseline never draws (queue item 99, widePad)
//!   --route sketch[.N]=LAYER the sketch block's mod N (0 if none is named)
//!                            driven by the layer's signal as it goes into the
//!                            mix (melody, chords, bass, texture, drums): a
//!                            sketch that hears the floor drop (queue item 99,
//!                            dropSpotlight). The sketch declares `MODS`
//!                            (`core/sketch/README.md`).
//!   --from S                 the clip starts S seconds in (rendered from
//!                            the start, written from S)
//!
//! The first six need a build with `--features sketch`; without it they
//! say so. `--from` works in any build.

use dlcore::Core;
use dlcore::player::Player;

/// What a clip asks for.
#[derive(Clone, Default)]
pub struct Clip {
    force: Vec<(String, String)>,
    sets: Vec<(String, f64)>,
    sketch: bool,
    /// Pins, resolved to (own block, setting, value).
    pub pins: Vec<(usize, usize, f64)>,
    /// Space settings to set: (setting, low, high).
    spaces: Vec<(usize, f64, f64)>,
    /// Routes into the sketch block: (mod, the layer's channel).
    routes: Vec<(usize, u32)>,
    /// Seconds trimmed off the head of the written clip.
    pub from: f64,
}

fn pair(word: &str, what: &str) -> Result<(String, String), String> {
    let (a, b) = word.split_once('=').ok_or_else(|| format!("{what} is NAME=VALUE, not {word}"))?;
    Ok((a.to_string(), b.to_string()))
}

fn value(word: &str, what: &str) -> Result<f64, String> {
    word.parse::<f64>().ok().filter(|x| x.is_finite()).ok_or_else(|| format!("{what}: '{word}' is not a number"))
}

impl Clip {
    /// Take `arg` (and its value, from `args`) if it is one of these
    /// options: true if it was.
    pub fn take(&mut self, arg: &str, args: &mut impl Iterator<Item = String>) -> Result<bool, String> {
        let mut next = || args.next().ok_or_else(|| format!("{arg} needs a value"));
        match arg {
            "--force" => self.force.push(pair(&next()?, "--force")?),
            "--set" => {
                let (name, v) = pair(&next()?, "--set")?;
                self.sets.push((name, value(&v, "--set")?));
            }
            "--sketch" => self.sketch = true,
            "--pin" => {
                let (name, v) = pair(&next()?, "--pin")?;
                self.pins.push(resolve_pin(&name, value(&v, "--pin")?)?);
            }
            "--space" => {
                let (name, v) = pair(&next()?, "--space")?;
                self.spaces.push(resolve_space(&name, &v)?);
            }
            "--route" => {
                let (name, layer) = pair(&next()?, "--route")?;
                self.routes.push(resolve_route(&name, &layer)?);
            }
            "--from" => self.from = value(&next()?, "--from")?.max(0.0),
            _ => return Ok(false),
        }
        Ok(true)
    }

    /// Whether any lever needs the sketch build.
    #[cfg(not(feature = "sketch"))]
    fn wants_sketch(&self) -> bool {
        !self.force.is_empty() || !self.sets.is_empty() || self.sketch || !self.pins.is_empty() || !self.spaces.is_empty() || !self.routes.is_empty()
    }

    /// Set the levers on the core and the player, before the loop is asked
    /// for.
    #[cfg(feature = "sketch")]
    pub fn apply(&self, core: &mut Core, player: &mut Player) -> Result<(), String> {
        for (name, v) in &self.sets {
            if !dlcore::tuning::set(name, *v) {
                return Err(format!("--set: no constant '{name}' (one of {})", dlcore::tuning::NAMES.join(", ")));
            }
        }
        for (id, option) in &self.force {
            let option = (option != "today").then_some(option.as_str());
            player.force_choice(id, option).map_err(|e| format!("--force {id}={}: {e}", option.unwrap_or("today")))?;
        }
        for &(which, lo, hi) in &self.spaces {
            if !core.space_range(which, lo, hi) {
                return Err(format!("--space: {lo} to {hi} is not a range for {}", dlcore::space::SETTINGS[which].name));
            }
        }
        // The block starts off in every channel: on where the data says.
        if self.sketch || !self.routes.is_empty() {
            let channel = sketch_channel()?;
            if self.sketch && !core.set(channel, dlcore::blocks::own::SKETCH, dlcore::blocks::sketch::ON, 1.0, 0.0) {
                return Err("the sketch block did not take its setting".to_string());
            }
            for &(which, from) in &self.routes {
                let tap = Some(dlcore::Tap { channel: from, block: None });
                if !core.route(channel, dlcore::blocks::own::SKETCH, which, tap, 0.0) {
                    return Err(format!("--route: the sketch block has no mod {which} (declare `const MODS` in core/sketch/block.rs; see sub_duck.rs)"));
                }
            }
        }
        Ok(())
    }

    /// Without the sketch build the levers say so.
    #[cfg(not(feature = "sketch"))]
    pub fn apply(&self, _: &mut Core, _: &mut Player) -> Result<(), String> {
        if self.wants_sketch() {
            return Err("--force, --set, --sketch and --pin need a build with --features sketch".to_string());
        }
        Ok(())
    }
}

/// `--space`'s setting and range: `NAME=V` or `NAME=LO:HI`.
#[cfg(feature = "sketch")]
fn resolve_space(name: &str, v: &str) -> Result<(usize, f64, f64), String> {
    let which = dlcore::space::SETTINGS
        .iter()
        .position(|s| s.name == name)
        .ok_or_else(|| format!("--space: no setting '{name}' (one of {})", dlcore::space::SETTINGS.iter().map(|s| s.name).collect::<Vec<_>>().join(", ")))?;
    let (lo, hi) = match v.split_once(':') {
        Some((lo, hi)) => (value(lo, "--space")?, value(hi, "--space")?),
        None => (value(v, "--space")?, value(v, "--space")?),
    };
    Ok((which, lo, hi))
}

#[cfg(not(feature = "sketch"))]
fn resolve_space(_: &str, _: &str) -> Result<(usize, f64, f64), String> {
    Err("--space needs a build with --features sketch".to_string())
}

/// `--route`'s block (only `sketch[.N]`: the mod N) and the layer it is
/// taken from.
fn resolve_route(name: &str, layer: &str) -> Result<(usize, u32), String> {
    let which = match name.split_once('.') {
        None if name == "sketch" => 0,
        Some(("sketch", n)) => n.parse::<usize>().map_err(|_| format!("--route: '{n}' is not a mod number"))?,
        _ => return Err(format!("--route is sketch[.N]=LAYER, not {name}={layer}")),
    };
    let from = match layer {
        "melody" => dlcore::MELODY,
        "chords" => dlcore::CHORDS,
        "bass" => dlcore::BASS_CHANNEL,
        "texture" => dlcore::TEXTURE_CHANNEL,
        "drums" => dlcore::DRUMS_CHANNEL,
        other => return Err(format!("--route: '{other}' is not a layer (melody, chords, bass, texture, drums)")),
    };
    Ok((which, from))
}

/// The channel `core/sketch/where.txt` names.
#[cfg(feature = "sketch")]
fn sketch_channel() -> Result<u32, String> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/sketch/where.txt");
    let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let line = text.lines().map(str::trim).find(|l| !l.is_empty() && !l.starts_with('#')).ok_or_else(|| format!("{path}: no line"))?;
    match line.split_whitespace().collect::<Vec<_>>().as_slice() {
        ["channel", layer] => match *layer {
            "melody" => Ok(dlcore::MELODY),
            "chords" => Ok(dlcore::CHORDS),
            "bass" => Ok(dlcore::BASS_CHANNEL),
            "texture" => Ok(dlcore::TEXTURE_CHANNEL),
            "drums" => Ok(dlcore::DRUMS_CHANNEL),
            other => Err(format!("{path}: '{other}' is not a layer (melody, chords, bass, texture, drums)")),
        },
        _ => Err(format!("{path}: a line is 'channel LAYER', not '{line}'")),
    }
}

#[cfg(feature = "sketch")]
fn resolve_pin(name: &str, v: f64) -> Result<(usize, usize, f64), String> {
    use dlcore::block::{Block, Channel};
    use dlcore::blocks::{self, Visit, own};

    /// The setting names of the channel block called `want`.
    struct Names<'a> {
        want: &'a str,
        found: Option<Vec<&'static str>>,
    }
    impl Visit for Names<'_> {
        fn voice<V: dlcore::block::Voice + Default>(&mut self, _: &'static str) {}
        fn part<B: Block + Default>(&mut self, _: &'static str) {}
        fn channel<C: Channel + Default>(&mut self, name: &'static str) {
            if name == self.want {
                self.found = Some(<C as Block>::SETTINGS.iter().map(|s| s.name).collect());
            }
        }
    }
    let (block, setting) = name.split_once('.').ok_or_else(|| format!("--pin is BLOCK.SETTING=VALUE, not {name}"))?;
    let number = match block {
        "organ" => own::ORGAN,
        "wash" => own::WASH,
        "sub_duck" => own::SUB_DUCK,
        "pad_sweep" => own::PAD_SWEEP,
        "sketch" => own::SKETCH,
        _ => return Err(format!("--pin: no channel block '{block}' (organ, wash, sub_duck, pad_sweep, sketch)")),
    };
    let mut names = Names { want: block, found: None };
    blocks::each(&mut names);
    let settings = names.found.unwrap_or_default();
    let at = settings.iter().position(|s| *s == setting).ok_or_else(|| format!("--pin: '{block}' has no setting '{setting}' (one of {})", settings.join(", ")))?;
    Ok((number, at, v))
}

#[cfg(not(feature = "sketch"))]
fn resolve_pin(_: &str, _: f64) -> Result<(usize, usize, f64), String> {
    Err("--pin needs a build with --features sketch".to_string())
}
