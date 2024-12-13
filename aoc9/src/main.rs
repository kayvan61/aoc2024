use std::io;
use std::io::Read;
use std::fs::File;
use std::fmt;
use std::mem;
use std::char::from_digit;

fn read_inp(fname: &str) -> io::Result<String> {
    let mut f = File::open(fname)?;
    let mut fcont = String::new();

    f.read_to_string(&mut fcont)?;

    Ok(fcont)
}

struct List {
    head: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    empty: bool,
    size: u32,
    file: Option<u32>,
    next: Option<Box<Node>>,
}

// impl fmt::Display for List {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 	let mut cur_node = &self.head;
// 	while let Some(n) = cur_node {
// 	    match n.file {
// 		Some(x) => write!(f, "{}, {}, {} => ", n.empty, n.size, x)?,
// 		None => write!(f, "{}, {} => ", n.empty, n.size)?
// 	    }
// 	    cur_node = &n.next;
// 	}
// 	Ok(())
//     }
// }

impl List {
    pub fn new() -> Self {
	List {head: None}
    }

    pub fn smart_defrag(&mut self) {
	let mut idx = self.len();
	loop {
	    if idx == 0{
		println!("break cause reached front of list");
		break;
	    }
	    let used = self.get_last_used_before(idx);
	    if used.is_none() {
		println!("break cause no more used blocks");
		break;
	    }
	    let size = used.as_ref().unwrap().size;
	    let file = used.as_ref().unwrap().file;
	    
	    
	    if !self.has_next_free_sized(size) {
		idx -= 1;
		continue;
	    }

	    let used = self.get_last_used_before(idx);
	    used.as_mut().unwrap().empty = true;
	    used.as_mut().unwrap().file = None;
	    // move time
	    let free = self.get_next_free_sized(size);
	    let free_size = free.as_ref().unwrap().size;
	    free.as_mut().unwrap().file = file;
	    free.as_mut().unwrap().empty = false;
	    free.as_mut().unwrap().size = size;
	    Self::insert_after(free, Box::new(Node{file: None, empty: true, size: free_size-size, next: None}));

	    idx -= 1;
	}
    }
    
    pub fn defrag(&mut self) {
	while self.is_fragmented() {
	    let used = self.get_last_used();
	    let mut size = used.as_ref().unwrap().size;
	    let file = used.as_ref().unwrap().file;
	    used.as_mut().unwrap().empty = true;
	    used.as_mut().unwrap().file = None;

	    while size > 0 {
		let free = self.get_next_free();
		let free_size = free.as_ref().unwrap().size;
		free.as_mut().unwrap().file = file;
		if size < free_size {
		    free.as_mut().unwrap().empty = false;
		    free.as_mut().unwrap().size = size;
		    Self::insert_after(free, Box::new(Node{file: None, empty: true, size: free_size-size, next: None}));
		    size = 0;
		}
		else {
		    free.as_mut().unwrap().empty = false;
		    size -= free_size;
		}
	    }
	}
    }

    pub fn to_string(&self) -> String {
	let mut ret = String::new();
	let mut cur_node = &self.head;
	loop {
	    match &mut cur_node {
		Some(n) => {
		    for i in 0..n.size {
			match n.file {
			    Some(d) => {
				ret.push(from_digit(d, 10).expect("Failed to parse file back into digit"));
			    },
			    None => {ret.push('.');}
			}
		    }
		}
		None => {break;}
	    }
	    cur_node = &cur_node.as_ref().unwrap().next;
	}
	ret 
    }

    pub fn insert_after(old_node: &mut Option<Box<Node>>, mut new_node: Box<Node>) {
	new_node.next = mem::replace(&mut old_node.as_mut().unwrap().next, None);
	old_node.as_mut().unwrap().next = Some(new_node);
    }

    pub fn is_fragmented(&self) -> bool {
	let mut found_empty = false;
	let mut cur_node = &self.head;
	loop {
	    match &mut cur_node {
		Some(n) => {
		    if n.empty {
			found_empty = true;
		    }
		    else if found_empty && !n.empty {
			return true;
		    }
		}
		None => {break;}
	    }
	    // Option::as_mut returns an Option<&mut T>, gets unwrapped into &mut T, goes next for a mut borrow
	    cur_node = &cur_node.as_ref().unwrap().next;
	}
	false
    }

    pub fn has_next_free_sized(&self, size: u32) -> bool {
	let mut cur_node = &self.head;
	loop {
	    match &mut cur_node {
		Some(n) => {
		    if n.empty && n.size >= size {
			return true;
		    }
		}
		None => {break;}
	    }
	    // Option::as_mut returns an Option<&mut T>, gets unwrapped into &mut T, goes next for a mut borrow
	    cur_node = &cur_node.as_ref().unwrap().next;
	}
	false
    }

