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

    /// Measure the fitness for a "saved" board
    pub fn measure_fitness_saved(board: &board::Saved) -> isize {
        measure(board.solved.period, board.solved.iterations)
    }

    /// Measure the fitness for a "solved" board
    pub fn measure_fitness_solved(board: &board::Solved) -> isize {
        measure(board.period, board.iterations)
    }

    /// Variables we mate over:
    /// * number of cells
    /// * number of starting subdivisions
    /// * dispersement over starting subdivisions
    pub fn mate(board1: &board::Saved, board2: &board::Saved) -> board::Initial {
        // Mate

        // Get the average of the two boards' cell counts as a starting point
        let mut num_cells =
            (board1.solved.initial.cells.len() + board2.solved.initial.cells.len()) / 2;
        let mut starting_subdivisions = (board1.solved.initial.starting_subdivisions
            + board2.solved.initial.starting_subdivisions)
            / 2;
        let mut starting_subdiv_utilization = (board1.solved.initial.starting_subdiv_utilization
            + board2.solved.initial.starting_subdiv_utilization)
            / 2;

        // Mutate
        num_cells = mutate_integer(&num_cells, 25);
        starting_subdivisions = mutate_integer(&starting_subdivisions, 3);
        starting_subdiv_utilization = mutate_integer(&starting_subdiv_utilization, 3);

        // we can't utilize more than exists
        starting_subdiv_utilization =
            std::cmp::min(starting_subdivisions, starting_subdiv_utilization);

        board::Initial {
            size: board1.solved.initial.size,
            cells: random_cells(board1.solved.initial.size, num_cells),
            starting_subdivisions,
            starting_subdiv_utilization,
        }
    }

    /// Via some strategy, gets a new board ready to solve. If there're enough boards in the DB,
    /// it'll mate two and return the child. Otherwise it'll create a random one.
    fn get_next_board(&self) -> board::Initial {
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
    fn generate_random_starter_board(&self) -> board::Initial {
        let num_cells = thread_rng().gen_range(1..=1000);
        let starting_subdivisions = thread_rng().gen_range(1..=15);
        let mut starting_subdiv_utilization = thread_rng().gen_range(1..=15);

        // We cannot utilize more than what we have
        starting_subdiv_utilization =
            std::cmp::min(starting_subdivisions, starting_subdiv_utilization);

        board::Initial {
            size: self.size,
            starting_subdivisions,
            starting_subdiv_utilization,
            cells: random_cells(self.size, num_cells),
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
                initial: board::Initial {
                    size,
                    cells: board.cells,
                    starting_subdivisions: board.starting_subdivisions,
                    starting_subdiv_utilization: board.starting_subdiv_utilization,
                },
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
                    if Evolver::measure_fitness_saved(&least_fit)
                        > Evolver::measure_fitness_saved(board)
                    {
                        least_fit = board;
                    }
                }

                // Check our newly solved board against the least fit of the saved boards
                if Evolver::measure_fitness_solved(&new_solved_board)
                    > Evolver::measure_fitness_saved(&least_fit)
                {
                    println!(
                        "thread {} made a more fit board! It has {} cells, starting_subdivisions of {}, starting_subdiv_utilization of {}
                        and is of period {:?} and has {} iterations",
                        thread_num,
                        new_solved_board.initial.cells.len(),
                        new_solved_board.initial.starting_subdivisions,
                        new_solved_board.initial.starting_subdiv_utilization,
                        new_solved_board.period,
                        new_solved_board.iterations
                    );
                    // If we're more fit than the least fit one, replace it in the db
                    self.db.save_board(&new_solved_board).unwrap();
                    self.db.delete_board(&least_fit.id).unwrap();
                } else {
                    println!(
                        "thread {} made an unfit board with {} cells, starting_subdivisions of {} and starting_subdiv_utilization of {}",
                        thread_num,
                        new_solved_board.initial.cells.len(),
                        new_solved_board.initial.starting_subdivisions,
                        new_solved_board.initial.starting_subdiv_utilization
                    );
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

/// This is actually the main fitness measurement right here
fn measure(period: Option<usize>, iterations: usize) -> isize {
    match period {
        Some(period) => (period * 20 + iterations) as isize,
        None => iterations as isize,
    }
}

fn mutate_integer(int: &usize, variation: usize) -> usize {
    let mut mutated = 0;
    let addition = thread_rng().gen_range(0..=variation);
    if thread_rng().gen_bool(0.5) {
        mutated = int + &addition;
    } else {
        // Gotta make sure not to dip below 0
        if int >= &addition {
            mutated = int - &addition;
        }
    }

    mutated
}
