use std::fs::read_to_string;
use regex::Regex;
use itertools::izip;
use std::collections::HashMap;

type Point = (i64, i64);

fn parse_single(a: &str, b: &str, c: &str) -> (Point, Point, Point) {
    
    let button_re = Regex::new(r"Button [AB]: X[\+-]([0-9]+), Y[\+-]([0-9]+)").expect("fail button regex");
    let prize_re  = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").expect("fail prize regex");
    
    izip!(button_re.captures_iter(a).map(|x| x.extract::<2>()).map(|x| (x.1[0].parse().unwrap(), x.1[1].parse().unwrap())),
	  button_re.captures_iter(b).map(|x| x.extract::<2>()).map(|x| (x.1[0].parse().unwrap(), x.1[1].parse().unwrap())),
	  prize_re.captures_iter(c).map(|x| x.extract::<2>()).map(|x| (x.1[0].parse::<i64>().unwrap(), x.1[1].parse::<i64>().unwrap()))).collect::<Vec<_>>()[0]
}

fn cost_to_reach(point: &Point, a_dir: &Point, b_dir: &Point, mem: &mut HashMap<Point,Option<u64>>) -> Option<u64>{
    let a_toks = 3;
    let b_toks = 1;

    let (tar_x, tar_y) = *point;
    let (a_dx, a_dy) = *a_dir;
    let (b_dx, b_dy) = *b_dir;
    
    if tar_x == 0 && tar_y == 0 {
	return Some(0);
    }
    if tar_y < 0 || tar_x < 0 {
	return None;
    }

    let rec = mem.get(point);
    if rec.is_some() {
	return *rec.unwrap();
    }
    
    let a_cost = cost_to_reach(&(tar_x - a_dx, tar_y - a_dy), &a_dir, &b_dir, mem);
    let b_cost = cost_to_reach(&(tar_x - b_dx, tar_y - b_dy), &a_dir, &b_dir, mem);
    if a_cost.is_none() && b_cost.is_none() {
	mem.insert(*point, None);
	return None;
    }
    if a_cost.is_none() {
	let res = Some(b_toks + b_cost.unwrap());
	mem.insert(*point, res);
	return res;
    }
    if b_cost.is_none() {	
	let res = Some(a_toks + a_cost.unwrap());
	mem.insert(*point, res);
	return res;
    }
    if a_cost.unwrap() < b_cost.unwrap() {
	let res = Some(a_toks + a_cost.unwrap());
	mem.insert(*point, res);
	return res;
    }
    else {	
	let res = Some(b_toks + b_cost.unwrap());
	mem.insert(*point, res);
	return res;
    }
}

fn part1(puzzle: &Vec<(Point, Point, Point)>) {
    let mut sum = 0;
    for i in puzzle {
	let (a, b, tar) = i;
	match cost_to_reach(tar, a, b, &mut HashMap::new()) {
	    Some(t) => {sum += t;},
	    _ => {}
	}
    }
    println!("part1: {}", sum);
}

// we have a point x, y
// we want to find i, j such that
// x = i * a_x + j * b_x
// y = i * a_y + j * b_y
// min 3*i + j
//
// solve {{a_x, b_x}, {a_y, b_y}} {{i}, {j}} = {{x}, {y}}
// cramers rule https://byjus.com/maths/cramers-rule/

fn cost_to_reach_cramer(xy: &Point, a_dir: &Point, b_dir: &Point) -> Option<u64> {
    let dx = xy.0 * b_dir.1 - xy.1 * b_dir.0;
    let dy = xy.1 * a_dir.0 - xy.0 * a_dir.1;
    let d  = a_dir.0 * b_dir.1 - a_dir.1 * b_dir.0;

    if dx % d != 0 || dy % d != 0 {
	return None;
    }
    
    let a_tok = dx / d;
    let b_tok = dy / d;
    Some((a_tok * 3 + b_tok).try_into().unwrap())
}

fn part2(puzzle: &Vec<(Point, Point, Point)>) {
    let mut sum = 0;
    const OFFS: i64 = 10000000000000;
    for (num, i) in puzzle.into_iter().enumerate() {
	let (a, b, (x, y)) = i;
	let tar = (x+OFFS, y+OFFS);
	match cost_to_reach_cramer(&tar, a, b) {
	    Some(t) => {sum += t;},
	    _ => {}
	}
    }
    println!("part2: {}", sum);
}

fn parse_strs(i: Vec<(&str, &str, &str)>) -> Vec<(Point, Point, Point)> {
    let mut res = vec!();
    for tup in i {
	let (a, b, c) = tup;
	res.push(parse_single(a,b,c));
    }
    res
}

fn main() {
    let fname = "input.txt";
    let file = read_to_string(&fname).expect("read");
    let line1 = file.lines().step_by(4);
    let line2 = file.lines().skip(1).step_by(4);
    let line3 = file.lines().skip(2).step_by(4);
    let puzzle_strs = line1.zip(line2).zip(line3).map(|x| (x.0.0, x.0.1, x.1)).collect::<Vec<_>>();
    let puzzle = parse_strs(puzzle_strs);
    part1(&puzzle);
    part2(&puzzle);
}
