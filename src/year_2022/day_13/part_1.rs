use advent_of_code::Solve;

use super::shared::{right_order, Data};

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "6046"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let sum = lines
            .into_iter()
            .filter(|line| !line.is_empty())
            .map(|line| Data::parse(line.strip_prefix('[').unwrap().strip_suffix(']').unwrap()))
            .collect::<Vec<_>>()
            .chunks_exact(2)
            .enumerate()
            .filter_map(|(i, pair)| {
                if right_order(&pair[0], &pair[1]).unwrap() {
                    Some(i + 1)
                } else {
                    None
                }
            })
            .sum::<usize>();

        sum.to_string()
    }
}
