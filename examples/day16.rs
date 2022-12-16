use std::collections::HashMap;

#[allow(unused)]
use adventofcode2022::{get_input,parse_lines,regex_parser};

#[derive(Clone,Debug)]
pub struct Valve {
    name: String,
    flow: usize,
    valves: Vec<String>,
    index: usize,
    valves_idx: Vec<usize>,
}

regex_parser!(parse_valve: Valve {
    U = r#"^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ([\w, ]+)$"# =>
        |name: String, flow: usize, names: String| {
            let valves = names.split(", ")
                              .map(Into::into)
                              .collect();
            let index = 0;
            let valves_idx = vec![];
            Valve { name, flow, valves, index, valves_idx }
        }
});

type Data = Vec<Valve>;
fn parse_input(input: &str) -> Data {
    let mut vs = parse_lines(input);
    vs.sort_by_key(|v: &Valve| v.name.clone());
    let mut name_to_idx = HashMap::new();
    for (i, v) in vs.iter_mut().enumerate() {
        (*v).index = i;
        name_to_idx.insert(v.name.clone(), i);
    }
    for v in vs.iter_mut() {
        for name in &v.valves {
            v.valves_idx.push(*name_to_idx.get(name).unwrap());
        }
    }
    vs
}

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
struct State {
    valve_idx: usize,
    open: usize,  // Bit set of open valves
    time_left: usize,
}

fn find_best_strategy(data: &Data, result: &mut HashMap<State, usize>, state: &State) -> usize {
    if result.contains_key(state) {
        return *result.get(state).unwrap();
    }
    if state.time_left == 0 {
        return 0;
    }

    let mut possibilities = vec![];
    // Otherwise try things.
    if ((state.open & (1<<state.valve_idx)) == 0) && (data[state.valve_idx].flow > 0) {
        let mut newstate = *state;
        newstate.open |= 1<<state.valve_idx;
        newstate.time_left -= 1;
        let new_flow_released = newstate.time_left * data[state.valve_idx].flow;
        possibilities.push(find_best_strategy(data, result, &newstate) + new_flow_released);
    }
    for &otherv in &data[state.valve_idx].valves_idx {
        let mut newstate = *state;
        newstate.time_left -= 1;
        newstate.valve_idx = otherv;
        possibilities.push(find_best_strategy(data, result, &newstate));
    }
    let best = possibilities.into_iter().max().unwrap();
    result.insert(*state, best);
    best
}

fn part1(data: &Data) -> usize {
    assert_eq!(data[0].name, "AA");
    let state = State {
        valve_idx: 0,
        open: 0,
        time_left: 30,
    };
    let mut results: HashMap<State, usize> = HashMap::new();

    find_best_strategy(data, &mut results, &state)
}
fn part2(data: &Data) -> usize {
    unimplemented!()
}

#[test]
fn test() {
    let tests = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 1651);
//    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(16)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
