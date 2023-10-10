use std::collections::HashSet;

use advent_of_code::Solve;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "2497"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let sum = lines
            .chunks(3)
            .map(|chunk| {
                let rucksacks = chunk
                    .iter()
                    .map(|rucksack| rucksack.chars().collect::<HashSet<_>>());

                let common = rucksacks
                    .reduce(|a, b| a.intersection(&b).copied().collect())
                    .unwrap()
                    .into_iter()
                    .next()
                    .unwrap();

                match common {
                    'a'..='z' => common as u32 - 96,
                    'A'..='Z' => common as u32 - 38,
                    _ => unreachable!(),
                }
            })
            .sum::<u32>();

        sum.to_string()
    }
}
