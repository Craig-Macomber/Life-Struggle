extern crate fixedbitset;
extern crate num_integer;
use self::num_integer::Integer;
use self::fixedbitset::FixedBitSet;
use std::marker::{Send, Sized, Sync};

pub trait LifeTileSrc {
    fn size(&self) -> usize;
    fn get(&self, x: usize, y: usize) -> bool;
}

pub trait LifeTile: LifeTileSrc
where
    Self: Sized + Clone + Eq + Send + Sync,
{
    fn new(size: usize) -> Self;
    fn set(&mut self, x: usize, y: usize, value: bool);

    fn copy_from(t_in: &LifeTileSrc) -> Self {
        let size = t_in.size();
        let mut t = Self::new(size);

        for x in 0..size {
            for y in 0..size {
                t.set(x, y, t_in.get(x, y));
            }
        }
        return t;
    }

    // Mirror over y == size/2
    fn mirror(&self) -> Self {
        let size = self.size();
        let mut t = Self::new(size);

        for x in 0..size {
            for y in 0..size {
                t.set(x, y, self.get(size - x - 1, y));
            }
        }
        return t;
    }

    // LifeTile is for use in a world where each row (along Y) of tiles is the same,
    // so we just need 3 tiles (instead of 9) to have a complete Moore neighborhood
    // for each cell in self.
    fn next_generation(&self, previous: &Self, next: &Self) -> Self {
        let size = self.size();

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

        // Write next generation into new tile
        let mut t = Self::new(size);

        for y in 0..size {
            for x in 0..size {
                // Count live cells in Moore neighborhood of (x,y)
                let mut c = 0;
                for yy in -1isize..=1 {
                    for xx in -1isize..=1 {
                        if at(xx + x as isize, yy + y as isize) {
                            c += 1;
                        }
                    }
                }

                // Apply Conway's Game of Life life and death rules
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

    fn print_line(&self, y: usize) {
        for x in 0..self.size() {
            let s = if self.get(x, y) { "X" } else { "." };
            print!("{}", s);
        }
    }

    fn print(&self) {
        for y in 0..self.size() {
            self.print_line(y);
            println!();
        }
    }
}

#[derive(Debug, Clone, Eq)]
pub struct VecTile {
    pub size: usize,
    cells: Vec<bool>,
}

impl LifeTileSrc for VecTile {
    fn size(&self) -> usize {
        self.size
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.cells[self.index(x, y)]
    }
}

impl LifeTile for VecTile {
    fn new(size: usize) -> VecTile {
        VecTile {
            size: size,
            cells: vec![false; size * size],
        }
    }

    fn set(&mut self, x: usize, y: usize, value: bool) {
        let index = self.index(x, y);
        self.cells[index] = value;
    }
}

impl VecTile {
    fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.size
    }
}

impl PartialEq for VecTile {
    fn eq(&self, other: &VecTile) -> bool {
        self.cells == other.cells
    }
}

#[derive(Debug, Clone, Eq)]
pub struct BitTile {
    pub size: usize,
    cells: FixedBitSet,
}

impl LifeTileSrc for BitTile {
    fn size(&self) -> usize {
        self.size
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.cells.contains(self.index(x, y))
    }
}

impl LifeTile for BitTile {
    fn new(size: usize) -> BitTile {
        BitTile {
            size: size,
            cells: FixedBitSet::with_capacity(size * size),
        }
    }

    fn set(&mut self, x: usize, y: usize, value: bool) {
        let index = self.index(x, y);
        self.cells.set(index, value);
    }
}

impl BitTile {
    fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.size
    }
}

impl PartialEq for BitTile {
    fn eq(&self, other: &BitTile) -> bool {
        self.cells == other.cells
    }
}
