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
#[derive(Copy, Clone, Debug, Eq)]
struct State2 {
    valve_idx: usize,
    valve_idx2: usize,
    closed: usize,  // Bit set of closed valves
    time_left: usize,
}

impl PartialEq for State2 {
    fn eq(&self, other: &Self) -> bool {
        (self.valve_idx == other.valve_idx && self.valve_idx2 == other.valve_idx2 && self.closed == other.closed && self.time_left == other.time_left) ||
        (self.valve_idx2 == other.valve_idx && self.valve_idx == other.valve_idx2 && self.closed == other.closed && self.time_left == other.time_left)
    }
}

impl std::hash::Hash for State2 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // don't care about order
        if self.valve_idx > self.valve_idx2 {
            self.valve_idx.hash(state);
            self.valve_idx2.hash(state);
        } else {
            self.valve_idx2.hash(state);
            self.valve_idx.hash(state);
        }
        self.closed.hash(state);
        self.time_left.hash(state);
    }
}

fn newstates(data: &Data, state: &State2) -> Vec<(usize, State2)> {
    let mut result1 = Vec::new();
    {
        if (state.closed & (1<<state.valve_idx) != 0) &&
            (data[state.valve_idx].flow > 0) {

                let mut newstate = *state;
                newstate.closed &= !(1<<state.valve_idx);
                newstate.time_left -= 1;
                let new_flow_released = newstate.time_left * data[state.valve_idx].flow;
                result1.push((new_flow_released, newstate)); 
            }
        for &otherv in &data[state.valve_idx].valves_idx {
            let mut newstate = *state;
            newstate.valve_idx = otherv;
            newstate.time_left -= 1;
            result1.push((0, newstate));
        }
    }

    let mut result2 = Vec::new();
    for (flow1, newstate1) in result1 {
        if (newstate1.closed & (1<<newstate1.valve_idx2) != 0) &&
            (data[newstate1.valve_idx2].flow > 0) {

            let mut newstate = newstate1;
            newstate.closed &= !(1<<newstate1.valve_idx2);
            let new_flow_released = newstate.time_left * data[newstate1.valve_idx2].flow;
            result2.push((new_flow_released + flow1, newstate)); 
        }
        for &otherv in &data[newstate1.valve_idx2].valves_idx {
            let mut newstate = newstate1;
            newstate.valve_idx2 = otherv;
            result2.push((flow1, newstate));
        }
    }
    result2
}

fn find_best2(data: &Data, result: &mut HashMap<State2, usize>, state: &State2) -> usize {
    if result.contains_key(state) {
        return *result.get(state).unwrap();
    }
    if state.time_left == 0 {
        return 0;
    }

    let mut possibilities = vec![];

    for (flow_released, newstate) in newstates(data, state) {
        possibilities.push(find_best2(data, result, &newstate) + flow_released);
    }
    let best = possibilities.into_iter().max().unwrap();
    result.insert(*state, best);
    best
}

fn part2(data: &Data) -> usize {
    assert_eq!(data[0].name, "AA");
    let mut useful_valves = 0;
    for (i, v) in data.iter().enumerate() {
        if v.flow > 0 {
            useful_valves |= 1<<i;
        }
    }
    let state = State2 {
        valve_idx: 0,
        valve_idx2: 0,
        closed: useful_valves,
        time_left: 26,
    };
    let mut results: HashMap<State2, usize> = HashMap::new();

    find_best2(data, &mut results, &state)
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
    assert_eq!(part2(&data), 1707);
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
