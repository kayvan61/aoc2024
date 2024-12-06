use std::io;
use std::fs::File;
use std::io::Read;
use std::iter::zip;

fn read_input() -> io::Result<Vec<Vec<i32>>> {
   let mut f = File::open("input.txt")?;
   let mut fcont = String::new();
   f.read_to_string(&mut fcont)?;

   Ok(fcont
	.lines()
	.map(|x| {
		 x.split(" ")
		   .map(|y| { y.parse::<i32>().unwrap() })
		   .collect::<Vec<i32>>()
		   }
	    ).collect::<Vec<Vec<i32>>>())
}

fn is_safe_pair(pair: (i32, i32), is_inc: bool) -> bool {
   let (i,j) = pair;
   if i == j { return false }
   if i > j && !is_inc { return false }
   if i < j && is_inc { return false }
   true
}

#[derive(Debug)]
enum AlmostErrorKind {
     Err,
     Rev(usize, usize),
     Ok
}

fn is_almost_monotonic(record: &Vec<i32>) -> AlmostErrorKind {
   let is_inc = record[0] > record[1];
   let mut remove_idx: Option<usize> = None;
   
   for (idx, (i, j)) in zip(record[0..].iter(), record[1..].iter()).enumerate() {
       if !is_safe_pair((*i,*j), is_inc) {
       	  if remove_idx.is_none() {
	     remove_idx = Some(idx);
	  }
	  else {
	     return AlmostErrorKind::Err;
	  }
       }
   }
   if remove_idx.is_some() {
       let idx = remove_idx.unwrap();
       return AlmostErrorKind::Rev(idx, idx+1);
   }
   AlmostErrorKind::Ok
}

fn is_monotonic(record: &Vec<i32>) -> bool {
   let is_inc = record[0] > record[1];
   
   for (i, j) in zip(record[0..].iter(), record[1..].iter()) {
       if !is_safe_pair((*i,*j), is_inc) {
       	  return false
       }
   }
   true
}

fn is_small_delta(record: &Vec<i32>) -> bool {   
   for (i, j) in zip(record[0..].iter(), record[1..].iter()) {
       if (i - j).abs() > 3 { return false; }
   }
   true
}

fn is_almost_small_delta(record: &Vec<i32>) -> AlmostErrorKind {
   let mut remove_idx: Option<usize> = None;
   
   for (idx, (i, j)) in zip(record[0..].iter(), record[1..].iter()).enumerate() {
       if (i - j).abs() > 3 {
       	  if remove_idx.is_none() {
       	     remove_idx = Some(idx);
	  }
	  else {
	       return AlmostErrorKind::Err;
	  }
       }
   }
   if remove_idx.is_some() {
       let idx = remove_idx.unwrap();
       return AlmostErrorKind::Rev(idx, idx+1);
   }
   AlmostErrorKind::Ok
}

fn is_safe(record: &Vec<i32>) -> bool {
   is_monotonic(record) && is_small_delta(record)
}

fn is_almost_safe(record: &Vec<i32>) -> bool {
   // brute force I give up :(
   for i in 0..record.len() {
       let mut rec_copy = record.clone();
       rec_copy.remove(i);
       let small_delta_err = is_small_delta(&rec_copy);
       let monotonic_err   = is_monotonic(&rec_copy);
       if monotonic_err && small_delta_err { return true; }
   }
   is_monotonic(record) && is_small_delta(record)
}

fn main() {
    let all_records = match read_input() {
    	  Err(_) => panic!("fuck file broken"),
	  Ok(t) => t,
    };

    println!("part 1: {:?}", all_records.iter().filter(|x| is_safe(&x)).collect::<Vec<_>>().len());
    println!("part 2: {:?}", all_records.iter().filter(|x| is_almost_safe(&x)).collect::<Vec<_>>().len());
}
