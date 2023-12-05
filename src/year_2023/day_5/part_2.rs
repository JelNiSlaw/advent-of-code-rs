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

        let mut i = 0;

        loop {
            let seed = almanac.reverse_map_value(i);

            if ranges.iter().any(|range| range.contains(&seed)) {
                break;
            }

            i += 1;
        }

        i.to_string()
    }
}
