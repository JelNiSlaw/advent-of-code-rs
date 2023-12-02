use advent_of_code::Solve;

use super::shared::Game;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "67953"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        lines
            .into_iter()
            .map(|line| Game::parse(&line))
            .map(|game| {
                let min_red = game.subsets.iter().map(|game| game.red).max().unwrap();
                let min_green = game.subsets.iter().map(|game| game.green).max().unwrap();
                let min_blue = game.subsets.iter().map(|game| game.blue).max().unwrap();

                u32::from(min_red) * u32::from(min_green) * u32::from(min_blue)
            })
            .sum::<u32>()
            .to_string()
    }
}
