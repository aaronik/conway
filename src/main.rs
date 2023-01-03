extern crate drawille;

use conway::{Cells, Snapshot, Game};
use drawille::Canvas;
use std::{thread, time};
// use termsize;

const RATE: u64 = 0;

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

    let mut cells = Cells::new();

    // Get some initial configuration
    let midpoint = size / 2;
    cells.birth(midpoint, midpoint);
    cells.birth(midpoint, midpoint + 1);
    cells.birth(midpoint + 1, midpoint);
    cells.birth(midpoint + 1, midpoint + 1);
    cells.birth(midpoint + 1, midpoint + 2);
    cells.birth(midpoint + 1, midpoint + 3);
    cells.birth(midpoint + 1, midpoint + 4);
    cells.birth(midpoint + 5, midpoint + 4);
    cells.birth(midpoint + 6, midpoint + 4);
    cells.birth(midpoint + 6, midpoint + 5);
    cells.birth(midpoint + 7, midpoint + 4);
    cells.birth(midpoint + 8, midpoint + 4);

    let snapshot = Snapshot::new();

    let mut game = Game::new(snapshot, size, cells, Some(canvas));
    // let mut game = Game::new(snapshot, size, cells, None);

    loop {
        thread::sleep(time::Duration::from_millis(RATE));
        game.step();
        println!("{}", game.unique_iterations);
    }
}

