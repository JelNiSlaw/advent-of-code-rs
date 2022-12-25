use advent_of_code::Solve;

use super::shared::{diffuse, parse_elves, DIRECTIONS};

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "982"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let mut positions = parse_elves(lines.into_iter());
        let mut directions = DIRECTIONS;

        let mut round = 0;
        loop {
            round += 1;
            let new_positions = diffuse(&positions, directions);
            if new_positions == positions {
                break;
            };
            positions = new_positions;
            directions.rotate_left(1);
        }

        round.to_string()
    }
}
