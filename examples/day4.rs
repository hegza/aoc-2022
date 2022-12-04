use std::ops::RangeInclusive;

use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day4.txt");

fn part1(pairs: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) {
    let mut count = 0;

    for (l, r) in pairs {
        let mut l_contains_r = true;
        let mut r_contains_l = true;
        for x in r.clone() {
            if !l.contains(&x) {
                l_contains_r = false;
                break;
            }
        }
        for x in l.clone() {
            if !r.contains(&x) {
                r_contains_l = false;
                break;
            }
        }
        if l_contains_r || r_contains_l {
            count += 1;
        }
    }

    println!("{count}");
}

fn part2(pairs: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) {
    let mut count = 0;

    for (l, r) in pairs {
        let mut l_contains_r = false;
        let mut r_contains_l = false;
        for x in r.clone() {
            if l.contains(&x) {
                l_contains_r = true;
                break;
            }
        }
        for x in l.clone() {
            if r.contains(&x) {
                r_contains_l = true;
                break;
            }
        }
        if l_contains_r || r_contains_l {
            count += 1;
        }
    }

    println!("{count}");
}

fn main() -> anyhow::Result<()> {
    let pairs = INPUT
        .lines()
        .map(|line| {
            let mut sections = line.split(',').map(|line| {
                line.split('-')
                    .map(|section| section.parse::<u32>().unwrap())
            });
            let mut l_sects = sections.next().unwrap();
            let mut r_sects = sections.next().unwrap();

            let l = l_sects.next().unwrap()..=l_sects.next().unwrap();
            let r = r_sects.next().unwrap()..=r_sects.next().unwrap();

            (l, r)
        })
        .collect_vec();

    part1(&pairs);
    part2(&pairs);

    Ok(())
}
