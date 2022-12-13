#[allow(unused)]
use adventofcode2022::{get_input,parse_lines,regex_parser};

#[derive(Clone)]
enum Num {
    Int(usize),
    List(Vec<Num>),
}

impl std::fmt::Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Num::Int(n) => {
                write!(f, "{}", n)?;
            }
            Num::List(l) => {
                write!(f, "[")?;
                for n in l {
                    write!(f, "{}", n)?;
                    write!(f, ",")?;
                }
                write!(f, "]")?;
            }
        }
        Ok(())
    }
}

type Data = Vec<(Num, Num)>;

fn parse_list(line: &[u8]) -> (Vec<Num>, usize) {
    let mut l = Vec::new();
    let mut pos = 0;
    while line[pos] != b']' {
        match line[pos] {
            b'[' | b'0'..=b'9' => {
                let (n, bytes_used) = parse_num(&line[pos..]);
                l.push(n);
                pos += bytes_used;
            }
            b',' => {
                pos += 1;
            }
            _ => panic!("Error: parse_list found {:?}", line[pos])
        }
    }
    (l, pos)
}

fn parse_num(line: &[u8]) -> (Num, usize) {
    let mut pos = 0;
    while pos < line.len() {
        match line[pos] {
            b'[' => {
                let (l, bytes_used) = parse_list(&line[pos+1..]);
                assert_eq!(line[pos+1+bytes_used], b']');
                return (Num::List(l), bytes_used+2);
            }
            b'0'..=b'9' => {
                let mut v = 0usize;
                loop {
                    let b = line[pos];
                    if b >= b'0' && b <= b'9' {
                        v = (v * 10) + (b - b'0') as usize;
                        pos += 1;
                    } else {
                        return (Num::Int(v), pos);
                    }
                }
            }
            _ => panic!("Error: parse_num found {:?}", line[pos])
        }
    }
    unreachable!()
}

fn parse_line(line: &[u8]) -> Num {
    parse_num(line).0
}

fn parse_input(input: &str) -> Data {
    let mut result = Vec::new();
    for pairs in input.split("\n\n") {
        let mut lines = pairs.lines();
        let n1 = parse_line(lines.next().unwrap().as_bytes());
        let n2 = parse_line(lines.next().unwrap().as_bytes());
        assert!(lines.next().is_none());
        result.push((n1, n2));
    }
    result
}

fn eq(a: &Num, b: &Num) -> bool {
    use Num::*;
    match (a, b) {
        (Int(n1), Int(n2)) => n1 == n2,
        (an@Int(_), bl) => eq(&List(vec![an.clone()]), bl),
        (al, bn@Int(_)) => eq(al, &List(vec![bn.clone()])),
        (List(al), List(bl)) => {
            if al.len() != bl.len() {
                return false;
            }
            for (aa, bb) in al.iter().zip(bl.iter()) {
                if !eq(aa, bb) {
                    return false;
                }
            }
            true
        }
    }
}

fn lt(a: &Num, b: &Num) -> bool {
    use Num::*;
    match (a, b) {
        (Int(n1), Int(n2)) => n1 < n2,
        (an@Int(_), bl) => lt(&List(vec![an.clone()]), &bl),
        (al, bn@Int(_)) => lt(&al, &List(vec![bn.clone()])),
        (List(al), List(bl)) => {
            let common = al.len().min(bl.len());
            for i in 0..common {
                if eq(&al[i], &bl[i]) {
                    continue;
                }
                return lt(&al[i], &bl[i]);
            }
            al.len() < bl.len()
        }
    }
}

fn part1(data: &Data) -> usize {
    let mut sum = 0;
    for (i, (a, b)) in data.iter().enumerate() {
        if lt(a, b) {
            sum += i+1;
        }
    }
    sum
}
fn part2(data: &Data) -> usize {
    unimplemented!()
}

#[test]
fn test() {
    let tests = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;
    let data = parse_input(&tests);
    /*
    for (a, b) in &data {
        println!("a={}", a);
        println!("b={}", b);
    }
    */

    assert_eq!(part1(&data), 13);
    //assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(13)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
