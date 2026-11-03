//! The voices and the layers a request asks for (queue item 91;
//! `askedVoice`, `layerOn` and `onPick` in `js/generator.js`, draw for draw).
//!
//! Every draw a layer makes is made as ever, and the request then overrules
//! one value: a voice already inside the set asked stands, else one of the
//! set is drawn on a stream of the layer's own seed; a layer asked on is
//! never left out by its own chances. A layer asked off is the spec's mute
//! (`Spec::layers_off`), not a change to what is drawn.

use super::js;
use super::request::VOICE_ORDER;
use super::rng::Rng;
use super::spec::Spec;
use super::Layer;

const VOICE_SALT: u32 = 0x564f_4943;
const ON_SALT: u32 = 0x4c41_5950;
/// A voice the profiles' pool lacks is weighed at this when one is asked, a
/// quarter of a weight, so no pair is forbidden (for Mikey's ears).
const VOICE_FLOOR: f64 = 0.25;

/// `askedVoice`: the voice the layer plays, `drawn` from its `pool` unless
/// the request asked for others. Inside the set it stands; else one of the
/// set, weighted by how much the pool leans toward each.
#[must_use]
pub fn voice(spec: &Spec, layer: Layer, drawn: &'static str, pool: &[(&'static str, f64)]) -> &'static str {
    let wanted = spec.voices[layer as usize];
    let list = VOICE_ORDER[layer as usize];
    let asked = |i: usize| wanted & (1 << i) != 0;
    if wanted == 0 || list.iter().enumerate().any(|(i, &v)| asked(i) && v == drawn) {
        return drawn;
    }
    let weight = |i: usize| {
        let mut have = 0.0;
        for &(name, w) in pool {
            if name == list[i] {
                have += w;
            }
        }
        js::max(have, VOICE_FLOOR)
    };
    let members = (0..list.len()).filter(|&i| asked(i));
    let mut r = Rng::new(js::salted(spec.layer_seed(layer), VOICE_SALT, 41));
    let chosen = r.weighted_index(members.clone().map(weight)).and_then(|k| members.clone().nth(k));
    chosen.map_or(drawn, |i| list[i])
}

/// `layerOn`: whether the request asked the layer on.
#[must_use]
pub fn on(spec: &Spec, layer: Layer) -> bool {
    spec.layers_on & (1 << layer as usize) != 0
}

/// `onPick`: one of `pool` but not `not`, for a layer asked on whose draw
/// landed on the layer's rest; drawn on a stream of the layer's own seed.
#[must_use]
pub fn on_pick(spec: &Spec, layer: Layer, pool: &[(&'static str, f64)], not: &str) -> &'static str {
    let any_other = pool.iter().any(|e| e.0 != not);
    let keep = |e: &&(&'static str, f64)| !any_other || e.0 != not;
    let mut r = Rng::new(js::salted(spec.layer_seed(layer), ON_SALT, 43));
    // The pool without `not` (or all of it, were `not` the whole pool).
    let rest = pool.iter().filter(keep);
    let chosen = r.weighted_index(rest.clone().map(|e| e.1)).and_then(|i| rest.clone().nth(i));
    chosen.map_or("", |e| e.0)
}
