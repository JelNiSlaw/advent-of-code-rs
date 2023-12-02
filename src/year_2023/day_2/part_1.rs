use advent_of_code::Solve;

use super::shared::Game;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "2278"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        lines
            .into_iter()
            .map(|line| Game::parse(&line))
            .zip(1..)
            .filter(|(game, _)| {
                game.subsets
                    .iter()
                    .all(|subset| subset.red <= 12 && subset.green <= 13 && subset.blue <= 14)
            })
            .map(|(_, i)| i)
            .sum::<u32>()
            .to_string()
    }
}
