use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day15.txt");

fn dist(a: (isize, isize), b: (isize, isize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

#[derive(Debug)]
struct Ball {
    pos: (isize, isize),
    rad: usize,
}

impl Ball {
    fn covers_point(&self, point: (isize, isize)) -> bool {
        dist(self.pos, point) <= self.rad
    }

    /// Returns line in projected plane
    fn project_x(&self, y: isize) -> Option<(isize, isize)> {
        let ydist = self.pos.1.abs_diff(y);
        if ydist > self.rad {
            None
        } else {
            let dx = self.rad - ydist;
            Some((self.pos.0 - dx as isize, self.pos.0 + dx as isize))
        }
    }
}

fn part1(balls: &[Ball]) -> usize {
    let scan_y = 2000000;
    let projections = balls.iter().filter_map(|ball| ball.project_x(scan_y));

    let projections_flat = projections.clone().map(|p| [p.0, p.1]).flatten();
    let min_x = projections_flat.clone().min().unwrap();
    let max_x = projections_flat.max().unwrap();

    (min_x..max_x)
        .filter(|x| balls.iter().any(|ball| ball.covers_point((*x, scan_y))))
        .count()
}

fn part2(balls: &[Ball]) -> i64 {
    let mut y = 0;
    while y <= 4000000 {
        let mut x = 0;
        'x: while x <= 4000000 {
            for ball in balls {
                // If we hit a beacon, project it to current y, and skip the
                // beacon by moving to its right edge
                if ball.covers_point((x, y)) {
                    x = ball.project_x(y).unwrap().1 + 1;
                    continue 'x;
                }
            }
            return x as i64 * 4000000 + y as i64;
        }
        y += 1;
    }
    panic!("no beacon found")
}

fn main() -> anyhow::Result<()> {
    let inputs = INPUT.lines().map(|line| {
        let toks = line.split_ascii_whitespace();
        let mut parsing_beacon = false;
        let (mut sx, mut sy, mut bx, mut by) = (None, None, None, None);
        for tok in toks {
            let first = tok.chars().nth(0).unwrap();
            match first {
                c @ 'x' | c @ 'y' => {
                    let rhs: String = tok
                        .split_once('=')
                        .unwrap()
                        .1
                        .chars()
                        .filter(|c| c.is_numeric() || *c == '-')
                        .collect();
                    let num = rhs.parse::<isize>().unwrap();
                    match (c, parsing_beacon) {
                        ('x', true) => bx = Some(num),
                        ('x', false) => sx = Some(num),
                        ('y', true) => by = Some(num),
                        ('y', false) => {
                            parsing_beacon = true;
                            sy = Some(num);
                        }
                        _ => unreachable!(),
                    }
                }
                _ => {}
            }
        }
        ((sx.unwrap(), sy.unwrap()), (bx.unwrap(), by.unwrap()))
    });

    let balls = inputs
        .map(|(sens, beac)| {
            let dist = dist(sens, beac);
            Ball {
                pos: sens,
                rad: dist,
            }
        })
        .collect_vec();

    println!("Part 1: {}", part1(&balls));
    println!("Part 2: {}", part2(&balls));

    Ok(())
}
