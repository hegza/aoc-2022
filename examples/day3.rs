use array_tool::vec::Intersect;

const INPUT: &str = include_str!("inputs/day3.txt");

fn part1() {
    let sacks: Vec<(Vec<char>, Vec<char>)> = INPUT
        .lines()
        .map(|line| {
            let (l, r) = line.split_at(line.len() / 2);
            (l.chars().collect(), r.chars().collect())
        })
        .collect();

    let priorities: Vec<(Vec<u32>, Vec<u32>)> = sacks
        .into_iter()
        .map(|(l, r)| {
            (
                l.into_iter().map(|c| char_into_prio(c)).collect(),
                r.into_iter().map(|c| char_into_prio(c)).collect(),
            )
        })
        .collect();

    let total_duplicate_prios = priorities
        .iter()
        .map(|(l, r)| l.intersect(r.to_vec()).iter().sum::<u32>())
        .sum::<u32>();

    println!("{total_duplicate_prios}");
}

fn part2() {
    let sacks: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let badges: Vec<char> = sacks
        .chunks(3)
        .map(|group| {
            group[0]
                .intersect(group[1].to_vec())
                .intersect(group[2].to_vec())[0]
        })
        .collect();

    let total_prio = badges.into_iter().map(|b| char_into_prio(b)).sum::<u32>();

    println!("{total_prio}");
}

fn char_into_prio(c: char) -> u32 {
    if c.is_lowercase() {
        (c as u32) - ('a' as u32) + 1
    } else {
        (c.to_ascii_lowercase() as u32) - ('a' as u32) + 27
    }
}

fn main() -> anyhow::Result<()> {
    part1();
    part2();

    Ok(())
}
