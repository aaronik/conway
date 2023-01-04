use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Error};

pub struct Db {
    connection: PooledConnection<SqliteConnectionManager>,
}

// Get all the boards
// SELECT board_id,size,i,j FROM Boards INNER JOIN Cells ON Boards.id = Cells.board_id;

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
            size INTEGER NOT NULL
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
    pub fn save_board(&mut self, size: u32, cells: &Vec<(u32, u32)>) -> Result<(), Error> {
        // Insert one new board
        self.connection.execute("INSERT INTO Boards (size) VALUES (?)", params![size])?;

        // What was that last id?
        let board_id = self.connection.last_insert_rowid();

        let tx = self.connection.transaction()?;
        for (i, j) in cells.iter() {
            tx.execute(
                "INSERT INTO Cells (board_id, i, j) VALUES (?, ?, ?)",
                params![board_id, i, j],
            )?;
        }
        tx.commit()?;

        Ok(())
    }
}
