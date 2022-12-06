use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day6.txt");

fn find_zero_duplicate_window(len: usize) -> usize {
    let chars = INPUT.chars().collect_vec();
    for (idx, win) in chars.windows(len).enumerate() {
        let mut v = win.iter().collect_vec();
        v.sort();
        v.dedup();
        if v.len() == len {
            return idx + len;
        }
    }
    0
}

fn main() -> anyhow::Result<()> {
    println!("Part 1: {}", find_zero_duplicate_window(4));
    println!("Part 2: {}", find_zero_duplicate_window(14));
    Ok(())
}
