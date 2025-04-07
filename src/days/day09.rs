use std::fs::{self};

use crate::{SolutionPair, etc::solution::Solution};

const NUM: &str = "09";

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1 = solution_one(format!(
        "/home/mtrottie/code/advent_2023/data/day{NUM}/one.txt"
    ));
    let sol2 = solution_two(format!(
        "/home/mtrottie/code/advent_2023/data/day{NUM}/two.txt"
    ));

    (Solution::from(sol1), Solution::from(sol2))
}

fn solution_one(path: String) -> i64 {
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let lines = contents.lines();
    lines
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let nums: Vec<i64> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            next_value(&nums)
        })
        .sum()
}

fn next_value(history: &Vec<i64>) -> i64 {
    if history.iter().all(|&x| x == history[0]) {
        return history[0];
    }

    let diffs: Vec<i64> = history.windows(2).map(|w| w[1] - w[0]).collect();
    history.last().unwrap() + next_value(&diffs)
}

fn solution_two(path: String) -> i64 {
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let lines = contents.lines();
    lines
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let nums: Vec<i64> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            previous_value(&nums)
        })
        .sum()
}

fn previous_value(history: &Vec<i64>) -> i64 {
    if history.iter().all(|&x| x == history[0]) {
        return history[0];
    }

    let diffs: Vec<i64> = history.windows(2).map(|w| w[1] - w[0]).collect();
    history[0] - previous_value(&diffs)
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
            114
        )
    }

    #[test]
    fn solution_one() {
        assert_eq!(
            super::solution_one(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/one.txt",
                super::NUM
            )),
            1806615041
        )
    }

    #[test]
    fn example_two() {
        assert_eq!(
            super::solution_two(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/two.test.txt",
                super::NUM
            )),
            2
        )
    }

    #[test]
    fn solution_two() {
        assert_eq!(
            super::solution_two(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/two.txt",
                super::NUM
            )),
            1211
        )
    }
}
