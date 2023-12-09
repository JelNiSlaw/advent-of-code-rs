use advent_of_code::Solve;

use super::shared::{differences, parse_values};

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "1050"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        parse_values(lines)
            .into_iter()
            .map(|history| extrapolate_backwards(&history))
            .sum::<i32>()
            .to_string()
    }
}

fn extrapolate_backwards(history: &[i32]) -> i32 {
    if history.windows(2).all(|values| values[0] == values[1]) {
        return history[0];
    }

    history[0] - extrapolate_backwards(&differences(history))
}
