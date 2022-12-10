use std::{collections::HashSet, fmt};

use euclid::Point2D;
use itertools::Itertools;
use substring::Substring;
type Point = euclid::Point2D<i32, i32>;

pub fn part_one(input: &str) -> Option<u32> {
    let moves = input
        .lines()
        .map(|l| {
            (
                l.chars().nth(0).unwrap(),
                l.substring(2, l.len()).parse::<i32>().unwrap(),
            )
        })
        .collect_vec();

    let mut visited: HashSet<Point> = HashSet::new();
    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);
    visited.insert(tail);

    for (dir, num) in moves {
        println!("{} {} -> {:?}", dir, num, head);
        for step in 0..num {
            // move head.
            match dir {
                'U' => head.y -= 1,
                'D' => head.y += 1,
                'L' => head.x -= 1,
                'R' => head.x += 1,
                _ => panic!("WTF"),
            }
            
            // not adjacent
            if (head.x - tail.x).abs() > 1 || (head.y - tail.y).abs() > 1{
                if head.x != tail.x && head.y != tail.y { // move diagonally
                    if head.x > tail.x { tail.x += 1; }
                    if head.x < tail.x { tail.x -= 1; }
                    if head.y > tail.y { tail.y += 1; }
                    if head.y < tail.y { tail.y -= 1; }
                }
                else{
                    if head.x - tail.x > 1 { tail.x += 1; }
                    if tail.x - head.x > 1 { tail.x -= 1; }
                    if head.y - tail.y > 1 { tail.y += 1; }
                    if tail.y - head.y > 1 { tail.y -= 1; }
                }
            }
            
            // move tail 1 step towards head
            println!("Head: {:?} Tail: {:?}", head, tail);

            visited.insert(tail);
        }
        
    }
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
