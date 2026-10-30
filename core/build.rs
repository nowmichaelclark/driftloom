//! The generation data, compiled in (queue item 79).
//!
//! The profiles, vibes and moods are plain text in `data/` (queue item 78),
//! one format the JavaScript generator reads at start (`js/data.js`) and
//! this script reads at build time: it parses the files and writes them out
//! as Rust tables (`$OUT_DIR/gen_data.rs`, included by `src/gen/data.rs`).
//! So the core carries the data as read-only statics: nothing to parse at
//! run time, nothing a host must hand over, no memory spent at `Core::new`,
//! and the same bytes on every target.
//!
//! The format and its readings are `js/data.js`'s, and this parser follows
//! it line for line: a comment from '#' to the end of a line, `[kind name]`
//! blocks, `key = rows` entries (rows split on ',', tokens on spaces), a line
//! that starts with a space or a tab continuing the entry above. A number is
//! read as the one nearest double (Rust's `f64::from_str`, correctly rounded,
//! as `Number` is), or a ratio `a/b`, one division; every number is written
//! out by its bits, so the table holds exactly the double JavaScript reads.
//! Anything malformed fails the build, naming the file, the block and the key.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::PathBuf;
use std::{env, fs};

struct Entry {
    key: String,
    rows: Vec<Vec<String>>,
}

struct Block {
    kind: String,
    name: String,
    entries: Vec<Entry>,
}

fn parse(text: &str, file: &str) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();
    let mut texts: Vec<Vec<(String, String)>> = Vec::new();
    for (i, raw) in text.split('\n').enumerate() {
        let n = i + 1;
        let mut line = raw.strip_suffix('\r').unwrap_or(raw);
        assert!(line.bytes().all(|c| c == b'\t' || (32..=126).contains(&c)), "{file}:{n}: plain printable ASCII only");
        if let Some(hash) = line.find('#') {
            line = &line[..hash];
        }
        if line.trim().is_empty() {
            continue;
        }
        if line.starts_with(' ') || line.starts_with('\t') {
            let block = texts.last_mut().unwrap_or_else(|| panic!("{file}:{n}: a continuation with nothing to continue"));
            let entry = block.last_mut().unwrap_or_else(|| panic!("{file}:{n}: a continuation with nothing to continue"));
            entry.1.push(' ');
            entry.1.push_str(line.trim());
            continue;
        }
        if let Some(head) = line.trim_end().strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            let (kind, name) = head.split_once(' ').unwrap_or_else(|| panic!("{file}:{n}: a block is [kind name]"));
            blocks.push(Block { kind: kind.to_string(), name: name.to_string(), entries: Vec::new() });
            texts.push(Vec::new());
            continue;
        }
        let (key, value) = line.split_once('=').unwrap_or_else(|| panic!("{file}:{n}: not a block, an entry or a comment"));
        let key = key.trim().to_string();
        assert!(key.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'.'), "{file}:{n}: a bad key");
        let block = texts.last_mut().unwrap_or_else(|| panic!("{file}:{n}: an entry before any block"));
        assert!(block.iter().all(|(k, _)| *k != key), "{file}:{n}: '{key}' twice in a block");
        block.push((key, value.trim().to_string()));
    }
    for (block, entries) in blocks.iter_mut().zip(texts) {
        for (key, text) in entries {
            let rows = text
                .split(',')
                .map(str::trim)
                .filter(|r| !r.is_empty())
                .map(|r| r.split_whitespace().map(str::to_string).collect())
                .collect();
            block.entries.push(Entry { key, rows });
        }
    }
    blocks
}

/// A token as a number, as `js/data.js` reads one.
fn number(token: &str, at: &str) -> f64 {
    if let Some((a, b)) = token.split_once('/') {
        let a: i64 = a.parse().unwrap_or_else(|_| panic!("{at}: '{token}' is not a number"));
        let b: i64 = b.parse().unwrap_or_else(|_| panic!("{at}: '{token}' is not a number"));
        #[allow(clippy::cast_precision_loss)] // small integers, exact
        return a as f64 / b as f64;
    }
    let ok = {
        let t = token.strip_prefix('-').unwrap_or(token);
        let (mant, exp) = match t.find(['e', 'E']) {
            Some(i) => (&t[..i], Some(&t[i + 1..])),
            None => (t, None),
        };
        let digits = |s: &str| !s.is_empty() && s.bytes().all(|c| c.is_ascii_digit());
        let mant_ok = match mant.split_once('.') {
            Some((a, b)) => (digits(a) && (b.is_empty() || digits(b))) || (a.is_empty() && digits(b)),
            None => digits(mant),
        };
        let exp_ok = exp.is_none_or(|e| digits(e.strip_prefix(['+', '-']).unwrap_or(e)));
        mant_ok && exp_ok
    };
    assert!(ok, "{at}: '{token}' is not a number");
    token.parse().unwrap_or_else(|_| panic!("{at}: '{token}' is not a number"))
}

