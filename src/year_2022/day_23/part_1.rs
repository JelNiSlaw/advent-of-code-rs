use std::collections::HashSet;

use advent_of_code::Solve;

use super::shared::{diffuse, parse_elves, DIRECTIONS};

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "3877"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let mut positions = parse_elves(lines.into_iter());
        let mut directions = DIRECTIONS;

        for _ in 0..10 {
            positions = diffuse(&positions, directions);
            directions.rotate_left(1);
        }

        elves_area(&positions).to_string()
    }
}

fn elves_area(elves: &HashSet<(i32, i32)>) -> i32 {
    let min_x = elves.iter().map(|elve| elve.0).min().unwrap();
    let max_x = elves.iter().map(|elve| elve.0).max().unwrap();
    let min_y = elves.iter().map(|elve| elve.1).min().unwrap();
    let max_y = elves.iter().map(|elve| elve.1).max().unwrap();

    (max_x - min_x + 1) * (max_y - min_y + 1) - i32::try_from(elves.len()).unwrap()
}
