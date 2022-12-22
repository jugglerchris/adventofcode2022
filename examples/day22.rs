#[allow(unused)]
use adventofcode2022::{get_input,parse_lines,regex_parser,timeit};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    pub fn turn_around(&self) -> Self {
        match self {
            Facing::Right => Facing::Left,
            Facing::Down => Facing::Up,
            Facing::Left => Facing::Right,
            Facing::Up => Facing::Down,
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
    pub fn forward_cube(&self, x: usize, y: usize, facing: Facing) -> (Facing, usize, usize) {
        match facing {
            Facing::Right => {
                let row = &self.rows[y];
                let mut newx = x+1;
                let mut newy = y;
                let mut newfacing = facing;
                if newx >= row.start + row.spaces.len() {
                    if newx == 150 {
                        assert!(y < 50);
                        newx = 99;
                        newy = 149-y;
                        newfacing = Facing::Left;
                    } else if newx == 100 && y < 100 {
                        newfacing = Facing::Up;
                        newx = 100 + (y - 50);
                        newy = 49;
                    } else if newx == 100 && y < 150 {
                        newx = 149;
                        newy = 149 - y;
                        newfacing = Facing::Left;
                    } else if newx == 50 {
                        assert!(y >= 150);
                        newy = 149;
                        newx = 50 + (y - 150);
                        newfacing = Facing::Up;
                    }
                }
                match self.get_space(newx, newy) {
                    Space::Wall => (facing, x, y),
                    Space::Empty => (newfacing, newx, newy),
                }
            }
            Facing::Down => {
                let mut newy = y + 1;
                let mut newx = x;
                let mut newfacing = facing;
                if !self.has_space(x, newy) {
                    if newy == 50 && x >= 100 {
                        newfacing = Facing::Left;
                        newx = 99;
                        newy = (x - 100) + 50;
                    } else if x >= 50 && newy == 150 {
                        newfacing = Facing::Left;
                        newx = 49;
                        newy = 150 + (x - 50);
                    } else if newy == 200 {
                        // ??
                        newfacing = Facing::Down;
                        newy = 0;
                        newx = x + 100;
                    }
                }
                match self.get_space(newx, newy) {
                    Space::Wall => (facing, x, y),
                    Space::Empty => (newfacing, newx, newy),
                }
            }
            Facing::Left => {
                let row = &self.rows[y];
                let mut newx = x;
                let mut newy = y;
                let mut newfacing = facing;
                if x == row.start {
                    if y < 50 {
                        newx = 0;
                        newy = 149 - y;
                        newfacing = Facing::Right;
                    } else if y < 100 {
                        newy = 100;
                        newx = y - 50;
                        newfacing = Facing::Down;
                    } else if y < 150 {
                        newx = 50;
                        newy = 149 - y;
                        newfacing = Facing::Right;
                    } else {
                        newy = 0;
                        newfacing = Facing::Down;
                        newx = y - 150 + 50;
                    }
                } else {
                    newx = x - 1;
                    newy = y;
                    newfacing = facing;
                }
                match self.get_space(newx, newy) {
                    Space::Wall => (facing, x, y),
                    Space::Empty => (newfacing, newx, newy),
                }
            }
            Facing::Up => {
                let mut newx = x;
                let mut newy = y;
                let mut newfacing = facing;
                if y == 0 || !self.has_space(x, y-1) {
                    if y == 0 && x < 100 {
                        newfacing = Facing::Right;
                        newy = x-50 + 150;
                        newx = 0;
                    } else if y == 0 && x >= 100 {
                        newfacing = Facing::Up;
                        newy = 199;
                        newx = x - 100;
                    } else if y == 100 && x < 50 {
                        newfacing = Facing::Right;
                        newx = 50;
                        newy = x + 50;
                    } else {
                        unreachable!();
                    }
                } else {
                    newx = x;
                    newy = y-1;
                }
                //dbg!((newx, newy));
                match self.get_space(newx, newy) {
                    Space::Wall => (facing, x, y),
                    Space::Empty => (newfacing, newx, newy),
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
    let mut x = data.board.rows[0].start;
    let mut y = 0;
    let mut facing = Facing::Right;
    for mv in &data.moves {
        //dbg!((x, y, facing, mv));
        match mv {
            Move::RotateLeft => {
                facing = facing.turn_left();
            }
            Move::RotateRight => {
                facing = facing.turn_right();
            }
            Move::Forward(dist) => {
                for _ in 0..*dist {
                    let (newfacing, newx, newy) = (data.board.forward_cube(x, y, facing));
                    /*
                    if (x, y) != (newx, newy) {
                        dbg!((newx, newy, newfacing.turn_around()));
                        let (back, oldx, oldy) = data.board.forward_cube(newx, newy, newfacing.turn_around());
                        dbg!(newfacing.turn_around());
                        assert_eq!((oldx, oldy, back.turn_around()), (x, y, facing));
                    }
                    */
                    facing = newfacing;
                    x = newx;
                    y = newy;
                }
            }
        }
    }
    ((y+1)*1000) + ((x+1) * 4) + (facing as usize)
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
