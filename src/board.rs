use image;
use rayon;
use rayon::prelude::*;
use std::marker::Sized;
use std::path::Path;
use tile::*;

pub trait Board<T>: Sized
where
    T: LifeTile,
{
    fn new(a: T, b: T) -> Option<Self> {
        assert!(a.size() == b.size());

        if a == b {
            return None;
        }

        Some(Self::new_inner(a, b))
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

    fn print_image<Q>(&self, path: Q)
    where
        Q: AsRef<Path>,
    {
        let first = self.lowest_non_a() - 1;
        let last = self.highest_non_b() + 1;
        let range = first..=last;
        let tile_size = self.tile_size();
        let mut imgbuf =
            image::GrayImage::new((tile_size * range.count()) as u32, tile_size as u32);

        //for x in range {
        //    img.put_pixel(x * tile_size, tile_size, image::Luma([255 as u8]))
        //    img.put_pixel((x +1) * tile_size - 1, tile_size, image::Luma([255 as u8]))
        //}

        for x in first..=last {
            let t = self.tile_at(x);
            for yy in 0usize..self.tile_size() {
                for xx in 0usize..self.tile_size() {
                    let b = t.get(xx, yy);
                    let luma: u8 = if b { 0 } else { 255 };
                    imgbuf.put_pixel(
                        ((x - first) as usize * tile_size + xx) as u32,
                        yy as u32,
                        image::Luma([luma]),
                    )
                }
            }
        }

        imgbuf.save(path).unwrap();
    }

    fn tile_at(&self, x: isize) -> &T;
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

        let (a_next, b_next): (T, T) = rayon::join(
            || self.a.next_generation(&self.a, &self.a),
            || self.b.next_generation(&self.b, &self.b),
        );

        if a_next == b_next {
            return None;
        }

        let mut tiles_new: Vec<T> = (first..last + 1)
            .into_par_iter()
            .map(|x| -> T {
                self.tile_at(x)
                    .next_generation(self.tile_at(x - 1), self.tile_at(x + 1))
            }).collect();

        let mut num_a_at_start_new = 0;
        for i in 0..tiles_new.len() {
            if tiles_new[i] == a_next {
                num_a_at_start_new = i as isize + 1;
            } else {
                break;
            }
        }

        while tiles_new.len() > 0 && tiles_new.last().unwrap() == &b_next {
            tiles_new.pop();
        }

        let b_new = VecBoard {
            a: a_next,
            b: b_next,
            tiles: tiles_new,
            num_a_at_start: num_a_at_start_new,
            vec_start: first,
        };

        debug_assert!(b_new.tile_at(b_new.lowest_non_a()) != &b_new.a);
        debug_assert!(b_new.tile_at(b_new.highest_non_b()) != &b_new.b);

        return Some(b_new);
    }

    fn new_inner(a: T, b: T) -> Self {
        VecBoard {
            a: a,
            b: b,
            tiles: vec![],
            num_a_at_start: 0,
            vec_start: 0,
        }
    }

    fn a_current(&self) -> &T {
        &self.a
    }

    fn b_current(&self) -> &T {
        &self.b
    }

    fn tile_at(&self, x: isize) -> &T {
        if x < self.lowest_non_a() {
            &self.a
        } else if x > self.highest_non_b() {
            &self.b
        } else {
            &self.tiles[(x - self.vec_start) as usize]
        }
    }
}
