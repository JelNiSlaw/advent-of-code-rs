use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

use crate::shared::print_answer;

pub fn main() {
    let lines = BufReader::new(File::open("src/year_2022/day_3/input.txt").unwrap())
        .lines()
        .flatten();

    let sum = lines
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let rucksacks = chunk
                .into_iter()
                .map(|rucksack| rucksack.chars().collect::<HashSet<_>>());

            let common = rucksacks
                .reduce(|a, b| a.intersection(&b).copied().collect())
                .unwrap()
                .into_iter()
                .next()
                .unwrap();

            match common {
                'a'..='z' => common as u32 - 96,
                'A'..='Z' => common as u32 - 38,
                _ => unreachable!(),
            }
        })
        .sum::<u32>();

    print_answer(2022, 3, 2, sum);
}
