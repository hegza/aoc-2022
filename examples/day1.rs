use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day1.txt");

fn _functional() -> u32 {
    let group_strs = INPUT.split("\n\n");
    let mut elves_in_groups = group_strs
        .map(|group| {
            // Convert group of strs into group of integers
            group
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    elves_in_groups.sort_by(|a, b| a.iter().sum::<u32>().cmp(&b.iter().sum::<u32>()));

    let fattest = elves_in_groups.last().unwrap();
    fattest.into_iter().sum::<u32>()
}

fn main() -> anyhow::Result<()> {
    let mut elves = vec![];

    let mut lines = INPUT.lines();
    let mut elf = vec![];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            // Store this elf and start building another one
            elves.push(elf);
            elf = vec![];
            continue;
        }

        let num = line.parse::<u32>()?;
        elf.push(num);
    }

    elves.sort_by(|a, b| a.iter().sum::<u32>().cmp(&b.iter().sum::<u32>()));

    part1(&mut elves);
    part2(&mut elves);

    Ok(())
}

fn part1(elves: &mut [Vec<u32>]) {
    let fattest = elves.last().unwrap();
    let n = fattest.into_iter().sum::<u32>();

    println!("{n}");
}

fn part2(elves: &mut [Vec<u32>]) {
    let three_fattest = elves
        .iter()
        .rev()
        .take(3)
        .map(|x| x.iter().sum::<u32>())
        .sum::<u32>();

    println!("{three_fattest}");
}
