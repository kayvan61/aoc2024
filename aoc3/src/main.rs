use std::io::Read;
use std::fs::File;
use regex::Regex;

fn proc_substr(s: &str) -> i32 {
   let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

   let mut sum = 0;
   for (_, [left, right]) in re.captures_iter(s).map(|c| c.extract()) {
       sum += left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap();
   }
   sum
}

fn main() {
   let mut f = match File::open("input.txt") {
       Err(why) => panic!("File open fail: {}", why),
       Ok(t)    => t,
   };
   let mut fcont = String::new();

   match f.read_to_string(&mut fcont) {
      Err(why) => panic!("File read fail: {}", why),
      _ => {}
   };

   fcont.push_str("don't()");
   let re_do = Regex::new(r"do\(\)").unwrap();
   let dont = Regex::new(r"don't\(\)").unwrap();

   let mut sum: i32 = 0;
   let splits: Vec<_> = re_do.split(&fcont).into_iter().collect();
   println!("{:?}", splits);
   println!("-----------------");
   for substr in splits {
       println!("{:?}", substr);
       let split2: Vec<_> = dont.split(substr).into_iter().collect();
       let enabled_region = split2[0];
       println!("{:?}", enabled_region);
       sum += proc_substr(enabled_region);
       println!("-----------------");
   }
   println!("part1: {}", proc_substr(&fcont));
   println!("part2: {}", sum);
   
}