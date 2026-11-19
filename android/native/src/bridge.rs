//! The page's side of the native player (session 27, queue item 107c).
//!
//! In the app the page's player talks to this instead of a worklet:
//! `js/core.js`'s `CoreHost` posts the same messages (`playLoad`, `param`,
//! `probe` and the rest, as text over the app's bridge, `js/native.js`), and
//! hears the same messages back (`events` with their grids, `cost`, `pong`,
//! `probe`, `picks`, `failed`), batched a few times a second as the worklet
//! batches them. `receive` is the worklet's `receive`: each message becomes
//! one of the engine's commands, with its request built here on the app's
//! thread by the same calls in the same order as the worklet's `ask` (so the
//! same parts are refused, and counted); `Drain` is the worklet's posting:
//! the engine's reports, read off its queue back, as the worklet's messages.
//! Nothing here runs on the audio thread, so it may allocate.
//!
//! One message more than the worklet's: `native`, the stream's figures and
//! the core's clock, which `js/native.js` keeps as its context's (the page's
//! `ctx.currentTime` is the core's clock, as a browser's context's is).

use std::fmt::Write as _;
use std::time::{Duration, Instant};

use dlcore::generator::LAYERS;
use dlcore::generator::data::VIBE_COUNT;
use dlcore::generator::request::{LayerAsk, Length, NO_SUCH_VIBE};
use dlcore::player::{Fades, Handover, Jump, Moment, Request, Track};

use crate::engine::{Arm, ArmEnd, Command, Engine, GridView, Out, Position, ProbeAnswer, Tally, at};
use crate::json::{self, Value, number, string};

/// The player's events are posted every this long, or at once with one that
/// is not a step (`js/worklet.js`, `EVENTS_EVERY`).
const EVENTS_EVERY: Duration = Duration::from_millis(50);
/// What rendering costs, every this many seconds of audio (`COST_EVERY`).
const COST_EVERY: f64 = 2.0;
/// The stream's figures and the clock, at least this often.
const NATIVE_EVERY: Duration = Duration::from_millis(250);
/// How busy the audio thread is, a window this many seconds of audio.
const BUSY_WINDOW: f64 = 0.25;
/// The kinds of event (`Kind::ALL`): a step's, and what the layers sounded
/// on one (session 30b), which ride with the steps rather than post at once.
const STEP: u8 = 0;
const HEARD: u8 = 9;

/// The stream's figures for the `native` message: what `aaudio::stream()`
/// and the engine's counts say.
#[derive(Clone, Copy, Debug, Default)]
pub struct Figures {
    pub running: bool,
    pub rate: f64,
    pub underruns: i64,
    pub burst: i32,
    pub buffer: i32,
    pub capacity: i32,
    pub channels: i32,
    pub performance: i32,
    pub sharing: i32,
    pub policy: i32,
    pub priority: i32,
    pub reopens: u32,
    pub busiest: f64,
    pub mean: f64,
    pub callbacks: u64,
    pub refused: u32,
    pub lost: u32,
    /// The stream's latency to the speaker as it measures it, in seconds (0
    /// where it can't say: the buffer stands in).
    pub latency: f64,
}

/// What a message asks of the next loop, built as the worklet's `ask`
/// builds it: the request, how many of its parts were refused, and the
/// first refusal's words.
pub struct Asked {
    pub request: Request,
    pub refused: u32,
    pub why: Option<&'static str>,
}

