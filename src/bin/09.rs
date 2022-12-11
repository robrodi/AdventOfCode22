use itertools::Itertools;
use std::collections::HashSet;
use substring::Substring;
type Point = euclid::Point2D<i32, i32>;

pub fn part_one(input: &str) -> Option<u32> {
    do_thing(input, 1)
}

fn do_thing(input: &str, rope_length: usize) -> Option<u32> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut head = Point::new(0, 0);
    let mut knots = Vec::new();
    for _ in 0..rope_length {
        knots.push(Point::new(0, 0));
    }

    visited.insert(*knots.last().unwrap());
    for (dir, num) in parse(input) {
        println!("{} {} -> {:?}", dir, num, head);
        for _step in 0..num {
            // move head.
            match dir {
                'U' => head.y -= 1,
                'D' => head.y += 1,
                'L' => head.x -= 1,
                'R' => head.x += 1,
                _ => panic!("WTF"),
            }
            let mut current = head;
            for i in 0..rope_length {
                let last = current;
                // let mut last = if (i == 0) {&head} else {&knots[i -1]};

                // pull a mutable referene to the knot we want to putz with.
                let tail = &mut knots[i];
                // not adjacent
                if (last.x - tail.x).abs() > 1 || (last.y - tail.y).abs() > 1 {
                    if last.x != tail.x && last.y != tail.y {
                        // move diagonally
                        if last.x > tail.x {
                            tail.x += 1;
                        }
                        if last.x < tail.x {
                            tail.x -= 1;
                        }
                        if last.y > tail.y {
                            tail.y += 1;
                        }
                        if last.y < tail.y {
                            tail.y -= 1;
                        }
                    } else {
                        if last.x - tail.x > 1 {
                            tail.x += 1;
                        }
                        if tail.x - last.x > 1 {
                            tail.x -= 1;
                        }
                        if last.y - tail.y > 1 {
                            tail.y += 1;
                        }
                        if tail.y - last.y > 1 {
                            tail.y -= 1;
                        }
                    }
                }
                current = *tail;
            }

            let last = knots.last().unwrap(); //Point::new(0, 0);
                                              // move tail 1 step towards head
            println!("Head: {:?} Tail: {:?}", head, last);

            visited.insert(*last);
        }
    }
    Some(visited.len() as u32)
}

fn parse(input: &str) -> Vec<(char, i32)> {
    let moves = input
        .lines()
        .map(|l| {
            (
                l.chars().nth(0).unwrap(),
                l.substring(2, l.len()).parse::<i32>().unwrap(),
            )
        })
        .collect_vec();
    moves
}

pub fn part_two(input: &str) -> Option<u32> {
    do_thing(input, 9)
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
        let other_input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(part_two(&other_input), Some(36));
    }
}
