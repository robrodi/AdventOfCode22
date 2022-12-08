#![allow(dead_code)]
#![allow(clippy::single_match)]
use std::collections::HashMap;
use substring::Substring;
pub fn part_one(input: &str) -> Option<u32> {
    let mut pwd: Vec<&str> = Vec::new();
    let mut sizes: HashMap<String, u32> = HashMap::new();
    pwd.push("/");
    for line in input.lines() {
        if line.starts_with('$') {
            // command
            // println!("command: {}", line);
            let x = line.chars().nth(2).unwrap();
            match x {
                'l' => { // ls
                }
                'c' => {
                    // cd
                    match line.chars().nth(5).unwrap() {
                        '.' => {
                            pwd.pop();
                        }
                        '/' => {
                            pwd.clear();
                        }
                        _ => {
                            pwd.push(line.substring(5, line.len()));
                        }
                    };
                }
                _ => {
                    println!("unsupported command {}", line);
                    panic!("unsupported command");
                }
            }
        } else {
            let path = &pwd.join("/");
            let c1 = line.chars().nth(2).unwrap();
            match c1 {
                '0'..='9' => {
                    // println!("x: {}", line);
                    let size = line
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .parse::<u32>()
                        .unwrap();
                    sizes
                        .entry(path.to_string())
                        .and_modify(|s| *s += size)
                        .or_insert(size);
                }
                _ => {
                    // directory
                }
            }
        }
    }

    let mut tot: u32 = 0;
    let mut root = Tree {
        value: *sizes.get("").unwrap(),
        path: "".to_string(),
        children: HashMap::new(),
    };

    for (path, size) in &sizes {
        if path.is_empty() {
            continue;
        }
        // inset this node.

        let mut cur: *mut Tree = &mut root;
        // let cur = &mut root; // borrow this mf.
        for dir in path.split('/') {
            unsafe {
                let c: &mut Tree = &mut *cur;
                if !c.children.contains_key(dir) {
                    println!(">{}<", c.path);
                    let size: u32 = *sizes.get(dir).unwrap();
                    let d = dir.to_owned();
                    c.add_child(size, d);
                    //let c = c.children.get("wtf");

                    // let m: &Tree = &*c.children.get("wtf").unwrap();
                    // cur = mut &*;
                    cur = &mut root;
                }
                // *cur.add_child(*, dir.to_string());
            }
        }
        if path.is_empty() {
            println!("/{} : {:#?}", path, size);
            // for (path2, size2) in &sizes {
            //     // if (path.contains(pat))
            // }
            if size <= &100000 {
                tot += size;
            }
        }
    }

    Some(tot)
}
struct Tree {
    // The value stored at this node.
    value: u32,
    path: String,
    // The child nodes of this tree.
    children: HashMap<String, Tree>,
}
impl Tree {
    fn add_child(&mut self, value: u32, path: String) {
        // Create a new tree with the given value,
        // and add it to the vector of children.
        let directory_name = path.split('/').into_iter().last().unwrap().to_string();
        self.children.insert(
            directory_name,
            Tree {
                value,
                path,
                children: HashMap::new(),
            },
        );
    }
}
pub fn part_two(_input: &str) -> Option<u32> {
    None
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
