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
//!   --from S                 the clip starts S seconds in (rendered from
//!                            the start, written from S)
//!
//! The first four need a build with `--features sketch`; without it they
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
            "--from" => self.from = value(&next()?, "--from")?.max(0.0),
            _ => return Ok(false),
        }
        Ok(true)
    }

    /// Whether any lever needs the sketch build.
    #[cfg(not(feature = "sketch"))]
    fn wants_sketch(&self) -> bool {
        !self.force.is_empty() || !self.sets.is_empty() || self.sketch || !self.pins.is_empty()
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
        // The block starts off in every channel: on where the data says.
        if self.sketch {
            let channel = sketch_channel()?;
            if !core.set(channel, dlcore::blocks::own::SKETCH, dlcore::blocks::sketch::ON, 1.0, 0.0) {
                return Err("the sketch block did not take its setting".to_string());
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
