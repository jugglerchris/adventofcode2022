use std::ops::RangeInclusive;

#[allow(unused)]
use adventofcode2022::{get_input,parse_lines,regex_parser};

#[derive(Debug,Clone)]
pub struct Sensor {
    pub x: isize,
    pub y: isize,
    pub bx: isize,
    pub by: isize,
}

regex_parser!(parse_sensor: Sensor {
    U = r#"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"# =>
        |x: isize, y: isize, bx: isize, by: isize| {
            Sensor { x, y, bx, by }
        }
});

type Data = Vec<Sensor>;
fn parse_input(input: &str) -> Data {
    parse_lines(input)
}

fn part1(data: &Data, y_to_check: isize) -> usize {
    let mut ruled_out = Vec::new();
    let mut xmin = isize::MAX;
    let mut xmax = isize::MIN;
    for sensor in data {
        let Sensor { x, y, bx, by } = sensor;
        let dist_from_check = (y - y_to_check).abs();
        let dist_to_beacon = (y - by).abs() + (x - bx).abs();
        if dist_from_check > dist_to_beacon {
            continue;
        }
        let width_of_exclusion = dist_to_beacon - dist_from_check;
        let mut min_exclusion = x - width_of_exclusion;
        let mut max_exclusion = x + width_of_exclusion;
        assert_eq!(((min_exclusion - x).abs() + (y_to_check - y).abs()), dist_to_beacon);

        if *by == y_to_check {
            if min_exclusion == *bx {
                min_exclusion += 1;
            }
            if max_exclusion == *by {
                max_exclusion -= 1;
            }
        }
        if min_exclusion <= max_exclusion {
            let full_range = RangeInclusive::new(min_exclusion, max_exclusion);
            if *by == y_to_check && full_range.contains(bx) {
                ruled_out.push(RangeInclusive::new(min_exclusion, bx-1));
                ruled_out.push(RangeInclusive::new(bx+1, max_exclusion));
            } else {
                ruled_out.push(full_range);
            }
            xmin = xmin.min(min_exclusion);
            xmax = xmax.max(max_exclusion);
        }
    }
    (xmin..=xmax).filter(|xx| ruled_out.iter()
                                    .any(|r| r.contains(xx)))
                 .count()
}
fn part2(data: &Data, maxcoord: isize) -> isize {
    for y_to_check in 0..=maxcoord {
        let mut ruled_out = Vec::new();
        for sensor in data {
            let Sensor { x, y, bx, by } = sensor;
            let dist_from_check = (y - y_to_check).abs();
            let dist_to_beacon = (y - by).abs() + (x - bx).abs();
            if dist_from_check > dist_to_beacon {
                continue;
            }
            let width_of_exclusion = dist_to_beacon - dist_from_check;
            let min_exclusion = x - width_of_exclusion;
            let max_exclusion = x + width_of_exclusion;
            assert_eq!(((min_exclusion - x).abs() + (y_to_check - y).abs()), dist_to_beacon);

            if min_exclusion <= max_exclusion {
                let full_range = RangeInclusive::new(min_exclusion, max_exclusion);
                ruled_out.push(full_range);
            }
        }
        ruled_out.sort_by_key(|r| *r.start());
        let mut min_possible_x = 0;
        for range in &ruled_out {
            if *range.start() <= min_possible_x {
                min_possible_x = min_possible_x.max(*range.end()+1);
                if min_possible_x > maxcoord {
                    break;
                }
            } else {
                dbg!("returning");
                return min_possible_x*4000000 + y_to_check;
            }
        }
    }
    unreachable!()
}

#[test]
fn test() {
    let tests = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data, 10), 26);
    assert_eq!(part2(&data, 20), 56000011);
}

fn main() -> std::io::Result<()>{
    let input = get_input(15)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data, 2000000));

    // Part 2
    println!("{}", part2(&data, 4000000));

    Ok(())
}
