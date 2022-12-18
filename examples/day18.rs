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
    let s: HashSet<Droplet> = data.iter()
                                  .cloned()
                                  .collect();

    let mut minx = isize::MAX;
    let mut maxx = isize::MIN;
    let mut miny = isize::MAX;
    let mut maxy = isize::MIN;
    let mut minz = isize::MAX;
    let mut maxz = isize::MIN;
    for Droplet(x, y, z) in data {
        minx = minx.min(*x);
        maxx = maxx.max(*x);
        miny = miny.min(*y);
        maxy = maxy.max(*y);
        minz = minz.min(*z);
        maxz = maxz.max(*z);
    }
    minx -= 1;
    maxx += 1;
    miny -= 1;
    maxy += 1;
    minz -= 1;
    maxz += 1;

    // Flood fill the outside
    let mut seen = HashSet::new();
    let mut to_visit = vec![Droplet(minx, miny, minz)];
    let mut area = 0;
    while let Some(Droplet(x, y, z)) = to_visit.pop() {
        if seen.contains(&Droplet(x, y, z)) {
            continue;
        }
        seen.insert(Droplet(x, y, z));
        //eprintln!("Looking at {:?}", Droplet(x, y, z));
        if x-1 >= minx {
            let next = Droplet(x-1, y, z );
            if s.contains(&next) {
                area += 1;
                //eprintln!("  Add one area for {:?}", next);
            } else {
                if !seen.contains(&next) {
                    to_visit.push(next);
                }
            }
        }
        if x+1 <= maxx {
            let next = Droplet(x+1, y, z);
            if s.contains(&next) {
                area += 1;
                //eprintln!("  Add one area for {:?}", &next);
            } else {
                if !seen.contains(&next) {
                    to_visit.push(next);
                }
            }
        }
        if y-1 >= miny {
            let next = Droplet(x, y-1, z);
            if s.contains(&next) {
                area += 1;
                //eprintln!("  Add one area for {:?}", &next);
            } else {
                if !seen.contains(&next) {
                    to_visit.push(next);
                }
            }
        }
        if y+1 <= maxy {
            let next = Droplet(x, y+1, z);
            if s.contains(&next) {
                area += 1;
                //eprintln!("  Add one area for {:?}", &next);
            } else {
                if !seen.contains(&next) {
                    to_visit.push(next);
                }
            }
        }
        if z-1 >= minz {
            let next = Droplet(x, y, z-1);
            if s.contains(&next) {
                area += 1;
                //eprintln!("  Add one area for {:?}", &next);
            } else {
                if !seen.contains(&next) {
                    to_visit.push(next);
                }
            }
        }
        if z+1 <= maxz {
            let next = Droplet(x, y, z+1);
            if s.contains(&next) {
                area += 1;
                //eprintln!("  Add one area for {:?}", &next);
            } else {
                if !seen.contains(&next) {
                    to_visit.push(next);
                }
            }
        }
    }
    area
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
    assert_eq!(part2(&data), 58);
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
