use std::fs::read_to_string;
use std::collections::HashSet;

fn read_input(fname: &str) -> Vec<Vec<char>> {
    read_to_string(fname).expect("read file fail").lines().map(|x| {
	x.chars().collect()
    }).collect()
}

type Point = (usize, usize);

fn validate_and_insert(board: &Vec<Vec<char>>, loc: Option<Point>, front: &mut Vec<Point>, level: char, sum: &mut u64) -> bool {
    let real_loc = match loc {
        Some(t) => t,
        None => {panic!("validate and insert none loc")}
    };
        
    let x = real_loc.0;
    let y = real_loc.1;
    if x >= board[0].len() {
	*sum += 1;
        return false;
    }
    if y >= board.len() {
	*sum += 1;
        return false;
    }
    
    let new_level = board[y][x];
    if new_level == level {
        front.push((x.try_into().unwrap(), y.try_into().unwrap()));
	true
    }
    else {
	*sum += 1;
	false
    }
}

#[derive(PartialEq, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

fn turn_right(dir: &Dir) -> Dir {
    use self::Dir::{Up, Right, Left, Down};
    match dir {
	Up => Right,
	Right => Down,
	Down => Left,
	Left => Up
    }
}

fn turn_left(dir: &Dir) -> Dir {
    use self::Dir::{Up, Right, Left, Down};
    match dir {
	Up => Left,
	Left => Down,
	Down => Right,
	Right => Up,
    }
}

fn walk(origin: &Point, dir: &Dir) -> Option<Point> {
    use self::Dir::{Up, Right, Left, Down};
    let (x, y): (usize, usize) = *origin;
    match dir {
	Up => Some((x, y+1)),
	Down => match y {
	    0 => None,
	    _ => Some((x, y-1))
	},
	Left => match x {
	    0 => None,
	    _ => Some((x-1, y))
	},
	Right => Some((x+1, y))
    }
}

fn vis_border(board: &Vec<Vec<char>>, border: &HashSet<Point>) {
    for (y, row) in board.into_iter().enumerate() {
	for (x, c) in row.into_iter().enumerate() {
	    if border.contains(&(x,y)) {
		print!("#");
	    }
	    else {
		print!("{}", c);
	    }
	}
	println!();
    }
}

fn walk_border(border: &HashSet<Point>, origin: &Point) -> u64 {
    let mut turns     = 0;

    let start_dir = Dir::Up;
    let start_pos = origin.clone();
    
    let mut cur_dir = Dir::Up;
    let mut cur_pos = walk(origin, &cur_dir);
    // oob turn
    if cur_pos.is_none() || !border.contains(&cur_pos.unwrap()) {
	cur_pos = Some(origin.clone());
	cur_dir = turn_right(&cur_dir);
	turns += 1;
    }
    
    while cur_dir != start_dir || cur_pos.unwrap() != start_pos {
	let left_dir = turn_left(&cur_dir);
	let right_dir = turn_right(&cur_dir);
	let next_pos_left  = walk(&cur_pos.unwrap(), &left_dir);
	let next_pos_front = walk(&cur_pos.unwrap(), &cur_dir);
	let next_pos_right = walk(&cur_pos.unwrap(), &right_dir);

	// left if possible
	if next_pos_left.is_some() && border.contains(&next_pos_left.unwrap()) {
	    cur_dir = left_dir;
	    cur_pos = next_pos_left;
	    turns += 1;
	}
	// front if possible
	else if next_pos_front.is_some() && border.contains(&next_pos_front.unwrap()){
	    cur_pos = next_pos_front;
	}
	// otherwise turn right
	else {
	    cur_dir = right_dir;
	    turns += 1;
	}
	//println!("{:?} {:?} {:?} {:?}", cur_dir, cur_pos, start_dir, start_pos);
    }

    turns
}

fn explore_plot(board: &Vec<Vec<char>>, claimed_cords: &mut HashSet<Point>, origin: Point) -> (u64, u64, u64) {
    let mut frontier: Vec<Point> = Vec::new();
    frontier.push(origin);
    let current_plant = board[origin.1][origin.0];
    let mut uniq_points: HashSet<Point> = HashSet::new();
    let mut border: HashSet<Point> = HashSet::new();
    let mut perim = 0;
    
    while frontier.len() > 0 {
	let cur_loc = frontier.pop().unwrap();
	if claimed_cords.contains(&cur_loc) {
	    continue;
	}
	claimed_cords.insert(cur_loc);
	uniq_points.insert(cur_loc);
	
	
	let (x, y) = cur_loc;
	
	let up    = Some((x, y + 1));
        let down  = match y {
            0 => {perim += 1; border.insert(cur_loc); None},
            _ => Some((x, y - 1))
        };
        let left  = match x {
            0 => {perim += 1; border.insert(cur_loc); None},
            _ => Some((x - 1, y))
        };
        let right = Some((x + 1, y));

	let mut is_border = false;
	is_border = is_border | !validate_and_insert(board, up,    &mut frontier, current_plant, &mut perim);
        is_border = is_border | match down {
	    Some(p) => !validate_and_insert(board, Some(p),  &mut frontier, current_plant, &mut perim),
	    None    => {true}
	};
        is_border = is_border | match left {
	    Some(p) => !validate_and_insert(board, Some(p),  &mut frontier, current_plant, &mut perim),
	    None    => {true}
	};
        is_border = is_border | !validate_and_insert(board, right, &mut frontier, current_plant, &mut perim);
	
	if is_border {
	    border.insert(cur_loc);
	}
    }
    //vis_border(board, &uniq_points);
    let sides = walk_border(&uniq_points, &origin);
    (perim, uniq_points.len().try_into().unwrap(), sides)
}

fn part1(board: &Vec<Vec<char>>) -> u64 {
    let mut claimed_cords: HashSet<Point> = HashSet::new();
    let mut regions: Vec<(char, (u64, u64, u64))> = Vec::new();
    for (y, row) in board.into_iter().enumerate() {
	for (x, plot) in row.into_iter().enumerate() {
	    if claimed_cords.contains(&(x,y)) {
		continue;
	    }
	    let pa = explore_plot(board, &mut claimed_cords, (x,y));
	    println!("{}: {:?}", *plot, pa);
	    regions.push((*plot, pa));
	}
    }
    regions.into_iter().map(|x| {x.1.0 * x.1.1}).sum()
}

fn part2(board: &Vec<Vec<char>>) -> u64 {
    let mut claimed_cords: HashSet<Point> = HashSet::new();
    let mut regions: Vec<(char, (u64, u64, u64))> = Vec::new();
    for (y, row) in board.into_iter().enumerate() {
	for (x, plot) in row.into_iter().enumerate() {
	    if claimed_cords.contains(&(x,y)) {
		continue;
	    }
	    let pa = explore_plot(board, &mut claimed_cords, (x,y));
	    regions.push((*plot, pa));
	    
	    
	}
    }
    regions.into_iter().map(|x| {x.1.2 * x.1.1}).sum()
}

fn main() {
    let board = read_input("test_input2.txt");
    println!("p1: {}", part1(&board));
    println!("p2: {}", part2(&board));
}
