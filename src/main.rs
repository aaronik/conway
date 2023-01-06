extern crate drawille;

use clap::Parser;
use conway::{Args, Commands, Db, Evolver};
use r2d2::PooledConnection;
use core::time;
use std::thread::{self, JoinHandle};

extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

use r2d2_sqlite::SqliteConnectionManager;

fn main() {
    let args = Args::parse();

    // For the time being we'll just stick with 150 characters^2 for the board size
    let size: u32 = 150;

    let manager = SqliteConnectionManager::file(args.db);
    let pool = r2d2::Pool::new(manager).unwrap();

    // Make sure the database file and tables exist
    Db::initialize(pool.get().unwrap());

    match &args.command {
        Commands::Evolve { threads } => evolve(*threads, pool.clone(), size),

        Commands::Display { delay }=> {
            display(*delay, pool.clone().get().unwrap());
        }
    }
}

// Spawn a new evolution for this many threads
fn evolve(threads: usize, pool: r2d2::Pool<SqliteConnectionManager>, size: u32) {
    (0..=threads)
        .map(|thread_num| {
            let pool = pool.clone();
            let db = Db::new(pool.get().unwrap());

            thread::spawn(move || {
                let mut evolution = Evolver::new(size, db);
                evolution.begin_evolving(thread_num as u32);
            })
        })
        .collect::<Vec<JoinHandle<()>>>()
        .into_iter()
        .map(thread::JoinHandle::join)
        .collect::<Result<(), _>>()
        .unwrap();
}

fn display(delay: usize, connection: PooledConnection<SqliteConnectionManager>) {
    let db = Db::new(connection);

    // Load all the boards
    let mut boards = db.load_boards().unwrap();

    // Sort em up for easier picking
    boards.sort_by_key(|b| {
        let measurable = conway::board::Measurable::Saved(b);
        Evolver::measure_fitness(&measurable)
    });

    // List all the boards
    for board in boards {
        if let Some(period) = board.period {
            println!(
                "id: {} || Period {} with {} unique iterations",
                board.id, period, board.iterations
            );
        } else {
            println!(
                "id: {} || Non repeating with {} unique iterations",
                board.id, board.iterations
            );
        }
    }

    // Ask for which they want
    println!("\nSelect an id from the list to run that configuration:\n");

    // Get the id of the board they want to display
    let mut board_id: String = String::from("");

    std::io::stdin()
        .read_line(&mut board_id)
        .expect("Error loading input");

    let board_id: i64 = board_id
        .trim()
        .parse()
        .expect("You must input a numerical id from the list of ids presented.");

    let board = db.load_board(board_id).expect(
        "There was an issue loading the board, are you sure you input the numerical id correctly?",
    );

    // Prepare the game
    let mut cells = conway::Cells::new(board.size);
    cells.birth_multiple(&board.cells);
    let canvas = Some(drawille::Canvas::new(board.size, board.size));
    let mut game = conway::Game::new(None, cells, canvas);

    // Run the game
    loop {
        game.step();

        // Bail if it's a barren death land
        if game.cells.num_living_cells() == 0 {
            println!("The board ran out of life!");
            break;
        }

        thread::sleep(time::Duration::from_millis(delay as u64));
    }

    // TODO
    // * Replace the display in evolve mode with output about trying / failing / succeeding in
    // creating a new board
}
