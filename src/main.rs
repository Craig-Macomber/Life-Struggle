#![feature(inclusive_range_syntax)]
mod tile;
mod board;
mod game;
extern crate time;
use time::PreciseTime;
use tile::*;

fn main() {
    println!("Life Struggle");

    let size = 8;

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

        let b = VecTile::new(size);

    let start = PreciseTime::now();
    let (score_a, score_b) = game::struggle(100, &a, &b);
    println!(
        "Time in MS: {}",
        start.to(PreciseTime::now()).num_milliseconds()
    );
    println!("Score: {} to {}", score_a, score_b);
}
