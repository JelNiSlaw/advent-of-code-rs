use advent_of_code::Solve;

use super::shared::{parse_sensors, Sensor};

const SIZE_LIMIT: i32 = 4_000_000;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "13743542639657"
    }

    // https://reddit.com/comments/zmcn64/-/j0b90nr
    fn solve(&self, lines: Vec<String>) -> String {
        let sensors = parse_sensors(lines.into_iter());

        let mut positive_segments = Vec::with_capacity(sensors.len() * 2);
        let mut negative_segments = Vec::with_capacity(sensors.len() * 2);

        for sensor in &sensors {
            let range = i32::try_from(sensor.range).unwrap();
            positive_segments.push(-sensor.x + sensor.y + range + 1);
            positive_segments.push(-sensor.x + sensor.y - range - 1);
            negative_segments.push(sensor.x + sensor.y + range + 1);
            negative_segments.push(sensor.x + sensor.y - range - 1);
        }

        let (x, y) = find_intersection(&positive_segments, &negative_segments, &sensors);

        let x = u64::try_from(x).unwrap();
        let y = u64::try_from(y).unwrap();

        (x * u64::try_from(SIZE_LIMIT).unwrap() + y).to_string()
    }
}

fn find_intersection(
    positive_segments: &[i32],
    negative_segments: &[i32],
    sensors: &[Sensor],
) -> (i32, i32) {
    for a in positive_segments {
        for b in negative_segments {
            if a > b || (a - b) % 2 != 0 {
                continue;
            }

            let x = (-a + b) / 2;
            let y = (a + b) / 2;

            if (0 < x && x < SIZE_LIMIT)
                && (0 < y && y < SIZE_LIMIT)
                && sensors
                    .iter()
                    .all(|sensor| x.abs_diff(sensor.x) + y.abs_diff(sensor.y) > sensor.range)
            {
                return (x, y);
            }
        }
    }

    unreachable!()
}
