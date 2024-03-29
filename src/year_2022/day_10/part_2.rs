use advent_of_code::{utils, Solve};

use super::shared::{parse_instructions, Instruction};

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "EZFPRAKL"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let mut register: i32 = 1;
        let mut cycle = 0;
        let mut display = String::new();

        for instruction in parse_instructions(lines) {
            cycle += 1;

            if register.abs_diff((cycle - 1) % 40) <= 1 {
                display.push('#');
            } else {
                display.push('.');
            }

            if cycle % 40 == 0 {
                display.push('\n');
            }

            match instruction {
                Instruction::AddX(value) => {
                    register += value;
                }
                Instruction::NoOp => (),
            };
        }

        utils::decode_big_letters(&display)
    }
}
