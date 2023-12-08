use advent_of_code::Solve;

use super::shared::Map;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "16897"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        Map::parse(lines)
            .path_length(*b"AAA", |node| node == *b"ZZZ")
            .to_string()
    }
}
