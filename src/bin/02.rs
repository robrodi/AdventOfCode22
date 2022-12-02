pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse(){

}
fn score(opp: char, mine: char) -> u32 {
    let myPick = match mine {
        'Y' => 2,
        'X' => 1,
        'Z' => 3,
        other => 0
    };

    let mut result: u32 = 0;
    if opp == mine{
        result = 3;
    }

    // 3 wins
    else if (opp == 'A' && mine == 'Y') ||
        (opp == 'B' && mine == 'Z') ||
        (opp == 'C' && mine == 'X') {
            result = 6;
    }
    else{
        result = 0
    }
    
    myPick + result
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_score(){
        assert_eq!(score('A', 'Y'), 8)
    }
}