/// `js/worklet.js`'s `ask`, call for call (`dl_play_vibe` and the rest, in
/// `core/src/wasm.rs`): each part of `a` the core may refuse, in the same
/// order, each refusal counted and the first one's words kept.
#[must_use]
pub fn ask(a: &Value) -> Asked {
    let mut r = Request::new();
    let mut refused = 0;
    let mut why = None;
    if !a.truthy() {
        return Asked { request: r, refused, why };
    }
    let mut take = |next: Result<Request, &'static str>, r: &mut Request| match next {
        Ok(n) => *r = n,
        Err(w) => {
            refused += 1;
            if !w.is_empty() {
                why.get_or_insert(w);
            }
        }
    };
    let index = |v: &Value| v.u32() as usize;
    for v in a.get("vibes").items() {
        let (i, w) = (index(v.at(0)), v.at(1));
        if w.is_null() {
            // `dl_play_vibe`: the vibe added even if the core has no such
            // one, and refused then.
            r = r.with(i);
            if i >= VIBE_COUNT {
                take(Err(NO_SUCH_VIBE), &mut r);
            }
        } else {
            take(r.with_weight(i, w.num()), &mut r);
        }
    }
    let tempo = a.get("tempo");
    if tempo.truthy() {
        take(r.with_tempo(tempo.at(0).num(), tempo.at(1).num()), &mut r);
    }
    for p in a.get("pins").items() {
        take(r.with_pin(index(p.at(0)), p.at(1).num()), &mut r);
    }
    if !a.get("theme").is_null() {
        r = r.with_theme(a.get("theme").u32());
    }
    let length = a.get("length");
    if !length.is_null() {
        let l = if length.str() == Some("endless") { Length::Endless } else { Length::Bars(length.num()) };
        take(r.with_length(l), &mut r);
    }
    if a.get("keys").truthy() {
        take(r.with_keys(a.get("keys").u32()), &mut r);
    }
    if a.get("scales").truthy() {
        take(r.with_scales(a.get("scales").u32()), &mut r);
    }
    if a.get("styles").truthy() {
        take(r.with_styles(a.get("styles").u32()), &mut r);
    }
    for (layer, mask) in a.get("voices").items().iter().enumerate() {
        if mask.truthy() {
            take(r.with_voices(layer, mask.u32()), &mut r);
        }
    }
    for (layer, state) in a.get("layers").items().iter().enumerate() {
        if state.truthy() {
            match state.u32() {
                1 => take(r.with_layer(layer, LayerAsk::On), &mut r),
                2 => take(r.with_layer(layer, LayerAsk::Off), &mut r),
                // `dl_play_layer` refuses a state there is no such number
                // for, with no words.
                _ => take(Err(""), &mut r),
            }
        }
    }
    for s in a.get("seeds").items() {
        take(r.with_layer_seed(index(s.at(0)), s.at(1).u32(), s.at(2).truthy()), &mut r);
    }
    for (which, v) in a.get("tone").items().iter().enumerate() {
        take(r.with_tone(which, v.num()), &mut r);
    }
    if !a.get("bpm").is_null() {
        take(r.with_bpm(a.get("bpm").num()), &mut r);
    }
    Asked { request: r, refused, why }
}

/// The track `kind` and `count` say (`dl_play_track`'s numbers, the core's
/// `Track::of`: a time and its fades among them, session 30c), or none.
fn track_of(kind: u32, count: u32) -> Option<(Track, Fades)> {
    Track::of(kind, count)
}

/// A crossfade's seconds as the core takes them: a negative one rings out.
fn handover(fade: f64) -> Handover {
    if fade < 0.0 { Handover::RingOut } else { Handover::Crossfade(fade) }
}

/// The page's side: what it has been refused, and what waits to be posted.
pub struct Bridge {
    /// The parts of its requests the core refused (the worklet's `refused`).
    pub refused: u32,
    /// Events waiting to be posted (four numbers each), the grids read with
    /// them (by the event's place), and whether one is not a step.
    events: Vec<f64>,
    grids: Vec<(usize, [Vec<f64>; 5])>,
    urgent: bool,
    events_at: Option<Instant>,
    /// Pongs, probes and the rest, waiting to be posted.
    replies: Vec<String>,
    /// The core's clock at the last cost report, and the last `native`.
    cost_at: Option<f64>,
    native_at: Option<Instant>,
}

impl Default for Bridge {
    fn default() -> Self {
        Self::new()
    }
}

impl Bridge {
    #[must_use]
    pub const fn new() -> Self {
        Bridge { refused: 0, events: Vec::new(), grids: Vec::new(), urgent: false, events_at: None, replies: Vec::new(), cost_at: None, native_at: None }
    }

    /// A message from the page, as text: the command it asks for queued on
    /// `engine`, and any answer due at once (the picks' `picks`) added to
    /// what `drain` posts next. `init` is the caller's (it starts the
    /// stream first): this is every other message. Returns false for text
    /// that is not a message, or a command the queue refused.
    pub fn receive(&mut self, engine: &Engine, m: &Value) -> bool {
        let Some(command) = self.command(m) else { return m.get("type").str().is_some() };
        engine.send(&command)
    }

    /// A page's player begins (the worklet's constructor): what is waiting
    /// from the last is dropped, and the refusals counted from nothing.
    pub fn begin(&mut self) {
        self.refused = 0;
        self.events.clear();
        self.grids.clear();
        self.urgent = false;
        self.cost_at = None;
    }

