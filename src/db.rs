use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Error};

use crate::Board;

pub struct Db {
    connection: PooledConnection<SqliteConnectionManager>,
}

impl Db {
    pub fn new(connection: PooledConnection<SqliteConnectionManager>) -> Self {
        Self { connection }
    }

    /// Initializes the DB to this app's specific shape. Creates tables if they don't exist.
    /// Can be called every time the program starts.
    pub fn initialize(connection: PooledConnection<SqliteConnectionManager>) {
        let create_boards = "
        CREATE TABLE IF NOT EXISTS Boards (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            size INTEGER NOT NULL,
            period INTEGER,
            iterations INTEGER
            );
        ";

        let create_cells = "
        CREATE TABLE IF NOT EXISTS Cells (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            board_id INTEGER NOT NULL,
            i INTEGER NOT NULL,
            j INTEGER NOT NULL
            );
        ";

        [create_boards, create_cells].map(|query| {
            connection.execute(query, params![]).unwrap();
        });
    }

    /// Takes a number of cells and a board size and saves that board to the db
    /// Returns Result<board_id>
    pub fn save_board(&mut self, board: &Board) -> Result<i64, Error> {
        // Insert one new board
        self.connection.execute(
            "INSERT INTO Boards (size, period, iterations) VALUES (?, ?, ?)",
            params![board.size, board.period, board.iterations],
        )?;

        // What was that last id?
        let board_id = self.connection.last_insert_rowid();

        let tx = self.connection.transaction()?;
        for (i, j) in board.cells.iter() {
            tx.execute(
                "INSERT INTO Cells (board_id, i, j) VALUES (?, ?, ?)",
                params![board_id, i, j],
            )?;
        }
        tx.commit()?;

        Ok(board_id)
    }

    pub fn load_board(&self, board_id: i64) -> Result<Board, Error> {
        let (size, iterations, period) = self.connection.query_row(
            "SELECT size,iterations,period FROM Boards WHERE id = ?",
            params![board_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )?;

        let mut stmt = self.connection.prepare(
            "SELECT i,j FROM Boards INNER JOIN Cells ON Boards.id = Cells.board_id WHERE Boards.id = ?",
        )?;
        let cells_iter = stmt.query_map(params![board_id], |row| Ok((row.get(0)?, row.get(1)?)))?;

        let mut cells: Vec<(u32, u32)> = vec![];

        for cell in cells_iter {
            let cell = cell.expect("Error in loading board");
            cells.push((cell.0, cell.1))
        }

        Ok(Board {
            size,
            iterations,
            period,
            cells,
        })
    }

    // pub fn load_boards(&self) -> Result<Vec<Board>, Error> {

    // }
}

#[test]
fn saving_and_loading_boards() {
    let manager = SqliteConnectionManager::file("database.test.db");
    let pool = r2d2::Pool::new(manager).unwrap();
    Db::initialize(pool.get().unwrap());
    let mut db = Db::new(pool.get().unwrap());

    let board = Board {
        size: 10,
        iterations: 100,
        period: Some(10),
        cells: vec![(1,1), (2,2), (3,3)],
    };

    let board_id = db.save_board(&board).unwrap();

    let retrieved_board = db.load_board(board_id).unwrap();

    assert_eq!(board.size, retrieved_board.size);
    assert_eq!(board.iterations, retrieved_board.iterations);
    assert_eq!(board.period, retrieved_board.period);
    assert_eq!(board.cells, retrieved_board.cells);
}
