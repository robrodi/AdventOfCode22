use std::num;

type Stacks = Vec<Vec<char>>;

pub fn part_one(input: &str) -> Option<u32> {
    parse(input);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}
pub fn parse(input: &str)
{
    let mut parts = input.split("\n\n");
    let stacks = parts.next().unwrap();

    parse_stacks(stacks);

    let moves = parts.last().unwrap();



}
pub fn parse_stacks(input: &str) { //} -> Stacks{
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
    for stack in stacks{
        println!("Stack: {:#?}", stack);
    }
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
        assert_eq!(part_two(&input), None);
    }
}