    /// A message, as the worklet's `receive` reads it, as a command (none
    /// for one that asks nothing of the player, or one it does not know).
    #[allow(clippy::too_many_lines, reason = "one arm a message, as the worklet's `receive`")]
    pub fn command(&mut self, m: &Value) -> Option<Command> {
        let ty = m.get("type").str()?;
        let num = |k: &str| m.get(k).num();
        let seed = || m.get("seed").u32();
        Some(match ty {
            "layout" => Command::Layout(m.get("channels").u32()),
            "cancel" => Command::Cancel(if m.get("from").is_null() { None } else { Some(num("from")) }),
            "param" => Command::Param { id: m.get("id").u32(), op: m.get("op").u32(), value: num("value"), time: num("time"), tau: num("tau") },
            "dispose" => Command::Dispose,
            "playLoad" => {
                let request = self.asked(m.get("ask"));
                Command::LoadAt { seed: seed(), request, at: m.get("at").num_or_zero(), resumed: m.get("resumed").truthy() }
            }
            "playChange" => {
                let request = self.asked(m.get("ask"));
                let moment = Moment::of(m.get("moment").u32())?;
                Command::Change { seed: seed(), request, moment, handover: handover(num("fade")) }
            }
            "playRevise" => {
                let request = self.asked(m.get("ask"));
                Command::Revise { seed: seed(), request }
            }
            "playTrack" => {
                let track = track_of(m.get("kind").u32(), m.get("count").u32());
                let arm = self.arm(m.get("arm"));
                self.urgent = true;
                Command::Track { track, arm }
            }
            "playArm" => Command::Arm(self.arm(m.get("arm"))),
            "playPicks" => {
                // The picks say their own refusals, and are held whole only
                // if nothing was refused (`dl_play_picks`).
                let a = ask(m.get("ask"));
                let tag = m.get("tag").u32();
                let met = a.why.is_none();
                let mut reply = String::from("{\"type\":\"picks\",\"tag\":");
                number(&mut reply, f64::from(tag));
                let _ = write!(reply, ",\"met\":{met},\"why\":");
                match a.why {
                    Some(w) => string(&mut reply, w),
                    None => reply.push_str("null"),
                }
                reply.push('}');
                self.replies.push(reply);
                Command::Picks { request: if met { a.request } else { a.request.vibes_only() }, tag }
            }
            "playDraws" => Command::Draws(seed()),
            "playNext" => {
                self.urgent = true;
                Command::NextAt { moment: Moment::of(m.get("moment").u32()), handover: handover(num("fade")) }
            }
            "playJump" => {
                self.urgent = true;
                let at = num("at");
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "a whole section number, saturating")]
                let to = match m.get("kind").u32() {
                    0 if at.is_finite() && at >= 0.0 => Some(Jump::Section(at as u64)),
                    1 => Some(Jump::Bar(at)),
                    2 => Some(Jump::Seconds(at)),
                    _ => None,
                };
                Command::Jump(to)
            }
            "playTone" => Command::Tone([num("warmth"), num("space"), num("wobble")]),
            "probe" => Command::Probe(num("id")),
            "playStop" => Command::Stop,
            "playResume" => Command::Resume(m.get("at").num_or_zero()),
            "playWander" => Command::WanderSalted { amount: num("amount"), salt: m.get("salt").u32() },
            "playMute" => {
                let layer = *LAYERS.get(m.get("layer").u32() as usize)?;
                Command::Mute { layer, muted: m.get("muted").truthy() }
            }
            "playTempo" => Command::Tempo(num("bpm")),
            "playVolume" => Command::Volume(num("volume")),
            "ping" => Command::Ping(num("id")),
            _ => return None,
        })
    }

    /// A message's ask as the request it builds, its refusals counted.
    fn asked(&mut self, a: &Value) -> Request {
        let a = ask(a);
        self.refused += a.refused;
        a.request
    }

    /// The loop that follows the track's end (`arm`).
    fn arm(&mut self, a: &Value) -> ArmEnd {
        if !a.truthy() {
            return ArmEnd::Drop;
        }
        let request = self.asked(a.get("ask"));
        let t = a.get("track");
        let track = if t.truthy() {
            match track_of(t.at(0).u32(), t.at(1).u32()) {
                Some(track) => Some(track),
                None => return ArmEnd::Keep,
            }
        } else {
            None
        };
        ArmEnd::Set(Arm { seed: a.get("seed").u32(), request, fade: a.get("fade").num(), track })
    }

    /// What is due to the page now, as one batch (a JSON array of the
    /// worklet's messages, and `native`), or none: the engine's reports
    /// taken off its queue back, the events posted every 50 ms or at once
    /// with one that is not a step, the cost every two seconds of audio.
    /// `figures` are the stream's.
    pub fn drain(&mut self, engine: &Engine, figures: &Figures, now: Instant) -> Option<String> {
        let status = engine.status.read();
        let epoch = status[at::EPOCH];
        let tally = tally_of(&status);
        let player = [status[at::STATE], status[at::STEP], status[at::STEP_TIME]];
        let refused = self.refused;
        let (events, grids, replies) = (&mut self.events, &mut self.grids, &mut self.replies);
        let mut urgent = false;
        engine.take(|out, grid, probe| match out {
            Out::Event { epoch: e, time, kind, number, step, grid: _ } => {
                if f64::from(e) < epoch {
                    return;
                }
                if kind != STEP && kind != HEARD {
                    urgent = true;
                }
                if let Some(g) = grid {
                    grids.push((events.len() / 4, grid_of(g)));
                }
                #[allow(clippy::cast_precision_loss, reason = "numbers and steps, far below 2^53")]
                events.extend_from_slice(&[time, f64::from(kind), number as f64, step as f64]);
            }
            Out::Pong { epoch: e, id, tally, position, busy, reduction, deepest } => {
                if f64::from(e) < epoch {
                    return;
                }
                let mut s = String::from("{\"type\":\"pong\",\"id\":");
                number(&mut s, id);
                counts(&mut s, &tally);
                let _ = write!(s, ",\"busy\":{busy},\"reduction\":[");
                number(&mut s, f64::from(reduction[0]));
                s.push(',');
                number(&mut s, f64::from(reduction[1]));
                s.push_str("],\"deepest\":[");
                number(&mut s, f64::from(deepest[0]));
                s.push(',');
                number(&mut s, f64::from(deepest[1]));
                s.push(']');
                player_of(&mut s, &position);
                s.push('}');
                replies.push(s);
            }
            Out::Probe { epoch: e, id } => {
                if f64::from(e) < epoch {
                    return;
                }
                if let Some(a) = probe {
                    replies.push(probe_of(id, a, refused));
                }
            }
        });
        self.urgent |= urgent;

        let mut batch: Vec<String> = Vec::new();
        batch.append(&mut self.replies);
        let due = self.events_at.is_none_or(|t| now.duration_since(t) >= EVENTS_EVERY);
        if self.urgent || (!self.events.is_empty() && due) {
            batch.push(self.events_message(&status));
            self.events.clear();
            self.grids.clear();
            self.urgent = false;
            self.events_at = Some(now);
        }
        if figures.running && self.cost_at.is_none_or(|t| status[at::TIME] - t >= COST_EVERY) {
            self.cost_at = Some(status[at::TIME]);
            batch.push(cost_of(engine, &status, &tally, &player));
        }
        if !batch.is_empty() || self.native_at.is_none_or(|t| now.duration_since(t) >= NATIVE_EVERY) {
            self.native_at = Some(now);
            batch.push(native_of(engine, &status, figures));
        }
        if batch.is_empty() {
            return None;
        }
        let mut out = String::from("[");
        out.push_str(&batch.join(","));
        out.push(']');
        Some(out)
    }

    /// The worklet's `events` message, from what waits.
    fn events_message(&self, status: &[f64; at::WORDS]) -> String {
        let mut s = String::from("{\"type\":\"events\",\"events\":[");
        for (i, x) in self.events.iter().enumerate() {
            if i > 0 {
                s.push(',');
            }
            number(&mut s, *x);
        }
        s.push_str("],\"grids\":[");
        for (i, (place, layers)) in self.grids.iter().enumerate() {
            if i > 0 {
                s.push(',');
            }
            let _ = write!(s, "{{\"at\":{place},\"grid\":[");
            for (l, steps) in layers.iter().enumerate() {
                if l > 0 {
                    s.push(',');
                }
                numbers(&mut s, steps);
            }
            s.push_str("]}");
        }
        s.push_str("],\"trackEnd\":");
        number(&mut s, status[at::TRACK_END]);
        s.push_str(",\"lost\":");
        number(&mut s, status[at::LOST]);
        s.push_str(",\"refused\":");
        number(&mut s, f64::from(self.refused));
        s.push_str(",\"place\":");
        place_of(&mut s, status);
        s.push('}');
        s
    }
}