    pub fn get_next_free_sized(&mut self, size: u32) -> &mut Option<Box<Node>> {
	let mut cur_node: &mut Option<Box<Node>> = &mut self.head;
	loop {
	    match &mut cur_node {
		Some(n) => {
		    if n.empty && n.size >= size {
			break;
		    }
		}
		None => {break;}
	    }
	    // Option::as_mut returns an Option<&mut T>, gets unwrapped into &mut T, goes next for a mut borrow
	    cur_node = &mut cur_node.as_mut().unwrap().next;
	}
	&mut *cur_node
    }
    
    pub fn get_next_free(&mut self) -> &mut Option<Box<Node>> {
	let mut cur_node: &mut Option<Box<Node>> = &mut self.head;
	loop {
	    match &mut cur_node {
		Some(n) => {
		    if n.empty {
			break;
		    }
		}
		None => {break;}
	    }
	    // Option::as_mut returns an Option<&mut T>, gets unwrapped into &mut T, goes next for a mut borrow
	    cur_node = &mut cur_node.as_mut().unwrap().next;
	}
	&mut *cur_node
    }

    pub fn len(&self) -> u32 {
	let mut i = 0;
	let mut cur_node: &Option<Box<Node>> = &self.head;
	loop {
	    match &cur_node {
		Some(n) => {
		    i += 1;
		}
		None => {break;}
	    }
	    // Option::as_mut returns an Option<&mut T>, gets unwrapped into &mut T, goes next for a mut borrow
	    cur_node = &cur_node.as_ref().unwrap().next;
	}
	i
    }

    fn get_nth_mut(&mut self, n: u32) -> &mut Option<Box<Node>> {
	let mut cur_node = &mut self.head;
	let mut i = 0;
	loop {
	    if i == n {
		break;
	    }
	    cur_node = &mut cur_node.as_mut().unwrap().next;
	    i += 1;
	}
	&mut *cur_node
    }

    pub fn get_last_used_before(&mut self, idx: u32) -> &mut Option<Box<Node>> {
	let mut i = 0;
	let endIdx = idx;
	let mut last_idx: Option<u32> = None;
	let mut cur_node = &mut self.head;
	
	loop {
	    match &mut cur_node {
		Some(n) => {
		    if !n.empty && endIdx > i {
			last_idx = Some(i);
		    }
		}
		None => {
		    break;
		}
	    }
	    cur_node = &mut cur_node.as_mut().unwrap().next;
	    i += 1;
	}
	match last_idx {
	    Some(t) => self.get_nth_mut(t),
	    None    => self.get_nth_mut(i)
	}
    }

    pub fn get_last_used(&mut self) -> &mut Option<Box<Node>> {
	let mut i = 0;
	let mut last_idx = 0;
	let mut cur_node = &mut self.head;
	
	loop {
	    match &mut cur_node {
		Some(n) => {
		    if !n.empty {
			last_idx = i;
		    }
		}
		None => {
		    break;
		}
	    }
	    cur_node = &mut cur_node.as_mut().unwrap().next;
	    i += 1;
	}
	self.get_nth_mut(last_idx)
    }

    pub fn push(&mut self, empty: bool, size: u32, file: Option<u32>) {
	let new_node = Box::new(Node{
	    empty,
	    size,
	    file,
	    next: None,
	});

	let mut cur_node = &mut self.head;
	
	// first time
	if cur_node.is_none() {
	    self.head = Some(new_node);
	    return;
	}

	// all others
	loop {
	    if cur_node.as_ref().unwrap().next.is_none() {
		break;
	    }
	    cur_node = &mut cur_node.as_mut().unwrap().next;
	}

	cur_node.as_mut().unwrap().next = Some(new_node);
    }

    pub fn sum_list(&self) -> u64 {
	let mut sum: u64 = 0;
	let mut pos_idx = 0;
	
	let mut cur_node = &self.head;
	loop {
	    match &mut cur_node {
		Some(n) => {
		    if !n.empty {
			let file_idx = n.file.unwrap();
			for i in 0..n.size {
			    sum += TryInto::<u64>::try_into(file_idx * (pos_idx + i)).unwrap();
			}
		    }
		    pos_idx += n.size;
		}
		None => {break;}
	    }
	    // Option::as_mut returns an Option<&mut T>, gets unwrapped into &mut T, goes next for a mut borrow
	    cur_node = &cur_node.as_ref().unwrap().next;
	}
	sum
    }
}

fn string_to_linked_list(s: String) -> List {
    let mut l = List::new();
    let mut e = false;
    let mut file_idx = 0;
    for c in s.chars() {
	match c.to_digit(10) {
	    Some(res) => {l.push(e, res, match e {false => Some(file_idx), true => None});},
	    _ => {break;}
	}
	if !e {
	    file_idx += 1;
	}
	e = !e;
    }
    l
}

fn main() {
    let fs = match read_inp("input.txt") {
	Ok(t) => t,
	_ => panic!("Error reading input")
    };
    let mut blocks = string_to_linked_list(fs);
    blocks.defrag();
    println!("Part1: {}", blocks.sum_list());

    let fs = match read_inp("input.txt") {
	Ok(t) => t,
	_ => panic!("Error reading input")
    };
    let mut blocks = string_to_linked_list(fs);
    blocks.smart_defrag();
    println!("Part2: {}", blocks.sum_list());
    
}
