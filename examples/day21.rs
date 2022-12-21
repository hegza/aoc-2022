use std::collections::HashMap;

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

enum Job {
    Num(usize),
    Op(Op, [String; 2]),
}

const INPUT: &str = include_str!("inputs/day21.txt");

fn resolve(start: &str, monkeys: &HashMap<String, Job>) -> usize {
    match &monkeys[start] {
        Job::Num(num) => *num,
        Job::Op(op, params) => {
            let (left, right) = (&params[0], &params[1]);
            let (left, right) = (resolve(left, monkeys), resolve(right, monkeys));
            match op {
                Op::Add => left + right,
                Op::Sub => left - right,
                Op::Mul => left * right,
                Op::Div => left / right,
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut jobs = HashMap::new();

    for line in INPUT.lines() {
        let (name_str, job_str) = line.split_once(':').unwrap();
        let job_str = job_str.trim();

        let job = match job_str.parse::<usize>() {
            Ok(num) => Job::Num(num),
            // If it's not a number, it's an op
            Err(_) => {
                let mut job_toks = job_str.split_whitespace();

                let left_str = job_toks.next().unwrap().trim().to_string();
                let op_str = job_toks.next().unwrap();
                let right_str = job_toks.next().unwrap().to_string();

                debug_assert!(left_str.len() == 4);
                debug_assert!(op_str.len() == 1);
                debug_assert!(right_str.len() == 4);

                let op = match op_str.chars().nth(0).unwrap() {
                    '+' => Op::Add,
                    '-' => Op::Sub,
                    '*' => Op::Mul,
                    '/' => Op::Div,
                    c => panic!("unknown op: '{}' from \"{}\"", c, op_str),
                };
                Job::Op(op, [left_str, right_str])
            }
        };
        jobs.insert(name_str.to_string(), job);
    }

    println!("Part 1: {}", resolve("root", &jobs));
    Ok(())
}