fn f(x: f64) -> String {
    format!("f({:#018x})", x.to_bits())
}

fn quote(s: &str) -> String {
    format!("{s:?}")
}

struct Reader<'a> {
    file: &'a str,
    block: &'a Block,
}

impl Reader<'_> {
    fn at(&self, key: &str) -> String {
        format!("{}: [{} {}] {key}", self.file, self.block.kind, self.block.name)
    }
    fn rows(&self, key: &str) -> Option<&Vec<Vec<String>>> {
        self.block.entries.iter().find(|e| e.key == key).map(|e| &e.rows)
    }
    fn one(&self, key: &str) -> Option<&str> {
        self.rows(key).map(|rows| {
            assert!(rows.len() == 1 && rows[0].len() == 1, "{}: expected one token", self.at(key));
            rows[0][0].as_str()
        })
    }
    fn num(&self, key: &str) -> Option<f64> {
        self.one(key).map(|t| number(t, &self.at(key)))
    }
    fn range(&self, key: &str) -> Option<[f64; 2]> {
        self.rows(key).map(|rows| {
            assert!(rows.len() == 1 && rows[0].len() == 2, "{}: expected two numbers", self.at(key));
            [number(&rows[0][0], &self.at(key)), number(&rows[0][1], &self.at(key))]
        })
    }
    fn words(&self, key: &str) -> Option<Vec<String>> {
        self.rows(key).map(|rows| {
            assert!(rows.len() <= 1, "{}: expected one row", self.at(key));
            rows.first().cloned().unwrap_or_default()
        })
    }
    fn pool(&self, key: &str) -> Option<Vec<(String, f64)>> {
        self.rows(key).map(|rows| {
            rows.iter()
                .map(|r| {
                    assert!(r.len() == 2, "{}: expected a word and a number", self.at(key));
                    (r[0].clone(), number(&r[1], &self.at(key)))
                })
                .collect()
        })
    }
    fn numpool(&self, key: &str) -> Option<Vec<(f64, f64)>> {
        self.rows(key).map(|rows| {
            rows.iter()
                .map(|r| {
                    assert!(r.len() == 2, "{}: expected two numbers", self.at(key));
                    (number(&r[0], &self.at(key)), number(&r[1], &self.at(key)))
                })
                .collect()
        })
    }
    /// The keys this block may hold; any other fails the build.
    fn only(&self, known: &[&str]) {
        for e in &self.block.entries {
            let head = e.key.split('.').next().unwrap_or("");
            let ok = known.iter().any(|k| *k == e.key || k.strip_suffix(".*").is_some_and(|h| h == head && e.key.contains('.')));
            assert!(ok, "{}: [{} {}] has no key '{}'", self.file, self.block.kind, self.block.name, e.key);
        }
    }
}

fn opt<T>(v: Option<T>, show: impl Fn(T) -> String) -> String {
    v.map_or_else(|| "None".to_string(), |x| format!("Some({})", show(x)))
}

fn show_pool(p: Vec<(String, f64)>) -> String {
    let mut s = String::from("&[");
    for (k, w) in p {
        let _ = write!(s, "({}, {}),", quote(&k), f(w));
    }
    s.push(']');
    s
}

fn show_numpool(p: Vec<(f64, f64)>) -> String {
    let mut s = String::from("&[");
    for (k, w) in p {
        let _ = write!(s, "({}, {}),", f(k), f(w));
    }
    s.push(']');
    s
}

fn show_range(r: [f64; 2]) -> String {
    format!("[{}, {}]", f(r[0]), f(r[1]))
}

