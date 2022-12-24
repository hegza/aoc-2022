use itertools::Itertools;
use std::{collections::VecDeque, iter};

const INPUT: &str = include_str!("inputs/day24.txt");

fn _print_blizzards(blizz: &Vec<Blizzard>, w: usize, h: usize) {
    for _ in 1..w + 1 {
        print!("#");
    }
    println!();
    for y in 1..h - 1 {
        print!("#");
        for x in 1..w - 1 {
            let co = (x, y);
            let bcount = blizz.iter().filter(|b| b.pos == co).count();
            if bcount > 1 {
                print!("{}", bcount);
            } else if let Some(b) = blizz.iter().find(|b| b.pos == co) {
                match b.dir {
                    Dir::Right => print!(">"),
                    Dir::Left => print!("<"),
                    Dir::Up => print!("^"),
                    Dir::Down => print!("v"),
                }
            } else {
                print!(".");
            }
        }
        println!("#");
    }
    for _ in 1..w + 1 {
        print!("#");
    }
    println!();
}

#[derive(Clone)]
struct Blizzard {
    pos: (usize, usize),
    dir: Dir,
}

#[derive(Clone, Copy)]
enum Dir {
    Right,
    Left,
    Down,
    Up,
}

fn next_blizzards(blizzards: &[Blizzard], w: usize, h: usize) -> Vec<Blizzard> {
    blizzards
        .into_iter()
        .map(|b| {
            let (x, y) = match b.dir {
                Dir::Right => {
                    if b.pos.0 != w - 2 {
                        (b.pos.0 + 1, b.pos.1)
                    } else {
                        (1, b.pos.1)
                    }
                }
                Dir::Left => {
                    if b.pos.0 != 1 {
                        (b.pos.0 - 1, b.pos.1)
                    } else {
                        (w - 2, b.pos.1)
                    }
                }
                Dir::Down => {
                    if b.pos.1 != h - 2 {
                        (b.pos.0, b.pos.1 + 1)
                    } else {
                        (b.pos.0, 1)
                    }
                }
                Dir::Up => {
                    if b.pos.1 != 1 {
                        (b.pos.0, b.pos.1 - 1)
                    } else {
                        (b.pos.0, h - 2)
                    }
                }
            };

            Blizzard { pos: (x, y), ..*b }
        })
        .collect_vec()
}

fn available<'b>(
    pos: (usize, usize),
    blizzards: &'b [Blizzard],
    w: usize,
    h: usize,
) -> impl Iterator<Item = (usize, usize)> + 'b + Clone {
    let (x, y) = pos;

    let xr = match x {
        0 => x + 1,
        1 => x,
        _ => x - 1,
    }..=if x == w - 1 {
        x - 1
    } else if x == w - 2 {
        x
    } else {
        x + 1
    };
    let yr = match y {
        0 => y + 1,
        1 => y,
        _ => y - 1,
    }..=if y == h - 1 {
        y - 1
    } else if y == h - 2 {
        y
    } else {
        y + 1
    };

    xr.cartesian_product(yr)
        .filter(move |co| !blizzards.iter().any(|b| b.pos == *co))
}

fn route_bfs(
    start: (usize, usize),
    dest: (usize, usize),
    mut blizz: Vec<Blizzard>,
    w: usize,
    h: usize,
) -> (usize, Vec<(usize, usize)>, Vec<Vec<Blizzard>>) {
    let mut q = VecDeque::new();

    blizz = next_blizzards(&blizz, w, h);
    q.extend(
        available(start, &blizz, w, h)
            .chain(iter::once(start))
            .map(|co| (1, co, vec![start], vec![blizz.clone()]))
            .collect_vec(),
    );

    let mut pdepth = 0;

    while let Some((depth, co, hist, hist_b)) = q.pop_front() {
        // Check if we are one-off from destination
        if co.0 >= dest.0 - 1 && co.0 <= dest.0 + 1 && co.1 + 1 == dest.1 {
            return (depth + 1, hist, hist_b);
        }

        // If we are starting to analyze the next depth, increment blizzards
        if depth > pdepth {
            blizz = next_blizzards(&blizz, w, h);
            pdepth = depth;
        }

        // Check available moves and append them to queue
        q.extend(
            available(co, &blizz, w, h)
                .map(|co| {
                    (
                        depth + 1,
                        co,
                        {
                            let mut h = hist.clone();
                            h.push(co);
                            h
                        },
                        {
                            let mut b = hist_b.clone();
                            b.push(blizz.clone());
                            b
                        },
                    )
                })
                .collect_vec(),
        );
    }
    (0, vec![], vec![])
}

fn main() -> anyhow::Result<()> {
    let mut blizzards = vec![];
    let walls = INPUT
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => true,
                    '.' => false,
                    b @ ('>' | 'v' | '^' | '<') => {
                        use Dir::*;
                        let dir = match b {
                            '>' => Right,
                            '<' => Left,
                            '^' => Up,
                            'v' => Down,
                            c => panic!("unrecognized char in input: {}", c),
                        };
                        blizzards.push(Blizzard { pos: (x, y), dir });
                        false
                    }
                    c => panic!("unrecognized char in input: {}", c),
                })
                .collect_vec()
        })
        .collect_vec();

    let (w, h) = (walls[0].len(), walls.len());
    let expedition = (walls[0].iter().position(|&wall| !wall).unwrap(), 0usize);
    let dest = (
        walls[walls.len() - 1]
            .iter()
            .position(|&wall| !wall)
            .unwrap(),
        walls.len() - 1,
    );
    let route_len = route_bfs(expedition, dest, blizzards, w, h);
    for (min, (co, bliz)) in route_len.1.iter().zip(route_len.2.iter()).enumerate() {
        println!("Minute {}, {:?}", min, co);
        println!("Next bliz:");
        _print_blizzards(bliz, w, h);
        println!();
    }
    println!("Part 1: {}", route_len.0);

    Ok(())
}
