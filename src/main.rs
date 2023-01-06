extern crate drawille;

use conway::{Db, Evolver, Args, Commands};
use r2d2::PooledConnection;
use std::thread::{self, JoinHandle};
use clap::Parser;

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

    match &args.command {
        Commands::Evolve { threads } => evolve(*threads, pool.clone(), size),

        Commands::Display => {
            display(pool.clone().get().unwrap());
        }
    }
}

// Spawn a new evolution for this many threads
fn evolve(threads: u32, pool: r2d2::Pool<SqliteConnectionManager>, size: u32) {
    (0..=threads)
        .map(|thread_num| {
            let pool = pool.clone();
            let db = Db::new(pool.get().unwrap());

            thread::spawn(move || {
                let mut evolution = Evolver::new(size, db);
                evolution.begin_evolving(thread_num);
            })
        })
        .collect::<Vec<JoinHandle<()>>>()
        .into_iter()
        .map(thread::JoinHandle::join)
        .collect::<Result<(), _>>()
        .unwrap();
}

fn display(connection: PooledConnection<SqliteConnectionManager>) {
    let db = Db::new(connection);

    let boards = db.load_boards().unwrap();

    // // TODO it'd be nice if these were sorted but currently having a little difficulty with it
    // boards.sort_by_key(|b| {
    //     let measurable = conway::board::Measurable::Saved(b);
    //     Evolver::measure_fitness(b)
    // });

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

    // TODO get the id and implement the run!
}
