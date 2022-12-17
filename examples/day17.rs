use itertools::Itertools;
use std::{
    collections::HashMap,
    ops,
    time::{Duration, SystemTime},
};

const INPUT: &str = include_str!("inputs/day17.txt");

const WIDTH: usize = 7;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum BlockKind {
    Dash,
    Plus,
    J,
    I,
    Square,
}

#[derive(Clone, Copy, Debug)]
struct Pos(usize, usize, usize);

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.2 == other.2
    }
}

impl From<(usize, usize)> for Pos {
    fn from(xy: (usize, usize)) -> Self {
        Self(xy.0, xy.1, xy.1 * WIDTH + xy.0)
    }
}

impl From<Pos> for (usize, usize) {
    fn from(pos: Pos) -> Self {
        (pos.0, pos.1)
    }
}

impl ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.0 + rhs.0;
        let y = self.1 + rhs.1;
        let hash = y * WIDTH + x;
        Pos(x, y, hash)
    }
}

#[derive(Clone)]
struct Block {
    pos: Pos,
    kind: BlockKind,
    // Layout is indexed from bot-left to top-right
    layout: [Pos; 7],
    width: usize,
    height: usize,
}

lazy_static::lazy_static! {
    static ref LAYOUTS: HashMap<BlockKind, Vec<Pos>> = [
            (BlockKind::Dash, [(0, 0), (1, 0), (2, 0), (3, 0), (0, 0), (0, 0), (0, 0)]),
            (BlockKind::Plus, [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2), (1, 0), (1, 0)]),
            (BlockKind::J, [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2), (0, 0), (0, 0)]),
            (BlockKind::I, [(0, 0), (0, 1), (0, 2), (0, 3), (0, 0), (0, 0), (0, 0)]),
            (BlockKind::Square, [(0, 0), (1, 0), (0, 1), (1, 1), (0, 0), (0, 0), (0, 0)]),
    ]
        .into_iter()
        .map(|(bk, x)| (bk, x.into_iter().map(|x| x.into()).collect_vec()))
        .collect();
    static ref LEFT_PROFILE: HashMap<BlockKind, Vec<Pos>> = [
            (BlockKind::Dash, vec![(0, 0)]),
            (BlockKind::Plus, vec![(1, 0), (0, 1)]),
            (BlockKind::J, vec![(0, 0), (2, 1), (2, 2)]),
            (BlockKind::I, vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
            (BlockKind::Square, vec![(0, 0), (0, 1)]),
    ]
        .into_iter()
        .map(|(bk, x)| (bk, x.into_iter().map(|x| x.into()).collect_vec()))
        .collect();
    static ref RIGHT_PROFILE: HashMap<BlockKind, Vec<Pos>> = [
            (BlockKind::Dash, vec![(3, 0)]),
            (BlockKind::Plus, vec![(1, 0), (2, 1)]),
            (BlockKind::J, vec![(2, 0), (2, 1), (2, 2)]),
            (BlockKind::I, vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
            (BlockKind::Square, vec![(1, 0), (1, 1)]),
    ]
        .into_iter()
        .map(|(bk, x)| (bk, x.into_iter().map(|x| x.into()).collect_vec()))
        .collect();
    static ref BOT_PROFILE: HashMap<BlockKind, Vec<Pos>> = [
            (BlockKind::Dash, vec![(0, 0), (1, 0), (2, 0), (3, 0)]),
            (BlockKind::Plus, vec![(0, 1), (1, 0), (2, 1)]),
            (BlockKind::J, vec![(0, 0), (1, 0), (2, 0)]),
            (BlockKind::I, vec![(0, 0)]),
            (BlockKind::Square, vec![(0, 0), (1, 0)]),
    ]
        .into_iter()
        .map(|(bk, x)| (bk, x.into_iter().map(|x| x.into()).collect_vec()))
        .collect();
}

impl Block {
    fn new(pos: Pos, kind: BlockKind) -> Self {
        let layout = match kind {
            BlockKind::Dash => [(0, 0), (1, 0), (2, 0), (3, 0), (0, 0), (0, 0), (0, 0)],
            BlockKind::Plus => [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2), (1, 0), (1, 0)],
            BlockKind::J => [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2), (0, 0), (0, 0)],
            BlockKind::I => [(0, 0), (0, 1), (0, 2), (0, 3), (0, 0), (0, 0), (0, 0)],
            BlockKind::Square => [(0, 0), (1, 0), (0, 1), (1, 1), (0, 0), (0, 0), (0, 0)],
        }
        .map(|xy| xy.into());

        let height = match kind {
            BlockKind::Dash => 1,
            BlockKind::Plus => 3,
            BlockKind::J => 3,
            BlockKind::I => 4,
            BlockKind::Square => 2,
        };
        let width = match kind {
            BlockKind::Dash => 4,
            BlockKind::Plus => 3,
            BlockKind::J => 3,
            BlockKind::I => 1,
            BlockKind::Square => 2,
        };

        Self {
            pos,
            layout,
            width,
            height,
            kind,
        }
    }
    fn with_pos(&self, pos: Pos) -> Self {
        Block {
            pos,
            ..self.clone()
        }
    }
    /// Returns the y of the square above this block
    fn above(&self) -> usize {
        self.pos.1 + self.height
    }
    /// Returns the x of the square right of this block
    fn to_right(&self) -> usize {
        self.pos.0 + self.width
    }
    /// Returns the topmost y of this block
    fn top(&self) -> usize {
        self.pos.1 + self.height - 1
    }
    /// Returns the bottom-most y of this block
    fn bottom(&self) -> usize {
        self.pos.1
    }
    /// Returns the rightmost x of this block
    fn right(&self) -> usize {
        self.pos.0 + self.width - 1
    }
    fn draw(&self, draw_map: &mut Vec<Vec<bool>>) {
        for (x, y) in self.layout.iter().map(|pos| (self.pos + *pos).into()) {
            draw_map[y][x] = true;
        }
    }

    fn as_coords(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.layout
            .iter()
            .map(|pos| (pos.0 + self.pos.0, pos.1 + self.pos.1))
    }
    fn as_coords_at_pos(
        &self,
        at_pos: (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.layout
            .iter()
            .map(move |pos| (pos.0 + at_pos.0, pos.1 + at_pos.1))
    }
    fn as_coords_bot(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        BOT_PROFILE[&self.kind]
            .iter()
            .map(|pos| (pos.0 + self.pos.0, pos.1 + self.pos.1))
    }
    fn as_coords_left(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        LEFT_PROFILE[&self.kind]
            .iter()
            .map(|pos| (pos.0 + self.pos.0, pos.1 + self.pos.1))
    }
    fn as_coords_right(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        RIGHT_PROFILE[&self.kind]
            .iter()
            .map(|pos| (pos.0 + self.pos.0, pos.1 + self.pos.1))
    }
    fn collide_point(&self, pos: &Pos) -> bool {
        // Short circuit if y-distance is too much
        if pos.1 > self.top() || pos.0 > self.right() {
            return false;
        }
        for spos in self.layout.iter().map(|pos| *pos + self.pos) {
            if &spos == pos {
                return true;
            }
        }
        false
    }

    /// Check for any overlap
    fn collide_block(&self, newer: &Block) -> bool {
        // Short circuit if y-distance is too much
        if newer.pos.1 > self.top() || newer.pos.0 > self.right() {
            return false;
        }
        for spos in self.layout.iter().map(|pos| *pos + self.pos) {
            for opos in newer.layout.iter().map(|pos| *pos + newer.pos) {
                if spos == opos {
                    return true;
                }
            }
        }
        false
    }

    // Returns true if the block came to rest
    fn tick_with_full_collision(
        &mut self,
        push_right: bool,
        map: &[Block],
        floor_h: usize,
    ) -> bool {
        let Pos(x, y, _) = self.pos;

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
                .any(|b| b.collide_block(&self.with_pos((nx, y).into())))
            {
                self.pos.0 = nx;
            }
        }

        let Pos(x, y, _) = self.pos;

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
            .any(|b| b.collide_block(&self.with_pos((x, ny).into())))
        {
            self.pos.1 = ny;
            false
        }
        // There would be a collision, so block came to rest
        else {
            true
        }
    }

    // Returns true if the block came to rest
    fn tick_optimized(&mut self, push_right: bool, map: &[[bool; 7]], skip_coll: bool) -> bool {
        let Pos(x, y, _) = self.pos;

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
            if skip_coll || !self.as_coords_at_pos((nx, y)).any(|co| map[co.1][co.0]) {
                self.pos.0 = nx;
            }
        }

        let Pos(x, y, _) = self.pos;

        // 2. Fall
        if y == 0 {
            // If the block is already on the floor, it came to rest
            return true;
        }

        let ny = y - 1;

        // If there is no collision, update y
        if !skip_coll || !self.as_coords_at_pos((x, ny)).any(|co| map[co.1][co.0]) {
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
            if block.tick_with_full_collision(push_dir, &blocks, 0) {
                break;
            }
        }
        // Update height at end of round
        if block.above() > height {
            height = block.above();
        }
        blocks.push(block);
        floor = blocks
            .iter()
            .map(|b| b.pos.1)
            .min()
            .unwrap_or(floor)
            .max(floor);
    }
    height
}

