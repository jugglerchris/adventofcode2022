#[allow(unused)]
use adventofcode2022::{get_input,parse_lines,regex_parser};
use std::ops::Range;

#[derive(Clone,Debug)]
pub struct Info(Range<usize>, Range<usize>);

regex_parser!(parse_elf: Info {
    RE = r#"^(\d+)-(\d+),(\d+)-(\d+)"# =>
        |x0: usize, x1: usize, y0: usize, y1: usize|
          Info(Range { start: x0, end: x1 },
            Range { start: y0, end: y1 })
});

type Data = Vec<Info>;
fn parse_input(input: &str) -> Data {
    parse_lines(input)
}

fn part1(data: &Data) -> usize {
    data.iter()
        .filter(|Info(r1, r2)| {
            (r1.start <= r2.start && r1.end >= r2.end) ||
            (r2.start <= r1.start && r2.end >= r1.end)
          })
        .count()
}
fn part2(data: &Data) -> usize {
    unimplemented!()
}

#[test]
fn test() {
    let tests = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 2);
    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(4)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
