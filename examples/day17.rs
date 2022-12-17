#[allow(unused)]
use adventofcode2022::{get_input,parse_lines,regex_parser};

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
enum Blow {
    Left,
    Right
}

type Data = Vec<Blow>;
fn parse_input(input: &str) -> Data {
    input.trim()
         .chars()
         .map(|c| match c {
             '<' => Blow::Left,
             '>' => Blow::Right,
             _ => panic!("Got unexpected character '{c}'"),
         })
         .collect()
}

static PIECES: [&[u8]; 5] = [
    &[0b11110000
    ],
    &[0b01000000,
      0b11100000,
      0b01000000
    ],
    &[0b11100000,
      0b00100000,
      0b00100000
    ],
    &[0b10000000,
      0b10000000,
      0b10000000,
      0b10000000
    ],
    &[0b11000000,
      0b11000000
    ]
];

fn draw_board(board: &[u8], piece: &[u8], piece_x: i32, piece_y: usize) {
    let max_y = board.len().max(piece_y + piece.len());
    eprintln!("");
    let min_y = if max_y >= 8 { max_y - 8 } else { 0 };
    for y in (min_y..=max_y).rev() {
        eprint!("|");
        for x in 0..7 {
            if y < board.len() && (board[y] << x) & 0x80 != 0 {
                eprint!("#");
            } else if (y >= piece_y) &&
                      (y < piece_y + piece.len()) &&
                      (piece[y-piece_y] >> piece_x << x) & 0x80 != 0 {
                eprint!("@");
            } else {
                eprint!(".");
            }
        }
        eprintln!("|");
    }
    eprintln!("+-------+");
}

fn part1(data: &Data) -> usize {
    let mut board: Vec<u8> = Vec::new();

    let mut moves = data.iter().cycle();
    let mut pieces = PIECES.iter().cycle();

    for _ in 0..2022 {
        let mut piece_y = board.len() + 3;
        let piece: Vec<u8> = pieces.next()
                                   .unwrap()
                                   .iter()
                                   .cloned()
                                   .collect();
        let mut piece_x = 2;
        loop {
            //draw_board(&board, &piece, piece_x, piece_y);
            match moves.next().unwrap() {
                Blow::Left => {
                    if can_move_left(&board, &piece, piece_x, piece_y) {
                        piece_x -= 1;
                    }
                }
                Blow::Right => {
                    if can_move_right(&board, &piece, piece_x, piece_y) {
                        piece_x += 1;
                    }
                }
            }
            if can_move_down(&board, &piece, piece_x, piece_y) {
                piece_y -= 1;
            } else {
                for (i, b) in piece.iter().enumerate() {
                    let y = i + piece_y;
                    if y >= board.len() {
                        assert_eq!(y, board.len());
                        board.push(b >> piece_x);
                    } else {
                        board[y] |= b >> piece_x;
                    }
                }
                break;
            }
        }
    }
    board.len()
}

fn can_move_down(board: &[u8], piece: &[u8], piece_x: i32, piece_y: usize) -> bool {
    if piece_y == 0 {
        return false;
    }
    let top = board.len();
    for (dy, b) in piece.iter().enumerate() {
        let y = piece_y + dy - 1;
        if y < top {
            if ((b >> piece_x) & board[y]) != 0 {
                return false;
            }
        }
    }
    return true;
}

fn can_move_right(board: &[u8], piece: &[u8], piece_x: i32, piece_y: usize) -> bool {
    let new_x = piece_x + 1;
    for &b in piece {
        if (b >> new_x) & 1 != 0 {
            return false;
        }
    }
    let top = board.len();
    for (dy, b) in piece.iter().enumerate() {
        if piece_y + dy < top {
            if ((b >> (piece_x+1)) & board[piece_y + dy]) != 0 {
                return false;
            }
        }
    }
    return true;
}

fn can_move_left(board: &[u8], piece: &[u8], piece_x: i32, piece_y: usize) -> bool {
    if piece_x == 0 {
        return false;
    }
    let top = board.len();
    for (dy, b) in piece.iter().enumerate() {
        if piece_y + dy < top {
            if ((b >> (piece_x-1)) & board[piece_y + dy]) != 0 {
                return false;
            }
        }
    }
    return true;
}

fn do_piece(board: &mut Vec<u8>,
            moves: &mut dyn Iterator<Item=&Blow>,
            pieces: &mut dyn Iterator<Item=&&[u8]>) {
    let mut piece_y = board.len() + 3;
    let piece: Vec<u8> = pieces.next()
        .unwrap()
        .iter()
        .cloned()
        .collect();
    let mut piece_x = 2;
    loop {
        //draw_board(&board, &piece, piece_x, piece_y);
        match moves.next().unwrap() {
            Blow::Left => {
                if can_move_left(&board, &piece, piece_x, piece_y) {
                    piece_x -= 1;
                }
            }
            Blow::Right => {
                if can_move_right(&board, &piece, piece_x, piece_y) {
                    piece_x += 1;
                }
            }
        }
        if can_move_down(&board, &piece, piece_x, piece_y) {
            piece_y -= 1;
        } else {
            for (i, b) in piece.iter().enumerate() {
                let y = i + piece_y;
                if y >= board.len() {
                    assert_eq!(y, board.len());
                    board.push(b >> piece_x);
                } else {
                    board[y] |= b >> piece_x;
                }
            }
            break;
        }
    }
}

fn part2(data: &Data) -> usize {
    let mut board: Vec<u8> = Vec::new();
    let num_blows = data.len();
    let num_pieces = PIECES.len();
    let loop_size = dbg!(num_blows) * dbg!(num_pieces);

    let wanted_pieces = 1000000000000usize;
    let repeat_start = wanted_pieces % loop_size + loop_size;

    let mut moves = data.iter().cycle();
    let mut pieces = PIECES.iter().cycle();

    let mut num_pieces = 0;
    for _ in 0..repeat_start {
        do_piece(&mut board, &mut moves, &mut pieces);
    }
    num_pieces += repeat_start;

    fn get_top_board(board: &[u8]) -> u64 {
        let blen = board.len();
        let mut result = 0u64;
        for i in (blen-8)..blen {
            result = (result << 8) | (board[i] as u64);
        }
        result
    }

    let start_state = get_top_board(&board);
    let repeat_start_count = board.len();
    let mut num_loops_to_repeat = 0;
    loop {
        for _ in 0..loop_size {
            do_piece(&mut board, &mut moves, &mut pieces);
        }
        num_loops_to_repeat += 1;
        num_pieces += loop_size;
        if get_top_board(&board) == start_state {
            break;
        }
    }
    let per_repeat_height = board.len() - repeat_start_count;
    let mega_loop = loop_size * num_loops_to_repeat;
    dbg!((loop_size, num_loops_to_repeat, mega_loop));
    let num_mega_loops = (wanted_pieces - num_pieces) / mega_loop;
    let extra = num_mega_loops * per_repeat_height;
    dbg!(extra);

    let left_over = wanted_pieces - (mega_loop * num_mega_loops) - num_pieces;
    for _ in 0..left_over {
        do_piece(&mut board, &mut moves, &mut pieces);
    }

    board.len() + extra
}

#[test]
fn test() {
    let tests = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 3068);
    assert_eq!(part2(&data), 1514285714288);
}

fn main() -> std::io::Result<()>{
    let input = get_input(17)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
