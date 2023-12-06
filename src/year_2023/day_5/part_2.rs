use advent_of_code::Solve;

use super::shared::Almanac;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "1928058"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let almanac = Almanac::parse(lines);

        let ranges = almanac
            .seeds
            .chunks_exact(2)
            .map(|range| range[0]..range[0] + range[1])
            .collect::<Vec<_>>();

        almanac
            .map_ranges(ranges)
            .into_iter()
            .map(|range| range.start)
            .min()
            .unwrap()
            .to_string()
    }
}
