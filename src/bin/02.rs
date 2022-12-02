pub fn part_one(input: &str) -> Option<u32> {
    let scores: Vec<u32> = input.lines().map(|l|  score_l(l)).collect();
    Some(scores.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let scores: Vec<u32>  = input.lines().map(|l| score2(l)).collect();
    Some(scores.iter().sum())
}
fn score2(l: &str) -> u32{
    let opp = l.chars().nth(0).unwrap();
    let outcome = l.chars().nth(2).unwrap();
    match outcome{
        'X' => lose_score(opp),
        'Y' => draw_score(opp),
        'Z' => win_score(opp),
        _ => 0
    }
}

fn draw_score (input: char) -> u32{
    3 + match input {
        'A' => 1, // rock
        'B' => 2, // paper
        'C' => 3, // scissors
        other => 0
    }
}
fn win_score (input: char) -> u32{
    6 + match input {
        'A' => 2, // rock
        'B' => 3, // paper
        'C' => 1, // scissors
        other => 0
    }
}
fn lose_score (input: char) -> u32{
    match input {
        'A' => 3, // rock
        'B' => 1, // paper
        'C' => 2, // scissors
        other => 0
    }
}
fn score_l(l: &str) -> u32{
    score(l.chars().nth(0).unwrap(), l.chars().nth(2).unwrap())
}
fn score(opp: char, mine: char) -> u32 {
    let my_pick = match mine {
        'Y' => 2,
        'X' => 1,
        'Z' => 3,
        other => 0
    };

    let result: u32;
    if (opp == 'C' && mine == 'Z') || // tie scissor
        (opp == 'B' && mine == 'Y') || // tie rock
        (opp == 'A' && mine == 'X'){ // tie paper
        result = 3;
    }
    // 3 wins
    else if (opp == 'A' && mine == 'Y') || // rock, paper
        (opp == 'B' && mine == 'Z') || // Paper, Scissors
        (opp == 'C' && mine == 'X') { // scissors, Rock
            result = 6;
    }
    else{ // loss
        result = 0
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
    fn test_score(){
        let s = 's';
        assert_eq!(score('A', 'Y'), 8, "Paper, Win");
        assert_eq!(score('B', 'X'), 1, "Rock, Loss");
        assert_eq!(score('C', 'Z'), 6, "scissor, tie");
    }
    #[test]
    fn test_score_2(){
        assert_eq!(score2("A Y"), 4, "Rock, Draw");
        assert_eq!(score2("B X"), 1, "Paper, Loss");
        assert_eq!(score2("C Z"), 7, "scissor, Win");
    }
}
