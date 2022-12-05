use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::shared::print_answer;

pub fn main() {
    let lines = BufReader::new(File::open("src/year_2022/day_2/input.txt").unwrap())
        .lines()
        .flatten();

    #[allow(clippy::identity_op)]
    let points = lines
        .map(|round| match round.as_ref() {
            "A X" => 3 + 0,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            _ => unreachable!(),
        })
        .sum::<u32>();

    print_answer(2022, 2, 2, points);
}
