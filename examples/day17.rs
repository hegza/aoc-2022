use itertools::Itertools;
use std::{
    ops,
    time::{Duration, SystemTime},
};

const INPUT: &str = include_str!("inputs/day17.txt");

const WIDTH: usize = 7;

#[derive(Clone, Copy)]
enum BlockKind {
    Dash,
    Plus,
    J,
    I,
    Square,
}

#[derive(Clone, Copy, PartialEq)]
struct Pos(usize, usize);

impl From<(usize, usize)> for Pos {
    fn from(xy: (usize, usize)) -> Self {
        Self(xy.0, xy.1)
    }
}

impl From<Pos> for (usize, usize) {
    fn from(pos: Pos) -> Self {
        (pos.0, pos.1)
    }
}

#[derive(Clone)]
struct Block {
    pos: Pos,
    // Layout is indexed from bot-left to top-right
    layout: Vec<Pos>,
}

impl ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Block {
    fn new(pos: Pos, kind: BlockKind) -> Self {
        let layout = match kind {
            BlockKind::Dash => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            BlockKind::Plus => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            BlockKind::J => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            BlockKind::I => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            BlockKind::Square => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        }
        .into_iter()
        .map(|xy| xy.into())
        .collect_vec();
        Self { pos, layout }
    }
    fn with_pos(&self, pos: Pos) -> Self {
        Block {
            pos,
            ..self.clone()
        }
    }
    /// Returns the y of the square above this block
    fn above(&self) -> usize {
        self.layout.iter().map(|pos| pos.1).max().unwrap() + self.pos.1 + 1
    }
    fn top(&self) -> usize {
        self.layout.iter().map(|pos| pos.1).max().unwrap() + self.pos.1
    }
    /// Returns the x of the square right of this block
    fn to_right(&self) -> usize {
        self.layout.iter().map(|pos| pos.0).max().unwrap() + self.pos.0 + 1
    }
    fn draw(&self, draw_map: &mut Vec<Vec<bool>>) {
        for (x, y) in self.layout.iter().map(|pos| (self.pos + *pos).into()) {
            draw_map[y][x] = true;
        }
    }

    fn collide_point(&self, pos: &Pos) -> bool {
        for spos in self.layout.iter().map(|pos| *pos + self.pos) {
            if &spos == pos {
                return true;
            }
        }
        false
    }
    /// Check for any overlap
    fn collide(&self, other: &Block) -> bool {
        for spos in self.layout.iter().map(|pos| *pos + self.pos) {
            for opos in other.layout.iter().map(|pos| *pos + other.pos) {
                if spos == opos {
                    return true;
                }
            }
        }
        false
    }

    // Returns true if the block came to rest
    fn tick(&mut self, push_right: bool, map: &[Block], floor_h: usize) -> bool {
        let Pos(x, y) = self.pos;

        // 1. Push (right or left)
        let nx = if push_right && self.to_right() < WIDTH {
            Some(x + 1)
        } else if !push_right && x != 0 {
            Some(x - 1)
        } else {
            None
        };

        if let Some(nx) = nx {
            // If there is no collision, update x
            if !map
                .iter()
                // Reverse iterate for efficiency because top-most blocks are...
                // at the top of the stack
                .rev()
                .any(|b| b.collide(&self.with_pos((nx, y).into())))
            {
                self.pos.0 = nx;
            }
        }

        let Pos(x, y) = self.pos;

        // 2. Fall
        if y == floor_h {
            // If the block is already on the floor, it came to rest
            return true;
        }

        let ny = y - 1;
        // If there is no collision, update y
        if !map
            .iter()
            // Reverse iterate for efficiency because top-most blocks are...
            // at the top of the stack
            .rev()
            .any(|b| b.collide(&self.with_pos((x, ny).into())))
        {
            self.pos.1 = ny;
            false
        }
        // There would be a collision, so block came to rest
        else {
            true
        }
    }
}

