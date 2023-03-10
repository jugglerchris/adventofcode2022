use std::collections::HashSet;

#[allow(unused)]
use adventofcode2022::{get_input,parse_lines,regex_parser};

#[derive(Copy,Clone,PartialEq,Eq,Debug)]
pub enum Move {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

regex_parser!(parse_move: Move {
    U = r#"^U (\d+)"# => |n: isize| Move::Up(n),
    D = r#"^D (\d+)"# => |n: isize| Move::Down(n),
    L = r#"^L (\d+)"# => |n: isize| Move::Left(n),
    R = r#"^R (\d+)"# => |n: isize| Move::Right(n)
});

type Data = Vec<Move>;
fn parse_input(input: &str) -> Data {
    parse_lines(input)
}

fn part1(data: &Data) -> usize {
    let mut tail_seen = HashSet::new();
    tail_seen.insert((0, 0));

    let mut hx = 0isize;
    let mut hy = 0isize;
    let mut tx = 0isize;
    let mut ty = 0isize;

    for mv in data {
        let (dx, dy, &n) = match mv {
            Move::Up(n) => (0, -1, n),
            Move::Down(n) => (0, 1, n),
            Move::Left(n) => (-1, 0, n),
            Move::Right(n) => (1, 0, n),
        };
        for _ in 0..n {
            hx += dx;
            hy += dy;
            let same_x = hx == tx;
            let same_y = hy == ty;

            if (tx - hx).abs() > 1 || (ty - hy).abs() > 1 {
                // Need to do a move
                if same_y {
                    tx += if hx > tx { 1 } else { -1 };
                } else if same_x {
                    ty += if hy > ty { 1 } else { -1 };
                } else {
                    tx += if hx > tx { 1 } else { -1 };
                    ty += if hy > ty { 1 } else { -1 };
                }
            }
            tail_seen.insert((tx, ty));
        }
    }
    tail_seen.len()
}
fn part2(data: &Data) -> usize {
    let mut tail_seen = HashSet::new();
    tail_seen.insert((0, 0));

    let mut rope = vec![(0isize, 0isize);10];

    for mv in data {
        let (dx, dy, &n) = match mv {
            Move::Up(n) => (0, -1, n),
            Move::Down(n) => (0, 1, n),
            Move::Left(n) => (-1, 0, n),
            Move::Right(n) => (1, 0, n),
        };
        for _ in 0..n {
            rope[0].0 += dx;
            rope[0].1 += dy;
            for ridx in 1..10 {
                let (hx, hy) = rope[ridx-1];
                let (mut tx, mut ty) = rope[ridx];
                let same_x = hx == tx;
                let same_y = hy == ty;

                if (tx - hx).abs() > 1 || (ty - hy).abs() > 1 {
                    // Need to do a move
                    if same_y {
                        tx += if hx > tx { 1 } else { -1 };
                    } else if same_x {
                        ty += if hy > ty { 1 } else { -1 };
                    } else {
                        tx += if hx > tx { 1 } else { -1 };
                        ty += if hy > ty { 1 } else { -1 };
                    }
                }
                rope[ridx] = (tx, ty);
            }
            tail_seen.insert(rope[9]);
        }
    }
    tail_seen.len()
}

#[test]
fn test() {
    let tests = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 13);
    assert_eq!(part2(&data), 1);

    let test2 = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;
    let data2 = parse_input(&test2);
    assert_eq!(part2(&data2), 36);
}

fn main() -> std::io::Result<()>{
    let input = get_input(9)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
