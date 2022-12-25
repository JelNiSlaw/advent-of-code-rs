use advent_of_code::Solve;

use super::shared::from_snafu;
use crate::year_2022::day_25::shared::to_snafu;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "2---1010-0=1220-=010"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        to_snafu(lines.into_iter().map(|line| from_snafu(&line)).sum())
    }
}
