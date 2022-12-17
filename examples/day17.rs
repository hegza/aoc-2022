use itertools::Itertools;
use std::ops;

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
    fn top(&self) -> usize {
        self.layout.iter().map(|pos| pos.1).max().unwrap() + self.pos.1 + 1
    }
    /// Returns the x of the square right of this block
    fn right(&self) -> usize {
        self.layout.iter().map(|pos| pos.0).max().unwrap() + self.pos.0 + 1
    }
    fn draw(&self, draw_map: &mut Vec<Vec<bool>>) {
        for (x, y) in self.layout.iter().map(|pos| (self.pos + *pos).into()) {
            draw_map[y][x] = true;
        }
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
    fn tick(&mut self, push_right: bool, map: &[Block]) -> bool {
        let Pos(x, y) = self.pos;

        // 1. Push (right or left)
        let nx = if push_right && self.right() < WIDTH {
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
                .any(|b| b.collide(&self.with_pos((nx, y).into())))
            {
                self.pos.0 = nx;
            }
        }

        let Pos(x, y) = self.pos;

        // 2. Fall
        if y == 0 {
            // If the block is already on the floor, it came to rest
            return true;
        }

        let ny = y - 1;
        // If there is no collision, update y
        if !map
            .iter()
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

fn main() -> anyhow::Result<()> {
    let mut push_dirs = INPUT
        .chars()
        .filter(|c| ['>', '<'].contains(c))
        .map(|push| push == '>')
        .cycle();
    let mut block_order = [
        BlockKind::Dash,
        BlockKind::Plus,
        BlockKind::J,
        BlockKind::I,
        BlockKind::Square,
    ]
    .iter()
    .cycle();

    let mut blocks = vec![];

    let mut top = 0;
    for round in 0..2022 {
        let mut block = Block::new((2, top + 3).into(), *block_order.next().unwrap());
        loop {
            let push_dir = push_dirs.next().unwrap();
            if block.tick(push_dir, &blocks) {
                break;
            }
        }
        blocks.push(block);
        top = blocks.iter().map(|b: &Block| b.top()).max().unwrap_or(0);

        /*
        if round + 1 <= 3 {
            println!("Block #{}", round + 1);
            println!();
            draw_blocks(&blocks);
            println!("Top: {}", top);
            println!();
        }
        */
    }

    println!("Part 1: {}", top);
    //println!("Part 2: {}", find_zero_duplicate_window(14));
    Ok(())
}
