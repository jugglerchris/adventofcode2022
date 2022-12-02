#[allow(unused)]
use adventofcode2022::{get_input,parse_lines};

enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone)]
enum Outcome {
    Win,
    Draw,
    Lose
}

impl Move {
    fn beats(&self, other: Move) -> Outcome {
        use Move::*;
        use Outcome::*;
        match (self, other) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Lose,
            (Rock, Scissors) => Win,
            (Paper, Rock) => Win,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Lose,
            (Scissors, Rock) => Lose,
            (Scissors, Paper) => Win,
            (Scissors, Scissors) => Draw,
        }
    }
}

type Data = Vec<(u8, u8)>;
fn parse_input(input: &str) -> Data {
    let mut result = Vec::new();
    for line in input.lines() {
        let bline = line.as_bytes();
        result.push((bline[0], bline[2]));
    }
    result
}

fn part1(data: &Data) -> usize {
    use Move::*;
    let mut score = 0usize;
    for &(a, b) in data {
        let theirs = match a {
            b'A' => Rock,
            b'B' => Paper,
            b'C' => Scissors,
            _ => panic!(),
        };
        let mine = match b {
            b'X' => Rock,
            b'Y' => Paper,
            b'Z' => Scissors,
            _ => panic!(),
        };

        use Outcome::*;
        let score1 = match mine.beats(theirs) {
            Win => 6,
            Draw => 3,
            Lose => 0,
        };
        let score2 = match mine {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };
        score += score1 + score2;
    }
    score
}
fn part2(data: &Data) -> usize {
    use Move::*;
    use Outcome::*;
    let mut score = 0usize;
    for &(a, b) in data {
        let theirs = match a {
            b'A' => Rock,
            b'B' => Paper,
            b'C' => Scissors,
            _ => panic!(),
        };
        let outcome = match b {
            b'X' => Lose,
            b'Y' => Draw,
            b'Z' => Win,
            _ => panic!(),
        };
        let mine = match (theirs, outcome) {
            (Rock, Lose) => Scissors,
            (Rock, Draw) => Rock,
            (Rock, Win) => Paper,
            (Paper, Lose) => Rock,
            (Paper, Draw) => Paper,
            (Paper, Win) => Scissors,
            (Scissors, Lose) => Paper,
            (Scissors, Draw) => Scissors,
            (Scissors, Win) => Rock,
        };

        let score1 = match outcome {
            Win => 6,
            Draw => 3,
            Lose => 0,
        };
        let score2 = match mine {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };
        score += score1 + score2;
    }
    score
}

#[test]
fn test() {
    let tests = r#"A Y
B X
C Z"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 15);
    assert_eq!(part2(&data), 12);
}

fn main() -> std::io::Result<()>{
    let input = get_input(2)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
