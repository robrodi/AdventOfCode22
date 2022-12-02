pub fn part_one(input: &str) -> Option<i32> {
    let scores: Vec<i32> = input.lines().map(score_l).collect();
    Some(scores.iter().sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let scores: Vec<i32> = input.lines().map(score2).collect();
    Some(scores.iter().sum())
}

fn score2(l: &str) -> i32 {
    let opp = l.chars().next().unwrap();
    match l.chars().nth(2).unwrap() {
        // outcome is char[2]
        'X' => lose_score(opp),
        'Y' => 3 + draw_score(opp),
        'Z' => 6 + win_score(opp),
        _ => 0,
    }
}

fn draw_score(input: char) -> i32 {
    opp_to_int(input)
}
fn win_score(input: char) -> i32 {
    match input {
        'A' => 2, // rock
        'B' => 3, // paper
        'C' => 1, // scissors
        _ => 0,
    }
}
fn lose_score(input: char) -> i32 {
    match input {
        'A' => 3, // rock
        'B' => 1, // paper
        'C' => 2, // scissors
        _ => 0,
    }
}
fn opp_to_int(i: char) -> i32 {
    i as i32 - 64
}
fn my_to_int(i: char) -> i32 {
    i as i32 - 87
}
fn score_l(l: &str) -> i32 {
    score(l.chars().next().unwrap(), l.chars().nth(2).unwrap())
}
fn score(opp: char, mine: char) -> i32 {
    let opp_pick = opp_to_int(opp);
    let my_pick = my_to_int(mine);

    let mut result: i32 = 0;
    if opp_pick == my_pick {
        result = 3;
    } // tie
    if my_pick - opp_pick % 3 == 1 {
        result = 6;
    }

    my_pick + result
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
        assert_eq!(part_two(&input), Some(12));
    }

    #[test]
    fn test_score() {
        assert_eq!('A'.to_digit(16).unwrap() - 9, 1, "a");
        assert_eq!('B'.to_digit(16).unwrap(), 11, "b");
        assert_eq!('C'.to_digit(16).unwrap(), 12, "c");
        assert_eq!('X' as u32 - 87, 1, "x");

        assert_eq!(score('A', 'Y'), 8, "Paper, Win");
        assert_eq!(score('B', 'X'), 1, "Rock, Loss");
        assert_eq!(score('C', 'Z'), 6, "scissor, tie");
    }
    #[test]
    fn test_score_2() {
        assert_eq!(score2("A Y"), 4, "Rock, Draw");
        assert_eq!(score2("B X"), 1, "Paper, Loss");
        assert_eq!(score2("C Z"), 7, "scissor, Win");
        assert_eq!(score2("A Z"), 8, "rock, Win");
        assert_eq!(score2("B Z"), 9, "paper, Win");
    }
}
