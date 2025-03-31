use std::{collections::HashMap, fs::read_to_string};

use crate::{SolutionPair, etc::solution::Solution};

const NUM: &str = "01";

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
    let mut sum = 0;
    for line in read_to_string(file_path).unwrap().lines() {
        let row: Vec<char> = line.chars().collect::<Vec<char>>();
        let first_digit = row.iter().find(|&&c| c.is_digit(10));
        let last_digit = row.iter().rev().find(|&&c| c.is_digit(10));

        match (first_digit, last_digit) {
            (Some(f), Some(l)) => {
                sum += format!("{}{}", f, l).parse::<usize>().unwrap();
            }
            _ => panic!("No digits found in line: {}", line),
        }
    }

    sum
}

fn solution_two(file_path: String) -> usize {
    let word_to_digit: HashMap<&str, char> = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]
    .iter()
    .cloned()
    .collect();

    let mut sum = 0;

    for line in read_to_string(file_path).unwrap().lines() {
        let mut first_digit: Option<char> = None;
        let mut last_digit: Option<char> = None;

        for i in 0..line.len() {
            if line.chars().nth(i).unwrap().is_digit(10) {
                first_digit = Some(line.chars().nth(i).unwrap());
                break;
            }
            for (word, &digit) in &word_to_digit {
                if line[i..].starts_with(word) {
                    first_digit = Some(digit);
                    break;
                }
            }
            if first_digit.is_some() {
                break;
            }
        }

        for i in (0..line.len()).rev() {
            if line.chars().nth(i).unwrap().is_digit(10) {
                last_digit = Some(line.chars().nth(i).unwrap());
                break;
            }
            for (word, &digit) in &word_to_digit {
                if line[..=i].ends_with(word) {
                    last_digit = Some(digit);
                    break;
                }
            }
            if last_digit.is_some() {
                break;
            }
        }

        match (first_digit, last_digit) {
            (Some(f), Some(l)) => sum += format!("{}{}", f, l).parse::<usize>().unwrap(),
            _ => panic!("Shouldn't happen, faulty input."),
        }
    }

    sum
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
            142
        )
    }

    #[test]
    fn solution_one() {
        assert_eq!(
            super::solution_one(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/one.txt",
                super::NUM
            )),
            54338
        )
    }

    #[test]
    fn example_two() {
        assert_eq!(
            super::solution_two(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/two.test.txt",
                super::NUM
            )),
            281
        )
    }

    #[test]
    fn solution_two() {
        assert_eq!(
            super::solution_two(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/two.txt",
                super::NUM
            )),
            53389
        )
    }
}
