use advent_of_code::Solve;

use super::shared::Directory;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "1427048"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let filesystem = Directory::parse_lines(lines);

        let sum = filesystem
            .all_directories()
            .into_iter()
            .map(Directory::size)
            .filter(|size| *size <= 100_000)
            .sum::<u32>();

        sum.to_string()
    }
}
