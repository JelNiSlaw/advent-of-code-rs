use advent_of_code::Solve;

use super::shared::Card;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "23678"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        lines
            .into_iter()
            .map(|line| Card::parse(&line))
            .map(|card| {
                let power = card.count_winning_numbers();
                if power == 0 {
                    0
                } else {
                    1 << (power - 1)
                }
            })
            .sum::<u16>()
            .to_string()
    }
}
