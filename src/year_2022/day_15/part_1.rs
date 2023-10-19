use std::collections::HashSet;

use advent_of_code::Solve;

use super::shared::parse_sensors;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "4748135"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let sensors = parse_sensors(lines.into_iter());

        let xs = sensors
            .into_iter()
            .filter_map(|sensor| {
                let y_diff = sensor.y.abs_diff(2_000_000);
                let range_diff =
                    i32::try_from(sensor.range).unwrap() - i32::try_from(y_diff).unwrap() - 1;
                if range_diff > 0 {
                    Some((sensor.x - range_diff - 1)..=(sensor.x + range_diff))
                } else {
                    None
                }
            })
            .flatten()
            .collect::<HashSet<_>>();

        xs.len().to_string()
    }
}
