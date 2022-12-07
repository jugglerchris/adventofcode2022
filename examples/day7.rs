#![allow(unused_braces)]
#[allow(unused)]
use adventofcode2022::{get_input,parse_lines,regex_parser};
use std::collections::HashMap;

#[derive(Clone, Debug,PartialEq,Eq)]
pub enum Command {
    CdRoot,
    Cd(String),
    CdUp,
    Ls,
    File(usize, String),
    DirEntry(String),
}

use Command::*;

regex_parser!(parse_command: Command {
    CDR = r#"^\$ cd /"# => | | { CdRoot },
    CDUP = r#"^\$ cd \.\."# => | | { Command::CdUp },
    CD = r#"^\$ cd (\w+)"# => |dir: String| { Cd(dir) },
    LS = r#"^\$ ls"# => | | { Ls },
    FILE = r#"^(\d+) (\w+)"# => |size: usize, file: String| { File(size, file) },
    DIR = r#"^dir (\w+)"# => |name: String| { DirEntry(name) }
});

type Data = Vec<Command>;
fn parse_input(input: &str) -> Data {
    parse_lines(input)
}

type Files = HashMap<String, usize>;
type Subdirs = HashMap<String, usize>;

#[derive(Clone, Debug)]
struct Dir {
    name: String,
    files: Files,
    subdirs: Subdirs,
    contained_size: Option<usize>,
}

impl Dir {
    fn new(name: String) -> Dir {
        Dir {
            name,
            files: Default::default(),
            subdirs: Default::default(),
            contained_size: None,
        }
    }
}

fn traverse(data: &Data) -> Vec<Dir> {
    assert_eq!(data[0], CdRoot);
    let mut dirs = vec![
        Dir::new("".into())
    ];
    let mut cwd = vec![0];

    for cmd in &data[1..] {
        match cmd {
            CdRoot => panic!(),
            Cd(name) => {
                let curdir = *cwd.last().unwrap();
                if !dirs[curdir].subdirs.contains_key(name) {
                    panic!()
                }
                cwd.push(*dirs[curdir].subdirs.get(name).unwrap());
            }
            CdUp => { cwd.pop(); },
            Ls => {},
            File(size, name) => {
                let curdir = *cwd.last().unwrap();
                if !dirs[curdir].files.contains_key(name) {
                    dirs[curdir].files.insert(name.into(), *size);
                }
            }
            DirEntry(name) => {
                let curdir = *cwd.last().unwrap();
                if !dirs[curdir].subdirs.contains_key(name) {
                    let dirnum = dirs.len();
                    dirs.push(Dir::new(name.into()));
                    dirs[curdir].subdirs.insert(name.into(), dirnum);
                }
            }
        }
    }
    dirs
}

fn calc_dir_sizes(dirs: &mut Vec<Dir>, start: usize) {
    if let Some(_) = dirs[start].contained_size {
        return;
    }
    let mut sum = 0usize;
    for fsize in dirs[start].files.values() {
        sum += fsize;
    }
    for subdir in dirs[start].subdirs.values().cloned().collect::<Vec<_>>() {
        calc_dir_sizes(dirs, subdir);
        sum += dirs[subdir].contained_size.unwrap();
    }
    dirs[start].contained_size = Some(sum);
}

fn part1(data: &Data) -> usize {
    let mut dirs = traverse(data);
    calc_dir_sizes(&mut dirs, 0);

    dirs.iter()
        .filter(|d| d.contained_size.unwrap() <= 100000)
        .map(|d| d.contained_size.unwrap())
        .sum()
}

fn part2(data: &Data) -> usize {
    unimplemented!()
}

#[test]
fn test() {
    let tests = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 95437);
//    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(7)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
