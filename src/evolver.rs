use crate::{board, Cells};
use drawille::Canvas;
use rand::{Rng, thread_rng};

// The evolver's responsibility is to:
// * Orchestrate a single thread of evolution
//   * Easily callable from whoever is orchestrating threads
//   * House all the logic about evolution

pub struct Evolver {
    db: crate::Db,
    size: u32,
}

impl Evolver {
    pub fn new(size: u32, db: crate::Db) -> Self {
        Self { db, size }
    }

    /// this fitness function weighs period much heavier than iterations,
    /// because that's what I think it should be.
    pub fn measure_fitness(board: &board::Measurable) -> isize {
        fn measure(period: Option<usize>, iterations: usize) -> isize {
            match period {
                Some(period) => (period * 20 + iterations) as isize,
                None => iterations as isize,
            }
        }

        match &board {
            &board::Measurable::Saved(s) => measure(s.period, s.iterations),
            &board::Measurable::Solved(s) => measure(s.period, s.iterations),
        }
    }

    /// Variables we mate over:
    /// * number of cells
    pub fn mate(board1: &board::Saved, board2: &board::Saved) -> board::Unsolved {
        // Mate
        let mut num_cells = (board1.cells.len() + board2.cells.len()) / 2;

        // Mutate
        let addition = thread_rng().gen_range(0..=5);
        if thread_rng().gen_bool(0.5) {
            num_cells = num_cells + addition
        } else {
            // Gotta make sure not to dip below 0
            if num_cells - addition > 0 {
                num_cells = num_cells - addition
            }
        }

        board::Unsolved {
            cells: random_cells(board1.size, num_cells),
            size: board1.size,
        }
    }

    /// Via some strategy, gets a new board ready to solve. If there're enough boards in the DB,
    /// it'll mate two and return the child. Otherwise it'll create a random one.
    fn get_next_board(&self) -> board::Unsolved {
        let board_count = self.db.get_board_count().unwrap();

        if board_count < 2 {
            return self.generate_random_starter_board();
        }

        let (board1, board2) = self.retrieve_two_fit_individuals();

        Evolver::mate(&board1, &board2)
    }

    /// Pick two individuals from the database at random
    /// panics if there are fewer than two boards saved in the db
    fn retrieve_two_fit_individuals(&self) -> (board::Saved, board::Saved) {
        let mut boards = self.db.load_boards().unwrap();

        if boards.len() < 2 {
            panic!("retrieve_two_fit_individuals called with fewer than two boards in the db");
        }

        let first_index = thread_rng().gen_range(0..boards.len());
        let second_index = thread_rng().gen_range(0..boards.len());

        (boards.remove(first_index), boards.remove(second_index))
    }

    fn generate_random_starter_board(&self) -> board::Unsolved {
        let num_cells = thread_rng().gen_range(1..=200);
        let initial_cells = random_cells(self.size, num_cells);

        board::Unsolved {
            size: self.size,
            cells: initial_cells,
        }
    }

    pub fn begin_evolving(&mut self, thread_num: i32) {
        let size = self.size;

        // Variables we're iterating over, and can bring around via mating
        // * Number of initial cells
        // * Size of starting area, right now it's 1/5 right in the middle

        loop {
            // To create a real evolution, we're gonna want to:
            // * Pick two boards at random from the list of best performers

            let board = self.get_next_board();

            let mut cells = Cells::new(size);
            cells.birth_multiple(&board.cells);

            let snapshot = crate::Snapshot::new(size);
            let canvas: Option<Canvas>;

            // This is weird, makes a kind of illusion of there being a single evolution.
            // We're really just showing the results from a single thread.
            if thread_num == 0 {
                canvas = Some(Canvas::new(size, size));
            } else {
                canvas = None
            }

            let mut game = crate::Game::new(snapshot, cells, canvas);

            // Iterate a single board
            loop {
                game.step();

                // Bail if it's a barren death land
                if game.cells.num_living_cells() == 0 {
                    break;
                }

                // Bail if we're in an infinite loop
                if game.snapshot.has_repeat() {
                    break;
                }
            }

            let new_solved_board = board::Solved {
                size,
                cells: board.cells,
                iterations: game.iterations,
                period: game.snapshot.period(),
            };

            // * Remove weakest board (fitness)
            let boards = self.db.load_boards().unwrap();
            if boards.len() < 10 {
                // TODO use var instead of raw 10
                // Add boards to the list
                self.db
                    .save_board(&new_solved_board)
                    .expect("error saving board");
            } else {
                // Check our board against all the rest
                // If we're not the weakest, delete that and add ours
                for board in &boards {
                    let new_measurable = board::Measurable::Solved(&new_solved_board);
                    let old_measurable = board::Measurable::Saved(board);
                    if Evolver::measure_fitness(&new_measurable)
                        > Evolver::measure_fitness(&old_measurable)
                    {
                        self.db.save_board(&new_solved_board).unwrap();
                        self.db.delete_board(&board.id).unwrap();
                        break; // replace only a single board, so the best board doesn't
                               // overwrite the whole set
                    }
                }
            }
        }
    }
}

fn random_cells(size: u32, num: usize) -> Vec<(u32, u32)> {
    let mut cells = vec![];

    for _ in 0..num {
        let range_i = (size * 2) / 5..(size * 3) / 5;
        let range_j = range_i.clone();
        let rand_i = thread_rng().gen_range(range_i);
        let rand_j = thread_rng().gen_range(range_j);
        cells.push((rand_i, rand_j));
    }

    cells
}
