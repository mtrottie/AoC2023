use std::{cmp::Ordering, collections::HashMap, fs};

use itertools::Itertools;

use crate::{SolutionPair, etc::solution::Solution};

const NUM: &str = "07";

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
    let hands = fs::read_to_string(file_path).unwrap();
    let mut parsed_hands: Vec<Hand> = hands
        .lines()
        .map(|entry| {
            let parts: Vec<&str> = entry.split_whitespace().collect();
            let cards = parts[0];
            let bid: usize = parts[1].parse().unwrap();
            Hand::new(cards, bid)
        })
        .collect();

    parsed_hands.sort_by(|a, b| {
        if a.hand_type == b.hand_type {
            for (a_card, b_card) in a.cards.iter().zip(b.cards.iter()) {
                let card_value_map: HashMap<char, i32> = vec![
                    ('2', 2),
                    ('3', 3),
                    ('4', 4),
                    ('5', 5),
                    ('6', 6),
                    ('7', 7),
                    ('8', 8),
                    ('9', 9),
                    ('T', 10),
                    ('J', 11),
                    ('Q', 12),
                    ('K', 13),
                    ('A', 14),
                ]
                .into_iter()
                .collect();
                let value_a = card_value_map.get(a_card).unwrap();
                let value_b = card_value_map.get(b_card).unwrap();

                if value_a < value_b {
                    return Ordering::Less;
                } else if value_a > value_b {
                    return Ordering::Greater;
                }
            }
            Ordering::Equal
        } else {
            a.hand_type.cmp(&b.hand_type)
        }
    });

    for (rank, hand) in parsed_hands.iter().enumerate() {
        println!(
            "Rank {}: Hand: {:?}, Bid: {}, Type: {:?}",
            rank + 1,
            hand.cards,
            hand.bid,
            hand.hand_type
        );
    }

    let total_winnings: usize = parsed_hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank + 1))
        .sum();

    total_winnings
}

#[derive(Debug, PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        use HandType::*;
        match (self, other) {
            (FiveOfAKind, FiveOfAKind) => Ordering::Equal,
            (FiveOfAKind, _) => Ordering::Greater,
            (_, FiveOfAKind) => Ordering::Less,
            (FourOfAKind, FourOfAKind) => Ordering::Equal,
            (FourOfAKind, _) => Ordering::Greater,
            (_, FourOfAKind) => Ordering::Less,
            (FullHouse, FullHouse) => Ordering::Equal,
            (FullHouse, _) => Ordering::Greater,
            (_, FullHouse) => Ordering::Less,
            (ThreeOfAKind, ThreeOfAKind) => Ordering::Equal,
            (ThreeOfAKind, _) => Ordering::Greater,
            (_, ThreeOfAKind) => Ordering::Less,
            (TwoPair, TwoPair) => Ordering::Equal,
            (TwoPair, _) => Ordering::Greater,
            (_, TwoPair) => Ordering::Less,
            (OnePair, OnePair) => Ordering::Equal,
            (OnePair, _) => Ordering::Greater,
            (_, OnePair) => Ordering::Less,
            (HighCard, HighCard) => Ordering::Equal,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<char>,
    hand_type: HandType,
    bid: usize,
}

impl Hand {
    fn new(cards: &str, bid: usize) -> Self {
        let card_vec: Vec<char> = cards.chars().collect();
        let hand_type = Hand::determine_hand_type(&card_vec);
        Hand {
            cards: card_vec,
            hand_type,
            bid,
        }
    }

    fn determine_hand_type(cards: &[char]) -> HandType {
        let mut counts = HashMap::new();
        for &card in cards {
            *counts.entry(card).or_insert(0) += 1;
        }

        let mut count_values: Vec<_> = counts.values().cloned().collect();
        count_values.sort_unstable_by(|a, b| b.cmp(a));
        match count_values.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

fn solution_two(file_path: String) -> usize {
    let hands = fs::read_to_string(file_path).unwrap();
    let mut parsed_hands: Vec<Handv2> = hands
        .lines()
        .map(|entry| {
            let parts: Vec<&str> = entry.split_whitespace().collect();
            let cards = parts[0];
            let bid: usize = parts[1].parse().unwrap();
            Handv2::new(cards, bid)
        })
        .collect();

    parsed_hands.sort_by(|a, b| {
        if a.hand_type == b.hand_type {
            for (a_card, b_card) in a.cards.iter().zip(b.cards.iter()) {
                let card_value_map: HashMap<char, i32> = vec![
                    ('J', 0),
                    ('2', 2),
                    ('3', 3),
                    ('4', 4),
                    ('5', 5),
                    ('6', 6),
                    ('7', 7),
                    ('8', 8),
                    ('9', 9),
                    ('T', 10),
                    ('Q', 12),
                    ('K', 13),
                    ('A', 14),
                ]
                .into_iter()
                .collect();

                let value_a = card_value_map.get(a_card).unwrap();
                let value_b = card_value_map.get(b_card).unwrap();
                if value_a < value_b {
                    return Ordering::Less;
                } else if value_a > value_b {
                    return Ordering::Greater;
                }
            }
            Ordering::Equal
        } else {
            a.hand_type.cmp(&b.hand_type)
        }
    });

    for (rank, hand) in parsed_hands.iter().enumerate() {
        println!(
            "Rank {}: Hand: {:?}, Bid: {}, Type: {:?}",
            rank + 1,
            hand.cards,
            hand.bid,
            hand.hand_type
        );
    }

    let total_winnings: usize = parsed_hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank + 1))
        .sum();

    total_winnings
}

#[derive(Debug, PartialEq, Eq)]
struct Handv2 {
    cards: Vec<char>,
    hand_type: HandType,
    bid: usize,
}

impl Handv2 {
    fn new(cards: &str, bid: usize) -> Self {
        let card_vec: Vec<char> = cards.chars().collect();
        let hand_type = Handv2::hand_type(&card_vec);
        Handv2 {
            cards: card_vec,
            hand_type,
            bid,
        }
    }

    fn hand_type(cards: &[char]) -> HandType {
        let mut j_count = 0;
        for &card in cards {
            if card == 'J' {
                j_count += 1;
            }
        }

        if j_count == 0 {
            return Hand::determine_hand_type(cards);
        }

        let mut joker_count = 0;
        let mut frequency = HashMap::new();
        for &card in cards {
            if card != 'J' {
                *frequency.entry(card).or_insert(0) += 1;
            } else {
                joker_count += 1;
            }
        }

        let mut frequency_vec: Vec<i32> = frequency
            .values()
            .into_iter()
            .sorted_by(|a, b| b.cmp(&a))
            .map(|a| a.to_owned())
            .collect();

        match frequency_vec.first_mut() {
            Some(f) => *f += joker_count,
            None => frequency_vec.push(joker_count),
        }

        match frequency_vec.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
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
            6440
        )
    }

    #[test]
    fn solution_one() {
        assert_eq!(
            super::solution_one(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/one.txt",
                super::NUM
            )),
            248396258
        )
    }

    #[test]
    fn example_two() {
        assert_eq!(
            super::solution_two(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/two.test.txt",
                super::NUM
            )),
            5905
        )
    }

    #[test]
    fn solution_two() {
        assert_eq!(
            super::solution_two(format!(
                "/home/mtrottie/code/advent_2023/data/day{}/two.txt",
                super::NUM
            )),
            246436046
        )
    }
}
