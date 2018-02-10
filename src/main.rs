use std::collections::BTreeMap;
extern crate num_integer;
extern crate rayon;
use num_integer::Integer;
use rayon::prelude::*;

fn main() {
    println!("Life Struggle");
    let mut a = Tile::new(20);
    //a.set(1, 0, true);
    //a.set(2, 1, true);
    //a.set(0, 2, true);
    //a.set(1, 2, true);
    //a.set(2, 2, true);
    {
        let mut q = |x: usize, y: usize| a.set(x, y, true);

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

    let mut b = Tile::new(20);
    //b.set(0, 2, true);
    //b.set(1, 2, true);
    //b.set(2, 2, true);

    b.set(9, 0, true);
    b.set(8, 1, true);
    b.set(10, 2, true);
    b.set(9, 2, true);
    b.set(8, 2, true);

    b.set(9, 10, true);
    b.set(8, 11, true);
    b.set(10, 12, true);
    b.set(9, 12, true);
    b.set(8, 12, true);

    b = b.mirror();

    struggle(&a, &b);
}

fn struggle(tile_a: &Tile, tile_b: &Tile) {
    let tile_b_mirror = tile_b.mirror();
    let mut b = Board::new(tile_a, &tile_b_mirror);
    println!(
        "Cycle lengths: {}, {}",
        b.cycle_a.tiles.len(),
        b.cycle_b.tiles.len()
    );
    println!("Begin!");

    b.print();

    for _ in 0..10000 {
        b.next_generation();
    }

    b.print();
    let (score_a, score_b) = b.score();
    println!("Score: {} to {}!", score_a, score_b);
}

#[derive(Debug)]
struct Board {
    tile_size: usize,
    tiles: BTreeMap<isize, Tile>,
    generation: usize,
    cycle_a: TileCycle,
    cycle_b: TileCycle,
}

#[derive(Debug, Clone)]
struct Tile {
    pub size: usize,
    cells: Vec<bool>,
}

impl Board {
    fn new(a: &Tile, b: &Tile) -> Board {
        assert!(a.size == b.size);
        let tiles = BTreeMap::new();

        Board {
            tile_size: a.size,
            cycle_a: TileCycle::new(a),
            cycle_b: TileCycle::new(b),
            tiles: tiles,
            generation: 0,
        }
    }

    fn next_generation(&mut self) {
        let mut tiles_new = BTreeMap::new();

        let first = self.first_or(-1) - 1;
        let last = self.last_or(0) + 1;

        for x in (first..last + 1) {
            let t = self.tile_at(x);
            let tnew = t.next_generation(self.tile_at(x - 1), self.tile_at(x + 1));

            if &tnew != self.default_tile_at_gen(x, self.generation + 1) {
                tiles_new.insert(x, tnew);
            }
        }

        self.tiles = tiles_new;
        self.generation += 1;

        if self.generation % 200 == 0 {
            println!("generation: {}", self.generation);
        }
    }

    fn score(&self) -> (isize, isize) {
        let mut score_a = 0;
        let mut score_b = 0;

        for (x, tile) in self.tiles.iter() {
            if x < &0 {
                score_a -= 1;
                if tile == self.cycle_b.default_at_gen(self.generation) {
                    score_b += 1;
                }
            } else {
                score_b -= 1;
                if tile == self.cycle_a.default_at_gen(self.generation) {
                    score_a += 1;
                }
            }
        }
        return (score_a, score_b);
    }

    fn first_or(&self, x: isize) -> isize {
        *self.tiles.keys().next().unwrap_or(&x)
    }

    fn last_or(&self, x: isize) -> isize {
        *self.tiles.keys().next_back().unwrap_or(&x)
    }

    fn print(&self) {
        let first = self.first_or(-1);
        let last = self.last_or(0);

        for x in first..last + 1 {
            print!("|{x:^width$}|", x = x, width = self.tile_size - 2);
        }
        println!();
        for y in 0..self.tile_size {
            for x in first..last + 1 {
                self.tile_at(x).print_line(y);
            }
            println!();
        }
    }

    fn default_tile_at(&self, x: isize) -> &Tile {
        self.default_tile_at_gen(x, self.generation)
    }

    fn default_tile_at_gen(&self, x: isize, generation: usize) -> &Tile {
        let cycle = if x < 0 {
            &self.cycle_a
        } else {
            &self.cycle_b
        };
        return &cycle.default_at_gen(generation);
    }

    fn tile_at(&self, x: isize) -> &Tile {
        match self.tiles.get(&x) {
            Some(v) => v,
            None => self.default_tile_at(x),
        }
    }
}

impl TileCycle {
    // tile, when in an infinate grid of itself, must repeate (or will assert)
    // evaluate that repeat cycle, and store it.
    fn new(tile: &Tile) -> TileCycle {
        let mut tc = TileCycle { tiles: vec![] };
        let mut t = tile.clone();
        loop {
            let t2 = t.next_generation(&t, &t);
            tc.tiles.push(t);
            if &t2 == tile {
                break;
            }
            t = t2;

            if tc.tiles.len() > 2 {
                // Check illegal start tiles that converge to a cycle that does not include the start state
                // Floyd’s Cycle detection algorithm
                let a = tc.tiles.len() / 2;
                let b = tc.tiles.len() - 1;
                assert!(tc.tiles[a] != tc.tiles[b], "a = {}, b = {}", a, b);
            }
        }

        return tc;
    }

    fn default_at_gen(&self, generation: usize) -> &Tile {
        &self.tiles[generation % self.tiles.len()]
    }
}

#[derive(Debug)]
struct TileCycle {
    pub tiles: Vec<Tile>,
}

impl Tile {
    fn new(size: usize) -> Tile {
        Tile {
            size: size,
            cells: vec![false; size * size],
        }
    }

    fn mirror(&self) -> Tile {
        let mut t = Tile {
            size: self.size,
            cells: vec![false; self.size * self.size],
        };

        for x in 0..self.size {
            for y in 0..self.size {
                t.set(x, y, self.get(self.size - x - 1, y));
            }
        }
        return t;
    }

    fn next_generation(&self, previous: &Tile, next: &Tile) -> Tile {
        let size = self.size;

        let at = |x: isize, y: isize| {
            let y2 = y.mod_floor(&(size as isize));
            let x2 = x.mod_floor(&(size as isize));
            let t = if x < 0 {
                &previous
            } else if x >= size as isize {
                &next
            } else {
                &self
            };

            return t.get(x2 as usize, y2 as usize);
        };

        let mut t = Tile::new(size);

        for x in 0..size {
            for y in 0..size {
                let mut c = 0;

                for xx in -1isize..2 {
                    for yy in -1isize..2 {
                        if at(xx + x as isize, yy + y as isize) {
                            c += 1;
                        }
                    }
                }

                let v = if self.get(x, y) {
                    c == 3 || c == 4
                } else {
                    c == 3
                };

                t.set(x, y, v);
            }
        }
        return t;
    }

    fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.size
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.cells[self.index(x, y)]
    }

    fn set(&mut self, x: usize, y: usize, value: bool) {
        let index = self.index(x, y);
        self.cells[index] = value;
    }

    fn print_line(&self, y: usize) {
        for x in 0..self.size {
            let s = if self.get(x, y) { "X" } else { "." };
            print!("{}", s);
        }
    }

    fn print(&self) {
        for y in 0..self.size {
            self.print_line(y);
            println!();
        }
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Tile) -> bool {
        self.cells == other.cells
    }
}
impl Eq for Tile {}