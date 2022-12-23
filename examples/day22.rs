use std::{collections::HashMap, ops::Mul};

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

#[derive(Clone, Eq, PartialEq)]
struct Matrix {
    m: [i8; 9],
}

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
struct Vector {
    v: [i8; 3],
}

impl std::fmt::Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Matrix").field("m", &self.m).finish()
    }
}

impl Matrix {
    pub fn identity() -> Matrix {
        Matrix {
            m: [1, 0, 0,
                0, 1, 0,
                0, 0, 1],
        }
    }

    pub fn fold_right(&self) -> Matrix {
        self * &Matrix {
            m: [0, 0, 1,
                0, 1, 0,
                -1, 0, 0,
            ]
        }
    }

    pub fn fold_left(&self) -> Matrix {
        self * &Matrix {
            m: [0, 0, -1,
                0, 1, 0,
                1, 0, 0,
            ]
        }
    }

    pub fn fold_down(&self) -> Matrix {
        self * &Matrix {
            m: [1, 0, 0,
            0, 0, 1,
            0, -1, 0,
            ]
        }
    }
}

impl Vector {
    pub fn new(x: i8, y: i8, z: i8) -> Vector {
        Vector {
            v: [x, y, z],
        }
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        let mut m: [i8;9] = Default::default();
        for y in 0..3 {
            for x in 0..3 {
                let mut acc = 0;
                for i in 0..3 {
                    acc += rhs.m[i*3 + x] * self.m[y*3 + i];
                }
                m[y*3+x] = acc;
            }
        }
        Matrix { m }
    }
}

impl Mul<Vector> for &Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let mut v: [i8;3] = Default::default();
        for y in 0..3 {
            let mut acc = 0;
            for i in 0..3 {
                acc += rhs.v[i] * self.m[y*3 + i];
            }
            v[y] = acc;
        }
        Vector { v }
    }
}

#[test]
fn test_vector() {
    let iden = Matrix::identity();
    assert_eq!(&iden * Vector::new(-1, -1, 1), Vector::new(-1, -1, 1));
    assert_eq!(&iden * Vector::new(1, 1, 1), Vector::new(1, 1, 1));
    assert_eq!(&iden * Vector::new(-1, 1, 1), Vector::new(-1, 1, 1));
    assert_eq!(&iden * Vector::new(1, -1, 1), Vector::new(1, -1, 1));

    let fr = iden.fold_right();
    assert_eq!(&fr * Vector::new(-1, -1, 1), Vector::new(1, -1, 1));
    assert_eq!(&fr * Vector::new(1, 1, 1), Vector::new(1, 1, -1));
    assert_eq!(&fr * Vector::new(-1, 1, 1), Vector::new(1, 1, 1));
    assert_eq!(&fr * Vector::new(1, -1, 1), Vector::new(1, -1, -1));

    let fl = iden.fold_left();
    assert_eq!(&fl * Vector::new(-1, -1, 1), Vector::new(-1, -1, -1));
    assert_eq!(&fl * Vector::new(1, 1, 1), Vector::new(-1, 1, 1));
    assert_eq!(&fl * Vector::new(-1, 1, 1), Vector::new(-1, 1, -1));
    assert_eq!(&fl * Vector::new(1, -1, 1), Vector::new(-1, -1, 1));

    let fd = iden.fold_down();
    assert_eq!(&fd * Vector::new(-1, -1, 1), Vector::new(-1, 1, 1));
    assert_eq!(&fd * Vector::new(1, 1, 1), Vector::new(1, 1, -1));
    assert_eq!(&fd * Vector::new(-1, 1, 1), Vector::new(-1, 1, -1));
    assert_eq!(&fd * Vector::new(1, -1, 1), Vector::new(1, 1, 1));

    assert_eq!(&(&(&fd * &fd) * &fd) * &fd, iden);
    assert_eq!(&(&(&fl * &fl) * &fl) * &fl, iden);
    assert_eq!(&(&(&fr * &fr) * &fr) * &fr, iden);
}

struct CubeMap<'d> {
    data: &'d Data,
    side_len: usize,
    // transformation matrix for each square, indexed by top-left
    square_mappings: HashMap<(usize, usize), Matrix>,
    // Corner labels from normalized corner coordinates
    coord_to_corner_name: HashMap<Vector, u8>,
    // Map from pairs of coordinate labels in left,right order as you enter the square, to 
    // the corresponding corner coordinates on the flat map and the direction you face
    // to enter it.
    edge_to_locations: HashMap<(u8, u8), ((usize, usize), (usize, usize), Facing)>,
}