/// A degree in a progression, as `js/characters.js` reads one: a number, or
/// `4M` (a major chord), `6fM` (flattened, major).
fn degree(token: &str, at: &str) -> String {
    let digits: String = token.chars().take_while(char::is_ascii_digit).collect();
    let rest = &token[digits.len()..];
    assert!(!digits.is_empty() && ["", "f", "M", "fM"].contains(&rest), "{at}: '{token}' is not a degree");
    let d: i32 = digits.parse().unwrap_or_else(|_| panic!("{at}: '{token}' is not a degree"));
    let flat = rest.contains('f');
    let major = rest.contains('M');
    format!("Degree {{ d: {d}, flat: {flat}, major: {major}, spelled: {} }}", flat || major)
}

fn progression_rows(rows: &[Vec<String>], at: &str) -> String {
    let mut s = String::from("&[");
    for r in rows {
        s.push_str("&[");
        for t in r {
            s.push_str(&degree(t, at));
            s.push(',');
        }
        s.push_str("],");
    }
    s.push(']');
    s
}

const CHARACTER_KEYS: &[&str] = &[
    "label",
    "groove",
    "polymeter",
    "microLoop",
    "borrowedBright",
    "tone.*",
    "feel.*",
    "gaits",
    "progressions",
    "progressions.*",
    "bpm",
    "swing",
    "hatDensity",
    "stepsPerBar",
    "bars",
    "chordSize",
    "scales",
    "chordVoices",
    "melodyVoices",
    "bassStyles",
    "bassVoices",
    "textures",
    "kits",
    "weight",
    "level",
    "airy",
    "drums",
    "restBar",
    "flatSeven",
    "quartal",
    "pointillist",
    "skipStep",
    "glide",
    "dubEcho",
    "rolls",
    "fourFloor",
    "pump",
    "drone",
    "ornament",
    "harmonize",
    "chug",
];
const CHARACTER_NUMS: &[(&str, &str)] = &[
    ("weight", "weight"),
    ("level", "level"),
    ("airy", "airy"),
    ("drums", "drums"),
    ("restBar", "rest_bar"),
    ("flatSeven", "flat_seven"),
    ("quartal", "quartal"),
    ("pointillist", "pointillist"),
    ("skipStep", "skip_step"),
    ("glide", "glide"),
    ("dubEcho", "dub_echo"),
    ("rolls", "rolls"),
    ("fourFloor", "four_floor"),
    ("pump", "pump"),
    ("drone", "drone"),
    ("ornament", "ornament"),
    ("harmonize", "harmonize"),
    ("chug", "chug"),
];
const CHARACTER_POOLS: &[(&str, &str)] = &[
    ("scales", "scales"),
    ("chordVoices", "chord_voices"),
    ("melodyVoices", "melody_voices"),
    ("bassStyles", "bass_styles"),
    ("bassVoices", "bass_voices"),
    ("textures", "textures"),
    ("kits", "kits"),
];

fn flag(r: &Reader, key: &str) -> Option<bool> {
    r.one(key).map(|t| match t {
        "true" => true,
        "false" => false,
        _ => panic!("{}: '{t}' is not true or false", r.at(key)),
    })
}

