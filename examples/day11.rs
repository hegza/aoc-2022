use itertools::Itertools;
use std::{num::ParseIntError, str::FromStr};

const INPUT: &str = include_str!("inputs/day11.txt");
impl std::str::FromStr for Monkey {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        // Skip heading "Monkey X"
        lines.next().unwrap();

        let items = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split(',')
            .map(|tok| tok.trim().parse::<usize>().unwrap())
            .collect_vec();

        let op_str = lines.next().unwrap().split_once(':').unwrap().1.trim();
        let (operator, param) = {
            let (_, rhs) = op_str.split_once('=').unwrap();
            let mut toks = rhs.trim().split_ascii_whitespace();

            // Skip 'old'
            let old_str = toks.next().unwrap();
            assert_eq!(old_str, "old");

            let operator = toks.next().unwrap().chars().next().unwrap();
            let param = match toks.next().unwrap() {
                "old" => None,
                num => Some(num.parse::<usize>().unwrap()),
            };
            (operator, param)
        };
        let op = move |old: usize| {
            let param = match param {
                Some(n) => n,
                None => old,
            };
            match operator {
                '*' => old * param,
                '+' => old + param,
                _ => panic!("unknown op: {}", operator),
            }
        };
        let op = Box::new(op);

        let test_var = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()?;
        let next_if_true = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()?;
        let next_if_false = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()?;

        Ok(Monkey {
            items,
            op,
            test_var,
            next_if_true,
            next_if_false,
        })
    }
}

struct Monkey {
    items: Vec<usize>,
    op: Box<dyn Fn(usize) -> usize>,
    test_var: usize,
    next_if_true: usize,
    next_if_false: usize,
}

impl Monkey {
    // Returns (where, what)
    fn take_turn(&mut self, relief: bool, common_denom: usize) -> Vec<(usize, usize)> {
        let mut thrown = Vec::with_capacity(self.items.len());
        let items = self.items.clone();
        self.items.clear();
        for item in items {
            thrown.push(self.inspect(item, relief, common_denom));
        }
        thrown
    }

    // Returns thrown (where, what)
    fn inspect(&self, item: usize, relief: bool, common_denom: usize) -> (usize, usize) {
        let item = (self.op)(item);

        // Part 1 only
        let item = if relief {
            item / 3
        } else {
            item % common_denom
        };

        let next = if (item % self.test_var) == 0 {
            self.next_if_true
        } else {
            self.next_if_false
        };
        (next, item)
    }
}

fn round(monkeys: &mut [Monkey], relief: bool, divs: &[usize]) -> [usize; 8] {
    let mut inspections = [0; 8];

    let common_denom: usize = divs.iter().product();
    for idx in 0..8 {
        let monkey = &mut monkeys[idx];
        let thrown = monkey.take_turn(relief, common_denom);
        inspections[idx] += thrown.len() as usize;
        for (next, item) in thrown {
            monkeys[next].items.push(item);
        }
    }

    inspections
}

fn part1(mut monkeys: Vec<Monkey>) -> usize {
    let monkeys = monkeys.as_mut_slice();

    let mut inspections = vec![0; 8];
    for _ in 0..20 {
        let this_round = round(monkeys, true, &[]);
        this_round
            .iter()
            .zip(inspections.iter_mut())
            .for_each(|(round, i)| {
                *i += *round;
            });
    }

    inspections.sort_by(|a, b| b.cmp(a));
    let two_most_active = &inspections[0..2];

    // Monkey business
    two_most_active[0] * two_most_active[1]
}

fn part2(mut monkeys: Vec<Monkey>) -> usize {
    let divisors = monkeys.iter().map(|m| m.test_var).collect_vec();

    let monkeys = monkeys.as_mut_slice();

    let mut inspections = vec![0; 8];
    for _rnd in 0..10_000 {
        let this_round = round(monkeys, false, &divisors);
        this_round
            .iter()
            .zip(inspections.iter_mut())
            .for_each(|(round, i)| {
                *i += *round;
            });
    }

    inspections.sort_by(|a, b| b.cmp(a));
    let two_most_active = &inspections[0..2];

    // Monkey business
    two_most_active[0] * two_most_active[1]
}

fn main() -> anyhow::Result<()> {
    let monkeys = INPUT
        .split("\n\n")
        .map(|text| Monkey::from_str(text).unwrap())
        .collect_vec();

    //println!("Part 1: {}", part1(monkeys));
    println!("Part 2: {}", part2(monkeys));
    Ok(())
}
