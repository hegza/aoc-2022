use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day20.txt");

fn new_pos(old_pos: usize, value: isize, len: usize) -> usize {
    let len = len - 1;
    let new_pos = old_pos as isize + value;
    (((new_pos % len as isize) + len as isize) % len as isize) as usize
}

fn collect(positions: &[usize], file: &[isize]) -> Vec<isize> {
    positions.iter().map(|pos| file[*pos]).collect_vec()
}

fn mix(positions: &mut Vec<usize>, file: &[isize]) {
    for pos_idx in 0..file.len() {
        let curpos = positions.iter().position(|p| *p == pos_idx).unwrap();
        let value = file[pos_idx];
        let npos = new_pos(curpos, value, file.len());

        // Remove at the location where the idx is currently
        let item = positions.remove(curpos);

        // Insert the item at new position
        positions.insert(npos, item);
    }
}

fn mix_and_collect(file: &[isize]) -> Vec<isize> {
    let mut positions = (0..file.len()).collect_vec();
    mix(&mut positions, &file);
    collect(&positions, &file)
}

fn part1(file: &[isize]) -> isize {
    let mixed = mix_and_collect(&file);

    let pos_of_zero = mixed.iter().position(|v| *v == 0).unwrap();

    let n1 = mixed[(pos_of_zero + 1000) % mixed.len()];
    let n2 = mixed[(pos_of_zero + 2000) % mixed.len()];
    let n3 = mixed[(pos_of_zero + 3000) % mixed.len()];
    n1 + n2 + n3
}

fn part2(file: &[isize]) -> isize {
    let key = 811589153;
    let file = file.iter().map(|x| *x * key).collect_vec();

    let mut positions = (0..file.len()).collect_vec();
    for _ in 0..10 {
        mix(&mut positions, &file);
    }
    let mixed = collect(&positions, &file);

    let pos_of_zero = mixed.iter().position(|v| *v == 0).unwrap();

    let n1 = mixed[(pos_of_zero + 1000) % mixed.len()];
    let n2 = mixed[(pos_of_zero + 2000) % mixed.len()];
    let n3 = mixed[(pos_of_zero + 3000) % mixed.len()];
    n1 + n2 + n3
}

fn main() -> anyhow::Result<()> {
    let file = INPUT
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect_vec();

    println!("Part 2: {}", part1(&file));
    println!("Part 2: {}", part2(&file));

    Ok(())
}
