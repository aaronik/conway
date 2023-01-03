extern crate drawille;

use drawille::Canvas;
use drawille::PixelColor;
use rand::Rng;
use std::{thread, time};
use termsize;
use std::collections::HashMap;

const RATE: u64 = 500;

// Conway's rules:
// Births: Each dead cell adjacent to exactly three live neighbors will become live in the next generation.
// Death by isolation: Each live cell with one or fewer live neighbors will die in the next generation.
// Death by overcrowding: Each live cell with four or more live neighbors will die in the next generation.
// Survival: Each live cell with either two or three live neighbors will remain alive for the next generation.

fn main() {
    let termsize::Size { rows, cols } = termsize::get().unwrap();
    // let size: u32 = std::cmp::min(rows, cols) as u32;
    let size: u32 = 100;

    let mut canvas = Canvas::new(size, size);

    let mut living_cells: HashMap<String, bool> = HashMap::new();

    let midpoint = size / 2;
    living_cells.insert(format_cell_key(midpoint, midpoint), true);
    living_cells.insert(format_cell_key(midpoint, midpoint + 1), true);
    living_cells.insert(format_cell_key(midpoint + 1, midpoint), true);
    living_cells.insert(format_cell_key(midpoint + 1, midpoint + 1), true);
    living_cells.insert(format_cell_key(midpoint + 1, midpoint + 2), true);
    living_cells.insert(format_cell_key(midpoint + 1, midpoint + 3), true);
    living_cells.insert(format_cell_key(midpoint + 1, midpoint + 4), true);

    loop {
        thread::sleep(time::Duration::from_millis(RATE));
        canvas.clear();

        for i in 1..size {
            for j in 1..size {
                let key = format_cell_key(i, j);
                // Perform rules on cell
                let num_living_neighbors = get_num_living_neighbors(i, j, &living_cells);

                // Every living cell with <= 1 neighbor dies
                if num_living_neighbors <= 1 {
                    living_cells.remove(&key);
                }

                // Every living cell with >= 4 neighbors dies
                if num_living_neighbors >= 4 {
                    living_cells.remove(&key);
                }

                // // Each live cell with 2 or 3 neighbors lives
                // if living_cells.contains_key(&key) && (num_living_neighbors == 2 || num_living_neighbors == 3) {
                //     // we stay alive, these checks are not necessary
                // }

                // Every dead cell with 3 neighbors is born
                if !living_cells.contains_key(&key) && num_living_neighbors == 3 {
                    living_cells.insert(key.clone(), true);
                }

                // Draw the cell if it's alive
                if let Some(_) = living_cells.get(&key) {
                    let color = random_color();
                    canvas.set_colored(j, i, color);
                }
            }
        }

        print!("{}[2J", 27 as char); // Clear the term
        print!("{}", canvas.frame());
    }
}

pub struct Cells {
    living_cells: HashMap<String, bool>
}

fn get_num_living_neighbors(i: u32, j: u32, living_cells: &HashMap<String, bool>) -> u32 {
    let neighbors = vec![
        format_cell_key(i - 1, j - 1),
        format_cell_key(i - 1, j),
        format_cell_key(i - 1, j + 1),
        format_cell_key(i, j - 1),
        format_cell_key(i, j + 1),
        format_cell_key(i + 1, j - 1),
        format_cell_key(i + 1, j),
        format_cell_key(i + 1, j + 1)
    ];

    let mut num_living_neighbors = 0;

    neighbors.iter().for_each(|key| {
        if let Some(_) = living_cells.get(key) {
            num_living_neighbors += 1;
        }
    });

    num_living_neighbors
}

fn format_cell_key(i: u32, j: u32) -> String {
    format!("{i}-{j}")
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