/// The counts in the snapshot.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "counts written from u32s")]
fn tally_of(status: &[f64; at::WORDS]) -> Tally {
    let n = |i: usize| status[i] as u32;
    Tally { late: n(at::LATE), missed: n(at::MISSED), dropped: n(at::DROPPED), by: [n(at::DROPPED_BY), n(at::DROPPED_BY + 1), n(at::DROPPED_BY + 2)] }
}

/// A grid slot's layers, as the worklet's `grid` (an array a layer).
fn grid_of(g: &GridView) -> [Vec<f64>; 5] {
    std::array::from_fn(|l| g.layer(l).to_vec())
}

fn numbers(s: &mut String, xs: &[f64]) {
    s.push('[');
    for (i, x) in xs.iter().enumerate() {
        if i > 0 {
            s.push(',');
        }
        number(s, *x);
    }
    s.push(']');
}

/// The worklet's `counts()`, as fields of the object being written.
fn counts(s: &mut String, t: &Tally) {
    let _ = write!(s, ",\"late\":{},\"missed\":{},\"dropped\":{},\"droppedBy\":[{},{},{}]", t.late, t.missed, t.dropped, t.by[0], t.by[1], t.by[2]);
}

/// The worklet's `player` field.
fn player_of(s: &mut String, p: &Position) {
    s.push_str(",\"player\":{\"state\":");
    number(s, p[0]);
    s.push_str(",\"step\":");
    number(s, p[1]);
    s.push_str(",\"time\":");
    number(s, p[2]);
    s.push('}');
}

