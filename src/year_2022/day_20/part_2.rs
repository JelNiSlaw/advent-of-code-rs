use advent_of_code::Solve;

use super::shared::grove_coordinates;
use crate::year_2022::day_20::shared::mix_numbers;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "1640221678213"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        grove_coordinates(&mix_numbers(
            lines
                .into_iter()
                .map(|line| line.parse::<i64>().unwrap() * 811_589_153),
            10,
        ))
        .to_string()
    }
}
