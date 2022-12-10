use itertools::Itertools;
use substring::Substring;

pub fn part_one(input: &str) -> Option<u32> {
    let x = input.lines()
            .map(|l| (l.chars().nth(0).unwrap(), 
                            l.substring(2, l.len()).parse::<u32>().unwrap())).collect_vec();
    for (dir, num) in x {
        println!("{} {}", dir, num);        
    }
  None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
