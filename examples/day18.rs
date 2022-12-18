use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("inputs/day18.txt");

const XLEN: usize = 22;
const YLEN: usize = 22;
const ZLEN: usize = 21;

fn part1(points: Vec<(usize, usize, usize)>, space: &Vec<Vec<Vec<bool>>>) -> usize {
    let mut sum = 0;
    for (x, y, z) in points {
        let mut score = 0;

        // Edges of space always have edge-facing side open

        if x == 0 || !space[z][y][x - 1] {
            score += 1;
        }
        if x == XLEN - 1 || !space[z][y][x + 1] {
            score += 1;
        }
        if y == 0 || !space[z][y - 1][x] {
            score += 1;
        }
        if y == YLEN - 1 || !space[z][y + 1][x] {
            score += 1;
        }
        if z == 0 || !space[z - 1][y][x] {
            score += 1;
        }
        if z == ZLEN - 1 || !space[z + 1][y][x] {
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

    if x < XLEN - 1 && !volume[z][y][x + 1] && !visited.contains(&(x + 1, y, z)) {
        v.extend(dfs_exterior_points(x + 1, y, z, volume, visited));
    }
    if y < YLEN - 1 && !volume[z][y + 1][x] && !visited.contains(&(x, y + 1, z)) {
        v.extend(dfs_exterior_points(x, y + 1, z, volume, visited));
    }
    if z < ZLEN - 1 && !volume[z + 1][y][x] && !visited.contains(&(x, y, z + 1)) {
        v.extend(dfs_exterior_points(x, y, z + 1, volume, visited));
    }
    if x != 0 && !volume[z][y][x - 1] && !visited.contains(&(x - 1, y, z)) {
        v.extend(dfs_exterior_points(x - 1, y, z, volume, visited));
    }
    if y != 0 && !volume[z][y - 1][x] && !visited.contains(&(x, y - 1, z)) {
        v.extend(dfs_exterior_points(x, y - 1, z, volume, visited));
    }
    if z != 0 && !volume[z - 1][y][x] && !visited.contains(&(x, y, z - 1)) {
        v.extend(dfs_exterior_points(x, y, z - 1, volume, visited));
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
                if z == 0 || y == 0 || x == 0 || z == ZLEN - 1 || y == YLEN - 1 || x == XLEN - 1 {
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
