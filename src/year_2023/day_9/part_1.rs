use advent_of_code::Solve;

use super::shared::{differences, parse_values};

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "1708206096"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        parse_values(lines)
            .into_iter()
            .map(|history| extrapolate(&history))
            .sum::<i32>()
            .to_string()
    }
}

fn extrapolate(history: &[i32]) -> i32 {
    if history.windows(2).all(|values| values[0] == values[1]) {
        return history[0];
    }

    history.last().unwrap() + extrapolate(&differences(history))
}
