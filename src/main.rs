#![feature(inclusive_range_syntax)]
mod tile;
mod board;
mod game;
extern crate time;
use time::PreciseTime;
use tile::*;

fn main() {
    println!("Life Struggle");
    let size = 40;

        let mut a = VecTile::new(size);
        {
            let mut q = |x: usize, y: usize| a.set(x, y, true);

            // Light weight space ship going +x
            q(0, 0);
            q(0, 2);
            q(1, 3);
            q(2, 3);
            q(3, 3);
            q(4, 3);
            q(4, 2);
            q(4, 2);
            q(4, 1);
            q(3, 0);
        }

        let mut b = VecTile::new(size);
        {
            let mut q = |x: usize, y: usize| b.set(x, y, false);

            // Two gliders going -x+y
            q(9, 0);
            q(8, 1);
            q(10, 2);
            q(9, 2);
            q(8, 2);

            q(9, 10);
            q(8, 11);
            q(10, 12);
            q(9, 12);
            q(8, 12);
        }

        // send gliders +x+y
        //b = b.mirror();
    let start = PreciseTime::now();
    let (score_a, score_b) = game::struggle(1000, &a, &b);
    println!(
        "Time in MS: {}",
        start.to(PreciseTime::now()).num_milliseconds()
    );
    println!("Score: {} to {}", score_a, score_b);
}