/// The worklet's `place()`: null with nothing playing.
fn place_of(s: &mut String, status: &[f64; at::WORDS]) {
    if status[at::PLACE] == 0.0 {
        s.push_str("null");
        return;
    }
    s.push_str("{\"bar\":");
    number(s, status[at::BAR]);
    s.push_str(",\"bars\":");
    if status[at::BARS] < 0.0 { s.push_str("null") } else { number(s, status[at::BARS]) }
    s.push_str(",\"played\":");
    number(s, status[at::PLAYED]);
    s.push_str(",\"left\":");
    if status[at::LEFT] < 0.0 { s.push_str("null") } else { number(s, status[at::LEFT]) }
    s.push_str(",\"section\":");
    number(s, status[at::SECTION]);
    s.push('}');
}

/// The worklet's `probe` answer.
fn probe_of(id: f64, a: &ProbeAnswer, refused: u32) -> String {
    let mut s = String::from("{\"type\":\"probe\",\"id\":");
    number(&mut s, id);
    counts(&mut s, &a.tally);
    s.push_str(",\"about\":");
    string(&mut s, &String::from_utf8_lossy(&a.about[..a.len.min(a.about.len())]));
    let _ = write!(s, ",\"lanes\":{}", a.lanes);
    player_of(&mut s, &a.position);
    s.push_str(",\"trackEnd\":");
    number(&mut s, a.track_end);
    s.push_str(",\"trackAt\":");
    number(&mut s, a.track_at);
    s.push_str(",\"nextChange\":");
    number(&mut s, a.next_change);
    let _ = write!(s, ",\"lost\":{},\"refused\":{refused},\"picksTag\":{}", a.lost, a.picks_tag);
    s.push('}');
    s
}

/// The worklet's `cost` report: the counts, the cost so far each layout
/// apart, where the player is, and how busy the audio thread has been since
/// the last.
fn cost_of(engine: &Engine, status: &[f64; at::WORDS], tally: &Tally, player: &Position) -> String {
    let (max, behind, windows) = engine.take_busy();
    let mut s = String::from("{\"type\":\"cost\"");
    counts(&mut s, tally);
    s.push_str(",\"ms\":");
    numbers(&mut s, &[status[at::COST_MS], status[at::COST_MS + 1]]);
    s.push_str(",\"seconds\":");
    numbers(&mut s, &[status[at::COST_SECONDS], status[at::COST_SECONDS + 1]]);
    player_of(&mut s, player);
    s.push_str(",\"busy\":{\"max\":");
    number(&mut s, max);
    let _ = write!(s, ",\"behind\":{behind},\"windows\":{windows},\"seconds\":");
    number(&mut s, f64::from(windows) * BUSY_WINDOW);
    s.push_str("}}");
    s
}

