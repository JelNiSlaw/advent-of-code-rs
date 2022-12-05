use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

use crate::shared::print_answer;

pub fn main() {
    let mut lines = BufReader::new(File::open("src/year_2022/day_5/input.txt").unwrap())
        .lines()
        .flatten();

    let mut stacks = Vec::new();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        for (i, mut item) in line.chars().chunks(4).into_iter().enumerate() {
            if i >= stacks.len() {
                stacks.push(Vec::new());
            }

            if item.by_ref().next().unwrap() == '[' {
                stacks[i].insert(0, item.by_ref().next().unwrap());
            }
        }
    }

    for line in lines.by_ref() {
        let mut chars = line.chars();

        let move_amount = chars
            .by_ref()
            .skip(5)
            .take_while(char::is_ascii_digit)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let move_from = chars
            .by_ref()
            .skip(5)
            .take_while(char::is_ascii_digit)
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
            - 1;
        let move_to = chars
            .by_ref()
            .skip(3)
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
            - 1;

        for _ in 0..move_amount {
            let item = stacks[move_from].pop().unwrap();
            stacks[move_to].push(item);
        }
    }

    let top = stacks
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect::<String>();

    print_answer(2022, 5, 1, top);
}
