use advent_of_code::Solve;

use super::shared::Race;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "39132886"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let time = lines[0]
            .bytes()
            .skip("Time:     ".len())
            .filter(u8::is_ascii_digit)
            .map(char::from)
            .collect::<String>()
            .parse::<u64>()
            .unwrap();

        let distance = lines[1]
            .bytes()
            .skip("Distance: ".len())
            .filter(u8::is_ascii_digit)
            .map(char::from)
            .collect::<String>()
            .parse::<u64>()
            .unwrap();

        Race { time, distance }.ways_of_winning().to_string()
    }
}
