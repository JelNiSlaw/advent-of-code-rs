use advent_of_code::Solve;

use super::shared::{parse_fish, simulate_fish};

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "352151"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let fish = parse_fish(&lines.into_iter().next().unwrap());
        simulate_fish(fish, 80).to_string()
    }
}
