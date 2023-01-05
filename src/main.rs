extern crate drawille;

use conway::{Cells, Db, Game, Board, Evolution};
use drawille::Canvas;
use rand::Rng;
use std::{thread, time};
// use termsize;

extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

use r2d2_sqlite::SqliteConnectionManager;

// TODO
// * Once there are interesting patterns found, I want to :
//   - store them (sqllite: https://stackoverflow.com/questions/62560396/how-to-use-sqlite-via-rusqlite-from-multiple-threads)
//   - Replay them (So if I could store each as a list of cells, either one table for grid/one for
//   cells, or just one for grid which has a datatype of a collection of tuples somehow.)
//   - grid: id,size
// * Wrap the grid

// How to start looking for life:
// * Fitness function -- bigger unique_iterations X Going to fall into local maxima of loops
//                       Keep a history of snapshots, look for loops. Stop when you re-reach a
//                       state.
//                    -- I like loops, loops with high period are the best.
// * Mate -- A meme is a contiguous group, or a localized grouping with a small amount of space
//        -- Or, could make a meme a grid breakdown of the board, like quadrants
//
// * Mutations are memes placed nearby or randomly, or just random squares in the beginning

fn main() {
    let manager = SqliteConnectionManager::file("database.db");
    let pool = r2d2::Pool::new(manager).unwrap();
    // pool.clone() for each thread

    Db::initialize(pool.get().unwrap());

    // TODO Ultimately we'll have one of these in each thread, after cloning the pool
    let mut db = Db::new(pool.get().unwrap());

    // (0..10)
    //     .map(|i| {
    //         let pool = pool.clone();
    //         thread::spawn(move || {
    //             let conn = pool.get().unwrap();
    //             conn.execute("INSERT INTO foo (bar) VALUES (?)", &[&i])
    //                 .unwrap();
    //         })
    //     })
    //     .collect::<Vec<_>>()
    //     .into_iter()
    //     .map(thread::JoinHandle::join)
    //     .collect::<Result<_, _>>()
    //     .unwrap();

    // let termsize::Size { rows, cols } = termsize::get().unwrap();
    // let size: u32 = std::cmp::min(rows, cols) as u32;
    let size: u32 = 150;

    let canvas = Canvas::new(size, size);

    // bring the term to its lowest position, just looks cleaner this way
    print!("{}", canvas.frame());
    print!("{}", canvas.frame());

    // let mut cells = Cells::new(size);

    // // Get some initial configuration
    // let midpoint = size / 3;
    // let initial_cells = vec![
    //     (midpoint, midpoint),
    //     (midpoint + 1, midpoint),
    //     (midpoint + 1, midpoint + 1),
    //     (midpoint + 1, midpoint + 2),
    //     (midpoint + 1, midpoint + 3),
    //     (midpoint + 1, midpoint + 4),
    //     (midpoint + 5, midpoint + 4),
    //     (midpoint + 6, midpoint + 4),
    //     (midpoint + 6, midpoint + 5),
    //     (midpoint + 7, midpoint + 4),
    //     (midpoint + 8, midpoint + 4),
    //     (midpoint + 0, midpoint + 4),
    // ];

    // cells.birth_multiple(&initial_board.1);

    // let mut game = Game::new(Some(conway::Snapshot::new(size)), cells, Some(canvas));
    // let mut game = Game::new(None, cells, Some(canvas));
    // let mut game = Game::new(None, cells, None);

    // // OG Game loop
    // loop {
    //     thread::sleep(time::Duration::from_millis(50));
    //     game.step();

    //     // Bail if it's a barren death land
    //     if game.cells.num_living_cells() == 0 {
    //         println!("Game has no more life");
    //         std::process::exit(0);
    //     }

    //     // Demo snapshot abilities
    //     if let Some(snapshot) = &game.snapshot {
    //         if snapshot.has_repeat() {
    //             println!("snapshot has repeat of period {}", snapshot.period());
    //         } else {
    //             println!("");
    //         }
    //     }
    // }

    // Create a new board
    loop {
        // If this were real evolution, to generate these boards, we'd:
        // * Wait until we have a full list of boards
        // * Mate the top boards, with mutation
        // * Cull the bottom
        let mut cells = Cells::new(size);
        let initial_cells = random_cells(size, 100);
        cells.birth_multiple(&initial_cells);

        let canvas = Canvas::new(size, size);
        let snapshot = conway::Snapshot::new(size);
        let mut game = Game::new(Some(snapshot), cells, Some(canvas));

        // Iterate a single board
        loop {
            thread::sleep(time::Duration::from_millis(1));
            game.step();

            // Bail if it's a barren death land
            if game.cells.num_living_cells() == 0 {
                println!("died after {} iterations", game.iterations);
                thread::sleep(time::Duration::from_millis(3000));
                break;
            }

            if let Some(snapshot) = &game.snapshot {
                if snapshot.has_repeat() {
                    println!("period {} after {} iterations", snapshot.period().unwrap(), game.iterations);
                    thread::sleep(time::Duration::from_millis(3000));
                    break;
                }
            }
        }

        let board = Board {
            size,
            cells: initial_cells,
            iterations: game.iterations,
            period: game.snapshot.unwrap().period(),
        };

        // Add boards to the list
        db.save_board(&board).expect("error saving board");

        // * Remove weakest board (fitness)
        // Evolution::measure_fitness()
    }
}

fn random_cells(size: u32, num: u32) -> Vec<(u32, u32)> {
    let mut cells = vec![];

    for _ in 0..num {
        let range_i = (size*2) / 5..(size * 3) / 5;
        let range_j = range_i.clone();
        let rand_i = rand::thread_rng().gen_range(range_i);
        let rand_j = rand::thread_rng().gen_range(range_j);
        cells.push((rand_i, rand_j));
    }

    cells
}
