use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day5.txt");

#[derive(Clone)]
struct Stacks(Vec<Vec<char>>);

impl Stacks {
    fn from_init_lines(lines: &[&str]) -> Self {
        let mut stacks = vec![Vec::with_capacity(8); 9];

        let lines = lines.iter().rev();

        // Skip the number heading
        let lines = lines.skip(1).collect_vec();

        for line in lines {
            // Break into crates
            for (idx, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
                // Pick the characted, which is always 2nd
                let letter = match chunk.nth(1).unwrap() {
                    ' ' => None,
                    l => Some(l),
                };

                if let Some(letter) = letter {
                    stacks[idx].push(letter);
                }
            }
        }

        Stacks(stacks)
    }

    fn move_crate(&mut self, from: usize, to: usize) {
        self.move_crate_stack(from, to, 1);
    }

    fn move_crates_one_by_one(&mut self, from: usize, to: usize, n: usize) {
        for _ in 0..n {
            self.move_crate(from, to);
        }
    }

    fn move_crate_stack(&mut self, from: usize, to: usize, n: usize) {
        let from_stack = &mut self.0[from];
        let popped = from_stack.split_off(from_stack.len() - n);

        self.0[to].extend_from_slice(&popped);
    }

    fn top(&self, stack: usize) -> Option<char> {
        self.0[stack].last().copied()
    }

    fn top_string(&self) -> String {
        (0..self.0.len())
            .into_iter()
            .filter_map(|x| self.top(x))
            .collect::<String>()
    }
}

fn part1(mut stacks: Stacks, cmds: &[(usize, usize, usize)]) -> Stacks {
    for (n, from, to) in cmds {
        stacks.move_crates_one_by_one(*from, *to, *n);
    }
    stacks
}

fn part2(mut stacks: Stacks, cmds: &[(usize, usize, usize)]) -> Stacks {
    for (n, from, to) in cmds {
        stacks.move_crate_stack(*from, *to, *n);
    }
    stacks
}

fn main() -> anyhow::Result<()> {
    let lines = INPUT.lines().collect_vec();
    let init_lines = &lines[0..9];
    let cmd_lines = &lines[10..];

    let stacks = Stacks::from_init_lines(init_lines);

    let commands = cmd_lines
        .iter()
        .map(|line| {
            let toks = line.split_ascii_whitespace().collect_vec();
            let n = toks[1].parse::<usize>().unwrap();
            let from = toks[3].parse::<usize>().unwrap() - 1;
            let to = toks[5].parse::<usize>().unwrap() - 1;
            (n, from, to)
        })
        .collect_vec();

    let p1 = part1(stacks.clone(), &commands).top_string();
    println!("Part 1: {p1}");

    let p2 = part2(stacks.clone(), &commands).top_string();
    println!("Part 2: {p2}");

    Ok(())
}
