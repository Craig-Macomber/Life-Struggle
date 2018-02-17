#![feature(inclusive_range_syntax)]
mod tile;
mod board;
mod game;
extern crate image;
extern crate rand;
extern crate rayon;
extern crate time;
use time::PreciseTime;
use tile::*;
use rand::Rng;
use std::fs::File;
use board::Board;
use std::cmp::Ordering;

type T = VecTile;

fn main() {
    println!("Life Struggle");
    struggle_random();
}

#[derive(Debug, Clone)]
struct Player {
    tile: T,
    name: String,
    wins: usize,
    losses: usize,
    point_difference: isize,
    keep: bool,
}

impl Player {
    fn new(tile: T, name: String, keep: bool) -> Player {
        Player {
            tile: tile,
            name: name,
            wins: 0,
            losses: 0,
            point_difference: 0,
            keep: keep,
        }
    }
}

pub fn struggle_random() {
    let size = 8;
    let mut rng = rand::thread_rng();

    let player_empty = Player::new(T::new(size), "Empty".to_string(), true);
    let mut player_glider = Player::new(T::new(size), "Glider".to_string(), true);
    {
        let mut q = |x: usize, y: usize| player_glider.tile.set(x, y, true);
        q(0, 2);
        q(1, 2);
        q(2, 2);
        q(2, 1);
        q(1, 0);
    }

    let mut player_lwss = Player::new(T::new(size), "Lwss".to_string(), true);
    {
        let mut q = |x: usize, y: usize| player_lwss.tile.set(x, y, true);

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

    let mut players = Vec::<Player>::with_capacity(10);

    players.push(player_empty);
    players.push(player_glider);
    players.push(player_lwss);

    for i in 0..7 {
        let mut t = T::new(size);
        for y in 0..size {
            for x in 0..size {
                t.set(x, y, rng.gen());
            }
        }

        players.push(Player::new(t, format!("Random {}", i), false));
    }

    let c_players = players.len();
    let generations = 1000;
    for evolve_gen in 0..10 {
        for i in 0..(c_players - 1) {
            for i2 in (i + 1)..c_players {
                let (part_1, part_2) = players.split_at_mut(i2);
                let pa = &mut part_1[i];
                let pb = &mut part_2[0];
                let board = {
                    let a = &pa.tile;
                    let b = &pb.tile;
                    let b_mirror = b.mirror_over_x();
                    let b = if rng.gen() { &b_mirror } else { b };
                    game::struggle_board(generations, a, b)
                };

                match board {
                    Some(x) => {
                        let (a_s, b_s) = x.score();
                        let ref mut fout = File::create(format!(
                            "./images/{:03}-{:02}: {} to {}.png",
                            pa.name, pb.name, a_s, b_s
                        )).unwrap();
                        x.print_image(fout);
                        if a_s > b_s {
                            pa.wins += 1;
                            pb.losses += 1;
                        } else if b_s > a_s {
                            pb.wins += 1;
                            pa.losses += 1;
                        }
                        pa.point_difference += a_s - b_s;
                        pb.point_difference -= a_s - b_s;
                    }
                    None => {
                        println!("convergance draw: {:03}-{:02}", pa.name, pb.name);
                    }
                }
            }
        }
        println!("Results:");

        players.sort_by(|a, b| {
            let ord = b.wins.cmp(&a.wins);
            if ord == Ordering::Equal {
                return b.point_difference.cmp(&a.point_difference);
            } else {
                return ord;
            }
        });

        for ref p in &players {
            println!(
                "{}: {}  {}  {}",
                p.name, p.wins, p.losses, p.point_difference
            );
            p.tile.print();
        }

        for i in 0..c_players {
            if players.len() <= 6 {
                break;
            }
            let index = c_players - 1 - i;
            if players[index].keep == false {
                println!("Removing Player {}", &players[index].name);
                players.swap_remove(index);
            }
        }

        let mut i = 0;
        while players.len() < c_players {
            let mut t = T::new(size);
            for y in 0..size {
                for x in 0..size {
                    t.set(x, y, rng.gen());
                }
            }

            players.push(Player::new(
                t,
                format!("Random({}:{})", evolve_gen, i),
                false,
            ));
            i += 1;
        }

        for p in &mut players {
            p.wins = 0;
            p.losses = 0;
            p.point_difference = 0;
        }
    }
}
