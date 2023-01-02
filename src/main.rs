extern crate drawille;

use drawille::Canvas;
use drawille::PixelColor;
use rand::Rng;
use std::{thread, time};

const SIZE: u32 = 150;
const RATE: u64 = 500;

// When x is size / 2, y is at size
// when y is size - 1, x is size / 2+ 1
// when y is size - 2, x is size / 2 + 2
fn main() {
    let mut canvas = Canvas::new(10, 10);

    let matrix = [[1; SIZE as usize]; SIZE as usize];

    loop {
        thread::sleep(time::Duration::from_millis(RATE));
        canvas.clear();

        canvas.set(SIZE, SIZE); // to ensure the thing stays the same size on every pass

        for i in 1..SIZE {
            for j in 1..SIZE {
                if matrix[i as usize][j as usize] == 1 {
                    let color = random_color();
                    canvas.set_colored(i, j, color);
                }
            }
        }

        print!("{}[2J", 27 as char); // Clear the term
        println!("{}", canvas.frame());
    }
}

fn random_color() -> PixelColor {
    let rnum = rand::thread_rng().gen_range(1..=12);
    match rnum {
        1 => PixelColor::Blue,
        2 => PixelColor::Cyan,
        3 => PixelColor::Black,
        4 => PixelColor::Green,
        5 => PixelColor::BrightGreen,
        6 => PixelColor::Yellow,
        7 => PixelColor::Magenta,
        8 => PixelColor::BrightRed,
        9 => PixelColor::BrightYellow,
        10 => PixelColor::BrightBlue,
        11 => PixelColor::BrightCyan,
        12 => PixelColor::BrightBlack,
        _ => PixelColor::Cyan,
    }
}
