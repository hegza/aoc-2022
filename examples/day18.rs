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

fn main() -> anyhow::Result<()> {
    let mut points = vec![];
    let mut space = vec![vec![vec![false; XLEN]; YLEN]; ZLEN];

    for line in INPUT.lines() {
        let mut toks = line.split(',');
        let x = toks.next().unwrap().parse::<usize>()?;
        let y = toks.next().unwrap().parse::<usize>()?;
        let z = toks.next().unwrap().parse::<usize>()?;

        space[z][y][x] = true;
        points.push((x, y, z));
    }

    let sum = part1(points, &space);
    println!("Part 1: {sum}");

    Ok(())
}
