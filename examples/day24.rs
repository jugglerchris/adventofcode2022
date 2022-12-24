use std::collections::HashSet;

#[allow(unused)]
use adventofcode2022::{get_input,parse_lines,regex_parser,timeit};

struct Valley {
    width: usize,
    height: usize,
    // initial positions of blizzards moving left etc.
    b_left: HashSet<(usize, usize)>,
    b_right: HashSet<(usize, usize)>,
    b_up: HashSet<(usize, usize)>,
    b_down: HashSet<(usize, usize)>,
}

type Data = Valley;
fn parse_input(input: &str) -> Data {
    let mut lines = input.lines();
    let mut b_left = HashSet::new();
    let mut b_right = HashSet::new();
    let mut b_up = HashSet::new();
    let mut b_down = HashSet::new();
    let line0 = lines.next().unwrap();
    assert_eq!(&line0[..3], "#.#");
    let width = line0.len() - 2;
    let mut height = 0;

    for (y, line) in lines.enumerate() {
        if &line[..3] == "###" {
            height = y;
            break;
        }
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    assert!(x == 0 || x == width+1);
                }
                '.' => {}
                '<' => {
                    b_left.insert((x-1, y));
                }
                '>' => {
                    b_right.insert((x-1, y));
                }
                '^' => {
                    b_up.insert((x-1, y));
                }
                'v' => {
                    b_down.insert((x-1, y));
                }
                _ => { unreachable!() }
            }
        }
    }
    Valley {
        width,
        height,
        b_left,
        b_right,
        b_up,
        b_down
    }
}

fn will_be_empty(data: &Data, t: usize, (x, y): (usize, usize)) -> bool {
    //eprintln!("will_be_empty(t={t} pos=({x},{y})");
    if y < 1 {
        //eprintln!("  false from y={y}");
        return false;
    }
    if data.b_left.contains(&((x + t) % data.width, y-1)) {
        //eprintln!("  false from b_left");
        return false;
    }
    let tr = t % data.width;
    if data.b_right.contains(&((x + data.width - tr) % data.width, y-1)) {
        //eprintln!("  false from b_right");
        return false;
    }
    if data.b_up.contains(&(x, (y-1 + t) % data.height)) {
        //eprintln!("  false from b_up");
        return false;
    }
    let td = t % data.height;
    if data.b_down.contains(&(x, (y-1 + data.height - td) % data.height)) {
        //eprintln!("  false from b_down");
        return false;
    }
        //eprintln!("  true");
    true
}

fn draw_board(data: &Data, t: usize, positions: &HashSet<(usize, usize)>) {
    println!("");
    if positions.contains(&(0, 0)) {
        print!("#E");
    } else {
        print!("#.");
    }
    for _ in 0..data.width {
        print!("#");
    }
    println!("");
    let mut num_blizzards = 0;
    for ym1 in 0..data.height {
        let y = ym1 + 1;
        print!("#");
        for x in 0..data.width {
            let have_left = data.b_left.contains(&((x + t) % data.width, y-1));
            let tr = t % data.width;
            let have_right = data.b_right.contains(&((x + data.width - tr) % data.width, y-1));
            let have_up = data.b_up.contains(&(x, (y-1 + t) % data.height));
            let td = t % data.height;
            let have_down = data.b_down.contains(&(x, (y-1 + data.height - td) % data.height));
            let num_things = have_left as usize + have_right as usize +
                             have_up as usize + have_down as usize;
            num_blizzards += num_things;
            if num_things == 0 {
                if positions.contains(&(x, y)) {
                    print!("E");
                } else {
                    print!(".");
                }
            } else if positions.contains(&(x, y)) {
                print!("X");
            } else if num_things > 1 {
                print!("{}", num_things);
            } else if have_left {
                print!("<");
            } else if have_right {
                print!(">");
            } else if have_up {
                print!("^");
            } else if have_down {
                print!("v");
            } else {
                unreachable!()
            }
        }
        println!("#");
    }
    for _ in 0..data.width {
        print!("#");
    }
    println!(".#");
    println!("blizzards: {num_blizzards}");
}

fn find_route(data: &Data, t0: usize, start: (usize, usize), end: (usize, usize)) -> usize
{
    let mut t = t0;
    // Positions here have the starting place at (0, 0),
    // so the left wall is at x=-1 and the top wall at y=0.
    let mut positions = HashSet::new();
    positions.insert(start);

    loop {
        //draw_board(data, t, &positions);
        let mut new_positions = HashSet::new();
        /*
        for (xx, yy) in &positions {
            eprintln!("({xx},{yy})");
        }
        */
        for (x, y) in positions.into_iter() {
            if y == 0 {
                // Special case from the top
                if will_be_empty(data, t+1, (0, 1)) {
                    new_positions.insert((0, 1));
                }
                // Can also stay put.
                new_positions.insert((0, 0));
            } else if (x, y) == end {
                return t;
            } else {
                if will_be_empty(data, t+1, (x, y)) {
                    new_positions.insert((x, y));
                }
                if will_be_empty(data, t+1, (x, y-1)) {
                    new_positions.insert((x, y-1));
                }
                if x > 0 && will_be_empty(data, t+1, (x-1, y)) {
                    new_positions.insert((x-1, y));
                }
                if (x < data.width-1) && will_be_empty(data, t+1, (x+1, y)) {
                    new_positions.insert((x+1, y));
                }
                if (y+1 <= data.height) && will_be_empty(data, t+1, (x, y+1)) {
                    new_positions.insert((x, y+1));
                }
            }
        }
        assert!(new_positions.len() > 0);
        positions = new_positions;
        t += 1;
    }
}

timeit!{
fn part1(data: &Data) -> usize {
    find_route(data, 0, (0, 0), (data.width-1, data.height)) + 1
}}
timeit!{
fn part2(data: &Data) -> usize {
    let t1 = find_route(data, 0, (0, 0), (data.width-1, data.height)) + 1;
    let t2 = find_route(data, t1, (data.width-1, data.height+1), (0, 1)) + 1;
    find_route(data, t2, (0, 0), (data.width-1, data.height)) + 1
}}

#[test]
fn test() {
    let tests = r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 18);
    assert_eq!(part2(&data), 54);
}

fn main() -> std::io::Result<()>{
    let input = get_input(24)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
