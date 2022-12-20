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
    // Orig position to new position
    let mut index: VecDeque<usize> = (0..tot).collect();
    for orig_pos in 0..tot {
        let v = data[orig_pos];
        let is_pos = v >= 0;
        let av = v.abs() as usize;
        let start = *index.get(orig_pos % tot).unwrap();
        let mut a = start;
        for _ in 0..av {
            dbg!(a);
            if is_pos {
                if a == tot-1 {
                    // First move it to the start
                    let tmp = index.pop_back().unwrap();
                    index.push_front(tmp);
                    // And swap it
                    index.swap(0, 1);
                    a = 1;
                } else if a == tot-2 {
                    index.swap(tot-2, tot-1);
                    let tmp = index.pop_back().unwrap();
                    index.push_front(tmp);
                    a = 0;
                } else {
                    index.swap(a, a+1);
                    a+= 1;
                }
            } else {
                if a == 1 {
                    index.swap(0, 1);
                    let tmp = index.pop_front().unwrap();
                    index.push_back(tmp);
                    a = tot-1;
                } else if a == 0 {
                    let tmp = index.pop_front().unwrap();
                    index.push_back(tmp);
                    index.swap(tot-2, tot-1);
                    a = tot-2;
                } else {
                    index.swap(a, a-1);
                    a -= 1;
                }
            }
            let y = index.iter()
                .map(|i| data[*i])
                .collect::<Vec<_>>();
            dbg!((v, y, &index));
        }
        let x = index.iter()
            .map(|i| data[*i])
            .collect::<Vec<_>>();
        dbg!((v, x));
    }
    //dbg!(data);
    //dbg!(&index);
    data[index[999]] + data[index[1999]] + data[index[2999]]
}}
timeit!{
fn part2(data: &Data) -> isize {
    unimplemented!()
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
//    assert_eq!(part2(&data), 0);
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
