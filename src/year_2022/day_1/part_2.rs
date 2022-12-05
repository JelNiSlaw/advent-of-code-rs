use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::shared::print_answer;

pub fn main() {
    let lines = BufReader::new(File::open("src/year_2022/day_1/input.txt").unwrap())
        .lines()
        .flatten();

    let mut elves = vec![];
    let mut current = 0;

    for line in lines {
        if line.is_empty() {
            elves.push(current);
            current = 0;
            continue;
        }

        current += line.parse::<u32>().unwrap();
    }

    elves.sort_by(|a, b| b.cmp(a));
    let top_sum = elves.into_iter().take(3).sum::<u32>();

    print_answer(2022, 1, 2, top_sum);
}
