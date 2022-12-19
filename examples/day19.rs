use std::collections::HashMap;
use rayon::prelude::*;

#[allow(unused)]
use adventofcode2022::{get_input,parse_lines,regex_parser,timeit};

#[derive(Debug)]
pub struct Blueprint {
    n: usize,
    ore_cost: usize,
    clay_cost: usize, 
    obsidian_cost_ore: usize,
    obsidian_cost_clay: usize,
    geode_cost_ore: usize,
    geode_cost_obsidian: usize,
}

regex_parser!(parse_bp: Blueprint {
    BP = r#"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$"# =>
        |n: usize, ore_cost: usize, clay_cost: usize,
         obsidian_cost_ore: usize, obsidian_cost_clay: usize,
         geode_cost_ore: usize, geode_cost_obsidian: usize|
             Blueprint { n, ore_cost, clay_cost, 
             obsidian_cost_ore, obsidian_cost_clay,
             geode_cost_ore, geode_cost_obsidian }
});

type Data = Vec<Blueprint>;
fn parse_input(input: &str) -> Data {
    parse_lines(input)
}

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
struct State {
    num_ore_robots: usize,
    num_clay_robots: usize,
    num_obsidian_robots: usize,
    num_geode_robots: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    time_left: usize,
}

fn find_best_strategy(bp: &Blueprint, result: &mut HashMap<State, usize>, state: &State) -> usize {
    if result.contains_key(state) {
        return *result.get(state).unwrap();
    }
    if state.time_left == 0 {
        return 0;
    }

    let mut newstate = *state;
    newstate.ore += state.num_ore_robots;
    newstate.clay += state.num_clay_robots;
    newstate.obsidian += state.num_obsidian_robots;
    newstate.time_left -= 1;
    let geodes_now = state.num_geode_robots;

    let mut possibilities = vec![];
    // Can we build a geode robot?  If so always do so.
    if state.obsidian >= bp.geode_cost_obsidian &&
       state.ore >=      bp.geode_cost_ore {
        let mut test_state = newstate;
        test_state.num_geode_robots += 1;
        test_state.obsidian -= bp.geode_cost_obsidian;
        test_state.ore -= bp.geode_cost_ore;
        possibilities.push(find_best_strategy(bp, result, &test_state));
    } else {
        // Can we build an obsidian robot? 
        if state.clay >= bp.obsidian_cost_clay &&
           state.ore >=  bp.obsidian_cost_ore {
            let mut test_state = newstate;
            test_state.num_obsidian_robots += 1;
            test_state.clay -= bp.obsidian_cost_clay;
            test_state.ore -= bp.obsidian_cost_ore;
            possibilities.push(find_best_strategy(bp, result, &test_state));
        }
        // Or a clay robot?
        if state.ore >=  bp.clay_cost {
            let mut test_state = newstate;
            test_state.num_clay_robots += 1;
            test_state.ore -= bp.clay_cost;
            possibilities.push(find_best_strategy(bp, result, &test_state));
        }
        // Or an ore robot?
        if state.ore >=  bp.ore_cost {
            let mut test_state = newstate;
            test_state.num_ore_robots += 1;
            test_state.ore -= bp.ore_cost;
            possibilities.push(find_best_strategy(bp, result, &test_state));
        }
        possibilities.push(find_best_strategy(bp, result, &newstate));
    }

    let best = possibilities.into_iter().max().unwrap();
    result.insert(*state, best);
    best + geodes_now
}

