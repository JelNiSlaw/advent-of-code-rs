use std::collections::HashSet;

use advent_of_code::Solve;

use super::shared::calculate_move;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "6494"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let mut tail_positions = HashSet::new();

        let mut head = (0, 0);
        let mut tail = (0, 0);

        for line in lines {
            let mut chars = line.chars();
            let direction = chars
                .by_ref()
                .take_while(|c| !c.is_ascii_whitespace())
                .collect::<String>();

            let amount = chars.by_ref().collect::<String>().parse::<u32>().unwrap();

            for _ in 0..amount {
                match direction.as_str() {
                    "R" => head.0 += 1,
                    "L" => head.0 -= 1,
                    "U" => head.1 += 1,
                    "D" => head.1 -= 1,
                    _ => unreachable!(),
                };

                let the_move = calculate_move(head, tail);
                tail.0 += the_move.0;
                tail.1 += the_move.1;

                tail_positions.insert(tail);
            }
        }

        tail_positions.len().to_string()
    }
}
