use super::{Cells, Snapshot};
use drawille::Canvas;
use drawille::PixelColor;
// use rand::Rng;

pub struct Game {
    pub snapshot: Option<Snapshot>,
    pub canvas: Option<Canvas>,
    pub cells: Cells,
    pub iterations: usize,
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
            iterations: 0,
        }
    }

    // Conway's rules:
    // Births: Each dead cell adjacent to exactly three live neighbors will become live in the next generation.
    // Death by isolation: Each live cell with one or fewer live neighbors will die in the next generation.
    // Death by overcrowding: Each live cell with four or more live neighbors will die in the next generation.
    // Survival: Each live cell with either two or three live neighbors will remain alive for the next generation.

    pub fn step(&mut self) {
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
                        snapshot.add_cell(i, j);
                    }

                    // If there's a canvas, draw on it
                    if let Some(canvas) = &mut self.canvas {
                        // Note this won't be perfect because the term can only make a unique color
                        // per character, and the brail characters come out multiple in one
                        // character slot
                        canvas.set_colored(j, i, color_by_age(self.cells.get_age(i, j)));
                    }
                }
            });

        self.cells.commit();

        self.iterations += 1;

        // Draw
        if let Some(canvas) = &mut self.canvas {
            print!("{}[2J", 27 as char); // Clear the term
            println!("{}\n", canvas.frame()); // \n helps prevent _some_ jitteriness
            canvas.clear();
        }

        // Keep track
        if let Some(snapshot) = &mut self.snapshot {
            snapshot.commit_cells();
        }

    }
}

fn color_by_age(age: usize) -> PixelColor {
    match age {
        1 => PixelColor::BrightBlack,
        2 => PixelColor::Blue,
        3 => PixelColor::BrightBlue,
        4 => PixelColor::Magenta,
        5 => PixelColor::BrightMagenta,
        6 => PixelColor::Cyan,
        7 => PixelColor::BrightCyan,
        8 => PixelColor::Green,
        9 => PixelColor::BrightGreen,
        10 => PixelColor::Yellow,
        11 => PixelColor::BrightYellow,
        12 => PixelColor::Red,
        _ => PixelColor::BrightRed,
    }
}
