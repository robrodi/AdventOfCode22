use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(shared_char).map(char_score).sum())
}

fn shared_char(line: &str) -> char {
    let mid = line.len() / 2;
    let (first, second) = line.split_at(mid);

    let l = to_hash(first);
    let r = to_hash(second);
    *l.intersection(&r).next().unwrap()
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
    let lines: Vec<&str> = input.lines().collect();
    Some(
        lines
            .windows(3)
            .step_by(3)
            .map(find_common_badge_score)
            .sum(),
    )
}
fn find_common_badge_score(group: &[&str]) -> u32 {
    let a = to_hash(group[0]);
    let b = to_hash(group[1]);

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
