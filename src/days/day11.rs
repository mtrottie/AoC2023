use std::{collections::HashSet, fs};

use crate::{SolutionPair, etc::solution::Solution};

const NUM: &str = "11";

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
    let raw_input = fs::read_to_string(path).unwrap();
    let map: Vec<Vec<char>> = raw_input.lines().map(|l| l.chars().collect()).collect();
    let height = map.len();
    let width = map[0].len();

    let mut galaxies = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == '#' {
                galaxies.push((x as i64, y as i64));
            }
        }
    }

    let mut empty_rows = HashSet::new();
    let mut empty_cols = HashSet::new();

    'outer_row: for y in 0..height {
        for x in 0..width {
            if map[y][x] == '#' {
                continue 'outer_row;
            }
        }
        empty_rows.insert(y);
    }

    'outer_col: for x in 0..width {
        for y in 0..height {
            if map[y][x] == '#' {
                continue 'outer_col;
            }
        }
        empty_cols.insert(x);
    }

    let mut expanded = Vec::new();
    for (x, y) in galaxies {
        let extra_cols = (0..x as usize).filter(|c| empty_cols.contains(c)).count() as i64;
        let extra_rows = (0..y as usize).filter(|r| empty_rows.contains(r)).count() as i64;
        expanded.push((x + extra_cols, y + extra_rows));
    }

    let mut sum = 0;
    for i in 0..expanded.len() {
        for j in i + 1..expanded.len() {
            let dx = (expanded[i].0 - expanded[j].0).abs();
            let dy = (expanded[i].1 - expanded[j].1).abs();
            sum += dx + dy;
        }
    }

    sum
}

fn solution_two(path: String) -> i64 {
    let expansion = 1_000_000;
    let raw_input = fs::read_to_string(path).unwrap();
    let map: Vec<Vec<char>> = raw_input.lines().map(|l| l.chars().collect()).collect();
    let height = map.len();
    let width = map[0].len();

    let mut galaxies = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == '#' {
                galaxies.push((x as i64, y as i64));
            }
        }
    }

    let mut empty_rows = HashSet::new();
    let mut empty_cols = HashSet::new();

    'outer_row: for y in 0..height {
        for x in 0..width {
            if map[y][x] == '#' {
                continue 'outer_row;
            }
        }
        empty_rows.insert(y);
    }

    'outer_col: for x in 0..width {
        for y in 0..height {
            if map[y][x] == '#' {
                continue 'outer_col;
            }
        }
        empty_cols.insert(x);
    }

    let mut expanded = Vec::new();
    for (x, y) in galaxies {
        let extra_cols = (0..x as usize).filter(|c| empty_cols.contains(c)).count() as i64;
        let extra_rows = (0..y as usize).filter(|r| empty_rows.contains(r)).count() as i64;
        let new_x = x + extra_cols * (expansion - 1);
        let new_y = y + extra_rows * (expansion - 1);
        expanded.push((new_x, new_y));
    }

    let mut sum = 0;
    for i in 0..expanded.len() {
        for j in i + 1..expanded.len() {
            let dx = (expanded[i].0 - expanded[j].0).abs();
            let dy = (expanded[i].1 - expanded[j].1).abs();
            sum += dx + dy;
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
            374
        )
    }

    #[test]
    fn solution_one() {
        assert_eq!(
            super::solution_one(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/one.txt",
                super::NUM
            )),
            9623138
        )
    }

    #[test]
    fn example_two() {
        assert_eq!(
            super::solution_two(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/two.test.txt",
                super::NUM
            )),
            82000210
        )
    }

    #[test]
    fn solution_two() {
        assert_eq!(
            super::solution_two(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/two.txt",
                super::NUM
            )),
            726820169514
        )
    }
}
