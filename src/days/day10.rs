use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

use crate::{SolutionPair, etc::solution::Solution};

const NUM: &str = "10";

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

type Point = (i32, i32);

fn solution_one(path: String) -> i32 {
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let grid: HashMap<Point, char> = contents
        .lines()
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect();

    let start = grid
        .iter()
        .find(|(_, c)| **c == 'S')
        .map(|(&pos, _)| pos)
        .unwrap();

    let directions: HashMap<char, Vec<(i32, i32)>> = [
        ('|', vec![(0, -1), (0, 1)]),
        ('-', vec![(-1, 0), (1, 0)]),
        ('L', vec![(0, -1), (1, 0)]),
        ('J', vec![(0, -1), (-1, 0)]),
        ('7', vec![(0, 1), (-1, 0)]),
        ('F', vec![(0, 1), (1, 0)]),
        ('S', vec![]),
    ]
    .into_iter()
    .collect();

    let reverse: HashMap<(i32, i32), (i32, i32)> = [
        ((0, -1), (0, 1)),
        ((0, 1), (0, -1)),
        ((-1, 0), (1, 0)),
        ((1, 0), (-1, 0)),
    ]
    .into_iter()
    .collect();

    let mut neighbors = vec![];
    for dir in &[(0, -1), (0, 1), (-1, 0), (1, 0)] {
        let np = (start.0 + dir.0, start.1 + dir.1);
        if let Some(&c) = grid.get(&np) {
            if let Some(conns) = directions.get(&c) {
                if conns.contains(&reverse[dir]) {
                    neighbors.push(*dir);
                }
            }
        }
    }

    let s_pipe = match neighbors.as_slice() {
        [(0, -1), (0, 1)] => '|',
        [(-1, 0), (1, 0)] => '-',
        [(0, -1), (1, 0)] => 'L',
        [(0, -1), (-1, 0)] => 'J',
        [(0, 1), (-1, 0)] => '7',
        [(0, 1), (1, 0)] => 'F',
        _ => panic!("Invalid start connections"),
    };

    let mut grid = grid.clone();
    grid.insert(start, s_pipe);

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();

    visited.insert(start);
    queue.push_back((start, 0));
    distances.insert(start, 0);

    while let Some((pos, dist)) = queue.pop_front() {
        let pipe = grid[&pos];
        if let Some(dirs) = directions.get(&pipe) {
            for dir in dirs {
                let next = (pos.0 + dir.0, pos.1 + dir.1);
                if visited.contains(&next) {
                    continue;
                }
                if let Some(&next_pipe) = grid.get(&next) {
                    if let Some(next_dirs) = directions.get(&next_pipe) {
                        if next_dirs.contains(&reverse[dir]) {
                            visited.insert(next);
                            distances.insert(next, dist + 1);
                            queue.push_back((next, dist + 1));
                        }
                    }
                }
            }
        }
    }

    distances.values().max().unwrap().to_owned()
}

fn solution_two(_: String) -> usize {
    0
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
            6968
        )
    }

    #[test]
    fn example_two() {
        assert_eq!(
            super::solution_two(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/two.test.txt",
                super::NUM
            )),
            10
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
