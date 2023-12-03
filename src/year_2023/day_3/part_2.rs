use std::collections::BTreeMap;

use advent_of_code::Solve;

use crate::year_2023::day_3::shared::{decimal_len, EngineSchematic};

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "75741499"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let engine_schematic = EngineSchematic::parse(&lines, true);

        let mut gear_states: BTreeMap<(u8, u8), Vec<u16>> = BTreeMap::new();

        for number in engine_schematic.numbers {
            let x_start_offset = if number.x == 0 { 0 } else { -1 };

            for x_offset in x_start_offset..=i16::from(decimal_len(number.value)) + 1 {
                let y_start_offset = if number.y == 0 { 0 } else { -1 };

                for y_offset in y_start_offset..=1 {
                    let x = u8::try_from(i16::try_from(number.x).unwrap() + x_offset).unwrap();
                    let y = u8::try_from(i16::try_from(number.y).unwrap() + y_offset).unwrap();

                    if engine_schematic.symbols.contains(&(x, y)) {
                        gear_states
                            .entry((x, y))
                            .and_modify(|numbers| numbers.push(number.value))
                            .or_insert_with(|| vec![number.value]);
                    }
                }
            }
        }

        gear_states
            .into_iter()
            .filter_map(|(_, numbers)| {
                let &[first, second] = numbers.as_slice() else {
                    return None;
                };

                Some(u32::from(first) * u32::from(second))
            })
            .sum::<u32>()
            .to_string()
    }
}