/// The `native` message: the core's clock, the stream's figures and how
/// loud the output was, for `js/native.js` (its context) and Diagnostics.
fn native_of(engine: &Engine, status: &[f64; at::WORDS], f: &Figures) -> String {
    let mut s = String::from("{\"type\":\"native\",\"time\":");
    number(&mut s, status[at::TIME]);
    let _ = write!(s, ",\"running\":{},\"rate\":", f.running);
    number(&mut s, f.rate);
    // How long a frame the core renders now takes to reach the speaker
    // (session 30b): as the stream measures it, or where it can't yet, what
    // is queued ahead of the speaker, the stream's buffer. The layers'
    // playhead reads it (js/native.js, `outputLatency`).
    s.push_str(",\"latency\":");
    let buffered = if f.rate > 0.0 { f64::from(f.buffer) / f.rate } else { 0.0 };
    number(&mut s, if f.latency > 0.0 { f.latency } else { buffered });
    let _ = write!(s, ",\"measured\":{}", u8::from(f.latency > 0.0));
    let _ = write!(
        s,
        ",\"underruns\":{},\"burst\":{},\"buffer\":{},\"capacity\":{},\"channels\":{},\"performance\":{},\"sharing\":{},\"policy\":{},\"priority\":{},\"reopens\":{},\"callbacks\":{},\"queueRefused\":{},\"reportsLost\":{}",
        f.underruns, f.burst, f.buffer, f.capacity, f.channels, f.performance, f.sharing, f.policy, f.priority, f.reopens, f.callbacks, f.refused, f.lost
    );
    s.push_str(",\"busiest\":");
    number(&mut s, f.busiest);
    s.push_str(",\"mean\":");
    number(&mut s, f.mean);
    s.push_str(",\"level\":");
    number(&mut s, engine.take_level());
    s.push_str(",\"sounding\":");
    number(&mut s, if f.rate > 0.0 { status[at::SOUNDING] / f.rate } else { 0.0 });
    s.push_str(",\"layout\":");
    number(&mut s, status[at::LAYOUT]);
    s.push('}');
    s
}

/// The `init` message's player: the full mix unless it asks for lite, and
/// its layout (1 mono, else 2).
#[must_use]
pub fn init_of(m: &Value) -> (bool, u32) {
    let full = m.get("quality").str() != Some("lite");
    let layout = if m.get("layout").u32() == 2 { 2 } else { 1 };
    (full, layout)
}

/// A message's text read, or why not.
///
/// # Errors
///
/// Text that is not JSON.
pub fn read(text: &str) -> Result<Value, &'static str> {
    json::parse(text)
}

/// The `failed` message (the worklet's when its core can't run): the page
/// says it cannot play, and why.
#[must_use]
pub fn failed(why: &str) -> String {
    let mut s = String::from("[{\"type\":\"failed\",\"message\":");
    string(&mut s, why);
    s.push_str("}]");
    s
}

#[cfg(test)]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss, reason = "the tests' own counts, small")]
mod tests {
    use super::{Bridge, Figures, ask, init_of, read};
    use crate::engine::{Command, Engine};
    use crate::json::Value;
    use std::time::{Duration, Instant};

    const RATE: f64 = 48_000.0;

    /// The page's player starting, as `CoreHost` and `PlayerEngine._begin`
    /// send it: the engine, the bridge, and one message each.
    fn page(quality: &str) -> (Box<Engine>, Bridge) {
        let engine = Engine::new(RATE, 2, true).unwrap();
        let mut bridge = Bridge::new();
        let init = read(&format!(r#"{{"type":"init","quality":"{quality}","layout":2,"burn":0}}"#)).unwrap();
        let (full, layout) = init_of(&init);
        assert!(engine.send(&Command::Init { full, layout }));
        bridge.begin();
        (engine, bridge)
    }

    fn post(engine: &Engine, bridge: &mut Bridge, text: &str) {
        let m = read(text).unwrap();
        assert!(bridge.receive(engine, &m), "{text}");
    }

    /// Render `seconds` as a device would, 192 frames a callback, draining
    /// every 25 ms of audio as the app's thread does; every batch posted.
    fn play(engine: &Engine, bridge: &mut Bridge, seconds: f64, at: &mut Instant) -> Vec<Value> {
        let figures = Figures { running: true, rate: RATE, buffer: 384, ..Figures::default() };
        let mut out = vec![0.0f32; 192 * 2];
        let mut posted = Vec::new();
        let callbacks = (seconds * RATE / 192.0) as usize;
        for i in 0..callbacks {
            engine.fill(&mut out, 2);
            if i % 6 == 5 {
                *at += Duration::from_millis(24);
                if let Some(batch) = bridge.drain(engine, &figures, *at) {
                    match read(&batch).unwrap() {
                        Value::Arr(items) => posted.extend(items),
                        _ => panic!("a batch is an array"),
                    }
                }
            }
        }
        posted
    }

    fn of_type<'a>(posted: &'a [Value], ty: &str) -> Vec<&'a Value> {
        posted.iter().filter(|m| m.get("type").str() == Some(ty)).collect()
    }

