use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day23.txt");

/// Diffusion grid
struct Grid {
    cx: isize,
    cy: isize,
    se: Vec<Vec<bool>>,
    sw: Vec<Vec<bool>>,
    nw: Vec<Vec<bool>>,
    ne: Vec<Vec<bool>>,
    min_x: isize,
    min_y: isize,
    max_x: isize,
    max_y: isize,
}

impl Grid {
    fn from_initial(initial: Vec<Vec<bool>>) -> Grid {
        let (cx, cy) = (initial[0].len() / 2, initial.len() / 2);

        let se = initial[cy..]
            .iter()
            .map(|line| line[cx..].to_vec())
            .collect_vec();
        let sw = initial[cy..]
            .iter()
            .map(|line| line[..cx].iter().rev().cloned().collect_vec())
            .collect_vec();
        let nw = initial[..cy]
            .iter()
            .rev()
            .map(|line| line[..cx].iter().rev().cloned().collect_vec())
            .collect_vec();
        let ne = initial[..cy]
            .iter()
            .rev()
            .map(|line| line[cx..].to_vec())
            .collect_vec();

        Grid {
            se,
            sw,
            nw,
            ne,
            cx: cx as isize,
            cy: cy as isize,
            min_x: 0,
            min_y: 0,
            max_x: initial[0].len() as isize - 1,
            max_y: initial.len() as isize - 1,
        }
    }

    fn get_mut(&mut self, x: isize, y: isize) -> &mut bool {
        if x > self.max_x {
            self.max_x = x;
        } else if x < self.min_x {
            self.min_x = x;
        }
        if y > self.max_y {
            self.max_y = y;
        } else if y < self.min_y {
            self.min_y = y;
        }
        let (container, x, y) = match (x >= self.cx, y >= self.cy) {
            // SE
            (true, true) => {
                let x = (x - self.cx) as usize;
                let y = (y - self.cy) as usize;

                (&mut self.se, x, y)
            }
            // NE
            (true, false) => {
                let x = (x - self.cx) as usize;
                let y = (-y + self.cy - 1) as usize;
                (&mut self.ne, x, y)
            }
            // SW
            (false, true) => {
                let x = (-x + self.cx - 1) as usize;
                let y = (y - self.cy) as usize;
                (&mut self.sw, x, y)
            }
            // NW
            (false, false) => {
                let x = (-x + self.cx - 1) as usize;
                let y = (-y + self.cy - 1) as usize;
                (&mut self.nw, x, y)
            }
        };
        if y >= container.len() {
            // Double y-length
            let v = vec![vec![false; container[0].len()]; container.len()];
            container.extend(v.into_iter())
        }
        if x >= container[y].len() {
            // Double x-length
            let v = vec![false; container[y].len()];
            container[y].extend(v.into_iter());
        }

        &mut container[y][x]
    }
    fn get(&mut self, x: isize, y: isize) -> bool {
        *self.get_mut(x, y)
    }
    fn clear(&mut self, x: isize, y: isize) {
        // TODO: update minx, miny
        *self.get_mut(x, y) = false;
    }
    fn set(&mut self, x: isize, y: isize) {
        // TODO: update minx, miny
        *self.get_mut(x, y) = true;
    }
}

