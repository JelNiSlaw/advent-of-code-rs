use std::collections::HashSet;

use advent_of_code::Solve;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "7872"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let sum = lines
            .into_iter()
            .map(|line| {
                let mut chars = line.chars();
                let left = chars
                    .by_ref()
                    .take(line.chars().count() / 2)
                    .collect::<HashSet<_>>();
                let right = chars.by_ref().collect::<HashSet<_>>();

                let common = *left.intersection(&right).next().unwrap();

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
