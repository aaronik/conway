/// Before it's solved and before it's saved
pub struct Initial {
    /// size of the whole board
    pub size: u32,

    /// initial living cells
    pub cells: Vec<(u32, u32)>,

    /// How many subdivisions the board is broken down into, used
    /// for starting cell placement
    pub starting_subdivisions: usize,

    /// How many of the subdivisions the initial cells are
    /// placed into
    pub starting_subdiv_utilization: usize,
}

/// After it's solved but still before it's saved
pub struct Solved {
    pub initial: Initial,

    /// How many iterations this board ended up having
    pub iterations: usize,

    /// If the board ended up looping, how many unique iterations existed within that loop
    /// before it repeated
    pub period: Option<usize>,
}

/// From the DB, it's been solved and saved
pub struct Saved {
    pub solved: Solved,

    /// As assigned by the database
    pub id: i64,
}