#[allow(clippy::too_many_lines)] // one profile's every key, in the struct's order
fn characters(out: &mut String, blocks: &[Block]) -> Vec<String> {
    let file = "data/characters.txt";
    let mut ids = Vec::new();
    // The most names a pool of a blend can hold: every distinct name the
    // profiles give that pool (a blend's pools are unions).
    let mut union = 0;
    for (key, _) in CHARACTER_POOLS {
        let mut names = BTreeSet::new();
        for b in blocks.iter().filter(|b| b.kind == "character") {
            if let Some(p) = (Reader { file, block: b }).pool(key) {
                names.extend(p.into_iter().map(|(n, _)| n));
            }
        }
        union = union.max(names.len());
    }
    for key in ["stepsPerBar", "bars", "chordSize"] {
        let mut names = BTreeSet::new();
        for b in blocks.iter().filter(|b| b.kind == "character") {
            if let Some(p) = (Reader { file, block: b }).numpool(key) {
                names.extend(p.into_iter().map(|(n, _)| n.to_bits()));
            }
        }
        union = union.max(names.len());
    }
    let _ = writeln!(out, "/// The most entries any pool of a blended profile can hold.\npub const POOL_MAX: usize = {union};");
    // The largest bar count, step count and chord size any profile draws:
    // the generator's lists are sized from them (`generator::plan`).
    for (key, name) in [("bars", "MAX_BARS"), ("stepsPerBar", "MAX_STEPS_PER_BAR"), ("chordSize", "MAX_CHORD_SIZE")] {
        let mut most = 0.0f64;
        for b in blocks.iter().filter(|b| b.kind == "character") {
            for (n, _) in (Reader { file, block: b }).numpool(key).unwrap_or_default() {
                assert!(n >= 1.0 && n.fract() == 0.0, "{file}: [character {}] {key}: {n} is not a whole number", b.name);
                most = most.max(n);
            }
        }
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // whole numbers, checked
        let most = most as usize;
        let _ = writeln!(out, "/// The largest `{key}` any profile draws.\npub const {name}: usize = {most};");
    }
    let count = blocks.iter().filter(|b| b.kind == "character").count();
    let _ = writeln!(out, "/// How many profiles there are.\npub const CHARACTER_COUNT: usize = {count};");
    out.push_str(
        "/// The profiles, in the file's order (`data/characters.txt`).\npub static CHARACTERS: [Character; CHARACTER_COUNT] = [\n",
    );
    for b in blocks.iter().filter(|b| b.kind == "character") {
        let r = Reader { file, block: b };
        r.only(CHARACTER_KEYS);
        ids.push(b.name.clone());
        let _ = write!(out, "Character {{ id: {}, ", quote(&b.name));
        let _ = write!(out, "label: {}, ", opt(r.one("label"), quote));
        let _ = write!(out, "groove: {}, ", opt(r.one("groove"), quote));
        let _ = write!(out, "polymeter: {}, ", opt(flag(&r, "polymeter"), |x| x.to_string()));
        let _ = write!(out, "micro_loop: {}, ", opt(flag(&r, "microLoop"), |x| x.to_string()));
        let _ = write!(
            out,
            "borrowed_bright: {}, ",
            opt(r.words("borrowedBright"), |w| format!("&[{}]", w.iter().map(|x| quote(x)).collect::<Vec<_>>().join(",")))
        );
        let need = |k: &str| r.range(k).unwrap_or_else(|| panic!("{}: missing", r.at(k)));
        let _ = write!(
            out,
            "tone: [{}, {}, {}], ",
            show_range(need("tone.warmth")),
            show_range(need("tone.space")),
            show_range(need("tone.wobble"))
        );
        let _ = write!(
            out,
            "feel: [{}, {}, {}], ",
            show_range(need("feel.lift")),
            show_range(need("feel.energy")),
            show_range(need("feel.warmth"))
        );
        let gaits = r.rows("gaits").map(|rows| {
            let mut s = String::from("&[");
            for g in rows {
                assert!(g.len() == 5, "{}: expected weight, metre, steps a bar and a tempo range", r.at("gaits"));
                let at = r.at("gaits");
                let _ = write!(
                    s,
                    "Gait {{ weight: {}, metre: {}, steps_per_bar: {}, bpm: [{}, {}] }},",
                    f(number(&g[0], &at)),
                    quote(&g[1]),
                    f(number(&g[2], &at)),
                    f(number(&g[3], &at)),
                    f(number(&g[4], &at))
                );
            }
            s.push(']');
            s
        });
        let _ = write!(out, "gaits: {}, ", opt(gaits, |s| s));
        let progressions = if let Some(rows) = r.rows("progressions") {
            Some(format!("Progressions::List({})", progression_rows(rows, &r.at("progressions"))))
        } else {
            let per: Vec<&Entry> = b.entries.iter().filter(|e| e.key.starts_with("progressions.")).collect();
            if per.is_empty() {
                None
            } else {
                let mut s = String::from("Progressions::PerMode(&[");
                for e in per {
                    let _ = write!(s, "({}, {}),", quote(&e.key["progressions.".len()..]), progression_rows(&e.rows, &r.at(&e.key)));
                }
                s.push_str("])");
                Some(s)
            }
        };
        let _ = write!(out, "progressions: {}, ", opt(progressions, |s| s));
        for (k, name) in [("bpm", "bpm"), ("swing", "swing"), ("hatDensity", "hat_density")] {
            let _ = write!(out, "{name}: {}, ", opt(r.range(k), show_range));
        }
        for (k, name) in [("stepsPerBar", "steps_per_bar"), ("bars", "bars"), ("chordSize", "chord_size")] {
            let _ = write!(out, "{name}: {}, ", opt(r.numpool(k), show_numpool));
        }
        for (k, name) in CHARACTER_POOLS {
            let _ = write!(out, "{name}: {}, ", opt(r.pool(k), show_pool));
        }
        for (k, name) in CHARACTER_NUMS {
            let _ = write!(out, "{name}: {}, ", opt(r.num(k), f));
        }
        out.push_str("},\n");
    }
    out.push_str("];\n");
    ids
}

