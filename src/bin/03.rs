use std::collections::HashSet;
use substring::Substring;

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(shared_char).map(char_score).sum())
}

fn shared_char(line: &str) -> char {
    let l = to_hash(line.substring(0, line.len() / 2));
    let mut r = line.substring(line.len() / 2, line.len()).chars();
    r.find(|c| l.contains(c)).unwrap()
}

fn char_score(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

fn to_hash(input: &str) -> HashSet<char> {
    let set: HashSet<char> = input.chars().collect();
    set
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let lines: Vec<&str> = input.lines().collect();
    // lines, groups of 3 lines,
    for n in 0..lines.len() / 3 {
        let start_line = n * 3;
        let a = to_hash(lines[start_line]);
        let b = to_hash(lines[start_line + 1]);
        let c = lines[start_line + 2]
            .chars()
            .find(|c| a.contains(c) && b.contains(c))
            .unwrap();
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
    fn test_shared_chars_one() {
        assert_eq!(shared_char("vJrwpWtwJgWrhcsFMMfFFhFp"), 'p');
        assert_eq!(shared_char("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 'L');
        assert_eq!(shared_char("PmmdzqPrVvPwwTWBwg"), 'P');
        assert_eq!(shared_char("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 'v');
    }
    #[test]
    fn test_char_score() {
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
