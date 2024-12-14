use std::io::Read;
use std::fs::File;
use std::collections::HashSet;

fn read_input(fname: &str) -> Vec<Vec<u32>> {
    let mut cont = String::new();
    File::open(fname).expect("Bad open.").read_to_string(&mut cont).expect("Can't read file.");

    cont.lines().map(|x| x.chars().map(|x| x.to_digit(10).expect("Invalid digit in input file")).collect::<Vec<_>>()).collect()
}

fn find_starts(board: &Vec<Vec<u32>>) -> Vec<(u32, u32)> {
    let mut res: Vec<(u32, u32)> = vec!();
    for (y, row) in board.into_iter().enumerate() {
        for (x, level) in row.into_iter().enumerate() {
            if *level == 0 {
                res.push((x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
    }
    res
}

fn validate_and_insert(board: &Vec<Vec<u32>>, loc: Option<(usize, usize)>, front: &mut Vec<(u32,u32)>, level: u32) {
    let real_loc = match loc {
        Some(t) => t,
        None => {return;}
    };
        
    let x = real_loc.0;
    let y = real_loc.1;
    if x >= board[0].len() {
        return;
    }
    if y >= board.len() {
        return;
    }
    
    let new_level = board[y][x];
    if new_level == level+1 {
        front.push((x.try_into().unwrap(), y.try_into().unwrap()));
    }
}

fn num_paths(board: &Vec<Vec<u32>>, origin: &(u32, u32)) -> u32 {
    let mut visited: HashSet<(u32,u32)> = HashSet::new();
    let mut frontier: Vec<(u32, u32)> = vec!();
    frontier.push(origin.clone());

    let mut num_paths = 0;
    
    while frontier.len() > 0 {
        let cur_point = frontier.pop().unwrap();
        if visited.contains(&cur_point) {
            continue;
        }
        
        let (x, y): (u32, u32) = cur_point;
        let x_i: usize = x.try_into().unwrap();
        let y_i: usize = y.try_into().unwrap();
        
        if board[y_i][x_i] == 9 {
            num_paths += 1;
            continue;
        }
        //visited.insert(cur_point);

        let cur_level = board[y_i][x_i];
        // check n s e w
        let up    = Some((x_i, y_i + 1));
        let down  = match y_i {
            0 => None,
            _ => Some((x_i, y_i - 1))
        };
        let left  = match x_i {
            0 => None,
            _ => Some((x_i - 1, y_i))
        };
        let right = Some((x_i + 1, y_i));

        validate_and_insert(board, up,    &mut frontier, cur_level);
        validate_and_insert(board, down,  &mut frontier, cur_level);
        validate_and_insert(board, left,  &mut frontier, cur_level);
        validate_and_insert(board, right, &mut frontier, cur_level);
    }

    num_paths
}

fn main() {
    let board = read_input("input.txt");
    let starts = find_starts(&board);
    let mut p1 = 0;
    for i in starts {
        println!("{}", num_paths(&board, &i));
        p1 += num_paths(&board, &i);
    }
    println!("part1: {}", p1);
}