fn find_best_strategy2(bp: &Blueprint, result: &mut HashMap<State, usize>, state: &State) -> usize {
    if result.contains_key(state) {
        return *result.get(state).unwrap();
    }
    if state.time_left == 0 {
        return 0;
    }

    let mut newstate = *state;
    newstate.ore += state.num_ore_robots;
    newstate.clay += state.num_clay_robots;
    newstate.obsidian += state.num_obsidian_robots;
    newstate.time_left -= 1;
    let geodes_now = state.num_geode_robots;

    fn time_to_make_ore_robot(bp: &Blueprint, state: &State) -> usize {
        if state.ore >= bp.ore_cost {
            return 0;
        }
        (bp.ore_cost - state.ore + state.num_ore_robots - 1) / state.num_ore_robots
    }
    fn time_to_make_clay_robot(bp: &Blueprint, state: &State) -> usize {
        if state.ore >= bp.clay_cost {
            return 0;
        }
        (bp.clay_cost - state.ore + state.num_ore_robots - 1) / state.num_ore_robots
    }
    fn time_to_make_obsidian_robot(bp: &Blueprint, state: &State) -> (usize, usize) {
        let ore_time = if state.ore >= bp.obsidian_cost_ore {
            0
        } else {
            (bp.obsidian_cost_ore - state.ore + state.num_ore_robots - 1) / state.num_ore_robots
        };
        let clay_time = if state.clay >= bp.obsidian_cost_clay {
            0
        } else if state.num_clay_robots == 0 {
            usize::MAX
        } else {
            (bp.obsidian_cost_clay - state.clay + state.num_clay_robots - 1) / state.num_clay_robots
        };
        (ore_time, clay_time)
    }
    fn time_to_make_geode_robot(bp: &Blueprint, state: &State) -> (usize, usize) {
        let ore_time = if state.ore >= bp.geode_cost_ore {
            0
        } else {
            (bp.geode_cost_ore - state.ore + state.num_ore_robots - 1) / state.num_ore_robots
        };
        let obsidian_time = if state.obsidian >= bp.geode_cost_obsidian {
            0
        } else if state.num_obsidian_robots == 0 {
            usize::MAX
        } else {
            (bp.geode_cost_obsidian - state.obsidian + state.num_obsidian_robots - 1) / state.num_obsidian_robots
        };
        (ore_time, obsidian_time)
    }

    let mut possibilities = vec![];
    // Can we build a geode robot?  If so always do so.
    if state.obsidian >= bp.geode_cost_obsidian &&
       state.ore >=      bp.geode_cost_ore {
        let mut test_state = newstate;
        test_state.num_geode_robots += 1;
        test_state.obsidian -= bp.geode_cost_obsidian;
        test_state.ore -= bp.geode_cost_ore;
        possibilities.push(find_best_strategy2(bp, result, &test_state));
    } else {
        // We want to build a geode robot as soon as possible.  How soon?
        let (geode_ore_time, geode_obsidian_time) = time_to_make_geode_robot(bp, &newstate);
        if geode_ore_time >= geode_obsidian_time {
            // Work towards building an ore robot
            if newstate.ore >= bp.ore_cost {
                // We can build one
                let mut test_state = newstate;
                test_state.num_ore_robots += 1;
                test_state.ore -= bp.ore_cost;
                possibilities.push(find_best_strategy2(bp, result, &test_state));
            } else {
                // We can't build one, we'll have to wait
                possibilities.push(find_best_strategy2(bp, result, &newstate));
            }
        }
        if geode_ore_time <= geode_obsidian_time {
            // Work towards building an obsidian robot
            let (obsidian_ore_time, obsidian_clay_time) = time_to_make_obsidian_robot(bp, &newstate);
            if obsidian_ore_time >= obsidian_clay_time {
                // Work towards building an ore robot
                if newstate.ore >= bp.ore_cost {
                    // We can build one
                    let mut test_state = newstate;
                    test_state.num_ore_robots += 1;
                    test_state.ore -= bp.ore_cost;
                    possibilities.push(find_best_strategy2(bp, result, &test_state));
                } else {
                    // We can't build one, we'll have to wait
                    possibilities.push(find_best_strategy2(bp, result, &newstate));
                }
            }
            if obsidian_ore_time <= obsidian_clay_time {
                // Work towards building a clay robot
                if newstate.ore >= bp.clay_cost {
                    // We can build one
                    let mut test_state = newstate;
                    test_state.num_clay_robots += 1;
                    test_state.ore -= bp.clay_cost;
                    possibilities.push(find_best_strategy2(bp, result, &test_state));
                } else {
                    // We can't build one, we'll have to wait
                    possibilities.push(find_best_strategy2(bp, result, &newstate));
                }
            }
        }
    }

    let best = possibilities.into_iter().max().unwrap();
    result.insert(*state, best);
    best + geodes_now
}

timeit!{
fn part1(data: &Data) -> usize {
    data.par_iter()
        .map(|bp| {
            let mut cache = HashMap::new();
            let state = State {
                num_ore_robots: 1,
                num_clay_robots: 0,
                num_obsidian_robots: 0,
                num_geode_robots: 0,
                ore: 0,
                clay: 0,
                obsidian: 0,
                time_left: 24,
            };

            let geodes = find_best_strategy2(bp, &mut cache, &state);
            dbg!(geodes);
            geodes * bp.n
        })
        .sum()
}}

timeit!{
fn part2(data: &[Blueprint]) -> usize {
    data.par_iter()
        .map(|bp| {
            let mut cache = HashMap::new();
            let state = State {
                num_ore_robots: 1,
                num_clay_robots: 0,
                num_obsidian_robots: 0,
                num_geode_robots: 0,
                ore: 0,
                clay: 0,
                obsidian: 0,
                time_left: 32,
            };

            let geodes = find_best_strategy2(bp, &mut cache, &state);
            dbg!(geodes);
            geodes * bp.n
        })
        .sum()
}}

#[test]
fn test() {
    let tests = r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 33);
    assert_eq!(part2(&data[1..]), 62);
}

fn main() -> std::io::Result<()>{
    let input = get_input(19)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data[0..3]));

    Ok(())
}
