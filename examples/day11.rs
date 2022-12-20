#[allow(unused)]
use adventofcode2022::{get_input,parse_lines,regex_parser};

use std::collections::VecDeque;
use regex::Regex;

type Item = isize;

#[derive(Debug, Copy, Clone)]
enum Arg {
    Const(isize),
    Old,
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Times(Arg),
    Add(Arg),
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<Item>,
    op: Operation,
    divisor: isize,
    throw_true: usize,
    throw_false: usize,
    inspects: usize,
}

fn parse_monkey(input: &str) -> Monkey {
    let re = Regex::new(r"(?m)Monkey (\d+):
  Starting items: ([0-9, ]*)
  Operation: new = old ([*+]) (old|\d+)
  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)").unwrap();

    let caps = re.captures(input).unwrap();
    let item_str = &caps[2];
    let items: VecDeque<Item> =
        item_str.split(", ")
                .map(|s| s.parse().unwrap())
                .collect();
    let arg = match &caps[4] {
        "old" => Arg::Old,
        ns => Arg::Const(ns.parse().unwrap()),
    };
    let op = match &caps[3] {
        "*" => Operation::Times(arg),
        "+" => Operation::Add(arg),
        _ => panic!(),
    };
    let divisor = caps[5].parse().unwrap();
    let throw_true = caps[6].parse().unwrap();
    let throw_false = caps[7].parse().unwrap();
    Monkey {
        items,
        op,
        divisor,
        throw_true,
        throw_false,
        inspects: 0,
    }
}

type Data = Vec<Monkey>;
fn parse_input(input: &str) -> Data {
    input.split("\n\n")
         .map(parse_monkey)
         .collect()
}

fn part1(data: &Data) -> usize {
    let mut monkeys: Vec<Monkey> = data.clone();
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.iter().cloned().collect::<Vec<_>>();
            monkeys[i].items.clear();
            for mut item in items {
                use Operation::*;
                use Arg::*;
                match monkeys[i].op {
                    Times(Old) => { item *= item; }
                    Times(Const(n)) => { item *= n; }
                    Add(Old) => { item += item; }
                    Add(Const(n)) => { item += n; }
                }
                item /= 3;
                let other = if (item % monkeys[i].divisor) == 0 {
                    monkeys[i].throw_true
                } else {
                    monkeys[i].throw_false
                };
                monkeys[other].items.push_back(item);
                monkeys[i].inspects += 1;
            }
        }
    }
    let mut inspects =
        monkeys.iter()
               .map(|m| m.inspects)
               .collect::<Vec<_>>();
    inspects.sort_by(|a, b| Ord::cmp(b, a));
    inspects[0] * inspects[1]
}
fn part2(data: &Data) -> usize {
    let mut monkeys: Vec<Monkey> = data.clone();
    let modulus: Item = monkeys.iter()
                               .map(|m| m.divisor)
                               .product();
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.iter().cloned().collect::<Vec<_>>();
            monkeys[i].items.clear();
            for mut item in items {
                use Operation::*;
                use Arg::*;
                match monkeys[i].op {
                    Times(Old) => { item *= item; }
                    Times(Const(n)) => { item *= n; }
                    Add(Old) => { item += item; }
                    Add(Const(n)) => { item += n; }
                }
                item = item % modulus;
                let other = if (item % monkeys[i].divisor) == 0 {
                    monkeys[i].throw_true
                } else {
                    monkeys[i].throw_false
                };
                monkeys[other].items.push_back(item);
                monkeys[i].inspects += 1;
            }
        }
    }
    let mut inspects =
        monkeys.iter()
               .map(|m| m.inspects)
               .collect::<Vec<_>>();
    inspects.sort_by(|a, b| Ord::cmp(b, a));
    inspects[0] * inspects[1]
}

#[test]
fn test() {
    let tests = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 10605);
    assert_eq!(part2(&data), 2713310158);
}

fn main() -> std::io::Result<()>{
    let input = get_input(11)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
