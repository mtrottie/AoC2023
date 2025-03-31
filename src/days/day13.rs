use crate::{SolutionPair, etc::solution::Solution};

const NUM: &str = "13";

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

fn solution_one(_: String) -> usize {
    0
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
