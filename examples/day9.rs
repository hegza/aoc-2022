use sliding_windows::{IterExt, Storage};
use std::collections::HashSet;

const INPUT: &str = include_str!("inputs/day9.txt");

fn is_touching(hx: i32, hy: i32, tx: i32, ty: i32) -> bool {
    [(0, 0), (1, 0), (0, 1), (1, 1)].contains(&((hx - tx).abs(), (hy - ty).abs()))
}

fn mov_head(hx: &mut i32, hy: &mut i32, dir: (i32, i32)) {
    // Move head
    *hx += dir.0;
    *hy += dir.1;
}

/// Returns new tail position
fn update_tail(hx: i32, hy: i32, tx: &mut i32, ty: &mut i32) -> (i32, i32) {
    if !is_touching(hx, hy, *tx, *ty) {
        *tx += (hx - *tx).signum();
        *ty += (hy - *ty).signum();
    }
    (*tx, *ty)
}

/// Returns new tail position (final tail)
fn update_tail_many(rope: &mut [(i32, i32)]) -> (i32, i32) {
    for idx in 0..rope.len() - 1 {
        let (hx, hy) = rope[idx];
        let (tx, ty) = &mut rope[idx + 1];
        update_tail(hx, hy, tx, ty);
    }

    rope[rope.len() - 1]
}

fn part1(cmds: &[(char, usize)]) -> usize {
    let (mut hx, mut hy) = (0, 0);
    let (mut tx, mut ty) = (0, 0);

    let mut tail_visited = HashSet::new();

    // Insert initial position as visited
    tail_visited.insert((tx, ty));

    for (cmd, dist) in cmds {
        match cmd {
            'R' => {
                for _ in 0..*dist {
                    mov_head(&mut hx, &mut hy, (1, 0));
                    let ntail = update_tail(hx, hy, &mut tx, &mut ty);
                    tail_visited.insert(ntail);
                }
            }
            'L' => {
                for _ in 0..*dist {
                    mov_head(&mut hx, &mut hy, (-1, 0));
                    let ntail = update_tail(hx, hy, &mut tx, &mut ty);
                    tail_visited.insert(ntail);
                }
            }
            'D' => {
                for _ in 0..*dist {
                    mov_head(&mut hx, &mut hy, (0, 1));
                    let ntail = update_tail(hx, hy, &mut tx, &mut ty);
                    tail_visited.insert(ntail);
                }
            }
            'U' => {
                for _ in 0..*dist {
                    mov_head(&mut hx, &mut hy, (0, -1));
                    let ntail = update_tail(hx, hy, &mut tx, &mut ty);
                    tail_visited.insert(ntail);
                }
            }
            d => panic!("unknown direction: {}", d),
        }
    }

    tail_visited.len()
}

fn part2(cmds: &[(char, usize)]) -> usize {
    let mut rope = vec![(0, 0); 10];
    let mut tail_visited = HashSet::new();

    // Insert initial position as visited
    tail_visited.insert(rope[9]);

    for (cmd, dist) in cmds {
        match cmd {
            'R' => {
                for _ in 0..*dist {
                    let (hx, hy) = &mut rope[0];
                    mov_head(hx, hy, (1, 0));
                    update_tail_many(&mut rope);
                    tail_visited.insert(rope[rope.len() - 1]);
                }
            }
            'L' => {
                for _ in 0..*dist {
                    let (hx, hy) = &mut rope[0];
                    mov_head(hx, hy, (-1, 0));
                    update_tail_many(&mut rope);
                    tail_visited.insert(rope[rope.len() - 1]);
                }
            }
            'D' => {
                for _ in 0..*dist {
                    let (hx, hy) = &mut rope[0];
                    mov_head(hx, hy, (0, 1));
                    update_tail_many(&mut rope);
                    tail_visited.insert(rope[rope.len() - 1]);
                }
            }
            'U' => {
                for _ in 0..*dist {
                    let (hx, hy) = &mut rope[0];
                    mov_head(hx, hy, (0, -1));
                    update_tail_many(&mut rope);
                    tail_visited.insert(rope[rope.len() - 1]);
                }
            }
            d => panic!("unknown direction: {}", d),
        }
    }
    tail_visited.len()
}

fn main() -> anyhow::Result<()> {
    let cmds: Vec<(char, usize)> = INPUT
        .lines()
        .map(|line| {
            let mut toks = line.split_ascii_whitespace();
            let c = toks.next().unwrap().chars().nth(0).unwrap();
            let dist = toks.next().unwrap().parse::<usize>().unwrap();
            (c, dist)
        })
        .collect();

    println!("Part 1: {}", part1(&cmds));
    println!("Part 2: {}", part2(&cmds));
    Ok(())
}
