#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(shared_char).map(char_score).sum())
}

fn shared_char(line: &str) -> char {
    let (first, second) = line.split_at(line.len() / 2);
    let l = first.chars().collect::<HashSet<char>>();
    let r = second.chars().collect::<HashSet<char>>();
    *l.intersection(&r).next().unwrap()
}

fn char_score(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    Some(
        lines
            .windows(3)
            .step_by(3)
            .map(find_common_badge_score)
            .sum(),
    )
}
fn find_common_badge_score(group: &[&str]) -> u32 {
    let a = group[0].chars().collect::<HashSet<char>>();
    let b = group[1].chars().collect::<HashSet<char>>();
    let c = group[2]
        .chars()
        .find(|c| a.contains(c) && b.contains(c))
        .unwrap();
    char_score(c)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

// #[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }
    // #[test]
    fn test_shared_chars_one() {
        assert_eq!(shared_char("vJrwpWtwJgWrhcsFMMfFFhFp"), 'p');
        assert_eq!(shared_char("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 'L');
        assert_eq!(shared_char("PmmdzqPrVvPwwTWBwg"), 'P');
        assert_eq!(shared_char("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 'v');
    }
    // #[test]
    fn test_char_score() {
        // A = 65
        // a = 97
        assert_eq!(char_score('a'), 1, "a");
        assert_eq!(char_score('A'), 27, "A");
    }

    // #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
