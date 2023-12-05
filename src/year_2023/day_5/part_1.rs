use advent_of_code::Solve;

use super::shared::Almanac;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "165788812"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let almanac = Almanac::parse(lines);

        almanac
            .seeds
            .iter()
            .map(|&seed| almanac.map_value(seed))
            .min()
            .unwrap()
            .to_string()
    }
}
