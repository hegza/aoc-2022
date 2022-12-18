use regex::Regex;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("inputs/day16.txt");

type Id = [char; 2];

fn str_to_id(s: &str) -> Id {
    let mut chars = s.chars();
    [chars.next().unwrap(), chars.next().unwrap()]
}

fn id_to_string(id: &Id) -> String {
    [id[0], id[1]].iter().collect()
}

/// Returns the best strategy available when starting at this position, with
/// `min_rem` minutes remaining.
fn best_strategy(
    pos: &Id,
    time_rem: usize,
    jumps: &HashMap<Id, Vec<Id>>,
    nz_wgts: &HashMap<Id, usize>,
    opened: HashSet<Id>,
    mut curpath: Vec<Id>,
) -> usize {
    curpath.push(*pos);

    // If there is no time left, there are no more strategies left
    if time_rem == 0 {
        return 0;
    }

    // All non-zero valves open, no further actions can add value
    if opened.len() == nz_wgts.len() {
        return 0;
    }

    let mut all_strats = vec![];

    let wgt = nz_wgts.get(pos).unwrap_or(&0);

    // If this is not already open, the strategy of just opening this one is
    // available
    let mut open_this = 0;
    if !opened.contains(pos) {
        open_this = (time_rem - 1) * wgt;
        all_strats.push(open_this);
    };

    // Add strategies in subsequent nodes
    for dest in &jumps[pos] {
        // Strategies where we revisit nodes do not make sense
        if curpath.contains(dest) {
            continue;
        }

        let mut opened = opened.clone();

        // Strats for **not** opening this node (-0) + moving through tunnel (-1)
        let dont_open = best_strategy(
            dest,
            time_rem - 1,
            jumps,
            nz_wgts,
            opened.clone(),
            curpath.clone(),
        );
        all_strats.push(dont_open);

        // Strats for opening this node (-1) + moving through tunnel (-1)
        if open_this != 0 && time_rem > 2 {
            opened.insert(*pos);
            let open = open_this
                + best_strategy(dest, time_rem - 2, jumps, nz_wgts, opened, curpath.clone());
            all_strats.push(open);
        }
    }

    // Identify the best strategy available at this position with current time
    let best_here = *all_strats.iter().max().unwrap_or(&0);

    best_here
}

fn part1(jumps: &HashMap<Id, Vec<Id>>, nz_wgts: &HashMap<Id, usize>) -> usize {
    let curpos = str_to_id("AA");
    best_strategy(&curpos, 30, &jumps, &nz_wgts, HashSet::new(), vec![])
}

fn part2(jumps: &HashMap<Id, Vec<Id>>, nz_wgts: &HashMap<Id, usize>) -> usize {
    let mypos = str_to_id("AA");
    let elepos = str_to_id("AA");

    best_strategy(&mypos, 24, &jumps, &nz_wgts, HashSet::new(), vec![mypos])
}

fn main() -> anyhow::Result<()> {
    let valve_re = Regex::new(r"([A-Z]{2})").unwrap();
    let flow_re = Regex::new(r"(\d+)").unwrap();
    let edges = INPUT
        .lines()
        .map(|line| {
            let mut valves = valve_re.captures_iter(line);

            let flow_tok = &flow_re.captures_iter(line).next().unwrap()[0];
            let flow = flow_tok.parse::<usize>().unwrap();

            // Each line creates N edges
            let mut edges = vec![];
            let origin = valves.next().unwrap()[0].to_string();
            for dest in valves {
                let dest = &dest[0];
                edges.push((str_to_id(&origin), str_to_id(dest), flow));
            }
            edges
        })
        .flatten();

    let (jumps, flows): (Vec<(_, _)>, HashMap<_, _>) = edges
        .clone()
        .map(|(orig, dest, flow)| ((orig, dest), (orig, flow)))
        .unzip();
    let flows: HashMap<_, _> = flows.into_iter().filter(|(_, wgt)| *wgt != 0).collect();
    let jumps = {
        let mut njumps = HashMap::new();
        for (orig, dest) in jumps {
            let entry = njumps.entry(orig).or_insert(vec![]);
            entry.push(dest);
        }
        njumps
    };

    println!("Part 1: {}", part1(&jumps, &flows));
    //println!("Part 2: {}", find_zero_duplicate_window(14));
    Ok(())
}