const VIBE_TABLES: &[(&str, &str)] = &[
    ("feel", "feel"),
    ("tone", "tone"),
    ("profiles", "profiles"),
    ("modes", "modes"),
    ("melody", "melody"),
    ("chords", "chords"),
    ("bass", "bass"),
    ("bassStyle", "bass_style"),
    ("texture", "texture"),
];
const VIBE_NUMS: &[(&str, &str)] = &[
    ("tempo", "tempo"),
    ("swing", "swing"),
    ("drums", "drums"),
    ("hats", "hats"),
    ("rests", "rests"),
    ("long", "long"),
    ("bigChords", "big_chords"),
    ("six8", "six8"),
];

fn vibes(out: &mut String, blocks: &[Block]) {
    let file = "data/vibes.txt";
    let order = blocks.iter().find(|b| b.kind == "order" && b.name == "vibes").expect("data/vibes.txt has no [order vibes]");
    let ids = (Reader { file, block: order }).words("ids").expect("[order vibes] has no ids");
    let _ = writeln!(out, "/// How many vibes there are.\npub const VIBE_COUNT: usize = {};", ids.len());
    out.push_str("/// The vibes in the order share codes write them (`[order vibes]`): a\n/// vibe's index here is its number. Every id has its recipe.\npub static VIBES: [Vibe; VIBE_COUNT] = [\n");
    for id in &ids {
        let b = blocks
            .iter()
            .find(|b| b.kind == "vibe" && b.name == *id)
            .unwrap_or_else(|| panic!("{file}: '{id}' is in [order vibes] but has no [vibe {id}]"));
        let r = Reader { file, block: b };
        let mut known: Vec<&str> = VIBE_TABLES.iter().map(|(k, _)| *k).collect();
        known.extend(VIBE_NUMS.iter().map(|(k, _)| *k));
        r.only(&known);
        let _ = write!(out, "Vibe {{ id: {}, ", quote(id));
        for (k, name) in VIBE_TABLES {
            // A table: Object.fromEntries, so a repeated name keeps its last
            // number (none repeats today).
            let t = r.pool(k).map(|p| {
                let mut seen: Vec<(String, f64)> = Vec::new();
                for (n, w) in p {
                    if let Some(e) = seen.iter_mut().find(|(x, _)| *x == n) {
                        e.1 = w;
                    } else {
                        seen.push((n, w));
                    }
                }
                seen
            });
            let _ = write!(out, "{name}: {}, ", opt(t, show_pool));
        }
        for (k, name) in VIBE_NUMS {
            let _ = write!(out, "{name}: {}, ", opt(r.num(k), f));
        }
        out.push_str("},\n");
    }
    out.push_str("];\n");
}

fn moods(out: &mut String, blocks: &[Block]) {
    let file = "data/moods.txt";
    let count = blocks.iter().filter(|b| b.kind == "mood").count();
    let _ = writeln!(out, "/// How many moods there are.\npub const MOOD_COUNT: usize = {count};");
    out.push_str("/// The moods, in the file's order (`data/moods.txt`).\npub static MOODS: [Mood; MOOD_COUNT] = [\n");
    for b in blocks.iter().filter(|b| b.kind == "mood") {
        let r = Reader { file, block: b };
        r.only(&["code", "word", "lift", "energy", "warmth"]);
        let need = |k: &str| r.num(k).unwrap_or_else(|| panic!("{}: missing", r.at(k)));
        let _ = writeln!(
            out,
            "Mood {{ id: {}, lift: {}, energy: {}, warmth: {} }},",
            quote(&b.name),
            f(need("lift")),
            f(need("energy")),
            f(need("warmth"))
        );
    }
    out.push_str("];\n");
}

