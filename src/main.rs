extern crate drawille;

use conway::{Cells, Game};
// use conway::Snapshot;
use drawille::Canvas;
use std::{thread, time};
// use termsize;

const RATE: u64 = 20;

// TODO
// * I think Cells needs a second memory space. B/c we might be doing calculation based on a state
// half adjusted from the last state.
// * I'd really love it for Game to not iterate over every cell every time, instead just looking
// at the cells it needs to look at.
//   - Get live cells from Cells
//   - Get all neighbors, uniquely (loving me some hash sets these days)
//   - Iterate through those
// * Snapshot will need to keep a memory of all the different states so it can check for loops

// How to start looking for life:
// * Fitness function -- bigger unique_iterations X Going to fall into local maxima of loops
//                       Keep a history of snapshots, look for loops. Stop when you re-reach a
//                       state.
// * Mate -- A meme is a contiguous group, or a localized grouping with a small amount of space
// * Mutations are memes placed nearby or randomly, or just random squares in the beginning

fn main() {
    // let termsize::Size { rows, cols } = termsize::get().unwrap();
    // let size: u32 = std::cmp::min(rows, cols) as u32;
    let size: u32 = 150;

    let canvas = Canvas::new(size, size);

    // bring the term to its lowest position, just looks cleaner this way
    print!("{}", canvas.frame());
    print!("{}", canvas.frame());

    let mut cells = Cells::new(size);

    // Get some initial configuration
    let midpoint = size / 3;
    cells.birth_multiple(&[
        (midpoint, midpoint),
        (midpoint + 1, midpoint),
        (midpoint + 1, midpoint + 1),
        (midpoint + 1, midpoint + 2),
        (midpoint + 1, midpoint + 3),
        (midpoint + 1, midpoint + 4),
        (midpoint + 5, midpoint + 4),
        (midpoint + 6, midpoint + 4),
        (midpoint + 6, midpoint + 5),
    ]);

    // let snapshot = Snapshot::new();

    let mut game = Game::new(None, size, cells, Some(canvas));
    // let mut game = Game::new(snapshot, size, cells, None);

    loop {
        thread::sleep(time::Duration::from_millis(RATE));
        game.step();
        println!("{}", game.unique_iterations);
    }
}

