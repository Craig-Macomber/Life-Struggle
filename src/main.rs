#![feature(inclusive_range_syntax)]
mod tile;
mod board;
extern crate time;
use time::PreciseTime;
use tile::*;
use board::Board;

fn main() {
    println!("Life Struggle");
    let size = 400;

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
        let mut q = |x: usize, y: usize| b.set(x, y, true);

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
    b = b.mirror();

    let start = PreciseTime::now();
    struggle(&a, &b);
    println!(
        "Time in MS: {}",
        start.to(PreciseTime::now()).num_milliseconds()
    )
}

// Life Struggle:
// 1 vs 1 competitive version of Conway's Game of Life
// Each player (a and b) provide a tile design of the same square dimensions.
// The world is an infinite plane of tiles,
// split along x=0, with player a tiles to the -x and player b tiles to the +x
// Player b's tiles are mirrored so both can be assume enemy tiles to the +x direction.
// After some fixed number of generations, a score is computed:
// 1 point added for each tile of enemy territory converted into your tile
// 1 point deducted for each tile of your territory disrupted.
fn struggle(tile_a: &LifeTileSrc, tile_b: &LifeTileSrc) {
    // Tile format to use for simulation
    type T = VecTile;

    let bit_tile_a = T::copy_from(tile_a);
    let bit_tile_b = T::copy_from(tile_b).mirror();

    if bit_tile_a == bit_tile_b {
        // Can not distinguish the two players, they are identical after mirroring
        println!("Mirror Draw!");
        return;
    }

    let mut b = Board::new(&bit_tile_a, &bit_tile_b);
    println!("Cycle lengths: {}, {}", b.cycle_a.len(), b.cycle_b.len());
    println!("Begin!");

    //b.print();

    for _ in 0..100 {
        b.next_generation();
        if b.generation % 50 == 0 {
            println!("generation: {}", b.generation);
        }
    }

    //b.print();
    let (score_a, score_b) = b.score();
    println!("Score: {} to {}!", score_a, score_b);
}
