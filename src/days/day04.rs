use std::fs;

use crate::{SolutionPair, etc::solution::Solution};

const NUM: &str = "04";

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

fn solution_one(file_path: String) -> usize {
    calculate_scratchcard_points(&file_path).try_into().unwrap()
}

fn calculate_scratchcard_points(input: &str) -> i32 {
    let input = fs::read_to_string(input).expect("Failed to read file");

    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() != 2 {
                return 0;
            }
            let winning_numbers: Vec<i32> = parts[0]
                .split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();
            let my_numbers: Vec<i32> = parts[1]
                .split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();
            let matches = my_numbers
                .iter()
                .filter(|&&n| winning_numbers.contains(&n))
                .count();
            if matches == 0 { 0 } else { 1 << (matches - 1) }
        })
        .sum()
}

fn solution_two(file_path: String) -> usize {
    let input = fs::read_to_string(file_path).expect("Failed to read file");
    process_scratchcards(&input).try_into().unwrap()
}

fn process_scratchcards(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut card_counts = vec![1; lines.len()]; // Each card starts with 1 copy

    for (index, line) in lines.iter().enumerate() {
        let Some((_, card_data)) = line.split_once(':') else {
            continue;
        }; // Safely extract after ':'
        let parts: Vec<&str> = card_data.trim().split('|').collect();

        if parts.len() != 2 {
            continue; // Skip invalid lines
        }

        let winning_numbers: Vec<i32> = parts[0]
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();
        let my_numbers: Vec<i32> = parts[1]
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();
        let matches = my_numbers
            .iter()
            .filter(|&&n| winning_numbers.contains(&n))
            .count();

        for offset in 1..=matches {
            if index + offset < lines.len() {
                card_counts[index + offset] += card_counts[index];
            }
        }
    }

    card_counts.iter().sum()
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
            13
        )
    }

    #[test]
    fn solution_one() {
        assert_eq!(
            super::solution_one(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/one.txt",
                super::NUM
            )),
            26443
        )
    }

    #[test]
    fn example_two() {
        assert_eq!(
            super::solution_two(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/two.test.txt",
                super::NUM
            )),
            30
        )
    }

    #[test]
    fn solution_two() {
        assert_eq!(
            super::solution_two(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/two.txt",
                super::NUM
            )),
            6284877
        )
    }
}
