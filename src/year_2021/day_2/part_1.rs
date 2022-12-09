use advent_of_code::Solve;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "1635930"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let mut position = 0;
        let mut depth = 0;

        for line in lines {
            let mut chars = line.chars();
            let instruction = chars
                .by_ref()
                .take_while(|c| !c.is_ascii_whitespace())
                .collect::<String>();
            let amount = chars.by_ref().collect::<String>().parse::<u32>().unwrap();

            match instruction.as_str() {
                "forward" => position += amount,
                "down" => depth += amount,
                "up" => depth -= amount,
                _ => unreachable!(),
            }
        }

        (position * depth).to_string()
    }
}
