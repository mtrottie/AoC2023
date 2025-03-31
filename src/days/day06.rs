use std::fs;

use crate::{SolutionPair, etc::solution::Solution};

const NUM: &str = "06";

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
    let input = fs::read_to_string(file_path).unwrap();
    let lines: Vec<&str> = input.trim().lines().collect();

    let times: Vec<usize> = lines[0]
        .replace("Time:", "")
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let distances: Vec<usize> = lines[1]
        .replace("Distance:", "")
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let races: Vec<(usize, usize)> = times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &record)| (time, record))
        .collect();

    let total_ways: usize = races
        .iter()
        .map(|&(time, record)| calculate_ways_to_win(time, record))
        .product();

    total_ways
}

fn calculate_ways_to_win(time: usize, record: usize) -> usize {
    let mut ways = 0;

    for hold_time in 0..=time {
        let speed = hold_time;
        let travel_time = time - hold_time;
        let distance = speed * travel_time;

        if distance > record {
            ways += 1;
        }
    }

    ways
}

fn solution_two(file_path: String) -> usize {
    let input = fs::read_to_string(file_path).unwrap();
    let lines: Vec<&str> = input.trim().lines().collect();
    let time: usize = lines[0]
        .replace("Time:", "")
        .replace(" ", "")
        .trim()
        .parse()
        .unwrap();
    let record: usize = lines[1]
        .replace("Distance:", "")
        .replace(" ", "")
        .trim()
        .parse()
        .unwrap();
    calculate_ways_to_win(time, record)
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
            288
        )
    }

    #[test]
    fn solution_one() {
        assert_eq!(
            super::solution_one(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/one.txt",
                super::NUM
            )),
            293046
        )
    }

    #[test]
    fn example_two() {
        assert_eq!(
            super::solution_two(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/two.test.txt",
                super::NUM
            )),
            71503
        )
    }

    #[test]
    fn solution_two() {
        assert_eq!(
            super::solution_two(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/two.txt",
                super::NUM
            )),
            35150181
        )
    }
}
