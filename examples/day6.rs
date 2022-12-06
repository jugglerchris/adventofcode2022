#[allow(unused)]
use adventofcode2022::{get_input,parse_lines};

type Data = String;
fn parse_input(input: &str) -> Data {
    input.into()
}

fn part1(data: &Data) -> usize {
    let bytes = data.as_bytes();
    let mut l = 0;
    for chunk in bytes.windows(4) {
        let mut bits = 0usize;
        for &b in chunk {
            bits |= 1<<((b - b'a') as usize)
        }
        if bits.count_ones() == 4 {
            return l+4;
        }
        l += 1;
    }
    panic!()
}
fn part2(data: &Data) -> usize {
    unimplemented!()
}

#[test]
fn test() {
    let tests = &[
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 0),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 0),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 0),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 0),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 0),
    ];

    for &(s, res, _) in tests {
        let data = parse_input(s);
        assert_eq!(part1(&data), res);
    }
    //assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(6)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