impl Reader<'_> {
    // The readings a setting's keys need, each failing the build when the
    // key is missing: every key `js/choices.js` reads is read there too.
    fn need_num(&self, key: &str) -> String {
        f(self.num(key).unwrap_or_else(|| panic!("{}: missing", self.at(key))))
    }
    fn need_range(&self, key: &str) -> String {
        show_range(self.range(key).unwrap_or_else(|| panic!("{}: missing", self.at(key))))
    }
    fn need_words(&self, key: &str) -> String {
        let w = self.words(key).unwrap_or_else(|| panic!("{}: missing", self.at(key)));
        format!("&[{}]", w.iter().map(|x| quote(x)).collect::<Vec<_>>().join(","))
    }
    fn need_pool(&self, key: &str) -> String {
        show_pool(self.pool(key).unwrap_or_else(|| panic!("{}: missing", self.at(key))))
    }
    fn need_numpool(&self, key: &str) -> String {
        show_numpool(self.numpool(key).unwrap_or_else(|| panic!("{}: missing", self.at(key))))
    }
    /// A table (`READ.table`: `Object.fromEntries`, so a name given twice
    /// keeps its last number), as a pool.
    fn table(&self, key: &str) -> Option<String> {
        self.pool(key).map(|p| {
            let mut seen: Vec<(String, f64)> = Vec::new();
            for (n, w) in p {
                if let Some(e) = seen.iter_mut().find(|(x, _)| *x == n) {
                    e.1 = w;
                } else {
                    seen.push((n, w));
                }
            }
            show_pool(seen)
        })
    }
    fn need_table(&self, key: &str) -> String {
        self.table(key).unwrap_or_else(|| panic!("{}: missing", self.at(key)))
    }
    /// The entries `HEAD.part` (a `'HEAD.*'` key), in the block's order:
    /// each part and its key.
    fn parts(&self, head: &str) -> Vec<(String, String)> {
        let prefix = format!("{head}.");
        self.block.entries.iter().filter_map(|e| e.key.strip_prefix(&prefix).map(|p| (p.to_string(), e.key.clone()))).collect()
    }
    /// `HEAD.NAME` tables by name, as `&[(name, table)]`.
    fn tables_by_name(&self, head: &str) -> String {
        let mut s = String::from("&[");
        for (part, key) in self.parts(head) {
            let _ = write!(s, "({}, {}),", quote(&part), self.need_table(&key));
        }
        s.push(']');
        s
    }
    /// `HEAD.NUMBER` tables by number, as `&[(number, table)]`. The
    /// JavaScript looks one up by the number's own text (`ECHO.vibes[b]`),
    /// so a part must be written as JavaScript writes its number ('0.5',
    /// not '.5' or '0.50'), or the two would disagree on what it names.
    fn tables_by_number(&self, head: &str) -> String {
        let mut s = String::from("&[");
        for (part, key) in self.parts(head) {
            let n = number(&part, &self.at(&key));
            assert!(format!("{n}") == part, "{}: write the number as {n}", self.at(&key));
            let _ = write!(s, "({}, {}),", f(n), self.need_table(&key));
        }
        s.push(']');
        s
    }
    /// `HEAD.NAME`'s range, for a NAME that must be there. Every part of
    /// `HEAD` must read as a range, as the JavaScript reads them all.
    fn part_range(&self, head: &str, name: &str) -> String {
        for (_, key) in self.parts(head) {
            self.need_range(&key);
        }
        self.need_range(&format!("{head}.{name}"))
    }
}

