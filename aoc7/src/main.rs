use std::io;
use std::fs::File;
use std::io::Read;

fn read_input(fname: &str) -> io::Result<Vec<(i64, Vec<i64>)>> {
    let mut f = File::open(fname)?;
    let mut fcont = String::new();

    f.read_to_string(&mut fcont)?;

    Ok(fcont.lines().map(|x| (x.split(':').filter(|x| x.len() > 0).collect::<Vec<_>>()[0].parse::<i64>().unwrap(),
                              x.split(':')
                               .filter(|x| x.len() > 0)
                               .collect::<Vec<_>>()[1]
                               .split(' ')
                               .filter(|x| x.len() > 0)
                               .map(|y| y.parse::<i64>()
                                         .unwrap())
                               .collect()))
       .collect())
}

fn i_concat(l:i64, r:i64) -> i64 {
    let mut s = l.to_string();
    s.push_str(&r.to_string());
    s.parse::<i64>().unwrap()
}

fn is_possible(target: i64, running_total:i64, list: &[i64], operators: &[char]) -> bool {
    if list.len() == 0 {
        return target == running_total;
    }

    let mut possi = false;
    for op in operators {
        let opr1 = list[0];
        possi = possi || match op {
            '+' => is_possible(target, running_total+opr1, &list[1..], operators),
            '*' => is_possible(target, running_total*opr1, &list[1..], operators),
            '|' => is_possible(target, i_concat(running_total,opr1), &list[1..], operators),
            _ => panic!("unrecognized operand")
        }
    }
    possi
}

fn main() {
    let recs = match read_input("test_input.txt") {
        Err(_) => panic!("fak"),
        Ok(t) => t,
    };

    let mut sum = 0;
    for re in recs {
        let (tar, ref nums) = re;
        if is_possible(tar, 0, &nums[..], &['+', '*']) {
            sum += tar;
        }
    }
    println!("Part1: {}", sum);

    let recs = match read_input("test_input.txt") {
        Err(_) => panic!("fak"),
        Ok(t) => t,
    };
    let mut sum = 0;
    for re in recs {
        let (tar, ref nums) = re;
        if is_possible(tar, 0, &nums[..], &['+', '*', '|']) {
            sum += tar;
        }
    }
    println!("Part2: {}", sum);
}
