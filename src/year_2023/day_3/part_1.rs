use advent_of_code::Solve;

use super::shared::{decimal_len, EngineSchematic};

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "536576"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let engine_schematic = EngineSchematic::parse(&lines, false);

        engine_schematic
            .numbers
            .into_iter()
            .filter(|number| {
                let x_start_offset = if number.x == 0 { 0 } else { -1 };

                (x_start_offset..=(i16::from(decimal_len(number.value))) + 1).any(|x_offset| {
                    let y_start_offset = if number.y == 0 { 0 } else { -1 };

                    (y_start_offset..=1).any(|y_offset| {
                        engine_schematic.symbols.contains(&(
                            u8::try_from(i16::try_from(number.x).unwrap() + x_offset).unwrap(),
                            u8::try_from(i16::try_from(number.y).unwrap() + y_offset).unwrap(),
                        ))
                    })
                })
            })
            .map(|number| u32::from(number.value))
            .sum::<u32>()
            .to_string()
    }
}
