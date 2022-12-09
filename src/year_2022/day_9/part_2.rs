use std::collections::HashSet;

use advent_of_code::Solve;

use super::shared::calculate_move;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "2691"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let mut tail_positions = HashSet::new();

        let mut rope = [(0, 0); 10];

        for line in lines {
            let mut chars = line.chars();
            let direction = chars
                .by_ref()
                .take_while(|c| !c.is_ascii_whitespace())
                .collect::<String>();

            let amount = chars.by_ref().collect::<String>().parse::<u32>().unwrap();

            for _ in 0..amount {
                match direction.as_str() {
                    "R" => rope[0].0 += 1,
                    "L" => rope[0].0 -= 1,
                    "U" => rope[0].1 += 1,
                    "D" => rope[0].1 -= 1,
                    _ => unreachable!(),
                };

                for i in 0..rope.len() - 1 {
                    let the_move = calculate_move(rope[i], rope[i + 1]);
                    rope[i + 1].0 += the_move.0;
                    rope[i + 1].1 += the_move.1;

                    tail_positions.insert(rope.last().copied().unwrap());
                }
            }
        }

        tail_positions.len().to_string()
    }
}
