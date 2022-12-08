#[allow(unused)]
use adventofcode2022::{get_input,parse_lines};

type Data = Vec<Vec<u8>>;
fn parse_input(input: &str) -> Data {
    input.lines()
        .map(|l| l.as_bytes()
                  .iter()
                  .cloned()
                  .collect())
        .collect()
}

fn part1(data: &Data) -> usize {
    let width = data[0].len();
    let height = data.len();

    let mut visible: Vec<Vec<bool>> =
        data.iter()
            .map(|row| {
                    row.iter()
                       .map(|_| false)
                       .collect()
                })
            .collect();

    // From left
    for y in 0..height {
        let mut max = 0u8;
        for (x, &b) in data[y].iter().enumerate() {
            if b > max {
                max = b;
                visible[y][x] = true;
            }
        }
    }
    // From right
    for y in 0..height {
        let mut max = 0u8;
        for (x, &b) in data[y].iter().enumerate().rev() {
            if b > max {
                max = b;
                visible[y][x] = true;
            }
        }
    }
    // From top
    for x in 0..width {
        let mut max = 0u8;
        for y in 0..height {
            let b = data[y][x];
            if b > max {
                max = b;
                visible[y][x] = true;
            }
        }
    }
    // From bottom
    for x in 0..width {
        let mut max = 0u8;
        for y in (0..height).rev() {
            let b = data[y][x];
            if b > max {
                max = b;
                visible[y][x] = true;
            }
        }
    }
    visible.iter()
           .map(|row| row.iter().filter(|b| **b).count())
           .sum()
}

fn calc_scenic_score(data: &Data, x: usize, y: usize) -> usize {
    let start_height = data[y][x];
    let height = data.len();
    let width = data[0].len();
    let mut max = 1;
    // Look up
    {
        let mut seen = 0;
        let mut highest = 0;
        let mut ny = y;
        while ny > 0 {
            ny -= 1;
            let tree = data[ny][x];
            seen += 1;
            if tree >= highest {
                highest = tree;
            }
            if tree >= start_height {
                // No more after this.
                break;
            }
        }
        max *= seen;
    }
    // Look down
    {
        let mut seen = 0;
        let mut highest = 0;
        let mut ny = y;
        while ny+1 < height {
            ny += 1;
            let tree = data[ny][x];
            seen += 1;
            if tree >= highest {
                highest = tree;
            }
            if tree >= start_height {
                // No more after this.
                break;
            }
        }
        max *= seen;
    }
    // Look left
    {
        let mut seen = 0;
        let mut highest = 0;
        let mut nx = x;
        while nx > 0 {
            nx -= 1;
            let tree = data[y][nx];
            seen += 1;
            if tree >= highest {
                highest = tree;
            }
            if tree >= start_height {
                // No more after this.
                break;
            }
        }
        max *= seen;
    }
    // Look right
    {
        let mut seen = 0;
        let mut highest = 0;
        let mut nx = x;
        while nx+1 < width {
            nx += 1;
            let tree = data[y][nx];
            seen += 1;
            if tree >= highest {
                highest = tree;
            }
            if tree >= start_height {
                // No more after this.
                break;
            }
        }
        max *= seen;
    }
    max
}

fn part2(data: &Data) -> usize {
    let width = data[0].len();
    let height = data.len();

    (0..height).map(|y| {
        (0..width).map(|x| calc_scenic_score(data, x, y))
                  .max()
                  .unwrap()
    })
               .max().unwrap()
}

#[test]
fn test() {
    let tests = r#"30373
25512
65332
33549
35390"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 21);
    assert_eq!(part2(&data), 8);
}

fn main() -> std::io::Result<()>{
    let input = get_input(8)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
