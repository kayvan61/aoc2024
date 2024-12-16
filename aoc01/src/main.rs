use std::io::prelude::*;
use std::fs::File;
use std::io::Read;
use std::io;
use std::iter::zip;

fn read_lists(fname: &str) -> Option<(Vec<i32>, Vec<i32>)> {
    let mut f = match File::open(fname) {
        Err(why) => panic!("Couldn't open file {} for reading! {}", fname, why),
        Ok(file) => file,
    };
    let mut l1: Vec<i32> = vec!();
    let mut l2: Vec<i32> = vec!();
    let mut fcontents: String = String::new();
    f.read_to_string(&mut fcontents).ok()?;
    for line in fcontents.lines() {
        let parts: Vec<&str> = line.split(" ").filter(|tok| tok.len() > 0).collect();
        l1.push(parts[0].parse().unwrap());
        l2.push(parts[1].parse().unwrap());
    }
    Option::Some((l1, l2))
}

fn count_in_list(list: &Vec<i32>, element: i32) -> i32 {
    let mut sum = 0;
    for i in list {
        if element == *i {
            sum += 1;
        }
    }
    sum
}

fn main() {
    let (mut a, mut b) = match read_lists("input.txt") {
        Some(a) => a,
        _ => panic!("Error reading file!"),
    };
    a.sort();
    b.sort();
    let mut sum = 0;
    for (i,j) in zip(&a, &b) {
        sum += (*i-*j).abs()
    }
    
    let mut sum2 = 0;
    for i in a {
        sum2 += i * count_in_list(&b, i)
    }
    println!("part1: {}", sum);
    println!("part2: {}", sum2);
    
}
