use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day8.txt");

struct Grid(Vec<Vec<usize>>);

impl Grid {
    fn parse(input: &str) -> Grid {
        let v = input
            .lines()
            .map(|line| {
                line.chars()
                    .filter_map(|c| c.to_digit(10).map(|x| x as usize))
                    .collect_vec()
            })
            .collect::<Vec<Vec<usize>>>();
        Self(v)
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }
    fn height(&self) -> usize {
        self.0.len()
    }

    fn is_visible_from_dir(&self, x: usize, y: usize, xd: isize, yd: isize) -> bool {
        let h = self.0[y][x];
        let mut ny = y as isize + yd;
        let mut nx = x as isize + xd;

        // While within the grid
        while ny >= 0 && ny < self.height() as isize && nx >= 0 && nx < self.width() as isize {
            let h_ = self.0[ny as usize][nx as usize];
            if h_ >= h {
                return false;
            }
            nx = nx + xd;
            ny = ny + yd;
        }

        true
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        self.is_visible_from_dir(x, y, -1, 0)
            || self.is_visible_from_dir(x, y, 1, 0)
            || self.is_visible_from_dir(x, y, 0, -1)
            || self.is_visible_from_dir(x, y, 0, 1)
    }

    fn viewing_distance(&self, x: usize, y: usize, xd: isize, yd: isize) -> usize {
        let h = self.0[y][x];
        let mut ny = y as isize + yd;
        let mut nx = x as isize + xd;
        let mut dist = 0;

        // While within the grid
        while ny >= 0 && ny < self.height() as isize && nx >= 0 && nx < self.width() as isize {
            let h_ = self.0[ny as usize][nx as usize];
            // Tree is higher or same
            if h_ >= h {
                // Add this tree and break
                dist += 1;
                break;
            }
            // Tree is shorter
            else {
                // Add this tree and continue
                dist += 1;
            }
            nx = nx + xd;
            ny = ny + yd;
        }
        dist
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let w = self.viewing_distance(x, y, -1, 0);
        let e = self.viewing_distance(x, y, 1, 0);
        let n = self.viewing_distance(x, y, 0, -1);
        let s = self.viewing_distance(x, y, 0, 1);

        // Score
        w * e * n * s
    }
}

fn part1(grid: &Grid) -> usize {
    let mut vis_cnt = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid.is_visible(x, y) {
                vis_cnt += 1;
            }
        }
    }
    vis_cnt
}

fn part2(grid: &Grid) -> usize {
    let mut max_score = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let score = grid.scenic_score(x, y);
            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}

fn main() -> anyhow::Result<()> {
    let grid = Grid::parse(INPUT);

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
    Ok(())
}
