use advent_of_code::Solve;

use super::shared::{parse_instructions, parse_stacks};

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "TDGJQTZSL"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let mut lines = lines.into_iter();
        let mut stacks = parse_stacks(lines.by_ref());

        for instruction in parse_instructions(lines) {
            let stack = stacks.get_mut(instruction.move_from).unwrap();
            let items = stack
                .drain(stack.len() - instruction.move_amount..)
                .collect::<Vec<_>>();
            stacks[instruction.move_to].extend(items);
        }

        stacks
            .into_iter()
            .map(|stack| *stack.last().unwrap())
            .collect::<String>()
    }
}
