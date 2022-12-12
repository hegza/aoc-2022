use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day12.txt");

fn available_cardinals(hmap: &Vec<Vec<usize>>, this: (usize, usize)) -> Vec<(usize, usize)> {
    let mut av = vec![];
    let curh = hmap[this.1][this.0];
    if this.0 != 0 && hmap[this.1][this.0 - 1] <= curh + 1 {
        av.push((this.0 - 1, this.1));
    }
    if this.0 != hmap[0].len() - 1 && hmap[this.1][this.0 + 1] <= curh + 1 {
        av.push((this.0 + 1, this.1));
    }
    if this.1 != 0 && hmap[this.1 - 1][this.0] <= curh + 1 {
        av.push((this.0, this.1 - 1));
    }
    if this.1 != hmap.len() - 1 && hmap[this.1 + 1][this.0] <= curh + 1 {
        av.push((this.0, this.1 + 1));
    }
    av
}

fn shortest_path_bfs(
    hmap: &Vec<Vec<usize>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let mut q = VecDeque::new();
    let mut explored = HashSet::new();
    explored.insert(start);
    q.push_back(vec![start]);

    while let Some(cur_path) = q.pop_front() {
        let cur = *cur_path.last().unwrap();
        //print!("Inspecting {} at {:?}...", hmap[cur.1][cur.0], cur);
        if cur == end {
            return Some(cur_path);
        }

        let insert = available_cardinals(hmap, cur)
            .into_iter()
            .filter(|coord| !explored.contains(coord))
            .collect_vec();
        //println!(" inserted {:?}", insert);
        for coord in insert {
            explored.insert(coord);
            let mut npath = cur_path.clone();
            npath.push(coord);
            q.push_back(npath);
        }
    }
    None
}

fn main() -> anyhow::Result<()> {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let hmap: Vec<Vec<usize>> = INPUT
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let c = match c {
                        'S' => {
                            start = (x, y);
                            'a'
                        }
                        'E' => {
                            end = (x, y);
                            'z'
                        }
                        c => c,
                    };
                    c as usize - 97
                })
                .collect_vec()
        })
        .collect();

    println!(
        "Part 1: {}",
        shortest_path_bfs(&hmap, start, end).unwrap().len() - 1
    );

    let mut relevant_starts = Vec::new();

    for (y, line) in hmap.iter().enumerate() {
        for (x, h) in line.iter().enumerate() {
            if *h == 0 {
                relevant_starts.push((x, y));
            }
        }
    }

    println!("#-of relevant starts: {}", relevant_starts.len());

    let shortest_paths = relevant_starts
        .iter()
        .filter_map(|start| shortest_path_bfs(&hmap, *start, end).map(|x| x.len() - 1))
        .collect_vec();

    println!("Part 2: {}", *shortest_paths.iter().min().unwrap());

    Ok(())
}
