use std::slice::Windows;

use colored::Colorize;

pub fn part_one(input: &str) -> Option<u32> {
    let mut cpu = Processor::new();
    cpu.exec(input);
    let measurement_points = [20, 60, 100, 140, 180, 220];
    for i in measurement_points {
        println!(
            "{}     {} : {}",
            i as i32 * cpu.register_history[i - 1],
            i,
            cpu.register_history[i - 1]
        );
    }
    let measures = measurement_points
        .map(|p| p as i32 * cpu.register_history[p - 1])
        .iter()
        .sum::<i32>();
    Some(measures as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cpu = Processor::new();
    cpu.exec(input);
    let mut gpu = GPU::new(40);
    gpu.render(cpu.register_history);
    None
}

struct Processor {
    X: i32,
    clock: usize,
    register_history: Vec<i32>,
}
impl Processor {
    fn new() -> Self {
        Self {
            X: 1,
            clock: 0,
            register_history: vec![1], // register starts at 1
        }
    }
    fn noop(&mut self) {
        println!(
            "{} clock [{:0>3}] Register      {: >3}",
            format!("NOOP ").blue(),
            self.clock,
            self.X
        );

        self.clock += 1;
        self.register_history.push(self.X);
    }
    fn add_x(&mut self, value: i32) {
        println!(
            "{} clock [{:0>3}] Register(pre) {:0>3} value: {: >4} ",
            format!("ADDX=").red(),
            self.clock,
            self.X,
            value
        );
        self.clock += 1;
        self.register_history.push(self.X);

        println!(
            "{} clock [{:0>3}] Register(pre) {: >3} value: {: >4} ",
            format!("ADDX+").red(),
            self.clock,
            self.X,
            value
        );
        self.clock += 1;
        self.X += value;
        self.register_history.push(self.X);
    }
    fn exec(&mut self, input: &str) {
        for m in input.lines() {
            match m {
                "noop" => {
                    self.noop();
                }
                _ => {
                    let x = m[5..m.len()].parse::<i32>().unwrap();
                    self.add_x(x);
                }
            };
        }
    }
}

struct GPU {
    //?
    width: usize,
}
impl GPU {
    fn new(width: usize) -> Self {
        Self { width }
    }
    fn render(&self, register_history: Vec<i32>) {
        for i in 0..register_history.len() {
            if i % self.width == 0 {
                println!();
            }
            let value = register_history[i];
            let pixel = i as i32 % 40;
            if self.is_hit(pixel, value) {
                print!("#");
            } else {
                print!(".");
            }
        }
    }
    fn is_hit(&self, pixel: i32, value: i32) -> bool {
        // direct hit, one left, one right
        pixel == value
            || (pixel % 40 != 0 && pixel - 1 == value)
            || (pixel % 40 != 0 && pixel + 1 == value)
    }
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let input = "noop
addx 3
addx -5";
        let mut cpu = Processor::new();
        cpu.exec(input);
        let expected = [1, 1, 1, 4, 4];
        for i in 0..expected.len() {
            assert_eq!(cpu.register_history[i], expected[i], "DURING cycle {}", i);
        }
    }
    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
