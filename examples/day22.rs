use core::panic;

use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day22.txt");

#[derive(PartialEq, Clone, Copy, Debug)]
enum Tile {
    Off,
    Empty,
    Wall,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            ' ' => Tile::Off,
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            _ => panic!("Invalid char for tile: {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Input {
    Fwd(usize),
    Right,
    Left,
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Dir {
    fn invert(self) -> Dir {
        match self {
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Up => Dir::Down,
        }
    }
}

fn parse_inputs(s: &str) -> Vec<Input> {
    let mut out = vec![];
    let mut temp_num = String::new();

    for c in s.chars() {
        if c.is_digit(10) {
            temp_num.push(c);
        } else {
            if let Ok(n) = temp_num.parse::<usize>() {
                out.push(Input::Fwd(n));
            }
            let d = match c {
                'L' => Input::Left,
                'R' => Input::Right,
                c => panic!("{} is not L/R", c),
            };
            out.push(d);
            temp_num.clear();
        }
    }

    // Parse the final number, if there is one
    if !temp_num.is_empty() {
        if let Ok(n) = temp_num.parse::<usize>() {
            out.push(Input::Fwd(n));
        }
    }
    out
}

fn line(pos: (isize, isize), dir: Dir, len: usize) -> impl Iterator<Item = (isize, isize)> {
    (0usize..=len).map(move |d: usize| match dir {
        Dir::Right => (pos.0 + d as isize, pos.1),
        Dir::Down => (pos.0, pos.1 + d as isize),
        Dir::Left => (pos.0 - d as isize, pos.1),
        Dir::Up => (pos.0, pos.1 - d as isize),
    })
}

fn is_on_map(pos: (isize, isize), map: &Vec<Vec<Tile>>) -> bool {
    let ret = pos.1 >= 0
        && pos.0 >= 0
        && (pos.1 as usize) < map.len()
        && (pos.0 as usize) < map[pos.1 as usize].len()
        && map[pos.1 as usize][pos.0 as usize] != Tile::Off;
    ret
}

fn wrap(pos: (isize, isize), dir: Dir, map: &Vec<Vec<Tile>>) -> (usize, usize) {
    // No need to wrap if the coordinate is on the map
    if is_on_map(pos, map) {
        return (pos.0 as usize, pos.1 as usize);
    }

    // Draw an inverted line until we find the edges; skip the current pos because we know it's not
    // on the map
    let mut line_iter = line(pos, dir.invert(), usize::MAX).skip(1);
    let close_on_map = line_iter.find(|co| is_on_map(*co, map)).unwrap();
    let far_not_on_map = line_iter.find(|co| !is_on_map(*co, map)).unwrap();
    let dir_unit = (0, 0); //dir.unit();
    let far_on_map = (far_not_on_map.0 + dir_unit.0, far_not_on_map.1 + dir_unit.1);
    // The wrap point is at (far_on_map + (pos - close_on_map))
    (
        (far_on_map.0 + (pos.0 - close_on_map.0)) as usize,
        (far_on_map.1 + (pos.1 - close_on_map.1)) as usize,
    )
}

fn _print_on_wrap(co: (isize, isize), x: usize, y: usize) {
    if (co.0, co.1) != (x as isize, y as isize) {
        println!(
            "{}, {} wrapped to {}, {} (+{}, +{})",
            co.0,
            co.1,
            x,
            y,
            (x as isize - co.0),
            (y as isize - co.1)
        );
    }
}

fn get_pos(pos: (usize, usize), dir: Dir, len: usize, map: &Vec<Vec<Tile>>) -> (usize, usize) {
    let mut latest_ok = pos;

    // Inspect every point in hypothetical line
    let (x, y) = (pos.0 as isize, pos.1 as isize);
    for co in line((x, y), dir, len) {
        let (x, y) = wrap(co, dir, map);
        //_print_on_wrap(co, x, y);

        // We can never be on an off-map tile after wrap
        debug_assert!(map.len() >= y, "y out of range while going {:?}", dir);
        debug_assert!(map[y][x] != Tile::Off, "while going {:?}", dir);

        // Stop moving upon colliding with a wall
        if map[y][x] == Tile::Wall {
            break;
        }

        latest_ok = (x as usize, y as usize);
    }

    latest_ok
}

fn turn(input: Input, dir: Dir) -> Dir {
    match input {
        Input::Right => match dir {
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Up => Dir::Right,
        },
        Input::Left => match dir {
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Up => Dir::Left,
        },
        Input::Fwd(_) => panic!("cannot turn with a fwd command"),
    }
}

fn sim(
    mut pos: (usize, usize),
    mut dir: Dir,
    inputs: &[Input],
    map: &Vec<Vec<Tile>>,
) -> ((usize, usize), Dir) {
    for input in inputs {
        match input {
            Input::Fwd(len) => pos = get_pos(pos, dir, *len, map),
            t @ (Input::Left | Input::Right) => dir = turn(*t, dir),
        }
    }
    (pos, dir)
}

fn main() -> anyhow::Result<()> {
    let (map_str, input_str) = INPUT.split_once("\n\n").unwrap();
    let map: Vec<Vec<Tile>> = map_str
        .lines()
        .map(|line| line.chars().map(|c| Tile::from(c)).collect_vec())
        .collect();

    let inputs = parse_inputs(input_str.trim());

    let init_pos = (
        map[0].iter().position(|tile| tile == &Tile::Empty).unwrap(),
        0,
    );
    let init_dir = Dir::Right;

    let ((ox, oy), ofacing) = sim(init_pos, init_dir, &inputs, &map);
    let (ocol, orow) = (ox + 1, oy + 1);

    let part1 = 1000 * orow + 4 * ocol + ofacing as usize;

    println!("Part 1: {}", part1);
    Ok(())
}
