extern crate rayon;
use std::collections::BTreeMap;
use tile::*;
use std::marker::{Sized, Sync, Send};
//use rayon::prelude::*;

pub trait Board<T>: Sized
where
    T: LifeTile,
{
    fn new(a: T, b: T) -> Option<Self> {
        assert!(a.size() == b.size());

        if a == b {
            return None;
        }

        Some(Self::new_inner(a,b))
    }

    fn new_inner(a: T, b: T) -> Self;
    fn lowest_non_a(&self) -> isize;
    fn highest_non_b(&self) -> isize;
    fn next_generation(&self) -> Option<Self>;
    fn a_current(&self) -> &T;
    fn b_current(&self) -> &T;

    fn score(&self) -> (isize, isize) {
        let first = self.lowest_non_a();
        let last = self.highest_non_b();

        // init to contigious tiles captured (negative is tiles were lost)
        let mut score_a = first;
        let mut score_b = -last - 1;

        // Go through contested area, and see if any tiles match a or b
        for x in first..=last {
            let t = self.tile_at(x);
            if t == self.a_current() {
                score_a += 1;
            } else if t == self.b_current() {
                score_b += 1;
            }
        }

        return (score_a, score_b);
    }

    fn tile_size(&self) -> usize {
        self.a_current().size()
    }

    fn print(&self) {
        let first = self.lowest_non_a() - 1;
        let last = self.highest_non_b() + 1;

        for x in first..=last {
            print!("|{x:^width$}|", x = x, width = self.tile_size() - 2);
        }
        println!();
        for y in 0..self.tile_size() {
            for x in first..=last {
                self.tile_at(x).print_line(y);
            }
            println!();
        }
    }

    fn tile_at(&self, x: isize) -> &T;
}

#[derive(Debug)]
pub struct TreeBoard<T>
where
    T: LifeTile,
{
    tiles: BTreeMap<isize, T>,
    a: T,
    b: T,
}

impl<T> Board<T> for TreeBoard<T>
where
    T: LifeTile,
{
    fn lowest_non_a(&self) -> isize {
        self.first_or(0)
    }

    fn highest_non_b(&self) -> isize {
        self.last_or(-1)
    }

    fn next_generation(&self) -> Option<Self> {
        let first = self.lowest_non_a() - 1;
        let last = self.highest_non_b() + 1;

        let a_next = self.a.next_generation(&self.a, &self.a);
        let b_next = self.b.next_generation(&self.b, &self.b);
        if a_next == b_next {
            return None;
        }

        let mut tiles_new = BTreeMap::new();

        for x in first..=last {
            let t = self.tile_at(x);
            let tnew = t.next_generation(self.tile_at(x - 1), self.tile_at(x + 1));

            if &tnew != if x < 0 { &a_next } else { &b_next } {
                tiles_new.insert(x, tnew);
            }
        }

        return Some(TreeBoard {
            a: a_next,
            b: b_next,
            tiles: tiles_new,
        });
    }

    fn new_inner(a: T, b: T) -> Self {
        TreeBoard {
            a: a,
            b: b,
            tiles: BTreeMap::new(),
        }
    }

    fn a_current(&self) -> &T{
        &self.a
    }

    fn b_current(&self) -> &T{
        &self.b
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

impl<T> TreeBoard<T>
where
    T: LifeTile,
{
    fn first_or(&self, x: isize) -> isize {
        *self.tiles.keys().next().unwrap_or(&x)
    }

    fn last_or(&self, x: isize) -> isize {
        *self.tiles.keys().next_back().unwrap_or(&x)
    }
}

#[derive(Debug)]
pub struct VecBoard<T>
where
    T: LifeTile,
{
    tiles: Vec<T>,
    vec_start: isize,
    num_a_at_start: isize,
    a: T,
    b: T,
}

impl<T> Board<T> for VecBoard<T>
where
    T: LifeTile,
{
    fn lowest_non_a(&self) -> isize {
        self.vec_start + self.num_a_at_start
    }

    fn highest_non_b(&self) -> isize {
        self.vec_start + self.tiles.len() as isize - 1
    }

    fn next_generation(&self) -> Option<Self> {
        let first = self.lowest_non_a() - 1;
        let last = self.highest_non_b() + 1;

        let a_next = self.a.next_generation(&self.a, &self.a);
        let b_next = self.b.next_generation(&self.b, &self.b);
        if a_next == b_next {
            return None;
        }

        let mut tiles_new = Vec::with_capacity(((last-first) as usize)+1);

        for x in first..=last {
            let t = self.tile_at(x);
            let tnew = t.next_generation(self.tile_at(x - 1), self.tile_at(x + 1));
            tiles_new.push(tnew);
        }

        let mut num_a_at_start_new = 0;
        for i in 0..tiles_new.len(){
            if tiles_new[i]==a_next{
                num_a_at_start_new = i as isize;
            }
            else{
                break;
            }
        }

        while tiles_new.len() > 0 && tiles_new.last().unwrap() == &b_next{
            tiles_new.pop();
        }

        return Some(VecBoard {
            a: a_next,
            b: b_next,
            tiles: tiles_new,
            num_a_at_start: num_a_at_start_new,
            vec_start: first,
        });
    }

    fn new_inner(a: T, b: T) -> Self {
        VecBoard {
            a: a,
            b: b,
            tiles: vec!(),
            num_a_at_start: 0,
            vec_start: 0,
        }
    }

    fn a_current(&self) -> &T{
        &self.a
    }

    fn b_current(&self) -> &T{
        &self.b
    }

    fn tile_at(&self, x: isize) -> &T {
        if x < self.lowest_non_a(){
            &self.a
        } else if x > self.highest_non_b(){
            &self.b
        } else {
            &self.tiles[(x - self.vec_start) as usize]
        }
    }
}