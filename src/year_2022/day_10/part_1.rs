use advent_of_code::Solve;

use super::shared::{parse_instructions, Instruction};

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "12460"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let instructions = parse_instructions(lines);

        let mut register = 1;
        let mut cycle = 0;
        let mut measurements = Vec::new();

        for instruction in instructions {
            cycle += 1;

            if (cycle + 20) % 40 == 0 {
                measurements.push(cycle * register);
            }

            match instruction {
                Instruction::AddX(value) => {
                    register += value;
                }
                Instruction::NoOp => (),
            };
        }

        measurements.into_iter().sum::<i32>().to_string()
    }
}
