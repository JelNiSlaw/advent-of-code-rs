use advent_of_code::Solve;
use itertools::Itertools;

use super::shared::Race;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "2374848"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let times = lines[0]
            .bytes()
            .skip("Time:     ".len())
            .group_by(u8::is_ascii_digit);

        let times = times
            .into_iter()
            .filter(|&(is_digit, _)| is_digit)
            .map(|(_, bytes)| bytes.map(char::from).collect::<String>().parse().unwrap());

        let distances = lines[1]
            .bytes()
            .skip("Distance: ".len())
            .group_by(u8::is_ascii_digit);

        let distances = distances
            .into_iter()
            .filter(|&(is_digit, _)| is_digit)
            .map(|(_, bytes)| bytes.map(char::from).collect::<String>().parse().unwrap());

        times
            .zip(distances)
            .map(|(time, distance)| Race { time, distance }.ways_of_winning())
            .product::<u64>()
            .to_string()
    }
}
