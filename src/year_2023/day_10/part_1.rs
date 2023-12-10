use advent_of_code::Solve;

use crate::year_2023::day_10::shared::Maze;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "7173"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let maze = Maze::parse(lines);

        let pipe_count = maze
            .pipe_loop_locations()
            .into_iter()
            .map(|row| row.into_iter().filter(|&pipe| pipe.is_some()).count())
            .sum::<usize>();

        (pipe_count / 2).to_string()
    }
}
