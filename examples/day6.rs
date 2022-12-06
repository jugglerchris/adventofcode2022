#[allow(unused)]
use adventofcode2022::{get_input,parse_lines};

type Data = String;
fn parse_input(input: &str) -> Data {
    input.into()
}

fn find_start_length(data: &Data, length: usize) -> usize {
    let bytes = data.as_bytes();
    let mut l = 0;
    for chunk in bytes.windows(length) {
        let mut bits = 0usize;
        for &b in chunk {
            bits |= 1<<((b - b'a') as usize)
        }
        if bits.count_ones() == length as u32 {
            return l+length;
        }
        l += 1;
    }
    panic!()
}

fn part1(data: &Data) -> usize {
    find_start_length(data, 4)
}
fn part2(data: &Data) -> usize {
    find_start_length(data, 14)
}

#[test]
fn test() {
    let tests = &[
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
    ];

    for &(s, res, res2) in tests {
        let data = parse_input(s);
        assert_eq!(part1(&data), res);
        assert_eq!(part2(&data), res2);
    }
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
