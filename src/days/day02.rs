use std::{collections::HashMap, fs::read_to_string};

use crate::{SolutionPair, etc::solution::Solution};

const NUM: &str = "02";

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
    let max_counts: HashMap<&str, i32> = [("red", 12), ("green", 13), ("blue", 14)]
        .iter()
        .cloned()
        .collect();

    let mut sum_of_possible_game_ids = 0;

    for game in read_to_string(file_path).unwrap().lines() {
        let parts: Vec<&str> = game.split(": ").collect();
        let game_id: usize = parts[0].split_whitespace().nth(1).unwrap().parse().unwrap();
        let draws: Vec<&str> = parts[1].split("; ").collect();
        let mut possible = true;

        for draw in draws.iter() {
            let cubes: Vec<&str> = draw.split(", ").collect();

            for cube in cubes.iter() {
                let cube_parts: Vec<&str> = cube.split_whitespace().collect();
                let count: i32 = cube_parts[0].parse().unwrap();
                let color = cube_parts[1];

                if count > *max_counts.get(color).unwrap() {
                    possible = false;
                    break;
                }
            }

            if !possible {
                break;
            }
        }

        if possible {
            sum_of_possible_game_ids += game_id;
        }
    }

    sum_of_possible_game_ids
}

fn solution_two(file_path: String) -> usize {
    let mut total_power = 0;

    for game in read_to_string(file_path).unwrap().lines() {
        let parts: Vec<&str> = game.split(": ").collect();
        let draws: Vec<&str> = parts[1].split("; ").collect();

        let mut min_counts: HashMap<&str, i32> = HashMap::new();
        min_counts.insert("red", 0);
        min_counts.insert("green", 0);
        min_counts.insert("blue", 0);

        for draw in draws.iter() {
            let cubes: Vec<&str> = draw.split(", ").collect();

            for cube in cubes.iter() {
                let cube_parts: Vec<&str> = cube.split_whitespace().collect();
                let count: i32 = cube_parts[0].parse().unwrap();
                let color = cube_parts[1];

                if count > *min_counts.get(color).unwrap() {
                    min_counts.insert(color, count);
                }
            }
        }

        let power = min_counts["red"] * min_counts["green"] * min_counts["blue"];
        total_power += power;
    }

    total_power as usize
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
            8
        )
    }

    #[test]
    fn solution_one() {
        assert_eq!(
            super::solution_one(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/one.txt",
                super::NUM
            )),
            2283
        )
    }

    #[test]
    fn example_two() {
        assert_eq!(
            super::solution_two(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/two.test.txt",
                super::NUM
            )),
            2286
        )
    }

    #[test]
    fn solution_two() {
        assert_eq!(
            super::solution_two(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/two.txt",
                super::NUM
            )),
            78669
        )
    }
}
