#![allow(unused_imports)]
#![allow(clippy::explicit_counter_loop)]
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    thing(input, 4)
}

fn thing(input: &str, window_size: usize) -> Option<u32> {
    let mut i: u32 = 0;
    for w in input.chars().collect::<Vec<char>>().windows(window_size) {
        if w.iter().copied().unique().count() == window_size {
            return Some(i + window_size as u32);
        }
        i += 1;
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    thing(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

// #[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
        assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(part_one("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
        assert_eq!(part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
    }

    // #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));

        assert_eq!(part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb:"), Some(19));
        assert_eq!(part_two("bvwbjplbgvbhsrlpgdmjqwftvncz:"), Some(23));
        assert_eq!(part_two("nppdvjthqldpwncqszvftbrmjlhg:"), Some(23));
        assert_eq!(part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg:"), Some(29));
        assert_eq!(part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw:"), Some(26));
    }
}
