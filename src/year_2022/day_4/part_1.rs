use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::shared::print_answer;

pub fn main() {
    let lines = BufReader::new(File::open("src/year_2022/day_4/input.txt").unwrap())
        .lines()
        .flatten();

    let count = lines
        .filter(|line| {
            let [start_a, end_a, start_b, end_b] = parse_line(line);

            (start_a <= start_b && end_a >= end_b) || (start_b <= start_a && end_b >= end_a)
        })
        .count();

    print_answer(2022, 4, 1, count);
}

fn parse_line(line: &str) -> [u32; 4] {
    let mut chars = line.chars();
    let start_a = chars
        .by_ref()
        .take_while(|c| *c != '-')
        .collect::<String>()
        .parse::<u32>()
        .unwrap();
    let end_a = chars
        .by_ref()
        .take_while(|c| *c != ',')
        .collect::<String>()
        .parse::<u32>()
        .unwrap();
    let start_b = chars
        .by_ref()
        .take_while(|c| *c != '-')
        .collect::<String>()
        .parse::<u32>()
        .unwrap();
    let end_b = chars.by_ref().collect::<String>().parse::<u32>().unwrap();

    [start_a, end_a, start_b, end_b]
}
