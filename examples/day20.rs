#[allow(unused)]
use adventofcode2022::{get_input,parse_lines,regex_parser,timeit};
use std::collections::VecDeque;

type Data = Vec<isize>;
fn parse_input(input: &str) -> Data {
    parse_lines(input)
}

timeit!{
fn part1(data: &Data) -> isize {
    let tot = data.len();
    // orig_pos, val
    let mut list: VecDeque<(usize, isize)> =
        data.iter()
            .cloned()
            .enumerate()
            .collect();
    for orig_pos in 0..tot {
        let v = data[orig_pos];
        let pos = list.iter()
                   .enumerate()
                   .find(|(_pos, (op, _))| *op == orig_pos)
                   .unwrap()
                   .0;
        //dbg!((v, pos));
        let av = v.abs() as usize;
        let mut a = pos;
        for _ in 0..av {
            //dbg!(a);
            if v >= 0 {
                // pos
                let b = if a == tot-1 { 0 } else { a+1 };
                list.swap(a, b);
                a = b;
            } else {
                // neg
                let b = if a == 0 { tot-1 } else { a-1 };
                list.swap(a, b);
                a = b;
            }
        }
        //dbg!(list.iter().map(|(_, v)| v).collect::<Vec<_>>());
    }
    //dbg!(data);
    //dbg!(&index);
    let pos = list.iter()
               .enumerate()
               .find(|(_pos, (_, v))| *v == 0)
               .unwrap().0;
    //dbg!(pos);
    list.get((1000 + pos) % tot).unwrap().1 +
    list.get((2000 + pos) % tot).unwrap().1 +
    list.get((3000 + pos) % tot).unwrap().1
}}
timeit!{
fn part2(data: &Data) -> isize {
    let tot = data.len();
    // orig_pos, val
    let mut list: VecDeque<(usize, isize)> =
        data.iter()
            .map(|v| v*811589153)
            .enumerate()
            .collect();
    for _ in 0..10 {
        for orig_pos in 0..tot {
            let (pos, (_, v)) = list.iter()
                .enumerate()
                .find(|(_pos, (op, _val))| *op == orig_pos)
                .unwrap();
            let v = *v;
            //dbg!((v, pos));
            let av = (v.abs() as usize) % (tot-1);
            let mut a = pos;
            for _ in 0..av {
                //dbg!(a);
                if v >= 0 {
                    // pos
                    if a == tot-2 {
                        list.swap(tot-2, tot-1);
                        let tmp = list.pop_back().unwrap();
                        list.push_front(tmp);
                        a = 0;
                    } else if a == tot-1 {
                        let tmp = list.pop_back().unwrap();
                        list.push_front(tmp);
                        list.swap(0, 1);
                        a = 1;
                    } else {
                        list.swap(a, a+1);
                        a += 1;
                    }
                } else {
                    // neg
                    if a == 1 {
                        list.swap(0, 1);
                        let tmp = list.pop_front().unwrap();
                        list.push_back(tmp);
                        a = tot-1;
                    } else if a == 0 {
                        let tmp = list.pop_front().unwrap();
                        list.push_back(tmp);
                        list.swap(tot-1, tot-2);
                        a = tot-2;
                    } else {
                        list.swap(a, a-1);
                        a -= 1;
                    }
                }
            }
            //dbg!(list.iter().map(|(_, v)| v).collect::<Vec<_>>());
        }
    }
    //dbg!(data);
    //dbg!(&index);
    let pos = list.iter()
               .enumerate()
               .find(|(_pos, (_, v))| *v == 0)
               .unwrap().0;
    //dbg!(pos);
    list.get((1000 + pos) % tot).unwrap().1 +
    list.get((2000 + pos) % tot).unwrap().1 +
    list.get((3000 + pos) % tot).unwrap().1
}}

#[test]
fn test() {
    let tests = r#"1
2
-3
3
-2
0
4"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 3);
    assert_eq!(part2(&data), 1623178306);
}

fn main() -> std::io::Result<()>{
    let input = get_input(20)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
