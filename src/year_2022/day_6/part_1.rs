use std::collections::{HashSet, VecDeque};

use advent_of_code::Solve;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "1760"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let line = lines.into_iter().next().unwrap();

        let mut last_chars = VecDeque::with_capacity(4);
        let mut position = 0;
        for (i, char) in line.chars().enumerate() {
            last_chars.push_back(char);

            if last_chars.len() >= 4 {
                let unique = last_chars.iter().collect::<HashSet<_>>();
                if unique.len() >= 4 {
                    position = i + 1;
                    break;
                }

                last_chars.pop_front();
            }
        }

        position.to_string()
    }
}
