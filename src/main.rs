extern crate drawille;

use conway::Db;
use conway::Evolver;
use std::thread::{self, JoinHandle};

extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

use r2d2_sqlite::SqliteConnectionManager;

// An evolutionary solver to conway's game of life, in color!

fn main() {
    // For the time being we'll just stick with 150 characters^2 for the board size
    let size: u32 = 150;

    let manager = SqliteConnectionManager::file("database.db");
    let pool = r2d2::Pool::new(manager).unwrap();

    // Make sure the database file and tables exist
    Db::initialize(pool.get().unwrap());

    // Spawn a new evolution for this many threads
    (0..=2).map(|thread_num| {
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
