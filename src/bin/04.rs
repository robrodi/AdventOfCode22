use std::cmp::{max, min};
pub fn part_one(input: &str) -> Option<u32> {
    
    let ns = parse(input);
    let chunks = ns.chunks(4);
    let mut count = 0;
    for pairs in chunks{
        if (pairs[0] >= pairs[2] && pairs[1] <= pairs[3])  // second pair contians first
            || (pairs[0] <= pairs[2] && pairs[1] >= pairs[3]) { // first pair contains second
            count += 1;
        }
    }
    Some(count)
}

fn parse(input: &str) -> Vec<u32>{
    input.split(&['\n', ',', '-'][..])
         .map(|n| n.parse::<u32>().unwrap())// to numbers
         .collect::<Vec<u32>>()
}

pub fn part_two(input: &str) -> Option<u32> {
    let ns = parse(input);
    let mut count = 0;
    for pairs in ns.chunks(4){
        if max(pairs[0], pairs[2]) <= min(pairs[1], pairs[3]){
            count += 1;
        }
    }
    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
