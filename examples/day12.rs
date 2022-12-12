#[allow(unused)]
use adventofcode2022::{get_input,parse_lines,regex_parser};

#[derive(Debug)]
struct Data {
    start: (i32, i32),
    end: (i32, i32),

    map: Vec<Vec<u8>>,
}

fn parse_input(input: &str) -> Data {
    let mut map = Vec::new();
    let mut start = None;
    let mut end = None;

    for (y, line) in input.lines().enumerate() {
        let mut rowv = Vec::new();
        for (x, &b) in line.as_bytes().iter().enumerate() {
            let height = match b {
                b'a'..=b'z' => { b - b'a' }
                b'S' => {
                    start = Some((x as i32, y as i32));
                    0
                }
                b'E' => {
                    end = Some((x as i32, y as i32));
                    25
                }
                _ => panic!()
            };
            rowv.push(height);
        }
        map.push(rowv);
    }
    let start = start.unwrap();
    let end = end.unwrap();
    Data {
        start,
        end,
        map
    }
}

fn part1(data: &Data) -> usize {
    let mut dists = data.map.iter()
                        .map(|v| v.iter().map(|_| None).collect::<Vec<Option<usize>>>())
                        .collect::<Vec<_>>();
    let mut work = vec![((data.start.0 as usize, data.start.1 as usize), 0)];

    while !work.is_empty() {
        let ((x, y), dist) = work.pop().unwrap();
        let height = data.map[y][x];
        let olddist = dists[y][x];
        if olddist.is_none() || olddist.unwrap() > dist {
            // Better path
            dists[y][x] = Some(dist);
            if y > 0 && data.map[y-1][x] <= (height+1) {
                work.push(((x, y-1), dist+1));
            }
            if y+1 < data.map.len() && data.map[y+1][x] <= (height+1) {
                work.push(((x, y+1), dist+1));
            }
            if x > 0 && data.map[y][x-1] <= (height+1) {
                work.push(((x-1, y), dist+1));
            }
            if x+1 < data.map[0].len() && data.map[y][x+1] <= (height+1) {
                work.push(((x+1, y), dist+1));
            }
        }
    }
    dists[data.end.1 as usize][data.end.0 as usize].unwrap()
}
fn part2(data: &Data) -> usize {
    unimplemented!()
}

#[test]
fn test() {
    let tests = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 31);
    //assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(12)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
