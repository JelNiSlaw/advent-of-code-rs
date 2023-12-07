use advent_of_code::Solve;

use super::shared::Hand;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "248781813"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let mut hands = lines
            .into_iter()
            .map(|line| Hand::parse(&line, true))
            .collect::<Vec<_>>();

        hands.sort_by_cached_key(|hand| (hand.r#type(), hand.cards));

        hands
            .into_iter()
            .rev()
            .zip(1..)
            .map(|(hand, i)| hand.bid * i)
            .sum::<u32>()
            .to_string()
    }
}
