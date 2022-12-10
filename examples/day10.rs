#[allow(unused)]
use adventofcode2022::{get_input,parse_lines,regex_parser};

#[derive(Copy,Clone,Debug)]
pub enum Insn {
    Noop,
    Addx(isize),
}

regex_parser!(parse_insn: Insn {
    NOOP = r#"^noop$"# => | | Insn::Noop,
    ADDX = r#"^addx (-?\d+)$"# => |n: isize| Insn::Addx(n)
});

type Data = Vec<Insn>;
fn parse_input(input: &str) -> Data {
    parse_lines(input)
}

fn part1(data: &Data) -> isize {
    let mut t = 1;
    let mut x = 1;
    let mut tot_sig = 0;
    for insn in data {
        if (t % 40) == 20 {
            tot_sig += t * x;
        }
        match insn {
            Insn::Noop => {
                t += 1;
            }
            Insn::Addx(n) => {
                t += 1;
                // Check intermediate clock
                if (t % 40) == 20 {
                    tot_sig += t * x;
                }
                t += 1;
                let newx = x + n;
                x = newx;
            }
        }
    }
    tot_sig
}
fn part2(data: &Data) -> usize {
    unimplemented!()
}

#[test]
fn test() {
    let tests = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 13140);
    //assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(10)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
