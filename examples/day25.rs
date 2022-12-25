#[allow(unused)]
use adventofcode2022::{get_input,parse_lines,regex_parser,timeit};

fn from_snafu(s: &str) -> isize {
    let mut result = 0;
    for &b in s.as_bytes() {
        let digit = match b {
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            b'-' => -1,
            b'=' => -2,
            _ => panic!(),
        };
        result = result * 5 + digit;
    }

    result
}

fn to_snafu(mut v: isize) -> String {
    let mut digits = vec![];
    while v != 0 {
        let (digit, c) = match v % 5 {
            2 | -3 => (2, '2'),
            1 | -4 => (1, '1'),
            0  => (0, '0'),
            -1 | 4 => (-1, '-'),
            -2 | 3 => (-2, '='),
            _ => panic!(),
        };
        v = (v - digit) / 5;
        digits.push(c);
    }
    let mut result = String::new();
    for c in digits.into_iter().rev() {
        result.push(c);
    }
    result
}

type Data = Vec<isize>;
fn parse_input(input: &str) -> Data {
    input.lines()
         .map(from_snafu)
         .collect()
}

timeit!{
fn part1(data: &Data) -> String {
    to_snafu(data.iter().sum())
}}
timeit!{
fn part2(data: &Data) -> usize {
    unimplemented!()
}}

#[test]
fn test() {
    let tests = r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), "2=-1=0");
    //assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(25)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