    /// A loop through the page's messages: it plays, its events come back
    /// in the worklet's shape (steps with grids at a pass, the loop's own
    /// event with its seed), the cost and the clock with them, a probe
    /// describes it, a ping answers after what came before it.
    #[test]
    fn the_pages_messages_play_a_loop_and_its_events_come_back() {
        let (engine, mut bridge) = page("full");
        let mut at = Instant::now();
        for text in [
            r#"{"type":"playVolume","volume":0.8}"#,
            r#"{"type":"playDraws","seed":20261117}"#,
            r#"{"type":"playWander","amount":1,"salt":0}"#,
            r#"{"type":"playLoad","seed":2693262067,"ask":{"vibes":[[0,null]],"tempo":null,"pins":[],"length":null,"keys":0,"scales":0,"styles":0,"voices":[0,0,0,0,0],"layers":[0,0,0,0,0],"seeds":[],"tone":null,"bpm":null},"at":0,"resumed":false}"#,
            r#"{"type":"playMute","layer":0,"muted":false}"#,
            r#"{"type":"playTrack","kind":0,"count":0,"arm":null}"#,
            r#"{"type":"param","id":18,"op":4,"value":0,"time":0,"tau":0}"#,
            r#"{"type":"param","id":18,"op":1,"value":1,"time":0.02,"tau":0}"#,
        ] {
            post(&engine, &mut bridge, text);
        }
        let posted = play(&engine, &mut bridge, 12.0, &mut at);
        let events = of_type(&posted, "events");
        assert!(events.len() > 1);
        let flat: Vec<f64> = events.iter().flat_map(|m| m.get("events").items().iter().map(Value::num)).collect();
        let kinds: Vec<u32> = flat.chunks(4).map(|e| e[1] as u32).collect();
        assert!(kinds.contains(&4), "a Loop event");
        let loop_at = kinds.iter().position(|k| *k == 4).unwrap();
        assert_eq!(flat[loop_at * 4 + 2] as u32, 2_693_262_067, "the Loop event's number is its seed");
        assert!(kinds.iter().filter(|k| **k == 0).count() > 30, "steps");
        let grids: Vec<&Value> = events.iter().flat_map(|m| m.get("grids").items()).collect();
        assert!(!grids.is_empty(), "a grid with the loop's first pass");
        assert_eq!(grids[0].get("grid").items().len(), 5);
        assert!(grids[0].get("grid").items().iter().any(|l| !l.items().is_empty()));
        assert!(events.iter().all(|m| !m.get("place").is_null() || m.get("place").is_null()));
        // Steps a sixteenth apart, as the core decides them.
        let steps: Vec<&[f64]> = flat.chunks(4).filter(|e| e[1] == 0.0).collect();
        assert!(steps.windows(2).all(|w| w[1][0] > w[0][0]));

        let costs = of_type(&posted, "cost");
        assert!(costs.len() >= 5, "a cost report every two seconds of audio: {}", costs.len());
        let last = costs.last().unwrap();
        assert!(last.get("seconds").at(1).num() > 10.0, "stereo seconds counted");
        assert!(last.get("busy").get("windows").num() >= 1.0);
        assert_eq!(last.get("late").num(), 0.0);
        let natives = of_type(&posted, "native");
        let t = natives.last().unwrap().get("time").num();
        assert!((t - 12.0).abs() < 0.1, "the core's clock: {t}");
        assert!(natives.iter().any(|m| m.get("level").num() > 1e-4), "something sounds");

        post(&engine, &mut bridge, r#"{"type":"probe","id":1000000001}"#);
        post(&engine, &mut bridge, r#"{"type":"ping","id":7}"#);
        let posted = play(&engine, &mut bridge, 0.5, &mut at);
        let probe = of_type(&posted, "probe");
        assert_eq!(probe.len(), 1);
        let p = probe[0];
        assert_eq!(p.get("id").num(), 1_000_000_001.0);
        assert_eq!(p.get("player").get("state").num(), 3.0);
        assert_eq!(p.get("lanes").num(), 2.0);
        let about = p.get("about").str().unwrap();
        assert!(about.contains("seed=2693262067"), "{about}");
        assert!(about.lines().any(|l| l.starts_with("name=")));
        let pong = of_type(&posted, "pong");
        assert_eq!(pong.len(), 1);
        assert_eq!(pong[0].get("id").num(), 7.0);
        assert_eq!(pong[0].get("player").get("state").num(), 3.0);
        assert_eq!(bridge.refused, 0);
    }

    /// The picks answer at once, held whole or as their vibes alone with the
    /// core's words; a request's refusals are counted as the worklet counts
    /// them, the picks' apart.
    #[test]
    fn the_picks_answer_and_refusals_are_counted() {
        let (engine, mut bridge) = page("lite");
        let mut at = Instant::now();
        post(&engine, &mut bridge, r#"{"type":"playPicks","ask":{"vibes":[[2,null]],"tempo":[300,400]},"tag":3}"#);
        post(&engine, &mut bridge, r#"{"type":"playPicks","ask":{"vibes":[[2,null]],"tempo":[70,90]},"tag":4}"#);
        assert_eq!(bridge.refused, 0, "the picks say their own refusals");
        post(&engine, &mut bridge, r#"{"type":"playLoad","seed":5,"ask":{"vibes":[[999,null]],"bpm":1000},"at":0}"#);
        assert_eq!(bridge.refused, 2, "no such vibe, and the tempo");
        let posted = play(&engine, &mut bridge, 0.2, &mut at);
        let picks = of_type(&posted, "picks");
        assert_eq!(picks.len(), 2);
        assert_eq!(picks[0].get("tag").num(), 3.0);
        assert!(!picks[0].get("met").truthy());
        assert!(picks[0].get("why").str().is_some_and(|w| w.contains("tempo")), "{:?}", picks[0].get("why"));
        assert!(picks[1].get("met").truthy() && picks[1].get("why").is_null());
        post(&engine, &mut bridge, r#"{"type":"probe","id":9}"#);
        let posted = play(&engine, &mut bridge, 0.2, &mut at);
        let p = of_type(&posted, "probe");
        assert_eq!(p[0].get("picksTag").num(), 4.0, "the core holds the last picks");
        assert_eq!(p[0].get("refused").num(), 2.0);
    }

    /// What `ask` reads, part by part, as the worklet's calls take them.
    #[test]
    fn ask_reads_every_part() {
        let a = read(r#"{"vibes":[[1,null],[3,0.5]],"tempo":[70,80],"pins":[],"theme":12345,"length":"endless","keys":5,"scales":0,"styles":0,"voices":[0,0,0,0,0],"layers":[0,1,0,2,0],"seeds":[[0,99,true]],"tone":[0.1,0.5,0.9],"bpm":75}"#).unwrap();
        let asked = ask(&a);
        assert_eq!(asked.refused, 0, "{:?}", asked.why);
        let bad = read(r#"{"layers":[3]}"#).unwrap();
        let asked = ask(&bad);
        assert_eq!((asked.refused, asked.why), (1, None), "a layer state there is no number for");
        assert_eq!(ask(&Value::Null).refused, 0);
    }

    /// A page's player let go: the next one hears nothing of the last, and
    /// its clock goes on.
    #[test]
    fn dispose_then_init_is_a_new_player_on_the_same_clock() {
        let (engine, mut bridge) = page("full");
        let mut at = Instant::now();
        post(&engine, &mut bridge, r#"{"type":"playLoad","seed":2693262067,"ask":null,"at":0}"#);
        let _ = play(&engine, &mut bridge, 4.0, &mut at);
        post(&engine, &mut bridge, r#"{"type":"dispose"}"#);
        assert!(engine.send(&Command::Init { full: false, layout: 1 }));
        bridge.begin();
        let mut posted = play(&engine, &mut bridge, 1.0, &mut at);
        post(&engine, &mut bridge, r#"{"type":"probe","id":1}"#);
        posted.extend(play(&engine, &mut bridge, 0.1, &mut at));
        assert!(of_type(&posted, "events").iter().all(|m| m.get("events").items().is_empty()), "no event of the last player");
        let p = of_type(&posted, "probe");
        assert_eq!(p[0].get("player").get("state").num(), 0.0, "idle");
        assert_eq!(p[0].get("lanes").num(), 1.0, "the new player's layout");
        let t = of_type(&posted, "native").last().unwrap().get("time").num();
        assert!(t > 4.9, "the clock went on: {t}");
    }
}
