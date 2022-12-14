use std::collections::HashSet;

#[allow(unused)]
use adventofcode2022::{get_input,parse_lines,regex_parser};

// Positions of rock
type Data = HashSet<(i32, i32)>; 

fn parse_input(input: &str) -> Data {
    let mut result = HashSet::new();
    for line in input.lines() {
        let mut prev = None;
        for pair in line.split(" -> ") {
            let coords = pair.split(",")
                             .map(|s| s.parse().unwrap())
                             .collect::<Vec<i32>>();
            assert_eq!(coords.len(), 2);
            let x = coords[0];
            let y = coords[1];

            if let Some((prevx, prevy)) = prev {
                if x == prevx {
                    let miny = y.min(prevy);
                    let maxy = y.max(prevy);
                    for y in miny..=maxy {
                        result.insert((x, y));
                    }
                } else if y == prevy {
                    let minx = x.min(prevx);
                    let maxx = x.max(prevx);
                    for x in minx..=maxx {
                        result.insert((x, y));
                    }
                } else {
                    panic!();
                }
            }
            prev = Some((x, y));
        }
    }
    result
}

fn part1(data: &Data) -> usize {
    let maxy = data.iter()
                   .map(|(_, y)| *y)
                   .max()
                   .unwrap();
    let mut cave = data.clone();
    loop {
        // Drop from (500,0)
        let (mut x, mut y) = (500, 0);
        loop {
            if y > maxy {
                return cave.len() - data.len();
            } else if !cave.contains(&(x, y+1)) {
                y += 1;
                continue;
            } else if !cave.contains(&(x-1, y+1)) {
                x -= 1;
                y += 1;
                continue;
            } else if !cave.contains(&(x+1, y+1)) {
                x += 1;
                y += 1;
                continue;
            } else {
                // Stuck
                cave.insert((x, y));
                break;
            }
        }
    }
}
fn part2(data: &Data) -> usize {
    unimplemented!()
}

#[test]
fn test() {
    let tests = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 24);
   // assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(14)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