/// The choices and every setting they draw from (queue items 80 and 81):
/// the draw's counts and leans, each choice's options and vibes, and the
/// settings of the arc, the tuned kick, subDuck, lateFloor, the floor drop's
/// shapes, the pedal, padSweep, the echo time and the bass's level, each
/// block's keys exactly the ones `js/choices.js` reads.
#[allow(clippy::too_many_lines)] // every setting, key by key, in the file's order
fn choices(out: &mut String, blocks: &[Block]) {
    let file = "data/choices.txt";
    let find = |kind: &str, name: &str| {
        let block = blocks.iter().find(|b| b.kind == kind && b.name == name).unwrap_or_else(|| panic!("{file} has no [{kind} {name}]"));
        Reader { file, block }
    };
    let all = find("choices", "all");
    all.only(&["changes", "lean"]);
    let _ = writeln!(out, "/// How many changes a loop gets, by weight.\npub static CHANGE_COUNTS: NumPool = {};", all.need_numpool("changes"));
    let _ = writeln!(out, "/// A choice's lean for one vibe is held between these.\npub const CHOICE_LEAN: [f64; 2] = {};", all.need_range("lean"));
    out.push_str("/// The choices, in the order the draw walks them.\npub static CHOICES: &[Choice] = &[\n");
    for b in blocks.iter().filter(|b| b.kind == "choice") {
        let r = Reader { file, block: b };
        r.only(&["options", "weight", "vibes", "optionVibes.*"]);
        let _ = writeln!(
            out,
            "Choice {{ id: {}, options: {}, weight: {}, vibes: {}, option_vibes: {} }},",
            quote(&b.name),
            r.need_pool("options"),
            opt(r.num("weight"), f),
            opt(r.table("vibes"), |s| s),
            r.tables_by_name("optionVibes")
        );
    }
    out.push_str("];\n");

    let arc = find("setting", "arc");
    arc.only(&["shapes", "shapeVibes.*", "period.*", "calm", "bright", "glacialNear", "glacialChance", "from.*", "floor", "dip"]);
    let _ = writeln!(
        out,
        "/// The arc's settings (`[setting arc]`).\npub static ARC: Arc = Arc {{ shapes: {}, shape_vibes: {}, period: ArcPeriods {{ calm: {}, bright: {}, rest: {}, glacial: {}, glacial_near: {} }}, calm: {}, bright: {}, glacial_near: {}, glacial_chance: {}, from_deep: {}, floor: {}, dip: {} }};",
        arc.need_pool("shapes"),
        arc.tables_by_name("shapeVibes"),
        arc.part_range("period", "calm"),
        arc.part_range("period", "bright"),
        arc.part_range("period", "rest"),
        arc.part_range("period", "glacial"),
        arc.part_range("period", "glacialNear"),
        arc.need_words("calm"),
        arc.need_words("bright"),
        arc.need_words("glacialNear"),
        arc.need_num("glacialChance"),
        arc.part_range("from", "deep"),
        arc.need_range("floor"),
        arc.need_num("dip")
    );
    let kick = find("setting", "kick");
    kick.only(&["note", "short", "long", "loud", "tailLoud", "band", "swap"]);
    let _ = writeln!(
        out,
        "/// The tuned kick's settings (`[setting kick]`).\npub static KICK: Kick = Kick {{ note: {}, short: {}, long: {}, loud: {}, tail_loud: {}, band: {}, swap: {} }};",
        kick.need_range("note"),
        kick.need_range("short"),
        kick.need_range("long"),
        kick.need_num("loud"),
        kick.need_num("tailLoud"),
        kick.need_num("band"),
        kick.need_num("swap")
    );
    let duck = find("setting", "subduck");
    duck.only(&["depth", "recover", "beat"]);
    let _ = writeln!(
        out,
        "/// subDuck's settings (`[setting subduck]`).\npub static SUBDUCK: SubDuck = SubDuck {{ depth: {}, recover: {}, beat: {} }};",
        duck.need_range("depth"),
        duck.need_range("recover"),
        duck.need_num("beat")
    );
    let late = find("setting", "latefloor");
    late.only(&["seconds", "round", "first", "orders", "gap", "maxGap", "gapLong", "longVibes"]);
    let _ = writeln!(
        out,
        "/// lateFloor's settings (`[setting latefloor]`).\npub static LATEFLOOR: LateFloor = LateFloor {{ seconds: {}, round: {}, first: {}, orders: {}, gap: {}, max_gap: {}, gap_long: {}, long_vibes: {} }};",
        late.need_range("seconds"),
        late.need_num("round"),
        late.need_num("first"),
        late.need_pool("orders"),
        late.need_numpool("gap"),
        late.need_num("maxGap"),
        late.need_num("gapLong"),
        late.need_table("longVibes")
    );
    let drop = find("setting", "floordrop");
    drop.only(&[
        "shapes",
        "needBass",
        "shapeVibes.*",
        "staggerShort",
        "stutterShort",
        "stutter",
        "longSeconds",
        "longBars",
        "wholeUnder",
    ]);
    let _ = writeln!(
        out,
        "/// floorDrop's shapes (`[setting floordrop]`).\npub static FLOORDROP: FloorDrop = FloorDrop {{ shapes: {}, need_bass: {}, shape_vibes: {}, stagger_short: {}, stutter_short: {}, stutter: {}, long_seconds: {}, long_bars: {}, whole_under: {} }};",
        drop.need_pool("shapes"),
        drop.need_words("needBass"),
        drop.tables_by_name("shapeVibes"),
        drop.need_num("staggerShort"),
        drop.need_num("stutterShort"),
        drop.need_range("stutter"),
        drop.need_num("longSeconds"),
        drop.need_range("longBars"),
        drop.need_num("wholeUnder")
    );
    let pedal = find("setting", "pedal");
    pedal.only(&["styles", "rhythms", "neighbors", "neighborVibes.*", "share", "low", "high"]);
    let _ = writeln!(
        out,
        "/// pedalBass's settings (`[setting pedal]`).\npub static PEDAL: Pedal = Pedal {{ styles: {}, rhythms: {}, neighbors: {}, neighbor_vibes: {}, share: {}, low: {}, high: {} }};",
        pedal.need_words("styles"),
        pedal.need_pool("rhythms"),
        pedal.need_numpool("neighbors"),
        pedal.tables_by_number("neighborVibes"),
        pedal.need_range("share"),
        pedal.need_num("low"),
        pedal.need_num("high")
    );
    let pad = find("setting", "padsweep");
    pad.only(&["sustained", "lo", "oct", "period", "q"]);
    let _ = writeln!(
        out,
        "/// padSweep's settings (`[setting padsweep]`).\npub static PADSWEEP: PadSweep = PadSweep {{ sustained: {}, lo: {}, oct: {}, period: {}, q: {} }};",
        pad.need_words("sustained"),
        pad.need_range("lo"),
        pad.need_range("oct"),
        pad.need_range("period"),
        pad.need_range("q")
    );
    let echo = find("setting", "echo");
    echo.only(&["beats", "vibes.*"]);
    let _ = writeln!(
        out,
        "/// echoTime's settings (`[setting echo]`).\npub static ECHO: Echo = Echo {{ beats: {}, vibes: {} }};",
        echo.need_numpool("beats"),
        echo.tables_by_number("vibes")
    );
    let lift = find("setting", "basslift");
    lift.only(&["kick", "noKick"]);
    let _ = writeln!(
        out,
        "/// The bass layer's level (`[setting basslift]`).\npub static BASS_LIFT: BassLift = BassLift {{ kick: {}, no_kick: {} }};",
        lift.need_num("kick"),
        lift.need_num("noKick")
    );
}

