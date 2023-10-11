use advent_of_code::Solve;

use super::shared;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "3969000"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let number_count = u32::try_from(lines.len()).unwrap();
        let bit_count = lines[0].bytes().len();

        let numbers = lines
            .into_iter()
            .map(|line| u32::from_str_radix(&line, 2).unwrap())
            .collect::<Vec<_>>();

        let bit_counts = shared::bit_counts(&numbers, bit_count);
        let most_common_bits = most_common_bits(&bit_counts, number_count);

        let gamma_rate = most_common_bits
            .iter()
            .rev()
            .zip(0..)
            .filter(|(bit, _)| **bit)
            .map(|(_, i)| 2u32.pow(i))
            .sum::<u32>();

        let epsilon_rate = most_common_bits
            .iter()
            .rev()
            .zip(0..)
            .filter(|(bit, _)| !**bit)
            .map(|(_, i)| 2u32.pow(i))
            .sum::<u32>();

        (gamma_rate * epsilon_rate).to_string()
    }
}

fn most_common_bits(bit_counts: &[u32], line_count: u32) -> Vec<bool> {
    bit_counts
        .iter()
        .map(|bit| *bit * 2 > line_count)
        .collect::<Vec<_>>()
}
