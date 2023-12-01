use advent_of_code::Solve;

const TEXT_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "54719"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        lines
            .into_iter()
            .map(|line| {
                let first_digit = find_first_digit(line.bytes(), false);
                let last_digit = find_first_digit(line.bytes().rev(), true);
                first_digit * 10 + last_digit
            })
            .sum::<u32>()
            .to_string()
    }
}

fn find_first_digit(line: impl Iterator<Item = u8>, reversed: bool) -> u32 {
    let mut text = String::new();

    for byte in line {
        if byte.is_ascii_digit() {
            return char::from(byte).to_digit(10).unwrap();
        }

        if reversed {
            text.insert(0, char::from(byte));
        } else {
            text.push(char::from(byte));
        }

        for (text_digit, i) in TEXT_DIGITS.into_iter().zip(1..) {
            let condition = if reversed {
                text.starts_with(text_digit)
            } else {
                text.ends_with(text_digit)
            };

            if condition {
                return i;
            }
        }
    }

    unreachable!();
}
