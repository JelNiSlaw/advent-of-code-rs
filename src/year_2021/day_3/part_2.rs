use advent_of_code::Solve;

use super::shared::bit_counts;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "4267809"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let bit_count = lines[0].bytes().len();

        let numbers = lines
            .into_iter()
            .map(|line| u32::from_str_radix(&line, 2).unwrap())
            .collect::<Vec<_>>();

        let oxygen_generator_rating = system_rating(numbers.clone(), bit_count, false);
        let co2_scrubber_rating = system_rating(numbers, bit_count, true);

        (oxygen_generator_rating * co2_scrubber_rating).to_string()
    }
}

fn system_rating(mut numbers: Vec<u32>, bit_count: usize, least_common: bool) -> u32 {
    for n in 0..bit_count {
        let number_count = u32::try_from(numbers.len()).unwrap();
        let bit_counts = bit_counts(&numbers, bit_count);
        let mut most_common = bit_counts[n] * 2 >= number_count;

        if least_common {
            most_common = !most_common;
        }

        numbers.retain(|number| (number & 1 << (bit_count - n - 1) != 0) == most_common);

        if numbers.len() == 1 {
            break;
        }
    }

    numbers.into_iter().next().unwrap()
}
