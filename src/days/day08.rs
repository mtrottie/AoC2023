use std::{collections::HashMap, fs};

use crate::{SolutionPair, etc::solution::Solution};

const NUM: &str = "08";

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: usize = solution_one(format!(
        "/home/mtrottie/code/advent_2023/data/day{NUM}/one.txt"
    ));
    let sol2: usize = solution_two(format!(
        "/home/mtrottie/code/advent_2023/data/day{NUM}/two.txt"
    ));

    (Solution::from(sol1), Solution::from(sol2))
}

fn solution_one(path: String) -> usize {
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let mut lines = contents.lines();
    let instructions = lines.next().unwrap().chars().collect::<Vec<_>>();
    lines.next();

    let mut map = HashMap::new();
    for line in lines {
        let parts: Vec<&str> = line.split(" = ").collect();
        let node = parts[0];
        let targets = parts[1].trim_matches(|c| c == '(' || c == ')');
        let split: Vec<&str> = targets.split(", ").collect();
        map.insert(node, (split[0], split[1]));
    }

    let mut current = "AAA";
    let mut steps = 0;
    let mut i = 0;

    while current != "ZZZ" {
        let (left, right) = map[current];
        current = if instructions[i] == 'L' { left } else { right };
        i = (i + 1) % instructions.len();
        steps += 1;
    }

    steps
}

fn solution_two(path: String) -> usize {
    let contents = fs::read_to_string(path).expect("Failed to read file");

    let mut lines = contents.lines();
    let instructions = lines.next().unwrap().chars().collect::<Vec<_>>();
    lines.next();

    let mut map = HashMap::new();
    for line in lines {
        let parts: Vec<&str> = line.split(" = ").collect();
        let node = parts[0];
        let targets = parts[1].trim_matches(|c| c == '(' || c == ')');
        let split: Vec<&str> = targets.split(", ").collect();
        map.insert(
            node.to_string(),
            (split[0].to_string(), split[1].to_string()),
        );
    }

    let start_nodes: Vec<String> = map.keys().filter(|k| k.ends_with('A')).cloned().collect();

    let mut cycle_lengths = Vec::new();

    for start in start_nodes {
        let mut current = start;
        let mut i = 0;
        let mut steps = 0;

        loop {
            if current.ends_with('Z') {
                break;
            }
            let (left, right) = &map[&current];
            current = if instructions[i] == 'L' {
                left.clone()
            } else {
                right.clone()
            };
            i = (i + 1) % instructions.len();
            steps += 1;
        }

        cycle_lengths.push(steps);
    }

    cycle_lengths.into_iter().reduce(|a, b| lcm(a, b)).unwrap()
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_one() {
        assert_eq!(
            super::solution_one(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/one.test.txt",
                super::NUM
            )),
            6
        )
    }

    #[test]
    fn solution_one() {
        assert_eq!(
            super::solution_one(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/one.txt",
                super::NUM
            )),
            20221
        )
    }

    #[test]
    fn example_two() {
        assert_eq!(
            super::solution_two(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/two.test.txt",
                super::NUM
            )),
            6
        )
    }

    #[test]
    fn solution_two() {
        assert_eq!(
            super::solution_two(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/two.txt",
                super::NUM
            )),
            14616363770447
        )
    }
}
