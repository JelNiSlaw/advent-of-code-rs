use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::shared::print_answer;

pub fn main() {
    let lines = BufReader::new(File::open("src/year_2022/day_3/input.txt").unwrap())
        .lines()
        .flatten();

    let sum = lines
        .map(|line| {
            let mut chars = line.chars();
            let left = chars
                .by_ref()
                .take(line.chars().count() / 2)
                .collect::<HashSet<_>>();
            let right = chars.by_ref().collect::<HashSet<_>>();

            let common = *left.intersection(&right).next().unwrap();

            match common {
                'a'..='z' => common as u32 - 96,
                'A'..='Z' => common as u32 - 38,
                _ => unreachable!(),
            }
        })
        .sum::<u32>();

    print_answer(2022, 3, 1, sum);
}
