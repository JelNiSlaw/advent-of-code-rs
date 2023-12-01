use advent_of_code::Solve;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "55971"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        lines
            .into_iter()
            .map(|line| {
                let first_digit = find_first_digit(line.bytes());
                let last_digit = find_first_digit(line.bytes().rev());
                first_digit * 10 + last_digit
            })
            .sum::<u32>()
            .to_string()
    }
}

fn find_first_digit(mut line: impl Iterator<Item = u8>) -> u32 {
    let byte = line.find(u8::is_ascii_digit).unwrap();
    char::from(byte).to_digit(10).unwrap()
}
