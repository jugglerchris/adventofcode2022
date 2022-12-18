use std::collections::HashSet;

#[allow(unused)]
use adventofcode2022::{get_input,parse_lines,regex_parser,timeit};

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
pub struct Droplet(isize, isize, isize);

regex_parser!(parse_droplet: Droplet {
    U = r#"^(-?\d+),(-?\d+),(-?\d+)$"# =>
        |x: isize, y: isize, z: isize| Droplet(x, y, z)
});

type Data = Vec<Droplet>;
fn parse_input(input: &str) -> Data {
    parse_lines(input)
}

timeit!{
fn part1(data: &Data) -> usize {
    let s: HashSet<Droplet> = data.iter()
                                  .cloned()
                                  .collect();

    let mut count = 0;
    for Droplet(x, y, z) in &s {
        if !s.contains(&Droplet(*x-1, *y, *z)) {
            count += 1;
        }
        if !s.contains(&Droplet(*x, *y-1, *z)) {
            count += 1;
        }
        if !s.contains(&Droplet(*x, *y, *z-1)) {
            count += 1;
        }
        if !s.contains(&Droplet(*x+1, *y, *z)) {
            count += 1;
        }
        if !s.contains(&Droplet(*x, *y+1, *z)) {
            count += 1;
        }
        if !s.contains(&Droplet(*x, *y, *z+1)) {
            count += 1;
        }
    }
    count
}}
timeit!{
fn part2(data: &Data) -> usize {
    unimplemented!()
}}

#[test]
fn test() {
    let tests = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 64);
//    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(18)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
