use board::*;
use tile::*;

// Tile format to use for simulation
type T = VecTile;
type B = VecBoard<T>;

// Life Struggle:
// 1 vs 1 competitive version of Conway's Game of Life
// Each player (a and b) provide a tile design of the same square dimensions.
// The world is an infinite plane of tiles,
// split along x=0, with player a tiles to the -x and player b tiles to the +x
// Player b's tiles are mirrored so both can be assume enemy tiles to the +x direction.
// After some fixed number of generations, a score is computed:
// 1 point added for each tile of enemy territory converted into your tile
// 1 point deducted for each tile of your territory disrupted.
// Returns (score_a, score_b).
pub fn struggle(generations: usize, tile_a: &LifeTileSrc, tile_b: &LifeTileSrc) -> (isize, isize) {
    let b = struggle_board(generations, tile_a, tile_b);

    match b {
        Some(x) => {
            x.print_image("life.png");
            return x.score();
        }
        None => {
            println!("convergance draw");
            return (0, 0);
        }
    }
}

pub fn struggle_board(generations: usize, tile_a: &LifeTileSrc, tile_b: &LifeTileSrc) -> Option<B> {
    let bit_tile_a = T::copy_from(tile_a);
    let bit_tile_b = T::copy_from(tile_b).mirror();
    let mut b = B::new(bit_tile_a, bit_tile_b);

    for g in 0..generations {
        match b {
            Some(x) => {
                //x.print();
                b = x.next_generation();
            }
            None => {
                break;
            }
        }
        if g % 200 == 0 {
            //println!("generation: {}", g);
        }
    }

    return b;
}

#[cfg(test)]
use test::Bencher;

mod tests {
    use super::*;

    #[test]
    fn test_lwss_vs_gliders() {
        let size = 40;

        let mut a = VecTile::new(size);
        lwss_at(&mut a, 0, 0);

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

        let (score_a, score_b) = struggle(500, &a, &b);
        println!("Score: {} to {}", score_a, score_b);
        assert_eq!(score_a, -2);
        assert_eq!(score_b, -1);

        let (score_a, score_b) = struggle(2000, &a, &b);
        println!("Score: {} to {}", score_a, score_b);
        assert_eq!(score_a, -3);
        assert_eq!(score_b, -1);
    }

    fn lwss_at<T: LifeTile>(t: &mut T, x: usize, y: usize) {
        let mut q = |xx: usize, yy: usize| t.set(x + xx, y + yy, true);

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

    #[test]
    fn test_lwss_vs_empty() {
        let size = 8;

        let mut a = VecTile::new(size);
        lwss_at(&mut a, 0, 0);

        let b = VecTile::new(size);

        {
            let (score_a, score_b) = struggle(100, &a, &b);
            assert_eq!(score_a, 6);
            assert_eq!(score_b, -6);
        }

        {
            let (score_b, score_a) = struggle(100, &b, &a);
            assert_eq!(score_a, 6);
            assert_eq!(score_b, -6);
        }

        {
            let (score_b, score_a) = struggle(100, &a, &a);
            assert_eq!(score_a, 0);
            assert_eq!(score_b, 0);
        }

        {
            let (score_b, score_a) = struggle(100, &b, &b);
            assert_eq!(score_a, 0);
            assert_eq!(score_b, 0);
        }
    }

    #[bench]
    fn bench_lwss_200(b: &mut Bencher) {
        let size = 200;

        let mut a = VecTile::new(size);
        for x in 0..(size / 10) {
            for y in 0..(size / 10) {
                lwss_at(&mut a, x * 10, y * 10);
            }
        }

        b.iter(|| {
            a = a.next_generation(&a, &a);
        });
    }

    #[bench]
    fn bench_lwss_2000(b: &mut Bencher) {
        let size = 2000;

        let mut a = VecTile::new(size);
        for x in 0..(size / 10) {
            for y in 0..(size / 10) {
                lwss_at(&mut a, x * 10, y * 10);
            }
        }

        b.iter(|| {
            a = a.next_generation(&a, &a);
        });
    }
}
