use std::collections::HashSet;

const RADIX: u32 = 10;

pub fn part_one(input: &str) -> Option<i32> {
    let grid = Grid::new(input);
    Some(count(&grid))
}
fn count(grid: &Grid) -> i32 {
    let w = grid.width;
    let h = grid.height;
    println!("Grid Size: W: {} H: {}", w, h);

    let mut treez: HashSet<i32> = HashSet::new();
    for y in 0..h {
        let mut cur_max: i32 = -1;
        for x in 0..w {
            let i = grid.get(x, y);
            if i > cur_max {
                treez.insert(grid.get_index(y, x));
                cur_max = i;
            }
        }
        cur_max = -1;

        for x in 0..w {
            let x1 = w - x - 1;
            let i = grid.get(x1, y);
            if i > cur_max {
                let index = grid.get_index(y, x1);
                if !treez.contains(&index) {
                    treez.insert(index);
                }

                cur_max = i;
            }
        }
    }

    for x in 0..w {
        let mut cur_max: i32 = -1;
        // l->r
        for y in 0..h {
            let i = grid.get(x, y);
            if i > cur_max {
                treez.insert(grid.get_index(y, x));
                cur_max = i;
            }
        }
        cur_max = -1;
        for y1 in 0..h {
            let y = h - y1 - 1;
            let i = grid.get(x, y);
            if i > cur_max {
                treez.insert(grid.get_index(y, x));
                cur_max = i;
            }
        }
    }
    display(&treez, w, h);
    treez.len() as i32
}
fn display(input: &HashSet<i32>, w: i32, h: i32) {
    println!();
    for y in 0..h {
        for x in 0..w {
            let i: i32 = w * y + x;
            print!("{}", if input.contains(&i) { "#" } else { "*" });
        }
        println!();
    }
}
#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<i32>,
    width: i32,
    height: i32,
    // is_vis_map: HashSet<i32>
}
impl Grid {
    fn new(input: &str) -> Self {
        let mut result: Vec<i32> = Vec::new();
        let mut w = 0;
        let mut h = 0;
        for line in input.lines() {
            h += 1;
            w = line.len();
            for c in line.chars() {
                result.push(c.to_digit(RADIX).unwrap() as i32);
            }
        }

        Self {
            grid: result,
            width: w as i32,
            height: h,
            //    is_vis_map: HashSet::new()
        }
    }
    fn get(&self, x: i32, y: i32) -> i32 {
        *self.grid.get(self.get_index(y, x) as usize).unwrap()
    }
    fn get_index(&self, y: i32, x: i32) -> i32 {
        self.width * y + x
    }
    // fn visibile(&mut self, y: i32, x: i32) {
    //     let i = self.get_index(y, x);
    //     self.is_vis_map.insert(i);
    //     println!("len: {}", self.is_vis_map.len());
    // }
}
// fn get<u8>(vec: Vec<u8>, x: i32, y: i32, width: i32) -> u8{
//     let index =
//     vec[index]
// }

pub fn part_two(input: &str) -> Option<i32> {
    let grid = Grid::new(input);
    for y in 1..(grid.height - 1){
        for x in 1..(grid.width -1){
            let value = grid.get(x, y);

            let mut scenic = 0;
            // up
            for i in 0..(x-1){

            }
            // down
            // left
            // right

        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
