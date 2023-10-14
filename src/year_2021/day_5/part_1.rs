use advent_of_code::Solve;

use super::shared::{evaluate_ocean_floor, Line};

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "3990"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let lines = lines.into_iter().map(|line| Line::parse(&line));
        let ocean_floor = evaluate_ocean_floor(lines, true);

        let dangerous_areas = ocean_floor
            .into_iter()
            .filter(|&(_, value)| value >= 2)
            .count();

        dangerous_areas.to_string()
    }
}
