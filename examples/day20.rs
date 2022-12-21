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

fn mix(file: &[isize]) -> Vec<isize> {
    let mut positions = (0..file.len()).collect_vec();

    //println!("Initial:\n{:?}", &collect(&positions, &file));

    // Just sort the indices
    let mut pos_idx = 0;
    while pos_idx != file.len() {
        let curpos = positions.iter().position(|p| *p == pos_idx).unwrap();
        let value = file[pos_idx];
        let npos = new_pos(curpos, value, file.len());

        // Remove at the location where the idx is currently
        let item = positions.remove(curpos);

        // Insert the item at new position
        positions.insert(npos, item);

        //println!("{:?}", &collect(&positions, &file));

        debug_assert_eq!(
            file[positions[npos]], value,
            "incorrect result when moving {} from {} to {}",
            value, curpos, npos
        );

        pos_idx += 1;
    }

    collect(&positions, &file)
}

fn main() -> anyhow::Result<()> {
    let file = INPUT
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect_vec();

    let mixed = mix(&file);

    let pos_of_zero = mixed.iter().position(|v| *v == 0).unwrap();

    let n1 = mixed[(pos_of_zero + 1000) % mixed.len()];
    let n2 = mixed[(pos_of_zero + 2000) % mixed.len()];
    let n3 = mixed[(pos_of_zero + 3000) % mixed.len()];
    let part1 = n1 + n2 + n3;

    println!("Part 1: {}", part1);

    Ok(())
}