impl<'d> CubeMap<'d> {
    pub fn new(data: &'d Data) -> CubeMap<'d> {
        let side_len =
            data.board
            .rows
            .iter()
            .map(|r| r.spaces.len())
            .min()
            .unwrap();

        let x = data.board.rows[0].start;
        let y = 0;
        let matrix = Matrix::identity();

        let mut work = vec![(x, y, matrix)];
        let mut square_mappings = HashMap::new();

        let coord_to_corner_name: HashMap<Vector, u8> =
            [(Vector::new(-1, -1, 1), b'A'),
            (Vector::new(1, -1, 1), b'B'),
            (Vector::new(-1, 1, 1), b'C'),
            (Vector::new(1, 1, 1), b'D'),
            (Vector::new(-1, -1, -1), b'E'),
            (Vector::new(1, -1, -1), b'F'),
            (Vector::new(-1, 1, -1), b'G'),
            (Vector::new(1, 1, -1), b'H'),
            ].into_iter()
                .collect();

        let mut edge_to_locations = HashMap::new();

        while let Some((x, y, matrix)) = work.pop() {
            if square_mappings.contains_key(&(x, y)) {
                assert_eq!(square_mappings.get(&(x, y)).unwrap(), &matrix);
                continue;
            }
            square_mappings.insert((x, y), matrix.clone());
            let tl = &matrix * Vector::new(-1, -1, 1);
            let tr = &matrix * Vector::new(1, -1, 1);
            let bl = &matrix * Vector::new(-1, 1, 1);
            let br = &matrix * Vector::new(1, 1, 1);
            let tl = *coord_to_corner_name.get(&tl).unwrap();
            let tlc = (x, y);
            let tr = *coord_to_corner_name.get(&tr).unwrap();
            let trc = (x+side_len-1, y);
            let bl = *coord_to_corner_name.get(&bl).unwrap();
            let blc = (x, y+side_len-1);
            let br = *coord_to_corner_name.get(&br).unwrap();
            let brc = (x+side_len-1, y+side_len-1);

            edge_to_locations.insert((tr, tl), (trc, tlc, Facing::Down));
            edge_to_locations.insert((br, tr), (brc, trc, Facing::Left));
            edge_to_locations.insert((bl, br), (blc, brc, Facing::Up));
            edge_to_locations.insert((tl, bl), (tlc, blc, Facing::Right));

            if data.board.has_space(x+side_len, y) {
                work.push((x+side_len, y, matrix.fold_right()));
            }
            if data.board.has_space(x, y+side_len) {
                work.push((x, y+side_len, matrix.fold_down()));
            }
            if x > 0 && data.board.has_space(x-side_len, y) {
                work.push((x-side_len, y, matrix.fold_left()));
            }
        }

        CubeMap {
            data,
            side_len,
            square_mappings,
            coord_to_corner_name,
            edge_to_locations
        }
    }

    #[cfg(test)]
    pub fn print_plain(&self) {
        for (y, row) in self.data.board.rows.iter().enumerate() {
            for _ in 0..row.start {
                print!(" ");
            }
            for (ox, sp) in row.spaces.iter().enumerate() {
                let x = ox + row.start;
                let local_x = x % self.side_len;
                let local_y = y % self.side_len;
                if (local_x == 0 || local_x == self.side_len-1) &&
                    (local_y == 0 || local_y == self.side_len-1) {
                        let sq_x = x-(x%self.side_len);
                        let sq_y = y-(y%self.side_len);
                        let mat = self.square_mappings.get(&(sq_x, sq_y)).unwrap();
                        let x3 = if local_x == 0 { -1 } else { 1 };
                        let y3 = if local_y == 0 { -1 } else { 1 };
                        let coord = mat * Vector::new(x3, y3, 1);
                        print!("{}", *self.coord_to_corner_name.get(&coord).unwrap() as char);
                    } else {
                        match sp {
                            Space::Wall => print!("#"),
                            Space::Empty => print!("."),
                        }
                    }
            }
            println!("");
        }
    }

    fn map_edge_pos(&self, offset: usize, start: (usize, usize), end: (usize, usize)) -> (usize, usize)
    {
        if start.0 > end.0 {
            (start.0 - offset, start.1)
        } else if start.0 < end.0 {
            (start.0 + offset, start.1)
        } else if start.1 > end.1 {
            (start.0, start.1 - offset)
        } else if start.1 < end.1 {
            (start.0, start.1 + offset)
        } else {
            unreachable!()
        }
    }

    pub fn forward(&self, x: usize, y: usize, facing: Facing) -> (Facing, usize, usize) {
        let board = &self.data.board;
        match facing {
            Facing::Right => {
                let row = &board.rows[y];
                let mut newx = x+1;
                let mut newy = y;
                let mut newfacing = facing;
                if newx >= row.start + row.spaces.len() {
                    let matrix = self.get_matrix((x, y));
                    let edge_end0 = matrix * Vector::new(1, -1, 1);
                    let edge_end1 = matrix * Vector::new(1, 1, 1);
                    let cornerl = *self.coord_to_corner_name.get(&edge_end0).unwrap();
                    let cornerr = *self.coord_to_corner_name.get(&edge_end1).unwrap();
                    let (el, er, newfacing2) = self.edge_to_locations.get(&(cornerl, cornerr)).unwrap().clone();
                    newfacing = newfacing2;
                    let (newx2, newy2) = self.map_edge_pos(y % self.side_len, el, er);
                    newx = newx2;
                    newy = newy2;
                }
                match board.get_space(newx, newy) {
                    Space::Wall => (facing, x, y),
                    Space::Empty => (newfacing, newx, newy),
                }
            }
            Facing::Down => {
                let mut newy = y + 1;
                let mut newx = x;
                let mut newfacing = facing;
                if !board.has_space(x, newy) {
                    let matrix = self.get_matrix((x, y));
                    let edge_end0 = matrix * Vector::new(1, 1, 1);
                    let edge_end1 = matrix * Vector::new(-1, 1, 1);
                    let cornerl = *self.coord_to_corner_name.get(&edge_end0).unwrap();
                    let cornerr = *self.coord_to_corner_name.get(&edge_end1).unwrap();
                    let (el, er, newfacing2) = self.edge_to_locations.get(&(cornerl, cornerr)).unwrap().clone();
                    newfacing = newfacing2;
                    let (newx2, newy2) = self.map_edge_pos(self.side_len - 1 - (x % self.side_len), el, er);
                    newx = newx2;
                    newy = newy2;
                }
                match board.get_space(newx, newy) {
                    Space::Wall => (facing, x, y),
                    Space::Empty => (newfacing, newx, newy),
                }
            }
            Facing::Left => {
                let row = &board.rows[y];
                let newx;
                let newy;
                let newfacing;
                if x == row.start {
                    let matrix = self.get_matrix((x, y));
                    let edge_end0 = matrix * Vector::new(-1, 1, 1);
                    let edge_end1 = matrix * Vector::new(-1, -1, 1);
                    let cornerl = *self.coord_to_corner_name.get(&edge_end0).unwrap();
                    let cornerr = *self.coord_to_corner_name.get(&edge_end1).unwrap();
                    let (el, er, newfacing2) = self.edge_to_locations.get(&(cornerl, cornerr)).unwrap().clone();
                    newfacing = newfacing2;
                    let (newx2, newy2) = self.map_edge_pos(self.side_len - 1 - (y % self.side_len), el, er);
                    newx = newx2;
                    newy = newy2;
                } else {
                    newx = x - 1;
                    newy = y;
                    newfacing = facing;
                }
                match board.get_space(newx, newy) {
                    Space::Wall => (facing, x, y),
                    Space::Empty => (newfacing, newx, newy),
                }
            }
            Facing::Up => {
                let newx;
                let newy;
                let mut newfacing = facing;
                if y == 0 || !board.has_space(x, y-1) {
                    let matrix = self.get_matrix((x, y));
                    let edge_end0 = matrix * Vector::new(-1, 1, 1);
                    let edge_end1 = matrix * Vector::new(1, 1, 1);
                    let cornerl = *self.coord_to_corner_name.get(&edge_end0).unwrap();
                    let cornerr = *self.coord_to_corner_name.get(&edge_end1).unwrap();
                    let (el, er, newfacing2) = self.edge_to_locations.get(&(cornerl, cornerr)).unwrap().clone();
                    newfacing = newfacing2;
                    let (newx2, newy2) = self.map_edge_pos(x % self.side_len, el, er);
                    newx = newx2;
                    newy = newy2;
                } else {
                    newx = x;
                    newy = y-1;
                }
                match board.get_space(newx, newy) {
                    Space::Wall => (facing, x, y),
                    Space::Empty => (newfacing, newx, newy),
                }
            }
        }
    }

    fn get_matrix(&self, (x, y): (usize, usize)) -> &Matrix {
        let ox = x - (x % self.side_len);
        let oy = y - (y % self.side_len);
        self.square_mappings.get(&(ox, oy)).unwrap()
    }
}

timeit!{
    fn part2(data: &Data) -> usize {
        let cube_map = CubeMap::new(data);
        #[cfg(test)]
        cube_map.print_plain();
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
                    let (newfacing, newx, newy) = cube_map.forward(x, y, facing);
                    x = newx;
                    y = newy;
                    facing = newfacing;
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
    assert_eq!(part2(&data), 5031);
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
