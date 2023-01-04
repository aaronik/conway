extern crate drawille;

use conway::{Cells, Game};
// use conway::Snapshot;
use drawille::Canvas;
use std::{thread, time};
// use termsize;

// TODO
// * Once there are interesting patterns found, I want to :
//   - store them (sqllite: https://stackoverflow.com/questions/62560396/how-to-use-sqlite-via-rusqlite-from-multiple-threads)
//   - Replay them (So if I could store each as a list of cells, either one table for grid/one for
//   cells, or just one for grid which has a datatype of a collection of tuples somehow.)
//   - grid: id,size

// How to start looking for life:
// * Fitness function -- bigger unique_iterations X Going to fall into local maxima of loops
//                       Keep a history of snapshots, look for loops. Stop when you re-reach a
//                       state.
//                    -- I like loops, loops with high period are the best.
// * Mate -- A meme is a contiguous group, or a localized grouping with a small amount of space
//        -- Or, could make a meme a grid breakdown of the board, like quadrants
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
        (midpoint + 7, midpoint + 4),
        (midpoint + 8, midpoint + 4),
        (midpoint + 0, midpoint + 4),
    ]);

    let snapshot = conway::Snapshot::new(size);

    let mut game = Game::new(Some(snapshot), cells, Some(canvas));
    // let mut game = Game::new(None, cells, None);

    loop {
        thread::sleep(time::Duration::from_millis(50));
        game.step();

        // Bail if it's a barren death land
        if game.cells.num_living_cells() == 0 {
            println!("Game has no more life");
            std::process::exit(0);
        }
    }
}

