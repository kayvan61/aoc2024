use std::io;
use std::fs::File;
use std::io::Read;

fn read_game(fname: &str) -> io::Result<Vec<Vec<char>>> {
    let mut f = File::open(fname)?;
    let mut fconts = String::new();

    f.read_to_string(&mut fconts)?;
    io::Result::Ok(fconts.lines().map(|x| x.chars().collect()).collect())
}

fn check_xmas(board: &Vec<Vec<char>>, orig: (i32, i32), dir: (i32, i32)) -> bool {
    let mut cur_pos = orig.clone();
    let board_x_size: i32 = board.len().try_into().unwrap();
    let board_y_size: i32 = board[0].len().try_into().unwrap();
    for target in "XMAS".chars() {
        // bounds check
        if cur_pos.0 >= board_x_size || cur_pos.0 < 0 {
            return false;
        }
        if cur_pos.1 >= board_y_size || cur_pos.1 < 0 {
            return false;
        }

        let board_x: usize = cur_pos.0.try_into().unwrap();
        let board_y: usize = cur_pos.1.try_into().unwrap(); 
        if board[board_x][board_y] != target {
            return false;
        }
        cur_pos.0 += dir.0;
        cur_pos.1 += dir.1;
    }
    true
}

// check every letter around the orig, and its opposite
fn check_diag_mas(board: &Vec<Vec<char>>, orig: (i32, i32)) -> bool {
    if board[TryInto::<usize>::try_into(orig.0).unwrap()][TryInto::<usize>::try_into(orig.1).unwrap()] != 'A' {
        return false;
    }
    let all_dirs = [(1,-1), (1,1), (-1,-1), (-1,1)];
    let mut num_cross: i32 = 0;
    for dir in all_dirs {
        let m_x: usize = match (orig.0 + dir.0).try_into() {
                Ok(t) => t,
                Err(_) => continue,
        };
        let m_y: usize = match (orig.1 + dir.1).try_into() {
                Ok(t) => t,
                Err(_) => continue,
        };
        if m_y >= board[0].len() || m_x >= board.len() {
            continue;
        }
        if board[m_x][m_y] == 'M' {
            let s_x: usize = match (orig.0 - dir.0).try_into() {
                Ok(t) => t,
                Err(_) => continue,
            };
            let s_y: usize = match (orig.1 - dir.1).try_into() {
                Ok(t) => t,
                Err(_) => continue,
            };
            if s_y >= board[0].len() || s_x >= board.len() {
                continue;
            }
            if board[s_x][s_y] == 'S' {
                num_cross += 1;
            }
        }
    }
    num_cross >= 2
}

fn main() {
    let board = match read_game("input.txt") {
        Ok(t) => t,
        Err(why) => panic!("err reading file {}", why),
    };

    let h = board.len();
    let w = board[0].len();

    let mut part1 = 0;
    let mut part2 = 0;
    for i in 0..h {
        for j in 0..w {
            let x:i32 = i.try_into().unwrap();
            let y:i32 = j.try_into().unwrap();
            for dir in [(0,-1), (0,1), (1,0), (1,-1), (1,1), (-1,0), (-1,-1), (-1,1)] {
                if check_xmas(&board, (x,y), dir) {
                    part1 += 1;
                }
            }
            if check_diag_mas(&board, (x,y)) {
                part2 += 1;
            }
        }
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
