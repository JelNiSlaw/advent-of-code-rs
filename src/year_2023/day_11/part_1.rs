use advent_of_code::Solve;

use super::shared::{distance_sum, Image};

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "9742154"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        distance_sum(&Image::parse(lines).galaxies(2)).to_string()
    }
}
