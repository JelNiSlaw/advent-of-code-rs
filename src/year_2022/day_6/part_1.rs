use advent_of_code::Solve;

use super::shared::find_marker;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "1760"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        find_marker(lines.first().unwrap(), 4).to_string()
    }
}
