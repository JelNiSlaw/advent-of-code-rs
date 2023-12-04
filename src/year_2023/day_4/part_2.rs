use std::collections::VecDeque;

use advent_of_code::Solve;

use super::shared::Card;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "15455663"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let mut cards = lines
            .into_iter()
            .map(|line| (Card::parse(&line), 1))
            .collect::<VecDeque<(_, u32)>>();

        let mut count = 0;

        loop {
            let Some((card, amount)) = cards.pop_front() else {
                break;
            };

            count += amount;

            for card in cards.iter_mut().take(card.count_winning_numbers()) {
                card.1 += amount;
            }
        }

        count.to_string()
    }
}
