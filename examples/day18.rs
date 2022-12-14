use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("inputs/day18.txt");

const XLEN: usize = 22;
const YLEN: usize = 22;
const ZLEN: usize = 21;
const XMAX: usize = XLEN - 1;
const YMAX: usize = YLEN - 1;
const ZMAX: usize = ZLEN - 1;

fn part1(points: Vec<(usize, usize, usize)>, space: &Vec<Vec<Vec<bool>>>) -> usize {
    let mut sum = 0;
    for (x, y, z) in points {
        let mut score = 0;

        // Edges of space always have edge-facing side open

        if x == 0 || !space[z][y][x - 1] {
            score += 1;
        }
        if x == XMAX || !space[z][y][x + 1] {
            score += 1;
        }
        if y == 0 || !space[z][y - 1][x] {
            score += 1;
        }
        if y == YMAX || !space[z][y + 1][x] {
            score += 1;
        }
        if z == 0 || !space[z - 1][y][x] {
            score += 1;
        }
        if z == ZMAX || !space[z + 1][y][x] {
            score += 1;
        }
        sum += score;
    }
    sum
}

fn dfs_exterior_points(
    x: usize,
    y: usize,
    z: usize,
    volume: &Vec<Vec<Vec<bool>>>,
    visited: &mut HashSet<(usize, usize, usize)>,
) -> Vec<(usize, usize, usize)> {
    visited.insert((x, y, z));

    let mut v = vec![];
    if !volume[z][y][x] {
        v.push((x, y, z));
    }

    // Check for empty space in each direction

    // Add available directions
    let mut dirs = Vec::with_capacity(6);
    if x < XMAX {
        dirs.push((x + 1, y, z));
    }
    if y < YMAX {
        dirs.push((x, y + 1, z));
    }
    if z < ZMAX {
        dirs.push((x, y, z + 1));
    }
    if x != 0 {
        dirs.push((x - 1, y, z));
    }
    if y != 0 {
        dirs.push((x, y - 1, z));
    }
    if z != 0 {
        dirs.push((x, y, z - 1))
    }

    // DFS through available directions
    for (x, y, z) in dirs {
        if !volume[z][y][x] && !visited.contains(&(x, y, z)) {
            v.extend(dfs_exterior_points(x, y, z, volume, visited));
        }
    }

    v
}

fn main() -> anyhow::Result<()> {
    let mut points = vec![];
    let mut volume = vec![vec![vec![false; XLEN]; YLEN]; ZLEN];

    for line in INPUT.lines() {
        let mut toks = line.split(',');
        let x = toks.next().unwrap().parse::<usize>()?;
        let y = toks.next().unwrap().parse::<usize>()?;
        let z = toks.next().unwrap().parse::<usize>()?;

        volume[z][y][x] = true;
        points.push((x, y, z));
    }

    let sum = part1(points.clone(), &volume);
    println!("Part 1: {sum}");

    let mut face_points = vec![];
    for z in 0..ZLEN {
        for y in 0..YLEN {
            for x in 0..XLEN {
                if z == 0 || y == 0 || x == 0 || z == ZMAX || y == YMAX || x == XMAX {
                    face_points.push((x, y, z));
                }
            }
        }
    }

    // Start a DFS from each face point to find the full exterior
    let mut visited = HashSet::new();
    let exterior_points = face_points
        .into_iter()
        .flat_map(|(x, y, z)| dfs_exterior_points(x, y, z, &volume, &mut visited))
        .collect_vec();

    let mut exterior_space = vec![vec![vec![false; XLEN]; YLEN]; ZLEN];
    for (x, y, z) in exterior_points.into_iter() {
        exterior_space[z][y][x] = true;
    }

    // Invert exterior space to get inner topology with holes filled
    for z in 0..ZLEN {
        for y in 0..YLEN {
            for x in 0..XLEN {
                exterior_space[z][y][x] = !exterior_space[z][y][x]
            }
        }
    }

    // Use part 1 solution to calculate surface area of filled topology
    let sum = part1(points, &exterior_space);
    println!("Part 2: {sum}");

    Ok(())
}
