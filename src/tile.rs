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

    fn mirror_over_x(&self) -> Self {
        let size = self.size();
        let mut t = Self::new(size);

        for x in 0..size {
            for y in 0..size {
                t.set(x, y, self.get(x, size - y - 1));
            }
        }
        return t;
    }

    // LifeTile is for use in a world where each row (along Y) of tiles is the same,
    // so we just need 3 tiles (instead of 9) to have a complete Moore neighborhood
    // for each cell in self.
    fn next_generation(&self, previous: &Self, next: &Self) -> Self {
        let size = self.size();

        // Write next generation into new tile
        let mut t = Self::new(size);

        // Do edges with general logic
        for y in [0, size - 1].iter() {
            for x in 0..size {
                t.set(x, *y, self.next_generation_cell(previous, next, x, *y));
            }
        }

        for x in [0, size - 1].iter() {
            for y in 1..(size - 1) {
                t.set(*x, y, self.next_generation_cell(previous, next, *x, y));
            }
        }

        // Do center woth optimized logic
        for y in 1..(size - 1) {
            for x in 1..(size - 1) {
                // Count live cells in Moore neighborhood of (x,y)
                // Hand unrolling this has been tested to be a perf win.
                let mut c1 = 0;
                let mut c2 = 0;
                let mut c3 = 0;
                let mut at = |cc: &mut usize, ix: usize, iy: usize| {
                    if self.get(ix, iy) {
                        *cc += 1;
                    };
                };
                at(&mut c1, x - 1, y - 1);
                at(&mut c2, x + 0, y - 1);
                at(&mut c3, x + 1, y - 1);
                at(&mut c1, x - 1, y + 0);
                at(&mut c2, x + 0, y + 0);
                at(&mut c3, x + 1, y + 0);
                at(&mut c1, x - 1, y + 1);
                at(&mut c2, x + 0, y + 1);
                at(&mut c3, x + 1, y + 1);

                let c = c1 + c2 + c3;

                // Apply Conway's Game of Life life and death rules
                let v = if self.get(x, y) {
                    c == 3 || c == 4
                } else {
                    c == 3
                };

                if v {
                    t.set(x, y, v);
                }
            }
        }

        return t;
    }

    // LifeTile is for use in a world where each row (along Y) of tiles is the same,
    // so we just need 3 tiles (instead of 9) to have a complete Moore neighborhood
    // for each cell in self.
    fn next_generation_cell(&self, previous: &Self, next: &Self, x: usize, y: usize) -> bool {
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
        if self.get(x, y) {
            c == 3 || c == 4
        } else {
            c == 3
        }
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
