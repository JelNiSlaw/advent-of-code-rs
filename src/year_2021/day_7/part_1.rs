use advent_of_code::Solve;

use super::shared::parse_crabs;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "349769"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let crabs = parse_crabs(&lines[0]);

        let fuel_prices = (0..u16::try_from(crabs.len()).unwrap()).map(|position| {
            crabs
                .iter()
                .map(|&crab| u32::from(position.abs_diff(crab)))
                .sum::<u32>()
        });

        fuel_prices.into_iter().min().unwrap().to_string()
    }
}
