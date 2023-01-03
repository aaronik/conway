use super::{Cells, Snapshot};
use drawille::Canvas;
use drawille::PixelColor;
use rand::Rng;

pub struct Game {
    pub snapshot: Option<Snapshot>,
    pub canvas: Option<Canvas>,
    pub cells: Cells,
    pub unique_iterations: usize,
}

impl Game {
    /// Implement a new Game object, which orchestrates the conways game of life. Pass in a canvas
    /// and it'll print the game to the screen at every step.
    pub fn new(
        snapshot: Option<Snapshot>,
        cells: Cells,
        canvas: Option<Canvas>,
    ) -> Game {
        Game {
            snapshot,
            canvas,
            cells,
            unique_iterations: 0,
        }
    }

    // Conway's rules:
    // Births: Each dead cell adjacent to exactly three live neighbors will become live in the next generation.
    // Death by isolation: Each live cell with one or fewer live neighbors will die in the next generation.
    // Death by overcrowding: Each live cell with four or more live neighbors will die in the next generation.
    // Survival: Each live cell with either two or three live neighbors will remain alive for the next generation.

    pub fn step(&mut self) {
        // Optimization:
        // Prevent from iterating over every cell on map:
        //   - Get live cells from Cells
        //   - Get all neighbors, uniquely (loving me some hash sets these days)
        //   - Iterate through those

        // TODO Hah Ok the above optimzation worked so profoundly, especially when the grid was
        // only thinly populated. However, the HashSet returns its list in an unordered way,
        // and because of the issue I noted in main of updating the grid while analyzing the grid,
        // we're getting nondeterministic behavior, l0lz. Needa have cells be able to cycle like
        // snapshot. Then snapshot needs to not do that anymore, hah.
        self.cells
            .living_cells_and_neighbors()
            .iter()
            .for_each(|coord| {
                let i = coord.0;
                let j = coord.1;

                // Perform rules on cell
                let num_living_neighbors = self.cells.num_living_neighbors(i, j);

                // Every living cell with <= 1 neighbor dies
                if num_living_neighbors <= 1 {
                    self.cells.kill(i, j);
                }

                // Every living cell with >= 4 neighbors dies
                if num_living_neighbors >= 4 {
                    self.cells.kill(i, j);
                }

                // Each live cell with 2 or 3 neighbors lives
                if self.cells.is_alive(i, j) && [2, 3].contains(&num_living_neighbors) {
                    self.cells.birth(i, j); // We say birth, but mean stay alive
                }

                // Every dead cell with 3 neighbors is born
                if !self.cells.is_alive(i, j) && num_living_neighbors == 3 {
                    self.cells.birth(i, j);
                }

                // Draw the cell if it's alive, add to snapshot
                // This'll be behind by one iteration
                if self.cells.is_alive(i, j) {
                    if let Some(snapshot) = &mut self.snapshot {
                        snapshot.add(i, j);
                    }

                    // If there's a canvas, draw on it
                    if let Some(canvas) = &mut self.canvas {
                        canvas.set_colored(j, i, random_color());
                    }
                }
            });

        self.cells.commit();

        // Draw
        if let Some(canvas) = &mut self.canvas {
            print!("{}[2J", 27 as char); // Clear the term
            print!("{}", canvas.frame());
            canvas.clear();
        }

        // Keep track
        if let Some(snapshot) = &mut self.snapshot {
            if !snapshot.is_same() {
                self.unique_iterations += 1;
            }
            snapshot.cycle();
        }
    }
}

fn random_color() -> PixelColor {
    let rnum = rand::thread_rng().gen_range(1..=11);
    match rnum {
        1 => PixelColor::Blue,
        2 => PixelColor::Cyan,
        3 => PixelColor::BrightBlack,
        4 => PixelColor::Green,
        5 => PixelColor::BrightGreen,
        6 => PixelColor::Yellow,
        7 => PixelColor::Magenta,
        8 => PixelColor::BrightRed,
        9 => PixelColor::BrightYellow,
        10 => PixelColor::BrightBlue,
        11 => PixelColor::BrightCyan,
        _ => PixelColor::Cyan,
    }
}
