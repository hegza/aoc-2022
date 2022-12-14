use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day14.txt");

#[derive(Clone, PartialEq, PartialOrd)]
struct Point(isize, isize);

impl Point {
    fn x(&self) -> isize {
        self.0
    }
    fn y(&self) -> isize {
        self.1
    }
}

#[derive(Clone)]
struct Wall(Point, Point);

impl Wall {
    fn is_hor(&self) -> bool {
        let start = &self.0;
        let end = &self.1;
        start.y() == end.y()
    }
    fn is_ver(&self) -> bool {
        let start = &self.0;
        let end = &self.1;
        start.x() == end.x()
    }
    fn collides(&self, point: &Point) -> bool {
        (self.is_hor()
            && point.y() == self.0.y()
            && (point.x() >= self.0.x() && point.x() <= self.1.x()))
            || (self.is_ver()
                && point.x() == self.0.x()
                && (point.y() >= self.0.y() && point.y() <= self.1.y()))
    }
}

#[test]
fn walls_collide() {
    let point1 = Point(5, 0);
    let point2 = Point(5, 1);
    let wall = Wall(Point(3, 0), Point(6, 0));

    assert!(wall.is_hor());
    assert!(!wall.is_ver());

    assert!(wall.collides(&point1));
    assert!(!wall.collides(&point2));
}

// Returns true if move happened, false on rest
fn step(sand: &mut Point, walls: &[Wall]) -> bool {
    let possible = &[
        Point(sand.x(), sand.y() + 1),
        Point(sand.x() - 1, sand.y() + 1),
        Point(sand.x() + 1, sand.y() + 1),
    ];
    for next in possible.into_iter() {
        // If none collide, this is valid
        if !walls.iter().any(|wall| wall.collides(next)) {
            *sand = next.clone();
            return true;
        }
    }
    false
}

fn part1(mut walls: Vec<Wall>, bottom: isize) -> usize {
    let spawn = Point(500, 0);
    let mut sand_count = 0;

    // Spawn and step sand until it doesn't work anymore
    'outer: loop {
        let mut sand = spawn.clone();
        while step(&mut sand, &walls) {
            if sand.y() >= bottom {
                break 'outer;
            }
        }
        // Make sand into a wall
        walls.push(Wall(sand.clone(), sand.clone()));
        sand_count += 1;
    }
    sand_count
}

fn part2(mut walls: Vec<Wall>, bottom: isize) -> usize {
    // Add floor
    walls.push(Wall(Point(isize::MIN, bottom), Point(isize::MAX, bottom)));

    let spawn = Point(500, 0);
    let mut sand_count = 0;

    // Spawn and step sand until it doesn't work anymore
    loop {
        let mut sand = spawn.clone();
        while step(&mut sand, &walls) {}

        // Make sand into a wall
        walls.push(Wall(sand.clone(), sand.clone()));
        sand_count += 1;

        // Stop when sand comes to rest on spawn point
        if sand == spawn {
            break;
        }
    }

    sand_count
}

fn main() -> anyhow::Result<()> {
    let walls = INPUT
        .lines()
        .map(|line| {
            line.split("->")
                .map(|s| {
                    let (x, y) = s.trim().split_once(',').unwrap();
                    Point(x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap())
                })
                .tuple_windows::<(Point, Point)>()
                .map(|(p0, p1)| {
                    // Sort the points in ascending order for algo simplicity later
                    if p0 <= p1 {
                        Wall(p0, p1)
                    } else {
                        Wall(p1, p0)
                    }
                })
        })
        .flatten()
        .collect_vec();
    let bottom = walls
        .iter()
        .map(|w| vec![w.0.y(), w.1.y()])
        .flatten()
        .max()
        .unwrap()
        + 2;

    println!("Part 1: {}", part1(walls.clone(), bottom));
    println!("Part 2: {}", part2(walls.clone(), bottom));
    Ok(())
}
