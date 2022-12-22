#[allow(unused)]
use adventofcode2022::{get_input,parse_lines,regex_parser,timeit};

#[derive(Copy, Clone, Debug)]
pub enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Facing {
    pub fn turn_right(&self) -> Self {
        match self {
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
            Facing::Up => Facing::Right,
        }
    }
    pub fn turn_left(&self) -> Self {
        match self {
            Facing::Right => Facing::Up,
            Facing::Down => Facing::Right,
            Facing::Left => Facing::Down,
            Facing::Up => Facing::Left,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Space {
    Wall,
    Empty,
}

#[derive(Debug)]
struct Row {
    start: usize,
    spaces: Vec<Space>,
}

#[derive(Debug)]
struct Board {
    rows: Vec<Row>,
}

impl Board {
    pub fn has_space(&self, x: usize, y: usize) -> bool {
        if y >= self.rows.len() {
            return false;
        }
        let row = &self.rows[y];
        x >= row.start && (x-row.start) < row.spaces.len()
    }
    pub fn get_space(&self, x: usize, y: usize) -> Space {
        let row = &self.rows[y];
        row.spaces[x - row.start]
    }
    pub fn forward(&self, x: usize, y: usize, facing: Facing) -> (usize, usize) {
        match facing {
            Facing::Right => {
                let row = &self.rows[y];
                let mut newx = x+1;
                if newx >= row.start + row.spaces.len() {
                    newx = row.start;
                }
                match row.spaces[newx - row.start] {
                    Space::Wall => (x, y),
                    Space::Empty => (newx, y),
                }
            }
            Facing::Down => {
                let mut newy = y + 1;
                if newy >= self.rows.len() {
                    newy = 0;
                }
                while !self.has_space(x, newy) {
                    newy += 1;
                    if newy >= self.rows.len() {
                        newy = 0;
                    }
                }
                match self.get_space(x, newy) {
                    Space::Wall => (x, y),
                    Space::Empty => (x, newy),
                }
            }
            Facing::Left => {
                let row = &self.rows[y];
                let newx = if x > row.start {
                    x - 1
                } else {
                    row.start + row.spaces.len() - 1
                };
                match row.spaces[newx - row.start] {
                    Space::Wall => (x, y),
                    Space::Empty => (newx, y),
                }
            }
            Facing::Up => {
                let mut newy = if y == 0 {
                    self.rows.len()
                } else {
                    y-1
                };
                while !self.has_space(x, newy) {
                    newy = if newy == 0 {
                        self.rows.len()
                    } else {
                        newy-1
                    };
                }
                match self.get_space(x, newy) {
                    Space::Wall => (x, y),
                    Space::Empty => (x, newy),
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Move {
    RotateLeft,
    RotateRight,
    Forward(usize),
}

struct Input {
    board: Board,
    moves: Vec<Move>,
}

type Data = Input;
fn parse_input(input: &str) -> Data {
    let (board_s, moves_s) = input.split_once("\n\n").unwrap();

    let mut rows = vec![];
    for line in board_s.lines() {
        let mut row = vec![];
        let mut start = 0;
        for &b in line.as_bytes() {
            match b {
                b' ' => {
                    assert!(row.is_empty());
                    start += 1;
                }
                b'.' => {
                    row.push(Space::Empty);
                }
                b'#' => {
                    row.push(Space::Wall);
                }
                _ => panic!(),
            }
        }
        rows.push(Row { start, spaces: row });
    }

    let mut moves = vec![];
    let mut have_dist = false;
    let mut val = 0;
    for b in moves_s.bytes() {
        match b {
            b'0'..=b'9' => {
                have_dist = true;
                val = (val * 10) + (b - b'0') as usize;
            }
            b'L' => {
                if have_dist {
                    moves.push(Move::Forward(val));
                    have_dist = false;
                    val = 0;
                }
                moves.push(Move::RotateLeft);
            }
            b'R' => {
                if have_dist {
                    moves.push(Move::Forward(val));
                    have_dist = false;
                    val = 0;
                }
                moves.push(Move::RotateRight);
            }
            b'\n' => {}
            _ => panic!("Unexpected char '{}'", b as char),
        }
    }
    if have_dist {
        moves.push(Move::Forward(val));
    }

    Input {
        board: Board {
            rows,
        },
        moves,
    }
}

timeit!{
fn part1(data: &Data) -> usize {
    let mut x = data.board.rows[0].start;
    let mut y = 0;
    let mut facing = Facing::Right;
    for mv in &data.moves {
        match mv {
            Move::RotateLeft => {
                facing = facing.turn_left();
            }
            Move::RotateRight => {
                facing = facing.turn_right();
            }
            Move::Forward(dist) => {
                for _ in 0..*dist {
                    let (newx, newy) = data.board.forward(x, y, facing);
                    x = newx;
                    y = newy;
                }
            }
        }
    }
    ((y+1)*1000) + ((x+1) * 4) + (facing as usize)
}}
timeit!{
fn part2(data: &Data) -> usize {
    unimplemented!()
}}

#[test]
fn test() {
    let tests = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 6032);
//    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(22)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
