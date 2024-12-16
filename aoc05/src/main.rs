use std::io;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn read_input(fname: &str) -> io::Result<(HashMap<i32, Vec<i32>>, Vec<Vec<i32>>)> {
    let mut f = File::open(fname)?;
    let mut fcont = String::new();
    f.read_to_string(&mut fcont)?;

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut lists: Vec<Vec<i32>> = vec!();
    let mut it = fcont.lines();
    
    // ordering rules
    loop {
        match it.next() {
            None => break,
            Some(line) => {
                if line.len() == 0 {
                    break;
                }
                let rule: Vec<i32> = line.split('|').into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
                match rules.get_mut(&rule[0]) {
                    None => {rules.insert(rule[0], vec!(rule[1]));},
                    Some(vec) => {vec.push(rule[1])},
                };
            },
        }
    }

    // read lists
    loop {
        match it.next() {
            None => break,
            Some(line) => {
                if line.len() == 0 {
                    println!("found section break!");
                    break;
                }
                lists.push(
                    line.split(',').into_iter().map(|x| x.parse::<i32>().unwrap()).collect()
                );
            },
        }
    }
    Ok((rules, lists))
}

// checks if any of sub_list appears in global_list
fn appears_in(sub_list: Option<&Vec<i32>>, global_list: &[i32]) -> bool {
    if sub_list.is_none() {
        return false;
    }
    sub_list.unwrap().into_iter().filter(|x| global_list.contains(x)).collect::<Vec<_>>().len() > 0
}

fn verify_part_1(rules: &HashMap<i32, Vec<i32>>, lists: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for list in lists.into_iter() {
        let mut bad_list = false;
        for i in 0..list.len() {
            let rule = rules.get(&list[i]);
            if appears_in(rule, &list[0..i]) {
                bad_list = true;
                break;
            }
        }
        if !bad_list {
            sum += list[TryInto::<usize>::try_into(list.len()/2).unwrap()]
        }
    }
    sum
}

// take a list and make it follow the ordering rules...
// rules give "before" relationship
// attempt to build the list in reverse and then return the flipped list.
// for each rule for the elements we care about
//     if the rule list is empty place the element, remove our element from the rules
//
// Returns the reverse of the fixed list. since its median idc about reverse :P
fn fix_list(rules: &HashMap<i32, Vec<i32>>, list: &Vec<i32>) -> Vec<i32> {
    // get relevant records
    let mut our_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut remaining_elements = list.clone();
    let mut ret: Vec<i32> = vec!();
    
    for ele in list {
        match rules.get(ele) {
            None => {our_rules.insert(*ele, vec!());},
            Some(t) => {our_rules.insert(*ele, t.clone().into_iter().filter(|x| list.contains(x)).collect());},
        };
    }
    while remaining_elements.len() > 0 {
        let mut placed_ele: Option<i32> = None;
        for (k,v) in our_rules.iter() {
            if v.len() == 0 && !ret.contains(k){
                // free to place
                ret.push(*k);
                placed_ele = Some(*k);
                break;
            }
        }
        if placed_ele.is_none() {
            println!("{:?}", our_rules);
            println!("{:?}", remaining_elements);
            panic!("couldn't find a var to add to the list!")
        }
        remaining_elements.retain(|x| *x != placed_ele.unwrap());
        for (_k, v) in our_rules.iter_mut() {
            v.retain(|x| *x != placed_ele.unwrap());
        }
    }

    ret
}

fn verify_part_2(rules: &HashMap<i32, Vec<i32>>, lists: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for list in lists.into_iter() {
        let mut bad_list = false;
        for i in 0..list.len() {
            let rule = rules.get(&list[i]);
            if appears_in(rule, &list[0..i]) {
                bad_list = true;
                break;
            }
        }
        if bad_list {
            let fixed_list = fix_list(rules, list);
            sum += fixed_list[TryInto::<usize>::try_into(fixed_list.len()/2).unwrap()]
        }
    }
    sum
}

fn main() {
    let (rules, lists) = match read_input("input.txt") {
        Ok(t) => t,
        Err(why) => panic!("fail parse D: {}", why),
    };

    // rules is a map.
    // key is a page.
    // value is a list of all the pages that must be AFTER the current page.
    // that is, there cannot pages in the value list for a given key before the key.
    println!("part1: {}", verify_part_1(&rules, &lists));
    println!("part2: {}", verify_part_2(&rules, &lists));
}
