use advent_of_code::Solve;

use super::shared::Directory;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "2940614"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let filesystem = Directory::parse_lines(lines);

        let free_space = 70_000_000 - filesystem.size();
        let missing_space = 30_000_000 - free_space;

        let smallest_dir = filesystem
            .all_directories()
            .into_iter()
            .map(Directory::size)
            .filter(|size| *size >= missing_space)
            .min()
            .unwrap();

        smallest_dir.to_string()
    }
}
