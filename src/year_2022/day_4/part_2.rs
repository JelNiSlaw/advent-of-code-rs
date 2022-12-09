use advent_of_code::Solve;

use super::shared::parse_line;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "770"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let count = lines
            .into_iter()
            .filter(|line| {
                let [start_a, end_a, start_b, end_b] = parse_line(line);

                start_a <= end_b && start_b <= end_a
            })
            .count();

        count.to_string()
    }
}
