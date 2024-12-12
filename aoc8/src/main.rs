use std::io;
use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;

fn read_board(fname: &str) -> io::Result<Vec<Vec<char>>> {
    let mut f = File::open(fname)?;
    let mut fcont = String::new();
    f.read_to_string(&mut fcont)?;

    Ok(
	fcont.lines().map(|x| x.chars().collect()).collect()
    )
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Hotspot {
    loc: Point,
    sig: char
}

fn get_anten_map(board: &Vec<Vec<char>>) -> HashMap<char, Vec<Hotspot>> {
    let mut ret: HashMap<char, Vec<Hotspot>> = HashMap::new();

    for (y, row) in board.iter().enumerate() {
	for (x, col) in row.iter().enumerate() {
	    let x:i32 = x.try_into().unwrap();
	    let y:i32 = y.try_into().unwrap();
	    
	    if *col == '.' {
		continue;
	    }
	    match ret.get_mut(col) {
		Some(l) => {l.push(Hotspot{loc: Point{x, y}, sig:*col});},
		None => {ret.insert(*col, vec!(Hotspot{loc: Point{x, y}, sig:*col}));}
	    }
	}
    }
    
    ret
}

fn get_vec(a: &Hotspot, b: &Hotspot) -> (i32, i32) {
    let dx = a.loc.x - b.loc.x;
    let dy = a.loc.y - b.loc.y;
    (dx, dy)
}

fn get_hs(a: &Hotspot, b: &Hotspot, i: i32) -> Hotspot {
    let v = get_vec(a, b);
    let loc = Point {x: a.loc.x + (v.0*i), y: a.loc.y + (v.1*i)};
    Hotspot {loc, sig: a.sig}
}

fn print_board(board: &Vec<Vec<char>>) {
    for (y, row) in board.iter().enumerate() {
	for (x, col) in row.iter().enumerate() {
	    print!("{}", col);
	}
	println!("");
    }
}

fn valid_loc(board: &Vec<Vec<char>>, p: &Hotspot) -> bool {
    let my: i32 = board.len().try_into().unwrap();
    let mx: i32 = board[0].len().try_into().unwrap();
    let Hotspot {loc: Point {x, y}, ..} = p;
    *x >= 0 && *x < mx && *y >= 0 && *y < my
}

fn print_board_hotspots(board: &Vec<Vec<char>>, hs: &HashSet<Point>) {
    for (y, row) in board.iter().enumerate() {
	for (x, col) in row.iter().enumerate() {
	    if hs.contains(&Point {x: x.try_into().unwrap(), y: y.try_into().unwrap()}) {
		print!("{}", '#');
		continue;
	    }
	    print!("{}", col);
	}
	println!("");
    }
}

fn part1(board: &Vec<Vec<char>>) -> i32 {
    let antens = get_anten_map(board);
    let mut hs: HashSet<Hotspot> = HashSet::new();
    for (sig, anten) in antens {

	for src in anten.iter() {
	    for dst in anten.iter() {
		if src == dst {
		    continue;
		}
		let p = get_hs(src, dst, 1);
		if valid_loc(board, &p) {
		    hs.insert(p);
		}
	    }
	}
    }
    let hs_points: HashSet<Point> = hs.iter().map(|x| Point {x: x.loc.x, y: x.loc.y}).collect();
    print_board_hotspots(board, &hs_points);
    hs_points.len().try_into().unwrap()
}

fn part2(board: &Vec<Vec<char>>) -> i32 {
    let antens = get_anten_map(board);
    let mut hs: HashSet<Hotspot> = HashSet::new();
    for (sig, anten) in antens {

	for src in anten.iter() {
	    for dst in anten.iter() {
		if src == dst {
		    continue;
		}
		let mut i = 0;
		loop {
		let p = get_hs(src, dst, i);
		    if !valid_loc(board, &p) {
			break;
		    }
		    i += 1;
		    hs.insert(p);
		}
	    }
	}
    }
    let hs_points: HashSet<Point> = hs.iter().map(|x| Point {x: x.loc.x, y: x.loc.y}).collect();
    print_board_hotspots(board, &hs_points);
    hs_points.len().try_into().unwrap()
}

fn main() {
    let b = match read_board("input.txt") {
	Ok(t) => t,
	_ => panic!("Can't read file")
    };
    println!("Part1: {}", part1(&b));
    println!("Part2: {}", part2(&b));
}
