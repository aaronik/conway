use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Error};

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
        self.connection
            .execute("INSERT INTO Boards (size) VALUES (?)", params![size])?;

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

    pub fn load_board(&self, board_id: usize) -> Result<(u32, Vec<(u32, u32)>), Error> {
        let size: u32 = self.connection.query_row("SELECT size FROM Boards WHERE id = ?", params![board_id], |row| row.get(0))?;

        let mut stmt = self.connection.prepare(
            "SELECT i,j FROM Boards INNER JOIN Cells ON Boards.id = Cells.board_id WHERE Boards.id = ?",
        )?;
        let cells_iter = stmt.query_map(params![board_id], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;

        let mut cells_vec: Vec<(u32, u32)> = vec![];

        for cell in cells_iter {
            let cell = cell.expect("Error in loading board");
            cells_vec.push((cell.0, cell.1))
        }

        Ok((size, cells_vec))
    }
}
