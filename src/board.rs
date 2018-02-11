extern crate rayon;
use std::collections::BTreeMap;
use tile::*;

#[derive(Debug)]
pub struct Board<T>
where
    T: LifeTile,
{
    tiles: BTreeMap<isize, T>,
    a: T,
    b: T,
}

impl<T> Board<T>
where
    T: LifeTile,
{
    pub fn new(a: T, b: T) -> Option<Self> {
        assert!(a.size() == b.size());

        if a == b {
            return None;
        }

        let tiles = BTreeMap::new();

        Some(Board {
            a: a,
            b: b,
            tiles: tiles,
        })
    }

    fn lowest_non_a(&self) -> isize {
        self.first_or(0)
    }

    fn highest_non_b(&self) -> isize {
        self.last_or(-1)
    }

    pub fn next_generation(&self) -> Option<Self> {
        let mut tiles_new = BTreeMap::new();

        let first = self.lowest_non_a() - 1;
        let last = self.highest_non_b() + 1;

        let a_next = self.a.next_generation(&self.a, &self.a);
        let b_next = self.b.next_generation(&self.b, &self.b);
        if a_next == b_next {
            return None;
        }

        for x in first..=last {
            let t = self.tile_at(x);
            let tnew = t.next_generation(self.tile_at(x - 1), self.tile_at(x + 1));

            if &tnew != if x < 0 { &a_next } else { &b_next } {
                tiles_new.insert(x, tnew);
            }
        }

        return Some(Board {
            a: a_next,
            b: b_next,
            tiles: tiles_new,
        });
    }

    pub fn score(&self) -> (isize, isize) {
        let first = self.lowest_non_a();
        let last = self.highest_non_b();

        // init to contigious tiles captured (negative is tiles were lost)
        let mut score_a = first;
        let mut score_b = -last - 1;

        // Go through contested area, and see if any tiles match a or b
        for x in first..=last {
            let t = self.tile_at(x);
            if t == &self.a {
                score_a += 1;
            } else if t == &self.b {
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
        self.a.size()
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

    fn tile_at(&self, x: isize) -> &T {
        match self.tiles.get(&x) {
            Some(v) => v,
            None => if x < 0 {
                &self.a
            } else {
                &self.b
            },
        }
    }
}