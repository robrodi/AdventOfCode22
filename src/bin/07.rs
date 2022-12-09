#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let tree = process(input);
    let total_size = tree
        .get_all_children()
        .iter()
        .filter(|x| x.is_dir && x.size <= 100000)
        .fold(0, |a, x| a + x.size);
    Some(total_size as u32)
}

fn process(raw: &str) -> File {
    let mut tree = File::new("root");
    let mut path = Vec::new();
    for line in raw.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();

        if parts[..2] == ["$", "cd"] {
            match parts[2] {
                "/" => continue,
                ".." => {
                    path.pop().unwrap();
                    continue;
                }
                _ => {}
            }

            let parent = tree.get_path(&path);
            path.push(parts[2].to_owned());
            if parent.children.iter().any(|x| x.name == parts[2]) {
                continue;
            }

            parent.children.push(File::new(parts[2]));
            continue;
        }

        if parts[0] == "dir" {
            let parent = tree.get_path(&path);
            if let Some(i) = parent.children.iter_mut().find(|x| x.name == parts[1]) {
                i.is_dir = true;
                continue;
            }

            let mut child = File::new(parts[1]);
            child.is_dir = true;
            parent.children.push(child);
            continue;
        }

        if let Ok(i) = parts[0].parse::<usize>() {
            let mut child = File::new(parts[1]);
            child.size = i;
            tree.get_path(&path).children.push(child);
        }
    }
    tree.propagate_size();

    tree
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct File {
    name: String,
    size: usize,
    children: Vec<File>,
    is_dir: bool,
}

impl File {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            size: 0,
            children: Vec::new(),
            is_dir: false,
        }
    }

    fn get_path(&mut self, path: &[String]) -> &mut Self {
        let mut current = self;
        for part in path {
            current = current
                .children
                .iter_mut()
                .find(|f| f.name == *part)
                .unwrap();
        }

        current
    }

    fn propagate_size(&mut self) -> usize {
        for i in &mut self.children {
            self.size += i.propagate_size();
        }

        self.size
    }

    fn get_all_children(&self) -> HashSet<Self> {
        let mut out = HashSet::new();

        for i in &self.children {
            out.insert(i.clone());
            out.extend(i.get_all_children());
        }

        out
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let folders = process(input);
    let needed_space = 30000000 - (70000000 - folders.size);
    let folder_vec = folders.get_all_children();
    let mut folder_vec = folder_vec.iter().collect::<Vec<_>>();
    folder_vec.sort_by(|a, b| a.size.cmp(&b.size));

    let val = folder_vec
        .iter()
        .find(|x| x.is_dir && x.size > needed_space)
        .unwrap()
        .size;
    Some(val as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

// #[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    // #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