#[test]
fn grid_works() {
    // 1 0 0
    // 0 1 1
    let mut grid = Grid::from_initial(vec![vec![true, false, false], vec![false, true, true]]);

    assert!(grid.get_mut(0, 0));
    assert!(grid.get_mut(1, 1));
    assert!(grid.get_mut(2, 1));
    assert!(!grid.get_mut(2, 0));
    assert!(!grid.get_mut(1, 0));
    assert!(!grid.get_mut(0, 1));

    assert_eq!(grid.min_x, 0);
    assert_eq!(grid.min_y, 0);
    assert_eq!(grid.max_x, 2);
    assert_eq!(grid.max_y, 1);

    // Check outside the grid as well
    assert!(!grid.get_mut(-1, -1));
    assert!(!grid.get_mut(2, 2));

    assert_eq!(grid.min_x, -1);
    assert_eq!(grid.min_y, -1);
    assert_eq!(grid.max_x, 2);
    assert_eq!(grid.max_y, 2);
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Dir {
    N,
    S,
    W,
    E,
    Ne,
    Nw,
    Se,
    Sw,
}

fn add_dir(x: isize, y: isize, dir: Dir) -> (isize, isize) {
    match dir {
        Dir::Nw => (x - 1, y - 1),
        Dir::N => (x, y - 1),
        Dir::Ne => (x + 1, y - 1),
        Dir::W => (x - 1, y),
        Dir::E => (x + 1, y),
        Dir::Sw => (x - 1, y + 1),
        Dir::S => (x, y + 1),
        Dir::Se => (x + 1, y + 1),
    }
}

fn neighborhood_with_dir(x: isize, y: isize) -> impl Iterator<Item = (Dir, isize, isize)> {
    [
        (Dir::Nw, x - 1, y - 1),
        (Dir::N, x, y - 1),
        (Dir::Ne, x + 1, y - 1),
        (Dir::W, x - 1, y),
        (Dir::E, x + 1, y),
        (Dir::Sw, x - 1, y + 1),
        (Dir::S, x, y + 1),
        (Dir::Se, x + 1, y + 1),
    ]
    .into_iter()
}

/// Returns whether there is something in the eight positions around the point
fn scan(x: isize, y: isize, grid: &mut Grid) -> HashMap<Dir, bool> {
    neighborhood_with_dir(x, y)
        .map(|(d, x, y)| (d, grid.get(x, y)))
        .collect()
}

fn propose(
    dir_rotation: &[[Dir; 3]; 4],
    dir_rotation_offset: usize,
    scan: &HashMap<Dir, bool>,
) -> Dir {
    let dirs = dir_rotation
        .iter()
        .cycle()
        .skip(dir_rotation_offset)
        .take(4);

    for dirs in dirs {
        if dirs.iter().all(|d| !scan[d]) {
            return dirs[0];
        }
    }
    panic!("algo failure")
}

fn remove_when_more_than_one<T>(grid: &mut Vec<Vec<Option<T>>>)
where
    T: Eq + std::hash::Hash + Clone,
{
    let mut rem_set = HashSet::new();
    let mut once_set = HashMap::new();
    let mut post_remove = vec![];

    grid.iter_mut().enumerate().for_each(|(y, line)| {
        line.iter_mut().enumerate().for_each(|(x, e)| {
            if let Some(inner) = e {
                // If this is marked as remove -> remove
                if rem_set.contains(inner) {
                    post_remove.push((x, y));
                }

                // If we already have one of this element, mark it as to be removed
                if once_set.contains_key(inner) {
                    rem_set.insert(inner.clone());
                    // ... and set the original instance as to be removed as well
                    post_remove.push(once_set[inner]);
                }
                once_set.insert(inner.clone(), (x, y));
            }
        })
    });

    for (x, y) in post_remove {
        grid[y][x] = None;
    }
}

fn main() -> anyhow::Result<()> {
    let initial_grid = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!(),
                })
                .collect_vec()
        })
        .collect_vec();
    let mut grid = Grid::from_initial(initial_grid);

    use Dir::*;
    // The cardinal direction must be first in each sublist for the algorithm to work correctly
    let dir_rotation = &[[N, Ne, Nw], [S, Se, Sw], [W, Nw, Sw], [E, Ne, Se]];
    let mut dir_rotation_offset = 0;

    for _round in 0..10 {
        let mut proposals = (grid.min_y..=grid.max_y)
            .map(|y| {
                (grid.min_x..=grid.max_x)
                    .filter_map(|x| {
                        grid.get_mut(x, y).then_some({
                            let nb = scan(x, y, &mut grid);
                            nb.iter().any(|(_, occupied)| *occupied).then_some({
                                let propose_dir = propose(&dir_rotation, dir_rotation_offset, &nb);
                                let propose_pos = add_dir(x, y, propose_dir);
                                propose_pos
                            })
                        })
                    })
                    .collect_vec()
            })
            .collect_vec();
        remove_when_more_than_one(&mut proposals);

        // Second half
        for y in grid.min_y..=grid.max_y {
            for x in grid.min_x..=grid.max_x {
                if let Some(dest) = proposals[(y + grid.min_y) as usize][(x + grid.min_x) as usize]
                {
                    // Null the source
                    grid.clear(x, y);
                    // Set the destination
                    grid.set(dest.0 as isize, dest.1 as isize);
                }
            }
        }

        // Update rotation
        dir_rotation_offset = (dir_rotation_offset + 1) % 4;
    }

    //println!("Part 1: {}", find_zero_duplicate_window(4));
    Ok(())
}
