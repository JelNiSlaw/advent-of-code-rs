use advent_of_code::Solve;

use super::shared::parse_sensors;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "4748135"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let sensors = parse_sensors(lines.into_iter());

        let mut ranges = sensors
            .into_iter()
            .filter_map(|sensor| {
                let y_diff = sensor.y.abs_diff(2_000_000);
                let range_diff =
                    i32::try_from(sensor.range).unwrap() - i32::try_from(y_diff).unwrap() - 1;
                if range_diff > 0 {
                    Some((sensor.x - range_diff - 1, sensor.x + range_diff))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        ranges.sort_unstable();

        let mut taken_positions = 0;
        let mut last_position: Option<i32> = None;

        for range in ranges {
            let taken_end =
                last_position.map_or(range.0, |last_position| last_position.max(range.0));

            if range.1 > taken_end {
                taken_positions += range.1 - taken_end + 1;
            }

            if last_position.map_or(true, |last_position| range.1 > last_position) {
                last_position = Some(range.1 + 1);
            }
        }

        taken_positions.to_string()
    }
}
