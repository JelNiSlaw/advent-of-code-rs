use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::shared::print_answer;

pub fn main() {
    let lines = BufReader::new(File::open("src/year_2022/day_1/input.txt").unwrap())
        .lines()
        .flatten();

    let mut max = 0;
    let mut current = 0;

    for line in lines {
        if line.is_empty() {
            max = current.max(max);
            current = 0;
            continue;
        }

        current += line.parse::<u32>().unwrap();
    }

    print_answer(2022, 1, 1, max);
}
