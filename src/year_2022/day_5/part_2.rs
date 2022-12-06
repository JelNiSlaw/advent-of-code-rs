use advent_of_code::Solve;
use itertools::Itertools;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "TDGJQTZSL"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let mut lines = lines.into_iter();
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

            let mut items = Vec::with_capacity(move_amount);
            for _ in 0..move_amount {
                items.push(stacks[move_from].pop().unwrap());
            }
            stacks[move_to].extend(items.into_iter().rev());
        }

        stacks
            .into_iter()
            .map(|stack| *stack.last().unwrap())
            .collect::<String>()
    }
}
