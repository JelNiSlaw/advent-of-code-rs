use advent_of_code::Solve;

use super::shared::{grove_coordinates, mix_numbers};

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "7395"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let numbers = mix_numbers(
            lines.into_iter().map(|line| line.parse::<i64>().unwrap()),
            1,
        );

        grove_coordinates(&numbers).to_string()
    }
}
