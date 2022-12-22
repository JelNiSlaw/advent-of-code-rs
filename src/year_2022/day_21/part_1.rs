use advent_of_code::Solve;

use super::shared::{monkey_value, parse_monkeys};

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "66174565793494"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let mut monkeys = parse_monkeys(lines.into_iter());
        monkey_value("root", &mut monkeys).to_string()
    }
}
