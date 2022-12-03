use substring::Substring;
use std::collections::HashSet;


pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(shared_char).map(char_score).sum())
}
fn shared_char(line: &str) -> char{
    let l: HashSet<char> = line.substring(0, line.len() /2).chars().collect();
    let r = line.substring(line.len()/2,line.len()).chars();
    r.filter(|c| l.contains(c)).next().unwrap()
}
fn char_score(c: char) -> u32{
    if c.is_lowercase() {c as u32 - 96}
    else {c as u32- 38}
}

pub fn part_two(input: &str) -> Option<u32> {
    // lines, groups of 3 lines, 
    let mut sum = 0;
    let lines: Vec<&str> = input.lines().collect();
    for n in 0..lines.len() / 3{
        let start_line = n * 3;
        let a: HashSet<char> = lines[start_line].chars().collect();
        let b: HashSet<char> = lines[start_line + 1].chars().collect();
        let c = lines[start_line + 2].chars().filter(|c| a.contains(c) && b.contains(c)).next().unwrap();   
        sum += char_score(c);     
    }
    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }
    #[test]
    fn test_shared_chars_one(){
        assert_eq!(shared_char("vJrwpWtwJgWrhcsFMMfFFhFp"), 'p');
        assert_eq!(shared_char("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 'L');
        assert_eq!(shared_char("PmmdzqPrVvPwwTWBwg"), 'P');
        assert_eq!(shared_char("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 'v');
    }
    #[test]
    fn test_char_score(){
        // A = 65
        // a = 97
        assert_eq!(char_score('a'), 1, "a");
        assert_eq!(char_score('A'), 27, "A");
    }


    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
