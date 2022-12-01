#[allow(unused)]
use adventofcode2022::{get_input,parse_lines};

type Data = Vec<Vec<usize>>;
fn parse_input(input: &str) -> Data {
    let mut data = vec![vec![]];
    for line in input.lines() {
        if line == "" {
            data.push(vec![]);
        } else {
            let val: usize = line.parse().unwrap();
            data.last_mut().unwrap().push(val);
        }
    }
    data
}

fn part1(data: &Data) -> usize {
    data.iter()
        .map(|v| v.iter().sum())
        .max()
        .unwrap()
}
fn part2(data: &Data) -> usize {
    let mut summed = data.iter()
                         .map(|v| v.iter().sum())
                         .collect::<Vec<usize>>();
    summed.sort_by(|a, b| Ord::cmp(b, a));
    summed[0..=2].iter().sum()
}

#[test]
fn test() {
    let tests = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 24000);
    assert_eq!(part2(&data), 45000);
}

fn main() -> std::io::Result<()>{
    let input = get_input(1)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
