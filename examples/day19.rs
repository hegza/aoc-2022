use itertools::Itertools;
use regex::Regex;
use std::{
    collections::HashMap,
    convert::Infallible,
    str::FromStr,
    time::{Duration, SystemTime},
};

const INPUT: &str = include_str!("inputs/day19.txt");

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl FromStr for Resource {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "ore" => Resource::Ore,
            "clay" => Resource::Clay,
            "obsidian" => Resource::Obsidian,
            "geode" => Resource::Geode,
            s => panic!("cannot parse Resource from \"{}\"", s),
        })
    }
}

#[derive(Default, Debug, Clone)]
struct Costs(Vec<(Resource, usize)>);

impl From<Vec<(Resource, usize)>> for Costs {
    fn from(v: Vec<(Resource, usize)>) -> Self {
        Self(v)
    }
}

#[derive(Default, Debug)]
struct Blueprint(usize, HashMap<Resource, Costs>);

impl Blueprint {
    fn from_bot_costs(id: usize, bot_costs: Vec<(Resource, Costs)>) -> Self {
        let bp = Blueprint(id, bot_costs.into_iter().collect());

        debug_assert!(
            bp.ore_bot().0.len() != 0
                && bp.clay_bot().0.len() != 0
                && bp.obsidian_bot().0.len() != 0
                && bp.geode_bot().0.len() != 0
        );

        bp
    }

    fn ore_bot(&self) -> &Costs {
        &self.1[&Resource::Ore]
    }
    fn clay_bot(&self) -> &Costs {
        &self.1[&Resource::Clay]
    }
    fn obsidian_bot(&self) -> &Costs {
        &self.1[&Resource::Obsidian]
    }
    fn geode_bot(&self) -> &Costs {
        &self.1[&Resource::Geode]
    }
}

impl Blueprint {
    fn parse(s: &str) -> anyhow::Result<Blueprint> {
        let (id_str, cost_sents) = s.split_once(':').unwrap();

        let bot_re = Regex::new(r"Each (\w+) robot")?;
        let cost_re = Regex::new(r"(\d+) ([a-z]+)")?;

        let bot_costs = cost_sents[..cost_sents.len() - 1]
            .split('.')
            .map(|cost_sent| {
                let bot_kind_caps = bot_re.captures(cost_sent).unwrap();
                let bot_kind = Resource::from_str(&bot_kind_caps[1]).unwrap();
                let costs = cost_re
                    .captures_iter(cost_sent)
                    .map(|caps| {
                        let num = &caps[1];
                        let res = &caps[2];
                        (
                            Resource::from_str(res).unwrap(),
                            num.parse::<usize>().unwrap(),
                        )
                    })
                    .collect_vec();
                (bot_kind, costs.into())
            })
            .collect_vec();

        let id = id_str
            .split_ascii_whitespace()
            .skip(1)
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let bp = Blueprint::from_bot_costs(id, bot_costs);
        Ok(bp)
    }
}

fn options(
    ignore: &[Resource],
    resources: &HashMap<Resource, usize>,
    blueprint: &Blueprint,
) -> Vec<(Resource, Costs)> {
    let mut options = Vec::with_capacity(4);
    for (bot, bot_costs) in &blueprint.1 {
        if ignore.contains(bot) {
            continue;
        }
        if bot_costs
            .0
            .iter()
            .all(|(res, cost)| resources[res] >= *cost)
        {
            options.push((*bot, bot_costs.clone()));
        }
    }
    options
}

fn triangular(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        2 => 3,
        3 => 6,
        4 => 10,
        5 => 15,
        6 => 21,
        _ => unimplemented!(),
    }
}

static mut TURNS_SIMULATED: usize = 0;
static mut STRATEGIES_SIMULATED: usize = 0;

