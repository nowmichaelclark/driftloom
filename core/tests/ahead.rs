//! Rendering ahead (queue item 87): a loop rendered in chunks a host asks
//! for is the same samples as one rendered straight through, wandering pass
//! after pass and with a change of loop waiting at a boundary; a pass at a
//! time (`Player::pass_frames`) lands each chunk on the pass's end; and the
//! estimate of a chunk's cost reads the loop's voices. Chunk sizes from a
//! block to a pass are swept in WebAssembly by `tools/ahead.mjs --sweep`;
//! these keep the cases in the core's own tests, natively.

use dlcore::Core;
use dlcore::player::{Handover, Moment, Player, Request, State};

/// A thread with room for a core and a player built on its stack (a debug
/// build makes them there before boxing them).
fn roomy(f: impl FnOnce() + Send + 'static) {
    let stack = (std::mem::size_of::<Core>() + std::mem::size_of::<Player>()) * 4 + (16 << 20);
    std::thread::Builder::new().stack_size(stack).spawn(f).expect("a thread").join().expect("the test");
}

/// A short loop (shaisk-sha, a pass of 3 s) and the loop a
/// change asks for.
const SEED: u32 = 1_027_928_796;
const NEXT: u32 = 209_265_687;
const RATE: f64 = 22_050.0;

/// A player at `RATE`, stereo, the loop made at once.
fn start() -> (Box<Core>, Box<Player>) {
    let mut core = Box::new(Core::new());
    let mut player = Box::new(Player::new());
    player.start(&mut core, RATE, 2, true).expect("a player");
    player.load(SEED, &Request::new());
    player.ready(&mut *core);
    (core, player)
}

/// How the chunks are cut: a fixed size, or a pass at a time.
#[derive(Clone, Copy, Debug)]
enum Cut {
    Size(usize),
    Pass,
    Mixed,
}

/// `frames` frames, cut as `cut` says, with a change to `NEXT` at the next
/// pass asked once `change_at` frames are rendered (a chunk is cut there).
fn render(cut: Cut, frames: usize, change_at: Option<usize>) -> Vec<f32> {
    let (mut core, mut player) = start();
    let mut left = vec![0.0f32; frames];
    let mut right = vec![0.0f32; frames];
    let mut done = 0;
    let mut k = 0usize;
    while done < frames {
        if change_at == Some(done) {
            player.change(NEXT, &Request::new(), Moment::Pass, Handover::default());
        }
        let want = match cut {
            Cut::Size(n) => n,
            Cut::Pass => usize::try_from(player.pass_frames(&core).expect("a loop plays")).expect("frames"),
            Cut::Mixed => [128, 1, 4410, 777, 2048, 3][k % 6],
        };
        k += 1;
        let mut n = want.min(frames - done);
        if let Some(at) = change_at
            && done < at
        {
            n = n.min(at - done);
        }
        player.render(&mut core, &mut [&mut left[done..done + n], &mut right[done..done + n]]).expect("a render");
        done += n;
        assert_eq!(core.served(), done as u64, "the core says where the host stands");
    }
    left.extend_from_slice(&right);
    left
}

#[test]
fn chunks_of_any_size_render_as_straight_through() {
    roomy(|| {
        let (core, player) = start();
        let pass = usize::try_from(player.pass_frames(&core).expect("a loop plays")).expect("frames");
        drop((core, player));
        // Three passes and a little more, so the wander draws two.
        let frames = pass * 3 + 5_000;
        let straight = render(Cut::Size(frames), frames, None);
        for cut in [Cut::Size(128), Cut::Size(1), Cut::Size(333), Cut::Size(4096), Cut::Pass, Cut::Mixed, Cut::Size(pass + 1)] {
            assert!(render(cut, frames, None) == straight, "{cut:?}: the same samples as straight through");
        }
    });
}

#[test]
fn a_change_waiting_at_a_boundary_lands_alike_in_any_chunks() {
    roomy(|| {
        let (core, player) = start();
        let pass = usize::try_from(player.pass_frames(&core).expect("a loop plays")).expect("frames");
        drop((core, player));
        let frames = pass * 3;
        let at = pass / 3;
        let straight = render(Cut::Size(frames), frames, Some(at));
        assert!(straight != render(Cut::Size(frames), frames, None), "the change is heard");
        for cut in [Cut::Size(128), Cut::Size(1000), Cut::Pass, Cut::Mixed] {
            assert!(render(cut, frames, Some(at)) == straight, "{cut:?}: the change lands alike");
        }
    });
}

#[test]
#[allow(clippy::cast_precision_loss)] // a pass's frames, far below 2^53
fn a_pass_at_a_time_ends_each_chunk_on_the_pass() {
    roomy(|| {
        let (mut core, mut player) = start();
        let g = player.generator().spec();
        let pass_seconds = g.bars * g.steps_per_bar * 15.0 / g.bpm;
        let mut buf = vec![0.0f32; 1 << 20];
        let mut right = vec![0.0f32; 1 << 20];
        let first = player.pass_frames(&core).expect("a loop plays");
        // The first: the loop's start (`START`, 0.05 s) and a whole pass.
        assert!((first as f64 / RATE - (pass_seconds + 0.05)).abs() < 2.0 / RATE, "the first pass: {first} frames");
        for _ in 0..4 {
            let n = usize::try_from(player.pass_frames(&core).expect("a loop plays")).expect("frames");
            player.render(&mut core, &mut [&mut buf[..n], &mut right[..n]]).expect("a render");
            let next = player.pass_frames(&core).expect("a loop plays");
            assert!((next as f64 / RATE - pass_seconds).abs() < 2.0 / RATE, "each next: a whole pass, {next} frames");
        }
        assert_eq!(player.state(), State::Playing);
    });
}

#[test]
fn the_estimate_reads_the_voices() {
    roomy(|| {
        let (mut core, mut player) = start();
        // A loop just made: the typical figure, in proportion to the frames.
        let typical = player.estimate(&core, 128);
        assert!(typical > 0.0 && (player.estimate(&core, 1280) - 10.0 * typical).abs() < 1e-9);
        assert_eq!(player.units(), None, "nothing measured yet");
        let frames = usize::try_from(player.pass_frames(&core).expect("a loop plays")).expect("frames") + 6_000;
        let mut left = vec![0.0f32; frames];
        let mut right = vec![0.0f32; frames];
        player.render(&mut core, &mut [&mut left, &mut right]).expect("a render");
        let units = player.units().expect("a pass measured");
        assert!(units > 0.0 && units < 1000.0, "the units held, {units}");
        let measured = player.estimate(&core, 128);
        assert!(measured > 0.0 && (measured > typical) == (units > dlcore::player::ahead::TYPICAL_UNITS[0]), "the units measured are read: {units}");
    });
}
