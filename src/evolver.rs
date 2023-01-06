use crate::{board, Cells};
use rand::{thread_rng, Rng};

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
        let second_index = thread_rng().gen_range(0..boards.len() - 1);

        (boards.remove(first_index), boards.remove(second_index))
    }

    // Maybe we don't have enough boards in the pool -- sometimes we need to just make
    // a random one
    fn generate_random_starter_board(&self) -> board::Unsolved {
        let num_cells = thread_rng().gen_range(1..=200);
        let initial_cells = random_cells(self.size, num_cells);

        board::Unsolved {
            size: self.size,
            cells: initial_cells,
        }
    }

    pub fn begin_evolving(&mut self, thread_num: u32) {
        let size = self.size;

        // Variables we're iterating over, and can bring around via mating
        // * Number of initial cells
        // * Size of starting area, right now it's 1/5 right in the middle

        loop {
            // Get our strategically generated new board
            let board = self.get_next_board();

            // populate our cells container
            let mut cells = Cells::new(size);
            cells.birth_multiple(&board.cells);

            let snapshot = crate::Snapshot::new(size);

            // Game's responsibility is to provide the step() function and a few
            // winning metrics.
            let mut game = crate::Game::new(Some(snapshot), cells, None);

            // Iterate a single board
            loop {
                game.step();

                // Bail if it's a barren death land
                if game.cells.num_living_cells() == 0 {
                    break;
                }

                // Bail if we're in an infinite loop
                if let Some(snapshot) = &game.snapshot {
                    if snapshot.has_repeat() {
                        break;
                    }
                }
            }

            // After that loop, the board's been solved. Now we'll check it to see
            // its fitness.
            let new_solved_board = board::Solved {
                size,
                cells: board.cells,
                iterations: game.iterations,
                period: game.snapshot.unwrap().period(),
            };

            let boards = self.db.load_boards().unwrap();

            // We'll keep the best 10 configurations.
            if boards.len() < 10 {
                // If we don't have 10 yet, every configuration is in the best 10.
                println!(
                    "thread {} found insuficiently populated database and is saving board",
                    thread_num
                );
                self.db
                    .save_board(&new_solved_board)
                    .expect("error saving board");
            } else {
                // Find the least fit saved board
                let mut least_fit = boards.get(0).unwrap();
                for board in &boards {
                    let least = board::Measurable::Saved(least_fit);
                    let current = board::Measurable::Saved(board);
                    if Evolver::measure_fitness(&least) > Evolver::measure_fitness(&current) {
                        least_fit = board;
                    }
                }

                // Check our newly solved board against the least fit of the saved boards
                let least = board::Measurable::Saved(least_fit);
                let ours = board::Measurable::Solved(&new_solved_board);
                if Evolver::measure_fitness(&ours) > Evolver::measure_fitness(&least) {
                    println!(
                    "thread {} made a more fit board! It is of period {:?} and has {} iterations",
                    thread_num,
                    new_solved_board.period,
                    new_solved_board.iterations
                );
                    // If we're more fit than the least fit one, replace it in the db
                    self.db.save_board(&new_solved_board).unwrap();
                    self.db.delete_board(&least_fit.id).unwrap();
                } else {
                    println!("thread {} made an unfit board", thread_num);
                }
            }
        }
    }
}

// TODO We should evolve over the numbers in range_i too
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