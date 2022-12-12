use advent_of_code::Solve;

use super::shared::breadth_first_search;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "388"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        breadth_first_search(lines.into_iter(), &['S', 'a'], 'E').to_string()
    }
}
