#[allow(unused)]
use adventofcode2022::{get_input,parse_lines,regex_parser};

#[derive(Debug,Copy,Clone)]
pub struct Move {
    count: usize,
    from: usize,
    to: usize,
}

regex_parser!(parse_move: Move {
    RE = r#"^move (\d+) from (\d+) to (\d+)"# =>
        |count: usize, from: usize, to: usize|
          Move { count, from, to }
});

type Stack = Vec<u8>;

#[derive(Debug,Clone)]
struct State {
    pub stacks: Vec<Stack>,
    pub moves: Vec<Move>,
}

impl State {
    fn run_move(&mut self, mv: &Move) {
        for _ in 0..mv.count {
            let item = self.stacks[mv.from-1].pop().unwrap();
            self.stacks[mv.to-1].push(item);
        }
    }
    fn run_move_9001(&mut self, mv: &Move) {
        let mut temp = vec![];
        for _ in 0..mv.count {
            let item = self.stacks[mv.from-1].pop().unwrap();
            temp.push(item);
        }
        for _ in 0..mv.count {
            let item = temp.pop().unwrap();
            self.stacks[mv.to-1].push(item);
        }
    }
    fn tops(&self) -> String {
        let mut result = String::new();
        for stack in &self.stacks {
            if let Some(l) = stack.last() {
                result.push(*l as char);
            }
        }
        result
    }
}

type Data = State;
fn parse_input(input: &str) -> Data {
    let (stack_string, move_string) = input.split_once("\n\n").unwrap();
    let moves = parse_lines(move_string);

    let mut stacks = vec![];
    let mut lines = stack_string.lines().collect::<Vec<_>>();
    let header = lines.pop().unwrap().as_bytes();
    assert!(header.len() > 4);
    assert_eq!(header[0], b' ');
    assert_eq!(header[1], b'1');
    let num_stacks = (header.len() + 1) / 4;
    for _ in 0..num_stacks {
        stacks.push(vec![]);
    }
    while let Some(line) = lines.pop() {
        let bytes = line.as_bytes();
        for i in 0..num_stacks {
            if bytes[i*4] == b'[' {
                stacks[i].push(bytes[i*4+1]);
            }
        }
    }

    State { stacks, moves }
}

fn part1(data: &Data) -> String {
    let mut state = data.clone();
    // Using data's moves, not state for borrowing
    for mv in &data.moves {
        state.run_move(mv);
    }
    state.tops()
}
fn part2(data: &Data) -> String {
    let mut state = data.clone();
    // Using data's moves, not state for borrowing
    for mv in &data.moves {
        state.run_move_9001(mv);
    }
    state.tops()
}

#[test]
fn test() {
    let tests = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), "CMZ");
    assert_eq!(part2(&data), "MCD");
}

fn main() -> std::io::Result<()>{
    let input = get_input(5)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
