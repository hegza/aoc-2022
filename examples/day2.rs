use core::panic;

const INPUT: &str = include_str!("inputs/day2.txt");

#[derive(Clone, Copy)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Choice {
    fn shape_score(&self) -> u32 {
        *self as u32
    }

    fn from_letter(c: char) -> Self {
        match c {
            'A' | 'X' => Choice::Rock,
            'B' | 'Y' => Choice::Paper,
            'C' | 'Z' => Choice::Scissors,
            _ => panic!("invalid letter"),
        }
    }

    fn beats(&self, other: Choice) -> bool {
        match (self, other) {
            (Choice::Rock, Choice::Scissors) => true,
            (Choice::Paper, Choice::Rock) => true,
            (Choice::Scissors, Choice::Paper) => true,
            _ => false,
        }
    }

    fn pick_lose(&self) -> Self {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }
    fn pick_win(&self) -> Self {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }
}

// Part 1
fn part1(toks: &[(char, char)]) -> u32 {
    toks.iter()
        .map(|(opp, me)| {
            let opp = Choice::from_letter(*opp);
            let me = Choice::from_letter(*me);

            let round_score = me.shape_score()
                + if me.beats(opp) {
                    6
                } else if opp.beats(me) {
                    0
                } else {
                    3
                };
            round_score
        })
        .sum::<u32>()
}

fn part2(toks: &[(char, char)]) -> u32 {
    toks.iter()
        .map(|(opp, me)| {
            let opp = Choice::from_letter(*opp);
            let me = match me {
                'X' => opp.pick_lose(),
                'Y' => opp,
                'Z' => opp.pick_win(),
                _ => panic!(),
            };

            let round_score = me.shape_score()
                + if me.beats(opp) {
                    6
                } else if opp.beats(me) {
                    0
                } else {
                    3
                };
            round_score
        })
        .sum::<u32>()
}

fn main() -> anyhow::Result<()> {
    let toks: Vec<(char, char)> = INPUT
        .lines()
        .map(|line| {
            let mut toks = line.split_ascii_whitespace();
            let opp = toks.nth(0).unwrap().chars().nth(0).unwrap();
            let me = toks.nth(0).unwrap().chars().nth(0).unwrap();
            (opp, me)
        })
        .collect();

    let answer = part1(&toks);
    println!("{answer}");
    let answer = part2(&toks);
    println!("{answer}");
    Ok(())
}
