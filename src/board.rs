extern crate rayon;
use std::collections::BTreeMap;
use tile::*;

#[derive(Debug)]
pub struct Board<T>
where
    T: LifeTile,
{
    tiles: BTreeMap<isize, T>,
    pub generation: usize,
    pub cycle_a: TileCycle<T>,
    pub cycle_b: TileCycle<T>,
}

impl<T> Board<T>
where
    T: LifeTile,
{
    pub fn new(a: &T, b: &T) -> Board<T> {
        assert!(a.size() == b.size());
        assert!(a != b);

        let tiles = BTreeMap::new();

        let (ca, cb) = rayon::join(|| TileCycle::new(a), || TileCycle::new(b));

        Board {
            cycle_a: ca,
            cycle_b: cb,
            tiles: tiles,
            generation: 0,
        }
    }

    fn lowest_non_a(&self) -> isize {
        self.first_or(0)
    }

    fn highest_non_b(&self) -> isize {
        self.last_or(-1)
    }

    pub fn next_generation(&mut self) {
        let mut tiles_new = BTreeMap::new();

        let first = self.lowest_non_a() - 1;
        let last = self.highest_non_b() + 1;

        for x in first..=last {
            let t = self.tile_at(x);
            let tnew = t.next_generation(self.tile_at(x - 1), self.tile_at(x + 1));

            if &tnew != self.default_tile_at_gen(x, self.generation + 1) {
                tiles_new.insert(x, tnew);
            }
        }

        self.tiles = tiles_new;
        self.generation += 1;
    }

    pub fn score(&self) -> (isize, isize) {
        let first = self.lowest_non_a();
        let last = self.highest_non_b();

        // init to contigious tiles captured (negative is tiles were lost)
        let mut score_a = first; 
        let mut score_b = -last-1;

        // Go through contested area, and see if any tiles match a or b
        for x in first..=last {
            let t = self.tile_at(x);
            if t == self.cycle_a.default_at_gen(self.generation) {
                score_a += 1;
            } else if t == self.cycle_b.default_at_gen(self.generation) {
                score_b += 1;
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

    fn tile_size(&self) -> usize {
        self.cycle_a.tile_size()
    }

    pub fn print(&self) {
        let first = self.first_or(-1);
        let last = self.last_or(0);

        for x in first..last + 1 {
            print!("|{x:^width$}|", x = x, width = self.tile_size() - 2);
        }
        println!();
        for y in 0..self.tile_size() {
            for x in first..last + 1 {
                self.tile_at(x).print_line(y);
            }
            println!();
        }
    }

    fn default_tile_at(&self, x: isize) -> &T {
        self.default_tile_at_gen(x, self.generation)
    }

    fn default_tile_at_gen(&self, x: isize, generation: usize) -> &T {
        let cycle = if x < 0 {
            &self.cycle_a
        } else {
            &self.cycle_b
        };
        return &cycle.default_at_gen(generation);
    }

    fn tile_at(&self, x: isize) -> &T {
        match self.tiles.get(&x) {
            Some(v) => v,
            None => self.default_tile_at(x),
        }
    }
}

#[derive(Debug)]
pub struct TileCycle<T>
where
    T: LifeTile,
{
    tiles: Vec<T>,
}

impl<T> TileCycle<T>
where
    T: LifeTile,
{
    // tile, when in an infinate grid of itself, must repeate (or will assert)
    // evaluate that repeat cycle, and store it.
    pub fn new(tile: &T) -> TileCycle<T> {
        let mut tc = TileCycle::<T> { tiles: vec![] };
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

    fn default_at_gen(&self, generation: usize) -> &T {
        &self.tiles[generation % self.tiles.len()]
    }

    fn tile_size(&self) -> usize {
        self.tiles[0].size()
    }

    pub fn len(&self) -> usize {
        self.tiles.len()
    }
}
