use std::collections::{HashSet, VecDeque};
use std::fs;

use crate::shared::print_answer;

pub fn main() {
    let buffer = fs::read_to_string("src/year_2022/day_6/input.txt").unwrap();

    let mut last_chars = VecDeque::with_capacity(4);
    let mut position = 0;
    for (i, char) in buffer.chars().enumerate() {
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

    print_answer(2022, 6, 1, position);
}
