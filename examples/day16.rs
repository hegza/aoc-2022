use regex::Regex;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("inputs/day16.txt");

type Id = [char; 2];

fn str_to_id(s: &str) -> Id {
    let mut chars = s.chars();
    [chars.next().unwrap(), chars.next().unwrap()]
}

fn _id_to_string(id: &Id) -> String {
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
) -> (usize, Vec<Id>) {
    curpath.push(*pos);

    // If there is no time left, there are no more strategies left
    if time_rem == 0 {
        return (0, curpath);
    }

    // All non-zero valves open, no further actions can add value
    if opened.len() == nz_wgts.len() {
        return (0, curpath);
    }

    let mut all_strats = vec![];

    let wgt = nz_wgts.get(pos).unwrap_or(&0);

    // If this is not already open, the strategy of just opening this one is
    // available
    let mut open_this = 0;
    if !opened.contains(pos) {
        open_this = (time_rem - 1) * wgt;
        all_strats.push((open_this, curpath.clone()));
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
            let strat = best_strategy(dest, time_rem - 2, jumps, nz_wgts, opened, curpath.clone());
            let open_val = open_this + strat.0;
            all_strats.push((open_val, strat.1));
        }
    }

    // Identify the best strategy available at this position with current time
    let best_here = all_strats.iter().max().unwrap_or(&(0, curpath)).clone();

    best_here
}

fn part1(jumps: &HashMap<Id, Vec<Id>>, nz_wgts: &HashMap<Id, usize>) -> usize {
    let curpos = str_to_id("AA");
    best_strategy(&curpos, 30, &jumps, &nz_wgts, HashSet::new(), vec![]).0
}

fn do_greedy_next(
    curpos: &Id,
    time_rem: usize,
    opened: &mut HashSet<Id>,
    jumps: &HashMap<Id, Vec<Id>>,
    nz_wgts: &HashMap<Id, usize>,
) -> (Id, usize) {
    // Find the strategy that is currently the best
    let best = best_strategy(curpos, time_rem, jumps, nz_wgts, opened.clone(), vec![]).1;

    // If there is a valve here, open it
    if let Some(wgt) = nz_wgts.get(curpos) {
        opened.insert(*curpos);
        let pressure = (time_rem - 1) * wgt;
        return (*curpos, pressure);
    } else {
        // If the next step is somewhere else, we change position
        return (*best.get(1).unwrap_or(curpos), 0);
    }
}

/// Greedy
fn part2(jumps: &HashMap<Id, Vec<Id>>, nz_wgts: &HashMap<Id, usize>) -> usize {
    let mut released = 0;

    let mut mypos = str_to_id("AA");
    let mut elepos = str_to_id("AA");
    let mut time_rem = 24;
    let mut opened = HashSet::new();

    while time_rem != 0 {
        println!("Time remaining: {time_rem}");
        let mynext = do_greedy_next(&mypos, time_rem, &mut opened, jumps, nz_wgts);
        mypos = mynext.0;
        released += mynext.1;

        let elenext = do_greedy_next(&elepos, time_rem, &mut opened, jumps, nz_wgts);
        elepos = elenext.0;
        released += elenext.1;

        time_rem -= 1;
    }
    released
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
    println!("Part 2: {}", part2(&jumps, &flows));
    Ok(())
}
