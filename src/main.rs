extern crate drawille;

use conway::{Cells, Snapshot, Game};
use drawille::Canvas;
use std::{thread, time};
// use termsize;

const RATE: u64 = 50;

fn main() {
    // let termsize::Size { rows, cols } = termsize::get().unwrap();
    // let size: u32 = std::cmp::min(rows, cols) as u32;
    let size: u32 = 100;

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

    let snapshot = Snapshot::new();

    let mut game = Game::new(snapshot, size, cells, Some(canvas));

    loop {
        thread::sleep(time::Duration::from_millis(RATE));
        game.step();
    }
}

