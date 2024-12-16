use std::fs::read_to_string;
use regex::Regex;
use std::collections::HashSet;

const BOARD_X: i32 = 101;
const BOARD_Y: i32 = 103;
const SIM_SEC: i32 = 100;

type Point = (i32, i32);

fn find_group(board: &Vec<Point>, origin: Point) -> i32 {
    let mut front: Vec<Point> = Vec::new();
    let mut visited: HashSet<Point> = HashSet::new();
    front.push(origin);

    while front.len() > 0 {
	let o = front.pop().unwrap();
	let (x, y) = o.clone();
	if visited.contains(&o) {
	    continue;
	}
	visited.insert(o);

	let u = (x, y+1);
	if board.contains(&u) {
	    front.push(u);
	}
	let d = (x, y-1);
	if board.contains(&d) {
	    front.push(d);
	}
	let l = (x-1, y);
	if board.contains(&l) {
	    front.push(l);
	}
	let r = (x+1, y);
	if board.contains(&r) {
	    front.push(r);
	}
    }
    visited.len().try_into().unwrap()
}

fn biggest_group(board: &Vec<Point>) -> i32 {
    let mut biggest = -1;
    for p in board {
	let size = find_group(board, *p);
	if size > biggest {
	    biggest = size;
	}
    }
    biggest
}

fn print_board(board: &Vec<Point>) {
    for y in 0..BOARD_Y {
	for x in 0..BOARD_X {
	    if board.contains(&(x, y)) {
		print!("*")
	    }
	    else {
		print!(" ");
	    }
	}
	println!();
    }
}

fn main() {
    let fname = "input.txt";
    let re = Regex::new(r"p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)").expect("regs");
    let file = read_to_string(fname).expect("reads");
    let eval = |sec| file.clone().lines()
	.map(|x| re.captures_iter(x).next().expect("no match????").extract::<4>())
	.map(|x| (x.1[0].parse::<i32>().unwrap(), x.1[1].parse::<i32>().unwrap(), x.1[2].parse::<i32>().unwrap(), x.1[3].parse::<i32>().unwrap()))
	.map(|x| (((x.0 + (x.2 * sec))%BOARD_X), (x.1 + (x.3 * sec))%BOARD_Y))
	.map(|x| (if x.0 >= 0 {x.0} else {x.0 + BOARD_X}, if x.1 >= 0 {x.1} else {x.1 + BOARD_Y}))
	.collect::<Vec<_>>();

    let q0 = eval(SIM_SEC).into_iter().filter(|x| x.0 < BOARD_X/2 && x.1 < BOARD_Y/2).collect::<Vec<_>>().len();
    let q1 = eval(SIM_SEC).into_iter().filter(|x| x.0 > BOARD_X/2 && x.1 < BOARD_Y/2).collect::<Vec<_>>().len();
    let q2 = eval(SIM_SEC).into_iter().filter(|x| x.0 < BOARD_X/2 && x.1 > BOARD_Y/2).collect::<Vec<_>>().len();
    let q3 = eval(SIM_SEC).into_iter().filter(|x| x.0 > BOARD_X/2 && x.1 > BOARD_Y/2).collect::<Vec<_>>().len();
    
    println!("part 1: {:?}", q0*q1*q2*q3);

    let mut sec = 0;
    let mut big = 0;
    while big < 15 {
	sec += 1;
	big = biggest_group(&eval(sec));
    }
    print_board(&eval(sec));
    println!("part 2: {:?}", sec);
}
