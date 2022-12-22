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

fn part1(monkeys: &HashMap<String, Job>) -> usize {
    resolve("root", monkeys)
}

fn err(x: usize, left: &str, right: &str, monkeys: &mut HashMap<String, Job>) -> isize {
    *monkeys.get_mut("humn").unwrap() = Job::Num(x);

    let (left, right) = (resolve(left, &monkeys), resolve(right, &monkeys));
    left as isize - right as isize
}

// N.b. somehow this converges on a number that's one too big ':D
fn part2(mut monkeys: HashMap<String, Job>) -> usize {
    let root_job = monkeys.remove("root").unwrap();

    if let Job::Op(_, sources) = root_job {
        let (left, right) = (&sources[0], &sources[1]);

        let mut x = 1_000_000_000_000.;

        for _ in 0..10 {
            let x1 = x + (x / 1_000_000_000.);

            let y = err(x as usize, left, right, &mut monkeys) as f64;

            let y1 = err(x1 as usize, left, right, &mut monkeys) as f64;

            let diff_y = y1 - y;
            let diff_x = x1 - x;
            let diff = diff_y / diff_x;

            println!("x: {:.0}, y: {}", x, y);

            // Newton's method
            x -= y / diff;
        }

        return x as usize;
    }

    0
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
    println!("Part 1: {}", part1(&jobs));

    let job = jobs.remove("root").unwrap();
    if let Job::Op(_, sources) = job {
        // I figured this out with the part2 Newton's iteration above
        *jobs.get_mut("humn").unwrap() = Job::Num(3093175982595);
        /* 3093175982596 */
        let (left, right) = (&sources[0], &sources[1]);
        println!("left: {}, right: {}", left, right);
        let left = resolve(left, &jobs);
        let right = resolve(right, &jobs);
        println!("left: {}, right: {}", left, right);
        println!("diff: {}", left - right);
    }

    Ok(())
}
