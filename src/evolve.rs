use crate::Board;

pub struct Evolution {
    db: Option<crate::Db>,
}

impl Evolution {
    pub fn new(db: Option<crate::Db>) -> Self {
        Self { db }
    }

    /// this fitness function weighs period much heavier than iterations,
    /// because that's what I think it should be.
    pub fn measure_fitness(board: Board) -> usize {
        match board.period {
            Some(period) => period * 3 + board.iterations,
            None => board.iterations
        }
    }
}
