use std::collections::{HashSet, HashMap};

#[allow(unused)]
use adventofcode2022::{get_input,parse_lines,regex_parser,timeit};

type Data = Vec<Vec<bool>>;
fn parse_input(input: &str) -> Data {
    let mut result = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.as_bytes() {
            row.push(match c {
                b'.' => false,
                b'#' => true,
                _ => panic!(),
            });
        }
        result.push(row);
    }
    result
}

fn propose_dest(dests: &mut HashMap<(isize, isize), usize>,
                moves: &mut Vec<((isize, isize), (isize, isize))>,
                from: (isize, isize),
                to: (isize, isize))
{
    moves.push((from, to));
    *dests.entry(to)
         .or_default()
         += 1;
}

#[cfg(test)]
fn dump_board(elves: &HashSet<(isize, isize)>) {
    let mut minx = isize::MAX;
    let mut maxx = isize::MIN;
    let mut miny = isize::MAX;
    let mut maxy = isize::MIN;
    for &(x, y) in elves {
        minx = minx.min(x);
        maxx = maxx.max(x);
        miny = miny.min(y);
        maxy = maxy.max(y);
    }

    println!("");
    for y in miny..=maxy {
        for x in minx..=maxx {
            if elves.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn count_empty_spaces(elves: &HashSet<(isize, isize)>) -> usize {
    let mut minx = isize::MAX;
    let mut maxx = isize::MIN;
    let mut miny = isize::MAX;
    let mut maxy = isize::MIN;
    for &(x, y) in elves {
        minx = minx.min(x);
        maxx = maxx.max(x);
        miny = miny.min(y);
        maxy = maxy.max(y);
    }

    ((maxx - minx + 1) * (maxy - miny + 1)) as usize - elves.len()
}

timeit!{
fn part1(data: &Data) -> usize {
    let mut elves = HashSet::new();
    for y in 0..data.len() {
        for (x, &is_elf) in data[y].iter().enumerate() {
            if is_elf {
                elves.insert((x as isize, y as isize));
            }
        }
    }

    let mut first_direction = 0;
    for _ in 0..10 {
        #[cfg(test)]
        dump_board(&elves);
        let mut proposed_dests = HashMap::new();
        let mut moves = Vec::new();
        for &(x, y) in &elves {
            let mut neighbours = 0;
            for xx in (x-1)..=(x+1) {
                for yy in (y-1)..=(y+1) {
                    if elves.contains(&(xx, yy)) {
                        neighbours += 1;
                    }
                }
            }
            if neighbours == 1 {
                // Just the elf
                continue;
            }

            for dir in 0..4 {
                match (first_direction + dir) & 0x3 {
                    0 => {
                        if !elves.contains(&(x-1, y-1)) &&
                           !elves.contains(&(x, y-1)) &&
                           !elves.contains(&(x+1, y-1)) {
                           propose_dest(&mut proposed_dests,
                                        &mut moves,
                                        (x, y), (x, y-1));
                           break;
                        }
                    }
                    1 => {
                        if !elves.contains(&(x-1, y+1)) &&
                           !elves.contains(&(x, y+1)) &&
                           !elves.contains(&(x+1, y+1)) {
                           propose_dest(&mut proposed_dests,
                                        &mut moves,
                                        (x, y), (x, y+1));
                           break;
                        }
                    }
                    2 => {
                        if !elves.contains(&(x-1, y-1)) &&
                           !elves.contains(&(x-1, y)) &&
                           !elves.contains(&(x-1, y+1)) {
                           propose_dest(&mut proposed_dests,
                                        &mut moves,
                                        (x, y), (x-1, y));
                           break;
                        }
                    }
                    3 => {
                        if !elves.contains(&(x+1, y-1)) &&
                           !elves.contains(&(x+1, y)) &&
                           !elves.contains(&(x+1, y+1)) {
                           propose_dest(&mut proposed_dests,
                                        &mut moves,
                                        (x, y), (x+1, y));
                           break;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        first_direction = (first_direction + 1) & 0x3;

        for (from, to) in moves {
            if *proposed_dests.get(&to).unwrap() == 1 {
                elves.insert(to);
                elves.remove(&from);
            }
        }
    }
    #[cfg(test)]
    dump_board(&elves);
    count_empty_spaces(&elves)
}}
timeit!{
fn part2(data: &Data) -> usize {
    let mut elves = HashSet::new();
    for y in 0..data.len() {
        for (x, &is_elf) in data[y].iter().enumerate() {
            if is_elf {
                elves.insert((x as isize, y as isize));
            }
        }
    }

    let mut first_direction = 0;
    for i in 1.. {
        let mut proposed_dests = HashMap::new();
        let mut moves = Vec::new();
        let mut have_moved = false;
        for &(x, y) in &elves {
            let mut neighbours = 0;
            for xx in (x-1)..=(x+1) {
                for yy in (y-1)..=(y+1) {
                    if elves.contains(&(xx, yy)) {
                        neighbours += 1;
                    }
                }
            }
            if neighbours == 1 {
                // Just the elf
                continue;
            }

            for dir in 0..4 {
                match (first_direction + dir) & 0x3 {
                    0 => {
                        if !elves.contains(&(x-1, y-1)) &&
                           !elves.contains(&(x, y-1)) &&
                           !elves.contains(&(x+1, y-1)) {
                           propose_dest(&mut proposed_dests,
                                        &mut moves,
                                        (x, y), (x, y-1));
                           break;
                        }
                    }
                    1 => {
                        if !elves.contains(&(x-1, y+1)) &&
                           !elves.contains(&(x, y+1)) &&
                           !elves.contains(&(x+1, y+1)) {
                           propose_dest(&mut proposed_dests,
                                        &mut moves,
                                        (x, y), (x, y+1));
                           break;
                        }
                    }
                    2 => {
                        if !elves.contains(&(x-1, y-1)) &&
                           !elves.contains(&(x-1, y)) &&
                           !elves.contains(&(x-1, y+1)) {
                           propose_dest(&mut proposed_dests,
                                        &mut moves,
                                        (x, y), (x-1, y));
                           break;
                        }
                    }
                    3 => {
                        if !elves.contains(&(x+1, y-1)) &&
                           !elves.contains(&(x+1, y)) &&
                           !elves.contains(&(x+1, y+1)) {
                           propose_dest(&mut proposed_dests,
                                        &mut moves,
                                        (x, y), (x+1, y));
                           break;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        first_direction = (first_direction + 1) & 0x3;

        for (from, to) in moves {
            if *proposed_dests.get(&to).unwrap() == 1 {
                elves.insert(to);
                elves.remove(&from);
                have_moved = true;
            }
        }
        if !have_moved {
            return i;
        }
    }
    unreachable!()
}}

#[test]
fn test() {
    let tests = r#"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 110);
    assert_eq!(part2(&data), 20);
}

fn main() -> std::io::Result<()>{
    let input = get_input(23)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
