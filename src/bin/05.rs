use std::num;

type Stacks = Vec<Vec<char>>;
type Move = [usize; 3];
type Moves = Vec<Move>;

pub fn part_one(input: &str) -> Option<u32> {
    let (mut stacks, moves) = parse(input);

    for m in moves{
        // println!("MOVE {} from {} to {}.", m[0], m[1], m[2]);
        for i in 0..m[0]{
            let val = &stacks[m[1] -1].pop().unwrap();
            // println!("  MOVE {} from {} to {}.", val, m[1], m[2]);
            stacks[m[2] - 1].push(*val);
        }
    }
    println!("DONE?");
    for mut stack in stacks{
        print!("{}", stack.pop().unwrap())
    }
    println!("");

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut stacks, moves) = parse(input);

    for m in moves{
        println!("MOVE {} from {} to {}.", m[0], m[1], m[2]);
        let mut stack_to_move:Vec<char> = Vec::new();
        for i in 0..m[0]{
            let val = &stacks[m[1] -1].pop().unwrap();
            println!("  MOVE {} from {} to {}.", val, m[1], m[2]);
            stack_to_move.push(*val);
        }
        for i in 0..m[0]{
            stacks[m[2] - 1].push(stack_to_move.pop().unwrap());
        }
    }
    println!("DONE?");
    for mut stack in stacks{
        print!("{}", stack.pop().unwrap())
    }
    println!("");

    None
}
pub fn parse(input: &str) -> (Stacks, Moves) {
    let mut parts = input.split("\n\n");
    let stacks = parts.next().unwrap();

    let stacks = parse_stacks(stacks);

    let moves = parts.last().unwrap();
    let moves = parse_moves(moves);

    (stacks, moves)

}
fn parse_stacks(input: &str) -> Stacks { //} -> Stacks{
    println!("Stacks:");

    let lines = input.lines().collect::<Vec<&str>>();
    let num_stacks = (lines[0].len() + 1)/ 4;
    let mut stacks: Stacks = vec![Vec::new(); num_stacks];
    for line in &lines[..lines.len() -1]{
        let l = line.chars().collect::<Vec<char>>();
        for i in 0..num_stacks{
            let val = l[i*4 + 1];
            if val != ' '{
                stacks[i].push(val);
            }
        }
    }
    let mut result: Stacks = Vec::new();
    
    for mut stack in stacks{
        stack.reverse();
        result.push(stack);
    }
    result
}
fn parse_moves(input: &str) -> Vec<Move>{
    let mut result: Moves = Vec::new();
    for mut line in input.lines(){
        let tokens = line.split(" ").collect::<Vec<&str>>();
        let num = tokens[1].parse::<usize>().unwrap();
        let source = tokens[3].parse::<usize>().unwrap();
        let target = tokens[5].parse::<usize>().unwrap();
        result.push([num, source, target]);
    }
    result
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(6));
    }
}
