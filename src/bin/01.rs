pub fn part_one(input: &str) -> Option<u32> {
    let cal_counts = group_lines(input);
    cal_counts.last().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let cal_counts = group_lines(input);
    let top3_sum: u32 = cal_counts[cal_counts.len() - 3..].iter().sum();
    Some(top3_sum)
}
fn group_lines(input: &str) -> Vec<u32> {
    // split into per-elf.
    let cal_lists = input.split("\n\n");
    //for each group of cals, convert each to int & sum
    let mut sums = cal_lists
        .map(|list| list.lines().map(|cal| cal.parse::<u32>().unwrap()).sum())
        .collect::<Vec<_>>();
    sums.sort();
    sums
}
// Consumes the itera
fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