fn voices(out: &mut String, blocks: &[Block]) {
    let file = "data/voices.txt";
    let layers = blocks.iter().find(|b| b.kind == "voices" && b.name == "layers").expect("data/voices.txt has no [voices layers]");
    let order = (Reader { file, block: layers }).words("order").expect("[voices layers] has no order");
    // The generator's layers are fixed in the core (`generator::Layer`); the file
    // must agree, in its order.
    assert_eq!(order, ["drums", "bass", "chords", "melody", "texture"], "{file}: the layers are not the core's (generator::Layer)");
    // The voice budget's costs, which the player asks as the JavaScript synth
    // does (queue item 82): the keys `js/synth.js` reads (`COSTS`).
    let costs = blocks.iter().find(|b| b.kind == "voices" && b.name == "costs").expect("data/voices.txt has no [voices costs]");
    let costs = Reader { file, block: costs };
    costs.only(&["default", "dubstab", "cost"]);
    let _ = writeln!(
        out,
        "/// The voice budget's costs (`[voices costs]`).\npub static VOICE_COSTS: VoiceCosts = VoiceCosts {{ default: {}, dubstab: {}, cost: {} }};",
        costs.need_num("default"),
        costs.need_num("dubstab"),
        costs.need_table("cost")
    );
}

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR")).join("..").join("data");
    let read = |name: &str| {
        let path = root.join(format!("{name}.txt"));
        println!("cargo::rerun-if-changed={}", path.display());
        let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("data/{name}.txt: {e}"));
        parse(&text, &format!("data/{name}.txt"))
    };
    let mut out = String::from("// Written by core/build.rs from data/*.txt. Do not edit.\n");
    let _ = characters(&mut out, &read("characters"));
    vibes(&mut out, &read("vibes"));
    moods(&mut out, &read("moods"));
    choices(&mut out, &read("choices"));
    voices(&mut out, &read("voices"));
    let dest = PathBuf::from(env::var("OUT_DIR").expect("cargo sets OUT_DIR")).join("gen_data.rs");
    fs::write(dest, out).expect("can write the generated tables");
    println!("cargo::rerun-if-changed=build.rs");
}
