use std::collections::HashMap;

use substring::Substring;

pub fn part_one(input: &str) -> Option<u32> {
    let mut pwd:Vec<&str>= Vec::new();
    let mut sizes: HashMap<String, u32> = HashMap::new();

    for line in input.lines(){
            if line.chars().nth(0).unwrap() == '$' { // command
            println!("command: {}", line);
            let x = line.chars().nth(2).unwrap();
            match x {
                'l' => { // ls
                    let path = &pwd.join("/");
                    let cur = if sizes.contains_key(path) {sizes[path]} else { 0 };

                },
                'c' => { // cd
                    println!("cd: {}", line);
                    
                    let y = line.chars().nth(5).unwrap();
                    match y{
                        '.' => {
                            println!("PWD (..): {}", pwd.join("/"));
                            pwd.pop();
                        },
                        '/' => {pwd.clear()},
                        _ => { 
                            pwd.push(line.substring(5, line.len())); 
                            println!("PWD: {}", pwd.concat());
                        }
                    };
                },
                _ => {
                    println!("unsupported command {}", line);
                    panic!("unsupported command");
                }
            }
            if line.chars().nth(3).unwrap() == 'l' { // ls

            }
            
        }
        else {
            println!("x: {}", line);
            
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
