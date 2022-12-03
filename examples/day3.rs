#[allow(unused)]
use adventofcode2022::{get_input,parse_lines};
use std::collections::HashSet;

fn letter_to_priority(l: &u8) -> u8 {
    match l {
        b'a'..=b'z' => l-b'a'+1,
        b'A'..=b'Z' => l-b'A'+27,
        _ => panic!(),
    }
}

type Data = Vec<(HashSet<u8>, HashSet<u8>)>;
fn parse_input(input: &str) -> Data {
    let mut result = Vec::new();
    for line in input.lines() {
        let bytes = line.as_bytes();
        let size = bytes.len()/2;
        let set1 = bytes[0..size].iter().map(letter_to_priority).collect();
        let set2 = bytes[size..].iter().map(letter_to_priority).collect();
        result.push((set1, set2));
    }
    result
}

fn part1(data: &Data) -> usize {
    let mut sum = 0;
    for (a, b) in data {
        let intersec = a.intersection(b).cloned().collect::<Vec<u8>>();
        assert_eq!(intersec.len(), 1);
        sum += intersec[0] as usize;
    }
    sum
}
fn part2(data: &Data) -> usize {
    unimplemented!()
}

#[test]
fn test() {
    let tests = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 156);
    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(3)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
