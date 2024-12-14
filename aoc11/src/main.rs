use std::fs::read_to_string;
use std::collections::HashMap;

fn even_digits(num: u64) -> bool {
    num.to_string().len() % 2 == 0
}

fn rules(num: u64) -> Vec<u64> {
    if num == 0 {
	return Vec::from([1]);
    }
    else if even_digits(num) {
	let str_num = num.to_string();
	let l = &str_num[0..str_num.len()/2];
	let r = &str_num[str_num.len()/2..];
	return Vec::from([l.parse::<u64>().expect("parse fail"), r.parse::<u64>().expect("parse fail")]);
    }
    Vec::from([num * 2024])
}

fn n_blink_len(list: &Vec<u64>, num_blinks: u64, map: &mut HashMap<(u64, u64), u64>) -> u64 {
    if num_blinks == 0 {
	return list.len().try_into().unwrap();
    }
    
    let mut sum = 0;
    for i in list {
	let res = match map.get(&(*i, num_blinks-1)) {
	    None => {
		let future = rules(*i);
		let temp = n_blink_len(&future, num_blinks-1, map);    
		map.insert((*i, num_blinks-1), temp);
		temp
	    },
	    Some(t) => {
		*t
	    },
	};
	sum += res;
    }    
    sum
}

fn blink(list: Vec<u64>) -> Vec<u64> {
    list.into_iter().flat_map(|x| {rules(x)}).collect()
}

fn main() {
    let mut inp = read_to_string("input.txt").expect("fail file read").lines().flat_map(
	|line| {
	    line.split(' ').map(|num| {
		num.parse::<u64>().expect("fail int parse")
	    }).collect::<Vec<_>>()
	}
    ).collect::<Vec<_>>();

    let p2 = inp.clone();
    
    for _ in 0..25 {
	inp = blink(inp);
    }
    
    println!("part1: {}", inp.len());
    println!("part2: {}", n_blink_len(&p2, 75, &mut HashMap::new()));
}
