use advent_of_code::Solve;

use crate::year_2023::day_4::shared::Card;

pub enum CardCache {
    Card(Card),
    CardCount(u32),
}

struct Cards(Vec<CardCache>);

impl Cards {
    fn count_cards(&mut self, i: usize) -> u32 {
        match &self.0[i] {
            CardCache::Card(card) => {
                let card_count = 1
                    + (1..=card.count_winning_numbers())
                        .map(|offset| self.count_cards(i + offset))
                        .sum::<u32>();

                self.0[i] = CardCache::CardCount(card_count);
                card_count
            }
            CardCache::CardCount(cards) => *cards,
        }
    }

    fn count_all_cards(&mut self) -> u32 {
        (0..self.0.len()).map(|i| self.count_cards(i)).sum()
    }
}

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "15455663"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let mut cards = Cards(
            lines
                .into_iter()
                .map(|line| CardCache::Card(Card::parse(&line)))
                .collect::<Vec<_>>(),
        );

        cards.count_all_cards().to_string()
    }
}
