extern crate drawille;

use drawille::Canvas;
use drawille::PixelColor;
use rand::Rng;
use std::{thread, time};

const SIZE: u32 = 150;
const RATE: u64 = 500;

fn main() {
    let mut canvas = Canvas::new(SIZE, SIZE);

    let matrix = [[1; SIZE as usize]; SIZE as usize];

    loop {
        thread::sleep(time::Duration::from_millis(RATE));
        canvas.clear();

        for i in 1..SIZE {
            for j in 1..SIZE {
                if matrix[i as usize][j as usize] == 1 {
                    let color = random_color();
                    canvas.set_colored(j, i, color);
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