fn simulate_dfs(
    choice: Option<(Resource, Costs)>,
    time: usize,
    mut resources: HashMap<Resource, usize>,
    mut bots: HashMap<Resource, usize>,
    blueprint: &Blueprint,
) -> Vec<usize> {
    // Pay resources first...
    if let Some((_, costs)) = &choice {
        for (res, count) in &costs.0 {
            *resources.get_mut(res).unwrap() -= count;
        }
    }

    // Then gain resources
    for (bot, bot_cnt) in &bots {
        *resources.get_mut(&bot).unwrap() += bot_cnt;
    }

    // Then gain bot
    if let Some((bot, _)) = &choice {
        *bots.get_mut(&bot).unwrap() += 1;
    }

    // Resolve and explore options for this round

    unsafe { TURNS_SIMULATED += 1 };

    if time == 1 {
        unsafe { STRATEGIES_SIMULATED += 1 };
        return vec![resources[&Resource::Geode] + bots[&Resource::Geode]];
    }

    let time = time - 1;
    let mut ignore = Vec::with_capacity(4);
    if time <= 2 {
        ignore.push(Resource::Clay);
    }
    let opts = options(&ignore, &resources, &blueprint);

    // If we cannot benefit from any more geode bots, return whatever current bots are able to mine
    if time <= 6 {
        let geode_bot_cost_in_obsidian = blueprint.geode_bot().0[1].1;
        let maximum_possible_obsidian =
            // Current obsidian + minable by current bots + minable by future bots (if we build one per turn)
            resources[&Resource::Obsidian] + time * bots[&Resource::Obsidian] + triangular(time);
        if geode_bot_cost_in_obsidian > maximum_possible_obsidian {
            unsafe { STRATEGIES_SIMULATED += 1 };
            return vec![resources[&Resource::Geode] + time * bots[&Resource::Geode]];
        }
    }

    let mut v = vec![];

    // Simulate the option where we don't build anything, though skip it if we
    // have the choice of building any of the four bots (because then there is
    // nothing to save for)
    if opts.len() != 4 {
        let geodes = simulate_dfs(None, time, resources.clone(), bots.clone(), blueprint);
        v.extend(geodes);
    }

    // Simulate all bot options
    for opt in opts {
        let geodes = simulate_dfs(Some(opt), time, resources.clone(), bots.clone(), blueprint);
        v.extend(geodes);
    }
    v
}

fn simulate_all(blueprint: &Blueprint) -> Vec<usize> {
    let bots: HashMap<_, _> = [
        (Resource::Ore, 1),
        (Resource::Clay, 0),
        (Resource::Obsidian, 0),
        (Resource::Geode, 0),
    ]
    .into_iter()
    .collect();

    let resources: HashMap<_, _> = [
        (Resource::Ore, 0),
        (Resource::Clay, 0),
        (Resource::Obsidian, 0),
        (Resource::Geode, 0),
    ]
    .into_iter()
    .collect();
    let time = 24;

    // Initial options available
    let initial_opts = options(&vec![], &resources, &blueprint);

    // Simulate all options
    let mut v = vec![];
    for opt in initial_opts {
        let geodes = simulate_dfs(Some(opt), time, resources.clone(), bots.clone(), blueprint);
        v.extend(geodes);
    }
    // Also simulate the option where we don't build anything
    let geodes = simulate_dfs(None, time, resources.clone(), bots.clone(), blueprint);
    v.extend(geodes);
    v
}

static mut PERF: bool = true;

fn main() -> anyhow::Result<()> {
    let blueprints = INPUT
        .lines()
        .map(|line| Blueprint::parse(line).unwrap())
        .collect_vec();

    // Make a performance test thread
    let perf_thread = std::thread::spawn(|| {
        let mut ptime = SystemTime::now();
        let mut pturns = unsafe { TURNS_SIMULATED };
        let mut pstrats = unsafe { STRATEGIES_SIMULATED };
        const INTERVAL_SEC: u64 = 5;
        while unsafe { PERF } {
            let since_last_measure = SystemTime::now().duration_since(ptime).unwrap();
            if since_last_measure >= Duration::from_secs(INTERVAL_SEC) {
                let turns = unsafe { TURNS_SIMULATED };
                let strats = unsafe { STRATEGIES_SIMULATED };
                let ktps = (turns - pturns) / 1000 / INTERVAL_SEC as usize;
                let ksps = (strats - pstrats) / 1000 / INTERVAL_SEC as usize;
                println!(
                    "Turns per second: {} k, strategies per second: {} k",
                    ktps, ksps
                );
                pturns = turns;
                pstrats = strats;
                ptime = SystemTime::now();
            }
        }
    });

    let sum: usize = blueprints
        //.par_iter()
        .iter()
        .map(|bp| {
            let id = bp.0;
            let geodes = simulate_all(bp);
            let best = geodes.into_iter().max().unwrap();
            println!(
                "Blueprint {} simulated: {} (quality = {})",
                id,
                best,
                id * best
            );
            id * best
        })
        .sum();

    println!("Part 1: {}", sum);
    //println!("Part 2: {}", find_zero_duplicate_window(14));

    unsafe {
        PERF = false;
    }
    perf_thread.join().unwrap();
    Ok(())
}