fn draw_map(draw_map: &Vec<Vec<bool>>) {
    for line in draw_map.iter().rev() {
        print!("|");
        for x in line {
            if *x {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("|");
        println!();
    }
    println!("+-------+");
}

fn draw_blocks(blocks: &[Block]) {
    let mut v = vec![vec![false; 7]; 10];
    for block in blocks.iter() {
        block.draw(&mut v);
    }
    draw_map(&v);
}

fn any_collision(point: &Pos, blocks: &[Block]) -> bool {
    blocks.iter().rev().any(|b| b.collide_point(point))
}

fn part1(
    mut push_dirs: impl Iterator<Item = bool>,
    mut block_order: impl Iterator<Item = BlockKind>,
) -> usize {
    let mut blocks = vec![];

    let mut height = 0;
    let mut floor = 0;
    const NUM_BLOCKS: usize = 2022usize;
    for _ in 0..NUM_BLOCKS {
        let mut block = Block::new((2, height + 3).into(), block_order.next().unwrap());
        while let Some(push_dir) = push_dirs.next() {
            if block.tick(push_dir, &blocks, 0) {
                break;
            }
        }
        blocks.push(block);
        height = blocks
            .iter()
            .map(|b: &Block| b.above())
            .max()
            .unwrap_or(0)
            .max(height);
        floor = blocks
            .iter()
            .map(|b| b.pos.1)
            .min()
            .unwrap_or(floor)
            .max(floor);
    }
    height
}

/// Returns the number of optimized blocks
fn optimize(blocks: &mut Vec<Block>, floor: usize) -> usize {
    let prelen = blocks.len();
    blocks.retain(|block| block.pos.1 >= floor);
    let postlen = blocks.len();

    /*
    if prelen != postlen {
        println!("Optimized {} of {}", prelen - postlen, prelen);
    }
    */

    prelen - postlen
}

fn part2(
    mut push_dirs: impl Iterator<Item = bool>,
    mut block_order: impl Iterator<Item = BlockKind>,
) -> usize {
    let mut blocks = vec![];

    let mut height = 0;
    let mut floor = 0;

    let mut pr = 0;
    let mut pt = SystemTime::now();
    const NUM_BLOCKS: usize = 1000000000000usize;
    for round in 0..NUM_BLOCKS {
        let since_last_measure = SystemTime::now().duration_since(pt).unwrap();
        if since_last_measure >= Duration::from_secs(1) {
            let rps = round - pr;
            println!(
                "Blocks per second: {} ({:.0} minutes remaining)",
                rps,
                ((NUM_BLOCKS - round) / rps) as f64 / 60.0
            );
            pr = round;
            pt = SystemTime::now();
        }

        let mut block = Block::new((2, height + 3).into(), block_order.next().unwrap());
        while let Some(push_dir) = push_dirs.next() {
            if block.tick(push_dir, &blocks, 0) {
                break;
            }
        }
        blocks.push(block);

        // Update height at end of round
        height = blocks.iter().map(|b: &Block| b.above()).max().unwrap_or(0);
        floor = blocks.iter().map(|b| b.pos.1).min().unwrap_or(floor);

        // Lift floor
        if round % 100 == 0 {
            if let Some(f) = (floor..height)
                .find(|&y| (0..WIDTH).all(|x| any_collision(&(x, y).into(), &blocks)))
            {
                floor = f;
            }
            optimize(&mut blocks, floor);
        }
        /*
        if round % (NUM_BLOCKS / 100) == 0 {
            println!("Progress: {} %", round / (NUM_BLOCKS / 100));
        }
        */
    }
    height
}

fn main() -> anyhow::Result<()> {
    let push_dirs = INPUT
        .chars()
        .filter(|c| ['>', '<'].contains(c))
        .map(|push| push == '>')
        .cycle();
    let block_order = [
        BlockKind::Dash,
        BlockKind::Plus,
        BlockKind::J,
        BlockKind::I,
        BlockKind::Square,
    ]
    .iter()
    .cycle();

    /*println!(
        "Part 1: {}",
        part1(push_dirs.clone(), block_order.clone().cloned())
    );*/
    println!(
        "Part 2: {}",
        part2(push_dirs.clone(), block_order.clone().cloned())
    );

    Ok(())
}
