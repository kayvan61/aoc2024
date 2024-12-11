use std::io;
use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn read_input(fname: &str) -> io::Result<Vec<Vec<char>>> {
    let mut f = File::open(fname)?;
    let mut fcont = String::new();

    f.read_to_string(&mut fcont)?;

    Ok(fcont.lines().map(|x| x.chars().collect()).collect())
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Loc {
    x: i32,
    y: i32,
    dir: Dir,
}

#[derive(Clone, Debug)]
enum Valid<T> {
    Yes(T),
    No,
}

fn find_player(board: &Vec<Vec<char>>) -> Option<Loc> {
    for (y,row) in board.iter().enumerate() {
        for (x,col) in row.iter().enumerate() {
            if *col == '>' {
                return Some( Loc {x: x.try_into().unwrap(),
                                  y: y.try_into().unwrap(),
                                  dir:Dir::RIGHT});
            }
            else if *col == '<' {
                return Some( Loc {x: x.try_into().unwrap(),
                                  y: y.try_into().unwrap(),
                                  dir:Dir::LEFT});
            }
            else if *col == '^' {
                return Some( Loc {x: x.try_into().unwrap(),
                                  y: y.try_into().unwrap(),
                                  dir:Dir::UP});
            }
            else if *col == 'v' {
                return Some( Loc {x: x.try_into().unwrap(),
                                  y :y.try_into().unwrap(),
                                  dir:Dir::DOWN});
            }
        }
    }
    None
}

// returns if location is valid. if it is valid, then return if its an obstruction
fn valid_loc (board: &Vec<Vec<char>>, origin: &Loc) -> Valid::<bool> {
    let y_max:i32 = board.len().try_into().unwrap();
    let x_max:i32 = board[0].len().try_into().unwrap();
    if origin.y >= y_max || origin.x >= x_max {
        return Valid::No;
    }
    if origin.y < 0 || origin.x < 0 {
        return Valid::No;
    }
    let x_cord:usize = origin.x.try_into().unwrap();
    let y_cord:usize = origin.y.try_into().unwrap();
    Valid::Yes(board[y_cord][x_cord] != '#')
}

fn walk(origin: &Loc) -> Loc {
    match origin.clone() {
        Loc {x:a, y:b, dir:Dir::UP}    => Loc {x: a,   y:b-1, dir:Dir::UP},
        Loc {x:a, y:b, dir:Dir::DOWN}  => Loc {x: a,   y:b+1, dir:Dir::DOWN},
        Loc {x:a, y:b, dir:Dir::LEFT}  => Loc {x: a-1, y:b,   dir:Dir::LEFT},
        Loc {x:a, y:b, dir:Dir::RIGHT} => Loc {x: a+1, y:b,   dir:Dir::RIGHT},
    }
}

fn turn_right(dir: Dir) -> Dir {
    match dir {
        Dir::UP    => Dir::RIGHT,
        Dir::DOWN  => Dir::LEFT,
        Dir::LEFT  => Dir::UP,
        Dir::RIGHT => Dir::DOWN,
    }
}

fn advance_board(board: &Vec<Vec<char>>, origin: Loc) -> Valid::<Loc> {
    let next_loc = walk(&origin);
    let v = valid_loc(board, &next_loc);
    match v {
        Valid::Yes(t) => match t {
            true  => Valid::Yes(next_loc),
            false => Valid::Yes(Loc{x:origin.x, y:origin.y, dir:turn_right(origin.dir)}),
        },
        Valid::No     => Valid::No,
    }
}

fn simulate(board: &Vec<Vec<char>>, origin: &Loc) -> HashSet<(i32, i32)> {
    let mut cur_loc = origin.clone();
    let mut points: HashSet<(i32, i32)> = HashSet::new();
    points.insert((cur_loc.x, cur_loc.y));
    loop {
        match advance_board(board, cur_loc) {
            Valid::No     => {return points;},
            Valid::Yes(t) => {points.insert((t.x, t.y)); cur_loc=t;}
        }
    }
}

fn simulate_with_dir(board: &Vec<Vec<char>>, origin: &Loc) -> i32 {
    let mut cur_loc = origin.clone();
    let mut points: HashSet<Loc> = HashSet::new();
    let mut blocks: HashSet<Loc> = HashSet::new();
    points.insert(cur_loc.clone());
    loop {
        // check if adding a block infront of us will cause a loop
        let mut spec_loc = origin.clone();
        let mut spec_path: Vec<Loc> = vec!();

        // fuck it lmao. make a new board and add the block
        // set to up always so location is all that matters
        let mut spec_block = walk(&cur_loc);
        spec_block.dir = Dir::UP;
        let mut spec_board = board.clone();

        // check if we can add a thing in front of
        match valid_loc(&spec_board, &spec_block) {
            Valid::Yes(t) => {
                match t {
                    true => {
                        let board_cord: (usize, usize) = (spec_block.y.try_into().unwrap(), spec_block.x.try_into().unwrap());
                        spec_board[board_cord.0][board_cord.1] = '#';
                        true
                    }
                    _ => {false} // already barrier
                }
            }
            _ => {false} // out of bounds
        };
        loop {
            match advance_board(&spec_board, spec_loc) {
                Valid::Yes(t) => {
                    // in the board, check for loop in speculative path
                    if spec_path.contains(&t) {
                        blocks.insert(spec_block.clone());
                        break;
                    }
                    spec_path.push(t.clone());
                    spec_loc = t;
                }
                // left the board. retreading done.
                _ => {break;}
            }
        }
        
        // proceed as normal
        match advance_board(board, cur_loc) {
            Valid::No     => {break;},
            Valid::Yes(t) => {
                points.insert(t.clone());
                cur_loc=t;
            }
        }
    }
    
    return blocks.len().try_into().unwrap();
}

fn main() {
    let board = match read_input("input.txt") {
        Ok(t) => t,
        Err(why) => panic!("AHHH BROKEN {}", why),
    };
    let origin = find_player(&board).unwrap();
    match valid_loc(&board, &origin) {
        Valid::No => panic!("start is invalid"),
        _ => {}
    }
    
    let path = simulate(&board, &origin);
    println!("Part1: {}", path.len());
    println!("part2: {}", simulate_with_dir(&board, &origin))
}