fn freeze(block_map: &mut Vec<[bool; 7]>, block: &Block) {
    for co in block.as_coords() {
        block_map[co.1][co.0] = true;
    }
}

fn part2(
    mut push_dirs: impl Iterator<Item = bool>,
    mut block_order: impl Iterator<Item = BlockKind>,
) -> usize {
    let mut block_map = vec![[false; WIDTH]; 1024];

    let mut height = 0;
    let mut floor = 0;
    let mut score = 0;

    let mut pr = 0;
    let mut pt = SystemTime::now();
    const NUM_BLOCKS: usize = 1_000_000_000_000usize;
    for round in 0..NUM_BLOCKS {
        let since_last_measure = SystemTime::now().duration_since(pt).unwrap();
        if since_last_measure >= Duration::from_secs(1) {
            let rps = round - pr;
            println!(
                "Blocks per second: {} ({} seconds remaining)",
                rps,
                ((NUM_BLOCKS - round) / rps)
            );
            pr = round;
            pt = SystemTime::now();
        }

        let mut block = Block::new((2, height + 3).into(), block_order.next().unwrap());

        while let Some(push_dir) = push_dirs.next() {
            if block.tick_optimized(push_dir, &block_map, block.pos.1 > height) {
                break;
            }
        }
        // Update height at end of round
        if block.above() > height {
            height = block.above();
        }
        freeze(&mut block_map, &block);

        // Extend the block map if necessary
        if block.above() + 10 > block_map.len() {
            // Find a new floor
            for y in (0..block.pos.1).rev() {
                if block_map[y].iter().all(|b| *b) {
                    floor = y;
                    height -= y;
                    score += y;
                    break;
                }
            }
            let nmap = block_map[floor..].iter().chain(
                std::iter::once(&[false; WIDTH])
                    .cycle()
                    .take(block_map.len() - floor),
            );
            block_map = nmap.cloned().collect();
        }
    }
    score + height
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
