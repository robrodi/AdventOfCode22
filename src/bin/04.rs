pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;
    let numbers = input.split(&['\n', ',', '-'][..]);
    let ns: Vec<u32> = numbers.map(|n| n.parse::<u32>().unwrap()).collect();
    for pairs in ns.chunks(4){
        if (pairs[0] >= pairs[2] && pairs[1] <= pairs[3]) || (pairs[0] <= pairs[2] && pairs[1] >= pairs[3]) {
            println!(">{}-{}< and >{}-{}< overlap", pairs[0], pairs[1], pairs[2], pairs[3]);
            count += 1;
        }
    }
    // println!("Ranges: {:?}", ranges);
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
