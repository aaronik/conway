extern crate drawille;

use conway::Db;
use conway::Evolver;
use std::thread::{self, JoinHandle};
// use termsize;

extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

use r2d2_sqlite::SqliteConnectionManager;

fn main() {
    // let termsize::Size { rows, cols } = termsize::get().unwrap();
    // let size: u32 = std::cmp::min(rows, cols) as u32;
    let size: u32 = 150;

    let manager = SqliteConnectionManager::file("database.db");
    let pool = r2d2::Pool::new(manager).unwrap();

    Db::initialize(pool.get().unwrap());

    let mut threads: Vec<JoinHandle<()>> = vec![];

    for thread_num in 1..=8 {
        let pool = pool.clone();
        let db = Db::new(pool.get().unwrap());

        let thread_return = thread::spawn(move || {
            let mut evolution = Evolver::new(size, db);
            evolution.begin_evolving(thread_num);
        });

        threads.push(thread_return);
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
