const INPUT: &str = include_str!("inputs/day10.txt");

fn do_cycle(cycle: &mut i32, reg: i32) -> Option<i32> {
    // Update sig
    let sig = *cycle * reg;
    let ret = match cycle {
        20 | 60 | 100 | 140 | 180 | 220 => Some(sig),
        _ => None,
    };
    *cycle += 1;
    ret
}

fn part1() -> i32 {
    let mut reg = 1;
    let mut cycle = 1;
    let mut sum = 0;
    let extras = "noop\nnoop\nnoop\n";

    for line in INPUT.lines().chain(extras.lines()) {
        let mut toks = line.split_ascii_whitespace();
        let first_tok = toks.next().unwrap();

        match first_tok {
            "noop" => {
                if let Some(x) = do_cycle(&mut cycle, reg) {
                    sum += x;
                };
            }
            "addx" => {
                if let Some(x) = do_cycle(&mut cycle, reg) {
                    sum += x;
                };
                if let Some(x) = do_cycle(&mut cycle, reg) {
                    sum += x;
                };
                let param = toks.next().unwrap().parse::<i32>().unwrap();
                reg += param;
            }
            _ => panic!("unknown instr"),
        }
    }

    sum
}

fn do_cycle_p2(cycle: &mut i32, reg: i32, screen: &mut Vec<Vec<bool>>) {
    let x = (*cycle - 1) % 40;
    let y = (*cycle - 1) / 40;
    if ((reg - 1)..=(reg + 1)).contains(&x) {
        screen[y as usize][x as usize] = true;
    }

    *cycle += 1;
}

fn main() -> anyhow::Result<()> {
    let mut reg = 1;
    let mut cycle = 1;
    let mut sum = 0;
    let extras = "noop\nnoop\nnoop\n";
    let mut screen = vec![vec![false; 40]; 6];

    for line in INPUT.lines().chain(extras.lines()) {
        let mut toks = line.split_ascii_whitespace();
        let first_tok = toks.next().unwrap();

        match first_tok {
            "noop" => {
                do_cycle_p2(&mut cycle, reg, &mut screen);
            }
            "addx" => {
                do_cycle_p2(&mut cycle, reg, &mut screen);
                do_cycle_p2(&mut cycle, reg, &mut screen);
                let param = toks.next().unwrap().parse::<i32>()?;
                reg += param;
            }
            _ => panic!("unknown instr"),
        }
    }

    println!("Part 1: {}", part1());
    println!("Part 2:");
    for line in screen {
        println!(
            "{}",
            line.into_iter()
                .map(|b| if b { '#' } else { ' ' })
                .collect::<String>()
        )
    }
    Ok(())
}
