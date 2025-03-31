use std::fs;

use crate::{SolutionPair, etc::solution::Solution};

const NUM: &str = "03";

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
    let input = fs::read_to_string(file_path).expect("Failed to read file");
    let schematic: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    sum_part_numbers(&schematic).try_into().unwrap()
}

fn sum_part_numbers(schematic: &[Vec<char>]) -> i32 {
    let mut total_sum = 0;
    let rows = schematic.len();
    let cols = schematic[0].len();

    for i in 0..rows {
        let mut j = 0;
        while j < cols {
            if schematic[i][j].is_digit(10) {
                let (number, end_j) = extract_number(schematic, i, j);
                if is_part_number(schematic, i, j, end_j) {
                    total_sum += number;
                }
                j = end_j;
            } else {
                j += 1;
            }
        }
    }
    total_sum
}

fn extract_number(schematic: &[Vec<char>], row: usize, start_col: usize) -> (i32, usize) {
    let mut num_str = String::new();
    let mut col = start_col;
    while col < schematic[row].len() && schematic[row][col].is_digit(10) {
        num_str.push(schematic[row][col]);
        col += 1;
    }
    (num_str.parse::<i32>().unwrap(), col)
}

fn is_part_number(schematic: &[Vec<char>], row: usize, start_col: usize, end_col: usize) -> bool {
    let rows = schematic.len();
    let cols = schematic[0].len();

    for i in row.saturating_sub(1)..=usize::min(row + 1, rows - 1) {
        for j in start_col.saturating_sub(1)..=usize::min(end_col, cols - 1) {
            if !schematic[i][j].is_digit(10) && schematic[i][j] != '.' {
                return true;
            }
        }
    }
    false
}

fn solution_two(file_path: String) -> usize {
    let input = fs::read_to_string(file_path).expect("Failed to read file");
    let schematic: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    sum_gear_ratios(&schematic).try_into().unwrap()
}

fn sum_gear_ratios(schematic: &[Vec<char>]) -> i32 {
    let mut total_gear_ratio = 0;
    let rows = schematic.len();
    let cols = schematic[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if schematic[i][j] == '*' {
                let part_numbers = find_adjacent_part_numbers(schematic, i, j);
                if part_numbers.len() == 2 {
                    total_gear_ratio += part_numbers[0] * part_numbers[1];
                }
            }
        }
    }
    total_gear_ratio
}

fn extract_number_two(schematic: &[Vec<char>], row: usize, col: usize) -> (i32, usize, usize) {
    let mut num_str = String::new();
    let mut start_col = col;
    while start_col > 0 && schematic[row][start_col - 1].is_digit(10) {
        start_col -= 1;
    }
    let mut end_col = start_col;
    while end_col < schematic[row].len() && schematic[row][end_col].is_digit(10) {
        num_str.push(schematic[row][end_col]);
        end_col += 1;
    }
    (num_str.parse::<i32>().unwrap(), start_col, end_col - 1)
}

fn find_adjacent_part_numbers(schematic: &[Vec<char>], row: usize, col: usize) -> Vec<i32> {
    let rows = schematic.len();
    let cols = schematic[0].len();
    let mut part_numbers = Vec::new();
    let mut visited_positions = Vec::new();

    for i in row.saturating_sub(1)..=usize::min(row + 1, rows - 1) {
        for j in col.saturating_sub(1)..=usize::min(col + 1, cols - 1) {
            if schematic[i][j].is_digit(10) && !visited_positions.contains(&(i, j)) {
                let (number, start_col, end_col) = extract_number_two(schematic, i, j);
                part_numbers.push(number);
                for k in start_col..=end_col {
                    visited_positions.push((i, k));
                }
            }
        }
    }
    part_numbers
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
            4361
        )
    }

    #[test]
    fn solution_one() {
        assert_eq!(
            super::solution_one(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/one.txt",
                super::NUM
            )),
            532331
        )
    }

    #[test]
    fn example_two() {
        assert_eq!(
            super::solution_two(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/two.test.txt",
                super::NUM
            )),
            467835
        )
    }

    #[test]
    fn solution_two() {
        assert_eq!(
            super::solution_two(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/two.txt",
                super::NUM
            )),
            82301120
        )
    }
}
